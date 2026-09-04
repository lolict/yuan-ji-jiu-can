/*
 * step_h2_l0_bootstrap_interp.c
 *
 * MCCP Phase 3 · Step H2 — L0 完全自举: 袋子即元表
 *
 * 核心洞见 (来自 Step H1 的下一步):
 *   - H1: 袋子携带 enum opcode, C 编译期硬编码 7 种积木
 *   - H2: 袋子携带"声母名字" + 元表是函数指针表 (C 仅提供种子函数)
 *   - DSL "册" 语句: 袋子把自己携带的操作登记为元表新条目
 *   - 袋子的操作可以是已知的 C 函数, 也可以是 DSL 定义的纯袋子组合
 *   - 袋子既是产物也是元表的写入者 —— 袋子即元表, 完全自举闭环
 *
 * 语法 (UTF-8 中文):
 *   袋 袋名 = <公约数> [声母]        — 创建袋,可选绑声母
 *   注 声母 袋名                      — 给袋换装声母 (改袋不登记元表)
 *   册 声母 袋名                      — 把袋子的操作登记入元表 (自举)
 *   用 袋名 袋A 袋B -> 袋C            — 用袋子声母对应的函数执行
 *   照 袋名                          — 快照袋状态
 *   元袋                              — 快照解释器全貌 (含可扩元表)
 *   进/退/益/损/裁 袋名               — 袋子自操作 (同 H1)
 *
 * 满全法 5 项:
 *   层号 = 袋子的元阶 (公约数 ÷ 64)
 *   声母 = 袋子当前携带的声母名
 *   韵母 = 袋子名韵母组合
 *   折叠状态 = 屉满(32袋)进位
 *   下一步 = 袋子写入 C 原语之外的新操作 (袋子自创积木)
 *
 * 与 H1 的关键差异:
 *   H1: enum opcode 硬编码, 语法预知全家, 元表 = 查找表
 *   H2: 声母即元表键, 函数指针驱动, DSL 元表自扩, 袋子 = 元表写入器
 *   H1: 注 (改袋) → 元表也静默更新
 *   H2: 注 vs 册 严格分离 — 注改袋不写元表, 册把袋操作登记元表
 *
 * 下一步: 袋自创组合积木 ("册 新声母 = 袋A 袋B"), 袋子自创操作, DSL 脱离 C 种子
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS   32     /* 屉容量: 32 袋为一元阶 */
#define MAX_OPS    32     /* 元表上限 (H1 仅 8, H2 可扩至 32) */
#define MAX_LINE  512
#define NAME_SZ    64
#define TRACE_SZ   64
#define THRESHOLD  10
#define RADIX      64

/* 极性 */
enum polarity { YIN = 0, YANG = 1 };

/* 袋子声母对应的操作函数签名 */
typedef int (*op_fn_t)(int a, int b, enum polarity *pol);

/* 元表条目: 声母名字 → 函数指针 */
typedef struct {
    char      name[NAME_SZ];     /* 声母: 合/裁/照/进/退/益/损/... */
    op_fn_t   fn;                /* C 函数指针, 种子由 C 提供, 后期由 JIT/DSL 写入 */
} meta_entry_t;

/* 元表 (运行时自扩) */
typedef struct {
    meta_entry_t entries[MAX_OPS];
    int          count;
} meta_tab_t;

/* 袋 (容器): 名字 + 公约数 + 元阶 + 极性 + 声母名 (字符串, 不依赖 enum) */
typedef struct {
    char           name[NAME_SZ];
    int            gongyueshu;
    int            layer;
    enum polarity  pol;
    char           shengmu[NAME_SZ];   /* 袋子当前声母, 空=NOP */
} bag_t;

/* 袋屉 */
typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* 解释器 */
typedef struct {
    drawer_t   drawer;
    meta_tab_t meta;
    int        trace[TRACE_SZ];
    int        trace_n;
} interp_t;

/* ═══════════════════════════════════════════════════════════════
 * C 种子函数集 (启动元表的 7 个声母)
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
    /* 种子 7 声母: 函数指针驱动 */
    struct { const char *name; op_fn_t fn; } seeds[] = {
        { "合", c_add   },
        { "裁", c_judge },
        { "照", c_snap  },
        { "进", c_inc   },
        { "退", c_dec   },
        { "益", c_mul2  },
        { "损", c_div2  },
    };
    for (int i = 0; i < 7; i++) {
        strncpy(m->entries[m->count].name, seeds[i].name, NAME_SZ - 1);
        m->entries[m->count].name[NAME_SZ - 1] = '\0';
        m->entries[m->count].fn = seeds[i].fn;
        m->count++;
    }
}

/* 元表注册一个新声母 (如果已存在则覆盖) — 即"册"的底层 */
static int meta_register(interp_t *it, const char *name, op_fn_t fn) {
    for (int i = 0; i < it->meta.count; i++) {
        if (strcmp(it->meta.entries[i].name, name) == 0) {
            it->meta.entries[i].fn = fn;
            return 0; /* 更新 */
        }
    }
    if (it->meta.count >= MAX_OPS) return -1;
    strncpy(it->meta.entries[it->meta.count].name, name, NAME_SZ - 1);
    it->meta.entries[it->meta.count].name[NAME_SZ - 1] = '\0';
    it->meta.entries[it->meta.count].fn = fn;
    it->meta.count++;
    return 0; /* 新增 */
}

/* 查找声母 → 函数指针, 找不到返回 NULL */
static op_fn_t meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->entries[i].name, name) == 0) return m->entries[i].fn;
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════
 * UTF-8 辅助
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
    if (shengmu && strlen(shengmu) > 0)
        strncpy(b->shengmu, shengmu, NAME_SZ - 1);
    else
        b->shengmu[0] = '\0';
    b->shengmu[NAME_SZ - 1] = '\0';
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
 * UTF-8 tokens
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

    char *toks[16];
    int nt = tokenize(line, toks, 16);
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

    /* ── 注 声母 袋名 ── (换装: 改袋, 不动元表) */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
        strncpy(b->shengmu, toks[1], NAME_SZ - 1);
        b->shengmu[NAME_SZ - 1] = '\0';
        printf("  📌 注 袋 %s 换装声母 %s (袋改, 元表未动)\n", b->name, b->shengmu);
        return 0;
    }

    /* ── 册 声母 袋名 ── (自举核心: 把袋子的操作登记入元表) */
    if (strcmp(toks[0], "册") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
        if (!b->shengmu[0]) { printf("  ❌ 袋 %s 无声母, 无法册入元表\n", b->name); return -2; }
        /* 从元表寻袋当前声母对应的函数指针 */
        op_fn_t fn = meta_find(&it->meta, b->shengmu);
        if (!fn) { printf("  ❌ 声母 %s 尚在元表之外, 册不了\n", b->shengmu); return -2; }
        /* 以 toks[1] (新名字) 登记 */
        int existed = (meta_find(&it->meta, toks[1]) != NULL);
        int rc = meta_register(it, toks[1], fn);
        if (rc != 0) { printf("  ⚠️ 元表满, 册不入\n"); return -2; }
        printf("  📖 册 声母 %s → 元表 (自袋「%s」, 函数%p) %s\n",
               toks[1], b->name, (void*)fn, existed ? "[覆盖]" : "[新增]");
        return 0;
    }

    /* ── 用 袋名 袋A 袋B -> 袋C ── */
    if (strcmp(toks[0], "用") == 0) {
        if (nt < 6) return -2;
        bag_t *bag_op = drawer_find(&it->drawer, toks[1]);
        if (!bag_op) { printf("  ❓ 操作袋未找到: %s\n", toks[1]); return -2; }
        if (!bag_op->shengmu[0]) { printf("  ❓ 袋 %s 无声母\n", toks[1]); return -2; }
        op_fn_t fn = meta_find(&it->meta, bag_op->shengmu);
        if (!fn) { printf("  ❓ 声母 %s 未在元表\n", bag_op->shengmu); return -2; }
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        enum polarity pol = YIN;
        int r = fn(a->gongyueshu, b->gongyueshu, &pol);
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[5], r, "", &folded);
        interp_trace(it, r);
        if (folded) { printf("  ⚠️ 屉满\n"); return -1; }
        c->pol = pol;
        if (strcmp(bag_op->shengmu, "裁") == 0) a->pol = pol;
        printf("  ⚡ 用 %s[%s] %s %s → %s = %d %s\n",
               bag_op->name, bag_op->shengmu, a->name, b->name, c->name, r,
               pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 照 ── */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋数=%d 元表条目=%d 痕迹=%d\n",
                   it->drawer.count, it->meta.count, it->trace_n);
            for (int i = 0; i < it->meta.count; i++)
                printf("       表[%d] %-6s → fn@%p\n", i,
                       it->meta.entries[i].name, (void*)it->meta.entries[i].fn);
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

    /* ── 进/退/益/损/裁 作用于袋 (是元表查找,非 enum) ── */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        op_fn_t fn = meta_find(&it->meta, toks[0]);
        if (!fn) { printf("  ❓ 声母 %s 未在元表\n", toks[0]); return -2; }
        enum polarity pol = YIN;
        b->gongyueshu = fn(b->gongyueshu, 0, &pol);
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
    printf("\n【Step H2 完全自举 测试套件】\n");

    /* 1. 袋绑声母 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 合袋 = 20 合\n");
      bag_t *b = drawer_find(&it.drawer, "合袋");
      ASSERT(b && strcmp(b->shengmu, "合") == 0, "袋合袋 绑声母 合"); }

    /* 2. 用袋声母做加法 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "袋 a = 7\n"
        "袋 b = 13\n"
        "用 合袋 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 20, "用合袋[合]: 7+13=20"); }

    /* 3. 注: 换装不动元表 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 实验袋 = 5 益\n"
        "注 合 实验袋\n");
      bag_t *b = drawer_find(&it.drawer, "实验袋");
      ASSERT(b && strcmp(b->shengmu, "合") == 0, "注 合 → 实验袋改声母为合");
      ASSERT(meta_find(&it.meta, "合") != NULL, "元表原有 合 仍在"); }

    /* 4. 册: 袋子登记新声母入元表 (自举!) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "册 加 合袋\n");
      /* 现在元表多了一个 "加", 函数同 合 */
      op_fn_t fn = meta_find(&it.meta, "加");
      ASSERT(fn != NULL, "册 加 → 元表新增声母 加");
      enum polarity pol = YIN;
      int r = fn(7, 13, &pol);
      ASSERT(r == 20, "元表新声母 加 调用: 7+13=20"); }

    /* 5. 册后袋用新声母: 袋子自创的积木被复用 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 加袋 = 0 合\n"
        "册 加 加袋\n"         /* 把 合 登记为 加 */
        "袋 加袋2 = 0 加\n"    /* 新袋子直接穿 加 声母 */
        "袋 a = 5\n袋 b = 9\n"
        "用 加袋2 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 14, "册后用: 加袋2[加函数] 5+9=14"); }

    /* 6. 裁裁定 (函数指针版) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 裁袋 = 0 裁\n"
        "袋 好袋 = 25\n"
        "用 裁袋 好袋 好袋 -> 判袋\n");
      bag_t *b = drawer_find(&it.drawer, "好袋");
      ASSERT(b && b->pol == YANG, "裁: 25 >= 10 阳"); }

    /* 7. 元袋快照 + 元表打印 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "册 加 合袋\n"
        "袋 倍增袋 = 0 益\n"
        "册 倍 倍增袋\n"
        "照 元袋\n");
      ASSERT(it.meta.count >= 9, "元袋: 7种子 + 册入2个 ≥ 9"); }

    /* 8. 同袋多次册不同名 + 袋子穿上新声母后驱动: 完整自举环 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 倍袋 = 0 益\n"
        "册 倍 倍袋\n"         /* 元表新增 倍 */
        "册 乘 倍袋\n"         /* 元表新增 乘 (同函数不同名) */
        "袋 倍操 = 0 倍\n"     /* 袋子穿上新声母 倍 */
        "袋 乘操 = 0 乘\n"     /* 袋子穿上新声母 乘 */
        "袋 a = 6\n"
        "进 a\n"               /* a: 6→7 */
        "用 倍操 a a -> b\n"   /* 倍: 7*2=14 */
        "用 乘操 a a -> c\n"); /* 乘: 7*2=14 */
      bag_t *b = drawer_find(&it.drawer, "b");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(b && b->gongyueshu == 14, "倍: 7*2=14");
      ASSERT(c && c->gongyueshu == 14, "乘: 7*2=14 (同函数不同名)"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ═══════════════════════════════════════════════════════════════
 * 主演示
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H2 · L0 完全自举: 袋子即元表\n");
    printf("  袋/注/册/用/照 — 袋子携带声母,册入元表,袋子写出元表\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    const char *program =
        "# 囡囝共生 Step H2 — 袋子即元表, 完全自举\n"
        "袋 元袋 = 0\n"
        "\n"
        "# 1. 袋子绑声母\n"
        "袋 合袋 = 0 合\n"
        "袋 裁袋 = 0 裁\n"
        "袋 益袋 = 0 益\n"
        "\n"
        "# 2. 基本袋\n"
        "袋 容器甲 = 7\n"
        "袋 容器乙 = 13\n"
        "\n"
        "# 3. 用袋做加法\n"
        "用 合袋 容器甲 容器乙 -> 融合袋\n"
        "\n"
        "# 4. 用裁袋裁定\n"
        "用 裁袋 融合袋 融合袋 -> 判定袋\n"
        "\n"
        "# 5. 注 vs 册: 注改袋, 册写元表\n"
        "袋 实验袋 = 0 益\n"
        "注 合 实验袋\n"         /* 改为合, 元表不动 */
        "册 加 合袋\n"          /* 合袋的合 登记为 加 → 元表新增 */
        "册 倍 益袋\n"          /* 益袋的益 登记为 倍 → 元表新增 */
        "\n"
        "# 6. 用册来的声母\n"
        "袋 加袋 = 0 加\n"
        "袋 倍袋 = 0 倍\n"
        "袋 a = 5\n"
        "袋 b = 9\n"
        "用 加袋 a b -> 加结果\n"
        "用 倍袋 a b -> 倍结果\n"
        "\n"
        "# 7. 元袋自观: 元表从 7 扩到 9\n"
        "照 元袋\n"
        "\n"
        "# 8. 序列 : (a+b) 后倍\n"
        "用 倍袋 加结果 a -> 最终\n"
        "照 最终\n";

    printf("\n【第 1 步】自举 DSL 执行\n");
    run_script(&it, program);

    suite();

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H2 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  当前层号:    %d (最大公约数 %d ÷ %d)\n",
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].layer : 0,
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].gongyueshu : 0,
           RADIX);
    printf("  当前声母:    合/裁/益/... + 册入的 加/倍\n");
    printf("  当前韵母:    容器甲+容器乙 → 册成新声母 → 复用它袋\n");
    printf("  折叠状态:    %s\n",
           it.drawer.count >= MAX_BAGS ? "已折叠" : "屉未满(32)");
    printf("  下一步:      袋子自创组合积木 (\"册 新声母 = 袋A 袋B\")\n");
    printf("  袋子总数:    %d    元表: %d    痕迹: %d\n",
           it.drawer.count, it.meta.count, it.trace_n);
    printf("  测试结果:    %d 通过, %d 失败\n", g_pass, g_fail);
    if (g_fail == 0)
        printf("\n  ✅ Step H2 闭环 — 袋子即元表, 册入即自举, 袋子写出元表!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
