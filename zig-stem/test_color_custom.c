// test_color_custom.c — 六系42色颜色定制测试
// 验证: 42色完整性 + 64符号→42色映射 + 六系分配

#include "color_custom.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

// 测试1: 42色完整定义
void test_42_colors_defined(void) {
    for (int i = 0; i < CC_TOTAL_COLORS; i++) {
        const cc_color_info_t* ci = cc_get_color_info(i);
        assert(ci != NULL);
        assert(ci->name != NULL);
        assert(strlen(ci->name) > 0);
        assert(ci->system < CC_SYSTEM_COUNT);
        assert(ci->row < CC_SYSTEM_COUNT);
        assert(ci->col < CC_COLOR_PER_SYSTEM);
    }
    printf("  ✓ 42色全部定义测试通过\n");
}

// 测试2: 六系名称/品质
void test_six_systems(void) {
    assert(strcmp(cc_system_name(CS_RED), "红系(暖)") == 0);
    assert(strcmp(cc_system_name(CS_YELLOW), "黄系(光)") == 0);
    assert(strcmp(cc_system_name(CS_GREEN), "绿系(生)") == 0);
    assert(strcmp(cc_system_name(CS_BLUE), "蓝系(冷)") == 0);
    assert(strcmp(cc_system_name(CS_STONE), "泽系(石)") == 0);
    assert(strcmp(cc_system_name(CS_LIGHT), "光系(韵)") == 0);

    assert(strcmp(cc_system_quality(CS_RED), "暖") == 0);
    assert(strcmp(cc_system_quality(CS_LIGHT), "韵") == 0);

    printf("  ✓ 六系名称/品质测试通过 (暖/光/生/冷/石/韵)\n");
}

// 测试3: 第1行红系7色
void test_red_system(void) {
    // 妃粉彤赤棕绛赭
    const cc_color_info_t* c0 = cc_get_color_info(CC_FEI);
    assert(strcmp(c0->name, "妃") == 0);
    assert(strcmp(c0->pinyin, "fēi") == 0);
    assert(c0->system == CS_RED);

    const cc_color_info_t* c6 = cc_get_color_info(CC_ZHE);
    assert(strcmp(c6->name, "赭") == 0);
    assert(c6->col == 6);

    printf("  ✓ 红系7色测试通过 (妃fēi→赭zhě)\n");
}

// 测试4: 第6行光系7色
void test_light_system(void) {
    // 曦辉霓旖靡暝黟
    const cc_color_info_t* c0 = cc_get_color_info(CC_XI);
    assert(strcmp(c0->name, "曦") == 0);
    assert(strcmp(c0->quality, "韵") == 0);

    const cc_color_info_t* c6 = cc_get_color_info(CC_YI2);
    assert(strcmp(c6->name, "黟") == 0);
    assert(c6->row == 5 && c6->col == 6);

    printf("  ✓ 光系7色测试通过 (曦xī→黟yī)\n");
}

// 测试5: 64符号颜色映射
void test_64_symbol_colors(void) {
    int colored = 0;
    for (int i = 0; i < CC_SYMBOL_COUNT; i++) {
        const cc_color_symbol_t* cs = cc_get_symbol_color(i);
        if (cs && cs->color_name && strlen(cs->color_name) > 0) {
            colored++;
        }
    }
    assert(colored == 64);
    printf("  ✓ 64符号颜色映射测试通过 (全部着色)\n");
}

// 测试6: 夫妻层→红系
void test_couple_layer_red(void) {
    // SMV_YMBA=0, SMV_CEIQ=1 → 红系
    const cc_color_symbol_t* cs0 = cc_get_symbol_color(SMV_YMBA);
    const cc_color_symbol_t* cs1 = cc_get_symbol_color(SMV_CEIQ);
    assert(cs0->system == CS_RED);
    assert(cs1->system == CS_RED);
    assert(strcmp(cs0->color_name, "妃") == 0);
    assert(strcmp(cs1->color_name, "粉") == 0);

    printf("  ✓ 夫妻层→红系测试通过 (啊=妃, 哦=粉)\n");
}

// 测试7: 高维层→光系
void test_high_dim_light(void) {
    // 高维层部分符号→光系
    const cc_color_symbol_t* cs = cc_get_symbol_color(SMV_4D);
    assert(cs->system == CS_LIGHT || cs->system == CS_BLUE);
    printf("  ✓ 高维层颜色测试通过 (%s→%s)\n",
        smv_get_hanzi(SMV_4D), cs->color_name);
}

int main(void) {
    printf("\n  ═══ 六系42色颜色定制测试 ═══\n\n");

    test_42_colors_defined();
    test_six_systems();
    test_red_system();
    test_light_system();
    test_64_symbol_colors();
    test_couple_layer_red();
    test_high_dim_light();

    printf("\n  ═══ 测试结果: 7/7 通过 ═══\n");
    printf("  42色完整 ✓ | 六系品质 ✓ | 红系7色 ✓\n");
    printf("  光系7色 ✓ | 64映射 ✓ | 夫妻→红 ✓\n\n");

    return 0;
}
