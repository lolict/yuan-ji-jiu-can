/**
 * color_system.c — 颜色建制实现
 *
 * 关卡执行顺序:
 *   第0关: 层级栈校验 → 跨层级直接拦截
 *   第1关: 类型身份校验 → 类型不符拦截
 *   第2关: 锚校验 → 不同锚拦截
 *   第3关: 审美过滤 → 不美观拦截
 *   第4关: 编织平衡 → 力不平衡拦截
 */

#include "color_system.h"
#include <string.h>

/* 审美阈值 */
#define AESTHETIC_THRESHOLD 0.3f
/* 编织平衡阈值 */
#define WEAVING_THRESHOLD 0.5f

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void color_system_init(color_system_t *sys) {
    memset(sys, 0, sizeof(*sys));
    sys->magic = COLOR_SYSTEM_MAGIC;
    
    /* 初始化三个子系统 */
    type_discipline_init(&sys->police);
    type_discipline_init_defaults(&sys->police);
    diffraction_court_init(&sys->court, &sys->police);
    weaving_init(&sys->weaving);
    
    sys->verbose = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 注册符号 (外部扩展用)
 * ═══════════════════════════════════════════════════════════════ */

int color_system_register(color_system_t *sys,
                           const char *name,
                           symbol_type_t type,
                           symbol_level_t level,
                           uint8_t anchor) {
    if (!sys || sys->magic != COLOR_SYSTEM_MAGIC || !name) return -1;
    return type_discipline_register_l(&sys->police, name, type, level, anchor);
}

/* ═══════════════════════════════════════════════════════════════
 * 完整审判 (五关)
 * ═══════════════════════════════════════════════════════════════ */

pair_verdict_t color_system_trial(color_system_t *sys,
                                   const char *name_a,
                                   const char *name_b) {
    if (!sys || sys->magic != COLOR_SYSTEM_MAGIC) return PAIR_REJECT_UNKNOWN;
    if (!name_a || !name_b) return PAIR_REJECT_UNKNOWN;
    
    sys->total_trials++;
    
    symbol_level_t level_a = type_discipline_get_level(&sys->police, name_a);
    symbol_level_t level_b = type_discipline_get_level(&sys->police, name_b);
    symbol_type_t type_a = type_discipline_get_type(&sys->police, name_a);
    symbol_type_t type_b = type_discipline_get_type(&sys->police, name_b);
    
    /* 第0关: 层级栈校验 */
    if (level_a == LEVEL_UNKNOWN || level_b == LEVEL_UNKNOWN) {
        sys->reject_level++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [LEVEL] UNKNOWN level → REJECT\n");
        return PAIR_REJECT_UNKNOWN;
    }
    if (level_a == LEVEL_L2_MODIFIER || level_b == LEVEL_L2_MODIFIER) {
        sys->reject_level++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [LEVEL] L2 modifier → REJECT\n");
        return PAIR_REJECT_MODIFIER;
    }
    if (level_a != level_b) {
        sys->reject_level++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [LEVEL] L%u vs L%u → REJECT\n",
                                  level_a >> 4, level_b >> 4);
        return PAIR_REJECT_CROSS_LEVEL;
    }
    
    /* 第1关: 类型身份校验 */
    if (type_a == TYPE_UNKNOWN || type_b == TYPE_UNKNOWN) {
        sys->reject_type++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [TYPE] UNKNOWN type → REJECT\n");
        return PAIR_REJECT_UNKNOWN;
    }
    if (type_a == TYPE_EXTRA || type_b == TYPE_EXTRA) {
        sys->reject_type++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [TYPE] EXTRA symbol → REJECT\n");
        return PAIR_REJECT_EXTRA;
    }
    if (type_a != type_b) {
        sys->reject_type++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [TYPE] type %u vs %u → REJECT\n", type_a, type_b);
        return PAIR_REJECT_DIFFERENT;
    }
    
    /* 第2关: 锚校验 */
    uint8_t anchor_a = 0xFF, anchor_b = 0xFF;
    for (uint16_t i = 0; i < sys->police.count; i++) {
        if (strcmp(sys->police.entries[i].name, name_a) == 0)
            anchor_a = sys->police.entries[i].anchor;
        if (strcmp(sys->police.entries[i].name, name_b) == 0)
            anchor_b = sys->police.entries[i].anchor;
    }
    if (anchor_a != anchor_b && anchor_a != 0xFF && anchor_b != 0xFF) {
        sys->reject_anchor++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [ANCHOR] anchor %u vs %u → REJECT\n", anchor_a, anchor_b);
        return PAIR_REJECT_CROSS_TONE;
    }
    
    /* 第3关: 审美过滤 */
    pair_diffraction_t pd;
    diffraction_court_diffract_pair(&sys->court, name_a, name_b, &pd);
    if (pd.overall_aesthetic < AESTHETIC_THRESHOLD) {
        sys->reject_aesthetic++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [AESTHETIC] score %.2f < %.2f → REJECT\n",
                                  pd.overall_aesthetic, AESTHETIC_THRESHOLD);
        return PAIR_REJECT_DIFFERENT; /* 审美失败归属为类型不匹配 */
    }
    
    /* 第4关: 编织平衡 */
    /* 先把节点和边加入编织图 */
    weaving_add_node(&sys->weaving, name_a, 0.5f);
    weaving_add_node(&sys->weaving, name_b, 0.5f);
    weaving_add_edge(&sys->weaving, name_a, name_b, pd.weaving_tension);
    weaving_compute_balance(&sys->weaving);
    if (!sys->weaving.is_balanced) {
        sys->reject_weaving++;
        sys->total_rejected++;
        if (sys->verbose) printf("  [WEAVING] balance %.2f → REJECT\n",
                                  sys->weaving.balance_score);
        return PAIR_REJECT_DIFFERENT;
    }
    
    /* 通过 */
    sys->total_passed++;
    if (sys->verbose) printf("  [PASS] all gates passed\n");
    return PAIR_PASS;
}

/* ═══════════════════════════════════════════════════════════════
 * 衍射分析
 * ═══════════════════════════════════════════════════════════════ */

void color_system_diffract(color_system_t *sys,
                           const char *name,
                           diffraction_result_t *result) {
    if (!sys || sys->magic != COLOR_SYSTEM_MAGIC || !name || !result) return;
    diffraction_court_diffract(&sys->court, name, result);
}

void color_system_pair_diffract(color_system_t *sys,
                                 const char *name_a,
                                 const char *name_b,
                                 pair_diffraction_t *result) {
    if (!sys || sys->magic != COLOR_SYSTEM_MAGIC || !name_a || !name_b || !result) return;
    diffraction_court_diffract_pair(&sys->court, name_a, name_b, result);
}

/* ═══════════════════════════════════════════════════════════════
 * 统计报告
 * ═══════════════════════════════════════════════════════════════ */

void color_system_stats(color_system_t *sys, FILE *output) {
    if (!sys || sys->magic != COLOR_SYSTEM_MAGIC || !output) return;
    
    fprintf(output, "═══════ Color System Reports ═══════\n\n");
    fprintf(output, "Total trials:   %u\n", sys->total_trials);
    fprintf(output, "Passed:         %u (%.1f%%)\n", sys->total_passed,
            sys->total_trials > 0 ? (float)sys->total_passed / sys->total_trials * 100 : 0);
    fprintf(output, "Rejected:       %u (%.1f%%)\n", sys->total_rejected,
            sys->total_trials > 0 ? (float)sys->total_rejected / sys->total_trials * 100 : 0);
    fprintf(output, "\n[Rejection by Gate]\n");
    fprintf(output, "  Gate 0 (Level):     %u\n", sys->reject_level);
    fprintf(output, "  Gate 1 (Type):      %u\n", sys->reject_type);
    fprintf(output, "  Gate 2 (Anchor):    %u\n", sys->reject_anchor);
    fprintf(output, "  Gate 3 (Aesthetic): %u\n", sys->reject_aesthetic);
    fprintf(output, "  Gate 4 (Weaving):   %u\n", sys->reject_weaving);
}

/* ═══════════════════════════════════════════════════════════════
 * 标签字符串
 * ═══════════════════════════════════════════════════════════════ */

const char* color_system_level_str(symbol_level_t level) {
    switch (level) {
        case LEVEL_L1_BASE:      return "L1-Base";
        case LEVEL_L2_MODIFIER:  return "L2-Modifier";
        case LEVEL_L3_ASSEMBLED: return "L3-Assembled";
        default:                 return "L?-Unknown";
    }
}

const char* color_system_type_str(symbol_type_t type) {
    switch (type) {
        case TYPE_BASE:     return "Base";
        case TYPE_TONE:     return "Tone";
        case TYPE_COMPOUND: return "Compound";
        case TYPE_SYMBOL:   return "Symbol";
        case TYPE_EXTRA:    return "Extra";
        case TYPE_SHENG_MU: return "ShengMu";
        case TYPE_MODIFIER: return "Modifier";
        default:            return "Unknown";
    }
}
