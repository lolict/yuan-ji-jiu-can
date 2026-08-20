/**
 * common_divisor_gen.c — 公约数生成器实现
 *
 * 核心: 最小符号 × 幂次方 = 无限容器
 *   不需要新表头 · 几个常量生成万千编码
 */

#include "common_divisor_gen.h"
#include <string.h>

void cd_init(cd_generator_t *gen, uint32_t base) {
    memset(gen, 0, sizeof(*gen));
    gen->base = base;
    gen->count = 0;
}

uint32_t cd_generate(cd_generator_t *gen, uint8_t symbol, uint8_t power) {
    if (power >= CD_MAX_POWER) power = CD_MAX_POWER - 1;

    uint32_t result = symbol;
    /* 幂次展开: result = symbol × (base ^ power) */
    for (uint8_t i = 0; i < power; i++) {
        result *= gen->base;
    }

    /* 记录已生成 */
    if (gen->count < CD_MAX_SYMBOLS) {
        gen->symbols[gen->count].symbol = symbol;
        gen->symbols[gen->count].power = power;
        gen->symbols[gen->count].common_val = result;
        gen->count++;
    }

    return result;
}

uint32_t cd_generate_batch(cd_generator_t *gen, uint32_t *out, uint32_t max) {
    uint32_t generated = 0;
    for (uint8_t s = 0; s < gen->base && generated < max; s++) {
        for (uint8_t p = 0; p < CD_MAX_POWER && generated < max; p++) {
            out[generated++] = cd_generate(gen, s, p);
        }
    }
    return generated;
}
