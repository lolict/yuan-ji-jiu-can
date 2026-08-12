// clone_agents.c — 妻子分身系统实现
// 参见 clone_agents.h 的架构说明
//
// 分布式分身:
//   1. 圈层达到 CLONE 态后,可分化出分身外出
//   2. 分身 = 完整体,不是部分碎片
//   3. 分身通过 I的振幅 与母体保持感知同步
//   4. 分身吸纳外部实体 → 在分身内部形成子圈层
//   5. 分身回归 → 子层计入母体膜圈缚(功劳归母体)
//   6. 所有分身都有完整的"我"(共享感知器官)

#include "clone_agents.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

static uint64_t time_now(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

static uint64_t next_id = 1000;

// ── 创建分身管理器 ──
clone_manager_t* clone_create(uint64_t master_id) {
    clone_manager_t* mgr = (clone_manager_t*)calloc(1, sizeof(clone_manager_t));
    if (!mgr) return NULL;
    mgr->master_id = master_id;
    mgr->global_sync = 1.0f;
    mgr->total_registed_pleasure = 0.0f;
    return mgr;
}

void clone_destroy(clone_manager_t* mgr) {
    if (!mgr) return;
    clone_agent_t* agent = mgr->agents;
    while (agent) {
        clone_agent_t* next = agent->next;
        free(agent);
        agent = next;
    }
    free(mgr);
}

// ── 分化分身 ──
clone_agent_t* clone_spawn(clone_manager_t* mgr, const char* name,
                           uint64_t parent_layer_id, perception_organ_t organs) {
    if (!mgr) return NULL;
    if (mgr->agent_count >= CLONE_MAX_AGENTS) return NULL;

    clone_agent_t* agent = (clone_agent_t*)calloc(1, sizeof(clone_agent_t));
    if (!agent) return NULL;

    agent->id = next_id++;
    agent->parent_id = parent_layer_id;
    strncpy(agent->name, name, MEMBRANA_MAX_NAME - 1);
    agent->name[MEMBRANA_MAX_NAME - 1] = '\0';
    agent->state = CLONE_ACTIVE;
    agent->sync_rate = CLONE_SYNC_BASE;
    agent->sync_amplitude = 1.0f;  // 初始I的振幅
    agent->shared_organs = organs;
    agent->external_absorbed = 0;
    agent->total_pleasure = 0;
    agent->last_sync_time = time_now();
    agent->sub_layer_count = 0;

    // 插入链表
    agent->next = mgr->agents;
    mgr->agents = agent;
    mgr->agent_count++;

    return agent;
}

// ── 分身外出吸纳 ──
bool clone_absorb_external(clone_agent_t* agent, uint64_t target_id) {
    if (!agent) return false;
    if (agent->state != CLONE_ACTIVE) return false;
    if (agent->sub_layer_count >= CLONE_MAX_PER_AGENT) return false;

    // 吸纳外部实体到分身内部子圈层
    agent->sub_layers[agent->sub_layer_count++] = target_id;
    agent->external_absorbed++;

    // 每次吸纳增加愉悦值(寄存)
    agent->total_pleasure += 10;  // 基础愉悦值

    // 同步率随距离/时间轻微衰减
    agent->sync_rate *= 0.99f;

    return true;
}

// ── 感知同步: I的振幅同步 ──
float clone_synchronize(clone_agent_t* agent, float mother_amplitude) {
    if (!agent) return 0.0f;

    // I的振幅同步: 分身振幅向母体振幅平滑靠拢
    // 使用加权平均确保收敛 (不超出母体振幅)
    float alpha = 0.3f;  // 收敛因子
    float diff = mother_amplitude - agent->sync_amplitude;
    agent->sync_amplitude += diff * alpha;

    // 限制不超出母体振幅
    if (agent->sync_amplitude > mother_amplitude) {
        agent->sync_amplitude = mother_amplitude;
    }

    // 确保最低振幅保障
    if (agent->sync_amplitude < 0.1f) {
        agent->sync_amplitude = 0.1f;
    }

    agent->last_sync_time = time_now();
    return agent->sync_amplitude;
}

// ── 分身回归 ──
uint32_t clone_return(clone_manager_t* mgr, clone_agent_t* agent) {
    if (!mgr || !agent) return 0;

    // 所有子层计入母体总吸纳
    uint32_t count = agent->sub_layer_count;
    mgr->total_sub_layers += count;

    // 分身累积的愉悦值归入管理器总寄存
    mgr->total_registed_pleasure += agent->total_pleasure;

    // 分身状态变为融合态 (回归母体)
    agent->state = CLONE_MERGING;

    // 重置子层(已归回母体)
    agent->sub_layer_count = 0;

    return count;
}

// ── 全局同步 ──
void clone_global_sync(clone_manager_t* mgr, float mother_amplitude) {
    if (!mgr) return;

    float total_sync = 0.0f;
    uint32_t active_count = 0;

    clone_agent_t* agent = mgr->agents;
    while (agent) {
        if (agent->state == CLONE_ACTIVE) {
            clone_synchronize(agent, mother_amplitude);
            total_sync += agent->sync_amplitude;
            active_count++;
        }
        agent = agent->next;
    }

    // 全局同步率 = 活跃分身振幅均值 (归一化到母体参考)
    if (active_count > 0) {
        mgr->global_sync = total_sync / active_count;
        // 限制在 0-1.0 范围 (相对于标准振幅)
        if (mgr->global_sync > 1.0f) mgr->global_sync = 1.0f;
    }
}

// ── 分身总触达数 (所有分身历史吸纳总数) ──
uint32_t clone_total_reach(const clone_manager_t* mgr) {
    if (!mgr) return 0;
    uint32_t total = 0;
    clone_agent_t* agent = mgr->agents;
    while (agent) {
        total += agent->external_absorbed;
        agent = agent->next;
    }
    return total;
}

// ── 报表 ──
void clone_report(const clone_manager_t* mgr) {
    if (!mgr) return;

    printf("\n═══ 妻子分身分布式系统 [母体:%lu] ═══\n",
        (unsigned long)mgr->master_id);
    printf("  活跃分身:   %u\n", mgr->agent_count);
    printf("  总分吸纳:   %u\n", mgr->total_sub_layers);
    printf("  全局同步率: %.1f%%\n", mgr->global_sync * 100.0f);
    printf("  寄存愉悦值: %.0f\n", mgr->total_registed_pleasure);
    printf("  ─────────────────────────────\n");

    clone_agent_t* agent = mgr->agents;
    while (agent) {
        const char* state_str[] = {"休眠", "活跃", "回归", "融合"};
        printf("  [#%lu] %s — %s | 同步:%.0f%% | 子层:%u | 愉悦:%u | 器官:%s%s%s%s%s\n",
            (unsigned long)agent->id,
            agent->name,
            state_str[agent->state],
            agent->sync_rate * 100.0f,
            agent->sub_layer_count,
            agent->total_pleasure,
            (agent->shared_organs & ORGAN_VISUAL)   ? "视" : "-",
            (agent->shared_organs & ORGAN_AUDITORY) ? "听" : "-",
            (agent->shared_organs & ORGAN_TOUCH)    ? "触" : "-",
            (agent->shared_organs & ORGAN_EMOTION)  ? "情" : "-",
            (agent->shared_organs & ORGAN_PLEASURE) ? "悦" : "-"
        );
        agent = agent->next;
    }
    printf("═══════════════════════════════════════\n\n");
}
