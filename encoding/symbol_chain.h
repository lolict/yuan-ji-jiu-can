/**
 * symbol_chain.h — 符号链声明
 *
 * 每个 .c/.h 文件通过包含此头文件来声明:
 *   1. 自己属于哪个容器
 *   2. 自己产出的符号列表
 *
 * 这是全局互锁的"链式传导"机制:
 *   横轴 (global_interlock) ← 齿轮 (容器) ← 圆珠 (符号) ← 链环 (文件)
 *
 * 用法 (在 enum 定义之后):
 *   #include "global_interlock.h"
 *   #include "symbol_chain.h"
 *
 *   SYMBOL_CHAIN_DECLARE(my_container_id);
 *   SYMBOL_CHAIN_EXPORT(0x01, 0x02, 0x03);  // 本文件产出的符号
 */

#ifndef NJSG_SYMBOL_CHAIN_H
#define NJSG_SYMBOL_CHAIN_H

/* ── 文件声明宏 ── */

/* 声明本文件属于哪个容器 */
#define SYMBOL_CHAIN_DECLARE(container_id) \
    static const uint8_t _sym_chain_container = (container_id);

/* 声明本文件产出的符号 (最多 16 个) */
#define SYMBOL_CHAIN_EXPORT(...) \
    static const uint32_t _sym_chain_symbols[] = { __VA_ARGS__ }; \
    static const uint8_t _sym_chain_symbol_count = \
        (sizeof(_sym_chain_symbols) / sizeof(_sym_chain_symbols[0]));

/* 声明本文件依赖的符号 (不产出, 只使用) */
#define SYMBOL_CHAIN_DEPEND(...) \
    static const uint32_t _sym_chain_depends[] = { __VA_ARGS__ }; \
    static const uint8_t _sym_chain_depend_count = \
        (sizeof(_sym_chain_depends) / sizeof(_sym_chain_depends[0]));

/* ── 链环注册 (在文件加载时调用) ── */
/* 注意: 需要先调用 SYMBOL_CHAIN_DECLARE 才能使用 */

#define symbol_chain_register(filename) \
    interlock_declare_file(filename, _sym_chain_container)

#define symbol_chain_verify(filename) \
    interlock_check_file_symbols(filename)

#endif /* NJSG_SYMBOL_CHAIN_H */
