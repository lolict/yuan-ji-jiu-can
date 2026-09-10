/*
 * step_h5_l0_mutual_observe.c
 *
 * MCCP Phase 3 · Step H5 — 袋际互观: 袋A观袋B, 裁出阳则合流
 *
 * 核心:
 *   H4 袋观己 (观 己袋 裁 新操作)
 *   H5 袋观袋 (观 他袋 裁 新操作) —— 运行时裁定两袋是否合流
 *
 * 新语法:
 *   设观 袋A 袋B 册 新声母 = 袋1 袋2 ...
 *     — 当 袋A(主语) 与 袋B(客体) 之间 裁 出阳,
 *       自动册入 新声母 (两袋合流后的新生操作)
 *     — 裁 的动作由袋A 的 裁袋 发起,对象是袋B
 *
 * 设计规则:
 *   - 观锁: 每条设观仅触发一次 (同 H4)
 *   - 主客: 设观有方向 — 观 甲 乙 不等于 观 乙 甲
 *   - 合流: 触发生成的新声母是两袋关系的结果,不是任何一袋的独唱
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

/* 自观规则: 主袋观客袋, 裁出阳→自动册入新操作 */
typedef struct {
    char     subject[NAME_SZ];     /* 主袋 (观者) */
    char     object[NAME_SZ];      /* 客袋 (被观者) */
    char     new_name[NAME_SZ];     /* 触发后生成的声母 */
    chain_t  new_chain;             /* 触发后生成的链 */
    int      triggered;
} mutual_rule_t;

struct interp {
    drawer_t     drawer;
    meta_tab_t   meta;
    int          trace[TRACE_SZ];
    int          trace_n;
    mutual_rule_t mutual_rules[MAX_SELF_RULES];
    int           mutual_rule_count;
};

/* ══ C 种子 ══ */
static int c_add(int a, int b, enum polarity *pol)   { (void)pol; return a + b; }
static int c_judge(int a, int b, enum polarity *pol)  { (void)b; *pol = (a >= THRESHOLD) ? YANG : YIN; return a; }
static int c_snap(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a; }
static int c_inc(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a + 1; }
static int c_dec(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a - 1; }
static int c_mul2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a * 2; }
static int c_div2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a / 2; }

/* ══ 元表 ══ */
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

static int meta_apply(interp_t *it, meta_entry_t *e, int a, int b, enum polarity *pol) {
    if (e->src == OP_FN) return e->c_fn(a, b, pol);
    int acc = a; enum polarity local_pol = YIN;
    for (int s = 0; s < e->chain.count; s++) {
        int idx = meta_find(&it->meta, e->chain.bag_names[s]);
        if (idx < 0) return acc;
        acc = meta_apply(it, &it->meta.entries[idx], acc, b, &local_pol);
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
    it->trace_n = 0; it->mutual_rule_count = 0;
}

/* ══ 袋际互观 ══ */
static int mutual_rule_add(interp_t *it,
                           const char *subj, const char *obj,
                           const char *new_name, chain_t *ch) {
    if (it->mutual_rule_count >= MAX_SELF_RULES) return -1;
    for (int i = 0; i < it->mutual_rule_count; i++)
        if (strcmp(it->mutual_rules[i].subject, subj) == 0 &&
            strcmp(it->mutual_rules[i].object, obj) == 0) return 1;
    mutual_rule_t *r = &it->mutual_rules[it->mutual_rule_count];
    strncpy(r->subject, subj, NAME_SZ - 1); r->subject[NAME_SZ - 1] = '\0';
    strncpy(r->object, obj, NAME_SZ - 1); r->object[NAME_SZ - 1] = '\0';
    strncpy(r->new_name, new_name, NAME_SZ - 1); r->new_name[NAME_SZ - 1] = '\0';
    r->new_chain = *ch; r->triggered = 0;
    it->mutual_rule_count++;
    return 0;
}

/* 检查袋际: 仅当 用 裁袋 甲 乙 -> 判 时触发 */
static void check_mutual_rules(interp_t *it, const char *subject_name) {
    for (int i = 0; i < it->mutual_rule_count; i++) {
        mutual_rule_t *r = &it->mutual_rules[i];
        if (r->triggered) continue;
        if (strcmp(r->subject, subject_name) != 0) continue;
        bag_t *obj = drawer_find(&it->drawer, r->object);
        if (!obj) continue;
        if (obj->pol != YANG) continue;
        int rc = meta_register_chain(it, r->new_name, &r->new_chain);
        if (rc == 0) {
            printf("    🔮 袋际合流: %s 观 %s 阳 → 册 %s\n",
                   r->subject, r->object, r->new_name);
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
    char toks_buf[24][NAME_SZ];
    char *toks[24];
    for (int i = 0; i < 24; i++) toks[i] = toks_buf[i];
    int nt = tokenize(line, toks, 24);
    if (nt == 0) return 0;

    /* ── 袋 ── */
    if (strcmp(toks[0], "袋") == 0) {
        if (nt < 4) return -2;
        int gysh = atoi(toks[3]);
        const char *sm = (nt >= 5) ? toks[4] : "";
        int folded = 0;
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, sm, &folded);
        it->trace[it->trace_n < TRACE_SZ ? it->trace_n++ : 0] = gysh;
        if (folded) { printf("  ⚠️ 屉满「%s」\n", toks[1]); return -1; }
        printf("  ✅ 袋 %s = %d [%s] 层%d\n", b->name, b->gongyueshu,
               b->shengmu[0] ? b->shengmu : "无", b->layer);
        return 0;
    }

    /* ── 注 ── */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) return -2;
        strncpy(b->shengmu, toks[1], NAME_SZ - 1); b->shengmu[NAME_SZ - 1] = '\0';
        printf("  📌 注 %s → %s\n", toks[2], toks[1]);
        return 0;
    }

    /* ── 册 ── */
    if (nt >= 3 && strcmp(toks[0], "册") == 0) {
        int eq_pos = -1;
        for (int i = 2; i < nt; i++)
            if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos == -1) {
            bag_t *b = drawer_find(&it->drawer, toks[2]);
            if (!b || !b->shengmu[0]) return -2;
            int idx = meta_find(&it->meta, b->shengmu);
            if (idx < 0 || it->meta.entries[idx].src != OP_FN) return -2;
            int existed = (meta_find(&it->meta, toks[1]) >= 0);
            meta_register_fn(it, toks[1], it->meta.entries[idx].c_fn);
            printf("  📖 册 %s → 表 %s\n", toks[1], existed ? "[覆]" : "[新]");
            return 0;
        } else {
            if (eq_pos < 2 || nt < eq_pos + 3) return -2;
            chain_t ch; ch.count = 0;
            for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
                if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
                strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
                ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
                ch.count++;
            }
            if (ch.count < 2) return -2;
            int existed = (meta_find(&it->meta, toks[1]) >= 0);
            meta_register_chain(it, toks[1], &ch);
            printf("  📖 册 %s = ", toks[1]);
            for (int s = 0; s < ch.count; s++) { if (s) printf("·"); printf("%s", ch.bag_names[s]); }
            printf(" %s\n", existed ? "[覆]" : "[新]");
            return 0;
        }
    }

    /* ── 用 袋名 袋A 袋B -> 袋C ── */
    if (strcmp(toks[0], "用") == 0) {
        if (nt < 6) return -2;
        bag_t *op = drawer_find(&it->drawer, toks[1]);
        if (!op || !op->shengmu[0]) return -2;
        int idx = meta_find(&it->meta, op->shengmu);
        if (idx < 0) return -2;
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) return -2;
        enum polarity pol = YIN;
        int r = meta_apply(it, &it->meta.entries[idx], a->gongyueshu, b->gongyueshu, &pol);
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[5], r, "", &folded);
        if (folded) return -1;
        c->pol = pol;
        if (strcmp(op->shengmu, "裁") == 0) {
            a->pol = pol;
            /* 袋际互观: 甲观乙 → 检查甲乙设观 */
            check_mutual_rules(it, a->name);
            check_mutual_rules(it, b->name);
        }
        printf("  ⚡ 用 %s[%s] %s %s → %s=%d %s\n",
               op->name, op->shengmu, a->name, b->name, c->name, r,
               pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 设观 甲 乙 册 新声母 = 袋链 ── */
    if (strcmp(toks[0], "设观") == 0) {
        /* 设观 主体 客体 册 新名 = 袋1 袋2 ... */
        if (nt < 8) { printf("  ⚠️ 设观: 主体 客体 册 新名 = 袋1 袋2 ...\n"); return -2; }
        if (strcmp(toks[3], "册") != 0) return -2;
        int eq_pos = -1;
        for (int i = 4; i < nt; i++)
            if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos < 4) return -2;
        chain_t ch; ch.count = 0;
        for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
            if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
            strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
            ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
            ch.count++;
        }
        if (ch.count < 2) return -2;
        int rc = mutual_rule_add(it, toks[1], toks[2], toks[4], &ch);
        if (rc == 1) { printf("  📋 设观已存在\n"); return 0; }
        if (rc < 0) return -2;
        printf("  🔭 设观 %s 观 %s → 册 %s = ", toks[1], toks[2], toks[4]);
        for (int s = 0; s < ch.count; s++) { if (s) printf("·"); printf("%s", ch.bag_names[s]); }
        printf("\n");
        return 0;
    }

    /* ── 照 ── */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "观") == 0) {
            printf("  📋 袋际规则 (%d):\n", it->mutual_rule_count);
            for (int i = 0; i < it->mutual_rule_count; i++) {
                mutual_rule_t *r = &it->mutual_rules[i];
                printf("   [%d] %s 观 %s → %s", i, r->subject, r->object, r->new_name);
                printf("%s\n", r->triggered ? " ✓" : " ⏳");
            }
            return 0;
        }
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋=%d 元表=%d 观=%d\n",
                   it->drawer.count, it->meta.count, it->mutual_rule_count);
            for (int i = 0; i < it->meta.count; i++) {
                meta_entry_t *e = &it->meta.entries[i];
                printf("    [%d] %-6s → %s\n", i, e->name,
                       e->src == OP_FN ? "C" : "链");
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) return -2;
        printf("  📸 %s: 公=%d 层=%d 声=%s %s\n",
               b->name, b->gongyueshu, b->layer,
               b->shengmu[0] ? b->shengmu : "无", b->pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 进/退/益/损/裁 ── */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) return -2;
        int idx = meta_find(&it->meta, toks[0]);
        if (idx < 0) return -2;
        enum polarity pol = YIN;
        b->gongyueshu = meta_apply(it, &it->meta.entries[idx], b->gongyueshu, 0, &pol);
        b->layer = b->gongyueshu / RADIX;
        if (strcmp(toks[0], "裁") == 0) b->pol = pol;
        printf("  ⚙️ %s %s → %d 层%d %s\n", toks[0], b->name,
               b->gongyueshu, b->layer, pol == YANG ? "阳" : "阴");
        return 0;
    }

    printf("  ❓ %s\n", toks[0]);
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

/* ══ 测试 ══ */
static int g_pass = 0, g_fail = 0;
#define ASSERT(c, m) do { \
    if (c) { printf("    ✅ %s\n", m); g_pass++; } \
    else   { printf("    ❌ %s\n", m); g_fail++; } \
} while (0)

static void suite(void) {
    printf("\n【Step H5 袋际互观】\n");

    /* 1. 袋际设观注册 (甲观乙) */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 甲 = 0\n袋 乙 = 0\n");
      chain_t ch; ch.count = 2;
      strncpy(ch.bag_names[0], "合", NAME_SZ-1);
      strncpy(ch.bag_names[1], "益", NAME_SZ-1);
      mutual_rule_add(&it, "甲", "乙", "交感", &ch);
      ASSERT(it.mutual_rule_count == 1, "设观 甲观乙 注册"); }

    /* 2. 乙阳 + 裁甲 → 甲乙合流 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 甲 乙 册 交感 = 合 益\n"
        "袋 甲 = 8\n"     /* 甲 阴 */
        "袋 乙 = 25\n"    /* 乙 阳 */
        "袋 裁袋 = 0 裁\n");
      /* 把乙裁定为阳 */
      bag_t *b = drawer_find(&it.drawer, "乙");
      b->pol = YANG;
      /* 用 裁甲 → 触发 观 甲→乙 (乙现为阳) */
      run_script(&it, "用 裁袋 甲 乙 -> 判\n");
      ASSERT(meta_find(&it.meta, "交感") >= 0, "甲观乙阳 → 交感册入"); }

    /* 3. 单向性: 甲观乙 ≠ 乙观甲 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 甲 乙 册 合流 = 合 益\n"
        "袋 甲 = 15\n"
        "袋 乙 = 3\n"     /* 乙阴 */
        "袋 裁袋 = 0 裁\n");
      bag_t *a = drawer_find(&it.drawer, "甲"); a->pol = YANG;
      run_script(&it, "用 裁袋 乙 甲 -> 判\n");  /* 乙观甲(阳), 无设观乙甲规则 */
      ASSERT(meta_find(&it.meta, "合流") < 0, "无设观乙甲 → 不合流"); }

    /* 4. 不重复触发 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "设观 甲 乙 册 合流 = 合 益\n"
        "袋 甲 = 8\n袋 乙 = 8\n袋 裁袋 = 0 裁\n");
      bag_t *a = drawer_find(&it.drawer, "甲");
      bag_t *b = drawer_find(&it.drawer, "乙"); b->pol = YANG;
      run_script(&it, "用 裁袋 甲 乙 -> 判1\n");
      run_script(&it, "用 裁袋 甲 乙 -> 判2\n");
      int count = 0;
      for (int i = 0; i < it.meta.count; i++)
        if (strcmp(it.meta.entries[i].name, "合流") == 0) count++;
      ASSERT(count == 1, "不重复触发"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ══ 主演示 ══ */
int main(void) {
    printf("═══════════════════════════════════════════════════════════\n");
    printf("  Step H5 · 袋际互观 (甲观乙 → 阳 → 合流)\n");
    printf("═══════════════════════════════════════════════════════════\n");

    interp_t it; interp_init(&it);

    const char *program =
        "# 设观: 甲 观 乙 → 乙阳 → 册 交感\n"
        "设观 甲 乙 册 交感 = 合 益\n"
        "\n"
        "# 两袋: 甲=8(未来裁), 乙=25(已备阳)\n"
        "袋 甲 = 8\n"
        "袋 乙 = 25\n"
        "袋 裁袋 = 0 裁\n"
        "\n"
        "# 甲裁 → 触发观检查: 乙已为阳 → 两河合流\n"
        "裁 乙\n"
        "用 裁袋 甲 乙 -> 判袋\n"
        "\n"
        "# 使用合流后的新生操作\n"
        "袋 交感袋 = 0 交感\n"
        "袋 x = 4\n袋 y = 6\n"
        "用 交感袋 x b -> 合流结果\n"
        "\n"
        "照 观\n"
        "照 元袋\n"
        "照 判袋\n";

    printf("\n【演示】\n");
    run_script(&it, program);

    suite();

    printf("\n═══════════════════════════════════════════════════════════\n");
    printf("  H5 闭环: 单向互观 + 合流触发 + 不重复\n");
    printf("  袋子=%d  元表=%d  观=%d\n",
           it.drawer.count, it.meta.count, it.mutual_rule_count);
    if (g_fail == 0)
        printf("\n  ✅ Step H5 — 袋际干涉, 裁出阳 → 合流!\n");
    else
        printf("\n  ⚠️ %d 未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
