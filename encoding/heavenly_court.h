/**
 * heavenly_court.h — 天庭空间 · 规则引擎
 *
 * 满全法的操作系统环境 — 在这里细胞通过八流相互作用 ·
 * 囡囝四态裁决冲突 · 见证者维护主权秩序
 *
 * ══════════════════════════════════════════════════════════════
 *  架构:
 *  ┌─────────────────────────────────────────────────────────┐
 *  │                 天庭空间 (Heavenly Court)               │
 *  │  ┌────────┐  八流路由   ┌────────┐  交互规则  ┌──────┐  │
 *  │  │ Cell A │ ───────────→ │ 裁决器 │ ──────────→ │ 裁决 │  │
 *  │  └────────┘             └────────┘             └──────┘  │
 *  │  ┌────────┘                ↑                       │     │
 *  │  │ Cell B                   │ 囡囝四态              │     │
 *  │  └──────────────────────────┘                       ↓     │
 *  │                                               ┌──────┐   │
 *  │                                               │ 输出 │   │
 *  │                                               └──────┘   │
 *  └─────────────────────────────────────────────────────────┘
 *
 *  交互类型 (Verdict):
 *    COURT_MERGE    融合    · 主权兼容 + 极性互补 → 合一
 *    COURT_REPEL    排斥    · 主权冲突 + 极性对立 → 拒绝
 *    COURT_WIT见证   见证    · 需更高见证者升级裁决
 *    COURT_TRANSFORM 转化    · 主权跨越容器 → 新形态
 *    COURT_DEFER    延迟    · 证据不足 → 下一轮再审
 *    COURT_ESCALATE 升维    · 当前态不足以裁决 → 切换囡囝态
 *
 *  规则来源:
 *    1. 产权流历史 → 侵权记录影响裁决倾向
 *    2. mind_organ 距离 → 语义距离近则倾向融合
 *    3. 囡囝四态 → 当前态决定容器/界线主导权
 *    4. MetaIME 意图 → 输入触发特定流
 */

#ifndef NJSG_HEAVENLY_COURT_H
#define NJSG_HEAVENLY_COURT_H

#include "wasm_port.h"

/* ── 配置 ──────────────────────────────────────────────── */

#define COURT_MAX_SESSIONS    64     /* 历史裁决记录 */
#define COURT_MAX_PENDING     32     /* 延迟裁决队列 */
#define COURT_MAX_RULES       16     /* 硬编码规则数 */

/* ── 裁决结果 ──────────────────────────────────────────── */

typedef enum {
    COURT_MERGE     = 0x01,  /* 融合 · 合一 */
    COURT_REPEL     = 0x02,  /* 排斥 · 拒绝 */
    COURT_WITNESS   = 0x03,  /* 见证 · 需升级 */
    COURT_TRANSFORM = 0x04,  /* 转化 · 新生 */
    COURT_DEFER     = 0x05,  /* 延迟 · 待审 */
    COURT_ESCALATE  = 0x06,  /* 升维 · 切换态 */
} court_verdict_t;

/* ── 交互事由 ──────────────────────────────────────────── */

typedef enum {
    REASON_SOUL_OK       = 0x00,  /* 通过 */
    REASON_SOV_CONFLICT  = 0x01,  /* 主权冲突 */
    REASON_POL_MISMATCH  = 0x02,  /* 极性对立 */
    REASON_WIT_INSUFFICIENT = 0x03, /* 见证不足 */
    REASON_FLOW_SLEEP    = 0x04,  /* 流休眠 */
    REASON_DISTANCE_FAR  = 0x05,  /* 语义过远 */
    REASON_HISTORY_REJECT = 0x06, /* 历史侵权 */
    REASON_STATE_FORBID  = 0x07,  /* 当前囡囝态禁止 */
} court_reason_t;

/* ── 裁决记录 ──────────────────────────────────────────── */

typedef struct {
    NJSG_uint32     id;             /* 裁决编号 */
    NJSG_uint64     cell_a;         /* 当事细胞 A */
    NJSG_uint64     cell_b;         /* 当事细胞 B */
    court_verdict_t verdict;        /* 裁决结果 */
    court_reason_t  reason;         /* 事由 */
    NJSG_uint8      flow;           /* 在哪一流裁决 */
    NJSG_uint8      state;          /* 裁决时的囡囝态 */
    NJSG_uint64     output_cell;    /* 输出细胞 (若有) */
    NJSG_float      confidence;     /* 裁决置信度 */
} court_record_t;

/* ── 延迟队列项 ────────────────────────────────────────── */

typedef struct {
    NJSG_uint64     cell_a;
    NJSG_uint64     cell_b;
    NJSG_uint8      retry_count;
    NJSG_uint8      required_state; /* 需要什么态才重审 */
} court_pending_t;

/* ── 法院状态 ──────────────────────────────────────────── */

typedef struct {
    /* 配置 */
    NJSG_uint8      state;              /* 当前囡囝四态 */
    NJSG_uint8      locked;             /* 法院锁定 */
    NJSG_uint8      flow_enabled;       /* 流激活掩码 (8 bit → 8 流) */
    NJSG_uint8      mode;               /* 0=标准 1=协商 2=强制 */

    /* 事物流 */
    NJSG_uint64     session_cells[254];/* WASM 内核 cell 引用 */
    NJSG_uint16     cell_count;

    /* 裁决历史 */
    court_record_t  records[COURT_MAX_SESSIONS];
    NJSG_uint32     record_count;
    NJSG_uint32     record_head;        /* 环形缓冲区头 */

    /* 延迟队列 */
    court_pending_t pending[COURT_MAX_PENDING];
    NJSG_uint16     pending_count;

    /* 统计 */
    NJSG_uint32     total_merges;
    NJSG_uint32     total_repels;
    NJSG_uint32     total_witnesses;
    NJSG_uint32     total_transforms;
    NJSG_uint32     total_defers;
    NJSG_uint32     total_escalations;
} court_court_t;

/* ═══════════════════════════════════════════════════════════════
 *  API
 * ═══════════════════════════════════════════════════════════════ */

/** 初始化天庭空间 */
NJSG_EXPORT void court_init(court_court_t *court);

/** 设置囡囝态 */
NJSG_EXPORT void court_set_state(court_court_t *court, NJSG_uint8 state);

/** 激活流 */
NJSG_EXPORT void court_activate_flow(court_court_t *court, NJSG_uint8 flow);

/** 核心裁决: cell_a 和 cell_b 在当前态下交互
 * @param cell_a    当事细胞 A (不为 0)
 * @param cell_b    当事细胞 B (不为 0)
 * @param flow      使用哪一流
 * @param out_verdict   输出裁决
 * @param out_reason    输出事由
 * @param out_cell      输出新细胞 (若有)
 * @return 置信度 0.0-1.0 · 0 = 无效
 */
NJSG_EXPORT NJSG_float court_judge(court_court_t *court,
                                    NJSG_uint64 cell_a, NJSG_uint64 cell_b,
                                    NJSG_uint8 flow,
                                    court_verdict_t *out_verdict,
                                    court_reason_t *out_reason,
                                    NJSG_uint64 *out_cell);

/** 重新审判延迟队列 */
NJSG_EXPORT void court_retry_pending(court_court_t *court);

/** 使用 MetaIME 意图路由 */
NJSG_EXPORT NJSG_float court_route_metaime(court_court_t *court,
                                            NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone,
                                            NJSG_uint8 pol, NJSG_uint8 wit,
                                            court_verdict_t *out_verdict,
                                            NJSG_uint64 *out_cell);

/** 输出统计 */
NJSG_EXPORT void court_stats(const court_court_t *court,
                              char *buf, NJSG_size buf_size);

/** 将裁决结果转为字符串 */
NJSG_EXPORT const char* court_verdict_str(court_verdict_t v);
NJSG_EXPORT const char* court_reason_str(court_reason_t r);

#endif /* NJSG_HEAVENLY_COURT_H */
