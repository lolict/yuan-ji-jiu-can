/*
 * chain_block.c — 区块链式层级追踪实现
 *
 * 核心: 每个链节记录自己如何被创造，指向上一链节。
 * 系统通过遍历链条来观察自身的组成和历史。
 */

#include "chain_block.h"
#include <string.h>
#include <stdio.h>
#include <time.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

int 链_init(chain_t* chain, uint8_t category, const char* name) {
    if (!chain) return -1;
    
    memset(chain, 0, sizeof(chain_t));
    chain->category = category;
    chain->next_version = 1;  /* 版本号从 1 开始 */
    chain->count = 0;
    
    if (name) {
        strncpy(chain->name, name, CHAIN_MAX_CONTENT - 1);
        chain->name[CHAIN_MAX_CONTENT - 1] = '\0';
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 追加链节 — 创造新记录
 * ═══════════════════════════════════════════════════════════════ */

int 链_追加(chain_t* chain, uint8_t level, const char* content,
            const char* creator, const char* observation) {
    if (!chain || !content) return -1;
    if (chain->count >= CHAIN_MAX_NODES) return -1;
    
    chain_node_t* node = &chain->nodes[chain->count];
    
    /* 填充链节 */
    node->version = chain->next_version++;
    node->prev_version = (chain->count > 0) ? chain->nodes[chain->count - 1].version : 0;
    node->level = level;
    node->category = chain->category;
    
    strncpy(node->content, content, CHAIN_MAX_CONTENT - 1);
    node->content[CHAIN_MAX_CONTENT - 1] = '\0';
    
    if (creator) {
        strncpy(node->creator, creator, CHAIN_MAX_CREATOR - 1);
        node->creator[CHAIN_MAX_CREATOR - 1] = '\0';
    } else {
        strcpy(node->creator, "创世");
    }
    
    node->timestamp = (uint64_t)time(NULL);
    
    if (observation) {
        strncpy(node->observation, observation, CHAIN_MAX_OBSERVATION - 1);
        node->observation[CHAIN_MAX_OBSERVATION - 1] = '\0';
    } else {
        node->observation[0] = '\0';
    }
    
    node->flags = 0x01;  /* 可见 */
    
    chain->count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查找
 * ═══════════════════════════════════════════════════════════════ */

chain_node_t* 链_寻版本(chain_t* chain, uint64_t version) {
    if (!chain) return NULL;
    
    for (uint64_t i = 0; i < chain->count; i++) {
        if (chain->nodes[i].version == version) {
            return &chain->nodes[i];
        }
    }
    return NULL;
}

chain_node_t* 链_寻内容(chain_t* chain, const char* content) {
    if (!chain || !content) return NULL;
    
    for (uint64_t i = 0; i < chain->count; i++) {
        if (strcmp(chain->nodes[i].content, content) == 0) {
            return &chain->nodes[i];
        }
    }
    return NULL;
}

uint64_t 链_长度(const chain_t* chain) {
    return chain ? chain->count : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 追踪 — 从创世到指定版本的完整路径
 * ═══════════════════════════════════════════════════════════════ */

void 链_追踪(chain_t* chain, uint64_t version) {
    if (!chain) return;
    
    chain_node_t* target = 链_寻版本(chain, version);
    if (!target) {
        printf("版本 %lu 不存在\n", (unsigned long)version);
        return;
    }
    
    /* 从创世追踪到目标版本 */
    printf("═══ 追踪: %s (版本 %lu) ═══\n", target->content, (unsigned long)version);
    
    for (uint64_t i = 0; i < chain->count; i++) {
        chain_node_t* node = &chain->nodes[i];
        const char* arrow = (node->version == version) ? " ◄◄◄" : "";
        const char* prev_str = (node->prev_version > 0) ? "↩" : "◯";
        
        printf("  [%lu] %s %s ← %s%s\n",
               (unsigned long)node->version,
               node->content,
               prev_str,
               node->creator,
               arrow);
        
        if (node->version == version) break;
    }
}
