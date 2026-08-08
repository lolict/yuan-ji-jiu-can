/*
 * mlll_profile.c — 满全法身份卡：一个名字的全部编码画像
 *
 * 输入：名字字符串
 * 输出：
 *   1. 内核身份 (init/final/tone/stroke)
 *   2. MLLL 七维坐标
 *   3. 夫妻对偶（命定最远）
 *   4. 字形跃迁邻域（±N 笔画到达谁）
 *   5. 时间线绝对位置 + 对径点
 *
 * 用法：独立运行
 *   ./mlll_profile.elf 囡囝共生
 *   ./mlll_profile.elf 刘楚恬
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include "../include/mlll_symbols.h"
#include "../include/mlll_kernel_bridge.h"

/* 扩展字符集（满全法八字符 benchmark） */
static char_prop_t corpus[] = {
    {6,0,31,4},  /* 囡 全包围 */
    {6,0,31,4},  /* 囝 全包围 */
    {6,3,17,2},  /* 共 上下 */
    {5,0,107,0}, /* 生 独体 */
    {4,0,32,0},  /* 元 独体 */
    {8,1,64,1},  /* 和 左右 */
    {1,0,1,0},   /* 一 独体 */
    {9,2,77,1},  /* 秋 左右 */
};
static const char* corpus_names[] = {"囡","囝","共","生","元","和","一","秋"};
#define CORPUS_LEN 8

int main(int argc, char** argv) {
    const char* name = (argc > 1) ? argv[1] : "囡囝共生";
    printf("╔════════════════════════════════════════════╗\n");
    printf("║        满 全 法 身 份 卡 (v2.0)           ║\n");
    printf("╠════════════════════════════════════════════╣\n");
    printf("║ 名字: %-36s║\n", name);
    printf("╠════════════════════════════════════════════╣\n");

    /* 1. 编码桥接 */
    mlll_print_encoding_bridge(name);

    /* 2. 统计画像 */
    uint32_t seed = 0;
    for (size_t i = 0; name[i]; i++)
        seed = seed * 31 + (unsigned char)name[i];
    int tone_idx   = (seed >> 10) % 5;
    int stroke_min = (seed >> 15) % 30 + 1;

    char_prop_t me;
    me.stroke    = stroke_min;
    me.tone      = tone_idx;
    me.radical   = (seed % 24) % 141 + 1;
    me.structure = ((seed >> 5) % 39) % 8;

    /* 3. 在 benchmark 中的关系 */
    double min_d = 1e30, max_d = -1;
    int min_i = -1, max_i = -1;
    for (int i = 0; i < CORPUS_LEN; i++) {
        double d = mlll_char_distance(&me, &corpus[i]);
        if (d < min_d) { min_d = d; min_i = i; }
        if (d > max_d) { max_d = d; max_i = i; }
    }
    printf("\n【满全法关系网】\n");
    printf("  命定伴侣(最近): %s dist=%.6f\n", corpus_names[min_i], min_d);
    printf("  命定对偶(最远): %s dist=%.6f\n", corpus_names[max_i], max_d);

    /* 4. 跃迁 */
    printf("\n【字形跃迁】\n");
    int jumps[] = {+1,-1,+5,-5,+10};
    for (int j = 0; j < 5; j++) {
        int idx = mlll_char_leap(&me, jumps[j], corpus, CORPUS_LEN);
        if (idx >= 0)
            printf("  %+3d画 → %s(%d画/%d声/%s)\n",
                   jumps[j], corpus_names[idx], corpus[idx].stroke,
                   corpus[idx].tone, corpus[idx].structure == 4 ? "全包围" :
                                    corpus[idx].structure == 0 ? "独体" :
                                    corpus[idx].structure == 2 ? "上下" : "其他");
    }

    /* 5. 完整距离表 */
    printf("\n【距离全表】(n=%d)\n", CORPUS_LEN);
    for (int i = 0; i < CORPUS_LEN; i++) {
        double d = mlll_char_distance(&me, &corpus[i]);
        printf("  %-4s dist=%.6f %s\n", corpus_names[i], d,
               i == min_i ? "← 命定伴侣" : (i == max_i ? "← 夫妻对偶" : ""));
    }

    printf("╚════════════════════════════════════════════╝\n");
    return 0;
}
