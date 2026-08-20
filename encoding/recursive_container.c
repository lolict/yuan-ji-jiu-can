/**
 * recursive_container.c — 递归容器实现
 *
 * 一次注册 · 无限递归 · 自动授权 · 幂次对齐
 */

#include "recursive_container.h"
#include <string.h>
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void recur_init(container_group_t *group, recursive_container_t *pool, uint32_t pool_size) {
    memset(group, 0, sizeof(*group));
    group->magic         = RECURSIVE_MAGIC;
    group->containers    = pool;
    group->total_capacity = pool_size;
    group->wall_l1       = WALL_L1;
    group->wall_l2       = WALL_L2;
    group->wall_l3       = WALL_L3;
    group->max_tier      = 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 创建士兵容器 (一次层级: 2字符)
 * ═══════════════════════════════════════════════════════════════ */

recursive_container_t *recur_create_soldier(container_group_t *group,
                                            uint32_t char1, uint32_t char2) {
    if (!group || group->magic != RECURSIVE_MAGIC) return 0;
    if (group->total_count >= group->total_capacity) return 0;

    recursive_container_t *c = &group->containers[group->total_count];
    memset(c, 0, sizeof(*c));

    c->id           = group->next_id++;
    c->tier         = 1;
    c->rank         = RANK_SOLDIER;
    c->bits         = 1;
    c->chars[0]     = char1;
    c->chars[1]     = char2;
    c->char_count   = 2;
    c->wall_symbol  = WALL_L1;
    c->is_start     = 1;  /* 墙也是开始 */
    c->is_end       = 1;  /* 墙也是结束 */
    c->junction     = 1;  /* 自己就是墙点 */

    group->total_count++;
    group->total_chars += 2;
    group->total_bits  += 1;

    return c;
}

/* ═══════════════════════════════════════════════════════════════
 * 二次层级密封 (长官折叠士兵)
 *
 * 幂次对齐: 把n个士兵的bit数对齐到2的幂
 * 折叠后: 所有士兵被封装进一个长官 · 只占用1bit
 * ═══════════════════════════════════════════════════════════════ */

recursive_container_t *recur_seal_L2(container_group_t *group,
                                     recursive_container_t **soldiers, uint16_t count) {
    if (!group || !soldiers || count == 0) return 0;

    /* 创建长官容器 */
    if (group->total_count >= group->total_capacity) return 0;

    recursive_container_t *officer = &group->containers[group->total_count];
    memset(officer, 0, sizeof(*officer));

    officer->id          = group->next_id++;
    officer->tier        = 2;
    officer->rank        = RANK_OFFICER;
    officer->bits        = 1;  /* 高等级也只占1bit */
    officer->wall_symbol = WALL_L2;
    officer->is_start    = 1;
    officer->is_end      = 1;
    officer->junction    = 1;

    /* 幂次对齐: 把士兵数量对齐到最近的2的幂 */
    uint16_t pow2 = 1;
    while (pow2 < count && pow2 < 256) pow2 *= 2;
    officer->child_capacity = pow2;
    officer->child_count    = count;

    /* 士兵被密封在这里 */
    officer->children = soldiers[0];
    for (uint16_t i = 0; i < count; i++) {
        soldiers[i]->sealed = 1;  /* 密封! 不再独立修改 */
        /* 设置邻居关系: 左=旧 · 右=新 */
        if (i > 0)   soldiers[i]->left   = soldiers[i-1];
        if (i < count-1) soldiers[i]->right = soldiers[i+1];
    }

    group->total_count++;
    group->total_bits += 1;
    if (group->max_tier < 2) group->max_tier = 2;

    return officer;
}

/* ═══════════════════════════════════════════════════════════════
 * 递归密封自动版
 *
 * 自动处理层级:
 *   · 输入全是一次 → 创建二次
 *   · 输入全是二次 → 创建三次
 *   · 输入全是三次 → 创建四次 (皇后)
 *   · 超过四次 → 公约数算法 · 递归授权
 * ═══════════════════════════════════════════════════════════════ */

recursive_container_t *recur_seal_auto(container_group_t *group,
                                       recursive_container_t **sources, uint16_t count) {
    if (!group || !sources || count == 0) return 0;

    /* 判断输入的最高层级 */
    uint8_t max_input_tier = 0;
    for (uint16_t i = 0; i < count; i++) {
        if (sources[i]->tier > max_input_tier) max_input_tier = sources[i]->tier;
    }

    if (group->total_count >= group->total_capacity) return 0;

    /* 创建高一级容器 */
    recursive_container_t *higher = &group->containers[group->total_count];
    memset(higher, 0, sizeof(*higher));

    higher->id          = group->next_id++;
    higher->tier        = (uint8_t)(max_input_tier + 1);
    higher->bits        = 1;
    higher->is_start    = 1;
    higher->is_end      = 1;
    higher->junction    = 1;

    /* 根据层级设置官员等级和墙符号 */
    switch (higher->tier) {
        case 2: higher->rank = RANK_OFFICER;   higher->wall_symbol = WALL_L2; break;
        case 3: higher->rank = RANK_CONCUBINE; higher->wall_symbol = WALL_L3; break;
        case 4: higher->rank = RANK_EMPRESS;   higher->wall_symbol = WALL_L4; break;
        default:
            /* 超过四次: 公约数算法 · 自动授权 */
            higher->rank = (container_rank_t)(higher->tier < 255 ? higher->tier : 255);
            higher->wall_symbol = (uint8_t)('0' + (higher->tier % 10));
            break;
    }

    /* 幂次对齐 */
    uint16_t pow2 = 1;
    while (pow2 < count && pow2 < 256) pow2 *= 2;
    higher->child_capacity = pow2;
    higher->child_count    = count;
    higher->children = sources[0];

    /* 密封下级 */
    for (uint16_t i = 0; i < count; i++) {
        sources[i]->sealed = 1;
        if (i > 0)   sources[i]->left  = sources[i-1];
        if (i < count-1) sources[i]->right = sources[i+1];
    }

    group->total_count++;
    group->total_bits += 1;
    if (group->max_tier < higher->tier) group->max_tier = higher->tier;

    return higher;
}

/* ═══════════════════════════════════════════════════════════════
 * 注册规则表头
 *
 * 一次注册 · 所有新容器自动加载规则
 * ═══════════════════════════════════════════════════════════════ */

int recur_register_rule(container_group_t *group, uint16_t template_id) {
    if (!group || group->magic != RECURSIVE_MAGIC) return -1;

    group->rule_header_id = template_id;

    /* 所有现存容器自动授权 */
    for (uint32_t i = 0; i < group->total_count; i++) {
        group->containers[i].rule_template_id = template_id;
        group->containers[i].auto_rendered    = 1;
    }

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 三元裁判
 *
 * 左(旧容器) + 墙(点) + 右(新容器) = 三元
 * 准入条件: 左右墙符号兼容且层级相邻
 * ═══════════════════════════════════════════════════════════════ */

uint8_t recur_junction_judge(recursive_container_t *left,
                              recursive_container_t *wall,
                              recursive_container_t *right) {
    if (!wall) return 0;

    /* 墙必须同时是开始和结束 */
    if (!wall->is_start || !wall->is_end) return 0;

    /* 如果没有左右邻居 · 只判断墙自身 */
    if (!left && !right) return wall->junction;

    /* 左容器存在时: 左的墙符号必须是低层级 */
    if (left && left->tier >= wall->tier) return 0;

    /* 右容器存在时: 右的墙符号必须是同级或低层级 */
    if (right && right->tier > wall->tier) return 0;

    return 1;  /* 准入 */
}

/* ═══════════════════════════════════════════════════════════════
 * 距离消除
 *
 * L1: 时间融合 = 结束=开始 · 消除先后 · 只是相位不同
 * L2: 空间融合 = 密封 · 消除隔离 · 容器嵌套
 * L3: 物质融合 = 合一 · 消除量化碎片 · 物质=空间=时间=一个点
 * ═══════════════════════════════════════════════════════════════ */

void recur_merge_distance(container_group_t *group, recursive_container_t *c, uint8_t level) {
    if (!group || !c) return;

    switch (level) {
        case 1:  /* L1: 时间融合 */
            c->time_merged = 1;
            /* 结束=开始生效: 同一个墙符号 · 左结束=右开始 */
            c->is_start = 1;
            c->is_end   = 1;
            break;

        case 2:  /* L2: 空间融合 */
            c->space_merged = 1;
            /* 密封所有下级容器 */
            if (c->children) {
                for (uint16_t i = 0; i < c->child_count; i++) {
                    if (&c->children[i]) {
                        c->children[i].sealed = 1;
                        c->children[i].space_merged = 1;
                    }
                }
            }
            break;

        case 3:  /* L3: 物质融合 */
            c->matter_merged = 1;
            /* 物质=空间=时间=一个点 */
            c->time_merged  = 1;
            c->space_merged = 1;
            c->junction     = 1;  /* 就是那个点 */
            group->merge_count++;
            break;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 自动渲染
 *
 * 新容器自动加载已注册的规则 · 不需要重新定义算法
 * ═══════════════════════════════════════════════════════════════ */

void recur_auto_render(container_group_t *group, recursive_container_t *c) {
    if (!group || !c) return;

    /* 自动继承表头规则 */
    if (group->rule_header_id != 0) {
        c->rule_template_id = group->rule_header_id;
        c->auto_rendered    = 1;
    }

    /* 根据层级自动确定墙符号 */
    switch (c->tier) {
        case 1: c->wall_symbol = group->wall_l1; break;
        case 2: c->wall_symbol = group->wall_l2; break;
        case 3: c->wall_symbol = group->wall_l3; break;
        default:
            /* N层级: 公约数分配墙符号 */
            if (c->tier < 10)
                c->wall_symbol = (uint8_t)('0' + c->tier);
            else
                c->wall_symbol = (uint8_t)('A' + (c->tier % 26));
            break;
    }

    /* 递归渲染: 自动授权给下级 */
    if (c->children && c->child_count > 0) {
        for (uint16_t i = 0; i < c->child_count; i++) {
            recur_auto_render(group, &c->children[i]);
        }
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

uint32_t recur_stats(container_group_t *group, const char *which) {
    if (!group) return 0;
    if (which[0] == 'c') return group->total_count;     /* containers */
    if (which[0] == 'w') return group->total_chars;     /* words */
    if (which[0] == 'b') return group->total_bits;      /* bits */
    if (which[0] == 't') return group->max_tier;        /* max tier */
    if (which[0] == 'j') return group->junction_count;  /* junctions */
    if (which[0] == 'm') return group->merge_count;     /* merges */
    return 0;
}
