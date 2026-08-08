/*
 * mlll_kernel_bridge.c — MLLL ↔ 囡囝共生内核 编码桥接
 *
 * 桥接原理：
 *   内核 njsg_encode 把名字序列化为 {声母, 韵母, 声调, 笔画}
 *   这四个维度里，mlll 已经覆盖了：
 *     - 声调 tone_idx[0..4]  ↔ mlll SYM_TONE (8 个声调坐标)
 *     - 笔画 stroke[1..30]  ↔ mlll SYM_STROKE (1-64 画)
 *
 * 声母、韵母 在 mlll 七维中没有对应，所以先作占位，
 * 等扩展"声音维度"时再接入。
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "../include/mlll_symbols.h"
#include "../include/mlll_kernel_bridge.h"

/* 提取内核身份中的 MLLL 可映射字段 */
void mlll_from_kernel_identity(const void* kernel_id, char_prop_t* out) {
    /* njsg_identity_t 布局：text folowed by init_idx/final_idx/tone_idx/stroke_count */
    const uint8_t* p = (const uint8_t*)kernel_id;
    /* 简化：直接读偏移后的整数阵列 (NJFS_AUTH_LEN=128, 之后是4个int) */
    const int* fields = (const int*)(p + 128);
    int _init = fields[0];   /* 0..23 */
    int _final = fields[1];  /* 0..38 */
    int _tone = fields[2];   /* 0..4 */
    int _stroke = fields[3]; /* 1..30 */
    /* 映射 */
    out->stroke    = _stroke;         /* 直接 */
    out->tone      = _tone % 8;       /* mlll 8 声调内归一化 */
    out->radical   = (_init % 141) + 1; /* 声母号 → 康熙部首 1..141 */
    out->structure = _final % 8;      /* 韵母号 → 字形结构 0..7 */
}

/* 反向：MLLL 坐标推算回内核身份字段 */
void mlll_to_kernel_identity(const char_prop_t* cp, int* init_idx, int* final_idx, int* tone_idx, int* stroke_count) {
    *stroke_count = (cp->stroke < 1) ? 1 : (cp->stroke > 30 ? 30 : cp->stroke);
    *tone_idx     = cp->tone % 5;          /* kernel 只有 5 声 */
    *init_idx     = (cp->radical - 1) % 24;
    *final_idx    = cp->structure % 39;
}

/* 打印完整的编码对照 */
void mlll_print_encoding_bridge(const char* text) {
    /* 模拟 njsg_encode 的确定性哈希 */
    uint32_t seed = 0;
    for (size_t i = 0; text[i]; i++)
        seed = seed * 31 + (unsigned char)text[i];
    int init_idx   = seed % 24;
    int final_idx  = (seed >> 5) % 39;
    int tone_idx   = (seed >> 10) % 5;
    int stroke_cnt = (seed >> 15) % 30 + 1;
    printf("\n编码桥接: '%s' → 内核身份\n", text);
    printf("  init_idx=%d final_idx=%d tone_idx=%d stroke=%d\n",
           init_idx, final_idx, tone_idx, stroke_cnt);
    /* MLLL 映射 */
    char_prop_t cp;
    cp.stroke    = stroke_cnt;
    cp.tone     = tone_idx;
    cp.radical  = (init_idx % 141) + 1;
    cp.structure = final_idx % 8;
    printf("  MLLL 坐标: stroke=%d tone=%d radical=%d structure=%d\n",
           cp.stroke, cp.tone, cp.radical, cp.structure);
    /* 获取 MLLL 符号指针 */
    mlll_init(0.1);
    mlll_symbol_t *st_sym = mlll_cross_query(SYM_STROKE, cp.stroke, SYM_STROKE);
    mlll_symbol_t *tn_sym = mlll_cross_query(SYM_TONE, cp.tone, SYM_TONE);
    printf("  MLLL 时间线: stroke=%s tone=%s\n",
           st_sym ? st_sym->name : "(null)",
           tn_sym ? tn_sym->name : "(null)");
    /* 坐标定位 */
    uint32_t timeline_pos = 587 + 8 + cp.tone; /* tone 维度偏移 */
    printf("  时间线绝对位置[tone]: pos=%u / %d (总: 841)\n", timeline_pos, 841);
    /* 对径点 = 夫妻命定另一端 */
    uint32_t opposite = (timeline_pos + 420) % 841; /* 半周 841/2 ≈ 420 */
    printf("  对径点(夫妻命定): pos_%u ↔ pos_%u (半圆映射)\n", timeline_pos, opposite);
}
