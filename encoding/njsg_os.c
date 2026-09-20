/**
 * ═══════════════════════════════════════════════════════════════════════════
 * njsg_os.c — 满全法颜色进制操作系统 (单文件完整版)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   一个文件 = 编码 + 审判 + 衍射 + 编织 + 像素运算 + 部件拆解 + 天庭
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编码空间: 0x00(空) | 0x01(我) | 0x02..0xFE(254字符) | 0xFF(溢)
 * 每字符: 8比特 = 1字节 (信息论最小值, 不可再少)
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o njsg njsg_os.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 核心常量
 * ═══════════════════════════════════════════════════════════════════════════ */

#define CODE_EMPTY    0x00    /* 刘楚恬 · 空容器 */
#define CODE_SELF     0x01    /* 我 · 锚定容器 */
#define CODE_OVERFLOW 0xFF    /* 溢出 · 终止符 */

#define BITS_PER_CHAR 8
#define CHAR_CAPACITY 254

/* ═══════════════════════════════════════════════════════════════════════════
 * 类型定义
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7, T_UNKNOWN=0xFF } sym_type_t;
typedef enum { L1=0x10, L2=0x20, L3=0x30, L_UNKNOWN=0xFF } sym_level_t;
typedef enum { PASS=0, REJ_LEVEL=1, REJ_MODIFIER=2, REJ_TONE=3, REJ_EXTRA=4, REJ_UNKNOWN=5, REJ_DIFFERENT=6 } verdict_t;

static const char* vstr(verdict_t v) {
    switch(v) {
        case PASS: return "PASS — 许可配对";
        case REJ_LEVEL: return "REJECT — 跨层级";
        case REJ_MODIFIER: return "REJECT — 修饰零件";
        case REJ_TONE: return "REJECT — 跨基调";
        case REJ_EXTRA: return "REJECT — 溢出符号";
        case REJ_UNKNOWN: return "REJECT — 未分类";
        case REJ_DIFFERENT: return "REJECT — 类型不同";
    }
    return "?";
}

static const char* tstr(sym_type_t t) {
    switch(t) {
        case T_BASE: return "基韵锚";
        case T_TONE: return "声调韵";
        case T_COMPOUND: return "复韵母";
        case T_SYMBOL: return "配对符号";
        case T_EXTRA: return "溢出";
        default: return "?";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编码表: 254字符
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    const char *name;
    uint8_t     code;       /* 字节值 0x02..0xFE */
    sym_type_t  type;       /* 类型 */
    uint8_t     tier;       /* 容器层级: 1/2/3/4 */
    uint8_t     anchor;     /* 锚: 0~5=基韵, 6=声调, 7=复韵/符号, 8=溢出 */
} cell_t;

static const cell_t TABLE[] = {
    /* ═══ 1-bit 本体层: 6基韵锚 ═══ */
    {"a",0x02,T_BASE,1,0}, {"o",0x03,T_BASE,1,1}, {"e",0x04,T_BASE,1,2},
    {"i",0x05,T_BASE,1,3}, {"u",0x06,T_BASE,1,4}, {"ü",0x07,T_BASE,1,5},

    /* ═══ 2-bit 融合层: 28复韵母名 ═══ */
    {"ai",0x10,T_COMPOUND,2,7}, {"ei",0x11,T_COMPOUND,2,7}, {"ui",0x12,T_COMPOUND,2,7},
    {"ao",0x13,T_COMPOUND,2,7}, {"ou",0x14,T_COMPOUND,2,7}, {"iu",0x15,T_COMPOUND,2,7},
    {"ie",0x16,T_COMPOUND,2,7}, {"üe",0x17,T_COMPOUND,2,7}, {"er",0x18,T_COMPOUND,2,7},
    {"an",0x19,T_COMPOUND,2,7}, {"en",0x1A,T_COMPOUND,2,7}, {"in",0x1B,T_COMPOUND,2,7},
    {"un",0x1C,T_COMPOUND,2,7}, {"ün",0x1D,T_COMPOUND,2,7}, {"ang",0x1E,T_COMPOUND,2,7},
    {"eng",0x1F,T_COMPOUND,2,7}, {"ing",0x20,T_COMPOUND,2,7}, {"ong",0x21,T_COMPOUND,2,7},
    {"ia",0x22,T_COMPOUND,2,7}, {"ian",0x23,T_COMPOUND,2,7}, {"iang",0x24,T_COMPOUND,2,7},
    {"iong",0x25,T_COMPOUND,2,7}, {"ua",0x26,T_COMPOUND,2,7}, {"uo",0x27,T_COMPOUND,2,7},
    {"uai",0x28,T_COMPOUND,2,7}, {"uan",0x29,T_COMPOUND,2,7}, {"uang",0x2A,T_COMPOUND,2,7},
    {"üan",0x2B,T_COMPOUND,2,7},

    /* ═══ 2-bit 融合层: 66配对符号 ═══ */
    {"ꬰ",0x30,T_SYMBOL,2,7}, {"ꬱ",0x31,T_SYMBOL,2,7}, {"ə",0x32,T_SYMBOL,2,7}, {"Ə",0x33,T_SYMBOL,2,7},
    {"ѱ",0x34,T_SYMBOL,2,7}, {"Ѱ",0x35,T_SYMBOL,2,7}, {"ꜵ",0x36,T_SYMBOL,2,7}, {"Ꜵ",0x37,T_SYMBOL,2,7},
    {"ȣ",0x38,T_SYMBOL,2,7}, {"Ȣ",0x39,T_SYMBOL,2,7}, {"ꭎ",0x3A,T_SYMBOL,2,7}, {"Ⴞ",0x3B,T_SYMBOL,2,7},
    {"ɤ",0x3C,T_SYMBOL,2,7}, {"Ɣ",0x3D,T_SYMBOL,2,7}, {"ᵫ",0x3E,T_SYMBOL,2,7}, {"ꭂ",0x3F,T_SYMBOL,2,7},
    {"ɛ",0x40,T_SYMBOL,2,7}, {"Ɛ",0x41,T_SYMBOL,2,7}, {"ꙙ",0x42,T_SYMBOL,2,7}, {"Ꙙ",0x43,T_SYMBOL,2,7},
    {"ⱄ",0x44,T_SYMBOL,2,7}, {"ⱅ",0x45,T_SYMBOL,2,7}, {"ĩ",0x46,T_SYMBOL,2,7}, {"Ĩ",0x47,T_SYMBOL,2,7},
    {"տ",0x48,T_SYMBOL,2,7}, {"໙",0x49,T_SYMBOL,2,7}, {"Ⴐ",0x4A,T_SYMBOL,2,7}, {"Ⴆ",0x4B,T_SYMBOL,2,7},
    {"Ⴊ",0x4C,T_SYMBOL,2,7}, {"ӫ",0x4D,T_SYMBOL,2,7}, {"Ӫ",0x4E,T_SYMBOL,2,7}, {"ꙟ",0x4F,T_SYMBOL,2,7},
    {"Ꙟ",0x50,T_SYMBOL,2,7}, {"ꭁ",0x51,T_SYMBOL,2,7}, {"ꭀ",0x52,T_SYMBOL,2,7}, {"ꙗ",0x53,T_SYMBOL,2,7},
    {"Ꙗ",0x54,T_SYMBOL,2,7}, {"ꙝ",0x55,T_SYMBOL,2,7}, {"Ꙝ",0x56,T_SYMBOL,2,7}, {"ѭ",0x57,T_SYMBOL,2,7},
    {"Ѭ",0x58,T_SYMBOL,2,7}, {"ȱ",0x59,T_SYMBOL,2,7}, {"Ȱ",0x5A,T_SYMBOL,2,7}, {"ǟ",0x5B,T_SYMBOL,2,7},
    {"Ǟ",0x5C,T_SYMBOL,2,7}, {"ȫ",0x5D,T_SYMBOL,2,7}, {"Ȫ",0x5E,T_SYMBOL,2,7}, {"ꜻ",0x5F,T_SYMBOL,2,7},
    {"Ꜻ",0x60,T_SYMBOL,2,7}, {"Ⴄ",0x61,T_SYMBOL,2,7}, {"Ⴂ",0x62,T_SYMBOL,2,7}, {"ꙛ",0x63,T_SYMBOL,2,7},
    {"Ꙛ",0x64,T_SYMBOL,2,7}, {"ѫ",0x65,T_SYMBOL,2,7}, {"Ѫ",0x66,T_SYMBOL,2,7},
    {"ꬲ",0x67,T_SYMBOL,2,7}, {"ꬳ",0x68,T_SYMBOL,2,7}, {"ꬴ",0x69,T_SYMBOL,2,7}, {"ꬵ",0x6A,T_SYMBOL,2,7},
    {"ꬶ",0x6B,T_SYMBOL,2,7}, {"ꬷ",0x6C,T_SYMBOL,2,7}, {"ꬸ",0x6D,T_SYMBOL,2,7}, {"ꬹ",0x6E,T_SYMBOL,2,7},
    {"ꬺ",0x6F,T_SYMBOL,2,7}, {"ꬻ",0x70,T_SYMBOL,2,7}, {"ꬼ",0x71,T_SYMBOL,2,7}, {"ꬽ",0x72,T_SYMBOL,2,7},
    {"ꬾ",0x73,T_SYMBOL,2,7}, {"ꬿ",0x74,T_SYMBOL,2,7}, {"ꭑ",0x75,T_SYMBOL,2,7},

    /* ═══ 3-bit 时间层: 24声调韵 ═══ */
    {"ā",0x80,T_TONE,3,6}, {"á",0x81,T_TONE,3,6}, {"ǎ",0x82,T_TONE,3,6}, {"à",0x83,T_TONE,3,6},
    {"ō",0x84,T_TONE,3,6}, {"ó",0x85,T_TONE,3,6}, {"ǒ",0x86,T_TONE,3,6}, {"ò",0x87,T_TONE,3,6},
    {"ē",0x88,T_TONE,3,6}, {"é",0x89,T_TONE,3,6}, {"ě",0x8A,T_TONE,3,6}, {"è",0x8B,T_TONE,3,6},
    {"ī",0x8C,T_TONE,3,6}, {"í",0x8D,T_TONE,3,6}, {"ǐ",0x8E,T_TONE,3,6}, {"ì",0x8F,T_TONE,3,6},
    {"ū",0x90,T_TONE,3,6}, {"ú",0x91,T_TONE,3,6}, {"ǔ",0x92,T_TONE,3,6}, {"ù",0x93,T_TONE,3,6},
    {"ǖ",0x94,T_TONE,3,6}, {"ǘ",0x95,T_TONE,3,6}, {"ǚ",0x96,T_TONE,3,6}, {"ǜ",0x97,T_TONE,3,6},

    /* ═══ 4+bit 外部层: 70溢出符号 ═══ */
    {"Ӱ",0xA0,T_EXTRA,4,8}, {"ӱ",0xA1,T_EXTRA,4,8}, {"Ѧ",0xA2,T_EXTRA,4,8}, {"ѧ",0xA3,T_EXTRA,4,8},
    {"Ѽ",0xA4,T_EXTRA,4,8}, {"ѽ",0xA5,T_EXTRA,4,8}, {"Ѥ",0xA6,T_EXTRA,4,8}, {"ѥ",0xA7,T_EXTRA,4,8},
    {"Ё",0xA8,T_EXTRA,4,8}, {"ё",0xA9,T_EXTRA,4,8}, {"Є",0xAA,T_EXTRA,4,8}, {"є",0xAB,T_EXTRA,4,8},
    {"Ʝ",0xAC,T_EXTRA,4,8}, {"႘",0xAD,T_EXTRA,4,8}, {"Ꙍ",0xAE,T_EXTRA,4,8}, {"ꙍ",0xAF,T_EXTRA,4,8},
    {"ꭃ",0xB0,T_EXTRA,4,8}, {"ꭄ",0xB1,T_EXTRA,4,8}, {"Ⴘ",0xB2,T_EXTRA,4,8}, {"Ⴏ",0xB3,T_EXTRA,4,8},
    {"Ⴎ",0xB4,T_EXTRA,4,8}, {"Ⴌ",0xB5,T_EXTRA,4,8}, {"Ⴋ",0xB6,T_EXTRA,4,8}, {"☁",0xB7,T_EXTRA,4,8},
    {"Ӭ",0xB8,T_EXTRA,4,8}, {"ӭ",0xB9,T_EXTRA,4,8}, {"Ѿ",0xBA,T_EXTRA,4,8}, {"ѿ",0xBB,T_EXTRA,4,8},
    {"Ȥ",0xBC,T_EXTRA,4,8}, {"Ç",0xBD,T_EXTRA,4,8}, {"Ŋ",0xBE,T_EXTRA,4,8}, {"ȥ",0xBF,T_EXTRA,4,8},
    {"ç",0xC0,T_EXTRA,4,8}, {"Ӗ",0xC1,T_EXTRA,4,8}, {"Ḁ",0xC2,T_EXTRA,4,8}, {"ḁ",0xC3,T_EXTRA,4,8},
    {"Ḃ",0xC4,T_EXTRA,4,8}, {"ḃ",0xC5,T_EXTRA,4,8}, {"Ḅ",0xC6,T_EXTRA,4,8}, {"ḅ",0xC7,T_EXTRA,4,8},
    {"Ḇ",0xC8,T_EXTRA,4,8}, {"ḇ",0xC9,T_EXTRA,4,8}, {"Ḉ",0xCA,T_EXTRA,4,8}, {"ḉ",0xCB,T_EXTRA,4,8},
    {"Ḋ",0xCC,T_EXTRA,4,8}, {"ḋ",0xCD,T_EXTRA,4,8}, {"Ḍ",0xCE,T_EXTRA,4,8}, {"ḍ",0xCF,T_EXTRA,4,8},
    {"Ḏ",0xD0,T_EXTRA,4,8}, {"ḏ",0xD1,T_EXTRA,4,8}, {"Ḑ",0xD2,T_EXTRA,4,8}, {"ḑ",0xD3,T_EXTRA,4,8},
    {"Ḓ",0xD4,T_EXTRA,4,8}, {"ḓ",0xD5,T_EXTRA,4,8}, {"Ḕ",0xD6,T_EXTRA,4,8}, {"ḕ",0xD7,T_EXTRA,4,8},
    {"Ḗ",0xD8,T_EXTRA,4,8}, {"ḗ",0xD9,T_EXTRA,4,8}, {"Ḙ",0xDA,T_EXTRA,4,8}, {"ḙ",0xDB,T_EXTRA,4,8},
    {"Ḛ",0xDC,T_EXTRA,4,8}, {"ḛ",0xDD,T_EXTRA,4,8}, {"Ḝ",0xDE,T_EXTRA,4,8}, {"ḝ",0xDF,T_EXTRA,4,8},
    {"Ḟ",0xE0,T_EXTRA,4,8}, {"ḟ",0xE1,T_EXTRA,4,8}, {"Ḡ",0xE2,T_EXTRA,4,8}, {"ḡ",0xE3,T_EXTRA,4,8},
    {"Ḣ",0xE4,T_EXTRA,4,8}, {"ḣ",0xE5,T_EXTRA,4,8},
};
#define TABLE_SZ ((int)(sizeof(TABLE)/sizeof(TABLE[0])))

/* ═══════════════════════════════════════════════════════════════════════════
 * 部件分解表: 把符号拆解为部件
 * (这是"颜色像素加减乘除"的基础)
 *
 * 每个符号由若干部件组成:
 *   - 基元 (base): 6基韵锚 a/o/e/i/u/ü
 *   - 声调标记 (tone): ˉ ˊ ˇ ˋ ˙
 *   - 鼻音标记 (nasal): n/ng
 *   - 符号修饰 (modifier): 各种视觉变体
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    const char *name;       /* 部件名 */
    uint8_t     code;       /* 部件编码 (低4位=部件类型, 高4位=变体) */
} part_t;

/* 基元部件 (5bits足够, 但统一用8bits) */
enum {
    PART_a = 0x01, PART_o = 0x02, PART_e = 0x03,
    PART_i = 0x04, PART_u = 0x05, PART_v = 0x06,  /* v = ü */
    PART_TONE_1 = 0x10, PART_TONE_2 = 0x11, PART_TONE_3 = 0x12,
    PART_TONE_4 = 0x13, PART_TONE_0 = 0x14,
    PART_N = 0x20, PART_NG = 0x21,
};

/* 部件签名表: 每个符号对应的部件序列 */
typedef struct {
    const char *name;       /* 符号名 */
    const uint8_t *parts;   /* 部件序列 */
    int          nparts;    /* 部件数 */
} sig_t;

static const uint8_t SIG_a[]  = {PART_a};
static const uint8_t SIG_o[]  = {PART_o};
static const uint8_t SIG_e[]  = {PART_e};
static const uint8_t SIG_i[]  = {PART_i};
static const uint8_t SIG_u[]  = {PART_u};
static const uint8_t SIG_v[]  = {PART_v};
static const uint8_t SIG_ai[] = {PART_a, PART_i};
static const uint8_t SIG_ei[] = {PART_e, PART_i};
static const uint8_t SIG_ao[] = {PART_a, PART_o};
static const uint8_t SIG_ou[] = {PART_o, PART_u};
static const uint8_t SIG_an[] = {PART_a, PART_N};
static const uint8_t SIG_en[] = {PART_e, PART_N};
static const uint8_t SIG_ang[] = {PART_a, PART_NG};
static const uint8_t SIG_eng[] = {PART_e, PART_NG};
static const uint8_t SIG_er[] = {PART_e, 0x30};

static const uint8_t SIG_a1[] = {PART_a, PART_TONE_1};
static const uint8_t SIG_a2[] = {PART_a, PART_TONE_2};
static const uint8_t SIG_a3[] = {PART_a, PART_TONE_3};
static const uint8_t SIG_a4[] = {PART_a, PART_TONE_4};
static const uint8_t SIG_e1[] = {PART_e, PART_TONE_1};
static const uint8_t SIG_e2[] = {PART_e, PART_TONE_2};
static const uint8_t SIG_e3[] = {PART_e, PART_TONE_3};
static const uint8_t SIG_e4[] = {PART_e, PART_TONE_4};
static const uint8_t SIG_o1[] = {PART_o, PART_TONE_1};
static const uint8_t SIG_o2[] = {PART_o, PART_TONE_2};
static const uint8_t SIG_i1[] = {PART_i, PART_TONE_1};
static const uint8_t SIG_i2[] = {PART_i, PART_TONE_2};
static const uint8_t SIG_u1[] = {PART_u, PART_TONE_1};
static const uint8_t SIG_u2[] = {PART_u, PART_TONE_2};

static const sig_t SIGS[] = {
    {"a", SIG_a, 1}, {"o", SIG_o, 1}, {"e", SIG_e, 1},
    {"i", SIG_i, 1}, {"u", SIG_u, 1}, {"ü", SIG_v, 1},
    {"ai", SIG_ai, 2}, {"ei", SIG_ei, 2}, {"ao", SIG_ao, 2}, {"ou", SIG_ou, 2},
    {"an", SIG_an, 2}, {"en", SIG_en, 2}, {"ang", SIG_ang, 2}, {"eng", SIG_eng, 2},
    {"er", SIG_er, 2},
    {"ā", SIG_a1, 2}, {"á", SIG_a2, 2}, {"ǎ", SIG_a3, 2}, {"à", SIG_a4, 2},
    {"ē", SIG_e1, 2}, {"é", SIG_e2, 2}, {"ě", SIG_e3, 2}, {"è", SIG_e4, 2},
    {"ō", SIG_o1, 2}, {"ó", SIG_o2, 2},
    {"ī", SIG_i1, 2}, {"í", SIG_i2, 2},
    {"ū", SIG_u1, 2}, {"ú", SIG_u2, 2},
};
#define SIGS_SZ ((int)(sizeof(SIGS)/sizeof(SIGS[0])))

/* ═══════════════════════════════════════════════════════════════════════════
 * 注册表 (运行时查询)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_R 256
typedef struct { uint32_t magic, count; struct { const char *name; sym_type_t type; sym_level_t level; uint8_t anchor; } e[MAX_R]; } reg_t;

static void reg_init(reg_t *r) { memset(r, 0, sizeof(*r)); r->magic = 0x54594453; }

static void reg_load(reg_t *r) {
    if (!r || r->magic != 0x54594453) return;
    for (int i = 0; i < TABLE_SZ; i++) {
        sym_level_t lvl = (TABLE[i].type == T_TONE) ? L3 : (TABLE[i].type == T_MODIFIER) ? L2 : L1;
        r->e[r->count].name = TABLE[i].name;
        r->e[r->count].type = TABLE[i].type;
        r->e[r->count].level = lvl;
        r->e[r->count].anchor = TABLE[i].anchor;
        r->count++;
    }
}

static sym_type_t reg_type(reg_t *r, const char *n) {
    if (!r || !n) return T_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].type;
    return T_UNKNOWN;
}
static sym_level_t reg_level(reg_t *r, const char *n) {
    if (!r || !n) return L_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].level;
    return L_UNKNOWN;
}
static uint8_t reg_anchor(reg_t *r, const char *n) {
    if (!r || !n) return 0xFF;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].anchor;
    return 0xFF;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 审判引擎
 * ═══════════════════════════════════════════════════════════════════════════ */

static verdict_t judge(reg_t *r, const char *a, const char *b) {
    if (!a || !b) return REJ_UNKNOWN;
    sym_type_t ta = reg_type(r, a), tb = reg_type(r, b);
    sym_level_t la = reg_level(r, a), lb = reg_level(r, b);
    if (la == L_UNKNOWN || lb == L_UNKNOWN) return REJ_UNKNOWN;
    if (la == L2 || lb == L2) return REJ_MODIFIER;
    if (la != lb) return REJ_LEVEL;
    if (ta == T_UNKNOWN || tb == T_UNKNOWN) return REJ_UNKNOWN;
    if (ta == T_EXTRA || tb == T_EXTRA) return REJ_EXTRA;
    if (ta != tb) return REJ_DIFFERENT;
    uint8_t aa = reg_anchor(r, a), ab = reg_anchor(r, b);
    if (ta == T_BASE && aa != ab) return REJ_DIFFERENT;
    if (ta == T_TONE) return REJ_TONE;
    if (ta == T_COMPOUND || ta == T_SYMBOL) return REJ_DIFFERENT;
    return PASS;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编码查询
 * ═══════════════════════════════════════════════════════════════════════════ */

static int n2c(const char *name) {
    if (!name) return -1;
    for (int i = 0; i < TABLE_SZ; i++) if (strcmp(TABLE[i].name, name) == 0) return TABLE[i].code;
    return -1;
}
static const char* c2n(uint8_t code) {
    if (code == CODE_EMPTY) return "[空/刘楚恬]";
    if (code == CODE_SELF)  return "[我]";
    if (code == CODE_OVERFLOW) return "[溢出]";
    for (int i = 0; i < TABLE_SZ; i++) if (TABLE[i].code == code) return TABLE[i].name;
    return "[-]";
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 部件分解查询
 * ═══════════════════════════════════════════════════════════════════════════ */

static const sig_t* find_sig(const char *name) {
    for (int i = 0; i < SIGS_SZ; i++) if (strcmp(SIGS[i].name, name) == 0) return &SIGS[i];
    return NULL;
}

static const char* part_name(uint8_t p) {
    switch (p) {
        case PART_a: return "a";
        case PART_o: return "o";
        case PART_e: return "e";
        case PART_i: return "i";
        case PART_u: return "u";
        case PART_v: return "ü";
        case PART_TONE_1: return "ˉ(阴平)";
        case PART_TONE_2: return "ˊ(阳平)";
        case PART_TONE_3: return "ˇ(上声)";
        case PART_TONE_4: return "ˋ(去声)";
        case PART_TONE_0: return "˙(轻声)";
        case PART_N: return "n";
        case PART_NG: return "ng";
        case 0x30: return "r(儿化)";
    }
    return "?";
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 像素相位运算 (只允许加减, 乘除禁用)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { PIX_ADD = 0, PIX_SUB = 1, PIX_MUL = 2, PIX_DIV = 3 } pix_op_t;

typedef struct {
    pix_op_t op;
    int      success;
    float    result_weight;
} pix_result_t;

static float vw(const char *s) {
    if (!s) return 0.5f;
    int l = (int)strlen(s);
    if (l <= 1) return 0.3f;
    if (l == 2) return 0.6f;
    return 0.9f;
}

static pix_result_t pix_arith(pix_op_t op, const char *src, const char *tgt) {
    pix_result_t r;
    memset(&r, 0, sizeof(r));
    r.op = op;
    switch (op) {
        case PIX_ADD:
            if (!src || !tgt) { r.success = 0; r.result_weight = 0; return r; }
            r.success = 1;
            r.result_weight = vw(src) + vw(tgt) * 0.3f;
            return r;
        case PIX_SUB:
            if (!src || !tgt) { r.success = 0; r.result_weight = 0; return r; }
            r.success = 1;
            r.result_weight = vw(src) - vw(tgt) * 0.3f;
            if (r.result_weight < 0.1f) r.result_weight = 0.1f;
            return r;
        case PIX_MUL:
            r.success = 0; r.result_weight = 0;
            return r;
        case PIX_DIV:
            r.success = 0; r.result_weight = 0;
            return r;
    }
    return r;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编织平衡
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_N 64
#define MAX_E 128
typedef struct { const char *name; float w, po, pn, nf; uint8_t bal; } node_t;
typedef struct { uint32_t from, to; float tens; uint8_t act; } edge_t;
typedef struct { uint32_t magic, nc, ec; node_t nodes[MAX_N]; edge_t edges[MAX_E]; float bal_score; uint8_t is_bal; uint32_t stress_n; } graph_t;

static void ginit(graph_t *g) { memset(g, 0, sizeof(*g)); g->magic = 0x57454156; }
static int gfind(graph_t *g, const char *n) {
    if (!g || !n) return -1;
    for (uint32_t i = 0; i < g->nc; i++) if (strcmp(g->nodes[i].name, n) == 0) return (int)i;
    return -1;
}
static int gadd_n(graph_t *g, const char *n, float w) {
    if (!g || g->magic != 0x57454156 || !n || g->nc >= MAX_N) return -1;
    int ex = gfind(g, n); if (ex >= 0) return ex;
    uint32_t i = g->nc++; g->nodes[i].name = n; g->nodes[i].w = w; return (int)i;
}
static int gadd_e(graph_t *g, const char *f, const char *t, float tens) {
    if (!g || g->magic != 0x57454156 || g->ec >= MAX_E) return -1;
    int fi = gfind(g, f), ti = gfind(g, t); if (fi < 0 || ti < 0) return -1;
    uint32_t i = g->ec++; g->edges[i].from = fi; g->edges[i].to = ti; g->edges[i].tens = tens; g->edges[i].act = 1;
    g->nodes[fi].po += tens; g->nodes[ti].pn += tens; return 0;
}
static void gcalc(graph_t *g) {
    if (!g || g->magic != 0x57454156) return;
    double tot = 0; uint32_t sn = 0;
    for (uint32_t i = 0; i < g->nc; i++) {
        g->nodes[i].nf = g->nodes[i].po - g->nodes[i].pn;
        tot += fabs(g->nodes[i].nf);
        if (fabs(g->nodes[i].nf) < 0.1f) { g->nodes[i].bal = 1; }
        else { g->nodes[i].bal = 0; if (fabs(g->nodes[i].nf) > 0.5f) sn++; }
    }
    g->bal_score = g->ec > 0 ? 1.0f - (float)(tot / g->ec) : 1.0f;
    if (g->bal_score < 0) g->bal_score = 0;
    g->is_bal = (g->bal_score >= 0.7f && sn == 0) ? 1 : 0;
    g->stress_n = sn;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 报告输出
 * ═══════════════════════════════════════════════════════════════════════════ */

static void print_report(reg_t *r, graph_t *g, const char *a, const char *b) {
    verdict_t v = judge(r, a, b);
    int ca = n2c(a), cb = n2c(b);
    if (g) { gadd_n(g, a, vw(a)); gadd_n(g, b, vw(b)); float vb = vw(a) - vw(b); if (vb < 0) vb = -vb; gadd_e(g, a, b, vb); gcalc(g); }

    printf("\n╔══════════════════════════════════════════════════════════════╗\n");
    printf("║           满 全 法 · 完 整 审 判 报 告                       ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ %-12s ↔ %-12s                              ║\n", a, b);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [编码]                                                       ║\n");
    if (ca >= 0) { printf("║   %-6s → 0x%02X (", a, ca); for (int i = 7; i >= 0; i--) printf("%d", (ca >> i) & 1); printf(") 8比特\n"); }
    else printf("║   %-6s → [未找到]\n", a);
    if (cb >= 0) { printf("║   %-6s → 0x%02X (", b, cb); for (int i = 7; i >= 0; i--) printf("%d", (cb >> i) & 1); printf(") 8比特\n"); }
    else printf("║   %-6s → [未找到]\n", b);

    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [类型] A=%-8s L%d 锚=%d | B=%-8s L%d 锚=%d   ║\n",
           tstr(reg_type(r,a)), reg_level(r,a)>>4, reg_anchor(r,a),
           tstr(reg_type(r,b)), reg_level(r,b)>>4, reg_anchor(r,b));

    /* 部件分解 */
    const sig_t *sa = find_sig(a), *sb = find_sig(b);
    if (sa || sb) {
        printf("╠══════════════════════════════════════════════════════════════╣\n");
        printf("║ [部件分解]                                                   ║\n");
        if (sa) { printf("║   %s = ", a); for (int i = 0; i < sa->nparts; i++) printf("%s%s", i?"+":"", part_name(sa->parts[i])); printf("\n"); }
        if (sb) { printf("║   %s = ", b); for (int i = 0; i < sb->nparts; i++) printf("%s%s", i?"+":"", part_name(sb->parts[i])); printf("\n"); }
    }

    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [编织] 平衡=%.2f %s                                          ║\n",
           g ? g->bal_score : 0, g && g->is_bal ? "✓" : "✗");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [审判]                                                       ║\n");
    if (v == PASS) printf("║   ★★★ 通过 ★★★                                          ║\n");
    else printf("║   ✗✗✗ 拒绝 — %s                              ║\n", vstr(v));
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 天庭积分 (奖励/惩罚历史)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_HISTORY 64
typedef struct {
    int  pass_count;
    int  reject_count;
    int  history[MAX_HISTORY];  /* 0=reject, 1=pass */
    int  hist_len;
    int  hist_pos;
} heavenly_judgment_t;

static void hj_init(heavenly_judgment_t *hj) { memset(hj, 0, sizeof(*hj)); }
static void hj_record(heavenly_judgment_t *hj, int pass) {
    if (!hj) return;
    if (pass) hj->pass_count++; else hj->reject_count++;
    hj->history[hj->hist_pos % MAX_HISTORY] = pass ? 1 : 0;
    hj->hist_pos++;
    if (hj->hist_len < MAX_HISTORY) hj->hist_len++;
}
static float hj_mercy(heavenly_judgment_t *hj) {
    if (!hj || hj->hist_len == 0) return 0.5f;
    return (float)hj->pass_count / (float)(hj->pass_count + hj->reject_count);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 自检
 * ═══════════════════════════════════════════════════════════════════════════ */

static int run_tests(void) {
    reg_t r; reg_init(&r); reg_load(&r);
    graph_t g; ginit(&g);
    heavenly_judgment_t hj; hj_init(&hj);
    int err = 0, tot = 0;

    printf("═══ 满全法OS · 完整自检 ═══\n\n");

    /* 审判 */
    printf("[审判]\n");
    struct { const char *a, *b; verdict_t ev; const char *d; } jt[] = {
        {"a","á",REJ_LEVEL,"a(L1)≠á(L3)"},
        {"e","ě",REJ_LEVEL,"e(L1)≠ě(L3)"},
        {"a","o",REJ_DIFFERENT,"a vs o 不同锚"},
        {"ā","á",REJ_TONE,"声调韵不可配对"},
        {"Ӱ","ӱ",REJ_EXTRA,"溢出符号"},
        {NULL,NULL,0,NULL}
    };
    for (int i = 0; jt[i].a; i++) {
        tot++; verdict_t got = judge(&r, jt[i].a, jt[i].b);
        hj_record(&hj, got == PASS);
        if (got == jt[i].ev) printf("  [PASS] %s\n", jt[i].d);
        else { printf("  [FAIL] %s got=%d exp=%d\n", jt[i].d, got, jt[i].ev); err++; }
    }

    /* 编码 */
    printf("\n[编码]\n");
    tot++; int c = n2c("a"); if (c == 0x02) printf("  [PASS] a→0x02\n"); else { printf("  [FAIL] a→%d\n", c); err++; }
    tot++; c = n2c("á"); if (c == 0x81) printf("  [PASS] á→0x81\n"); else { printf("  [FAIL] á→%d\n", c); err++; }
    tot++; c = n2c("ai"); if (c == 0x10) printf("  [PASS] ai→0x10\n"); else { printf("  [FAIL] ai→%d\n", c); err++; }
    tot++; c = n2c("ꬰ"); if (c == 0x30) printf("  [PASS] ꬰ→0x30\n"); else { printf("  [FAIL] ꬰ→%d\n", c); err++; }

    /* 反向查询 */
    tot++; const char *nm = c2n(0x02); if (strcmp(nm, "a") == 0) printf("  [PASS] 0x02→a\n"); else { printf("  [FAIL] 0x02→%s\n", nm); err++; }

    /* 表完整性 */
    printf("\n[表完整性]\n");
    int uniq = 1;
    for (int i = 0; i < TABLE_SZ; i++) for (int j = i + 1; j < TABLE_SZ; j++)
        if (TABLE[i].code == TABLE[j].code) { printf("  [FAIL] 冲突: %s=%s=0x%02X\n", TABLE[i].name, TABLE[j].name, TABLE[i].code); uniq = 0; err++; }
    if (uniq) printf("  [PASS] %d编码全部唯一\n", TABLE_SZ);

    /* 像素运算 */
    printf("\n[像素运算]\n");
    pix_result_t pr = pix_arith(PIX_ADD, "a", "i");
    tot++; if (pr.success) printf("  [PASS] a+i 加法成功 w=%.2f\n", pr.result_weight); else { printf("  [FAIL] a+i\n"); err++; }
    pr = pix_arith(PIX_MUL, "a", "á");
    tot++; if (!pr.success) printf("  [PASS] a×á 乘法禁用\n"); else { printf("  [FAIL] a×á\n"); err++; }
    pr = pix_arith(PIX_DIV, "a", "á");
    tot++; if (!pr.success) printf("  [PASS] a÷á 除法禁用\n"); else { printf("  [FAIL] a÷á\n"); err++; }

    /* 部件分解 */
    printf("\n[部件分解]\n");
    const sig_t *sig = find_sig("ai");
    tot++; if (sig && sig->nparts == 2 && sig->parts[0] == PART_a && sig->parts[1] == PART_i) printf("  [PASS] ai = a + i\n"); else { printf("  [FAIL] ai\n"); err++; }
    sig = find_sig("á");
    tot++; if (sig && sig->nparts == 2 && sig->parts[0] == PART_a && sig->parts[1] == PART_TONE_2) printf("  [PASS] á = a + ˊ\n"); else { printf("  [FAIL] á\n"); err++; }

    /* 天庭 */
    printf("\n[天庭] 通过=%d 拒绝=%d 宽恕度=%.2f\n", hj.pass_count, hj.reject_count, hj_mercy(&hj));

    printf("\n═══ 结果: %d/%d 通过, %d 失败 ═══\n", tot - err, tot, err);
    return err;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(reg_t *r) {
    char line[256], a[64], b[64];
    printf("\n═══ 满全法 · 交互模式 ═══\n");
    printf("  <A> <B>  审判+编码+报告\n");
    printf("  d <名>   部件分解\n");
    printf("  e <名>   查编码\n");
    printf("  p A B    像素运算\n");
    printf("  t        自检\n");
    printf("  q        退出\n");
    printf("─────────────────────────────────────────\n");
    graph_t g; ginit(&g);
    while (1) {
        printf("\n⮞ "); fflush(stdout);
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line); while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = 0;
        if (l == 0) continue;
        if (strcmp(line, "q") == 0) break;
        if (strcmp(line, "t") == 0) { run_tests(); continue; }
        if (line[0] == 'e' && line[1] == ' ') {
            const char *nm = line + 2; int cd = n2c(nm);
            if (cd >= 0) { printf("  %s → 0x%02X (", nm, cd); for (int i = 7; i >= 0; i--) printf("%d", (cd >> i) & 1); printf(") 8比特\n"); }
            else printf("  %s → [未找到]\n", nm);
            continue;
        }
        if (line[0] == 'd' && line[1] == ' ') {
            const char *nm = line + 2;
            const sig_t *s = find_sig(nm);
            if (s) { printf("  %s = ", nm); for (int i = 0; i < s->nparts; i++) printf("%s%s", i?" + ":"", part_name(s->parts[i])); printf("\n"); }
            else { int cd = n2c(nm); if (cd >= 0) printf("  %s → 0x%02X (原子符号,无部件表)\n", nm, cd); else printf("  %s → [未找到]\n", nm); }
            continue;
        }
        if (line[0] == 'p' && line[1] == ' ') {
            if (sscanf(line + 2, "%63s %63s", a, b) == 2) {
                printf("  [像素运算]\n");
                pix_result_t r;
                r = pix_arith(PIX_ADD, a, b); printf("    %s + %s: %s w=%.2f\n", a, b, r.success ? "✓" : "✗", r.result_weight);
                r = pix_arith(PIX_SUB, a, b); printf("    %s - %s: %s w=%.2f\n", a, b, r.success ? "✓" : "✗", r.result_weight);
                r = pix_arith(PIX_MUL, a, b); printf("    %s × %s: %s\n", a, b, r.success ? "✓" : "✗(禁用)");
                r = pix_arith(PIX_DIV, a, b); printf("    %s ÷ %s: %s\n", a, b, r.success ? "✓" : "✗(禁用)");
            }
            continue;
        }
        if (sscanf(line, "%63s %63s", a, b) == 2) {
            ginit(&g);
            print_report(r, &g, a, b);
        } else printf("  未知命令\n");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    reg_t r; reg_init(&r); reg_load(&r);
    graph_t g; ginit(&g);

    if (argc == 1) return run_tests();
    if (argc == 2 && (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0)) return run_tests();
    if (argc == 2 && (strcmp(argv[1], "-i") == 0 || strcmp(argv[1], "--interactive") == 0)) { interactive(&r); return 0; }
    if (argc >= 3) { print_report(&r, &g, argv[1], argv[2]); return judge(&r, argv[1], argv[2]) == PASS ? 0 : 1; }
    printf("用法: %s [A B] | -t | -i\n", argv[0]);
    return 0;
}
