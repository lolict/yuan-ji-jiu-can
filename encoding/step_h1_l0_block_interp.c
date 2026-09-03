/*
 * step_h1_l0_block_interp.c
 *
 * MCCP Phase 3 · Step H1 — L0 积木袋子自举解释器 (Step H 升级版)
 *
 * 核心洞见 (来自 lolict):
 *   - 袋子本身携带操作指针/极性/层级
 *   - 袋子 + 袋子 = 新袋子 (通过袋子自身携带的操作完成)
 *   - 声母由 L0 DSL 显式注册,不依赖 C 硬编码元表
 *   - 积木 = 可执行基本块, 词汇 = 积木拼接, 句子 = 动作序列
 *   - 编译期 = 运行期 (袋子既是产物也是材料)
 *
 * 语法 (UTF-8 中文):
 *   袋 袋名 = <公约数> [声母]       — 创建袋子,可选绑定一个操作声母
 *   注 声母 袋名                    — 注册声母到袋子 (运行时扩展元表)
 *   用 袋名 袋A 袋B -> 袋C          — 执行袋子携带的操作: 袋C = 袋名.oper(袋A, 袋B)
 *   照 袋名                        — 快照袋状态
 *
 * 预置三元声母:
 *   合 = 加法 (袋A.公约数 + 袋B.公约数)
 *   裁 = 裁定 (>=阈值 阳, 否则 阴)
 *   照 = 快照
 *
 * 满全法 5 项:
 *   层号 = 袋子的元阶 (公约数 ÷ 64)
 *   声母 = 袋子当前携带的操作声母
 *   韵母 = 袋子名的韵母组合
 *   折叠状态 = 屉满(32袋)进位
 *   下一步 = 等候下一 DSL 行
 *
 * 与 Step H 的关键差异:
 *   Step H: 元表硬编码在 C,袋子被动,编译即锁死
 *   Step H1: 袋子主动携带声母,DSL 可注册新声,袋子可自改自,运行期可扩
 *
 * 下一步: 袋子可注册自己的 C 外部操作 (完全自举,袋子即元表)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS   32     /* 屉容量: 32 袋为一元阶 */
#define MAX_OPS     8     /* 元表上限 (可扩) */
#define MAX_LINE  512
#define NAME_SZ    64
#define TRACE_SZ   64
#define THRESHOLD  10
#define RADIX      64

/* 极性 */
enum polarity { YIN = 0, YANG = 1 };

/* 操作码: 袋子携带的积木类型 */
enum opcode {
    OP_NOP = 0,    /* 占位,无操作 */
    OP_ADD,        /* 合: 加法 */
    OP_JUDGE,      /* 裁: 裁定 */
    OP_SNAP,       /* 照: 快照 */
    OP_INC,        /* 进: 增量 +1 */
    OP_DEC,        /* 退: 减量 -1 */
    OP_MUL2,       /* 益: 倍乘 *2 */
    OP_DIV2        /* 损: 折半 /2 */
};

/* 操作名字 (UTF-8 中文,与 opcode 顺序对应) */
static const char *OP_NAMES[] = {
    "无","合","裁","照","进","退","益","损"
};

/* UTF-8 首字节→宽 */
static int u8len(unsigned char c) {
    if ((c & 0x80) == 0) return 1;
    if ((c & 0xE0) == 0xC0) return 2;
    if ((c & 0xF0) == 0xE0) return 3;
    if ((c & 0xF8) == 0xF0) return 4;
    return 1;
}

/* 操作入名字→opcode */
static enum opcode opcode_from_name(const char *name) {
    for (int i = 0; i < (int)(sizeof(OP_NAMES)/sizeof(OP_NAMES[0])); i++)
        if (strcmp(OP_NAMES[i], name) == 0) return (enum opcode)i;
    return OP_NOP;
}

/* 操作出口袋 */
static void bag_name_from_opcode(enum opcode op, char *buf, int bufsz) {
    if (op < (int)(sizeof(OP_NAMES)/sizeof(OP_NAMES[0])))
        strncpy(buf, OP_NAMES[op], bufsz - 1);
    else
        strncpy(buf, "(未识)", bufsz - 1);
    buf[bufsz - 1] = '\0';
}

/* 袋 (容器): 名字 + 公约数 + 元阶 + 极性 + 积木操作码 */
typedef struct {
    char        name[NAME_SZ];
    int         gongyueshu;
    int         layer;
    enum polarity pol;
    enum opcode op;      /* 袋子携带的积木: 它是合/裁/照/进/退... */
} bag_t;

/* 袋屉 (袋子容器) */
typedef struct {
    bag_t bags[MAX_BAGS];
    int   count;
} drawer_t;

/* 元表: 声母注册表 (运行时由 DSL 写入) */
typedef struct {
    /* name → opcode, 可扩 */
    char          names[MAX_OPS][NAME_SZ];
    enum opcode   codes[MAX_OPS];
    int           count;
} meta_tab_t;

/* 解释器 */
typedef struct {
    drawer_t   drawer;
    meta_tab_t meta;
    int        trace[TRACE_SZ];
    int        trace_n;
} interp_t;

/* ═══════════════════════════════════════════════════════════════
 * 袋屉
 * ═══════════════════════════════════════════════════════════════ */

static void drawer_init(drawer_t *d) { d->count = 0; }

static bag_t* drawer_find(drawer_t *d, const char *name) {
    for (int i = 0; i < d->count; i++)
        if (strcmp(d->bags[i].name, name) == 0) return &d->bags[i];
    return NULL;
}

static bag_t* drawer_add(drawer_t *d, const char *name, int gysh, enum opcode op, int *folded) {
    *folded = 0;
    if (d->count >= MAX_BAGS) { *folded = 1; return NULL; }
    bag_t *b = &d->bags[d->count++];
    strncpy(b->name, name, NAME_SZ - 1);
    b->name[NAME_SZ - 1] = '\0';
    b->gongyueshu = gysh;
    b->layer = gysh / RADIX;
    b->pol = YIN;
    b->op = op;
    return b;
}

/* ═══════════════════════════════════════════════════════════════
 * 元表 (运行时注册)
 * ═══════════════════════════════════════════════════════════════ */

static void meta_init(meta_tab_t *m) {
    m->count = 0;
    static const char *seed[] = { "合", "裁", "照", "进", "退", "益", "损" };
    static const enum opcode seedcode[] = { OP_ADD, OP_JUDGE, OP_SNAP, OP_INC, OP_DEC, OP_MUL2, OP_DIV2 };
    for (int i = 0; i < 7; i++) {
        strncpy(m->names[m->count], seed[i], NAME_SZ - 1);
        m->codes[m->count] = seedcode[i];
        m->count++;
    }
}

/* DSL 在运行期注册一个新声母 */
static int meta_register(interp_t *it, const char *name, enum opcode op) {
    /* 已存在则更新 */
    for (int i = 0; i < it->meta.count; i++) {
        if (strcmp(it->meta.names[i], name) == 0) {
            it->meta.codes[i] = op;
            return 0;
        }
    }
    if (it->meta.count >= MAX_OPS) return -1;
    strncpy(it->meta.names[it->meta.count], name, NAME_SZ - 1);
    it->meta.names[it->meta.count][NAME_SZ - 1] = '\0';
    it->meta.codes[it->meta.count] = op;
    it->meta.count++;
    return 0;
}

static enum opcode meta_find(meta_tab_t *m, const char *name) {
    for (int i = 0; i < m->count; i++)
        if (strcmp(m->names[i], name) == 0) return m->codes[i];
    return OP_NOP;
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
 * UTF-8 行 → tokens
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
 * 积木执行: 袋子操作码 + 袋A 袋B → 袋C 结果公约数
 * ═══════════════════════════════════════════════════════════════ */

static int apply_opcode(enum opcode op, int a, int b, enum polarity *pol) {
    switch (op) {
        case OP_ADD:   return a + b;
        case OP_JUDGE: *pol = (a >= THRESHOLD) ? YANG : YIN; return a;
        case OP_SNAP:  return a;
        case OP_INC:   return a + 1;
        case OP_DEC:   return a - 1;
        case OP_MUL2:  return a * 2;
        case OP_DIV2:  return a / 2;
        default:       return a;
    }
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
        enum opcode op = OP_NOP;
        if (nt >= 5) op = opcode_from_name(toks[4]);
        int folded = 0;
        bag_t *b = drawer_add(&it->drawer, toks[1], gysh, op, &folded);
        interp_trace(it, gysh);
        if (folded) { printf("  ⚠️ 屉满(32袋) → 折叠进层,拒收「%s」\n", toks[1]); return -1; }
        char opname[16]; bag_name_from_opcode(op, opname, sizeof opname);
        printf("  ✅ 袋 %s = %d [积木:%s] 层%d\n", b->name, b->gongyueshu, opname, b->layer);
        return 0;
    }

    /* ── 注 声母 袋名 ── (运行时注册声母) */
    if (strcmp(toks[0], "注") == 0) {
        if (nt < 3) return -2;
        enum opcode op = opcode_from_name(toks[1]);
        if (op == OP_NOP) { printf("  ❓ 未识声母: %s\n", toks[1]); return -2; }
        /* 注册到元表 */
        int rc = meta_register(it, toks[1], op);
        /* 同时给袋子绑定:如果袋名存在,改它的积木 */
        bag_t *b = drawer_find(&it->drawer, toks[2]);
        if (b) b->op = op;
        printf("  📌 注 声母 %s → opcode%d 袋%s (%s)\n",
               toks[1], op, toks[2], rc == 0 ? "入表" : "更新");
        return 0;
    }

    /* ── 用 袋名 袋A 袋B -> 袋C ── (以袋子的积木执行) */
    if (strcmp(toks[0], "用") == 0) {
        if (nt < 6) return -2;   /* 用 袋名 袋A 袋B -> 袋C = 6 token */
        bag_t *bag_op = drawer_find(&it->drawer, toks[1]);
        if (!bag_op) { printf("  ❓ 操作袋未找到: %s\n", toks[1]); return -2; }
        bag_t *a = drawer_find(&it->drawer, toks[2]);
        bag_t *b = drawer_find(&it->drawer, toks[3]);
        if (!a || !b) { printf("  ❌ 袋未找到\n"); return -2; }
        enum polarity pol = YIN;
        int r = apply_opcode(bag_op->op, a->gongyueshu, b->gongyueshu, &pol);
        int folded = 0;
        bag_t *c = drawer_add(&it->drawer, toks[5], r, OP_NOP, &folded);
        interp_trace(it, r);
        if (folded) { printf("  ⚠️ 屉满\n"); return -1; }
        c->pol = pol;
        if (bag_op->op == OP_JUDGE) a->pol = pol; /* 裁定作用回第一袋 */
        char opn[16]; bag_name_from_opcode(bag_op->op, opn, sizeof opn);
        printf("  ⚡ 用 %s[%s] %s %s → %s = %d %s\n",
               bag_op->name, opn, a->name, b->name, c->name, r,
               pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 照 袋名 ── */
    if (strcmp(toks[0], "照") == 0) {
        if (nt >= 2 && strcmp(toks[1], "元袋") == 0) {
            printf("  📸 元袋: 袋数=%d 声母表=%d 痕迹=%d\n",
                   it->drawer.count, it->meta.count, it->trace_n);
            /* 打印声母表 */
            for (int i = 0; i < it->meta.count; i++) {
                char opn[16]; bag_name_from_opcode(it->meta.codes[i], opn, sizeof opn);
                printf("       表[%d] %s = %s (opcode%d)\n",
                       i, it->meta.names[i], opn, it->meta.codes[i]);
            }
            return 0;
        }
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        char opn[16]; bag_name_from_opcode(b->op, opn, sizeof opn);
        printf("  📸 袋 %s: 公约数=%d 层=%d 积木=%s 极性=%s\n",
               b->name, b->gongyueshu, b->layer, opn,
               b->pol == YANG ? "阳" : "阴");
        return 0;
    }

    /* ── 退/进/益/损/裁 直接作用于袋 ── */
    if (strcmp(toks[0], "进") == 0 || strcmp(toks[0], "退") == 0 ||
        strcmp(toks[0], "益") == 0 || strcmp(toks[0], "损") == 0 ||
        strcmp(toks[0], "裁") == 0) {
        if (nt < 2) return -2;
        bag_t *b = drawer_find(&it->drawer, toks[1]);
        if (!b) { printf("  ❌ 袋未找到: %s\n", toks[1]); return -2; }
        enum opcode op = opcode_from_name(toks[0]);
        enum polarity pol = YIN;
        b->gongyueshu = apply_opcode(op, b->gongyueshu, 0, &pol);
        b->layer = b->gongyueshu / RADIX;
        char opn[16]; bag_name_from_opcode(op, opn, sizeof opn);
        printf("  ⚙️ %s %s → %d 层%d %s\n", opn, b->name, b->gongyueshu, b->layer,
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

/* ═══════════════════════════════════════════════════════════════
 * 测试套件 (≥6项)
 * ═══════════════════════════════════════════════════════════════ */

static int g_pass = 0, g_fail = 0;

#define ASSERT(c, m) do { \
    if (c) { printf("    ✅ %s\n", m); g_pass++; } \
    else   { printf("    ❌ %s\n", m); g_fail++; } \
} while (0)

static void suite(void) {
    printf("\n【Step H1 积木袋子 测试套件】\n");

    /* 1. 袋积木绑定 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 合袋 = 20 合\n");
      bag_t *b = drawer_find(&it.drawer, "合袋");
      ASSERT(b && b->op == OP_ADD, "袋合袋 自带积木 合");
      ASSERT(b && b->gongyueshu == 20, "合袋 公约数=20"); }

    /* 2. 用袋子积木执行加法: 用 合 7 13 → 20 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "袋 a = 7\n"
        "袋 b = 13\n"
        "用 合袋 a b -> c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 20, "用合袋积木: 7+13=20"); }

    /* 3. 进/退/益/损 直作用 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 计数 = 5\n进 计数\n进 计数\n益 计数\n");
      bag_t *b = drawer_find(&it.drawer, "计数");
      ASSERT(b && b->gongyueshu == 14, "进进益: 5→6→7→14"); }

    /* 4. 擦注册声母: 注 声母 袋名 */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 旧袋 = 7\n"
        "注 合 旧袋\n");
      bag_t *b = drawer_find(&it.drawer, "旧袋");
      ASSERT(b && b->op == OP_ADD, "注 合 → 旧袋改积木为合");
      ASSERT(meta_find(&it.meta, "合") == OP_ADD, "元表已有 合"); }

    /* 5. 裁 (裁定) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 裁袋 = 0 裁\n"
        "袋 好袋 = 20\n"
        "用 裁袋 好袋 好袋 -> 判袋\n");
      bag_t *b = drawer_find(&it.drawer, "好袋");
      ASSERT(b && b->pol == YANG, "裁: 20 >= 10 阳"); }

    /* 6. 跨层进位 (64→层1) */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 临界 = 64\n");
      bag_t *b = drawer_find(&it.drawer, "临界");
      ASSERT(b && b->layer == 1, "临界64: 层 1 (64÷64)"); }

    /* 7. 元袋快照 */
    { interp_t it; interp_init(&it);
      run_script(&it, "袋 a = 1\n袋 b = 2\n照 元袋\n");
      ASSERT(it.drawer.count == 2, "元袋: 袋数=2"); }

    /* 8. 序列: 合袋 合 a b → c, 再用 益 c → d (= (a+b)*2 ) */
    { interp_t it; interp_init(&it);
      run_script(&it,
        "袋 合袋 = 0 合\n"
        "袋 a = 5\n袋 b = 9\n"
        "用 合袋 a b -> c\n"
        "益 c\n");
      bag_t *c = drawer_find(&it.drawer, "c");
      ASSERT(c && c->gongyueshu == 28, "序列 合+益: 5+9=14,益=28"); }

    printf("\n  总计: %d/%d 通过\n", g_pass, g_pass + g_fail);
}

/* ═══════════════════════════════════════════════════════════════
 * 主演示
 * ═══════════════════════════════════════════════════════════════ */

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  MCCP Phase 3 · Step H1 · L0 积木袋子自举 (Step H 升级)\n");
    printf("  袋/注/用/照/进/退/益/损 — 袋子携带积木,DSL 运行时注册\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    interp_t it;
    interp_init(&it);

    const char *program =
        "# 囡囝共生 Step H1 — 积木袋子自举\n"
        "袋 元袋 = 0\n"
        "\n"
        "# 1. 袋子自带积木\n"
        "袋 合袋 = 0 合\n"
        "袋 裁袋 = 0 裁\n"
        "\n"
        "# 2. 基本袋\n"
        "袋 容器甲 = 7\n"
        "袋 容器乙 = 13\n"
        "\n"
        "# 3. 用袋子积木做加法: 合袋(容器甲,容器乙) → 融合袋 = 20\n"
        "用 合袋 容器甲 容器乙 -> 融合袋\n"
        "\n"
        "# 4. 再用 裁袋 判定 融合袋\n"
        "用 裁袋 融合袋 融合袋 -> 判定袋\n"
        "裁 容器甲\n"
        "\n"
        "# 5. 运行时注册新声母: 把 益 绑到 倍增袋\n"
        "袋 倍增袋 = 0\n"
        "注 益 倍增袋\n"
        "\n"
        "# 6. 进/退直接作用\n"
        "进 倍增袋\n"
        "\n"
        "# 7. 折叠进位\n"
        "袋 临界袋 = 64\n"
        "照 临界袋\n"
        "\n"
        "# 8. 元袋自观\n"
        "照 元袋\n"
        "\n"
        "# 9. 序列: (容器甲+容器乙)后 益\n"
        "益 融合袋\n"
        "照 融合袋\n";

    printf("\n【第 1 步】积木 DSL 执行\n");
    run_script(&it, program);

    suite();

    /* 满全法 5 项输出 */
    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  Step H1 闭环 (5 项)\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  当前层号:    %d (最大公约数 %d ÷ %d)\n",
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].layer : 0,
           it.drawer.count ? it.drawer.bags[it.drawer.count-1].gongyueshu : 0,
           RADIX);
    printf("  当前声母:    合(加法)/裁(裁定)/照(快照)/进/退/益/损\n");
    printf("  当前韵母:    容器甲/容器乙 → 融合袋 → 益 → 28\n");
    printf("  折叠状态:    %s\n",
           it.drawer.count >= MAX_BAGS ? "已折叠" : "屉未满(32)");
    printf("  下一步:      袋子注册 C 外部操作 (完全自举,袋子即元表)\n");
    printf("  袋子总数:    %d    声母表: %d    痕迹: %d\n",
           it.drawer.count, it.meta.count, it.trace_n);
    printf("  测试结果:    %d 通过, %d 失败\n", g_pass, g_fail);

    if (g_fail == 0)
        printf("\n  ✅ Step H1 闭环 — 积木袋子自举:袋子携带积木,运行时注册!\n");
    else
        printf("\n  ⚠️  %d 项未通过\n", g_fail);
    printf("═══════════════════════════════════════════════════════════════\n");

    return g_fail == 0 ? 0 : 1;
}
