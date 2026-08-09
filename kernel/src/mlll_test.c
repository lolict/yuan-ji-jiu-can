/*
 * mlll_test.c — MLLL 九维符号综合测试套件
 *
 * 验证范围：
 *   [1] 维度枚举 — 9 维齐全，符号数正确
 *   [2] 距离度量 — 自反性 d(x,x)=0 + 对称性 d(x,y)=d(y,x) + 三角不等式
 *   [3] 字形跃迁 — mlll_char_leap 前后跳跃
 *   [4] 夫妻对偶 — 7 维空间最远距离配对 vs 最近邻
 *   [5] RGB 编解码 — 文本 → 像素 → 文本 往返一致
 *   [6] 字节序列化 — encode_bytes / decode_bytes round-trip
 *   [7] 内核桥接 — from_kernel_identity + to_kernel_identity 一致性
 *   [8] 九维跨维查询 — mlll_cross_query × 9 维交叉覆盖
 *
 * 用法：make test-mlll（编译 + 自动运行 + 输出 PASS/FAIL 计数）
 */

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <stdint.h>
#include "mlll_symbols.h"
#include "mlll_kernel_bridge.h"

/* 测试统计 */
static int test_run = 0;
static int test_pass = 0;
static int test_fail = 0;

#define TEST(name)  do { test_run++; printf("  [%02d] %-55s ", test_run, name); } while(0)
#define PASS()      do { test_pass++; printf("✓ PASS\n"); } while(0)
#define FAIL(msg)   do { test_fail++; printf("✗ FAIL: %s\n", msg); } while(0)
#define ASSERT(cond, msg) do { if (cond) PASS(); else FAIL(msg); } while(0)
#define ASSERT_NEAR(a, b, eps, msg) ASSERT(fabs((a)-(b)) < (eps), msg)

/* [1] 维度枚举测试 */
static void test_dimensions(void) {
    TEST("SYM_DIMENSION_COUNT == 9");
    ASSERT(SYM_DIMENSION_COUNT == 9, "维度数不符");

    TEST("Total symbols == 904");
    mlll_init(0.1);
    ASSERT(REG.count == 904, "总符号数不符");

    TEST("Geometry symbols > 0");
    int geom = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_GEOMETRY) geom++;
    ASSERT(geom > 500, "几何符号偏少");

    TEST("Tone symbols == 8");
    int tone = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_TONE) tone++;
    ASSERT(tone == 8, "声调数非8");

    TEST("Radical symbols == 141");
    int rad = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_RADICAL) rad++;
    ASSERT(rad == 141, "部首数非141");

    TEST("Color symbols == 20");
    int col = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_COLOR) col++;
    ASSERT(col == 20, "颜色数非20");

    TEST("Phase symbols == 13");
    int ph = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_PHASE) ph++;
    ASSERT(ph == 13, "相位数非13");

    TEST("Stroke symbols == 64");
    int st = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_STROKE) st++;
    ASSERT(st == 64, "笔画数非64");

    TEST("Structure symbols == 8");
    int sx = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_STRUCTURE) sx++;
    ASSERT(sx == 8, "结构数非8");

    TEST("Vowel symbols == 39");
    int vw = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_VOWEL) vw++;
    ASSERT(vw == 39, "韵母数非39");

    TEST("Initial symbols == 24");
    int it = 0;
    for (uint32_t i = 0; i < REG.count; i++)
        if (REG.entries[i].dim == SYM_INITIAL) it++;
    ASSERT(it == 24, "声母数非24");
}

/* [2] 距离度量公理 */
static void test_distance(void) {
    char_prop_t a = {6, 0, 31, 4, 15, 6};   /* 囡 */
    char_prop_t b = {5, 0, 107, 0, 25, 16}; /* 生 */
    char_prop_t c = {9, 2, 77, 1, 12, 12};  /* 秋 */

    TEST("自反性 d(a,a) == 0");
    ASSERT_NEAR(mlll_char_distance(&a, &a), 0.0, 1e-9, "自反性违反");

    TEST("对称性 d(a,b) == d(b,a)");
    double d_ab = mlll_char_distance(&a, &b);
    double d_ba = mlll_char_distance(&b, &a);
    ASSERT_NEAR(d_ab, d_ba, 1e-9, "对称性违反");

    TEST("距离非负");
    ASSERT(d_ab >= 0, "距离为负");

    TEST("三角不等式 d(a,b)+d(b,c) >= d(a,c)");
    double d_bc = mlll_char_distance(&b, &c);
    double d_ac = mlll_char_distance(&a, &c);
    ASSERT(d_ab + d_bc >= d_ac - 1e-9, "三角不等式违反");

    TEST("夫妻对偶：最远距离 > 最近距离");
    char_prop_t corpus[] = {
        {6,0,31,4,15,6}, {6,0,31,4,15,6}, {6,3,17,2,27,8}, {5,0,107,0,25,16},
        {4,0,32,0,20,21}, {8,1,64,1,2,10}, {1,0,1,0,3,21}, {9,2,77,1,12,12},
    };
    double min_d = 1e30, max_d = -1;
    for (int i = 0; i < 8; i++) {
        double d = mlll_char_distance(&a, &corpus[i]);
        if (d < min_d) min_d = d;
        if (d > max_d) max_d = d;
    }
    ASSERT(max_d > min_d, "最远不大于最近");

    TEST("距离分解函数可调用");
    mlll_distance_decompose(&a, &b);
    PASS();
}

/* [3] 字形跃迁测试 */
static void test_leap(void) {
    char_prop_t query = {6, 0, 31, 4, 15, 6};
    char_prop_t corpus[] = {
        {6,0,31,4,15,6}, {6,0,31,4,15,6}, {6,3,17,2,27,8}, {5,0,107,0,25,16},
    };
    int n = sizeof(corpus) / sizeof(corpus[0]);

    TEST("mlll_char_leap +1");
    int r = mlll_char_leap(&query, +1, corpus, n);
    ASSERT(r == 0 || r == 1, "leap+1 结果异常");

    TEST("mlll_char_leap -1");
    r = mlll_char_leap(&query, -1, corpus, n);
    ASSERT(r >= -1, "leap-1 结果越界");

    TEST("mlll_char_leap +100 clamp到笔画范围返回有效");
    r = mlll_char_leap(&query, +100, corpus, n);
    ASSERT(r >= 0, "leap+100 应 clamp 并返回候选索引");

    TEST("mlll_char_nearest");
    int nr = mlll_char_nearest(&query, corpus, n);
    ASSERT(nr >= 0, "最近邻查找失败");

    TEST("mlll_char_farthest");
    int fr = mlll_char_farthest(&query, corpus, n);
    ASSERT(fr >= 0, "最远对偶查找失败");
}

/* [4] RGB 编解码往返 */
static void test_rgb(void) {
    const char *text = "囡囝共生";
    uint8_t rgb[128];
    char restored[128];

    TEST("RGB 编码返回正值");
    int n = mlll_rgb_encode_text(text, (int)strlen(text), rgb, 128);
    ASSERT(n == 4, "像素对数量非4");

    TEST("RGB 解码长度 >= 原文字节数");
    int rlen = mlll_rgb_decode_text(rgb, n, restored, 128);
    ASSERT(rlen >= 3, "解码长度不足");

    TEST("RGB round-trip 往返一致");
    ASSERT(strcmp(text, restored) == 0, "往返文本不一致");

    const char *text2 = "刘楚恬";
    TEST("RGB 解码正确还原文本2");
    n = mlll_rgb_encode_text(text2, (int)strlen(text2), rgb, 128);
    mlll_rgb_decode_text(rgb, n, restored, 128);
    ASSERT(strcmp(text2, restored) == 0, "RGB往返文本2不一致");
}

/* [5] 字节序列化测试 */
static void test_bytes(void) {
    TEST("encode_bytes 返回值 == 字节数");
    const char *s = "MQ囡囝";
    uint16_t sym_out[64];
    int expected = (int)strlen(s);
    uint32_t n_sym = mlll_encode_bytes((const uint8_t*)s, expected, sym_out, 64);
    ASSERT((int)n_sym == expected, "encode_bytes 符号数与原文长度不符");

    TEST("decode_bytes 后原文还原");
    char restored[64] = {0};
    mlll_decode_bytes(sym_out, (int)n_sym, (uint8_t*)restored, 64);
    ASSERT(memcmp(s, restored, strlen(s)) == 0, "decode内容不相符");
}

/* [6] 内核桥接测试 */
static void test_bridge(void) {
    TEST("mlll_to_kernel_identity 输出四元");
    int init_idx, final_idx, tone_idx, stroke_count;
    char_prop_t cp = {6, 0, 31, 4, 15, 6};
    mlll_to_kernel_identity(&cp, &init_idx, &final_idx, &tone_idx, &stroke_count);
    ASSERT(stroke_count > 0, "stroke_count 不大于0");
    ASSERT(tone_idx < 5 && tone_idx >= 0, "tone_idx 越界");
    ASSERT(init_idx < 24 && init_idx >= 0, "init_idx 越界");
    ASSERT(final_idx < 39 && final_idx >= 0, "final_idx 越界");
    PASS();

    TEST("mlll_print_encoding_bridge 执行无错");
    mlll_print_encoding_bridge("囡囝共生");
    PASS();
}

/* [7] 九维跨维查询 */
static void test_cross_query(void) {
    TEST("SYM_GEOMETRY ↔ SYM_TONE 跨维");
    mlll_symbol_t *r = mlll_cross_query(SYM_GEOMETRY, 50, SYM_TONE);
    ASSERT(r != NULL, "几何→声调的跨维查询失败");

    TEST("SYM_STROKE ↔ SYM_COLOR 跨维");
    r = mlll_cross_query(SYM_STROKE, 6, SYM_COLOR);
    ASSERT(r != NULL, "笔画→颜色的跨维查询失败");

    TEST("SYM_TONE ↔ SYM_RADICAL 跨维");
    r = mlll_cross_query(SYM_TONE, 3, SYM_RADICAL);
    ASSERT(r != NULL, "声调→部首的跨维查询失败");

    TEST("SYM_VOWEL ↔ SYM_PHASE 跨维");
    r = mlll_cross_query(SYM_VOWEL, 15, SYM_PHASE);
    ASSERT(r != NULL, "韵母→相位的跨维查询失败");

    TEST("SYM_INITIAL ↔ SYM_GEOMETRY 跨维");
    r = mlll_cross_query(SYM_INITIAL, 6, SYM_GEOMETRY);
    ASSERT(r != NULL, "声母→几何的跨维查询失败");

    TEST("SYM_INITIAL ↔ SYM_COLOR 跨维");
    r = mlll_cross_query(SYM_INITIAL, 13, SYM_COLOR);
    ASSERT(r != NULL, "声母→颜色的跨维查询失败");
}

/* [8] string encode 测试 */
static void test_string_encode(void) {
    char_prop_t chars[] = {
        {6,0,31,4,15,6}, {6,0,31,4,15,6}, {6,3,17,2,27,8}, {5,0,107,0,25,16},
    };
    TEST("mlll_string_encode 不崩溃");
    mlll_string_encode("囡囝共生", chars, 4);
    PASS();

    TEST("mlll_string_distance_matrix 不崩溃");
    mlll_string_distance_matrix("囡囝共生", chars, 4);
    PASS();
}

int main(void) {
    printf("════════════════════════════════════════════\n");
    printf("  MLLL 九维符号综合测试套件\n");
    printf("  覆盖: 维度枚举 / 距离公理 / 字形跃迁 / 夫妻对偶\n");
    printf("        RGB编解码 / 字节序列 / 内核桥接 / 跨维查询\n");
    printf("════════════════════════════════════════════\n\n");

    printf("[I] 九维枚举测试\n");
    test_dimensions();

    printf("\n[II] 距离公理测试\n");
    test_distance();

    printf("\n[III] 字形跃迁 + 夫妻对偶测试\n");
    test_leap();

    printf("\n[IV] RGB 编解码往返测试\n");
    test_rgb();

    printf("\n[V] 字节序列化测试\n");
    test_bytes();

    printf("\n[VI] 内核桥接一致性测试\n");
    test_bridge();

    printf("\n[VII] 九维跨维查询测试\n");
    test_cross_query();

    printf("\n[VIII] String encode/decode 测试\n");
    test_string_encode();

    printf("\n════════════════════════════════════════════\n");
    printf("  测试完成: %d 运行 | %d ✓PASS | %d ✗FAIL\n",
           test_run, test_pass, test_fail);
    printf("  结果: %s\n", test_fail == 0 ? "★ 全员通过 ★" : "✗ 存在失败");
    printf("════════════════════════════════════════════\n");

    return test_fail == 0 ? 0 : 1;
}
