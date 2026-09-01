/*
 * observer_v2.h — 云观察 v2
 *
 * 融合记忆链、生长环、版本图的综合观察引擎
 * 观察维度扩展:
 *   - 自观: 观察当前链结构
 *   - 外观: 观察外部符号
 *   - 过程: 追踪创造过程
 *   - 历史: 版本变迁
 *   - 记忆: 跨会话自传
 *   - 生长: 奖励回路轨迹
 *   - 图谱: 版本DAG结构
 */

#ifndef NJSG_OBSERVER_V2_H
#define NJSG_OBSERVER_V2_H

#include "chain_block.h"
#include "memory_chain.h"
#include "growth_ring.h"
#include "version_graph.h"
#include <stdint.h>

#define OBSV2_MAX_CHAINS    32
#define OBSV2_NAME_LEN      64

/* 扩展观察模式 */
typedef enum {
    OBSV2_SELF          = 0,  /* 自观 */
    OBSV2_EXTERNAL      = 1,  /* 外观 */
    OBSV2_PROCESS       = 2,  /* 过程观 */
    OBSV2_HISTORY       = 3,  /* 历史观 */
    OBSV2_MEMORY        = 4,  /* 记忆观 — 跨会话自传 */
    OBSV2_GROWTH        = 5,  /* 生长观 — 奖励回路 */
    OBSV2_GRAPH         = 6,  /* 图谱观 — 版本DAG */
    OBSV2_MODE_COUNT
} obsv2_mode_t;

/* v2 观察引擎 */
typedef struct {
    obsv2_mode_t        mode;
    char                name[OBSV2_NAME_LEN];
    uint64_t            total_observations;
    uint32_t            mode_counts[OBSV2_MODE_COUNT];
    
    /* 子系统 */
    chain_t*            chains[OBSV2_MAX_CHAINS];
    uint32_t            chain_count;
    memory_chain_t*     memory;
    growth_ring_t*      growth;
    version_graph_t*    graph;
    
    /* 观察深度 0-7 */
    uint8_t             perception_level;
} observer_v2_t;

/* 生命周期 */
int 云观察_v2_init(observer_v2_t* obs, const char* name);
int 云观察_v2_绑定记忆链(observer_v2_t* obs, memory_chain_t* mem);
int 云观察_v2_绑定生长环(observer_v2_t* obs, growth_ring_t* gr);
int 云观察_v2_绑定版本图(observer_v2_t* obs, version_graph_t* vg);
int 云观察_v2_注册链(observer_v2_t* obs, chain_t* chain);

/* 观察操作 */
int 云观察_v2_观察(observer_v2_t* obs, obsv2_mode_t mode);
int 云观察_v2_自观(observer_v2_t* obs);
int 云观察_v2_外观(observer_v2_t* obs, const char* target);
int 云观察_v2_过程(observer_v2_t* obs, uint32_t version);
int 云观察_v2_历史(observer_v2_t* obs, uint32_t category);
int 云观察_v2_记忆(observer_v2_t* obs);       /* 记忆观测 */
int 云观察_v2_生长(observer_v2_t* obs);       /* 生长观测 */
int 云观察_v2_图谱(observer_v2_t* obs);       /* 图谱观测 */

/* 综合报告 */
void 云观察_v2_报告(observer_v2_t* obs);

#endif /* NJSG_OBSERVER_V2_H */
