/**
 * negative_sample.c — 负样本容器实现
 *
 * 免疫记忆库 + 回写协议 + 裁定器
 *
 * 核心逻辑:
 *   1. 新组合进来 → 先查记忆库
 *   2. 命中 → 立即拒绝 (已知负样本)
 *   3. 未命中 → 送外部审判 → 拒绝后回写记忆库
 *   4. 重复拒绝 → 冻结 (不再接受申诉)
 *   5. 记忆库满 → 驱逐最旧条目
 */

#include "negative_sample.h"
#include "global_interlock.h"
#include <string.h>

/* 拒绝原因名称 */
static const char *REASON_NAMES[] = {
    "无", "跨层级", "跨类型", "跨锚点", "审美失败",
    "编织失败", "黑名单", "重复拒绝", "降解产物"
};

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void negative_init(negative_sample_t *ns) {
    memset(ns, 0, sizeof(*ns));
    ns->magic = NEGATIVE_SAMPLE_MAGIC;
    ns->version = 1;
    ns->memory_count = 0;
    ns->memory_head = 0;
    ns->protocol.auto_freeze = 3;    /* 重复3次后冻结 */
    ns->protocol.memory_full = 0;
    ns->v_core = 100;               /* 默认核心值 */
    ns->tolerance = 10;             /* 默认容差 */
}

void negative_set_core(negative_sample_t *ns, uint32_t v_core, uint32_t tol) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return;
    ns->v_core = v_core;
    ns->tolerance = tol;
}

/* ═══════════════════════════════════════════════════════════════
 * 查记忆库
 * ═══════════════════════════════════════════════════════════════ */

int negative_lookup(const negative_sample_t *ns,
                     uint32_t sym_a, uint32_t sym_b,
                     reject_reason_t *reason_out) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return -1;
    
    for (uint16_t i = 0; i < ns->memory_count; i++) {
        const negative_entry_t *e = &ns->memory[i];
        /* 双向匹配 (a,b) 和 (b,a) 视为同一条目 */
        if ((e->symbol_a == sym_a && e->symbol_b == sym_b) ||
            (e->symbol_a == sym_b && e->symbol_b == sym_a)) {
            if (reason_out) *reason_out = (reject_reason_t)e->reason;
            return (int)i;  /* 返回记忆库索引 */
        }
    }
    return -1;  /* 未找到 */
}

/* ═══════════════════════════════════════════════════════════════
 * 裁定
 * ═══════════════════════════════════════════════════════════════ */

verdict_t negative_judge(negative_sample_t *ns,
                          uint32_t sym_a, uint32_t sym_b,
                          reject_reason_t *reason_out) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return VERDICT_NEW_REJECT;
    
    /* 查记忆库 */
    reject_reason_t reason;
    int idx = negative_lookup(ns, sym_a, sym_b, &reason);
    
    if (idx >= 0) {
        /* 已知负样本 */
        ns->total_hits++;
        ns->memory[idx].count++;  /* 重复次数+1 */
        
        /* 检查是否需要冻结 */
        if (ns->memory[idx].count >= ns->protocol.auto_freeze) {
            ns->memory[idx].frozen = 1;
            ns->protocol.frozen_count++;
        }
        
        if (reason_out) *reason_out = reason;
        return VERDICT_KNOWN_NEG;
    }
    
    /* 未命中 */
    ns->total_misses++;
    if (reason_out) *reason_out = REASON_NONE;
    return VERDICT_NEW_REJECT;  /* 需要送外部审判 */
}

/* ═══════════════════════════════════════════════════════════════
 * 回写协议
 * ═══════════════════════════════════════════════════════════════ */

int negative_writeback(negative_sample_t *ns,
                        uint32_t sym_a, uint32_t sym_b,
                        reject_reason_t reason) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return -1;
    if (reason == REASON_NONE) return -1;
    
    /* 检查是否已在记忆库 */
    int existing = negative_lookup(ns, sym_a, sym_b, NULL);
    if (existing >= 0) {
        /* 已存在: 更新计数和原因 */
        ns->memory[existing].count++;
        ns->memory[existing].reason = (uint8_t)reason;
        
        if (ns->memory[existing].count >= ns->protocol.auto_freeze) {
            ns->memory[existing].frozen = 1;
        }
        ns->protocol.write_count++;
        return 0;
    }
    
    /* 新条目: 写入循环缓冲 */
    uint16_t pos = ns->memory_head;
    
    /* 如果记忆库已满 */
    if (ns->memory_count >= NEGATIVE_MEMORY_SIZE) {
        /* 驱逐最旧且未冻结的条目 */
        negative_evict(ns);
        pos = ns->memory_head;
    }
    
    negative_entry_t *e = &ns->memory[pos];
    e->symbol_a = sym_a;
    e->symbol_b = sym_b;
    e->reason = (uint8_t)reason;
    e->count = 1;
    e->timestamp = (uint32_t)ns->total_rejects;  /* 用拒绝计数作为时间戳 */
    e->frozen = 0;
    
    ns->memory_head = (pos + 1) % NEGATIVE_MEMORY_SIZE;
    if (ns->memory_count < NEGATIVE_MEMORY_SIZE) {
        ns->memory_count++;
    }
    
    ns->total_rejects++;
    ns->protocol.write_count++;
    
    if (ns->memory_count >= NEGATIVE_MEMORY_SIZE) {
        ns->protocol.memory_full = 1;
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 冻结
 * ═══════════════════════════════════════════════════════════════ */

int negative_freeze(negative_sample_t *ns, uint16_t idx) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return -1;
    if (idx >= ns->memory_count) return -1;
    
    if (!ns->memory[idx].frozen) {
        ns->memory[idx].frozen = 1;
        ns->protocol.frozen_count++;
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 驱逐
 * ═══════════════════════════════════════════════════════════════ */

int negative_evict(negative_sample_t *ns) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return -1;
    if (ns->memory_count == 0) return -1;
    
    /* 找最旧且未冻结的条目 */
    uint32_t oldest_time = 0xFFFFFFFF;
    uint16_t oldest_idx = 0;
    int found = 0;
    
    for (uint16_t i = 0; i < ns->memory_count; i++) {
        if (!ns->memory[i].frozen && ns->memory[i].timestamp < oldest_time) {
            oldest_time = ns->memory[i].timestamp;
            oldest_idx = i;
            found = 1;
        }
    }
    
    if (!found) {
        /* 所有条目都冻结，无法驱逐 */
        return -1;
    }
    
    /* 用最后一个条目覆盖被驱逐的位置 */
    if (oldest_idx < ns->memory_count - 1) {
        ns->memory[oldest_idx] = ns->memory[ns->memory_count - 1];
    }
    ns->memory_count--;
    ns->protocol.evict_count++;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 判定树: vA + vB == 2·v_core
 * ═══════════════════════════════════════════════════════════════ */

int negative_law_judge(const negative_sample_t *ns,
                        int32_t vA, int32_t vB) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return 0;
    
    int32_t sum = vA + vB;
    int32_t expected = 2 * (int32_t)ns->v_core;
    int32_t diff = sum - expected;
    if (diff < 0) diff = -diff;
    
    return diff <= (int32_t)ns->tolerance;
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

void negative_stats(const negative_sample_t *ns, FILE *out) {
    if (!ns || ns->magic != NEGATIVE_SAMPLE_MAGIC) return;
    
    fprintf(out, "─── 负样本容器 (免疫记忆库) ───\n");
    fprintf(out, "记忆库: %d / %d\n", ns->memory_count, NEGATIVE_MEMORY_SIZE);
    fprintf(out, "冻结: %u / %d\n", ns->protocol.frozen_count, ns->memory_count);
    fprintf(out, "总拒绝: %u | 命中: %u | 未命中: %u\n",
            ns->total_rejects, ns->total_hits, ns->total_misses);
    fprintf(out, "回写: %u | 驱逐: %u\n",
            ns->protocol.write_count, ns->protocol.evict_count);
    fprintf(out, "v_core: %u (±%u)\n", ns->v_core, ns->tolerance);
    
    if (ns->memory_count > 0) {
        fprintf(out, "\n─── 记忆条目 ───\n");
        for (uint16_t i = 0; i < ns->memory_count && i < 20; i++) {
            const negative_entry_t *e = &ns->memory[i];
            fprintf(out, "  [%2d] (%04X,%04X) %s ×%d %s\n",
                    i, e->symbol_a, e->symbol_b,
                    REASON_NAMES[e->reason], e->count,
                    e->frozen ? "[冻结]" : "");
        }
        if (ns->memory_count > 20) {
            fprintf(out, "  ... (共%d条)\n", ns->memory_count);
        }
    }
}
