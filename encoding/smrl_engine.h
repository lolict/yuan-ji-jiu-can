/*
 * smrl_engine.h — SMRL 引擎
 *
 * 64位单元格编码:
 *   坐标(8) | 极性(8) | 主权(8) | 价值(8) | 层级(8) | 链(8) | 版本(16)
 *
 * 几何裁判: 三元见证裁定准入
 *   极性相容 AND 层级兼容 AND 主权不侵 = 准入
 */

#ifndef NJSG_SMRL_ENGINE_H
#define NJSG_SMRL_ENGINE_H

#include <stdint.h>
#include "chain_block.h"

/* 解码后的单元格 */
typedef struct {
    uint8_t     coord;          /* 坐标 0-255 (静态绝对位置) */
    uint8_t     polarity;       /* 极性 (阴阳盈虚表里...) */
    uint8_t     sovereignty;    /* 主权 (0=无主, 255=全主) */
    uint8_t     value;          /* 价值 (0-255) */
    uint8_t     level;          /* 层级 (元/阶/层/代/世/界/域/场) */
    uint8_t     chain_id;       /* 链标识 */
    uint16_t    version;        /* 版本号 */
} cell_t;

/* 编码/解码 */
uint64_t    smrl_encode(uint8_t coord, uint8_t polarity, uint8_t sovereignty,
                        uint8_t value, uint8_t level, uint8_t chain_id,
                        uint16_t version);
cell_t      smrl_decode(uint64_t code);

/* 几何裁判: 三元见证裁定 */
int         smrl_judge(cell_t candidate, cell_t existing);

/* 链 → SMRL 编码 */
int         smrl_encode_chain(const chain_t* chain, uint64_t* codes, int max_codes);

/* 报告 */
void        smrl_report(uint64_t code);
void        smrl_engine_report(void);

#endif /* NJSG_SMRL_ENGINE_H */
