/**
 * test_pipeline.c — 容器流水线完整测试
 *
 * 验证从UTF-8汉字输入到容器编码输出的完整通路
 * 打印每一步的状态便于人工验证
 */

#include <stdio.h>
#include <string.h>
#include "container_pipeline.h"
#include "husk_compiler.h"
#include "mccp_encdec.h"

/* ── 测试用UTF-8汉字常量 ── */
static const char *TEST_HANZI[] = {
    "回",   /* huí · FAMILY_CONTAINER */
    "好",   /* hǎo  · FAMILY_COUPLE */
    "一",   /* yī   · FAMILY_BOUNDARY */
    "国",   /* guó  · FAMILY_CONTAINER */
    "人",   /* rén  · FAMILY_BOUNDARY */
    "我",   /* wǒ   · FAMILY_COUPLE */
    "你",   /* nǐ   · FAMILY_COUPLE */
    "明",   /* míng · FAMILY_COUPLE */
    "看",   /* kàn  · FAMILY_COUPLE */
    "想",   /* xiǎng· FAMILY_COUPLE */
    "说",   /* shuō · FAMILY_COUPLE */
    "话",   /* huà  · FAMILY_COUPLE */
    "语",   /* yǔ   · FAMILY_COUPLE */
    "二",   /* èr   · FAMILY_BOUNDARY */
    "三",   /* sān  · FAMILY_BOUNDARY */
    "十",   /* shí  · FAMILY_BOUNDARY */
    "上",   /* shàng· FAMILY_BOUNDARY */
    "下",   /* xià  · FAMILY_BOUNDARY */
    "中",   /* zhōng· FAMILY_BOUNDARY */
    "天",   /* tiān · FAMILY_BOUNDARY */
    "地",   /* dì   · FAMILY_BOUNDARY */
    "网",   /* wǎng · FAMILY_WEB */
    "开",   /* kāi  · FAMILY_WEB */
    "关",   /* guān · FAMILY_WEB */
    "时",   /* shí  · FAMILY_NEUTRAL */
    "间",   /* jiān · FAMILY_NEUTRAL */
    "的",   /* de   · FAMILY_NEUTRAL */
    "是",   /* shì  · FAMILY_NEUTRAL */
    "不",   /* bù   · FAMILY_NEUTRAL */
    "在",   /* zài  · FAMILY_NEUTRAL */
    "有",   /* yǒu  · FAMILY_NEUTRAL */
    "和",   /* hé   · FAMILY_NEUTRAL */
    /* WEB 新增: 半包围 */
    "往",   /* wǎng · FAMILY_WEB */
    "径",   /* jìng · FAMILY_WEB */
    "待",   /* dài  · FAMILY_WEB */
    "徒",   /* tú   · FAMILY_WEB */
    "得",   /* dé   · FAMILY_WEB */
    "微",   /* wēi  · FAMILY_WEB */
    "户",   /* hù   · FAMILY_WEB */
    "房",   /* fáng · FAMILY_WEB */
    "所",   /* suǒ  · FAMILY_WEB */
    "扇",   /* shàn · FAMILY_WEB */
    /* NEUTRAL 新增: 五行 */
    "木",   /* mù   · FAMILY_NEUTRAL */
    "火",   /* huǒ  · FAMILY_NEUTRAL */
    "土",   /* tǔ   · FAMILY_NEUTRAL */
    "金",   /* jīn  · FAMILY_NEUTRAL */
    "水",   /* shuǐ · FAMILY_NEUTRAL */
    /* NEUTRAL 新增: 方位 */
    "东",   /* dōng · FAMILY_NEUTRAL */
    "西",   /* xī   · FAMILY_NEUTRAL */
    "南",   /* nán  · FAMILY_NEUTRAL */
    "北",   /* běi  · FAMILY_NEUTRAL */
    /* NEUTRAL 新增: 颜色 */
    "红",   /* hóng · FAMILY_NEUTRAL */
    "黄",   /* huáng· FAMILY_NEUTRAL */
    "蓝",   /* lán  · FAMILY_NEUTRAL */
    "绿",   /* lǜ   · FAMILY_NEUTRAL */
    "白",   /* bái  · FAMILY_NEUTRAL */
    "黑",   /* hēi  · FAMILY_NEUTRAL */
    /* BOUNDARY 新增 */
    "风",   /* fēng · FAMILY_BOUNDARY */
    "云",   /* yún  · FAMILY_BOUNDARY */
    "雨",   /* yǔ   · FAMILY_BOUNDARY */
    "电",   /* diàn · FAMILY_BOUNDARY */
    "末",   /* mò   · FAMILY_BOUNDARY */
    "未",   /* wèi  · FAMILY_BOUNDARY */
    "之",   /* zhī  · FAMILY_BOUNDARY */
    "予",   /* yǔ   · FAMILY_BOUNDARY */
    "即",   /* jí   · FAMILY_BOUNDARY */
    /* COUPLE 新增 */
    "她",   /* tā   · FAMILY_COUPLE """
    "吧",   /* ba   · FAMILY_COUPLE */
    "呢",   /* ne   · FAMILY_COUPLE */
    "宝",   /* bǎo  · FAMILY_COUPLE """
    "家",   /* jiā  · FAMILY_COUPLE """
    "树",   /* shù  · FAMILY_COUPLE */
    "永",   /* yǒng · FAMILY_COUPLE """
    "求",   /* qiú  · FAMILY_COUPLE """
    "传",   /* chuán· FAMILY_COUPLE """
    "体",   /* tǐ   · FAMILY_COUPLE """
    "李",   /* lǐ   · FAMILY_COUPLE */
    "果",   /* guǒ  · FAMILY_COUPLE */
    /* WEB 新增 */
    "病",   /* bìng · FAMILY_WEB */
    "痛",   /* tòng · FAMILY_WEB */
    "建",   /* jiàn · FAMILY_WEB */
    "庭",   /* tíng · FAMILY_WEB */
    "处",   /* chù  · FAMILY_WEB */
    "庆",   /* qìng · FAMILY_WEB */
    "递",   /* dì   · FAMILY_WEB */
    "通",   /* tōng · FAMILY_WEB """
    "遇",   /* yù   · FAMILY_WEB */
    "广",   /* guǎng· FAMILY_WEB */
    "去",   /* qù   · FAMILY_WEB */
    "办",   /* bàn  · FAMILY_WEB */
    /* NEUTRAL 新增 */
    "爱",   /* ài   · FAMILY_NEUTRAL */
    "情",   /* qíng · FAMILY_NEUTRAL """
    "想",   /* xiǎng· FAMILY_NEUTRAL """
    "知",   /* zhī  · FAMILY_NEUTRAL */
    "今",   /* jīn  · FAMILY_NEUTRAL */
    "同",   /* tóng · FAMILY_NEUTRAL */
    "面",   /* miàn · FAMILY_NEUTRAL */
    "身",   /* shēn · FAMILY_NEUTRAL */
    /* ─── 新增批次验证: BOUNDARY ─── */
    "斗",   /* dòu  · FAMILY_BOUNDARY */
    "瓦",   /* wǎ   · FAMILY_BOUNDARY */
    "互",   /* hù   · FAMILY_BOUNDARY */
    "册",   /* cè   · FAMILY_BOUNDARY */
    "凸",   /* tū   · FAMILY_BOUNDARY */
    "凹",   /* āo   · FAMILY_BOUNDARY */
    "亮",   /* liàng· FAMILY_BOUNDARY */
    "麦",   /* mài  · FAMILY_BOUNDARY */
    "酋",   /* qiú  · FAMILY_BOUNDARY */
    "雯",   /* wén  · FAMILY_BOUNDARY */
    /* ─── 新增批次验证: COUPLE ─── */
    "谁",   /* shéi · FAMILY_COUPLE */
    "那",   /* nà   · FAMILY_COUPLE */
    "哪",   /* nǎ   · FAMILY_COUPLE */
    "这",   /* zhè  · FAMILY_COUPLE */
    "进",   /* jìn  · FAMILY_COUPLE */
    "两",   /* liǎng· FAMILY_COUPLE */
    "被",   /* bèi  · FAMILY_COUPLE */
    "满",   /* mǎn  · FAMILY_COUPLE */
    "解",   /* jiě  · FAMILY_COUPLE */
    "啊",   /* a    · FAMILY_COUPLE */
    /* ─── 新增批次验证: WEB ─── */
    "医",   /* yī   · FAMILY_WEB */
    "匣",   /* xiá  · FAMILY_WEB */
    "匪",   /* fěi  · FAMILY_WEB */
    "匿",   /* nì   · FAMILY_WEB */
    "匙",   /* chí  · FAMILY_WEB */
    "厌",   /* yàn  · FAMILY_WEB */
    "厢",   /* xiāng· FAMILY_WEB */
    "厨",   /* chú  · FAMILY_WEB */
    "厉",   /* lì   · FAMILY_WEB */
    "宠",   /* chǒng· FAMILY_WEB */
    /* ─── 新增批次验证: NEUTRAL ─── */
    "候",   /* hòu  · FAMILY_NEUTRAL */
    "先",   /* xiān · FAMILY_NEUTRAL */
    "来",   /* lái  · FAMILY_NEUTRAL */
    "出",   /* chū  · FAMILY_NEUTRAL */
    "入",   /* rù   · FAMILY_NEUTRAL */
    "放",   /* fàng · FAMILY_NEUTRAL */
    "收",   /* shōu · FAMILY_NEUTRAL */
    "答",   /* dá   · FAMILY_NEUTRAL */
    "给",   /* gěi  · FAMILY_NEUTRAL */
    "写",   /* xiě  · FAMILY_NEUTRAL */
    /* ─── 满全法全量验证: 新增高频字 ─── */
    "丑",   /* chǒu · FAMILY_BOUNDARY */
    "丰",   /* fēng · FAMILY_BOUNDARY */
    "为",   /* wéi  · FAMILY_BOUNDARY */
    "举",   /* jǔ   · FAMILY_BOUNDARY */
    "乐",   /* lè   · FAMILY_BOUNDARY */
    "亚",   /* yà   · FAMILY_BOUNDARY */
    "产",   /* chǎn · FAMILY_BOUNDARY */
    "今",   /* jīn  · FAMILY_BOUNDARY */
    "以",   /* yǐ   · FAMILY_BOUNDARY */
    "传",   /* chuán· FAMILY_BOUNDARY */
    "住",   /* zhù  · FAMILY_BOUNDARY */
    "作",   /* zuò  · FAMILY_BOUNDARY */
    "便",   /* biàn · FAMILY_BOUNDARY */
    "信",   /* xìn  · FAMILY_BOUNDARY */
    "倒",   /* dào  · FAMILY_BOUNDARY */
    "元",   /* yuán · FAMILY_BOUNDARY */
    "先",   /* xiān · FAMILY_BOUNDARY */
    "克",   /* kè   · FAMILY_BOUNDARY */
    "免",   /* miǎn · FAMILY_BOUNDARY */
    "入",   /* rù   · FAMILY_BOUNDARY */
    "般",   /* bān  · FAMILY_BOUNDARY */
    "颁",   /* bān  · FAMILY_BOUNDARY */
    "版",   /* bǎn  · FAMILY_BOUNDARY */
    "扮",   /* bàn  · FAMILY_BOUNDARY */
    "拌",   /* bàn  · FAMILY_BOUNDARY */
    "瓣",   /* bàn  · FAMILY_BOUNDARY */
    "邦",   /* bāng · FAMILY_BOUNDARY */
    "懂",   /* dǒng · FAMILY_NEUTRAL */
    "教",   /* jiào · FAMILY_NEUTRAL */
    "放",   /* fàng · FAMILY_NEUTRAL */
    NULL
};

/* 打印容器编码的位域解包 */
static void print_container_decode(uint32_t code) {
    uint8_t family   = CONTAINER_GET_FAMILY(code);
    uint8_t polarity = CONTAINER_GET_POLARITY(code);
    uint8_t stype    = CONTAINER_GET_STRUCT(code);
    uint8_t sm       = CONTAINER_GET_SHENGMU(code);
    uint8_t ym       = CONTAINER_GET_YUNMU(code);
    uint8_t tn       = CONTAINER_GET_TONE(code);

    const char *family_names[] = {"容器CONTAINER", "边界BOUNDARY", "夫妻COUPLE", "网状WEB", "中性NEUTRAL"};
    const char *struct_names[] = {"独体", "左右", "上下", "半包围", "全包围", "品字", "左中右", "上中下"};

    printf("    ├─ 家族: %s\n", family < 5 ? family_names[family] : "?");
    printf("    ├─ 极性: %s\n", polarity ? "我侧(阳)" : "刘楚恬侧(阴)");
    printf("    ├─ 结构: %s\n", stype < 8 ? struct_names[stype] : "?");
    printf("    ├─ 声母index: %d\n", sm);
    printf("    ├─ 韵母index: %d\n", ym);
    printf("    └─ 声调: %d\n", tn);
}

/* 测试1: 汉字 → 容器编码 */
static int test_hanzi_pipeline(void) {
    printf("\n═══ 测试1: UTF-8汉字 → 容器编码 ═══\n\n");

    int pass = 0, fail = 0;

    for (int i = 0; TEST_HANZI[i]; i++) {
        const char *hz = TEST_HANZI[i];
        printf("「%s」:\n", hz);

        pipeline_result_t r = container_pipeline_process_utf8(hz);

        if (r.status == PATH_OK) {
            printf("  ✅ 通路成功\n");
            printf("  UCS-2: U+%04X\n", r.ucs2);
            printf("  容器编码: 0x%08X\n", r.container_code);
            print_container_decode(r.container_code);
            pass++;
        } else {
            printf("  ❌ 通路失败 (status=%d)\n", r.status);
            fail++;
        }
        printf("\n");
    }

    printf("结果: %d 通过, %d 失败\n", pass, fail);
    return fail;
}

/* 测试2: 脱壳编译器 — 旧进制符号检测 */
static int test_husk_detect(void) {
    printf("\n═══ 测试2: 壳检测 (旧进制 → 壳类型) ═══\n\n");

    struct {
        const char *symbol;
        husk_type_t expected;
        const char *label;
    } cases[] = {
        {"int",    HUSK_C_ARITHMETIC,   "C int类型"},
        {"float",  HUSK_C_ARITHMETIC,   "C float类型"},
        {"char",   HUSK_C_ARITHMETIC,   "C char类型"},
        {"42",     HUSK_NUMERIC_LITERAL, "数字字面量"},
        {"3.14",   HUSK_FLOAT_IEEE,     "IEEE浮点"},
        {"ptr",    HUSK_ASCII_ONLY,     "通用ASCII"},
        {"impl",   HUSK_RUST_TRAIT,     "Rust impl"},
        {"&mut",   HUSK_RUST_OWNERSHIP, "Rust 借用"},
        {"concept",HUSK_NIM_TYPECLASS,  "Nim concept"},
        {"comptime", HUSK_ZIG_COMPTIME, "Zig comptime"},
        {NULL, 0, NULL}
    };

    int pass = 0, fail = 0;
    for (int i = 0; cases[i].symbol; i++) {
        husk_type_t detected = husk_detect(cases[i].symbol, strlen(cases[i].symbol));
        const char *result_str = (detected == cases[i].expected) ? "✅" : "❌";
        printf("  %s %-20s → husk_type=%-4d (%s)\n",
               result_str, cases[i].symbol, detected, cases[i].label);
        if (detected == cases[i].expected) pass++; else fail++;
    }

    printf("\n结果: %d 通过, %d 失败\n", pass, fail);
    return fail;
}

/* 测试3: 脱壳 → 编排 全流程 */
static int test_husk_full_flow(void) {
    printf("\n═══ 测试3: 脱壳全流程 (旧符号 → 容器编码) ═══\n\n");

    const char *symbols[] = {"int", "42", "float", "ptr", "impl", "comptime", NULL};
    int pass = 0, fail = 0;

    for (int i = 0; symbols[i]; i++) {
        const char *sym = symbols[i];
        uint32_t code = husk_process(sym, strlen(sym));
        if (code) {
            printf("  「%-10s」→ 容器编码 0x%08X\n", sym, code);
            pass++;
        } else {
            printf("  「%-10s」→ ❌ 失败\n", sym);
            fail++;
        }
    }

    printf("\n结果: %d 通过, %d 失败\n", pass, fail);
    return fail;
}

/* 测试4: MCCP 编解码 */
static int test_mccp_codec(void) {
    printf("\n═══ 测试4: MCCP 编解码 ═══\n\n");
    mccp_encdec_init();

    /* 拼音字符串 → tokens */
    const char *pinyin_input = "zhong1 guo2 ren2";
    uint32_t tokens[32];
    size_t tok_count = 0;

    printf("  输入拼音: %s\n", pinyin_input);
    if (mccp_encode(pinyin_input, tokens, 32, &tok_count) == 0) {
        printf("  编码得到 %zu tokens:\n", tok_count);
        for (size_t i = 0; i < tok_count; i++) {
            uint8_t kind = MCCP_KIND(tokens[i]);
            uint16_t idx = MCCP_INDEX(tokens[i]);
            uint8_t tone = MCCP_METAA(tokens[i]);
            const char *kind_name = (kind == MCCP_INITIAL) ? "声母" :
                                    (kind == MCCP_FINAL)   ? "韵母" : "汉字";
            printf("    [%2zu] %s idx=%d tone=%d\n", i, kind_name, idx, tone);
        }
    } else {
        printf("  ❌ 编码失败\n");
        return 1;
    }

    /* tokens → 拼音字符串 */
    char decoded[128];
    size_t dec_len = 0;
    if (mccp_decode(tokens, tok_count, decoded, sizeof(decoded), &dec_len) == 0) {
        printf("  解码回拼音: %s\n", dec_len ? decoded : "(空)");
    } else {
        printf("  ❌ 解码失败\n");
        return 1;
    }

    printf("\n结果: 1 通过\n");
    return 0;
}

/* 测试5: 感知场统计 */
static int test_perception_stats(void) {
    printf("\n═══ 测试5: 感知场裁判统计 ═══\n\n");
    uint32_t admit = 0, reject = 0;
    hanzi_get_judge_stats(&admit, &reject);
    printf("  裁判统计: 准入 %u, 拒绝 %u\n", admit, reject);
    printf("  表内汉字数: %d\n", hanzi_count());
    printf("\n结果: 1 通过\n");
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * main
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  Hunyuan-OS Encoding Test Suite         ║\n");
    printf("║  满全法编码系统 · 容器流水线测试         ║\n");
    printf("╚══════════════════════════════════════════╝\n");

    hanzi_init();
    mccp_encdec_init();

    int total_fail = 0;
    total_fail += test_hanzi_pipeline();
    total_fail += test_husk_detect();
    total_fail += test_husk_full_flow();
    total_fail += test_mccp_codec();
    total_fail += test_perception_stats();

    printf("\n═══════════════════════════════════════════\n");
    if (total_fail == 0) {
        printf("✅ 全部通过 · 容器流水线通畅\n");
    } else {
        printf("⚠️  %d 项失败 · 需排查\n", total_fail);
    }
    printf("═══════════════════════════════════════════\n");

    return total_fail ? 1 : 0;
}
