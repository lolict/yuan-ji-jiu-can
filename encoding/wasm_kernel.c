/**
 * wasm_kernel.c — WASM 内核实现
 *
 * 满全法的 WASM 灵魂内核:
 *   64位 cell 编码 + 囡囝四态状态机 + 八流路由
 *
 * 阶梯制·双向公约:
 *   同一份源码在 wasm32/wasm64 上编译后行为一致
 *   所有宽度敏感操作通过 NJSG_* 类型进行
 */

#include "wasm_kernel.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

NJSG_EXPORT void wk_init(wk_kernel_t *kernel, NJSG_uint8 *mem_base, NJSG_uint32 mem_cap) {
    if (!kernel) return;
    
    memset(kernel, 0, sizeof(*kernel));
    kernel->magic   = WK_MAGIC;
    kernel->version = WK_VERSION;
    kernel->state   = WK_STATE_NJSG;  /* 默认平等态 */
    
    kernel->mem_base     = mem_base;
    kernel->mem_size     = 0;
    kernel->mem_capacity = mem_cap;
    
    kernel->symbol_count = 0;
    
    /* 八流全部初始化为休眠 */
    for (int i = 0; i < WK_FLOW_MAX; i++) {
        kernel->flow_state[i] = 0;
    }
    
    kernel->total_cells     = 0;
    kernel->total_routes    = 0;
    kernel->total_judgments = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * Cell 操作
 * ═══════════════════════════════════════════════════════════════ */

NJSG_EXPORT int wk_add_cell(wk_kernel_t *kernel, NJSG_uint64 cell) {
    if (!kernel || kernel->magic != WK_MAGIC) return -1;
    if (kernel->symbol_count >= WK_SYMBOL_MAX) return -1;
    
    /* 检查是否已存在 (幂等) */
    for (NJSG_uint16 i = 0; i < kernel->symbol_count; i++) {
        if (kernel->symbols[i] == cell) return 0;
    }
    
    kernel->symbols[kernel->symbol_count++] = cell;
    kernel->total_cells++;
    return 0;
}

NJSG_EXPORT NJSG_uint64 wk_find_by_mccp(const wk_kernel_t *kernel, NJSG_uint32 mccp) {
    if (!kernel || kernel->magic != WK_MAGIC) return 0;
    
    for (NJSG_uint16 i = 0; i < kernel->symbol_count; i++) {
        if (WK_CELL_MCP(kernel->symbols[i]) == mccp) {
            return kernel->symbols[i];
        }
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 状态机 (囡囝四态)
 * ═══════════════════════════════════════════════════════════════ */

NJSG_EXPORT int wk_set_state(wk_kernel_t *kernel, wk_state_t new_state) {
    if (!kernel || kernel->magic != WK_MAGIC) return -1;
    if (new_state > WK_STATE_JN) return -1;
    
    /* 四态自由切换 — 唯二条件:
     *   1. 双方同 sign
     *   2. 锚点 hash 不变
     * 在 WASM 内核中, 这些由上层协议保证 */
    kernel->state = (NJSG_uint8)new_state;
    
    /* 状态切换影响极性语义 */
    switch (new_state) {
        case WK_STATE_NN:   /* 妻壳 → wife polarity 主导 */
        case WK_STATE_NJSG: /* 平等 → 双方共存 */
        case WK_STATE_JJ:   /* 夫壳 → husband polarity 主导 */
        case WK_STATE_JN:   /* 夫壳妻进入 → 进入语义 */
        default:
            break;
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 路由判决 (三层分别器简化版)
 * ═══════════════════════════════════════════════════════════════ */

NJSG_EXPORT int wk_route(wk_kernel_t *kernel, NJSG_uint64 symbol, wk_flow_t flow_type) {
    if (!kernel || kernel->magic != WK_MAGIC) return -1;
    if (flow_type >= WK_FLOW_MAX) return -1;
    
    kernel->total_routes++;
    
    NJSG_uint32 mccp = WK_CELL_MCP(symbol);
    if (mccp == 0) return -1;  /* 无效码点 */
    
    /* 第0层: 感知家族 — 检查 MCCP 是否在符号表 */
    NJSG_uint64 found = wk_find_by_mccp(kernel, mccp);
    if (found == 0) {
        /* 未登记符号 · 如果内核未锁定则放行 · 锁定后拒绝 */
        if (!kernel->locked) return 0;
        kernel->total_judgments++;
        return -2;  /* DENY: 未登记 */
    }
    
    /* 第1层: 谐音靶点 — 主权匹配 */
    NJSG_uint32 req_sov  = WK_CELL_SOVEREIGN(symbol);
    NJSG_uint32 found_sov = WK_CELL_SOVEREIGN(found);
    
    if (req_sov != found_sov && kernel->locked) {
        /* 主权不匹配 · 检查极性容忍 */
        NJSG_uint8 req_pol  = WK_CELL_POLARITY(symbol);
        NJSG_uint8 found_pol = WK_CELL_POLARITY(found);
        
        /* 系统极性可穿透 */
        if (req_pol != WK_POLARITY_SYSTEM && found_pol != WK_POLARITY_SYSTEM) {
            kernel->total_judgments++;
            return -3;  /* DENY: 主权不符 */
        }
    }
    
    /* 第2层: 主权路由 — 流状态检查 */
    if (kernel->flow_state[flow_type] == 0 && kernel->locked) {
        /* 该流未激活 · 拒绝 */
        kernel->total_judgments++;
        return -4;  /* DENY: 流休眠 */
    }
    
    /* 通过 */
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 内核统计
 * ═══════════════════════════════════════════════════════════════ */

NJSG_EXPORT void wk_stats(const wk_kernel_t *kernel, char *buf, NJSG_size buf_size) {
    if (!kernel || !buf || buf_size == 0) return;
    
    const char *state_names[] = {"囡囡", "囡囝", "囝囝", "囝囡"};
    const char *state_name = (kernel->state <= WK_STATE_JN) 
        ? state_names[kernel->state] : "?";
    
    /* 手动格式化 (不依赖 sprintf · WASM 友好) */
    NJSG_size pos = 0;
    
    /* 简单的字符串辅助 */
    #define WK_APPEND(s) do { \
        const char *_p = (s); \
        while (*_p && pos < buf_size - 1) buf[pos++] = *_p++; \
    } while(0)
    
    WK_APPEND("WASM Kernel v");
    buf[pos++] = '0' + (kernel->version >> 8);
    buf[pos++] = '.';
    buf[pos++] = '0' + (kernel->version & 0xFF);
    buf[pos++] = '\n';
    
    WK_APPEND("State: ");
    WK_APPEND(state_name);
    WK_APPEND("  Cells: ");
    
    /* 数字转字符串 */
    char num[12];
    NJSG_uint32 vals[4] = {kernel->symbol_count, kernel->total_cells, kernel->total_routes, kernel->total_judgments};
    const char *labels[] = {"", "/", "/", "/"};
    for (int i = 0; i < 4; i++) {
        WK_APPEND(labels[i]);
        int n = 11;
        NJSG_uint32 v = vals[i];
        if (v == 0) {
            num[--n] = '0';
        } else {
            while (v > 0 && n > 0) {
                num[--n] = '0' + (v % 10);
                v /= 10;
            }
        }
        for (int j = n; j < 11; j++) buf[pos++] = num[j];
        if (pos < buf_size - 1) buf[pos++] = ' ';
    }
    if (pos > 0 && buf[pos-1] == ' ') pos--;
    
    buf[pos] = '\0';
    
    #undef WK_APPEND
}
