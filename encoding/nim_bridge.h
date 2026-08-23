/**
 * nim_bridge.h — Nim ↔ C 桥接头文件
 * 
 * 定义 Nim 层（声韵调 + 42色 + 镜像）与 C 层（容器框架 + 审判系统）之间的接口。
 * 
 * 使用方式:
 *   1. Nim 端: 用 {.importc, cdecl.} 声明以下函数
 *   2. C 端: 编译时链接此头文件对应的 nim_bridge.c
 *   3. 最终结果: 单个可执行文件，Nim 提供数据，C 提供运算
 */

#ifndef NJSG_NIM_BRIDGE_H
#define NJSG_NIM_BRIDGE_H

#include <stdint.h>
#include "color_system.h"
#include "container_codec.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ═══════════════════════════════════════════════════════════════
 * 桥接 API — 供 Nim 调用
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化整个系统（C 层全套）
 * 返回: 0=成功, -1=失败
 */
int bridge_init(void);

/**
 * 审判一对符号（完整过五门）
 * 返回: pair_verdict_t 的值
 */
int bridge_trial(const char *name_a, const char *name_b);

/**
 * 获取审判结果的字符串描述
 */
const char* bridge_verdict_str(int verdict);

/**
 * 衍射分析单个符号
 * 输出: 写入 buf
 */
void bridge_diffract(const char *name, char *buf, int bufsize);

/**
 * 注册一个声韵调符号到容器框架
 * 参数: 符号名, 类型编码, 层级编码, 字重, 家族
 */
int bridge_register_symbol(const char *name, int type, int level,
                            float weight, int family);

/**
 * 批量注册（从 Nim 数组）
 * 参数: 名字数组, 类型数组, 层级数组, 字重数组, 数量
 */
int bridge_register_batch(const char **names, const int *types,
                           const int *levels, const float *weights,
                           int count);

/**
 * 把所有已注册符号封装进容器框架（1bit/16字）
 * 返回: 使用的字框数
 */
int bridge_seal_all(void);

/**
 * 渲染指定字框到圆筒坐标
 */
int bridge_render_frame(int frame_idx, int *x, int *y, int *z,
                          uint8_t *color, uint8_t *phase_left, uint8_t *phase_right);

/**
 * 获取系统统计报告
 */
void bridge_get_stats(char *buf, int bufsize);

/**
 * 获取总审判数/通过数/拒绝数
 */
void bridge_get_counts(int *total, int *passed, int *rejected);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_NIM_BRIDGE_H */
