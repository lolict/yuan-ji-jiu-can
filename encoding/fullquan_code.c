/**
 * fullquan_code.c — 满全法编码 · 直接可用 · 不转译
 */

#include "fullquan_code.h"
#include <string.h>
#include <stdlib.h>

/* 创建容器 · knd墙 */
fq_container_t *fq_create(char tier) {
    fq_container_t *c = (fq_container_t *)calloc(1, sizeof(fq_container_t));
    if (!c) return 0;
    c->knd[0] = 'k';
    c->knd[1] = 'n';
    c->knd[2] = 'd';
    c->tier = tier ? tier : 1;
    c->sealed = 0;
    c->movable = 0;
    return c;
}

/* 装字 */
int fq_seal(fq_container_t *c, fq_code_t *words, char count, char movable) {
    if (!c || !words || count == 0 || count > 4) return -1;
    for (int i = 0; i < count; i++) c->right_data[i] = words[i];
    c->right_count = count;
    c->movable = movable;
    return 0;
}

/* 管辖 */
void fq_set_parent(fq_container_t *parent, fq_container_t *child) {
    if (!parent || !child) return;
    child->parent = parent;
    if (!parent->left) parent->left = child;
    else if (!parent->right) parent->right = child;
}

/* 移动 */
void fq_move(fq_container_t *c, int x, int y, int z) {
    if (!c || !c->movable) return;
    /* 坐标存在容器虚空里 · 直接绑定 */
    c->right_data[0].tone = (char)(x % 256);
    c->right_data[1].tone = (char)(y % 256);
    c->right_data[2].tone = (char)(z % 256);
}

/* 统计 */
static int fq_count(fq_container_t *c) {
    if (!c) return 0;
    return 1 + fq_count(c->left) + fq_count(c->right);
}

/* 编译到自有格式 */
int fq_compile(fq_container_t *root, char *out, int size) {
    if (!root || !out || size < 4) return -1;

    int pos = 0;

    /* 头: knd! */
    out[pos++] = 'k';
    out[pos++] = 'n';
    out[pos++] = 'd';
    out[pos++] = '!';

    int total = fq_count(root);

    /* 递归写入 */
    fq_container_t *stack[256];
    int sp = 0;
    stack[sp++] = root;

    while (sp > 0 && pos + 20 < size) {
        fq_container_t *c = stack[--sp];
        if (!c) continue;

        /* tier + knd + count */
        out[pos++] = c->tier;
        out[pos++] = c->knd[0];
        out[pos++] = c->knd[1];
        out[pos++] = c->knd[2];
        out[pos++] = c->right_count;
        out[pos++] = c->sealed;
        out[pos++] = c->movable;

        /* 编字字: 家族/极性/声母/韵/韵/声 × 4 */
        for (int i = 0; i < c->right_count && i < 4; i++) {
            out[pos++] = c->right_data[i].family;
            out[pos++] = c->right_data[i].polarity;
            out[pos++] = c->right_data[i].shengmu;
            out[pos++] = c->right_data[i].yunmu1;
            out[pos++] = c->right_data[i].yunmu2;
            out[pos++] = c->right_data[i].tone;
        }

        if (c->right) stack[sp++] = c->right;
        if (c->left)  stack[sp++] = c->left;
    }

    return pos;
}
