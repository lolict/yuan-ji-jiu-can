/**
 * dimension_space.h — 多维几何容器
 *
 * 核心: 多个子容器之间的维度生长和跨容器遍历
 *
 * 维度结构:
 *   0维 = 起点 (公约数本身, 静态点)
 *   1维 = 线 (单个容器内的遍历, 如48个声母排成一线)
 *   2维 = 面 (两个容器交叉, 声母×韵母 = 48×41 的平面)
 *   3维 = 体 (声母×韵母×声调 = 48×41×4 的空间)
 *
 * 边界 = 各维度尺寸的乘积 (量化统计的所有地址)
 * 折叠 = 一维用完不进位, 折起形成新层 (递归折叠)
 *
 * 跨容器关系:
 *   从声母容器取一个元素 → 遍历韵母容器所有元素 → 形成一条遍历线
 *   这条线就是"关系"的物理形态
 *   遍历完韵母容器后 → 折叠 → 取下一个声母 → 再遍历
 *
 * 产权: 本容器统治多个子容器之间的交互规则
 */

#ifndef NJSG_DIMENSION_SPACE_H
#define NJSG_DIMENSION_SPACE_H

#include <stdint.h>
#include <stdio.h>

#define DIMENSION_MAGIC     0x44494D53  /* "DIMS" */
#define MAX_SHENGMU         48
#define MAX_YUNMU           41   /* 6基元 + 35复合 */
#define MAX_TONE            4
#define MAX_DIMENSIONS      3

/* 维度枚举 */
typedef enum {
    DIM_SHENGMU = 0,    /* 声母维 */
    DIM_YUNMU   = 1,    /* 韵母维 */
    DIM_TONE    = 2,    /* 声调维 */
} dimension_t;

/* 子容器声明 */
typedef struct {
    const char *name;           /* 容器名称 */
    uint16_t   size;            /* 容器内元素数量 */
    uint16_t   capacity;        /* 容器容量上限 */
    const char **labels;        /* 元素名称表 (数组指针) */
} sub_container_t;

/* 交叉点: 一个具体的 (声母,韵母,声调) 组合 */
typedef struct {
    uint16_t    shengmu_idx;
    uint16_t    yunmu_idx;
    uint8_t     tone;
    uint32_t    linear_addr;    /* 一维线性化地址 */
    uint8_t     valid;          /* 是否有效组合 */
    uint16_t    fold_level;     /* 折叠层级 */
} cross_point_t;

/* 遍历路径: 一次跨容器遍历的记录 */
typedef struct {
    uint16_t    anchor_idx;     /* 锚点容器中的索引 (如声母b=1) */
    uint16_t    traverse_count; /* 遍历了多少个目标容器元素 */
    uint32_t    path_length;    /* 路径总长度 (bit) */
    uint8_t     dimension;      /* 在哪一维上遍历 */
} traverse_path_t;

/* 多维几何容器 */
typedef struct {
    uint32_t        magic;
    uint16_t        version;
    uint8_t         dim_count;      /* 当前维度数 (1-3) */
    
    /* 三个子容器 */
    sub_container_t containers[MAX_DIMENSIONS];
    
    /* 边界 = 各维度尺寸的乘积 */
    uint32_t        border_total;   /* 总边界 (所有可计算地址) */
    uint32_t        border_valid;   /* 有效地址 (实际存在的组合) */
    
    /* 交叉点表 (二维平面展开) */
    cross_point_t   cross[MAX_SHENGMU][MAX_YUNMU];
    uint16_t        cross_count;
    
    /* 遍历路径记录 */
    traverse_path_t paths[MAX_SHENGMU];
    uint16_t        path_count;
    
    /* 折叠状态 */
    uint8_t         fold_state;     /* 当前折叠状态 */
    uint32_t        fold_count;     /* 已完成的折叠次数 */
} dimension_space_t;

/* ── API ───────────────────────────────────────────────────── */

void    dimension_init(dimension_space_t *ds);
int     dimension_register_container(dimension_space_t *ds, dimension_t dim,
                                      const char *name, uint16_t capacity);
int     dimension_set_elements(dimension_space_t *ds, dimension_t dim,
                                const char **labels, uint16_t count);
int     dimension_compute_border(dimension_space_t *ds);
int     dimension_traverse(dimension_space_t *ds, dimension_t anchor_dim,
                            uint16_t anchor_idx, dimension_t target_dim);
int     dimension_fold(dimension_space_t *ds);
uint32_t dimension_linear_addr(const dimension_space_t *ds,
                                 uint16_t sm, uint16_t ym, uint8_t tone);
void    dimension_stats(const dimension_space_t *ds, FILE *out);

#endif /* NJSG_DIMENSION_SPACE_H */
