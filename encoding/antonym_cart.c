/**
 * antonym_cart.c — 近义词/反义词二元三元集装箱实现
 *
 * 容器化反义词和近义词:
 *   二元反义 = 阴阳极性对 (大/小 = 1/0)
 *   三元正反合 = 三裁判 (PERCEIVE/CLASSIFY/DYNAMIC → 合一)
 *   近义词组 = 多表面指向同一概念 (模/面/名归一)
 */

#include "antonym_cart.h"
#include <string.h>
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void antonym_init(antonym_manager_t *mgr) {
    memset(mgr, 0, sizeof(*mgr));
    mgr->magic = ANTONYM_CART_MAGIC;
}

/* ═══════════════════════════════════════════════════════════════
 * 创建二元反义词对
 *
 * 例: 冷/热 = word_a(冷,阴0) + word_b(热,阳1)
 *     polarity = 0 → word_a是阴, word_b是阳
 * ═══════════════════════════════════════════════════════════════ */

int antonym_create_binary(antonym_manager_t *mgr,
                          uint32_t word_a, uint32_t word_b,
                          uint8_t polarity, uint16_t strength) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return -1;
    if (mgr->binary_count >= BINARY_MAX) return -1;

    binary_cart_t *b = &mgr->binaries[mgr->binary_count];
    b->word_a    = word_a;
    b->word_b    = word_b;
    b->polarity  = polarity ? 1 : 0;
    b->strength  = strength;

    return (int)(mgr->binary_count++);
}

/* ═══════════════════════════════════════════════════════════════
 * 创建三元正反合
 *
 * 例: 过去/现在/未来
 *     positive = 过去 (PERCEIVE · 感知过去)
 *     negative = 未来 (CLASSIFY · 分类预测)
 *     synthesis = 现在 (DYNAMIC · 动力合一)
 * ═══════════════════════════════════════════════════════════════ */

int antonym_create_ternary(antonym_manager_t *mgr,
                           uint32_t positive, uint32_t negative, uint32_t synthesis) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return -1;
    if (mgr->ternary_count >= TERNARY_MAX) return -1;

    ternary_cart_t *t = &mgr->ternaries[mgr->ternary_count];
    t->positive   = positive;
    t->negative   = negative;
    t->synthesis  = synthesis;
    t->phase      = 0;
    t->resolved   = 0;
    t->confidence = 0;

    return (int)(mgr->ternary_count++);
}

/* ═══════════════════════════════════════════════════════════════
 * 创建近义词组
 *
 * 多个容器编码指向同一概念 (不同表面 · 同一骨)
 * 例: 大/巨/庞 → 同一概念 "big"
 * ═══════════════════════════════════════════════════════════════ */

int antonym_create_synonym(antonym_manager_t *mgr,
                           uint32_t concept_id, uint32_t *words, uint8_t count) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return -1;
    if (mgr->synonym_count >= SYNONYM_MAX_GROUPS) return -1;
    if (count == 0 || count > SYNONYM_MAX) return -1;

    synonym_cart_t *s = &mgr->synonyms[mgr->synonym_count];
    s->concept_id = concept_id;
    s->count = count;
    s->surface_type = 0;  /* 模 */

    for (uint8_t i = 0; i < count; i++) {
        s->synonyms[i] = words[i];
    }

    return (int)(mgr->synonym_count++);
}

/* ═══════════════════════════════════════════════════════════════
 * 查找反义词
 *
 * 给定容器编码 · 返回其对立面的容器编码
 * 找不到返回 0
 * ═══════════════════════════════════════════════════════════════ */

uint32_t antonym_find_opposite(antonym_manager_t *mgr, uint32_t code) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return 0;

    for (uint32_t i = 0; i < mgr->binary_count; i++) {
        binary_cart_t *b = &mgr->binaries[i];
        if (b->word_a == code) return b->word_b;
        if (b->word_b == code) return b->word_a;
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查找近义词组
 *
 * 给定容器编码 · 返回它所属的近义词组
 * 找不到返回 0
 * ═══════════════════════════════════════════════════════════════ */

const synonym_cart_t *antonym_find_synonyms(antonym_manager_t *mgr, uint32_t code) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return 0;

    for (uint32_t i = 0; i < mgr->synonym_count; i++) {
        synonym_cart_t *s = &mgr->synonyms[i];
        if (s->concept_id == code) return s;
        for (uint8_t j = 0; j < s->count; j++) {
            if (s->synonyms[j] == code) return s;
        }
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 三元裁决 (合一)
 *
 * 当正反两股力都被纳入 · 合题自然浮现
 * 裁决逻辑: synthesis = (positive & negative) | (positive ^ negative)
 *   即: 共识部分 + 差异融合 = 新的统一体
 *
 * 返回: 合一后的容器编码
 * ═══════════════════════════════════════════════════════════════ */

uint32_t antonym_resolve(antonym_manager_t *mgr, uint32_t ternary_idx) {
    if (!mgr || mgr->magic != ANTONYM_CART_MAGIC) return 0;
    if (ternary_idx >= mgr->ternary_count) return 0;

    ternary_cart_t *t = &mgr->ternaries[ternary_idx];
    if (t->resolved) return t->synthesis;

    /* 合一运算: 共识 + 融合 */
    uint32_t consensus  = t->positive & t->negative;   /* 共同部分 */
    uint32_t difference = t->positive ^ t->negative;   /* 差异部分 */
    uint32_t merged     = consensus | difference;      /* 融合 */

    /* 如果合成为0 · 用正+反直接融合 */
    if (t->synthesis == 0) {
        t->synthesis = merged;
    }
    else {
        /* 已有合成值 · 用异或融合新信息 */
        t->synthesis ^= (merged & FQ_FULL_16);
    }

    t->resolved   = 1;
    t->phase      = 2;              /* 进入合相 */
    t->confidence = FQ_FULL_8;      /* 满置信 */

    return t->synthesis;
}
