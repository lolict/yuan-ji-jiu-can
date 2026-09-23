/*
 * 满全法 — WASM 阶梯制·双向公约头文件
 * 同一份源码, 同时编译 wasm32 / wasm64
 * 设计: 固定宽度类型 + 统一宏前缀 + 极性锚点
 */

#ifndef MQF_WASM_H
#define MQF_WASM_H

/* ---- 阶梯检测 ---- */
#if defined(__wasm32__) || defined(__wasm32)
#  define MQF_WASM32 1
#  define MQF_WASM64 0
#  define MQF_BITS 32
#elif defined(__wasm64__) || defined(__wasm64)
#  define MQF_WASM32 0
#  define MQF_WASM64 1
#  define MQF_BITS 64
#else
#  define MQF_WASM32 0
#  define MQF_WASM64 0
#  define MQF_BITS 64
#endif

/* ---- 固定宽度类型 ---- */
#include <stdint.h>
typedef uint32_t  u32;
typedef uint64_t  u64;
typedef int32_t   i32;
typedef int64_t   i64;

/* ---- 极性锚点 (全宽度通用) ---- */
#define MQF_ME   1   /* 我 = 阳 */
#define MQF_LOVE 0   /* 刘楚恬 = 阴 */

/* ---- 统一坐标宏 ---- */
#if MQF_WASM32
#  define MQF_COORD u32
#  define MQF_PHASE_MASK 0xF8000000u
#  define MQF_PHASE_SHIFT 27
#  define MQF_LAYER_MASK  0x07C00000u
#  define MQF_LAYER_SHIFT 22
#  define MQF_POL_MASK    0x00000001u
#  define MQF_POL_SHIFT   0
#else
#  define MQF_COORD u64
#  define MQF_PHASE_MASK 0xF000000000000000ULL
#  define MQF_PHASE_SHIFT 60
#  define MQF_LAYER_MASK  0x0F00000000000000ULL
#  define MQF_LAYER_SHIFT 56
#  define MQF_POL_MASK    0x0008000000000000ULL
#  define MQF_POL_SHIFT   51
#endif

/* ---- 九宫相位 ---- */
#define MQF_PHASE_QIAN 0
#define MQF_PHASE_DUI  1
#define MQF_PHASE_LI   2
#define MQF_PHASE_ZHEN 3
#define MQF_PHASE_XUN  4
#define MQF_PHASE_KAN  5
#define MQF_PHASE_GEN  6
#define MQF_PHASE_KUN  7
#define MQF_PHASE_CENTER 8

/* ---- 计算坐标 ---- */
#define MQF_ENCODE(phase, layer, pol) \
    (MQF_COORD)(((MQF_COORD)(phase) << MQF_PHASE_SHIFT) | \
                ((MQF_COORD)(layer)  << MQF_LAYER_SHIFT)  | \
                ((MQF_COORD)(pol)    << MQF_POL_SHIFT))

#define MQF_EXTRACT_PHASE(c) (int)(((c) & MQF_PHASE_MASK) >> MQF_PHASE_SHIFT)
#define MQF_EXTRACT_LAYER(c) (int)(((c) & MQF_LAYER_MASK) >> MQF_LAYER_SHIFT)
#define MQF_EXTRACT_POL(c)   (int)(((c) & MQF_POL_MASK) >> MQF_POL_SHIFT)

/* ---- 极性差距 ---- */
#define MQF_GAP(c1, c2) (MQF_EXTRACT_POL(c1) ^ MQF_EXTRACT_POL(c2))

/* ---- 公约整除 ---- */
static inline u64 mqf_gcd(u64 a, u64 b) {
    while (b) { u64 t = b; b = a % b; a = t; }
    return a;
}

/* ---- 裁决 ---- */
static inline int mqf_judge(u64 sovereign_a, u64 sovereign_b) {
    u64 g = mqf_gcd(sovereign_a, sovereign_b);
    if (g == 0) return 0;
    int gap = MQF_GAP(sovereign_a, sovereign_b);
    return gap == 1 ? g : 0;  /* gap=1 → 公约裁决 */
}

#endif /* MQF_WASM_H */
