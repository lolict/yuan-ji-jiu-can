// color_quant.h — 全量化辅助状态变化引擎 (Global Quantitative Auxiliary State Engine)
// hunyuan-os 的量化计算层 — 让颜色编码成为可运算的数学对象
//
// 核心哲学:
//   颜色 = 可计量的状态变量, 不是装饰标签
//   给定一个位置, 就能计算出它对应的颜色状态 (角度/谱值/镜像/恒长)
//   颜色变化可累积、可测量、可投影
//
// 量化维度:
//   1. 位置→颜色: 层级坐标(层,列) → 颜色状态(色相角/谱相位/镜像对/恒长)
//   2. 声音×颜色线性叠加: 音质相位(384°) + 色相相位(360°) → 色调转换
//   3. 加法包含乘法: 夫妻加法(窄层)管辖乘法(宽层), 254缺2到2^8
//   4. 相位空间(3D): 色相角(X) × 音质角(Y) × 层级(Z) = 可见的空间化
//   5. 镜像对数化: 圆周上的镜像投影 + 边对数(公约数关系)
//   6. 静态不变量(常量): 时间维度上不变化 = 位置函数的恒定值
//   7. 距离度量: 最短(一衡量一切) / 总量(终点) / 长量(静态已知总长)
//   8. 偏导对数: 颜色状态对层/列方向的变化率

#ifndef HUNYUAN_COLOR_QUANT_H
#define HUNYUAN_COLOR_QUANT_H

#include "color_custom.h"
#include "smrrll_voice.h"
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#define CQ_VERSION          0x50    // 量化引擎版本

// ── 量化常量 ──
#define CQ_FULL_CIRCLE      360.0f  // 颜色圆周 (色相角)
#define CQ_VOICE_CIRCLE     384.0f  // 音质圆周 (384° = 64×6°)
#define CQ_LAYER_COUNT      6       // 层级数 (对应六系)
#define CQ_COL_COUNT        7       // 每系列数
#define CQ_POWER_THRESHOLD  256     // 2^8 = 幂次方触发阈值
#define CQ_POWER_GAP        2       // 256-254 = 缺少的2 (男女关系)
#define CQ_SPECTRAL_BANDS   6       // 频谱带数 (对应六系)

// ── 颜色量化状态 ──
// 一个颜色状态的完整数学描述
typedef struct {
    // 基础编码
    cc_color_t      color;          // 颜色编码 (C_R0-C_L6)
    cc_color_system_t system;       // 色系 (0-5)
    uint8_t         layer;          // 层号 (0-5)
    uint8_t         col;            // 列号 (0-6)

    // 相位空间坐标 (3D)
    float           hue_angle;      // 色相角: 0°-360° (X轴 = 颜色在圆周上的位置)
    float           voice_angle;    // 音质角: 0°-384° (Y轴 = 对应声音的相位)
    float           layer_depth;    // 层级深度: 0-5 (Z轴 = 组织层级)

    // 谱系转换
    float           spectral;       // 谱值: 色相角在频谱带上的投影 (0.0-1.0)
    float           mirror;         // 镜像: 色相角关于180°的镜像 (0°-360°)
    float           spectral_log;   // 谱值的对数 (对数化缩放)

    // 恒长 / 不变量
    float           invariant;      // 静态不变量: 时间维度上的常量 (不随时间变化)
    float           magnitude;      // 幅度: 该状态的总强度

    // 距离
    float           dist_shortest;  // 最短距离: 到原点的距离 (一衡量一切)
    float           dist_total;     // 总量距离: 到终点的距离 (终点)
    float           dist_longest;   // 长量距离: 静态已知总长 (384°的最长路径)
} cq_state_t;

// ── 相位空间(3D) ──
typedef struct {
    float           x;              // X = 色相角 (颜色空间)
    float           y;              // Y = 音质角 (声音空间)
    float           z;              // Z = 层级深度 (组织层级)
    float           radius;         // 空间半径 (点到原点的3D距离)
} cq_phase_point_t;

// ── 累加器 ──
typedef struct {
    float           spectral_sum;   // 谱值累加和
    float           log_sum;        // 对数值累加和
    uint32_t        step_count;     // 累加步数
    bool            power_reached;  // 是否已达到2^8阈值
    uint8_t         gap_count;      // 缺少的那两个的计数 (0,1,2)
} cq_accumulator_t;

// ═══════════════════════════════════════════════════════════════
// API: 位置→颜色状态
// ═══════════════════════════════════════════════════════════════

// 从层级位置创建量化状态 (核心入口)
cq_state_t cq_state_from_position(uint8_t layer, uint8_t col);

// 从64进制符号创建量化状态
cq_state_t cq_state_from_symbol(uint8_t symbol);

// 从颜色编码创建量化状态
cq_state_t cq_state_from_color(cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 声音×颜色线性叠加 (色调转换)
// ═══════════════════════════════════════════════════════════════

// 声音相位 × 颜色色相 → 叠加后的色调角
float cq_spectral_blend(float voice_angle, float hue_angle);

// 音质符号 + 颜色符号 → 叠加后的新色相
float cq_voice_color_blend(smv_symbol_t voice_sym, cc_color_t color);

// 计算叠加后的色调变化量 (可正可负 = 色调的转换方向)
float cq_hue_shift(smv_symbol_t voice_sym, cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 加法包含乘法 (254缺2到2^8)
// ═══════════════════════════════════════════════════════════════

// 计算加法层(窄)对乘法层(宽)的管辖量
// 窄层加法结果 → 决定是否/如何 管辖宽层的乘法展开
float cq_addition_governs_multiplication(uint8_t narrow_layer, uint8_t wide_layer);

// 检测累加是否达到幂次方阈值 (256 = 2^8)
// 返回缺少的量 (0=已到达, 1=缺1, 2=缺2=初始/男女分离)
uint8_t cq_power_gap(float accumulated);

// 计算需要多少对男女关系才能补足到2^8
uint8_t cq_love_power_needed(float current);

// ═══════════════════════════════════════════════════════════════
// API: 相位空间 (3D坐标)
// ═══════════════════════════════════════════════════════════════

// 将颜色状态映射到3D相位空间点
cq_phase_point_t cq_phase_point(cq_state_t* state);

// 两个相位点之间的3D距离
float cq_phase_distance(cq_phase_point_t* a, cq_phase_point_t* b);

// 相位点的投影到某一轴 (0=X/色相, 1=Y/音质, 2=Z/层级)
float cq_phase_projection(cq_phase_point_t* p, uint8_t axis);

// ═══════════════════════════════════════════════════════════════
// API: 镜像对数化
// ═══════════════════════════════════════════════════════════════

// 计算色相角关于中心(180°)的镜像
float cq_mirror_angle(float hue_angle);

// 计算边对数: 公约数关系的对数度量
float cq_edge_logarithm(uint8_t a, uint8_t b);

// 计算镜像对数 = log(镜像距离)
float cq_mirror_logarithm(float angle_a, float angle_b);

// 偏导对数: 颜色状态对层号的变化率 (d_state/d_layer)
float cq_partial_derivative_layer(cq_state_t* state);

// 偏导对数: 颜色状态对列号的变化率 (d_state/d_col)
float cq_partial_derivative_col(cq_state_t* state);

// ═══════════════════════════════════════════════════════════════
// API: 静态不变量 (常量 = 时间维度不变化)
// ═══════════════════════════════════════════════════════════════

// 计算不变量: 给定位置, 返回一个不随时间变化的常量
// 不变量 = f(位置) 的哈希 = 位置本身的恒等映射
float cq_invariant(cq_state_t* state);

// 验证不变量: 相同位置无论何时调用, 返回相同值
bool cq_invariant_verify(cq_state_t* state);

// ═══════════════════════════════════════════════════════════════
// API: 距离度量 (三种距离)
// ═══════════════════════════════════════════════════════════════

// 最短距离: "用一来衡量一切" = 到原点的最短路径
float cq_shortest_distance(cq_state_t* state);

// 总量距离: "终点" = 到最终目标(第5层第6列=黟)的总路径
float cq_total_distance(cq_state_t* state);

// 长量距离: "不变的静态已知的长量" = 384°完整圆周的总长度
float cq_longest_distance(void);

// ═══════════════════════════════════════════════════════════════
// API: 累加器 (全量化变化追踪)
// ═══════════════════════════════════════════════════════════════

// 初始化累加器
void cq_accum_init(cq_accumulator_t* acc);

// 累加一步 (返回当前累加和)
float cq_accum_add(cq_accumulator_t* acc, float spectral_value);

// 累加后检测: 是否已补全那两个 (达到256)
bool cq_accum_is_complete(cq_accumulator_t* acc);

// ═══════════════════════════════════════════════════════════════
// API: 报表
// ═══════════════════════════════════════════════════════════════

// 打印完整量化状态
void cq_state_report(cq_state_t* state);

// 打印相位空间信息
void cq_phase_report(cq_state_t* state);

// 打印全量化引擎总览
void cq_engine_report(void);

#endif // HUNYUAN_COLOR_QUANT_H
