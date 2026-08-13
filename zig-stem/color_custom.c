// color_custom.c — 六系42色颜色定制实现
// 传统色彩体系: 暖/光/生/冷/石/韵
// 编码层使用纯数字序号 (C_R0/C_Y0/...), 汉字仅作显示标签

#include "color_custom.h"
#include <stdio.h>
#include <string.h>

// ── 42色定义 (编码=纯数字, 汉字=显示标签) ──
static const cc_color_info_t color_info[CC_TOTAL_COLORS] = {
    // ═══════════════════════════════════════
    // 红系 R0-R6 (暖): 妃粉彤赤棕绛赭
    // ═══════════════════════════════════════
    {C_R0, CS_RED,   "妃", "fēi",  "暖", 0, 0},
    {C_R1, CS_RED,   "粉", "fěn",  "暖", 0, 1},
    {C_R2, CS_RED,   "彤", "tóng", "暖", 0, 2},
    {C_R3, CS_RED,   "赤", "chì",  "暖", 0, 3},
    {C_R4, CS_RED,   "棕", "zōng", "暖", 0, 4},
    {C_R5, CS_RED,   "绛", "jiàng","暖", 0, 5},
    {C_R6, CS_RED,   "赭", "zhě",  "暖", 0, 6},

    // ═══════════════════════════════════════
    // 黄系 Y0-Y6 (光): 缃金黄褐黧乌黑
    // ═══════════════════════════════════════
    {C_Y0, CS_YELLOW,"缃", "xiāng","光", 1, 0},
    {C_Y1, CS_YELLOW,"金", "jīn",  "光", 1, 1},
    {C_Y2, CS_YELLOW,"黄", "huáng","光", 1, 2},
    {C_Y3, CS_YELLOW,"褐", "hè",   "光", 1, 3},
    {C_Y4, CS_YELLOW,"黧", "lí",   "光", 1, 4},
    {C_Y5, CS_YELLOW,"乌", "wū",   "光", 1, 5},
    {C_Y6, CS_YELLOW,"黑", "hēi",  "光", 1, 6},

    // ═══════════════════════════════════════
    // 绿系 G0-G6 (生): 缥翠绿青苍黛玄
    // ═══════════════════════════════════════
    {C_G0, CS_GREEN, "缥", "piǎo", "生", 2, 0},
    {C_G1, CS_GREEN, "翠", "cuì",  "生", 2, 1},
    {C_G2, CS_GREEN, "绿", "lǜ",   "生", 2, 2},
    {C_G3, CS_GREEN, "青", "qīng", "生", 2, 3},
    {C_G4, CS_GREEN, "苍", "cāng", "生", 2, 4},
    {C_G5, CS_GREEN, "黛", "dài",  "生", 2, 5},
    {C_G6, CS_GREEN, "玄", "xuán", "生", 2, 6},

    // ═══════════════════════════════════════
    // 蓝系 B0-B6 (冷): 素银蓝紫靛绀黯
    // ═══════════════════════════════════════
    {C_B0, CS_BLUE,  "素", "sù",   "冷", 3, 0},
    {C_B1, CS_BLUE,  "银", "yín",  "冷", 3, 1},
    {C_B2, CS_BLUE,  "蓝", "lán",  "冷", 3, 2},
    {C_B3, CS_BLUE,  "紫", "zǐ",   "冷", 3, 3},
    {C_B4, CS_BLUE,  "靛", "diàn", "冷", 3, 4},
    {C_B5, CS_BLUE,  "绀", "gàn",  "冷", 3, 5},
    {C_B6, CS_BLUE,  "黯", "àn",   "冷", 3, 6},

    // ═══════════════════════════════════════
    // 泽系 S0-S6 (石): 玉琅晶璃珀瑙璧
    // ═══════════════════════════════════════
    {C_S0, CS_STONE, "玉", "yù",   "石", 4, 0},
    {C_S1, CS_STONE, "琅", "láng", "石", 4, 1},
    {C_S2, CS_STONE, "晶", "jīng", "石", 4, 2},
    {C_S3, CS_STONE, "璃", "lí",   "石", 4, 3},
    {C_S4, CS_STONE, "珀", "pò",   "石", 4, 4},
    {C_S5, CS_STONE, "瑙", "nǎo",  "石", 4, 5},
    {C_S6, CS_STONE, "璧", "bì",   "石", 4, 6},

    // ═══════════════════════════════════════
    // 光系 L0-L6 (韵): 曦辉霓旖靡暝黟
    // ═══════════════════════════════════════
    {C_L0, CS_LIGHT, "曦", "xī",   "韵", 5, 0},
    {C_L1, CS_LIGHT, "辉", "huī",  "韵", 5, 1},
    {C_L2, CS_LIGHT, "霓", "ní",   "韵", 5, 2},
    {C_L3, CS_LIGHT, "旖", "yǐ",   "韵", 5, 3},
    {C_L4, CS_LIGHT, "靡", "mí",   "韵", 5, 4},
    {C_L5, CS_LIGHT, "暝", "míng", "韵", 5, 5},
    {C_L6, CS_LIGHT, "黟", "yī",   "韵", 5, 6},
};

// ── 64进制符号→42色映射表 ──
// 按层级分配: 夫妻→红系, 中心→黄系, 感知→绿系, 骨干→蓝系, 物质→泽系, 高维→光系
static const cc_color_symbol_t symbol_color_map[CC_SYMBOL_COUNT] = {
    // 夫妻层(0-7): 红系 R0-R6 + 循环
    {0,  C_R0, CS_RED},    // 啊 → 妃(R0)
    {1,  C_R1, CS_RED},    // 哦 → 粉(R1)
    {2,  C_R2, CS_RED},    // 工 → 彤(R2)
    {3,  C_R3, CS_RED},    // 空 → 赤(R3)
    {4,  C_R4, CS_RED},    // 灰 → 棕(R4)
    {5,  C_R5, CS_RED},    // 开 → 绛(R5)
    {6,  C_R6, CS_RED},    // 知 → 赭(R6)
    {7,  C_R0, CS_RED},    // 出 → 妃(R0)

    // 中心层(8-15): 黄系 Y0-Y6
    {8,  C_Y0, CS_YELLOW}, // 西 → 缃(Y0)
    {9,  C_Y1, CS_YELLOW}, // 起 → 金(Y1)
    {10, C_Y2, CS_YELLOW}, // 机 → 黄(Y2)
    {11, C_Y3, CS_YELLOW}, // 挂 → 褐(Y3)
    {12, C_Y4, CS_YELLOW}, // 福 → 黧(Y4)
    {13, C_Y5, CS_YELLOW}, // 序 → 乌(Y5)
    {14, C_Y6, CS_YELLOW}, // 物 → 黑(Y6)
    {15, C_Y0, CS_YELLOW}, // 土 → 缃(Y0)

    // 感知层(16-23): 绿系 G0-G6
    {16, C_G0, CS_GREEN},  // 力 → 缥(G0)
    {17, C_G1, CS_GREEN},  // 亲 → 翠(G1)
    {18, C_G2, CS_GREEN},  // 量 → 绿(G2)
    {19, C_G3, CS_GREEN},  // 端 → 青(G3)
    {20, C_G4, CS_GREEN},  // 收 → 苍(G4)
    {21, C_G5, CS_GREEN},  // 面 → 黛(G5)
    {22, C_G6, CS_GREEN},  // 令 → 玄(G6)
    {23, C_G0, CS_GREEN},  // 一 → 缥(G0)

    // 骨干层(24-31): 蓝系 B0-B6
    {24, C_B0, CS_BLUE},   // 一 → 素(B0)
    {25, C_B1, CS_BLUE},   // 二 → 银(B1)
    {26, C_B2, CS_BLUE},   // 三 → 蓝(B2)
    {27, C_B3, CS_BLUE},   // 四 → 紫(B3)
    {28, C_B4, CS_BLUE},   // 五 → 靛(B4)
    {29, C_B5, CS_BLUE},   // 六 → 绀(B5)
    {30, C_B6, CS_BLUE},   // 七 → 黯(B6)
    {31, C_B0, CS_BLUE},   // 八 → 素(B0)

    // 行页层(32-39): 泽系 S0-S6
    {32, C_S0, CS_STONE},  // 十 → 玉(S0)
    {33, C_S1, CS_STONE},  // 百 → 琅(S1)
    {34, C_S2, CS_STONE},  // 千 → 晶(S2)
    {35, C_S3, CS_STONE},  // 万 → 璃(S3)
    {36, C_S4, CS_STONE},  // 层一 → 珀(S4)
    {37, C_S5, CS_STONE},  // 层二 → 瑙(S5)
    {38, C_S6, CS_STONE},  // 层三 → 璧(S6)
    {39, C_S0, CS_STONE},  // 层四 → 玉(S0)

    // 交汇层+价值层+流通层(40-55): 光系 L0-L6
    {40, C_L0, CS_LIGHT},  // 层五 → 曦(L0)
    {41, C_L1, CS_LIGHT},  // 层六 → 辉(L1)
    {42, C_L2, CS_LIGHT},  // 时 → 霓(L2)
    {43, C_L3, CS_LIGHT},  // 季 → 旖(L3)
    {44, C_L4, CS_LIGHT},  // 刻 → 靡(L4)
    {45, C_L5, CS_LIGHT},  // 流 → 暝(L5)
    {46, C_L6, CS_LIGHT},  // 合 → 黟(L6)
    {47, C_L0, CS_LIGHT},  // 伴 → 曦(L0)
    {48, C_L1, CS_LIGHT},  // 享 → 辉(L1)
    {49, C_L2, CS_LIGHT},  // 治 → 霓(L2)
    {50, C_L3, CS_LIGHT},  // 吉 → 旖(L3)
    {51, C_L4, CS_LIGHT},  // 凶 → 靡(L4)
    {52, C_L5, CS_LIGHT},  // 平 → 暝(L5)
    {53, C_L6, CS_LIGHT},  // 转 → 黟(L6)
    {54, C_L0, CS_LIGHT},  // 主 → 曦(L0)
    {55, C_L1, CS_LIGHT},  // 有 → 辉(L1)

    // 高维层(56-63): 混合六系
    {56, C_B2, CS_BLUE},   // 外 → 蓝(B2)
    {57, C_G2, CS_GREEN},  // 同 → 绿(G2)
    {58, C_S2, CS_STONE},  // 体 → 晶(S2)
    {59, C_L0, CS_LIGHT},  // 周 → 曦(L0)
    {60, C_R3, CS_RED},    // 灵 → 赤(R3)
    {61, C_Y2, CS_YELLOW}, // 魂 → 黄(Y2)
    {62, C_R0, CS_RED},    // 心 → 妃(R0)
    {63, C_B0, CS_BLUE},   // 念 → 素(B0)
};

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

const cc_color_info_t* cc_get_color_info(cc_color_t c) {
    if (c >= CC_TOTAL_COLORS) return NULL;
    return &color_info[c];
}

const cc_color_symbol_t* cc_get_symbol_color(uint8_t symbol) {
    if (symbol >= CC_SYMBOL_COUNT) return NULL;
    return &symbol_color_map[symbol];
}

const char* cc_get_color_name(uint8_t symbol) {
    if (symbol >= CC_SYMBOL_COUNT) return "无色";
    return color_info[symbol_color_map[symbol].color].name;
}

cc_color_system_t cc_get_symbol_system(uint8_t symbol) {
    if (symbol >= CC_SYMBOL_COUNT) return CS_RED;
    return symbol_color_map[symbol].system;
}

const char* cc_system_name(cc_color_system_t sys) {
    switch (sys) {
        case CS_RED:    return "R:红系(暖)";
        case CS_YELLOW: return "Y:黄系(光)";
        case CS_GREEN:  return "G:绿系(生)";
        case CS_BLUE:   return "B:蓝系(冷)";
        case CS_STONE:  return "S:泽系(石)";
        case CS_LIGHT:  return "L:光系(韵)";
        default:        return "?";
    }
}

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

cc_color_t cc_layer_color(uint8_t layer, uint8_t position) {
    if (layer >= 8 || position >= 7) return C_R0;
    static const cc_color_system_t layer_system[8] = {
        CS_RED, CS_YELLOW, CS_GREEN, CS_BLUE,
        CS_STONE, CS_LIGHT, CS_RED, CS_BLUE
    };
    cc_color_system_t sys = layer_system[layer];
    return (cc_color_t)(sys * 7 + position);
}

void cc_report(void) {
    printf("\n  ═══ 六系42色颜色定制 ═══\n");
    printf("  42色 = 6系 × 7色 (编码=纯数字序号)\n");
    printf("  ─────────────────────────────\n");

    for (int sys = 0; sys < CC_SYSTEM_COUNT; sys++) {
        printf("  %s[%s]: ", cc_system_name((cc_color_system_t)sys),
            cc_system_quality((cc_color_system_t)sys));
        for (int col = 0; col < CC_COLOR_PER_SYSTEM; col++) {
            int idx = sys * 7 + col;
            printf("%s ", color_info[idx].name);
        }
        printf("\n");
    }
    printf("  ─────────────────────────────\n");
    printf("  64符号→42色: 按层级分配 R/Y/G/B/S/L\n");
    printf("  ═════════════════════════════\n");
}
