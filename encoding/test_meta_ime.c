/**
 * test_meta_ime.c — 元输入法全链路测试
 *
 * 验证:
 *   七层编码/解码闭环 · 音节约输入 · 规则匹配 · 自动触发
 *   JNI 桥接 · 活码编译器 · 时钟动作
 */

#include "meta_ime_jni.h"
#include "shengmu.h"
#include <stdio.h>
#include <string.h>

/* 断言宏 */
#define ASSERT_EQ(a, b, label) do { \
    if ((a) == (b)) { printf("  ✅ %s\n", label); pass++; } \
    else { printf("  ❌ %s: got %u, expected %u\n", label, (unsigned)(a), (unsigned)(b)); fail++; } \
} while(0)

#define ASSERT_NEQ(a, b, label) do { \
    if ((a) != (b)) { printf("  ✅ %s\n", label); pass++; } \
    else { printf("  ❌ %s: got %u (should differ)\n", label, (unsigned)(a)); fail++; } \
} while(0)

#define ASSERT_TRUE(cond, label) do { \
    if (cond) { printf("  ✅ %s\n", label); pass++; } \
    else { printf("  ❌ %s\n", label); fail++; } \
} while(0)

static int pass = 0, fail = 0;

/* ═══════════════════════════════════════════════════════════════
 * 声母韵母索引表 (从 shengmu.h / yunmu.h 提取的部分)
 * ═══════════════════════════════════════════════════════════════ */

/* 声母索引: b=0, p=1, m=2, f=3, d=4, t=5, n=6, l=7
 *              g=8, k=9, h=10, w=11, j=12, q=13, x=14, y=15
 *              z=16, c=17, s=18, r=19, zh=20, ch=21, sh=22, ri=23 */

/* 韵母索引 (简化): a=0, o=1, e=2, i=3, u=4, v=5
 *                    ai=6, ei=7, ao=8, ou=9, an=10, en=11
 *                    ang=12, eng=13, ong=14, ia=15, ie=16
 *                    iu=17, in=18, un=19, ua=20, uo=21
 *                    ue=22, ui=23, iao=24, ian=25, iang=26
 *                    iong=27, uai=28, uan=29, uang=30
 *                    er=31, ve=32, vn=33 */

/* ═══════════════════════════════════════════════════════════════
 * 测试: 七层编解码
 * ═══════════════════════════════════════════════════════════════ */

static void test_seven_layer_codec(void) {
    printf("\n[1] 七层编解码闭环\n");

    /* 编码: sm=b(0), ym=a(0), tone=1, pol=WIFE(0), wit=3, flow=PRELOAD(0), sov=0x12345 */
    NJSG_uint64 cell = metaime_encode(0, 0, 1, 0, 3, 0, 0x12345);

    NJSG_uint8 sm, ym, tone, pol, wit, flow;
    NJSG_uint32 sov;
    metaime_decode(cell, &sm, &ym, &tone, &pol, &wit, &flow, &sov);

    ASSERT_EQ(sm, 0, "声母 b");
    ASSERT_EQ(ym, 0, "韵母 a");
    ASSERT_EQ(tone, 1, "声调 1");
    ASSERT_EQ(pol, METAIME_POL_WIFE, "极性 妻");
    ASSERT_EQ(wit, 3, "见证 3");
    ASSERT_EQ(flow, 0, "流 预加载");
    ASSERT_EQ(sov, 0x12345, "主权 0x12345");
}

/* ═══════════════════════════════════════════════════════════════
 * 测试: 单音节编码
 * ═══════════════════════════════════════════════════════════════ */

static void test_syllable_codec(void) {
    printf("\n[2] 单音节编码\n");

    /* 妈: m(2) + a(0) + tone1 */
    NJSG_uint16 ma = SYLLABLE(2, 0, 1);
    ASSERT_EQ(SYLLABLE_SM(ma), 2, "妈 声母 m");
    ASSERT_EQ(SYLLABLE_YM(ma), 0, "妈 韵母 a");
    ASSERT_EQ(SYLLABLE_TONE(ma), 1, "妈 声调 1");

    /* 好: h(10) + ao(8) + tone3 */
    NJSG_uint16 hao = SYLLABLE(10, 8, 3);
    ASSERT_EQ(SYLLABLE_SM(hao), 10, "好 声母 h");
    ASSERT_EQ(SYLLABLE_YM(hao), 8, "好 韵母 ao");
    ASSERT_EQ(SYLLABLE_TONE(hao), 3, "好 声调 3");

    /* 你: n(6) + i(3) + tone3 */
    NJSG_uint16 ni = SYLLABLE(6, 3, 3);
    ASSERT_EQ(SYLLABLE_SM(ni), 6, "你 声母 n");
    ASSERT_EQ(SYLLABLE_YM(ni), 3, "你 韵母 i");

    /* 通配符 */
    ASSERT_TRUE(SYLLABLE_WILDCARD == 0xFFFF, "通配符 0xFFFF");
}

/* ═══════════════════════════════════════════════════════════════
 * 测试: 规则匹配
 * ═══════════════════════════════════════════════════════════════ */

static void test_pattern_matching(void) {
    printf("\n[3] 规则匹配\n");

    metaime_engine_t engine;
    metaime_init(&engine);
    engine.auto_fire = 70;

    /* 规则: "你好" → 输出 "你好世界" */
    /* 你: n6 i3 tone3 · 好: h10 ao8 tone3 */
    metaime_pattern_t pattern_nihao;
    memset(&pattern_nihao, 0, sizeof(pattern_nihao));
    pattern_nihao.len = 2;
    pattern_nihao.syllables[0] = SYLLABLE(6, 3, 3);   /* 你 */
    pattern_nihao.syllables[1] = SYLLABLE(10, 8, 3);   /* 好 */

    metaime_action_t action_nihao;
    memset(&action_nihao, 0, sizeof(action_nihao));
    snprintf(action_nihao.text, METAIME_MAX_OUTPUT, "你好世界");
    action_nihao.cell = metaime_encode(6, 3, 3, METAIME_POL_OBSERVER, 5, 3, 0xABC);
    action_nihao.flow = 3;
    action_nihao.priority = 8;

    int idx = metaime_add_rule(&engine, &pattern_nihao, &action_nihao);
    ASSERT_TRUE(idx >= 0, "添加规则 '你好'");

    /* 逐个输入: 妈(a) → 无匹配 · 你(n i3) → 仍不够 · 好(hao) → 匹配 */
    metaime_feed(&engine, 2, 0, 1);   /* 妈 */
    /* 无匹配 */

    int matched = metaime_feed(&engine, 6, 3, 3);   /* 你 */
    ASSERT_NEQ(matched, 1, "输入'你' (未完整)");

    matched = metaime_feed(&engine, 10, 8, 3);     /* 好 → 匹配! */
    ASSERT_EQ(matched, 1, "输入'好'→匹配成功");

    /* 验证匹配结果 */
    ASSERT_EQ(engine.last_match.confidence, 100, "置信度 100%");
}

/* ═══════════════════════════════════════════════════════════════
 * 测试: 自动触发
 * ═══════════════════════════════════════════════════════════════ */

static void test_autofire(void) {
    printf("\n[4] 自动触发\n");

    metaime_engine_t engine;
    metaime_init(&engine);
    engine.auto_fire = 70;

    /* 规则: "满" m(2) + an(10) + tone3 → 输出 "满全法" */
    metaime_pattern_t pat;
    memset(&pat, 0, sizeof(pat));
    pat.len = 1;
    pat.syllables[0] = SYLLABLE(2, 10, 3);

    metaime_action_t act;
    memset(&act, 0, sizeof(act));
    snprintf(act.text, METAIME_MAX_OUTPUT, "满全法");
    act.cell = metaime_encode(2, 10, 3, METAIME_POL_ARBITER, 7, 7, 0xFFF);
    act.priority = 10;

    metaime_add_rule(&engine, &pat, &act);

    metaime_feed(&engine, 2, 10, 3);  /* 满 */

    /* 自动触发 */
    char out_text[METAIME_MAX_OUTPUT];
    NJSG_uint64 out_cell;
    int fired = metaime_autofire(&engine, out_text, sizeof(out_text), &out_cell);

    ASSERT_EQ(fired, 1, "自动触发成功");
    ASSERT_TRUE(strcmp(out_text, "满全法") == 0, "输出 '满全法'");
    ASSERT_NEQ(out_cell, 0, "附带 cell");

    /* 验证 cell 解码 */
    NJSG_uint8 d_sm, d_ym, d_tone, d_pol, d_wit, d_flow;
    NJSG_uint32 d_sov;
    metaime_decode(out_cell, &d_sm, &d_ym, &d_tone, &d_pol, &d_wit, &d_flow, &d_sov);
    ASSERT_EQ(d_pol, METAIME_POL_ARBITER, "cell 极性=仲裁");
    ASSERT_EQ(d_wit, 7, "cell 见证=7");
}

/* ═══════════════════════════════════════════════════════════════
 * 测试: JNI 桥接
 * ═══════════════════════════════════════════════════════════════ */

static void test_jni_bridge(void) {
    printf("\n[5] JNI 桥接\n");

    void *engine = NULL;
    int rc = metaime_jni_create(&engine);
    ASSERT_EQ(rc, 0, "创建引擎");

    /* 加载规则 (模拟) */
    rc = metaime_jni_load_rules(engine, (const NJSG_uint8 *)"\x01\x02", 2);
    ASSERT_EQ(rc, 0, "加载规则桩");

    /* 输入 */
    rc = metaime_jni_feed(engine, 2, 10, 3);  /* 满 */
    ASSERT_NEQ(rc, -1, "JNI 喂入音节");

    /* 获取匹配 */
    NJSG_uint8 confidence;
    char out[64];
    NJSG_uint64 cell;
    rc = metaime_jni_get_match(engine, &confidence, out, sizeof(out), &cell);
    ASSERT_TRUE(rc >= 0, "JNI 获取匹配");

    /* 时钟动作 */
    rc = metaime_jni_clock(engine, MI_TICK_KEYDOWN);
    ASSERT_EQ(rc, 0, "时钟 keydown");
    rc = metaime_jni_clock(engine, MI_TICK_COMMIT);
    ASSERT_EQ(rc, 0, "时钟 commit (清空缓冲)");

    /* 销毁 */
    rc = metaime_jni_destroy(engine);
    ASSERT_EQ(rc, 0, "销毁引擎");
}

/* ═══════════════════════════════════════════════════════════════
 * 测试: 活码编译器
 * ═══════════════════════════════════════════════════════════════ */

static void test_live_code(void) {
    printf("\n[6] 活码编译器\n");

    mi_live_code_t lc;
    memset(&lc, 0, sizeof(lc));

    /* 激活: 高熵未固定 */
    mi_live_activate(&lc, 0xDEADBEEFCAFEULL);
    ASSERT_EQ(lc.key, 0xDEADBEEFCAFEULL, "钥匙设置");
    ASSERT_EQ(lc.fixed, 0, "未固定");
    ASSERT_NEQ(lc.value, 0, "值高熵非零");

    /* 未固定 → 累加无效 */
    NJSG_uint32 old_val = lc.value;
    mi_live_accumulate(&lc, 100);
    ASSERT_EQ(lc.value, old_val, "未固定不累积");

    /* 固定 */
    mi_live_fix(&lc, 5);
    ASSERT_EQ(lc.fixed, 1, "已固定");
    ASSERT_EQ(lc.order, 5, "排列顺序=5");

    /* 固定后 → 加法统计 */
    mi_live_accumulate(&lc, 10);
    /* value = order*100 + delta*order = 500 + 50 = 550 */
    ASSERT_EQ(lc.value, 550, "加法统计: 5*100 + 10*5");

    mi_live_accumulate(&lc, 5);
    ASSERT_EQ(lc.value, 575, "加法统计: 550 + 5*5");
}

/* ═══════════════════════════════════════════════════════════════
 * main
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══ 元输入法 MetaIME 全链路测试 ═══\n");
    printf("目标: %s, 指针宽度: %d-bit\n", NJSG_WASM_STR, NJSG_PTR_WIDTH);

    test_seven_layer_codec();
    test_syllable_codec();
    test_pattern_matching();
    test_autofire();
    test_jni_bridge();
    test_live_code();

    char stats[256];
    metaime_engine_t eng;
    metaime_init(&eng);
    metaime_stats(&eng, stats, sizeof(stats));

    printf("\n═══ 结果: %d 通过 / %d 失败 ═══\n", pass, fail);
    return fail > 0 ? 1 : 0;
}
