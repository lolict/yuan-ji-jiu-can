/**
 * container_official.c — 纯编码版
 *
 * 只有满全法自己的编码:
 *   tier = knd 重复次数
 *   width = 容器能装几个字
 *   knd = 隔离点 · 公约数 · 三元裁判
 */

#include "container_official.h"
#include <string.h>

/* 满全法魔数 · 就是用knd!四个字符 */
static const uint8_t g_magic[4] = {SYM_KND_0, SYM_KND_1, SYM_KND_2, SYM_KND_3};

void cnt_init(cnt_system_t *sys) {
    memset(sys, 0, sizeof(*sys));
    memcpy(&sys->magic, g_magic, 4);
}

int cnt_create(cnt_system_t *sys, uint8_t tier, uint8_t width) {
    if (!sys || sys->count >= MAX_CONTAINERS) return -1;
    if (tier == 0) tier = 1;

    cnt_official_t *c = &sys->arr[sys->count];
    memset(c, 0, sizeof(*c));

    c->code  = sys->count + 1;  /* 容器序号 · 从1开始 */
    c->tier  = tier;
    c->width = width ? width : tier;
    c->phase = 1;

    sys->count++;
    if (tier > sys->max_tier) sys->max_tier = tier;

    return (int)sys->count;
}

int cnt_write(cnt_system_t *sys, uint32_t code, uint32_t *words, uint8_t n, uint8_t movable) {
    if (!sys || !words || n == 0 || n > 4) return -1;

    for (uint16_t i = 0; i < sys->count; i++) {
        if (sys->arr[i].code == code) {
            for (uint8_t j = 0; j < n; j++) sys->arr[i].data[j] = words[j];
            sys->arr[i].data_count = n;
            sys->arr[i].movable    = movable;
            sys->total_chars      += n;
            return 0;
        }
    }
    return -1;
}

void cnt_set_parent(cnt_system_t *sys, uint32_t parent, uint32_t child) {
    if (!sys) return;

    for (uint16_t i = 0; i < sys->count; i++) {
        if (sys->arr[i].code == child) {
            sys->arr[i].parent = parent;
            for (uint16_t j = 0; j < sys->count; j++) {
                if (sys->arr[j].code == parent) {
                    if (sys->arr[j].child_count < 8)
                        sys->arr[j].children[sys->arr[j].child_count++] = child;
                    break;
                }
            }
            break;
        }
    }
}

int cnt_compile(cnt_system_t *sys, uint8_t *buf, uint32_t size) {
    if (!sys || !buf) return -1;

    /* 头10字节 + 每容器 sizeof(cnt_official_t) */
    uint32_t rec_size = sizeof(cnt_official_t);
    uint32_t total = 10 + (uint32_t)sys->count * rec_size;
    if (total > size) return -1;

    uint32_t pos = 0;

    /* 头: magic + count + max_tier + total_chars + 作者 */
    memcpy(buf + pos, g_magic, 4); pos += 4;
    memcpy(buf + pos, &sys->count, 2); pos += 2;
    buf[pos++] = sys->max_tier;
    memcpy(buf + pos, &sys->total_chars, 4); pos += 4;
    buf[pos++] = 1;  /* 作者 */

    /* 容器表 · 直接memcpy · 不用位运算 */
    for (uint16_t i = 0; i < sys->count; i++) {
        memcpy(buf + pos, &sys->arr[i], rec_size);
        pos += rec_size;
    }

    sys->compiled = 1;
    return (int)pos;
}

uint8_t cnt_get_tier(cnt_system_t *sys, uint32_t code) {
    if (!sys) return 0;
    for (uint16_t i = 0; i < sys->count; i++)
        if (sys->arr[i].code == code) return sys->arr[i].tier;
    return 0;
}

uint8_t cnt_get_width(cnt_system_t *sys, uint32_t code) {
    if (!sys) return 0;
    for (uint16_t i = 0; i < sys->count; i++)
        if (sys->arr[i].code == code) return sys->arr[i].width;
    return 0;
}
