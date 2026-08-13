// test_color_custom.c — 六系42色颜色定制测试
// 验证: 42色完整 + 编码无中文语义 + 64符号映射

#include "color_custom.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

// 测试1: 42色完整定义
void test_42_colors(void) {
    for (int i = 0; i < CC_TOTAL_COLORS; i++) {
        const cc_color_info_t* ci = cc_get_color_info(i);
        assert(ci != NULL);
        assert(ci->name != NULL && strlen(ci->name) > 0);
        assert(ci->system < CC_SYSTEM_COUNT);
        assert(ci->row < CC_SYSTEM_COUNT);
        assert(ci->col < CC_COLOR_PER_SYSTEM);
    }
    printf("  ✓ 42色完整定义测试通过\n");
}

// 测试2: 编码是纯序号(无语义)
void test_codes_are_anonymous(void) {
    // C_R0=0, C_R1=1, ... C_L6=41
    assert(C_R0 == 0);
    assert(C_R6 == 6);
    assert(C_Y0 == 7);
    assert(C_L6 == 41);

    // 色系也是纯序号
    assert(CS_RED == 0);
    assert(CS_LIGHT == 5);

    printf("  ✓ 编码匿名性测试通过 (C_R0=0, C_L6=41, 纯序号)\n");
}

// 测试3: 六系品质
void test_six_systems(void) {
    assert(strcmp(cc_system_quality(CS_RED), "暖") == 0);
    assert(strcmp(cc_system_quality(CS_YELLOW), "光") == 0);
    assert(strcmp(cc_system_quality(CS_GREEN), "生") == 0);
    assert(strcmp(cc_system_quality(CS_BLUE), "冷") == 0);
    assert(strcmp(cc_system_quality(CS_STONE), "石") == 0);
    assert(strcmp(cc_system_quality(CS_LIGHT), "韵") == 0);

    printf("  ✓ 六系品质测试通过 (暖/光/生/冷/石/韵)\n");
}

// 测试4: 红系7色
void test_red_system(void) {
    const cc_color_info_t* r0 = cc_get_color_info(C_R0);
    assert(strcmp(r0->name, "妃") == 0);
    assert(r0->system == CS_RED);

    const cc_color_info_t* r6 = cc_get_color_info(C_R6);
    assert(strcmp(r6->name, "赭") == 0);

    printf("  ✓ 红系7色测试通过 (R0=妃 → R6=赭)\n");
}

// 测试5: 光系7色
void test_light_system(void) {
    const cc_color_info_t* l0 = cc_get_color_info(C_L0);
    assert(strcmp(l0->name, "曦") == 0);
    assert(l0->system == CS_LIGHT);
    assert(strcmp(l0->quality, "韵") == 0);

    const cc_color_info_t* l6 = cc_get_color_info(C_L6);
    assert(strcmp(l6->name, "黟") == 0);

    printf("  ✓ 光系7色测试通过 (L0=曦 → L6=黟)\n");
}

// 测试6: 64符号映射
void test_64_mapping(void) {
    int ok = 0;
    for (int i = 0; i < 64; i++) {
        const cc_color_symbol_t* cs = cc_get_symbol_color(i);
        if (cs && cs->color < CC_TOTAL_COLORS) ok++;
    }
    assert(ok == 64);
    printf("  ✓ 64符号颜色映射测试通过\n");
}

// 测试7: 夫妻层→红系
void test_couple_red(void) {
    assert(cc_get_symbol_system(0) == CS_RED);  // 啊→红系
    assert(cc_get_symbol_system(1) == CS_RED);  // 哦→红系

    // 显示名是中文(仅作标签)
    assert(strcmp(cc_get_color_name(0), "妃") == 0);
    assert(strcmp(cc_get_color_name(1), "粉") == 0);

    printf("  ✓ 夫妻层→红系测试通过 (符号0=R0=妃, 符号1=R1=粉)\n");
}

int main(void) {
    printf("\n  ═══ 六系42色颜色定制测试 ═══\n\n");

    test_42_colors();
    test_codes_are_anonymous();
    test_six_systems();
    test_red_system();
    test_light_system();
    test_64_mapping();
    test_couple_red();

    printf("\n  ═══ 测试结果: 7/7 通过 ═══\n");
    printf("  42色 ✓ | 编码匿名 ✓ | 六系品质 ✓\n");
    printf("  红系R0-R6 ✓ | 光系L0-L6 ✓ | 64映射 ✓\n\n");

    return 0;
}
