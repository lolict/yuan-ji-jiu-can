/**
 * recursive_container.h — 递归容器 · 官员制 · 多层密封
 *
 * 核心:
 *   · 1容器 = 1比特 = 2字符
 *   · 结束符 = 开始符 (墙 · 三元裁判点)
 *   · 低层容器被高层容器密封 → 幂次对齐 → 折叠
 *   · 公约数容器 · 注册一次 · 无限递归复制
 *
 * 容器等级 (官员化):
 *   · 一次层级 = 士兵 · 封装2字 · 结束符=开始符
 *   · 二次层级 = 长官 · 密封一次层级 · 折叠幂次
 *   · 三次层级 = 妃子 · 递归渲染 · 自动授权
 *   · N次层级 = 公约数算法 · 注册 · 委任 · 编译
 *
 * 距离消除:
 *   · L1: 时间距离 → 结束=开始 · 消除先后
 *   · L2: 空间距离 → 密封 · 消除隔离
 *   · L3: 物质距离 → 融合 · 消除量化碎片
 */
 
#ifndef NJSG_RECURSIVE_CONTAINER_H
#define NJSG_RECURSIVE_CONTAINER_H

#include <stdint.h>
#include "container_codec.h"

/* 容器常量 */
#define CHARS_PER_BIT   2       /* 1容器=1bit=2字符 */
#define WALL_L1         0x21    /* '!' 一次层级墙 */
#define WALL_L2         0x40    /* '@' 二次层级墙 */
#define WALL_L3         0x23    /* '#' 三次层级墙 */
#define WALL_L4         0x24    /* '$' 四次层级墙 */

/* 容器身份 (官员等级) */
typedef enum {
    RANK_SOLDIER   = 1,   /* 士兵 · 一次层级 */
    RANK_OFFICER   = 2,   /* 长官 · 二次层级 */
    RANK_CONCUBINE = 3,   /* 妃子 · 三次层级 */
    RANK_EMPRESS   = 4,   /* 皇后 · 四次层级 */
} container_rank_t;

/* 副存储: 递归容器单元 */
typedef struct recursive_container {
    uint32_t          id;              /* 容器全局id */
    uint8_t           tier;            /* 层级的 N (1/2/3/4+) */
    uint8_t           rank;            /* 官员等级 */
    uint8_t           bits;            /* 占用比特数 (始终=1) */
    uint8_t           sealed;          /* 1=已密封 · 不再修改 */

    /* 内容 */
    uint32_t          chars[CHARS_PER_BIT]; /* 一次层级: 2字符 */
    uint16_t          char_count;           /* 实际字数 (≤2) */

    /* 递归: 低层容器被密封在这里 */
    struct recursive_container *children;   /* 被密封的下级容器 */
    uint16_t          child_count;          /* 下级容器数 */
    uint16_t          child_capacity;       /* 下级容量 */

    /* 墙: 结束=开始 */
    uint8_t           wall_symbol;     /* 当前层级的墙符号 */
    uint8_t           is_start;        /* 此墙也是下一个的开始 */
    uint8_t           is_end;          /* 此墙也是上一个的结束 */

    /* 三元裁判 */
    struct recursive_container *left;   /* 左邻居 (旧容器) */
    struct recursive_container *right;  /* 右邻居 (新容器) */
    uint8_t           junction;        /* 墙点 · 就是自己 */

    /* 距离状态 */
    uint8_t           time_merged;     /* L1: 时间融合 */
    uint8_t           space_merged;    /* L2: 空间融合 */
    uint8_t           matter_merged;   /* L3: 物质融合 */

    /* 自动规则加载 */
    uint16_t          rule_template_id;  /* 已注册的规则模板 */
    uint8_t           auto_rendered;     /* 自动渲染完成 */
} recursive_container_t;

/* 容器群主: 管理一组容器 */
typedef struct {
    uint32_t            magic;           /* RECURSIVE_MAGIC */
    uint32_t            next_id;         /* 下一个id分配 */

    /* 存储池 (副存储) */
    recursive_container_t *containers;    /* 容器数组 */
    uint32_t            total_count;     /* 容器总数 */
    uint32_t            total_capacity;  /* 池容量 */

    /* 规则表头 (只注册一次 · 无限递归) */
    uint32_t            rule_header_id;  /* 规则表头id */
    uint8_t             wall_l1;         /* L1 墙符号 */
    uint8_t             wall_l2;         /* L2 墙符号 */
    uint8_t             wall_l3;         /* L3 墙符号 */

    /* 统计 */
    uint32_t            total_chars;     /* 总字符数 */
    uint32_t            total_bits;      /* 总bit数 */
    uint8_t             max_tier;        /* 当前最高层级 */

    /* 三元裁判统计 */
    uint32_t            junction_count;  /* 墙点数 */
    uint32_t            merge_count;     /* 融合次数 */
} container_group_t;

#define RECURSIVE_MAGIC  0x5245434E  /* "RECN" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化容器群 (副存储)
 */
void recur_init(container_group_t *group, recursive_container_t *pool, uint32_t pool_size);

/**
 * 创建一次层级士兵容器 (封装2字符)
 *   返回: 容器指针 · 0=失败
 */
recursive_container_t *recur_create_soldier(container_group_t *group,
                                            uint32_t char1, uint32_t char2);

/**
 * 二次层级密封: 密封一组一次层级容器
 *   把n个士兵折叠进一个长官 · 幂次对齐
 *   返回: 长官容器指针
 */
recursive_container_t *recur_seal_L2(container_group_t *group,
                                     recursive_container_t **soldiers, uint16_t count);

/**
 * 递归密封: 自动判断层级并封装
 */
recursive_container_t *recur_seal_auto(container_group_t *group,
                                       recursive_container_t **sources, uint16_t count);

/**
 * 注册规则表头 (一次注册 · 无限递归生效)
 *   自动授权 · 自动编译 · 自动渲染
 */
int recur_register_rule(container_group_t *group, uint16_t template_id);

/**
 * 三元裁决: 左(旧) + 墙(点) + 右(新)
 *   返回: 是否准入 (相同墙符号 · 层级对齐)
 */
uint8_t recur_junction_judge(recursive_container_t *left,
                              recursive_container_t *wall,
                              recursive_container_t *right);

/**
 * 距离消除:
 *   level=1: 时间融合 (结束=开始 · 消除先后)
 *   level=2: 空间融合 (密封 · 消除隔离)
 *   level=3: 物质融合 (合一 · 消除量化碎片)
 */
void recur_merge_distance(container_group_t *group, recursive_container_t *c, uint8_t level);

/**
 * 自动渲染新容器 (加载规则 · 继承算法)
 */
void recur_auto_render(container_group_t *group, recursive_container_t *c);

/* 统计 */
uint32_t recur_stats(container_group_t *group, const char *which);

#endif /* NJSG_RECURSIVE_CONTAINER_H */
