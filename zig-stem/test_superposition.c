/**
 * test_superposition.c — 叠加态单元测试
 */

#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include "superposition.h"

/* 测试1：叠加态创建 */
static void test_superposition_create(void) {
    superposition_t s = superposition_create(0, 1, 0);
    assert(s.id > 0);
    assert(s.type == REL_COUPLE);
    assert(s.inner_a == 0);
    assert(s.inner_b == 1);
    assert(s.generation == 0);
    assert(!s.is_closed);
    printf("[PASS] test_superposition_create\n");
}

/* 测试2：层级创建 */
static void test_layer_create(void) {
    superposition_t couple = superposition_create(0, 1, 0);
    layer_t layer = layer_create(&couple, 0);
    assert(layer.layer_id > 0);
    assert(layer.state == LAYER_OPEN);
    assert(layer.couple_id == couple.id);
    assert(layer.alignment_count == 0);
    assert(layer.content_count == 0);
    assert(layer.generation == 0);
    printf("[PASS] test_layer_create\n");
}

/* 测试3：对齐操作 */
static void test_align_single(void) {
    superposition_t couple = superposition_create(0, 1, 0);
    layer_t layer = layer_create(&couple, 0);
    superposition_t content = superposition_create(10, 11, 0);

    align_result r = layer_align(&layer, &content);
    assert(r == ALIGN_OK);
    assert(layer.alignment_count == 1);
    assert(layer.content_count == 1);
    assert(layer.state == LAYER_ALIGNING);
    printf("[PASS] test_align_single\n");
}

/* 测试4：254次对齐达到阈值 */
static void test_align_threshold(void) {
    superposition_t couple = superposition_create(0, 1, 0);
    layer_t layer = layer_create(&couple, 0);
    superposition_t c1 = superposition_create(10, 11, 0);
    superposition_t c2 = superposition_create(12, 13, 0);

    align_result r = ALIGN_OK;
    for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
        superposition_t* content = (i % 2 == 0) ? &c1 : &c2;
        r = layer_align(&layer, content);
    }

    assert(r == ALIGN_THRESHOLD);
    assert(layer.alignment_count == ALIGNMENT_THRESHOLD);
    assert(layer.state == LAYER_CLOSED);
    assert(is_alignment_complete(layer.alignment_count));
    assert(alignment_progress(layer.alignment_count) == 1.0f);
    printf("[PASS] test_align_threshold (254次对齐)\n");
}

/* 测试5：闭合后不能再对齐 */
static void test_align_after_close(void) {
    superposition_t couple = superposition_create(0, 1, 0);
    layer_t layer = layer_create(&couple, 0);
    superposition_t c = superposition_create(10, 11, 0);

    for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
        layer_align(&layer, &c);
    }
    assert(layer.state == LAYER_CLOSED);

    align_result r = layer_align(&layer, &c);
    assert(r == ALIGN_ERROR_CLOSED);
    printf("[PASS] test_align_after_close\n");
}

/* 测试6：升级操作 */
static void test_promote(void) {
    superposition_t couple = superposition_create(0, 1, 0);
    layer_t layer = layer_create(&couple, 0);
    superposition_t c = superposition_create(10, 11, 0);

    // 未闭合时升级失败
    promote_result pr = layer_promote(&layer);
    assert(!pr.success);

    // 完成对齐后升级成功
    for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
        layer_align(&layer, &c);
    }
    pr = layer_promote(&layer);
    assert(pr.success);
    assert(pr.new_content_id == couple.id);
    assert(pr.new_generation == 1);
    assert(layer.state == LAYER_PROMOTED);
    printf("[PASS] test_promote (管辖者→被管辖者)\n");
}

/* 测试7：64进制换算 */
static void test_base64_conversion(void) {
    base64_pair p = to_base64(254);
    assert(p.high == 3);
    assert(p.low == 62);

    base64_pair p2 = to_base64(0);
    assert(p2.high == 0);
    assert(p2.low == 0);

    base64_pair p3 = to_base64(255);
    assert(p3.high == 3);
    assert(p3.low == 63);
    printf("[PASS] test_base64_conversion (254=3:62)\n");
}

/* 测试8：递归状态计算 */
static void test_recursive_status(void) {
    superposition_t c1 = superposition_create(0, 1, 0);
    superposition_t c2 = superposition_create(2, 3, 1);
    layer_t layers[2];
    layers[0] = layer_create(&c1, 0);
    layers[1] = layer_create(&c2, 1);

    // 初始状态：未闭合
    recursive_status s = compute_recursive_status(layers, 2);
    assert(s.total_layers == 2);
    assert(!s.is_fully_closed);
    assert(s.current_generation == 1);

    // 闭合第0层
    superposition_t content = superposition_create(10, 11, 0);
    for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
        layer_align(&layers[0], &content);
    }
    layer_promote(&layers[0]);

    s = compute_recursive_status(layers, 2);
    // 第0层已 PROMOTED，第1层仍然 OPEN
    assert(!s.is_fully_closed);
    printf("[PASS] test_recursive_status\n");
}

/* 测试9：代数递增验证 */
static void test_generation_chain(void) {
    superposition_t c0 = superposition_create(0, 1, 0);
    layer_t l0 = layer_create(&c0, 0);
    superposition_t ct = superposition_create(10, 11, 0);

    for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
        layer_align(&l0, &ct);
    }
    promote_result pr0 = layer_promote(&l0);
    assert(pr0.new_generation == 1);

    // 第1层
    superposition_t c1 = superposition_create(c0.id, 99, 1);
    layer_t l1 = layer_create(&c1, 1);
    assert(l1.generation == 1);

    printf("[PASS] test_generation_chain (递归代数递增)\n");
}

int main(void) {
    printf("=== 叠加态单元测试 ===\n\n");
    test_superposition_create();
    test_layer_create();
    test_align_single();
    test_align_threshold();
    test_align_after_close();
    test_promote();
    test_base64_conversion();
    test_recursive_status();
    test_generation_chain();
    printf("\n=== 全部测试通过 ===\n");
    return 0;
}
