/*
 * step_h7_l0_adult_return.c
 *
 * MCCP Phase 3 · Step H7 — 飞走归来: 裸机 syscall 复通宿主
 *
 * 核心洞见:
 *   H6 裸机产出 bag_bare.c — 袋子行为跑在无 libc 环境,
 *   但结果只在内存, 因为无 syscall write/exit 无法外吐。
 *   H7 成虫飞回: 裸机袋自己打 x86_64 syscall,
 *   把袋行为结果格式化为 UTF-8 → write(1, buf, len) → exit(0)
 *   完全无 libc 中介, 袋子→内核 直接对话。
 *
 * 这是蚕蛹反向寄生壳的原型循环:
 *   蚕: C 壳托管 → 蛹: 茧内自给 → 虫: 飞走裸机 → 归: syscall 复通
 *   = 袋子学会穿任何壳 (C壳/WASM壳/Linux内核壳/Bare壳)
 *
 * 新增概念:
 *   化归 — 裸机袋把整数转字符串 (itoa, 无 libc)
 *   归写 — syscall write(1, ...) 吐袋结果到宿主 stdout
 *   归出 — syscall exit(0) 干净收束
 *
 * 总装: H6 产 bag_bare.c → H7 注入 syscall 穿衣层 → bag_bare_suit.c
 *   外层 syscall 调用内层 bag_table — 穿衣的仍是袋子行为,只是多了壳接口
 *
 * 满全法:
 *   层号: H6 裸机层 → + 穿衣层 (袋子与宿主 OS 对话)
 *   声母: syscall 写 = 袋子对外吐; syscall 出 = 袋子寿命终
 *   韵母: 宿主 OS 入口 (Linux x86_64 或直接 bpf)
 *   折叠: 整数 → UTF-8 字节流 (digit 折叠)
 *   下一步 (H8): 跨宿主迁移 — 同一 bag_table 穿上 WASM/BPF 壳
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════
 * 袋子行为函数 (跟 H6 完全一致, 不含任何 libc)
 * ═══════════════════════════════════════════════════ */

typedef enum { YIN = 0, YANG = 1 } polarity_t;

static int bag_add(int a, int b, polarity_t *pol)  { *pol = YANG; return a + b; }
static int bag_sub(int a, int b, polarity_t *pol)  { *pol = YIN;  return a - b; }
static int bag_mul(int a, int b, polarity_t *pol)  { *pol = (a*b>=0)?YANG:YIN; return a*b; }
static int bag_avg(int a, int b, polarity_t *pol)  { *pol = (a-b>=0)?YANG:YIN; return (a+b)/2; }

typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);
typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;

static bag_entry_t bag_table[] = {
    { "加", bag_add },
    { "减", bag_sub },
    { "乘", bag_mul },
    { "简", bag_avg },
};
#define BAG_TABLE_SIZE ((int)(sizeof(bag_table)/sizeof(bag_table[0])))

/* ─────────────────────────────────────────────────
 * 路径 A (宿主模式): 直接调袋行为 + printf 展示
 * 证明 H7 跟 H6 行为一致
 * ───────────────────────────────────────────────── */

#define RECORD_CAP 64
static struct {
    const char *bag_name;
    int a, b;
    int result;
    polarity_t pol;
} result_buf[RECORD_CAP];
static int result_count = 0;

static void record_result(const char *name, int a, int b, int r, polarity_t pol) {
    if (result_count >= RECORD_CAP) return;
    result_buf[result_count].bag_name = name;
    result_buf[result_count].a = a;
    result_buf[result_count].b = b;
    result_buf[result_count].result = r;
    result_buf[result_count].pol = pol;
    result_count++;
}

static int run_host_mode(void) {
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  飞走归来 ┄ 宿主模式 (验证袋行为)           ║\n");
    printf("╚══════════════════════════════════════════════╝\n\n");

    polarity_t pol;
    int vals[][2] = {{6,4},{10,3},{8,8},{7,2}};
    for (int i = 0; i < BAG_TABLE_SIZE; i++)
        for (int j = 0; j < 4; j++) {
            pol = YIN;
            int r = bag_table[i].fn(vals[j][0], vals[j][1], &pol);
            record_result(bag_table[i].name, vals[j][0], vals[j][1], r, pol);
        }

    printf("  结果记录 (%d 条):\n", result_count);
    for (int i = 0; i < result_count; i++) {
        printf("    %s(%d,%d)=%d [%s]\n",
               result_buf[i].bag_name, result_buf[i].a, result_buf[i].b,
               result_buf[i].result, result_buf[i].pol == YANG ? "阳" : "阴");
    }
    return result_count;
}

/* ─────────────────────────────────────────────────
 * 路径 B: 裸机穿衣 — 注入 syscall 层
 *
 * 生成 bag_bare_suit.c:
 *   - 袋子行为 (不变)
 *   - 化归: int → UTF-8 char buf (无 libc)
 *   - 归写: asm volatile syscall write(1, buf, len)
 *   - 归出: asm volatile syscall exit(0)
 *   - _start: 调 bag_run_all → syscall 穿衣 → exit
 * ───────────────────────────────────────────────── */

/* itoa: int → ASCII digits, written backwards */
static void bake_ito_a(FILE *f) {
    fprintf(f, "/* itoa: integer → ASCII 十进制 (无 libc, 无除法之外的外部调用) */\n");
    fprintf(f, "static int itoa(int v, char *buf, int bufsz) {\n");
    fprintf(f, "    char tmp[12]; int n = 0, neg = 0;\n");
    fprintf(f, "    if (v < 0) { neg = 1; v = -v; }\n");
    fprintf(f, "    if (v == 0) { tmp[n++] = '0'; }\n");
    fprintf(f, "    else { while (v > 0 && n < 11) { tmp[n++] = '0' + (v %% 10); v /= 10; } }\n");
    fprintf(f, "    int pos = 0;\n");
    fprintf(f, "    if (neg && pos < bufsz-1) buf[pos++] = '-';\n");
    fprintf(f, "    for (int i = n-1; i >= 0 && pos < bufsz-1; i--) buf[pos++] = tmp[i];\n");
    fprintf(f, "    buf[pos] = '\\0';\n");
    fprintf(f, "    return pos;\n");
    fprintf(f, "}\n\n");
}

/* syscall write / exit for x86_64 Linux */
static void bake_syscall(FILE *f) {
    fprintf(f, "static long sys_write(int fd, const void *buf, long count) {\n");
    fprintf(f, "    long ret;\n");
    fprintf(f, "    __asm__ volatile (\n");
    fprintf(f, "        \"syscall\"\n");
    fprintf(f, "        : \"=a\"(ret)\n");
    fprintf(f, "        : \"0\"((long)1), \"D\"((long)fd), \"S\"(buf), \"d\"(count)\n");
    fprintf(f, "        : \"rcx\", \"r11\", \"memory\"\n");
    fprintf(f, "    );\n");
    fprintf(f, "    return ret;\n");
    fprintf(f, "}\n\n");

    fprintf(f, "static void sys_exit(int code) {\n");
    fprintf(f, "    __asm__ volatile (\n");
    fprintf(f, "        \"syscall\"\n");
    fprintf(f, "        :\n");
    fprintf(f, "        : \"a\"((long)60), \"di\"((long)code)\n");
    fprintf(f, "        : \"rcx\", \"r11\", \"memory\"\n");
    fprintf(f, "    );\n");
    fprintf(f, "    __builtin_unreachable();\n");
    fprintf(f, "}\n\n");
}

/* Format one bag call to a line and write it out via syscall */
static void bake_emit_line(FILE *f) {
    fprintf(f, "static void emit_bag_result(const char *name, int a, int b, int res, polarity_t pol) {\n");
    fprintf(f, "    char buf[64]; int pos = 0;\n");
    fprintf(f, "    /* bag name (UTF-8 copy) */\n");
    fprintf(f, "    const char *p = name;\n");
    fprintf(f, "    while (*p && pos < 60) buf[pos++] = *p++;\n");
    fprintf(f, "    buf[pos++] = '(';\n");
    fprintf(f, "    pos += itoa(a, buf + pos, 64 - pos);\n");
    fprintf(f, "    buf[pos++] = ',';\n");
    fprintf(f, "    pos += itoa(b, buf + pos, 64 - pos);\n");
    fprintf(f, "    buf[pos++]= ')'; buf[pos++] = '=';\n");
    fprintf(f, "    pos += itoa(res, buf + pos, 64 - pos);\n");
    fprintf(f, "    buf[pos++] = ' '; buf[pos++] = '[';\n");
    fprintf(f, "    const char *ps = (pol == YANG) ? \"阳\" : \"阴\";\n");
    fprintf(f, "    while (*ps && pos < 60) buf[pos++] = *ps++;\n");
    fprintf(f, "    buf[pos++] = ']'; buf[pos++] = '\\n';\n");
    fprintf(f, "    sys_write(1, buf, pos);\n");
    fprintf(f, "}\n\n");
}

static void bake_run_all_with_syscall(FILE *f) {
    fprintf(f, "static void bag_run_all(void) {\n");
    fprintf(f, "    polarity_t pol;\n");
    fprintf(f, "    int vals[][2] = {{6,4},{10,3},{8,8},{7,2}};\n");
    fprintf(f, "    for (int i = 0; i < BAG_TABLE_SIZE; i++)\n");
    fprintf(f, "        for (int j = 0; j < 4; j++) {\n");
    fprintf(f, "            pol = YIN;\n");
    fprintf(f, "            int r = bag_table[i].fn(vals[j][0], vals[j][1], &pol);\n");
    fprintf(f, "            emit_bag_result(bag_table[i].name, vals[j][0], vals[j][1], r, pol);\n");
    fprintf(f, "        }\n");
    fprintf(f, "}\n\n");
}

static void bake_bag_names(FILE *f) {
    const char *names[] = { "加", "减", "乘", "简" };
    for (int i = 0; i < 4; i++)
        fprintf(f, "static const char bag_name_%d[] = \"%s\";\n", i, names[i]);
    fprintf(f, "\n");
}

static void bake_functions(FILE *f) {
    fprintf(f, "static int bag_add(int a, int b, polarity_t *pol)  { *pol=YANG; return a+b; }\n");
    fprintf(f, "static int bag_sub(int a, int b, polarity_t *pol)  { *pol=YIN;  return a-b; }\n");
    fprintf(f, "static int bag_mul(int a, int b, polarity_t *pol)  { *pol=(a*b>=0)?YANG:YIN; return a*b; }\n");
    fprintf(f, "static int bag_avg(int a, int b, polarity_t *pol)  { *pol=(a-b>=0)?YANG:YIN; return (a+b)/2; }\n\n");
}

static void bake_table(FILE *f) {
    fprintf(f, "static bag_entry_t bag_table[] = {\n");
    fprintf(f, "    { bag_name_0, bag_add },\n");
    fprintf(f, "    { bag_name_1, bag_sub },\n");
    fprintf(f, "    { bag_name_2, bag_mul },\n");
    fprintf(f, "    { bag_name_3, bag_avg },\n");
    fprintf(f, "};\n");
    fprintf(f, "#define BAG_TABLE_SIZE %d\n\n", BAG_TABLE_SIZE);
}

static void emit_bare_suit_c(const char *fname) {
    FILE *f = fopen(fname, "w");
    if (!f) { printf("  ⚠ 写入失败 %s\n", fname); return; }

    fprintf(f, "/* ── H7 成虫 · 裸机穿衣 syscall 复通 ── */\n");
    fprintf(f, "/* 由 step_h7_l0_adult_return.c 自动生成                     */\n");
    fprintf(f, "/* 编译: cc -ffreestanding -nostdlib -nostdinc -fno-pie -no-pie  */\n");
    fprintf(f, "/*       -Wl,-e,_start -o bag_bare_suit bag_bare_suit.c        */\n\n");

    fprintf(f, "typedef enum { YIN=0, YANG=1 } polarity_t;\n");
    fprintf(f, "typedef int  (*bag_fn_t)(int a, int b, polarity_t *pol);\n");
    fprintf(f, "typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;\n\n");

    bake_functions(f);
    bake_bag_names(f);
    bake_table(f);
    bake_ito_a(f);
    bake_syscall(f);
    bake_emit_line(f);
    bake_run_all_with_syscall(f);

    /* _start: bag_run_all → sys_exit(0) */
    fprintf(f, "void _start(void) {\n");
    fprintf(f, "    bag_run_all();\n");
    fprintf(f, "    sys_exit(0);\n");
    fprintf(f, "}\n");

    fclose(f);
    printf("  📄 裸机穿衣源码: %s\n", fname);
}

/* ─────────────────────────────────────────────────
 * 编译 + 运行 裸机穿衣版
 * 预期: 直接吐 16 行袋行为结果到 stdout (不经 libc)
 * ───────────────────────────────────────────────── */

static int run_bare_suit_mode(void) {
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  飞走归来 ┄ 裸机穿衣                       ║\n");
    printf("║  袋子行为 → syscall write → 宿主 stdout    ║\n");
    printf("╚══════════════════════════════════════════════╝\n\n");

    emit_bare_suit_c("bag_bare_suit.c");

    /* 编译裸机穿衣版 */
    printf("  🔧 编译裸机穿衣版...\n");
    const char *ccmd =
        "cc -ffreestanding -nostdlib -nostdinc "
        "-fno-stack-protector -fno-pie -no-pie "
        "-Wl,-e,_start "
        "-o bag_bare_suit bag_bare_suit.c 2>&1";

    int rc = system(ccmd);
    if (rc != 0) {
        printf("  ⚠ 裸机编译失败 (可能无 -ffreestanding)\n");
        return -1;
    }
    printf("  ✅ 裸机穿衣二进制: bag_bare_suit\n");

    /* readelf 验证 */
    printf("  🔍 入口点验证:\n");
    system("readelf -h bag_bare_suit 2>&1 | grep -E 'Type|Entry' | sed 's/^/    /'");

    /* 裸机外部符号检查 */
    printf("  🔍 未定义符号:\n");
    system("cc -c bag_bare_suit.c -o bag_bare_suit.o 2>/dev/null && nm -u bag_bare_suit.o 2>/dev/null | grep -v '^$' | sed 's/^/    /' || echo '    (none)'");

    /* 跑裸机穿衣版 — 这一次应该正常 exit(0) */
    printf("\n  🦋 裸机穿衣运行 — 袋子直接 syscall 吐结果:\n");
    rc = system("./bag_bare_suit 2>&1 | sed 's/^/    /'");
    if (rc == 0)
        printf("\n  ✅ 裸机穿衣成功 — 袋子通过 syscall 与宿主对话\n");
    else
        printf("\n  ⚠ 裸机运行返回 0x%04x\n", rc);

    return 0;
}

/* ═══════════════════════════════════════════════════
 * 主入口
 * ═══════════════════════════════════════════════════ */

int main(void) {
    setvbuf(stdout, NULL, _IONBF, 0);

    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  飞 走 归 来  (H7)                              ║\n");
    printf("║  裸机穿衣 — 袋子 syscall 复通宿主 OS            ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");

    /* A: 宿主模式验算 */
    int n = run_host_mode();

    /* B: 裸机穿衣 */
    run_bare_suit_mode();

    /* 总结 */
    printf("\n╔══════════════════════════════════════════════════╗\n");
    printf("║  H7 飞走归来 — 完成                            ║\n");
    printf("║  1. 宿主验算: %d 条袋行为                     ║\n", n);
    printf("║  2. 裸机穿衣: bag_bare_suit.c (syscall 写+出)  ║\n");
    printf("║  3. 入口: _start → bag_run_all → sys_exit(0)   ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║  层: H6 裸机层 → + 穿衣层 (内核接口)           ║\n");
    printf("║  声母: 袋子行为 → syscall → 宿主 stdout        ║\n");
    printf("║  韵母: Linux x86_64 内核 (write=1, exit=60)    ║\n");
    printf("║  折叠: 整数→十进制字节 (itoa 折叠)             ║\n");
    printf("║  下一步 (H8): 跨宿主迁移 — WASM/BPF 壳         ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    return 0;
}
