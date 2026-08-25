/**
 * global_interlock.c — 全局互锁实现
 *
 * 横轴齿轮转动机制:
 *   1. 新容器注册 → 横轴感知
 *   2. 新符号登记 → 横轴记录
 *   3. 新文件创建 → 必须引用横轴
 *   4. 符号冲突检测 → 横轴互锁
 *
 * 鼓膜 = 用户 (驱动源)
 * 横轴 = 本文件 (全局互锁)
 * 齿轮 = 各容器 (被驱动)
 * 圈环 = 边界 (内=历史, 外=未来)
 */

#include "global_interlock.h"
#include <string.h>
#include <stdio.h>
#include "symbol_chain.h"

/* 互锁状态 */
typedef struct {
    uint32_t    magic;
    uint16_t    version;
    uint16_t    container_count;
    uint32_t    symbol_count;
    uint32_t    total_writes;
    uint8_t     locked;         /* 互锁是否激活 */
    /* 链式传导: 文件节点 */
    char        files[GLOBAL_MAX_FILES][GLOBAL_PATH_MAX];
    uint8_t     file_containers[GLOBAL_MAX_FILES];
    uint16_t    file_count;
    /* 符号登记表 */
    uint32_t    symbols[GLOBAL_MAX_SYMBOLS];
    uint8_t     symbol_containers[GLOBAL_MAX_SYMBOLS];
} interlock_state_t;

static interlock_state_t g_state;

/* ═══════════════════════════════════════════════════════════════
 * 初始化横轴
 * ═══════════════════════════════════════════════════════════════ */

void interlock_init(void) {
    memset(&g_state, 0, sizeof(g_state));
    g_state.magic = FQM_MAGIC;
    g_state.version = FQM_VERSION;
    g_state.locked = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 容器注册 · 新齿轮套上横轴
 * ═══════════════════════════════════════════════════════════════ */

int interlock_register_container(uint8_t container_id, const char *name) {
    if (g_state.magic != FQM_MAGIC) interlock_init();
    if (g_state.container_count >= GLOBAL_MAX_CONTAINERS) return -1;
    
    g_state.container_count++;
    g_state.total_writes++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 符号登记 · 圆珠入圈
 * ═══════════════════════════════════════════════════════════════ */

int interlock_register_symbol(uint32_t symbol, uint8_t container) {
    if (g_state.magic != FQM_MAGIC) interlock_init();
    if (g_state.symbol_count >= GLOBAL_MAX_SYMBOLS) return -1;
    
    /* 检查是否已登记 */
    for (uint32_t i = 0; i < g_state.symbol_count; i++) {
        if (g_state.symbols[i] == symbol) return 0;  /* 已存在 · 幂等 */
    }
    
    g_state.symbols[g_state.symbol_count] = symbol;
    g_state.symbol_containers[g_state.symbol_count] = container;
    g_state.symbol_count++;
    g_state.total_writes++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 互锁检查 · 符号是否已登记
 * ═══════════════════════════════════════════════════════════════ */

int interlock_check_symbol(uint32_t symbol) {
    if (g_state.magic != FQM_MAGIC) return 0;

    /* 在已登记符号表中精确查找 */
    for (uint32_t i = 0; i < g_state.symbol_count; i++) {
        if (g_state.symbols[i] == symbol) return 1;
    }

    /* 未找到: 未激活时放行 · 激活后拒绝 */
    return g_state.locked ? 0 : 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 激活互锁 · 横轴开始约束所有齿轮
 * ═══════════════════════════════════════════════════════════════ */

void interlock_activate(void) {
    if (g_state.magic != FQM_MAGIC) interlock_init();
    g_state.locked = 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 链式传导: 文件级声明
 * ═══════════════════════════════════════════════════════════════ */

int interlock_declare_file(const char *filename, uint8_t container) {
    if (!filename || container == 0) return -1;
    if (g_state.magic != FQM_MAGIC) return -1;
    
    /* 查找空槽位或同名槽位 */
    int slot = -1;
    for (int i = 0; i < GLOBAL_MAX_FILES; i++) {
        if (g_state.files[i][0] == '\0') { slot = i; break; }
        if (strncmp(g_state.files[i], filename, GLOBAL_PATH_MAX-1) == 0) { slot = i; break; }
    }
    if (slot < 0) return -1;  /* 文件表满 */
    
    strncpy(g_state.files[slot], filename, GLOBAL_PATH_MAX-1);
    g_state.files[slot][GLOBAL_PATH_MAX-1] = '\0';
    g_state.file_containers[slot] = container;
    g_state.file_count++;
    g_state.total_writes++;
    
    return 0;
}

/* ── 符号白名单 (横轴上所有已注册的符号) ── */
static uint32_t registry_snapshot[GLOBAL_MAX_SYMBOLS];
static int registry_count = 0;

static void build_registry_snapshot(void) {
    registry_count = 0;
    for (int i = 0; i < GLOBAL_MAX_SYMBOLS && registry_count < GLOBAL_MAX_SYMBOLS; i++) {
        if (g_state.symbols[i]) {
            registry_snapshot[registry_count++] = g_state.symbols[i];
        }
    }
}

/* ── 检查文件的符号声明完整性 ── */
int interlock_check_file_symbols(const char *filename) {
    if (!filename) return -1;
    if (g_state.magic != FQM_MAGIC) return -1;
    
    /* 确认该文件已声明 */
    int found = 0;
    for (int i = 0; i < GLOBAL_MAX_FILES; i++) {
        if (strcmp(g_state.files[i], filename) == 0) { found = 1; break; }
    }
    if (!found) return -1;  /* 未声明的文件 */
    
    build_registry_snapshot();
    return registry_count;
}

/* ═══════════════════════════════════════════════════════════════
 * 横轴状态输出
 * ═══════════════════════════════════════════════════════════════ */

void interlock_stats(FILE *out) {
    if (!out) return;
    
    fprintf(out, "═══ 全局互锁横轴 ═══\n");
    fprintf(out, "魔环: FQM v%04X\n", g_state.version);
    fprintf(out, "容器齿轮: %d / %d\n", g_state.container_count, GLOBAL_MAX_CONTAINERS);
    fprintf(out, "符号圆珠: %d / %d\n", g_state.symbol_count, GLOBAL_MAX_SYMBOLS);
    fprintf(out, "文件节点: %d / %d\n", g_state.file_count, GLOBAL_MAX_FILES);
    fprintf(out, "总转动: %u\n", g_state.total_writes);
    fprintf(out, "互锁状态: %s\n", g_state.locked ? "已激活" : "未激活");
    fprintf(out, "════════════════════\n");
}
