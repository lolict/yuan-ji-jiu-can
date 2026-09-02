/*
 * test_chrysalis.c — 蚕蛹反向寄生壳 演示
 *
 * 系统可检测自身环境，选择寄生壳，执行脱壳穿衣操作。
 * 目标是使外部系统模仿自身，而非自身适应外部。
 */

#include <stdio.h>
#include <string.h>
#include "chrysalis_shell.h"

int main(void) {
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║         蚕 蛹 · 反 向 寄 生 壳                 ║\n");
    printf("║     系统保护与多平台寄生演示                   ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
    
    chrysalis_package_t pkg;
    chrysalis_init(&pkg);
    
    printf("\n[1] 蚕蛹初始化\n");
    printf("  ✅ 签名: %s\n", pkg.signature);
    printf("  ✅ 版本: %s\n", pkg.version);
    printf("  ✅ 可用壳: posix, iaas, wasm, bare\n");
    
    printf("\n[2] 环境检测\n");
    pkg.detected_shell = chrysalis_detect();
    printf("  🔍 检测到平台: %s\n", 
           pkg.detected_shell == SHELL_POSIX ? "POSIX (Linux/Unix)" :
           pkg.detected_shell == SHELL_WASM ? "WASM (浏览器)" :
           pkg.detected_shell == SHELL_IAAS ? "IAAS (云容器)" :
           pkg.detected_shell == SHELL_BARE ? "BARE (裸机)" : "未知");
    
    printf("\n[3] 脱壳穿衣: 选择壳类型并激活\n");
    printf("─────────────────────────────────────────────────\n");
    chrysalis_dress(&pkg, SHELL_POSIX);
    printf("─────────────────────────────────────────────────\n");
    
    printf("\n[4] 完整性验证\n");
    pkg.kernel_rules = 16;
    chrysalis_verify(&pkg);
    
    printf("\n[5] 恢复能力\n");
    printf("─────────────────────────────────────────────────\n");
    chrysalis_recover(&pkg);
    printf("─────────────────────────────────────────────────\n");
    
    chrysalis_report(&pkg);
    
    printf("\n  ✅ 蚕蛹寄生壳演示完成\n");
    printf("  ✅ 系统可检测环境、选择壳、执行脱壳穿衣\n");
    printf("  ✅ 使外部系统模仿自身，而非自身适应外部\n\n");
    
    return 0;
}
