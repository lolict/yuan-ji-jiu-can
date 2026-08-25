/**
 * test_wasm_kernel.c — WASM 内核测试
 *
 * 验证:
 *   1. 初始化
 *   2. Cell 编码/解码
 *   3. 囡囝四态切换
 *   4. 路由判决
 *   5. 跨架构类型安全
 */

#include "wasm_kernel.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    printf("═══ WASM 内核测试 ═══\n");
    printf("目标: %s, 指针宽度: %d-bit\n", NJSG_WASM_STR, NJSG_PTR_WIDTH);
    
    /* ── 1. 初始化 ── */
    wk_kernel_t kernel;
    NJSG_uint8 mem[4096];
    wk_init(&kernel, mem, sizeof(mem));
    
    printf("\n[1] 初始化\n");
    printf("  magic: 0x%08X (expected 0x%08X) %s\n",
           kernel.magic, WK_MAGIC, kernel.magic == WK_MAGIC ? "OK" : "FAIL");
    printf("  state: 0x%02X (NJSG平等态) %s\n",
           kernel.state, kernel.state == WK_STATE_NJSG ? "OK" : "FAIL");
    printf("  mem_cap: %u bytes\n", kernel.mem_capacity);
    
    /* ── 2. Cell 编码 ── */
    printf("\n[2] 64位 Cell 编码\n");
    
    /* 构造一个 cell: MCCP=0x12345678, sovereignty=0xABCDE, polarity=WIFE, witness=3 */
    NJSG_uint64 cell = WK_CELL(0x12345678, 0xABCDE, WK_POLARITY_WIFE, 3);
    printf("  cell = 0x%016llX\n", (unsigned long long)cell);
    printf("  MCCP:       0x%08X (expected 0x12345678) %s\n",
           WK_CELL_MCP(cell), WK_CELL_MCP(cell) == 0x12345678 ? "OK" : "FAIL");
    printf("  sovereignty: 0x%05X (expected 0x0ABCDE) %s\n",
           WK_CELL_SOVEREIGN(cell), WK_CELL_SOVEREIGN(cell) == 0xABCDE ? "OK" : "FAIL");
    printf("  polarity:   %u (WIFE=0) %s\n",
           WK_CELL_POLARITY(cell), WK_CELL_POLARITY(cell) == 0 ? "OK" : "FAIL");
    printf("  witness:    %u (expected 3) %s\n",
           WK_CELL_WITNESS(cell), WK_CELL_WITNESS(cell) == 3 ? "OK" : "FAIL");
    
    /* ── 3. 符号表操作 ── */
    printf("\n[3] 符号表\n");
    int rc = wk_add_cell(&kernel, cell);
    printf("  add cell: %s (rc=%d)\n", rc == 0 ? "OK" : "FAIL", rc);
    
    /* 添加更多 cell */
    NJSG_uint64 cell2 = WK_CELL(0x87654321, 0xFEDCB, WK_POLARITY_HUSBAND, 7);
    wk_add_cell(&kernel, cell2);
    printf("  symbol_count: %u (expected 2)\n", kernel.symbol_count);
    
    /* 查找 */
    NJSG_uint64 found = wk_find_by_mccp(&kernel, 0x12345678);
    printf("  find 0x12345678: 0x%016llX %s\n",
           (unsigned long long)found, found == cell ? "OK" : "FAIL");
    
    /* 未找到 */
    NJSG_uint64 notfound = wk_find_by_mccp(&kernel, 0x99999999);
    printf("  find 0x99999999: 0x%016llX (expected 0) %s\n",
           (unsigned long long)notfound, notfound == 0 ? "OK" : "FAIL");
    
    /* ── 4. 囡囝四态 ── */
    printf("\n[4] 囡囝四态\n");
    const char *state_names[] = {"囡囡(NN)", "囡囝(NJSG)", "囝囝(JJ)", "囝囡(JN)"};
    for (int s = 0; s < 4; s++) {
        rc = wk_set_state(&kernel, (wk_state_t)s);
        printf("  set %s: state=0x%02X rc=%d %s\n",
               state_names[s], kernel.state, rc,
               kernel.state == (NJSG_uint8)s ? "OK" : "FAIL");
    }
    
    /* ── 5. 路由判决 ── */
    printf("\n[5] 路由判决\n");
    
    /* 未锁定 — 未登记符号也放行 */
    kernel.locked = 0;
    rc = wk_route(&kernel, WK_CELL(0xDEADBEEF, 0, 0, 0), WK_FLOW_PRELOAD);
    printf("  未锁定 + 未登记符号: rc=%d (expected 0=ALLOW) %s\n",
           rc, rc == 0 ? "OK" : "FAIL");
    
    /* 锁定后 — 未登记符号拒绝 */
    kernel.locked = 1;
    rc = wk_route(&kernel, WK_CELL(0xDEADBEEF, 0, 0, 0), WK_FLOW_PRELOAD);
    printf("  锁定 + 未登记符号: rc=%d (expected -2=DENY) %s\n",
           rc, rc == -2 ? "OK" : "FAIL");
    
    /* 锁定 + 已登记 + 主权匹配 */
    rc = wk_route(&kernel, cell, WK_FLOW_PRELOAD);
    printf("  锁定 + 已登记 + 主权匹配: rc=%d (expected 0=ALLOW) %s\n",
           rc, rc == 0 ? "OK" : "FAIL");
    
    /* 锁定 + 已登记 + 主权不匹配 */
    rc = wk_route(&kernel, WK_CELL(0x12345678, 0xFFFFF, WK_POLARITY_WIFE, 3), WK_FLOW_PRELOAD);
    printf("  锁定 + 主权不匹配: rc=%d (expected -3=DENY) %s\n",
           rc, rc == -3 ? "OK" : "FAIL");
    
    /* ── 6. 统计 ── */
    printf("\n[6] 统计\n");
    char stats[256];
    wk_stats(&kernel, stats, sizeof(stats));
    printf("  %s\n", stats);
    
    printf("\n═══ 测试完成 ═══\n");
    return 0;
}
