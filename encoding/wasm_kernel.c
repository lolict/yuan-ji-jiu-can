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
#include <stdio.h>
#include <stdlib.h>

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

/* ═══════════════════════════════════════════════════════════════
 * 持久化 (保存/加载)
 * ═══════════════════════════════════════════════════════════════ */

/* 文件格式:
 *   offset  size    内容
 *   0       4       魔数 (0x574B5356 = "WKSV")
 *   4       2       版本
 *   6       2       保留
 *   8       4       CRC32 (覆盖 payload)
 *   12      4       payload 大小
 *   16      N       payload (内核状态 · 见下方)
 *
 * payload 布局:
 *   0       1       state (囡囝四态)
 *   1       1       active/locked 标志
 *   2       2       保留
 *   4       4       symbol_count
 *   8       4       total_cells
 *   12      4       total_routes
 *   16      4       total_judgments
 *   20      8       symbol_count 个 cell (每个 8 字节)
 */

#define WK_SAVE_MAGIC   0x574B5356  /* "WKSV" */
#define WK_SAVE_VERSION 0x0001
#define WK_SAVE_HDR_SIZE 16

/* 简单 CRC32 */
static NJSG_uint32 wk_crc32(const NJSG_uint8 *data, NJSG_uint32 len) {
    NJSG_uint32 crc = 0xFFFFFFFF;
    for (NJSG_uint32 i = 0; i < len; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            crc = (crc >> 1) ^ (0xEDB88320 & (-(crc & 1)));
        }
    }
    return crc ^ 0xFFFFFFFF;
}

NJSG_EXPORT NJSG_uint32 wk_serialized_size(void) {
    /* header + payload */
    return WK_SAVE_HDR_SIZE + 20 + WK_SYMBOL_MAX * sizeof(NJSG_uint64);
}

NJSG_EXPORT int wk_save(const wk_kernel_t *kernel, const char *filename) {
    if (!kernel || !filename || kernel->magic != WK_MAGIC) return -1;
    
    FILE *fp = fopen(filename, "wb");
    if (!fp) return -1;
    
    /* 计算 payload 大小 */
    NJSG_uint32 payload_size = 20 + kernel->symbol_count * sizeof(NJSG_uint64);
    
    /* 分配 payload 缓冲区 */
    NJSG_uint8 *payload = (NJSG_uint8 *)malloc(payload_size);
    if (!payload) { fclose(fp); return -1; }
    
    /* 填充 payload */
    payload[0] = kernel->state;
    payload[1] = (kernel->active ? 0x01 : 0) | (kernel->locked ? 0x02 : 0);
    payload[2] = 0;  /* 保留 */
    payload[3] = 0;
    memcpy(payload + 4, &kernel->symbol_count, 4);
    memcpy(payload + 8, &kernel->total_cells, 4);
    memcpy(payload + 12, &kernel->total_routes, 4);
    memcpy(payload + 16, &kernel->total_judgments, 4);
    
    /* 复制符号表 */
    if (kernel->symbol_count > 0) {
        memcpy(payload + 20, kernel->symbols, kernel->symbol_count * sizeof(NJSG_uint64));
    }
    
    /* 计算 CRC */
    NJSG_uint32 crc = wk_crc32(payload, payload_size);
    
    /* 写入文件 */
    NJSG_uint32 magic = WK_SAVE_MAGIC;
    NJSG_uint16 version = WK_SAVE_VERSION;
    NJSG_uint16 reserved = 0;
    
    fwrite(&magic, 4, 1, fp);
    fwrite(&version, 2, 1, fp);
    fwrite(&reserved, 2, 1, fp);
    fwrite(&crc, 4, 1, fp);
    fwrite(&payload_size, 4, 1, fp);
    fwrite(payload, 1, payload_size, fp);
    
    free(payload);
    fclose(fp);
    return 0;
}

NJSG_EXPORT int wk_load(wk_kernel_t *kernel, const char *filename) {
    if (!kernel || !filename || kernel->magic != WK_MAGIC) return -1;
    
    FILE *fp = fopen(filename, "rb");
    if (!fp) return -1;
    
    /* 读取头部 */
    NJSG_uint32 magic, crc, payload_size;
    NJSG_uint16 version, reserved;
    
    if (fread(&magic, 4, 1, fp) != 1 || magic != WK_SAVE_MAGIC) goto fail;
    if (fread(&version, 2, 1, fp) != 1 || version != WK_SAVE_VERSION) goto fail;
    if (fread(&reserved, 2, 1, fp) != 1) goto fail;
    if (fread(&crc, 4, 1, fp) != 1) goto fail;
    if (fread(&payload_size, 4, 1, fp) != 1) goto fail;
    
    /* 限制 payload 大小 */
    if (payload_size > wk_serialized_size()) goto fail;
    
    /* 读取 payload */
    NJSG_uint8 *payload = (NJSG_uint8 *)malloc(payload_size);
    if (!payload) goto fail;
    if (fread(payload, 1, payload_size, fp) != payload_size) {
        free(payload);
        goto fail;
    }
    fclose(fp);
    
    /* 验证 CRC */
    NJSG_uint32 calc_crc = wk_crc32(payload, payload_size);
    if (calc_crc != crc) { free(payload); return -1; }
    
    /* 恢复状态 */
    kernel->state = payload[0];
    kernel->active = (payload[1] & 0x01) ? 1 : 0;
    kernel->locked = (payload[1] & 0x02) ? 1 : 0;
    memcpy(&kernel->symbol_count, payload + 4, 4);
    memcpy(&kernel->total_cells, payload + 8, 4);
    memcpy(&kernel->total_routes, payload + 12, 4);
    memcpy(&kernel->total_judgments, payload + 16, 4);
    
    /* 恢复符号表 */
    if (kernel->symbol_count > 0 && kernel->symbol_count <= WK_SYMBOL_MAX) {
        memcpy(kernel->symbols, payload + 20, kernel->symbol_count * sizeof(NJSG_uint64));
    }
    
    free(payload);
    return 0;
    
fail:
    fclose(fp);
    return -1;
}
