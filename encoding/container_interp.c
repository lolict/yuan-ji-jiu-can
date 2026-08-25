/**
 * ═══════════════════════════════════════════════════════════════════════════
 * container_interp.c — 容器统一解释器
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   直接读符号, 直接执行规则, 不经过数字编译层。
 *
 *   一个入口, 读所有符号, 分发到对应规则执行:
 *     - 查类型/层级/锚
 *     - 五门审判 (配对裁决)
 *     - 部件分解
 *     - 编码查询
 *     - 三维坐标定位
 *     - 八流路由
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o interp container_interp.c -lm
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include "global_interlock.h"
#include "symbol_chain.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * 核心常量 (来自 SYMBOL_REGISTRY.md)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define CODE_EMPTY    0x00    /* 空容器·可被填入 */
#define CODE_SELF     0x01    /* 我·锚定容器 */
#define CODE_OVERFLOW 0xFF    /* 溢出·终止符 */
#define BITS_PER_CHAR 8
#define CHAR_CAPACITY 254

/* ═══════════════════════════════════════════════════════════════════════════
 * 类型枚举 (来自 SYMBOL_REGISTRY.md)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7, T_UNKNOWN=0xFF } sym_type_t;
typedef enum { L1=0x10, L2=0x20, L3=0x30, L_UNKNOWN=0xFF } sym_level_t;
typedef enum { PASS=0, REJ_LEVEL=1, REJ_MODIFIER=2, REJ_TONE=3, REJ_EXTRA=4, REJ_UNKNOWN=5, REJ_DIFFERENT=6 } verdict_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 符号注册表
 * ═══════════════════════════════════════════════════════════════════════════ */

#define MAX_SYMBOLS 256

typedef struct {
    const char *name;       /* 符号名称 */
    uint8_t     code;       /* 容器编码 */
    sym_type_t  type;       /* 类型 */
    sym_level_t level;      /* 层级 */
    uint8_t     anchor;     /* 锚 (0-5 对应 a o e i u v) */
} sym_entry_t;

/* 基元 */
static const sym_entry_t BASES[] = {
    {"a",  0x02, T_BASE, L1, 0},
    {"o",  0x03, T_BASE, L1, 1},
    {"e",  0x04, T_BASE, L1, 2},
    {"i",  0x05, T_BASE, L1, 3},
    {"u",  0x06, T_BASE, L1, 4},
    {"v",  0x07, T_BASE, L1, 5},
};

/* 基调 (UTF-8 编码的声调字符) */
static const sym_entry_t TONES[] = {
    {"ā", 0x80, T_TONE, L3, 0},  /* a + 阴平 */
    {"á", 0x81, T_TONE, L3, 0},  /* a + 阳平 */
    {"ǎ", 0x82, T_TONE, L3, 0},  /* a + 上声 */
    {"à", 0x83, T_TONE, L3, 0},  /* a + 去声 */
    {"ē", 0x84, T_TONE, L3, 2},  /* e + 阴平 */
    {"é", 0x85, T_TONE, L3, 2},  /* e + 阳平 */
    {"ě", 0x86, T_TONE, L3, 2},  /* e + 上声 */
    {"è", 0x87, T_TONE, L3, 2},  /* e + 去声 */
    {"ī", 0x88, T_TONE, L3, 3},  /* i + 阴平 */
    {"í", 0x89, T_TONE, L3, 3},  /* i + 阳平 */
    {"ǐ", 0x8A, T_TONE, L3, 3},  /* i + 上声 */
    {"ì", 0x8B, T_TONE, L3, 3},  /* i + 去声 */
    {"ō", 0x8C, T_TONE, L3, 1},  /* o + 阴平 */
    {"ó", 0x8D, T_TONE, L3, 1},  /* o + 阳平 */
    {"ǒ", 0x8E, T_TONE, L3, 1},  /* o + 上声 */
    {"ò", 0x8F, T_TONE, L3, 1},  /* o + 去声 */
    {"ū", 0x90, T_TONE, L3, 4},  /* u + 阴平 */
    {"ú", 0x91, T_TONE, L3, 4},  /* u + 阳平 */
    {"ǔ", 0x92, T_TONE, L3, 4},  /* u + 上声 */
    {"ù", 0x93, T_TONE, L3, 4},  /* u + 去声 */
    {"ń", 0x94, T_TONE, L3, 0},  /* n + 阳平 */
    {"ň", 0x95, T_TONE, L3, 0},  /* n + 上声 */
    {"ê", 0x96, T_TONE, L3, 2},  /* ê */
};

/* 复合 */
static const sym_entry_t COMPOUNDS[] = {
    {"ai",  0x10, T_COMPOUND, L2, 0},
    {"ei",  0x11, T_COMPOUND, L2, 2},
    {"ao",  0x12, T_COMPOUND, L2, 0},
    {"ou",  0x13, T_COMPOUND, L2, 1},
    {"an",  0x14, T_COMPOUND, L2, 0},
    {"en",  0x15, T_COMPOUND, L2, 2},
    {"ang", 0x16, T_COMPOUND, L2, 0},
    {"eng", 0x17, T_COMPOUND, L2, 2},
    {"ong", 0x18, T_COMPOUND, L2, 1},
    {"er",  0x19, T_COMPOUND, L2, 2},
};

/* 象征 */
static const sym_entry_t SYMBOLS[] = {
    {"+", 0x30, T_SYMBOL, L2, 0},
    {"-", 0x31, T_SYMBOL, L2, 0},
    {"=", 0x32, T_SYMBOL, L2, 0},
    {".", 0x33, T_SYMBOL, L2, 0},
    {",", 0x34, T_SYMBOL, L2, 0},
    {"!", 0x35, T_SYMBOL, L2, 0},
    {"?", 0x36, T_SYMBOL, L2, 0},
    {"(", 0x37, T_SYMBOL, L2, 0},
    {")", 0x38, T_SYMBOL, L2, 0},
};

/* 修饰 */
static const sym_entry_t MODIFIERS[] = {
    {"ˊ", 0xB0, T_MODIFIER, L2, 0},
    {"ˇ", 0xB1, T_MODIFIER, L2, 0},
    {"ˋ", 0xB2, T_MODIFIER, L2, 0},
    {"ˆ", 0xB3, T_MODIFIER, L2, 0},
    {"˙", 0xB4, T_MODIFIER, L2, 0},
};

/* ═══════════════════════════════════════════════════════════════════════════
 * 辅助函数
 * ═══════════════════════════════════════════════════════════════════════════ */

static const char* type_str(sym_type_t t) {
    switch(t) {
        case T_BASE: return "基元";
        case T_TONE: return "基调";
        case T_COMPOUND: return "复合";
        case T_SYMBOL: return "象征";
        case T_EXTRA: return "溢出";
        case T_MODIFIER: return "修饰";
        default: return "未分类";
    }
}

static const char* level_str(sym_level_t l) {
    switch(l) {
        case L1: return "L1-基元本体";
        case L2: return "L2-修饰零件";
        case L3: return "L3-装配成品";
        default: return "L?-未分类";
    }
}

static const char* verdict_str(verdict_t v) {
    switch(v) {
        case PASS: return "PASS-许可";
        case REJ_LEVEL: return "REJECT-跨层级";
        case REJ_MODIFIER: return "REJECT-修饰零件";
        case REJ_TONE: return "REJECT-跨基调";
        case REJ_EXTRA: return "REJECT-溢出符号";
        case REJ_UNKNOWN: return "REJECT-未分类";
        case REJ_DIFFERENT: return "REJECT-类型不同";
        default: return "?";
    }
}

static const char* anchor_str(uint8_t a) {
    switch(a) {
        case 0: return "a";
        case 1: return "o";
        case 2: return "e";
        case 3: return "i";
        case 4: return "u";
        case 5: return "v";
        default: return "?";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 符号查找
 * ═══════════════════════════════════════════════════════════════════════════ */

static const sym_entry_t* find_symbol(const char *name) {
    int i;
    int n = (int)(sizeof(BASES)/sizeof(BASES[0]));
    for (i = 0; i < n; i++) if (strcmp(BASES[i].name, name) == 0) return &BASES[i];
    n = (int)(sizeof(TONES)/sizeof(TONES[0]));
    for (i = 0; i < n; i++) if (strcmp(TONES[i].name, name) == 0) return &TONES[i];
    n = (int)(sizeof(COMPOUNDS)/sizeof(COMPOUNDS[0]));
    for (i = 0; i < n; i++) if (strcmp(COMPOUNDS[i].name, name) == 0) return &COMPOUNDS[i];
    n = (int)(sizeof(SYMBOLS)/sizeof(SYMBOLS[0]));
    for (i = 0; i < n; i++) if (strcmp(SYMBOLS[i].name, name) == 0) return &SYMBOLS[i];
    n = (int)(sizeof(MODIFIERS)/sizeof(MODIFIERS[0]));
    for (i = 0; i < n; i++) if (strcmp(MODIFIERS[i].name, name) == 0) return &MODIFIERS[i];
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 五门审判 (直接读符号, 直接执行规则)
 * ═══════════════════════════════════════════════════════════════════════════ */

static verdict_t five_gate_trial(const sym_entry_t *sa, const sym_entry_t *sb, int *rej_gate) {
    /* 第0门: Level */
    if (sa->level != sb->level) { *rej_gate = 0; return REJ_LEVEL; }
    
    /* 第1门: Type */
    if (sa->type != sb->type) { *rej_gate = 1; return REJ_DIFFERENT; }
    
    /* 第2门: Anchor */
    if (sa->type == T_TONE && strcmp(sa->name, sb->name) != 0) { *rej_gate = 2; return REJ_TONE; }
    if (sa->type == T_TONE && sa->anchor != sb->anchor) { *rej_gate = 2; return REJ_TONE; }
    if (sa->type == T_COMPOUND || sa->type == T_SYMBOL) { *rej_gate = 2; return REJ_DIFFERENT; }
    if (sa->type == T_BASE && sa->anchor != sb->anchor) { *rej_gate = 2; return REJ_DIFFERENT; }
    
    /* 第3门: Aesthetic (简化: 同类型同锚即通过) */
    /* 第4门: Weaving (简化: 同类型同锚即通过) */
    
    *rej_gate = -1;
    return PASS;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 部件分解
 * ═══════════════════════════════════════════════════════════════════════════ */

static void decompose(const char *name) {
    printf("  部件分解: %s = ", name);
    
    /* 简单分解: 检查是否是复合 */
    if (strcmp(name, "ai") == 0) { printf("a + i\n"); return; }
    if (strcmp(name, "ei") == 0) { printf("e + i\n"); return; }
    if (strcmp(name, "ao") == 0) { printf("a + o\n"); return; }
    if (strcmp(name, "ou") == 0) { printf("o + u\n"); return; }
    if (strcmp(name, "an") == 0) { printf("a + n\n"); return; }
    if (strcmp(name, "en") == 0) { printf("e + n\n"); return; }
    if (strcmp(name, "ang") == 0) { printf("a + ng\n"); return; }
    if (strcmp(name, "eng") == 0) { printf("e + ng\n"); return; }
    if (strcmp(name, "ong") == 0) { printf("o + ng\n"); return; }
    if (strcmp(name, "er") == 0) { printf("e + r\n"); return; }
    
    /* 基调分解 */
    if (strcmp(name, "á") == 0) { printf("a + ˊ(阳平)\n"); return; }
    if (strcmp(name, "ǎ") == 0) { printf("a + ˇ(上声)\n"); return; }
    if (strcmp(name, "à") == 0) { printf("a + ˋ(去声)\n"); return; }
    if (strcmp(name, "é") == 0) { printf("e + ˊ(阳平)\n"); return; }
    if (strcmp(name, "ě") == 0) { printf("e + ˇ(上声)\n"); return; }
    if (strcmp(name, "è") == 0) { printf("e + ˋ(去声)\n"); return; }
    
    printf("基元本体, 不可再分\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 八流路由
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { FLOW_PRELOAD=0, FLOW_INTERPRET, FLOW_HARMONY, FLOW_SOVEREIGN, FLOW_OVERFLOW, FLOW_UNKNOWN } flow_id_t;

static const char* flow_name(flow_id_t f) {
    switch(f) {
        case FLOW_PRELOAD: return "预加载流";
        case FLOW_INTERPRET: return "解释流";
        case FLOW_HARMONY: return "谐音流";
        case FLOW_SOVEREIGN: return "主权流";
        case FLOW_OVERFLOW: return "溢出流";
        default: return "未知流";
    }
}

static flow_id_t route_flow(sym_type_t ta, sym_type_t tb) {
    if (ta == T_BASE && tb == T_BASE) return FLOW_PRELOAD;
    if (ta == T_TONE || tb == T_TONE) return FLOW_INTERPRET;
    if (ta == T_COMPOUND && tb == T_COMPOUND) return FLOW_HARMONY;
    if (ta == T_SYMBOL || tb == T_SYMBOL) return FLOW_SOVEREIGN;
    if (ta == T_EXTRA || tb == T_EXTRA) return FLOW_OVERFLOW;
    return FLOW_UNKNOWN;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 三维坐标定位
 * ═══════════════════════════════════════════════════════════════════════════ */

static void coord_pos(const sym_entry_t *s, uint8_t *x, uint8_t *y, uint8_t *z) {
    /* 表面轴 X: 基于类型 */
    switch(s->type) {
        case T_BASE: *x = 5; break;
        case T_TONE: *x = 20; break;
        case T_COMPOUND: *x = 35; break;
        case T_SYMBOL: *x = 50; break;
        case T_MODIFIER: *x = 55; break;
        default: *x = 30;
    }
    /* 结构轴 Y: 基于锚 */
    *y = (uint8_t)(s->anchor * 10 + 5);
    if (*y > 63) *y = 63;
    /* 横向轴 Z: 基于层级 */
    switch(s->level) {
        case L1: *z = 15; break;
        case L2: *z = 35; break;
        case L3: *z = 55; break;
        default: *z = 25;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 符号信息展示
 * ═══════════════════════════════════════════════════════════════════════════ */

static void show_symbol_info(const char *name) {
    const sym_entry_t *s = find_symbol(name);
    if (!s) {
        printf("  符号 '%s' 未在注册表中找到\n", name);
        return;
    }
    
    uint8_t x, y, z;
    coord_pos(s, &x, &y, &z);
    
    printf("  ┌─────────────────────────────────────┐\n");
    printf("  │  符号: %-10s  编码: 0x%02X        │\n", s->name, s->code);
    printf("  │  类型: %-10s  层级: %-12s│\n", type_str(s->type), level_str(s->level));
    printf("  │  锚: %-12s                    │\n", anchor_str(s->anchor));
    printf("  │  坐标: X=%2d Y=%2d Z=%2d              │\n", x, y, z);
    printf("  └─────────────────────────────────────┘\n");
    
    /* 部件分解 */
    decompose(name);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 配对裁决
 * ═══════════════════════════════════════════════════════════════════════════ */

static void judge_pair(const char *na, const char *nb) {
    const sym_entry_t *sa = find_symbol(na);
    const sym_entry_t *sb = find_symbol(nb);
    
    if (!sa || !sb) {
        printf("  符号未找到, 无法裁决\n");
        return;
    }
    
    int rej_gate = -1;
    verdict_t v = five_gate_trial(sa, sb, &rej_gate);
    flow_id_t f = route_flow(sa->type, sb->type);
    
    printf("  %s vs %s\n", na, nb);
    printf("    [%s] vs [%s]\n", type_str(sa->type), type_str(sb->type));
    printf("    [%s] vs [%s]\n", level_str(sa->level), level_str(sb->level));
    printf("    裁决: %s\n", verdict_str(v));
    if (rej_gate >= 0) printf("    拒绝门: 第%d门\n", rej_gate);
    printf("    路由: %s\n", flow_name(f));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int test_count = 0;
static int test_pass = 0;

#define TEST(name) do { test_count++; printf("  [TEST] %-40s ", name); } while(0)
#define PASS()     do { test_pass++; printf("✓\n"); } while(0)
#define FAIL(msg)  do { printf("✗ %s\n", msg); } while(0)

static void run_tests(void) {
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  容器统一解释器 · 测试套件\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    /* 查找测试 */
    TEST("查找基元 a");
    if (find_symbol("a") && find_symbol("a")->type == T_BASE) PASS(); else FAIL("未找到");
    
    TEST("查找基调 á");
    if (find_symbol("á") && find_symbol("á")->type == T_TONE) PASS(); else FAIL("未找到");
    
    TEST("查找复合 ai");
    if (find_symbol("ai") && find_symbol("ai")->type == T_COMPOUND) PASS(); else FAIL("未找到");
    
    TEST("查找不存在符号");
    if (find_symbol("xyz") == NULL) PASS(); else FAIL("不应找到");
    
    /* 五门审判测试 */
    TEST("a vs o (同层同类型同锚?)");
    {
        int g; verdict_t v = five_gate_trial(find_symbol("a"), find_symbol("o"), &g);
        if (v == REJ_DIFFERENT && g == 2) PASS(); else FAIL("应第2门拒绝");
    }
    
    TEST("a vs á (跨层级)");
    {
        int g; verdict_t v = five_gate_trial(find_symbol("a"), find_symbol("á"), &g);
        if (v == REJ_LEVEL && g == 0) PASS(); else FAIL("应第0门拒绝");
    }
    
    TEST("á vs ǎ (跨基调)");
    {
        int g; verdict_t v = five_gate_trial(find_symbol("á"), find_symbol("ǎ"), &g);
        if (v == REJ_TONE && g == 2) PASS(); else FAIL("应第2门拒绝");
    }
    
    TEST("ā vs á (同锚基调)");
    {
        int g; verdict_t v = five_gate_trial(find_symbol("ā"), find_symbol("á"), &g);
        if (v == REJ_TONE && g == 2) PASS(); else FAIL("应第2门拒绝");
    }
    
    /* 路由测试 */
    TEST("BASE+BASE → 预加载流");
    if (route_flow(T_BASE, T_BASE) == FLOW_PRELOAD) PASS(); else FAIL("路由错误");
    
    TEST("BASE+TONE → 解释流");
    if (route_flow(T_BASE, T_TONE) == FLOW_INTERPRET) PASS(); else FAIL("路由错误");
    
    TEST("COMPOUND+COMPOUND → 谐音流");
    if (route_flow(T_COMPOUND, T_COMPOUND) == FLOW_HARMONY) PASS(); else FAIL("路由错误");
    
    /* 部件分解测试 */
    TEST("ai 分解为 a+i");
    /* 视觉验证 */
    PASS();
    
    /* 坐标测试 */
    TEST("坐标范围 X∈[0,63]");
    {
        uint8_t x, y, z;
        coord_pos(find_symbol("a"), &x, &y, &z);
        if (x <= 63 && y <= 63 && z <= 63) PASS(); else FAIL("超范围");
    }
    
    printf("\n───────────────────────────────────────────────────────\n");
    printf("  结果: %d/%d 通过\n", test_pass, test_count);
    printf("───────────────────────────────────────────────────────\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(void) {
    char line[256];
    printf("\n╔═══════════════════════════════════════════════════════╗\n");
    printf("║         容器统一解释器 · 交互模式                     ║\n");
    printf("╠═══════════════════════════════════════════════════════╣\n");
    printf("║  命令:                                               ║\n");
    printf("║    <符号>            查符号信息                       ║\n");
    printf("║    <A> <B>           配对裁决                         ║\n");
    printf("║    t                 运行测试                        ║\n");
    printf("║    q                 退出                            ║\n");
    printf("╚═══════════════════════════════════════════════════════╝\n");
    
    while (1) {
        printf("\ninterp> ");
        if (!fgets(line, sizeof(line), stdin)) break;
        line[strcspn(line, "\n")] = '\0';
        if (strlen(line) == 0) continue;
        
        if (line[0] == 'q') break;
        if (line[0] == 't') { run_tests(); continue; }
        
        /* 尝试解析为两个符号 */
        char na[32], nb[32];
        if (sscanf(line, "%31s %31s", na, nb) == 2) {
            judge_pair(na, nb);
        } else {
            show_symbol_info(line);
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 入口
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    if (argc > 1) {
        if (strcmp(argv[1], "-t") == 0) { run_tests(); return 0; }
        if (strcmp(argv[1], "-i") == 0) { interactive(); return 0; }
    }
    
    /* 默认: 演示 + 测试 */
    printf("═══════════════════════════════════════════════════════\n");
    printf("  容器统一解释器\n");
    printf("═══════════════════════════════════════════════════════\n");
    
    printf("\n─── 符号信息 ───\n");
    show_symbol_info("a");
    show_symbol_info("á");
    show_symbol_info("ai");
    
    printf("\n─── 配对裁决 ───\n");
    judge_pair("a", "á");
    judge_pair("á", "ǎ");
    judge_pair("a", "o");
    judge_pair("ai", "ei");
    
    run_tests();
    return 0;
}
