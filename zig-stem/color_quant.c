// color_quant.c — 全量化辅助状态变化引擎实现
// 让颜色编码成为可运算的数学对象
//
// 核心公式:
//   色相角 = 层 × 60° + 列 × (60/7)°   → 0°-360° 圆周
//   音质角 = 颜色序号 × (384/42)°      → 0°-384° 声音圆周
//   谱值   = 色相角 / 360°             → 0-1 归一化
//   镜像   = (360 - 色相角) mod 360    → 关于180°的中心对称
//   不变量 = √(_norm_hue² + norm_voice² + norm_depth²) → 位置的身份恒等

#include "color_quant.h"
#include <stdio.h>
#include <string.h>

// ═══════════════════════════════════════════════════════════════
// 位置→颜色状态 (核心构造)
// ═══════════════════════════════════════════════════════════════

cq_state_t cq_state_from_position(uint8_t layer, uint8_t col) {
    cq_state_t s;
    memset(&s, 0, sizeof(s));

    // 边界保护
    if (layer >= CQ_LAYER_COUNT) layer = 0;
    if (col >= CQ_COL_COUNT) col = 0;

    s.layer = layer;
    s.col = col;
    s.system = (cc_color_system_t)layer;
    s.color = (cc_color_t)(layer * CQ_COL_COUNT + col);

    // ── 色相角: 每系60°, 每列60/7≈8.57° ──
    s.hue_angle = (float)layer * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT)
                + (float)col * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT / CQ_COL_COUNT);

    // ── 音质角: 42色均分384°圆周 ──
    s.voice_angle = (float)(layer * CQ_COL_COUNT + col)
                  * (CQ_VOICE_CIRCLE / (float)CC_TOTAL_COLORS);

    // ── 层级深度 ──
    s.layer_depth = (float)layer;

    // ── 谱值: 色相角在频谱上的归一化投影 ──
    s.spectral = s.hue_angle / CQ_FULL_CIRCLE;

    // ── 镜像: 关于180°中心对称 ──
    s.mirror = fmodf(CQ_FULL_CIRCLE - s.hue_angle, CQ_FULL_CIRCLE);

    // ── 谱值对数化: (0,1)→(0,1) 对数缩放 ──
    s.spectral_log = logf(1.0f + s.spectral * (expf(1.0f) - 1.0f));

    // ── 不变量: 位置的身份恒等 (不随时间变化) ──
    float nh = s.hue_angle / CQ_FULL_CIRCLE;
    float nv = s.voice_angle / CQ_VOICE_CIRCLE;
    float nd = s.layer_depth / (float)(CQ_LAYER_COUNT - 1);
    s.invariant = sqrtf(nh * nh + nv * nv + nd * nd);

    // ── 幅度: 状态的强度 ──
    s.magnitude = sqrtf(s.hue_angle * s.hue_angle + s.voice_angle * s.voice_angle);

    // ── 三种距离 (内联计算, 避免递归) ──
    // 最短距离: 从原点到当前状态
    s.dist_shortest = sqrtf(nh * nh + nv * nv + nd * nd) / sqrtf(3.0f);

    // 总量距离: 到终点(第5层第6列=黟)的距离 (内联)
    float eh = (CQ_LAYER_COUNT - 1) * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT)
             + (CQ_COL_COUNT - 1) * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT / CQ_COL_COUNT);
    float ev = (float)((CQ_LAYER_COUNT - 1) * CQ_COL_COUNT + (CQ_COL_COUNT - 1))
             * (CQ_VOICE_CIRCLE / (float)CC_TOTAL_COLORS);
    float el = (float)(CQ_LAYER_COUNT - 1);
    float dh = (s.hue_angle - eh) / CQ_FULL_CIRCLE;
    float dv = (s.voice_angle - ev) / CQ_VOICE_CIRCLE;
    float dl = (s.layer_depth - el) / (float)(CQ_LAYER_COUNT - 1);
    s.dist_total = sqrtf(dh * dh + dv * dv + dl * dl) / sqrtf(3.0f);

    // 长量距离: 静态已知总长 = 384°
    s.dist_longest = CQ_VOICE_CIRCLE;

    return s;
}

cq_state_t cq_state_from_symbol(uint8_t symbol) {
    // 从64进制符号找到对应的颜色
    const cc_color_symbol_t* bind = cc_get_symbol_color(symbol);
    if (!bind) return cq_state_from_position(0, 0);
    return cq_state_from_color(bind->color);
}

cq_state_t cq_state_from_color(cc_color_t color) {
    if (color >= CC_TOTAL_COLORS) color = C_R0;
    uint8_t layer = (uint8_t)(color) / CQ_COL_COUNT;
    uint8_t col = (uint8_t)(color) % CQ_COL_COUNT;
    return cq_state_from_position(layer, col);
}

// ═══════════════════════════════════════════════════════════════
// 声音×颜色线性叠加 (色调转换)
// ═══════════════════════════════════════════════════════════════

float cq_spectral_blend(float voice_angle, float hue_angle) {
    // 归一化到同一圆周后取线性叠加
    float nv = voice_angle / CQ_VOICE_CIRCLE;   // 0-1
    float nh = hue_angle / CQ_FULL_CIRCLE;      // 0-1
    // 线性叠加: 两相位的加权平均
    float blended = (nv + nh) * 0.5f;
    return fmodf(blended * CQ_FULL_CIRCLE, CQ_FULL_CIRCLE);
}

float cq_voice_color_blend(smv_symbol_t voice_sym, cc_color_t color) {
    // 音质符号的相位
    float va = (float)(uint8_t)voice_sym * (CQ_VOICE_CIRCLE / (float)SV_SYMBOL_COUNT);
    // 颜色的色相角
    cq_state_t cs = cq_state_from_color(color);
    return cq_spectral_blend(va, cs.hue_angle);
}

float cq_hue_shift(smv_symbol_t voice_sym, cc_color_t color) {
    // 叠加后的色调变化量 = 新色相 - 原色相
    cq_state_t cs = cq_state_from_color(color);
    float blended = cq_voice_color_blend(voice_sym, color);
    float shift = blended - cs.hue_angle;
    // 归一化到 -180~+180
    if (shift > 180.0f) shift -= 360.0f;
    if (shift < -180.0f) shift += 360.0f;
    return shift;
}

// ═══════════════════════════════════════════════════════════════
// 加法包含乘法 (254缺2到2^8)
// ═══════════════════════════════════════════════════════════════

float cq_addition_governs_multiplication(uint8_t narrow_layer, uint8_t wide_layer) {
    // 窄层(加法)对宽层(乘法)的管辖力
    // 夫妻加法(0层) → 中心乘法(1层) → 感知展开(2层) → ...
    if (wide_layer <= narrow_layer) return 1.0f;  // 自身或无效

    // 加法层的合力 = narrow + 1 (夫妻=1, 中心=2, 感知=3, ...)
    float add_power = (float)(narrow_layer + 1);

    // 乘法层的展开量 = 2^(宽-窄)  (每层翻倍)
    float mul_expansion = powf(2.0f, (float)(wide_layer - narrow_layer));

    // 管辖力 = 加法合力 / 乘法展开量
    // 近距离: 高管辖 (加法直接管辖乘法)
    // 远距离: 指数衰减 (加法对远层的管辖减弱)
    return add_power / mul_expansion;
}

uint8_t cq_power_gap(float accumulated) {
    // 距离2^8 = 256 还差多少
    // 254 = 缺2 (男女分离态)
    // 255 = 缺1 (即将合一)
    // 256+ = 已到达 (夫妻共同体完成)
    if (accumulated >= CQ_POWER_THRESHOLD) return 0;
    if (accumulated >= 255.0f) return 1;
    return 2;  // 最多缺2 = 男女关系尚未激活
}

uint8_t cq_love_power_needed(float current) {
    return cq_power_gap(current);
}

// ═══════════════════════════════════════════════════════════════
// 相位空间 (3D坐标)
// ═══════════════════════════════════════════════════════════════

cq_phase_point_t cq_phase_point(cq_state_t* state) {
    cq_phase_point_t p;
    p.x = state->hue_angle;       // X = 色相 (颜色空间)
    p.y = state->voice_angle;     // Y = 音质 (声音空间)
    p.z = state->layer_depth * (CQ_VOICE_CIRCLE / CQ_LAYER_COUNT);  // Z = 层级 (时间深度)
    // 空间半径
    p.radius = sqrtf(p.x * p.x + p.y * p.y + p.z * p.z);
    return p;
}

float cq_phase_distance(cq_phase_point_t* a, cq_phase_point_t* b) {
    float dx = a->x - b->x;
    float dy = a->y - b->y;
    float dz = a->z - b->z;
    return sqrtf(dx * dx + dy * dy + dz * dz);
}

float cq_phase_projection(cq_phase_point_t* p, uint8_t axis) {
    switch (axis) {
        case 0: return p->x;   // X = 色相轴
        case 1: return p->y;   // Y = 音质轴
        case 2: return p->z;   // Z = 层级轴
        default: return 0.0f;
    }
}

// ═══════════════════════════════════════════════════════════════
// 镜像对数化
// ═══════════════════════════════════════════════════════════════

float cq_mirror_angle(float hue_angle) {
    // 关于180°的中心对称 (镜像)
    return fmodf(CQ_FULL_CIRCLE - hue_angle, CQ_FULL_CIRCLE);
}

float cq_edge_logarithm(uint8_t a, uint8_t b) {
    // 边对数: 公约数关系的对数度量
    // 计算 gcd
    uint8_t x = a + 1, y = b + 1;
    while (y) { uint8_t t = y; y = x % y; x = t; }
    uint8_t g = x;  // gcd
    if (g == 0) g = 1;
    // 归一化对数: log2(gcd) / log2(max(a+1,b+1))
    float max_val = (float)(a > b ? a : b) + 1.0f;
    if (max_val <= 1.0f) max_val = 1.0f;
    return log2f((float)g) / log2f(max_val);
}

float cq_mirror_logarithm(float angle_a, float angle_b) {
    // 镜像对数: log(1 + |mirror(a) - b|)
    float ma = cq_mirror_angle(angle_a);
    float diff = fabsf(ma - angle_b);
    return logf(1.0f + diff);
}

float cq_partial_derivative_layer(cq_state_t* state) {
    // ∂hue/∂layer: 层方向的变化率
    (void)state;  // 变化率是系统常数, 与具体状态无关
    return (CQ_FULL_CIRCLE / CQ_LAYER_COUNT) / CQ_FULL_CIRCLE;  // = 1/6
}

float cq_partial_derivative_col(cq_state_t* state) {
    // ∂hue/∂col: 列方向的变化率
    (void)state;  // 变化率是系统常数, 与具体状态无关
    return (CQ_FULL_CIRCLE / CQ_LAYER_COUNT / CQ_COL_COUNT) / CQ_FULL_CIRCLE;  // = 1/42
}

// ═══════════════════════════════════════════════════════════════
// 静态不变量 (常量 = 时间维度不变化)
// ═══════════════════════════════════════════════════════════════

float cq_invariant(cq_state_t* state) {
    return state->invariant;  // 已在构造时计算, 纯位置函数
}

bool cq_invariant_verify(cq_state_t* state) {
    // 验证: 相同位置重新构造, 不变量应相同
    cq_state_t re_test = cq_state_from_position(state->layer, state->col);
    float diff = fabsf(state->invariant - re_test.invariant);
    return diff < 0.0001f;  // 浮点精度内一致
}

// ═══════════════════════════════════════════════════════════════
// 距离度量 (三种距离)
// ═══════════════════════════════════════════════════════════════

float cq_shortest_distance(cq_state_t* state) {
    // "用一来衡量一切": 从原点到该状态的距离 (归一化)
    float nh = state->hue_angle / CQ_FULL_CIRCLE;
    float nv = state->voice_angle / CQ_VOICE_CIRCLE;
    float nd = state->layer_depth / (float)(CQ_LAYER_COUNT - 1);
    // 归一化到 0-1 (除以 √3 得到单位球内的距离)
    return sqrtf(nh * nh + nv * nv + nd * nd) / sqrtf(3.0f);
}

float cq_total_distance(cq_state_t* state) {
    // "终点": 到最终状态 (第5层第6列 = 黟) 的距离 (内联终点值, 避免递归)
    float eh = (CQ_LAYER_COUNT - 1) * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT)
             + (CQ_COL_COUNT - 1) * (CQ_FULL_CIRCLE / CQ_LAYER_COUNT / CQ_COL_COUNT);
    float ev = (float)((CQ_LAYER_COUNT - 1) * CQ_COL_COUNT + (CQ_COL_COUNT - 1))
             * (CQ_VOICE_CIRCLE / (float)CC_TOTAL_COLORS);
    float el = (float)(CQ_LAYER_COUNT - 1);
    float dh = (state->hue_angle - eh) / CQ_FULL_CIRCLE;
    float dv = (state->voice_angle - ev) / CQ_VOICE_CIRCLE;
    float dl = (state->layer_depth - el) / (float)(CQ_LAYER_COUNT - 1);
    return sqrtf(dh * dh + dv * dv + dl * dl) / sqrtf(3.0f);
}

float cq_longest_distance(void) {
    // "不变的静态已知的长量" = 音质圆周全长 = 384°
    return CQ_VOICE_CIRCLE;
}

// ═══════════════════════════════════════════════════════════════
// 累加器
// ═══════════════════════════════════════════════════════════════

void cq_accum_init(cq_accumulator_t* acc) {
    acc->spectral_sum = 0.0f;
    acc->log_sum = 0.0f;
    acc->step_count = 0;
    acc->power_reached = false;
    acc->gap_count = 2;  // 初始: 缺少2 (男女分离)
}

float cq_accum_add(cq_accumulator_t* acc, float spectral_value) {
    acc->spectral_sum += spectral_value;
    acc->log_sum += logf(1.0f + spectral_value);
    acc->step_count++;

    // 检测幂次方到达
    uint8_t gap = cq_power_gap(acc->spectral_sum);
    acc->gap_count = gap;
    if (gap == 0) acc->power_reached = true;

    return acc->spectral_sum;
}

bool cq_accum_is_complete(cq_accumulator_t* acc) {
    return acc->power_reached;
}

// ═══════════════════════════════════════════════════════════════
// 报表
// ═══════════════════════════════════════════════════════════════

void cq_state_report(cq_state_t* state) {
    const cc_color_info_t* info = cc_get_color_info(state->color);
    const char* name = info ? info->name : "?";

    printf("  ┌─── 颜色量化状态 ───┐\n");
    printf("  │ 颜色: %s (编码=%d)\n", name, state->color);
    printf("  │ 层=%d 列=%d 色系=%d\n", state->layer, state->col, state->system);
    printf("  │ 色相角=%.2f° 音质角=%.2f°\n", state->hue_angle, state->voice_angle);
    printf("  │ 谱值=%.4f 谱对数=%.4f\n", state->spectral, state->spectral_log);
    printf("  │ 镜像角=%.2f° 幅度=%.2f\n", state->mirror, state->magnitude);
    printf("  │ 不变量=%.6f  (常量/恒等)\n", state->invariant);
    printf("  │ 最短距=%.4f 总量距=%.4f 长量=%.1f°\n",
        state->dist_shortest, state->dist_total, state->dist_longest);
    printf("  └─────────────────────┘\n");
}

void cq_phase_report(cq_state_t* state) {
    cq_phase_point_t p = cq_phase_point(state);
    printf("  ┌─── 相位空间(3D) ───┐\n");
    printf("  │ X(色相)=%.2f°\n", p.x);
    printf("  │ Y(音质)=%.2f°\n", p.y);
    printf("  │ Z(层级)=%.2f°\n", p.z);
    printf("  │ 半径=%.2f\n", p.radius);
    printf("  └─────────────────────┘\n");
}

void cq_engine_report(void) {
    printf("\n");
    printf("  ╔═══════════════════════════════════════════╗\n");
    printf("  ║     全量化辅助状态变化引擎               ║\n");
    printf("  ║  ───────────────────────────────────────  ║\n");
    printf("  ║  颜色 = 可计量的状态变量                  ║\n");
    printf("  ║  相位空间: 色相° × 音质° × 层级°          ║\n");
    printf("  ║  加法管辖乘法: 窄层/宽层 → 幂次力          ║\n");
    printf("  ║  距离: 最短(一衡量) / 总量(终点) / 长量    ║\n");
    printf("  ║  不变量 = 位置恒等 = 时间常量              ║\n");
    printf("  ║  镜像对数: log(1+|mirror(a)-b|)            ║\n");
    printf("  ║  幂阈: 256=2^8 (254缺2=男女待合)          ║\n");
    printf("  ╚═══════════════════════════════════════════╝\n");
}
