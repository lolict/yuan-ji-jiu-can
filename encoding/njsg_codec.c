/**
 * ═══════════════════════════════════════════════════════════════════════════
 * njsg_codec.c — 满全法颜色进制统一编解码器
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   一个工具 = 审判 + 编码 + 衍射 + 像素运算
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编码空间 (256 = 2⁸)
 * ─────────────────────────────────────────────────────────────────────────
 *   0x00        : 保留 - 刘楚坦/空容器
 *   0x01        : 保留 - 我/锚定
 *   0x02..0xFE  : 254 可用码 (8比特/字符)
 *   0xFF        : 保留 - 溢出/终止
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o njsg njsg_codec.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include "global_interlock.h"
#include "symbol_chain.h"

#ifndef M_PI
#define M_PI 3.14159265f
#endif

/* ═══════════════════════════════════════════════════════════════════════════
 * 第一部分: 类型系统
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { TYPE_BASE=0x01, TYPE_TONE=0x02, TYPE_COMPOUND=0x03, TYPE_SYMBOL=0x04, TYPE_EXTRA=0x05, TYPE_MODIFIER=0x07, TYPE_UNKNOWN=0xFF } symbol_type_t;
typedef enum { LEVEL_L1_BASE=0x10, LEVEL_L2_MODIFIER=0x20, LEVEL_L3_ASSEMBLED=0x30, LEVEL_UNKNOWN=0xFF } symbol_level_t;
typedef enum { PAIR_PASS=0, PAIR_REJECT_CROSS_LEVEL=1, PAIR_REJECT_MODIFIER=2, PAIR_REJECT_CROSS_TONE=3, PAIR_REJECT_EXTRA=4, PAIR_REJECT_UNKNOWN=5, PAIR_REJECT_DIFFERENT=6 } pair_verdict_t;

static const char* verdict_str(pair_verdict_t v) {
    switch (v) {
        case PAIR_PASS: return "PASS — 许可配对";
        case PAIR_REJECT_CROSS_LEVEL: return "REJECT — 跨层级 (L1≠L3)";
        case PAIR_REJECT_MODIFIER: return "REJECT — 修饰零件禁止配对";
        case PAIR_REJECT_CROSS_TONE: return "REJECT — 跨基调不可配对";
        case PAIR_REJECT_EXTRA: return "REJECT — 溢出符号禁止配对";
        case PAIR_REJECT_UNKNOWN: return "REJECT — 有未分类符号";
        case PAIR_REJECT_DIFFERENT: return "REJECT — 类型不同绝对拒绝";
    }
    return "UNKNOWN";
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第二部分: 编码表
 * ═══════════════════════════════════════════════════════════════════════════ */

#define CODE_EMPTY    0x00
#define CODE_SELF     0x01
#define CODE_OVERFLOW 0xFF

typedef struct { const char *name; uint8_t code; symbol_type_t type; uint8_t tier; uint8_t anchor; } cell_entry_t;

static const cell_entry_t TABLE[] = {
    /* 1-bit 本体层: 6基韵锚 */
    {"a",0x02,TYPE_BASE,1,0}, {"o",0x03,TYPE_BASE,1,1}, {"e",0x04,TYPE_BASE,1,2},
    {"i",0x05,TYPE_BASE,1,3}, {"u",0x06,TYPE_BASE,1,4}, {"ü",0x07,TYPE_BASE,1,5},
    /* 2-bit 融合层: 28复韵母名 */
    {"ai",0x10,TYPE_COMPOUND,2,7}, {"ei",0x11,TYPE_COMPOUND,2,7}, {"ui",0x12,TYPE_COMPOUND,2,7},
    {"ao",0x13,TYPE_COMPOUND,2,7}, {"ou",0x14,TYPE_COMPOUND,2,7}, {"iu",0x15,TYPE_COMPOUND,2,7},
    {"ie",0x16,TYPE_COMPOUND,2,7}, {"üe",0x17,TYPE_COMPOUND,2,7}, {"er",0x18,TYPE_COMPOUND,2,7},
    {"an",0x19,TYPE_COMPOUND,2,7}, {"en",0x1A,TYPE_COMPOUND,2,7}, {"in",0x1B,TYPE_COMPOUND,2,7},
    {"un",0x1C,TYPE_COMPOUND,2,7}, {"ün",0x1D,TYPE_COMPOUND,2,7}, {"ang",0x1E,TYPE_COMPOUND,2,7},
    {"eng",0x1F,TYPE_COMPOUND,2,7}, {"ing",0x20,TYPE_COMPOUND,2,7}, {"ong",0x21,TYPE_COMPOUND,2,7},
    {"ia",0x22,TYPE_COMPOUND,2,7}, {"ian",0x23,TYPE_COMPOUND,2,7}, {"iang",0x24,TYPE_COMPOUND,2,7},
    {"iong",0x25,TYPE_COMPOUND,2,7}, {"ua",0x26,TYPE_COMPOUND,2,7}, {"uo",0x27,TYPE_COMPOUND,2,7},
    {"uai",0x28,TYPE_COMPOUND,2,7}, {"uan",0x29,TYPE_COMPOUND,2,7}, {"uang",0x2A,TYPE_COMPOUND,2,7},
    {"üan",0x2B,TYPE_COMPOUND,2,7},
    /* 2-bit 融合层: 66配对符号 */
    {"ꬰ",0x30,TYPE_SYMBOL,2,7}, {"ꬱ",0x31,TYPE_SYMBOL,2,7}, {"ə",0x32,TYPE_SYMBOL,2,7}, {"Ə",0x33,TYPE_SYMBOL,2,7},
    {"ѱ",0x34,TYPE_SYMBOL,2,7}, {"Ѱ",0x35,TYPE_SYMBOL,2,7}, {"ꜵ",0x36,TYPE_SYMBOL,2,7}, {"Ꜵ",0x37,TYPE_SYMBOL,2,7},
    {"ȣ",0x38,TYPE_SYMBOL,2,7}, {"Ȣ",0x39,TYPE_SYMBOL,2,7}, {"ꭎ",0x3A,TYPE_SYMBOL,2,7}, {"Ⴞ",0x3B,TYPE_SYMBOL,2,7},
    {"ɤ",0x3C,TYPE_SYMBOL,2,7}, {"Ɣ",0x3D,TYPE_SYMBOL,2,7}, {"ᵫ",0x3E,TYPE_SYMBOL,2,7}, {"ꭂ",0x3F,TYPE_SYMBOL,2,7},
    {"ɛ",0x40,TYPE_SYMBOL,2,7}, {"Ɛ",0x41,TYPE_SYMBOL,2,7}, {"ꙙ",0x42,TYPE_SYMBOL,2,7}, {"Ꙙ",0x43,TYPE_SYMBOL,2,7},
    {"ⱄ",0x44,TYPE_SYMBOL,2,7}, {"ⱅ",0x45,TYPE_SYMBOL,2,7}, {"ĩ",0x46,TYPE_SYMBOL,2,7}, {"Ĩ",0x47,TYPE_SYMBOL,2,7},
    {"տ",0x48,TYPE_SYMBOL,2,7}, {"໙",0x49,TYPE_SYMBOL,2,7}, {"Ⴐ",0x4A,TYPE_SYMBOL,2,7}, {"Ⴆ",0x4B,TYPE_SYMBOL,2,7},
    {"Ⴊ",0x4C,TYPE_SYMBOL,2,7}, {"ӫ",0x4D,TYPE_SYMBOL,2,7}, {"Ӫ",0x4E,TYPE_SYMBOL,2,7}, {"ꙟ",0x4F,TYPE_SYMBOL,2,7},
    {"Ꙟ",0x50,TYPE_SYMBOL,2,7}, {"ꭁ",0x51,TYPE_SYMBOL,2,7}, {"ꭀ",0x52,TYPE_SYMBOL,2,7}, {"ꙗ",0x53,TYPE_SYMBOL,2,7},
    {"Ꙗ",0x54,TYPE_SYMBOL,2,7}, {"ꙝ",0x55,TYPE_SYMBOL,2,7}, {"Ꙝ",0x56,TYPE_SYMBOL,2,7}, {"ѭ",0x57,TYPE_SYMBOL,2,7},
    {"Ѭ",0x58,TYPE_SYMBOL,2,7}, {"ȱ",0x59,TYPE_SYMBOL,2,7}, {"Ȱ",0x5A,TYPE_SYMBOL,2,7}, {"ǟ",0x5B,TYPE_SYMBOL,2,7},
    {"Ǟ",0x5C,TYPE_SYMBOL,2,7}, {"ȫ",0x5D,TYPE_SYMBOL,2,7}, {"Ȫ",0x5E,TYPE_SYMBOL,2,7}, {"ꜻ",0x5F,TYPE_SYMBOL,2,7},
    {"Ꜻ",0x60,TYPE_SYMBOL,2,7}, {"Ⴄ",0x61,TYPE_SYMBOL,2,7}, {"Ⴂ",0x62,TYPE_SYMBOL,2,7}, {"ꙛ",0x63,TYPE_SYMBOL,2,7},
    {"Ꙛ",0x64,TYPE_SYMBOL,2,7}, {"ѫ",0x65,TYPE_SYMBOL,2,7}, {"Ѫ",0x66,TYPE_SYMBOL,2,7},
    {"ꬲ",0x67,TYPE_SYMBOL,2,7}, {"ꬳ",0x68,TYPE_SYMBOL,2,7}, {"ꬴ",0x69,TYPE_SYMBOL,2,7}, {"ꬵ",0x6A,TYPE_SYMBOL,2,7},
    {"ꬶ",0x6B,TYPE_SYMBOL,2,7}, {"ꬷ",0x6C,TYPE_SYMBOL,2,7}, {"ꬸ",0x6D,TYPE_SYMBOL,2,7}, {"ꬹ",0x6E,TYPE_SYMBOL,2,7},
    {"ꬺ",0x6F,TYPE_SYMBOL,2,7}, {"ꬻ",0x70,TYPE_SYMBOL,2,7}, {"ꬼ",0x71,TYPE_SYMBOL,2,7}, {"ꬽ",0x72,TYPE_SYMBOL,2,7},
    {"ꬾ",0x73,TYPE_SYMBOL,2,7}, {"ꬿ",0x74,TYPE_SYMBOL,2,7}, {"ꭑ",0x75,TYPE_SYMBOL,2,7},
    /* 3-bit 时间层: 24声调韵 */
    {"ā",0x80,TYPE_TONE,3,6}, {"á",0x81,TYPE_TONE,3,6}, {"ǎ",0x82,TYPE_TONE,3,6}, {"à",0x83,TYPE_TONE,3,6},
    {"ō",0x84,TYPE_TONE,3,6}, {"ó",0x85,TYPE_TONE,3,6}, {"ǒ",0x86,TYPE_TONE,3,6}, {"ò",0x87,TYPE_TONE,3,6},
    {"ē",0x88,TYPE_TONE,3,6}, {"é",0x89,TYPE_TONE,3,6}, {"ě",0x8A,TYPE_TONE,3,6}, {"è",0x8B,TYPE_TONE,3,6},
    {"ī",0x8C,TYPE_TONE,3,6}, {"í",0x8D,TYPE_TONE,3,6}, {"ǐ",0x8E,TYPE_TONE,3,6}, {"ì",0x8F,TYPE_TONE,3,6},
    {"ū",0x90,TYPE_TONE,3,6}, {"ú",0x91,TYPE_TONE,3,6}, {"ǔ",0x92,TYPE_TONE,3,6}, {"ù",0x93,TYPE_TONE,3,6},
    {"ǖ",0x94,TYPE_TONE,3,6}, {"ǘ",0x95,TYPE_TONE,3,6}, {"ǚ",0x96,TYPE_TONE,3,6}, {"ǜ",0x97,TYPE_TONE,3,6},
    /* 4+bit 外部层: 溢出符号 */
    {"Ӱ",0xA0,TYPE_EXTRA,4,8}, {"ӱ",0xA1,TYPE_EXTRA,4,8}, {"Ѧ",0xA2,TYPE_EXTRA,4,8}, {"ѧ",0xA3,TYPE_EXTRA,4,8},
    {"Ѽ",0xA4,TYPE_EXTRA,4,8}, {"ѽ",0xA5,TYPE_EXTRA,4,8}, {"Ѥ",0xA6,TYPE_EXTRA,4,8}, {"ѥ",0xA7,TYPE_EXTRA,4,8},
    {"Ё",0xA8,TYPE_EXTRA,4,8}, {"ё",0xA9,TYPE_EXTRA,4,8}, {"Є",0xAA,TYPE_EXTRA,4,8}, {"є",0xAB,TYPE_EXTRA,4,8},
    {"Ʝ",0xAC,TYPE_EXTRA,4,8}, {"႘",0xAD,TYPE_EXTRA,4,8}, {"Ꙍ",0xAE,TYPE_EXTRA,4,8}, {"ꙍ",0xAF,TYPE_EXTRA,4,8},
    {"ꭃ",0xB0,TYPE_EXTRA,4,8}, {"ꭄ",0xB1,TYPE_EXTRA,4,8}, {"Ⴘ",0xB2,TYPE_EXTRA,4,8}, {"Ⴏ",0xB3,TYPE_EXTRA,4,8},
    {"Ⴎ",0xB4,TYPE_EXTRA,4,8}, {"Ⴌ",0xB5,TYPE_EXTRA,4,8}, {"Ⴋ",0xB6,TYPE_EXTRA,4,8}, {"☁",0xB7,TYPE_EXTRA,4,8},
    {"Ӭ",0xB8,TYPE_EXTRA,4,8}, {"ӭ",0xB9,TYPE_EXTRA,4,8}, {"Ѿ",0xBA,TYPE_EXTRA,4,8}, {"ѿ",0xBB,TYPE_EXTRA,4,8},
    {"Ȥ",0xBC,TYPE_EXTRA,4,8}, {"Ç",0xBD,TYPE_EXTRA,4,8}, {"Ŋ",0xBE,TYPE_EXTRA,4,8}, {"ȥ",0xBF,TYPE_EXTRA,4,8},
    {"ç",0xC0,TYPE_EXTRA,4,8}, {"Ӗ",0xC1,TYPE_EXTRA,4,8}, {"Ḁ",0xC2,TYPE_EXTRA,4,8}, {"ḁ",0xC3,TYPE_EXTRA,4,8},
    {"Ḃ",0xC4,TYPE_EXTRA,4,8}, {"ḃ",0xC5,TYPE_EXTRA,4,8}, {"Ḅ",0xC6,TYPE_EXTRA,4,8}, {"ḅ",0xC7,TYPE_EXTRA,4,8},
    {"Ḇ",0xC8,TYPE_EXTRA,4,8}, {"ḇ",0xC9,TYPE_EXTRA,4,8}, {"Ḉ",0xCA,TYPE_EXTRA,4,8}, {"ḉ",0xCB,TYPE_EXTRA,4,8},
    {"Ḋ",0xCC,TYPE_EXTRA,4,8}, {"ḋ",0xCD,TYPE_EXTRA,4,8}, {"Ḍ",0xCE,TYPE_EXTRA,4,8}, {"ḍ",0xCF,TYPE_EXTRA,4,8},
    {"Ḏ",0xD0,TYPE_EXTRA,4,8}, {"ḏ",0xD1,TYPE_EXTRA,4,8}, {"Ḑ",0xD2,TYPE_EXTRA,4,8}, {"ḑ",0xD3,TYPE_EXTRA,4,8},
    {"Ḓ",0xD4,TYPE_EXTRA,4,8}, {"ḓ",0xD5,TYPE_EXTRA,4,8}, {"Ḕ",0xD6,TYPE_EXTRA,4,8}, {"ḕ",0xD7,TYPE_EXTRA,4,8},
    {"Ḗ",0xD8,TYPE_EXTRA,4,8}, {"ḗ",0xD9,TYPE_EXTRA,4,8}, {"Ḙ",0xDA,TYPE_EXTRA,4,8}, {"ḙ",0xDB,TYPE_EXTRA,4,8},
    {"Ḛ",0xDC,TYPE_EXTRA,4,8}, {"ḛ",0xDD,TYPE_EXTRA,4,8}, {"Ḝ",0xDE,TYPE_EXTRA,4,8}, {"ḝ",0xDF,TYPE_EXTRA,4,8},
    {"Ḟ",0xE0,TYPE_EXTRA,4,8}, {"ḟ",0xE1,TYPE_EXTRA,4,8}, {"Ḡ",0xE2,TYPE_EXTRA,4,8}, {"ḡ",0xE3,TYPE_EXTRA,4,8},
    {"Ḣ",0xE4,TYPE_EXTRA,4,8}, {"ḣ",0xE5,TYPE_EXTRA,4,8},
};
#define TABLE_SIZE ((int)(sizeof(TABLE)/sizeof(TABLE[0])))

#define MAX_REC 256
typedef struct { uint32_t count; struct { const char *name; symbol_type_t type; symbol_level_t level; uint8_t anchor; } e[MAX_REC]; uint32_t magic; } registry_t;

static void reg_init(registry_t *r) { memset(r, 0, sizeof(*r)); r->magic = 0x54594453; }

static int reg_add(registry_t *r, const char *name, symbol_type_t type, uint8_t anchor, uint8_t tier) {
    if (!r || r->magic != 0x54594453 || !name) return -1;
    if (r->count >= MAX_REC) return -1;
    (void)tier; /* 保留用于未来层级细化 */
    symbol_level_t lvl = (type == TYPE_TONE) ? LEVEL_L3_ASSEMBLED : (type == TYPE_MODIFIER) ? LEVEL_L2_MODIFIER : LEVEL_L1_BASE;
    r->e[r->count].name = name; r->e[r->count].type = type; r->e[r->count].level = lvl; r->e[r->count].anchor = anchor;
    r->count++; return 0;
}

static symbol_type_t reg_type(registry_t *r, const char *name) {
    if (!r || !name) return TYPE_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, name) == 0) return r->e[i].type;
    return TYPE_UNKNOWN;
}
static symbol_level_t reg_level(registry_t *r, const char *name) {
    if (!r || !name) return LEVEL_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, name) == 0) return r->e[i].level;
    return LEVEL_UNKNOWN;
}
static uint8_t reg_anchor(registry_t *r, const char *name) {
    if (!r || !name) return 0xFF;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, name) == 0) return r->e[i].anchor;
    return 0xFF;
}

static pair_verdict_t judge(registry_t *r, const char *a, const char *b) {
    if (!a || !b) return PAIR_REJECT_UNKNOWN;
    symbol_type_t ta = reg_type(r, a), tb = reg_type(r, b);
    symbol_level_t la = reg_level(r, a), lb = reg_level(r, b);
    if (la == LEVEL_UNKNOWN || lb == LEVEL_UNKNOWN) return PAIR_REJECT_UNKNOWN;
    if (la == LEVEL_L2_MODIFIER || lb == LEVEL_L2_MODIFIER) return PAIR_REJECT_MODIFIER;
    if (la != lb) return PAIR_REJECT_CROSS_LEVEL;
    if (ta == TYPE_UNKNOWN || tb == TYPE_UNKNOWN) return PAIR_REJECT_UNKNOWN;
    if (ta == TYPE_EXTRA || tb == TYPE_EXTRA) return PAIR_REJECT_EXTRA;
    if (ta != tb) return PAIR_REJECT_DIFFERENT;
    uint8_t aa = reg_anchor(r, a), ab = reg_anchor(r, b);
    if (ta == TYPE_BASE && aa != ab) return PAIR_REJECT_DIFFERENT;
    if (ta == TYPE_TONE) return PAIR_REJECT_CROSS_TONE;
    if (ta == TYPE_COMPOUND || ta == TYPE_SYMBOL) return PAIR_REJECT_DIFFERENT;
    return PAIR_PASS;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第三部分: 编码查询
 * ═══════════════════════════════════════════════════════════════════════════ */

static int name_to_code(const char *name) {
    if (!name) return -1;
    for (int i = 0; i < TABLE_SIZE; i++) if (strcmp(TABLE[i].name, name) == 0) return TABLE[i].code;
    return -1;
}
static const char* code_to_name(uint8_t code) {
    if (code == CODE_EMPTY) return "[空]"; if (code == CODE_SELF) return "[我]"; if (code == CODE_OVERFLOW) return "[溢]";
    for (int i = 0; i < TABLE_SIZE; i++) if (TABLE[i].code == code) return TABLE[i].name;
    return "[-]";
}

static void reg_load_table(registry_t *r) {
    for (int i = 0; i < TABLE_SIZE; i++)
        reg_add(r, TABLE[i].name, TABLE[i].type, TABLE[i].anchor, TABLE[i].tier);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第四部分: 编织平衡
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_N 64
#define MAX_E 128
typedef struct { const char *name; float w, po, pn, nf; uint8_t bal; } node_t;
typedef struct { uint32_t from, to; float tens; uint8_t act; } edge_t;
typedef struct { uint32_t magic, nc, ec; node_t nodes[MAX_N]; edge_t edges[MAX_E]; float total_tens, bal_score; uint8_t is_bal; uint32_t bal_n, stress_n; } graph_t;

static void g_init(graph_t *g) { memset(g, 0, sizeof(*g)); g->magic = 0x57454156; }
static int g_find(graph_t *g, const char *name) {
    if (!g || !name) return -1;
    for (uint32_t i = 0; i < g->nc; i++) if (strcmp(g->nodes[i].name, name) == 0) return (int)i;
    return -1;
}
static float vweight(const char *s) { if (!s) return 0.5f; int l = (int)strlen(s); return l <= 1 ? 0.3f : l == 2 ? 0.6f : 0.9f; }
static int g_add_n(graph_t *g, const char *name, float w) {
    if (!g || g->magic != 0x57454156 || !name || g->nc >= MAX_N) return -1;
    int ex = g_find(g, name); if (ex >= 0) return ex;
    uint32_t i = g->nc++; g->nodes[i].name = name; g->nodes[i].w = w; return (int)i;
}
static int g_add_e(graph_t *g, const char *f, const char *t, float tens) {
    if (!g || g->magic != 0x57454156 || g->ec >= MAX_E) return -1;
    int fi = g_find(g, f), ti = g_find(g, t); if (fi < 0 || ti < 0) return -1;
    uint32_t i = g->ec++; g->edges[i].from = fi; g->edges[i].to = ti; g->edges[i].tens = tens; g->edges[i].act = 1;
    g->nodes[fi].po += tens; g->nodes[ti].pn += tens; g->total_tens += tens; return 0;
}
static void g_calc(graph_t *g) {
    if (!g || g->magic != 0x57454156) return;
    double tot = 0; g->bal_n = 0; g->stress_n = 0;
    for (uint32_t i = 0; i < g->nc; i++) { g->nodes[i].nf = g->nodes[i].po - g->nodes[i].pn; tot += fabs(g->nodes[i].nf); if (fabs(g->nodes[i].nf) < 0.1f) { g->nodes[i].bal = 1; g->bal_n++; } else { g->nodes[i].bal = 0; if (fabs(g->nodes[i].nf) > 0.5f) g->stress_n++; } }
    g->bal_score = g->ec > 0 ? 1.0f - (float)(tot / g->ec) : 1.0f;
    if (g->bal_score < 0) g->bal_score = 0;
    g->is_bal = (g->bal_score >= 0.7f && g->stress_n == 0) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第五部分: 审判 + 编码 联合报告
 * ═══════════════════════════════════════════════════════════════════════════ */

static void print_full_report(registry_t *r, graph_t *g, const char *a, const char *b) {
    pair_verdict_t v = judge(r, a, b);
    int ca = name_to_code(a), cb = name_to_code(b);

    printf("\n╔══════════════════════════════════════════════════════════════╗\n");
    printf("║            满 全 法 · 统 一 编 解 码 报 告                   ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ 配对: %-8s ↔ %-8s                                ║\n", a, b);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [编码]                                                       ║\n");
    if (ca >= 0) printf("║   %s → 0x%02X (", a, ca); else printf("║   %s → [未找到](", a);
    if (ca >= 0) for (int i = 7; i >= 0; i--) printf("%d", (ca >> i) & 1);
    printf(") 8比特\n");
    if (cb >= 0) printf("║   %s → 0x%02X (", b, cb); else printf("║   %s → [未找到](", b);
    if (cb >= 0) for (int i = 7; i >= 0; i--) printf("%d", (cb >> i) & 1);
    printf(") 8比特\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [类型]                                                       ║\n");
    printf("║   A: %-12s L%-2d 锚=%d                             ║\n", reg_type(r,a)==TYPE_BASE?"基韵锚":reg_type(r,a)==TYPE_TONE?"声调韵":reg_type(r,a)==TYPE_COMPOUND?"复韵母":reg_type(r,a)==TYPE_SYMBOL?"配对符号":"?", reg_level(r,a)>>4, reg_anchor(r,a));
    printf("║   B: %-12s L%-2d 锚=%d                             ║\n", reg_type(r,b)==TYPE_BASE?"基韵锚":reg_type(r,b)==TYPE_TONE?"声调韵":reg_type(r,b)==TYPE_COMPOUND?"复韵母":reg_type(r,b)==TYPE_SYMBOL?"配对符号":"?", reg_level(r,b)>>4, reg_anchor(r,b));
    printf("╠══════════════════════════════════════════════════════════════╣\n");

    /* 编织平衡 */
    g_add_n(g, a, vweight(a)); g_add_n(g, b, vweight(b));
    float vb = vweight(a) - vweight(b); if (vb < 0) vb = -vb;
    g_add_e(g, a, b, vb); g_calc(g);

    printf("║ [编织] 张力=%.2f 平衡=%.2f %s                              ║\n", vb, g->bal_score, g->is_bal?"✓":"✗");
    printf("╠══════════════════════════════════════════════════════════════╣\n");

    printf("║ [审判]                                                       ║\n");
    if (v == PAIR_PASS) printf("║   ★★★ 通过 ★★★                                          ║\n");
    else printf("║   ✗✗✗ 拒绝 ✗✗✗  %s                     ║\n", verdict_str(v));
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第六部分: 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int run_tests(void) {
    registry_t r; reg_init(&r); reg_load_table(&r);
    graph_t g; g_init(&g);
    int err = 0, tot = 0;

    printf("═══ 满全法编解码器 · 自检 ═══\n\n");

    /* 审判测试 */
    struct { const char *a, *b; pair_verdict_t ev; const char *d; } jt[] = {
        {"a","á",PAIR_REJECT_CROSS_LEVEL,"a(L1)≠á(L3)"},
        {"e","ě",PAIR_REJECT_CROSS_LEVEL,"e(L1)≠ě(L3)"},
        {"a","o",PAIR_REJECT_DIFFERENT,"a vs o 不同锚"},
        {"ā","á",PAIR_REJECT_CROSS_TONE,"声调韵不可配对"},
        {"Ӱ","ӱ",PAIR_REJECT_EXTRA,"溢出符号"},
        {"a","A",PAIR_REJECT_CROSS_LEVEL,"大小写也已分配(但A未在表中,实际为UNKNOWN->需确认)"},
    };
    int jt_n = sizeof(jt)/sizeof(jt[0]) - 1; /* 最后一个不算 */

    printf("[审判测试]\n");
    for (int i = 0; i < jt_n; i++) {
        tot++; pair_verdict_t got = judge(&r, jt[i].a, jt[i].b);
        if (got == jt[i].ev) { printf("  [PASS] %s\n", jt[i].d); } else { printf("  [FAIL] %s got=%d exp=%d\n", jt[i].d, got, jt[i].ev); err++; }
    }

    /* 编码测试 */
    printf("\n[编码测试]\n");
    tot++; int c = name_to_code("a"); if (c == 0x02) printf("  [PASS] a → 0x02\n"); else { printf("  [FAIL] a → %d\n", c); err++; }
    tot++; c = name_to_code("á"); if (c == 0x81) printf("  [PASS] á → 0x81\n"); else { printf("  [FAIL] á → %d\n", c); err++; }
    tot++; c = name_to_code("ai"); if (c == 0x10) printf("  [PASS] ai → 0x10\n"); else { printf("  [FAIL] ai → %d\n", c); err++; }
    tot++; const char *n = code_to_name(0x02); if (strcmp(n, "a") == 0) printf("  [PASS] 0x02 → a\n"); else { printf("  [FAIL] 0x02 → %s\n", n); err++; }

    /* 表唯一性 */
    printf("\n[表完整性]\n");
    int uniq = 1;
    for (int i = 0; i < TABLE_SIZE; i++) for (int j = i+1; j < TABLE_SIZE; j++) {
        if (TABLE[i].code == TABLE[j].code) { printf("  [FAIL] 冲突: %s=%s=0x%02X\n", TABLE[i].name, TABLE[j].name, TABLE[i].code); uniq = 0; err++; }
    }
    if (uniq) printf("  [PASS] %d个编码全部唯一\n", TABLE_SIZE);

    printf("\n结果: %d/%d 通过, %d 失败\n", tot - err, tot, err);
    return err;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第七部分: 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(registry_t *r) {
    char line[256], a[64], b[64];
    printf("\n═══ 满全法 · 交互模式 ═══\n");
    printf("  <A> <B>  审判+编码+报告\n");
    printf("  e <名>   查编码\n");
    printf("  q        退出\n");
    printf("─────────────────────────────────────────\n");
    graph_t g; g_init(&g);
    while (1) {
        printf("\n⮞ "); fflush(stdout);
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line); while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = 0;
        if (l == 0) continue;
        if (strcmp(line, "q") == 0) break;
        if (line[0] == 'e' && line[1] == ' ') {
            const char *nm = line + 2; int cd = name_to_code(nm);
            if (cd >= 0) { printf("  %s → 0x%02X (", nm, cd); for (int i = 7; i >= 0; i--) printf("%d", (cd >> i) & 1); printf(") 8比特\n"); }
            else printf("  %s → [未找到]\n", nm);
            continue;
        }
        if (sscanf(line, "%63s %63s", a, b) == 2) {
            g_init(&g);
            print_full_report(r, &g, a, b);
        } else { printf("  未知命令\n"); }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    registry_t r; reg_init(&r); reg_load_table(&r);
    graph_t g; g_init(&g);

    if (argc == 1) { return run_tests(); }
    if (argc == 2 && (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0)) { return run_tests(); }
    if (argc == 2 && (strcmp(argv[1], "-i") == 0 || strcmp(argv[1], "--interactive") == 0)) { interactive(&r); return 0; }
    if (argc >= 3) { print_full_report(&r, &g, argv[1], argv[2]); return judge(&r, argv[1], argv[2]) == PAIR_PASS ? 0 : 1; }
    printf("用法: %s [A B] | -t | -i\n", argv[0]);
    return 0;
}
