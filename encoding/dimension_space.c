/**
 * dimension_space.c — 多维几何容器实现
 *
 * 核心操作:
 *   1. 注册子容器 (声母/韵母/声调)
 *   2. 计算边界 (各维度尺寸乘积)
 *   3. 跨容器遍历 (从一个容器取锚点, 遍历另一个容器所有元素)
 *   4. 折叠 (一维用完, 折起形成新层)
 *   5. 线性化地址 (多维坐标 → 一维地址)
 */

#include "dimension_space.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void dimension_init(dimension_space_t *ds) {
    memset(ds, 0, sizeof(*ds));
    ds->magic = DIMENSION_MAGIC;
    ds->version = 1;
    ds->dim_count = 0;
    ds->border_total = 0;
    ds->border_valid = 0;
    ds->cross_count = 0;
    ds->path_count = 0;
    ds->fold_state = 0;
    ds->fold_count = 0;
}

int dimension_register_container(dimension_space_t *ds, dimension_t dim,
                                 const char *name, uint16_t capacity) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return -1;
    if (dim >= MAX_DIMENSIONS) return -1;
    
    ds->containers[dim].name = name;
    ds->containers[dim].capacity = capacity;
    ds->containers[dim].size = 0;
    ds->containers[dim].labels = NULL;
    
    if (dim >= ds->dim_count) ds->dim_count = dim + 1;
    return 0;
}

int dimension_set_elements(dimension_space_t *ds, dimension_t dim,
                            const char **labels, uint16_t count) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return -1;
    if (dim >= ds->dim_count) return -1;
    if (count > ds->containers[dim].capacity) return -1;
    
    ds->containers[dim].labels = labels;
    ds->containers[dim].size = count;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 计算边界
 * ═══════════════════════════════════════════════════════════════ */

int dimension_compute_border(dimension_space_t *ds) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return -1;
    if (ds->dim_count == 0) return -1;
    
    uint32_t total = 1;
    for (uint8_t d = 0; d < ds->dim_count; d++) {
        total *= ds->containers[d].size;
    }
    ds->border_total = total;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 跨容器遍历
 *   从 anchor_dim 容器取第 anchor_idx 个元素作为锚点
 *   遍历 target_dim 容器的所有元素
 *   记录每一次交叉 (cross point)
 * ═══════════════════════════════════════════════════════════════ */

int dimension_traverse(dimension_space_t *ds, dimension_t anchor_dim,
                        uint16_t anchor_idx, dimension_t target_dim) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return -1;
    if (anchor_dim >= ds->dim_count || target_dim >= ds->dim_count) return -1;
    if (anchor_idx >= ds->containers[anchor_dim].size) return -1;
    
    sub_container_t *anchor = &ds->containers[anchor_dim];
    sub_container_t *target = &ds->containers[target_dim];
    
    /* 记录遍历路径 */
    if (ds->path_count >= MAX_SHENGMU) return -1;
    traverse_path_t *path = &ds->paths[ds->path_count];
    path->anchor_idx = anchor_idx;
    path->traverse_count = target->size;
    path->dimension = target_dim;
    path->path_length = 0;
    ds->path_count++;
    
    /* 生成交叉点 */
    if (anchor_dim == DIM_SHENGMU && target_dim == DIM_YUNMU) {
        for (uint16_t j = 0; j < target->size; j++) {
            if (anchor_idx < MAX_SHENGMU && j < MAX_YUNMU) {
                cross_point_t *cp = &ds->cross[anchor_idx][j];
                cp->shengmu_idx = anchor_idx;
                cp->yunmu_idx = j;
                cp->tone = 0;  /* 无声调 */
                cp->linear_addr = anchor_idx * target->size + j;
                cp->valid = 1;
                cp->fold_level = 0;
                ds->cross_count++;
                path->path_length += 10;  /* 每个交叉点约10bit */
            }
        }
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 折叠
 *   一维用完 → 不进位 → 折起形成新层
 * ═══════════════════════════════════════════════════════════════ */

int dimension_fold(dimension_space_t *ds) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return -1;
    
    ds->fold_count++;
    ds->fold_state = (ds->fold_state + 1) & 0x03;
    
    /* 给所有交叉点增加折叠层级 */
    for (uint16_t i = 0; i < MAX_SHENGMU; i++) {
        for (uint16_t j = 0; j < MAX_YUNMU; j++) {
            if (ds->cross[i][j].valid) {
                ds->cross[i][j].fold_level = ds->fold_count;
            }
        }
    }
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 线性地址 (多维 → 一维)
 * ═══════════════════════════════════════════════════════════════ */

uint32_t dimension_linear_addr(const dimension_space_t *ds,
                                uint16_t sm, uint16_t ym, uint8_t tone) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return 0;
    
    uint32_t addr = 0;
    uint32_t stride = 1;
    
    /* 从最内维到最外维累加 */
    if (ds->dim_count > DIM_TONE) {
        addr += tone * stride;
        stride *= ds->containers[DIM_TONE].size;
    }
    if (ds->dim_count > DIM_YUNMU) {
        addr += ym * stride;
        stride *= ds->containers[DIM_YUNMU].size;
    }
    if (ds->dim_count > DIM_SHENGMU) {
        addr += sm * stride;
    }
    
    return addr;
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

void dimension_stats(const dimension_space_t *ds, FILE *out) {
    if (!ds || ds->magic != DIMENSION_MAGIC) return;
    
    fprintf(out, "─── 多维几何容器 ──-\n");
    fprintf(out, "维度数: %d\n", ds->dim_count);
    
    for (uint8_t d = 0; d < ds->dim_count; d++) {
        const sub_container_t *c = &ds->containers[d];
        const char *dim_names[] = {"声母","韵母","声调"};
        fprintf(out, "  [%d] %-6s: %d / %d\n", d,
                dim_names[d], c->size, c->capacity);
    }
    
    fprintf(out, "边界总量: %u 地址\n", ds->border_total);
    fprintf(out, "交叉点数: %u\n", ds->cross_count);
    fprintf(out, "遍历路径: %u\n", ds->path_count);
    fprintf(out, "折叠次数: %u\n", ds->fold_count);
    
    if (ds->path_count > 0) {
        fprintf(out, "\n─── 遍历路径 ───\n");
        for (uint16_t i = 0; i < ds->path_count; i++) {
            const traverse_path_t *p = &ds->paths[i];
            const char *dim_names[] = {"声母","韵母","声调"};
            fprintf(out, "  路径#%d: 锚点%s[%d] → 遍历%s(%d个) → %ubit\n",
                    i, dim_names[0], p->anchor_idx,
                    dim_names[p->dimension], p->traverse_count, p->path_length);
        }
    }
}
