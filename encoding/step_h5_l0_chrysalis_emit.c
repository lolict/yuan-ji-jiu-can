/*
 * step_h5_l0_chrysalis_emit.c
 *
 * MCCP Phase 3 · Step H5 — 蚕蛹脱壳: 袋子反向产出 C
 *
 * 核心理念:
 *   H4 袋观己,写新元表. 但元表解释执行,还没脱离"袋子容器"
 *   H5 袋反向产 C: 袋子指定签名+体, 蚕蛹生成可独立编译的 .c 文件
 *   脱壳 = 袋子无需解释器就运行; 产出的 C 是独立成虫
 *   穿衣 = 产出的 C 适配目标壳 (posix/wasm/bare)
 *
 * 新语法:
 *   袋 <名> <声母> <公约数> [链...]
 *   反袋 <袋名> "<c_name>" <ret> <a> <b>
 *   反体 <袋名> "C 代码体"
 *   脱壳 posix|wasm|bare
 *   穿衣 <shell> [文件名]
 *
 * 满全法:
 *   层号: 脱壳层,袋子内的袋子运行→编译后脱离袋子=新层
 *   声母: 反袋语法, 袋子→C 反向声母
 *   韵母: 目标壳 (posix/wasm/bare)
 *   折叠: 袋观的屉满度→决定哪条袋子先脱壳
 *   下一步 (H6): 成虫飞走 — wasm 编译/裸机烧录
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_BAGS        32
#define MAX_EMIT        32
#define MAX_LINE        512
#define NAME_SZ         64
#define BODY_SZ         256
#define FNAME_SZ        64
#define SHELL_SZ        16

enum polarity { YIN = 0, YANG = 1 };
typedef int (*c_fn_t)(int a, int b, enum polarity *pol);

/* 前向声明 */
static int fn_he(int a, int b, enum polarity *pol);
static int fn_sun(int a, int b, enum polarity *pol);
static int fn_cheng(int a, int b, enum polarity *pol);
static int fn_chu(int a, int b, enum polarity *pol);
static int fn_jian(int a, int b, enum polarity *pol);

/* 袋子 / 元表 / 屉 */
typedef struct {
    char   bag_names[6][NAME_SZ];
    int    count;
} chain_t;

typedef enum { OP_FN = 0, OP_CHAIN } op_src_t;

typedef struct {
    char      name[NAME_SZ];
    op_src_t  src;
    union { c_fn_t c_fn; chain_t chain; };
} meta_entry_t;

typedef struct {
    meta_entry_t entries[MAX_EMIT];
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

/* 反袋表: 袋子 → C 函数 */
typedef struct {
    char      bag_name[NAME_SZ];
    char      c_name[FNAME_SZ];
    char      c_ret[NAME_SZ];
    char      c_a[NAME_SZ];
    char      c_b[NAME_SZ];
    char      c_body[BODY_SZ];
    int       has_body;
} emit_entry_t;

typedef struct {
    emit_entry_t entries[MAX_EMIT];
    int          count;
    char         target_shell[SHELL_SZ];
} emit_tab_t;

static meta_tab_t   meta;
static drawer_t     drawer;
static emit_tab_t   emit_tab;

/* ── 查找 ── */
static bag_t* find_bag(const char *n) {
    for (int i = 0; i < drawer.count; i++)
        if (strcmp(drawer.bags[i].name, n) == 0) return &drawer.bags[i];
    return NULL;
}
static meta_entry_t* find_meta(const char *n) {
    for (int i = 0; i < meta.count; i++)
        if (strcmp(meta.entries[i].name, n) == 0) return &meta.entries[i];
    return NULL;
}
static emit_entry_t* find_emit(const char *n) {
    for (int i = 0; i < emit_tab.count; i++)
        if (strcmp(emit_tab.entries[i].bag_name, n) == 0) return &emit_tab.entries[i];
    return NULL;
}

/* 提取 "..." 内容 */
static const char* extract_quoted(const char *p, char *buf, int bufsz) {
    while (*p && *p != '"') p++;
    if (!*p) return NULL;
    p++;
    int i = 0;
    while (*p && *p != '"' && i < bufsz - 1) buf[i++] = *p++;
    buf[i] = '\0';
    return (*p == '"') ? p + 1 : p;
}

/* ── 袋子行为函数 ── */
static int fn_he(int a, int b, enum polarity *pol) {
    *pol = YANG; return a + b;
}
static int fn_sun(int a, int b, enum polarity *pol) {
    *pol = YIN;  return a - b;
}
static int fn_cheng(int a, int b, enum polarity *pol) {
    *pol = (a * b >= 0) ? YANG : YIN; return a * b;
}
static int fn_chu(int a, int b, enum polarity *pol) {
    if (b == 0) { *pol = YIN; return 0; }
    *pol = (a / b >= 0) ? YANG : YIN; return a / b;
}
static int fn_jian(int a, int b, enum polarity *pol) {
    *pol = (a - b >= 0) ? YANG : YIN; return (a + b) / 2;
}

/* ── 袋子元表只入一次 ── */
static void ensure_meta(const char *name, chain_t *chain) {
    if (find_meta(name)) return;
    if (meta.count >= MAX_EMIT) return;
    strncpy(meta.entries[meta.count].name, name, NAME_SZ - 1);
    meta.entries[meta.count].src = OP_CHAIN;
    meta.entries[meta.count].chain.count = chain->count;
    for (int i = 0; i < chain->count; i++)
        strncpy(meta.entries[meta.count].chain.bag_names[i], chain->bag_names[i], NAME_SZ - 1);
    meta.count++;
}

/* ── 袋命令 ── */
static int cmd_bag(const char *args) {
    char name[NAME_SZ], shengmu[NAME_SZ];
    int gongyueshu;
    if (sscanf(args, "%63s %63s %d", name, shengmu, &gongyueshu) < 3) return -1;
    if (drawer.count >= MAX_BAGS || find_bag(name)) return -1;

    bag_t *b = &drawer.bags[drawer.count];
    strncpy(b->name, name, NAME_SZ - 1);
    strncpy(b->shengmu, shengmu, NAME_SZ - 1);
    b->gongyueshu = gongyueshu;
    b->layer = 0; b->pol = YANG;
    drawer.count++;

    /* 收集链 */
    const char *p = args;
    for (int i = 0; i < 3 && *p;) { while (*p && !isspace(*p)) p++; while (*p && isspace(*p)) p++; i++; }
    chain_t chain = { .count = 0 };
    while (*p && chain.count < 6) {
        while (*p && isspace(*p)) p++;
        if (!*p || *p == '\n') break;
        int n = 0; char tmp[NAME_SZ];
        while (*p && !isspace(*p) && *p != '\n' && n < NAME_SZ - 1) tmp[n++] = *p++;
        tmp[n] = '\0';
        if (n > 0) strncpy(chain.bag_names[chain.count++], tmp, NAME_SZ - 1);
    }

    /* 声母→fn */
    c_fn_t fn = NULL;
    if (strcmp(shengmu, "合") == 0) fn = fn_he;
    else if (strcmp(shengmu, "损") == 0) fn = fn_sun;
    else if (strcmp(shengmu, "乘") == 0) fn = fn_cheng;
    else if (strcmp(shengmu, "除") == 0) fn = fn_chu;
    else if (strcmp(shengmu, "简") == 0) fn = fn_jian;

    if (fn && !find_meta(name)) {
        strncpy(meta.entries[meta.count].name, name, NAME_SZ - 1);
        meta.entries[meta.count].src = OP_FN;
        meta.entries[meta.count].c_fn = fn;
        meta.count++;
    }
    if (chain.count > 0 && !fn)
        ensure_meta(name, &chain);

    printf("  🟢 袋 %s 声母=%s 公约数=%d 入屉[%d]\n", name, shengmu, gongyueshu, drawer.count);
    return 0;
}

/* ── 用命令 ── */
static int exec_bag(const char *name, int a, int b, enum polarity *pol) {
    meta_entry_t *me = find_meta(name);
    if (!me) return -9999;
    if (me->src == OP_FN && me->c_fn)
        return me->c_fn(a, b, pol);
    else if (me->src == OP_CHAIN && me->chain.count > 0) {
        int r = a; *pol = YANG;
        for (int i = 0; i < me->chain.count; i++) {
            enum polarity sub_pol;
            r = exec_bag(me->chain.bag_names[i], r, b, &sub_pol);
            if (sub_pol == YIN) *pol = YIN;
        }
        return r;
    }
    return -9999;
}

static int cmd_use(const char *args) {
    char name[NAME_SZ]; int a, b;
    if (sscanf(args, "%63s %d %d", name, &a, &b) < 3) return -1;
    if (!find_bag(name)) { printf("  ⚠ 袋 %s 不存在\n", name); return -1; }
    enum polarity pol;
    int r = exec_bag(name, a, b, &pol);
    printf("  📤 用 %s %d %d → %d [%s]", name, a, b, r, pol == YANG ? "阳" : "阴");
    emit_entry_t *e = find_emit(name);
    if (e) printf(" ↳反袋 %s %s(%s,%s)", e->c_ret, e->c_name, e->c_a, e->c_b);
    printf("\n");
    return 0;
}

/* ── 反袋命令 ── */
static int cmd_emit_bag(const char *args) {
    char bag_name[NAME_SZ];
    const char *p = args;
    while (*p && isspace(*p)) p++;
    int n = 0;
    while (*p && !isspace(*p) && n < NAME_SZ - 1) bag_name[n++] = *p++;
    bag_name[n] = '\0';
    while (*p && isspace(*p)) p++;
    if (!find_bag(bag_name)) { printf("  ⚠ 袋 %s 不存在\n", bag_name); return -1; }

    char c_name[FNAME_SZ] = {0}, c_ret[NAME_SZ] = {0}, c_a[NAME_SZ] = {0}, c_b[NAME_SZ] = {0};
    p = extract_quoted(p, c_name, FNAME_SZ);
    if (!p) { printf("  ⚠ 反袋语法: 反袋 <袋名> \"<c名>\" <ret> <a> <b>\n"); return -1; }
    sscanf(p, "%63s %63s %63s", c_ret, c_a, c_b);

    emit_entry_t *e = find_emit(bag_name);
    if (!e) {
        if (emit_tab.count >= MAX_EMIT) return -1;
        e = &emit_tab.entries[emit_tab.count++];
    }
    strncpy(e->bag_name, bag_name, NAME_SZ - 1);
    strncpy(e->c_name, c_name, FNAME_SZ - 1);
    strncpy(e->c_ret, c_ret, NAME_SZ - 1);
    strncpy(e->c_a, c_a, NAME_SZ - 1);
    strncpy(e->c_b, c_b, NAME_SZ - 1);
    e->has_body = 0;
    e->c_body[0] = '\0';
    printf("  🔄 反袋: %s → %s (%s, %s, %s)\n", bag_name, c_name, c_ret, c_a, c_b);
    return 0;
}

/* ── 反体命令 ── */
static int cmd_emit_body(const char *args) {
    char bag_name[NAME_SZ];
    const char *p = args;
    while (*p && isspace(*p)) p++;
    int n = 0;
    while (*p && !isspace(*p) && n < NAME_SZ - 1) bag_name[n++] = *p++;
    bag_name[n] = '\0';
    while (*p && isspace(*p)) p++;
    char body[BODY_SZ] = {0};
    p = extract_quoted(p, body, BODY_SZ);
    if (!p) { printf("  ⚠ 反体语法: 反体 <袋名> \"<C代码>\"\n"); return -1; }

    emit_entry_t *e = find_emit(bag_name);
    if (!e) { printf("  ⚠ 袋 %s 未反袋\n", bag_name); return -1; }
    strncpy(e->c_body, body, BODY_SZ - 1);
    e->has_body = 1;
    printf("  🔄 反体: %s → { %s }\n", bag_name, body);
    return 0;
}

/* ── 脱壳: 生成 C 源码 ── */
static void emit_header(FILE *out, const char *shell) {
    fprintf(out, "/* 蚕蛹·脱壳·成虫 | 壳=%s | 自动生成 step_h5_l0_chrysalis_emit */\n\n", shell);
    fprintf(out, "#include <stdio.h>\n#include <stdint.h>\n");
    if (strcmp(shell, "wasm") == 0)
        fprintf(out, "#define printf(...) /* wasm nop */\n");
    fprintf(out, "\ntypedef enum { YIN=0, YANG=1 } polarity_t;\n");
    fprintf(out, "#define BAG_A a\n#define BAG_B b\n#define BAG_POL (*pol)\n\n");
}

static void emit_function(FILE *out, const emit_entry_t *e) {
    fprintf(out, "%s %s(%s a, %s b, polarity_t *pol) {\n", e->c_ret, e->c_name, e->c_a, e->c_b);
    if (e->has_body && e->c_body[0])
        fprintf(out, "    %s\n", e->c_body);
    else {
        fprintf(out, "    (void)a; (void)b; (void)pol;\n");
        fprintf(out, "    return (%s)0;\n", e->c_ret);
    }
    fprintf(out, "}\n\n");
}

static void emit_table(FILE *out) {
    fprintf(out, "typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);\n");
    fprintf(out, "typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;\n\n");
    fprintf(out, "static bag_entry_t bag_table[] = {\n");
    for (int i = 0; i < emit_tab.count; i++)
        fprintf(out, "    { \"%s\", %s },\n", emit_tab.entries[i].bag_name, emit_tab.entries[i].c_name);
    fprintf(out, "};\n#define BAG_TABLE_SIZE %d\n\n", emit_tab.count);
}

static void emit_main(FILE *out, const char *shell) {
    fprintf(out, "void bag_run_all(void) {\n");
    fprintf(out, "    polarity_t pol;\n");
    fprintf(out, "    int vals[][2] = {{6,4},{10,3},{8,8},{7,2}};\n");
    fprintf(out, "    for (int i = 0; i < BAG_TABLE_SIZE; i++)\n");
    fprintf(out, "        for (int j = 0; j < 4; j++) {\n");
    fprintf(out, "            pol = YIN;\n");
    fprintf(out, "            int r = bag_table[i].fn(vals[j][0], vals[j][1], &pol);\n");
    fprintf(out, "            printf(\"%%s(%%d,%%d)=%%d [%%s]\\n\",\n");
    fprintf(out, "                   bag_table[i].name, vals[j][0], vals[j][1], r,\n");
    fprintf(out, "                   pol == YANG ? \"阳\" : \"阴\");\n");
    fprintf(out, "        }\n");
    fprintf(out, "}\n\n");
    if (strcmp(shell, "bare") != 0) {
        fprintf(out, "int main(void) { bag_run_all(); return 0; }\n");
    } else {
        fprintf(out, "void bag_start(void) { bag_run_all(); }\n");
    }
}

static void emit_to_file(FILE *out, const char *shell) {
    emit_header(out, shell);
    for (int i = 0; i < emit_tab.count; i++) emit_function(out, &emit_tab.entries[i]);
    emit_table(out);
    emit_main(out, shell);
}

static int cmd_shed(const char *args) {
    char shell[SHELL_SZ] = {0};
    sscanf(args, "%15s", shell);
    if (strcmp(shell,"posix")!=0 && strcmp(shell,"wasm")!=0 && strcmp(shell,"bare")!=0) {
        printf("  ⚠ 脱壳目标: posix|wasm|bare\n"); return -1;
    }
    if (emit_tab.count == 0) { printf("  ⚠ 无反袋\n"); return -1; }
    strncpy(emit_tab.target_shell, shell, SHELL_SZ - 1);
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  蚕蛹·脱壳·穿衣  shell=%-10s           ║\n", shell);
    printf("╠══════════════════════════════════════════════╣\n");
    printf("║ 袋子反袋数: %-4d                             ║\n", emit_tab.count);
    printf("╚══════════════════════════════════════════════╝\n\n");
    emit_to_file(stdout, shell);
    return 0;
}

/* ── 穿衣: 写入文件 ── */
static int cmd_dress(const char *args) {
    char shell[SHELL_SZ] = {0}, fname[128] = {0};
    sscanf(args, "%15s %127s", shell, fname);
    if (!fname[0]) snprintf(fname, sizeof(fname), "step_h5_emitter_out.c");
    if (emit_tab.count == 0) { printf("  ⚠ 无反袋\n"); return -1; }
    strncpy(emit_tab.target_shell, shell, SHELL_SZ - 1);
    FILE *f = fopen(fname, "w");
    if (!f) { printf("  ⚠ 写入失败: %s\n", fname); return -1; }
    emit_to_file(f, shell);
    fclose(f);
    printf("  📄 已写入: %s (shell=%s)\n", fname, shell);
    char cmd[256];
    snprintf(cmd, sizeof(cmd), "cc -Wall -o step_h5_emitted_run %s 2>&1 && ./step_h5_emitted_run", fname);
    printf("  🔧 编译验证...\n");
    int rc = system(cmd);
    if (rc == 0)
        printf("  ✅ 穿衣成功\n");
    else
        printf("  ⚠ 编译未通过 (无cc或语法错误)\n");
    return 0;
}

/* ── 报告 ── */
static void cmd_report(void) {
    printf("\n── 蚕蛹 ┄ 反袋表 (%d 条) ──\n", emit_tab.count);
    for (int i = 0; i < emit_tab.count; i++) {
        emit_entry_t *e = &emit_tab.entries[i];
        printf("  %s → %s %s(%s,%s) %s\n",
               e->bag_name, e->c_ret, e->c_name, e->c_a, e->c_b,
               e->has_body ? "✓" : "…");
    }
    printf("\n── 屉 (%d 袋) ──\n", drawer.count);
    for (int i = 0; i < drawer.count; i++)
        printf("  %s (%s) 公约数=%d\n", drawer.bags[i].name, drawer.bags[i].shengmu, drawer.bags[i].gongyueshu);
    printf("\n");
}

/* ── 行解析 ── */
static int process_line(const char *line) {
    if (!line || !*line || *line == '\n' || *line == '#') return 0;
    if (strncmp(line, "袋 ", 4) == 0)   return cmd_bag(line + 4);
    if (strncmp(line, "用 ", 4) == 0)   return cmd_use(line + 4);
    if (strncmp(line, "反袋 ", 7) == 0) return cmd_emit_bag(line + 7);
    if (strncmp(line, "反体 ", 7) == 0) return cmd_emit_body(line + 7);
    if (strncmp(line, "穿衣 ", 7) == 0) return cmd_dress(line + 7);
    if (strncmp(line, "脱壳 ", 7) == 0) return cmd_shed(line + 7);
    if (strcmp(line, "报告\n") == 0 || strcmp(line, "报告") == 0) { cmd_report(); return 0; }
    printf("  ⚠ 未知命令: %s", line);
    return -1;
}

/* ── main ── */
int main(void) {
    setvbuf(stdout, NULL, _IONBF, 0);
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  蚕 蛹  ·  脱 壳  ·  成 虫  (H5)               ║\n");
    printf("║  袋反向产 C → 编译运行 → 脱离袋子解释器        ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");

    const char *script[] = {
        "袋 加 合 1 损",
        "袋 减 损 1 合",
        "袋 乘 乘 4 合 损",
        "袋 简 简 2 合 损",
        "用 加 6 4",
        "用 减 10 3",
        "用 乘 6 4",
        "用 简 6 4",
        "反袋 加 \"bag_add\" int int int",
        "反体 加 \"  return (BAG_A + BAG_B); \"",
        "反袋 减 \"bag_sub\" int int int",
        "反体 减 \"  return (BAG_A - BAG_B); \"",
        "反袋 乘 \"bag_mul\" int int int",
        "反体 乘 \"  return (BAG_A * BAG_B); \"",
        "反袋 简 \"bag_avg\" int int int",
        "反体 简 \"  return (BAG_A + BAG_B) / 2; \"",
        "报告",
        "脱壳 posix",
        "穿衣 posix step_h5_emitter_out.c",
        NULL
    };

    printf("── 执行脚本 ──\n");
    for (int i = 0; script[i]; i++) {
        printf("  > %s\n", script[i]);
        process_line(script[i]);
    }
    return 0;
}
