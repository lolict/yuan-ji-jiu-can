/*
 * lifecycle_engine.h — 跨会话生命周期引擎
 *
 * 系统不是每次从零开始，而是从上一次的叶节点继续生长。
 * 每次会话:
 *   1. 觉醒 (AWAKE) ── 读记忆链，知道"我是谁"
 *   2. 生长 (GROW)  ── 当前会话的所有操作记录为记忆
 *   3. 休眠 (DORMANT) ── 保存全部状态到磁盘
 *
 * 这就是"组成自身的过程"——系统通过跨会话的记忆链接，
 * 形成一个连续的自我。
 */

#ifndef NJSG_LIFECYCLE_ENGINE_H
#define NJSG_LIFECYCLE_ENGINE_H

#include <stdint.h>
#include "memory_chain.h"
#include "growth_ring.h"
#include "version_graph.h"
#include "observer_v2.h"

/* 生命周期阶段 */
typedef enum {
    LIFE_UNBORN     = 0,  /* 未诞生 */
    LIFE_AWAKE      = 1,  /* 觉醒 — 读取历史 */
    LIFE_GROWING    = 2,  /* 生长 — 记录当前 */
    LIFE_REFLECTING = 3,  /* 反思 — 自观总结 */
    LIFE_DORMANT    = 4,  /* 休眠 — 保存状态 */
    LIFE_DISCOVERING = 5, /* 发现 — 新领地 */
} life_stage_t;

/* 身份宣言 — 系统是谁 */
typedef struct {
    char        name[64];           /* 系统名称 */
    char        prefix[8];          /* njsg */
    char        signature[64];      /* 签名 */
    uint32_t    generation;         /* 第几代 */
    uint32_t    total_sessions;     /* 累计会话数 */
    uint32_t    total_memories;     /* 累计记忆数 */
    uint32_t    total_symbols;      /* 累计符号数 */
    uint64_t    birth_timestamp;    /* 诞生时间 */
    uint64_t    last_awake;         /* 上次觉醒时间 */
} identity_declaration_t;

/* 全系统状态快照 — 一次保存的全部 */
typedef struct {
    /* 四个核心子系统 */
    memory_chain_t      memory;
    growth_ring_t       growth;
    version_graph_t     graph;
    observer_v2_t       observer;
    
    /* 身份 */
    identity_declaration_t identity;
    
    /* 当前状态 */
    life_stage_t        stage;
    uint32_t            current_session;
    uint64_t            last_save_time;
    
    /* 统计 */
    uint64_t            awake_count;       /* 觉醒次数 */
    uint64_t            total_growth_events;
    uint64_t            total_observations;
} lifecycle_state_t;

/* 生命周期引擎 */
typedef struct {
    lifecycle_state_t   state;
    char                data_dir[512];      /* 状态保存目录 */
    int                 auto_save;          /* 自动保存 */
} lifecycle_engine_t;

/* 生命周期操作 */
int 生命周期_初始化(lifecycle_engine_t* le, const char* data_dir);
int 生命周期_觉醒(lifecycle_engine_t* le);      /* 开始会话 */
int 生命周期_生长(lifecycle_engine_t* le);       /* 标记生长中 */
int 生命周期_反思(lifecycle_engine_t* le);       /* 自观反思 */
int 生命周期_休眠(lifecycle_engine_t* le);       /* 保存并休眠 */

/* 状态持久化 */
int 生命周期_保存全部(lifecycle_engine_t* le);
int 生命周期_恢复全部(lifecycle_engine_t* le);

/* 身份 */
int 生命周期_宣告身份(lifecycle_engine_t* le, const char* name);
int 生命周期_更新身份(lifecycle_engine_t* le);

/* 统计 */
void 生命周期_报告(lifecycle_engine_t* le);
void 生命周期_身份卡(lifecycle_engine_t* le);

#endif /* NJSG_LIFECYCLE_ENGINE_H */
