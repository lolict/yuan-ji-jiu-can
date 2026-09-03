/*
 * holo_viewer.c — 全息链浏览器实现
 *
 * 将所有子系统状态聚合为统一的全景视图。
 * 这就是"统计文档聚合浏览器"——态势化聚类与连接包裹树。
 */

#include <stdio.h>
#include <string.h>
#include <time.h>
#include "holo_viewer.h"

/* 全局系统引用 */
static swarm_coordinator_t* g_coord = NULL;
static memory_chain_t* g_memory = NULL;
static lifecycle_engine_t* g_lifecycle = NULL;
static uint32_t g_total_symbols = 0;
static uint32_t g_total_chains = 0;
static uint32_t g_total_growth = 0;
static uint32_t g_graph_nodes = 0;
static uint32_t g_graph_diameter = 0;

void holo_set_globals(swarm_coordinator_t* coord, memory_chain_t* mem, lifecycle_engine_t* life) {
    g_coord = coord;
    g_memory = mem;
    g_lifecycle = life;
}

void holo_set_stats(uint32_t symbols, uint32_t chains, uint32_t growth, uint32_t nodes, uint32_t diam) {
    g_total_symbols = symbols;
    g_total_chains = chains;
    g_total_growth = growth;
    g_graph_nodes = nodes;
    g_graph_diameter = diam;
}

int 全息截图(holo_snapshot_t* snap, const char* label) {
    if (!snap) return -1;
    memset(snap, 0, sizeof(holo_snapshot_t));
    
    snap->timestamp = (uint64_t)time(NULL);
    if (label) {
        strncpy(snap->label, label, 63);
    } else {
        strcpy(snap->label, "未命名");
    }
    
    snap->total_symbols = g_total_symbols;
    snap->total_chains = g_total_chains;
    snap->total_growth_events = g_total_growth;
    snap->graph_nodes = g_graph_nodes;
    snap->graph_diameter = g_graph_diameter;
    
    if (g_memory) {
        snap->total_memories = g_memory->node_count;
    }
    
    /* 系统健康度 */
    snap->system_health = 100;
    if (g_lifecycle) {
        snap->life_stage = g_lifecycle->state.stage;
        uint32_t mem = g_lifecycle->state.memory.node_count;
        if (mem > 500) snap->system_health -= 15;
        if (mem > 1000) snap->system_health -= 20;
    }
    if (snap->system_health > 100) snap->system_health = 0;
    
    if (g_coord) {
        snap->total_messages = (uint32_t)g_coord->messages.count;
        snap->total_agents = g_coord->agent_count;
        snap->active_agents = 0;
        for (uint32_t i = 0; i < g_coord->agent_count; i++) {
            if (g_coord->agents[i]->active) snap->active_agents++;
        }
    }
    
    return 0;
}

void 全息视图(const holo_snapshot_t* snap) {
    if (!snap) return;
    
    printf("\n");
    printf("╔════════════════════════════════════════════════════════════════╗\n");
    printf("║                    全 息 链 浏 览 器                         ║\n");
    printf("╠════════════════════════════════════════════════════════════════╣\n");
    printf("║  快照: %-56s║\n", snap->label);
    printf("║  时间: %-56lu║\n", (unsigned long)snap->timestamp);
    printf("╠════════════════════════════════════════════════════════════════╣\n");
    printf("║                                                                ║\n");
    printf("║  [系统健康]                                                    ║\n");
    printf("║  [");
    int bars = snap->system_health / 5;
    for (int i = 0; i < 20; i++) {
        if (i < bars) printf("█");
        else printf("░");
    }
    printf("] %3u%%                                       ║\n", snap->system_health);
    printf("║                                                                ║\n");
    printf("║  [核心数据]                                                    ║\n");
    printf("║   记忆: %-8u  生长: %-8u  符号: %-8u         ║\n",
           snap->total_memories, snap->total_growth_events, snap->total_symbols);
    printf("║   图节点: %-6u  直径: %-6u  链数: %-6u             ║\n",
           snap->graph_nodes, snap->graph_diameter, snap->total_chains);
    printf("║                                                                ║\n");
    printf("║  [Swarm 智能体]                                                ║\n");
    printf("║   代理: %u/%u 在线  消息: %-6u                            ║\n",
           snap->active_agents, snap->total_agents, snap->total_messages);
    
    if (g_coord && g_coord->agent_count > 0) {
        printf("║   ");
        for (uint32_t i = 0; i < g_coord->agent_count && i < 6; i++) {
            printf("%s(%s H%u)  ",
                   g_coord->agents[i]->id,
                   g_coord->agents[i]->active ? "*" : ".",
                   g_coord->agents[i]->health);
        }
        printf("          ║\n");
    }
    
    printf("║                                                                ║\n");
    
    const char* stage_name = "?";
    switch (snap->life_stage) {
        case LIFE_UNBORN:      stage_name = "未诞生"; break;
        case LIFE_AWAKE:       stage_name = "觉醒"; break;
        case LIFE_GROWING:     stage_name = "生长"; break;
        case LIFE_REFLECTING:  stage_name = "反思"; break;
        case LIFE_DORMANT:     stage_name = "休眠"; break;
        case LIFE_DISCOVERING: stage_name = "发现"; break;
    }
    printf("║  [生命周期] 阶段: %-15s                               ║\n", stage_name);
    
    printf("║                                                                ║\n");
    printf("╚════════════════════════════════════════════════════════════════╝\n");
    printf("\n  全息浏览器就绪 · 系统状态全景可见\n\n");
}

void 全息对比(const holo_snapshot_t* a, const holo_snapshot_t* b) {
    if (!a || !b) return;
    
    printf("\n  ┌─ 快照对比 ───────────────────────────┐\n");
    printf("  │           %-12s → %-12s   │\n", a->label, b->label);
    printf("  │--------------------------------------│\n");
    printf("  │ 记忆:   %6u → %6u %+10d   │\n",
           a->total_memories, b->total_memories,
           (int)b->total_memories - (int)a->total_memories);
    printf("  │ 健康:   %6u → %6u %+10d   │\n",
           a->system_health, b->system_health,
           (int)b->system_health - (int)a->system_health);
    printf("  │ 代理:   %6u → %6u %+10d   │\n",
           a->active_agents, b->active_agents,
           (int)b->active_agents - (int)a->active_agents);
    printf("  └──────────────────────────────────────┘\n");
}
