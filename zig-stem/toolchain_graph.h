// toolchain_graph.h — 工具链关联图 (Toolchain Dependency Graph)
// hunyuan-os 所有工具的依赖关系与吉凶决策图
//
// 工具链图管理:
//   - 每个工具是一个节点 (编译器/运行时/结界/分身/账本/裁决)
//   - 工具有吉凶状态 (吉/凶/祸/福/平)
//   - 工具有归属权 (主体/妻子/分身/外部)
//   - 依赖边: 工具A依赖工具B (B先完成,A才能运行)
//   - 执行决策: 基于依赖满足度+吉凶+归属权的综合判断

#ifndef HUNYUAN_TOOLCHAIN_GRAPH_H
#define HUNYUAN_TOOLCHAIN_GRAPH_H

#include "arbiter.h"
#include <stdint.h>
#include <stdbool.h>

#define TCG_MAX_NODES       64      // 最大工具节点数 (64进制对齐)
#define TCG_MAX_EDGES       256     // 最大依赖边数
#define TCG_MAX_NAME        64      // 工具名称长度

// ── 工具状态 ──
typedef enum {
    TOOL_UNBORN     = 0,    // 未诞生 (未注册)
    TOOL_READY      = 1,    // 就绪 (可执行)
    TOOL_RUNNING    = 2,    // 运行中
    TOOL_COMPLETED  = 3,    // 完成
    TOOL_FAILED     = 4,    // 失败
    TOOL_FORTUNED   = 5,    // 已获吉凶判定
} tool_state_t;

// ── 工具节点 ──
typedef struct {
    uint32_t        id;                         // 节点ID
    char            name[TCG_MAX_NAME];         // 工具名称
    char            description[128];           // 工具描述
    tool_state_t    state;                      // 当前状态
    fortune_type_t  fortune;                    // 当前吉凶
    owner_type_t    owner;                      // 归属权
    uint32_t        version;                    // 版本号
    uint64_t        register_time;              // 注册时间
    uint64_t        last_exec_time;             // 最后执行时间
    float           health;                     // 健康度 0-1
    uint32_t        exec_count;                 // 执行次数
} tcg_node_t;

// ── 依赖边 ──
typedef struct {
    uint32_t        from;                       // 依赖者 (A)
    uint32_t        to;                         // 被依赖者 (B)
    bool            is_hard;                    // 硬依赖 (true=必须, false=软依赖)
    float           weight;                     // 依赖权重
} tcg_edge_t;

// ── 执行决策结果 ──
typedef struct {
    uint32_t        tool_id;                    // 目标工具
    bool            can_execute;                // 是否可执行
    float           confidence;                 // 执行信心度 (0-1)
    float           fortune_score;              // 吉凶评分 (-1到+1)
    uint32_t        blocking_deps[8];           // 阻塞的依赖ID
    uint32_t        blocking_count;             // 阻塞依赖数
    char            reason[128];                // 决策原因
} tcg_decision_t;

// ── 工具链图 ──
typedef struct {
    tcg_node_t      nodes[TCG_MAX_NODES];       // 节点表
    uint32_t        node_count;                 // 节点数
    tcg_edge_t      edges[TCG_MAX_EDGES];       // 边表
    uint32_t        edge_count;                 // 边数
    bool            is_compiled;                // 图是否已编译(拓扑排序完成)
    arbiter_engine_t* arbiter;                 // 关联的裁决引擎
} toolchain_graph_t;

// ── API ──

// 创建工具链图
toolchain_graph_t* tcg_create(arbiter_engine_t* arbiter);
void tcg_destroy(toolchain_graph_t* g);

// 注册工具节点
uint32_t tcg_add_node(toolchain_graph_t* g, const char* name, const char* desc,
                      owner_type_t owner);

// 添加依赖边 (from 依赖 to)
bool tcg_add_edge(toolchain_graph_t* g, uint32_t from, uint32_t to, bool hard);

// 更新工具状态
bool tcg_set_state(toolchain_graph_t* g, uint32_t node_id, tool_state_t state);

// 设置工具吉凶
bool tcg_set_fortune(toolchain_graph_t* g, uint32_t node_id, fortune_type_t f);

// 综合决策: 某工具是否可以执行
tcg_decision_t tcg_decide(toolchain_graph_t* g, uint32_t node_id);

// 拓扑排序: 获取执行顺序
uint32_t tcg_topological_order(toolchain_graph_t* g, uint32_t* order, uint32_t max);

// 模拟执行: 按依赖顺序依次执行所有可执行工具
uint32_t tcg_simulate_execute(toolchain_graph_t* g);

// 获取关键路径 (影响最多后续工具的节点)
uint32_t tcg_critical_path(const toolchain_graph_t* g, uint32_t* path, uint32_t max);

// 可视化输出 (文本)
void tcg_visualize(const toolchain_graph_t* g);

// 报表
void tcg_report(const toolchain_graph_t* g);

#endif // HUNYUAN_TOOLCHAIN_GRAPH_H
