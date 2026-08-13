// color_ontology.c — 全量化辅助状态变化引擎 · 本体论层实现
//
// 数学化:
//   中心 = 公约数 = 正交点 = 静态不变 = 男人 (第0层/妃)
//   边界 = 公倍数 = 曲线范畴 = 动态包裹 = 女人 (第5层/黟)
//   正交线 = 二元切割 = 状态分叉
//   弧弦比 = 曲线总范畴 / 直线范畴

#include "color_ontology.h"
#include <stdio.h>
#include <string.h>

// ═══════════════════════════════════════════════════════════════
// 内部辅助
// ═══════════════════════════════════════════════════════════════

// 角度转弧度
static float deg2rad(float deg) { return deg * 3.14159265f / 180.0f; }

// 计算 gcd
static uint32_t gcd(uint32_t a, uint32_t b) {
    while (b) { uint32_t t = b; b = a % b; a = t; }
    return a ? a : 1;
}

// 计算 lcm (限制上限避免溢出)
static uint32_t lcmLimited(uint32_t a, uint32_t b, uint32_t limit) {
    if (a == 0 || b == 0) return 0;
    uint32_t g = gcd(a, b);
    // 检查溢出: a/g * b > limit?
    if (a / g > limit / b) return limit;
    uint32_t l = (a / g) * b;
    return l > limit ? limit : l;
}

// 根据颜色序号获取其层半径 (归一化 0-1)
static float color_radius(uint8_t color_idx) {
    uint8_t layer = color_idx / CQ_COL_COUNT;
    return (float)(layer + 1) / (float)CQ_LAYER_COUNT;  // 1/6, 2/6, ..., 1
}

// 获取颜色的色相角
static float color_hue(uint8_t color_idx) {
    return (float)color_idx * (CQ_FULL_CIRCLE / (float)CC_TOTAL_COLORS);
}

// ═══════════════════════════════════════════════════════════════
// 正交中心 (公约数/男人/静态不变性)
// ═══════════════════════════════════════════════════════════════

co_center_t co_center_compute(void) {
    co_center_t c;
    memset(&c, 0, sizeof(c));

    // 中心 = 颜色序号 0 (妃/R0/第0层) = 正交点
    c.center_color = C_R0;

    // 所有颜色序号的最大公约数 = 1 (不可再分的基本单位)
    uint32_t g = 1;
    for (uint32_t i = 1; i < CC_TOTAL_COLORS; i++) {
        g = gcd(g, i);
    }
    c.gcd_value = (float)g;

    // 中心的静态不变量 (不随任何操作变化)
    cq_state_t center_state = cq_state_from_color(C_R0);
    c.invariant = center_state.invariant;

    // 最大半径: 从中心到最远颜色 (序号41 = 黟)
    cq_state_t far_state = cq_state_from_color((cc_color_t)(CC_TOTAL_COLORS - 1));
    float dx = far_state.hue_angle - center_state.hue_angle;
    float dy = far_state.voice_angle - center_state.voice_angle;
    c.radius_max = sqrtf(dx * dx + dy * dy);

    // 元素总数 (常驻静态/字典不变性)
    c.cardinality = CC_TOTAL_COLORS;

    return c;
}

bool co_is_center(cc_color_t color) {
    return color == C_R0;
}

float co_radius_from_center(cc_color_t color) {
    // 颜色到正交中心的半径距离
    cq_state_t cs = cq_state_from_color(color);
    cq_state_t center = cq_state_from_color(C_R0);
    float dx = cs.hue_angle - center.hue_angle;
    float dy = cs.voice_angle - center.voice_angle;
    return sqrtf(dx * dx + dy * dy);
}

// ═══════════════════════════════════════════════════════════════
// 统摄边界 (公倍数/女人/动态包裹)
// ═══════════════════════════════════════════════════════════════

co_boundary_t co_boundary_compute(void) {
    co_boundary_t b;
    memset(&b, 0, sizeof(b));

    // 边界 = 最外层颜色 (序号41 = 黟/L6)
    b.boundary_color = (cc_color_t)(CC_TOTAL_COLORS - 1);

    // 所有颜色序号的最小公倍数 (限制上限)
    uint32_t l = 1;
    for (uint32_t i = 2; i < CC_TOTAL_COLORS; i++) {
        l = lcmLimited(l, i, 1000000);
    }
    b.lcm_value = (float)l;

    // 边界弧长: 沿第5层半径的圆周长
    float r = color_radius(CC_TOTAL_COLORS - 1);  // = 1.0 (最外层)
    b.arc_length = 2.0f * 3.14159265f * r;        // 2πr ≈ 6.28

    // 边界弦长: 起终点之间的直线 (闭合弦 = 0, 改为直径)
    b.chord_length = 2.0f * r;                     // 直径 = 2

    // 弧弦比: 圆周/直径 = π ≈ 3.14
    b.arc_chord_ratio = b.arc_length / b.chord_length;  // = π

    return b;
}

bool co_is_boundary(cc_color_t color) {
    return color == (cc_color_t)(CC_TOTAL_COLORS - 1);
}

// ═══════════════════════════════════════════════════════════════
// 二元切割 (正交线分裂)
// ═══════════════════════════════════════════════════════════════

co_split_t co_orthogonal_split(uint8_t axis_angle) {
    co_split_t s;
    memset(&s, 0, sizeof(s));

    s.axis_angle = axis_angle;
    s.pivot = C_R0;  // 默认为中心

    float axis_rad = deg2rad((float)axis_angle);
    // 正交线方向向量
    float ax = cosf(axis_rad);
    float ay = sinf(axis_rad);

    uint8_t pos = 0, neg = 0, on_line = 0;

    for (uint8_t i = 0; i < CC_TOTAL_COLORS; i++) {
        cq_state_t cs = cq_state_from_color((cc_color_t)i);
        cq_state_t center = cq_state_from_color(C_R0);

        // 颜色相对于中心的向量
        float vx = cs.hue_angle - center.hue_angle;
        float vy = cs.voice_angle - center.voice_angle;

        // 投影到正交轴: 点积
        float proj = vx * ax + vy * ay;

        if (proj > 0.001f) pos++;
        else if (proj < -0.001f) neg++;
        else { on_line++; s.pivot = (cc_color_t)i; }
    }

    s.group_pos = pos;
    s.group_neg = neg;
    (void)on_line;  // 仅用于计数, 平衡度基于 pos/neg

    // 平衡度: |pos - neg| / (pos + neg), 0=完全对称, 1=极端不对称
    uint8_t total = pos + neg;
    int diff = (int)pos - (int)neg;
    s.balance = total > 0 ? (float)(diff >= 0 ? diff : -diff) / (float)total : 0.0f;

    return s;
}

int8_t co_split_side(cc_color_t color, uint8_t axis_angle) {
    float axis_rad = deg2rad((float)axis_angle);
    float ax = cosf(axis_rad);
    float ay = sinf(axis_rad);

    cq_state_t cs = cq_state_from_color(color);
    cq_state_t center = cq_state_from_color(C_R0);

    float vx = cs.hue_angle - center.hue_angle;
    float vy = cs.voice_angle - center.voice_angle;
    float proj = vx * ax + vy * ay;

    if (proj > 0.001f) return 1;
    if (proj < -0.001f) return -1;
    return 0;  // 位于正交线上 (正交点/中心)
}

// ═══════════════════════════════════════════════════════════════
// 弧弦对偶 (曲线范畴 vs 直线范畴)
// ═══════════════════════════════════════════════════════════════

float co_arc_length(cc_color_t a, cc_color_t b) {
    // 弧长 = 沿圆周的曲线距离 = r × Δθ
    float ra = color_radius(a);
    float rb = color_radius(b);
    float r = (ra + rb) * 0.5f;  // 平均半径

    float ha = color_hue(a);
    float hb = color_hue(b);
    float dtheta = fabsf(ha - hb);
    // 取短弧
    if (dtheta > 180.0f) dtheta = 360.0f - dtheta;

    return r * deg2rad(dtheta);  // s = rθ
}

float co_chord_length(cc_color_t a, cc_color_t b) {
    // 弦长 = 直线距离 = 2r × sin(Δθ/2)
    float ra = color_radius(a);
    float rb = color_radius(b);
    float r = (ra + rb) * 0.5f;

    float ha = color_hue(a);
    float hb = color_hue(b);
    float dtheta = fabsf(ha - hb);
    if (dtheta > 180.0f) dtheta = 360.0f - dtheta;

    return 2.0f * r * sinf(deg2rad(dtheta) * 0.5f);
}

float co_arc_chord_ratio(cc_color_t a, cc_color_t b) {
    // 弧弦比: >1 表示曲线主导 (包裹/女人)
    float chord = co_chord_length(a, b);
    if (chord < 0.0001f) return 1.0f;
    return co_arc_length(a, b) / chord;
}

// ═══════════════════════════════════════════════════════════════
// 相对运动对数偏移 (扇形/摆动/直线偏差)
// ═══════════════════════════════════════════════════════════════

co_relative_motion_t co_relative_motion(cc_color_t color, uint8_t axis_angle) {
    co_relative_motion_t m;
    memset(&m, 0, sizeof(m));

    float hue = color_hue(color);
    m.angle_diff = hue - (float)axis_angle;
    // 归一化到 -180~180
    while (m.angle_diff > 180.0f) m.angle_diff -= 360.0f;
    while (m.angle_diff < -180.0f) m.angle_diff += 360.0f;

    // 对数偏移: log(1 + |Δθ|)
    m.log_offset = logf(1.0f + fabsf(m.angle_diff));

    // 扇形区域: 八等分 (0-7)
    float norm = m.angle_diff + 180.0f;  // 0-360
    m.sector = ((uint8_t)(norm / 45.0f)) % 8;

    // 摆动检测: 角度差 > 0 = 正在摆动
    m.is_swinging = fabsf(m.angle_diff) > 1.0f;

    return m;
}

uint8_t co_sector_of(cc_color_t color) {
    float hue = color_hue(color);
    return ((uint8_t)((hue + 360.0f) / 45.0f)) % 8;
}

// ═══════════════════════════════════════════════════════════════
// 自相似圆周 (等比缩放)
// ═══════════════════════════════════════════════════════════════

co_self_similar_t co_self_similar_compute(void) {
    co_self_similar_t ss;
    memset(&ss, 0, sizeof(ss));

    ss.level_count = CQ_LAYER_COUNT;  // 6层

    // 基圆半径 = 第0层
    ss.base_radius = color_radius(0);  // = 1/6

    // 缩放因子: 第5层/第0层 = 6 (等比: 每层半径 ×1)
    float r5 = color_radius(CC_TOTAL_COLORS - 1);  // = 1
    ss.scale_factor = r5 / ss.base_radius;  // = 6

    // 所有层弧长总和
    ss.total_arc = 0.0f;
    for (uint8_t i = 0; i < CQ_LAYER_COUNT; i++) {
        float r = color_radius(i * CQ_COL_COUNT);  // 每层第一个颜色
        ss.total_arc += 2.0f * 3.14159265f * r;
    }

    return ss;
}

float co_self_similar_radius(uint8_t level) {
    if (level >= CQ_LAYER_COUNT) level = CQ_LAYER_COUNT - 1;
    return color_radius(level * CQ_COL_COUNT);
}

// ═══════════════════════════════════════════════════════════════
// 常驻静态约束 (字典不变性)
// ═══════════════════════════════════════════════════════════════

const cc_color_symbol_t* co_dictionary_entry(smv_symbol_t sym) {
    return cc_get_symbol_color(sym);
}

bool co_dictionary_verify(void) {
    // 验证: 所有64个符号的映射都有效且一致
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const cc_color_symbol_t* entry = cc_get_symbol_color(i);
        if (!entry) return false;
        if (entry->color >= CC_TOTAL_COLORS) return false;
        if (entry->symbol != (smv_symbol_t)i) return false;
    }
    return true;
}

uint32_t co_cardinality(void) {
    return CC_TOTAL_COLORS;
}

// ═══════════════════════════════════════════════════════════════
// 状态机扩展 (自反/分叉/聚合/吸收)
// ═══════════════════════════════════════════════════════════════

bool co_is_absorbed(cc_color_t color) {
    // 吸收态: 位于正交中心 (收敛终点, 无法再收敛)
    return co_is_center(color);
}

bool co_can_bifurcate(cc_color_t color, uint8_t axis_angle) {
    // 分叉条件: 颜色恰好位于正交线上 (投影 ≈ 0)
    return co_split_side(color, axis_angle) == 0;
}

bool co_can_aggregate(cc_color_t a, cc_color_t b) {
    // 聚合条件: 两颜色在同一层 (同半径) → 可收敛为一点
    uint8_t la = (uint8_t)a / CQ_COL_COUNT;
    uint8_t lb = (uint8_t)b / CQ_COL_COUNT;
    return la == lb;
}

bool co_can_self_reflect(cc_color_t color) {
    // 自反条件: 颜色的色相角和音质角都已完全发展 (层号 >= 2 = 第3层起)
    // 低层只有径向坐标, 发展到第3层才同时有角度和径向的自参照能力
    uint8_t layer = (uint8_t)color / CQ_COL_COUNT;
    return layer >= 2;
}

// ═══════════════════════════════════════════════════════════════
// 报表
// ═══════════════════════════════════════════════════════════════

void co_center_report(void) {
    co_center_t c = co_center_compute();
    const cc_color_info_t* info = cc_get_color_info(c.center_color);
    printf("  ┌─── 正交中心 (公约数/男人/静态) ───┐\n");
    printf("  │ 中心颜色: %s (序号=%d)\n", info ? info->name : "?", c.center_color);
    printf("  │ 公约数(GCD): %.0f (不可再分的基本单位)\n", c.gcd_value);
    printf("  │ 静态不变量: %.6f\n", c.invariant);
    printf("  │ 统摄半径: %.2f° (到最远颜色)\n", c.radius_max);
    printf("  │ 元素总数: %u (常驻静态/字典不变)\n", c.cardinality);
    printf("  └─────────────────────────────────────┘\n");
}

void co_boundary_report(void) {
    co_boundary_t b = co_boundary_compute();
    const cc_color_info_t* info = cc_get_color_info(b.boundary_color);
    printf("  ┌─── 统摄边界 (公倍数/女人/动态包裹) ───┐\n");
    printf("  │ 边界颜色: %s (序号=%d)\n", info ? info->name : "?", b.boundary_color);
    printf("  │ 公倍数(LCM): %.0f\n", b.lcm_value);
    printf("  │ 弧长(曲线范畴): %.4f = 2πr\n", b.arc_length);
    printf("  │ 弦长(直线范畴): %.4f = 直径\n", b.chord_length);
    printf("  │ 弧弦比: %.4f (π ≈ 曲线/直线)\n", b.arc_chord_ratio);
    printf("  └─────────────────────────────────────────┘\n");
}

void co_split_report(uint8_t axis_angle) {
    co_split_t s = co_orthogonal_split(axis_angle);
    printf("  ┌─── 二元切割 (正交线/状态分叉 @ %d°) ───┐\n", axis_angle);
    printf("  │ 轴一侧: %u 色 | 轴另一侧: %u 色\n", s.group_pos, s.group_neg);
    printf("  │ 平衡度: %.4f (0=对称 / 1=极端不对称)\n", s.balance);
    printf("  │ 正交点上颜色: 序号=%d\n", s.pivot);
    printf("  └────────────────────────────────────────────┘\n");
}

void co_engine_report(void) {
    printf("\n");
    printf("  ╔═══════════════════════════════════════════════════╗\n");
    printf("  ║     全量化辅助状态变化引擎 · 本体论层             ║\n");
    printf("  ║  ───────────────────────────────────────────────  ║\n");
    printf("  ║  中心 = 公约数 = 正交点 = 男人 = 静态不变        ║\n");
    printf("  ║  边界 = 公倍数 = 曲线范畴 = 女人 = 动态包裹      ║\n");
    printf("  ║  正交线 = 二元切割 = 状态分叉 = 镜像对偶        ║\n");
    printf("  ║  弧弦比 = 曲线总范畴 / 直线范畴                   ║\n");
    printf("  ║  常驻静态 = 字典不变性 = 符号唯一性               ║\n");
    printf("  ║  自相似圆周 = 等比缩放 = 层级嵌套                 ║\n");
    printf("  ║  状态机: 自反/分叉/聚合/吸收                      ║\n");
    printf("  ╚═══════════════════════════════════════════════════╝\n");
}
