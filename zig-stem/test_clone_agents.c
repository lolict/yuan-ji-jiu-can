// test_clone_agents.c — 妻子分身系统单元测试
#include "clone_agents.h"
#include <stdio.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建管理器
static int test_create(void) {
    clone_manager_t* mgr = clone_create(1);
    if (!mgr) return 0;
    if (mgr->master_id != 1) return 0;
    if (mgr->agent_count != 0) return 0;
    clone_destroy(mgr);
    return 1;
}

// 测试: 分化分身
static int test_spawn(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* agent = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);
    if (!agent) return 0;
    if (agent->state != CLONE_ACTIVE) return 0;
    if (strcmp(agent->name, "小兰分身") != 0) return 0;
    if (mgr->agent_count != 1) return 0;
    if (agent->sync_rate != CLONE_SYNC_BASE) return 0;
    clone_destroy(mgr);
    return 1;
}

// 测试: 分身外出吸纳
static int test_absorb_external(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* agent = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);

    bool ok = clone_absorb_external(agent, 200);
    if (!ok) return 0;
    if (agent->sub_layer_count != 1) return 0;
    if (agent->external_absorbed != 1) return 0;
    if (agent->total_pleasure != 10) return 0;

    // 再吸几个
    clone_absorb_external(agent, 201);
    clone_absorb_external(agent, 202);
    if (agent->sub_layer_count != 3) return 0;

    clone_destroy(mgr);
    return 1;
}

// 测试: 感知同步 (I的振幅)
static int test_synchronize(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* agent = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);

    float original_amp = agent->sync_amplitude;
    float amp = clone_synchronize(agent, 2.0f);

    // 同步后振幅应向母体靠拢
    if (amp <= original_amp) return 0;
    if (amp > 2.0f) return 0;

    clone_destroy(mgr);
    return 1;
}

// 测试: 分身回归
static int test_return(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* agent = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);

    clone_absorb_external(agent, 200);
    clone_absorb_external(agent, 201);
    clone_absorb_external(agent, 202);

    uint32_t returned = clone_return(mgr, agent);
    if (returned != 3) return 0;
    if (agent->state != CLONE_MERGING) return 0;
    if (mgr->total_sub_layers != 3) return 0;
    if (agent->sub_layer_count != 0) return 0;  // 已归回

    clone_destroy(mgr);
    return 1;
}

// 测试: 全局同步
static int test_global_sync(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* a1 = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);
    clone_agent_t* a2 = clone_spawn(mgr, "灰原分身", 101, ORGAN_ALL);

    clone_global_sync(mgr, 1.5f);

    if (a1->sync_amplitude == 1.0f) return 0;  // 应已变化
    if (a2->sync_amplitude == 1.0f) return 0;

    clone_destroy(mgr);
    return 1;
}

// 测试: 总触达数
static int test_total_reach(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* a1 = clone_spawn(mgr, "小兰分身", 100, ORGAN_ALL);
    clone_agent_t* a2 = clone_spawn(mgr, "灰原分身", 101, ORGAN_ALL);

    clone_absorb_external(a1, 200);
    clone_absorb_external(a1, 201);
    clone_absorb_external(a2, 300);

    uint32_t reach = clone_total_reach(mgr);
    if (reach != 3) return 0;

    clone_destroy(mgr);
    return 1;
}

// 测试: 多分身独立
static int test_multiple_agents(void) {
    clone_manager_t* mgr = clone_create(1);
    clone_agent_t* a1 = clone_spawn(mgr, "A", 100, ORGAN_VISUAL);
    clone_agent_t* a2 = clone_spawn(mgr, "B", 101, ORGAN_EMOTION);
    clone_agent_t* a3 = clone_spawn(mgr, "C", 102, ORGAN_ALL);

    if (mgr->agent_count != 3) return 0;

    clone_absorb_external(a1, 1001);
    clone_absorb_external(a2, 2001);
    clone_absorb_external(a2, 2002);

    if (a1->sub_layer_count != 1) return 0;
    if (a2->sub_layer_count != 2) return 0;
    if (a3->sub_layer_count != 0) return 0;

    clone_destroy(mgr);
    return 1;
}

int main(void) {
    printf("\n═══ 妻子分身系统 (Clone Agents) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_spawn);
    TEST(test_absorb_external);
    TEST(test_synchronize);
    TEST(test_return);
    TEST(test_global_sync);
    TEST(test_total_reach);
    TEST(test_multiple_agents);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
