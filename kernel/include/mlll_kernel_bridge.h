/*
 * mlll_kernel_bridge.h — MLLL 七维符号 ↔ 囡囝共生内核 编码桥接
 *
 * 内核 njsg_encode() 产生的身份 = 声母/韵母/声调/笔画 → hash
 * MLLL 七维符号             = 笔画/声调/部首/结构/几何/颜色/相位
 *
 * 桥接法则：
 *   - 声调: 内核 tone_idx[0..4] ↔ mlll SYM_TONE property_a
 *   - 笔画: 内核 stroke_count[1..30] ↔ mlll SYM_STROKE property_a
 */

#ifndef MLLL_KERNEL_BRIDGE_H
#define MLLL_KERNEL_BRIDGE_H

#include "mlll_symbols.h"

/* 由内核身份推导 MLLL 坐标（能映射的维度） */
void mlll_from_kernel_identity(const void* kernel_id, char_prop_t* out);

/* MLLL 坐标 → 内核身份维度 */
void mlll_to_kernel_identity(const char_prop_t* cp, int* init_idx, int* final_idx, int* tone_idx, int* stroke_count);

/* 打印编码对照表 */
void mlll_print_encoding_bridge(const char* text);

#endif
