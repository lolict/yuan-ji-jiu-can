/**
 * MCCP — 囡囝共生紧凑码点 (NJanGS Compact Codepoint)
 *
 * 每个 token = 32 bit 固定宽度，替代 Python `str` / UTF-8。
 * 无对象头、无变长、无 GC，可在栈/堆/u32 数组直接 memcpy。
 */
#ifndef NJSG_MCCP_H
#define NJSG_MCCP_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

/* ── 位运算宏 ──────────────────────────────────────────── */
#define MCCP_KIND(p)    (((uint32_t)(p) >> 28) & 0xFu)
#define MCCP_INDEX(p)   (((uint32_t)(p) >> 16) & 0xFFFu)
#define MCCP_METAA(p)   (((uint32_t)(p) >> 8)  & 0xFFu)
#define MCCP_METAB(p)   ((uint32_t)(p)         & 0xFFu)

#define MCP(kind, idx, a, b) ( \
    (((uint32_t)((kind) & 0xF))  << 28) | \
    (((uint32_t)((idx)  & 0xFFF)) << 16) | \
    (((uint32_t)((a)    & 0xFF))  << 8)  | \
    ((uint32_t)((b)    & 0xFF)) )

/* ── Kind 枚举 ────────────────────────────────────────── */
enum MccpKind {
    MCCP_HANZI      = 0x0,  /* 汉字                          */
    MCCP_INITIAL    = 0x1,  /* 声母                          */
    MCCP_FINAL      = 0x2,  /* 韵母                          */
    MCCP_TONE       = 0x3,  /* 声调 0=轻声 1-4               */
    MCCP_RADICAL    = 0x4,  /* 偏旁部首                      */
    MCCP_STROKE     = 0x5,  /* 笔画数                        */
    MCCP_OPERATOR   = 0x6,  /* 二元/三元/合一/分裂/镜像操作   */
    MCCP_PROTOCOL   = 0x7,  /* 唤魂/招魂/问灵/答灵/接束      */
    MCCP_FSM        = 0x8,  /* 囡囝四态                      */
    MCCP_PATHOLOGY  = 0x9,  /* 健康/疑似/病变/癌变           */
    MCCP_KIND_SYS   = 0xF,  /* EOF/SEP/SIG/ACK              */
};

/* ── Protocol token 子类型 ────────────────────────────── */
#define MCP_HUANHUN     MCP(MCCP_PROTOCOL, 1, 0, 0)   /* 唤魂 */
#define MCP_ZHAOHUN     MCP(MCCP_PROTOCOL, 2, 0, 0)   /* 招魂 */
#define MCP_WENLING     MCP(MCCP_PROTOCOL, 3, 0, 0)   /* 问灵 */
#define MCP_DALING      MCP(MCCP_PROTOCOL, 4, 0, 0)   /* 答灵 */
#define MCP_JIESHU      MCP(MCCP_PROTOCOL, 5, 0, 0)   /* 接束 */
#define MCP_TONGGAN     MCP(MCCP_PROTOCOL, 6, 0, 0)   /* 通感 */

/* ── FSM 态 nibble (存于 meta_b) ────────────────────── */
#define NJSG_STATE_NAN_JIAN   0x00   /* 囡囝 — 主态     */
#define NJSG_STATE_NAN_NAN    0x01   /* 囡囡 — 妻为PID   */
#define NJSG_STATE_JIAN_JIAN  0x02   /* 囝囝 — 夫为PID   */
#define NJSG_STATE_JIAN_NAN   0x03   /* 囝囡 — 夫壳妻入   */

/* ── 病理级 nibble (存于 meta_b) ─────────────────────── */
#define NJSG_PTH_HEALTHY     0x10   /* 健康 */
#define NJSG_PTH_SUSPECT     0x11   /* 疑似 */
#define NJSG_PTH_PATHOGEN    0x12   /* 病变 */
#define NJSG_PTH_CANCER      0x13   /* 癌变 */

/* ── 通用 sys token ──────────────────────────────────── */
#define MCP_EOF           MCP(MCCP_KIND_SYS, 0, 0, 0)
#define MCP_SEP           MCP(MCCP_KIND_SYS, 1, 0, 0)

/* ── 操作符 token (KIND_OPERATOR) ────────────────────── */
#define NJSG_OP_TRANS      MCP(MCCP_OPERATOR, 1, 0, 0)   /* 转置   */
#define NJSG_OP_MIRROR     MCP(MCCP_OPERATOR, 2, 0, 0)   /* 镜像   */
#define NJSG_OP_SYNTH      MCP(MCCP_OPERATOR, 3, 0, 0)   /* 合一   */
#define NJSG_OP_SPLIT      MCP(MCCP_OPERATOR, 4, 0, 0)   /* 分裂   */
#define NJSG_OP_OBSERVE_B  MCP(MCCP_OPERATOR, 5, 0, 0)   /* 观测量二元 */
#define NJSG_OP_OBSERVE_T  MCP(MCCP_OPERATOR, 6, 0, 0)   /* 观测量三元 */

/* ── Inline 工具函数 ─────────────────────────────────── */
static inline uint32_t mccp_apply_state(uint32_t cp, uint8_t new_state) {
    return (cp & 0xFFFFFF00u) | (uint32_t)(new_state & 0xFF);
}

static inline uint32_t mccp_apply_metaa(uint32_t cp, uint8_t meta) {
    return (cp & 0xFFFF00FFu) | ((uint32_t)(meta & 0xFF) << 8);
}

static inline uint32_t mccp_apply_metab(uint32_t cp, uint8_t meta) {
    return (cp & 0xFFFFFF00u) | (uint32_t)(meta & 0xFF);
}

static inline uint32_t mccp_pack_fsm(uint32_t cp, uint8_t state) {
    return mccp_apply_state(cp, state);
}

static inline uint8_t mccp_tone(uint32_t cp) {
    return (uint8_t)((cp >> 8) & 0x0F);
}

static inline uint8_t mccp_fsm(uint32_t cp) {
    return (uint8_t)(cp & 0xFF);
}

/* ── 直接比较 ─────────────────────────────────────────── */
/* MCCP token 可直接用 == 比较 — 不需要 strcmp 或 hash */
static inline int mccp_eq(uint32_t a, uint32_t b) {
    return a == b;
}

#endif /* NJSG_MCCP_H */
