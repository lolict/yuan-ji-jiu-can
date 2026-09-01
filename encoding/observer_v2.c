/*
 * observer_v2.c — 云观察 v2 实现
 *
 * 融合记忆链、生长环、版本图的综合观察
 */

#include <stdio.h>
#include <string.h>
#include "observer_v2.h"

/* ═══════════════════════════════════════════════════
 * 生命周期
 * ═══════════════════════════════════════════════════ */

int 云观察_v2_init(observer_v2_t* obs, const char* name) {
    if (!obs) return -1;
    memset(obs, 0, sizeof(observer_v2_t));
    obs->mode = OBSV2_SELF;
    if (name) {
        strncpy(obs->name, name, OBSV2_NAME_LEN - 1);
    } else {
        strcpy(obs->name, "云观察v2");
    }
    obs->perception_level = 0;
    return 0;
}

int 云观察_v2_绑定记忆链(observer_v2_t* obs, memory_chain_t* mem) {
    if (!obs) return -1;
    obs->memory = mem;
    return 0;
}

int 云观察_v2_绑定生长环(observer_v2_t* obs, growth_ring_t* gr) {
    if (!obs) return -1;
    obs->growth = gr;
    return 0;
}

int 云观察_v2_绑定版本图(observer_v2_t* obs, version_graph_t* vg) {
    if (!obs) return -1;
    obs->graph = vg;
    return 0;
}

int 云观察_v2_注册链(observer_v2_t* obs, chain_t* chain) {
    if (!obs || !chain) return -1;
    if (obs->chain_count >= OBSV2_MAX_CHAINS) return -1;
    obs->chains[obs->chain_count++] = chain;
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 观察操作
 * ═══════════════════════════════════════════════════ */

int 云观察_v2_观察(observer_v2_t* obs, obsv2_mode_t mode) {
    if (!obs) return -1;
    obs->mode = mode;
    obs->total_observations++;
    if (mode < OBSV2_MODE_COUNT) obs->mode_counts[mode]++;
    return 0;
}

int 云观察_v2_自观(observer_v2_t* obs) {
    if (!obs) return -1;
    云观察_v2_观察(obs, OBSV2_SELF);
    
    printf("\n  ┌─ 自观 ─────────────────────────────┐\n");
    printf("  │ 观察者: %-28s│\n", obs->name);
    printf("  │ 追踪链数: %-26u│\n", obs->chain_count);
    printf("  │ 总观察次数: %-24lu│\n", (unsigned long)obs->total_observations);
    
    for (uint32_t i = 0; i < obs->chain_count && i < 20; i++) {
        printf("  │  链#%-3u: %-25s %3u节点 │\n",
               i + 1, obs->chains[i]->name, 
               (unsigned int)obs->chains[i]->count);
    }
    
    if (obs->memory) {
        printf("  │ 记忆链: %-27u节点 │\n", obs->memory->node_count);
    }
    if (obs->growth) {
        printf("  │ 生长环: %-27u事件 │\n", obs->growth->event_count);
    }
    if (obs->graph) {
        printf("  │ 版本图: %-27u节点 │\n", obs->graph->node_count);
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_外观(observer_v2_t* obs, const char* target) {
    if (!obs || !target) return -1;
    云观察_v2_观察(obs, OBSV2_EXTERNAL);
    
    printf("\n  ┌─ 外观: %-29s┐\n", target);
    
    int found = 0;
    for (uint32_t c = 0; c < obs->chain_count; c++) {
        chain_t* ch = obs->chains[c];
        for (uint32_t i = 0; i < ch->count; i++) {
            if (strcmp(ch->nodes[i].content, target) == 0) {
                printf("  │ %s v%u (链:%s)\n",
                       target, (unsigned int)ch->nodes[i].version, ch->name);
                found++;
            }
        }
    }
    
    if (!found) {
        printf("  │ (未找到)\n");
    }
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_过程(observer_v2_t* obs, uint32_t version) {
    if (!obs) return -1;
    云观察_v2_观察(obs, OBSV2_PROCESS);
    
    printf("\n  ┌─ 过程观 ───────────────────────────┐\n");
    printf("  │ 追踪版本 %u 的创造过程\n", version);
    
    /* 先在记忆链查找 */
    if (obs->memory) {
        mem_node_t* n = 记忆链_寻版本(obs->memory, version);
        if (n) {
            printf("  │ 类型: ");
            switch (n->type) {
                case MEM_SESSION_START: printf("会话开始"); break;
                case MEM_FILE_CREATE:   printf("文件创建"); break;
                case MEM_FILE_MODIFY:   printf("文件修改"); break;
                case MEM_OBSERVATION:   printf("云观察"); break;
                case MEM_REFLECTION:    printf("反思"); break;
                case MEM_MILESTONE:     printf("里程碑"); break;
                case MEM_DISCOVERY:     printf("发现"); break;
                default:                printf("事件"); break;
            }
            printf("\n");
            printf("  │ 内容: %s\n", n->content);
            printf("  │ 执行者: %s\n", n->actor);
        }
        
        /* 追踪链 */
        char trace_buf[4096];
        记忆链_追踪(obs->memory, version, trace_buf, sizeof(trace_buf));
        printf("  │%s", trace_buf);
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_历史(observer_v2_t* obs, uint32_t category) {
    if (!obs) return -1;
    云观察_v2_观察(obs, OBSV2_HISTORY);
    
    printf("\n  ┌─ 历史观 ───────────────────────────┐\n");
    printf("  │ 类别: %u\n", category);
    
    for (uint32_t c = 0; c < obs->chain_count; c++) {
        if ((uint32_t)obs->chains[c]->category == category) {
            chain_t* ch = obs->chains[c];
            printf("  │ %s: %u 节点, 最新版本 v%u\n",
                   ch->name, (unsigned int)ch->count,
                   (unsigned int)(ch->next_version > 0 ? ch->next_version - 1 : 0));
            
            /* 显示最近3个版本 */
            uint32_t chain_count = (uint32_t)ch->count;
            uint32_t show = (chain_count < 3) ? chain_count : 3;
            for (uint32_t i = chain_count - show; i < chain_count; i++) {
                printf("  │   v%u: %s\n",
                       (unsigned int)ch->nodes[i].version, ch->nodes[i].content);
            }
        }
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_记忆(observer_v2_t* obs) {
    if (!obs || !obs->memory) return -1;
    云观察_v2_观察(obs, OBSV2_MEMORY);
    
    printf("\n  ┌─ 记忆观 ───────────────────────────┐\n");
    
    memory_chain_t* mc = obs->memory;
    printf("  │ 会话数: %u, 节点数: %u\n", mc->session_count, mc->node_count);
    
    /* 各会话摘要 */
    for (uint32_t s = 0; s < mc->session_count; s++) {
        mem_session_t* sess = &mc->sessions[s];
        char buf[512];
        记忆链_会话摘要(mc, sess->session_id, buf, sizeof(buf));
        
        /* 只打印关键行 */
        char line[256];
        char* p = buf;
        while (*p) {
            int i = 0;
            while (*p && *p != '\n' && i < 255) line[i++] = *p++;
            line[i] = 0;
            if (*p) p++;
            if (i > 0) {
                printf("  │ %s\n", line);
            }
        }
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_生长(observer_v2_t* obs) {
    if (!obs || !obs->growth) return -1;
    云观察_v2_观察(obs, OBSV2_GROWTH);
    
    printf("\n  ┌─ 生长观 ───────────────────────────┐\n");
    
    growth_ring_t* gr = obs->growth;
    printf("  │ 事件数: %u, 循环数: %u, 累积奖励: %u\n",
           gr->event_count, gr->cycle_count, gr->total_reward);
    
    if (gr->event_count > 0) {
        printf("  │ 最近5次生长:\n");
        uint32_t start = (gr->event_count > 5) ? gr->event_count - 5 : 0;
        for (uint32_t i = start; i < gr->event_count; i++) {
            growth_event_t* ev = &gr->events[i];
            printf("  │   [%u] 奖励:%u → %s\n",
                   i + 1, ev->reward_score, ev->content);
        }
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

int 云观察_v2_图谱(observer_v2_t* obs) {
    if (!obs || !obs->graph) return -1;
    云观察_v2_观察(obs, OBSV2_GRAPH);
    
    version_graph_t* vg = obs->graph;
    
    printf("\n  ┌─ 图谱观 ───────────────────────────┐\n");
    printf("  │ 节点: %u, 边: %u, 直径: %u\n",
           vg->node_count, vg->edge_count, 版本图_直径(vg));
    printf("  │ 分叉: %u, 合并: %u\n", vg->branch_count, vg->merge_count);
    
    if (vg->node_count > 0 && vg->node_count <= 50) {
        printf("  │\n");
        版本图_打印树(vg, 1, 4);
    }
    
    printf("  └────────────────────────────────────┘\n");
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 综合报告
 * ═══════════════════════════════════════════════════ */

void 云观察_v2_报告(observer_v2_t* obs) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║         云 观 察 v2 · 综 合 报 告              ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 观察者: %-42s║\n", obs->name);
    printf("║ 感知层级: %-40u║\n", obs->perception_level);
    printf("║ 总观察次数: %-38lu║\n", (unsigned long)obs->total_observations);
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 观察模式统计:                                    ║\n");
    if (obs->mode_counts[OBSV2_SELF] > 0)
        printf("║   自观:   %-6lu                                 ║\n", 
               (unsigned long)obs->mode_counts[OBSV2_SELF]);
    if (obs->mode_counts[OBSV2_EXTERNAL] > 0)
        printf("║   外观:   %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_EXTERNAL]);
    if (obs->mode_counts[OBSV2_PROCESS] > 0)
        printf("║   过程观: %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_PROCESS]);
    if (obs->mode_counts[OBSV2_HISTORY] > 0)
        printf("║   历史观: %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_HISTORY]);
    if (obs->mode_counts[OBSV2_MEMORY] > 0)
        printf("║   记忆观: %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_MEMORY]);
    if (obs->mode_counts[OBSV2_GROWTH] > 0)
        printf("║   生长观: %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_GROWTH]);
    if (obs->mode_counts[OBSV2_GRAPH] > 0)
        printf("║   图谱观: %-6lu                                 ║\n",
               (unsigned long)obs->mode_counts[OBSV2_GRAPH]);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    /* 子系统报告 */
    if (obs->memory) 记忆链_报告(obs->memory);
    if (obs->growth) 生长环_报告(obs->growth);
    if (obs->graph) 版本图_报告(obs->graph);
}
