/**
 * weaving_balancer.c — 编织平衡器实现
 *
 * 计算每个节点的力平衡:
 *   每个节点的净力 = 施加的力 - 承受的力
 *   净力 ≈ 0 → 平衡
 *   净力很大的正数 → 这个节点压别人太多
 *   净力很大的负数 → 这个节点被压太多
 *
 * 全局平衡度 = 1 - (|所有节点净力之和| / 总边数)
 */

#include "weaving_balancer.h"
#include <string.h>
#include <math.h>

/* 平衡阈值: |net_force| < 此值视为平衡 */
#define BALANCE_THRESHOLD 0.1f

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void weaving_init(weaving_graph_t *wg) {
    memset(wg, 0, sizeof(*wg));
    wg->magic = WEAVING_MAGIC;
}

/* ═══════════════════════════════════════════════════════════════
 * 添加节点
 * ═══════════════════════════════════════════════════════════════ */

int weaving_add_node(weaving_graph_t *wg, const char *name, float weight) {
    if (!wg || wg->magic != WEAVING_MAGIC) return -1;
    if (wg->node_count >= MAX_NODES) return -1;
    if (!name) return -1;
    
    /* 重复检查 */
    int existing = weaving_find_node(wg, name);
    if (existing >= 0) return existing;
    
    uint32_t idx = wg->node_count;
    wg->nodes[idx].name = name;
    wg->nodes[idx].weight = weight;
    wg->nodes[idx].pressure_out = 0.0f;
    wg->nodes[idx].pressure_in = 0.0f;
    wg->nodes[idx].net_force = 0.0f;
    wg->nodes[idx].balanced = 0;
    wg->nodes[idx].edge_count = 0;
    wg->node_count++;
    
    return (int)idx;
}

/* ═══════════════════════════════════════════════════════════════
 * 查找节点
 * ═══════════════════════════════════════════════════════════════ */

int weaving_find_node(weaving_graph_t *wg, const char *name) {
    if (!wg || !name) return -1;
    for (uint32_t i = 0; i < wg->node_count; i++) {
        if (strcmp(wg->nodes[i].name, name) == 0) return (int)i;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 添加边
 * ═══════════════════════════════════════════════════════════════ */

int weaving_add_edge(weaving_graph_t *wg, const char *from, const char *to, float tension) {
    if (!wg || wg->magic != WEAVING_MAGIC) return -1;
    if (wg->edge_count >= MAX_EDGES) return -1;
    
    int idx_from = weaving_find_node(wg, from);
    int idx_to = weaving_find_node(wg, to);
    if (idx_from < 0 || idx_to < 0) return -1;
    
    uint32_t eidx = wg->edge_count;
    wg->edges[eidx].from = (uint32_t)idx_from;
    wg->edges[eidx].to = (uint32_t)idx_to;
    wg->edges[eidx].tension = tension;
    wg->edges[eidx].active = 1;
    wg->edge_count++;
    
    /* 立即更新节点的力 */
    wg->nodes[idx_from].pressure_out += tension;
    wg->nodes[idx_to].pressure_in += tension;
    wg->nodes[idx_from].edge_count++;
    wg->nodes[idx_to].edge_count++;
    wg->total_tension += tension;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 计算力平衡
 * ═══════════════════════════════════════════════════════════════ */

void weaving_compute_balance(weaving_graph_t *wg) {
    if (!wg || wg->magic != WEAVING_MAGIC) return;
    
    double total_abs_net = 0.0;
    wg->balanced_nodes = 0;
    wg->stressed_nodes = 0;
    
    for (uint32_t i = 0; i < wg->node_count; i++) {
        weaving_node_t *n = &wg->nodes[i];
        
        /* 净力 = 施加 - 承受 */
        n->net_force = n->pressure_out - n->pressure_in;
        total_abs_net += fabs((double)n->net_force);
        
        /* 判断是否平衡 */
        if (fabs((double)n->net_force) < BALANCE_THRESHOLD) {
            n->balanced = 1;
            wg->balanced_nodes++;
        } else {
            n->balanced = 0;
            if (fabs((double)n->net_force) > 0.5f) {
                wg->stressed_nodes++;
            }
        }
    }
    
    /* 全局平衡度 */
    if (wg->edge_count > 0) {
        wg->balance_score = 1.0f - (float)(total_abs_net / wg->edge_count);
        if (wg->balance_score < 0.0f) wg->balance_score = 0.0f;
    } else {
        wg->balance_score = 1.0f;
    }
    
    /* 全局判定 */
    wg->is_balanced = (wg->balance_score >= 0.7f && wg->stressed_nodes == 0) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 节点状态描述
 * ═══════════════════════════════════════════════════════════════ */

const char* weaving_node_status(const weaving_node_t *node) {
    if (!node) return "INVALID";
    if (node->balanced) return "BALANCED";
    if (node->net_force > 0.5f) return "OVERPRESS pressing others too much";
    if (node->net_force < -0.5f) return "OVERWHELMED pressed too much";
    return "SLIGHT_IMBALANCE";
}

/* ═══════════════════════════════════════════════════════════════
 * 诊断
 * ═══════════════════════════════════════════════════════════════ */

uint32_t weaving_diagnose(weaving_graph_t *wg) {
    if (!wg || wg->magic != WEAVING_MAGIC) return 0;
    
    uint32_t problems = 0;
    
    for (uint32_t i = 0; i < wg->node_count; i++) {
        if (!wg->nodes[i].balanced) {
            problems++;
        }
    }
    
    return problems;
}
