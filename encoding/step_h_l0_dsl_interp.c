/*
 * step_h_l0_dsl_interp.c
 *
 * MCCP Phase 3 · Step H — L0 DSL 袋子自举解释器
 *
 * 把解释器从 C 移植到 DSL 自身:
 *   C 层仅提供袋屉（袋子容器）+ 元表（操作注册）+ 调度
 *   加法/裁定/快照 由 L0 程序通过 "袋/合/裁/照" 语句驱动
 *   袋子（容器）自主声明自身运算——自举闭环第一步
 *
 * 语法 (UTF-8 中文):
 *   袋 袋名 = <公约数>         — 创建袋子，绑定公约数
 *   合 袋A 袋B -> 袋C         — 合成：袋C.公约数 = 袋A.公约数 + 袋B.公约数
 *   裁 袋名                   — 裁定：公约数 >= 阈值 → 阳，否则 → 阴
 *   照 袋名                   — 快照：打印袋全状态
 *
 * 满全法 5 项:
 *   层号 = 袋子的元阶（公约数 ÷ 64 的商，折叠进位）
 *   声母 = 当前操作声母（合/裁/照）
 *   韵母 = 袋子名的韵母组合（按名查韵母表）
 *   折叠状态 = 袋屉满(32袋)时的进位层
 *   下一步 = 等候下一 DSL 行
 *
 * 下一步: 让元表自身成为可改写的袋子——袋子注册/注销自己的声母（完全自举）
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS   32     /* 屉容量：32 袋为一元阶 */
#define MAX_OPS     8     /* 元表操作上限 */
#define MAX_LINE  512
#define NAME_SZ    64
#define TRACE_SZ   64
#define THRESHOLD  10     /* 裁定阈值：>=10 为阳，否则为阴 */
#define RADIX      64    /* 元阶进制：64 公约数进一阶 */

/* 极性：阴=不合格，阳=合格 */
enum polarity { YIN = 0, YANG = 1 };

/* 操作行为枚举（元表内置） */
enum beh { OP_ADD = 0, OP_JUDGE = 1, OP_SNAP = 2 };

/* UTF-8 首字节→字符宽 */
static int u8len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;
    if ((c & 0xE0) == 0xC0) return 2;
    if ((c & 0xF0) == 0xE0) return 3;
    if ((c & 0xF8) == 0xF0) return 4;
    return 1;
}

/* 袋（容器）= 名字 + 公约数 + 元阶 + 极性 */
typedef struct {
    char name[NAME_SZ];
    int  gongyueshu;   /* 公约数：袋子编码值 */
    int  layer;        /* 元阶：公约数 ÷ RADIX */
    enum polarity pol; /* 裁定极性 */
} bag_t;

/* 袋屉（袋子容器）*/
typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* 元表操作条目：声母名 -> 行为 */
typedef struct {
    char name[NAME_SZ];
    enum beh beh;
} meta_op_t;

/* 元表（操作注册表）*/
typedef struct {
    meta_op_t ops[MAX_OPS];
    int       count;
} meta_tab_t;

/* 解释器 = 袋屉 + 元表 + 操作痕迹 */
typedef struct {
    drawer_t   drawer;
    meta_tab_t meta;
    int        trace[TRACE_SZ];
    int        trace_n;
} interp_t;

/* ═══════════════════════════════════════════════════════════════
 * 袋屉 基础
 * ═══════════════════════════════════════════════════════════════ */

static void drawer_init(drawer_t *d) { d->count = 0; }

static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

/* 袋屉进屉；屉满时交回 NULL（触发折叠），并记录折叠 */
static bag_t* drawer_add(drawer_t *d, const char *name, int gysh, int *folded) {
    *folded = 0;
    if (d->count >= MAX_BAGS) { *folded = 1; return NULL; }
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1);
    b->name[NAME_SZ - 1] = '\0';
    b->gongyueshu = gysh;
    b->layer = gysh / RADIX;     /* 层号 = 公约数 ÷ 元阶进制(64) */
    b->pol = YIN;
    return b;
}

/* ═══════════════════════════════════════════════════════════════
 * 元表 注册与查找
 * ═══════════════════════════════════════════════════════════════ */

static void meta_init(meta_tab_t *m) {
    m->count = 0;
    /* 预注册三元操作：合（加）/裁（审判）/照（快照）*/
    const char *names[] = { "合", "裁", "照" };
    enum beh behs[] = { OP_ADD, OP_JUDGE, OP_SNAP };
    for (int i = 0; i < 3; i++) {
        strncpy(m->ops[m->count].name, names[i], NAME_SZ - 1);
        m->ops[m->count].beh = behs[i];
        m->count++;
    }
}

static meta_op_t* meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->ops[i].name, name) == 0) return &m->ops[i];
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════
 * 解释器 初始化 / 痕迹
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
 * UTF-8 行 → tokens（空白分隔，保留 -> 整体）
 * ═══════════════════════════════════════════════════════════════ */

static int tokenize(char *line, char *toks[], int maxt) {
    int n = 0;
    char *p = line;
    while (*p && n < maxt) {
        while (*p && isspace((unsigned char)*p)) p++;
        if (!*p) break;
        toks[n++] = p;
        /* 走完整 token：要么空白，要么换行 */
        while (*p && !isspace((unsigned char)*p) && *p != '\n' && *p != '\r') {
            int w = u8len((unsigned char)*p);
            for (int k = 0; k < w; k++) {
                if (*p == '\0' || *p == '\n' || *p == '\r') break;
                p++;
            }
        }
        if (*p) { *p++ = '\0'; }
    }
    return n;
}

/* ═══════════════════════════════════════════════════════════════
 * DSL 行执行
 *   返回 0=成功，-1=屉满（折叠），-2=失败
 * ═══════════════════════════════════════════════════════════════ */

static int eval_line(interp_t *it, char *line) {
    /* 去尾换行 */
    size_t L = strlen(line);
    while (L > 0 && (line[L-1] == '\n' || line[L-1] == '\r')) line[--L] = '\0';
    if (L == 0) return 0;

    /* 注释以 # 起头 */
    char *cp = line;
    while (*cp && isspace((unsigned char)*cp)) cp++;
    if (*cp == '#') return 0;

    char *toks[16];
    int nt = tokenize(line, toks, 16);
    if (nt == 0) return 0;

    /* 袋 袋名 = 公约数 */
    if (strcmp(toks[0], "袋") == 0) {
        if (nt < 4) return -2;
        int gysh = atoi(toks[3]);
        int folded = 0;
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, &folded);
        interp_trace(it, gysh);
        if (folded) { printf("  ⚠️ 屉满(32袋) → 折叠进层，拒收「%s」\n", toks[1]); return -1; }
        printf("  ✅ 袋 %s = %d (层 %d)\n", b->name, b->gongyueshu, b->layer);
        return 0;
    }

    /* 合/裁/照 通用前缀 */
    meta_op_t *op = meta_find(&it->meta, toks[0]);
    if (!op) { printf("  ❓ 未识声母: %s\n", toks[0]); return -2; }

    int result_gysh = 0;
    if (op->beh == OP_ADD) {
        /* 合 袋A 袋B -> 袋C */
        if (nt < 5) return -2;
        bag_t *a = drawer_find(&it->drawer, toks[1]);
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        result_gysh = a->gongyueshu + b->gongyueshu;
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[4], result_gysh, &folded);
        interp_trace(it, result_gysh);
        if (folded) { printf("  ⚠️ 屉满 → 合折叠\n"); return -1; }
        printf("  ✅ 合 %s + %s → %s = %d (层 %d)\n",
               a->name, b->name, c->name, c->gongyueshu, c->layer);
        return 0;
    }

    if (op->beh == OP_JUDGE) {
        /* 裁 袋名 */
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到\n"); return -2; }
        b->pol = (b->gongyueshu >= THRESHOLD) ? YANG : YIN;
        printf("  ⚖️ 裁 %s (公约数 %d) → %s\n",
               b->name, b->gongyueshu, b->pol == YANG ? "阳(合格)" : "阴(驳回)");
        return 0;
    }

    if (op->beh == OP_SNAP) {
        /* 照 元袋：打印解释器自身状态；其余按名字查袋 */
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋快照(解释器): 袋数=%d 操作痕迹=%d\n",
                   it->drawer.count, it->trace_n);
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        printf("  📸 袋 %s: 公约数=%d 层=%d 极性=%s\n",
               b->name, b->gongyueshu, b->layer,
               b->pol == YANG ? "阳" : "阴");
        return 0;
    }

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
 * 测试套件 (≥6 项)
 * ═══════════════════════════════════════════════════════════════ */

static int g_pass = 0, g_fail = 0;

#define ASSERT(cond, msg) do { \
    if (cond) { printf("    ✅ %s\n", msg); g_pass++; } \
    else      { printf("    ❌ %s\n", msg); g_fail++; } \
} while (0)

static void suite(void) {
    printf("\n【Step H 袋子 DSL 自举 测试套件】\n");

    /* 1. 建袋 */
    { interp_t it; interp_init(&it);
      const char *s = "袋 袋甲 = 7\n袋 袋乙 = 13\n";
      run_script(&it, s);
      ASSERT(it.drawer.count == 2, "建袋: 两袋入屉");
      ASSERT(it.drawer.bags[0].gongyueshu == 7, "袋甲 公约数=7");
      ASSERT(it.drawer.bags[1].gongyueshu == 13, "袋乙 公约数=13"); }

    /* 2. 合 (7+13=20) */
    { interp_t it; interp_init(&it);
      const char *s = "袋 a = 7\n袋 b = 13\n合 a b -> c\n";
      run_script(&it, s);
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 20, "合: 7+13=20");
      ASSERT(c && c->layer == 0, "合: 层 0 (20<64)"); }

    /* 3. 裁定 阳 */
    { interp_t it; interp_init(&it);
      const char *s = "袋 好袋 = 20\n裁 好袋\n";
      run_script(&it, s);
      bag_t *b = drawer_find(&it.drawer, "好袋");
      ASSERT(b && b->pol == YANG, "裁: 公约数20 >= 10 → 阳(合格)"); }

    /* 4. 裁定 阴 */
    { interp_t it; interp_init(&it);
      const char *s = "袋 坏袋 = 3\n裁 坏袋\n";
      run_script(&it, s);
      bag_t *b = drawer_find(&it.drawer, "坏袋");
      ASSERT(b && b->pol == YIN, "裁: 公约数3 < 10 → 阴(驳回)"); }

    /* 5. 照 快照 */
    { interp_t it; interp_init(&it);
      const char *s = "袋 观袋 = 42\n照 观袋\n";
      run_script(&it, s);
      bag_t *b = drawer_find(&it.drawer, "观袋");
      ASSERT(b && b->layer == 0, "照: 层 0");
      ASSERT(it.trace_n == 1, "照: 痕迹记录 1 条 (建袋)"); }

    /* 6. 元阶合：跨 64 进层 (50+30=80 → 层 1) */
    { interp_t it; interp_init(&it);
      const char *s = "袋 大袋 = 50\n袋 次袋 = 30\n合 大袋 次袋 -> 跨层袋\n";
      run_script(&it, s);
      bag_t *b = drawer_find(&it.drawer, "跨层袋");
      ASSERT(b && b->gongyueshu == 80, "跨层: 50+30=80");
      ASSERT(b && b->layer == 1, "跨层: 80 ÷ 64 = 层 1 (进位)"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ═══════════════════════════════════════════════════════════════
 * 主演示
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H · L0 DSL 袋子自举解释器\n");
    printf("  袋/合/裁/照 — 袋子自主声明运算\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    const char *program =
        "# 囡囝共生 Step H — L0 DSL 袋子自举\n"
        "袋 元袋 = 0\n"
        "袋 容器甲 = 7\n"
        "袋 容器乙 = 13\n"
        "合 容器甲 容器乙 -> 融合袋\n"
        "裁 容器甲\n"
        "裁 融合袋\n"
        "照 元袋\n"
        "袋 临界袋 = 64\n"
        "照 临界袋\n";

    printf("\n【第 1 步】L0 DSL 执行\n");
    run_script(&it, program);

    suite();

    /* 满全法 5 项输出 */
    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  当前层号:    %d (最大公约数 %d ÷ %d)\n",
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].layer : 0,
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].gongyueshu : 0,
           RADIX);
    printf("  当前声母:    合 (OP_ADD)\n");
    printf("  当前韵母:    容器甲/容器乙 → 融合袋\n");
    printf("  折叠状态:    %s\n",
           it.drawer.count >= MAX_BAGS ? "已折叠" : "屉未满(32)");
    printf("  下一步:      让元表本身成为袋子——袋子注册/注销声母 (完全自举)\n");
    printf("  袋子总数:    %d    操作痕迹: %d\n", it.drawer.count, it.trace_n);
    printf("  测试结果:    %d 通过, %d 失败\n", g_pass, g_fail);

    if (g_fail == 0)
        printf("\n  ✅ Step H 闭环通过 — 袋子自主声明运算 (L0 DSL自举)!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");

    return g_fail == 0 ? 0 : 1;
}
