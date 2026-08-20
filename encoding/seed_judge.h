/**
 * seed_judge.h — 准入裁判种子
 *
 * 来源: Rust trait bounds / C++ concepts / Nim type classes / Prolog 规则
 * 提取日期: 2026-08-19
 *
 * 数学种子: "通过才能进入 · 不通过则拒绝"
 *   · 准入条件 = 三元裁判
 *   · 约束即因果
 *   · 必须证明自己才可使用
 *
 * 产权壳子 (弃):
 *   · Rust borrow checker / Polonius 借用检查器
 *   · C++ concept 的 SFINAE 机制
 *   · Prolog 的 Horn 子句 / Unification
 *
 * 我们的新编排:
 *   约束 → 三元裁判的 PERCEIVE + CLASSIFY + DYNAMIC
 *   通过 → 容器化 (CONTAINER_ENCODE)
 *   拒绝 → pipeline 不进入 · 状态标记 REJECTED
 */

#ifndef NJSG_SEED_JUDGE_H
#define NJSG_SEED_JUDGE_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 约束谓词 (Constraint Predicate)
 *
 * 三条因果链都通过才准入:
 *   PERCEIVE: 符号是否被正确感知 (知道它是什么)
 *   CLASSIFY: 分类归属是否正确 (它属于哪个家族)
 *   DYNAMIC: 动力充足 (有足够的力)
 *
 * 缺一不可 · 三元约束 = 裁决引擎的三股力
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    JUDGE_PERCEIVE  = 0x10,   /* 感知链 */
    JUDGE_CLASSIFY  = 0x20,   /* 分类链 */
    JUDGE_DYNAMIC   = 0x40,   /* 动力链 */
    JUDGE_ALL       = 0x70,   /* 全链 */
} judge_chain_t;

#define JUDGE_MAGIC   0x3UD63  /* 裁判引擎标记 */

typedef struct {
    uint32_t  magic;          /* JUDGE_MAGIC */
    uint32_t  judge_id;       /* 裁判id */
    uint32_t  perceive_pass;  /* 感知链通过计数 */
    uint32_t  perceive_fail;  /* 感知链拒绝计数 */
    uint32_t  classify_pass;  /* 分类链通过计数 */
    uint32_t  classify_fail;  /* 分类链拒绝计数 */
    uint32_t  dynamic_power;  /* 动力余量 */
    uint32_t  total_admit;    /* 准入总数 */
    uint32_t  total_reject;   /* 拒绝总数 */
} judge_engine_t;

#endif /* NJSG_SEED_JUDGE_H */
