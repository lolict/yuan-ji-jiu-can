/**
 * ═══════════════════════════════════════════════════════════════════════════
 * separator.c — 分别器 · 八流统一
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   核心思想: 不同的输入走不同的流, 而不是千篇一律
 *
 * 八流:
 *   ┌─────────────────────────────────────────────────────────────────┐
 *   │ 流0 预加载流: 基韵锚对基韵锚 → 直接放行 (无需审判)             │
 *   │ 流1 解释流: 声调韵相关 → 详细分析声调差异                      │
 *   │ 流2 谐音路由: 复韵母对复韵母 → 谐音相似度计算                  │
 *   │ 流3 主权路由: 配对符号 → 符号衍射分析                          │
 *   │ 流4 溢出处理: 溢出符号 → 集装箱分配                            │
 *   │ 流5 历史回溯: 查犯罪记录 → 洋流报告                            │
 *   │ 流6 自举流: 系统自我检测 → 完整自检                            │
 *   │ 流7 未知流: 无法分类 → 拒绝并记录                              │
 *   └─────────────────────────────────────────────────────────────────┘
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o separator separator.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 类型定义
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7, T_UNKNOWN=0xFF } sym_type_t;
typedef enum { L1=0x10, L2=0x20, L3=0x30, L_UNKNOWN=0xFF } sym_level_t;
typedef enum { PASS=0, REJ_LEVEL=1, REJ_MODIFIER=2, REJ_TONE=3, REJ_EXTRA=4, REJ_UNKNOWN=5, REJ_DIFFERENT=6 } verdict_t;

/* 八流枚举 */
typedef enum {
    FLOW_PRELOAD = 0,   /* 预加载流: 基韵锚对基韵锚 */
    FLOW_INTERPRET,     /* 解释流: 声调韵 */
    FLOW_HARMONY,       /* 谐音路由: 复韵母 */
    FLOW_SOVEREIGN,     /* 主权路由: 配对符号 */
    FLOW_OVERFLOW,      /* 溢出处理 */
    FLOW_HISTORY,       /* 历史回溯 */
    FLOW_BOOTSTRAP,     /* 自举流 */
    FLOW_UNKNOWN        /* 未知流 */
} flow_id_t;

static const char* flow_name(flow_id_t f) {
    switch (f) {
        case FLOW_PRELOAD: return "预加载流(基韵锚配对)";
        case FLOW_INTERPRET: return "解释流(声调分析)";
        case FLOW_HARMONY: return "谐音路由(复韵衍射)";
        case FLOW_SOVEREIGN: return "主权路由(符号衍射)";
        case FLOW_OVERFLOW: return "溢出处理(集装箱)";
        case FLOW_HISTORY: return "历史回溯(洋流报告)";
        case FLOW_BOOTSTRAP: return "自举流(系统自检)";
        case FLOW_UNKNOWN: return "未知流(拒绝记录)";
    }
    return "?";
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编码表 (精简版, 只包含八流分类所需)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct { const char *name; uint8_t code; sym_type_t type; uint8_t anchor; } sep_cell_t;

static const sep_cell_t SEP_TBL[] = {
    {"a",0x02,T_BASE,0}, {"o",0x03,T_BASE,1}, {"e",0x04,T_BASE,2},
    {"i",0x05,T_BASE,3}, {"u",0x06,T_BASE,4}, {"ü",0x07,T_BASE,5},
    {"ā",0x80,T_TONE,6}, {"á",0x81,T_TONE,6}, {"ǎ",0x82,T_TONE,6}, {"à",0x83,T_TONE,6},
    {"ō",0x84,T_TONE,6}, {"ó",0x85,T_TONE,6}, {"ǒ",0x86,T_TONE,6}, {"ò",0x87,T_TONE,6},
    {"ē",0x88,T_TONE,6}, {"é",0x89,T_TONE,6}, {"ě",0x8A,T_TONE,6}, {"è",0x8B,T_TONE,6},
    {"ī",0x8C,T_TONE,6}, {"í",0x8D,T_TONE,6}, {"ǐ",0x8E,T_TONE,6}, {"ì",0x8F,T_TONE,6},
    {"ū",0x90,T_TONE,6}, {"ú",0x91,T_TONE,6}, {"ǔ",0x92,T_TONE,6}, {"ù",0x93,T_TONE,6},
    {"ǖ",0x94,T_TONE,6}, {"ǘ",0x95,T_TONE,6}, {"ǚ",0x96,T_TONE,6}, {"ǜ",0x97,T_TONE,6},
    {"ai",0x10,T_COMPOUND,7}, {"ei",0x11,T_COMPOUND,7}, {"ao",0x13,T_COMPOUND,7},
    {"ou",0x14,T_COMPOUND,7}, {"an",0x19,T_COMPOUND,7}, {"en",0x1A,T_COMPOUND,7},
    {"ang",0x1E,T_COMPOUND,7}, {"eng",0x1F,T_COMPOUND,7}, {"ong",0x21,T_COMPOUND,7},
    {"ꬰ",0x30,T_SYMBOL,7}, {"ə",0x32,T_SYMBOL,7}, {"ѱ",0x34,T_SYMBOL,7},
    {"ȣ",0x38,T_SYMBOL,7}, {"ɤ",0x3C,T_SYMBOL,7}, {"ɛ",0x40,T_SYMBOL,7},
    {"Ӱ",0xA0,T_EXTRA,8}, {"ӱ",0xA1,T_EXTRA,8}, {"Ѧ",0xA2,T_EXTRA,8},
    {"Ё",0xA8,T_EXTRA,8}, {"Ӭ",0xB8,T_EXTRA,8}, {"☁",0xB7,T_EXTRA,8},
};
#define SEP_TBL_SZ ((int)(sizeof(SEP_TBL)/sizeof(SEP_TBL[0])))

static sym_type_t sep_type(const char *n) {
    if (!n) return T_UNKNOWN;
    for (int i = 0; i < SEP_TBL_SZ; i++) if (strcmp(SEP_TBL[i].name, n) == 0) return SEP_TBL[i].type;
    return T_UNKNOWN;
}
static uint8_t sep_anchor(const char *n) {
    if (!n) return 0xFF;
    for (int i = 0; i < SEP_TBL_SZ; i++) if (strcmp(SEP_TBL[i].name, n) == 0) return SEP_TBL[i].anchor;
    return 0xFF;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 流统计
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    int flow_counts[8];         /* 各流处理次数 */
    int total_routed;           /* 总路由数 */
    int total_passed;           /* 总通过数 */
    int total_rejected;         /* 总拒绝数 */
    int current_flow;           /* 当前激活的流 */
} separator_t;

static void sep_init(separator_t *s) { memset(s, 0, sizeof(*s)); }

/* ═══════════════════════════════════════════════════════════════════════════
 * 路由决策: 根据两个符号的类型决定走哪条流
 * ═══════════════════════════════════════════════════════════════════════════ */

static flow_id_t sep_route(separator_t *s, const char *a, const char *b) {
    if (!s || !a || !b) return FLOW_UNKNOWN;
    sym_type_t ta = sep_type(a), tb = sep_type(b);

    flow_id_t flow;
    if (ta == T_UNKNOWN || tb == T_UNKNOWN) {
        flow = FLOW_UNKNOWN;
    } else if (ta == T_BASE && tb == T_BASE) {
        flow = FLOW_PRELOAD;        /* 两个都是基韵锚 → 预加载流 */
    } else if (ta == T_TONE || tb == T_TONE) {
        flow = FLOW_INTERPRET;      /* 涉及声调韵 → 解释流 */
    } else if (ta == T_COMPOUND && tb == T_COMPOUND) {
        flow = FLOW_HARMONY;        /* 两个复韵母 → 谐音路由 */
    } else if (ta == T_SYMBOL || tb == T_SYMBOL) {
        flow = FLOW_SOVEREIGN;      /* 涉及配对符号 → 主权路由 */
    } else if (ta == T_EXTRA || tb == T_EXTRA) {
        flow = FLOW_OVERFLOW;       /* 溢出符号 → 溢出处理 */
    } else {
        flow = FLOW_UNKNOWN;        /* 跨类型 → 未知流 */
    }

    s->flow_counts[flow]++;
    s->total_routed++;
    s->current_flow = flow;
    return flow;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 各流处理函数
 * ═══════════════════════════════════════════════════════════════════════════ */

static int flow_preload(const char *a, const char *b) {
    uint8_t aa = sep_anchor(a), ab = sep_anchor(b);
    printf("  [预加载流] 基韵锚直接比较: 锚%s vs 锚%s → %s\n",
           aa==0?"a":aa==1?"o":aa==2?"e":aa==3?"i":aa==4?"u":aa==5?"ü":"?",
           ab==0?"a":ab==1?"o":ab==2?"e":ab==3?"i":ab==4?"u":ab==5?"ü":"?",
           aa == ab ? "同锚(合法)" : "不同锚(拒绝)");
    return aa == ab ? 1 : 0;
}

static int flow_interpret(const char *a, const char *b) {
    sym_type_t ta = sep_type(a), tb = sep_type(b);
    printf("  [解释流] 声调分析:\n");
    if (ta == T_BASE && tb == T_TONE) {
        printf("    %s(基韵锚, L1) vs %s(声调韵, L3) → 跨层级!\n", a, b);
        printf("    基韵锚是公约数锚, 声调韵是公倍数 → 不可直接配对\n");
        return 0;
    }
    if (ta == T_TONE && tb == T_TONE) {
        printf("    %s vs %s 都是声调韵 → 不可配对(声调韵彼此独立)\n", a, b);
        return 0;
    }
    printf("    %s vs %s → 涉及声调的复杂配对\n", a, b);
    return 0;
}

static int flow_harmony(const char *a, const char *b) {
    uint8_t aa = sep_anchor(a), ab = sep_anchor(b);
    /* 谐音相似度: 共享的基韵锚越多越相似 */
    printf("  [谐音路由] 复韵母衍射分析:\n");
    printf("    %s(锚=%d) vs %s(锚=%d)\n", a, aa, b, ab);
    if (aa == ab && aa == 7) {
        printf("    同一锚点(复韵母层) → 谐音相似度: 高\n");
        printf("    可以形成共振, 但符号层不可直接配对\n");
    } else {
        printf("    不同锚点 → 谐音相似度: 低\n");
    }
    return 0;
}

static int flow_sovereign(const char *a, const char *b) {
    printf("  [主权路由] 配对符号衍射:\n");
    printf("    %s vs %s → 进入符号衍射空间\n", a, b);
    printf("    符号层不直接配对, 需通过衍射找到镜像伴侣\n");
    printf("    衍射条件: 同类型同锚点的大写↔小写\n");
    return 0;
}

static int flow_overflow(const char *a, const char *b) {
    printf("  [溢出处理] 集装箱分配:\n");
    printf("    %s vs %s 是溢出符号 → 进集装箱\n", a, b);
    printf("    集装箱编号: 0/1/2/5 (根据码值模4)\n");
    uint8_t ca = 0, cb = 0;
    for (int i = 0; i < SEP_TBL_SZ; i++) {
        if (strcmp(SEP_TBL[i].name, a) == 0) ca = SEP_TBL[i].code;
        if (strcmp(SEP_TBL[i].name, b) == 0) cb = SEP_TBL[i].code;
    }
    if (ca) printf("    %s(0x%02X) → 集装箱%d\n", a, ca, ca % 4);
    if (cb) printf("    %s(0x%02X) → 集装箱%d\n", b, cb, cb % 4);
    return 0;
}

static int flow_unknown(const char *a, const char *b) {
    printf("  [未知流] 无法分类:\n");
    printf("    %s(类型=%d) vs %s(类型=%d) → 拒绝并记录\n",
           a, sep_type(a), b, sep_type(b));
    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 分别器主函数: 路由 + 调度
 * ═══════════════════════════════════════════════════════════════════════════ */

static int sep_dispatch(separator_t *s, const char *a, const char *b) {
    if (!s || !a || !b) return 0;

    flow_id_t flow = sep_route(s, a, b);
    printf("\n  ↓ 路由至: 流%d (%s)\n", flow, flow_name(flow));

    int result = 0;
    switch (flow) {
        case FLOW_PRELOAD:  result = flow_preload(a, b); break;
        case FLOW_INTERPRET:result = flow_interpret(a, b); break;
        case FLOW_HARMONY:  result = flow_harmony(a, b); break;
        case FLOW_SOVEREIGN:result = flow_sovereign(a, b); break;
        case FLOW_OVERFLOW: result = flow_overflow(a, b); break;
        case FLOW_UNKNOWN:  result = flow_unknown(a, b); break;
        default: result = 0; break;
    }

    if (result) s->total_passed++; else s->total_rejected++;
    return result;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 流报告
 * ═══════════════════════════════════════════════════════════════════════════ */

static void sep_report(separator_t *s) {
    if (!s) return;
    printf("\n");
    printf("╔══════════════════════════════════════════════════════════════╗\n");
    printf("║              分 别 器 · 八 流 统 一 报 告                    ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║  总路由: %-5d  通过: %-5d  拒绝: %-5d                  ║\n",
           s->total_routed, s->total_passed, s->total_rejected);
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║  流分布:                                                     ║\n");
    int max_c = 0;
    for (int i = 0; i < 8; i++) if (s->flow_counts[i] > max_c) max_c = s->flow_counts[i];
    for (int i = 0; i < 8; i++) {
        int bl = (max_c > 0) ? (s->flow_counts[i] * 25 / max_c) : 0;
        printf("║   流%d %-12s [%3d] ", i, flow_name(i), s->flow_counts[i]);
        for (int b = 0; b < bl; b++) printf("█");
        for (int b = bl; b < 25; b++) printf(" ");
        printf("\n");
    }
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 自检
 * ═══════════════════════════════════════════════════════════════════════════ */

static int run_demo(void) {
    separator_t s;
    sep_init(&s);

    printf("═══ 分别器 · 八流统一演示 ═══\n\n");

    /* 流0: 预加载 */
    printf("[测试1] 基韵锚对基韵锚 (预加载流):\n");
    sep_dispatch(&s, "a", "o");
    sep_dispatch(&s, "a", "a");
    printf("\n");

    /* 流1: 解释流 */
    printf("[测试2] 基韵锚对声调韵 (解释流):\n");
    sep_dispatch(&s, "a", "á");
    sep_dispatch(&s, "e", "ě");
    printf("\n");

    /* 流2: 谐音路由 */
    printf("[测试3] 复韵母对复韵母 (谐音路由):\n");
    sep_dispatch(&s, "ai", "ao");
    sep_dispatch(&s, "an", "ang");
    printf("\n");

    /* 流3: 主权路由 */
    printf("[测试4] 配对符号 (主权路由):\n");
    sep_dispatch(&s, "ꬰ", "ə");
    printf("\n");

    /* 流4: 溢出处理 */
    printf("[测试5] 溢出符号 (溢出处理):\n");
    sep_dispatch(&s, "Ӱ", "Ѧ");
    sep_dispatch(&s, "Ё", "☁");
    printf("\n");

    /* 流7: 未知流 */
    printf("[测试6] 未知/未分类 (未知流):\n");
    sep_dispatch(&s, "a", "zzz");
    printf("\n");

    sep_report(&s);
    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(separator_t *s) {
    char line[256], a[64], b[64];
    printf("\n═══ 分别器 · 交互模式 ═══\n");
    printf("  <A> <B>  路由审判\n");
    printf("  r        流统计报告\n");
    printf("  t        八流演示\n");
    printf("  q        退出\n");
    printf("─────────────────────────────────────────\n");
    while (1) {
        printf("\n⮞ "); fflush(stdout);
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line); while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = 0;
        if (l == 0) continue;
        if (strcmp(line, "q") == 0) break;
        if (strcmp(line, "r") == 0) { sep_report(s); continue; }
        if (strcmp(line, "t") == 0) { run_demo(); continue; }
        if (sscanf(line, "%63s %63s", a, b) == 2) {
            int pass = sep_dispatch(s, a, b);
            printf("  → %s\n", pass ? "通过" : "拒绝");
        } else printf("  未知命令\n");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    separator_t s; sep_init(&s);

    if (argc == 1) return run_demo();
    if (argc == 2 && (strcmp(argv[1], "-i") == 0)) { interactive(&s); return 0; }
    if (argc == 2 && (strcmp(argv[1], "-t") == 0)) return run_demo();
    if (argc >= 3) { int p = sep_dispatch(&s, argv[1], argv[2]); return p ? 0 : 1; }
    printf("用法: %s [A B] | -i | -t\n", argv[0]);
    return 0;
}
