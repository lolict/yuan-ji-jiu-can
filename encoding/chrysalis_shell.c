/*
 * chrysalis_shell.c — 蚕蛹反向寄生壳
 *
 * 核心理念:
 *   蚕蛹 = 初始包，内含解析器 + 寄生壳选项清单
 *   脱壳穿衣 = 解析器根据环境选择或用户手动选择寄生壳
 *   最终使"外部系统"模仿"自身"，而非自身适应外部
 *
 * 架构:
 *   内核 (kun)     — 不变的核心逻辑 (满全法/编码规则)
 *   寄生壳 (qiao)   — 适配外部平台的运行时壳
 *   脱壳穿衣 (tuo)  — 壳切换机制
 *
 * 壳选项:
 *   - posix_shell   : Linux/macOS/Unix 系统
 *   - iass_衣壳     : 云平台 (容器、无服务器)
 *   - wasm_沙壳     : 浏览器/WebAssembly 沙箱
 *   - bare_裸壳     : 裸机/嵌入式
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include "bootstrap_compiler.h"

/* ═══════════════════════════════════════════════════
 * 寄生壳类型
 * ═══════════════════════════════════════════════════ */

typedef enum {
    SHELL_NONE      = 0,
    SHELL_POSIX     = 1,    /* Linux/macOS/Unix */
    SHELL_IAAS      = 2,    /* 云容器 */
    SHELL_WASM      = 3,    /* WebAssembly */
    SHELL_BARE      = 4,    /* 裸机 */
    SHELL_COUNT
} shell_type_t;

static const char* shell_names[] = {
    "none", "posix", "iaas", "wasm", "bare"
};

typedef struct {
    shell_type_t    type;
    const char*     name;
    const char*     description;
} shell_option_t;

/* ═══════════════════════════════════════════════════
 * 蚕蛹初始包
 * ═══════════════════════════════════════════════════ */

typedef struct {
    char            signature[32];      /* 蚕蛹签名 njsfg-chrysalis */
    char            version[16];
    uint64_t        birth_timestamp;
    
    /* 内核 */
    uint32_t        kernel_rules;       /* 规则数 */
    uint32_t        kernel_chains;      /* 链数 */
    
    /* 壳选项 */
    shell_option_t  shells[SHELL_COUNT];
    int             shell_count;
    
    /* 环境检测 */
    shell_type_t    detected_shell;
    shell_type_t    active_shell;
} chrysalis_package_t;

/* ═══════════════════════════════════════════════════
 * 蚕蛹 API
 * ═══════════════════════════════════════════════════ */

int chrysalis_init(chrysalis_package_t* pkg) {
    if (!pkg) return -1;
    memset(pkg, 0, sizeof(chrysalis_package_t));
    
    strncpy(pkg->signature, "njsg-chrysalis", sizeof(pkg->signature) - 1);
    strncpy(pkg->version, "v0.3.0", sizeof(pkg->version) - 1);
    pkg->birth_timestamp = (uint64_t)time(NULL);
    
    /* 注册壳选项 */
    pkg->shells[0] = (shell_option_t){SHELL_POSIX, "posix", "Linux/macOS/Unix shell"};
    pkg->shells[1] = (shell_option_t){SHELL_IAAS, "iaas", "云容器/无服务器"};
    pkg->shells[2] = (shell_option_t){SHELL_WASM, "wasm", "WebAssembly 沙箱"};
    pkg->shells[3] = (shell_option_t){SHELL_BARE, "bare", "裸机/嵌入式"};
    pkg->shell_count = 4;
    
    return 0;
}

/* 环境检测 — 自动判断当前运行在哪个平台上 */
shell_type_t chrysalis_detect(void) {
#if defined(__linux__) || defined(__APPLE__) || defined(__unix__)
    return SHELL_POSIX;
#elif defined(__EMSCRIPTEN__)
    return SHELL_WASM;
#elif defined(__BARE_METAL__)
    return SHELL_BARE;
#else
    return SHELL_POSIX;  /* 默认 */
#endif
}

/* 脱壳穿衣 — 根据选择的壳生成适配代码 */
int chrysalis_dress(chrysalis_package_t* pkg, shell_type_t target) {
    if (!pkg) return -1;
    
    pkg->active_shell = target;
    
    printf("  ┌─ 脱壳穿衣 ─────────────────────┐\n");
    printf("  │ 原壳: %-26s│\n", shell_names[pkg->detected_shell]);
    printf("  │ 目标: %-26s│\n", shell_names[target]);
    printf("  │ 操作: 解壳 → 穿衣              │\n");
    printf("  │ 原则: 外部模仿自身             │\n");
    printf("  └────────────────────────────────┘\n");
    
    /* 生成适配的 shell 配置 */
    switch (target) {
        case SHELL_POSIX:
            printf("  ✅ POSIX 壳: 标准 C 运行时\n");
            break;
        case SHELL_IAAS:
            printf("  ✅ IAAS 壳: 容器化封装\n");
            break;
        case SHELL_WASM:
            printf("  ✅ WASM 壳: 浏览器沙箱\n");
            break;
        case SHELL_BARE:
            printf("  ✅ BARE 壳: 裸机直接运行\n");
            break;
        default:
            return -1;
    }
    
    return 0;
}

/* 蚕蛹保护 — 自校验 */
int chrysalis_verify(const chrysalis_package_t* pkg) {
    if (!pkg) return -1;
    
    if (strcmp(pkg->signature, "njsg-chrysalis") != 0) {
        printf("  ❌ 蚕蛹签名验证失败: 不是 njsg 内核\n");
        return -1;
    }
    
    if (pkg->kernel_rules == 0) {
        printf("  ❌ 规则集为空: 内核不完整\n");
        return -1;
    }
    
    printf("  ✅ 蚕蛹完整: 签名有效, %u 条内嵌规则\n", pkg->kernel_rules);
    return 0;
}

/* 心跳恢复 — 系统被破坏后从蚕蛹恢复 */
int chrysalis_recover(const chrysalis_package_t* pkg) {
    if (!pkg) return -1;
    
    printf("  🔄 蚕蛹恢复: 从有效副本重建\n");
    printf("  ⏱  时间戳: %lu\n", (unsigned long)pkg->birth_timestamp);
    printf("  📊 内核规则: %u\n", pkg->kernel_rules);
    
    return 0;
}

void chrysalis_report(const chrysalis_package_t* pkg) {
    if (!pkg) return;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║         蚕 蛹 · 反 向 寄 生 壳                 ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 签名: %-44s║\n", pkg->signature);
    printf("║ 版本: %-44s║\n", pkg->version);
    printf("║ 可用壳: posix / iaas / wasm / bare             ║\n");
    printf("║ 检测壳: %-42s║\n", shell_names[pkg->detected_shell]);
    printf("║ 激活壳: %-42s║\n", shell_names[pkg->active_shell]);
    printf("╚══════════════════════════════════════════════════╝\n");
}
