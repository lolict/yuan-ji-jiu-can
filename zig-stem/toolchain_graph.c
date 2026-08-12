// toolchain_graph.c — 工具链关联图实现
// 参见 toolchain_graph.h 的架构说明
//
// 工具链图核心:
//   - 有向图管理工具依赖
//   - 基于吉凶+归属权+依赖满足度的综合决策
//   - 拓扑排序确定执行顺序
//   - 关键路径分析

#include "toolchain_graph.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

static uint64_t time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// ── 创建/销毁 ──
toolchain_graph_t* tcg_create(arbiter_engine_t* arbiter) {
    toolchain_graph_t* g = (toolchain_graph_t*)calloc(1, sizeof(toolchain_graph_t));
    if (!g) return NULL;
    g->node_count = 0;
    g->edge_count = 0;
    g->is_compiled = false;
    g->arbiter = arbiter;
    return g;
}

void tcg_destroy(toolchain_graph_t* g) {
    if (g) free(g);
}

// ── 添加节点 ──
uint32_t tcg_add_node(toolchain_graph_t* g, const char* name, const char* desc,
                      owner_type_t owner) {
    if (!g || g->node_count >= TCG_MAX_NODES) return 0;

    uint32_t id = g->node_count++;
    tcg_node_t* n = &g->nodes[id];
    n->id = id;
    strncpy(n->name, name, TCG_MAX_NAME - 1);
    n->name[TCG_MAX_NAME - 1] = '\0';
    if (desc) {
        strncpy(n->description, desc, sizeof(n->description) - 1);
        n->description[sizeof(n->description) - 1] = '\0';
    }
    n->state = TOOL_READY;
    n->fortune = FORTUNE_NEUTRAL;
    n->owner = owner;
    n->version = 1;
    n->register_time = time_now_ms();
    n->last_exec_time = 0;
    n->health = 1.0f;
    n->exec_count = 0;

    return id;
}

// ── 添加边 ──
bool tcg_add_edge(toolchain_graph_t* g, uint32_t from, uint32_t to, bool hard) {
    if (!g || from >= g->node_count || to >= g->node_count) return false;
    if (g->edge_count >= TCG_MAX_EDGES) return false;

    tcg_edge_t* e = &g->edges[g->edge_count++];
    e->from = from;
    e->to = to;
    e->is_hard = hard;
    e->weight = hard ? 1.0f : 0.5f;
    g->is_compiled = false;  // 图变更,需重新编译
    return true;
}

// ── 更新状态 ──
bool tcg_set_state(toolchain_graph_t* g, uint32_t node_id, tool_state_t state) {
    if (!g || node_id >= g->node_count) return false;
    g->nodes[node_id].state = state;
    return true;
}

// ── 设置吉凶 ──
bool tcg_set_fortune(toolchain_graph_t* g, uint32_t node_id, fortune_type_t f) {
    if (!g || node_id >= g->node_count) return false;
    g->nodes[node_id].fortune = f;
    return true;
}

// ── 综合决策 ──
tcg_decision_t tcg_decide(toolchain_graph_t* g, uint32_t node_id) {
    tcg_decision_t dec = {0};
    dec.tool_id = node_id;
    dec.can_execute = true;
    dec.confidence = 1.0f;
    dec.fortune_score = 0.0f;
    dec.blocking_count = 0;
    dec.reason[0] = '\0';

    if (!g || node_id >= g->node_count) {
        dec.can_execute = false;
        dec.confidence = 0.0f;
        snprintf(dec.reason, sizeof(dec.reason), "节点不存在");
        return dec;
    }

    tcg_node_t* node = &g->nodes[node_id];

    // 条件1: 工具状态检查
    if (node->state == TOOL_UNBORN || node->state == TOOL_FAILED) {
        dec.can_execute = false;
        dec.confidence = 0.0f;
        snprintf(dec.reason, sizeof(dec.reason), "工具状态异常: %d", node->state);
        return dec;
    }
    if (node->state == TOOL_COMPLETED) {
        dec.can_execute = false;
        dec.confidence = 0.0f;
        snprintf(dec.reason, sizeof(dec.reason), "工具已完成");
        return dec;
    }

    // 条件2: 归属权检查
    if (node->owner == OWNER_EXTERNAL) {
        dec.can_execute = false;
        dec.confidence = 0.0f;
        snprintf(dec.reason, sizeof(dec.reason), "工具未归属(外部)");
        return dec;
    }

    // 条件3: 健康度
    if (node->health < 0.3f) {
        dec.can_execute = false;
        dec.confidence = 0.0f;
        snprintf(dec.reason, sizeof(dec.reason), "工具健康度不足: %.0f%%", node->health * 100);
        return dec;
    }

    // 条件4: 遍历所有边,检查依赖
    float dep_score = 0.0f;
    float dep_total = 0.0f;

    for (uint32_t i = 0; i < g->edge_count; i++) {
        if (g->edges[i].from == node_id) {
            uint32_t dep_id = g->edges[i].to;
            dep_total += g->edges[i].weight;

            tcg_node_t* dep = &g->nodes[dep_id];

            // 被依赖者状态检查
            if (dep->state != TOOL_COMPLETED && dep->state != TOOL_FORTUNED) {
                if (g->edges[i].is_hard) {
                    // 硬依赖未满足 → 阻塞
                    if (dec.blocking_count < 8) {
                        dec.blocking_deps[dec.blocking_count++] = dep_id;
                    }
                }
            } else {
                // 依赖满足
                dep_score += g->edges[i].weight;
            }

            // 被依赖者的吉凶影响
            if (dep->fortune == FORTUNE_MISFORTUNE || dep->fortune == FORTUNE_INAUSPICIOUS) {
                if (g->edges[i].is_hard) {
                    // 硬依赖有凶 → 阻塞
                    if (dec.blocking_count < 8) {
                        dec.blocking_deps[dec.blocking_count++] = dep_id;
                    }
                }
            }
        }
    }

    // 判断阻塞
    if (dec.blocking_count > 0) {
        dec.can_execute = false;
        dec.confidence = dep_total > 0 ? dep_score / dep_total : 0.0f;
        snprintf(dec.reason, sizeof(dec.reason),
            "%u个硬依赖未满足", dec.blocking_count);
        return dec;
    }

    // 计算吉凶评分
    switch (node->fortune) {
        case FORTUNE_BLESSING:
        case FORTUNE_AUSPICIOUS:
            dec.fortune_score = 0.8f; break;
        case FORTUNE_NEUTRAL:
            dec.fortune_score = 0.0f; break;
        case FORTUNE_INAUSPICIOUS:
            dec.fortune_score = -0.5f; break;
        case FORTUNE_MISFORTUNE:
            dec.fortune_score = -1.0f; break;
        default:
            dec.fortune_score = 0.0f; break;
    }

    // 综合信心度 = 依赖满足度 × 吉凶因子 × 健康度
    float dep_factor = dep_total > 0 ? dep_score / dep_total : 1.0f;
    float fortune_factor = (dec.fortune_score + 1.0f) / 2.0f;  // 归一化到 0-1
    dec.confidence = dep_factor * fortune_factor * node->health;

    snprintf(dec.reason, sizeof(dec.reason),
        "依赖满足:%.0f%% 吉凶:%.0f%% 健康:%.0f%%",
        dep_factor * 100, fortune_factor * 100, node->health * 100);

    return dec;
}

// ── 拓扑排序 (Kahn算法) ──
// 语义: edge.from 依赖于 edge.to (to 是 prerequisite)
//   in_degree[x] = x 的未满足依赖数
//   order[] = 执行顺序 (先执行的在前)
uint32_t tcg_topological_order(toolchain_graph_t* g, uint32_t* order, uint32_t max) {
    if (!g || !order || max == 0) return 0;

    // 计算入度: 每个节点的依赖数量
    uint32_t in_degree[TCG_MAX_NODES] = {0};
    for (uint32_t i = 0; i < g->edge_count; i++) {
        in_degree[g->edges[i].from]++;  // from 依赖于 to
    }

    // BFS
    uint32_t queue[TCG_MAX_NODES];
    uint32_t q_head = 0, q_tail = 0;

    for (uint32_t i = 0; i < g->node_count; i++) {
        if (in_degree[i] == 0) {
            queue[q_tail++] = i;
        }
    }

    uint32_t count = 0;
    while (q_head < q_tail && count < max) {
        uint32_t cur = queue[q_head++];
        order[count++] = cur;

        // cur 完成 → 释放依赖于 cur 的节点
        for (uint32_t i = 0; i < g->edge_count; i++) {
            if (g->edges[i].to == cur) {
                uint32_t dep = g->edges[i].from;  // dep 依赖于 cur
                in_degree[dep]--;
                if (in_degree[dep] == 0) {
                    queue[q_tail++] = dep;
                }
            }
        }
    }

    g->is_compiled = (count == g->node_count);
    return count;
}

// ── 模拟执行 ──
uint32_t tcg_simulate_execute(toolchain_graph_t* g) {
    if (!g) return 0;

    uint32_t order[TCG_MAX_NODES];
    uint32_t count = tcg_topological_order(g, order, TCG_MAX_NODES);
    uint32_t executed = 0;

    for (uint32_t i = 0; i < count; i++) {
        tcg_decision_t dec = tcg_decide(g, order[i]);
        if (dec.can_execute) {
            g->nodes[order[i]].state = TOOL_COMPLETED;
            g->nodes[order[i]].exec_count++;
            g->nodes[order[i]].last_exec_time = time_now_ms();
            executed++;
        }
    }

    return executed;
}

// ── 关键路径 (简化: 出度最大的路径) ──
uint32_t tcg_critical_path(const toolchain_graph_t* g, uint32_t* path, uint32_t max) {
    if (!g || !path || max == 0) return 0;

    // 计算每个节点的出度
    uint32_t out_degree[TCG_MAX_NODES] = {0};
    for (uint32_t i = 0; i < g->edge_count; i++) {
        out_degree[g->edges[i].from]++;
    }

    // 找入度为0的起点
    uint32_t in_degree[TCG_MAX_NODES] = {0};
    for (uint32_t i = 0; i < g->edge_count; i++) {
        in_degree[g->edges[i].to]++;
    }

    // 找最长路径 (贪心)
    uint32_t start = 0;
    for (uint32_t i = 0; i < g->node_count; i++) {
        if (in_degree[i] == 0 && out_degree[i] > out_degree[start]) {
            start = i;
        }
    }

    // 追踪
    uint32_t count = 0;
    uint32_t cur = start;
    bool visited[TCG_MAX_NODES] = {0};

    while (count < max && cur < g->node_count && !visited[cur]) {
        path[count++] = cur;
        visited[cur] = true;

        // 找出度最大的邻居
        uint32_t best_next = cur;
        uint32_t best_deg = 0;
        for (uint32_t i = 0; i < g->edge_count; i++) {
            if (g->edges[i].from == cur && !visited[g->edges[i].to]) {
                if (out_degree[g->edges[i].to] > best_deg) {
                    best_deg = out_degree[g->edges[i].to];
                    best_next = g->edges[i].to;
                }
            }
        }
        if (best_next == cur) break;  // 没有未访问的邻居
        cur = best_next;
    }

    return count;
}

// ── 可视化 ──
void tcg_visualize(const toolchain_graph_t* g) {
    if (!g) return;

    const char* state_icons[] = {"✗", "○", "►", "✓", "✗", "☯"};
    const char* fortune_icons[] = {"◇", "◆", "▽", "✗", "★"};

    printf("\n  工具链关联图 (%u 节点, %u 边):\n", g->node_count, g->edge_count);
    printf("  ──────────────────────────────────\n");

    for (uint32_t i = 0; i < g->node_count; i++) {
        const tcg_node_t* n = &g->nodes[i];
        const char* st_icon = (n->state < 6) ? state_icons[n->state] : "?";
        const char* fr_icon = (n->fortune < 5) ? fortune_icons[n->fortune] : "?";
        printf("  %s %s [%u] %s (v%u) %s 健康:%.0f%%\n",
            fr_icon, st_icon, n->id, n->name, n->version,
            n->description, n->health * 100);

        // 显示依赖
        for (uint32_t j = 0; j < g->edge_count; j++) {
            if (g->edges[j].from == i) {
                const char* dep_name = g->nodes[g->edges[j].to].name;
                printf("      ↓ %s%s\n",
                    g->edges[j].is_hard ? "[硬]" : "[软]", dep_name);
            }
        }
    }
    printf("  ──────────────────────────────────\n");
}

// ── 报表 ──
void tcg_report(const toolchain_graph_t* g) {
    if (!g) return;

    const char* state_names[] = {"未诞生", "就绪", "运行中", "完成", "失败", "已判定"};
    const char* fortune_names[] = {"平", "吉", "凶", "祸", "福"};
    const char* owner_names[] = {"主体", "妻子", "分身", "外部", "竞争者"};

    printf("\n═══ 工具链关联图 ═══\n");
    printf("  节点数:     %u\n", g->node_count);
    printf("  边数:       %u\n", g->edge_count);
    printf("  拓扑完成:   %s\n", g->is_compiled ? "是" : "否");
    printf("  ─────────────────────────────\n");

    uint32_t state_count[6] = {0};
    uint32_t fortune_count[5] = {0};
    for (uint32_t i = 0; i < g->node_count; i++) {
        if (g->nodes[i].state < 6) state_count[g->nodes[i].state]++;
        if (g->nodes[i].fortune < 5) fortune_count[g->nodes[i].fortune]++;
    }

    printf("  状态: ");
    for (int i = 0; i < 6; i++) {
        if (state_count[i] > 0) printf("%s:%u ", state_names[i], state_count[i]);
    }
    printf("\n");

    printf("  吉凶: ");
    for (int i = 0; i < 5; i++) {
        if (fortune_count[i] > 0) printf("%s:%u ", fortune_names[i], fortune_count[i]);
    }
    printf("\n  ─────────────────────────────\n");

    // 每个工具
    for (uint32_t i = 0; i < g->node_count; i++) {
        const tcg_node_t* n = &g->nodes[i];
        printf("  [%-2u] %-12s  %s %s %s  执行:%u次\n",
            n->id, n->name,
            state_names[n->state],
            fortune_names[n->fortune],
            owner_names[n->owner],
            n->exec_count);
    }
    printf("═══════════════════════════════════\n\n");
}
