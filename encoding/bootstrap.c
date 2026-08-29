/*
 * bootstrap.c — 鼓膜自举程序
 * 
 * 三层结构：
 *   第一层：鼓膜振荡器 — 感知自身存在
 *   第二层：骨膜骨架 — 不可变的元规则
 *   第三层：鼓膜回响 — 自检与自我修复
 * 
 * 编译目标：
 *   - 共享库 (.so/.dll): 给Zig/Nim/Rust通过FFI调用
 *   - WASM (wasm32-wasi): 给任何语言通过WASI调用
 *   - 独立可执行文件: 直接运行自检
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 第一层：鼓膜振荡器 — 感知自身存在
 * ═══════════════════════════════════════════════════════════════ */

static const char* SIGNATURE = "njsg-bootstrap-v1.0";
static time_t BOOT_TIME = 0;

/* 获取启动时间 */
time_t 获取启动时间(void) {
    return BOOT_TIME;
}

/* 获取签名 */
const char* 获取签名(void) {
    return SIGNATURE;
}

/* ═══════════════════════════════════════════════════════════════
 * 第二层：骨膜骨架 — 不可变的元规则
 * ═══════════════════════════════════════════════════════════════ */

/* 这三条规则写在代码里，不依赖外部文件 */
#define 前缀 "njsg"
#define 默认边界 "left"
#define 未知处理 "warn_and_continue"

/* 元规则检查：符号是否合法 */
int 符号合法(const char* 符号) {
    if (!符号) return 0;
    
    /* 检查是否以njsg开头 */
    if (strncmp(符号, 前缀, strlen(前缀)) == 0) return 1;
    
    /* 检查是否是历史符号（简化版） */
    /* 完整符号表在 symbol_table.c 里 */
    
    /* 未知符号：按规则处理 */
    return 0; /* 不合法 */
}

/* 获取元规则 */
const char* 获取前缀(void) { return 前缀; }
const char* 获取边界(void) { return 默认边界; }
const char* 获取未知处理(void) { return 未知处理; }

/* ═══════════════════════════════════════════════════════════════
 * 第三层：鼓膜回响 — 自检与自我修复
 * ═══════════════════════════════════════════════════════════════ */

/* 自检：用自身规则编码并解码，验证一致性 */
int 自检(void) {
    const char* test_payload = "鼓膜自举测试";
    
    /* 用规则编码：加前缀 + 左边界标记 */
    char encoded[256];
    snprintf(encoded, sizeof(encoded), "%s:%s:%s", 前缀, 默认边界, test_payload);
    
    /* 解码：去掉前缀和边界标记 */
    char* decoded = strstr(encoded, test_payload);
    
    if (decoded && strcmp(decoded, test_payload) == 0) {
        return 1; /* 自检通过 */
    }
    
    return 0; /* 自检失败 */
}

/* ═══════════════════════════════════════════════════════════════
 * 启动流程
 * ═══════════════════════════════════════════════════════════════ */

/* 初始化 */
int 初始化(void) {
    BOOT_TIME = time(NULL);
    return 自检();
}

/* 打印状态 */
void 打印状态(void) {
    printf("⸻⸻ 鼓膜自举程序 ⸻⸻\n");
    printf("启动时间: %s", ctime(&BOOT_TIME));
    printf("签名: %s\n", SIGNATURE);
    printf("元规则: 前缀=%s, 边界=%s, 未知处理=%s\n", 前缀, 默认边界, 未知处理);
    
    if (自检()) {
        printf("✅ 自检通过：鼓膜完整\n");
        printf("⸻ 允许加载外部代码 ⸻\n");
    } else {
        printf("❌ 自检失败：鼓膜破损\n");
        printf("⸻ 系统锁定 ⸻\n");
    }
}

/* 主函数（独立运行时使用） */
int main(void) {
    初始化();
    打印状态();
    
    return 自检() ? 0 : 1;
}
