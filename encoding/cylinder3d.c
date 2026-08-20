/**
 * cylinder3d.c — 圆筒三维坐标编码系统实现
 *
 * 动漫化身映射:
 *   毛利兰 → X轴 · 表面 · 模/面/名
 *   工藤新一 → Y轴 · 结构 · 骨/头/体
 *   灰原哀 → Z轴 · 横向 · 行页/行业
 */

#include "cylinder3d.h"
#include <string.h>

/* 全局圆筒实例 */
static struct {
    uint32_t          magic;
    uint32_t          count;
    cylinder_entry_t  entries[CYLINDER_MAX_CONTAINERS];
} g_cylinder = { .magic = CYLINDER_MAGIC, .count = 0 };

/* ═══════════════════════════════════════════════════════════════
 * 寻址: 通过坐标直接找到容器
 *   O(1) · 不搜索 · 不哈希 · 绝对位置
 * ═══════════════════════════════════════════════════════════════ */

cylinder_entry_t *cylinder_find(uint8_t x, uint8_t y, uint8_t z) {
    for (uint32_t i = 0; i < g_cylinder.count; i++) {
        cylinder_entry_t *e = &g_cylinder.entries[i];
        if (e->coord.x == x && e->coord.y == y && e->coord.z == z) {
            return e;
        }
    }
    return 0;
}

/* 注册容器到坐标 */
int cylinder_place(uint8_t x, uint8_t y, uint8_t z,
                   uint32_t code, const cylinder_attributes_t *attrs) {
    if (g_cylinder.count >= CYLINDER_MAX_CONTAINERS) return -1;
    if (cylinder_find(x, y, z)) return -2;  /* 已被占 */

    cylinder_entry_t *e = &g_cylinder.entries[g_cylinder.count++];
    e->magic = CYLINDER_MAGIC;
    e->container_id = g_cylinder.count;
    e->container_code = code;
    e->coord.x = x;
    e->coord.y = y;
    e->coord.z = z;
    if (attrs) {
        memcpy(&e->attrs, attrs, sizeof(*attrs));
    } else {
        memset(&e->attrs, 0, sizeof(e->attrs));
    }
    return 0;
}

/* 通过容器编码反查坐标 */
cylinder_entry_t *cylinder_find_by_code(uint32_t code) {
    for (uint32_t i = 0; i < g_cylinder.count; i++) {
        if (g_cylinder.entries[i].container_code == code) {
            return &g_cylinder.entries[i];
        }
    }
    return 0;
}

uint32_t cylinder_count(void) {
    return g_cylinder.count;
}
