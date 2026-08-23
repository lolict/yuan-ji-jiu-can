/**
 * color_system.h — 颜色建制总包
 *
 * 这是整个编码系统的总入口, 把以下模块打包为一体:
 *   ┌────────────────────────────────────────────────────────────────┐
 *   │ 1. type_discipline    — 类型警察 (类型纪律)                   │
 *   │ 2. diffraction_engine — 衍射引擎+法庭 (模式涌现)               │
 *   │ 3. weaving_balancer   — 编织平衡 (力平衡)                      │
 *   └────────────────────────────────────────────────────────────────┘
 *
 * 关卡执行顺序 (不可调换):
 *   第0关: 层级栈校验   L1/L2/L3
 *   第1关: 类型身份校验  BASE/TONE/COMPOUND/SYMBOL/EXTRA
 *   第2关: 锚校验       同锚才旋转
 *   第3关: 审美过滤     对称/比例/密度
 *   第4关: 编织平衡     力均匀分布
 *
 * 用法:
 *   #include "color_system.h"
 *   // 初始化
 *   color_system_t sys;
 *   color_system_init(&sys);
 *   // 审判一对
 *   pair_verdict_t v = color_system_trial(&sys, "a", "á"); // REJECT
 *   // 衍射分析
 *   color_system_diffract(&sys, "a", &result);
 *
 * 编译:
 *   $(CC) $(CFLAGS) -o color_system color_system.c -lm
 */

#ifndef NJSG_COLOR_SYSTEM_H
#define NJSG_COLOR_SYSTEM_H

#include "type_discipline.h"
#include "diffraction_engine.h"
#include "weaving_balancer.h"
#include <stdio.h>

/* ═══════════════════════════════════════════════════════════════
 * 颜色建制主结构
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t           magic;          /* COLOR_SYSTEM_MAGIC */
    
    /* 三个子系统 */
    type_discipline_t  police;         /* 类型警察 */
    diffraction_court_t court;         /* 法庭/衍射引擎 */
    weaving_graph_t    weaving;        /* 编织图 */
    
    /* 审判计数 */
    uint32_t           total_trials;   /* 总审判数 */
    uint32_t           total_passed;   /* 通过数 */
    uint32_t           total_rejected; /* 拒绝数 */
    
    /* 各关卡拒绝计数 */
    uint32_t           reject_level;   /* 第0关: 层级拒绝 */
    uint32_t           reject_type;    /* 第1关: 类型拒绝 */
    uint32_t           reject_anchor;  /* 第2关: 锚拒绝 */
    uint32_t           reject_aesthetic; /* 第3关: 审美拒绝 */
    uint32_t           reject_weaving; /* 第4关: 编织拒绝 */
    
    /* 输出开关 */
    uint8_t            verbose;        /* 详细输出 */
} color_system_t;

#define COLOR_SYSTEM_MAGIC 0x434F4C53  /* "COLS" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化颜色建制 (三个子系统 + 默认数据)
 */
void color_system_init(color_system_t *sys);

/**
 * 注册一个符号 (扩展用)
 * @return 0=成功, -1=失败
 */
int color_system_register(color_system_t *sys,
                           const char *name,
                           symbol_type_t type,
                           symbol_level_t level,
                           uint8_t anchor);

/**
 * 完整审判 (过所有关卡)
 * @return 0=通过, >0=犯罪类型(在第几关被拦截)
 */
pair_verdict_t color_system_trial(color_system_t *sys,
                                   const char *name_a,
                                   const char *name_b);

/**
 * 衍射分析
 */
void color_system_diffract(color_system_t *sys,
                           const char *name,
                           diffraction_result_t *result);

/**
 * 配对衍射分析
 */
void color_system_pair_diffract(color_system_t *sys,
                                 const char *name_a,
                                 const char *name_b,
                                 pair_diffraction_t *result);

/**
 * 获取统计报告
 */
void color_system_stats(color_system_t *sys, FILE *output);

/**
 * 获取层级标签字符串
 */
const char* color_system_level_str(symbol_level_t level);

/**
 * 获取类型标签字符串
 */
const char* color_system_type_str(symbol_type_t type);

#endif /* NJSG_COLOR_SYSTEM_H */
