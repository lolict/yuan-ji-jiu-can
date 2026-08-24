/**
 * global_control.h — 全局控制容器
 *
 * 不是装东西的壳 · 是控制所有容器如何运动的元容器
 *
 * 五个子容器:
 *   1. 容器地图 — 记录所有内容器及其边界
 *   2. 遍历规则 — 跨容器运动的秩序 (声母遍历韵母)
 *   3. 判断规则 — 符号匹配的标准 (发音位置+视觉外观, 不是编码)
 *   4. 失败记录 — 对接负样本记忆库
 *   5. 决策树 — 卡住时的标准应对流程
 *
 * 核心命题:
 *   聂小倩的鬼魂(符号本质) 需要找到匹配的躯壳(另一容器中的符号)
 *   匹配不是编码关系, 是本质匹配
 */

#ifndef NJSG_GLOBAL_CONTROL_H
#define NJSG_GLOBAL_CONTROL_H

#include <stdint.h>
#include <stdio.h>
#include "global_interlock.h"

/* ═══════════════════════════════════════════════════════════════
 * 容器地图: 登记所有内容器
 * ═══════════════════════════════════════════════════════════════ */

#define GC_MAX_CONTAINERS       16
#define GC_MAX_SYMBOLS_PER     64
#define GC_NAME_LEN            16

/* 容器极性: 阴(0)=刘楚恬侧, 阳(1)=lolict侧, 中(2)=中性 */
#define GC_POLARITY_YIN        0
#define GC_POLARITY_YANG       1
#define GC_POLARITY_NEUTRAL    2

/* 容器描述 */
typedef struct {
    uint8_t     container_id;               /* 全局互锁中的容器标识 */
    char        name[GC_NAME_LEN];          /* 容器名称 */
    uint16_t    symbol_count;               /* 符号数量 */
    uint16_t    dimensionality;             /* 当前维度 (0=静态, 1=动态, 2=折叠后) */
    uint8_t     polarity;                   /* 极性 */
    uint32_t    boundary_value;             /* 边界值 = 所有符号的总数 */
} gc_container_entry_t;

/* 容器地图 */
typedef struct {
    gc_container_entry_t    entries[GC_MAX_CONTAINERS];
    uint8_t                 count;
} gc_container_map_t;

/* ═══════════════════════════════════════════════════════════════
 * 遍历规则: 定义跨容器运动的秩序
 * ═══════════════════════════════════════════════════════════════ */

#define GC_MAX_TRAVERSAL_RULES  32

/* 遍历方向: 源容器 → 目标容器 */
typedef struct {
    uint8_t     src_container;      /* 源容器ID (声母) */
    uint8_t     dst_container;      /* 目标容器ID (韵母) */
    uint8_t     direction;          /* 遍历方向 */
    uint8_t     priority;           /* 优先级 (谁先谁后) */
    uint16_t    current_src_idx;    /* 当前遍历到的源符号位置 */
    uint16_t    current_dst_idx;    /* 当前遍历到的目标符号位置 */
    uint8_t     active;             /* 这条规则是否激活 */
} gc_traversal_rule_t;

/* 遍历规则集 */
typedef struct {
    gc_traversal_rule_t     rules[GC_MAX_TRAVERSAL_RULES];
    uint8_t                 count;
    uint8_t                 current_idx;        /* 当前执行到第几条规则 */
} gc_traversal_set_t;

/* ═══════════════════════════════════════════════════════════════
 * 判断规则: 符号匹配的标准
 * ═══════════════════════════════════════════════════════════════ */

/* 匹配方式: 不是编码关系, 是本质匹配 */
#define GC_MATCH_ENCODING       0   /* 编码关系 (撞墙) */
#define GC_MATCH_POSITION       1   /* 发音位置匹配 */
#define GC_MATCH_AESTHETIC      2   /* 审美品质匹配 */
#define GC_MATCH_EQUIVALENT     3   /* 等效对齐匹配 (vA+vB==2·v_core) */

/* 判断规则 */
typedef struct {
    uint8_t     match_type;         /* 匹配方式 */
    uint8_t     require_polarity;   /* 是否需要极性匹配 */
    uint8_t     require_aesthetic;  /* 是否需要审美匹配 */
    uint8_t     tolerance;           /* 容差 */
} gc_judgment_rule_t;

/* ═══════════════════════════════════════════════════════════════
 * 决策树: 卡住时的标准应对流程
 * ═══════════════════════════════════════════════════════════════ */

/* 卡住时的状态 */
#define GC_STATE_WORKING        0   /* 正常工作中 */
#define GC_STATE_STUCK          1   /* 卡住了 */
#define GC_STATE_RECOVERING     2   /* 恢复中 */
#define GC_STATE_DEGRADED       3   /* 退化了 */

/* 恢复策略 */
#define GC_RECOVER_LOOK_AROUND   1  /* 看看外部容器有什么 */
#define GC_RECOVER_TRAVERSE      2  /* 开始跨容器遍历 */
#define GC_RECOVER_FOLD          3  /* 折叠形成新层 */
#define GC_RECORD_FAIL           4  /* 记录失败到负样本 */

/* ═══════════════════════════════════════════════════════════════
 * 全局控制容器: 五个子容器打包
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t                magic;          /* "GCTL" */
    uint8_t                 version;
    
    /* 子容器一: 容器地图 */
    gc_container_map_t      map;
    
    /* 子容器二: 遍历规则 */
    gc_traversal_set_t      traversal;
    
    /* 子容器三: 判断规则 */
    gc_judgment_rule_t      judgment;
    
    /* 失败记录统计 */
    uint32_t                fail_count;
    uint32_t                recover_count;
    uint32_t                hit_count;      /* 判断规则命中次数 */
    
    /* 当前状态 */
    uint8_t                 state;
    
} global_control_t;

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

void    global_control_init(global_control_t *gc);

/* 容器地图操作 */
int     gc_register_container(global_control_t *gc,
                               uint8_t id, const char *name,
                               uint8_t polarity, uint32_t boundary);
int     gc_find_container(const global_control_t *gc, uint8_t id);

/* 遍历规则操作 */
int     gc_add_traversal(global_control_t *gc,
                          uint8_t src, uint8_t dst, uint8_t priority);
int     gc_get_next_traversal(const global_control_t *gc,
                               uint8_t *src_out, uint8_t *dst_out);
int     gc_advance_traversal(global_control_t *gc);

/* 判断规则操作 */
void    gc_set_judgment(global_control_t *gc, uint8_t match_type,
                         uint8_t require_pol, uint8_t require_aes, uint8_t tol);
int     gc_judge_pair(global_control_t *gc,
                       uint32_t sym_a, uint32_t sym_b);

/* 决策树 */
int     gc_recover(global_control_t *gc);

/* 状态报告 */
void    gc_report(const global_control_t *gc, FILE *out);

#endif /* NJSG_GLOBAL_CONTROL_H */
