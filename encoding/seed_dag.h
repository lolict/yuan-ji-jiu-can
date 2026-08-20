/**
 * seed_dag.h — 有向无环图种子
 *
 * 来源: PrestoSQL query plan / Mojo MLIR graph / C++ ADL / Cargo dependency
 * 提取日期: 2026-08-19
 *
 * 数学种子: "节点+边=流向 · 无环=可拓扑排序=可编译"
 *   · DAG = 将高维关系映射到线性时间线
 *   · 节点 = 容器 (数据)
 *   · 边 = 二进制绑定 (从哪来→到哪去)
 *
 * 产权壳子 (弃):
 *   · PrestoSQL Trino engine 的分布式执行器
 *   · Mojo MLIR + LLVM codegen pipeline
 *   · Cargo semver 解析器
 *   · C++ 头文件包含树
 *
 * 我们的新编排:
 *   节点 → 容器哈希定位 (我们的五门搜索 · 非哈希)
 *   边 → 筷子夹持 (二元公倍数)
 *   排水 → 流水线 FIFO 单向 · 汇入一元时间线
 */

#ifndef NJSG_SEED_DAG_H
#define NJSG_SEED_DAG_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 有向无环图 (DAG) — 预编译流图
 *
 * 每个节点 = 一条待执行的命令
 * 边 = 数据依赖
 * 拓扑序 = 流水线时序
 * ═══════════════════════════════════════════════════════════════ */

#define DAG_MAX_NODES    1024
#define DAG_MAX_EDGES    4096
#define DAG_MAGIC        0xD46

typedef uint32_t node_id_t;

typedef struct {
    uint32_t  magic;          /* DAG_MAGIC */
    uint32_t  node_count;
    uint32_t  edge_count;
    uint32_t  next_topo;      /* 下一个入度为0的待排水节点 */
} dag_graph_t;

typedef struct {
    node_id_t id;
    uint32_t  container_id;   /* 与此节点绑定的容器 */
    uint32_t  in_degree;      /* 入度 (待输入) */
    uint32_t  out_degree;     /* 出度 (待输出) */
} dag_node_t;

typedef struct {
    node_id_t from;           /* 源节点 */
    node_id_t to;             /* 目标节点 */
    uint32_t  polarity;       /* 极性: 0=正向 · 1=镜像返回 */
} dag_edge_t;

#endif /* NJSG_SEED_DAG_H */
