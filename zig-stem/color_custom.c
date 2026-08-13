// color_custom.c — 六系42色颜色定制实现
// 以人类传统色彩体系为进制: 暖/光/生/冷/石/韵

#include "color_custom.h"
#include <stdio.h>
#include <string.h>

// ── 42色完整定义 (名称+拼音+特质) ──
static const cc_color_info_t color_info[CC_TOTAL_COLORS] = {
    // ═══════════════════════════════════════
    // 第1行: 红系(暖) — 妃粉彤赤棕绛赭
    // ═══════════════════════════════════════
    {CC_FEI,   CS_RED,   "妃", "fēi",  "暖", 0, 0},
    {CC_FEN,   CS_RED,   "粉", "fěn",  "暖", 0, 1},
    {CC_TONG,  CS_RED,   "彤", "tóng", "暖", 0, 2},
    {CC_CHI,   CS_RED,   "赤", "chì",  "暖", 0, 3},
    {CC_ZONG,  CS_RED,   "棕", "zōng", "暖", 0, 4},
    {CC_JIANG, CS_RED,   "绛", "jiàng","暖", 0, 5},
    {CC_ZHE,   CS_RED,   "赭", "zhě",  "暖", 0, 6},

    // ═══════════════════════════════════════
    // 第2行: 黄系(光) — 缃金黄褐黧乌黑
    // ═══════════════════════════════════════
    {CC_XIANG, CS_YELLOW,"缃", "xiāng","光", 1, 0},
    {CC_JIN,   CS_YELLOW,"金", "jīn",  "光", 1, 1},
    {CC_HUANG, CS_YELLOW,"黄", "huáng","光", 1, 2},
    {CC_HE,    CS_YELLOW,"褐", "hè",   "光", 1, 3},
    {CC_LI,    CS_YELLOW,"黧", "lí",   "光", 1, 4},
    {CC_WU,    CS_YELLOW,"乌", "wū",   "光", 1, 5},
    {CC_HEI,   CS_YELLOW,"黑", "hēi",  "光", 1, 6},

    // ═══════════════════════════════════════
    // 第3行: 绿系(生) — 缥翠绿青苍黛玄
    // ═══════════════════════════════════════
    {CC_PIAO,  CS_GREEN, "缥", "piǎo", "生", 2, 0},
    {CC_CUI,   CS_GREEN, "翠", "cuì",  "生", 2, 1},
    {CC_LV,    CS_GREEN, "绿", "lǜ",   "生", 2, 2},
    {CC_QING,  CS_GREEN, "青", "qīng", "生", 2, 3},
    {CC_CANG,  CS_GREEN, "苍", "cāng", "生", 2, 4},
    {CC_DAI,   CS_GREEN, "黛", "dài",  "生", 2, 5},
    {CC_XUAN,  CS_GREEN, "玄", "xuán", "生", 2, 6},

    // ═══════════════════════════════════════
    // 第4行: 蓝系(冷) — 素银蓝紫靛绀黯
    // ═══════════════════════════════════════
    {CC_SU,    CS_BLUE,  "素", "sù",   "冷", 3, 0},
    {CC_YIN,   CS_BLUE,  "银", "yín",  "冷", 3, 1},
    {CC_LAN,   CS_BLUE,  "蓝", "lán",  "冷", 3, 2},
    {CC_ZI,    CS_BLUE,  "紫", "zǐ",   "冷", 3, 3},
    {CC_DIAN,  CS_BLUE,  "靛", "diàn", "冷", 3, 4},
    {CC_GAN,   CS_BLUE,  "绀", "gàn",  "冷", 3, 5},
    {CC_AN,    CS_BLUE,  "黯", "àn",   "冷", 3, 6},

    // ═══════════════════════════════════════
    // 第5行: 泽系(石) — 玉琅晶璃珀瑙璧
    // ═══════════════════════════════════════
    {CC_YU,    CS_STONE, "玉", "yù",   "石", 4, 0},
    {CC_LANG,  CS_STONE, "琅", "láng", "石", 4, 1},
    {CC_JING,  CS_STONE, "晶", "jīng", "石", 4, 2},
    {CC_LI_SHI,CS_STONE, "璃", "lí",   "石", 4, 3},
    {CC_PO,    CS_STONE, "珀", "pò",   "石", 4, 4},
    {CC_NAO,   CS_STONE, "瑙", "nǎo",  "石", 4, 5},
    {CC_BI,    CS_STONE, "璧", "bì",   "石", 4, 6},

    // ═══════════════════════════════════════
    // 第6行: 光系(韵) — 曦辉霓旖靡暝黟
    // ═══════════════════════════════════════
    {CC_XI,    CS_LIGHT, "曦", "xī",   "韵", 5, 0},
    {CC_HUI,   CS_LIGHT, "辉", "huī",  "韵", 5, 1},
    {CC_NI,    CS_LIGHT, "霓", "ní",   "韵", 5, 2},
    {CC_YI,    CS_LIGHT, "旖", "yǐ",   "韵", 5, 3},
    {CC_MI,    CS_LIGHT, "靡", "mí",   "韵", 5, 4},
    {CC_MING,  CS_LIGHT, "暝", "míng", "韵", 5, 5},
    {CC_YI2,   CS_LIGHT, "黟", "yī",   "韵", 5, 6},
};

// ── 64符号→42色映射表 ──
// 按层级关系分配颜色: 夫妻层→红系, 中心层→黄系, 感知层→绿系, ...
// 42色覆盖64符号: 每系7色, 部分层级共享色系
static const cc_color_symbol_t symbol_color_map[CC_SYMBOL_COUNT] = {
    // 夫妻层(0-7): 红系(暖) + 重复最深2色
    {SMV_YMBA,  CC_FEI,   "妃", CS_RED},    // 啊 → 妃 (妻=红系起点)
    {SMV_CEIQ,  CC_FEN,   "粉", CS_RED},    // 哦 → 粉 (夫=红系次色)
    {SMV_KNDO,  CC_TONG,  "彤", CS_RED},    // 工 → 彤
    {SMV_NVFM,  CC_CHI,   "赤", CS_RED},    // 空 → 赤
    {SMV_HIEY,  CC_ZONG,  "棕", CS_RED},    // 灰 → 棕
    {SMV_QIDI,  CC_JIANG, "绛", CS_RED},    // 开 → 绛
    {SMV_SHUF,  CC_ZHE,   "赭", CS_RED},    // 知 → 赭
    {SMV_ZUJM,  CC_TONG,  "彤", CS_RED},    // 出 → 彤 (循环)

    // 中心层(8-15): 黄系(光)
    {SMV_XITA,  CC_XIANG, "缃", CS_YELLOW}, // 西 → 缃
    {SMV_QITA,  CC_JIN,   "金", CS_YELLOW}, // 起 → 金
    {SMV_JIQU,  CC_HUANG, "黄", CS_YELLOW}, // 机 → 黄
    {SMV_GUAX,  CC_HE,    "褐", CS_YELLOW}, // 挂 → 褐
    {SMV_FUHR,  CC_LI,    "黧", CS_YELLOW}, // 福 → 黧
    {SMV_XUBI,  CC_WU,    "乌", CS_YELLOW}, // 序 → 乌
    {SMV_WUHR,  CC_HEI,   "黑", CS_YELLOW}, // 物 → 黑
    {SMV_TUDO,  CC_XIANG, "缃", CS_YELLOW}, // 土 → 缃

    // 感知层(16-23): 绿系(生)
    {SMV_LIDO,  CC_PIAO,  "缥", CS_GREEN},  // 力 → 缥
    {SMV_QION,  CC_CUI,   "翠", CS_GREEN},  // 亲 → 翠
    {SMV_LIAM,  CC_LV,    "绿", CS_GREEN},  // 量 → 绿
    {SMV_DUEN,  CC_QING,  "青", CS_GREEN},  // 端 → 青
    {SMV_SHOU,  CC_CANG,  "苍", CS_GREEN},  // 收 → 苍
    {SMV_MIAN,  CC_DAI,   "黛", CS_GREEN},  // 面 → 黛
    {SMV_LING,  CC_XUAN,  "玄", CS_GREEN},  // 令 → 玄
    {SMV_YIGE,  CC_PIAO,  "缥", CS_GREEN},  // 一 → 缥

    // 骨干层(24-31): 蓝系(冷)
    {SMV_YI,    CC_SU,    "素", CS_BLUE},   // 一 → 素
    {SMV_ER,    CC_YIN,   "银", CS_BLUE},   // 二 → 银
    {SMV_SAN,   CC_LAN,   "蓝", CS_BLUE},   // 三 → 蓝
    {SMV_SI,    CC_ZI,    "紫", CS_BLUE},   // 四 → 紫
    {SMV_WU,    CC_DIAN,  "靛", CS_BLUE},   // 五 → 靛
    {SMV_LIU,   CC_GAN,   "绀", CS_BLUE},   // 六 → 绀
    {SMV_QI,    CC_AN,    "黯", CS_BLUE},   // 七 → 黯
    {SMV_BA,    CC_SU,    "素", CS_BLUE},   // 八 → 素

    // 行页层(32-39): 泽系(石)
    {SMV_SHI,   CC_YU,    "玉", CS_STONE},  // 十 → 玉
    {SMV_BAI,   CC_LANG,  "琅", CS_STONE},  // 百 → 琅
    {SMV_QIAN,  CC_JING,  "晶", CS_STONE},  // 千 → 晶
    {SMV_WAN,   CC_LI_SHI,"璃", CS_STONE},  // 万 → 璃
    {SMV_LAYER1,CC_PO,    "珀", CS_STONE},  // 层一 → 珀
    {SMV_LAYER2,CC_NAO,   "瑙", CS_STONE},  // 层二 → 瑙
    {SMV_LAYER3,CC_BI,    "璧", CS_STONE},  // 层三 → 璧
    {SMV_LAYER4,CC_YU,    "玉", CS_STONE},  // 层四 → 玉

    // 交汇层+价值层+流通层(40-55): 光系(韵)
    {SMV_LAYER5,CC_XI,    "曦", CS_LIGHT},  // 层五 → 曦
    {SMV_LAYER6,CC_HUI,   "辉", CS_LIGHT},  // 层六 → 辉
    {SMV_TIME,  CC_NI,    "霓", CS_LIGHT},  // 时 → 霓
    {SMV_CYCLE, CC_YI,    "旖", CS_LIGHT},  // 季 → 旖
    {SMV_MOMENT,CC_MI,    "靡", CS_LIGHT},  // 刻 → 靡
    {SMV_FLOW,  CC_MING,  "暝", CS_LIGHT},  // 流 → 暝
    {SMV_UNION, CC_YI2,   "黟", CS_LIGHT},  // 合 → 黟
    {SMV_BIND,  CC_XI,    "曦", CS_LIGHT},  // 伴 → 曦
    {SMV_SHARE, CC_HUI,   "辉", CS_LIGHT},  // 享 → 辉
    {SMV_GOVERN,CC_NI,    "霓", CS_LIGHT},  // 治 → 霓
    {SMV_LUCK,  CC_YI,    "旖", CS_LIGHT},  // 吉 → 旖
    {SMV_DARK,  CC_MI,    "靡", CS_LIGHT},  // 凶 → 靡
    {SMV_NEUTRAL,CC_MING, "暝", CS_LIGHT},  // 平 → 暝
    {SMV_TURN,  CC_YI2,   "黟", CS_LIGHT},  // 转 → 黟
    {SMV_SOVEREIGN,CC_XI, "曦", CS_LIGHT},  // 主 → 曦
    {SMV_OWNER, CC_HUI,   "辉", CS_LIGHT},  // 有 → 辉

    // 高维层(56-63): 循环六系
    {SMV_EXTERNAL,CC_LAN, "蓝", CS_BLUE},   // 外 → 蓝
    {SMV_MERGED,CC_LV,    "绿", CS_GREEN},  // 同 → 绿
    {SMV_3D,    CC_JING,  "晶", CS_STONE},  // 体 → 晶
    {SMV_4D,    CC_XI,    "曦", CS_LIGHT},  // 周 → 曦
    {SMV_SPIRIT,CC_CHI,   "赤", CS_RED},    // 灵 → 赤
    {SMV_SOUL,  CC_HUANG,"黄", CS_YELLOW}, // 魂 → 黄
    {SMV_HEART, CC_FEI,   "妃", CS_RED},    // 心 → 妃
    {SMV_MIND,  CC_SU,    "素", CS_BLUE},   // 念 → 素
};

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

// 获取颜色信息
const cc_color_info_t* cc_get_color_info(cc_color_t c) {
    if (c >= CC_TOTAL_COLORS) return NULL;
    return &color_info[c];
}

// 获取符号绑定的颜色
const cc_color_symbol_t* cc_get_symbol_color(smv_symbol_t sym) {
    if (sym >= CC_SYMBOL_COUNT) return NULL;
    return &symbol_color_map[sym];
}

// 获取符号的颜色名称
const char* cc_get_color_name(smv_symbol_t sym) {
    if (sym >= CC_SYMBOL_COUNT) return "无色";
    return symbol_color_map[sym].color_name;
}

// 获取符号的颜色拼音
const char* cc_get_color_pinyin(smv_symbol_t sym) {
    if (sym >= CC_SYMBOL_COUNT) return "wú sè";
    cc_color_t c = symbol_color_map[sym].color;
    if (c >= CC_TOTAL_COLORS) return "?";
    return color_info[c].pinyin;
}

// 获取符号的色系
cc_color_system_t cc_get_symbol_system(smv_symbol_t sym) {
    if (sym >= CC_SYMBOL_COUNT) return CS_RED;
    return symbol_color_map[sym].system;
}

// 色系名称
const char* cc_system_name(cc_color_system_t sys) {
    switch (sys) {
        case CS_RED:    return "红系(暖)";
        case CS_YELLOW: return "黄系(光)";
        case CS_GREEN:  return "绿系(生)";
        case CS_BLUE:   return "蓝系(冷)";
        case CS_STONE:  return "泽系(石)";
        case CS_LIGHT:  return "光系(韵)";
        default:        return "?";
    }
}

// 色系品质
const char* cc_system_quality(cc_color_system_t sys) {
    switch (sys) {
        case CS_RED:    return "暖";
        case CS_YELLOW: return "光";
        case CS_GREEN:  return "生";
        case CS_BLUE:   return "冷";
        case CS_STONE:  return "石";
        case CS_LIGHT:  return "韵";
        default:        return "?";
    }
}

// 层级颜色
cc_color_t cc_layer_color(uint8_t layer, uint8_t position) {
    if (layer >= 8 || position >= 7) return CC_FEI;
    // 每层7色, 按层分配色系
    static const cc_color_system_t layer_system[8] = {
        CS_RED, CS_YELLOW, CS_GREEN, CS_BLUE,
        CS_STONE, CS_LIGHT, CS_RED, CS_BLUE
    };
    cc_color_system_t sys = layer_system[layer];
    return (cc_color_t)(sys * 7 + position);
}

// 报表
void cc_report(void) {
    printf("\n  ═══ 六系42色颜色定制 ═══\n");
    printf("  42色 = 6系 × 7色\n");
    printf("  ─────────────────────────────\n");

    for (int sys = 0; sys < CC_SYSTEM_COUNT; sys++) {
        printf("  %s[%s]: ", cc_system_name((cc_color_system_t)sys),
            cc_system_quality((cc_color_system_t)sys));
        for (int col = 0; col < CC_COLOR_PER_SYSTEM; col++) {
            int idx = sys * 7 + col;
            printf("%s(%s) ", color_info[idx].name, color_info[idx].pinyin);
        }
        printf("\n");
    }
    printf("  ─────────────────────────────\n");
    printf("  64符号→42色: 按层级分配红/黄/绿/蓝/泽/光\n");
    printf("  ═════════════════════════════\n");
}
