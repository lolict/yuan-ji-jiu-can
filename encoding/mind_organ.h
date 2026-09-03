/**
 * mind_organ.h — 感知器官 · 距离感知聚类引擎 v2
 *
 * 核心命题:
 *   「声音和形状距离最近，0和1距离最近，阴和阳距离最近」
 *
 * 从 Rust mind_organ.rs 移植到 C, 对接32位容器编码:
 *   32位编码字段 → 距离维度:
 *     family(3bit)   → d0_family    (家族层, 权重 1.0)
 *     polarity(1bit) → polarity     (极性, 权重 10.0, 阴阳翻转=大变)
 *     struct(3bit)   → sovereignty  (结构主权, 权重 5.0)
 *     shengmu(6bit)  → d1_shengmu   (声母位置, 权重 0.5)
 *     yunmu(7bit)    → d2_yunmu     (韵母位置, 权重 0.5)
 *     tone(4bit)     → witness      (声调, 权重 0.1)
 *
 * 用法:
 *   mind_organ_init(engine, threshold);
 *   mind_organ_add_cell(engine, id, container_code);
 *   int n_clusters = mind_organ_clusterize(engine);
 *   mind_organ_format(engine, buf, bufsize);
 *
 * 聂小倩匹配:
 *   int body_id = mind_organ_find_ghost_body(engine, ghost_code);
 */

#ifndef NJSG_MIND_ORGAN_H
#define NJSG_MIND_ORGAN_H

#include <stdint.h>

#define MO_MAX_CELLS        64
#define MO_MAX_CLUSTERS     16
#define MO_MAX_MEMBERS      32
#define MO_NAME_LEN         16

/* ── 距离向量 ── */
typedef struct {
    int     source_id;
    int     target_id;

    double  d_family;       /* 家族距离 */
    double  d_polarity;     /* 极性距离 (最高权重) */
    double  d_sovereignty;  /* 结构主权距离 */
    double  d_shengmu;      /* 声母位置距离 */
    double  d_yunmu;        /* 韵母位置距离 */
    double  d_tone;         /* 声调距离 */

    double  total;          /* 加权总和 */
} mo_distance_t;

/* ── 从32位容器编码提取维度 ── */
typedef struct {
    uint8_t family;         /* [31:29] 家族 0-7 */
    uint8_t polarity;       /* [28] 极性 0=YIN,1=YANG */
    uint8_t structure;      /* [27:25] 结构 0-7 */
    uint8_t shengmu;        /* [24:19] 声母 0-63 */
    uint8_t yunmu;          /* [18:12] 韵母 0-127 */
    uint8_t tone;           /* [11:8] 声调 0-15 */
} mo_dimensions_t;

/* ── 一个感知单元 ── */
typedef struct {
    int             id;             /* 单元标识 */
    uint32_t        container_code; /* 原始容器编码 */
    mo_dimensions_t dim;            /* 提取的维度 */
    int             clustered;      /* 是否已归入簇 */
} mo_cell_t;

/* ── 一个聚类(同类范畴) ── */
typedef struct {
    int             id;
    int             member_ids[MO_MAX_MEMBERS];
    int             member_count;
    double          cohesion;       /* 簇内平均紧密度(越小越紧密) */
} mo_cluster_t;

/* ── 聚类引擎 ── */
typedef struct {
    mo_cell_t       cells[MO_MAX_CELLS];
    int             cell_count;

    mo_cluster_t    clusters[MO_MAX_CLUSTERS];
    int             cluster_count;
    int             next_cluster_id;

    double          threshold;      /* 可配对的最大距离 */
} mo_engine_t;

/* ── API ── */

void    mind_organ_init(mo_engine_t *engine, double threshold);
int     mind_organ_add_cell(mo_engine_t *engine, int id, uint32_t container_code);

/* 从容器编码提取维度 */
void    mind_organ_extract_dims(uint32_t code, mo_dimensions_t *dim);

/* 计算两个容器编码之间的距离 */
double  mind_organ_distance(uint32_t code_a, uint32_t code_b);

/* 聚类 */
int     mind_organ_clusterize(mo_engine_t *engine);

/* 格式化输出 */
int     mind_organ_format(mo_engine_t *engine, char *buf, int bufsize);

/* 聂小倩: 找最近的匹配躯壳 */
int     mind_organ_find_body(mo_engine_t *engine, uint32_t ghost_code, double *dist_out);

/* 簇内紧密度 */
double  mind_organ_cluster_cohesion(const mo_cluster_t *cluster, const mo_engine_t *engine);

/* 打印报告 */
void    mind_organ_report(const mo_engine_t *engine, char *buf, int bufsize);

#endif /* NJSG_MIND_ORGAN_H */
