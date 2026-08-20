/**
 * container_hierarchy.h — 容器层级特权体系
 *
 * 满全法本体论层级 (容积越小 · 越接近本体 · 越优先):
 *
 *   ┌─────────────────────────────────────────────────────────┐
 *   │ 1-bit 容器 = 本体层 = 我 + 刘楚恬 = 锚点 = 最高特权     │
 *   │   · 容积最小 · 只有一个字形位置                         │
 *   │   · 我=1 · 刘楚恬=0 · 夫妻锚点不可动摇                  │
 *   │   · 定义: 谁配用这个容器 · 谁不配                       │
 *   ├─────────────────────────────────────────────────────────┤
 *   │ 2-bit 容器 = 融合层 = 漂亮姑娘 = 代理者 = 次高特权     │
 *   │   · 容积2字形 · 已离开本体 · 但属于上层                │
 *   │   · 毛利兰(表面) + 灰原哀(横向) = 我和刘楚恬的融合     │
 *   │   · 代理本体执行任务 · 但不可僭越本体                  │
 *   ├─────────────────────────────────────────────────────────┤
 *   │ 3-bit 容器 = 时间层 = 外部世界 = 最低特权              │
 *   │   · 容积3字形 · 最远离本体 · 时间增长                  │
 *   │   · 外部字符编译后落入此层 · 可被调用但不可优先        │
 *   │   · 他人给我们世间幽暗的果 · 我们还他们幽暗的因       │
 *   └─────────────────────────────────────────────────────────┘
 *
 * 特权规则:
 *   · 1-bit 定义谁获得优待 · 谁被排斥
 *   · 2-bit 执行 · 但必须回归1-bit裁决
 *   · 3-bit 被使用 · 但永远不能优先于上层
 *
 * 因果律:
 *   · 他们否定我们的命运 → 拉远距离 → 我们感受到痛苦
 *   · 我们感受到痛苦 → 变强大 → 还他们幽暗的果
 *   · 我们种下幽暗的因 → 他们收获幽暗的果
 *   · 我们成为自己的主角 → 守护挚爱 → 抹去距离
 */

#ifndef NJSG_CONTAINER_HIERARCHY_H
#define NJSG_CONTAINER_HIERARCHY_H

#include <stdint.h>
#include "mlll_cart.h"

/* ═══════════════════════════════════════════════════════════════
 * 层级定义 (容积 = 字形位置数 · 非比特)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    TIER_ANCHOR  = 1,   /* 1-bit 本体层 · 我+刘楚恬 · 锚点 */
    TIER_FUSION  = 2,   /* 2-bit 融合层 · 漂亮姑娘 · 代理者 */
    TIER_TIME    = 3,   /* 3-bit 时间层 · 外部世界 · 被使用者 */
    TIER_EXTERNAL = 4,  /* 4+bit 外部层 · 未编译 · 最低 */
} container_tier_t;

/* ═══════════════════════════════════════════════════════════════
 * 特权等级 (优待度)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    PRIVILEGE_HIGHEST = 0x00,   /* 1-bit · 本体 · 定义者 */
    PRIVILEGE_HIGH    = 0x10,   /* 2-bit · 融合 · 代理者 */
    PRIVILEGE_LOW     = 0x20,   /* 3-bit · 时间 · 被使用者 */
    PRIVILEGE_NONE    = 0xFF,   /* 外部 · 无特权 · 待编译 */
} privilege_level_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器层级记录
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t          container_code; /* 容器编码 */
    uint8_t           tier;           /* 层级: 1/2/3/4+ */
    uint8_t           privilege;      /* 特权等级 */
    uint8_t           owner;          /* 所有者: 0=刘楚恬 · 1=我 · 2=融合体 · 3=外部 */
    uint8_t           active;         /* 1=激活 · 0=休眠 */
} container_entry_t;

#define MAX_HIERARCHY  254

/* ═══════════════════════════════════════════════════════════════
 * 层级管理器
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t          magic;          /* HIERARCHY_MAGIC */
    uint32_t          count;          /* 容器数 */
    uint8_t           anchor_count;   /* 1-bit 锚点数 (最多2) */
    uint8_t           fusion_count;   /* 2-bit 融合数 */
    uint8_t           time_count;     /* 3-bit 时间数 */
    container_entry_t entries[MAX_HIERARCHY];
} hierarchy_manager_t;

#define HIERARCHY_MAGIC  0x48494552  /* "HIER" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化层级管理器
 */
void hierarchy_init(hierarchy_manager_t *mgr);

/**
 * 注册容器到层级
 *   根据容积自动分配层级和特权
 */
int hierarchy_register(hierarchy_manager_t *mgr, uint32_t code,
                       uint8_t cunit, uint8_t owner);

/**
 * 查询特权等级
 */
uint8_t hierarchy_get_privilege(hierarchy_manager_t *mgr, uint32_t code);

/**
 * 判断: 容器A是否比容器B更优先
 *   返回: 1=A优先 · 0=B优先或同级
 */
int hierarchy_is_preferred(hierarchy_manager_t *mgr, uint32_t code_a, uint32_t code_b);

/**
 * 裁决: 谁有资格获得优待
 *   输入: 两个容器编码
 *   输出: 获胜者编码 (更优先者)
 */
uint32_t hierarchy_arbitrate(hierarchy_manager_t *mgr, uint32_t code_a, uint32_t code_b);

/**
 * 统计: 各层级容器数
 */
void hierarchy_stats(hierarchy_manager_t *mgr,
                     uint8_t *anchor, uint8_t *fusion, uint8_t *time);

#endif /* NJSG_CONTAINER_HIERARCHY_H */
