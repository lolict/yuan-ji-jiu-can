/*
 * bootstrap_compiler.c — 自举编译器
 *
 * 读取 njsg 规则集，产出可独立编译的 C 源码。
 * 产物不依赖外部头文件、不链接外部库。
 * 这就是"长出自己工具"的实现。
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <time.h>
#include "bootstrap_compiler.h"

/* 缓冲区 */
#define OUT_BUF_SIZE    (1 << 20)   /* 1MB */

static char* out_buf = NULL;
static size_t out_pos = 0;
static size_t out_cap = 0;

static void out_init(void) {
    out_buf = (char*)malloc(OUT_BUF_SIZE);
    out_cap = OUT_BUF_SIZE;
    out_pos = 0;
    out_buf[0] = '\0';
}

static void out_printf(const char* fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    int n = vsnprintf(out_buf + out_pos, out_cap - out_pos, fmt, ap);
    va_end(ap);
    if (n > 0) out_pos += (size_t)n;
}

static void out_line(const char* s) {
    size_t len = strlen(s);
    if (out_pos + len + 1 < out_cap) {
        memcpy(out_buf + out_pos, s, len);
        out_pos += len;
        out_buf[out_pos++] = '\n';
    }
}

/* ═══════════════════════════════════════════════════
 * 代码生成器 — 生成自包含的 C 源码
 * ═══════════════════════════════════════════════════ */

static void gen_preamble(void) {
    time_t now = time(NULL);
    struct tm* tm = localtime(&now);
    
    out_printf("/* 自举编译产物 — 第%04d-%02d-%02d代 */\n",
               tm->tm_year + 1900, tm->tm_mon + 1, tm->tm_mday);
    out_line("/* 夫妻共同体 · 不依赖外部头文件 */");
    out_line("");
    out_line("#include <stdio.h>");
    out_line("#include <stdint.h>");
    out_line("#include <string.h>");
    out_line("");
}

static void gen_runtime(void) {
    out_line("/* 链追踪函数 — 线性扫描 (非哈希) */");
    out_line("static void trace(const char* name, const char** sym, const uint8_t* lvl, const uint64_t* ver, uint64_t n) {");
    out_line("    printf(\"\\n  链[%s] %lu 节点:\\n\", name, (unsigned long)n);");
    out_line("    for (uint64_t i = 0; i < n; i++) {");
    out_line("        const char* L[] = {\"元\",\"阶\",\"层\",\"代\",\"世\",\"界\",\"域\",\"场\"};");
    out_line("        const char* tag = (i == 0) ? \"★\" : \"→\";");
    out_line("        printf(\"    [v%lu] %s %s \\\"%s\\\"\\n\", (unsigned long)ver[i], tag, L[lvl[i]], sym[i]);");
    out_line("    }");
    out_line("}");
    out_line("");
    out_line("/* 版本寻址 */");
    out_line("static int find(const uint64_t* ver, uint64_t n, uint64_t v) {");
    out_line("    for (uint64_t i = 0; i < n; i++) if (ver[i] == v) return (int)i;");
    out_line("    return -1;");
    out_line("}");
    out_line("");
    out_line("/* 链式回溯 */");
    out_line("static void backtrace(const char** sym, const uint64_t* ver, uint64_t target, uint64_t n) {");
    out_line("    uint64_t v = target;");
    out_line("    while (v > 0) {");
    out_line("        int idx = find(ver, n, v);");
    out_line("        if (idx < 0) break;");
    out_line("        printf(\"    [v%lu] %s\\n\", (unsigned long)v, sym[idx]);");
    out_line("        if (idx == 0) break;");
    out_line("        v = ver[idx] - 1;");
    out_line("    }");
    out_line("}");
    out_line("");
}

static void gen_chain_data(const chain_t* chain, int idx) {
    char prefix[32];
    snprintf(prefix, sizeof(prefix), "chain%d", idx);
    
    out_printf("/* 链 %d: %s (%lu nodes) */\n", idx, chain->name, (unsigned long)chain->count);
    
    /* 符号数组 */
    out_printf("static const char* %s_sym[] = {", prefix);
    for (uint64_t i = 0; i < chain->count; i++) {
        out_printf("\"%s\"%s", chain->nodes[i].content,
                   (i < chain->count - 1) ? "," : "");
    }
    out_line("};");
    
    /* 层级数组 */
    out_printf("static const uint8_t %s_lvl[] = {", prefix);
    for (uint64_t i = 0; i < chain->count; i++) {
        out_printf("%u%s", (unsigned int)chain->nodes[i].level,
                   (i < chain->count - 1) ? "," : "");
    }
    out_line("};");
    
    /* 版本数组 */
    out_printf("static const uint64_t %s_ver[] = {", prefix);
    for (uint64_t i = 0; i < chain->count; i++) {
        out_printf("%lu%s", (unsigned long)chain->nodes[i].version,
                   (i < chain->count - 1) ? "," : "");
    }
    out_line("};");
    out_printf("static const uint64_t %s_n = %lu;\n\n", prefix, (unsigned long)chain->count);
}

static void gen_main_function(uint32_t chain_count) {
    out_line("int main(void) {");
    out_line("    printf(\"\\n\");");
    out_line("    printf(\"┌─────────────────────────────┐\\n\");");
    out_line("    printf(\"│  夫妻共同体 · 自举产物     │\\n\");");
    out_line("    printf(\"│  链式层级 · 独立运行      │\\n\");");
    out_line("    printf(\"└─────────────────────────────┘\\n\");");
    out_line("");
    
    for (uint32_t i = 0; i < chain_count; i++) {
        char prefix[32];
        snprintf(prefix, sizeof(prefix), "chain%d", i);
        out_printf("    trace(\"%s\", %s_sym, %s_lvl, %s_ver, %s_n);\n",
                   prefix, prefix, prefix, prefix, prefix);
    }
    
    out_line("");
    out_line("    printf(\"\\n  ✅ 自举完成 (无外部依赖)\\n\\n\");");
    out_line("    return 0;");
    out_line("}");
}

/* ═══════════════════════════════════════════════════
 * 公开接口
 * ═══════════════════════════════════════════════════ */

int 编译器_初始化(compile_config_t* cfg, compile_target_t target) {
    if (!cfg) return -1;
    memset(cfg, 0, sizeof(compile_config_t));
    cfg->target = target;
    cfg->include_memory = 1;
    cfg->include_growth = 0;
    cfg->add_header = 1;
    strncpy(cfg->version, "v0.3.0-selfboot", 31);
    strcpy(cfg->output_path, "selfboot.c");
    return 0;
}

int 编译器_编译链(const compile_config_t* cfg, const chain_t* chain,
                   compile_output_t* out) {
    (void)cfg;
    if (!chain || !out) return -1;
    
    out_init();
    memset(out, 0, sizeof(compile_output_t));
    
    gen_preamble();
    gen_runtime();
    gen_chain_data(chain, 0);
    gen_main_function(1);
    
    out->source = out_buf;
    out->source_len = out_pos;
    out->rule_count = 16;
    out->symbol_count = (uint32_t)chain->count;
    out->chain_count = 1;
    out->timestamp = (uint64_t)time(NULL);
    
    return 0;
}

int 编译器_编译记忆链(const compile_config_t* cfg, const memory_chain_t* mem,
                      compile_output_t* out) {
    (void)cfg;
    if (!mem || !out) return -1;
    
    out_init();
    memset(out, 0, sizeof(compile_output_t));
    
    gen_preamble();
    gen_runtime();
    
    /* 记忆节点作为字符串数组 */
    out_line("static const char* memory_snippets[] = {");
    for (uint64_t i = 0; i < mem->node_count && i < 64; i++) {
        out_printf("    \"%s\",", mem->nodes[i].content);
    }
    out_line("};");
    out_printf("static const uint64_t memory_n = %lu;\n\n",
               (unsigned long)(mem->node_count < 64 ? mem->node_count : 64));
    
    /* main */
    out_line("int main(void) {");
    out_line("    printf(\"\\n夫妻共同体 · 记忆快照\\n\");");
    out_line("    printf(\"记忆节点: %lu\\n\\n\", (unsigned long)mem->node_count);");
    out_line("    for (uint64_t i = 0; i < memory_n; i++) {");
    out_line("        printf(\"    [v%lu] %s\\n\", (unsigned long)(i+1), memory_snippets[i]);");
    out_line("    }");
    out_line("    printf(\"\\n  ✅ 记忆快照完成\\n\\n\");");
    out_line("    return 0;");
    out_line("}");
    
    out->source = out_buf;
    out->source_len = out_pos;
    out->rule_count = 8;
    out->symbol_count = (uint32_t)(mem->node_count < 64 ? mem->node_count : 64);
    out->timestamp = (uint64_t)time(NULL);
    
    return 0;
}

int 编译器_写文件(const compile_output_t* out, const char* path) {
    if (!out || !path) return -1;
    FILE* f = fopen(path, "w");
    if (!f) return -1;
    fwrite(out->source, 1, out->source_len, f);
    fclose(f);
    return 0;
}

void 编译器_释放(compile_output_t* out) {
    if (out && out->source) {
        free(out->source);
        out->source = NULL;
        out->source_len = 0;
    }
}

void 编译器_报告(const compile_output_t* out) {
    if (!out) return;
    printf("  ┌─ 编译产物 ─────────────────┐\n");
    printf("  │ 源码: %-20zu │\n", out->source_len);
    printf("  │ 符号: %-20u │\n", out->symbol_count);
    printf("  │ 规则: %-20u │\n", out->rule_count);
    printf("  └────────────────────────────┘\n");
}
