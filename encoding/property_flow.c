/**
 * property_flow.c — 产权流实现
 *
 * 三级侵权裁决:
 *   第0级: 登记检查 → interlock_check_symbol
 *   第1级: 容器权限 → 检查请求者是否有权
 *   第2级: 历史追溯 → 检查是否有侵权历史
 */

#include "property_flow.h"
#include "global_interlock.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void property_flow_init(property_flow_t *pf) {
    memset(pf, 0, sizeof(*pf));
    pf->magic = PROPERTY_FLOW_MAGIC;
    pf->record_count = 0;
    pf->request_count = 0;
    pf->request_head = 0;
    pf->total_checks = 0;
    pf->total_denied = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 登记产权
 * ═══════════════════════════════════════════════════════════════ */

int property_flow_register(property_flow_t *pf, uint32_t symbol, uint8_t owner, int is_public) {
    if (!pf || pf->magic != PROPERTY_FLOW_MAGIC) return -1;
    if (pf->record_count >= 254) return -1;
    
    /* 检查是否已登记 */
    for (uint16_t i = 0; i < pf->record_count; i++) {
        if (pf->records[i].symbol == symbol) {
            /* 已存在 · 更新 */
            pf->records[i].owner_container = owner;
            pf->records[i].is_public = is_public ? 1 : 0;
            return 0;
        }
    }
    
    /* 新登记 */
    property_record_t *r = &pf->records[pf->record_count];
    r->symbol = symbol;
    r->owner_container = owner;
    r->registered = 1;
    r->is_public = is_public ? 1 : 0;
    r->has_history = 0;
    r->use_count = 0;
    r->deny_count = 0;
    pf->record_count++;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 三级侵权裁决
 * ═══════════════════════════════════════════════════════════════ */

property_verdict_t property_flow_check(property_flow_t *pf, uint32_t symbol, uint8_t requester, use_type_t use_type) {
    if (!pf || pf->magic != PROPERTY_FLOW_MAGIC) return PROPERTY_DENY_UNREGISTERED;
    
    pf->total_checks++;
    
    /* 记录请求 */
    request_record_t *req = &pf->requests[pf->request_head];
    req->symbol = symbol;
    req->requester = requester;
    req->use_type = (uint8_t)use_type;
    pf->request_head = (pf->request_head + 1) % 1024;
    if (pf->request_count < 1024) pf->request_count++;
    
    /* ─── 第0级: 登记检查 ─── */
    int found = 0;
    property_record_t *record = NULL;
    for (uint16_t i = 0; i < pf->record_count; i++) {
        if (pf->records[i].symbol == symbol) {
            found = 1;
            record = &pf->records[i];
            break;
        }
    }
    
    if (!found) {
        pf->total_denied++;
        req->verdict = PROPERTY_DENY_UNREGISTERED;
        return PROPERTY_DENY_UNREGISTERED;
    }
    
    /* ─── 第1级: 容器权限 ─── */
    /* 公共符号允许任何人读取 */
    if (record->is_public && use_type == USE_READ) {
        record->use_count++;
        req->verdict = PROPERTY_ALLOW;
        return PROPERTY_ALLOW;
    }
    
    /* 产权所有者总是允许 */
    if (record->owner_container == requester) {
        record->use_count++;
        req->verdict = PROPERTY_ALLOW;
        return PROPERTY_ALLOW;
    }
    
    /* 非公共符号的非所有者使用 → 检查使用类型 */
    if (use_type == USE_READ) {
        /* 只读使用非公共符号 → 有条件 (需要知道存在) */
        record->use_count++;
        req->verdict = PROPERTY_CONDITIONAL;
        return PROPERTY_CONDITIONAL;
    }
    
    if (use_type == USE_WRITE || use_type == USE_EXPORT || use_type == USE_DERIVE) {
        /* 写入/导出/派生使用非公共符号 → 拒绝 */
        record->deny_count++;
        pf->total_denied++;
        req->verdict = PROPERTY_DENY_WRONG_OWNER;
        return PROPERTY_DENY_WRONG_OWNER;
    }
    
    /* ─── 第2级: 历史追溯 ─── */
    if (record->has_history) {
        /* 有侵权历史的符号 · 非所有者一律拒绝 */
        if (requester != record->owner_container) {
            record->deny_count++;
            pf->total_denied++;
            req->verdict = PROPERTY_DENY_HISTORY;
            return PROPERTY_DENY_HISTORY;
        }
    }
    
    /* 默认允许 (只读) */
    record->use_count++;
    req->verdict = PROPERTY_ALLOW;
    return PROPERTY_ALLOW;
}

/* ═══════════════════════════════════════════════════════════════
 * 授予使用权
 * ═══════════════════════════════════════════════════════════════ */

void property_flow_grant(property_flow_t *pf, uint32_t symbol, uint8_t grantee) {
    if (!pf || pf->magic != PROPERTY_FLOW_MAGIC) return;
    
    for (uint16_t i = 0; i < pf->record_count; i++) {
        if (pf->records[i].symbol == symbol) {
            /* 找到符号 · 标记为公共 (授权给所有人) */
            pf->records[i].is_public = 1;
            return;
        }
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 字符串转换
 * ═══════════════════════════════════════════════════════════════ */

const char* property_verdict_str(property_verdict_t v) {
    switch (v) {
        case PROPERTY_ALLOW:             return "ALLOW (允许)";
        case PROPERTY_DENY_UNREGISTERED: return "DENY (未登记)";
        case PROPERTY_DENY_WRONG_OWNER:  return "DENY (产权不符)";
        case PROPERTY_DENY_HISTORY:      return "DENY (侵权历史)";
        case PROPERTY_CONDITIONAL:       return "CONDITIONAL (有条件)";
        default:                         return "UNKNOWN";
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 统计报告
 * ═══════════════════════════════════════════════════════════════ */

void property_flow_stats(const property_flow_t *pf, FILE *out) {
    if (!pf || pf->magic != PROPERTY_FLOW_MAGIC || !out) return;
    
    fprintf(out, "═══ 产权流 ═══\n");
    fprintf(out, "产权记录: %u symbols\n", pf->record_count);
    fprintf(out, "请求历史: %u / 1024\n", pf->request_count);
    fprintf(out, "总检查:   %u\n", pf->total_checks);
    fprintf(out, "总拒绝:   %u", pf->total_denied);
    if (pf->total_checks > 0) {
        fprintf(out, " (%.1f%%)", (float)pf->total_denied / pf->total_checks * 100);
    }
    fprintf(out, "\n");
    
    /* 产权表 */
    if (pf->record_count > 0) {
        fprintf(out, "\n[产权表]\n");
        for (uint16_t i = 0; i < pf->record_count && i < 20; i++) {
            const property_record_t *r = &pf->records[i];
            fprintf(out, "  0x%02X  container=%u  %s  used=%u  denied=%u\n",
                    r->symbol, r->owner_container,
                    r->is_public ? "PUBLIC" : "PRIVATE",
                    r->use_count, r->deny_count);
        }
        if (pf->record_count > 20) {
            fprintf(out, "  ... (%u more)\n", pf->record_count - 20);
        }
    }
}
