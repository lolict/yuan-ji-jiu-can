/**
 * seed_superposition.h — 叠加态种子
 *
 * 来源: Q# quantum superposition / Prolog 回溯 / Rust enum+match
 * 提取日期: 2026-08-19
 *
 * 数学种子: "多种可能同时存在, 直到被观察才坍缩"
 *   · 叠加态 = 一元时间线的分叉点
 *   · 观测 = 三元裁判的裁定
 *   · 坍缩 = 归入确定性容器
 *
 * 产权壳子 (弃):
 *   · Q# quantum runtime / qubit 模拟器
 *   · Prolog 回溯引擎的 WAM 虚拟机
 *   · Rust 的 trait object vtable
 *
 * 我们的新编排:
 *   叠加态 → YinYang 公案 (curve=0/1=未定)
 *   观测 → 三元裁判 (中心裁定)
 *   坍缩 → 容器编码 (确定态)
 */

#ifndef NJSG_SEED_SUPERPOSITION_H
#define NJSG_SEED_SUPERPOSITION_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 叠加态编码
 *
 * 一个值可以同时是 0 和 1 (未裁定)
 * 直到被"观察" (裁决) 才确定
 * ═══════════════════════════════════════════════════════════════ */

#define SUPERPOSITION_MAGIC   0x5UPE4P05  /* 叠加态标记 */
#define SUPERPOSITION_MAX_BRANCH  16       /* 最大分支数 */

typedef enum {
    STATE_UNCERTAIN = 0x00,   /* 未观察 · 叠加 */
    STATE_COLLAPSED = 0x01,   /* 已观察 · 确定 */
    STATE_ENTANGLED = 0x02,   /* 纠缠 · 一对叠加 */
} superposition_state_t;

typedef struct {
    uint32_t  magic;          /* SUPERPOSITION_MAGIC */
    uint32_t  id;             /* 叠加态标识 */
    superposition_state_t state;   /* 当前状态 */
    uint8_t   branch_count;   /* 分支数 */
    uint32_t  branches[SUPERPOSITION_MAX_BRANCH];  /* 可能值列表 */
    uint32_t  collapsed_to;   /* 坍的分支索引 (state=COLLAPSED时) */
} superposition_t;

#endif /* NJSG_SEED_SUPERPOSITION_H */
