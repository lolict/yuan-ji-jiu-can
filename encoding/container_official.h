/**
 * container_official.h — 容器建制 · 纯编码版
 */

#ifndef NJSG_CONTAINER_OFFICIAL_H
#define NJSG_CONTAINER_OFFICIAL_H

#include <stdint.h>

#define SYM_KND_0  'k'
#define SYM_KND_1  'n'
#define SYM_KND_2  'd'
#define SYM_KND_3  '!'

/* 容器 */
typedef struct {
    uint32_t code;
    uint8_t  tier;
    uint8_t  width;
    uint8_t  phase;
    uint8_t  sealed;
    uint32_t data[4];
    uint8_t  data_count;
    uint8_t  movable;
    uint32_t parent;
    uint32_t children[8];
    uint8_t  child_count;
} cnt_official_t;

#define MAX_CONTAINERS 254

/* 容器系 */
typedef struct {
    uint32_t      magic;
    cnt_official_t arr[MAX_CONTAINERS];
    uint16_t      count;
    uint8_t       max_tier;
    uint32_t      total_chars;
    uint8_t       compiled;
} cnt_system_t;

void    cnt_init(cnt_system_t *sys);
int     cnt_create(cnt_system_t *sys, uint8_t tier, uint8_t width);
int     cnt_write(cnt_system_t *sys, uint32_t code, uint32_t *words, uint8_t n, uint8_t movable);
void    cnt_set_parent(cnt_system_t *sys, uint32_t parent, uint32_t child);
int     cnt_compile(cnt_system_t *sys, uint8_t *buf, uint32_t size);
uint8_t cnt_get_tier(cnt_system_t *sys, uint32_t code);
uint8_t cnt_get_width(cnt_system_t *sys, uint32_t code);

#endif
