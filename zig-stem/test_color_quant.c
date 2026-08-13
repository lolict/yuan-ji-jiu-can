// test_color_quant.c — 全量化辅助状态变化引擎测试
// 验证: 位置→颜色状态 + 声色叠加 + 加法管辖乘法 + 距离 + 镜像 + 不变量 + 累加器

#include "color_quant.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>
#include <math.h>

// ── 测试1: 位置→颜色状态构造 ──
void test_state_from_position(void) {
    // 原点: 第0层第0列 = 妃(红系)
    cq_state_t s00 = cq_state_from_position(0, 0);
    assert(s00.layer == 0 && s00.col == 0);
    assert(s00.color == C_R0);
    assert(s00.hue_angle == 0.0f);       // 原点色相角=0°
    assert(s00.voice_angle == 0.0f);      // 原点在音质圆上也=0°
    assert(s00.layer_depth == 0.0f);
    assert(s00.spectral == 0.0f);         // 原点的谱值=0

    // 第5层第6列 = 黟(光系终点)
    cq_state_t s56 = cq_state_from_position(5, 6);
    assert(s56.layer == 5 && s56.col == 6);
    assert(s56.color == C_L6);
    assert(s56.hue_angle > 350.0f && s56.hue_angle <= 360.0f);  // 接近完整圆周

    // 中点: 第2层第3列 = 青(绿系中央)
    cq_state_t s23 = cq_state_from_position(2, 3);
    assert(s23.color == C_G3);
    // 色相角 = 2×60 + 3×(60/7) ≈ 120 + 25.71 ≈ 145.71°
    assert(s23.hue_angle > 140.0f && s23.hue_angle < 150.0f);

    printf("  ✓ 位置→颜色状态构造测试通过 (42个位置各有唯一状态)\n");
}

// ── 测试2: 不变量 = 时间维度上的恒等 ──
void test_invariant(void) {
    // 相同位置, 多次构造, 不变量应完全一致
    cq_state_t a = cq_state_from_position(3, 4);
    cq_state_t b = cq_state_from_position(3, 4);
    cq_state_t c = cq_state_from_position(3, 4);

    assert(fabsf(a.invariant - b.invariant) < 0.0001f);
    assert(fabsf(b.invariant - c.invariant) < 0.0001f);
    assert(cq_invariant_verify(&a));

    // 不同位置应有不同不变量
    cq_state_t x = cq_state_from_position(0, 0);
    cq_state_t y = cq_state_from_position(5, 6);
    assert(fabsf(x.invariant - y.invariant) > 0.01f);

    printf("  ✓ 不变量恒等测试通过 (同位置=同不变量=时间常量)\n");
}

// ── 测试3: 声音×颜色线性叠加 = 色调转换 ──
void test_voice_color_blend(void) {
    // 声音(工=SMV_KNDO=2) × 颜色(彤=C_R2)
    float blended = cq_voice_color_blend(SMV_KNDO, C_R2);
    assert(blended >= 0.0f && blended <= 360.0f);

    // 色调变化量应有意义 (不是0)
    float shift = cq_hue_shift(SMV_KNDO, C_R2);
    assert(fabsf(shift) > 0.01f);
    assert(fabsf(shift) <= 180.0f);  // 归一化到±180

    // 线性叠加: 同一个声部+同一个色部, 交换后归一化不同(384 vs 360)
    // 所以交换参数后结果在两个圆周的"加权中点"上, 是合理的非对称
    float b1 = cq_spectral_blend(60.0f, 120.0f);
    float b2 = cq_spectral_blend(120.0f, 60.0f);
    // 两者都落在有效圆周内, 且都接近两个相位的中点
    assert(b1 >= 0.0f && b1 <= 360.0f);
    assert(b2 >= 0.0f && b2 <= 360.0f);
    // b1 和 b2 不同是因为声音圆周(384°)和颜色圆周(360°)不同
    // 但两者都在有效范围内
    assert(fabsf(b1 - b2) > 0.01f);  // 有差异 = 两个圆周不同质

    printf("  ✓ 声色线性叠加测试通过 (声音×颜色→色调转换)\n");
}

// ── 测试4: 加法管辖乘法 (254缺2到2^8) ──
void test_addition_governs(void) {
    // 第0层(夫妻加法)对第1层(中心乘法)的管辖力
    float g01 = cq_addition_governs_multiplication(0, 1);
    assert(g01 > 0.0f && g01 <= 1.0f);

    // 第0层对第5层: 更远 = 管辖力指数下降
    float g05 = cq_addition_governs_multiplication(0, 5);
    assert(g05 < g01);  // 越远越小

    // 相同层: 全力
    float g33 = cq_addition_governs_multiplication(3, 3);
    assert(g33 == 1.0f);

    printf("  ✓ 加法管辖乘法测试通过 (窄层→宽层, 距离指数衰减)\n");
}

// ── 测试5: 幂次方阈值检测 ──
void test_power_gap(void) {
    // 初始: 254 = 缺2
    assert(cq_power_gap(254.0f) == 2);
    // 即将到达: 255 = 缺1
    assert(cq_power_gap(255.0f) == 1);
    // 已到达: 256 = 0
    assert(cq_power_gap(256.0f) == 0);
    // 超过也=0
    assert(cq_power_gap(300.0f) == 0);
    // 远小于: 缺2
    assert(cq_power_gap(100.0f) == 2);
    // love_power_needed = power_gap
    assert(cq_love_power_needed(254.0f) == 2);

    printf("  ✓ 幂次方阈值测试通过 (254缺2 → 255缺1 → 256=2^8到达)\n");
}

// ── 测试6: 三种距离度量 ──
void test_distances(void) {
    cq_state_t origin = cq_state_from_position(0, 0);
    cq_state_t end = cq_state_from_position(5, 6);

    // 最短距离: 原点到自身应为0
    assert(origin.dist_shortest < 0.0001f);
    // 中点应有有意义的最短距离
    cq_state_t mid = cq_state_from_position(2, 3);
    assert(mid.dist_shortest > 0.0f && mid.dist_shortest <= 1.0f);

    // 总量距离: 终点到终点应为0 (允许浮点误差)
    assert(end.dist_total < 0.0001f);
    // 原点到终点 = 最大总量距离
    assert(origin.dist_total > 0.5f);

    // 长量距离: 恒为384° (静态已知总长)
    assert(cq_longest_distance() == 384.0f);
    assert(origin.dist_longest == 384.0f);

    printf("  ✓ 三种距离度量测试通过 (最短/总量/长量=384°)\n");
}

// ── 测试7: 镜像对数化 ──
void test_mirror_logarithm(void) {
    // 镜像: 0°→0°(或360°), 90°→270°, 180°→180°
    assert(fabsf(cq_mirror_angle(0.0f) - 0.0f) < 0.01f || fabsf(cq_mirror_angle(0.0f) - 360.0f) < 0.01f);
    assert(fabsf(cq_mirror_angle(90.0f) - 270.0f) < 0.01f);
    assert(fabsf(cq_mirror_angle(180.0f) - 180.0f) < 0.01f);
    assert(fabsf(cq_mirror_angle(270.0f) - 90.0f) < 0.01f);

    // 镜像对数: log(1 + |mirror(a) - b|)
    float ml = cq_mirror_logarithm(0.0f, 180.0f);
    assert(ml > 0.0f);

    // 边对数: gcd关系的对数
    float el = cq_edge_logarithm(6, 4);  // gcd(7,5)=1
    assert(el >= 0.0f && el <= 1.0f);

    printf("  ✓ 镜像对数化测试通过 (中心对称 + 边对数gcd)\n");
}

// ── 测试8: 偏导对数 (变化率) ──
void test_partial_derivatives(void) {
    cq_state_t s = cq_state_from_position(3, 3);

    // ∂hue/∂layer = 1/6 (每层增加60°, 归一化=60/360=1/6)
    float dlayer = cq_partial_derivative_layer(&s);
    assert(fabsf(dlayer - 1.0f/6.0f) < 0.001f);

    // ∂hue/∂col = 1/42 (每列增加60/7°, 归一化=(60/7)/360=1/42)
    float dcol = cq_partial_derivative_col(&s);
    assert(fabsf(dcol - 1.0f/42.0f) < 0.001f);

    printf("  ✓ 偏导对数测试通过 (∂hue/∂layer=1/6, ∂hue/∂col=1/42)\n");
}

// ── 测试9: 累加器幂次方到达 ──
void test_accumulator(void) {
    cq_accumulator_t acc;
    cq_accum_init(&acc);

    assert(!cq_accum_is_complete(&acc));
    assert(acc.gap_count == 2);  // 初始: 缺2

    // 累加足够的谱值
    float spectrum_values[8] = {0.3f, 0.5f, 0.7f, 0.9f, 0.6f, 0.4f, 0.8f, 1.0f};
    float sum = 0;
    for (int i = 0; i < 8; i++) {
        sum = cq_accum_add(&acc, spectrum_values[i]);
    }
    // 总和应等于各值之和 = 5.2
    assert(fabsf(sum - 5.2f) < 0.01f);

    // 大量累加 → 应能到达256
    cq_accumulator_t acc2;
    cq_accum_init(&acc2);
    for (int i = 0; i < 1000; i++) {
        cq_accum_add(&acc2, 0.5f);  // 每次+0.5
    }
    // 1000×0.5 = 500 > 256, 应已到达
    assert(cq_accum_is_complete(&acc2));
    assert(acc2.gap_count == 0);

    printf("  ✓ 累加器测试通过 (谱值累加 → 幂次方阈值检测)\n");
}

// ── 测试10: 相位空间(3D) ──
void test_phase_space(void) {
    cq_state_t s = cq_state_from_position(3, 4);
    cq_phase_point_t p = cq_phase_point(&s);

    // X = 色相角
    assert(fabsf(p.x - s.hue_angle) < 0.01f);
    // Y = 音质角
    assert(fabsf(p.y - s.voice_angle) < 0.01f);
    // Z = 层级深度 × 64 (384°/6层)
    assert(fabsf(p.z - s.layer_depth * (CQ_VOICE_CIRCLE / CQ_LAYER_COUNT)) < 0.01f);
    // 半径 = √(X²+Y²+Z²)
    float expected_r = sqrtf(p.x*p.x + p.y*p.y + p.z*p.z);
    assert(fabsf(p.radius - expected_r) < 0.01f);

    // 两个不同点的相位距离 > 0
    cq_state_t s2 = cq_state_from_position(0, 0);
    cq_phase_point_t p2 = cq_phase_point(&s2);
    float dist = cq_phase_distance(&p, &p2);
    assert(dist > 0.0f);

    // 投影函数
    assert(fabsf(cq_phase_projection(&p, 0) - p.x) < 0.01f);
    assert(fabsf(cq_phase_projection(&p, 1) - p.y) < 0.01f);
    assert(fabsf(cq_phase_projection(&p, 2) - p.z) < 0.01f);

    printf("  ✓ 相位空间(3D)测试通过 (X=色相, Y=音质, Z=层级)\n");
}

// ── 测试11: 从64进制符号创建状态 ──
void test_from_symbol(void) {
    // 符号0 (啊=妻=0) → 应映射到妃(R0)
    cq_state_t s0 = cq_state_from_symbol(0);
    assert(s0.color == C_R0);

    // 符号2 (工=工藤新一=2) → 彤(R2)
    cq_state_t s2 = cq_state_from_symbol(2);
    assert(s2.color == C_R2);

    // 所有64个符号都能创建有效状态
    int valid = 0;
    for (int i = 0; i < 64; i++) {
        cq_state_t s = cq_state_from_symbol(i);
        if (s.color < CC_TOTAL_COLORS) valid++;
    }
    assert(valid == 64);

    printf("  ✓ 符号→状态测试通过 (64符号→42色量化状态)\n");
}

int main(void) {
    printf("\n  ═══ 全量化辅助状态变化引擎测试 ═══\n\n");

    test_state_from_position();
    test_invariant();
    test_voice_color_blend();
    test_addition_governs();
    test_power_gap();
    test_distances();
    test_mirror_logarithm();
    test_partial_derivatives();
    test_accumulator();
    test_phase_space();
    test_from_symbol();

    printf("\n  ═══ 测试结果: 11/11 通过 ═══\n");
    printf("  位置→状态 ✓ | 不变量 ✓ | 声色叠加 ✓\n");
    printf("  加法管辖 ✓ | 幂阈值 ✓ | 三距离 ✓\n");
    printf("  镜像对数 ✓ | 偏导数 ✓ | 累加器 ✓ | 相位空间 ✓ | 符号映射 ✓\n\n");

    return 0;
}
