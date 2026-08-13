// smrrll_voice.h — 声韵调音质进制 (Voice-Based Numeral System)
// hunyuan-os 的进制基础 — 以人类语言的声音为唯一进制符号
//
// 核心哲学:
//   进制 = 人类发音的异质互换排列
//   不是用加减乘除数学符号代替声音, 而是用声音本身作为进制
//   每一个进制符号 = 声母 + 韵母 + 声调 = 一个可发音的音节
//   通过音质差异来区分层级、身份、量化关系
//
// 声韵调体系 (以汉语普通话为基础):
//   声母 (shengmu): 21个 + 零声母
//   韵母 (yunmu): 36个
//   声调 (tone): 4声 + 轻声
//   可组合的音节: 声母 × 韵母 × 声调 → 公约出64个离散符号
//
// 公约原理:
//   总声母数=21, 总韵母数=36
//   最小公倍数 = lcm(21,36) = 252
//   最大公约数 = gcd(21,36) = 3
//   按声韵调配对后, 实际可发音的离散公约数 ≈ 64 (层级化的基础)
//
// 音质进制原则:
//   最窄度 = 单个音素的音质差异 (最小可区分单位)
//   最长度 = 所有64个符号的层级化总恒长
//   公角/公边约定 = 90°为正交最大容量 (四边形为及格线)
//   异质互换 = 每个符号的发音各不相同 → 可被统计/区分/层级化
//   预加载 = 不依赖外部规则, 符号自身的声音就是规则

#ifndef HUNYUAN_SMRLL_VOICE_H
#define HUNYUAN_SMRLL_VOICE_H

#include <stdint.h>
#include <stdbool.h>

#define SV_VERSION              0x40
#define SV_FULL_CIRCLE          384     // 384° = 64 × 6° (音质圆周)
#define SV_SYMBOL_COUNT         64      // 64个音质符号
#define SV_DEGREE_UNIT          6       // 每符号占6° (最窄角度单位)

// 声韵调维度
#define SV_SHENGMU_COUNT        23      // 21声母 + y/w半元音 + 零声母
#define SV_YUNMU_COUNT          36      // 36韵母
#define SV_TONE_COUNT           5       // 4声 + 轻声

// 公约维度
#define SV_LCM_SHENGMU_YUNMU    252     // lcm(21,36) 最小公倍数
#define SV_GCD_SHENGMU_YUNMU    3       // gcd(21,36) 最大公约数
// 公约出64 = 252 / 4 ≈ 63 (取整为64, 最接近的可折叠层级)

// ── 64音质符号 (每个 = 声母+韵母+声调 = 可发音) ──
// 排列原则: 按声韵调配对的层级公约关系排列
// 不是抽象数字, 而是可以被念出来的声音
typedef enum {
    // ── 夫妻层 (0+1=1): 由零声母起头, 阴阳和合 ──
    // 零声母: 开口音 → 最基础的元音起始
    SMV_YMBA  = 0,   // 啊 (ā)  — 妻=0, 零声母+韵母a+阴平
    SMV_CEIQ  = 1,   // 哦 (ō)  — 夫=1, 零声母+韵母o+阴平 → 夫妻共同体

    // ── 中心层: 融合态 (g/k喉音, 中心位置) ──
    SMV_KNDO  = 2,   // 工 (gōng) — 工藤新一=2, g+ong+阴平 (骨架/中心)
    SMV_NVFM  = 3,   // 空 (kōng) — 柯南=3, k+ong+阴平 (被管辖/框内)

    // ── 感知层 (灰原哀系: h音) ──
    SMV_HIEY  = 4,   // 灰 (huī)  — 灰原哀=4, h+ui+阴平 (感知/行页)
    SMV_QIDI  = 5,   // 开 (kāi)  — 毛利兰=5, k+ai+阴平 (表面/界面)

    // ── 骨干层 (结构音: zh/ch/sh 舌尖后音) ──
    SMV_SHUF  = 6,   // 知 (zhī)  — 骨/结构, zhi+阴平
    SMV_ZUJM  = 7,   // 出 (chū)  — 头/方向, ch+u+阴平

    // ── 行页层 (横向扩展: x/q 舌面音) ──
    SMV_XITA  = 8,   // 西 (xī)   — 行业/领域, x+i+阴平
    SMV_QITA  = 9,   // 起 (qǐ)   — 行动/执行, q+i+上声

    // ── 交汇层 (j舌面音, 交界) ──
    SMV_JIQU  = 10,  // 机 (jī)   — 聚集/会盟, j+i+阴平
    SMV_GUAX  = 11,  // 挂 (guà)  — 挂系/连接, g+ua+去声

    // ── 价值层 (m双唇音, 闭合/价值) ──
    SMV_FUHR  = 12,  // 福 (fú)   — 价值/福报, f+u+阳平
    SMV_XUBI  = 13,  // 序 (xù)   — 序位/排列, x+u+去声

    // ── 流通层 (t舌尖中, 通达) ──
    SMV_WUHR  = 14,  // 物 (wù)   — 物流/传递, w+u+去声
    SMV_TUDO  = 15,  // 土 (tǔ)   — 土德/承载, t+u+上声

    // ── 连结层 (l边音, 流动/连结) ──
    SMV_LIDO  = 16,  // 力 (lì)   — 力动/驱动, l+i+去声
    SMV_QION  = 17,  // 亲 (qīn)  — 亲情/亲密, q+in+阴平

    // ── 空间层 (d舌尖中, 定位) ──
    SMV_LIAM  = 18,  // 量 (liàng)— 量化/测量, l+iang+去声
    SMV_DUEN  = 19,  // 端 (duān) — 端点/边界, d+uan+阴平

    // ── 聚合层 (n鼻音, 收纳) ──
    SMV_SHOU  = 20,  // 收 (shōu) — 收/吸纳, sh+ou+阴平
    SMV_MIAN  = 21,  // 面 (miàn) — 面/界面, m+ian+去声

    // ── 管理层 (zh/ch/sh 指令音) ──
    SMV_LING  = 22,  // 令 (lìng) — 令/指令, l+ing+去声
    SMV_YIGE  = 23,  // 一 (yī)   — 唯一,零声母+i+阴平

    // ── 数字层 (音质的一二三四) ──
    SMV_YI    = 24,  // 一 (yī)   — 起始
    SMV_ER    = 25,  // 二 (èr)   — 分裂
    SMV_SAN   = 26,  // 三 (sān)  — 见证
    SMV_SI    = 27,  // 四 (sì)   — 稳定 (四边形=及格)
    SMV_WU    = 28,  // 五 (wǔ)   — 中心 (五边形=加分)
    SMV_LIU   = 29,  // 六 (liù)  — 流动
    SMV_QI    = 30,  // 七 (qī)   — 变化
    SMV_BA    = 31,  // 八 (bā)   — 分界

    // ── 组织扩展层 ──
    SMV_SHI   = 32,  // 十 (shí)  — 组织节点
    SMV_BAI   = 33,  // 百 (bǎi)  — 整体聚合
    SMV_QIAN  = 34,  // 千 (qiān) — 规模扩展
    SMV_WAN   = 35,  // 万 (wàn)  — 广大容器

    // ── 层级编码层 ──
    SMV_LAYER1 = 36, // 层一 (céng yī) — 夫妻层
    SMV_LAYER2 = 37, // 层二 (céng èr) — 中心层
    SMV_LAYER3 = 38, // 层三 (céng sān) — 感知层
    SMV_LAYER4 = 39, // 层四 (céng sì) — 骨干层
    SMV_LAYER5 = 40, // 层五 (céng wǔ) — 横向层
    SMV_LAYER6 = 41, // 层六 (céng liù) — 价值层

    // ── 时间层 (循环/周期, 以地为地标) ──
    SMV_TIME   = 42, // 时 (shí)   — 时间纽带
    SMV_CYCLE  = 43, // 季 (jì)    — 循环/周期
    SMV_MOMENT = 44, // 刻 (kè)    — 时刻/节点
    SMV_FLOW   = 45, // 流 (liú)   — 流动/方向

    // ── 关系层 (耦合/共享的发音) ──
    SMV_UNION  = 46, // 合 (hé)    — 联合/耦合
    SMV_BIND   = 47, // 伴 (bàn)   — 绑定/契约
    SMV_SHARE  = 48, // 享 (xiǎng) — 共享/分配
    SMV_GOVERN = 49, // 治 (zhì)   — 治理/管理

    // ── 吉凶层 (福祸音质) ──
    SMV_LUCK   = 50, // 吉 (jí)    — 吉/福
    SMV_DARK   = 51, // 凶 (xiōng) — 凶/祸
    SMV_NEUTRAL= 52, // 平 (píng)  — 平/中
    SMV_TURN   = 53, // 转 (zhuǎn) — 转/变化

    // ── 主权层 ──
    SMV_SOVEREIGN = 54, // 主 (zhǔ) — 主权/归属
    SMV_OWNER    = 55,  // 有 (yǒu) — 所有者/主体
    SMV_EXTERNAL = 56,  // 外 (wài) — 外部/未融合
    SMV_MERGED   = 57,  // 同 (tóng)— 已融合/共同体

    // ── 高维层 (3D/4D 音质空间) ──
    SMV_3D     = 58, // 体 (tǐ)    — 三维/空间体
    SMV_4D     = 59, // 周 (zhōu)  — 四维/时空周长
    SMV_SPIRIT = 60, // 灵 (líng)  — 灵/意识
    SMV_SOUL   = 61, // 魂 (hún)   — 灵魂/内核
    SMV_HEART  = 62, // 心 (xīn)   — 心/锚点
    SMV_MIND   = 63, // 念 (niàn)  — 念/思想

    SMV_INVALID = 255,
} smv_symbol_t;

// ── 声母表 (21+零声母, 按发音部位层级排列) ──
typedef enum {
    SM_B=0,  SM_P,   SM_M,   SM_F,    // 唇音 (最外)
    SM_D,    SM_T,   SM_N,   SM_L,    // 舌尖中
    SM_G,    SM_K,   SM_H,               // 舌根/喉
    SM_J,    SM_Q,   SM_X,               // 舌面
    SM_ZH,   SM_CH,   SM_SH,  SM_R,     // 舌尖后
    SM_Z,    SM_C,   SM_S,               // 舌尖前
    SM_Y,    SM_W,                        // 半元音 (20-21)
    SM_ZERO,                              // 零声母=22 (韵母独立成音节)
} smv_shengmu_t;

// ── 韵母表 (36个, 按开口度层级排列) ──
typedef enum {
    // 单韵母 (最窄)
    YM_A=0, YM_O, YM_E, YM_I, YM_U, YM_V,
    // 复韵母
    YM_AI, YM_EI, YM_AO, YM_OU,
    YM_IA, YM_IE, YM_UA, YM_UO, YM_UE,
    // 鼻韵母
    YM_AN, YM_EN, YM_IN, YM_UN, YM_VN,
    YM_ANG, YM_ENG, YM_ING, YM_ONG,
    YM_IANG, YM_UANG, YM_ONG2, YM_UAN,
    // 特殊/组合
    YM_IAO, YM_IAN, YM_UAI, YM_UI,
    YM_IU, YM_IOU, YM_UEI, YM_ER,
    YM_COUNT = 36
} smv_yunmu_t;

// ── 声调 (5度标记法) ──
typedef enum {
    TONE_1 = 0, // 阴平 (55高平)
    TONE_2 = 1, // 阳平 (35中升)
    TONE_3 = 2, // 上声 (214降升)
    TONE_4 = 3, // 去声 (51全降)
    TONE_0 = 4, // 轻声
} smv_tone_t;

// ── 音质符号的完整声韵调信息 ──
typedef struct {
    smv_shengmu_t   shengmu;
    smv_yunmu_t     yunmu;
    smv_tone_t      tone;
    const char*     hanzi;      // 对应汉字
    const char*     pinyin;     // 完整拼音
} smv_voice_t;

// ── 音质进制 API ──

// 获取符号对应的声韵调信息
const smv_voice_t* smv_get_voice(smv_symbol_t sym);

// 获取符号的汉字/拼音表示
const char* smv_get_pinyin(smv_symbol_t sym);
const char* smv_get_hanzi(smv_symbol_t sm);

// 计算声韵调的层级关系 (最大公约数/最小公倍数)
uint32_t smv_lcm(uint32_t a, uint32_t b);  // 最小公倍数 (可折叠层级)
uint32_t smv_gcd(uint32_t a, uint32_t b);  // 最大公约数 (层级最小单位)

// 音质角度 (384°圆周上的相位)
float smv_symbol_angle(smv_symbol_t sym);

// 音质距离: 两个符号在声韵调空间中的差异度
float smv_voice_distance(smv_symbol_t a, smv_symbol_t b);

// 异质互换: 判断两个符号是否足够不同 (可区分)
bool smv_is_distinct(smv_symbol_t a, smv_symbol_t b);

// 正交检查: 90°=最大容量 (四边形及格线)
bool smv_is_orthogonal(smv_symbol_t a, smv_symbol_t b);

#endif // HUNYUAN_SMRLL_VOICE_H
