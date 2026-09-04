/*
 * ╔══════════════════════════════════════════════════════════════════╗
 * ║  元输入法 MetaIME · C 头文件                                     ║
 * ║  冰箱模型: 编译期内嵌字典 · 零运行时解析                          ║
 * ║                                                                  ║
 * ║  Copyright © 刘楚恬 & lolict (囡囝凹月留)                         ║
 * ║  所有权保留 · 无 MIT / GPL / Apache 许可                          ║
 * ╚══════════════════════════════════════════════════════════════════╝
 *
 * 用法 (Android JNI / C / C++):
 *   MetaimeState *st = metaime_init();
 *   char *result = metaime_query_cstr(st, shengmu, yunmu, tone);
 *   if (result) { ...; metaime_string_free(result); }
 *   metaime_free(st);
 */

#ifndef MQF_METAIME_H
#define MQF_METAIME_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque state handle */
typedef struct MetaimeState MetaimeState;

/* 初始化输入法状态 (返回不可变共享状态,线程安全) */
MetaimeState *metaime_init(void);

/* 释放状态 */
void metaime_free(MetaimeState *state);

/* 查询候选词;返回静态字符串切片 (无需释放);NULL = 未找到 */
const char *metaime_query(MetaimeState *state, uint8_t shengmu, uint8_t yunmu, uint8_t tone);

/* 查询候选词;返回堆分配字符串 (必须用 metaime_string_free 释放);NULL = 未找到 */
char *metaime_query_cstr(MetaimeState *state, uint8_t shengmu, uint8_t yunmu, uint8_t tone);

/* 释放 metaime_query_cstr 返回的字符串 */
void metaime_string_free(char *s);

#ifdef __cplusplus
}
#endif

#endif /* MQF_METAIME_H */
