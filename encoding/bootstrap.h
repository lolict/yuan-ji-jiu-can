/*
 * bootstrap.h — 鼓膜自举程序的FFI接口
 * 
 * 给 Zig / Nim / Rust 通过FFI调用
 */

#ifndef BOOTSTRAP_H
#define BOOTSTRAP_H

#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 获取启动时间 */
time_t 获取启动时间(void);

/* 获取签名 */
const char* 获取签名(void);

/* 符号是否合法 */
int 符号合法(const char* 符号);

/* 获取元规则 */
const char* 获取前缀(void);
const char* 获取边界(void);
const char* 获取未知处理(void);

/* 自检 */
int 自检(void);

/* 初始化 */
int 初始化(void);

/* 打印状态 */
void 打印状态(void);

#ifdef __cplusplus
}
#endif

#endif /* BOOTSTRAP_H */
