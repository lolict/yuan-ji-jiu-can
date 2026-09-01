/*
 * version_graph.h — 版本图追踪
 *
 * 可视化版本间的父子关系（分叉/合并）
 * 不是线性链，而是有向无环图（DAG）
 *
 * 每个版本是一个节点
 * 每个 prev_version 是一条边
 * 分叉 = 一个版本有两个以上的子版本
 * 合并 = 一个版本引用了两个以上的父版本
 */

#ifndef NJSG_VERSION_GRAPH_H
#define NJSG_VERSION_GRAPH_H

#include <stdint.h>
#include "chain_block.h"
#include "memory_chain.h"

#define VG_MAX_NODES    2048
#define VG_MAX_EDGES    4096

/* 图节点类型 */
typedef enum {
    VG_NODE_ROOT        = 0,  /* 根节点 (version 1) */
    VG_NODE_LINEAR      = 1,  /* 线性节点 (只有一个父一个子) */
    VG_NODE_BRANCH      = 2,  /* 分叉点 (多个子节点) */
    VG_NODE_MERGE       = 3,  /* 合并点 (多个父节点) */
    VG_NODE_LEAF        = 4,  /* 叶节点 (无子节点) */
    VG_NODE_ISOLATED    = 5,  /* 孤立节点 */
} vg_node_type_t;

/* 图的边 */
typedef struct {
    uint32_t    from;       /* 父版本 */
    uint32_t    to;         /* 子版本 */
    uint8_t     weight;     /* 边的权重（关联强度） */
} vg_edge_t;

/* 图节点 */
typedef struct {
    uint32_t        version;        /* 版本号 */
    uint32_t        parent_count;   /* 父节点数 */
    uint32_t        parents[8];     /* 父版本数组 */
    uint32_t        child_count;    /* 子节点数 */
    uint32_t        children[16];   /* 子版本数组 */
    vg_node_type_t  type;           /* 节点类型 */
    uint32_t        depth;          /* 距根的距离 */
    uint8_t         level;          /* 层级（元→场） */
} vg_node_t;

/* 版本图 */
typedef struct {
    vg_node_t       nodes[VG_MAX_NODES];
    vg_edge_t       edges[VG_MAX_EDGES];
    uint32_t        node_count;
    uint32_t        edge_count;
    uint32_t        branch_count;   /* 分叉数 */
    uint32_t        merge_count;    /* 合并数 */
    
    /* 关联数据源 */
    chain_t**       chains;         /* 链数组 */
    uint32_t        chain_count;
    memory_chain_t* memory;         /* 记忆链 */
} version_graph_t;

/* 构建图 */
void    版本图_init(version_graph_t* vg);
int     版本图_添加链(version_graph_t* vg, chain_t* chain);
int     版本图_添加记忆链(version_graph_t* vg, memory_chain_t* mem);
int     版本图_构建(version_graph_t* vg);  /* 从零构建所有边 */

/* 查询 */
vg_node_t*  版本图_寻节点(version_graph_t* vg, uint32_t version);
vg_node_t*  版本图_寻分叉点(version_graph_t* vg, uint32_t after_version);
uint32_t    版本图_寻路径(version_graph_t* vg, uint32_t from, uint32_t to,
                            uint32_t* path_out, uint32_t max_len);

/* 分析 */
uint32_t    版本图_深度(version_graph_t* vg, uint32_t version);
uint32_t    版本图_直径(version_graph_t* vg);  /* 图中最长路径 */
void        版本图_拓扑排序(version_graph_t* vg, uint32_t* out_order, 
                             uint32_t max_count);

/* 可视化文本输出 */
void    版本图_打印树(version_graph_t* vg, uint32_t root_version, 
                       int max_depth);
void    版本图_报告(version_graph_t* vg);

#endif /* NJSG_VERSION_GRAPH_H */
