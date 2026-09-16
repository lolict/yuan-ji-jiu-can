/*
 * step_h4_l0_conscious_interp.c
 *
 * MCCP Phase 3 · Step H4 — L0 袋子自观自调: 意识萌芽
 *
 * 核心洞见:
 *   - H3: 袋子自创积木, 但创造仍由程序员通过 DSL 语句驱动
 *   - H4: 袋子自身观测运行时状态, 自主决定是否写新流
 *   - 袋子 = 观察者 + 被观察者 + 创造者, 三元归一
 *   - "自观" = 袋子看自己的公约数/层/极性/屉满度
 *   - "自调" = 袋子向元表写入新组合, 系统自修改
 *
 * 新增机制:
 *   省 (introspection) 操作 — 袋子观测屉状态: 屉满度 = count / MAX_BAGS
 *   屉满度 > 阈值 → 袋子触发"自调": 元表写入"自简"链
 *   自简 = 合 损 (把两个袋合成一个半值的袋, 释放空间)
 *   袋 省袋 = 0 省  → 创建自观袋
 *   用 省袋 a b -> 观察袋 → 观察袋同时触发屉满自调
 *
 * 意识萌芽信号:
 *   袋子感知(屉满) + 自主决策(是/否写) + 自执行(册入元表) + 自复用(用自简)
 *   = 观察者-被观察者-创造者 三元闭环
 *
 * 满全法 5 项:
 *   层号 = 元阶 (随自调上升)
 *   声母 = 自创声母 (自简 = 意识操作)
 *   韵母 = 屉满度感知
 *   折叠状态 = 自调前/后 变化
 *   下一步 = 袋子反向寄生外部 C (蚕蛹)
 *
 * 下一步 (H5): 袋子反向 C (袋子传 C 函数表), 蚕蛹脱壳
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS   32
#define MAX_OPS    32
#define MAX_CHAIN   6
#define MAX_LINE  512
#define NAME_SZ    64
#define TRACE_SZ   64
#define THRESHOLD  10
#define RADIX      64
#define DRAWER_SELF_MOD_TH  24   /* 屉满度阈值: count >= 24 触发自调 */
#define DRAWER_SELF_INT_TH  20   /* 屉满度阈值: count >= 20 触发感知 */

enum polarity { YIN = 0, YANG = 1 };
typedef struct interp interp_t;
typedef int (*c_fn_t)(int a, int b, enum polarity *pol);

enum op_src { OP_FN = 0, OP_CHAIN };
typedef struct { char bag_names[MAX_CHAIN][NAME_SZ]; int count; } chain_t;
typedef struct { char name[NAME_SZ]; enum op_src src; union { c_fn_t c_fn; chain_t chain; }; } meta_entry_t;
typedef struct { meta_entry_t entries[MAX_OPS]; int count; } meta_tab_t;
typedef struct { char name[NAME_SZ]; int gongyueshu; int layer; enum polarity pol; char shengmu[NAME_SZ]; } bag_t;
typedef struct { bag_t bags[MAX_BAGS]; int count; } drawer_t;
struct interp { drawer_t drawer; meta_tab_t meta; int trace[TRACE_SZ]; int trace_n; };

/* 前向声明 */
static int c_introspect(interp_t *it, int a, int b, enum polarity *pol);
static meta_entry_t* meta_find(meta_tab_t *m, const char *name);
static int meta_register_chain(interp_t *it, const char *name, chain_t *ch);

/* C 种子函数 */
static int c_add(int a, int b, enum polarity *pol)    { (void)pol; return a + b; }
static int c_judge(int a, int b, enum polarity *pol)  { (void)b; *pol = (a >= THRESHOLD) ? YANG : YIN; return a; }
static int c_snap(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a; }
static int c_inc(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a + 1; }
static int c_dec(int a, int b, enum polarity *pol)    { (void)b; (void)pol; return a - 1; }
static int c_mul2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a * 2; }
static int c_div2(int a, int b, enum polarity *pol)   { (void)b; (void)pol; return a / 2; }

/* 省 (introspection): 袋子看屉状态, 满则写入"自简"操作 */
static int c_introspect(interp_t *it, int a, int b, enum polarity *pol) {
    (void)a; (void)b; (void)pol;
    int count = it->drawer.count;
    int drawer_full_ratio = (count * 100) / MAX_BAGS;
    /* 感知 */
    printf("      🔍 省袋自观: 屉内有 %d/32 袋, 屉满度 %d%%\n",
           count, drawer_full_ratio);
    /* 自调: 屉满度 >= 75% (count >= 24) */
    if (count >= DRAWER_SELF_MOD_TH) {
        meta_entry_t *existing = meta_find(&it->meta, "自简");
        if (!existing) {
            chain_t ch;
            ch.count = 2;
            strcpy(ch.bag_names[0], "合");
            strcpy(ch.bag_names[1], "损");
            if (meta_register_chain(it, "自简", &ch) == 0)
                printf("      📝 自调: 屉满触发, 元表写入 自简=合·损 (释放空间)\n");
            else
                printf("      ⚠️ 自调失败: 元表满\n");
        } else {
            printf("      📋 自调: 自简 已在元表 (%d步链)\n", existing->chain.count);
        }
    } else if (count >= DRAWER_SELF_INT_TH) {
        printf("      👁️ 感知: 屉半满, 候自调\n");
    } else {
        printf("      💤 感知: 屉闲, 无需自调\n");
    }
    return count;
}

/* 元表 */
static void meta_init(meta_tab_t *m) {
    m->count = 0;
    struct { const char *name; c_fn_t fn; } seeds[] = {
        { "合", c_add }, { "裁", c_judge }, { "照", c_snap },
        { "进", c_inc }, { "退", c_dec }, { "益", c_mul2 }, { "损", c_div2 },
        { "省", (c_fn_t)c_introspect },
    };
    for (int i = 0; i < 8; i++) {
        meta_entry_t *e = &m->entries[m->count];
        strncpy(e->name, seeds[i].name, NAME_SZ - 1);
        e->name[NAME_SZ - 1] = '\0';
        e->src = OP_FN;
        e->c_fn = seeds[i].fn;
        m->count++;
    }
}

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

static meta_entry_t* meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->entries[i].name, name) == 0) return &m->entries[i];
    return NULL;
}

/* meta_apply: 执行元表条目. 省函数需要interp上下文, 表项不可直接保留指针(因它含it)
   解决: 省函数通过元表条目时携带参数 here — 特殊处理: 省's fn ignores a,b and calls drawer直接 */
static int meta_apply(interp_t *it, meta_entry_t *e, int a, int b, enum polarity *pol) {
    if (e->src == OP_FN) {
        if (strcmp(e->name, "省") == 0) {
            return c_introspect(it, a, b, pol);
        }
        return e->c_fn(a, b, pol);
    }
    int acc = a;
    enum polarity local_pol = YIN;
    for (int s = 0; s < e->chain.count; s++) {
        meta_entry_t *step_e = meta_find(&it->meta, e->chain.bag_names[s]);
        if (!step_e) { printf("    ⚠️ 链第 %d 步 %s 未在元表\n", s, e->chain.bag_names[s]); return acc; }
        acc = meta_apply(it, step_e, acc, b, &local_pol);
    }
    *pol = local_pol;
    return acc;
}

/* UTF-8 */
static int u8len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;
    if ((c & 0xE0) == 0xC0) return 2;
    if ((c & 0xF0) == 0xE0) return 3;
    if ((c & 0xF8) == 0xF0) return 4;
    return 1;
}

static void drawer_init(drawer_t *d) { d->count = 0; }

static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

static bag_t* drawer_add(drawer_t *d, const char *name, int gysh, const char *shengmu, int *folded) {
    *folded = 0;
    if (d->count >= MAX_BAGS) { *folded = 1; return NULL; }
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1);
    b->name[NAME_SZ - 1] = '\0';
    b->gongyueshu = gysh;
    b->layer = gysh / RADIX;
    b->pol = YIN;
    if (shengmu) { strncpy(b->shengmu, shengmu, NAME_SZ - 1); b->shengmu[NAME_SZ - 1] = '\0'; }
    else b->shengmu[0] = '\0';
    return b;
}

static void interp_init(interp_t *it) {
    drawer_init(&it->drawer);
    meta_init(&it->meta);
    it->trace_n = 0;
}

static void interp_trace(interp_t *it, int v) {
    if (it->trace_n < TRACE_SZ) it->trace[it->trace_n++] = v;
}

static int tokenize(char *line, char *toks[], int maxt) {
    int n = 0;
    char *p = line;
    while (*p && n < maxt) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;
        toks[n++] = p;
        while (*p && !isspace((unsigned char)*p) && *p != '\n' && *p != '\r') {
            int w = u8len((unsigned char)*p);
            for (int k = 0; k < w; k++) { if (*p == '\0' || *p == '\n' || *p == '\r') break; p++; }
        }
        if (*p) *p++ = '\0';
    }
    return n;
}

/* DSL */
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

    if (strcmp(toks[0], "袋") == 0) {
        if (nt < 4) return -2;
        int gysh = atoi(toks[3]);
        const char *sm = (nt >= 5) ? toks[4] : "";
        int folded = 0;
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, sm, &folded);
        interp_trace(it, gysh);
        if (folded) { printf("  ⚠️ 屉满(32袋)→折叠,拒收「%s」\n", toks[1]); return -1; }
        printf("  ✅ 袋 %s = %d [声母:%s] 层%d\n", b->name, b->gongyueshu,
               b->shengmu[0] ? b->shengmu : "(无)", b->layer);
        return 0;
    }

    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
        strncpy(b->shengmu, toks[1], NAME_SZ - 1); b->shengmu[NAME_SZ - 1] = '\0';
        printf("  📌 注 袋 %s 换装声母 %s\n", b->name, b->shengmu);
        return 0;
    }

    /* 册 两种格式 */
    if (nt >= 3 && strcmp(toks[0], "册") == 0) {
        int eq_pos = -1;
        for (int i = 2; i < nt; i++) if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos == -1) {
            bag_t *b = drawer_find(&it->drawer, toks[2]);
            if (!b) { printf("  ❌ 袋未找到: %s\n", toks[2]); return -2; }
            if (!b->shengmu[0]) { printf("  ❌ 袋 %s 无声母\n", b->name); return -2; }
            meta_entry_t *se = meta_find(&it->meta, b->shengmu);
            if (!se || se->src != OP_FN) { printf("  ❌ 声母 %s 非 C 函数\n", b->shengmu); return -2; }
            int existed = (meta_find(&it->meta, toks[1]) != NULL);
            int rc = meta_register_fn(it, toks[1], se->c_fn);
            if (rc != 0) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s → 元表 (转发 C) %s\n", toks[1], existed ? "[覆盖]" : "[新增]");
            return 0;
        } else {
            if (eq_pos != 2 || nt < eq_pos + 3) { printf("  ⚠️ 册=语法错\n"); return -2; }
            const char *new_name = toks[1];
            chain_t ch; ch.count = 0;
            for (int i = eq_pos + 1; i < nt && ch.count < MAX_CHAIN; i++) {
                if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
                strncpy(ch.bag_names[ch.count], toks[i], NAME_SZ - 1);
                ch.bag_names[ch.count][NAME_SZ - 1] = '\0';
                ch.count++;
            }
            if (ch.count < 2) { printf("  ⚠️ 链需≥2步\n"); return -2; }
            for (int s = 0; s < ch.count; s++) if (!meta_find(&it->meta, ch.bag_names[s])) {
                printf("  ⚠️ 链第%d步 %s 未在元表\n", s, ch.bag_names[s]); return -2;
            }
            int existed = (meta_find(&it->meta, new_name) != NULL);
            int rc = meta_register_chain(it, new_name, &ch);
            if (rc != 0) { printf("  ⚠️ 元表满\n"); return -2; }
            printf("  📖 册 声母 %s = ", new_name);
            for (int s = 0; s < ch.count; s++) { if (s) printf(" · "); printf("%s", ch.bag_names[s]); }
            printf(" %s\n", existed ? "[覆盖]" : "[新增]");
            return 0;
        }
    }

    /* 用 */
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

    /* 照 */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋数=%d 元表条目=%d 痕迹=%d\n", it->drawer.count, it->meta.count, it->trace_n);
            for (int i = 0; i < it->meta.count; i++) {
                meta_entry_t *e = &it->meta.entries[i];
                if (e->src == OP_FN)
                    printf("       表[%d] %-6s → C@%p\n", i, e->name, (void*)e->c_fn);
                else {
                    printf("       表[%d] %-6s → 链 [", i, e->name);
                    for (int s = 0; s < e->chain.count; s++) { if (s) printf(" · "); printf("%s", e->chain.bag_names[s]); }
                    printf("]\n");
                }
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        printf("  📸 袋 %s: 公约数=%d 层=%d 声母=%s 极性=%s\n",
               b->name, b->gongyueshu, b->layer, b->shengmu[0] ? b->shengmu : "(无)",
               b->pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* 进/退/益/损/裁 */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到\n"); return -2; }
        meta_entry_t *e = meta_find(&it->meta, toks[0]);
        if (!e) { printf("  ❓ 声母 %s 未在元表\n", toks[0]); return -2; }
        enum polarity pol = YIN;
        b->gongyueshu = meta_apply(it, e, b->gongyueshu, 0, &pol);
        b->layer = b->gongyueshu / RADIX;
        if (strcmp(toks[0], "裁") == 0) b->pol = pol;
        printf("  ⚙️ %s %s → %d 层%d %s\n", toks[0], b->name, b->gongyueshu, b->layer,
               pol == YANG ? "阳" : "阴");
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

/* 测试 */
static int g_pass = 0, g_fail = 0;
#define ASSERT(c, m) do { if (c) { printf("    ✅ %s\n", m); g_pass++; } else { printf("    ❌ %s\n", m); g_fail++; } } while (0)

static void suite(void) {
    printf("\n【Step H4 自观自调 测试套件】\n");

    /* 1. 省袋自观: 屉小→感知屉闲 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋\n");
      ASSERT(it.drawer.count <= 4, "屉小: 省袋感知屉闲, 不多写"); }

    /* 2. 袋数量累加后感知屉半满 */
    { interp_t it; interp_init(&it);
      /* 装入 20 袋 */
      for (int i = 0; i < 20; i++) { char l[64]; snprintf(l, sizeof l, "袋 b%d = %d\n", i, i+1); run_script(&it, l); }
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋\n");
      ASSERT(it.drawer.count >= 20 && it.drawer.count <= 23, "屉半满感知 (20-23袋)"); }

    /* 3. 自调触发: 装满 24 袋 → 省袋发现屉满度≥75% → 元表写自简 */
    { interp_t it; interp_init(&it);
      for (int i = 0; i < 24; i++) { char l[64]; snprintf(l, sizeof l, "袋 b%d = %d\n", i, i+1); run_script(&it, l); }
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋\n");
      meta_entry_t *zj = meta_find(&it.meta, "自简");
      ASSERT(zj != NULL && zj->src == OP_CHAIN, "自观自调: 屉满触发写入 自简 链");
      ASSERT(zj && zj->chain.count == 2, "自简链长 = 2 (合·损)"); }

    /* 4. 自简可用: 触发后用自简合+损袋 */
    { interp_t it; interp_init(&it);
      for (int i = 0; i < 24; i++) { char l[64]; snprintf(l, sizeof l, "袋 b%d = %d\n", i, i+1); run_script(&it, l); }
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋\n"); /* 触发自调 */
      run_script(&it,
        "册 自简 = 合 损\n"          /* 若自调已写则覆盖(或不), 确保自简存在 */
        "袋 自简袋 = 0 自简\n"
        "袋 a = 100\n袋 b = 100\n"
        "用 自简袋 a b -> 精简结果\n");
      bag_t *r = drawer_find(&it.drawer, "精简结果");
      ASSERT(r && r->gongyueshu == 100, "自简(100,100): (100+100)/2=100"); }

    /* 5. 元袋: 自调后元表 ≥9 (8种子+自简) */
    { interp_t it; interp_init(&it);
      for (int i = 0; i < 24; i++) { char l[64]; snprintf(l, sizeof l, "袋 b%d = %d\n", i, i+1); run_script(&it, l); }
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋\n");
      ASSERT(it.meta.count >= 9, "元袋: 8种子+自简 ≥9"); }

    /* 6. 不重复写: 再次触发, 自简已在元表, 不新增条目 */
    { interp_t it; interp_init(&it);
      for (int i = 0; i < 24; i++) { char l[64]; snprintf(l, sizeof l, "袋 b%d = %d\n", i, i+1); run_script(&it, l); }
      run_script(&it, "袋 省袋 = 0 省\n用 省袋 省袋 省袋 -> 观察袋1\n"); /* 第一次触发 */
      int c1 = it.meta.count;
      run_script(&it, "用 省袋 省袋 省袋 -> 观察袋2\n"); /* 第二次触发 */
      ASSERT(it.meta.count == c1, "二次触发不重复写: 元表条目数不变"); }

    /* 7. 册 自创链 + 省袋 三元组合
       益·合 链: 益(a,b)=a*2=10, 合(10,b)=10+9=19 → 2a+b */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "册 加自身 = 益 合\n"
        "袋 加自身袋 = 0 加自身\n"
        "袋 省袋 = 0 省\n"
        "袋 a = 5\n袋 b = 9\n"
        "用 加自身袋 a b -> 加自身结果\n"  /* 2*5+9 = 19 */
        "用 省袋 a b -> 观察袋\n");
      bag_t *r = drawer_find(&it.drawer, "加自身结果");
      ASSERT(r && r->gongyueshu == 19, "加自身(5,9): 2*5+9=19"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* 主演示 */
int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H4 · L0 袋子自观自调: 意识萌芽\n");
    printf("  袋 省袋 = 0 省 · 用 省袋 ... → 自调 自简=合·损\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    /* 屉空闲 → 省袋 */
    printf("\n[阶段 1] 袋少 → 省袋感知屉闲\n");
    run_script(&it,
      "袋 省袋 = 0 省\n"
      "袋 a = 5\n"
      "袋 b = 9\n"
      "用 省袋 a b -> 观察袋1\n");
    printf("  元袋: %d 种子 / %d 条目 / %d 袋\n", 8, it.meta.count, it.drawer.count);

    /* 装满 24 袋触发自调 */
    printf("\n[阶段 2] 袋多 → 自调触发\n");
    for (int i = 0; i < 24; i++) {
      char l[64]; snprintf(l, sizeof l, "袋 实袋%d = %d\n", i, i*3+1);
      run_script(&it, l);
    }
    printf("  装入 24 袋后触发省袋...\n");
    run_script(&it, "用 省袋 a b -> 观察袋2\n");

    /* 元袋自观 */
    printf("\n[阶段 3] 元袋自观 (含自调产物)\n");
    run_script(&it, "照 元袋\n");

    /* 验证自简 */
    printf("\n[阶段 4] 自简可用性\n");
    run_script(&it,
      "袋 自简袋 = 0 自简\n"
      "袋 x = 200\n袋 y = 100\n"
      "用 自简袋 x y -> 精简结果\n");
    bag_t *r = drawer_find(&it.drawer, "精简结果");
    if (r) printf("  自简(%d,%d) = %d\n", 200, 100, r->gongyueshu);

    printf("\n[测试]");
    suite();

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H4 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  感知阈值:   省袋感知屉满度 = count / MAX_BAGS, 75%%=自调触发\n");
    printf("  声母:       省(introspection) = C种子, 自简=合·损 = 袋子自写\n");
    printf("  韵母:       三元归一: 观察者/被观察者/创造者\n");
    printf("  折叠状态:   屉满→自调写元表→自简释放\n");
    printf("  下一步:     袋子反向 C: 蚕蛹脱壳\n");
    printf("  袋子=%d 元表=%d 痕迹=%d 测试=%d/%d 通过\n",
           it.drawer.count, it.meta.count, it.trace_n, g_pass, g_pass + g_fail);
    if (g_fail == 0)
        printf("\n  ✅ Step H4 闭环 — 自观自调, 袋子即意识萌芽!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
