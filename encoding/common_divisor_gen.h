/**
 * common_divisor_gen.h — 公约数生成器
 *
 * 设计:
 *   用最小符号(公约数)的循环拉开层级关系
 *   不需要新表头文件 · 不需要新标记符
 *   通过秘制循环无限生成容器
 *
 * 公式: 公约数容器 = 公约数符号 × 层级的幂次方
 *   一个公约数表头 → 无限公倍数容器
 *
 * 内存开销:
 *   每个容器 = 4字节 (一个uint32_t编码)
 *   表头本身 = 几个常量 · 不随容器数增长
 *   生成1000个容器 = 4000字节 ≈ 4KB
 */

#ifndef NJSG_COMMON_DIVISOR_GEN_H
#define NJSG_COMMON_DIVISOR_GEN_H

#include <stdint.h>

#define CD_MAX_SYMBOLS   48   /* 最大公约数符号数 (一声母一符号) */
#define CD_MAX_POWER     8    /* 最大幂次 (2^8 = 256) */

/* 公约数符号 (最小单位 · 公共用 · 非私用) */
typedef struct {
    uint8_t   symbol;        /* 符号索引 */
    uint8_t   power;         /* 幂次 */
    uint32_t  common_val;    /* 公倍数值 = symbol × 2^power */
} cd_symbol_t;

/* 公约数生成器 */
typedef struct {
    cd_symbol_t  symbols[CD_MAX_SYMBOLS];
    uint32_t     count;
    uint32_t     base;       /* 进制约数 (如48声母) */
} cd_generator_t;

/**
 * 初始化公约数生成器
 */
void cd_init(cd_generator_t *gen, uint32_t base);

/**
 * 通过公约数生成一个容器编码
 *   编码 = symbol × (base ^ power)
 */
uint32_t cd_generate(cd_generator_t *gen, uint8_t symbol, uint8_t power);

/**
 * 批量生成到缓冲区
 *   返回: 生成的数量
 */
uint32_t cd_generate_batch(cd_generator_t *gen, uint32_t *out, uint32_t max);

#endif /* NJSG_COMMON_DIVISOR_GEN_H */
