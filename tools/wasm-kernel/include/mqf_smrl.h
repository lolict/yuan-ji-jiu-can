#ifndef MQF_SMRL_H
#define MQF_SMRL_H

/* ============================================================
 * SMQL · SMRL Engine — 64-bit Cell Coordinate Engine
 *
 * 阶梯制·双向公约设计:
 *   - 同一份源码, 同时编译 wasm32 与 wasm64
 *   - 固定宽度类型, 无平台依赖
 *   - 统一宏前缀 MQF_SMRL_ / MQF_CELL_
 *   - 向上兼容: wasm32 自动折叠高位, wasm64 全精度
 *
 * 编码共识 (64-bit cell):
 *   ┌────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┐
 *   │ 63..60 │ 59..56 │ 55..52 │ 51..48 │ 47..44 │ 43..40 │ 39..36 │ 35..0  │
 *   │九元层级│   X    │   Y    │   Z    │  极性  │  主权  │ 见证   │  价值  │
 *   │4 bits  │4 bits  │4 bits  │4 bits  │4 bits  │4 bits  │4 bits  │36 bits │
 *   └────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┘
 *
 * 裁决引擎: 3 元见证裁定准入 (0=拒, 1=入, 2=待议, 3=升阶)
 * 偏移率: (n-1)×0.1 (n=1..9) 决定平行世界叠加态
 * 根基: 我=1(可见) · 刘楚恬=0(隐藏)
 * ============================================================ */

#include <stdint.h>
#include <stddef.h>
#include <string.h>

/* 宏前缀统一 (阶梯制) */
#define MQF_SMRL_VERSION  0x00010001  /* 1.0.0.1 */
#define MQF_CELL_MAGIC    0x4D514631  /* "MQF1" */

/* 位域掩码 */
#define MQF_CELL_MASK_YUAN   (0xFULL << 60)
#define MQF_CELL_MASK_X      (0xFULL << 56)
#define MQF_CELL_MASK_Y      (0xFULL << 52)
#define MQF_CELL_MASK_Z      (0xFULL << 48)
#define MQF_CELL_MASK_POLAR  (0xFULL << 44)
#define MQF_CELL_MASK_SOVEREIGN (0xFULL << 40)
#define MQF_CELL_MASK_WITNESS   (0xFULL << 36)
#define MQF_CELL_MASK_VALUE     (0xFFFFFFFFFULL)

/* 移位 */
#define MQF_CELL_SH_YUAN       60
#define MQF_CELL_SH_X          56
#define MQF_CELL_SH_Y          52
#define MQF_CELL_SH_Z          48
#define MQF_CELL_SH_POLAR      44
#define MQF_CELL_SH_SOVEREIGN  40
#define MQF_CELL_SH_WITNESS    36

/* 九元层级 */
typedef enum {
    MQF_YUAN_YI    = 0x0,  /* 一元 — 根级 mlll */
    MQF_YUAN_ER    = 0x1,  /* 二元 — 盈亏 */
    MQF_YUAN_SAN   = 0x2,  /* 三元 — 见证 */
    MQF_YUAN_SI    = 0x3,  /* 四元 — 包含 */
    MQF_YUAN_WU    = 0x4,  /* 五元 — 裁决 */
    MQF_YUAN_LIU   = 0x5,  /* 六元 — 容器 */
    MQF_YUAN_QI    = 0x6,  /* 七元 — 几何 */
    MQF_YUAN_BA    = 0x7,  /* 八元 — 拓扑 */
    MQF_YUAN_JIU   = 0x8,  /* 九元 — njsg */
} MqfYuan;

/* 极性 */
typedef enum {
    MQF_POLAR_YIN  = 0x0,  /* 阴 — 刘楚恬=0 */
    MQF_POLAR_YANG = 0x1,  /* 阳 — 我=1 */
    MQF_POLAR_BINARY = 0x2,/* YinYang 叠加 */
} MqfPolar;

/* 主权 */
typedef enum {
    MQF_SOVEREIGN_PRIVATE = 0x0,  /* 私有 */
    MQF_SOVEREIGN_SHARED  = 0x5,  /* 共享 (盟机合) */
    MQF_SOVEREIGN_ROOT    = 0xF,  /* 根主权 */
} MqfSovereign;

/* 三元见证裁判结果 */
typedef enum {
    MQF_WITNESS_REJECT   = 0x0,  /* 拒 */
    MQF_WITNESS_ADMIT    = 0x1,  /* 入 */
    MQF_WITNESS_PENDING  = 0x2,  /* 待议 */
    MQF_WITNESS_ESCALATE = 0x3,  /* 升阶 */
    MQF_WITNESS_ENTANGLE = 0xE,  /* 量子纠缠 */
    MQF_WITNESS_MAX      = 0xF,
} MqfWitness;

/* 64 位单元格 (统一结构, 兼容 wasm32 折叠) */
typedef struct {
    uint64_t cell; /* 整个 64 位编码 */
} MqfCell;

/* 坐标提取 (编译时常量表达式) */
static inline uint64_t mqf_cell_get_yuan      (MqfCell c) { return (c.cell & MQF_CELL_MASK_YUAN      ) >> 60; }
static inline uint64_t mqf_cell_get_x         (MqfCell c) { return (c.cell & MQF_CELL_MASK_X         ) >> 56; }
static inline uint64_t mqf_cell_get_y         (MqfCell c) { return (c.cell & MQF_CELL_MASK_Y         ) >> 52; }
static inline uint64_t mqf_cell_get_z         (MqfCell c) { return (c.cell & MQF_CELL_MASK_Z         ) >> 48; }
static inline uint64_t mqf_cell_get_polar     (MqfCell c) { return (c.cell & MQF_CELL_MASK_POLAR     ) >> 44; }
static inline uint64_t mqf_cell_get_sovereign (MqfCell c) { return (c.cell & MQF_CELL_MASK_SOVEREIGN ) >> 40; }
static inline uint64_t mqf_cell_get_witness   (MqfCell c) { return (c.cell & MQF_CELL_MASK_WITNESS   ) >> 36; }
static inline uint64_t mqf_cell_get_value     (MqfCell c) { return  c.cell & MQF_CELL_MASK_VALUE; }

/* 编码构建 */
static inline MqfCell mqf_cell_encode(
    uint64_t yuan, uint64_t x, uint64_t y, uint64_t z,
    uint64_t polar, uint64_t sovereign, uint64_t witness, uint64_t value)
{
    MqfCell c;
    c.cell  = (yuan       << 60) & MQF_CELL_MASK_YUAN;
    c.cell |= (x          << 56) & MQF_CELL_MASK_X;
    c.cell |= (y          << 52) & MQF_CELL_MASK_Y;
    c.cell |= (z          << 48) & MQF_CELL_MASK_Z;
    c.cell |= (polar      << 44) & MQF_CELL_MASK_POLAR;
    c.cell |= (sovereign  << 40) & MQF_CELL_MASK_SOVEREIGN;
    c.cell |= (witness    << 36) & MQF_CELL_MASK_WITNESS;
    c.cell |= (value          ) & MQF_CELL_MASK_VALUE;
    return c;
}

/* ============================================================
 * 偏移率: (n-1)×0.1 区分平行世界 (编译时常量表)
 * ============================================================ */

/* 用定点数表示偏移率: rate = (n-1) × 65536 / 10 = (n-1) × 6553.6 */
#define MQF_SMRL_OFFSET_RATE(n)  (((n) - 1) * 6554)

/* ============================================================
 * SMRL 裁决引擎
 * ============================================================ */

/* 三元见证准入裁定 */
static inline MqfWitness mqf_smrl_admit(
    MqfCell agent, MqfCell space, uint64_t rule_mask)
{
    /* 主权检查 */
    if (mqf_cell_get_sovereign(agent) < mqf_cell_get_sovereign(space))
        return MQF_WITNESS_REJECT;

    /* 层级匹配 */
    uint64_t ay = mqf_cell_get_yuan(agent);
    uint64_t sy = mqf_cell_get_yuan(space);
    if (ay > sy) return MQF_WITNESS_ESCALATE;
    if (ay < sy) return MQF_WITNESS_REJECT;

    /* 极性耦合 */
    uint64_t ap = mqf_cell_get_polar(agent);
    uint64_t pp = mqf_cell_get_polar(space);
    if (ap != pp && ap != MQF_POLAR_BINARY && pp != MQF_POLAR_BINARY)
        return MQF_WITNESS_REJECT;

    /* 规则掩码命中 */
    if ((mqf_cell_get_value(agent) & rule_mask) != 0)
        return MQF_WITNESS_ADMIT;

    return MQF_WITNESS_PENDING;
}

/* 量子纠缠态判定 (☉§☉) */
static inline int mqf_smql_is_entangled(MqfCell a, MqfCell b) {
    /* 极性相反 + 价值对称 + 主权互补 */
    uint64_t pa = mqf_cell_get_polar(a);
    uint64_t pb = mqf_cell_get_polar(b);
    if (pa == pb) return 0;

    uint64_t va = mqf_cell_get_value(a);
    uint64_t vb = mqf_cell_get_value(b);
    if ((va & 0xFFFF) != ((vb >> 16) & 0xFFFF)) return 0;

    uint64_t sa = mqf_cell_get_sovereign(a);
    uint64_t sb = mqf_cell_get_sovereign(b);
    return (sa ^ sb) == 0xF; /* 主权互补 */
}

/* 叠加态九元偏移 */
static inline uint64_t mqf_smrl_superposition_yuan(
    MqfCell base, int offset_worlds)
{
    uint64_t base_yuan = mqf_cell_get_yuan(base);
    int new_yuan = (int)base_yuan + offset_worlds;
    if (new_yuan < 0) new_yuan = 0;
    if (new_yuan > 8) new_yuan = 8;

    MqfCell result = base;
    result.cell &= ~MQF_CELL_MASK_YUAN;
    result.cell |= ((uint64_t)new_yuan << 60) & MQF_CELL_MASK_YUAN;
    return result.cell;
}

/* ============================================================
 * 双向公约: wasm32/wasm64 兼容层
 * ============================================================ */

#ifdef __wasm__
    #define MQF_SMRL_WASM 1
    #ifdef __wasm64__
        #define MQF_SMRL_ARCH_BITS 64
        #define MQF_SMRL_WASM32 0
    #else
        #define MQF_SMRL_ARCH_BITS 32
        #define MQF_SMRL_WASM32 1
        /* wasm32: 折叠 36-bit 价值到 28-bit */
        #define MQF_CELL_MASK_VALUE_WASM32 (0xFFFFFFFULL)
    #endif
#else
    #define MQF_SMRL_WASM 0
    #define MQF_SMRL_ARCH_BITS 64
    #define MQF_SMRL_WASM32 0
#endif

/* 宽度自适应宏 */
#if MQF_SMRL_WASM32
    #define MQF_CELL_MASK_ACTIVE_VALUE  MQF_CELL_MASK_VALUE_WASM32
#else
    #define MQF_CELL_MASK_ACTIVE_VALUE  MQF_CELL_MASK_VALUE
#endif

MqfCell mqf_smrl_builtin_admit(
    MqfCell agent, MqfCell space, uint64_t rule);

uint64_t mqf_smrl_cell_hash(MqfCell c);

#endif /* MQF_SMRL_H */
