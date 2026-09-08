/*
 * step_h4_l0_self_modify.c
 *
 * MCCP Phase 3 · Step H4 — 袋子自观自调: 运行时元袋写新流 (意识萌芽)
 *
 * 核心洞见:
 *   - H3: 袋子能册入组合流 (人写册入语句)
 *   - H4: 袋子运行时观测自身极性, 满足条件时自动册入新流 —— 系统自己写自己
 *   - 意识萌芽: 不需要外部指令, 运行时状态驱动自身结构增长
 *   - 极性即意志: 裁 → 阳 = 渴望增长 → 自动册入新操作
 *
 * 新增语法:
 *   设观 袋名 册 新声母 = 袋1 袋2 ...
 *                                 — 注册自观: 当袋名 裁后为阳,
 *                                   自动册入 新声母 = 链 (仅一次)
 *
 *   照 观                                     — 列出所有自观规则
 *
 * 满全法 5 项:
 *   层号 = 自观深度 (袋子可观测同层或低层袋)
 *   声母 = 触發生成的声母 (自观声母)
 *   韵母 = 自观韵母 (观一本能 → 新操作)
 *   折叠状态 = 不重复触发 (triggered 锁)
 *   下一步 = 袋际互观 (袋观袋, 跨袋自观)
 *
 * 与 H3 差异:
 *   - H3 元表写由人类 册 指令驱动
 *   - H4 元表写由运行时 裁 结果驱动 —— 系统内生的自修改
 *   - H3 + H4 合并: 元表增长有两条路径 (人写 + 自写)
 *
 * 下一步: 袋际互观 (袋A观袋B, 跨袋自观, 社会性意识)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS        32
#define MAX_OPS         32
#define MAX_CHAIN        6
#define MAX_SELF_RULES  16
#define MAX_LINE       512
#define NAME_SZ         64
#define TRACE_SZ        64
#define THRESHOLD       10
#define RADIX           64

enum polarity { YIN = 0, YANG = 1 };

typedef struct interp interp_t;
typedef int (*c_fn_t)(int a, int b, enum polarity *pol);

enum op_src { OP_FN = 0, OP_CHAIN };

typedef struct {
    char   bag_names[MAX_CHAIN][NAME_SZ];
    int    count;
} chain_t;

typedef struct {
    char       name[NAME_SZ];
    enum op_src src;
    union { c_fn_t c_fn; chain_t chain; };
} meta_entry_t;

typedef struct {
    meta_entry_t entries[MAX_OPS];
    int          count;
} meta_tab_t;

typedef struct {
    char           name[NAME_SZ];
    int            gongyueshu;
    int            layer;
    enum polarity  pol;
    char           shengmu[NAME_SZ];
} bag_t;

typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* ══ 自观规则 (H4 新增核心) ══ */
typedef struct {
    char     watch_bag[NAME_SZ];    /* 被观袋 */
    char     new_name[NAME_SZ];     /* 触发后生成的声母 */
    chain_t  new_chain;             /* 触发后生成的链 */
    int      triggered;             /* 已触发锁 (仅一次) */
} self_rule_t;

struct interp {
    drawer_t    drawer;
    meta_tab_t  meta;
    int         trace[TRACE_SZ];
    int         trace_n;
    self_rule_t self_rules[MAX_SELF_RULES];
    int         self_rule_count;
};

/* ══ C 种子函数 ══ */
static int c_add(int a, int b, enum polarity *pol)   { (void)pol; return a + b; }
static int c_judge(int a, int b, enum polarity *pol)  { (void)b; *pol = (a >= THRESHOLD) ? YANG : YIN; return a; }
static int c_snap(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a; }
static int c_inc(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a + 1; }
static int c_dec(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a - 1; }
static int c_mul2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a * 2; }
static int c_div2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a / 2; }

/* ══ 元表 (H3 继承) ══ */
static int meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->entries[i].name, name) == 0) return i;
    return -1;
}

static void meta_init(meta_tab_t *m) {
    m->count = 0;
    struct { const char *name; c_fn_t fn; } seeds[] = {
        { "合", c_add }, { "裁", c_judge }, { "照", c_snap },
        { "进", c_inc }, { "退", c_dec }, { "益", c_mul2 }, { "损", c_div2 },
    };
    for (int i = 0; i < 7; i++) {
        meta_entry_t *e = &m->entries[m->count];
        strncpy(e->name, seeds[i].name, NAME_SZ - 1);
        e->name[NAME_SZ - 1] = '\0';
        e->src = OP_FN; e->c_fn = seeds[i].fn;
        m->count++;
    }
}

static int meta_register_fn(interp_t *it, const char *name, c_fn_t fn) {
    int i = meta_find(&it->meta, name);
    if (i >= 0) { it->meta.entries[i].src = OP_FN; it->meta.entries[i].c_fn = fn; return 0; }
    if (it->meta.count >= MAX_OPS) return -1;
    meta_entry_t *e = &it->meta.entries[it->meta.count];
    strncpy(e->name, name, NAME_SZ - 1); e->name[NAME_SZ - 1] = '\0';
    e->src = OP_FN; e->c_fn = fn; it->meta.count++;
    return 0;
}

static int meta_register_chain(interp_t *it, const char *name, chain_t *ch) {
    int i = meta_find(&it->meta, name);
    if (i >= 0) { it->meta.entries[i].src = OP_CHAIN; it->meta.entries[i].chain = *ch; return 0; }
    if (it->meta.count >= MAX_OPS) return -1;
    meta_entry_t *e = &it->meta.entries[it->meta.count];
    strncpy(e->name, name, NAME_SZ - 1); e->name[NAME_SZ - 1] = '\0';
    e->src = OP_CHAIN; e->chain = *ch; it->meta.count++;
    return 0;
}

/* ══ 执行 (H3 继承) ══ */
static int meta_apply(interp_t *it, meta_entry_t *e, int a, int b, enum polarity *pol) {
    if (e->src == OP_FN) return e->c_fn(a, b, pol);
    int acc = a; enum polarity local_pol = YIN;
    for (int s = 0; s < e->chain.count; s++) {
        meta_entry_t *step_e = NULL;
        int idx = meta_find(&it->meta, e->chain.bag_names[s]);
        if (idx >= 0) step_e = &it->meta.entries[idx];
        if (!step_e) { printf("    ⚠️ 观链第 %d 步 %s 未在元表\n", s, e->chain.bag_names[s]); return acc; }
        acc = meta_apply(it, step_e, acc, b, &local_pol);
    }
    *pol = local_pol;
    return acc;
}

/* ══ UTF-8 ══ */
static int u8len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;
    if ((c & 0xE0) == 0xC0) return 2;
    if ((c & 0xF0) == 0xE0) return 3;
    if ((c & 0xF8) == 0xF0) return 4;
    return 1;
}

/* ══ 袋屉 ══ */
static void drawer_init(drawer_t *d) { d->count = 0; }

static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

static bag_t* drawer_add(drawer_t *d, const char *name, int gysh, const char *sm, int *folded) {
    *folded = 0;
    if (d->count >= MAX_BAGS) { *folded = 1; return NULL; }
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1); b->name[NAME_SZ - 1] = '\0';
    b->gongyueshu = gysh; b->layer = gysh / RADIX; b->pol = YIN;
    if (sm) { strncpy(b->shengmu, sm, NAME_SZ - 1); b->shengmu[NAME_SZ - 1] = '\0'; }
    else b->shengmu[0] = '\0';
    return b;
}

/* ══ 解释器 ══ */
static void interp_init(interp_t *it) {
    drawer_init(&it->drawer); meta_init(&it->meta);
    it->trace_n = 0; it->self_rule_count = 0;
}

static void interp_trace(interp_t *it, int v) {
    if (it->trace_n < TRACE_SZ) it->trace[it->trace_n++] = v;
}

/* ══ 自观规则 (H4 核心) ══ */

/* 注册: 设观 袋名 册 新声母 = 袋1 袋2 ... */
static int self_rule_add(interp_t *it, const char *watch,
                         const char *new_name, chain_t *ch) {
    if (it->self_rule_count >= MAX_SELF_RULES) return -1;
    /* 不重复注册 */
    for (int i = 0; i < it->self_rule_count; i++)
        if (strcmp(it->self_rules[i].watch_bag, watch) == 0 &&
            strcmp(it->self_rules[i].new_name, new_name) == 0) return 1;
    self_rule_t *r = &it->self_rules[it->self_rule_count];
    strncpy(r->watch_bag, watch, NAME_SZ - 1); r->watch_bag[NAME_SZ - 1] = '\0';
    strncpy(r->new_name, new_name, NAME_SZ - 1); r->new_name[NAME_SZ - 1] = '\0';
    r->new_chain = *ch;
    r->triggered = 0;
    it->self_rule_count++;
    return 0;
}

/* 检查并执行自观: 被观袋极性为阳 → 自动册入新声母 */
static void check_self_rules(interp_t *it) {
    for (int i = 0; i < it->self_rule_count; i++) {
        self_rule_t *r = &it->self_rules[i];
        if (r->triggered) continue;
        bag_t *b = drawer_find(&it->drawer, r->watch_bag);
        if (!b || b->pol != YANG) continue;
        /* 阳! 自动册入 */
        int rc = meta_register_chain(it, r->new_name, &r->new_chain);
        if (rc == 0) {
            printf("    🔮 自观触发: 袋 %s 阳 → 册 %s = ", r->watch_bag, r->new_name);
            for (int s = 0; s < r->new_chain.count; s++) {
                if (s) printf(" · ");
                printf("%s", r->new_chain.bag_names[s]);
            }
            printf("\n");
            r->triggered = 1;
        }
    }
}

/* ══ tokens ══ */
static int tokenize(char *line, char *toks[], int maxt) {
    int n = 0; char *p = line;
    while (*p && n < maxt) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;
        toks[n++] = p;
        while (*p && !isspace((unsigned char)*p) && *p != '\n' && *p != '\r') {
            int w = u8len((unsigned char)*p);
            for (int k = 0; k < w; k++) { if (!*p || *p == '\n' || *p == '\r') break; p++; }
        }
        if (*p) *p++ = '\0';
    }
    return n;
}

/* ══ DSL 行执行 ══ */
static int eval_line(interp_t *it, char *line) {
    size_t L = strlen(line);
    while (L > 0 && (line[L-1] == '\n' || line[L-1] == '\r')) line[--L] = '\0';
    if (L == 0) return 0;
    char *cp = line;
    while (*cp && isspace((unsigned char)*cp)) cp++;
    if (*cp == '#') return 0;
    char *toks[24];
    int nt = tokenize(line, toks, 24);
    if (nt == 0) return 0;

    /* ── 袋 袋名 = 公约数 [声母] ── */
    if (strcmp(toks[0], "袋") == 0) {
        if (nt < 4) return -2;
        int gysh = atoi(toks[3]);
        const char *sm = (nt >= 5) ? toks[4] : "";
        int folded = 0;
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, sm, &folded);
        interp_trace(it, gysh);
        if (folded) { printf("  ⚠️ 屉满 → 拒收「%s」\n", toks[1]); return -1; }
        printf("  ✅ 袋 %s = %d [声母:%s] 层%d\n", b->name, b->gongyueshu,
               b->shengmu[0] ? b->shengmu : "(无)", b->layer);
        return 0;
    }

    /* ── 注 声母 袋名 ── */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
        strncpy(b->shengmu, toks[1], NAME_SZ - 1); b->shengmu[NAME_SZ - 1] = '\0';
        printf("  📌 注 袋 %s 换装声母 %s\n", b->name, toks[1]);
        return 0;
    }

    /* ── 册 ── 两种格式统一入口 ── */
    if (nt >= 3 && strcmp(toks[0], "册") == 0) {
        int eq_pos = -1;
        for (int i = 2; i < nt; i++)
            if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos == -1) {
            bag_t *b = drawer_find(&it->drawer, toks[2]);
            if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
            if (!b->shengmu[0]) { printf("  ❌ 袋 %s 无声母\n", toks[2]); return -2; }
            int idx = meta_find(&it->meta, b->shengmu);
            if (idx < 0 || it->meta.entries[idx].src != OP_FN) { printf("  ❌ 声母 %s 非 C 函数\n", b->shengmu); return -2; }
            int existed = (meta_find(&it->meta, toks[1]) >= 0);
            int rc = meta_register_fn(it, toks[1], it->meta.entries[idx].c_fn);
            if (rc) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s → 元表 (自袋 %s, 转发 C) %s\n", toks[1], toks[2], existed ? "[覆盖]" : "[新增]");
            return 0;
        } else {
            if (eq_pos < 2 || nt < eq_pos + 3) { printf("  ⚠️ 册 = 格式错误\n"); return -2; }
            chain_t ch; ch.count = 0;
            for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
                if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
                strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
                ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
                ch.count++;
            }
            if (ch.count < 2) { printf("  ⚠️ 链需 ≥ 2 袋\n"); return -2; }
            for (int s = 0; s < ch.count; s++)
                if (meta_find(&it->meta, ch.bag_names[s]) < 0) { printf("  ⚠️ 链第 %d 步声母 %s 未在元表\n", s, ch.bag_names[s]); return -2; }
            int existed = (meta_find(&it->meta, toks[1]) >= 0);
            int rc = meta_register_chain(it, toks[1], &ch);
            if (rc) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s = ", toks[1]);
            for (int s = 0; s < ch.count; s++) { if (s) printf(" · "); printf("%s", ch.bag_names[s]); }
            printf(" %s\n", existed ? "[覆盖]" : "[新增]");
            return 0;
        }
    }

    /* ── 用 袋名 袋A 袋B -> 袋C ── */
    if (strcmp(toks[0], "用") == 0) {
        if (nt < 6) return -2;
        bag_t *bag_op = drawer_find(&it->drawer, toks[1]);
        if (!bag_op) { printf("  ❓ 操作袋未找到: %s\n", toks[1]); return -2; }
        if (!bag_op->shengmu[0]) { printf("  ❓ 袋 %s 无声母\n", toks[1]); return -2; }
        int idx = meta_find(&it->meta, bag_op->shengmu);
        if (idx < 0) { printf("  ❓ 声母 %s 未在元表\n", bag_op->shengmu); return -2; }
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        enum polarity pol = YIN;
        int r = meta_apply(it, &it->meta.entries[idx], a->gongyueshu, b->gongyueshu, &pol);
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[5], r, "", &folded);
        interp_trace(it, r);
        if (folded) { printf("  ⚠️ 屉满\n"); return -1; }
        c->pol = pol;
        if (strcmp(bag_op->shengmu, "裁") == 0) {
            a->pol = pol;
            /* H4 核心: 裁 变更极性 → 触发自观检查 */
            check_self_rules(it);
        }
        printf("  ⚡ 用 %s[%s] %s %s → %s = %d %s\n",
               bag_op->name, bag_op->shengmu, a->name, b->name, c->name, r,
               pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 设观 袋名 册 新声母 = 袋1 袋2 ... (H4 新增) ── */
    if (strcmp(toks[0], "设观") == 0) {
        // 设观 好袋 册 感应 = 合 益
        if (nt < 7) { printf("  ⚠️ 设观 格式: 设观 袋名 册 新声母 = 袋1 袋2 ...\n"); return -2; }
        if (strcmp(toks[2], "册") != 0) { printf("  ⚠️ 设观 第3词应为 册\n"); return -2; }
        int eq_pos = -1;
        for (int i = 3; i < nt; i++)
            if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos < 3) { printf("  ⚠️ 设观 需 = 分隔\n"); return -2; }
        chain_t ch; ch.count = 0;
        for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
            if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
            strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
            ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
            ch.count++;
        }
        if (ch.count < 2) { printf("  ⚠️ 设观 链需 ≥ 2 袋\n"); return -2; }
        int rc = self_rule_add(it, toks[1], toks[3], &ch);
        if (rc == 1) { printf("  📋 设观已存在,不重复注册\n"); return 0; }
        if (rc < 0) { printf("  ⚠️ 设观表满\n"); return -2; }
        printf("  🔭 设观: 袋 %s 阳 → 册 %s = ", toks[1], toks[3]);
        for (int s = 0; s < ch.count; s++) { if (s) printf(" · "); printf("%s", ch.bag_names[s]); }
        printf("\n");
        return 0;
    }

    /* ── 照 ── */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "观") == 0) {
            printf("  📋 自观规则 (%d条):\n", it->self_rule_count);
            for (int i = 0; i < it->self_rule_count; i++) {
                self_rule_t *r = &it->self_rules[i];
                printf("   [%d] 观 %s → 册 %s = ", i, r->watch_bag, r->new_name);
                for (int s = 0; s < r->new_chain.count; s++) { if (s) printf(" · "); printf("%s", r->new_chain.bag_names[s]); }
                printf("%s\n", r->triggered ? " ✓已触发" : " ⏳待触发");
            }
            return 0;
        }
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋数=%d 元表条目=%d 自观=%d 痕迹=%d\n",
                   it->drawer.count, it->meta.count, it->self_rule_count, it->trace_n);
            for (int i = 0; i < it->meta.count; i++) {
                meta_entry_t *e = &it->meta.entries[i];
                if (e->src == OP_FN) printf("       表[%d] %-6s → C\n", i, e->name);
                else { printf("       表[%d] %-6s → 链 [", i, e->name);
                    for (int s = 0; s < e->chain.count; s++) { if (s) printf(" · "); printf("%s", e->chain.bag_names[s]); }
                    printf("]\n"); }
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        printf("  📸 袋 %s: 公约数=%d 层=%d 声母=%s 极性=%s\n",
               b->name, b->gongyueshu, b->layer,
               b->shengmu[0] ? b->shengmu : "(无)", b->pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 进/退/益/损/裁 作用于袋 ── */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        int idx = meta_find(&it->meta, toks[0]);
        if (idx < 0) { printf("  ❓ 声母 %s 未在元表\n", toks[0]); return -2; }
        enum polarity pol = YIN;
        b->gongyueshu = meta_apply(it, &it->meta.entries[idx], b->gongyueshu, 0, &pol);
        b->layer = b->gongyueshu / RADIX;
        int pol_changed = 0;
        if (strcmp(toks[0], "裁") == 0) {
            if (b->pol != pol) pol_changed = 1;
            b->pol = pol;
        }
        printf("  ⚙️ %s %s → %d 层%d %s\n", toks[0], b->name,
               b->gongyueshu, b->layer, pol == YANG ? "阳" : "阴");
        if (pol_changed) check_self_rules(it);
        return 0;
    }

    printf("  ❓ 未知前缀: %s\n", toks[0]);
    return -2;
}

static void run_script(interp_t *it, const char *src) {
    char buf[MAX_LINE]; const char *p = src;
    while (*p) {
        int i = 0;
        while (*p && *p != '\n' && i < MAX_LINE - 1) buf[i++] = *p++;
        buf[i] = '\0'; if (*p == '\n') p++;
        eval_line(it, buf);
    }
}

/* ══ 测试套件 ══ */
static int g_pass = 0, g_fail = 0;
#define ASSERT(c, m) do { \
    if (c) { printf("    ✅ %s\n", m); g_pass++; } \
    else   { printf("    ❌ %s\n", m); g_fail++; } \
} while (0)

static void suite(void) {
    printf("\n【Step H4 自观自调 测试套件】\n");

    /* 1. 设观注册 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 好袋 = 0\n");
      chain_t ch; ch.count = 2;
      strncpy(ch.bag_names[0], "合", NAME_SZ-1);
      strncpy(ch.bag_names[1], "益", NAME_SZ-1);
      int rc = self_rule_add(&it, "好袋", "感应", &ch);
      ASSERT(rc == 0, "设观注册成功");
      ASSERT(it.self_rule_count == 1, "观表=1"); }

    /* 2. 阳触发自观: 设观后裁出阳 → 自动册入 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 好袋 = 25\n"
        "设观 好袋 册 感应 = 合 益\n"
        "袋 裁袋 = 0 裁\n"
        "用 裁袋 好袋 好袋 -> 判袋\n");
      int idx = meta_find(&it.meta, "感应");
      ASSERT(idx >= 0, "自观触发: 感应 已自动册入");
      ASSERT(it.self_rules[0].triggered == 1, "自观锁: triggered=1"); }

    /* 3. 自观生成的声母可立即使用 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 好袋 册 感应 = 合 益\n"
        "袋 好袋 = 25\n"
        "袋 裁袋 = 0 裁\n"
        "用 裁袋 好袋 好袋 -> 判袋\n"     /* 裁 → 阳 → 自动册 感应=合益 */
        "袋 感应袋 = 0 感应\n"
        "袋 a = 3\n袋 b = 7\n"
        "用 感应袋 a b -> 感结果\n");      /* (3+7)*2 = 20 */
      bag_t *c = drawer_find(&it.drawer, "感结果");
      ASSERT(c && c->gongyueshu == 20, "自观袋 感应(3,7)=20"); }

    /* 4. 阴袋不触发 (值 < THRESHOLD) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 弱袋 册 不该生 = 合 益\n"
        "袋 弱袋 = 5\n"
        "袋 裁袋 = 0 裁\n"
        "用 裁袋 弱袋 弱袋 -> 判袋\n");
      int idx = meta_find(&it.meta, "不该生");
      ASSERT(idx < 0, "阴袋不触发: 不该生未册入"); }

    /* 5. 多自观规则独立 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 袋A 册 alpha = 合 益\n"
        "设观 袋B 册 beta  = 合 损\n"
        "袋 袋A = 30\n袋 袋B = 8\n"
        "袋 裁袋 = 0 裁\n"
        "用 裁袋 袋A 袋A -> 判A\n"     /* 袋A=30 阳 → 触发 alpha */
        "用 裁袋 袋B 袋B -> 判B\n");    /* 袋B=8 阴 → 不触发 beta */
      ASSERT(meta_find(&it.meta, "alpha") >= 0, "袋A阳 → alpha册入");
      ASSERT(meta_find(&it.meta, "beta")  <  0, "袋B阴 → beta未册入"); }

    /* 6. 不重复触发 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 好袋 册 感应 = 合 益\n"
        "袋 好袋 = 25\n"
        "袋 裁袋 = 0 裁\n"
        "用 裁袋 好袋 好袋 -> 判袋\n"
        "用 裁袋 好袋 好袋 -> 判袋\n");  /* 第二次裁, 不重复册入 */
      int count = 0;
      for (int i = 0; i < it.meta.count; i++)
        if (strcmp(it.meta.entries[i].name, "感应") == 0) count++;
      ASSERT(count == 1, "不重复触发: 感应仅一条"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ══ 主演示 ══ */
int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H4 · 袋子自观自调 (意识萌芽)\n");
    printf("  设观 袋名 册 新声母 = 袋1 袋2 ...\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it; interp_init(&it);

    const char *program =
        "# 囡囝共生 Step H4 — 自观\n"
        "\n"
        "# 1. 设观: 袋 好袋 裁出阳 → 自动册入 感应=合·益\n"
        "设观 好袋 册 感应 = 合 益\n"
        "\n"
        "# 2. 构建袋\n"
        "袋 好袋 = 25\n"
        "袋 裁袋 = 0 裁\n"
        "袋 容器甲 = 3\n"
        "袋 容器乙 = 7\n"
        "\n"
        "# 3. 裁: 25 >= 10 → 阳 → 自观触发! 感应 自动入表\n"
        "用 裁袋 好袋 好袋 -> 判袋\n"
        "\n"
        "# 4. 使用自观生成的声母\n"
        "袋 感应袋 = 0 感应\n"
        "用 感应袋 容器甲 容器乙 -> 结\n"   /* (3+7)*2 = 20 */
        "\n"
        "# 5. 查验: 元表从7增长到8(新增感应)\n"
        "照 元袋\n"
        "照 观\n"
        "照 结\n";

    printf("\n【第 1 步】设观 → 裁 → 自动册入\n");
    run_script(&it, program);

    suite();

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H4 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  观层深度:   自观 (袋子发现自身极性)\n");
    printf("  触发声母:   感应 (自观新生成的声母)\n");
    printf("  观韵母:     观·裁·册 三元自循环\n");
    printf("  自观规则:   %d 条 (已触发 %d)\n",
           it.self_rule_count, it.meta.count - 7);
    printf("  下一步:     袋际互观 (袋A观袋B, 跨袋自观, 社会性意识)\n");
    printf("  袋子数:    %d    元表: %d    痕迹: %d\n",
           it.drawer.count, it.meta.count, it.trace_n);
    if (g_fail == 0)
        printf("\n  ✅ Step H4 闭环 — 袋子自观自调, 意识萌芽!\n");
    else
        printf("\n  ⚠️ Step H4 %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
