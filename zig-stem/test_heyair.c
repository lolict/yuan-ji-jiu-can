/**
 * test_heyair.c — 灰原哀横向层单元测试
 */

#include <stdio.h>
#include <assert.h>
#include <math.h>
#include "heyair.h"

/* 测试1：创建 */
static void test_create(void) {
    heyair_t h = heyair_create();
    assert(h.id > 0);
    assert(h.state == WEAVE_EMPTY);
    assert(h.channel_count == 0);
    assert(h.nen_power == 0.0f);
    printf("[PASS] test_create\n");
}

/* 测试2：感知通道 */
static void test_add_channel(void) {
    heyair_t h = heyair_create();
    perception_channel_t* ch = heyair_add_channel(&h, PERCEIVE_BINARY, 0.9f, 100);
    assert(ch != 0);
    assert(ch->id == 1);
    assert(ch->type == PERCEIVE_BINARY);
    assert(ch->sensitivity == 0.9f);
    assert(ch->target_node == 100);
    assert(ch->is_active);
    assert(h.channel_count == 1);
    printf("[PASS] test_add_channel\n");
}

/* 测试3：感知读取 */
static void test_perceive(void) {
    heyair_t h = heyair_create();
    heyair_add_channel(&h, PERCEIVE_BINARY, 0.5f, 100);
    heyair_nen_inject(&h, 10.0f);
    heyair_nen_focus(&h, 1.0f);

    float reading = heyair_perceive(&h, 1, 1.0f);
    /* reading = 1.0 * 0.5 * 1.0 = 0.5 */
    assert(fabsf(reading - 0.5f) < 0.001f);
    printf("[PASS] test_perceive (%.2f)\n", reading);
}

/* 测试4：管辖二进制 2合1 */
static void test_merge_binary(void) {
    /* 对等合并 */
    float m = heyair_merge_binary(0.0f, 1.0f, 0.5f);
    assert(fabsf(m - 0.5f) < 0.001f);

    /* a主导 */
    m = heyair_merge_binary(0.0f, 1.0f, 0.2f);
    assert(fabsf(m - 0.2f) < 0.001f);

    /* b主导 */
    m = heyair_merge_binary(0.0f, 1.0f, 0.8f);
    assert(fabsf(m - 0.8f) < 0.001f);
    printf("[PASS] test_merge_binary\n");
}

/* 测试5：融合判断 */
static void test_is_merged(void) {
    assert(heyair_is_merged(0.5f, 0.5f, 0.01f));
    assert(heyair_is_merged(0.5f, 0.55f, 0.1f));
    assert(!heyair_is_merged(0.5f, 0.9f, 0.1f));
    printf("[PASS] test_is_merged\n");
}

/* 测试6：横向编织 */
static void test_weave(void) {
    heyair_t h = heyair_create();
    float row1[] = {1.0f, 2.0f, 3.0f};
    float row2[] = {2.0f, 3.0f, 4.0f};

    assert(heyair_weave_add_row(&h, row1, 3));
    assert(heyair_weave_add_row(&h, row2, 3));
    assert(h.row_count == 2);
    assert(h.col_count == 3);
    assert(h.state == WEAVE_ACTIVE);

    heyair_weave_align_columns(&h);
    assert(h.state == WEAVE_SETTLING);

    /* 对齐后列均值应为0 */
    for (uint32_t col = 0; col < h.col_count; col++) {
        float sum = 0.0f;
        for (uint32_t row = 0; row < h.row_count; row++) {
            sum += h.weave_matrix[row][col];
        }
        assert(fabsf(sum) < 0.001f);
    }
    printf("[PASS] test_weave\n");
}

/* 测试7：念动力衰减 */
static void test_nen_decay(void) {
    heyair_t h = heyair_create();
    heyair_nen_inject(&h, 50.0f);
    heyair_nen_focus(&h, 1.0f);

    float d0 = heyair_nen_at_distance(&h, 0);
    float d1 = heyair_nen_at_distance(&h, 1);
    float d2 = heyair_nen_at_distance(&h, 2);

    assert(fabsf(d0 - 50.0f) < 0.001f);
    assert(fabsf(d1 - 45.0f) < 0.001f);
    assert(fabsf(d2 - 40.5f) < 0.001f);
    printf("[PASS] test_nen_decay (d0=%.1f d1=%.1f d2=%.1f)\n", d0, d1, d2);
}

/* 测试8：聚类 */
static void test_cluster(void) {
    float data[] = {0.1f, 0.15f, 0.2f, 0.9f, 0.95f, 5.0f, 5.1f};
    cluster_result cr = heyair_cluster(data, 7, 0.3f);

    assert(cr.count == 3);

    /* 第一族：0.1, 0.15, 0.2 → 质心约0.15 */
    assert(cr.clusters[0].member_count == 3);
    assert(fabsf(cr.clusters[0].centroid - 0.15f) < 0.01f);

    /* 第二族：0.9, 0.95 → 质心约0.925 */
    assert(cr.clusters[1].member_count == 2);

    /* 第三族：5.0, 5.1 → 质心约5.05 */
    assert(cr.clusters[2].member_count == 2);

    printf("[PASS] test_cluster (%u 族)\n", cr.count);
}

/* 测试9：聚焦增加输出 */
static void test_focus_amplifies(void) {
    heyair_t h = heyair_create();
    heyair_add_channel(&h, PERCEIVE_BINARY, 0.5f, 100);
    heyair_nen_inject(&h, 10.0f);
    heyair_nen_focus(&h, 0.5f);

    float unfocused = heyair_perceive(&h, 1, 1.0f);

    heyair_nen_focus(&h, 1.0f);
    float focused = heyair_perceive(&h, 1, 1.0f);

    assert(focused > unfocused);
    assert(fabsf(focused - 2.0f * unfocused) < 0.001f);
    printf("[PASS] test_focus_amplifies (%.2f → %.2f)\n", unfocused, focused);
}

int main(void) {
    printf("=== heyair 灰原哀 单元测试 ===\n\n");
    test_create();
    test_add_channel();
    test_perceive();
    test_merge_binary();
    test_is_merged();
    test_weave();
    test_nen_decay();
    test_cluster();
    test_focus_amplifies();
    printf("\n=== 全部测试通过 ===\n");
    return 0;
}
