/**
 * ═══════════════════════════════════════════════════════════════════════════
 * fqm.c — 容器系统统一调度器
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   一个命令行入口, 调用所有容器系统工具。
 *
 *   命令:
 *     fqm interp <符号>              查符号信息
 *     fqm judge <A> <B>              配对审判
 *     fqm chain                      全局互锁链条报告
 *     fqm base64 e <文本>            编码
 *     fqm base64 d <符号串>          解码
 *     fqm symbols                    显示64进制符号表
 *     fkm version                    显示版本时间链
 *     fqm mailbox                    显示任务收件站
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o fqm fqm.c container_interp.c fqm_base64.c fqm_chain.c -lm
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ─────────────────────────────────────────────────────────────────────────
 * 外部符号声明 (来自其他 .c 文件)
 * ───────────────────────────────────────────────────────────────────────── */

/* container_interp.c */
typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7, T_UNKNOWN=0xFF } sym_type_t;
typedef enum { L1=0x10, L2=0x20, L3=0x30, L_UNKNOWN=0xFF } sym_level_t;
typedef enum { PASS=0, REJ_LEVEL=1, REJ_MODIFIER=2, REJ_TONE=3, REJ_EXTRA=4, REJ_UNKNOWN=5, REJ_DIFFERENT=6 } verdict_t;

extern sym_type_t identify_type(const char *name);
extern sym_level_t identify_level(const char *name);
extern const char* type_str(sym_type_t t);
extern const char* level_str(sym_level_t l);
extern const char* verdict_str(verdict_t v);

/* fqm_base64.c */
extern int fqm64_init(void);
extern int fqm64_encode(const uint8_t *data, int len, char *out, int out_max);
extern int fqm64_decode(const char *str, uint8_t *out, int out_max);
extern void fqm64_show_table(void);

/* ─────────────────────────────────────────────────────────────────────────
 * 命令: interp - 查符号信息
 * ───────────────────────────────────────────────────────────────────────── */

static int cmd_interp(int argc, char **argv) {
    if (argc < 3) { printf("用法: fqm interp <符号>\n"); return 1; }
    const char *name = argv[2];
    
    sym_type_t t = identify_type(name);
    sym_level_t l = identify_level(name);
    
    if (t == T_UNKNOWN) {
        printf("符号 '%s' 未在注册表中找到\n", name);
        return 1;
    }
    
    printf("符号: %s\n", name);
    printf("类型: %s\n", type_str(t));
    printf("层级: %s\n", level_str(l));
    
    /* 显示编码 */
    extern int n2c(const char *name);
    int code = n2c(name);
    if (code >= 0) printf("编码: 0x%02X\n", code);
    
    return 0;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 命令: judge - 配对审判
 * ───────────────────────────────────────────────────────────────────────── */

static int cmd_judge(int argc, char **argv) {
    if (argc < 4) { printf("用法: fqm judge <符号A> <符号B>\n"); return 1; }
    const char *na = argv[2];
    const char *nb = argv[3];
    
    sym_type_t ta = identify_type(na);
    sym_type_t tb = identify_type(nb);
    
    if (ta == T_UNKNOWN || tb == T_UNKNOWN) {
        printf("符号未找到, 无法审判\n");
        return 1;
    }
    
    /* 简化审判逻辑 */
    if (ta != tb) {
        printf("%s vs %s: REJECT - 类型不同 (%s vs %s)\n", na, nb, type_str(ta), type_str(tb));
        return 0;
    }
    
    sym_level_t la = identify_level(na);
    sym_level_t lb = identify_level(nb);
    
    if (la != lb) {
        printf("%s vs %s: REJECT - 跨层级 (%s vs %s)\n", na, nb, level_str(la), level_str(lb));
        return 0;
    }
    
    printf("%s vs %s: PASS - 许可配对 (%s, %s)\n", na, nb, type_str(ta), level_str(la));
    return 0;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 命令: chain - 全局互锁链条
 * ───────────────────────────────────────────────────────────────────────── */

static int cmd_chain(int argc, char **argv) {
    extern void fqmchain_run(void);
    fqmchain_run();
    return 0;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 命令: symbols - 显示64进制符号表
 * ───────────────────────────────────────────────────────────────────────── */

static int cmd_symbols(int argc, char **argv) {
    fqm64_show_table();
    return 0;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 帮助
 * ───────────────────────────────────────────────────────────────────────── */

static void show_help(void) {
    printf("\n容器系统统一调度器\n\n");
    printf("命令:\n");
    printf("  interp <符号>             查符号信息\n");
    printf("  judge <A> <B>             配对审判\n");
    printf("  chain                     全局互锁链条\n");
    printf("  symbols                   显示64进制符号表\n");
    printf("  version                   版本时间链\n");
    printf("  help                      显示帮助\n");
}

/* ─────────────────────────────────────────────────────────────────────────
 * 入口
 * ───────────────────────────────────────────────────────────────────────── */

int main(int argc, char **argv) {
    if (argc < 2) { show_help(); return 0; }
    
    const char *cmd = argv[1];
    
    /* 初始化 */
    fqm64_init();
    
    if (strcmp(cmd, "interp") == 0) return cmd_interp(argc, argv);
    if (strcmp(cmd, "judge") == 0) return cmd_judge(argc, argv);
    if (strcmp(cmd, "chain") == 0) return cmd_chain(argc, argv);
    if (strcmp(cmd, "symbols") == 0) return cmd_symbols(argc, argv);
    if (strcmp(cmd, "version") == 0) {
        printf("读 VERSION_CHAIN.md 获取完整版本时间链\n");
        return 0;
    }
    if (strcmp(cmd, "help") == 0 || strcmp(cmd, "--help") == 0 || strcmp(cmd, "-h") == 0) {
        show_help();
        return 0;
    }
    
    printf("未知命令: %s\n", cmd);
    show_help();
    return 1;
}
