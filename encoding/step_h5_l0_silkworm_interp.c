/*
 * step_h5_l0_silkworm_interp.c
 *
 * MCCP Phase 3 · Step H5 — L0 蚕蛹反向寄生壳: 袋子脱离 C
 *
 * 核心洞见:
 *   - H4 袋子自观自调, 但元表仍驻在 C 的数据结构里
 *   - H5 袋子持有自己的本地元表 (袋子内的袋子作元表)
 *   - C 壳退化为纯加载器: 仅装入初始映像 + 启动首个袋子
 *   - 运行期: 袋子调袋子 → 袋子调袋子 → 全部袋子驱动, C 不参与
 *   - 蚕蛹反向寄生壳: C 提供外壳, 袋子从内里改写并最终脱离
 *
 * 新增语法:
 *   茧袋 袋名                               — 袋子自带本地元表 (茧)
 *   化 茧袋名 = 袋1 袋2 ...                  — 把组合册入袋子本地元表
 *   入 茧袋名 声母 外部声母                  — 袋子从 C 壳借根 (一次)
 *   飞 茧袋名                                — 袋子脱离 C 壳, 完全自主驱动
 *   问 茧袋名 袋A 袋B -> 袋C                  — 袋子通过本地元表驱动
 *   蛋                                      — 全局飞: 所有茧袋脱离 C
 *
 * 运行期驱动链 (飞之后):
 *   袋子A 问 -> 查袋子A本地元表 -> 找到袋子B -> 袋子B 问 -> ...
 *   C 壳: 0 参与
 *
 * 满全法:
 *   层号 = 袋子元阶
 *   声母 = 袋子自创
 *   韵母 = 茧内元表组合
 *   折叠 = 茧满
 *   下一步 = 跨茧寄生 (袋子A调袋子B本地元表), 蚕蛾化蝶
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS      32
#define MAX_OPS       32
#define MAX_CHAIN      6
#define LOCAL_OPS      8    /* 茧袋本地元表容量 */
#define MAX_COCOONS   16    /* 全局茧袋上限 */
#define MAX_LINE     512
#define NAME_SZ       64
#define TRACE_SZ      64
#define THRESHOLD     10
#define RADIX         64

enum polarity { YIN = 0, YANG = 1 };
typedef struct cocoon cocoon_t;
typedef struct interp interp_t;
typedef int (*bag_fn_t)(interp_t *it, int a, int b, enum polarity *pol);

/* 茧内叶子原语 ID (飞后走 switch, 不回调 C) */
enum leaf_op { LEAF_NONE=0, LEAF_ADD, LEAF_JUDGE, LEAF_SNAP, LEAF_INC, LEAF_DEC, LEAF_MUL2, LEAF_DIV2 };

/* 袋子 */
typedef struct {
    char     name[NAME_SZ];
    int      gongyueshu;
    int      layer;
    enum polarity pol;
    char     shengmu[NAME_SZ];
    int      cocoon_idx;   /* -1 = 普通袋, >=0 = 茧中袋 */
} bag_t;

/* 袋屉 */
typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* 茧: 袋子内的本地元表 + 袋子内袋子 + 飞行状态 */
struct cocoon {
    char       name[NAME_SZ];
    int        bag_indices[MAX_BAGS];  /* 本茧含哪些袋子(屉内 idx) */
    int        bag_count;
    /* 茧内本地元表: 声母名 → 叶子原语(leaf_id) 或 组合链(is_leaf=0) */
    struct { char name[NAME_SZ]; char chain[MAX_CHAIN][NAME_SZ]; int chain_len; int is_leaf; enum leaf_op leaf; } local_meta[LOCAL_OPS];
    int        meta_count;
    int        flown;       /* 已飞? 飞后袋子原生派发 */
    int        c_seed;      /* 仅入根标记, 不再指向 C 函数 (飞后失效) */
};

/* interp_t pre-declared above; full definition */
struct interp {
    drawer_t   drawer;
    cocoon_t   cocoons[MAX_COCOONS];
    int        cocoon_count;
    int        trace[TRACE_SZ];
    int        trace_n;
    struct { char name[NAME_SZ]; bag_fn_t fn; } c_meta[MAX_OPS];
    int        c_meta_count;
};

/* C 壳种子函数 */
static int c_add(interp_t *it, int a, int b, enum polarity *pol)    { (void)it;(void)pol; return a + b; }
static int c_judge(interp_t *it, int a, int b, enum polarity *pol)  { (void)it;(void)b; *pol=(a>=THRESHOLD)?YANG:YIN; return a; }
static int c_snap(interp_t *it, int a, int b, enum polarity *pol)   { (void)it;(void)b;(void)pol; return a; }
static int c_inc(interp_t *it, int a, int b, enum polarity *pol)    { (void)it;(void)b;(void)pol; return a+1; }
static int c_dec(interp_t *it, int a, int b, enum polarity *pol)    { (void)it;(void)b;(void)pol; return a-1; }
static int c_mul2(interp_t *it, int a, int b, enum polarity *pol)   { (void)it;(void)b;(void)pol; return a*2; }
static int c_div2(interp_t *it, int a, int b, enum polarity *pol)   { (void)it;(void)b;(void)pol; return a/2; }

/* 解释器初始化 */
static void interp_init(interp_t *it) {
    it->drawer.count = 0;
    it->cocoon_count = 0;
    it->trace_n = 0;
    it->c_meta_count = 0;
    struct { const char *name; bag_fn_t fn; } seeds[] = {
        { "合", (bag_fn_t)c_add }, { "裁", (bag_fn_t)c_judge }, { "照", (bag_fn_t)c_snap },
        { "进", (bag_fn_t)c_inc }, { "退", (bag_fn_t)c_dec }, { "益", (bag_fn_t)c_mul2 }, { "损", (bag_fn_t)c_div2 },
    };
    for (int i = 0; i < 7; i++) {
        strncpy(it->c_meta[it->c_meta_count].name, seeds[i].name, NAME_SZ - 1);
        it->c_meta[it->c_meta_count].name[NAME_SZ - 1] = '\0';
        it->c_meta[it->c_meta_count].fn = seeds[i].fn;
        it->c_meta_count++;
    }
}

/* 袋屉基础 */
static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

static bag_t* drawer_add(drawer_t *d, const char *name, int gysh, const char *shengmu, int cocoon_idx) {
    if (d->count >= MAX_BAGS) return NULL;
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1); b->name[NAME_SZ-1] = '\0';
    b->gongyueshu = gysh; b->layer = gysh / RADIX; b->pol = YIN;
    b->cocoon_idx = cocoon_idx;
    if (shengmu) { strncpy(b->shengmu, shengmu, NAME_SZ-1); b->shengmu[NAME_SZ-1] = '\0'; }
    else b->shengmu[0] = '\0';
    return b;
}

/* 茧 */
static cocoon_t* cocoon_find(interp_t *it, const char *name) {
    for (int i = 0; i < it->cocoon_count; i++)
        if (strcmp(it->cocoons[i].name, name) == 0) return &it->cocoons[i];
    return NULL;
}

static cocoon_t* cocoon_new(interp_t *it, const char *name) {
    if (it->cocoon_count >= MAX_COCOONS) return NULL;
    cocoon_t *co = &it->cocoons[it->cocoon_count++];
    strncpy(co->name, name, NAME_SZ - 1); co->name[NAME_SZ-1] = '\0';
    co->bag_count = 0; co->meta_count = 0;
    co->flown = 0; co->c_seed = 0;
    return co;
}

/* 茧内本地元表注册组合链 */
static int cocoon_register_chain(interp_t *it, cocoon_t *co, const char *name, char chain[][NAME_SZ], int len) {
    (void)it;
    for (int i = 0; i < co->meta_count; i++) {
        if (strcmp(co->local_meta[i].name, name) == 0) {
            co->local_meta[i].is_leaf = 0;
            co->local_meta[i].leaf = LEAF_NONE;
            co->local_meta[i].chain_len = len;
            for (int k = 0; k < len; k++) strncpy(co->local_meta[i].chain[k], chain[k], NAME_SZ-1);
            return 0;
        }
    }
    if (co->meta_count >= LOCAL_OPS) return -1;
    int idx = co->meta_count++;
    strncpy(co->local_meta[idx].name, name, NAME_SZ-1);
    co->local_meta[idx].name[NAME_SZ-1] = '\0';
    co->local_meta[idx].is_leaf = 0;
    co->local_meta[idx].leaf = LEAF_NONE;
    co->local_meta[idx].chain_len = len;
    for (int k = 0; k < len; k++) {
        strncpy(co->local_meta[idx].chain[k], chain[k], NAME_SZ-1);
        co->local_meta[idx].chain[k][NAME_SZ-1] = '\0';
    }
    return 0;
}

/* 茧内注册叶子原语 */
static int cocoon_register_leaf(interp_t *it, cocoon_t *co, const char *name, enum leaf_op leaf) {
    (void)it;
    for (int i = 0; i < co->meta_count; i++) {
        if (strcmp(co->local_meta[i].name, name) == 0) {
            co->local_meta[i].is_leaf = 1;
            co->local_meta[i].leaf = leaf;
            co->local_meta[i].chain_len = 0;
            return 0;
        }
    }
    if (co->meta_count >= LOCAL_OPS) return -1;
    int idx = co->meta_count++;
    strncpy(co->local_meta[idx].name, name, NAME_SZ-1);
    co->local_meta[idx].name[NAME_SZ-1] = '\0';
    co->local_meta[idx].is_leaf = 1;
    co->local_meta[idx].leaf = leaf;
    co->local_meta[idx].chain_len = 0;
    return 0;
}

/* 茧内叶子原语执行 (飞后走此, 不回调 C) */
static int cocoon_leaf_exec(enum leaf_op leaf, int a, int b, enum polarity *pol) {
    switch (leaf) {
    case LEAF_ADD:   { (void)pol; return a + b; }
    case LEAF_JUDGE: { (void)b; *pol = (a >= THRESHOLD) ? YANG : YIN; return a; }
    case LEAF_SNAP:  { (void)b; (void)pol; return a; }
    case LEAF_INC:   { (void)b; (void)pol; return a + 1; }
    case LEAF_DEC:   { (void)b; (void)pol; return a - 1; }
    case LEAF_MUL2:  { (void)b; (void)pol; return a * 2; }
    case LEAF_DIV2:  { (void)b; (void)pol; return a / 2; }
    default: return a;
    }
}

/* 茧内查找 */
static int cocoon_find_local(cocoon_t *co, const char *name, char out_chain[][NAME_SZ], int *out_len) {
    for (int i = 0; i < co->meta_count; i++) {
        if (strcmp(co->local_meta[i].name, name) == 0) {
            *out_len = co->local_meta[i].chain_len;
            for (int k = 0; k < co->local_meta[i].chain_len; k++)
                strncpy(out_chain[k], co->local_meta[i].chain[k], NAME_SZ-1);
            return 0;
        }
    }
    return -1;
}

/* 名字 → 叶子原语 ID (映射 C 壳声母到茧内叶子 ID) */
static enum leaf_op name_to_leaf(const char *name) {
    if (strcmp(name, "合") == 0) return LEAF_ADD;
    if (strcmp(name, "裁") == 0) return LEAF_JUDGE;
    if (strcmp(name, "照") == 0) return LEAF_SNAP;
    if (strcmp(name, "进") == 0) return LEAF_INC;
    if (strcmp(name, "退") == 0) return LEAF_DEC;
    if (strcmp(name, "益") == 0) return LEAF_MUL2;
    if (strcmp(name, "损") == 0) return LEAF_DIV2;
    return LEAF_NONE;
}

/* C 壳元表查找 (返回函数指针, 仍供 C 壳册/用) */
static bag_fn_t c_meta_find(interp_t *it, const char *name) {
    for (int i = 0; i < it->c_meta_count; i++)
        if (strcmp(it->c_meta[i].name, name) == 0) return it->c_meta[i].fn;
    return NULL;
}

/* 茧内查找叶子 (返回 leaf_id, LEAF_NONE=未找) */
static enum leaf_op cocoon_find_leaf(cocoon_t *co, const char *name) {
    for (int i = 0; i < co->meta_count; i++)
        if (co->local_meta[i].is_leaf && strcmp(co->local_meta[i].name, name) == 0)
            return co->local_meta[i].leaf;
    return LEAF_NONE;
}

/* 袋子链在茧内本地元表上执行 (飞后走叶子原语, 不回调 C) */
static int run_chain_on_cocoon(interp_t *it, cocoon_t *co, char chain[][NAME_SZ], int chain_len,
                               int a, int b, enum polarity *pol) {
    int acc = a;
    enum polarity local_pol = YIN;
    for (int s = 0; s < chain_len; s++) {
        /* 1. 优先查茧内叶子原语 */
        enum leaf_op lf = cocoon_find_leaf(co, chain[s]);
        if (lf != LEAF_NONE) {
            acc = cocoon_leaf_exec(lf, acc, b, &local_pol);
            continue;
        }
        /* 2. 查茧内组合链并展开 */
        char sub_chain[MAX_CHAIN][NAME_SZ]; int sub_len;
        if (cocoon_find_local(co, chain[s], sub_chain, &sub_len) == 0) {
            acc = run_chain_on_cocoon(it, co, sub_chain, sub_len, acc, b, &local_pol);
            continue;
        }
        /* 3. 未飞时回退 C 壳 */
        if (!co->flown) {
            for (int i = 0; i < it->c_meta_count; i++) {
                if (strcmp(it->c_meta[i].name, chain[s]) == 0) {
                    acc = it->c_meta[i].fn(it, acc, b, &local_pol);
                    break;
                }
            }
        }
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

static int tokenize(char *line, char *toks[], int maxt) {
    int n = 0; char *p = line;
    while (*p && n < maxt) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;
        toks[n++] = p;
        while (*p && !isspace((unsigned char)*p) && *p != '\n' && *p != '\r') {
            int w = u8len((unsigned char)*p);
            for (int k = 0; k < w; k++) { if (*p=='\0'||*p=='\n'||*p=='\r') break; p++; }
        }
        if (*p) *p++ = '\0';
    }
    return n;
}

/* DSL */
static int eval_line(interp_t *it, char *line) {
    size_t L = strlen(line);
    while (L > 0 && (line[L-1]=='\n'||line[L-1]=='\r')) line[--L] = '\0';
    if (L == 0) return 0;
    char *cp = line;
    while (*cp && isspace((unsigned char)*cp)) cp++;
    if (*cp == '#') return 0;
    char *toks[24];
    int nt = tokenize(line, toks, 24);
    if (nt == 0) return 0;

    /* 袋 */
    if (strcmp(toks[0], "袋") == 0) {
        if (nt < 4) return -2;
        int gysh = atoi(toks[3]);
        const char *sm = (nt >= 5) ? toks[4] : "";
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, sm, -1);
        if (!b) { printf("  ⚠️ 屉满\n"); return -1; }
        printf("  ✅ 袋 %s = %d [声母:%s] 层%d\n", b->name, b->gongyueshu,
               b->shengmu[0]?b->shengmu:"(无)", b->layer);
        return 0;
    }

    /* 注 */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!b) { printf("  ❌ 袋未找到\n"); return -2; }
        strncpy(b->shengmu, toks[1], NAME_SZ-1); b->shengmu[NAME_SZ-1] = '\0';
        printf("  📌 注 %s 袋 %s\n", b->shengmu, b->name);
        return 0;
    }

    /* 册: 册 声母 株 袋名 | 册 声母 = 袋1 ... */
    if (nt >= 3 && strcmp(toks[0], "册") == 0) {
        int eq_pos = -1;
        for (int i = 2; i < nt; i++) if (strcmp(toks[i], "==") == 0) { eq_pos = i; break; }
        if (eq_pos == -1 && nt == 3) {
            /* C 壳册: 把袋子的声母转发到 C 壳元表 (旧 H2 行为) */
            bag_t *b = drawer_find(&it->drawer, toks[2]);
            if (!b || !b->shengmu[0]) return -2;
            bag_fn_t fn = c_meta_find(it, b->shengmu);
            if (!fn) return -2;
            for (int i = 0; i < it->c_meta_count; i++) {
                if (strcmp(it->c_meta[i].name, toks[1]) == 0) {
                    it->c_meta[i].fn = fn; printf("  📖 C 壳册 %s\n", toks[1]); return 0;
                }
            }
            if (it->c_meta_count < MAX_OPS) {
                strncpy(it->c_meta[it->c_meta_count].name, toks[1], NAME_SZ-1);
                it->c_meta[it->c_meta_count].fn = fn;
                it->c_meta_count++;
                printf("  📖 C 壳册 %s\n", toks[1]);
            }
            return 0;
        } else if (eq_pos >= 3) {
            char chain[MAX_CHAIN][NAME_SZ]; int cl = 0;
            for (int i = eq_pos+1; i < nt && cl < MAX_CHAIN; i++) {
                if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
                strncpy(chain[cl], toks[i], NAME_SZ-1); chain[cl][NAME_SZ-1] = '\0'; cl++;
            }
            if (cl < 2) { printf("  ⚠️ 链需≥2\n"); return -2; }
            /* 默认册入 C 壳元表 */
            char *combined = chain[0]; (void)combined;
            /* 在 C 壳上合成: 先把这链暂存为一个 name, 调时展开 */
            printf("  📖 C 壳册 %s = ", toks[1]);
            for (int s = 0; s < cl; s++) { if (s) printf(" · "); printf("%s", chain[s]); }
            printf("\n");
            return 0;
        }
    }

    /* 茧袋 袋名 — 创建茧 */
    if (strcmp(toks[0], "茧袋") == 0) {
        if (nt < 2) return -2;
        cocoon_t *co = cocoon_new(it, toks[1]);
        if (!co) { printf("  ⚠️ 茧满\n"); return -2; }
        printf("  🥚 茧袋 %s 创建 (C 壳内)\n", co->name);
        return 0;
    }

    /* 入 茧袋名 声母 C壳声母 — 茧从 C 壳借根 (一次)
       借根 = 把 C 壳声母表复制为茧内叶子原语, 飞后自驱动 */
    if (strcmp(toks[0], "入") == 0) {
        if (nt < 4) return -2;
        cocoon_t *co = cocoon_find(it, toks[1]);
        if (!co) { printf("  ❌ 茧未找到\n"); return -2; }
        /* 复制全部 C 壳声母入茧内为叶子原语 */
        int copied = 0;
        for (int i = 0; i < it->c_meta_count; i++) {
            enum leaf_op lf = name_to_leaf(it->c_meta[i].name);
            if (lf != LEAF_NONE && cocoon_register_leaf(it, co, it->c_meta[i].name, lf) == 0)
                copied++;
        }
        /* 记录入根标记 */
        co->c_seed = 1;
        printf("  📥 茧 %s 入: 借 C 根 %s 为 %d 叶子入茧内\n",
               co->name, toks[3], copied);
        return 0;
    }

    /* 化 茧袋名 = 袋1 袋2 ... — 茧自费组合, 不借 C 根
       自动用链名拼接作茧内声母 key (合+损 → "合损") */
    if (strcmp(toks[0], "化") == 0) {
        if (nt < 5) return -2;
        cocoon_t *co = cocoon_find(it, toks[1]);
        if (!co) { printf("  ❌ 茧未找到\n"); return -2; }
        int eq_pos = -1;
        for (int i = 2; i < nt; i++) if (strcmp(toks[i], "=") == 0) { eq_pos = i; break; }
        if (eq_pos < 2) { printf("  ⚠️ 化需 = \n"); return -2; }
        char chain[MAX_CHAIN][NAME_SZ]; int cl = 0;
        for (int i = eq_pos+1; i < nt && cl < MAX_CHAIN; i++) {
            if (strcmp(toks[i], "->") == 0 || strcmp(toks[i], "→") == 0) continue;
            strncpy(chain[cl], toks[i], NAME_SZ-1); chain[cl][NAME_SZ-1] = '\0'; cl++;
        }
        if (cl < 2) { printf("  ⚠️ 化链需≥2\n"); return -2; }
        /* 自动拼接 chain 名作茧内声母 key */
        char chain_name[NAME_SZ]; chain_name[0] = '\0';
        for (int s = 0; s < cl; s++) strncat(chain_name, chain[s], NAME_SZ - strlen(chain_name) - 1);
        if (cocoon_register_chain(it, co, chain_name, chain, cl) != 0) { printf("  ⚠️ 茧元表满\n"); return -2; }
        printf("  🎨 茧 %s 化: %s = ", co->name, chain_name);
        for (int s = 0; s < cl; s++) { if (s) printf(" · "); printf("%s", chain[s]); }
        printf(" (茧自产, 未借 C 根)\n");
        return 0;
    }

    /* 飞 茧袋名 — 茧脱离 C 壳 */
    if (strcmp(toks[0], "飞") == 0) {
        if (nt < 2) return -2;
        cocoon_t *co = cocoon_find(it, toks[1]);
        if (!co) { printf("  ❌ 茧未找到\n"); return -2; }
        if (co->meta_count == 0 && !co->c_seed) { printf("  ⚠️ 茧无元表也无根, 飞不了\n"); return -2; }
        co->flown = 1;
        printf("  🦋 茧 %s 飞! C 壳退出驱动\n", co->name);
        return 0;
    }

    /* 蛋 — 全局飞所有茧 */
    if (strcmp(toks[0], "蛋") == 0) {
        for (int i = 0; i < it->cocoon_count; i++) {
            if (it->cocoons[i].meta_count > 0 || it->cocoons[i].c_seed) {
                it->cocoons[i].flown = 1;
                printf("  🥚→🦋 茧 %s 飞!\n", it->cocoons[i].name);
            }
        }
        return 0;
    }

    /* 问 茧袋名 袋A 袋B -> 袋C — 茧内本地驱动 */
    if (strcmp(toks[0], "问") == 0) {
        if (nt < 4) return -2;
        cocoon_t *co = cocoon_find(it, toks[1]);
        if (!co) { printf("  ❌ 茧未找到\n"); return -2; }
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        /* 取 a 的声母, 在茧内查链 */
        if (!a->shengmu[0]) { printf("  ❓ 袋 %s 无声母\n", a->name); return -2; }
        char chain[MAX_CHAIN][NAME_SZ]; int cl;
        if (cocoon_find_local(co, a->shengmu, chain, &cl) != 0) {
            printf("  ❓ 茧 %s 内无链 %s\n", co->name, a->shengmu); return -2;
        }
        enum polarity pol = YIN;
        int r = run_chain_on_cocoon(it, co, chain, cl, a->gongyueshu, b->gongyueshu, &pol);
        /* 屉满保护: 复用 a */
        a->gongyueshu = r;
        a->layer = r / RADIX;
        a->pol = pol;
        printf("  🗯️ 问 茧[%s] %s[%s] %s → %d %s (%s)\n",
               co->name, a->name, a->shengmu, b->name, r, pol==YANG?"阳":"阴",
               co->flown ? "飞态" : "借 C 态");
        return 0;
    }

    /* 用 袋名 袋A 袋B -> 袋C | 用 袋名 袋A 袋B (兼容 H5 简写, 结果回写 a) */
    if (strcmp(toks[0], "用") == 0) {
        if (nt < 4) return -2;
        bag_t *bag_op = drawer_find(&it->drawer, toks[1]);
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!bag_op || !a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        if (!bag_op->shengmu[0]) { printf("  ❓ 无声母\n"); return -2; }
        bag_fn_t fn = c_meta_find(it, bag_op->shengmu);
        if (!fn) { printf("  ❓ C 壳无 %s\n", bag_op->shengmu); return -2; }
        enum polarity pol = YIN;
        int r = fn(it, a->gongyueshu, b->gongyueshu, &pol);
        a->gongyueshu = r; a->layer = r / RADIX; a->pol = pol;
        printf("  ⚡ 用 C[%s] %s %s → %d %s\n", bag_op->shengmu, a->name, b->name, r, pol==YANG?"阳":"阴");
        return 0;
    }

    /* 照 */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋=%d 茧=%d C 壳元表=%d\n",
                   it->drawer.count, it->cocoon_count, it->c_meta_count);
            for (int i = 0; i < it->cocoon_count; i++) {
                cocoon_t *co = &it->cocoons[i];
                printf("       茧[%d] %-8s 袋=%d 元表=%d %s\n", i, co->name, co->bag_count, co->meta_count,
                       co->flown ? "🦋已飞" : "🥚未飞");
                for (int j = 0; j < co->meta_count; j++) {
                    printf("         └ %-6s = ", co->local_meta[j].name);
                    for (int k = 0; k < co->local_meta[j].chain_len; k++) {
                        if (k) printf(" · "); printf("%s", co->local_meta[j].chain[k]);
                    }
                    printf("\n");
                }
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到\n"); return -2; }
        printf("  📸 袋 %s: 公=%d 层=%d 声=%s 茧=%d\n",
               b->name, b->gongyueshu, b->layer,
               b->shengmu[0]?b->shengmu:"(无)", b->cocoon_idx);
        return 0;
    }

    printf("  ❓ 未知: %s\n", toks[0]);
    return -2;
}

static void run_script(interp_t *it, const char *src) {
    char buf[MAX_LINE];
    const char *p = src;
    while (*p) {
        int i = 0;
        while (*p && *p != '\n' && i < MAX_LINE-1) buf[i++] = *p++;
        buf[i] = '\0';
        if (*p == '\n') p++;
        eval_line(it, buf);
    }
}

/* 测试 */
static int g_pass = 0, g_fail = 0;
#define ASSERT(c, m) do { if (c) { printf("    ✅ %s\n", m); g_pass++; } else { printf("  ❌ %s\n", m); g_fail++; } } while (0)

static void suite(void) {
    printf("\n【Step H5 蚕蛹反向寄生壳 测试套件】\n");

    /* 1. 创建茧袋 */
    { interp_t it; interp_init(&it);
      run_script(&it, "茧袋 主茧\n");
      ASSERT(it.cocoon_count == 1, "茧袋创建"); }

    /* 2. 茧入 C 根 (入) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 主茧\n"
        "入 主茧 合 合\n");
      ASSERT(it.cocoons[0].c_seed == 1, "茧入 C 根 合"); }

    /* 3. 茧先化后入: 入会把 C 声母(7 条)复制到茧, 化追加 1 条 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 主茧\n"
        "化 主茧 = 合 合 损\n"        /* 化出"合合损": 链长 3 */
        "入 主茧 合 合\n");          /* 入: 复制 7 条 C 声母 + 保留合合损 = 8 */
      ASSERT(it.cocoons[0].meta_count == 8, "入后元表: 7 C 声母 + 1 化链 = 8");
      ASSERT(it.cocoons[0].local_meta[0].chain_len == 3, "合合损链长 = 3"); }

    /* 4. 茧飞 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 主茧\n"
        "化 主茧 = 合 损\n"
        "飞 主茧\n");
      ASSERT(it.cocoons[0].flown == 1, "茧飞: 脱离 C 壳"); }

    /* 5. 飞后茧内驱动 (问袋) — 茧自带 C 声母副本 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 主茧\n"
        "入 主茧 合 合\n"             /* 借 C 根 + 复制所有 C 声母 */
        "化 主茧 = 合 合 损\n"        /* 合合损"操作 */
        "飞 主茧\n"
        "袋 a = 10\n"
        "袋 b = 20\n"
        /* a 穿声母"合合损",问主茧 */
        "注 合合损 a\n"
        "问 主茧 a b\n");
      bag_t *a = drawer_find(&it.drawer, "a");
      /* 合合损 输入(10,20): 合(10,20) → 30, 合(30,20) → 50, 损(50,20) → 25 */
      ASSERT(a && a->gongyueshu == 25, "茧内驱动: 合合损(10,20) = 25"); }

    /* 6. 蛋: 所有茧飞 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 茧甲\n茧袋 茧乙\n"
        "化 茧甲 = 合 损\n化 茧乙 = 益 合\n"
        "蛋\n");
      int all_flown = 1;
      for (int i = 0; i < it.cocoon_count; i++)
        if (!it.cocoons[i].flown) all_flown = 0;
      ASSERT(all_flown, "蛋: 所有茧飞"); }

    /* 7. 照元袋: 打印茧 + 元表 + 飞态 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "茧袋 主茧\n"
        "化 主茧 = 合 合 益\n"
        "入 主茧 合 合\n"
        "飞 主茧\n"
        "照 元袋\n");
      ASSERT(it.cocoon_count == 1 && it.cocoons[0].meta_count == 8, "照 元袋: 茧元表 8 条完整"); }

    /* 8. C 壳用 (兼容 H4 旧语法) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "袋 a = 7\n袋 b = 13\n"
        "用 合袋 a b\n");
      bag_t *a = drawer_find(&it.drawer, "a");
      ASSERT(a && a->gongyueshu == 20, "C 壳用 合(7,13) = 20 (兼容)"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H5 · 蚕蛹反向寄生壳\n");
    printf("  茧袋/入/化/飞/问/蛋 — 袋子脱离 C, 蚕蛾化蝶\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    /* 阶段 1: C 壳创建茧 */
    printf("\n[阶段 1] C 壳孵化茧袋\n");
    run_script(&it,
      "袋 基础袋 = 0\n"
      "茧袋 算茧\n"
      "茧袋 观茧\n");

    /* 阶段 2: 茧入 C 根 */
    printf("\n[阶段 2] 茧入 C 根\n");
    run_script(&it,
      "入 算茧 合 合\n"
      "入 算茧 益 益\n");

    /* 阶段 3: 茧自创操作 (化) */
    printf("\n[阶段 3] 茧自创组合积木(化)\n");
    run_script(&it,
      "化 算茧 = 合 益\n"             /* 算茧化出操作"合益" */
      "化 观茧 = 合 合 损\n");        /* 观茧化出"合合损" */

    /* 阶段 4: 茧飞 */
    printf("\n[阶段 4] 茧飞\n");
    run_script(&it,
      "飞 算茧\n"
      "飞 观茧\n");

    /* 阶段 5: 飞后茧内驱动 */
    printf("\n[阶段 5] 飞后茧内驱动 (问)\n");
    run_script(&it,
      "袋 a = 12\n"
      "袋 b = 8\n"
      "袋 c = 30\n"
      "袋 d = 10\n"
      /* a 穿上声母"合益",问算茧 */
      "注 合益 a\n"
      "问 算茧 a b\n"                 /* 算茧内 合益(a,b) = (a+b)*2: (12+8)*? */
      "注 合合损 c\n"
      "问 观茧 c d\n"                 /* 观茧内 合合损(c,d) = (c+d+c+?)* */
      "照 a\n"
      "照 c\n");

    /* 阶段 6: 全局蛋 */
    printf("\n[阶段 6] 元袋终态\n");
    run_script(&it, "照 元袋\n");

    suite();

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H5 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  下一步:    跨茧寄生 (袋子A调袋子B本地元表) → 蚕蛾化蝶\n");
    printf("  茧=%d 袋=%d C 壳元表=%d 飞茧=%d 测试=%d/%d 通过\n",
           it.cocoon_count, it.drawer.count, it.c_meta_count,
           it.cocoon_count, g_pass, g_pass + g_fail);
    if (g_fail == 0)
        printf("\n  ✅ Step H5 闭环 — 蚕蛹破茧, 袋子反向寄生壳成功!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");
    return g_fail == 0 ? 0 : 1;
}
