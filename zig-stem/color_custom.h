// color_custom.h — 颜色定制进制 (Color Customization Numeral System)
// hunyuan-os 的颜色基础 — 以颜色为进制符号的可视化维度
//
// 哲学:
//   颜色 = 进制符号的视觉身份
//   每个进制符号不仅有声音(声韵调), 还有颜色
//   颜色不是装饰, 是进制的一部分 — 用颜色区分层级/身份/量化
//   颜色定制: 以人类视觉感知为基准的进制化
//
// 颜色进制原理:
//   色相(Hue): 映射到384°圆周 (与64进制对齐)
//   饱和度(Saturation): 纯度/浓度 → 层级浓度
//   明度(Lightness): 亮度 → 时间/因果的明暗
//
// 颜色与音质绑定:
//   声母 → 色相分区 (唇音=红/舌尖=黄/舌根=蓝/舌面=绿)
//   韵母 → 饱和度 (开口韵=高饱和/合口韵=低饱和)
//   声调 → 明度 (阴平=亮/去声=暗)
//
// 正交关系:
//   互补色(180°对立) = 对径符号 = 镜像关系
//   三原色(120°间隔) = 见证关系
//   类似色(30°内) = 同族关系

#ifndef HUNYUAN_COLOR_CUSTOM_H
#define HUNYUAN_COLOR_CUSTOM_H

#include "smrrll_voice.h"
#include <stdint.h>
#include <stdbool.h>

#define CC_VERSION          0x40
#define CC_FULL_CIRCLE      384     // 色相圆周 = 音质圆周
#define CC_SYMBOL_COUNT     64      // 64个颜色符号
#define CC_DEGREE_UNIT      6       // 每颜色占6°

// ── HSL颜色 (与64进制对齐) ──
typedef struct {
    float       hue;            // 色相 0-384° (与音质圆周对齐)
    float       saturation;     // 饱和度 0-1
    float       lightness;      // 明度 0-1
} cc_hsl_t;

// ── RGB颜色 (显示用) ──
typedef struct {
    uint8_t     r, g, b;        // 红绿蓝 0-255
} cc_rgb_t;

// ── 颜色符号 (绑定音质+颜色) ──
typedef struct {
    smv_symbol_t    symbol;         // 音质符号
    cc_hsl_t        hsl;            // HSL颜色
    cc_rgb_t        rgb;            // RGB颜色
    const char*     color_name;     // 颜色名称
    uint8_t         layer;          // 所属层级
} cc_color_symbol_t;

// ── 颜色层级 (按声母分组) ──
typedef enum {
    CL_WARM     = 0,    // 暖色层 (唇音 b/p/m/f → 红橙)
    CL_EARTH    = 1,    // 土色层 (舌尖中 d/t/n/l → 黄绿)
    CL_COOL     = 2,    // 冷色层 (舌根/喉 g/k/h → 青蓝)
    CL_LEAF    = 3,    // 叶色层 (舌面 j/q/x → 绿翠)
    CL_GOLD    = 4,    // 金色层 (舌尖后 zh/ch/sh/r → 金橙)
    CL_SAND    = 5,    // 沙色层 (舌尖前 z/c/s → 米黄)
    CL_AIR     = 6,    // 气色层 (半元音 y/w + 零声母 → 浅紫)
} cc_color_layer_t;

// ── 颜色关系 ──
typedef enum {
    CR_SAME     = 0,    // 同色 (0°)
    CR_ANALOGY  = 1,    // 类似色 (30°内)
    CR_WITNESS  = 2,    // 见证色 (120°三原色)
    CR_MIRROR   = 3,    // 镜像色 (180°互补)
    CR_NEUTRAL  = 4,    // 中性 (无特定关系)
} cc_color_relation_t;

// ── API ──

// 获取符号的颜色
const cc_color_symbol_t* cc_get_color(smv_symbol_t sym);

// HSL → RGB转换
cc_rgb_t cc_hsl_to_rgb(cc_hsl_t hsl);

// RGB → HSL转换
cc_hsl_t cc_rgb_to_hsl(cc_rgb_t rgb);

// 计算两个颜色的关系
cc_color_relation_t cc_color_relation(smv_symbol_t a, smv_symbol_t b);

// 计算颜色距离 (在384°色相圆周上)
float cc_color_distance(smv_symbol_t a, smv_symbol_t b);

// 获取颜色名称
const char* cc_get_color_name(smv_symbol_t sym);

// 获取层级颜色
cc_rgb_t cc_get_layer_color(cc_color_layer_t layer);

// ANSI终端颜色输出
void cc_print_colored(smv_symbol_t sym, const char* text);

// 颜色报表
void cc_report(void);

#endif // HUNYUAN_COLOR_CUSTOM_H
