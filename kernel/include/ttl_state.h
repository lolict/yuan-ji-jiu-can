/*
 * ttl_state.h — 时间线状态机 · 九维空间时间化
 *
 * 实现感知状态机生命周期:
 *   Init → Seed → Grow → Mature → Decay → Renew
 *
 * 核心机制:
 *   - tick 驱动 MLLL 九维坐标在时间线上的漂移
 *   - 每个 tick 按 offset_rate (delta=0.1) 推进叠加态
 *   - 漂移量由 TemporalLevel 决定 (Instant=0 → Eternity=7)
 *
 * 与 MLLL 的关系:
 *   MLLL 提供 9 维坐标空间, 本模块提供时间推进
 *   每个坐标 (stroke/tone/radical/structure/vowel/initial)
 *   在 tick 后按 offset_rate × level 产生相位偏移
 *
 * 理论根基:
 *   并集 → 交集 感知转化
 *   每个容器(分支)独立运行自己的生命周期
 *   在公约数时刻, 所有处于 Mature 的容器产生耦合
 */
#ifndef TTL_STATE_H
#define TTL_STATE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 时间层级 (从高频到低频) */
typedef enum {
    TTL_INSTANT  = 0,   /* 瞬态 · 单次触发 */
    TTL_TICK     = 1,   /* 滴答 · ~毫秒/秒 */
    TTL_BEAT     = 2,   /* 节拍 · ~秒/分 */
    TTL_CYCLE    = 3,   /* 周期 · ~分/时 */
    TTL_PHASE    = 4,   /* 阶段 · ~时/日 */
    TTL_ERA      = 5,   /* 时代 · ~日/月 */
    TTL_EPOCH    = 6,   /* 纪元 · ~月/年 */
    TTL_ETERNITY = 7,   /* 永恒 · 存在性常量 */
} ttl_level_t;

/* 生命周期状态 */
typedef enum {
    TTL_INIT   = 0,   /* 初始化 · 刚被创建 */
    TTL_SEED   = 1,   /* 种子 · 开始吸收资源 */
    TTL_GROW   = 2,   /* 生长 · 快速膨胀 */
    TTL_MATURE = 3,   /* 成熟 · 稳定产出 */
    TTL_DECAY  = 4,   /* 衰亡 · 缓慢退化 */
    TTL_RENEW  = 5,   /* 重生 · 回归 INIT 或消亡 */
} ttl_lifecycle_t;

/* 时间线分支节点 */
typedef struct {
    uint32_t         id;           /* 唯一标识 */
    ttl_level_t      level;        /* 所在时间层级 */
    ttl_lifecycle_t  state;        /* 当前生命周期状态 */
    uint32_t         age;          /* 在当前状态的刻数 */
    uint32_t         threshold;    /* 触发状态转移的阈值 */
    uint32_t         parent_id;    /* 父节点 ID (0 = 无父) */
    uint32_t         drift_offset; /* 当前九维漂移量 (×10000 精度) */
} ttl_branch_t;

/* 时间线调度器 */
typedef struct {
    ttl_branch_t*  branches;        /* 分支数组 */
    uint32_t       count;           /* 分支数量 */
    uint32_t       capacity;        /* 数组容量 */
    uint32_t       global_clock;    /* 全局时钟 (tick 数) */
    uint32_t       mature_count;    /* 处于 MATURE 的分支数 */
} ttl_scheduler_t;

/* ── 基本操作 ── */

ttl_scheduler_t* ttl_scheduler_create(uint32_t capacity);
void             ttl_scheduler_destroy(ttl_scheduler_t* s);
int              ttl_register(ttl_scheduler_t* s, ttl_branch_t branch);
int              ttl_link_parent(ttl_scheduler_t* s, uint32_t parent, uint32_t child);
int              ttl_step(ttl_scheduler_t* s);  /* 推进一个 tick, 返回转移数 */

/* ── 状态查询 ── */

ttl_lifecycle_t  ttl_state_next(ttl_lifecycle_t s);
int              ttl_is_active(ttl_lifecycle_t s);
const char*      ttl_state_name(ttl_lifecycle_t s);
const char*      ttl_level_name(ttl_level_t l);
uint32_t         ttl_level_base_ms(ttl_level_t l);

/* ── 九维漂移计算 ── */

/*
 * 计算 tick 后的漂移量
 * drift = base_offset × (1 + level × 0.1) × age
 * 精度: 除以 MLLL_OFFSET_PRECISION (10000) 得到实际值
 */
uint32_t ttl_calculate_drift(uint32_t base_offset, ttl_level_t level, uint32_t age);

/*
 * 应用漂移到 MLLL 偏移
 * new_offset = old_offset + drift
 */
uint32_t ttl_apply_drift_to_offset(uint32_t old_offset, uint32_t drift);

/* ── 统计 ── */

uint32_t ttl_count_active(ttl_scheduler_t* s);
uint32_t ttl_count_mature(ttl_scheduler_t* s);
uint32_t ttl_global_clock(ttl_scheduler_t* s);

#ifdef __cplusplus
}
#endif

#endif /* TTL_STATE_H */
