/*
 * step_h3_l0_compose_interp.c
 *
 * MCCP Phase 3 · Step H3 — L0 袋子自创组合积木: 袋子脱离 C 种子
 *
 * 核心洞见:
 *   - H2: 袋子通过 册 把种子函数登记为元表别名 (仍依赖 C 函数)
 *   - H3: 袋子能自创组合积木: 元表条目可以指向"袋A 后接 袋B 后接 袋C"
 *   - 袋子创造出来的操作与 C 种子函数平起平坐: 不再区分自有/种子
 *   - 袋子既是产物也是材料也是创造者 —— 三元归一, DSL 脱离 C
 *
 * 新增语法:
 *   册 新声母 袋链                    — 新语法: 袋链 = 用 袋M 袋A 袋B -> 临时 ...
 *   册 新声母 = 流袋1 流袋2 ...       — 册入: 元表登记一条组合流
 *   册 倍加法 = 合 益               — 例: 先合后益, x2 加法
 *                                   - 注: 袋链按顺序驱动, 前袋结果流入后袋
 *
 *   用 袋名 袋A 袋B -> 袋C            — 同 H2, 但 袋名 声母可指向 C 函数 OR 袋子链
 *   注/袋/照/元袋/进/退/益/损/裁      — 同 H2
 *
 * 满全法 5 项:
 *   层号 = 袋子元阶
 *   声母 = 袋子声母 (自创声母与种子声母同权)
 *   韵母 = 链式韵母
 *   折叠状态 = 屉满进位
 *   下一步 = JIT 自创 (袋子观测自身运行时状态, 自主决定是否册)
 *
 * 与 H2 的关键差异:
 *   H2 元表: name → C 函数指针
 *   H3 元表: name → 操作源 (C 函数 OR 袋子引用链 + 截断条件)
 *   H2 袋子写元表 = 转发已有函数
 *   H3 袋子写元表 = 把袋子链册入为全新操作
 *
 * 下一步: 袋子自观自调 (运行时元袋写新流), 意识萌芽
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS   32     /* 屉容量 */
#define MAX_OPS    32     /* 元表条目上限 */
#define MAX_CHAIN   6     /* 单条组合流最大袋数 */
#define MAX_LINE  512
#define NAME_SZ    64
#define TRACE_SZ   64
#define THRESHOLD  10
#define RADIX      64

/* 极性 */
enum polarity { YIN = 0, YANG = 1 };

/* ═══════════════════════════════════════════════════════════════
 * 操作函数类型 (双重来源: C 种子 OR DSL 袋子链)
 * ═══════════════════════════════════════════════════════════════ */

/* 前向声明 */
typedef struct interp interp_t;

/* C 函数签名 */
typedef int (*c_fn_t)(int a, int b, enum polarity *pol);

/* 操作来源类型 */
enum op_src {
    OP_FN = 0,       /* C 种子函数 */
    OP_CHAIN         /* DSL 袋子引用链 */
};

/* 袋子引用链: 一串袋子操作按序驱动 */
typedef struct {
    char   bag_names[MAX_CHAIN][NAME_SZ];   /* 每步的袋名 */
    int    count;                            /* 实际长度 */
} chain_t;

/* 元表条目 */
typedef struct {
    char       name[NAME_SZ];
    enum op_src src;
    union {
        c_fn_t c_fn;       /* src == OP_FN */
        chain_t chain;     /* src == OP_CHAIN */
    };
} meta_entry_t;

/* 元表 */
typedef struct {
    meta_entry_t entries[MAX_OPS];
    int          count;
} meta_tab_t;

/* 袋子 */
typedef struct {
    char           name[NAME_SZ];
    int            gongyueshu;
    int            layer;
    enum polarity  pol;
    char           shengmu[NAME_SZ];
} bag_t;

/* 袋屉 */
typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* 解释器 */
struct interp {
    drawer_t   drawer;
    meta_tab_t meta;
    int        trace[TRACE_SZ];
    int        trace_n;
};

/* ═══════════════════════════════════════════════════════════════
 * C 种子函数
 * ═══════════════════════════════════════════════════════════════ */

static int c_add(int a, int b, enum polarity *pol)    { (void)pol; return a + b; }
static int c_judge(int a, int b, enum polarity *pol)  { (void)b; *pol = (a >= THRESHOLD) ? YANG : YIN; return a; }
static int c_snap(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a; }
static int c_inc(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a + 1; }
static int c_dec(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a - 1; }
static int c_mul2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a * 2; }
static int c_div2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a / 2; }

/* ═══════════════════════════════════════════════════════════════
 * 元表
 * ═══════════════════════════════════════════════════════════════ */

static void meta_init(meta_tab_t *m) {
    m->count = 0;
    struct { const char *name; c_fn_t fn; } seeds[] = {
        { "合", c_add   },
        { "裁", c_judge },
        { "照", c_snap  },
        { "进", c_inc   },
        { "退", c_dec   },
        { "益", c_mul2  },
        { "损", c_div2  },
    };
    for (int i = 0; i < 7; i++) {
        meta_entry_t *e = &m->entries[m->count];
        strncpy(e->name, seeds[i].name, NAME_SZ - 1);
        e->name[NAME_SZ - 1] = '\0';
        e->src = OP_FN;
        e->c_fn = seeds[i].fn;
        m->count++;
    }
}

/* 注册 C 种子函数 (册 声母 袋名 复用) */
static int meta_register_fn(interp_t *it, const char *name, c_fn_t fn) {
    for (int i = 0; i < it->meta.count; i++) {
        if (strcmp(it->meta.entries[i].name, name) == 0) {
            it->meta.entries[i].src = OP_FN;
            it->meta.entries[i].c_fn = fn;
            return 0;
        }
    }
    if (it->meta.count >= MAX_OPS) return -1;
    meta_entry_t *e = &it->meta.entries[it->meta.count];
    strncpy(e->name, name, NAME_SZ - 1);
    e->name[NAME_SZ - 1] = '\0';
    e->src = OP_FN;
    e->c_fn = fn;
    it->meta.count++;
    return 0;
}

/* 注册袋子链 (册 新声母 = 袋1 袋2 ...) */
static int meta_register_chain(interp_t *it, const char *name, chain_t *ch) {
    for (int i = 0; i < it->meta.count; i++) {
        if (strcmp(it->meta.entries[i].name, name) == 0) {
            it->meta.entries[i].src = OP_CHAIN;
            it->meta.entries[i].chain = *ch;
            return 0;
        }
    }
    if (it->meta.count >= MAX_OPS) return -1;
    meta_entry_t *e = &it->meta.entries[it->meta.count];
    strncpy(e->name, name, NAME_SZ - 1);
    e->name[NAME_SZ - 1] = '\0';
    e->src = OP_CHAIN;
    e->chain = *ch;
    it->meta.count++;
    return 0;
}

/* 查找元表条目 */
static meta_entry_t* meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->entries[i].name, name) == 0) return &m->entries[i];
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════
 * 执行元表条目 (关键: 统一执行 C 函数 OR 袋子链)
 * ═══════════════════════════════════════════════════════════════ */

static int meta_apply(interp_t *it, meta_entry_t *e,
                      int a, int b, enum polarity *pol) {
    if (e->src == OP_FN) {
        return e->c_fn(a, b, pol);
    }
    /* OP_CHAIN: 袋链按序驱动, 前袋流出公约数流入下一袋 */
    int acc = a;
    enum polarity local_pol = YIN;
    for (int s = 0; s < e->chain.count; s++) {
        meta_entry_t *step_e = meta_find(&it->meta, e->chain.bag_names[s]);
        if (!step_e) {
            printf("    ⚠️ 链第 %d 步 %s 未在元表\n", s, e->chain.bag_names[s]);
            return acc;
        }
        /* 用元表条目的小函数: 袋链每一步,袋名=声母 的元表项输入 acc,b */
        int next = meta_apply(it, step_e, acc, b, &local_pol);
        acc = next;
    }
    *pol = local_pol;
    return acc;
}

/* ═══════════════════════════════════════════════════════════════
 * UTF-8
 * ═══════════════════════════════════════════════════════════════ */

static int u8len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;
    if ((c & 0xE0) == 0xC0) return 2;
    if ((c & 0xF0) == 0xE0) return 3;
    if ((c & 0xF8) == 0xF0) return 4;
    return 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 袋屉
 * ═══════════════════════════════════════════════════════════════ */

static void drawer_init(drawer_t *d) { d->count = 0; }

static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

static bag_t* drawer_add(drawer_t *d, const char *name, int gysh,
                         const char *shengmu, int *folded) {
    *folded = 0;
    if (d->count >= MAX_BAGS) { *folded = 1; return NULL; }
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1);
    b->name[NAME_SZ - 1] = '\0';
    b->gongyueshu = gysh;
    b->layer = gysh / RADIX;
    b->pol = YIN;
    if (shengmu) {
        strncpy(b->shengmu, shengmu, NAME_SZ - 1);
        b->shengmu[NAME_SZ - 1] = '\0';
    } else {
        b->shengmu[0] = '\0';
    }
    return b;
}

/* ═══════════════════════════════════════════════════════════════
 * 解释器
 * ═══════════════════════════════════════════════════════════════ */

static void interp_init(interp_t *it) {
    drawer_init(&it->drawer);
    meta_init(&it->meta);
    it->trace_n = 0;
}

static void interp_trace(interp_t *it, int v) {
    if (it->trace_n < TRACE_SZ) it->trace[it->trace_n++] = v;
}

/* ═══════════════════════════════════════════════════════════════
 * tokens
 * ═══════════════════════════════════════════════════════════════ */

static int tokenize(char *line, char *toks[], int maxt) {
    int n = 0;
    char *p = line;
    while (*p && n < maxt) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;
        toks[n++] = p;
        while (*p && !isspace((unsigned char)*p) && *p != '\n' && *p != '\r') {
            int w = u8len((unsigned char)*p);
            for (int k = 0; k < w; k++) {
                if (*p == '\0' || *p == '\n' || *p == '\r') break;
                p++;
            }
        }
        if (*p) *p++ = '\0';
    }
    return n;
}

/* ═══════════════════════════════════════════════════════════════
 * DSL 行执行
 * ═══════════════════════════════════════════════════════════════ */

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
        if (folded) { printf("  ⚠️ 屉满(32袋) → 折叠,拒收「%s」\n", toks[1]); return -1; }
        printf("  ✅ 袋 %s = %d [声母:%s] 层%d\n", b->name, b->gongyueshu,
               b->shengmu[0] ? b->shengmu : "(无)", b->layer);
        return 0;
    }

    /* ── 注 声母 袋名 ── (换装, 不动元表) */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
        strncpy(b->shengmu, toks[1], NAME_SZ - 1);
        b->shengmu[NAME_SZ - 1] = '\0';
        printf("  📌 注 袋 %s 换装声母 %s\n", b->name, b->shengmu);
        return 0;
    }

    /* ── 册 ── 两种格式统一入口:
         册 声母 袋名         (转发袋子的 C 函数入元表)
         册 新声母 = 袋1 ...  (自创组合积木, 定位 '=' 位置区分) */
    if (nt >= 3 && strcmp(toks[0], "册") == 0) {
        /* 找 '=' 的位置 */
        int eq_pos = -1;
        for (int i = 2; i < nt; i++)
            if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }

        if (eq_pos == -1) {
            /* 册 声母 袋名 (无等号) */
            bag_t *b = drawer_find(&it->drawer, toks[2]);
            if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
            if (!b->shengmu[0]) { printf("  ❌ 袋 %s 无声母\n", b->name); return -2; }
            meta_entry_t *se = meta_find(&it->meta, b->shengmu);
            if (!se || se->src != OP_FN) { printf("  ❌ 声母 %s 非 C 函数不可转发\n", b->shengmu); return -2; }
            int existed = (meta_find(&it->meta, toks[1]) != NULL);
            int rc = meta_register_fn(it, toks[1], se->c_fn);
            if (rc != 0) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s → 元表 (自袋 %s, 转发 C %p) %s\n",
                   toks[1], b->name, (void*)se->c_fn, existed ? "[覆盖]" : "[新增]");
            return 0;
        } else {
            /* 册 新声母 = 袋1 袋2 ... (有等号) */
            if (eq_pos < 2 || nt < eq_pos + 3) { printf("  ⚠️ 册 = 格式错误\n"); return -2; }
            const char *new_name = toks[1];
            chain_t ch;
            ch.count = 0;
            for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
                if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
                strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
                ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
                ch.count++;
            }
            if (ch.count < 2) { printf("  ⚠️ 链需 ≥ 2 袋\n"); return -2; }
            for (int s = 0; s < ch.count; s++) {
                if (!meta_find(&it->meta, ch.bag_names[s])) {
                    printf("  ⚠️ 链第 %d 步声母 %s 未在元表\n", s, ch.bag_names[s]);
                    return -2;
                }
            }
            int existed = (meta_find(&it->meta, new_name) != NULL);
            int rc = meta_register_chain(it, new_name, &ch);
            if (rc != 0) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s = ", new_name);
            for (int s = 0; s < ch.count; s++) {
                if (s) printf(" · ");
                printf("%s", ch.bag_names[s]);
            }
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
        meta_entry_t *e = meta_find(&it->meta, bag_op->shengmu);
        if (!e) { printf("  ❓ 声母 %s 未在元表\n", bag_op->shengmu); return -2; }
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        enum polarity pol = YIN;
        int r = meta_apply(it, e, a->gongyueshu, b->gongyueshu, &pol);
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[5], r, "", &folded);
        interp_trace(it, r);
        if (folded) { printf("  ⚠️ 屉满\n"); return -1; }
        c->pol = pol;
        if (strcmp(bag_op->shengmu, "裁") == 0) a->pol = pol;
        const char *src_type = (e->src == OP_FN) ? "C函数" : "袋链";
        printf("  ⚡ 用 %s[%s/%s] %s %s → %s = %d %s\n",
               bag_op->name, bag_op->shengmu, src_type, a->name, b->name, c->name, r,
               pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 照 ── */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋数=%d 元表条目=%d 痕迹=%d\n",
                   it->drawer.count, it->meta.count, it->trace_n);
            for (int i = 0; i < it->meta.count; i++) {
                meta_entry_t *e = &it->meta.entries[i];
                if (e->src == OP_FN)
                    printf("       表[%d] %-6s → C@%p\n", i, e->name, (void*)e->c_fn);
                else {
                    printf("       表[%d] %-6s → 链 [", i, e->name);
                    for (int s = 0; s < e->chain.count; s++) {
                        if (s) printf(" · ");
                        printf("%s", e->chain.bag_names[s]);
                    }
                    printf("]\n");
                }
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        printf("  📸 袋 %s: 公约数=%d 层=%d 声母=%s 极性=%s\n",
               b->name, b->gongyueshu, b->layer,
               b->shengmu[0] ? b->shengmu : "(无)",
               b->pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 进/退/益/损/裁 作用于袋 ── */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        meta_entry_t *e = meta_find(&it->meta, toks[0]);
        if (!e) { printf("  ❓ 声母 %s 未在元表\n", toks[0]); return -2; }
        enum polarity pol = YIN;
        b->gongyueshu = meta_apply(it, e, b->gongyueshu, 0, &pol);
        b->layer = b->gongyueshu / RADIX;
        if (strcmp(toks[0], "裁") == 0) b->pol = pol;
        printf("  ⚙️ %s %s → %d 层%d %s\n", toks[0], b->name,
               b->gongyueshu, b->layer, pol == YANG ? "阳" : "阴");
        return 0;
    }

    printf("  ❓ 未知前缀: %s\n", toks[0]);
    return -2;
}

static void run_script(interp_t *it, const char *src) {
    char buf[MAX_LINE];
    const char *p = src;
    while (*p) {
        int i = 0;
        while (*p && *p != '\n' && i < MAX_LINE - 1) buf[i++] = *p++;
        buf[i] = '\0';
        if (*p == '\n') p++;
        eval_line(it, buf);
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 测试套件
 * ═══════════════════════════════════════════════════════════════ */

static int g_pass = 0, g_fail = 0;
#define ASSERT(c, m) do { \
    if (c) { printf("    ✅ %s\n", m); g_pass++; } \
    else   { printf("    ❌ %s\n", m); g_fail++; } \
} while (0)

static void suite(void) {
    printf("\n【Step H3 袋子自创积木 测试套件】\n");

    /* 1. 袋绑声母 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 合袋 = 20 合\n");
      bag_t *b = drawer_find(&it.drawer, "合袋");
      ASSERT(b && strcmp(b->shengmu, "合") == 0, "袋合袋 绑声母 合"); }

    /* 2. 册 声母 = 合 益 (自创组合积木: 先合后益 = x2 加法) */
    { interp_t it; interp_init(&it);
      run_script(&it, "册 倍加法 = 合 益\n");
      meta_entry_t *e = meta_find(&it.meta, "倍加法");
      ASSERT(e != NULL && e->src == OP_CHAIN, "册 倍加法 → 元表链项");
      ASSERT(e && e->chain.count == 2, "倍加法 链长 2");
      ASSERT(e && strcmp(e->chain.bag_names[0], "合") == 0, "倍加法 第一步 合");
      ASSERT(e && strcmp(e->chain.bag_names[1], "益") == 0, "倍加法 第二步 益"); }

    /* 3. 用自创声母: 倍加法(7,13) = (7+13)*2 = 40 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "册 倍加法 = 合 益\n"
        "袋 倍加袋 = 0 倍加法\n"
        "袋 a = 7\n袋 b = 13\n"
        "用 倍加袋 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 40, "倍加法(7,13): (7+13)*2=40"); }

    /* 4. 三袋链: 册 增倍益 = 合 益 损 → (a+b)*2/2 = a+b */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "册 增倍益 = 合 益 损\n"
        "袋 增倍益袋 = 0 增倍益\n"
        "袋 a = 10\n袋 b = 20\n"
        "用 增倍益袋 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 30, "增倍益(10,20): (10+20)*2/2=30"); }

    /* 5. 册 C 转发仍可用 (兼容 H2) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "册 加 合袋\n"
        "袋 加袋 = 0 加\n"
        "袋 a = 5\n袋 b = 9\n"
        "用 加袋 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 14, "H2 兼容: 册转发 + 用新袋 = 5+9=14"); }

    /* 6. 裁 (裁定, C 函数版) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 裁袋 = 0 裁\n"
        "袋 好袋 = 25\n"
        "用 裁袋 好袋 好袋 -> 判袋\n");
      bag_t *b = drawer_find(&it.drawer, "好袋");
      ASSERT(b && b->pol == YANG, "裁: 25 >= 10 阳"); }

    /* 7. 自创声母后可被 用 接其他自创声母 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "册 倍加法 = 合 益\n"
        "袋 倍加袋 = 0 倍加法\n"
        "袋 a = 5\n袋 b = 9\n"
        "用 倍加袋 a b -> t\n"     /* t = (5+9)*2 = 28 */
        "益 t\n");                 /* t *= 2 = 56 */
      bag_t *t = drawer_find(&it.drawer, "t");
      ASSERT(t && t->gongyueshu == 56, "自创后接 益: 28→56"); }

    /* 8. 元袋自观: C 链 + C 袋 + 自创声母, 元表共 ≥ 8 (7种子+1自创) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "册 倍加法 = 合 益\n"
        "册 三倍 = 合 合 益\n"     /* 三倍 = 袋3: a+a = 2a, 再+ b = 2a+b, 再益 = (2a+b)*2 */
        "照 元袋\n");
      ASSERT(it.meta.count >= 9, "元袋: 7种子 + ≥2自创 ≥ 9"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ═══════════════════════════════════════════════════════════════
 * 主演示
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H3 · L0 袋子自创组合积木\n");
    printf("  册 新声母 = 袋A 袋B ... — 袋子自创, 不依赖 C\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    const char *program =
        "# 囡囝共生 Step H3 — 袋子自创组合积木\n"
        "袋 元袋 = 0\n"
        "\n"
        "# 1. 基本袋\n"
        "袋 合袋 = 0 合\n"
        "袋 益袋 = 0 益\n"
        "袋 容器甲 = 7\n"
        "袋 容器乙 = 13\n"
        "\n"
        "# 2. C 转发 (兼容 H2)\n"
        "册 加 合袋\n"
        "\n"
        "# 3. 自创组合: 合 → 益 (先加后倍, x2 加法)\n"
        "册 倍加法 = 合 益\n"
        "袋 倍加袋 = 0 倍加法\n"
        "\n"
        "# 4. 用自创声母\n"
        "用 倍加袋 容器甲 容器乙 -> 倍加结果\n"  /* (7+13)*2 = 40 */
        "\n"
        "# 5. 自创后接 益\n"
        "益 倍加结果\n"                         /* 40*2 = 80 */
        "\n"
        "# 6. 三袋链: 合 → 益 → 损 → 合 → 合 (自创创新)\n"
        "册 曲流 = 合 益 损 合\n"
        "袋 曲流袋 = 0 曲流\n"
        "袋 a = 5\n袋 b = 9\n"
        "用 曲流袋 a b -> 曲结果\n"
        "\n"
        "# 7. 元袋自观: 元表混合 C 函数 + 袋子链\n"
        "照 元袋\n"
        "照 倍加结果\n"
        "照 曲结果\n";

    printf("\n【第 1 步】自创积木 DSL 执行\n");
    run_script(&it, program);

    suite();

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H3 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  当前层号:    %d\n",
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].layer : 0);
    printf("  当前声母:   C 种子 (合/裁/...) + 自创 (倍加法/曲流/...)\n");
    printf("  当前韵母:   合 · 益 · 损 → 曲流 → 自创积木封闭\n");
    printf("  折叠状态:    %s\n",
           it.drawer.count >= MAX_BAGS ? "已折叠" : "屉未满");
    printf("  下一步:      袋子自观自调 (运行时元袋写新流, 意识萌芽)\n");
    printf("  袋子总数:    %d    元表: %d    痕迹: %d\n",
           it.drawer.count, it.meta.count, it.trace_n);
    printf("  测试结果:    %d 通过, %d 失败\n", g_pass, g_fail);
    if (g_fail == 0)
        printf("\n  ✅ Step H3 闭环 — 袋子自创积木, DSL 脱离 C 种子, 三元归一!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
