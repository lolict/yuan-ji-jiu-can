/**
 * antonym_cart.h — 近义词/反义词二元三元集装箱
 *
 * 概念:
 *   二元 = 一对反义词 (二元对立 · 阴阳极性)
 *      · 如: 冷/热, 大/小, 高/低, 快/慢
 *      · 映射回满全法: 阴(0) / 阳(1)
 *      · 装载格式: slot[0]=反义A, slot[1]=反义B
 *
 *   三元 = 正反合 (命题/反题/合题)
 *      · 如: 起点/终点/合一, 过去/现在/未来
 *      · 映射回三元裁判: PERCEIVE / CLASSIFY / DYNAMIC
 *      · 装载格式: slot[0]=正, slot[1]=反, slot[2]=合
 *
 * 近义词 = 同义集装箱 (多个容器指向同一概念)
 *   如: 大/巨/庞 → 同一"大"概念的不同表面
 *   映射: 多个 slot 共享同一 container_code
 *
 * 与 mlll 254+2 的关系:
 *   二元/三元是容器内的排列方式 · 不额外占槽位
 *   一个槽位 = 一个二元组或三元组
 */

#ifndef NJSG_ANTONYM_CART_H
#define NJSG_ANTONYM_CART_H

#include <stdint.h>
#include "mlll_cart.h"

/* ═══════════════════════════════════════════════════════════════
 * 二元集装箱 (反义词对)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t  word_a;       /* 反义A (阴 = 0 侧) */
    uint32_t  word_b;       /* 反义B (阳 = 1 侧) */
    uint8_t   polarity;     /* 0=A是阴B是阳 · 1=A是阳B是阴 */
    uint8_t   reserved;     /* 对齐 */
    uint16_t  strength;     /* 对立强度 (0..65535) */
} binary_cart_t;

/* ═══════════════════════════════════════════════════════════════
 * 三元集装箱 (正反合)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t  positive;     /* 正 = 命题 = PERCEIVE */
    uint32_t  negative;     /* 反 = 反题 = CLASSIFY */
    uint32_t  synthesis;    /* 合 = 合题 = DYNAMIC */
    uint8_t   phase;        /* 当前相位: 0=正 · 1=反 · 2=合 */
    uint8_t   resolved;     /* 是否已裁决 */
    uint16_t  confidence;   /* 裁决置信度 */
} ternary_cart_t;

/* ═══════════════════════════════════════════════════════════════
 * 近义词集装箱 (同义容器组)
 * ═══════════════════════════════════════════════════════════════ */

#define SYNONYM_MAX  8   /* 一个概念最多8个近义词 */

typedef struct {
    uint32_t  concept_id;              /* 概念id (核心锚点) */
    uint32_t  synonyms[SYNONYM_MAX];   /* 近义词容器编码数组 */
    uint8_t   count;                   /* 实际近义词数 */
    uint8_t   surface_type;            /* 表面类型: 模/面/名 */
} synonym_cart_t;

/* ═══════════════════════════════════════════════════════════════
 * 集装箱管理器
 * ═══════════════════════════════════════════════════════════════ */

#define BINARY_MAX   128   /* 最多128个二元组 */
#define TERNARY_MAX  64    /* 最多64个三元组 */
#define SYNONYM_MAX_GROUPS 64  /* 最多64个近义词组 */

typedef struct {
    uint32_t        magic;          /* ANTONYM_CART_MAGIC */
    uint32_t        binary_count;   /* 二元组计数 */
    uint32_t        ternary_count;  /* 三元组计数 */
    uint32_t        synonym_count;  /* 近义词组计数 */
    binary_cart_t   binaries[BINARY_MAX];
    ternary_cart_t  ternaries[TERNARY_MAX];
    synonym_cart_t  synonyms[SYNONYM_MAX_GROUPS];
} antonym_manager_t;

#define ANTONYM_CART_MAGIC  0x414E544D  /* "ANTM" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

void antonym_init(antonym_manager_t *mgr);

/* 创建二元反义词对 · 返回组索引 · -1=满 */
int antonym_create_binary(antonym_manager_t *mgr,
                          uint32_t word_a, uint32_t word_b,
                          uint8_t polarity, uint16_t strength);

/* 创建三元正反合 · 返回组索引 · -1=满 */
int antonym_create_ternary(antonym_manager_t *mgr,
                           uint32_t positive, uint32_t negative, uint32_t synthesis);

/* 创建近义词组 · 返回组索引 · -1=满 */
int antonym_create_synonym(antonym_manager_t *mgr,
                           uint32_t concept_id, uint32_t *words, uint8_t count);

/* 查询: 给定容器编码 · 找其反义词 */
uint32_t antonym_find_opposite(antonym_manager_t *mgr, uint32_t code);

/* 查询: 给定容器编码 · 找其近义词组 */
const synonym_cart_t *antonym_find_synonyms(antonym_manager_t *mgr, uint32_t code);

/* 裁决: 对三元组执行合一 · 返回合成结果 */
uint32_t antonym_resolve(antonym_manager_t *mgr, uint32_t ternary_idx);

#endif /* NJSG_ANTONYM_CART_H */
