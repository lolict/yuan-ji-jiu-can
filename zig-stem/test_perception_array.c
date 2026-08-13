// test_perception_array.c — 感知一体化阵列测试
// 验证: 五维叠加+感知状态机+同在论+关系代数

#include "perception_array.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

// 测试1: 创建/销毁
void test_create_destroy(void) {
    perception_array_t* pa = pa_create();
    assert(pa != NULL);
    assert(pa->count == 0);
    assert(pa->global_state == PS_IDLE);
    assert(pa->is_co_present == false);
    pa_destroy(pa);
    printf("  ✓ 创建/销毁测试通过\n");
}

// 测试2: 添加感知单元 (五维叠加)
void test_add_percept(void) {
    perception_array_t* pa = pa_create();
    assert(pa != NULL);

    pa_color_t color = {.hue = 30, .saturation = 50, .lightness = 40};
    pa_sound_t sound = {.shengmu = 3, .yunmu = 12, .tone = 2};
    pa_stroke_t stroke = {.stroke_count = 8, .stroke_order = {1,2,3,4,5,0,0,0}, .radical = 15};
    pa_wubi_t wubi = {.key_primary = 7, .key_secondary = 14, .zone = 1, .position = 3};
    pa_category_t category = {.object = 5, .morphism = 10, .functor = 20};

    uint32_t id = pa_add_percept(pa, "测试感知", &color, &sound, &stroke, &wubi, &category);
    assert(id == 0);
    assert(pa->count == 1);

    // 验证颜色角度 = hue × 6
    assert(pa->percepts[0].color.angle == 30.0f * 6.0f);
    // 验证笔画梯度在0-1范围
    assert(pa->percepts[0].stroke.gradient >= 0.0f && pa->percepts[0].stroke.gradient <= 1.0f);
    // 验证感知强度在0-1范围
    assert(pa->percepts[0].perception_strength >= 0.0f && pa->percepts[0].perception_strength <= 1.0f);

    pa_destroy(pa);
    printf("  ✓ 添加感知单元测试通过\n");
}

// 测试3: 感知状态机推进
void test_state_machine(void) {
    perception_array_t* pa = pa_create();

    pa_color_t c = {.hue = 10, .saturation = 40, .lightness = 30};
    pa_sound_t s = {.shengmu = 5, .yunmu = 8, .tone = 1};
    pa_stroke_t st = {.stroke_count = 5, .radical = 10};
    pa_wubi_t w = {.key_primary = 3, .zone = 0, .position = 2};
    pa_category_t cat = {.object = 2, .morphism = 4, .functor = 8};

    uint32_t id = pa_add_percept(pa, "状态测试", &c, &s, &st, &w, &cat);
    assert(id == 0);

    // 初始状态: IDLE
    assert(pa_get_state(pa, 0) == PS_IDLE);

    // 推进6步: IDLE→PERCEIVING→RECOGNIZING→ASSOCIATING→SUPERPOSING→PRESENT
    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_PERCEIVING);

    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_RECOGNIZING);

    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_ASSOCIATING);

    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_SUPERPOSING);

    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_PRESENT);

    // 稳定态保持
    pa_advance_state(pa, 0);
    assert(pa_get_state(pa, 0) == PS_PRESENT);

    pa_destroy(pa);
    printf("  ✓ 感知状态机测试通过\n");
}

// 测试4: 叠加态计算
void test_superposition(void) {
    perception_array_t* pa = pa_create();

    // 添加两个不同感知单元, 验证叠加结果不同
    pa_color_t c1 = {.hue = 0, .saturation = 63, .lightness = 32};    // 纯红
    pa_color_t c2 = {.hue = 32, .saturation = 63, .lightness = 32};   // 绿

    pa_sound_t s1 = {.shengmu = 0, .yunmu = 0, .tone = 0};
    pa_sound_t s2 = {.shengmu = 10, .yunmu = 20, .tone = 3};

    pa_stroke_t st1 = {.stroke_count = 1, .radical = 1};
    pa_stroke_t st2 = {.stroke_count = 30, .radical = 40};

    pa_wubi_t w1 = {.key_primary = 0, .zone = 0, .position = 0};
    pa_wubi_t w2 = {.key_primary = 20, .zone = 4, .position = 4};

    pa_category_t cat1 = {.object = 1, .morphism = 1, .functor = 1};
    pa_category_t cat2 = {.object = 50, .morphism = 40, .functor = 30};

    uint32_t id1 = pa_add_percept(pa, "红一", &c1, &s1, &st1, &w1, &cat1);
    uint32_t id2 = pa_add_percept(pa, "绿二", &c2, &s2, &st2, &w2, &cat2);

    // 叠加后的符号应该不同
    b64_symbol_t sym1 = pa_superpose(pa, id1);
    b64_symbol_t sym2 = pa_superpose(pa, id2);
    assert(sym1 != sym2);

    pa_destroy(pa);
    printf("  ✓ 叠加态计算测试通过\n");
}

// 测试5: 同在论
void test_co_presence(void) {
    perception_array_t* pa = pa_create();

    pa_color_t c = {.hue = 20, .saturation = 50, .lightness = 40};
    pa_sound_t s = {.shengmu = 5, .yunmu = 10, .tone = 2};
    pa_stroke_t st = {.stroke_count = 10, .radical = 20};
    pa_wubi_t w = {.key_primary = 12, .zone = 2, .position = 3};
    pa_category_t cat = {.object = 15, .morphism = 25, .functor = 35};

    uint32_t id = pa_add_percept(pa, "同在测试", &c, &s, &st, &w, &cat);

    // 初始: 未到达叠加态, 不共在
    assert(pa_is_co_present(pa, id) == false);

    // 推进到PRESENT
    for (int i = 0; i < 6; i++) {
        pa_advance_state(pa, id);
    }

    // 应该共在
    assert(pa_is_co_present(pa, id) == true);
    assert(pa->is_co_present == true);

    pa_destroy(pa);
    printf("  ✓ 同在论测试通过\n");
}

// 测试6: 关系代数 (相似度)
void test_similarity(void) {
    perception_array_t* pa = pa_create();

    // 两个完全相同的感知单元
    pa_color_t c = {.hue = 15, .saturation = 40, .lightness = 35};
    pa_sound_t s = {.shengmu = 5, .yunmu = 10, .tone = 2};
    pa_stroke_t st = {.stroke_count = 8, .radical = 16};
    pa_wubi_t w = {.key_primary = 10, .zone = 2, .position = 0};
    pa_category_t cat = {.object = 10, .morphism = 20, .functor = 30};

    uint32_t id1 = pa_add_percept(pa, "相同1", &c, &s, &st, &w, &cat);
    uint32_t id2 = pa_add_percept(pa, "相同2", &c, &s, &st, &w, &cat);

    // 完全相同的感知单元相似度应接近1
    float sim = pa_similarity(pa, id1, id2);
    assert(sim > 0.9f);

    // 完全不同的感知单元
    pa_color_t c3 = {.hue = 60, .saturation = 10, .lightness = 60};
    pa_sound_t s3 = {.shengmu = 20, .yunmu = 35, .tone = 4};
    pa_stroke_t st3 = {.stroke_count = 60, .radical = 55};
    pa_wubi_t w3 = {.key_primary = 24, .zone = 4, .position = 4};
    pa_category_t cat3 = {.object = 60, .morphism = 50, .functor = 40};

    uint32_t id3 = pa_add_percept(pa, "不同", &c3, &s3, &st3, &w3, &cat3);

    // 不同感知单元相似度应较低
    float sim_diff = pa_similarity(pa, id1, id3);
    assert(sim_diff < sim);

    pa_destroy(pa);
    printf("  ✓ 相似度测试通过\n");
}

// 测试7: 按相位排序
void test_sort_by_phase(void) {
    perception_array_t* pa = pa_create();

    // 添加多个感知单元, 不同色相 → 不同角度
    for (int i = 0; i < 6; i++) {
        pa_color_t c = {(uint8_t)(i * 10), 50, 35, 0, 0};
        pa_sound_t s = {5, 10, 2, 0, 0};
        pa_stroke_t st = {8, {1,2,3}, 16, 0, 0};
        pa_wubi_t w = {10, 5, 2, 0, 0, 0};
        pa_category_t cat = {10, 20, 30, 0, 0};

        char label[16];
        snprintf(label, sizeof(label), "感知%d", i);
        pa_add_percept(pa, label, &c, &s, &st, &w, &cat);
    }

    assert(pa->count == 6);

    // 排序
    pa_sort_by_phase(pa);

    // 排序后应按 unified_symbol/angle 递增
    for (uint32_t i = 1; i < pa->count; i++) {
        float angle_prev = (float)pa->percepts[i-1].unified_symbol * 6.0f;
        float angle_curr = (float)pa->percepts[i].unified_symbol * 6.0f;
        assert(angle_curr >= angle_prev);
    }

    pa_destroy(pa);
    printf("  ✓ 按相位排序测试通过\n");
}

// 测试8: 关系强度
void test_relation_strength(void) {
    perception_array_t* pa = pa_create();

    pa_color_t c = {.hue = 25, .saturation = 55, .lightness = 40};
    pa_sound_t s = {.shengmu = 7, .yunmu = 15, .tone = 1};
    pa_stroke_t st = {.stroke_count = 12, .radical = 22};
    pa_wubi_t w = {.key_primary = 8, .zone = 1, .position = 4};
    pa_category_t cat = {.object = 18, .morphism = 28, .functor = 38};

    uint32_t id1 = pa_add_percept(pa, "关系甲", &c, &s, &st, &w, &cat);
    uint32_t id2 = pa_add_percept(pa, "关系乙", &c, &s, &st, &w, &cat);

    // 推进状态以增加感知强度
    for (int i = 0; i < 5; i++) {
        pa_advance_state(pa, id1);
        pa_advance_state(pa, id2);
    }

    float strength = pa_relation_strength(pa, id1, id2);
    assert(strength > 0.0f);
    assert(strength <= 1.0f);

    pa_destroy(pa);
    printf("  ✓ 关系强度测试通过\n");
}

int main(void) {
    printf("\n  ═══ 感知一体化阵列测试 ═══\n\n");

    test_create_destroy();
    test_add_percept();
    test_state_machine();
    test_superposition();
    test_co_presence();
    test_similarity();
    test_sort_by_phase();
    test_relation_strength();

    printf("\n  ═══ 测试结果: 8/8 通过 ═══\n");
    printf("  五维叠加 ✓ | 状态机 ✓ | 叠加态 ✓ | 同在 ✓\n");
    printf("  相似度 ✓ | 排序 ✓ | 关系强度 ✓\n\n");

    return 0;
}
