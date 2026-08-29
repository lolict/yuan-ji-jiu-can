/*
 * symbol_table.h — 符号表的FFI接口
 *
 * 给 Zig / Nim / Rust 通过FFI调用
 * 功能固定，符号只定义一次
 */

#ifndef NJSG_SYMBOL_TABLE_H
#define NJSG_SYMBOL_TABLE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 符号 → 地址。返回 -1 表示不存在。 */
int 查地址(const char* 符号);

/* 地址 → 符号。返回 NULL 表示地址无效。 */
const char* 查符号(int 地址);

/* 符号是否存在。返回 1=存在, 0=不存在。 */
int 存在(const char* 符号);

/* 符号总数 */
int 符号数量(void);

/* 族起始地址：族编号 0=声母, 1=韵母, 2=声调, 3=容器, 4=操作, 5=极性, 6=层级, 7=审判, 8=产权 */
int 族起始(int 族编号);

/* 族内符号数量 */
int 族数量(int 族编号);

#ifdef __cplusplus
}
#endif

#endif /* NJSG_SYMBOL_TABLE_H */
