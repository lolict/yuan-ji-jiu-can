// test_color_ontology.c — 本体论层测试
// 验证: 正交中心/边界/二元切割/弧弦对偶/相对运动/自相似/常驻静态/状态机

#include "color_ontology.h"
#include <stdio.h>
#include <assert.h>
#include <math.h>

// ── 测试1: 正交中心 ──
void test_center(void) {
    co_center_t c = co_center_compute();
    // 中心 = 妃(R0), GCD = 1
    assert(c.center_color == C_R0);
    assert(c.gcd_value == 1.0f);
    // 元素总数 = 42
    assert(c.cardinality == 42);
    // 中心判断
    assert(co_is_center(C_R0));
    assert(!co_is_center(C_L6));
    // 中心半径 = 0
    assert(co_radius_from_center(C_R0) == 0.0f);
    printf("  ✓ 正交中心测试通过 (妃/R0=中心, GCD=1, 统摄半径>0)\n");
}

// ── 测试2: 统摄边界 ──
void test_boundary(void) {
    co_boundary_t b = co_boundary_compute();
    // 边界 = 黟(L6)
    assert(b.boundary_color == C_L6);
    // 弧弦比 = π ≈ 3.14
    assert(fabsf(b.arc_chord_ratio - 3.14159f) < 0.01f);
    // 弧长 > 曲线长
    assert(b.arc_length > b.chord_length);
    // 边界判断
    assert(co_is_boundary(C_L6));
    assert(!co_is_boundary(C_R0));
    printf("  ✓ 统摄边界测试通过 (黟/L6=边界, 弧弦比=π)\n");
}

// ── 测试3: 二元切割 ──
void test_split(void) {
    // 沿0°轴切割
    co_split_t s = co_orthogonal_split(0);
    // 两侧合计 = 总颜色数 - 轴上的颜色
    assert(s.group_pos + s.group_neg <= CC_TOTAL_COLORS);
    // 平衡度 0-1
    assert(s.balance >= 0.0f && s.balance <= 1.0f);

    // 沿90°轴切割应产生不同分布
    co_split_t s90 = co_orthogonal_split(90);
    (void)s90;  // 仅验证可执行

    // 中心(妃/R0)在任何切割下都在正交线上
    assert(co_split_side(C_R0, 0) == 0);
    assert(co_split_side(C_R0, 90) == 0);

    // 终点(黟/L6)在某一侧
    int8_t side_end = co_split_side(C_L6, 0);
    assert(side_end == 1 || side_end == -1);

    printf("  ✓ 二元切割测试通过 (正交线分裂为二元, 平衡度=%.2f)\n", s.balance);
}

// ── 测试4: 弧弦对偶 ──
void test_arc_chord(void) {
    // 妃(0)和黟(41)之间的弧弦比
    float ratio = co_arc_chord_ratio(C_R0, C_L6);
    assert(ratio >= 1.0f);  // 弧长 >= 曲线长

    // 同色的弧弦比 = 1 (弧长=曲线长=0 → 特殊处理=1)
    float ratio_same = co_arc_chord_ratio(C_R0, C_R0);
    assert(ratio_same == 1.0f);

    // 相邻颜色的弧长 < 对角颜色的弧长
    float arc_adj = co_arc_length(C_R0, C_R1);
    float arc_far = co_arc_length(C_R0, C_L6);
    assert(arc_far > arc_adj);

    printf("  ✓ 弧弦对偶测试通过 (弧弦比>=1, 弧长随角度增大)\n");
}

// ── 测试5: 相对运动对数偏移 ──
void test_relative_motion(void) {
    // 颜色相对于0°轴
    co_relative_motion_t m = co_relative_motion(C_R1, 0);
    // 角度差在有效范围
    assert(fabsf(m.angle_diff) <= 180.0f);
    // 对数偏移 >= 0
    assert(m.log_offset >= 0.0f);
    // 扇形 0-7
    assert(m.sector < 8);

    // 中心(妃)在任何轴上偏移 = 0
    co_relative_motion_t mc = co_relative_motion(C_R0, 0);
    assert(fabsf(mc.angle_diff) < 0.01f);
    assert(mc.log_offset < 0.01f);

    // 扇形计算
    assert(co_sector_of(C_R0) < 8);

    printf("  ✓ 相对运动对数偏移测试通过 (扇形8等分, log偏移>=0)\n");
}

// ── 测试6: 自相似圆周 ──
void test_self_similar(void) {
    co_self_similar_t ss = co_self_similar_compute();
    // 6层
    assert(ss.level_count == 6);
    // 缩放因子 = 6 (第5层/第0层)
    assert(fabsf(ss.scale_factor - 6.0f) < 0.01f);
    // 基圆半径 = 1/6
    assert(fabsf(ss.base_radius - 1.0f/6.0f) < 0.01f);
    // 各层半径递增
    float r0 = co_self_similar_radius(0);
    float r5 = co_self_similar_radius(5);
    assert(r5 > r0);
    assert(fabsf(r5 / r0 - 6.0f) < 0.1f);

    printf("  ✓ 自相似圆周测试通过 (6层, 缩放因子=6, 等比缩放)\n");
}

// ── 测试7: 常驻静态约束 ──
void test_static(void) {
    // 字典不变性验证
    assert(co_dictionary_verify());
    // 元素总数不变
    assert(co_cardinality() == 42);
    // 字典查询: 每个符号都能找到映射
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const cc_color_symbol_t* e = co_dictionary_entry(i);
        assert(e != NULL);
        assert(e->color < CC_TOTAL_COLORS);
    }
    printf("  ✓ 常驻静态约束测试通过 (字典不变, 42元素, 64符号映射有效)\n");
}

// ── 测试8: 状态机扩展 ──
void test_state_machine(void) {
    // 吸收态: 妃(R0) = 收敛终点
    assert(co_is_absorbed(C_R0));
    assert(!co_is_absorbed(C_L6));

    // 分叉: 在特定轴上可分裂
    // 妃(R0)在任何轴上都在正交线上 → 可分叉
    assert(co_can_bifurcate(C_R0, 0));
    assert(co_can_bifurcate(C_R0, 90));

    // 聚合: 同层颜色可聚合
    assert(co_can_aggregate(C_R0, C_R6));  // 同在第0层
    assert(!co_can_aggregate(C_R0, C_Y0)); // 不同层

    // 自反: 第3层以上有自参照能力
    assert(co_can_self_reflect(C_G0));  // 第2层 (layer>=2)
    assert(!co_can_self_reflect(C_R0));  // 第0层

    printf("  ✓ 状态机扩展测试通过 (吸收/分叉/聚合/自反)\n");
}

int main(void) {
    printf("\n  ═══ 本体论层测试 ═══\n\n");

    test_center();
    test_boundary();
    test_split();
    test_arc_chord();
    test_relative_motion();
    test_self_similar();
    test_static();
    test_state_machine();

    printf("\n  ═══ 测试结果: 8/8 通过 ═══\n");
    printf("  正交中心 ✓ | 统摄边界 ✓ | 二元切割 ✓ | 弧弦对偶 ✓\n");
    printf("  相对运动 ✓ | 自相似 ✓ | 常驻静态 ✓ | 状态机 ✓\n\n");

    return 0;
}
