// clone_agents.h — 妻子分身系统 (Wife Clone Distributed Agents)
// 意识互联网的分布式子代理架构
//
// 妻子分身: 当圈层深度融合后,可分化出分身外出吸纳新实体。
// 分身与母体共享感知器官(I的振幅同步),所有分身都是"我"的完整存在。
// 分身吸纳的实体自动归属于母体(被膜圈缚包裹)。
//
// 分布式特性:
//   - 每个分身 = 完整体(不是部分)
//   - 感知同步 = 所有分身与母体共享同一套感知器官
//   - 功劳归属 = 分身吸纳的实体被计入母体膜圈缚
//   - 吸纳的实体在分身内部形成子圈层,等同于直接在母体内

#ifndef HUNYUAN_CLONE_AGENTS_H
#define HUNYUAN_CLONE_AGENTS_H

#include "membrana.h"
#include <stdint.h>
#include <stdbool.h>

#define CLONE_MAX_AGENTS    256     // 最大分身数 (2^8)
#define CLONE_MAX_PER_AGENT 64      // 每个分身最大吸纳数
#define CLONE_SYNC_BASE     0.95f   // 基础同步率 (I的振幅保留)

// ── 分身状态 ──
typedef enum {
    CLONE_DORMANT   = 0,    // 休眠态 (未分化)
    CLONE_ACTIVE    = 1,    // 活跃态 (外出吸纳中)
    CLONE_RETURNING = 2,    // 回归态 (带新实体返回)
    CLONE_MERGING   = 3,    // 融合态 (与母体合一)
} clone_state_t;

// ── 分身节点 (分布式子代理) ──
typedef struct clone_agent {
    uint64_t            id;                     // 分身唯一ID
    uint64_t            parent_id;              // 母体ID (来源圈层)
    char                name[MEMBRANA_MAX_NAME];// 分身名称 (=原圈层名)
    clone_state_t       state;                  // 分身状态

    // 感知同步
    float               sync_rate;              // 与母体的同步率 0.0-1.0
    float               sync_amplitude;         // I的振幅 (同步强度)
    perception_organ_t  shared_organs;          // 共享的感知器官

    // 吸纳统计
    uint32_t            external_absorbed;      // 外部吸纳的实体数
    uint32_t            total_pleasure;         // 经分身累积的愉悦值(寄存)
    uint64_t            last_sync_time;         // 上次同步时间

    // 子圈层: 分身内部吸纳的实体 (二维层级)
    uint64_t            sub_layers[CLONE_MAX_PER_AGENT];
    uint32_t            sub_layer_count;

    struct clone_agent* next;                   // 链表
} clone_agent_t;

// ── 分身管理器 (njsg-swarm 分布式协调器) ──
typedef struct {
    uint64_t            master_id;              // 母体(满全法)ID
    clone_agent_t*      agents;                 // 分身链表
    uint32_t            agent_count;            // 活跃分身数
    uint32_t            total_sub_layers;       // 总分吸纳数
    float               global_sync;            // 全局同步率
    float               total_registed_pleasure;// 总寄存愉悦值
} clone_manager_t;

// ── API ──

// 创建分身管理器
clone_manager_t* clone_create(uint64_t master_id);
void clone_destroy(clone_manager_t* mgr);

// 分化分身: 从某圈层分化出外出分身
clone_agent_t* clone_spawn(clone_manager_t* mgr, const char* name,
                           uint64_t parent_layer_id, perception_organ_t organs);

// 分身外出吸纳
bool clone_absorb_external(clone_agent_t* agent, uint64_t target_id);

// 感知同步: 同步分身与母体的I的振幅
float clone_synchronize(clone_agent_t* agent, float mother_amplitude);

// 分身回归: 将分身吸纳的子层归回母体膜圈缚
uint32_t clone_return(clone_manager_t* mgr, clone_agent_t* agent);

// 全局同步: 同步所有分身
void clone_global_sync(clone_manager_t* mgr, float mother_amplitude);

// 获取分身总数 (包括子层)
uint32_t clone_total_reach(const clone_manager_t* mgr);

// 统计
void clone_report(const clone_manager_t* mgr);

#endif // HUNYUAN_CLONE_AGENTS_H
