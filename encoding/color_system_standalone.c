/**
 * ═══════════════════════════════════════════════════════════════════════════
 * color_system_standalone.c — 颜色进制总控工具
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   单文件 · 可编译 · 可解释 · 可打包
 *
 *   整个满全法编码进制体系整合为一个自包含的 C 工具:
 *     ┌─────────────────────────────────────────────────────────────────┐
 *     │ 第0层: 类型纪律       L1/L2/L3 层级栈 + 强制类型标签           │
 *     │ 第1层: 衍射引擎       单节点/配对 → 多维度衍射                  │
 *     │ 第2层: 编织平衡       力均衡计算 · 节点压/被压 制衡              │
 *     │ 第3层: 五门审判       Level→Type→Anchor→Aesthetic→Weaving       │
 *     │ 第4层: 像素相位运算   加法=拼装 减法=拆解 乘除禁用               │
 *     │ 第5层: 全局协调得分   综合五门拉力 → 模拟共振场(白盒可审计)      │
 *     └─────────────────────────────────────────────────────────────────┘
 *
 * 编译:
 *   cc -Wall -Wextra -std=c99 -O2 -o color_system color_system_standalone.c -lm
 *
 * 使用:
 *   ./color_system               运行自检测试
 *   ./color_system a á           审判一对符号 (输出完整报告)
 *   ./color_system -i            进入交互解释器
 *   ./color_system -d a á        输出衍射分析
 *   ./color_system -t            运行完整测试套件
 *   ./color_system -r a á        输出配对衍射报告
 *
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

#ifndef M_PI
#define M_PI 3.14159265f
#endif

/* ═══════════════════════════════════════════════════════════════════════════
 * 第零部分: 基础类型定义
 * ═══════════════════════════════════════════════════════════════════════════ */

/* ─────────────────────────────────────────────────────────────────────
 * 类型枚举 — 强制分类, 不可模糊
 * ───────────────────────────────────────────────────────────────────── */
typedef enum {
    TYPE_BASE     = 0x01,   /* 基韵锚: a o e i u ü = 公约数锚 */
    TYPE_TONE     = 0x02,   /* 声调韵: ā á ǎ à = 公倍数 */
    TYPE_COMPOUND = 0x03,   /* 复韵母名: ai ei ao... */
    TYPE_SYMBOL   = 0x04,   /* 配对符号: ꬰ ə... = 赋值的公倍数 */
    TYPE_EXTRA    = 0x05,   /* 溢出符号: 进集装箱 */
    TYPE_SHENG_MU = 0x06,   /* 声母: 48声 */
    TYPE_MODIFIER = 0x07,   /* 裸修饰零件: ˊ ˋ ˇ ˆ */
    TYPE_UNKNOWN  = 0xFF,   /* 未分类 = 不允许 */
} symbol_type_t;

/* ─────────────────────────────────────────────────────────────────────
 * 层级栈 L1/L2/L3
 * ───────────────────────────────────────────────────────────────────── */
typedef enum {
    LEVEL_L1_BASE      = 0x10,   /* 基元本体 */
    LEVEL_L2_MODIFIER  = 0x20,   /* 修饰零件 */
    LEVEL_L3_ASSEMBLED = 0x30,   /* 装配成品 */
    LEVEL_UNKNOWN      = 0xFF,   /* 未分类 */
} symbol_level_t;

/* ─────────────────────────────────────────────────────────────────────
 * 配对审判结果
 * ───────────────────────────────────────────────────────────────────── */
typedef enum {
    PAIR_PASS                = 0,
    PAIR_REJECT_CROSS_LEVEL  = 1,  /* 跨层级 (a≠á) */
    PAIR_REJECT_MODIFIER     = 2,  /* 修饰零件禁止配对 */
    PAIR_REJECT_CROSS_TONE   = 3,  /* 跨基调 */
    PAIR_REJECT_EXTRA        = 4,  /* 溢出符号 */
    PAIR_REJECT_UNKNOWN      = 5,  /* 未分类 */
    PAIR_REJECT_DIFFERENT    = 6,  /* 类型不同 */
} pair_verdict_t;

/* ─────────────────────────────────────────────────────────────────────
 * 审判结果描述
 * ───────────────────────────────────────────────────────────────────── */
static const char* verdict_str(pair_verdict_t v) {
    switch (v) {
        case PAIR_PASS:               return "PASS — 许可配对";
        case PAIR_REJECT_CROSS_LEVEL: return "REJECT — 跨层级 (L1≠L3)";
        case PAIR_REJECT_MODIFIER:    return "REJECT — 修饰零件禁止配对";
        case PAIR_REJECT_CROSS_TONE:  return "REJECT — 跨基调不可配对";
        case PAIR_REJECT_EXTRA:       return "REJECT — 溢出符号禁止配对";
        case PAIR_REJECT_UNKNOWN:     return "REJECT — 有未分类符号";
        case PAIR_REJECT_DIFFERENT:   return "REJECT — 类型不同绝对拒绝";
    }
    return "UNKNOWN";
}

static const char* level_str(symbol_level_t l) {
    switch (l) {
        case LEVEL_L1_BASE:      return "L1-Base(基元本体)";
        case LEVEL_L2_MODIFIER:  return "L2-Modifier(修饰零件)";
        case LEVEL_L3_ASSEMBLED: return "L3-Assembled(装配成品)";
        default:                 return "L?-Unknown(未分类)";
    }
}

static const char* type_str(symbol_type_t t) {
    switch (t) {
        case TYPE_BASE:     return "Base(基韵锚)";
        case TYPE_TONE:     return "Tone(声调韵)";
        case TYPE_COMPOUND: return "Compound(复韵母名)";
        case TYPE_SYMBOL:   return "Symbol(配对符号)";
        case TYPE_EXTRA:    return "Extra(溢出)";
        case TYPE_SHENG_MU: return "ShengMu(声母)";
        case TYPE_MODIFIER: return "Modifier(裸修饰)";
        default:            return "Unknown(未分类)";
    }
}

static const char* anchor_str(uint8_t a) {
    switch (a) {
        case 0: return "a";
        case 1: return "o";
        case 2: return "e";
        case 3: return "i";
        case 4: return "u";
        case 5: return "ü";
        default: return "-";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第一部分: 类型纪律系统 (内联)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_REGISTRY 256
#define TYPE_DISCIPLINE_MAGIC 0x54594453

typedef struct {
    uint32_t     magic;
    uint16_t     count;
    struct {
        const char    *name;
        symbol_type_t  type;
        symbol_level_t level;
        uint8_t        anchor;
    } entries[MAX_REGISTRY];
} type_discipline_t;

static void td_init(type_discipline_t *td) {
    memset(td, 0, sizeof(*td));
    td->magic = TYPE_DISCIPLINE_MAGIC;
}

static int td_register_l(type_discipline_t *td, const char *name,
                          symbol_type_t type, symbol_level_t level, uint8_t anchor) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return -1;
    if (td->count >= MAX_REGISTRY) return -1;
    if (!name || !name[0]) return -1;
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) {
            if (td->entries[i].type != type) return -2;
            return 0;
        }
    }
    td->entries[td->count].name   = name;
    td->entries[td->count].type   = type;
    td->entries[td->count].level  = level;
    td->entries[td->count].anchor = anchor;
    td->count++;
    return 0;
}

static int td_register(type_discipline_t *td, const char *name,
                        symbol_type_t type, uint8_t anchor) {
    symbol_level_t level;
    switch (type) {
        case TYPE_BASE: case TYPE_COMPOUND: case TYPE_SYMBOL:
        case TYPE_SHENG_MU: case TYPE_EXTRA:
            level = LEVEL_L1_BASE; break;
        case TYPE_TONE:
            level = LEVEL_L3_ASSEMBLED; break;
        case TYPE_MODIFIER:
            level = LEVEL_L2_MODIFIER; break;
        default:
            level = LEVEL_UNKNOWN; break;
    }
    return td_register_l(td, name, type, level, anchor);
}

static symbol_type_t td_get_type(type_discipline_t *td, const char *name) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC || !name) return TYPE_UNKNOWN;
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) return td->entries[i].type;
    }
    return TYPE_UNKNOWN;
}

static symbol_level_t td_get_level(type_discipline_t *td, const char *name) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC || !name) return LEVEL_UNKNOWN;
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) return td->entries[i].level;
    }
    return LEVEL_UNKNOWN;
}

/* 查找锚 */
static uint8_t td_get_anchor(type_discipline_t *td, const char *name) {
    if (!td || !name) return 0xFF;
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) return td->entries[i].anchor;
    }
    return 0xFF;
}

/* 核心: 配对审判 */
static pair_verdict_t td_judge(type_discipline_t *td,
                                const char *name_a, const char *name_b) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return PAIR_REJECT_UNKNOWN;
    if (!name_a || !name_b) return PAIR_REJECT_UNKNOWN;

    symbol_type_t ta = td_get_type(td, name_a);
    symbol_type_t tb = td_get_type(td, name_b);
    symbol_level_t la = td_get_level(td, name_a);
    symbol_level_t lb = td_get_level(td, name_b);

    /* 第0关: 层级栈校验 */
    if (la == LEVEL_UNKNOWN || lb == LEVEL_UNKNOWN) return PAIR_REJECT_UNKNOWN;
    if (la == LEVEL_L2_MODIFIER || lb == LEVEL_L2_MODIFIER) return PAIR_REJECT_MODIFIER;
    if (la != lb) return PAIR_REJECT_CROSS_LEVEL;

    /* 第1关: 类型身份校验 */
    if (ta == TYPE_UNKNOWN || tb == TYPE_UNKNOWN) return PAIR_REJECT_UNKNOWN;
    if (ta == TYPE_EXTRA || tb == TYPE_EXTRA) return PAIR_REJECT_EXTRA;
    if (ta != tb) return PAIR_REJECT_DIFFERENT;

    /* 第2关: 锚校验 */
    uint8_t aa = td_get_anchor(td, name_a);
    uint8_t ab = td_get_anchor(td, name_b);
    if (ta == TYPE_BASE && aa != ab) return PAIR_REJECT_DIFFERENT;
    if (ta == TYPE_TONE) {
        /* 声调韵是时钟上的独立位置, 无论锚是否相同、符号是否相同, 彼此不可配对 */
        return PAIR_REJECT_CROSS_TONE;
    }
    if (ta == TYPE_COMPOUND) return PAIR_REJECT_DIFFERENT;
    if (ta == TYPE_SYMBOL) return PAIR_REJECT_DIFFERENT;

    return PAIR_PASS;
}

/* 初始化默认数据 */
static void td_init_defaults(type_discipline_t *td) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return;

    /* 基韵锚 TYPE_BASE (60声) */
    td_register(td, "a",  TYPE_BASE, 0); td_register(td, "A",  TYPE_BASE, 0);
    td_register(td, "o",  TYPE_BASE, 1); td_register(td, "O",  TYPE_BASE, 1);
    td_register(td, "e",  TYPE_BASE, 2); td_register(td, "E",  TYPE_BASE, 2);
    td_register(td, "i",  TYPE_BASE, 3); td_register(td, "I",  TYPE_BASE, 3);
    td_register(td, "u",  TYPE_BASE, 4); td_register(td, "U",  TYPE_BASE, 4);
    td_register(td, "ü",  TYPE_BASE, 5); td_register(td, "Ü",  TYPE_BASE, 5);

    /* 声调韵 TYPE_TONE (24个) */
    td_register(td, "ā", TYPE_TONE, 0); td_register(td, "á", TYPE_TONE, 0);
    td_register(td, "ǎ", TYPE_TONE, 0); td_register(td, "à", TYPE_TONE, 0);
    td_register(td, "ō", TYPE_TONE, 1); td_register(td, "ó", TYPE_TONE, 1);
    td_register(td, "ǒ", TYPE_TONE, 1); td_register(td, "ò", TYPE_TONE, 1);
    td_register(td, "ē", TYPE_TONE, 2); td_register(td, "é", TYPE_TONE, 2);
    td_register(td, "ě", TYPE_TONE, 2); td_register(td, "è", TYPE_TONE, 2);
    td_register(td, "ī", TYPE_TONE, 3); td_register(td, "í", TYPE_TONE, 3);
    td_register(td, "ǐ", TYPE_TONE, 3); td_register(td, "ì", TYPE_TONE, 3);
    td_register(td, "ū", TYPE_TONE, 4); td_register(td, "ú", TYPE_TONE, 4);
    td_register(td, "ǔ", TYPE_TONE, 4); td_register(td, "ù", TYPE_TONE, 4);
    td_register(td, "ǖ", TYPE_TONE, 5); td_register(td, "ǘ", TYPE_TONE, 5);
    td_register(td, "ǚ", TYPE_TONE, 5); td_register(td, "ǜ", TYPE_TONE, 5);

    /* 复韵母名 TYPE_COMPOUND (33个) */
    td_register(td, "ai", TYPE_COMPOUND, 0xFF); td_register(td, "ei", TYPE_COMPOUND, 0xFF);
    td_register(td, "ui", TYPE_COMPOUND, 0xFF); td_register(td, "ao", TYPE_COMPOUND, 0xFF);
    td_register(td, "ou", TYPE_COMPOUND, 0xFF); td_register(td, "iu", TYPE_COMPOUND, 0xFF);
    td_register(td, "ie", TYPE_COMPOUND, 0xFF); td_register(td, "üe", TYPE_COMPOUND, 0xFF);
    td_register(td, "er", TYPE_COMPOUND, 0xFF); td_register(td, "an", TYPE_COMPOUND, 0xFF);
    td_register(td, "en", TYPE_COMPOUND, 0xFF); td_register(td, "in", TYPE_COMPOUND, 0xFF);
    td_register(td, "un", TYPE_COMPOUND, 0xFF); td_register(td, "ün", TYPE_COMPOUND, 0xFF);
    td_register(td, "ang", TYPE_COMPOUND, 0xFF); td_register(td, "eng", TYPE_COMPOUND, 0xFF);
    td_register(td, "ing", TYPE_COMPOUND, 0xFF); td_register(td, "ong", TYPE_COMPOUND, 0xFF);
    td_register(td, "ia", TYPE_COMPOUND, 0xFF); td_register(td, "ian", TYPE_COMPOUND, 0xFF);
    td_register(td, "iang", TYPE_COMPOUND, 0xFF); td_register(td, "iong", TYPE_COMPOUND, 0xFF);
    td_register(td, "ua", TYPE_COMPOUND, 0xFF); td_register(td, "uo", TYPE_COMPOUND, 0xFF);
    td_register(td, "uai", TYPE_COMPOUND, 0xFF); td_register(td, "uan", TYPE_COMPOUND, 0xFF);
    td_register(td, "uang", TYPE_COMPOUND, 0xFF); td_register(td, "üan", TYPE_COMPOUND, 0xFF);

    /* 复韵母配对符号 TYPE_SYMBOL (66个) */
    td_register(td, "ꬰ", TYPE_SYMBOL, 0xFF); td_register(td, "ꬱ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ə", TYPE_SYMBOL, 0xFF); td_register(td, "Ə", TYPE_SYMBOL, 0xFF);
    td_register(td, "ѱ", TYPE_SYMBOL, 0xFF); td_register(td, "Ѱ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ꜵ", TYPE_SYMBOL, 0xFF); td_register(td, "Ꜵ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ȣ", TYPE_SYMBOL, 0xFF); td_register(td, "Ȣ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ꭎ", TYPE_SYMBOL, 0xFF); td_register(td, "Ⴞ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ɤ", TYPE_SYMBOL, 0xFF); td_register(td, "Ɣ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ᵫ", TYPE_SYMBOL, 0xFF); td_register(td, "ꭂ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ɛ", TYPE_SYMBOL, 0xFF); td_register(td, "Ɛ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ꙙ", TYPE_SYMBOL, 0xFF); td_register(td, "Ꙙ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ⱄ", TYPE_SYMBOL, 0xFF); td_register(td, "ⱅ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ĩ", TYPE_SYMBOL, 0xFF); td_register(td, "Ĩ", TYPE_SYMBOL, 0xFF);
    td_register(td, "տ", TYPE_SYMBOL, 0xFF); td_register(td, "໙", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ⴐ", TYPE_SYMBOL, 0xFF); td_register(td, "Ⴆ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ⴊ", TYPE_SYMBOL, 0xFF); td_register(td, "ӫ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ӫ", TYPE_SYMBOL, 0xFF); td_register(td, "ꙟ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ꙟ", TYPE_SYMBOL, 0xFF); td_register(td, "ꭁ", TYPE_SYMBOL, 0xFF);
    td_register(td, "ꭀ", TYPE_SYMBOL, 0xFF); td_register(td, "ꙗ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ꙗ", TYPE_SYMBOL, 0xFF); td_register(td, "ꙝ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ꙝ", TYPE_SYMBOL, 0xFF); td_register(td, "ѭ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ѭ", TYPE_SYMBOL, 0xFF); td_register(td, "ȱ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ȱ", TYPE_SYMBOL, 0xFF); td_register(td, "ǟ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ǟ", TYPE_SYMBOL, 0xFF); td_register(td, "ȫ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ȫ", TYPE_SYMBOL, 0xFF); td_register(td, "ꜻ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ꜻ", TYPE_SYMBOL, 0xFF); td_register(td, "Ⴄ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ⴂ", TYPE_SYMBOL, 0xFF); td_register(td, "ꙛ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ꙛ", TYPE_SYMBOL, 0xFF); td_register(td, "ѫ", TYPE_SYMBOL, 0xFF);
    td_register(td, "Ѫ", TYPE_SYMBOL, 0xFF);

    /* 溢出符号 TYPE_EXTRA — 集装箱0/1/2/5 */
    td_register(td, "Ӱ", TYPE_EXTRA, 0xFF); td_register(td, "ӱ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ѧ", TYPE_EXTRA, 0xFF); td_register(td, "ѧ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ѽ", TYPE_EXTRA, 0xFF); td_register(td, "ѽ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ѥ", TYPE_EXTRA, 0xFF); td_register(td, "ѥ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ё", TYPE_EXTRA, 0xFF); td_register(td, "ё", TYPE_EXTRA, 0xFF);
    td_register(td, "Є", TYPE_EXTRA, 0xFF); td_register(td, "є", TYPE_EXTRA, 0xFF);
    td_register(td, "Ʝ", TYPE_EXTRA, 0xFF); td_register(td, "႘", TYPE_EXTRA, 0xFF);
    td_register(td, "Ꙍ", TYPE_EXTRA, 0xFF); td_register(td, "ꙍ", TYPE_EXTRA, 0xFF);
    td_register(td, "ꭃ", TYPE_EXTRA, 0xFF); td_register(td, "ꭄ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ⴘ", TYPE_EXTRA, 0xFF); td_register(td, "Ⴏ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ⴎ", TYPE_EXTRA, 0xFF); td_register(td, "Ⴌ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ⴋ", TYPE_EXTRA, 0xFF); td_register(td, "☁", TYPE_EXTRA, 0xFF);
    td_register(td, "Ӭ", TYPE_EXTRA, 0xFF); td_register(td, "ӭ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ѿ", TYPE_EXTRA, 0xFF); td_register(td, "ѿ", TYPE_EXTRA, 0xFF);
    td_register(td, "Ȥ", TYPE_EXTRA, 0xFF); td_register(td, "Ç", TYPE_EXTRA, 0xFF);
    td_register(td, "Ŋ", TYPE_EXTRA, 0xFF); td_register(td, "ȥ", TYPE_EXTRA, 0xFF);
    td_register(td, "ç", TYPE_EXTRA, 0xFF); td_register(td, "Ӗ", TYPE_EXTRA, 0xFF);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第二部分: 衍射引擎 (内联)
 * ═══════════════════════════════════════════════════════════════════════════ */

/* 单节点衍射结果 */
typedef struct {
    const char    *name;
    symbol_type_t  type_diffract;
    uint8_t        anchor_diffract;
    float          aesthetic_score;
    uint8_t        balanced;
    uint8_t        goal_code;
    uint8_t        can_cooperate;
    uint8_t        partner_count;
    float          pressure_given;
    float          pressure_received;
    float          equilibrium_delta;
} diffraction_result_t;

/* 配对衍射结果 */
typedef struct {
    const char    *name_a;
    const char    *name_b;
    symbol_type_t  type_a;
    symbol_type_t  type_b;
    uint8_t        type_match;
    uint8_t        anchor_match;
    float          symmetry;
    float          proportion;
    float          density_balance;
    float          overall_aesthetic;
    float          weaving_tension;
    float          weaving_balance;
    uint8_t        is_weaving_ok;
} pair_diffraction_t;

/* 视觉重量(简化代理: 字节长度) */
static float visual_weight(const char *s) {
    if (!s) return 0.5f;
    int len = (int)strlen(s);
    if (len <= 1) return 0.3f;
    if (len == 2) return 0.6f;
    return 0.9f;
}

static float visual_height(const char *s) {
    if (!s) return 0.5f;
    int len = (int)strlen(s);
    if (len >= 3) return 0.8f;
    if (len == 2) return 0.6f;
    return 0.4f;
}

/* 单节点衍射 */
static void diffract_single(type_discipline_t *td, const char *name,
                             diffraction_result_t *r) {
    if (!td || !name || !r) return;
    memset(r, 0, sizeof(*r));
    r->name = name;
    r->type_diffract = td_get_type(td, name);
    r->anchor_diffract = td_get_anchor(td, name);

    float w = visual_weight(name);
    float h = visual_height(name);
    r->aesthetic_score = 1.0f - fabsf(w - h);

    switch (r->type_diffract) {
        case TYPE_BASE:     r->goal_code = 1; break;
        case TYPE_TONE:     r->goal_code = 2; break;
        case TYPE_COMPOUND: r->goal_code = 3; break;
        case TYPE_SYMBOL:   r->goal_code = 3; break;
        case TYPE_EXTRA:    r->goal_code = 0; break;
        default:            r->goal_code = 0; break;
    }

    r->can_cooperate = (r->type_diffract != TYPE_UNKNOWN && r->type_diffract != TYPE_EXTRA);
    r->partner_count = r->can_cooperate ? 1 : 0;
    r->pressure_given = w;
    r->pressure_received = 0.0f;
    r->equilibrium_delta = w;
    r->balanced = 0;
}

/* 配对衍射 */
static void diffract_pair(type_discipline_t *td,
                           const char *name_a, const char *name_b,
                           pair_diffraction_t *r) {
    if (!td || !name_a || !name_b || !r) return;
    memset(r, 0, sizeof(*r));
    r->name_a = name_a;
    r->name_b = name_b;

    r->type_a = td_get_type(td, name_a);
    r->type_b = td_get_type(td, name_b);
    r->type_match = (r->type_a == r->type_b) ? 1 : 0;

    uint8_t aa = td_get_anchor(td, name_a);
    uint8_t ab = td_get_anchor(td, name_b);
    r->anchor_match = (aa == ab && aa != 0xFF) ? 1 : 0;

    float wa = visual_weight(name_a), wb = visual_weight(name_b);
    float ha = visual_height(name_a), hb = visual_height(name_b);

    r->symmetry = 1.0f - fabsf(wa - wb);
    r->proportion = 1.0f - fabsf(ha - hb);
    r->density_balance = 1.0f - (fabsf(wa - wb) + fabsf(ha - hb)) / 2.0f;
    r->overall_aesthetic = (r->symmetry + r->proportion + r->density_balance) / 3.0f;

    r->weaving_tension = fabsf(wa - wb) + fabsf(ha - hb);
    r->weaving_balance = 1.0f - r->weaving_tension / 2.0f;
    if (r->weaving_balance < 0) r->weaving_balance = 0;
    r->is_weaving_ok = (r->weaving_tension < 0.5f && r->type_match) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第三部分: 编织平衡 (内联)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_NODES 64
#define MAX_EDGES 128
#define WEAVING_MAGIC 0x57454156
#define BALANCE_THRESHOLD 0.1f

typedef struct {
    const char    *name;
    float          weight;
    float          pressure_out;
    float          pressure_in;
    float          net_force;
    uint8_t        balanced;
    uint32_t       edge_count;
} weaving_node_t;

typedef struct {
    uint32_t       from;
    uint32_t       to;
    float          tension;
    uint8_t        active;
} weaving_edge_t;

typedef struct {
    uint32_t       magic;
    uint32_t       node_count;
    uint32_t       edge_count;
    weaving_node_t nodes[MAX_NODES];
    weaving_edge_t edges[MAX_EDGES];
    float          total_tension;
    float          balance_score;
    uint8_t        is_balanced;
    uint32_t       balanced_nodes;
    uint32_t       stressed_nodes;
} weaving_graph_t;

static void weaving_init(weaving_graph_t *wg) {
    memset(wg, 0, sizeof(*wg));
    wg->magic = WEAVING_MAGIC;
}

static int weaving_find_node(weaving_graph_t *wg, const char *name) {
    if (!wg || !name) return -1;
    for (uint32_t i = 0; i < wg->node_count; i++)
        if (strcmp(wg->nodes[i].name, name) == 0) return (int)i;
    return -1;
}

static int weaving_add_node(weaving_graph_t *wg, const char *name, float weight) {
    if (!wg || wg->magic != WEAVING_MAGIC) return -1;
    if (wg->node_count >= MAX_NODES) return -1;
    if (!name) return -1;
    int existing = weaving_find_node(wg, name);
    if (existing >= 0) return existing;
    uint32_t idx = wg->node_count;
    wg->nodes[idx].name = name;
    wg->nodes[idx].weight = weight;
    wg->nodes[idx].pressure_out = 0.0f;
    wg->nodes[idx].pressure_in = 0.0f;
    wg->nodes[idx].net_force = 0.0f;
    wg->nodes[idx].balanced = 0;
    wg->nodes[idx].edge_count = 0;
    wg->node_count++;
    return (int)idx;
}

static int weaving_add_edge(weaving_graph_t *wg, const char *from,
                             const char *to, float tension) {
    if (!wg || wg->magic != WEAVING_MAGIC) return -1;
    if (wg->edge_count >= MAX_EDGES) return -1;
    int idx_from = weaving_find_node(wg, from);
    int idx_to = weaving_find_node(wg, to);
    if (idx_from < 0 || idx_to < 0) return -1;
    uint32_t eidx = wg->edge_count;
    wg->edges[eidx].from = (uint32_t)idx_from;
    wg->edges[eidx].to = (uint32_t)idx_to;
    wg->edges[eidx].tension = tension;
    wg->edges[eidx].active = 1;
    wg->edge_count++;
    wg->nodes[idx_from].pressure_out += tension;
    wg->nodes[idx_to].pressure_in += tension;
    wg->nodes[idx_from].edge_count++;
    wg->nodes[idx_to].edge_count++;
    wg->total_tension += tension;
    return 0;
}

static void weaving_compute_balance(weaving_graph_t *wg) {
    if (!wg || wg->magic != WEAVING_MAGIC) return;
    double total_abs_net = 0.0;
    wg->balanced_nodes = 0;
    wg->stressed_nodes = 0;
    for (uint32_t i = 0; i < wg->node_count; i++) {
        weaving_node_t *n = &wg->nodes[i];
        n->net_force = n->pressure_out - n->pressure_in;
        total_abs_net += fabs((double)n->net_force);
        if (fabs((double)n->net_force) < BALANCE_THRESHOLD) {
            n->balanced = 1;
            wg->balanced_nodes++;
        } else {
            n->balanced = 0;
            if (fabs((double)n->net_force) > 0.5f) wg->stressed_nodes++;
        }
    }
    if (wg->edge_count > 0) {
        wg->balance_score = 1.0f - (float)(total_abs_net / wg->edge_count);
        if (wg->balance_score < 0.0f) wg->balance_score = 0.0f;
    } else {
        wg->balance_score = 1.0f;
    }
    wg->is_balanced = (wg->balance_score >= 0.7f && wg->stressed_nodes == 0) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第四部分: 五门审判 + 全局协调得分
 * ═══════════════════════════════════════════════════════════════════════════ */

#define COLOR_SYSTEM_MAGIC 0x434F4C53

/* 五门审判的详细结果 */
typedef struct {
    pair_verdict_t verdict;         /* 最终判决 */
    int            rejected_gate;   /* -1=通过, 0~4=被第几门拒绝 */
    
    /* 各门输出 */
    int            gate_level_pass;     /* 第0门: 层级 */
    int            gate_type_pass;      /* 第1门: 类型 */
    int            gate_anchor_pass;    /* 第2门: 锚 */
    int            gate_aesthetic_pass; /* 第3门: 审美 */
    int            gate_weaving_pass;   /* 第4门: 编织 */
    
    /* 全局协调得分 (综合五门拉力, 模拟共振场但白盒可审计) */
    float          coordination_score;
    
    /* 输入信息 */
    symbol_level_t level_a, level_b;
    symbol_type_t  type_a, type_b;
    uint8_t        anchor_a, anchor_b;
    
    /* 衍射数据 */
    pair_diffraction_t diffraction;
} trial_report_t;

/* 五门审判完整实现 */
static pair_verdict_t five_gate_trial(type_discipline_t *td,
                                       weaving_graph_t *wg,
                                       const char *name_a, const char *name_b,
                                       trial_report_t *report) {
    memset(report, 0, sizeof(*report));
    report->rejected_gate = -1;
    report->verdict = PAIR_PASS;
    
    symbol_level_t la = td_get_level(td, name_a);
    symbol_level_t lb = td_get_level(td, name_b);
    symbol_type_t ta = td_get_type(td, name_a);
    symbol_type_t tb = td_get_type(td, name_b);
    uint8_t aa = td_get_anchor(td, name_a);
    uint8_t ab = td_get_anchor(td, name_b);
    
    report->level_a = la; report->level_b = lb;
    report->type_a = ta; report->type_b = tb;
    report->anchor_a = aa; report->anchor_b = ab;
    
    /* 第0门: 层级栈校验 */
    report->gate_level_pass = 1;
    if (la == LEVEL_UNKNOWN || lb == LEVEL_UNKNOWN) {
        report->gate_level_pass = 0;
        report->rejected_gate = 0;
        report->verdict = PAIR_REJECT_UNKNOWN;
        goto done;
    }
    if (la == LEVEL_L2_MODIFIER || lb == LEVEL_L2_MODIFIER) {
        report->gate_level_pass = 0;
        report->rejected_gate = 0;
        report->verdict = PAIR_REJECT_MODIFIER;
        goto done;
    }
    if (la != lb) {
        report->gate_level_pass = 0;
        report->rejected_gate = 0;
        report->verdict = PAIR_REJECT_CROSS_LEVEL;
        goto done;
    }
    
    /* 第1门: 类型身份校验 */
    report->gate_type_pass = 1;
    if (ta == TYPE_UNKNOWN || tb == TYPE_UNKNOWN) {
        report->gate_type_pass = 0;
        report->rejected_gate = 1;
        report->verdict = PAIR_REJECT_UNKNOWN;
        goto done;
    }
    if (ta == TYPE_EXTRA || tb == TYPE_EXTRA) {
        report->gate_type_pass = 0;
        report->rejected_gate = 1;
        report->verdict = PAIR_REJECT_EXTRA;
        goto done;
    }
    if (ta != tb) {
        report->gate_type_pass = 0;
        report->rejected_gate = 1;
        report->verdict = PAIR_REJECT_DIFFERENT;
        goto done;
    }
    
    /* 第2门: 锚校验 */
    report->gate_anchor_pass = 1;
    if (ta == TYPE_BASE && aa != ab) {
        report->gate_anchor_pass = 0;
        report->rejected_gate = 2;
        report->verdict = PAIR_REJECT_DIFFERENT;
        goto done;
    }
    if (ta == TYPE_TONE) {
        /* 声调韵是时钟上的独立位置, 彼此不可配对 */
        report->gate_anchor_pass = 0;
        report->rejected_gate = 2;
        report->verdict = PAIR_REJECT_CROSS_TONE;
        goto done;
    }
    if (ta == TYPE_COMPOUND || ta == TYPE_SYMBOL) {
        report->gate_anchor_pass = 0;
        report->rejected_gate = 2;
        report->verdict = PAIR_REJECT_DIFFERENT;
        goto done;
    }
    
    /* 第3门: 审美过滤 */
    diffract_pair(td, name_a, name_b, &report->diffraction);
    report->gate_aesthetic_pass = 1;
    if (report->diffraction.overall_aesthetic < 0.3f) {
        report->gate_aesthetic_pass = 0;
        report->rejected_gate = 3;
        report->verdict = PAIR_REJECT_DIFFERENT;
        goto done;
    }
    
    /* 第4门: 编织平衡 */
    report->gate_weaving_pass = 1;
    weaving_add_node(wg, name_a, visual_weight(name_a));
    weaving_add_node(wg, name_b, visual_weight(name_b));
    weaving_add_edge(wg, name_a, name_b, report->diffraction.weaving_tension);
    weaving_compute_balance(wg);
    if (!wg->is_balanced) {
        report->gate_weaving_pass = 0;
        report->rejected_gate = 4;
        report->verdict = PAIR_REJECT_DIFFERENT;
        goto done;
    }
    
done:
    /* 计算全局协调得分 = 五门通过率 × 审美 × 编织 */
    { float gate_sum = (float)(report->gate_level_pass + report->gate_type_pass +
                              report->gate_anchor_pass + report->gate_aesthetic_pass +
                              report->gate_weaving_pass);
      float gate_avg = gate_sum / 5.0f;
      float aesthetic_f = report->diffraction.overall_aesthetic;
      float weaving_f = wg->balance_score;
      report->coordination_score = gate_avg * 0.5f + aesthetic_f * 0.25f + weaving_f * 0.25f;
    }
    
    return report->verdict;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第五部分: 像素相位运算 (加减法 · 乘除禁用)
 * ═══════════════════════════════════════════════════════════════════════════ */

/* 像素相位运算类型 */
typedef enum {
    PIXEL_ADD = 0,   /* 加法: 部件拼装 (安装) */
    PIXEL_SUB = 1,   /* 减法: 部件拆解 (剥离) */
    PIXEL_MUL = 2,   /* 乘法: 禁用 (放大/复制无意义) */
    PIXEL_DIV = 3,   /* 除法: 禁用 (均分无意义) */
} pixel_op_t;

/* 像素相位运算结果 */
typedef struct {
    pixel_op_t op;
    int        success;
    const char *description;
    float      result_weight;
} pixel_arith_result_t;

/**
 * 像素相位运算 — 只允许加法(拼装)和减法(拆解)
 * 
 * 加法: 从源字形拆解部件 → 安装到目标字形 → 生成新字形
 * 减法: 从完整字形挖掉部件 → 还原为骨架碎片
 * 乘法: 禁用 (放大倍数破坏等效镜像距离)
 * 除法: 禁用 (均分切割没有可装配语义)
 */
static pixel_arith_result_t pixel_arithmetic(type_discipline_t *td,
                                              pixel_op_t op,
                                              const char *source,
                                              const char *target) {
    pixel_arith_result_t result;
    memset(&result, 0, sizeof(result));
    result.op = op;
    result.success = 0;
    
    switch (op) {
        case PIXEL_ADD:
            /* 加法: source + target部件 → 拼装 */
            if (!source || !target) {
                result.description = "加法失败: 空输入";
                result.success = 0;
                return result;
            }
            /* 检查: 目标必须是基元本体(L1)才能拼装 */
            if (td_get_level(td, target) != LEVEL_L1_BASE) {
                result.description = "加法失败: 目标必须是L1基元本体才能拼装";
                result.success = 0;
                return result;
            }
            /* 检查: 源不能是溢出符号 */
            if (td_get_type(td, source) == TYPE_EXTRA) {
                result.description = "加法失败: 溢出符号不能作为部件来源";
                result.success = 0;
                return result;
            }
            result.success = 1;
            result.result_weight = visual_weight(source) + visual_weight(target) * 0.3f;
            result.description = "加法成功: 部件拼装完成";
            return result;
            
        case PIXEL_SUB:
            /* 减法: source - target部件 → 拆解 */
            if (!source || !target) {
                result.description = "减法失败: 空输入";
                result.success = 0;
                return result;
            }
            /* 检查: 被拆的必须是装配成品(L3)才能拆解 */
            if (td_get_level(td, source) != LEVEL_L3_ASSEMBLED) {
                result.description = "减法失败: 只有L3装配成品才能拆解";
                result.success = 0;
                return result;
            }
            result.success = 1;
            result.result_weight = visual_weight(source) - visual_weight(target) * 0.3f;
            if (result.result_weight < 0.1f) result.result_weight = 0.1f;
            result.description = "减法成功: 部件剥离完成";
            return result;
            
        case PIXEL_MUL:
            /* 乘法: 禁用 */
            result.success = 0;
            result.result_weight = 0.0f;
            result.description = "乘法禁用: 倍数放大破坏等效镜像距离";
            return result;
            
        case PIXEL_DIV:
            /* 除法: 禁用 */
            result.success = 0;
            result.result_weight = 0.0f;
            result.description = "除法禁用: 均分切割无可装配语义";
            return result;
    }
    
    result.description = "未知运算";
    return result;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第六部分: 完整审判报告输出
 * ═══════════════════════════════════════════════════════════════════════════ */

static void print_trial_report(type_discipline_t *td, trial_report_t *report,
                                const char *name_a, const char *name_b) {
    (void)td; /* 保留用于未来扩展 */
    printf("\n");
    printf("╔══════════════════════════════════════════════════════════════╗\n");
    printf("║           颜 色 进 制 · 五 门 审 判 报 告                    ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ 配对: %-10s ↔ %-10s                              ║\n", name_a, name_b);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    printf("║ [基本信息]                                                   ║\n");
    printf("║   A: %s | %s | 锚=%s ║\n",
           level_str(report->level_a), type_str(report->type_a),
           anchor_str(report->anchor_a));
    printf("║   B: %s | %s | 锚=%s ║\n",
           level_str(report->level_b), type_str(report->type_b),
           anchor_str(report->anchor_b));
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    printf("║ [五门审判]                                                   ║\n");
    printf("║   第0门 层级栈:   %s                                    ║\n",
           report->gate_level_pass ? "✓ 通过" : "✗ 拒绝");
    printf("║   第1门 类型身份: %s                                    ║\n",
           report->gate_type_pass ? "✓ 通过" : "✗ 拒绝");
    printf("║   第2门 锚校验:   %s                                    ║\n",
           report->gate_anchor_pass ? "✓ 通过" : "✗ 拒绝");
    printf("║   第3门 审美过滤: %s                                    ║\n",
           report->gate_aesthetic_pass ? "✓ 通过" : "✗ 拒绝");
    printf("║   第4门 编织平衡: %s                                    ║\n",
           report->gate_weaving_pass ? "✓ 通过" : "✗ 拒绝");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    printf("║ [衍射数据]                                                   ║\n");
    printf("║   类型匹配: %s | 锚匹配: %s                             ║\n",
           report->diffraction.type_match ? "是" : "否",
           report->diffraction.anchor_match ? "是" : "否");
    printf("║   对称: %.2f | 比例: %.2f | 密度: %.2f | 审美: %.2f  ║\n",
           report->diffraction.symmetry, report->diffraction.proportion,
           report->diffraction.density_balance, report->diffraction.overall_aesthetic);
    printf("║   编织张力: %.2f | 编织平衡: %.2f | 编织合格: %s        ║\n",
           report->diffraction.weaving_tension, report->diffraction.weaving_balance,
           report->diffraction.is_weaving_ok ? "是" : "否");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    printf("║ [全局协调得分] %.4f                                         ║\n",
           report->coordination_score);
    printf("║   = 五门通过率×0.5 + 审美×0.25 + 编织平衡×0.25            ║\n");
    printf("║   (模拟共振场效果, 但白盒可审计, 非黑盒直觉)                 ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    printf("║ [最终判决]                                                   ║\n");
    if (report->verdict == PAIR_PASS) {
        printf("║   ★★★ 通过 ★★★                                          ║\n");
    } else {
        printf("║   ✗✗✗ 拒绝 ✗✗✗  (第%d门拦截)                            ║\n",
               report->rejected_gate);
        printf("║   原因: %-40s         ║\n", verdict_str(report->verdict));
    }
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第七部分: 衍射分析报告
 * ═══════════════════════════════════════════════════════════════════════════ */

static void print_diffraction_report(type_discipline_t *td,
                                      const char *name_a, const char *name_b) {
    diffraction_result_t ra, rb;
    pair_diffraction_t pr;
    
    diffract_single(td, name_a, &ra);
    diffract_single(td, name_b, &rb);
    diffract_pair(td, name_a, name_b, &pr);
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════════════════╗\n");
    printf("║              衍 射 分 析 报 告                                ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ 配对: %s ↔ %s                                              ║\n", name_a, name_b);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [单节点衍射]                                                 ║\n");
    printf("║   A: 类型=%s 锚=%s 审美=%.2f                        ║\n",
           type_str(ra.type_diffract), anchor_str(ra.anchor_diffract), ra.aesthetic_score);
    printf("║   B: 类型=%s 锚=%s 审美=%.2f                        ║\n",
           type_str(rb.type_diffract), anchor_str(rb.anchor_diffract), rb.aesthetic_score);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║ [配对衍射]                                                   ║\n");
    printf("║   类型匹配: %s | 锚匹配: %s                                 ║\n",
           pr.type_match ? "✓" : "✗", pr.anchor_match ? "✓" : "✗");
    printf("║   对称=%.2f 比例=%.2f 密度=%.2f 综合审美=%.2f           ║\n",
           pr.symmetry, pr.proportion, pr.density_balance, pr.overall_aesthetic);
    printf("║   编织张力=%.2f 编织平衡=%.2f 编织合格=%s                ║\n",
           pr.weaving_tension, pr.weaving_balance, pr.is_weaving_ok ? "✓" : "✗");
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第八部分: 交互解释器
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive_mode(type_discipline_t *td) {
    char input[256];
    char name_a[64], name_b[64];
    
    printf("\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  颜色进制 · 交互解释器\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  命令:\n");
    printf("    <符号A> <符号B>    — 审判一对符号\n");
    printf("    d <符号A> <符号B>  — 衍射分析\n");
    printf("    p <符号A> <符号B>  — 像素运算\n");
    printf("    info <符号>        — 查看符号信息\n");
    printf("    t                  — 运行自检测试\n");
    printf("    q                  — 退出\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    
    while (1) {
        printf("\n⮞ ");
        fflush(stdout);
        if (!fgets(input, sizeof(input), stdin)) break;
        
        /* 去掉尾部换行 */
        size_t len = strlen(input);
        while (len > 0 && (input[len-1] == '\n' || input[len-1] == '\r')) {
            input[--len] = '\0';
        }
        
        if (len == 0) continue;
        
        /* 退出 */
        if (strcmp(input, "q") == 0 || strcmp(input, "quit") == 0) {
            printf("再见\n");
            break;
        }
        
        /* 自检测试 */
        if (strcmp(input, "t") == 0 || strcmp(input, "test") == 0) {
            extern int self_test(void);
            self_test();
            continue;
        }
        
        /* 衍射分析 */
        if (input[0] == 'd' && input[1] == ' ') {
            if (sscanf(input + 2, "%63s %63s", name_a, name_b) == 2) {
                print_diffraction_report(td, name_a, name_b);
            } else {
                printf("用法: d <符号A> <符号B>\n");
            }
            continue;
        }
        
        /* 像素运算 */
        if (input[0] == 'p' && input[1] == ' ') {
            if (sscanf(input + 2, "%63s %63s", name_a, name_b) == 2) {
                printf("\n[像素相位运算]\n");
                pixel_arith_result_t r;
                
                r = pixel_arithmetic(td, PIXEL_ADD, name_a, name_b);
                printf("  加法(A+B): %s → %s\n", r.success ? "✓" : "✗", r.description);
                
                r = pixel_arithmetic(td, PIXEL_SUB, name_a, name_b);
                printf("  减法(A-B): %s → %s\n", r.success ? "✓" : "✗", r.description);
                
                r = pixel_arithmetic(td, PIXEL_MUL, name_a, name_b);
                printf("  乘法(A×B): %s → %s\n", r.success ? "✓" : "✗", r.description);
                
                r = pixel_arithmetic(td, PIXEL_DIV, name_a, name_b);
                printf("  除法(A÷B): %s → %s\n", r.success ? "✓" : "✗", r.description);
            } else {
                printf("用法: p <符号A> <符号B>\n");
            }
            continue;
        }
        
        /* 查看符号信息 */
        if (strncmp(input, "info ", 5) == 0) {
            const char *name = input + 5;
            diffraction_result_t r;
            diffract_single(td, name, &r);
            printf("\n[符号信息] %s\n", name);
            printf("  类型: %s\n", type_str(r.type_diffract));
            printf("  层级: %s\n", level_str(td_get_level(td, name)));
            printf("  锚: %s\n", anchor_str(r.anchor_diffract));
            printf("  审美: %.2f\n", r.aesthetic_score);
            printf("  目标码: %u\n", r.goal_code);
            printf("  可合作: %s\n", r.can_cooperate ? "是" : "否");
            continue;
        }
        
        /* 默认: 审判一对符号 */
        if (sscanf(input, "%63s %63s", name_a, name_b) == 2) {
            weaving_graph_t wg;
            weaving_init(&wg);
            trial_report_t report;
            five_gate_trial(td, &wg, name_a, name_b, &report);
            print_trial_report(td, &report, name_a, name_b);
        } else {
            printf("未知命令: %s\n", input);
            printf("  输入 't' 运行测试, 'q' 退出, 或输入两个符号名\n");
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 第九部分: 自检测试
 * ═══════════════════════════════════════════════════════════════════════════ */

int self_test(void) {
    type_discipline_t td;
    td_init(&td);
    td_init_defaults(&td);
    
    int total = 0, passed = 0, failed = 0;
    
    printf("\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("║           颜 色 进 制 · 自 检 测 试                            ║\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    
    /* ───────────────────────────────────────
     * 测试1: 类型查询
     * ─────────────────────────────────────── */
    printf("\n[测试1] 类型查询\n");
    
    struct { const char *name; symbol_type_t expected; } type_tests[] = {
        {"a", TYPE_BASE}, {"A", TYPE_BASE}, {"á", TYPE_TONE},
        {"ai", TYPE_COMPOUND}, {"ꬰ", TYPE_SYMBOL}, {"Ӱ", TYPE_EXTRA},
    };
    
    for (int i = 0; i < 6; i++) {
        total++;
        symbol_type_t got = td_get_type(&td, type_tests[i].name);
        if (got == type_tests[i].expected) {
            passed++;
            printf("  [PASS] %s → %s\n", type_tests[i].name, type_str(got));
        } else {
            failed++;
            printf("  [FAIL] %s → expected %s, got %s\n",
                   type_tests[i].name, type_str(type_tests[i].expected), type_str(got));
        }
    }
    
    /* ───────────────────────────────────────
     * 测试2: 层级查询
     * ─────────────────────────────────────── */
    printf("\n[测试2] 层级查询\n");
    
    struct { const char *name; symbol_level_t expected; } level_tests[] = {
        {"a", LEVEL_L1_BASE}, {"á", LEVEL_L3_ASSEMBLED},
        {"ai", LEVEL_L1_BASE}, {"Ӱ", LEVEL_L1_BASE},
    };
    
    for (int i = 0; i < 4; i++) {
        total++;
        symbol_level_t got = td_get_level(&td, level_tests[i].name);
        if (got == level_tests[i].expected) {
            passed++;
            printf("  [PASS] %s → %s\n", level_tests[i].name, level_str(got));
        } else {
            failed++;
            printf("  [FAIL] %s → expected %s, got %s\n",
                   level_tests[i].name, level_str(level_tests[i].expected), level_str(got));
        }
    }
    
    /* ───────────────────────────────────────
     * 测试3: a=á 必须拒绝
     * ─────────────────────────────────────── */
    printf("\n[测试3] a=á 必须拒绝 (核心测试)\n");
    
    struct { const char *a; const char *b; pair_verdict_t expected; const char *desc; } judge_tests[] = {
        /* 必须拒绝 */
        {"a", "á", PAIR_REJECT_CROSS_LEVEL, "a(L1) ≠ á(L3)"},
        {"e", "ě", PAIR_REJECT_CROSS_LEVEL, "e(L1) ≠ ě(L3)"},
        {"a", "o", PAIR_REJECT_DIFFERENT, "a vs o 不同锚"},
        {"a", "ꬰ", PAIR_REJECT_DIFFERENT, "BASE vs SYMBOL 不同类型"},
        {"ā", "á", PAIR_REJECT_CROSS_TONE, "ā vs á 跨基调"},
        {"Ӱ", "ӱ", PAIR_REJECT_EXTRA, "溢出符号禁止配对"},
        {"x", "y", PAIR_REJECT_UNKNOWN, "未分类符号"},
    };
    
    for (int i = 0; i < 7; i++) {
        total++;
        pair_verdict_t got = td_judge(&td, judge_tests[i].a, judge_tests[i].b);
        if (got == judge_tests[i].expected) {
            passed++;
            printf("  [PASS] %s → %s\n", judge_tests[i].desc, verdict_str(got));
        } else {
            failed++;
            printf("  [FAIL] %s → expected %s, got %s\n",
                   judge_tests[i].desc, verdict_str(judge_tests[i].expected), verdict_str(got));
        }
    }
    
    /* ───────────────────────────────────────
     * 测试4: 五门审判
     * ─────────────────────────────────────── */
    printf("\n[测试4] 五门审判\n");
    
    {
        weaving_graph_t wg;
        weaving_init(&wg);
        trial_report_t report;
        pair_verdict_t v;
        
        /* a vs á 应该被第0门(层级)拒绝 */
        v = five_gate_trial(&td, &wg, "a", "á", &report);
        total++;
        if (v == PAIR_REJECT_CROSS_LEVEL && report.rejected_gate == 0) {
            passed++;
            printf("  [PASS] a vs á → 第0门拒绝 (层级)\n");
        } else {
            failed++;
            printf("  [FAIL] a vs á → 应该在第0门拒绝, got gate=%d\n", report.rejected_gate);
        }
        
        /* e vs ě 应该被第0门拒绝 */
        v = five_gate_trial(&td, &wg, "e", "ě", &report);
        total++;
        if (v == PAIR_REJECT_CROSS_LEVEL && report.rejected_gate == 0) {
            passed++;
            printf("  [PASS] e vs ě → 第0门拒绝 (层级)\n");
        } else {
            failed++;
            printf("  [FAIL] e vs ě → 应该在第0门拒绝, got gate=%d\n", report.rejected_gate);
        }
    }
    
    /* ───────────────────────────────────────
     * 测试5: 像素相位运算
     * ─────────────────────────────────────── */
    printf("\n[测试5] 像素相位运算\n");
    
    {
        pixel_arith_result_t r;
        
        /* 加法: 基元+基元 → 成功 */
        r = pixel_arithmetic(&td, PIXEL_ADD, "a", "i");
        total++;
        if (r.success) {
            passed++;
            printf("  [PASS] a + i → 加法成功\n");
        } else {
            failed++;
            printf("  [FAIL] a + i → 应该成功: %s\n", r.description);
        }
        
        /* 减法: 装配成品拆解 → 成功 */
        r = pixel_arithmetic(&td, PIXEL_SUB, "á", "a");
        total++;
        if (r.success) {
            passed++;
            printf("  [PASS] á - a → 减法成功\n");
        } else {
            failed++;
            printf("  [FAIL] á - a → 应该成功: %s\n", r.description);
        }
        
        /* 乘法 → 禁用 */
        r = pixel_arithmetic(&td, PIXEL_MUL, "a", "á");
        total++;
        if (!r.success) {
            passed++;
            printf("  [PASS] a × á → 乘法禁用\n");
        } else {
            failed++;
            printf("  [FAIL] a × á → 应该禁用\n");
        }
        
        /* 除法 → 禁用 */
        r = pixel_arithmetic(&td, PIXEL_DIV, "a", "á");
        total++;
        if (!r.success) {
            passed++;
            printf("  [PASS] a ÷ á → 除法禁用\n");
        } else {
            failed++;
            printf("  [FAIL] a ÷ á → 应该禁用\n");
        }
    }
    
    /* ───────────────────────────────────────
     * 测试结果
     * ─────────────────────────────────────── */
    printf("\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  测试结果: %d/%d 通过, %d 失败\n", passed, total, failed);
    printf("═══════════════════════════════════════════════════════════════\n");
    
    return failed;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    /* 初始化类型纪律 */
    type_discipline_t td;
    td_init(&td);
    td_init_defaults(&td);
    
    /* 没有参数 → 自检 */
    if (argc == 1) {
        return self_test();
    }
    
    /* -t → 测试 */
    if (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0) {
        return self_test();
    }
    
    /* -i → 交互解释器 */
    if (strcmp(argv[1], "-i") == 0 || strcmp(argv[1], "--interactive") == 0) {
        interactive_mode(&td);
        return 0;
    }
    
    /* -d → 衍射分析 */
    if (strcmp(argv[1], "-d") == 0 || strcmp(argv[1], "--diffract") == 0) {
        if (argc < 4) {
            printf("用法: %s -d <符号A> <符号B>\n", argv[0]);
            return 1;
        }
        print_diffraction_report(&td, argv[2], argv[3]);
        return 0;
    }
    
    /* -r → 完整审判报告 (默认模式) */
    if (argc >= 3) {
        weaving_graph_t wg;
        weaving_init(&wg);
        trial_report_t report;
        pair_verdict_t v = five_gate_trial(&td, &wg, argv[1], argv[2], &report);
        print_trial_report(&td, &report, argv[1], argv[2]);
        return v == PAIR_PASS ? 0 : 1;
    }
    
    printf("颜色进制总控工具 — 使用方式:\n");
    printf("  %s              运行自检测试\n", argv[0]);
    printf("  %s -t           运行测试套件\n", argv[0]);
    printf("  %s -i           交互解释器\n", argv[0]);
    printf("  %s -d A B       衍射分析\n", argv[0]);
    printf("  %s <A> <B>      审判一对符号\n", argv[0]);
    return 0;
}
