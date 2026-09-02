/**
 * mind_organ.h — 距离感知聚类
 *
 * 基于 WASM 内核 cell 编码的语义距离计算和聚类分析
 *
 * 核心:
 *   每个 64位 cell 在抽象空间中有一个位置 ·
 *   距离 = 位置差异的度量 ·
 *   聚类 = 距离相近的 cell 分组
 *
 * 距离维度:
 *   1. 主权距离 (sovereignty distance) — 容器空间中的远近
 *   2. 极性距离 (polarity distance) — YinYang 差异
 *   3. 见证距离 (witness distance) — 裁判视角差异
 *   4. 几何距离 (geometric distance) — 综合空间距离
 *
 * 聚类算法:
 *   DBSCAN 变体 — 基于密度 · 不需要预设簇数
 *   密度 = ε 半径内的 cell 数量
 */

#ifndef NJSG_MIND_ORGAN_H
#define NJSG_MIND_ORGAN_H

#include "wasm_kernel.h"

/* ── 配置 ────────────────────────────────────────────────── */

#define MO_MAX_CELLS      254     /* 与 WK_SYMBOL_MAX 对齐 */
#define MO_MAX_CLUSTERS   32
#define MO_DEFAULT_EPS    0x10000 /* 默认邻域半径 */
#define MO_MIN_POINTS     2       /* 最小簇大小 */

/* ── 距离度量 ────────────────────────────────────────────── */

typedef enum {
    MO_DIST_GEOMETRIC  = 0,  /* 综合几何距离 */
    MO_DIST_SOVEREIGN  = 1,  /* 主权距离 */
    MO_DIST_POLARITY   = 2,  /* 极性距离 */
    MO_DIST_WITNESS    = 3,  /* 见证距离 */
    MO_DIST_MCCP       = 4,  /* 码点距离 */
} mo_dist_type_t;

/* ── 距离矩阵 (上三角) ──────────────────────────────────── */

typedef struct {
    NJSG_uint32    count;                          /* cell 数量 */
    NJSG_uint16    ids[MO_MAX_CELLS];              /* cell 索引 */
    NJSG_float     matrix[MO_MAX_CELLS * MO_MAX_CELLS]; /* 距离矩阵 */
} mo_distmatrix_t;

/* ── 聚类结果 ────────────────────────────────────────────── */

typedef struct {
    NJSG_uint16    member_ids[MO_MAX_CELLS];      /* 簇内 cell 索引 */
    NJSG_uint16    member_count;                  /* 簇大小 */
    NJSG_uint16    centroid_id;                    /* 质心 cell 索引 */
    NJSG_float     cohesion;                       /* 内聚度 (0-1) */
} mo_cluster_t;

typedef struct {
    mo_cluster_t   clusters[MO_MAX_CLUSTERS];
    NJSG_uint16    cluster_count;
    NJSG_uint16    noise_count;                    /* 噪声点数量 */
    NJSG_float     avg_cohesion;                   /* 平均内聚度 */
    mo_dist_type_t dist_type;                      /* 使用的距离类型 */
} mo_result_t;

/* ── API ────────────────────────────────────────────────── */

/**
 * 计算两个 cell 之间的距离
 */
NJSG_EXPORT NJSG_float mo_distance(NJSG_uint64 a, NJSG_uint64 b, mo_dist_type_t type);

/**
 * 构建距离矩阵
 */
NJSG_EXPORT void mo_build_matrix(const NJSG_uint64 *cells, NJSG_uint32 count,
                                  mo_distmatrix_t *matrix, mo_dist_type_t type);

/**
 * DBSCAN 聚类
 * @param cells     cell 数组
 * @param count     cell 数量
 * @param eps       邻域半径
 * @param min_pts  最小簇大小
 * @param type     距离类型
 * @param result   聚类结果
 */
NJSG_EXPORT void mo_cluster(const NJSG_uint64 *cells, NJSG_uint32 count,
                             NJSG_float eps, NJSG_uint16 min_pts,
                             mo_dist_type_t type, mo_result_t *result);

/**
 * 查找最近邻
 * @return 最近邻的索引 · -1 表示未找到
 */
NJSG_EXPORT int  mo_nearest(const NJSG_uint64 *cells, NJSG_uint32 count,
                              NJSG_uint32 idx, mo_dist_type_t type);

/**
 * 获取聚类统计字符串
 */
NJSG_EXPORT void mo_stats(const mo_result_t *result, char *buf, NJSG_size buf_size);

#endif /* NJSG_MIND_ORGAN_H */
