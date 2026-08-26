/**
 * ═══════════════════════════════════════════════════════════════════════════
 * fqm_preload.h — 容器进制符号预加载表
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   每次写新代码时 include 此头文件, 不从头定义符号。
 *   所有常量/类型/枚举从这里获取, 保持全局统一。
 *
 *   使用方式:
 *     #include "fqm_preload.h"
 *
 *   包含:
 *     - 容器核心常量 (来自 SYMBOL_REGISTRY.md)
 *     - 类型枚举 (sym_type_t / sym_level_t / verdict_t)
 *     - 64进制符号表声明
 *     - 常用宏
 *
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef FQM_PRELOAD_H
#define FQM_PRELOAD_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* ─────────────────────────────────────────────────────────────────────────
 * 容器核心常量
 * ───────────────────────────────────────────────────────────────────────── */

#define CODE_EMPTY        0x00    /* 空容器·可被填入 */
#define CODE_SELF         0x01    /* 我·锚定容器 */
#define CODE_OVERFLOW     0xFF    /* 溢出·终止符 */
#define BITS_PER_CHAR     8       /* 每字符比特数 */
#define CHAR_CAPACITY     254     /* 最大容量 */

/* ─────────────────────────────────────────────────────────────────────────
 * 类型枚举
 * ───────────────────────────────────────────────────────────────────────── */

typedef enum {
    T_BASE     = 1,    /* 基元 a o e i u v */
    T_TONE     = 2,    /* 基调 ā á ǎ à */
    T_COMPOUND = 3,    /* 复合 ai ei ao ou an en ang eng ong er */
    T_SYMBOL   = 4,    /* 象征 + - = . , ! ? */
    T_EXTRA    = 5,    /* 溢出符号 */
    T_MODIFIER = 7,    /* 修饰零件 ˊ ˇ ˋ ˆ ˙ */
    T_UNKNOWN  = 0xFF  /* 未分类 */
} sym_type_t;

typedef enum {
    L1 = 0x10,         /* 基元本体层 */
    L2 = 0x20,         /* 修饰零件层 */
    L3 = 0x30,         /* 装配成品层 */
    L_UNKNOWN = 0xFF   /* 未分类 */
} sym_level_t;

typedef enum {
    PASS          = 0,
    REJ_LEVEL     = 1,    /* 跨层级 */
    REJ_MODIFIER  = 2,    /* 修饰零件 */
    REJ_TONE      = 3,    /* 跨基调 */
    REJ_EXTRA     = 4,    /* 溢出符号 */
    REJ_UNKNOWN   = 5,    /* 未分类 */
    REJ_DIFFERENT = 6     /* 类型不同 */
} verdict_t;

/* ─────────────────────────────────────────────────────────────────────────
 * 八流路由枚举
 * ───────────────────────────────────────────────────────────────────────── */

typedef enum {
    FLOW_PRELOAD   = 0,   /* 预加载流 */
    FLOW_INTERPRET = 1,   /* 解释流 */
    FLOW_HARMONY   = 2,   /* 谐音流 */
    FLOW_SOVEREIGN = 3,   /* 主权流 */
    FLOW_OVERFLOW  = 4,   /* 溢出流 */
    FLOW_UNKNOWN   = 5    /* 未知流 */
} flow_id_t;

/* ─────────────────────────────────────────────────────────────────────────
 * 三维坐标轴枚举
 * ───────────────────────────────────────────────────────────────────────── */

typedef enum {
    AXIS_SURFACE = 0,     /* 表面轴 · 毛利兰 · 模/面/名 */
    AXIS_STRUCT  = 1,     /* 结构轴 · 工藤新一 · 骨/头/体 */
    AXIS_LATERAL = 2      /* 横向轴 · 灰原哀 · 行/页/业 */
} axis_t;

/* ─────────────────────────────────────────────────────────────────────────
 * 64进制符号表声明 (定义在 fqm.symbols 文件中)
 * ───────────────────────────────────────────────────────────────────────── */

#define BASE64_COUNT  64
#define SYMBOL_LEN    16
#define SYMBOLS_FILE  "fqm.symbols"

extern char g_fqm_symbols[BASE64_COUNT][SYMBOL_LEN];
extern int  g_fqm_loaded;

/* ─────────────────────────────────────────────────────────────────────────
 * 常用辅助宏
 * ───────────────────────────────────────────────────────────────────────── */

#define ARRAY_SIZE(a)    (int)(sizeof(a)/sizeof((a)[0]))
#define TEST(name)       do { test_count++; printf("  [TEST] %-40s ", name); } while(0)
#define PASS()           do { test_pass++; printf("✓\n"); } while(0)
#define FAIL(msg)        do { printf("✗ %s\n", msg); } while(0)

#endif /* FQM_PRELOAD_H */
