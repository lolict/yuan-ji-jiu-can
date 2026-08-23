/**
 * type_discipline.c — 类型纪律系统实现
 *
 * 核心原则: 类型不匹配 = 绝对拒绝
 * a=á 不通过, a=ꬰ 不通过, ā=á 不通过
 */

#include "type_discipline.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void type_discipline_init(type_discipline_t *td) {
    memset(td, 0, sizeof(*td));
    td->magic = TYPE_DISCIPLINE_MAGIC;
}

/* ═══════════════════════════════════════════════════════════════
 * 注册
 * ═══════════════════════════════════════════════════════════════ */

int type_discipline_register(type_discipline_t *td,
                              const char *name,
                              symbol_type_t type,
                              uint8_t anchor) {
    /* 自动推断层级 */
    symbol_level_t level;
    switch (type) {
        case TYPE_BASE:
        case TYPE_COMPOUND:
        case TYPE_SYMBOL:
        case TYPE_SHENG_MU:
        case TYPE_EXTRA:
            level = LEVEL_L1_BASE;
            break;
        case TYPE_TONE:
            level = LEVEL_L3_ASSEMBLED;
            break;
        case TYPE_MODIFIER:
            level = LEVEL_L2_MODIFIER;
            break;
        default:
            level = LEVEL_UNKNOWN;
            break;
    }
    return type_discipline_register_l(td, name, type, level, anchor);
}

int type_discipline_register_l(type_discipline_t *td,
                                const char *name,
                                symbol_type_t type,
                                symbol_level_t level,
                                uint8_t anchor) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return -1;
    if (td->count >= MAX_REGISTRY) return -1;
    if (!name || !name[0]) return -1;

    /* 检查是否已注册且类型冲突 */
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) {
            if (td->entries[i].type != type) return -2; /* 类型冲突 */
            return 0; /* 已存在且类型一致, 不重复添加 */
        }
    }

    td->entries[td->count].name   = name;
    td->entries[td->count].type   = type;
    td->entries[td->count].level  = level;
    td->entries[td->count].anchor = anchor;
    td->count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查询类型
 * ═══════════════════════════════════════════════════════════════ */

symbol_type_t type_discipline_get_type(type_discipline_t *td, const char *name) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC || !name) return TYPE_UNKNOWN;

    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) {
            return td->entries[i].type;
        }
    }
    return TYPE_UNKNOWN;
}

symbol_level_t type_discipline_get_level(type_discipline_t *td, const char *name) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC || !name) return LEVEL_UNKNOWN;

    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name) == 0) {
            return td->entries[i].level;
        }
    }
    return LEVEL_UNKNOWN;
}

/* ═══════════════════════════════════════════════════════════════
 * 核心: 配对审判
 *
 * 执行顺序 (不可调换):
 *   第0关: 层级栈校验 — 跨层级直接拦截
 *   第1关: 类型身份校验
 *   第2关: 锚校验
 * ═══════════════════════════════════════════════════════════════ */

pair_verdict_t type_discipline_judge(type_discipline_t *td,
                                     const char *name_a,
                                     const char *name_b) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return PAIR_REJECT_UNKNOWN;
    if (!name_a || !name_b) return PAIR_REJECT_UNKNOWN;

    symbol_type_t type_a = type_discipline_get_type(td, name_a);
    symbol_type_t type_b = type_discipline_get_type(td, name_b);
    symbol_level_t level_a = type_discipline_get_level(td, name_a);
    symbol_level_t level_b = type_discipline_get_level(td, name_b);

    /* 第0关: 层级栈校验 (最高优先级) */
    if (level_a == LEVEL_UNKNOWN || level_b == LEVEL_UNKNOWN)
        return PAIR_REJECT_UNKNOWN;

    /* L2修饰零件禁止和任何层级配对 */
    if (level_a == LEVEL_L2_MODIFIER || level_b == LEVEL_L2_MODIFIER)
        return PAIR_REJECT_MODIFIER;

    /* 跨层级 = 绝对禁止 (a(L1) ↔ á(L3)) */
    if (level_a != level_b)
        return PAIR_REJECT_CROSS_LEVEL;

    /* 第1关: 类型身份校验 */
    if (type_a == TYPE_UNKNOWN || type_b == TYPE_UNKNOWN)
        return PAIR_REJECT_UNKNOWN;

    if (type_a == TYPE_EXTRA || type_b == TYPE_EXTRA)
        return PAIR_REJECT_EXTRA;

    /* 类型不同 = 绝对拒绝 */
    if (type_a != type_b)
        return PAIR_REJECT_DIFFERENT;

    /* 第2关: 锚校验 (只对同类型同层级) */
    uint8_t anchor_a = 0xFF, anchor_b = 0xFF;
    for (uint16_t i = 0; i < td->count; i++) {
        if (strcmp(td->entries[i].name, name_a) == 0) anchor_a = td->entries[i].anchor;
        if (strcmp(td->entries[i].name, name_b) == 0) anchor_b = td->entries[i].anchor;
    }

    if (type_a == TYPE_BASE) {
        if (anchor_a != anchor_b) return PAIR_REJECT_DIFFERENT;
    }

    if (type_a == TYPE_TONE) {
        if (anchor_a != anchor_b) return PAIR_REJECT_CROSS_TONE;
        if (strcmp(name_a, name_b) == 0) return PAIR_REJECT_CROSS_TONE;
    }

    if (type_a == TYPE_COMPOUND) {
        return PAIR_REJECT_DIFFERENT;
    }

    if (type_a == TYPE_SYMBOL) {
        return PAIR_REJECT_DIFFERENT;
    }

    /* 通过 */
    return PAIR_PASS;
}

/* ═══════════════════════════════════════════════════════════════
 * 审判结果文字
 * ═══════════════════════════════════════════════════════════════ */

const char* type_discipline_verdict_str(pair_verdict_t v) {
    switch (v) {
        case PAIR_PASS:                return "PASS — 层级匹配, 许可配对";
        case PAIR_REJECT_CROSS_LEVEL:  return "REJECT — 跨层级 (a(L1)≠á(L3))";
        case PAIR_REJECT_MODIFIER:     return "REJECT — 修饰零件(L2)禁止配对";
        case PAIR_REJECT_CROSS_TONE:   return "REJECT — 跨基调不可配对 (ā≠á)";
        case PAIR_REJECT_EXTRA:        return "REJECT — 溢出符号禁止配对";
        case PAIR_REJECT_UNKNOWN:      return "REJECT — 有未分类符号";
        case PAIR_REJECT_DIFFERENT:    return "REJECT — 类型不同, 绝对拒绝";
    }
    return "UNKNOWN";
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化默认数据 — 按满全法编码体系严格分类
 *
 * 锚值: 0=a, 1=o, 2=e, 3=i, 4=u, 5=ü, 0xFF=不适用
 * ═══════════════════════════════════════════════════════════════ */

void type_discipline_init_defaults(type_discipline_t *td) {
    if (!td || td->magic != TYPE_DISCIPLINE_MAGIC) return;

    /* ── 基韵锚 TYPE_BASE (60声时钟层) ── */
    type_discipline_register(td, "a",  TYPE_BASE, 0);
    type_discipline_register(td, "A",  TYPE_BASE, 0);
    type_discipline_register(td, "o",  TYPE_BASE, 1);
    type_discipline_register(td, "O",  TYPE_BASE, 1);
    type_discipline_register(td, "e",  TYPE_BASE, 2);
    type_discipline_register(td, "E",  TYPE_BASE, 2);
    type_discipline_register(td, "i",  TYPE_BASE, 3);
    type_discipline_register(td, "I",  TYPE_BASE, 3);
    type_discipline_register(td, "u",  TYPE_BASE, 4);
    type_discipline_register(td, "U",  TYPE_BASE, 4);
    type_discipline_register(td, "ü",  TYPE_BASE, 5);
    type_discipline_register(td, "Ü",  TYPE_BASE, 5);

    /* ── 声调韵 TYPE_TONE (60声公倍数, 24个) ── */
    /* a系 */
    type_discipline_register(td, "ā", TYPE_TONE, 0);
    type_discipline_register(td, "á", TYPE_TONE, 0);
    type_discipline_register(td, "ǎ", TYPE_TONE, 0);
    type_discipline_register(td, "à", TYPE_TONE, 0);
    /* o系 */
    type_discipline_register(td, "ō", TYPE_TONE, 1);
    type_discipline_register(td, "ó", TYPE_TONE, 1);
    type_discipline_register(td, "ǒ", TYPE_TONE, 1);
    type_discipline_register(td, "ò", TYPE_TONE, 1);
    /* e系 */
    type_discipline_register(td, "ē", TYPE_TONE, 2);
    type_discipline_register(td, "é", TYPE_TONE, 2);
    type_discipline_register(td, "ě", TYPE_TONE, 2);
    type_discipline_register(td, "è", TYPE_TONE, 2);
    /* i系 */
    type_discipline_register(td, "ī", TYPE_TONE, 3);
    type_discipline_register(td, "í", TYPE_TONE, 3);
    type_discipline_register(td, "ǐ", TYPE_TONE, 3);
    type_discipline_register(td, "ì", TYPE_TONE, 3);
    /* u系 */
    type_discipline_register(td, "ū", TYPE_TONE, 4);
    type_discipline_register(td, "ú", TYPE_TONE, 4);
    type_discipline_register(td, "ǔ", TYPE_TONE, 4);
    type_discipline_register(td, "ù", TYPE_TONE, 4);
    /* ü系 */
    type_discipline_register(td, "ǖ", TYPE_TONE, 5);
    type_discipline_register(td, "ǘ", TYPE_TONE, 5);
    type_discipline_register(td, "ǚ", TYPE_TONE, 5);
    type_discipline_register(td, "ǜ", TYPE_TONE, 5);

    /* ── 复韵母名 TYPE_COMPOUND (33个, 0xFF=不适用锚) ── */
    type_discipline_register(td, "ai",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ei",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ui",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ao",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ou",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "iu",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ie",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "üe",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "er",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "an",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "en",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "in",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "un",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ün",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ang", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "eng", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ing", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ong", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ia",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ian", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "iang",TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "iong",TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "ua",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "uo",  TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "uai", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "uan", TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "uang",TYPE_COMPOUND, 0xFF);
    type_discipline_register(td, "üan", TYPE_COMPOUND, 0xFF);

    /* ── 复韵母配对符号 TYPE_SYMBOL (已确认的配对) ── */
    type_discipline_register(td, "ꬰ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꬱ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ə", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ə", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ѱ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ѱ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꜵ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꜵ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ȣ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ȣ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꭎ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴞ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ɤ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ɣ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ᵫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꭂ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ɛ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ɛ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꙙ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꙙ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ⱄ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ⱅ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ĩ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ĩ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "տ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "໙", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴐ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴆ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴊ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ӫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ӫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꙟ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꙟ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꭁ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꭀ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꙗ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꙗ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꙝ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꙝ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ѭ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ѭ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ȱ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ȱ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ǟ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ǟ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ȫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ȫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꜻ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꜻ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴄ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ⴂ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ꙛ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ꙛ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "ѫ", TYPE_SYMBOL, 0xFF);
    type_discipline_register(td, "Ѫ", TYPE_SYMBOL, 0xFF);

    /* ── 溢出符号 TYPE_EXTRA (集装箱0/1/2/5) ── */
    /* 集装箱0 */
    type_discipline_register(td, "Ӱ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ӱ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ѧ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ѧ", TYPE_EXTRA, 0xFF);
    /* 集装箱1 */
    type_discipline_register(td, "Ѽ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ѽ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ѥ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ѥ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ё", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ё", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Є", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "є", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ʝ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "႘", TYPE_EXTRA, 0xFF);
    /* 集装箱2 */
    type_discipline_register(td, "Ꙍ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ꙍ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ꭃ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ꭄ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ⴘ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ⴏ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ⴎ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ⴌ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ⴋ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "☁", TYPE_EXTRA, 0xFF);
    /* 集装箱5 */
    type_discipline_register(td, "Ӭ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ӭ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ѿ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ѿ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ȥ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ç", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ŋ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ȥ", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "ç", TYPE_EXTRA, 0xFF);
    type_discipline_register(td, "Ӗ", TYPE_EXTRA, 0xFF);
}
