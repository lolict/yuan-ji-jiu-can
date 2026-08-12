/**
 * test_gtxone.c — 融合中心单元测试
 */

#include <stdio.h>
#include <assert.h>
#include <math.h>
#include "gtxone.h"

/* 测试1：创建融合中心 */
static void test_create(void) {
    gtxone_t g = gtxone_create(0, 1, 0);
    assert(g.id > 0);
    assert(g.partner_a == 0);
    assert(g.partner_b == 1);
    assert(g.state == FUSION_NONE);
    assert(g.fusion_strength == 0.0f);
    assert(!g.is_unique);
    assert(g.generation == 0);
    printf("[PASS] test_create\n");
}

/* 测试2：融合推进 */
static void test_fusion_progression(void) {
    gtxone_t g = gtxone_create(0, 1, 0);

    /* 小增量 → 仍未融合 */
    gtxone_fuse(&g, 0.1f);
    assert(g.state == FUSION_NONE);
    assert(!g.is_unique);

    /* 继续推进 → 部分融合 */
    gtxone_fuse(&g, 0.4f);
    assert(g.state == FUSION_PARTIAL);
    assert(!g.is_unique);

    /* 推过阈值 → 唯一态 */
    bool done = gtxone_fuse(&g, 0.5f);
    assert(done);
    assert(g.state == FUSION_UNIQUE);
    assert(g.is_unique);
    printf("[PASS] test_fusion_progression\n");
}

/* 测试3：融合强度上限 */
static void test_fusion_cap(void) {
    gtxone_t g = gtxone_create(0, 1, 0);
    gtxone_fuse(&g, 0.7f);
    gtxone_fuse(&g, 0.5f);  /* 不应超过 1.0 */
    assert(g.fusion_strength <= 1.0f);
    printf("[PASS] test_fusion_cap\n");
}

/* 测试4：内化差异 */
static void test_internalize_diff(void) {
    gtxone_t g = gtxone_create(0, 1, 0);
    assert(gtxone_internalize_difference(&g, 0.1f));
    assert(gtxone_internalize_difference(&g, 0.5f));
    assert(g.internal_diff_count == 2);
    assert(fabsf(g.internal_diffs[0] - 0.1f) < 0.001f);
    assert(fabsf(g.internal_diffs[1] - 0.5f) < 0.001f);
    printf("[PASS] test_internalize_diff\n");
}

/* 测试5：未融合不能管辖 */
static void test_govern_requires_fusion(void) {
    gtxone_t g = gtxone_create(0, 1, 0);

    /* 未融合态：管辖应失败 */
    bool ok = gtxone_govern(&g, 100);
    assert(!ok);
    assert(g.governed_count == 0);
    assert(gtxone_power(&g) == 0);

    /* 融合后：管辖成功 */
    gtxone_fuse(&g, 1.0f);
    ok = gtxone_govern(&g, 100);
    assert(ok);
    assert(g.governed_count == 1);
    assert(gtxone_power(&g) == 1);
    printf("[PASS] test_govern_requires_fusion\n");
}

/* 测试6：视角切换 */
static void test_view_switching(void) {
    gtxone_t g = gtxone_create(0, 1, 0);
    gtxone_fuse(&g, 1.0f);
    gtxone_internalize_difference(&g, 0.3f);
    gtxone_govern(&g, 200);

    uint32_t count;
    const uint32_t* outward = gtxone_view_outward(&g, &count);
    assert(count == 1);
    assert(outward[0] == 200);

    const float* inward = gtxone_view_inward(&g, &count);
    assert(count == 1);
    assert(fabsf(inward[0] - 0.3f) < 0.001f);
    printf("[PASS] test_view_switching\n");
}

/* 测试7：唯一中心验证 */
static void test_unique_center_value(void) {
    assert(is_unique_center(GTXONE_VALUE));
    assert(!is_unique_center(0x00));
    assert(!is_unique_center(0x02));
    assert(!is_unique_center(0x3F));
    printf("[PASS] test_unique_center_value (GTXONE=0x01)\n");
}

/* 测试8：信息密度 */
static void test_density(void) {
    gtxone_t g = gtxone_create(0, 1, 0);
    gtxone_fuse(&g, 1.0f);

    /* 无内容 → 密度为0 */
    assert(gtxone_density(&g) == 0.0f);

    /* 有内容 */
    gtxone_internalize_difference(&g, 0.5f);
    gtxone_internalize_difference(&g, 0.5f);
    gtxone_govern(&g, 100);
    gtxone_govern(&g, 101);

    float d = gtxone_density(&g);
    /* 密度 = 2(差异) × 1.0(强度) / 2(管辖) = 1.0 */
    assert(fabsf(d - 1.0f) < 0.001f);
    printf("[PASS] test_density (%.4f)\n", d);
}

int main(void) {
    printf("=== gtxone 融合中心 单元测试 ===\n\n");
    test_create();
    test_fusion_progression();
    test_fusion_cap();
    test_internalize_diff();
    test_govern_requires_fusion();
    test_view_switching();
    test_unique_center_value();
    test_density();
    printf("\n=== 全部测试通过 ===\n");
    return 0;
}
