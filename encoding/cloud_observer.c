/*
 * cloud_observer.c — 云观察引擎实现
 *
 * 系统通过遍历自身链条来感知自己的存在。
 * 不是外部监控，是内在的自我观察。
 */

#include "cloud_observer.h"
#include <stdio.h>
#include <string.h>

/* 层级名称 */
static const char* 层级名[] = {
    "元", "阶", "层", "代", "世", "界", "域", "场"
};

/* 族名称 */
static const char* 族名[] = {
    "声母", "韵母", "声调", "容器", "操作", "极性", "层级", "审判", "产权"
};

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

int 云观察_init(cloud_observer_t* obs, const char* name) {
    if (!obs) return -1;
    
    memset(obs, 0, sizeof(cloud_observer_t));
    obs->chain_count = 0;
    obs->total_observations = 0;
    
    if (name) {
        strncpy(obs->state.name, name, OBSERVER_NAME_LEN - 1);
        obs->state.name[OBSERVER_NAME_LEN - 1] = '\0';
    } else {
        strcpy(obs->state.name, "云观察者");
    }
    
    obs->state.mode = OBSERVE_SELF;
    obs->state.awareness_level = 0;
    
    return 0;
}

int 云观察_注册链(cloud_observer_t* obs, chain_t* chain) {
    if (!obs || !chain) return -1;
    if (obs->chain_count >= OBSERVER_MAX_CHAINS) return -1;
    
    obs->chains[obs->chain_count] = chain;
    obs->chain_count++;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 自观 — 观察自身由什么组成
 * ═══════════════════════════════════════════════════════════════ */

int 云观察_自观(cloud_observer_t* obs) {
    if (!obs) return -1;
    
    obs->state.mode = OBSERVE_SELF;
    obs->state.observation_count++;
    obs->total_observations++;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【云观察 · 自观】                       ║\n");
    printf("║     系统观察自身的组成 — 我由什么构成             ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 观察者: %-12s  第 %lu 次观察              ║\n",
           obs->state.name, (unsigned long)obs->state.observation_count);
    printf("╠══════════════════════════════════════════════════╣\n");
    
    uint64_t total_symbols = 0;
    
    for (uint8_t i = 0; i < obs->chain_count; i++) {
        chain_t* ch = obs->chains[i];
        if (!ch || ch->count == 0) continue;
        
        printf("║                                                  ║\n");
        printf("║ 【%s链】 共 %lu 个符号                        ║\n",
               ch->name, (unsigned long)ch->count);
        printf("║                                                  ║\n");
        
        /* 按层级统计 */
        uint64_t level_count[8] = {0};
        for (uint64_t j = 0; j < ch->count; j++) {
            if (ch->nodes[j].level < 8) {
                level_count[ch->nodes[j].level]++;
            }
        }
        
        for (uint8_t lvl = 0; lvl < 8; lvl++) {
            if (level_count[lvl] > 0) {
                printf("║   %s层: %lu 个                              ║\n",
                      层级名[lvl], (unsigned long)level_count[lvl]);
            }
        }
        
        total_symbols += ch->count;
    }
    
    printf("║                                                  ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 总计: %lu 条链, %lu 个符号                    ║\n",
           (unsigned long)obs->chain_count, (unsigned long)total_symbols);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 外观 — 观察外部输入
 * ═══════════════════════════════════════════════════════════════ */

int 云观察_外观(cloud_observer_t* obs, const char* target) {
    if (!obs) return -1;
    
    obs->state.mode = OBSERVE_EXTERNAL;
    obs->state.observation_count++;
    obs->total_observations++;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【云观察 · 外观】                       ║\n");
    printf("║     系统观察外部 — 外部有什么                     ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    
    if (target) {
        printf("║ 观察目标: %s\n", target);
        printf("║                                                  ║\n");
        
        uint8_t found = 0;
        for (uint8_t i = 0; i < obs->chain_count; i++) {
            chain_t* ch = obs->chains[i];
            chain_node_t* node = 链_寻内容(ch, target);
            if (node) {
                printf("║ 发现于 【%s链】 版本 %lu                     ║\n",
                       ch->name, (unsigned long)node->version);
                printf("║   层级: %s                                   ║\n",
                      层级名[node->level]);
                printf("║   创造者: %s                                ║\n",
                      node->creator);
                if (node->observation[0]) {
                    printf("║   观察记录: %s                          ║\n",
                          node->observation);
                }
                found = 1;
            }
        }
        
        if (!found) {
            printf("║ 未在任何链中发现 \"%s\"                     ║\n", target);
        }
    }
    
    printf("╚══════════════════════════════════════════════════╝\n");
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 过程观 — 观察某个版本的创造过程
 * ═══════════════════════════════════════════════════════════════ */

int 云观察_过程(cloud_observer_t* obs, uint64_t version) {
    if (!obs) return -1;
    
    obs->state.mode = OBSERVE_PROCESS;
    obs->state.observation_count++;
    obs->total_observations++;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【云观察 · 过程观】                     ║\n");
    printf("║     追踪版本的创造过程 — 我是如何被创造的         ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    
    for (uint8_t i = 0; i < obs->chain_count; i++) {
        chain_t* ch = obs->chains[i];
        chain_node_t* target = 链_寻版本(ch, version);
        if (target) {
            printf("║ 在 【%s链】 发现版本 %lu:                   ║\n",
                   ch->name, (unsigned long)version);
            printf("║                                                  ║\n");
            printf("║ 创造链条:                                        ║\n");
            
            for (uint64_t j = 0; j < ch->count; j++) {
                chain_node_t* node = &ch->nodes[j];
                const char* marker = (node->version == version) ? " ◄◄◄ 当前" : "";
                
                printf("║   [v%lu] %s ← 由[%s]创造                    ║\n",
                       (unsigned long)node->version,
                       node->content,
                       node->creator);
                
                if (node->observation[0]) {
                    printf("║         观察: %s                         ║\n",
                          node->observation);
                }
                
                if (node->version == version) break;
            }
            
            printf("╚══════════════════════════════════════════════════╝\n");
            return 0;
        }
    }
    
    printf("║ 版本 %lu 不存在于任何链中                    ║\n", (unsigned long)version);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 历史观 — 观察某条链的版本变迁
 * ═══════════════════════════════════════════════════════════════ */

int 云观察_历史(cloud_observer_t* obs, uint8_t category) {
    if (!obs) return -1;
    
    obs->state.mode = OBSERVE_HISTORY;
    obs->state.observation_count++;
    obs->total_observations++;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【云观察 · 历史观】                     ║\n");
    printf("║     观察版本变迁 — 我从哪里来                     ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    
    for (uint8_t i = 0; i < obs->chain_count; i++) {
        chain_t* ch = obs->chains[i];
        if (ch->category != category) continue;
        
        printf("║ 【%s链】 共 %lu 个版本                       ║\n",
               ch->name, (unsigned long)ch->count);
        printf("║                                                  ║\n");
        
        for (uint64_t j = 0; j < ch->count; j++) {
            chain_node_t* node = &ch->nodes[j];
            const char* prev = (node->prev_version > 0) ? "↩" : "◯创世";
            
            printf("║   [v%lu] %s %s %-10s %s层          ║\n",
                   (unsigned long)node->version,
                   node->content,
                   prev,
                   node->creator,
                  层级名[node->level]);
        }
        
        printf("╚══════════════════════════════════════════════════╝\n");
        return 0;
    }
    
    printf("║ 类别 %s 的链不存在                          ║\n", 族名[category]);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 生成观察报告
 * ═══════════════════════════════════════════════════════════════ */

void 云观察_报告(cloud_observer_t* obs) {
    if (!obs) return;
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           【云观察 · 总报告】                     ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 观察者: %-12s                             ║\n", obs->state.name);
    printf("║ 总观察次数: %lu                              ║\n", (unsigned long)obs->total_observations);
    printf("║ 追踪链数: %d                                  ║\n", obs->chain_count);
    printf("║                                                  ║\n");
    
    uint64_t total_symbols = 0;
    for (uint8_t i = 0; i < obs->chain_count; i++) {
        if (obs->chains[i]) {
            total_symbols += obs->chains[i]->count;
        }
    }
    
    printf("║ 符号总数: %lu                              ║\n", (unsigned long)total_symbols);
    printf("║                                                  ║\n");
    printf("║ 各链状态:                                        ║\n");
    
    for (uint8_t i = 0; i < obs->chain_count; i++) {
        chain_t* ch = obs->chains[i];
        if (!ch) continue;
        printf("║   %s链: %lu 个符号, 最新版本 v%lu        ║\n",
               ch->name, (unsigned long)ch->count,
               (unsigned long)(ch->next_version - 1));
    }
    
    printf("╚══════════════════════════════════════════════════╝\n");
}
