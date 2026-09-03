/*
 * holo_viewer.h — 全息链浏览器
 *
 * 统一的全景视图，整合:
 *   - 链状态 (chain_block)
 *   - 记忆链 (memory_chain)
 *   - 生长环 (growth_ring)
 *   - 版本图 (version_graph)
 *   - 云观察 (observer_v2)
 *   - Swarm 状态
 *   - 3D 投影
 *   - 生命周期
 *
 * 这就是"统计文档聚合浏览器"——态势化聚类与连接包裹树。
 */

#ifndef NJSG_HOLO_VIEWER_H
#define NJSG_HOLO_VIEWER_H

#include "chain_block.h"
#include "memory_chain.h"
#include "growth_ring.h"
#include "version_graph.h"
#include "observer_v2.h"
#include "mlil_visual.h"
#include "njsg_swarm.h"
#include "lifecycle_engine.h"

/* 全息快照 — 系统某一时刻的完整状态 */
typedef struct {
    /* 时间戳 */
    uint64_t            timestamp;
    char                label[64];
    
    /* 各子系统摘要 */
    uint32_t            total_chains;
    uint32_t            total_symbols;
    uint32_t            total_memories;
    uint32_t            total_growth_events;
    uint32_t            total_agents;
    uint32_t            active_agents;
    uint32_t            total_messages;
    uint32_t            graph_nodes;
    uint32_t            graph_diameter;
    
    /* 健康度 */
    uint8_t             system_health;      /* 0-100 */
    life_stage_t        life_stage;
    
    /* 3D场景 */
    mlil_scene_t        scene;
} holo_snapshot_t;

/* 创建全息快照 */
int 全息截图(holo_snapshot_t* snap, const char* label);

/* 渲染全息视图 */
void 全息视图(const holo_snapshot_t* snap);

/* 区别对比 */
void 全息对比(const holo_snapshot_t* a, const holo_snapshot_t* b);

#endif /* NJSG_HOLO_VIEWER_H */
