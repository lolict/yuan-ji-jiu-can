/**
 * ═══════════════════════════════════════════════════════════════════════════
 * court.c — 犯罪笔录 + 法官系统
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   不是只抓一粒芝麻, 而是感知洋流的走向
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 犯罪笔录: 记录每一次拒绝的详情 (类型/配对/门/时间)
 * 法官系统:
 *   · 累积各类型犯罪的统计
 *   · 检测模式 (哪种错误最频繁? 哪个锚点最薄弱?)
 *   · 当某类犯罪达到阈值 → 涌现新规则 (自动拦截未来同类)
 *   · 生成法庭报告 (洋流图)
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o court court.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 犯罪类型 (8种)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum {
    CRIME_TYPE_MISMATCH = 0,    /* 类型不匹配 (BASE vs TONE) */
    CRIME_TIER_JUMP,            /* 层级跳跃 (L1≠L3) */
    CRIME_AESTHETIC_FAIL,       /* 审美失败 (对称/比例/密度) */
    CRIME_BALANCE_BREAK,        /* 编织失衡 (力不均匀) */
    CRIME_UNKNOWN_INPUT,        /* 未分类输入 */
    CRIME_ERROR_PROPAGATE,      /* 错误传播 (前序门已拒绝但继续) */
    CRIME_GOAL_DEVIATE,         /* 目标偏离 (goal_code不匹配) */
    CRIME_EXTRA_UNCATEGORIZED,  /* 溢出未归类 */
    CRIME_COUNT                 /* 总数 */
} crime_type_t;

static const char* crime_name(crime_type_t c) {
    switch (c) {
        case CRIME_TYPE_MISMATCH: return "类型不匹配";
        case CRIME_TIER_JUMP: return "层级跳跃";
        case CRIME_AESTHETIC_FAIL: return "审美失败";
        case CRIME_BALANCE_BREAK: return "编织失衡";
        case CRIME_UNKNOWN_INPUT: return "未分类输入";
        case CRIME_ERROR_PROPAGATE: return "错误传播";
        case CRIME_GOAL_DEVIATE: return "目标偏离";
        case CRIME_EXTRA_UNCATEGORIZED: return "溢出未归类";
        default: return "?";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 犯罪记录
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_RECORDS 256
#define MAX_RULES   16
#define EMERGE_THRESHOLD 3    /* 同类犯罪达到3次 → 涌现规则 */

typedef struct {
    int          seq;           /* 序号 */
    crime_type_t type;          /* 犯罪类型 */
    const char  *name_a;        /* 符号A */
    const char  *name_b;        /* 符号B */
    int          gate;          /* 拦截门 (0~4) */
    int          timestamp;     /* 时间戳 */
} crime_record_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 涌现的规则
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    int          active;        /* 是否激活 */
    crime_type_t target;        /* 针对哪种犯罪 */
    int          gate;          /* 在哪门拦截 */
    int          threshold;     /* 触发阈值 */
    char         desc[128];     /* 规则描述 */
    int          prevented;     /* 已阻止次数 (这条规则阻止了多少次) */
} emerged_rule_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 法庭
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    /* 犯罪记录 (环形缓冲区) */
    crime_record_t  records[MAX_RECORDS];
    int             record_count;       /* 总记录数 */
    int             record_pos;         /* 环形缓冲区写入位置 */

    /* 统计 */
    int             stats[CRIME_COUNT]; /* 各类型计数 */
    int             total_crimes;       /* 总犯罪数 */
    int             total_passed;       /* 总通过数 */
    int             gate_stats[5];      /* 各门拦截数 */

    /* 模式检测 */
    int             top_crime;          /* 最高频犯罪类型 */
    float           rejection_rate;     /* 拒绝率 */

    /* 涌现的规则 */
    emerged_rule_t  rules[MAX_RULES];
    int             rule_count;

    /* 锚点分析 */
    int             anchor_violations[9]; /* 各锚点违规次数 */
} court_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 法庭操作
 * ═══════════════════════════════════════════════════════════════════════════ */

static void court_init(court_t *c) {
    memset(c, 0, sizeof(*c));
    c->top_crime = -1;
}

/* 记录一次犯罪 */
static void court_log(court_t *c, crime_type_t type, const char *a, const char *b, int gate) {
    if (!c) return;

    /* 写入环形缓冲区 */
    crime_record_t *r = &c->records[c->record_pos % MAX_RECORDS];
    r->seq = c->record_count;
    r->type = type;
    r->name_a = a;
    r->name_b = b;
    r->gate = gate;
    r->timestamp = c->record_count;

    c->record_pos++;
    c->record_count++;

    /* 更新统计 */
    c->stats[type]++;
    c->total_crimes++;
    if (gate >= 0 && gate < 5) c->gate_stats[gate]++;

    /* 更新最高频 */
    int max_count = -1;
    for (int i = 0; i < CRIME_COUNT; i++) {
        if (c->stats[i] > max_count) {
            max_count = c->stats[i];
            c->top_crime = i;
        }
    }

    /* 更新拒绝率 */
    c->rejection_rate = (float)c->total_crimes / (float)(c->total_crimes + c->total_passed);

    /* 检查是否需要涌现规则 */
    if (c->stats[type] >= EMERGE_THRESHOLD) {
        /* 检查是否已有规则 */
        int has_rule = 0;
        for (int i = 0; i < c->rule_count; i++) {
            if (c->rules[i].active && c->rules[i].target == type) {
                has_rule = 1;
                break;
            }
        }
        if (!has_rule && c->rule_count < MAX_RULES) {
            emerged_rule_t *rule = &c->rules[c->rule_count];
            rule->active = 1;
            rule->target = type;
            rule->gate = gate;
            rule->threshold = EMERGE_THRESHOLD;
            snprintf(rule->desc, sizeof(rule->desc),
                     "[涌现规则#%d] 当「%s」达到%d次, 在第%d门自动拦截",
                     c->rule_count, crime_name(type), EMERGE_THRESHOLD, gate);
            rule->prevented = 0;
            c->rule_count++;
        }
    }
}

/* 记录一次通过 */
static void court_pass(court_t *c) {
    if (!c) return;
    c->total_passed++;
    c->rejection_rate = (float)c->total_crimes / (float)(c->total_crimes + c->total_passed);
}

/* 检查是否有涌现规则可以阻止 */
static int court_check_rules(court_t *c, crime_type_t type) {
    if (!c) return 0;
    for (int i = 0; i < c->rule_count; i++) {
        if (c->rules[i].active && c->rules[i].target == type) {
            c->rules[i].prevented++;
            return 1;
        }
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 法庭报告 (洋流图)
 * ═══════════════════════════════════════════════════════════════════════════ */

static void court_report(court_t *c) {
    if (!c) return;

    printf("\n");
    printf("╔══════════════════════════════════════════════════════════════╗\n");
    printf("║              法 庭 · 犯 罪 笔 录 报 告                      ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║                                                              ║\n");
    printf("║  ■ 总体统计                                                  ║\n");
    printf("║    审判总数: %-5d  通过: %-5d  拒绝: %-5d               ║\n",
           c->total_crimes + c->total_passed, c->total_passed, c->total_crimes);
    printf("║    拒绝率:   %.1f%%                                          ║\n", c->rejection_rate * 100.0f);
    printf("║                                                              ║\n");
    printf("║  ■ 犯罪类型分布 (洋流)                                       ║\n");

    /* 找出最大值用于缩放 */
    int max_stat = 0;
    for (int i = 0; i < CRIME_COUNT; i++)
        if (c->stats[i] > max_stat) max_stat = c->stats[i];

    for (int i = 0; i < CRIME_COUNT; i++) {
        int bar_len = (max_stat > 0) ? (c->stats[i] * 30 / max_stat) : 0;
        printf("║    %-10s [%3d] ", crime_name(i), c->stats[i]);
        for (int b = 0; b < bar_len; b++) printf("█");
        for (int b = bar_len; b < 30; b++) printf(" ");
        if (c->top_crime == i) printf(" ← 最高峰");
        printf("\n");
    }

    printf("║                                                              ║\n");
    printf("║  ■ 各门拦截统计                                              ║\n");
    const char *gate_names[] = {"层级栈", "类型身份", "锚校验", "审美过滤", "编织平衡"};
    for (int i = 0; i < 5; i++) {
        printf("║    第%d门 %-8s: %d\n", i, gate_names[i], c->gate_stats[i]);
    }

    printf("║                                                              ║\n");
    printf("║  ■ 涌现规则 (%d条)                                           ║\n", c->rule_count);
    if (c->rule_count == 0) {
        printf("║    (尚未涌现,继续累积犯罪...)                               ║\n");
    } else {
        for (int i = 0; i < c->rule_count; i++) {
            if (c->rules[i].active) {
                printf("║    %s (已阻止%d次)\n", c->rules[i].desc, c->rules[i].prevented);
            }
        }
    }

    printf("║                                                              ║\n");
    printf("║  ■ 最近犯罪记录 (最多10条)                                   ║\n");
    int start = c->record_count > 10 ? c->record_count - 10 : 0;
    for (int i = start; i < c->record_count; i++) {
        crime_record_t *r = &c->records[i % MAX_RECORDS];
        printf("║    #%d %s: %s↔%s (门%d)\n", r->seq, crime_name(r->type), r->name_a, r->name_b, r->gate);
    }

    printf("║                                                              ║\n");
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 简易审判接口 (为court提供)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7, T_UNKNOWN=0xFF } sym_type_t;
typedef enum { L1=0x10, L2=0x20, L3=0x30, L_UNKNOWN=0xFF } sym_level_t;

#define MAX_R 256
typedef struct { uint32_t count; struct { const char *name; sym_type_t type; sym_level_t level; uint8_t anchor; uint8_t tier; } e[MAX_R]; } simple_reg_t;

static void sr_init(simple_reg_t *r) { memset(r, 0, sizeof(*r)); r->count = 0; }
static void sr_add(simple_reg_t *r, const char *name, sym_type_t type, uint8_t anchor, uint8_t tier) {
    if (!r || !name || r->count >= MAX_R) return;
    sym_level_t lvl = (type == T_TONE) ? L3 : (type == T_MODIFIER) ? L2 : L1;
    r->e[r->count].name = name; r->e[r->count].type = type;
    r->e[r->count].level = lvl; r->e[r->count].anchor = anchor;
    r->e[r->count].tier = tier; r->count++;
}
static sym_type_t sr_type(simple_reg_t *r, const char *n) {
    if (!r || !n) return T_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].type;
    return T_UNKNOWN;
}
static sym_level_t sr_level(simple_reg_t *r, const char *n) {
    if (!r || !n) return L_UNKNOWN;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].level;
    return L_UNKNOWN;
}
static uint8_t sr_anchor(simple_reg_t *r, const char *n) {
    if (!r || !n) return 0xFF;
    for (uint32_t i = 0; i < r->count; i++) if (strcmp(r->e[i].name, n) == 0) return r->e[i].anchor;
    return 0xFF;
}

/* 审判并记录 */
static int judge_and_log(simple_reg_t *reg, court_t *court, const char *a, const char *b) {
    if (!a || !b) return 0;
    sym_type_t ta = sr_type(reg, a), tb = sr_type(reg, b);
    sym_level_t la = sr_level(reg, a), lb = sr_level(reg, b);

    /* 第0门: 层级栈 */
    if (la == L_UNKNOWN || lb == L_UNKNOWN) {
        court_log(court, CRIME_UNKNOWN_INPUT, a, b, 0);
        return 0;
    }
    if (la == L2 || lb == L2) {
        court_log(court, CRIME_TIER_JUMP, a, b, 0);
        return 0;
    }
    if (la != lb) {
        court_log(court, CRIME_TIER_JUMP, a, b, 0);
        return 0;
    }

    /* 第1门: 类型身份 */
    if (ta == T_UNKNOWN || tb == T_UNKNOWN) {
        court_log(court, CRIME_UNKNOWN_INPUT, a, b, 1);
        return 0;
    }
    if (ta == T_EXTRA || tb == T_EXTRA) {
        court_log(court, CRIME_EXTRA_UNCATEGORIZED, a, b, 1);
        return 0;
    }
    if (ta != tb) {
        court_log(court, CRIME_TYPE_MISMATCH, a, b, 1);
        return 0;
    }

    /* 第2门: 锚 */
    uint8_t aa = sr_anchor(reg, a), ab = sr_anchor(reg, b);
    if (ta == T_BASE && aa != ab) {
        court_log(court, CRIME_TYPE_MISMATCH, a, b, 2);
        return 0;
    }
    if (ta == T_TONE) {
        court_log(court, CRIME_TIER_JUMP, a, b, 2);
        return 0;
    }
    if (ta == T_COMPOUND || ta == T_SYMBOL) {
        court_log(court, CRIME_GOAL_DEVIATE, a, b, 2);
        return 0;
    }

    court_pass(court);
    return 1;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 加载编码表
 * ═══════════════════════════════════════════════════════════════════════════ */

static void load_table(simple_reg_t *r) {
    /* 6基韵锚 */
    sr_add(r, "a", T_BASE, 0, 1); sr_add(r, "o", T_BASE, 1, 1);
    sr_add(r, "e", T_BASE, 2, 1); sr_add(r, "i", T_BASE, 3, 1);
    sr_add(r, "u", T_BASE, 4, 1); sr_add(r, "ü", T_BASE, 5, 1);
    /* 24声调 */
    sr_add(r, "ā", T_TONE, 6, 3); sr_add(r, "á", T_TONE, 6, 3); sr_add(r, "ǎ", T_TONE, 6, 3); sr_add(r, "à", T_TONE, 6, 3);
    sr_add(r, "ō", T_TONE, 6, 3); sr_add(r, "ó", T_TONE, 6, 3); sr_add(r, "ǒ", T_TONE, 6, 3); sr_add(r, "ò", T_TONE, 6, 3);
    sr_add(r, "ē", T_TONE, 6, 3); sr_add(r, "é", T_TONE, 6, 3); sr_add(r, "ě", T_TONE, 6, 3); sr_add(r, "è", T_TONE, 6, 3);
    sr_add(r, "ī", T_TONE, 6, 3); sr_add(r, "í", T_TONE, 6, 3); sr_add(r, "ǐ", T_TONE, 6, 3); sr_add(r, "ì", T_TONE, 6, 3);
    sr_add(r, "ū", T_TONE, 6, 3); sr_add(r, "ú", T_TONE, 6, 3); sr_add(r, "ǔ", T_TONE, 6, 3); sr_add(r, "ù", T_TONE, 6, 3);
    sr_add(r, "ǖ", T_TONE, 6, 3); sr_add(r, "ǘ", T_TONE, 6, 3); sr_add(r, "ǚ", T_TONE, 6, 3); sr_add(r, "ǜ", T_TONE, 6, 3);
    /* 28复韵 */
    sr_add(r, "ai", T_COMPOUND, 7, 2); sr_add(r, "ei", T_COMPOUND, 7, 2); sr_add(r, "ui", T_COMPOUND, 7, 2);
    sr_add(r, "ao", T_COMPOUND, 7, 2); sr_add(r, "ou", T_COMPOUND, 7, 2); sr_add(r, "iu", T_COMPOUND, 7, 2);
    sr_add(r, "ie", T_COMPOUND, 7, 2); sr_add(r, "üe", T_COMPOUND, 7, 2); sr_add(r, "er", T_COMPOUND, 7, 2);
    sr_add(r, "an", T_COMPOUND, 7, 2); sr_add(r, "en", T_COMPOUND, 7, 2); sr_add(r, "in", T_COMPOUND, 7, 2);
    sr_add(r, "un", T_COMPOUND, 7, 2); sr_add(r, "ün", T_COMPOUND, 7, 2); sr_add(r, "ang", T_COMPOUND, 7, 2);
    sr_add(r, "eng", T_COMPOUND, 7, 2); sr_add(r, "ing", T_COMPOUND, 7, 2); sr_add(r, "ong", T_COMPOUND, 7, 2);
    sr_add(r, "ia", T_COMPOUND, 7, 2); sr_add(r, "ian", T_COMPOUND, 7, 2); sr_add(r, "iang", T_COMPOUND, 7, 2);
    sr_add(r, "iong", T_COMPOUND, 7, 2); sr_add(r, "ua", T_COMPOUND, 7, 2); sr_add(r, "uo", T_COMPOUND, 7, 2);
    sr_add(r, "uai", T_COMPOUND, 7, 2); sr_add(r, "uan", T_COMPOUND, 7, 2); sr_add(r, "uang", T_COMPOUND, 7, 2);
    sr_add(r, "üan", T_COMPOUND, 7, 2);
    /* 溢出符号 */
    sr_add(r, "Ӱ", T_EXTRA, 8, 4); sr_add(r, "ӱ", T_EXTRA, 8, 4);
    sr_add(r, "Ѧ", T_EXTRA, 8, 4); sr_add(r, "ѧ", T_EXTRA, 8, 4);
    sr_add(r, "Ё", T_EXTRA, 8, 4); sr_add(r, "ё", T_EXTRA, 8, 4);
    sr_add(r, "Ӭ", T_EXTRA, 8, 4); sr_add(r, "ӭ", T_EXTRA, 8, 4);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 自检: 演示规则涌现
 * ═══════════════════════════════════════════════════════════════════════════ */

static int run_demo(void) {
    simple_reg_t reg; sr_init(&reg);
    court_t court; court_init(&court);
    load_table(&reg);

    printf("═══ 法庭系统 · 规则涌现演示 ═══\n\n");

    /* 第一批: 触发层级跳跃 */
    printf("[批次1] 触发层级跳跃 (a=á, e=ě, i=í)\n");
    judge_and_log(&reg, &court, "a", "á");
    judge_and_log(&reg, &court, "e", "ě");
    judge_and_log(&reg, &court, "i", "í");
    printf("  层级跳跃=%d次, 规则=%d条\n\n", court.stats[CRIME_TIER_JUMP], court.rule_count);

    /* 第二批: 触发类型不匹配 */
    printf("[批次2] 触发类型不匹配 (a=ai, o=ou)\n");
    judge_and_log(&reg, &court, "a", "ai");
    judge_and_log(&reg, &court, "o", "ou");
    judge_and_log(&reg, &court, "e", "ie");
    printf("  类型不匹配=%d次, 规则=%d条\n\n", court.stats[CRIME_TYPE_MISMATCH], court.rule_count);

    /* 第三批: 触发溢出未归类 */
    printf("[批次3] 触发溢出符号 (Ӱ=ӱ, Ё=ё)\n");
    judge_and_log(&reg, &court, "Ӱ", "ӱ");
    judge_and_log(&reg, &court, "Ё", "ё");
    judge_and_log(&reg, &court, "Ӭ", "ӭ");
    printf("  溢出未归类=%d次, 规则=%d条\n\n", court.stats[CRIME_EXTRA_UNCATEGORIZED], court.rule_count);

    /* 测试涌现规则阻止 */
    printf("[批次4] 测试涌现规则阻止同类犯罪:\n");
    int before = court.rules[0].prevented;
    int blocked = court_check_rules(&court, CRIME_TIER_JUMP);
    printf("  检查层级跳跃规则: %s (已阻止%d次)\n\n", blocked ? "已阻止" : "未阻止", court.rules[0].prevented - before);

    /* 通过案例 */
    printf("[批次5] 合法配对 (不应有犯罪):\n");
    judge_and_log(&reg, &court, "a", "o"); /* 应该拒绝但不同类型锚 */
    printf("  总通过=%d\n\n", court.total_passed);

    /* 输出完整报告 */
    court_report(&court);

    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(simple_reg_t *reg, court_t *court) {
    char line[256], a[64], b[64];
    printf("\n═══ 法庭 · 交互模式 ═══\n");
    printf("  <A> <B>  审判并记录\n");
    printf("  r        法庭报告\n");
    printf("  t        演示规则涌现\n");
    printf("  q        退出\n");
    printf("─────────────────────────────────────────\n");
    while (1) {
        printf("\n⮞ "); fflush(stdout);
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line); while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = 0;
        if (l == 0) continue;
        if (strcmp(line, "q") == 0) break;
        if (strcmp(line, "r") == 0) { court_report(court); continue; }
        if (strcmp(line, "t") == 0) { run_demo(); continue; }
        if (sscanf(line, "%63s %63s", a, b) == 2) {
            int pass = judge_and_log(reg, court, a, b);
            printf("  %s ↔ %s → %s\n", a, b, pass ? "通过" : "拒绝");
        } else printf("  未知命令\n");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    simple_reg_t reg; sr_init(&reg);
    court_t court; court_init(&court);
    load_table(&reg);

    if (argc == 1) return run_demo();
    if (argc == 2 && (strcmp(argv[1], "-i") == 0)) { interactive(&reg, &court); return 0; }
    if (argc == 2 && (strcmp(argv[1], "-t") == 0)) return run_demo();
    if (argc >= 3) {
        int pass = judge_and_log(&reg, &court, argv[1], argv[2]);
        printf("%s ↔ %s → %s\n", argv[1], argv[2], pass ? "通过" : "拒绝");
        return pass ? 0 : 1;
    }
    printf("用法: %s [A B] | -i | -t\n", argv[0]);
    return 0;
}
