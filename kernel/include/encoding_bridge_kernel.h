/*
 * encoding_bridge_kernel.h — 编码系统 → 内核桥接
 *
 * 把编码系统的感知结果(家族/极性/流属性)注入内核决策链
 *
 * 内核原本只认识 char_prop_t (笔画/声调/部首/结构/韵母/声母)
 * 编码系统额外提供: 家族归属(FIVE_FAMILY), 极性(YinYang), 12位流属性
 *
 * 桥接法则:
 *   编码 container_code[31:29] family  → 内核路由策略 (容器/边界/夫妻/网状/中性)
 *   编码 container_code[28]   polarity  → YinYang 判向 (0=刘楚恬侧, 1=lolict侧)
 *   编码 container_code[17:16] shengmu  → 声母综合 (用于裁决权重)
 *   编码 container_code[15:0]  yunmu+tone → 韵母+声调 (用于时间线定位)
 *
 * 内核可以利用这些信息:
 *   - 家族路由: 容器→内核核心圈, 边界→裁决算界, 夫妻→协作对等
 *   - 极性判向: 阴性→被动接收, 阳性→主动发起
 *   - 流属性: 决定内核对该请求的响应策略
 */

#ifndef NJSG_ENCODING_BRIDGE_KERNEL_H
#define NJSG_ENCODING_BRIDGE_KERNEL_H

#include <stdint.h>
#include "mlll_symbols.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ── 编码系统初始化 (在使用任何其他函数前调用) ── */
void encoding_bridge_kernel_init(void);

/* ── 编码感知 → 内核 char_prop_t 填充 ── */
void encoding_bridge_to_prop(const char* utf8, char_prop_t* out);

/* ── 仅查家族 (快速路径) ── */
int encoding_bridge_get_family(const char* utf8);  /* 返回 FAMILY_* 枚举 */

/* ── 仅查极性 ── */
int encoding_bridge_get_polarity(const char* utf8); /* 0=YIN, 1=YANG */

/* ── 获取完整 32 位容器编码 ── */
uint32_t encoding_bridge_get_container_code(const char* utf8);

/* ── 12 位流属性 (用于内核裁决策略) ── */
uint32_t encoding_bridge_get_flow_bits(const char* utf8);

/* ── 桥接测试 ── */
void encoding_bridge_kernel_test(const char* text);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_ENCODING_BRIDGE_KERNEL_H */
