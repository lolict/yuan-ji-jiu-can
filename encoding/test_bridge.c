/**
 * test_bridge.c — 编码桥接 API 测试
 *
 * 验证 encoding_bridge.h 的便捷函数
 * 确保其他模块(C/Rust/Zig)可通过此桥接使用编码系统
 */

#include <stdio.h>
#include <string.h>
#include "encoding_bridge.h"

static int test_bridge_api(void) {
    printf("\n═══ 桥接 API 测试 ═══\n\n");
    int pass = 0, fail = 0;

    /* 初始化 */
    encoding_init();

    /* 测试1: UTF-8 → 容器编码 */
    const char *test_hz[] = {"回", "好", "一", "国", "人", "我", "你", "明", "看", "想", NULL};
    printf("── UTF-8 → 容器编码 ──\n");
    for (int i = 0; test_hz[i]; i++) {
        uint32_t code = encoding_utf8_to_container(test_hz[i]);
        if (code) {
            printf("  「%s」→ 0x%08X ✓\n", test_hz[i], code);
            pass++;
        } else {
            printf("  「%s」→ 失败 ✗\n", test_hz[i]);
            fail++;
        }
    }

    /* 测试2: 感知家族分类 */
    printf("\n── 感知家族分类 ──\n");
    struct { uint16_t ucs2; perception_family_t expected; const char *name; } family_tests[] = {
        {0x56DE, FAMILY_CONTAINER, "回"},
        {0x4E00, FAMILY_BOUNDARY,  "一"},
        {0x597D, FAMILY_COUPLE,    "好"},
        {0x7F51, FAMILY_WEB,       "网"},
        {0x65F6, FAMILY_NEUTRAL,   "时"},
        {0x4E00, FAMILY_BOUNDARY,  "二"},  /* 二也是边界 */
        {0, FAMILY_NEUTRAL, NULL}
    };
    for (int i = 0; family_tests[i].ucs2; i++) {
        perception_family_t fam = encoding_classify_family(family_tests[i].ucs2);
        if (fam == family_tests[i].expected) {
            printf("  U+%04X 「%s」→ %s ✓\n", family_tests[i].ucs2, family_tests[i].name,
                   fam == FAMILY_CONTAINER ? "CONTAINER" :
                   fam == FAMILY_BOUNDARY ? "BOUNDARY" :
                   fam == FAMILY_COUPLE ? "COUPLE" :
                   fam == FAMILY_WEB ? "WEB" : "NEUTRAL");
            pass++;
        } else {
            printf("  U+%04X 「%s」→ 期望%d 实际%d ✗\n", family_tests[i].ucs2, family_tests[i].name,
                   family_tests[i].expected, fam);
            fail++;
        }
    }

    /* 测试3: 流属性位 */
    printf("\n── 流属性位计算 ──\n");
    uint32_t bits = encoding_compute_flow_bits(0x56DE, FAMILY_CONTAINER);
    printf("  回 (CONTAINER): 0x%04X (REACHABLE=%d ACKABLE=%d TRUSTED=%d RECOVERABLE=%d)\n",
           bits,
           (bits >> 0) & 1, (bits >> 9) & 1, (bits >> 4) & 1, (bits >> 5) & 1);
    if ((bits & 0x021) == 0x021) { printf("  ✓\n"); pass++; } else { printf("  ✗\n"); fail++; }

    bits = encoding_compute_flow_bits(0x4E00, FAMILY_BOUNDARY);
    printf("  一 (BOUNDARY): 0x%04X (REACHABLE=%d ACKABLE=%d CIRCULABLE=%d INHERITABLE=%d)\n",
           bits,
           (bits >> 0) & 1, (bits >> 9) & 1, (bits >> 6) & 1, (bits >> 7) & 1);
    if ((bits & 0x021) == 0x021) { printf("  ✓\n"); pass++; } else { printf("  ✗\n"); fail++; }

    /* 测试4: 极性 */
    printf("\n── 极性判定 ──\n");
    uint8_t polarities[] = {
        encoding_get_polarity(0x56DE),  /* 回 → 阴(0) */
        encoding_get_polarity(0x4E00),  /* 一 → 阳(1) */
        encoding_get_polarity(0x597D),  /* 好 → 阴(0) */
    };
    const char *polarity_names[] = {"阴", "阳", "阴"};
    for (int i = 0; i < 3; i++) {
        const char *got = polarities[i] ? "阳" : "阴";
        if (strcmp(got, polarity_names[i]) == 0) {
            printf("  [%d] %s ✓\n", polarities[i], got);
            pass++;
        } else {
            printf("  [%d] 期望%s 实际%s ✗\n", polarities[i], polarity_names[i], got);
            fail++;
        }
    }

    /* 测试5: 汉字存在查询 */
    printf("\n── 汉字存在查询 ──\n");
    if (encoding_has_hanzi(0x56DE)) { printf("  回 ✓\n"); pass++; } else { printf("  回 ✗\n"); fail++; }
    if (encoding_has_hanzi(0x4E00)) { printf("  一 ✓\n"); pass++; } else { printf("  一 ✗\n"); fail++; }
    if (!encoding_has_hanzi(0x9FFF)) { printf("  未收录字 ✓\n"); pass++; } else { printf("  未收录字 ✗\n"); fail++; }

    /* 测试6: 计数 */
    printf("\n── 编码表状态 ──\n");
    printf("  表内汉字数: %d\n", encoding_hanzi_count());
    if (encoding_hanzi_count() >= 100) { printf("  ✓\n"); pass++; } else { printf("  ✗\n"); fail++; }

    printf("\n结果: %d 通过, %d 失败\n", pass, fail);
    return fail;
}

int main(void) {
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  Hunyuan-Encoding Bridge Test Suite     ║\n");
    printf("╚══════════════════════════════════════════╝\n");

    int fail = test_bridge_api();

    printf("\n═══════════════════════════════════════════\n");
    if (fail == 0) {
        printf("✅ 桥接 API 全部通过 · 可供 kernel/shell/cloudflare 使用\n");
    } else {
        printf("⚠️  %d 项失败\n", fail);
    }
    printf("═══════════════════════════════════════════\n");

    return fail ? 1 : 0;
}
