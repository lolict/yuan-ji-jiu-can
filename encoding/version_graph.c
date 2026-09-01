/*
 * version_graph.c — 版本图追踪实现
 *
 * 从链中构建版本图（有向无环图）
 * 追踪分叉、合并、深度、路径
 */

#include <stdio.h>
#include <string.h>
#include "version_graph.h"

/* ═══════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════ */

void 版本图_init(version_graph_t* vg) {
    memset(vg, 0, sizeof(version_graph_t));
    vg->chains = NULL;
    vg->chain_count = 0;
    vg->memory = NULL;
}

int 版本图_添加链(version_graph_t* vg, chain_t* chain) {
    (void)chain;
    return 0;
}

int 版本图_添加记忆链(version_graph_t* vg, memory_chain_t* mem) {
    if (!vg || !mem) return -1;
    vg->memory = mem;
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 构建图 — 从零构建所有节点和边
 * ═══════════════════════════════════════════════════ */

int 版本图_构建(version_graph_t* vg) {
    if (!vg->memory) return 0;
    
    memory_chain_t* mc = vg->memory;
    uint32_t base = 1;
    
    /* 第一遍：建立所有节点 */
    for (uint32_t i = 0; i < mc->node_count && vg->node_count < VG_MAX_NODES; i++) {
        mem_node_t* mn = &mc->nodes[i];
        uint32_t ver = base + mn->version - 1;
        
        if (版本图_寻节点(vg, ver)) continue;
        
        vg_node_t* vn = &vg->nodes[vg->node_count];
        memset(vn, 0, sizeof(vg_node_t));
        vn->version = ver;
        vn->parent_count = (mn->prev_version > 0) ? 1 : 0;
        vn->parents[0] = (mn->prev_version > 0) ? base + mn->prev_version - 1 : 0;
        vn->child_count = 0;
        
        switch (mn->type) {
            case MEM_SESSION_START: vn->type = VG_NODE_ROOT; break;
            case MEM_MILESTONE:     vn->type = VG_NODE_BRANCH; break;
            case MEM_DISCOVERY:     vn->type = VG_NODE_BRANCH; break;
            default:                vn->type = VG_NODE_LINEAR; break;
        }
        
        vn->level = (uint8_t)(mn->depth < 8 ? mn->depth : 7);
        
        vg->node_count++;
    }
    
    /* 第二遍：建立边 */
    for (uint32_t i = 0; i < mc->node_count; i++) {
        mem_node_t* mn = &mc->nodes[i];
        if (mn->prev_version == 0) continue;
        
        uint32_t from = base + mn->prev_version - 1;
        uint32_t to = base + mn->version - 1;
        
        for (uint32_t j = 0; j < vg->node_count; j++) {
            if (vg->nodes[j].version == from) {
                if (vg->nodes[j].child_count < 16) {
                    vg->nodes[j].children[vg->nodes[j].child_count++] = to;
                    if (vg->nodes[j].child_count > 1) {
                        vg->nodes[j].type = VG_NODE_BRANCH;
                    }
                }
                break;
            }
        }
        
        if (vg->edge_count < VG_MAX_EDGES) {
            vg_edge_t* e = &vg->edges[vg->edge_count];
            e->from = from;
            e->to = to;
            e->weight = 1;
            vg->edge_count++;
        }
    }
    
    /* 统计 */
    vg->branch_count = 0;
    vg->merge_count = 0;
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].child_count > 1) vg->branch_count++;
        if (vg->nodes[i].parent_count > 1) vg->merge_count++;
    }
    
    /* 标记根节点和计算深度 */
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].parent_count == 0 || 
            (vg->nodes[i].parent_count == 1 && vg->nodes[i].parents[0] == 0)) {
            vg->nodes[i].depth = 0;
            vg->nodes[i].type = VG_NODE_ROOT;
        }
    }
    
    /* BFS 计算深度 (迭代收敛) */
    for (uint32_t iter = 0; iter < 128; iter++) {
        int changed = 0;
        for (uint32_t i = 0; i < vg->node_count; i++) {
            vg_node_t* n = &vg->nodes[i];
            if (n->parent_count == 0) continue;
            if (n->parents[0] == 0) continue;
            if (n->depth > 0 && n->depth < 999) continue;
            
            uint32_t min_pd = 999;
            for (uint32_t p = 0; p < n->parent_count; p++) {
                vg_node_t* pn = 版本图_寻节点(vg, n->parents[p]);
                if (pn && pn->depth < min_pd) min_pd = pn->depth;
            }
            if (min_pd < 999) {
                n->depth = min_pd + 1;
                changed = 1;
            }
        }
        if (!changed) break;
    }
    
    /* 标记叶节点 */
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].child_count == 0 && vg->nodes[i].type != VG_NODE_ROOT) {
            vg->nodes[i].type = VG_NODE_LEAF;
        }
    }
    
    return (int)vg->node_count;
}

/* ═══════════════════════════════════════════════════
 * 查询
 * ═══════════════════════════════════════════════════ */

vg_node_t* 版本图_寻节点(version_graph_t* vg, uint32_t version) {
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].version == version) return &vg->nodes[i];
    }
    return NULL;
}

vg_node_t* 版本图_寻分叉点(version_graph_t* vg, uint32_t after_version) {
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].version > after_version && 
            vg->nodes[i].type == VG_NODE_BRANCH) {
            return &vg->nodes[i];
        }
    }
    return NULL;
}

uint32_t 版本图_寻路径(version_graph_t* vg, uint32_t from, uint32_t to,
                         uint32_t* path_out, uint32_t max_len) {
    uint32_t path[128];
    uint32_t len = 0;
    
    uint32_t cur = from;
    while (cur != to && cur > 0 && len < 127) {
        path[len++] = cur;
        vg_node_t* n = 版本图_寻节点(vg, cur);
        if (!n || n->parent_count == 0) break;
        cur = n->parents[0];
    }
    
    if (cur == to && len < 127) {
        path[len++] = to;
        uint32_t out_len = (len < max_len) ? len : max_len;
        for (uint32_t i = 0; i < out_len; i++) {
            path_out[i] = path[out_len - 1 - i];
        }
        return out_len;
    }
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 分析
 * ═══════════════════════════════════════════════════ */

uint32_t 版本图_深度(version_graph_t* vg, uint32_t version) {
    vg_node_t* n = 版本图_寻节点(vg, version);
    return n ? n->depth : 0;
}

uint32_t 版本图_直径(version_graph_t* vg) {
    uint32_t max_d = 0;
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].depth > max_d) max_d = vg->nodes[i].depth;
    }
    return max_d;
}

void 版本图_拓扑排序(version_graph_t* vg, uint32_t* out_order, 
                      uint32_t max_count) {
    uint32_t count = vg->node_count;
    if (count > max_count) count = max_count;
    
    for (uint32_t i = 0; i < count; i++) out_order[i] = i;
    
    for (uint32_t i = 0; i < count - 1; i++) {
        for (uint32_t j = i + 1; j < count; j++) {
            if (vg->nodes[out_order[j]].version < vg->nodes[out_order[i]].version) {
                uint32_t tmp = out_order[i];
                out_order[i] = out_order[j];
                out_order[j] = tmp;
            }
        }
    }
}

/* ═══════════════════════════════════════════════════
 * 可视化 — 迭代 DFS 打印版本树
 * ═══════════════════════════════════════════════════ */

typedef struct { uint32_t ver; int depth; } stack_item_t;

void 版本图_打印树(version_graph_t* vg, uint32_t root_version, int max_depth) {
    if (max_depth <= 0) max_depth = 20;
    
    stack_item_t stack[512];
    int sp = 0;
    
    stack[sp].ver = root_version;
    stack[sp].depth = 0;
    sp++;
    
    while (sp > 0) {
        sp--;
        uint32_t ver = stack[sp].ver;
        int depth = stack[sp].depth;
        
        if (depth > max_depth) continue;
        
        vg_node_t* n = 版本图_寻节点(vg, ver);
        if (!n) continue;
        
        /* 缩进 */
        for (int i = 0; i < depth; i++) printf("  ");
        
        /* 类型标记 */
        const char* type_mark = "·";
        switch (n->type) {
            case VG_NODE_ROOT:     type_mark = "★"; break;
            case VG_NODE_LINEAR:   type_mark = "·"; break;
            case VG_NODE_BRANCH:   type_mark = "⇉"; break;
            case VG_NODE_MERGE:    type_mark = "⊕"; break;
            case VG_NODE_LEAF:     type_mark = "◆"; break;
            case VG_NODE_ISOLATED: type_mark = "?"; break;
        }
        
        /* 层级名称 */
        const char* level_name = "?";
        switch (n->level) {
            case 0: level_name = "元"; break;
            case 1: level_name = "阶"; break;
            case 2: level_name = "层"; break;
            case 3: level_name = "代"; break;
            case 4: level_name = "世"; break;
            case 5: level_name = "界"; break;
            case 6: level_name = "域"; break;
            case 7: level_name = "场"; break;
        }
        
        printf("%s [v%u] D=%u %s\n", type_mark, ver, n->depth, level_name);
        
        /* 压栈子节点 (逆序，保证正序输出) */
        for (int i = (int)n->child_count - 1; i >= 0; i--) {
            if (sp < 512) {
                stack[sp].ver = n->children[i];
                stack[sp].depth = depth + 1;
                sp++;
            }
        }
    }
}

/* ═══════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════ */

void 版本图_报告(version_graph_t* vg) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           版 本 图 · 结 构                      ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 节点数: %-6u  边数: %-6u                     ║\n",
           vg->node_count, vg->edge_count);
    printf("║ 分叉点: %-6u  合并点: %-6u                 ║\n",
           vg->branch_count, vg->merge_count);
    printf("║ 图直径: %-6u (最长路径深度)                 ║\n",
           版本图_直径(vg));
    printf("╚══════════════════════════════════════════════════╝\n");
    
    uint32_t type_counts[6] = {0};
    for (uint32_t i = 0; i < vg->node_count; i++) {
        if (vg->nodes[i].type < 6) type_counts[vg->nodes[i].type]++;
    }
    
    printf("\n  节点类型分布:\n");
    if (type_counts[VG_NODE_ROOT] > 0)
        printf("    ★ 根节点: %u\n", type_counts[VG_NODE_ROOT]);
    if (type_counts[VG_NODE_LINEAR] > 0)
        printf("    · 线性: %u\n", type_counts[VG_NODE_LINEAR]);
    if (type_counts[VG_NODE_BRANCH] > 0)
        printf("    ⇉ 分叉: %u\n", type_counts[VG_NODE_BRANCH]);
    if (type_counts[VG_NODE_MERGE] > 0)
        printf("    ⊕ 合并: %u\n", type_counts[VG_NODE_MERGE]);
    if (type_counts[VG_NODE_LEAF] > 0)
        printf("    ◆ 叶: %u\n", type_counts[VG_NODE_LEAF]);
    
    if (vg->node_count > 0) {
        printf("\n  版本树预览 (前5层):\n");
        版本图_打印树(vg, 1, 5);
    }
    printf("\n");
}
