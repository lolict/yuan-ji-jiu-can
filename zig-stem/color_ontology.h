// color_ontology.h — 全量化辅助状态变化引擎 · 本体论层
//
// 中心 = 公约数 = 男人 = 静态不变性 = 正交点
// 边界 = 公倍数 = 女人 = 动态包裹 = 曲线范畴
// 正交线 = 二元切割 = 状态分叉
// 弧弦对偶 = 曲线总范畴 vs 直线乘积关系
//
// 本层在 color_quant 的度量层之上, 建立权利/身份/统摄结构的数学化:
//   谁拥有统摄权 (中心), 统摄的边界在哪 (边界/女人),
//   边界内的所有余量如何被量化 (公倍数/镜像/相对运动)

#ifndef HUNYUAN_COLOR_ONTOLOGY_H
#define HUNYUAN_COLOR_ONTOLOGY_H

#include "color_quant.h"
#include <stdint.h>
#include <stdbool.h>

#define CO_VERSION          0x60    // 本体论层版本

// ═══════════════════════════════════════════════════════════════
// 核心结构体
// ═══════════════════════════════════════════════════════════════

// ── 正交中心 (唯一/静态/公约数) ──
typedef struct {
    uint8_t         center_color;   // 中心对应的颜色 (所有颜色的公约数)
    float           invariant;      // 中心不变量 (不随任何操作变化)
    float           gcd_value;      // 所有位置的最大公约数
    float           radius_max;     // 到最远颜色的半径 (统摄范围)
    uint32_t        cardinality;    // 元素总数 (常驻静态/字典不变性)
} co_center_t;

// ── 统摄边界 (包裹/动态/公倍数) ──
typedef struct {
    uint8_t         boundary_color; // 边界对应的颜色 (所有颜色的公倍数)
    float           lcm_value;      // 所有位置的最小公倍数
    float           arc_length;     // 边界上的总弧长
    float           chord_length;   // 边界的弦长 (直线范畴)
    float           arc_chord_ratio;// 弧弦比对偶值 (>1=曲线主导, =1=直线, <1=不可能)
} co_boundary_t;

// ── 二元切割结果 ──
typedef struct {
    uint8_t         axis_angle;     // 切割轴角度 (0-359)
    uint8_t         group_pos;      // 正交大侧的颜色数
    uint8_t         group_neg;      // 切割对侧的颜色数
    float           balance;        // 平衡度: 0=完全对称, 1=极度不对称
    cc_color_t      pivot;          // 切割后位于正交线上的颜色 (正交点)
} co_split_t;

// ── 相对运动对数偏移 ──
typedef struct {
    float           angle_diff;     // 相对于正交轴的角度差
    float           log_offset;     // 对数偏移: log(1+|Δθ|)
    uint8_t         sector;         // 所在扇形 (0-7, 八等分球体)
    bool            is_swinging;    // 是否在摆动中
} co_relative_motion_t;

// ── 自相似圆周 ──
typedef struct {
    float           base_radius;    // 基圆半径 (第0层)
    float           scale_factor;   // 缩放因子 (相邻层之间的半径比)
    uint8_t         level_count;    // 层数 (自相似的层级深度)
    float           total_arc;      // 所有层的弧长总和
} co_self_similar_t;

// ═══════════════════════════════════════════════════════════════
// API: 正交中心 (公约数/男人/静态不变性)
// ═══════════════════════════════════════════════════════════════

// 计算正交中心: 静态不变的唯一中心 (所有颜色的公约数)
co_center_t co_center_compute(void);

// 判断一个颜色是否位于正交中心
bool co_is_center(cc_color_t color);

// 计算颜色到正交中心的距离
float co_radius_from_center(cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 统摄边界 (公倍数/女人/动态包裹)
// ═══════════════════════════════════════════════════════════════

// 计算统摄边界: 包裹所有颜色的外圈 (公倍数)
co_boundary_t co_boundary_compute(void);

// 判断颜色是否在边界上
bool co_is_boundary(cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 二元切割 (正交线分裂)
// ═══════════════════════════════════════════════════════════════

// 用给定角度的正交线切割颜色空间
co_split_t co_orthogonal_split(uint8_t axis_angle);

// 获取颜色在切割后属于哪一侧: +1=正交大侧, -1=切割对侧, 0=位于正交线上
int8_t co_split_side(cc_color_t color, uint8_t axis_angle);

// ═══════════════════════════════════════════════════════════════
// API: 弧弦对偶 (曲线范畴 vs 直线范畴)
// ═══════════════════════════════════════════════════════════════

// 计算两个颜色之间的弧长 (沿圆周 = 曲线范畴/女人)
float co_arc_length(cc_color_t a, cc_color_t b);

// 计算两个颜色之间的弦长 (直线连接 = 直线范畴/男人)
float co_chord_length(cc_color_t a, cc_color_t b);

// 计算弧弦比 (弧长/弦长): >1=曲线主导, =1=直线
float co_arc_chord_ratio(cc_color_t a, cc_color_t b);

// ═══════════════════════════════════════════════════════════════
// API: 相对运动对数偏移
// ═══════════════════════════════════════════════════════════════

// 计算颜色相对于正交轴的运动状态
co_relative_motion_t co_relative_motion(cc_color_t color, uint8_t axis_angle);

// 计算颜色摆动的扇形区域 (0-7)
uint8_t co_sector_of(cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 自相似圆周
// ═══════════════════════════════════════════════════════════════

// 计算自相似圆周参数
co_self_similar_t co_self_similar_compute(void);

// 计算第n层的半径 (等比缩放)
float co_self_similar_radius(uint8_t level);

// ═══════════════════════════════════════════════════════════════
// API: 常驻静态约束 (字典不变性)
// ═══════════════════════════════════════════════════════════════

// 获取符号→颜色的固定映射 (预加载字典)
const cc_color_symbol_t* co_dictionary_entry(smv_symbol_t sym);

// 验证字典不变性: 映射关系未被篡改
bool co_dictionary_verify(void);

// 获取元素总数 (常量/不变)
uint32_t co_cardinality(void);

// ═══════════════════════════════════════════════════════════════
// API: 状态机扩展 (自反/分叉/聚合/吸收)
// ═══════════════════════════════════════════════════════════════

// 状态机吸收判断: 该状态是否到达中心 (无法再收敛)
bool co_is_absorbed(cc_color_t color);

// 状态分叉: 颜色在给定轴上的正交切割中位于正交线上 (可分裂为二)
bool co_can_bifurcate(cc_color_t color, uint8_t axis_angle);

// 状态聚合: 两个颜色是否可收敛到同一中心
bool co_can_aggregate(cc_color_t a, cc_color_t b);

// 状态自反: 颜色是否位于能够观测自身的位置 (3阶以上)
bool co_can_self_reflect(cc_color_t color);

// ═══════════════════════════════════════════════════════════════
// API: 报表
// ═══════════════════════════════════════════════════════════════

void co_center_report(void);
void co_boundary_report(void);
void co_split_report(uint8_t axis_angle);
void co_engine_report(void);

#endif // HUNYUAN_COLOR_ONTOLOGY_H
