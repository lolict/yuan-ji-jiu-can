/**
 * test_weaving.c — 编织平衡器测试
 *
 * 编译: cc -Wall -o test_wov weaving_balancer.c test_weaving.c -lm
 * 运行: ./test_wov
 */

#include <stdio.h>
#include <string.h>
#include "weaving_balancer.h"

static int tc = 0, pc = 0, fc = 0;

void expect_int(int expected, int got, const char *desc) {
    tc++;
    int ok = (expected == got);
    printf("  %s  %s (expected=%d, got=%d)\n", ok ? "[PASS]" : "[FAIL]", desc, expected, got);
    if (ok) pc++; else fc++;
}

int main() {
    printf("===== Weaving Balancer =====\n\n");
    
    weaving_graph_t wg;
    weaving_init(&wg);
    
    /* 添加4个节点 */
    weaving_add_node(&wg, "a", 0.4f);
    weaving_add_node(&wg, "o", 0.4f);
    weaving_add_node(&wg, "ai", 0.7f);
    weaving_add_node(&wg, "\xc4\x9b", 0.7f); /* ě */
    
    printf("[Nodes added: %u]\n", wg.node_count);
    
    /* 添加编织边 */
    /* a 和 o 互压, 张力0.3 */
    weaving_add_edge(&wg, "a", "o", 0.3f);
    /* a 压 ai, 张力0.5 */
    weaving_add_edge(&wg, "a", "ai", 0.5f);
    /* ě 和 ai 互压, 张力0.2 */
    weaving_add_edge(&wg, "\xc4\x9b", "ai", 0.2f);
    
    printf("[Edges added: %u]\n", wg.edge_count);
    
    /* 计算平衡 */
    weaving_compute_balance(&wg);
    
    printf("\n[Force Analysis]\n");
    for (uint32_t i = 0; i < wg.node_count; i++) {
        weaving_node_t *n = &wg.nodes[i];
        printf("  %-5s  weight=%.1f  out=%.1f  in=%.1f  net=%.1f  %s\n",
               n->name, n->weight, n->pressure_out, n->pressure_in, n->net_force,
               weaving_node_status(n));
    }
    
    printf("\n[Global Balance]\n");
    printf("  Total tension: %.2f\n", wg.total_tension);
    printf("  Balance score: %.2f\n", wg.balance_score);
    printf("  Balanced nodes: %u/%u\n", wg.balanced_nodes, wg.node_count);
    printf("  Stressed nodes: %u\n", wg.stressed_nodes);
    printf("  Is balanced: %s\n", wg.is_balanced ? "YES" : "NO");
    
    uint32_t problems = weaving_diagnose(&wg);
    printf("  Problems found: %u\n", problems);
    
    printf("\n===== RESULT: %d/%d passed, %d failed =====\n", pc, tc, fc);
    return fc > 0 ? 1 : 0;
}
