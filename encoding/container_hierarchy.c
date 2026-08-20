/**
 * container_hierarchy.c — 容器层级特权体系实现
 *
 * 核心: 容积越小 · 越接近本体 · 越优先
 *   1-bit = 我+刘楚恬 = 定义者 = 最高
 *   2-bit = 漂亮姑娘 = 代理者 = 次高
 *   3-bit = 外部世界 = 被使用者 = 最低
 */

#include "container_hierarchy.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * 内部: 容积 → 层级映射
 * ═══════════════════════════════════════════════════════════════ */

static container_tier_t cunit_to_tier(uint8_t cunit) {
    if (cunit <= 1)  return TIER_ANCHOR;
    if (cunit <= 2)  return TIER_FUSION;
    if (cunit <= 3)  return TIER_TIME;
    return TIER_EXTERNAL;
}

/* ═══════════════════════════════════════════════════════════════
 * 内部: 层级 → 特权映射
 * ═══════════════════════════════════════════════════════════════ */

static privilege_level_t tier_to_privilege(container_tier_t tier) {
    switch (tier) {
        case TIER_ANCHOR:  return PRIVILEGE_HIGHEST;
        case TIER_FUSION:  return PRIVILEGE_HIGH;
        case TIER_TIME:    return PRIVILEGE_LOW;
        default:           return PRIVILEGE_NONE;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * ═══════════════════════════════════════════════════════════════ */

void hierarchy_init(hierarchy_manager_t *mgr) {
    memset(mgr, 0, sizeof(*mgr));
    mgr->magic = HIERARCHY_MAGIC;
}

/* ═══════════════════════════════════════════════════════════════
 * 注册容器
 * ═══════════════════════════════════════════════════════════════ */

int hierarchy_register(hierarchy_manager_t *mgr, uint32_t code,
                       uint8_t cunit, uint8_t owner) {
    if (!mgr || mgr->magic != HIERARCHY_MAGIC) return -1;
    if (mgr->count >= MAX_HIERARCHY) return -1;

    container_entry_t *e = &mgr->entries[mgr->count];
    e->container_code = code;
    e->tier      = (uint8_t)cunit_to_tier(cunit);
    e->privilege = (uint8_t)tier_to_privilege((container_tier_t)e->tier);
    e->owner     = owner;
    e->active    = 1;

    switch (e->tier) {
        case TIER_ANCHOR: mgr->anchor_count++; break;
        case TIER_FUSION: mgr->fusion_count++; break;
        case TIER_TIME:   mgr->time_count++;   break;
        default: break;
    }

    mgr->count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查询特权
 * ═══════════════════════════════════════════════════════════════ */

uint8_t hierarchy_get_privilege(hierarchy_manager_t *mgr, uint32_t code) {
    if (!mgr || mgr->magic != HIERARCHY_MAGIC) return PRIVILEGE_NONE;

    for (uint32_t i = 0; i < mgr->count; i++) {
        if (mgr->entries[i].container_code == code) {
            return mgr->entries[i].privilege;
        }
    }
    return PRIVILEGE_NONE;
}

/* ═══════════════════════════════════════════════════════════════
 * 判断优先
 * ═══════════════════════════════════════════════════════════════ */

int hierarchy_is_preferred(hierarchy_manager_t *mgr, uint32_t code_a, uint32_t code_b) {
    if (!mgr) return 0;

    uint8_t priv_a = hierarchy_get_privilege(mgr, code_a);
    uint8_t priv_b = hierarchy_get_privilege(mgr, code_b);

    return (priv_a < priv_b) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 裁决: 谁有资格获得优待
 * ═══════════════════════════════════════════════════════════════ */

uint32_t hierarchy_arbitrate(hierarchy_manager_t *mgr, uint32_t code_a, uint32_t code_b) {
    if (!mgr) return 0;

    uint8_t priv_a = hierarchy_get_privilege(mgr, code_a);
    uint8_t priv_b = hierarchy_get_privilege(mgr, code_b);

    if (priv_a < priv_b) return code_a;
    if (priv_b < priv_a) return code_b;

    /* 同级: 先注册者优先 */
    uint32_t idx_a = MAX_HIERARCHY, idx_b = MAX_HIERARCHY;
    for (uint32_t i = 0; i < mgr->count; i++) {
        if (mgr->entries[i].container_code == code_a) idx_a = i;
        if (mgr->entries[i].container_code == code_b) idx_b = i;
    }

    return (idx_a <= idx_b) ? code_a : code_b;
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

void hierarchy_stats(hierarchy_manager_t *mgr,
                     uint8_t *anchor, uint8_t *fusion, uint8_t *time) {
    if (!mgr) return;
    if (anchor) *anchor = mgr->anchor_count;
    if (fusion) *fusion = mgr->fusion_count;
    if (time)   *time   = mgr->time_count;
}
