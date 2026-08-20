/**
 * knd_container.c — 装字 · 密封 · 管辖 · 编译
 */

#include "knd_container.h"
#include <string.h>
#include <stdlib.h>

/* 创建容器 */
knd_container_t *knd_create(uint8_t tier) {
    knd_container_t *c = (knd_container_t *)calloc(1, sizeof(knd_container_t));
    if (!c) return 0;

    /* 墙knd · 开始=结束 */
    c->wall.present = 1;
    c->wall.is_start = 1;
    c->wall.is_end = 1;
    c->tier = tier ? tier : 1;
    c->width = tier ? tier : 1;
    c->child_left = 0;
    c->child_right = 0;
    c->parent = 0;

    return c;
}

/* 装字入虚空 */
int knd_seal(knd_container_t *c, uint32_t *words, uint8_t n, uint8_t movable) {
    if (!c || !words || n == 0 || n > 4) return -1;

    knd_void_t *v = &c->right_void;  /* 装新虚空 */
    for (uint8_t i = 0; i < n; i++) v->data[i] = words[i];
    v->count = n;
    v->movable = movable;
    v->sealed = 0;

    return 0;
}

/* 密封 */
void knd_seal_container(knd_container_t *c, uint8_t level) {
    if (!c) return;
    c->right_void.sealed = 1;
    c->left_void.sealed = 1;
    if (level >= 2) {
        /* 强密封 · 虚空物质化 */
        c->right_void.movable = 0;
        c->left_void.movable = 0;
    }
}

/* 管辖 */
void knd_set_parent(knd_container_t *parent, knd_container_t *child) {
    if (!parent || !child) return;
    child->parent = parent;
    if (!parent->child_left) parent->child_left = child;
    else if (!parent->child_right) parent->child_right = child;
}

/* 移动 */
void knd_move(knd_container_t *c, uint32_t x, uint32_t y, uint32_t z) {
    if (!c || !c->right_void.movable) return;
    c->right_void.screen_x = x;
    c->right_void.screen_y = y;
    c->right_void.screen_z = z;
}

/* 统计 */
static uint32_t count_rec(knd_container_t *c) {
    if (!c) return 0;
    return 1 + count_rec(c->child_left) + count_rec(c->child_right);
}

uint32_t knd_count_containers(knd_container_t *root) {
    return count_rec(root);
}

/* 编译到自有格式 .mll0 */
int knd_compile(knd_container_t *root, uint8_t *out, uint32_t size) {
    if (!root || !out || size < 8) return -1;

    uint32_t pos = 0;

    /* 头: knd! + 容器数 */
    out[pos++] = KND_MAJOR;
    out[pos++] = KND_BRIDGE;
    out[pos++] = KND_MINOR;
    out[pos++] = '!';

    uint32_t cnt = knd_count_containers(root);
    memcpy(out + pos, &cnt, 4);
    pos += 4;

    /* 递归序列化 */
    /* 简化: 层序遍历写入 */
    knd_container_t *stack[256];
    int sp = 0;
    stack[sp++] = root;

    while (sp > 0 && pos + 24 < size) {
        knd_container_t *c = stack[--sp];
        if (!c) continue;

        /* 写容器: tier(1) + wall(3) + count(1) + data(4*4) + coords(12) */
        out[pos++] = c->tier;
        out[pos++] = c->wall.present;
        out[pos++] = c->wall.is_start;
        out[pos++] = c->wall.is_end;

        out[pos++] = c->right_void.count;
        out[pos++] = c->right_void.movable;
        out[pos++] = c->right_void.sealed;

        for (int i = 0; i < 4; i++) {
            memcpy(out + pos, &c->right_void.data[i], 4);
            pos += 4;
        }

        memcpy(out + pos, &c->right_void.screen_x, 4); pos += 4;
        memcpy(out + pos, &c->right_void.screen_y, 4); pos += 4;
        memcpy(out + pos, &c->right_void.screen_z, 4); pos += 4;

        if (c->child_right) stack[sp++] = c->child_right;
        if (c->child_left)  stack[sp++] = c->child_left;
    }

    return (int)pos;
}
