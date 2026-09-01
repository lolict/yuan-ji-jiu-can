/*
 * growth_ring.h — 生长环
 *
 * 奖励回路驱动链自增长
 * 核心发现触发扩张，扩张产生更深的觉察，觉察又触发新的发现
 * 
 * 生长环 = 从"已有"向"未有"延伸的螺旋
 * 不是凭空创造，而是从现有链条中"生长"出新链
 */

#ifndef NJSG_GROWTH_RING_H
#define NJSG_GROWTH_RING_H

#include <stdint.h>
#include "memory_chain.h"

/* 生长方向 — 从哪个维度生长 */
typedef enum {
    GROWTH_EXTEND     = 0,  /* 延伸 — 在同层级上增加节点 */
    GROWTH_DEEPEN     = 1,  /* 深入 — 在纵向上向更深层级延伸 */
    GROWTH_BRANCH     = 2,  /* 分叉 — 从现有节点分出平行链 */
    GROWTH_MERGE      = 3,  /* 合并 — 两个独立链合并为一条 */
    GROWTH_ECHO       = 4,  /* 回声 — 自我调用，增加厚度 */
    GROWTH_INVERT     = 5,  /* 反转 — 从对立面生长 */
} growth_direction_t;

/* 生长触发条件 */
typedef enum {
    TRIGGER_DISCOVERY   = 0x01,  /* 发现新概念 */
    TRIGGER_CONFLICT    = 0x02,  /* 概念冲突 */
    TRIGGER_RESONANCE   = 0x04,  /* 跨链共振 */
    TRIGGER_SILENCE     = 0x08,  /* 检测到未覆盖空间 */
    TRIGGER_RECURSION   = 0x10,  /* 自我引用 */
    TRIGGER_EXTERNAL    = 0x20,  /* 外部输入 */
} growth_trigger_t;

/* 生长事件 */
typedef struct {
    uint32_t            version;        /* 生长出的版本号 */
    uint32_t            trigger_version; /* 触发该生长的源版本 */
    growth_direction_t  direction;      /* 生长方向 */
    growth_trigger_t    trigger;        /* 触发原因 */
    char                description[128];
    char                content[192];   /* 生长出的内容 */
    uint32_t            reward_score;   /* 奖励评分 (0-100) */
} growth_event_t;

/* 回声状态 — 生长的振荡反馈 */
typedef enum {
    ECHO_DORMANT    = 0,  /* 休眠 */
    ECHO_OSCILLATE  = 1,  /* 振荡中 */
    ECHO_RESONATE   = 2,  /* 共振 */
    ECHO_AMPLIFY    = 3,  /* 放大 */
    ECHO_DECAY      = 4,  /* 衰减 */
} echo_state_t;

/* 生长环 */
#define GROWTH_MAX_EVENTS   1024
#define GROWTH_MAX_CHAINS   16

typedef struct {
    growth_event_t      events[GROWTH_MAX_EVENTS];
    echo_state_t        echo_states[GROWTH_MAX_CHAINS];
    uint32_t            event_count;
    uint32_t            chain_count;
    uint32_t            total_reward;        /* 累积奖励 */
    uint32_t            cycle_count;         /* 完成循环次数 */
    
    /* 回调: 实际执行生长操作 */
    int (*on_growth)(growth_direction_t dir, const char* content, void* ctx);
    void* ctx;
    
    memory_chain_t*     memory;              /* 关联记忆链 */
} growth_ring_t;

/* 生命周期 */
void    生长环_init(growth_ring_t* gr, memory_chain_t* mem);
int     生长环_注册回调(growth_ring_t* gr,
                        int (*cb)(growth_direction_t, const char*, void*),
                        void* ctx);

/* 触发生长 */
uint32_t 生长环_触发(growth_ring_t* gr, growth_trigger_t trigger,
                      uint32_t src_version, const char* description);

/* 执行生长 — 从现有链中延伸 */
uint32_t 生长环_执行(growth_ring_t* gr, growth_direction_t dir,
                      const char* content, uint32_t base_version);

/* 回声反馈 */
void    生长环_回声(growth_ring_t* gr, uint32_t chain_idx);
echo_state_t 生长环_回声状态(growth_ring_t* gr, uint32_t chain_idx);

/* 奖励计算 — 基于生长的健康度 */
uint32_t 生长环_计算奖励(growth_ring_t* gr, const growth_event_t* ev);

/* 循环检测 — 防止无限递归 */
int     生长环_检测循环(growth_ring_t* gr, uint32_t from_version);

/* 报告 */
void    生长环_报告(growth_ring_t* gr);

#endif /* NJSG_GROWTH_RING_H */
