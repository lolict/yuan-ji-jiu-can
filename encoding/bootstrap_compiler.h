/*
 * bootstrap_compiler.h — 自举编译器
 *
 * 用自己的规则从文件系统长出工具。
 *
 * 核心原理:
 * 1. 读取 njsg 规则集 (符号链、层级规则、编码定义)
 * 2. 生成独立的 C 源码，将所有规则硬编码为常量数据
 * 3. 生成的代码不依赖任何外部文件
 * 4. 生成的代码可以自身再生成自己 (自举)
 *
 * 这就是"避免被他人规则绑架"的实现:
 * 编译产物不依赖系统头文件、不链接外部库，
 * 只依赖自己的链条和版本号。
 */

#ifndef NJSG_BOOTSTRAP_COMPILER_H
#define NJSG_BOOTSTRAP_COMPILER_H

#include <stdio.h>
#include <stdint.h>
#include "chain_block.h"
#include "memory_chain.h"

/* 编译目标 */
typedef enum {
    TARGET_STANDALONE_C = 0,    /* 独立 C 文件 */
    TARGET_MINIMAL_C    = 1,    /* 最小 C 文件 (无扩展符号) */
    TARGET_SELF_REPL   = 2,    /* 自复制格式 */
} compile_target_t;

/* 编译配置 */
typedef struct {
    compile_target_t    target;         /* 编译目标 */
    int                 include_memory; /* 是否包含记忆链 */
    int                 include_growth; /* 是否包含生长环 */
    int                 compress;       /* 压缩符号名 */
    int                 add_header;     /* 添加自举注释头 */
    char                version[32];    /* 输出版本 */
    char                output_path[512];
} compile_config_t;

/* 编译产物 */
typedef struct {
    char*       source;         /* 生成的源码 */
    size_t      source_len;     /* 源码长度 */
    uint32_t    rule_count;     /* 内置规则数 */
    uint32_t    symbol_count;   /* 内置符号数 */
    uint32_t    chain_count;    /* 引用的链数 */
    uint64_t    timestamp;      /* 生成时间 */
    char        self_hash[64];  /* 内容哈希 (指纹) */
} compile_output_t;

/* 编译操作 */
int 编译器_初始化(compile_config_t* cfg, compile_target_t target);
int 编译器_编译链(const compile_config_t* cfg, const chain_t* chain,
                   compile_output_t* out);
int 编译器_编译记忆链(const compile_config_t* cfg, const memory_chain_t* mem,
                       compile_output_t* out);
int 编译器_全编译(const compile_config_t* cfg, 
                   const chain_t* chains, uint32_t chain_count,
                   compile_output_t* out);

/* 输出 */
int 编译器_写文件(const compile_output_t* out, const char* path);
void 编译器_释放(compile_output_t* out);
void 编译器_报告(const compile_output_t* out);

#endif /* NJSG_BOOTSTRAP_COMPILER_H */
