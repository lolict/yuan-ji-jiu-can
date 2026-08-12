// test_toolchain_graph.c — 工具链关联图单元测试
#include "toolchain_graph.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建图
static int test_create(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);
    if (!g) { arbiter_destroy(ae); return 0; }
    if (g->node_count != 0) return 0;
    if (g->edge_count != 0) return 0;
    if (g->arbiter != ae) return 0;
    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 添加节点
static int test_add_node(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "mlil编译器", "毛利兰编程语言", OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "WASM运行时", "浏览器执行", OWNER_WIFE);

    if (g->node_count != 2) return 0;
    if (a != 0 || b != 1) return 0;
    if (strcmp(g->nodes[0].name, "mlil编译器") != 0) return 0;
    if (g->nodes[1].owner != OWNER_WIFE) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 添加边
static int test_add_edge(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "A", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "B", NULL, OWNER_MASTER);
    tcg_add_edge(g, a, b, true);  // A 硬依赖 B

    if (g->edge_count != 1) return 0;
    if (g->edges[0].from != a) return 0;
    if (g->edges[0].to != b) return 0;
    if (!g->edges[0].is_hard) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 决策 (依赖满足)
static int test_decide_ready(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "A", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "B", NULL, OWNER_MASTER);
    tcg_add_edge(g, a, b, true);

    // B 完成
    tcg_set_state(g, b, TOOL_COMPLETED);

    tcg_decision_t dec = tcg_decide(g, a);
    if (!dec.can_execute) return 0;
    if (dec.confidence <= 0) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 决策 (依赖未满足)
static int test_decide_blocked(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "A", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "B", NULL, OWNER_MASTER);
    tcg_add_edge(g, a, b, true);

    // B 未完成
    tcg_set_state(g, b, TOOL_READY);

    tcg_decision_t dec = tcg_decide(g, a);
    if (dec.can_execute) return 0;
    if (dec.blocking_count == 0) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 决策 (外部工具)
static int test_decide_external(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t ext = tcg_add_node(g, "外部工具", NULL, OWNER_EXTERNAL);

    tcg_decision_t dec = tcg_decide(g, ext);
    if (dec.can_execute) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 决策 (吉凶影响)
static int test_decide_fortune(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "A", NULL, OWNER_MASTER);
    tcg_set_fortune(g, a, FORTUNE_BLESSING);

    tcg_decision_t dec = tcg_decide(g, a);
    if (!dec.can_execute) return 0;
    if (dec.fortune_score < 0) return 0;  // 福应为正

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 拓扑排序
static int test_topo_order(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "基础", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "中层", NULL, OWNER_MASTER);
    uint32_t c = tcg_add_node(g, "顶层", NULL, OWNER_MASTER);

    tcg_add_edge(g, b, a, true);  // B依赖A
    tcg_add_edge(g, c, b, true);  // C依赖B

    uint32_t order[8];
    uint32_t count = tcg_topological_order(g, order, 8);

    if (count != 3) return 0;
    // A应在B前, B应在C前
    if (order[0] != a) return 0;
    if (order[1] != b) return 0;
    if (order[2] != c) return 0;

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 模拟执行
static int test_simulate(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "基础", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "中层", NULL, OWNER_MASTER);
    uint32_t c = tcg_add_node(g, "顶层", NULL, OWNER_MASTER);

    tcg_add_edge(g, b, a, true);
    tcg_add_edge(g, c, b, true);

    uint32_t executed = tcg_simulate_execute(g);
    if (executed != 3) return 0;

    // 所有工具应已完成
    for (uint32_t i = 0; i < g->node_count; i++) {
        if (g->nodes[i].state != TOOL_COMPLETED) return 0;
    }

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

// 测试: 关键路径
static int test_critical_path(void) {
    arbiter_engine_t* ae = arbiter_create("裁决");
    toolchain_graph_t* g = tcg_create(ae);

    uint32_t a = tcg_add_node(g, "基础", NULL, OWNER_MASTER);
    uint32_t b = tcg_add_node(g, "中层", NULL, OWNER_MASTER);
    uint32_t c = tcg_add_node(g, "顶层", NULL, OWNER_MASTER);
    uint32_t d = tcg_add_node(g, "旁路", NULL, OWNER_MASTER);

    tcg_add_edge(g, b, a, true);
    tcg_add_edge(g, c, b, true);
    tcg_add_edge(g, d, a, true);

    uint32_t path[8];
    uint32_t count = tcg_critical_path(g, path, 8);

    if (count < 2) return 0;  // 至少应有2个节点的路径

    tcg_destroy(g);
    arbiter_destroy(ae);
    return 1;
}

int main(void) {
    printf("\n═══ 工具链关联图 (Toolchain Graph) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_add_node);
    TEST(test_add_edge);
    TEST(test_decide_ready);
    TEST(test_decide_blocked);
    TEST(test_decide_external);
    TEST(test_decide_fortune);
    TEST(test_topo_order);
    TEST(test_simulate);
    TEST(test_critical_path);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
