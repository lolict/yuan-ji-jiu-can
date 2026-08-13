// color_custom.h — 六系42色颜色定制 (Six-System 42-Color Customization)
// hunyuan-os 的颜色基础 — 以人类传统色彩体系为进制符号
//
// 核心哲学:
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
// 64符号与42色的映射:
//   每个进制符号绑定一个传统颜色
//   42色不完全覆盖64符号, 部分符号同色系分级
//   按层级关系分配: 夫妻→红系, 中心→黄系, 感知→绿系, ...

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

// ── 色彩系 (6行) ──
typedef enum {
    CS_RED      = 0,    // 红系(暖): 妃粉彤赤棕绛赭
    CS_YELLOW   = 1,    // 黄系(光): 缃金黄褐黧乌黑
    CS_GREEN    = 2,    // 绿系(生): 缥翠绿青苍黛玄
    CS_BLUE     = 3,    // 蓝系(冷): 素银蓝紫靛绀黯
    CS_STONE    = 4,    // 泽系(石): 玉琅晶璃珀瑙璧
    CS_LIGHT    = 5,    // 光系(韵): 曦辉霓旖靡暝黟
} cc_color_system_t;

// ── 传统颜色名称 (42色) ──
typedef enum {
    // 红系(暖)
    CC_FEI = 0,   CC_FEN,   CC_TONG,  CC_CHI,   CC_ZONG,  CC_JIANG, CC_ZHE,
    // 黄系(光)
    CC_XIANG,     CC_JIN,   CC_HUANG, CC_HE,    CC_LI,    CC_WU,    CC_HEI,
    // 绿系(生)
    CC_PIAO,      CC_CUI,   CC_LV,    CC_QING,  CC_CANG,  CC_DAI,   CC_XUAN,
    // 蓝系(冷)
    CC_SU,        CC_YIN,   CC_LAN,   CC_ZI,    CC_DIAN,  CC_GAN,   CC_AN,
    // 泽系(石)
    CC_YU,        CC_LANG,  CC_JING,  CC_LI_SHI,CC_PO,    CC_NAO,   CC_BI,
    // 光系(韵)
    CC_XI,        CC_HUI,   CC_NI,    CC_YI,    CC_MI,    CC_MING,  CC_YI2,
} cc_color_t;

#define CC_COLOR_LI_STHI CC_LI_SHI  // 璃 (avoid name clash)

// ── 颜色信息 ──
typedef struct {
    cc_color_t      color;          // 颜色枚举
    cc_color_system_t system;       // 所属色系
    const char*     name;           // 颜色名称 (汉字)
    const char*     pinyin;         // 拼音
    const char*     quality;        // 特质 (暖/光/生/冷/石/韵)
    uint8_t         row;            // 行号 (0-5)
    uint8_t         col;            // 列号 (0-6)
} cc_color_info_t;

// ── 颜色符号绑定 (64符号→42色) ──
typedef struct {
    smv_symbol_t    symbol;         // 音质符号
    cc_color_t      color;          // 绑定颜色
    const char*     color_name;     // 颜色名称
    cc_color_system_t system;       // 所属色系
} cc_color_symbol_t;

// ── API ──
const cc_color_info_t* cc_get_color_info(cc_color_t c);
const cc_color_symbol_t* cc_get_symbol_color(smv_symbol_t sym);
const char* cc_get_color_name(smv_symbol_t sym);
const char* cc_get_color_pinyin(smv_symbol_t sym);
cc_color_system_t cc_get_symbol_system(smv_symbol_t sym);

// 色系名称/品质
const char* cc_system_name(cc_color_system_t sys);
const char* cc_system_quality(cc_color_system_t sys);

// 按层级获取颜色
cc_color_t cc_layer_color(uint8_t layer, uint8_t position);

// 报表
void cc_report(void);

#endif // HUNYUAN_COLOR_CUSTOM_H
