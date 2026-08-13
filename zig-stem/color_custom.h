// color_custom.h — 六系42色颜色定制 (Six-System 42-Color Customization)
// hunyuan-os 的颜色基础 — 以人类传统色彩体系为进制符号
//
// 哲学:
//   颜色 = 进制符号的视觉身份
//   进制以人类传统色彩为单位, 不是脱离语言的机械RGB角度
//   六系42色覆盖暖/光/生/冷/石/韵六个维度
//   每个颜色名称来自传统色彩体系, 可感知/可文化/可传承
//
// 六系42色 (6 systems × 7 colors = 42 colors):
//   第1行 红系(暖): 妃 粉 彤 赤 棕 绛 赭  → 夫妻层/情感层
//   第2行 黄系(光): 缃 金 黄 褐 黧 乌 黑  → 中心层/光照层
//   第3行 绿系(生): 缥 翠 绿 青 苍 黛 玄  → 感知层/生长层
//   第4行 蓝系(冷): 素 银 蓝 紫 靛 绀 黯  → 骨干层/结构层
//   第5行 泽系(石): 玉 琅 晶 璃 珀 瑙 璧  → 物质层/载体层
//   第6行 光系(韵): 曦 辉 霓 旖 靡 暝 黟  → 高维层/意识层
//
// 编码原则:
//   编码层使用纯数字 (C0-C41), 不含任何中文发音
//   显示层使用汉字 (妃/粉/彤...)
//   线索引使用字母+数字 (R0-R6 红系, Y0-Y6 黄系...)
//   这样编码只是序号, 不会被人误解为有意义的中文词

#ifndef HUNYUAN_COLOR_CUSTOM_H
#define HUNYUAN_COLOR_CUSTOM_H

#include "smrrll_voice.h"
#include <stdint.h>
#include <stdbool.h>

#define CC_VERSION          0x40
#define CC_SYSTEM_COUNT     6       // 六个色彩系
#define CC_COLOR_PER_SYSTEM 7       // 每系7色
#define CC_TOTAL_COLORS     42      // 总共42色
#define CC_SYMBOL_COUNT     64      // 64进制符号

// ── 色彩系 (6行) — 纯序号编码, 无中文语义 ──
typedef enum {
    CS_RED      = 0,    // R: 红系(暖)
    CS_YELLOW   = 1,    // Y: 黄系(光)
    CS_GREEN    = 2,    // G: 绿系(生)
    CS_BLUE     = 3,    // B: 蓝系(冷)
    CS_STONE    = 4,    // S: 泽系(石)
    CS_LIGHT    = 5,    // L: 光系(韵)
} cc_color_system_t;

// ── 42色编码 (纯序号 C0-C41, 无中文语义) ──
typedef enum {
    // 红系 R0-R6 (暖)
    C_R0 = 0,  C_R1,  C_R2,  C_R3,  C_R4,  C_R5,  C_R6,
    // 黄系 Y0-Y6 (光)
    C_Y0, C_Y1, C_Y2, C_Y3, C_Y4, C_Y5, C_Y6,
    // 绿系 G0-G6 (生)
    C_G0, C_G1, C_G2, C_G3, C_G4, C_G5, C_G6,
    // 蓝系 B0-B6 (冷)
    C_B0, C_B1, C_B2, C_B3, C_B4, C_B5, C_B6,
    // 泽系 S0-S6 (石)
    C_S0, C_S1, C_S2, C_S3, C_S4, C_S5, C_S6,
    // 光系 L0-L6 (韵)
    C_L0, C_L1, C_L2, C_L3, C_L4, C_L5, C_L6,
} cc_color_t;

// ── 颜色信息 (显示层: 汉字只作为字符串标签) ──
typedef struct {
    cc_color_t      code;           // 颜色编码 (纯数字, 0-41)
    cc_color_system_t system;       // 所属色系 (0-5)
    const char*     name;           // 显示名称 (汉字: 妃/粉/彤...)
    const char*     pinyin;         // 显示拼音 (fēi/fěn/tóng...)
    const char*     quality;        // 特质 (暖/光/生/冷/石/韵)
    uint8_t         row;            // 行号 (0-5)
    uint8_t         col;            // 列号 (0-6)
} cc_color_info_t;

// ── 颜色符号绑定 (64进制符号→42色) ──
typedef struct {
    uint8_t         symbol;         // 进制符号序号 (0-63)
    cc_color_t      color;          // 绑定颜色编码 (C0-C41)
    cc_color_system_t system;       // 所属色系
} cc_color_symbol_t;

// ── API ──
const cc_color_info_t* cc_get_color_info(cc_color_t c);
const cc_color_symbol_t* cc_get_symbol_color(uint8_t symbol);
const char* cc_get_color_name(uint8_t symbol);
cc_color_system_t cc_get_symbol_system(uint8_t symbol);

// 色系名称/品质
const char* cc_system_name(cc_color_system_t sys);
const char* cc_system_quality(cc_color_system_t sys);

// 按层级获取颜色
cc_color_t cc_layer_color(uint8_t layer, uint8_t position);

// 报表
void cc_report(void);

#endif // HUNYUAN_COLOR_CUSTOM_H
