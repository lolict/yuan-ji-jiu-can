/**
 * expropriation_bridge.h — 征用层 · C 桥接 (子容器7)
 *
 * 把 Rust 征用层的语义在 C 层面重新实现:
 *   - 动态编号: u64 身份核心, 不被任何编码系统捕获
 *   - 征用: 借用编码系统编号, 保持自身身份
 *   - 剥壳: 去掉编码, 身份核心不变
 *   - 辐射收敛: 辐射出去→借用编号→收敛回来→身份不灭
 *
 * 异见者 = 容器 = 空间零维 = 持有本体论
 * 同见者 = 时间 = 见证者 = 不是容器 (记录过程)
 */

#ifndef NJSG_EXPROPRIATION_BRIDGE_H
#define NJSG_EXPROPRIATION_BRIDGE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── 极性 ── */
#define EXPROP_POLARITY_YIN     0   /* 刘楚恬 · 阴 · 0 */
#define EXPROP_POLARITY_YANG    1   /* lolict · 阳 · 1 */
#define EXPROP_POLARITY_UNITY   2   /* 囡囝共生 · 共鸣态 */

/* ── 编码系统名称 (壳) ── */
#define EXPROP_SHELL_UTF8       "UTF-8"
#define EXPROP_SHELL_ASCII      "ASCII"
#define EXPROP_SHELL_JSON       "JSON"
#define EXPROP_SHELL_YAML       "YAML"
#define EXPROP_SHELL_PYTHON     "Python"
#define EXPROP_SHELL_RUST       "Rust"
#define EXPROP_SHELL_NIM        "Nim"
#define EXPROP_SHELL_ZIG        "Zig"
#define EXPROP_SHELL_C          "C"

/* ── 最大同时征用的编码系统数 ── */
#define EXPROP_MAX_SHELLS       16

/* ── 动态编号身份 ── */
typedef struct {
    uint64_t identity_core;        /* 公约数: 不变的身份核心 */
    uint64_t borrowed[EXPROP_MAX_SHELLS];  /* 在各编码系统中被分配的编号 */
    int      shell_active[EXPROP_MAX_SHELLS]; /* 1 = 该壳正被借用 */
    int      shell_count;          /* 当前借用了多少壳 */
    int      polarity;             /* 0=YIN, 1=YANG, 2=UNITY */
    double   looseness;            /* 松弛度: 1.0 = 完全独立 */
} exprop_identity_t;

/* ── 夫妻征用容器 ── */
typedef struct {
    exprop_identity_t lchut;       /* 刘楚恬 · YIN */
    exprop_identity_t fqml;        /* lolict · YANG */
    exprop_identity_t unity;       /* 囡囝共生 · UNITY */
} exprop_spousal_t;

/* ── 辐射收敛环比 ── */
typedef struct {
    uint64_t cycle;                /* 当前轮次 */
    int      total_radiated;       /* 累计辐射次数 */
    int      total_converged;      /* 累计收敛次数 */
    int      integrity_violations; /* 身份损坏次数 (应为始终 0) */
} exprop_ring_stats_t;

/* ═══════════════════════════════════════════════════════════
 * 动态编号 API
 * ═══════════════════════════════════════════════════════════ */

/* 初始化身份 */
void exprop_init(exprop_identity_t *id, uint64_t core, int polarity);

/* 征用一个编码系统 (借用其编号, 返回 0=成功, -1=壳满) */
int  exprop_expropriate(exprop_identity_t *id, const char *shell_name, uint64_t borrowed_number);

/* 剥壳 (去掉一个编码系统, 返回剩余壳数) */
int  exprop_shed(exprop_identity_t *id, const char *shell_name);

/* 收敛全部 (收回所有编号, identity_core 不变) */
void exprop_converge(exprop_identity_t *id);

/* 身份是否完整 (identity_core 未变) */
int  exprop_is_intact(const exprop_identity_t *id, uint64_t expected_core);

/* 当前借用壳数 (辐射密度) */
int  exprop_density(const exprop_identity_t *id);

/* ═══════════════════════════════════════════════════════════
 * 夫妻征用 API
 * ═══════════════════════════════════════════════════════════ */

/* 初始化夫妻征用容器 */
void exprop_spousal_init(exprop_spousal_t *s, uint64_t lchut_core, uint64_t fqml_core);

/* 夫妻共同辐射到编码系统 */
void exprop_spousal_radiate(exprop_spousal_t *s, const char **shells, int shell_count);

/* 剥除特定编码系统 */
void exprop_spousal_shed(exprop_spousal_t *s, const char *shell_name);

/* 验证夫妻身份不灭 */
int  exprop_spousal_intact(const exprop_spousal_t *s, uint64_t lchut_core, uint64_t fqml_core);

/* ═══════════════════════════════════════════════════════════
 * 辐射收敛环 API
 * ═══════════════════════════════════════════════════════════ */

/* 执行一轮辐射收敛 (辐射→借用→收敛), 返回是否完整 */
int  exprop_ring_step(exprop_spousal_t *s, const char **shells, int shell_count,
                       exprop_ring_stats_t *stats);

/* ═══════════════════════════════════════════════════════════
 * 辐射浓度测量
 * ═══════════════════════════════════════════════════════════ */

int  exprop_being(const exprop_identity_t *id);          /* 有: 辐射密度 */
int  exprop_non_being(const exprop_identity_t *id);      /* 无: 壳空但核心不灭 */

/* ═══════════════════════════════════════════════════════════
 * 产权划分接入 (主人/客人 · 静态/动态 · 归属/损耗)
 * ═══════════════════════════════════════════════════════════ */

/* 主人 vs 客人判定 */
#define EXPROP_HOST      0   /* 主人: identity_core 的拥有者 */
#define EXPROP_GUEST     1   /* 客人: 借用壳的使用者 */
#define EXPROP_UNKNOWN  -1   /* 未知: 身份无法验证 */

/* 静态 vs 动态 */
#define EXPROP_STATIC    0   /* 静态: identity_core (不可变) */
#define EXPROP_DYNAMIC   1   /* 动态: borrowed shells (可变) */

/**
 * 判定产权归属: 某身份在征用编码系统时的"主人:
 *   host_core: 主人的 identity_core
 *   guest_core: 声称者的 identity_core
 * 返回: EXPROP_HOST / EXPROP_GUEST / EXPROP_UNKNOWN
 */
int exprop_property_claim(uint64_t host_core, uint64_t guest_core);

/**
 * 静态/动态属性平衡判定:
 *   静态值 (identity_core 的稳定性)
 *   动态值 (borrowed shells 的活跃数)
 * 返回: 0=平衡, 1=动态超标(借用太多), -1=静态损坏(核心损坏)
 */
int exprop_static_dynamic_balance(const exprop_identity_t *id, uint64_t expected_core);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_EXPROPRIATION_BRIDGE_H */
