// smrrll_voice.c — 声韵调音质进制实现
// 每一个进制符号 = 声母 + 韵母 + 声调 = 可发音的音节
// 用音质差异来区分所有层级、身份、量化关系

#include "smrrll_voice.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

// ── 声母名称 (用于调试/显示) ──
static const char* shengmu_name(uint8_t sm) {
    static const char* names[] = {
        "b","p","m","f",        // 唇音
        "d","t","n","l",        // 舌尖中
        "g","k","h",            // 舌根/喉
        "j","q","x",            // 舌面
        "zh","ch","sh","r",     // 舌尖后
        "z","c","s",            // 舌尖前
        "y","w",                // 半元音
        "∅",                    // 零声母
    };
    return (sm <= SM_ZERO) ? names[sm] : "?";
}

// ── 韵母名称 ──
static const char* yunmu_name(uint8_t ym) {
    static const char* names[] = {
        "a","o","e","i","u","ü",
        "ai","ei","ao","ou",
        "ia","ie","ua","uo","üe",
        "an","en","in","un","ün",
        "ang","eng","ing","ong",
        "iang","uang","ong2","uan",
        "iao","ian","uai","ui",
        "iu","iou","uei","er",
    };
    return (ym < YM_COUNT) ? names[ym] : "?";
}

// ── 声调符号 ──
static const char* tone_symbol(uint8_t t) {
    static const char* marks[] = {"ˉ","ˊ","ˇ","ˋ","·"}; // 阴平/阳平/上声/去声/轻声
    return (t < 5) ? marks[t] : "?";
}

// ── 64音质符号的完整声韵调定义 ──
// 每个符号都必须是一个真实可发音的音节
static const smv_voice_t voice_table[SV_SYMBOL_COUNT] = {
    // ── 夫妻层 ──
    /* 0*/  {SM_ZERO,YM_A,  TONE_1, "啊", "ā"},      // 妻=0 开口音
    /* 1*/  {SM_ZERO,YM_O,  TONE_1, "哦", "ō"},      // 夫=1 合口音

    // ── 中心层 ──
    /* 2*/  {SM_G,   YM_ONG, TONE_1, "工", "gōng"},  // 工藤新一=2
    /* 3*/  {SM_K,   YM_ONG, TONE_1, "空", "kōng"},  // 柯南=3

    // ── 感知层 ──
    /* 4*/  {SM_H,   YM_UI,  TONE_1, "灰", "huī"},   // 灰原哀=4
    /* 5*/  {SM_K,   YM_AI,  TONE_1, "开", "kāi"},   // 毛利兰=5

    // ── 骨干层 ──
    /* 6*/  {SM_ZH,  YM_I,   TONE_1, "知", "zhī"},   // 骨/结构
    /* 7*/  {SM_CH,  YM_U,   TONE_1, "出", "chū"},   // 头/方向

    // ── 行页层 ──
    /* 8*/  {SM_X,   YM_I,   TONE_1, "西", "xī"},    // 行业
    /* 9*/  {SM_Q,   YM_I,   TONE_3, "起", "qǐ"},    // 行动

    // ── 交汇层 ──
    /*10*/  {SM_J,   YM_I,   TONE_1, "机", "jī"},    // 聚集
    /*11*/  {SM_G,   YM_UA,  TONE_4, "挂", "guà"},   // 挂系

    // ── 价值层 ──
    /*12*/  {SM_F,   YM_U,   TONE_2, "福", "fú"},    // 价值/福报
    /*13*/  {SM_X,   YM_U,   TONE_4, "序", "xù"},    // 序位

    // ── 流通层 ──
    /*14*/  {SM_W,   YM_U,   TONE_4, "物", "wù"},    // 物流
    /*15*/  {SM_T,   YM_U,   TONE_3, "土", "tǔ"},    // 土德

    // ── 连结层 ──
    /*16*/  {SM_L,   YM_I,   TONE_4, "力", "lì"},    // 力动
    /*17*/  {SM_Q,   YM_IN,  TONE_1, "亲", "qīn"},   // 亲情

    // ── 空间层 ──
    /*18*/  {SM_L,   YM_IANG,TONE_4, "量", "liàng"}, // 量化
    /*19*/  {SM_D,   YM_UAN, TONE_1, "端", "duān"},  // 端点

    // ── 聚合层 ──
    /*20*/  {SM_SH,  YM_OU,  TONE_1, "收", "shōu"},  // 收纳
    /*21*/  {SM_M,   YM_IAN, TONE_4, "面", "miàn"},  // 界面

    // ── 管理层 ──
    /*22*/  {SM_L,   YM_ING, TONE_4, "令", "lìng"},  // 指令
    /*23*/  {SM_Y,   YM_I,   TONE_1, "一", "yī"},    // 唯一

    // ── 数字层 ──
    /*24*/  {SM_Y,   YM_I,   TONE_1, "一", "yī"},    // 起始
    /*25*/  {SM_ZERO,YM_ER,  TONE_4, "二", "èr"},    // 分裂
    /*26*/  {SM_S,   YM_AN,  TONE_1, "三", "sān"},   // 见证
    /*27*/  {SM_S,   YM_I,   TONE_4, "四", "sì"},    // 稳定 (四边形=及格)
    /*28*/  {SM_W,   YM_U,   TONE_3, "五", "wǔ"},    // 中心 (五边形=加分)
    /*29*/  {SM_L,   YM_IU,  TONE_4, "六", "liù"},   // 流动
    /*30*/  {SM_Q,   YM_I,   TONE_1, "七", "qī"},    // 变化
    /*31*/  {SM_B,   YM_A,   TONE_1, "八", "bā"},    // 分界

    // ── 组织扩展层 ──
    /*32*/  {SM_SH,  YM_I,   TONE_2, "十", "shí"},   // 组织节点
    /*33*/  {SM_B,   YM_AI,  TONE_3, "百", "bǎi"},   // 整体
    /*34*/  {SM_Q,   YM_IAN, TONE_1, "千", "qiān"},  // 规模
    /*35*/  {SM_W,   YM_AN,  TONE_4, "万", "wàn"},   // 广大

    // ── 层级编码层 ──
    /*36*/  {SM_D,   YM_I,   TONE_1, "低", "dī"},    // 夫妻层
    /*37*/  {SM_D,   YM_I,   TONE_2, "敌", "dí"},    // 中心层
    /*38*/  {SM_D,   YM_I,   TONE_3, "底", "dǐ"},    // 感知层
    /*39*/  {SM_D,   YM_I,   TONE_4, "地", "dì"},    // 骨干层
    /*40*/  {SM_G,   YM_AO,  TONE_1, "高", "gāo"},   // 横向层
    /*41*/  {SM_J,   YM_IA,  TONE_1, "加", "jiā"},   // 价值层

    // ── 时间层 ──
    /*42*/  {SM_SH,  YM_I,   TONE_2, "时", "shí"},   // 时间
    /*43*/  {SM_J,   YM_I,   TONE_4, "季", "jì"},    // 周期
    /*44*/  {SM_K,   YM_E,   TONE_4, "刻", "kè"},    // 时刻
    /*45*/  {SM_L,   YM_IU,  TONE_2, "流", "liú"},   // 流动

    // ── 关系层 ──
    /*46*/  {SM_H,   YM_E,   TONE_2, "合", "hé"},    // 联合
    /*47*/  {SM_B,   YM_AN,  TONE_4, "伴", "bàn"},   // 绑定
    /*48*/  {SM_X,   YM_IANG,TONE_3, "享", "xiǎng"}, // 共享
    /*49*/  {SM_ZH,  YM_I,   TONE_4, "治", "zhì"},   // 治理

    // ── 吉凶层 ──
    /*50*/  {SM_J,   YM_I,   TONE_2, "吉", "jí"},    // 吉/福
    /*51*/  {SM_X,   YM_ONG, TONE_1, "凶", "xiōng"}, // 凶/祸
    /*52*/  {SM_P,   YM_ING, TONE_2, "平", "píng"},  // 平/中
    /*53*/  {SM_ZH,  YM_UAN, TONE_3, "转", "zhuǎn"}, // 转/变化

    // ── 主权层 ──
    /*54*/  {SM_ZH,  YM_U,   TONE_3, "主", "zhǔ"},   // 主权
    /*55*/  {SM_Y,   YM_OU,  TONE_3, "有", "yǒu"},   // 所有者
    /*56*/  {SM_W,   YM_AI,  TONE_4, "外", "wài"},   // 外部
    /*57*/  {SM_T,   YM_ONG, TONE_2, "同", "tóng"},  // 已融合

    // ── 高维层 ──
    /*58*/  {SM_T,   YM_I,   TONE_3, "体", "tǐ"},    // 三维
    /*59*/  {SM_ZH,  YM_OU,  TONE_1, "周", "zhōu"},  // 四维/周长
    /*60*/  {SM_L,   YM_ING, TONE_2, "灵", "líng"},  // 灵/意识
    /*61*/  {SM_H,   YM_UN,  TONE_2, "魂", "hún"},   // 灵魂
    /*62*/  {SM_X,   YM_IN,  TONE_1, "心", "xīn"},   // 心/锚点
    /*63*/  {SM_N,   YM_IAN, TONE_4, "念", "niàn"},  // 念/思想
};

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

// 获取完整声韵调信息
const smv_voice_t* smv_get_voice(smv_symbol_t sym) {
    if (sym >= SV_SYMBOL_COUNT) return NULL;
    return &voice_table[sym];
}

// 获取拼音
const char* smv_get_pinyin(smv_symbol_t sym) {
    if (sym >= SV_SYMBOL_COUNT) return "?";
    return voice_table[sym].pinyin;
}

// 获取汉字
const char* smv_get_hanzi(smv_symbol_t sym) {
    if (sym >= SV_SYMBOL_COUNT) return "?";
    return voice_table[sym].hanzi;
}

// 最大公约数 (欧几里得)
uint32_t smv_gcd(uint32_t a, uint32_t b) {
    while (b) { uint32_t t = b; b = a % b; a = t; }
    return a;
}

// 最小公倍数
uint32_t smv_lcm(uint32_t a, uint32_t b) {
    if (a == 0 || b == 0) return 0;
    return (a / smv_gcd(a, b)) * b;
}

// 音质角度 (在384°圆周上的相位)
float smv_symbol_angle(smv_symbol_t sym) {
    if (sym >= SV_SYMBOL_COUNT) return 0.0f;
    return (float)sym * SV_DEGREE_UNIT;  // sym × 6°
}

// 音质距离: 声韵调空间的差异度
// 声母差异 (发音部位/方法差异) + 韵母差异 (开口度差异) + 声调差异
float smv_voice_distance(smv_symbol_t a, smv_symbol_t b) {
    if (a >= SV_SYMBOL_COUNT || b >= SV_SYMBOL_COUNT) return 1.0f;
    const smv_voice_t* va = &voice_table[a];
    const smv_voice_t* vb = &voice_table[b];

    // 声母距离: 0-21 → 归一化 0-1
    float sm_dist = (float)abs((int)va->shengmu - (int)vb->shengmu) / (float)SV_SHENGMU_COUNT;

    // 韵母距离: 0-35 → 归一化 0-1
    float ym_dist = (float)abs((int)va->yunmu - (int)vb->yunmu) / (float)SV_YUNMU_COUNT;

    // 声调距离: 0-4 → 归一化 0-1
    float tn_dist = (float)abs((int)va->tone - (int)vb->tone) / (float)SV_TONE_COUNT;

    // 三维声韵调空间的综合距离
    return (sm_dist + ym_dist + tn_dist) / 3.0f;
}

// 异质互换: 两个符号是否足够不同
bool smv_is_distinct(smv_symbol_t a, smv_symbol_t b) {
    if (a == b) return false;
    if (a >= SV_SYMBOL_COUNT || b >= SV_SYMBOL_COUNT) return true;
    const smv_voice_t* va = &voice_table[a];
    const smv_voice_t* vb = &voice_table[b];

    // 声母不同 OR 韵母不同 OR 声调不同 = 可区分
    return (va->shengmu != vb->shengmu) ||
           (va->yunmu != vb->yunmu) ||
           (va->tone != vb->tone);
}

// 正交检查: 两符号在384°圆周上是否成90° (最大容量关系)
bool smv_is_orthogonal(smv_symbol_t a, smv_symbol_t b) {
    if (a >= SV_SYMBOL_COUNT || b >= SV_SYMBOL_COUNT) return false;
    float angle_a = smv_symbol_angle(a);
    float angle_b = smv_symbol_angle(b);
    float diff = fabsf(angle_a - angle_b);
    if (diff > 180.0f) diff = 384.0f - diff;
    // 90° = 15个符号间距 (90/6=15)
    float target = 90.0f;
    return fabsf(diff - target) < SV_DEGREE_UNIT;  // 允许±6°误差
}
