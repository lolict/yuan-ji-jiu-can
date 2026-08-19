/**
 * husk_compiler.h — 脱壳编译器
 *
 * 核心隐喻: 谷壳不可进粮仓
 *
 * 旧编程语言符号 = 带壳谷子
 *   壳 = 旧进制 · 私有命名 · 类型系统 · 产权编码  ← 不能进粮仓
 *     └─ 米 = 数学种子 (算法/逻辑/DAG/信号流)        ← 这个留下
 *
 * 脱壳三大原则:
 *   1. 不能带壳进仓 — 任何外部符号必须脱壳后才能进入
 *   2. 不能延迟脱壳 — 延迟 = 给壳发芽的时间 = 未来生根
 *   3. 不混用进制 — 只用我们自己的进制重新编排
 */

#ifndef NJSG_HUSK_COMPILER_H
#define NJSG_HUSK_COMPILER_H

#include <stdint.h>
#include "container_pipeline.h"

/* 壳类型 (旧进制的"骨骼") */
typedef enum {
    HUSK_C_ARITHMETIC    = 0x01,  /* C: int/char/float 十进制壳 */
    HUSK_C_POINTER       = 0x02,  /* C: * & 地址壳 */
    HUSK_CPP_TEMPLATE    = 0x03,  /* C++: template 泛型壳 */
    HUSK_CPP_NAMESPACE   = 0x04,  /* C++: namespace 壳 */
    HUSK_RUST_OWNERSHIP  = 0x05,  /* Rust: borrow/lifetime 壳 */
    HUSK_RUST_TRAIT      = 0x06,  /* Rust: trait 壳 */
    HUSK_ZIG_COMPTIME    = 0x07,  /* Zig: comptime 壳 */
    HUSK_NIM_TYPECLASS   = 0x08,  /* Nim: type class 壳 */
    HUSK_ASM_REGISTER    = 0x09,  /* 汇编: 寄存器命名壳 */
    HUSK_ASCII_ONLY      = 0x0A,  /* 纯 ASCII 符号壳 */
    HUSK_UNICODE_ESCAPE  = 0x0B,  /* \uXXXX 转义壳 */
    HUSK_NUMERIC_LITERAL = 0x0C,  /* 数字字面量壳 */
    HUSK_FLOAT_IEEE      = 0x0D,  /* IEEE 754 浮点壳 */
    HUSK_NONE            = 0x00,  /* 无壳 · 已脱壳 */
} husk_type_t;

/* 脱壳状态 */
typedef enum {
    HUSK_OK           = 0x00,  /* 脱壳成功 · 可入仓 */
    HUSK_DETECTED     = 0x01,  /* 检测到壳 · 需脱壳 */
    HUSK_REMOVED      = 0x02,  /* 壳已剥离 · 种子已提取 */
    HUSK_REJECTED     = 0x03,  /* 脱壳失败 · 拒绝入仓 */
    HUSK_STILL_HUSK   = 0x04,  /* 脱壳不净 · 仍有壳残留 */
} husk_status_t;

/* 脱壳结果 */
typedef struct {
    husk_status_t  status;         /* 脱壳状态 */
    husk_type_t    husk_type;      /* 壳类型 */
    uint32_t       seed_code;      /* 数学种子编码 */
    uint32_t       container_code; /* 容器编码 */
    int            husk_bytes;     /* 壳占用的字节数 */
    int            seed_bytes;     /* 种子占用的字节数 */
} husk_result_t;

/* 数学种子类型 */
typedef enum {
    SEED_ALGORITHM      = 0x01,  /* 算法逻辑 */
    SEED_CONTROL_FLOW   = 0x02,  /* 控制流 */
    SEED_DATA_STRUCTURE = 0x03,  /* 数据结构 */
    SEED_TYPE_SYSTEM    = 0x04,  /* 类型系统 */
    SEED_MEMORY_MODEL   = 0x05,  /* 内存模型 */
    SEED_CONCURRENCY    = 0x06,  /* 并发原语 */
    SEED_IO             = 0x07,  /* IO/副作用 */
    SEED_ABSTRACTION    = 0x08,  /* 抽象机制 */
    SEED_PATTERN_MATCH  = 0x09,  /* 模式匹配 */
    SEED_ERROR_HANDLING = 0x0A,  /* 错误处理 */
    SEED_COMPILATION    = 0x0B,  /* 编译期计算 */
    SEED_LINKAGE        = 0x0C,  /* 链接/模块 */
} seed_kind_t;

/* API · 脱壳编译器 */

/**
 * 检测符号是否还带着旧进制壳
 */
husk_type_t husk_detect(const char *symbol, int len);

/**
 * 剥离旧进制壳 · 提取数学种子
 */
husk_result_t husk_remove(const char *symbol, int len, husk_type_t husk);

/**
 * 用满全法进制重新编排种子
 */
uint32_t husk_compile(const husk_result_t *result);

/**
 * 完整脱壳流程: 检测 → 剥离 → 编排
 */
uint32_t husk_process(const char *symbol, int len);

/**
 * 检查符号是否已完全脱壳
 */
int husk_is_clean(const char *symbol, int len);

#endif /* NJSG_HUSK_COMPILER_H */
