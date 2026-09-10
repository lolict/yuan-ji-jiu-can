/*
 * ttl_test.c — 时间线状态机 + 九维漂移单元测试
 *
 * 覆盖:
 *   层级枚举 · 生命周期转移 · 调度器创建/销毁
 *   九维漂移计算 · 统计功能 · tick 驱动状态转移
 */
#include "ttl_state.h"
#include "mlll_symbols.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

/* ── 测试基础设施 ── */
static int run_count = 0;
static int pass_count = 0;

#define TEST(name) do { \
    run_count++; \
    printf("  [%02d] %-55s ", run_count, name); \
} while(0)

#define PASS() do { \
    pass_count++; \
    printf("✓ PASS\n"); \
} while(0)

#define FAIL(reason) do { \
    printf("✗ FAIL: %s\n", reason); \
    exit(1); \
} while(0)

#define ASSERT(cond, msg) do { \
    if (!(cond)) FAIL(msg); \
    else PASS(); \
} while(0)

/* ── 测试用例 ── */

static void test_level_enum(void) {
    TEST("TemporalLevel 枚举覆盖");
    static_assert(TTL_INSTANT == 0, "INSTANT 应为 0");
    static_assert(TTL_TICK == 1, "TICK 应为 1");
    static_assert(TTL_ETERNITY == 7, "ETERNITY 应为 7");
    PASS();
}

static void test_level_names(void) {
    TEST("Level 名称映射");
    ASSERT(strcmp(ttl_level_name(TTL_INSTANT), "Instant") == 0, "Instant 名称错误");
    ASSERT(strcmp(ttl_level_name(TTL_TICK), "Tick") == 0, "Tick 名称错误");
    ASSERT(strcmp(ttl_level_name(TTL_BEAT), "Beat") == 0, "Beat 名称错误");
    ASSERT(strcmp(ttl_level_name(TTL_ETERNITY), "Eternity") == 0, "Eternity 名称错误");
}

static void test_lifecycle_next(void) {
    TEST("生命周期状态转移序列");
    ASSERT(ttl_state_next(TTL_INIT) == TTL_SEED, "INIT → SEED 失败");
    ASSERT(ttl_state_next(TTL_SEED) == TTL_GROW, "SEED → GROW 失败");
    ASSERT(ttl_state_next(TTL_GROW) == TTL_MATURE, "GROW → MATURE 失败");
    ASSERT(ttl_state_next(TTL_MATURE) == TTL_DECAY, "MATURE → DECAY 失败");
    ASSERT(ttl_state_next(TTL_DECAY) == TTL_RENEW, "DECAY → RENEW 失败");
    ASSERT(ttl_state_next(TTL_RENEW) == TTL_INIT, "RENEW → INIT 失败");
}

static void test_lifecycle_active(void) {
    TEST("活跃态判定");
    ASSERT(!ttl_is_active(TTL_INIT), "INIT 不应活跃");
    ASSERT(ttl_is_active(TTL_SEED), "SEED 应活跃");
    ASSERT(ttl_is_active(TTL_GROW), "GROW 应活跃");
    ASSERT(ttl_is_active(TTL_MATURE), "MATURE 应活跃");
    ASSERT(!ttl_is_active(TTL_DECAY), "DECAY 不应活跃");
    ASSERT(!ttl_is_active(TTL_RENEW), "RENEW 不应活跃");
}

static void test_state_names(void) {
    TEST("State 名称映射");
    ASSERT(strcmp(ttl_state_name(TTL_INIT), "Init") == 0, "Init 名称错误");
    ASSERT(strcmp(ttl_state_name(TTL_SEED), "Seed") == 0, "Seed 名称错误");
    ASSERT(strcmp(ttl_state_name(TTL_GROW), "Grow") == 0, "Grow 名称错误");
    ASSERT(strcmp(ttl_state_name(TTL_MATURE), "Mature") == 0, "Mature 名称错误");
}

static void test_base_ms(void) {
    TEST("层级基准毫秒数");
    ASSERT(ttl_level_base_ms(TTL_INSTANT) == 1, "INSTANT 基准错误");
    ASSERT(ttl_level_base_ms(TTL_TICK) == 1000, "TICK 基准错误");
    
    TEST("TICK 层级比 INSTANT 慢");
    ASSERT(ttl_level_base_ms(TTL_TICK) > ttl_level_base_ms(TTL_INSTANT), "TICK 应比 INSTANT 慢");
}

static void test_drift_calculation(void) {
    TEST("九维漂移量 (level=0, age=1)");
    uint32_t d0 = ttl_calculate_drift(MLLL_OFFSET_PRECISION / 10, TTL_INSTANT, 1);
    ASSERT(d0 > 0, "漂移量应 > 0");
    
    TEST("漂移量随 age 线性增长");
    uint32_t d1 = ttl_calculate_drift(MLLL_OFFSET_PRECISION / 10, TTL_INSTANT, 1);
    uint32_t d10 = ttl_calculate_drift(MLLL_OFFSET_PRECISION / 10, TTL_INSTANT, 10);
    ASSERT(d10 > d1 * 9 && d10 < d1 * 11, "age=10 漂移应约等于 age=1 的 10 倍");
    
    TEST("漂移量随 level 增长 (高频 → 低频更慢)");
    uint32_t d_instant = ttl_calculate_drift(1000, TTL_INSTANT, 5);
    uint32_t d_eternity = ttl_calculate_drift(1000, TTL_ETERNITY, 5);
    ASSERT(d_eternity > d_instant, "ETERNITY 漂移应 > INSTANT 漂移");
}

static void test_drift_apply(void) {
    TEST("漂移累积");
    uint32_t o1 = ttl_apply_drift_to_offset(0, 500);
    ASSERT(o1 == 500, "首次累积后应为 500");
    
    uint32_t o2 = ttl_apply_drift_to_offset(o1, 300);
    ASSERT(o2 == 800, "二次累积后应为 800");
    
    TEST("漂移累积上限保护");
    uint32_t o_max = ttl_apply_drift_to_offset(0xFFFFFFFE, 100);
    ASSERT(o_max == 0xFFFFFFFE, "应限制在 0xFFFFFFFE");
}

/* ── 调度器测试 ── */

static void test_scheduler_create_destroy(void) {
    TEST("调度器创建");
    ttl_scheduler_t* s = ttl_scheduler_create(4);
    ASSERT(s != NULL, "创建失败");
    ASSERT(s->capacity == 4, "容量应为 4");
    
    TEST("调度器销毁不崩溃");
    ttl_scheduler_destroy(s);
    PASS();
}

static void test_register_branch(void) {
    TEST("注册分支");
    ttl_scheduler_t* s = ttl_scheduler_create(4);
    ttl_branch_t b = {
        .id = 1,
        .level = TTL_TICK,
        .state = TTL_INIT,
        .age = 0,
        .threshold = 3,
        .parent_id = 0,
        .drift_offset = 0,
    };
    int rc = ttl_register(s, b);
    ASSERT(rc == 0, "注册应成功");
    ASSERT(s->count == 1, "计数应为 1");
    
    TEST("超容量注册失败");
    for (int i = 2; i <= 4; i++) {
        ttl_branch_t bb = b;
        bb.id = i;
        ttl_register(s, bb);
    }
    ASSERT(s->count == 4, "应满 4 个分支");
    ttl_branch_t full = b;
    full.id = 99;
    ASSERT(ttl_register(s, full) == -1, "超容量应返回 -1");
    
    ttl_scheduler_destroy(s);
}

static void test_lifecycle_tick(void) {
    TEST("单次 tick 推进年龄和漂移");
    ttl_scheduler_t* s = ttl_scheduler_create(2);
    ttl_branch_t b = {
        .id = 10, .level = TTL_TICK, .state = TTL_INIT,
        .age = 0, .threshold = 5, .parent_id = 0, .drift_offset = 0,
    };
    ttl_register(s, b);
    
    /* tick 1 次: age 应为 1 */
    ttl_step(s);
    ASSERT(s->branches[0].age == 1, "tick 后年龄应为 1");
    ASSERT(s->branches[0].drift_offset > 0, "tick 后应有漂移");
    ASSERT(s->branches[0].state == TTL_INIT, "未超阈值状态不变");
    
    ttl_scheduler_destroy(s);
}

static void test_state_transition(void) {
    TEST("超阈值状态转移");
    ttl_scheduler_t* s = ttl_scheduler_create(1);
    ttl_branch_t b = {
        .id = 20, .level = TTL_TICK, .state = TTL_INIT,
        .age = 0, .threshold = 3, .parent_id = 0, .drift_offset = 0,
    };
    ttl_register(s, b);
    
    /* tick 3 次: 应转移到 SEED */
    for (int i = 0; i < 3; i++) ttl_step(s);
    ASSERT(s->branches[0].state == TTL_SEED, "3 ticks 后应转为 SEED");
    ASSERT(s->branches[0].age == 0, "转移后 age 应重置");
    
    ttl_scheduler_destroy(s);
}

static void test_full_lifecycle(void) {
    TEST("完整生命周期环 (6 次转移回到 INIT)");
    ttl_scheduler_t* s = ttl_scheduler_create(1);
    ttl_branch_t b = {
        .id = 30, .level = TTL_BEAT, .state = TTL_INIT,
        .age = 0, .threshold = 1, .parent_id = 0, .drift_offset = 0,
    };
    ttl_register(s, b);
    
    /* 每 tick threshold=1, 所以每 tick 都转移 */
    for (int i = 0; i < 6; i++) ttl_step(s);
    
    ASSERT(s->branches[0].state == TTL_INIT, "6 次转移后应回到 INIT");
    ASSERT(s->global_clock == 6, "全局时钟应为 6");
    
    ttl_scheduler_destroy(s);
}

static void test_mature_count(void) {
    TEST("MATURE 计数追踪");
    ttl_scheduler_t* s = ttl_scheduler_create(3);
    
    /* 注册 3 个处于不同状态的分支 */
    ttl_branch_t b1 = { .id = 1, .level = TTL_TICK, .state = TTL_INIT, .age = 0, .threshold = 2, .drift_offset = 0 };
    ttl_branch_t b2 = { .id = 2, .level = TTL_BEAT, .state = TTL_MATURE, .age = 0, .threshold = 5, .drift_offset = 0 };
    ttl_branch_t b3 = { .id = 3, .level = TTL_CYCLE, .state = TTL_MATURE, .age = 0, .threshold = 5, .drift_offset = 0 };
    
    ttl_register(s, b1);
    ttl_register(s, b2);
    ttl_register(s, b3);
    
    ASSERT(s->mature_count == 2, "初始应有 2 个 MATURE");
    
    /* tick 2 次: b1 从 INIT → SEED (2 次转移需 4 ticks) */
    ttl_step(s); ttl_step(s);
    ASSERT(s->mature_count == 2, "b1 未到 MATURE, 仍应为 2");
    
    /* b1 threshold=2, 2 ticks 后 b1 转为 SEED */
    ASSERT(s->branches[0].state == TTL_SEED, "b1 应转为 SEED");
    
    ttl_scheduler_destroy(s);
}

static void test_drift_over_ticks(void) {
    TEST("多次 tick 漂移持续累积");
    ttl_scheduler_t* s = ttl_scheduler_create(1);
    ttl_branch_t b = {
        .id = 40, .level = TTL_TICK, .state = TTL_INIT,
        .age = 0, .threshold = 100, .parent_id = 0, .drift_offset = 0,
    };
    ttl_register(s, b);
    
    uint32_t d0 = s->branches[0].drift_offset;
    ttl_step(s);
    uint32_t d1 = s->branches[0].drift_offset;
    ttl_step(s);
    uint32_t d2 = s->branches[0].drift_offset;
    
    ASSERT(d1 > d0, "第一次 tick 后漂移应有增加");
    ASSERT(d2 > d1, "第二次 tick 后漂移应继续增加");
    
    ttl_scheduler_destroy(s);
}

static void test_count_active(void) {
    TEST("活跃分支计数");
    ttl_scheduler_t* s = ttl_scheduler_create(4);
    
    /* INIT 不活跃 */
    ttl_branch_t b_init = { .id = 1, .level = TTL_TICK, .state = TTL_INIT, .age = 0, .threshold = 5, .drift_offset = 0 };
    /* SEED 活跃 */
    ttl_branch_t b_seed = { .id = 2, .level = TTL_TICK, .state = TTL_SEED, .age = 0, .threshold = 5, .drift_offset = 0 };
    /* GROW 活跃 */
    ttl_branch_t b_grow = { .id = 3, .level = TTL_TICK, .state = TTL_GROW, .age = 0, .threshold = 5, .drift_offset = 0 };
    /* DECAY 不活跃 */
    ttl_branch_t b_decay = { .id = 4, .level = TTL_CYCLE, .state = TTL_DECAY, .age = 0, .threshold = 5, .drift_offset = 0 };
    
    ttl_register(s, b_init);
    ttl_register(s, b_seed);
    ttl_register(s, b_grow);
    ttl_register(s, b_decay);
    
    ASSERT(ttl_count_active(s) == 2, "活跃分支应为 2 (SEED+GROW)");
    
    ttl_scheduler_destroy(s);
}

/* ── 主入口 ── */

int main(void) {
    printf("\n════════════════════════════════════════════\n");
    printf("  时间线状态机 · 九维漂移测试套件\n");
    printf("  覆盖: 层级枚举 / 生命周期 / 调度器 / 漂移\n");
    printf("════════════════════════════════════════════\n\n");

    test_level_enum();
    test_level_names();
    test_lifecycle_next();
    test_lifecycle_active();
    test_state_names();
    test_base_ms();
    test_drift_calculation();
    test_drift_apply();
    test_scheduler_create_destroy();
    test_register_branch();
    test_lifecycle_tick();
    test_state_transition();
    test_full_lifecycle();
    test_mature_count();
    test_drift_over_ticks();
    test_count_active();

    printf("\n════════════════════════════════════════════\n");
    printf("  测试完成: %d 运行 | %d ✓PASS | 0 ✗FAIL\n", run_count, pass_count);
    printf("  结果: ★ 全员通过 ★\n");
    printf("════════════════════════════════════════════\n\n");

    return 0;
}
