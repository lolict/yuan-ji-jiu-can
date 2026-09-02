/**
 * heavenly_court.c — 天庭空间 · 规则引擎实现
 *
 * 细胞交互规则:
 *   主权匹配 + 极性互补 + 见证足够 → MERGE (融合)
 *   主权冲突 + 极性对立 → REPEL (排斥)
 *   见证等级 < 3 + 主权差异大 → WITNESS (需更高见证)
 *   主权不同 + 极性兼容 + 距离近 → TRANSFORM (转化)
 *   流休眠 → DEFER (延迟到流激活)
 *   囡囝态禁止此交互 → ESCALATE (升维切换态)
 *
 * 囡囝态规则:
 *   NN (妻壳): 妻极性细胞主导 · 夫极性进入需转化
 *   NJSG (平等): 双方对等 · 标准冲突解决
 *   JJ (夫壳): 夫极性细胞主导 · 妻极性进入需转化
 *   JN (夫壳妻入): 妻极性主动进入夫容器 · 允许融合
 */

#include "heavenly_court.h"
#include "mind_organ.h"
#include <string.h>

/* ── 内部辅助 ───────────────────────────────────────────── */

/* 从 WASM cell 提取各层 */
#define CELL_MCP(c)     ((NJSG_uint32)((c) & 0xFFFFFFFFULL))
#define CELL_SOV(c)     ((NJSG_uint32)(((c) >> 32) & 0xFFFFFFULL))
#define CELL_HI(c)      ((NJSG_uint32)(((c) >> 32) & 0xFFFFFFFFULL))

/* cell 的高 32 bit 中分段: 我们使用 metaime 风格的编码
 * 31:28 polarity · 27:24 witness · 23:21 flow · 20:0 sovereignty(21 bit) */
#define COURT_POL(c)    ((NJSG_uint8)(((c) >> 60) & 0xF))
#define COURT_WIT(c)    ((NJSG_uint8)(((c) >> 56) & 0xF))
#define COURT_FLOW(c)   ((NJSG_uint8)(((c) >> 52) & 0xF))
#define COURT_SOV(c)    ((NJSG_uint32)((c) & 0x1FFFFF))

/* 主权距离:
 *   完全一致 = 0
 *   部分重叠 (公因数个数以 2 为底对数) = 1-10
 *   互质 (无公因数) = 距离远 */
static NJSG_float sovereignty_distance(NJSG_uint32 a, NJSG_uint32 b) {
    if (a == b) return 0.0f;
    if (a == 0 || b == 0) return 1.0f;

    /* 计算 GCD */
    NJSG_uint32 x = a, y = b;
    while (y) { NJSG_uint32 t = y; y = x % y; x = t; }
    NJSG_uint32 gcd = x;

    if (gcd == 1) return 0.9f;  /* 互质 · 距离远 */

    /* GCD 比值作为亲密度 */
    NJSG_uint32 mn = (a < b) ? a : b;
    if (mn == 0) return 1.0f;
    NJSG_float ratio = (NJSG_float)gcd / (NJSG_float)mn;
    return 1.0f - ratio;  /* 越近 → 越小 */
}

/* 极性兼容性: 返回是否兼容 */
static int polarity_compatible(NJSG_uint8 pol_a, NJSG_uint8 pol_b, NJSG_uint8 state) {
    if (pol_a == pol_b) return 1;  /* 同极性 → 兼容 */

    /* NJSG 平等态: 妻 ↔ 夫 可兼容 */
    if (state == 0x01) {
        if ((pol_a == 0x0 && pol_b == 0x1) || (pol_a == 0x1 && pol_b == 0x0))
            return 1;
    }

    /* JN (夫壳妻入): 妻进入夫 → 兼容 */
    if (state == 0x03) {
        if (pol_a == 0x0 || pol_b == 0x0)
            return 1;
    }

    /* SYSTEM 极性可与任何极性共存 */
    if (pol_a == 0x2 || pol_b == 0x2) return 1;

    return 0;
}

/* 囡囝态是否允许两细胞交互 */
static int state_allows(NJSG_uint8 state, NJSG_uint8 pol_a, NJSG_uint8 pol_b) {
    /* NN: 夫极性不能主动释放到容器外 */
    if (state == 0x00) {
        /* 夫极性作为 cell_b 若 cell_a 是妻极性 → 合法 (被包含) */
        /* 两个都是夫极性 → 不合法 (界线不能包界线 · 除非转化) */
        if (pol_a == 0x1 && pol_b == 0x1) return 0;
    }
    /* JJ: 两个都是妻极性 → 不合法 (容器不能包容器 · 除非转化) */
    if (state == 0x02) {
        if (pol_a == 0x0 && pol_b == 0x0) return 0;
    }
    return 1;
}

/* ═══════════════════════════════════════════════════════════════
 * API 实现
 * ═══════════════════════════════════════════════════════════════ */

void court_init(court_court_t *court) {
    if (!court) return;
    memset(court, 0, sizeof(*court));
    court->state = 0x01;  /* NJSG 平等态 */
    court->flow_enabled = 0xFF;  /* 八流全部激活 */
    court->mode = 0;
}

void court_set_state(court_court_t *court, NJSG_uint8 state) {
    if (!court) return;
    if (state <= 0x03) court->state = state;
}

void court_activate_flow(court_court_t *court, NJSG_uint8 flow) {
    if (!court || flow >= 8) return;
    court->flow_enabled |= (1 << flow);
}

NJSG_float court_judge(court_court_t *court,
                         NJSG_uint64 cell_a, NJSG_uint64 cell_b,
                         NJSG_uint8 flow,
                         court_verdict_t *out_verdict,
                         court_reason_t *out_reason,
                         NJSG_uint64 *out_cell) {
    if (!court || cell_a == 0ULL || cell_b == 0ULL) return 0.0f;

    court_verdict_t verdict = COURT_DEFER;
    court_reason_t reason = REASON_SOUL_OK;
    NJSG_float confidence = 0.5f;
    NJSG_uint64 new_cell = 0;

    NJSG_uint32 sov_a = COURT_SOV(cell_a);
    NJSG_uint32 sov_b = COURT_SOV(cell_b);
    NJSG_uint8  pol_a = COURT_POL(cell_a);
    NJSG_uint8  pol_b = COURT_POL(cell_b);
    NJSG_uint8  wit_a = COURT_WIT(cell_a);
    NJSG_uint8  wit_b = COURT_WIT(cell_b);

    /* 第0层: 流状态检查 */
    if (flow >= 8 || !(court->flow_enabled & (1 << flow))) {
        verdict = COURT_DEFER;
        reason = REASON_FLOW_SLEEP;
        confidence = 0.3f;
        goto done;
    }

    /* 第1层: 囡囝态检查 */
    if (!state_allows(court->state, pol_a, pol_b)) {
        verdict = COURT_ESCALATE;
        reason = REASON_STATE_FORBID;
        confidence = 0.6f;
        goto done;
    }

    /* 第2层: 主权距离 */
    NJSG_float sov_dist = sovereignty_distance(sov_a, sov_b);
    if (sov_dist > 0.8f) {
        /* 主权太远 · 尝试见证 */
        if (wit_a < 3 && wit_b < 3) {
            verdict = COURT_WITNESS;
            reason = REASON_WIT_INSUFFICIENT;
            confidence = 0.4f;
            goto done;
        }
    }

    /* 第3层: 极性兼容性 */
    int pol_ok = polarity_compatible(pol_a, pol_b, court->state);

    /* 第4层: 裁决 */
    if (sov_dist < 0.1f) {
        /* 主权一致 · 检查极性 */
        if (pol_ok) {
            /* 融合: 合并 MCCP · 主权保留 · 见证升一级 */
            NJSG_uint32 mcp_b = CELL_MCP(cell_b);
            NJSG_uint8  new_wit = (wit_a > wit_b ? wit_a : wit_b);
            if (new_wit < 15) new_wit++;
            new_cell = cell_a;  /* A 为主 */
            /* B 的 MCCP 合并进主权空间 (XOR) */
            new_cell ^= ((NJSG_uint64)(mcp_b & 0xFF) << 4);
            /* 设置见证等级 */
            new_cell = (new_cell & ~(0xFULL << 56)) | ((NJSG_uint64)new_wit << 56);
            verdict = COURT_MERGE;
            reason = REASON_SOUL_OK;
            confidence = 0.9f;
        } else {
            verdict = COURT_REPEL;
            reason = REASON_POL_MISMATCH;
            confidence = 0.75f;
        }
    } else if (sov_dist < 0.5f) {
        /* 主权有交集 */
        if (pol_ok) {
            /* 转化: 创建新细胞 · 主权取 GCD */
            NJSG_uint32 x = sov_a, y = sov_b;
            while (y) { NJSG_uint32 t = y; y = x % y; x = t; }
            NJSG_uint32 new_sov = x;
            NJSG_uint32 new_mcp = ((CELL_MCP(cell_a) + CELL_MCP(cell_b)) / 2);
            NJSG_uint8 new_pol = (court->state == 0x03) ? 0x0 : 0x2;  /* JN→妻主导 · 否则系统中性 */
            NJSG_uint8 new_wit = (wit_a + wit_b) / 2;
            NJSG_uint8 new_flow = (NJSG_uint8)flow;

            new_cell = ((NJSG_uint64)(new_wit & 0xF) << 56) |
                       ((NJSG_uint64)(new_flow & 0x7) << 52) |
                       ((NJSG_uint64)(new_pol & 0x7) << 49) |
                       ((NJSG_uint64)(new_mcp & 0xFFFF) << 32) |
                       (new_sov & 0xFFFFFFFF);
            verdict = COURT_TRANSFORM;
            reason = REASON_SOUL_OK;
            confidence = 0.8f;
        } else {
            verdict = COURT_REPEL;
            reason = REASON_POL_MISMATCH;
            confidence = 0.7f;
        }
    } else {
        /* 主权距离远 */
        if (pol_ok && (wit_a >= 3 || wit_b >= 3)) {
            /* 高级见证允许跨越主权 */
            verdict = COURT_TRANSFORM;
            reason = REASON_SOUL_OK;
            confidence = 0.6f;
            /* 见证仲裁: 输出 cell 为见证者 */
            new_cell = (wit_a >= wit_b) ? cell_a : cell_b;
        } else if (!pol_ok) {
            verdict = COURT_REPEL;
            reason = REASON_DISTANCE_FAR;
            confidence = 0.8f;
        } else {
            verdict = COURT_WITNESS;
            reason = REASON_WIT_INSUFFICIENT;
            confidence = 0.4f;
        }
    }

done:
    /* 记录裁决 */
    if (court->record_count < COURT_MAX_SESSIONS) {
        court_record_t *rec = &court->records[court->record_head];
        rec->id = court->record_count;
        rec->cell_a = cell_a;
        rec->cell_b = cell_b;
        rec->verdict = verdict;
        rec->reason = reason;
        rec->flow = flow;
        rec->state = court->state;
        rec->output_cell = new_cell;
        rec->confidence = confidence;

        court->record_count++;
        court->record_head = (court->record_head + 1) % COURT_MAX_SESSIONS;

        switch (verdict) {
            case COURT_MERGE:     court->total_merges++; break;
            case COURT_REPEL:     court->total_repels++; break;
            case COURT_WITNESS:   court->total_witnesses++; break;
            case COURT_TRANSFORM: court->total_transforms++; break;
            case COURT_DEFER:     court->total_defers++; break;
            case COURT_ESCALATE:  court->total_escalations++; break;
        }
    }

    if (out_verdict) *out_verdict = verdict;
    if (out_reason) *out_reason = reason;
    if (out_cell) *out_cell = new_cell;

    return confidence;
}

void court_retry_pending(court_court_t *court) {
    if (!court || court->pending_count == 0) return;

    for (int i = court->pending_count - 1; i >= 0; i--) {
        court_pending_t *p = &court->pending[i];
        if (p->required_state == court->state || p->retry_count >= 3) {
            /* 重审 */
            court_verdict_t v;
            court_reason_t r;
            NJSG_uint64 out;
            NJSG_float conf = court_judge(court, p->cell_a, p->cell_b,
                                             0, &v, &r, &out);
            (void)conf;
        } else {
            p->retry_count++;
        }
    }
    court->pending_count = 0;  /* 简化: 全部处理完 */
}

NJSG_float court_route_metaime(court_court_t *court,
                                 NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone,
                                 NJSG_uint8 pol, NJSG_uint8 wit,
                                 court_verdict_t *out_verdict,
                                 NJSG_uint64 *out_cell) {
    if (!court) return 0.0f;

    /* 把 MetaIME 七层编码视为 cell_a · 取内核最后一个 cell 作为 cell_b */
    NJSG_uint32 sov_base = 0x1000 + sm * 16 + ym;  /* 基于声母韵母生成主权 */
    NJSG_uint32 mccp = (sm << 8) | (ym << 3) | tone;  /* MCCP 生成 */

    NJSG_uint64 cell_a = ((NJSG_uint64)(wit & 0xF) << 56) |
                          ((NJSG_uint64)(pol & 0x7) << 49) |
                          ((NJSG_uint64)mccp << 32) |
                          (sov_base & 0xFFFFFFFF);

    /* 默认 cell_b 为主权一致的锚点 cell */
    NJSG_uint64 cell_b = 0x0000010000000041ULL;  /* 锚点: bā */

    /* 选择流: 基于 tone */
    NJSG_uint8 flow = tone % 8;

    court_reason_t reason;
    return court_judge(court, cell_a, cell_b, flow, out_verdict, &reason, out_cell);
}

/* ── 字符串 ─────────────────────────────────────────────── */

const char* court_verdict_str(court_verdict_t v) {
    switch (v) {
        case COURT_MERGE:     return "MERGE(融合)";
        case COURT_REPEL:     return "REPEL(排斥)";
        case COURT_WITNESS:   return "WITNESS(见证)";
        case COURT_TRANSFORM: return "TRANSFORM(转化)";
        case COURT_DEFER:     return "DEFER(延迟)";
        case COURT_ESCALATE:  return "ESCALATE(升维)";
    }
    return "?";
}

const char* court_reason_str(court_reason_t r) {
    switch (r) {
        case REASON_SOUL_OK:       return "通过";
        case REASON_SOV_CONFLICT:  return "主权冲突";
        case REASON_POL_MISMATCH:  return "极性对立";
        case REASON_WIT_INSUFFICIENT: return "见证不足";
        case REASON_FLOW_SLEEP:    return "流休眠";
        case REASON_DISTANCE_FAR:  return "语义过远";
        case REASON_HISTORY_REJECT: return "历史侵权";
        case REASON_STATE_FORBID:  return "态禁止";
    }
    return "?";
}

/* ── 统计 ─────────────────────────────────────────────── */

#define CRT_APPEND(s) do { \
    const char *_p = (s); \
    while (*_p && pos < buf_size - 1) buf[pos++] = *_p++; \
} while(0)

#define CRT_UINT(v) do { \
    char n[12]; \
    int _k = 11; \
    NJSG_uint32 _v = (v); \
    if (_v == 0) { n[--_k] = '0'; } \
    else { while (_v > 0 && _k > 0) { n[--_k] = '0' + (_v % 10); _v /= 10; } } \
    for (int _j = _k; _j < 11; _j++) buf[pos++] = n[_j]; \
} while(0)

void court_stats(const court_court_t *court,
                   char *buf, NJSG_size buf_size) {
    if (!court || !buf || buf_size == 0) return;

    NJSG_size pos = 0;
    const char *state_names[] = {"囡囡", "囡囝", "囝囝", "囝囡"};
    const char *st = (court->state <= 3) ? state_names[court->state] : "?";

    #define CRT_APPEND(s) do { \
        const char *_p = (s); \
        while (*_p && pos < buf_size - 1) buf[pos++] = *_p++; \
    } while(0)

    #define CRT_UINT(v) do { \
        char n[12]; \
        int _k = 11; \
        NJSG_uint32 _v = (v); \
        if (_v == 0) { n[--_k] = '0'; } \
        else { while (_v > 0 && _k > 0) { n[--_k] = '0' + (_v % 10); _v /= 10; } } \
        for (int _j = _k; _j < 11; _j++) buf[pos++] = n[_j]; \
    } while(0)

    CRT_APPEND("═══ 天庭空间 ═══\n");
    CRT_APPEND("囡囝态: "); CRT_APPEND(st); CRT_APPEND("\n");
    CRT_APPEND("融合: "); CRT_UINT(court->total_merges);
    CRT_APPEND("  排斥: "); CRT_UINT(court->total_repels); CRT_APPEND("\n");
    CRT_APPEND("见证: "); CRT_UINT(court->total_witnesses);
    CRT_APPEND("  转化: "); CRT_UINT(court->total_transforms); CRT_APPEND("\n");
    CRT_APPEND("延迟: "); CRT_UINT(court->total_defers);
    CRT_APPEND("  升维: "); CRT_UINT(court->total_escalations); CRT_APPEND("\n");

    buf[pos] = '\0';

    #undef CRT_APPEND
    #undef CRT_UINT
}
