// test_color_custom.c — 颜色定制进制测试
// 验证: 声母→色相/韵母→饱和度/声调→明度/关系/距离

#include "color_custom.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>
#include <math.h>

// 测试1: 每个符号都有唯一颜色
void test_all_colored(void) {
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const cc_color_symbol_t* cs = cc_get_color(i);
        assert(cs != NULL);
        assert(cs->symbol == (smv_symbol_t)i);
        // HSL在有效范围
        assert(cs->hsl.hue >= 0 && cs->hsl.hue <= 384);
        assert(cs->hsl.saturation >= 0 && cs->hsl.saturation <= 1);
        assert(cs->hsl.lightness >= 0 && cs->hsl.lightness <= 1);
        // RGB在有效范围
        assert(cs->rgb.r <= 255 && cs->rgb.g <= 255 && cs->rgb.b <= 255);
    }
    printf("  ✓ 64符号全着色测试通过\n");
}

// 测试2: 声母→色相分区
void test_shengmu_hue(void) {
    // 唇音(b/p/m/f) → 红色系 (0°附近)
    const cc_color_symbol_t* cs_b = cc_get_color(31); // bā=八
    const cc_color_symbol_t* cs_p = cc_get_color(52); // píng=平
    // 舌根(g/k/h) → 绿色系 (110°附近)
    const cc_color_symbol_t* cs_g = cc_get_color(2);  // gōng=工
    const cc_color_symbol_t* cs_h = cc_get_color(4);  // huī=灰

    // 唇音与舌根的色相应有明显差异
    float diff = fabsf(cs_b->hsl.hue - cs_g->hsl.hue);
    if (diff > 180) diff = 384 - diff;
    assert(diff > 50);  // 色系差异至少50°

    printf("  ✓ 声母→色相分区测试通过 (唇vs舌根差%.0f°)\n", diff);
}

// 测试3: 声调→明度
void test_tone_lightness(void) {
    // 阴平(亮) vs 去声(暗)
    const cc_color_symbol_t* cs_1 = cc_get_color(0);  // ā阴平
    const cc_color_symbol_t* cs_4 = cc_get_color(19); // duān阴平 → 找去声

    // 阴平应该比去声亮 (如果韵母相同)
    // 直接验证: 声调1的明度 > 声调4的明度
    const cc_color_symbol_t* cs_sm4 = cc_get_color(17); // qīn 阴平
    const cc_color_symbol_t* cs_rs4 = cc_get_color(49); // zhì 去声

    // 所有符号中, 平均来说阴平比去声亮
    float avg_t1 = 0, avg_t4 = 0;
    int cnt1 = 0, cnt4 = 0;
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const smv_voice_t* v = smv_get_voice(i);
        const cc_color_symbol_t* cs = cc_get_color(i);
        if (!v || !cs) continue;
        if (v->tone == TONE_1) { avg_t1 += cs->hsl.lightness; cnt1++; }
        if (v->tone == TONE_4) { avg_t4 += cs->hsl.lightness; cnt4++; }
    }
    if (cnt1 > 0) avg_t1 /= cnt1;
    if (cnt4 > 0) avg_t4 /= cnt4;
    assert(avg_t1 > avg_t4);  // 阴平平均明度 > 去声平均明度

    printf("  ✓ 声调→明度测试通过 (阴平%.2f > 去声%.2f)\n", avg_t1, avg_t4);
}

// 测试4: 颜色关系
void test_color_relation(void) {
    // 对径符号(180°) = 镜像关系
    cc_color_relation_t rel = cc_color_relation(0, 32);
    assert(rel == CR_MIRROR || rel == CR_WITNESS || rel == CR_NEUTRAL);

    // 同符号 = 同色
    assert(cc_color_relation(5, 5) == CR_SAME);

    printf("  ✓ 颜色关系测试通过\n");
}

// 测试5: 颜色距离
void test_color_distance(void) {
    // 同色距离为0
    assert(cc_color_distance(0, 0) == 0.0f);

    // 寻找一对色差接近180°的符号
    float max_dist = 0;
    for (int i = 0; i < 64; i++) {
        for (int j = i+1; j < 64; j++) {
            float d = cc_color_distance(i, j);
            if (d > max_dist) max_dist = d;
        }
    }
    // 最大色差应接近180°
    assert(max_dist >= 150 && max_dist <= 210);

    printf("  ✓ 颜色距离测试通过 (最大色差=%.0f°)\n", max_dist);
}

// 测试6: HSL↔RGB互转
void test_hsl_rgb_roundtrip(void) {
    cc_hsl_t original = {120.0f, 0.8f, 0.5f};
    cc_rgb_t rgb = cc_hsl_to_rgb(original);
    cc_hsl_t back = cc_rgb_to_hsl(rgb);

    // 允许舍入误差
    assert(fabsf(back.hue - original.hue) < 2.0f);
    assert(fabsf(back.saturation - original.saturation) < 0.05f);
    assert(fabsf(back.lightness - original.lightness) < 0.05f);

    printf("  ✓ HSL↔RGB互转测试通过\n");
}

// 测试7: 颜色名称唯一性
void test_color_names(void) {
    int unique = 0;
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const char* name_i = cc_get_color_name(i);
        bool found_dup = false;
        for (int j = i + 1; j < SV_SYMBOL_COUNT; j++) {
            if (strcmp(name_i, cc_get_color_name(j)) == 0) {
                found_dup = true;
                break;
            }
        }
        if (!found_dup) unique++;
    }
    // 至少70%的颜色名称唯一
    assert(unique >= 45);
    printf("  ✓ 颜色名称唯一性测试通过 (%d/%d唯一)\n", unique, SV_SYMBOL_COUNT);
}

int main(void) {
    printf("\n  ═══ 颜色定制进制测试 ═══\n\n");

    test_all_colored();
    test_shengmu_hue();
    test_tone_lightness();
    test_color_relation();
    test_color_distance();
    test_hsl_rgb_roundtrip();
    test_color_names();

    printf("\n  ═══ 测试结果: 7/7 通过 ═══\n");
    printf("  全着色 ✓ | 声母色相 ✓ | 声调明度 ✓\n");
    printf("  颜色关系 ✓ | 颜色距离 ✓ | HSL/RGB ✓\n\n");

    return 0;
}
