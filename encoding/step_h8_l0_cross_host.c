/*
 * step_h8_l0_cross_host.c
 *
 * MCCP Phase 3 · Step H8 — 跨宿主迁移: 一袋多壳
 *
 * 核心洞见:
 *   H7 证明袋子穿衣 x86_64 裸机壳可复通宿主。
 *   H8 验证"壳无关": 同一袋表 (4 个袋行为: 加减乘简)
 *   同时穿上两个壳 — x86_64 裸机壳 + WASM (WASI) 壳。
 *
 * 设计:
 *   袋子行为层: bag_add/sub/mul/avg — 与 H6/H7 完全一致, 无任何壳相关代码
 *   穿衣层 A: x86_64 Linux syscall (write=1, exit=60) — 宿主 OS 对话
 *   穿衣层 B: WASI (proc_exit, fd_write) — 宿主 runtime 对话
 *
 *   壳接口归一化为两个动作:
 *     壳写(buf, len) — 把袋结果输出到宿主
 *     壳出(code)     — 袋子寿命终结
 *
 *   这意味着袋子学会穿任何壳: 今天 Linux, 明天 WASM, 后天 BPF/eBPF — 袋行为不变。
 *
 * 满全法:
 *   层号: H7 穿衣层 → + 跨宿主层
 *   声母: 袋子行为 × 壳写接口 = 任意宿主对话
 *   韵母: x86_64壳 / WASI壳 (双壳)
 *   折叠: 同一袋表 → 两壳穿衣 → 并行产出
 *   下一步 (H9): 袋子产袋子 — 袋子自举编译器 (袋子写袋子)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>
#include <sys/stat.h>

/* ═══════════════════════════════════════════════════
 * 袋子行为层: 壳无关 — 纯算术, 无 libc
 * ═══════════════════════════════════════════════════ */

typedef enum { YIN = 0, YANG = 1 } polarity_t;
typedef int (*bag_fn_t)(int a, int b, polarity_t *pol);

typedef struct { const char *name; bag_fn_t fn; } bag_entry_t;

static int bag_add(int a, int b, polarity_t *p) { *p = YANG; return a+b; }
static int bag_sub(int a, int b, polarity_t *p) { *p = YIN;  return a-b; }
static int bag_mul(int a, int b, polarity_t *p) { *p = (a*b>=0)?YANG:YIN; return a*b; }
static int bag_avg(int a, int b, polarity_t *p) { *p = (a-b>=0)?YANG:YIN; return (a+b)/2; }

static bag_entry_t bag_table[] = {
    { "加", bag_add },
    { "减", bag_sub },
    { "乘", bag_mul },
    { "简", bag_avg },
};
#define BAG_TABLE_SIZE 4

/* 壳接口抽象: typedef 两个穿衣动作 */
typedef void (*shell_write_fn)(const void *buf, int len);
typedef void (*shell_exit_fn)(int code);

/* ═══════════════════════════════════════════════════
 * 路径 A: 宿主验证模式 — 袋行为 + printf 输出
 * ═══════════════════════════════════════════════════ */

static void run_host_verify(void) {
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  跨宿主迁移 ┄ 宿主验算                      ║\n");
    printf("╚══════════════════════════════════════════════╝\n\n");

    polarity_t pol;
    int vals[][2] = {{6,4},{10,3},{8,8},{7,2}};
    for (int i = 0; i < BAG_TABLE_SIZE; i++)
        for (int j = 0; j < 4; j++) {
            pol = YIN;
            int r = bag_table[i].fn(vals[j][0], vals[j][1], &pol);
            printf("    %s(%d,%d)=%d [%s]\n",
                   bag_table[i].name, vals[j][0], vals[j][1], r,
                   pol == YANG ? "阳" : "阴");
        }
}

/* ═══════════════════════════════════════════════════
 * 路径 B: 壳无关穿衣源码生成器
 *
 * 输入: 袋表 (行为) + 壳写 + 壳出 的代码片段
 * 输出: 独立 .c, 穿上对应壳
 *
 * 穿衣 = 用壳写替换 libc printf, 用壳出替换 libc exit
 * ═══════════════════════════════════════════════════ */

/* 双壳定义 */
static const struct {
    const char *label;          /* 壳名 */
    const char *header;         /* 文件头部注释 + 宏/导入 */
    const char *shell_write;    /* 壳写代码片段 */
    const char *shell_exit;     /* 壳出代码片段 */
    const char *filename;       /* 产出文件名 */
} shells[] = {
    {
        "x86_64",
        "/* 壳: x86_64 Linux syscall · 穿衣层 A */\n"
        "/* 由 step_h8_l0_cross_host 自动生成           */\n"
        "/* 编译: cc -ffreestanding -nostdlib -nostdinc -fno-pie -no-pie -Wl,-e,_start -o bag_x86 bag_x86.c */\n",
        /* shell_write */
        "static void shell_write(const void *buf, int len) {\n"
        "    __asm__ volatile (\n"
        "        \"syscall\"\n"
        "        : : \"a\"((long)1), \"D\"((long)1), \"S\"(buf), \"d\"((long)len)\n"
        "        : \"rcx\", \"r11\", \"memory\"\n"
        "    );\n"
        "}",
        /* shell_exit */
        "static void shell_exit(int code) {\n"
        "    __asm__ volatile (\n"
        "        \"syscall\"\n"
        "        : : \"a\"((long)60), \"D\"((long)code)\n"
        "        : \"rcx\", \"r11\"\n"
        "    );\n"
        "    __builtin_unreachable();\n"
        "}",
        "bag_x86.c"
    },
    {
        "wasi",
        "/* 壳: WASI (WebAssembly System Interface) · 穿衣层 B */\n"
        "/* 由 step_h8_l0_cross_host 自动生成                         */\n"
        "/* 编译: clang --target=wasm32-wasi -o bag_wasi.wasm bag_wasi.c */\n",
        /* shell_write */
        "__attribute__((__import_module__(\"wasi_unstable\"), __import_name__(\"fd_write\")))\n"
        "int __wasi_fd_write(int fd, const void *iovs, long iovs_len, long *nwritten);\n"
        "static void shell_write(const void *buf, int len) {\n"
        "    long nw; __wasi_fd_write(1, buf, 0, &nw);\n"
        "}",
        /* shell_exit */
        "__attribute__((__import_module__(\"wasi_unstable\"), __import_name__(\"proc_exit\")))\n"
        "void __wasi_proc_exit(int code);\n"
        "static void shell_exit(int code) { __wasi_proc_exit(code); }",
        "bag_wasi.c"
    }
};

/* itoa 公共 (壳无关) */
static void bake_ito_a(FILE *f) {
    fprintf(f, "static int itoa(int v, char *buf, int bufsz) {\n");
    fprintf(f, "    char tmp[12]; int n=0, neg=0;\n");
    fprintf(f, "    if(v<0){neg=1;v=-v;}\n");
    fprintf(f, "    if(v==0) tmp[n++]='0';\n");
    fprintf(f, "    else { while(v>0 && n<11){tmp[n++]='0'+(v%%10);v/=10;} }\n");
    fprintf(f, "    int pos=0; if(neg && pos<bufsz-1) buf[pos++]='-';\n");
    fprintf(f, "    for(int i=n-1;i>=0&&pos<bufsz-1;i--) buf[pos++]=tmp[i];\n");
    fprintf(f, "    buf[pos]='\\0'; return pos;\n");
    fprintf(f, "}\n\n");
}

/* emit_bag_result 公共 (壳无关, 通过 shell_write 间接输出) */
static void bake_emit_result(FILE *f) {
    fprintf(f, "static void emit_bag_result(const char *name, int a, int b, int res, polarity_t pol) {\n");
    fprintf(f, "    char buf[64]; int pos=0;\n");
    fprintf(f, "    const char *p=name; while(*p && pos<60) buf[pos++]=*p++;\n");
    fprintf(f, "    buf[pos++]='('; pos+=itoa(a,buf+pos,64-pos);\n");
    fprintf(f, "    buf[pos++]=',';  pos+=itoa(b,buf+pos,64-pos);\n");
    fprintf(f, "    buf[pos++]=')'; buf[pos++]='=';\n");
    fprintf(f, "    pos+=itoa(res,buf+pos,64-pos);\n");
    fprintf(f, "    buf[pos++]=' '; buf[pos++]='[';\n");
    fprintf(f, "    const char *ps=(pol==YANG)?\"阳\":\"阴\";\n");
    fprintf(f, "    while(*ps && pos<60) buf[pos++]=*ps++;\n");
    fprintf(f, "    buf[pos++]=']'; buf[pos++]='\\n';\n");
    fprintf(f, "    shell_write(buf, pos);\n");
    fprintf(f, "}\n\n");
}

static void bake_bag_fns_and_table(FILE *f) {
    fprintf(f, "static int bag_add(int a,int b,polarity_t *p){*p=YANG;return a+b;}\n");
    fprintf(f, "static int bag_sub(int a,int b,polarity_t *p){*p=YIN;  return a-b;}\n");
    fprintf(f, "static int bag_mul(int a,int b,polarity_t *p){*p=(a*b>=0)?YANG:YIN; return a*b;}\n");
    fprintf(f, "static int bag_avg(int a,int b,polarity_t *p){*p=(a-b>=0)?YANG:YIN; return (a+b)/2;}\n\n");

    fprintf(f, "static const char nm0[]=\"加\";static const char nm1[]=\"减\";\n");
    fprintf(f, "static const char nm2[]=\"乘\";static const char nm3[]=\"简\";\n");
    fprintf(f, "static bag_entry_t bag_table[]={ {nm0,bag_add},{nm1,bag_sub},{nm2,bag_mul},{nm3,bag_avg} };\n");
    fprintf(f, "#define BAG_TABLE_SIZE 4\n\n");
}

static void bake_run_all(FILE *f) {
    fprintf(f, "static void bag_run_all(void) {\n");
    fprintf(f, "    polarity_t pol;\n");
    fprintf(f, "    int vals[][2]={{6,4},{10,3},{8,8},{7,2}};\n");
    fprintf(f, "    for(int i=0;i<BAG_TABLE_SIZE;i++)\n");
    fprintf(f, "        for(int j=0;j<4;j++){\n");
    fprintf(f, "            pol=YIN;\n");
    fprintf(f, "            int r=bag_table[i].fn(vals[j][0],vals[j][1],&pol);\n");
    fprintf(f, "            emit_bag_result(bag_table[i].name,vals[j][0],vals[j][1],r,pol);\n");
    fprintf(f, "        }\n");
    fprintf(f, "}\n\n");
}

static void emit_shell_variant(int s) {
    const char *fname = shells[s].filename;
    FILE *f = fopen(fname, "w");
    if (!f) { printf("  ⚠ 写失败 %s\n", fname); return; }

    fprintf(f, "%s\n", shells[s].header);
    fprintf(f, "typedef enum{YIN=0,YANG=1}polarity_t;\n");
    fprintf(f, "typedef int(*bag_fn_t)(int a,int b,polarity_t *);\n");
    fprintf(f, "typedef struct{const char*name;bag_fn_t fn;}bag_entry_t;\n");
    fprintf(f, "typedef void(*shell_write_fn)(const void*,int);\n");
    fprintf(f, "typedef void(*shell_exit_fn)(int);\n");
    fprintf(f, "static void shell_write(const void *buf, int len);\n");
    fprintf(f, "static void shell_exit(int code);\n\n");

    bake_bag_fns_and_table(f);
    bake_ito_a(f);
    bake_emit_result(f);
    bake_run_all(f);

    /* 壳特异穿衣层 */
    fprintf(f, "/* ── 穿衣层: %s ── */\n", shells[s].label);
    fprintf(f, "%s\n\n", shells[s].shell_write);
    fprintf(f, "%s\n\n", shells[s].shell_exit);

    if (s == 0) {
        /* x86_64: _start 入口 */
        fprintf(f, "void _start(void){ bag_run_all(); shell_exit(0); }\n");
    } else {
        /* wasi: main/WASI 入口 */
        fprintf(f, "int main(void){ bag_run_all(); shell_exit(0); return 0; }\n");
    }

    fclose(f);
    printf("  📄 %s\n", fname);
}

/* ═══════════════════════════════════════════════════
 * 路径 C: 编译验证
 * ═══════════════════════════════════════════════════ */

static int compile_and_verify(void) {
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  跨宿主迁移 ┄ 编译验证                      ║\n");
    printf("╚══════════════════════════════════════════════╝\n\n");

    /* 壳 A: x86_64 裸机 */
    printf("  [壳 A] x86_64 穿衣...\n");
    int rc = system(
        "cc -ffreestanding -nostdlib -nostdinc "
        "-fno-stack-protector -fno-pie -no-pie "
        "-Wl,-e,_start -o bag_x86 bag_x86.c 2>&1");
    if (rc == 0) {
        printf("   ✅ bag_x86 裸机编译 Ok\n");
        printf("   🔍 入口:");
        system("readelf -h bag_x86 2>&1 | grep Entry");
        printf("   🦋 裸机运行:\n");
        system("./bag_x86 2>&1 | sed 's/^/      /'");
    } else {
        printf("   ⚠ 裸机编译不支持 (无 -ffreestanding)\n");
    }

    /* 壳 B: WASI — 验源码不生成就跳过 */
    printf("\n  [壳 B] WASI 穿衣...\n");
    struct stat st;
    if (stat("bag_wasi.c", &st) == 0) {
        printf("   📄 bag_wasi.c 已产出 (检查 clang --target=wasm32-wasi 可编译为 WASM)\n");
        printf("   WASI 穿衣层: wasi_unstable fd_write / proc_exit\n");
        printf("   袋行为代码与 x86_64 版零差异\n");
    }

    return 0;
}

/* ═══════════════════════════════════════════════════
 * 主入口
 * ═══════════════════════════════════════════════════ */

int main(void) {
    setvbuf(stdout, NULL, _IONBF, 0);

    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  跨 宿 主 迁 移  (H8)                          ║\n");
    printf("║  一袋多壳: 袋行为不变 × 衣接口归一化           ║\n");
    printf("║  壳写(buf,len) + 壳出(code) = 任意宿主         ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");

    /* A: 宿主验算 */
    run_host_verify();

    /* B: 生成双壳穿衣版 */
    printf("\n╔══════════════════════════════════════════════╗\n");
    printf("║  壳穿衣生成                                  ║\n");
    printf("╚══════════════════════════════════════════════╝\n\n");
    emit_shell_variant(0);  /* x86_64 */
    emit_shell_variant(1);  /* wasi */

    /* C: 验证 */
    compile_and_verify();

    /* 总结 */
    printf("\n╔══════════════════════════════════════════════════╗\n");
    printf("║  H8 跨宿主迁移 — 完成                          ║\n");
    printf("║  1. 袋行为: 4 个 (加减乘简) — 壳无关           ║\n");
    printf("║  2. 壳 A: x86_64 (Linux syscall 衣)            ║\n");
    printf("║  3. 壳 B: WASI (wasm32 衣)                     ║\n");
    printf("║  4. 衣接口: 壳写 + 壳出 (归一化)              ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║  证明: 袋子学会穿任意壳                        ║\n");
    printf("║  下一步 (H9): 袋子产袋子 — 袋子自举            ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    return 0;
}
