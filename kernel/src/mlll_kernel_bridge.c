/*
 * mlll_kernel_bridge.c — MLLL ↔ 囡囝共生内核 编码桥接
 *
 * 编码桥接四维 ↔ 八维对应法则：
 *   内核区位          MLLL 维度         映射关系
 *   ─────────────────────────────────────────────
 *   init_idx(声母)   → SYM_RADICAL       声母号 → 康熙部首 (声母与部首同源)
 *   final_idx(韵母)  → SYM_VOWEL         韵母号 → mlll 韵母 (直接 1:1)
 *   tone_idx(声调)   → SYM_TONE          声调号 → mlll 8 声调
 *   stroke_count(笔画) → SYM_STROKE       笔画数 → mlll 1-64 画
 *
 * 额外 MLLL 独有维度 (无内核对应，mlll 自描述)：
 *   SYM_GEOMETRY SYM_COLOR SYM_PHASE SYM_STRUCTURE
 *
 * 总效果：内核四字身份 在 mlll 时间线上有唯一八维坐标
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "../include/mlll_symbols.h"
#include "../include/mlll_kernel_bridge.h"

/* 提取内核身份中的 MLLL 可映射字段（完整 8 维） */
void mlll_from_kernel_identity(const void* kernel_id, char_prop_t* out) {
    const uint8_t* p = (const uint8_t*)kernel_id;
    const int* fields = (const int*)(p + 128);
    int _init   = fields[0];   /* 0..24 声母 */
    int _final  = fields[1];   /* 0..38 韵母 */
    int _tone   = fields[2];   /* 0..4  声调 */
    int _stroke = fields[3];   /* 1..30 笔画 */
    out->stroke    = _stroke;
    out->tone      = _tone % 8;
    out->radical   = (_init % 141) + 1;
    out->structure = _final % 8;
    out->vowel     = _final % 46;  /* 韵母直接映射到 SYM_VOWEL */
}

/* 反向：MLLL 八维坐标推算回内核身份字段 */
void mlll_to_kernel_identity(const char_prop_t* cp, int* init_idx, int* final_idx, int* tone_idx, int* stroke_count) {
    *stroke_count = (cp->stroke < 1) ? 1 : (cp->stroke > 30 ? 30 : cp->stroke);
    *tone_idx     = cp->tone % 5;
    *init_idx     = (cp->radical - 1) % 24;
    *final_idx    = cp->vowel % 39;  /* 韵母由 vowel 字段还原 */
}

/* 打印完整的编码对照（8维） */
void mlll_print_encoding_bridge(const char* text) {
    uint32_t seed = 0;
    for (size_t i = 0; text[i]; i++)
        seed = seed * 31 + (unsigned char)text[i];
    int init_idx   = seed % 24;
    int final_idx  = (seed >> 5) % 39;
    int tone_idx   = (seed >> 10) % 5;
    int stroke_cnt = (seed >> 15) % 30 + 1;
    printf("\n编码桥接: '%s' → 内核身份\n", text);
    printf("  内核4维: init_idx=%d final_idx=%d tone_idx=%d stroke=%d\n",
           init_idx, final_idx, tone_idx, stroke_cnt);
    /* MLLL 八维坐标 */
    char_prop_t cp;
    cp.stroke    = stroke_cnt;
    cp.tone      = tone_idx;
    cp.radical   = (init_idx % 141) + 1;
    cp.structure = final_idx % 8;
    cp.vowel     = final_idx % 46;
    printf("  MLLL八维: stroke=%d tone=%d radical=%d structure=%d vowel=%d\n",
           cp.stroke, cp.tone, cp.radical, cp.structure, cp.vowel);
    /* 韵母符号 */
    mlll_init(0.1);
    mlll_symbol_t *vw_sym = NULL;
    if (cp.vowel >= 0 && cp.vowel < 46)
        vw_sym = &REG.entries[833 + cp.vowel];
    printf("  韵母符号: %s (vowel_%d)\n",
           vw_sym ? vw_sym->name : "(null)", cp.vowel);
    /* 时间线定位 */
    mlll_symbol_t *st_sym = mlll_cross_query(SYM_STROKE, cp.stroke, SYM_STROKE);
    mlll_symbol_t *tn_sym = mlll_cross_query(SYM_TONE, cp.tone, SYM_TONE);
    uint32_t timeline_pos = 587 + 8 + cp.tone;
    uint32_t opposite = (timeline_pos + (REG.count / 2)) % REG.count;
    printf("  时间线: stroke=%s tone=%s vowel=%s\n",
           st_sym ? st_sym->name : "(null)",
           tn_sym ? tn_sym->name : "(null)",
           vw_sym ? vw_sym->name : "(null)");
    printf("  对径点(夫妻命定): pos_%u ↔ pos_%u (共%u符号,半圆%u)\n",
           timeline_pos, opposite, REG.count, REG.count / 2);
}
