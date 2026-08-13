// color_custom.c — 颜色定制进制实现
// 每个进制符号绑定一个颜色: 声母→色相, 韵母→饱和度, 声调→明度

#include "color_custom.h"
#include <stdio.h>
#include <math.h>
#include <string.h>

// ── 声母→色相分区 (384°圆周分为7区) ──
// 每区占 384/7 ≈ 54.86°, 按发音部位从唇到喉排列
static float shengmu_hue_base(smv_shengmu_t sm) {
    switch (sm) {
        case SM_B: case SM_P: case SM_M: case SM_F:
            return 0.0f;        // 唇音 → 红色系 (0°)
        case SM_D: case SM_T: case SM_N: case SM_L:
            return 55.0f;       // 舌尖中 → 黄色系 (55°)
        case SM_G: case SM_K: case SM_H:
            return 110.0f;      // 舌根/喉 → 绿色系 (110°)
        case SM_J: case SM_Q: case SM_X:
            return 165.0f;      // 舌面 → 青色系 (165°)
        case SM_ZH: case SM_CH: case SM_SH: case SM_R:
            return 220.0f;      // 舌尖后 → 蓝色系 (220°)
        case SM_Z: case SM_C: case SM_S:
            return 275.0f;      // 舌尖前 → 紫色系 (275°)
        case SM_Y: case SM_W:
            return 330.0f;      // 半元音 → 粉色系 (330°)
        case SM_ZERO:
            return 360.0f;      // 零声母 → 白色/中性 (360°)
        default:
            return 0.0f;
    }
}

// ── 声母→色相偏移 (同一分区内的细微差异) ──
static float shengmu_hue_offset(smv_shengmu_t sm) {
    switch (sm) {
        case SM_B:  return 0.0f;
        case SM_P:  return 8.0f;
        case SM_M:  return 16.0f;
        case SM_F:  return 24.0f;
        case SM_D:  return 0.0f;
        case SM_T:  return 8.0f;
        case SM_N:  return 16.0f;
        case SM_L:  return 24.0f;
        case SM_G:  return 0.0f;
        case SM_K:  return 16.0f;
        case SM_H:  return 32.0f;
        case SM_J:  return 0.0f;
        case SM_Q:  return 16.0f;
        case SM_X:  return 32.0f;
        case SM_ZH: return 0.0f;
        case SM_CH: return 12.0f;
        case SM_SH: return 24.0f;
        case SM_R:  return 36.0f;
        case SM_Z:  return 0.0f;
        case SM_C:  return 16.0f;
        case SM_S:  return 32.0f;
        case SM_Y:  return 0.0f;
        case SM_W:  return 24.0f;
        case SM_ZERO: return 0.0f;
        default: return 0.0f;
    }
}

// ── 韵母→饱和度 ──
// 开口韵(a/o/e) → 高饱和, 合口韵(u/ü) → 低饱和, 鼻韵母 → 中饱和
static float yunmu_saturation(smv_yunmu_t ym) {
    switch (ym) {
        case YM_A:  case YM_O:  case YM_E:
            return 0.95f;       // 开口韵 → 高饱和
        case YM_AI: case YM_EI: case YM_AO: case YM_OU:
            return 0.85f;       // 复韵母 → 中高饱和
        case YM_AN: case YM_EN: case YM_ANG: case YM_ENG:
            return 0.70f;       // 前鼻韵 → 中饱和
        case YM_IN: case YM_UN: case YM_ING: case YM_ONG:
            return 0.60f;       // 后鼻韵 → 中低饱和
        case YM_I:  case YM_U:  case YM_V:
            return 0.50f;       // 闭口韵 → 低饱和
        default:
            return 0.65f;       // 其他 → 中等
    }
}

// ── 声调→明度 ──
// 阴平(高平) → 最亮, 阳平(升) → 较亮, 上声(降升) → 较暗, 去声(全降) → 最暗, 轻声 → 中等
static float tone_lightness(smv_tone_t t) {
    switch (t) {
        case TONE_1: return 0.75f;   // 阴平 → 最亮
        case TONE_2: return 0.65f;   // 阳平 → 较亮
        case TONE_3: return 0.45f;   // 上声 → 较暗
        case TONE_4: return 0.35f;   // 去声 → 最暗
        case TONE_0: return 0.55f;   // 轻声 → 中等
        default:     return 0.50f;
    }
}

// ── 颜色名称表 (按层级) ──
static const char* color_names[SV_SYMBOL_COUNT] = {
    // 夫妻层
    "柔白", "暖橙",
    // 中心层
    "正绿", "墨绿",
    // 感知层
    "青蓝", "翠绿",
    // 骨干层
    "深蓝", "天蓝",
    // 行页层
    "靛蓝", "蓝紫",
    // 交汇层
    "紫色", "紫红",
    // 价值层
    "金色", "橙黄",
    // 流通层
    "土黄", "棕色",
    // 连结层
    "红色", "粉红",
    // 空间层
    "青色", "深青",
    // 聚合层
    "玫红", "浅紫",
    // 管理层
    "冷蓝", "纯白",
    // 数字层
    "纯白", "亮橙", "正绿", "深蓝", "紫色", "青色", "蓝绿", "红色",
    // 组织扩展层
    "金色", "橙色", "黄色", "米色",
    // 层级编码层
    "暗绿", "中绿", "亮绿", "深绿", "黄绿", "翠绿",
    // 时间层
    "暖黄", "冷绿", "深紫", "浅蓝",
    // 关系层
    "橙色", "粉色", "浅红", "深蓝",
    // 吉凶层
    "大红", "深灰", "中灰", "橙色",
    // 主权层
    "朱红", "浅橙", "灰白", "橙色",
    // 高维层
    "深紫", "蓝紫", "银白", "深紫", "浅粉", "浅蓝",
};

// ── 64颜色符号表 (运行时生成) ──
static cc_color_symbol_t color_table[SV_SYMBOL_COUNT];
static bool table_initialized = false;

// ── 初始化颜色表 ──
static void init_color_table(void) {
    if (table_initialized) return;

    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        smv_symbol_t sym = (smv_symbol_t)i;
        const smv_voice_t* v = smv_get_voice(sym);
        if (!v) continue;

        cc_color_symbol_t* cs = &color_table[i];
        cs->symbol = sym;
        cs->color_name = color_names[i];
        cs->layer = (uint8_t)(i / 8);  // 每8个符号一层

        // 色相 = 声母分区 + 声母偏移
        float hue = shengmu_hue_base(v->shengmu) + shengmu_hue_offset(v->shengmu);
        if (hue >= 384.0f) hue -= 384.0f;
        cs->hsl.hue = hue;

        // 饱和度 = 韵母决定
        cs->hsl.saturation = yunmu_saturation(v->yunmu);

        // 明度 = 声调决定
        cs->hsl.lightness = tone_lightness(v->tone);

        // HSL → RGB
        cs->rgb = cc_hsl_to_rgb(cs->hsl);
    }

    table_initialized = true;
}

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

const cc_color_symbol_t* cc_get_color(smv_symbol_t sym) {
    init_color_table();
    if (sym >= SV_SYMBOL_COUNT) return NULL;
    return &color_table[sym];
}

// HSL → RGB转换
cc_rgb_t cc_hsl_to_rgb(cc_hsl_t hsl) {
    cc_rgb_t rgb;
    float h = hsl.hue / 60.0f;  // 将色相转为60°单位
    float s = hsl.saturation;
    float l = hsl.lightness;

    float c = (1.0f - fabsf(2.0f * l - 1.0f)) * s;
    float x = c * (1.0f - fabsf(fmodf(h, 2.0f) - 1.0f));
    float m = l - c / 2.0f;

    float r = 0, g = 0, b = 0;
    if (h < 1)      { r = c; g = x; b = 0; }
    else if (h < 2) { r = x; g = c; b = 0; }
    else if (h < 3) { r = 0; g = c; b = x; }
    else if (h < 4) { r = 0; g = x; b = c; }
    else if (h < 5) { r = x; g = 0; b = c; }
    else            { r = c; g = 0; b = x; }

    rgb.r = (uint8_t)((r + m) * 255);
    rgb.g = (uint8_t)((g + m) * 255);
    rgb.b = (uint8_t)((b + m) * 255);
    return rgb;
}

// RGB → HSL转换
cc_hsl_t cc_rgb_to_hsl(cc_rgb_t rgb) {
    cc_hsl_t hsl;
    float r = rgb.r / 255.0f;
    float g = rgb.g / 255.0f;
    float b = rgb.b / 255.0f;

    float max = (r > g) ? ((r > b) ? r : b) : ((g > b) ? g : b);
    float min = (r < g) ? ((r < b) ? r : b) : ((g < b) ? g : b);
    float delta = max - min;

    // 明度
    hsl.lightness = (max + min) / 2.0f;

    // 饱和度
    if (delta < 0.001f) {
        hsl.saturation = 0;
        hsl.hue = 0;
    } else {
        hsl.saturation = (hsl.lightness < 0.5f) ?
            delta / (max + min) : delta / (2.0f - max - min);

        // 色相
        float h;
        if (r == max)      h = (g - b) / delta;
        else if (g == max) h = 2.0f + (b - r) / delta;
        else               h = 4.0f + (r - g) / delta;
        h *= 60.0f;
        if (h < 0) h += 360.0f;
        hsl.hue = h;
    }
    return hsl;
}

// 颜色关系
cc_color_relation_t cc_color_relation(smv_symbol_t a, smv_symbol_t b) {
    init_color_table();
    if (a >= SV_SYMBOL_COUNT || b >= SV_SYMBOL_COUNT) return CR_NEUTRAL;
    if (a == b) return CR_SAME;

    float hue_a = color_table[a].hsl.hue;
    float hue_b = color_table[b].hsl.hue;
    float diff = fabsf(hue_a - hue_b);
    if (diff > 180.0f) diff = 384.0f - diff;

    if (diff < 15.0f) return CR_ANALOGY;         // 类似色
    if (fabsf(diff - 120.0f) < 15.0f) return CR_WITNESS;  // 三原色见证
    if (fabsf(diff - 180.0f) < 15.0f) return CR_MIRROR;   // 互补镜像
    return CR_NEUTRAL;
}

// 颜色距离
float cc_color_distance(smv_symbol_t a, smv_symbol_t b) {
    init_color_table();
    if (a >= SV_SYMBOL_COUNT || b >= SV_SYMBOL_COUNT) return 384.0f;
    float diff = fabsf(color_table[a].hsl.hue - color_table[b].hsl.hue);
    if (diff > 180.0f) diff = 384.0f - diff;
    return diff;
}

// 获取颜色名称
const char* cc_get_color_name(smv_symbol_t sym) {
    init_color_table();
    if (sym >= SV_SYMBOL_COUNT) return "无色";
    return color_table[sym].color_name;
}

// 层级颜色
cc_rgb_t cc_get_layer_color(cc_color_layer_t layer) {
    cc_hsl_t hsl;
    switch (layer) {
        case CL_WARM:  hsl.hue = 0;   hsl.saturation = 0.9f; hsl.lightness = 0.6f; break;
        case CL_EARTH: hsl.hue = 55;  hsl.saturation = 0.8f; hsl.lightness = 0.5f; break;
        case CL_COOL:  hsl.hue = 110; hsl.saturation = 0.7f; hsl.lightness = 0.5f; break;
        case CL_LEAF:  hsl.hue = 165; hsl.saturation = 0.7f; hsl.lightness = 0.5f; break;
        case CL_GOLD:  hsl.hue = 220; hsl.saturation = 0.8f; hsl.lightness = 0.5f; break;
        case CL_SAND:  hsl.hue = 275; hsl.saturation = 0.6f; hsl.lightness = 0.6f; break;
        case CL_AIR:   hsl.hue = 330; hsl.saturation = 0.5f; hsl.lightness = 0.7f; break;
        default:       hsl.hue = 0;   hsl.saturation = 0;   hsl.lightness = 0.5f; break;
    }
    return cc_hsl_to_rgb(hsl);
}

// ANSI终端着色输出
void cc_print_colored(smv_symbol_t sym, const char* text) {
    init_color_table();
    if (sym >= SV_SYMBOL_COUNT) {
        printf("%s", text);
        return;
    }
    cc_rgb_t rgb = color_table[sym].rgb;
    // 使用24位真彩色ANSI转义
    printf("\033[38;2;%d;%d;%dm%s\033[0m", rgb.r, rgb.g, rgb.b, text);
}

// 颜色报表
void cc_report(void) {
    init_color_table();
    printf("\n  ═══ 颜色定制进制 ═══\n");
    printf("  64符号 = 64种颜色\n");
    printf("  声母→色相 | 韵母→饱和度 | 声调→明度\n");
    printf("  ─────────────────────────────\n");

    // 按层级显示颜色
    const char* layer_names[] = {
        "夫妻层", "中心层", "感知层", "骨干层",
        "行页层", "交汇层", "价值层", "流通层",
        "连结层", "空间层", "聚合层", "管理层",
        "数字层", "组织层", "层级层", "时间层",
        "关系层", "吉凶层", "主权层", "高维层"
    };

    for (int layer = 0; layer < 8; layer++) {
        printf("  [%s] ", layer_names[layer]);
        for (int i = 0; i < 8; i++) {
            int sym = layer * 8 + i;
            cc_rgb_t rgb = color_table[sym].rgb;
            printf("\033[38;2;%d;%d;%dm%s\033[0m ",
                rgb.r, rgb.g, rgb.b,
                smv_get_hanzi((smv_symbol_t)sym));
        }
        printf("\n");
    }
    printf("  ═════════════════════════════\n");
}
