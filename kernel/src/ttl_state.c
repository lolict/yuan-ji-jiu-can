/*
 * ttl_state.c — 时间线状态机实现
 *
 * 生命周期状态机 + 九维空间漂移计算
 * 与 MLLL 系统集成: 提供 tick 驱动坐标演化的基础设施
 */

#include "ttl_state.h"
#include "mlll_symbols.h"
#include <stdlib.h>
#include <string.h>

/* ── 内部辅助 ── */

static ttl_branch_t* ttl_find(ttl_scheduler_t* s, uint32_t id) {
    for (uint32_t i = 0; i < s->count; i++) {
        if (s->branches[i].id == id) {
            return &s->branches[i];
        }
    }
    return NULL;
}

/* ── 基本操作 ── */

ttl_scheduler_t* ttl_scheduler_create(uint32_t capacity) {
    ttl_scheduler_t* s = (ttl_scheduler_t*)malloc(sizeof(ttl_scheduler_t));
    if (!s) return NULL;
    s->branches = (ttl_branch_t*)calloc(capacity, sizeof(ttl_branch_t));
    if (!s->branches) { free(s); return NULL; }
    s->count = 0;
    s->capacity = capacity;
    s->global_clock = 0;
    s->mature_count = 0;
    return s;
}

void ttl_scheduler_destroy(ttl_scheduler_t* s) {
    if (s) {
        free(s->branches);
        free(s);
    }
}

int ttl_register(ttl_scheduler_t* s, ttl_branch_t branch) {
    if (s->count >= s->capacity) return -1;
    branch.drift_offset = 0;
    if (branch.state == TTL_MATURE) s->mature_count++;
    s->branches[s->count++] = branch;
    return (int)s->count - 1;
}

int ttl_link_parent(ttl_scheduler_t* s, uint32_t parent_id, uint32_t child_id) {
    ttl_branch_t* p = ttl_find(s, parent_id);
    ttl_branch_t* c = ttl_find(s, child_id);
    if (!p || !c) return -1;
    c->parent_id = parent_id;
    return 0;
}

/*
 * 推进一个 tick:
 * 1. 全局时钟 ++
 * 2. 每个分支 age++, 超过 threshold 则状态转移
 * 3. 转移时更新 mature_count
 * 4. 计算并累积 drift_offset
 * 5. 产生漂移事件 (返回给调用者)
 * 返回: 本次 tick 发生转移的分支数
 */
int ttl_step(ttl_scheduler_t* s) {
    s->global_clock++;
    int transitions = 0;

    for (uint32_t i = 0; i < s->count; i++) {
        ttl_branch_t* b = &s->branches[i];
        b->age++;

        /* 计算并累积漂移 */
        uint32_t drift = ttl_calculate_drift(
            MLLL_OFFSET_PRECISION / 10,  /* 基础偏移 = 0.1 */
            b->level,
            b->age
        );
        b->drift_offset = ttl_apply_drift_to_offset(b->drift_offset, drift);

        /* 状态转移 */
        if (b->age >= b->threshold) {
            ttl_lifecycle_t old = b->state;
            b->state = ttl_state_next(old);
            b->age = 0;

            /* 更新 mature 计数 */
            if (old == TTL_MATURE && b->state != TTL_MATURE) s->mature_count--;
            else if (old != TTL_MATURE && b->state == TTL_MATURE) s->mature_count++;

            transitions++;
        }
    }
    return transitions;
}

/* ── 状态查询 ── */

ttl_lifecycle_t ttl_state_next(ttl_lifecycle_t s) {
    switch (s) {
        case TTL_INIT:   return TTL_SEED;
        case TTL_SEED:   return TTL_GROW;
        case TTL_GROW:   return TTL_MATURE;
        case TTL_MATURE: return TTL_DECAY;
        case TTL_DECAY:  return TTL_RENEW;
        case TTL_RENEW:  return TTL_INIT;
        default:         return TTL_INIT;
    }
}

int ttl_is_active(ttl_lifecycle_t s) {
    return s == TTL_SEED || s == TTL_GROW || s == TTL_MATURE;
}

const char* ttl_state_name(ttl_lifecycle_t s) {
    switch (s) {
        case TTL_INIT:   return "Init";
        case TTL_SEED:   return "Seed";
        case TTL_GROW:   return "Grow";
        case TTL_MATURE: return "Mature";
        case TTL_DECAY:  return "Decay";
        case TTL_RENEW:  return "Renew";
        default:         return "Unknown";
    }
}

const char* ttl_level_name(ttl_level_t l) {
    switch (l) {
        case TTL_INSTANT:  return "Instant";
        case TTL_TICK:     return "Tick";
        case TTL_BEAT:     return "Beat";
        case TTL_CYCLE:    return "Cycle";
        case TTL_PHASE:    return "Phase";
        case TTL_ERA:      return "Era";
        case TTL_EPOCH:    return "Epoch";
        case TTL_ETERNITY: return "Eternity";
        default:           return "Unknown";
    }
}

uint32_t ttl_level_base_ms(ttl_level_t l) {
    switch (l) {
        case TTL_INSTANT:  return 1;
        case TTL_TICK:     return 1000;
        case TTL_BEAT:     return 60000;
        case TTL_CYCLE:    return 3600000;
        case TTL_PHASE:    return 86400000;
        case TTL_ERA:      return 604800000;
        case TTL_EPOCH:    return 2592000000u;
        case TTL_ETERNITY: return 0xFFFFFFFF;
        default:           return 0;
    }
}

/* ── 九维漂移计算 ── */

/*
 * drift = base_offset × (1 + level × 0.1) × age
 * 由于整数运算, 0.1 用 1000/10000 表示
 */
uint32_t ttl_calculate_drift(uint32_t base_offset, ttl_level_t level, uint32_t age) {
    /* (1 + level × 0.1) 用整数: factor = 1000 + level × 100 */
    uint32_t factor = 1000 + (uint32_t)level * 100;
    /* drift = base_offset × factor × age / 1000 */
    uint64_t result = (uint64_t)base_offset * factor * age / 1000;
    return (uint32_t)result;
}

uint32_t ttl_apply_drift_to_offset(uint32_t old_offset, uint32_t drift) {
    /* 限制最大值, 防止溢出 */
    uint64_t sum = (uint64_t)old_offset + drift;
    if (sum > 0xFFFFFFFE) return 0xFFFFFFFE;
    return (uint32_t)sum;
}

/* ── 统计 ── */

uint32_t ttl_count_active(ttl_scheduler_t* s) {
    uint32_t n = 0;
    for (uint32_t i = 0; i < s->count; i++) {
        if (ttl_is_active(s->branches[i].state)) n++;
    }
    return n;
}

uint32_t ttl_count_mature(ttl_scheduler_t* s) {
    return s->mature_count;
}

uint32_t ttl_global_clock(ttl_scheduler_t* s) {
    return s->global_clock;
}
