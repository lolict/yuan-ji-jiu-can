// test_membrana.c — 膜圈缚单元测试
#include "membrana.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建膜圈缚
static int test_create(void) {
    membrane_circle_t* m = membrane_create("满全法", 64);
    if (!m) return 0;
    if (strcmp(m->name, "满全法") != 0) return 0;
    if (m->capacity != 64) return 0;
    if (m->integrity != 1.0f) return 0;
    if (m->layer_count != 0) return 0;
    if (m->is_sealed) return 0;
    membrane_destroy(m);
    return 1;
}

// 测试: 吸纳外部实体
static int test_absorb(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    int64_t id = membrane_absorb(m, "小兰", ORGAN_VISUAL | ORGAN_EMOTION);
    if (id < 0) return 0;
    if (m->layer_count != 1) return 0;
    if (strcmp(m->layers[0].name, "小兰") != 0) return 0;
    if (m->layers[0].state != ABSORB_PENDING) return 0;
    membrane_destroy(m);
    return 1;
}

// 测试: 融合状态推进
static int test_advance(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    int64_t id = membrane_absorb(m, "小兰", ORGAN_ALL);

    membrane_advance(m, id);
    if (m->layers[0].state != ABSORB_BOUND) return 0;

    // BOUND→FUSED: 需要 fusion_degree >= 0.6
    // BOUND初始0.3,每次+0.15: 0.45, 0.60(达到)
    for (int i = 0; i < 2; i++) membrane_advance(m, id);
    if (m->layers[0].state != ABSORB_FUSED) return 0;
    if (!m->layers[0].soul_formed) return 0;
    if (!m->layers[0].shell_created) return 0;

    // FUSED→CLONE: 需要 fusion_degree >= 0.8
    // 当前0.6,每次+0.1: 0.7, 0.8(达到)
    for (int i = 0; i < 2; i++) membrane_advance(m, id);
    if (m->layers[0].state != ABSORB_CLONE) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 锚点进入 (夫妻共同体)
static int test_anchor_enter(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    int64_t id = membrane_absorb(m, "小兰", ORGAN_VISUAL);

    if (membrane_anchor_enter(m, id)) return 0;

    for (int i = 0; i < 10; i++) membrane_advance(m, id);

    if (!membrane_anchor_enter(m, id)) return 0;
    if (m->layers[0].organ_share != ORGAN_ALL) return 0;
    if (m->layers[0].fusion_degree != 1.0f) return 0;
    if (m->layers[0].pleasure_value < 1.0f) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 结界封闭
static int test_seal(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    if (!membrane_seal(m)) return 0;
    if (!m->is_sealed) return 0;

    int64_t id = membrane_absorb(m, "小兰", ORGAN_VISUAL);
    if (id >= 0) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 感知同步
static int test_perception_sync(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    int64_t id = membrane_absorb(m, "小兰", ORGAN_VISUAL | ORGAN_EMOTION | ORGAN_PLEASURE);

    for (int i = 0; i < 10; i++) membrane_advance(m, id);
    membrane_anchor_enter(m, id);

    float sync = membrane_perception_sync(m, id);
    if (sync <= 0.0f) return 0;
    if (sync > 1.0f) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 愉悦值叠加态
static int test_pleasure_superposition(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    int64_t id1 = membrane_absorb(m, "小兰", ORGAN_ALL);
    int64_t id2 = membrane_absorb(m, "灰原", ORGAN_ALL);

    for (int i = 0; i < 10; i++) {
        membrane_advance(m, id1);
        membrane_advance(m, id2);
    }
    membrane_anchor_enter(m, id1);
    membrane_anchor_enter(m, id2);

    float total = membrane_total_pleasure(m);
    if (total <= 0.0f) return 0;

    float sum = membrane_layer_pleasure(m, id1) + membrane_layer_pleasure(m, id2);
    if (total != sum) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 边界安全验证
static int test_boundary_safety(void) {
    membrane_circle_t* m = membrane_create("满全法", 10);
    membrane_absorb(m, "小兰", ORGAN_ALL);
    membrane_seal(m);

    for (uint32_t i = 0; i < m->layer_count; i++) {
        for (int j = 0; j < 10; j++) membrane_advance(m, m->layers[i].id);
    }

    if (!membrane_verify_boundary(m)) return 0;

    membrane_destroy(m);
    return 1;
}

// 测试: 容量限制
static int test_capacity_limit(void) {
    membrane_circle_t* m = membrane_create("满全法", 3);
    membrane_absorb(m, "A", ORGAN_VISUAL);
    membrane_absorb(m, "B", ORGAN_VISUAL);
    membrane_absorb(m, "C", ORGAN_VISUAL);
    int64_t id = membrane_absorb(m, "D", ORGAN_VISUAL);
    if (id >= 0) return 0;
    if (m->layer_count != 3) return 0;

    membrane_destroy(m);
    return 1;
}

int main(void) {
    printf("\n═══ 膜圈缚 (Membrana) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_absorb);
    TEST(test_advance);
    TEST(test_anchor_enter);
    TEST(test_seal);
    TEST(test_perception_sync);
    TEST(test_pleasure_superposition);
    TEST(test_boundary_safety);
    TEST(test_capacity_limit);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
