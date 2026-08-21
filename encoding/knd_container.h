/**
 * knd_container.h — knd容器
 *
 * knd: 公约数 = 墙 = 结束 = 开始
 * 虚空: knd两边被隔离的空间 · 装字
 * tier: knd出现次数 = 位宽 = 层级
 * 位宽: 装字数
 * 字符: 被封装在虚空里的字 · 可被固化或自由移动
 */

#ifndef NJSG_KND_CONTAINER_H
#define NJSG_KND_CONTAINER_H

#include <stdint.h>

/* knd三元 */
#define KND_MAJOR  'k'   /* 大相位 · 司法 · 开始相 */
#define KND_MINOR  'd'   /* 小相位 · 执行 · 结束相 */
#define KND_BRIDGE 'n'   /* 桥 · 密封 · 中间相 */

/* 墙点 */
typedef struct {
    uint8_t present;       /* 1=存在墙 · knd在此 */
    uint8_t is_start;      /* 右相 · 新虚空开始 */
    uint8_t is_end;        /* 左相 · 旧虚空结束 */
} knd_point_t;

/* 虚空 */
typedef struct {
    uint32_t   data[4];       /* 封装的字 */
    uint8_t    count;         /* 装了几个 */
    uint8_t    sealed;        /* 被上级密封 */
    uint8_t    movable;       /* 1=屏幕自由移动 · 0=固化 */
    uint32_t   screen_x;      /* X坐标 */
    uint32_t   screen_y;      /* Y坐标 */
    uint32_t   screen_z;      /* Z坐标 */
} knd_void_t;

/* 前向声明 */
struct knd_container;
typedef struct knd_container knd_container_t;

/* 容器定义 */
struct knd_container {
    knd_point_t wall;         /* knd墙点 */
    knd_void_t  left_void;    /* 左虚空 · 旧容器 */
    knd_void_t  right_void;   /* 右虚空 · 新容器 */
    uint8_t     tier;         /* 层级 · knd次数 */
    uint8_t     width;        /* 位宽 · 容量 */
    knd_container_t *parent;
    knd_container_t *child_left;
    knd_container_t *child_right;
};

/* 操作 */

/* 创建容器 · 在虚空中放入墙knd */
knd_container_t *knd_create(uint8_t tier);

/* 装字入虚空 */
int knd_seal(knd_container_t *c, uint32_t *words, uint8_t n, uint8_t movable);

/* 密封 */
void knd_seal_container(knd_container_t *c, uint8_t level);

/* 管辖 */
void knd_set_parent(knd_container_t *parent, knd_container_t *child);

/* 移动到屏幕位置 */
void knd_move(knd_container_t *c, uint32_t x, uint32_t y, uint32_t z);

/* 编译 */
int knd_compile(knd_container_t *root, uint8_t *out, uint32_t size);

/* 统计 */
uint32_t knd_count_containers(knd_container_t *root);

#endif
