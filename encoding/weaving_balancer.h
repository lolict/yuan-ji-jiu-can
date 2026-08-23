/**
 * weaving_balancer.h — 编织平衡器
 *
 * 原理: 所有配对关系是一张编织图
 *   节点 = 符号
 *   边 = 配对关系
 *   力 = 两个符号之间的"压/被压"张力
 *
 * 核心问题: 张力的分布是否均匀?
 *   ┌────────────────────────────────────────────────────────────────┐
 *   │ 均匀分布式: 每个节点承受的力 ≈ 施加的力 → 平衡 → 可编织      │
 *   │ 不均匀: 某个节点被过度压制 → 编织崩断 → 不可配对              │
 *   └────────────────────────────────────────────────────────────────┘
 *
 * 等效镜像对齐:
 *   只有力平衡的节点才能对齐 → 才能形成公倍数 → 才能服务于同一个公约数
 *   如果力不对齐 → 镜像无法等效 → 公倍数碎裂 → 目标偏离
 *
 * 计算:
 *   每个节点的 net_force = 施加的力总和 - 承受的力总和
 *   编织平衡度 = 1.0 - (所有节点的|net_force|总和 / 总边数)
 *   平衡度 > 0.7 = 合格编织
 */

#ifndef NJSG_WEAVING_BALANCER_H
#define NJSG_WEAVING_BALANCER_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 最大规模
 * ═══════════════════════════════════════════════════════════════ */

#define MAX_NODES 64
#define MAX_EDGES 128

/* ═══════════════════════════════════════════════════════════════
 * 节点 (编织中的一个符号)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    const char    *name;           /* 节点名称 */
    float          weight;         /* 节点自身重量 (视觉重量) */
    float          pressure_out;   /* 施加给别人的力总和 */
    float          pressure_in;    /* 承受的力总和 */
    float          net_force;      /* 净力 = out - in */
    uint8_t        balanced;       /* 是否达到力平衡 (|net| < 阈值) */
    uint32_t       edge_count;     /* 连接的边数 */
} weaving_node_t;

/* ═══════════════════════════════════════════════════════════════
 * 边 (两个符号之间的配对关系)
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t       from;           /* 起点节点索引 */
    uint32_t       to;             /* 终点节点索引 */
    float          tension;        /* 边的张力 */
    uint8_t        active;         /* 是否有效 */
} weaving_edge_t;

/* ═══════════════════════════════════════════════════════════════
 * 编织图
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t       magic;          /* WEAVING_MAGIC */
    uint32_t       node_count;     /* 节点数 */
    uint32_t       edge_count;     /* 边数 */
    weaving_node_t nodes[MAX_NODES];
    weaving_edge_t edges[MAX_EDGES];
    
    /* 全局统计 */
    float          total_tension;  /* 总张力 */
    float          balance_score;  /* 全局平衡度 0.0~1.0 */
    uint8_t        is_balanced;    /* 全局是否平衡 */
    uint32_t       balanced_nodes; /* 达到力平衡的节点数 */
    uint32_t       stressed_nodes; /* 受力过多的节点数 */
} weaving_graph_t;

#define WEAVING_MAGIC 0x57454156  /* "WEAV" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化编织图
 */
void weaving_init(weaving_graph_t *wg);

/**
 * 添加节点
 * @return 节点索引, 失败返回 -1
 */
int weaving_add_node(weaving_graph_t *wg, const char *name, float weight);

/**
 * 查找节点索引
 */
int weaving_find_node(weaving_graph_t *wg, const char *name);

/**
 * 添加边 (配对关系)
 * @param tension 张力 (力的大小)
 * @return 0=成功, -1=失败
 */
int weaving_add_edge(weaving_graph_t *wg, const char *from, const char *to, float tension);

/**
 * 核心: 计算全编织图的力平衡
 *   计算每个节点的 net_force, 判断平衡度
 */
void weaving_compute_balance(weaving_graph_t *wg);

/**
 * 获取节点的力状态描述
 */
const char* weaving_node_status(const weaving_node_t *node);

/**
 * 诊断: 找出编织中的问题点
 * @return 问题节点数
 */
uint32_t weaving_diagnose(weaving_graph_t *wg);

#endif /* NJSG_WEAVING_BALANCER_H */
