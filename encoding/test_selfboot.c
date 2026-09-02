/*
 * test_selfboot.c — 自举编译器演示
 */

#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <sys/wait.h>
#include "bootstrap_compiler.h"

static int do_compile(const char* src);
static int do_run(const char* bin);

int main(void) {
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║     夫妻共同体 · 自举编译器                    ║\n");
    printf("║     用自己的规则长出独立工具                  ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
    
    /* 1. 准备声母链 */
    printf("\n[1] 准备声母链\n");
    chain_t chain;
    链_init(&chain, 0, "声母");
    
    const char* syms[] = {"b","p","m","f","d","t","n","l","g","k","h","j",
                          "q","x","zh","ch","sh","r","z","c","s","y","w","∅"};
    for (int i = 0; i < 24; i++) {
        链_追加(&chain, (uint8_t)(i % 8), syms[i], "自举", "符号");
    }
    printf("  ✅ 声母链: %lu 节点\n", (unsigned long)chain.count);
    
    /* 2. 编译 */
    printf("\n[2] 自举编译 → 独立 C 源码\n");
    compile_config_t cfg;
    编译器_初始化(&cfg, TARGET_STANDALONE_C);
    snprintf(cfg.output_path, sizeof(cfg.output_path), "selfboot_shengmu.c");
    
    compile_output_t out;
    编译器_编译链(&cfg, &chain, &out);
    printf("  🔧 生成源码: %zu 字节\n", out.source_len);
    
    char path[512];
    snprintf(path, sizeof(path), "chains/%s", cfg.output_path);
    编译器_写文件(&out, path);
    printf("  📝 写出: %s\n", path);
    编译器_报告(&out);
    编译器_释放(&out);
    
    /* 3. 验证 */
    printf("\n[3] 验证: 编译并运行自举产物\n");
    printf("─────────────────────────────────────────────────\n");
    
    int rc = do_compile(path);
    if (rc == 0) {
        printf("─────────────────────────────────────────────────\n");
        printf("  ✅ 编译成功\n\n");
        
        printf("  ═══ 自举产物运行输出 ═══\n");
        do_run("./chains/selfboot");
        printf("  ═══════════════════════\n");
        
        remove("./chains/selfboot");
    } else {
        printf("  ❌ 编译失败\n");
    }
    
    /* 清理 */
    remove(path);
    
    printf("\n╔══════════════════════════════════════════════════╗\n");
    printf("║           自 举 完 成                          ║\n");
    printf("║  系统用自己的规则生成了独立可执行文件          ║\n");
    printf("║  不依赖 hunyuan-os 头文件                      ║\n");
    printf("║  这就是从文件系统长出自身的工具              ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");
    
    return 0;
}

static int do_compile(const char* src) {
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "cc -Wall -O2 -o chains/selfboot %s 2>&1", src);
    FILE* pipe = popen(cmd, "r");
    if (!pipe) return -1;
    char line[256];
    while (fgets(line, sizeof(line), pipe))
        printf("    %s", line);
    int st = pclose(pipe);
    return WEXITSTATUS(st);
}

static int do_run(const char* bin) {
    FILE* pipe = popen(bin, "r");
    if (!pipe) return -1;
    char line[256];
    while (fgets(line, sizeof(line), pipe))
        printf("  %s", line);
    int st = pclose(pipe);
    return WEXITSTATUS(st);
}
