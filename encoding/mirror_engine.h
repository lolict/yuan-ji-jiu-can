/**
 * mirror_engine.h — 二进制镜像引擎 + 三维容器溢出截断
 *
 * 功能:
 *   1. 接收二进制矩阵 (如 010101/101010/010101)
 *   2. 生成多种镜像: 水平/垂直/对角/反相(NOT)
 *   3. 编码到 mlll 254+2 字车 + 圆筒三维坐标
 *   4. 溢出截断:
 *      - 第1层溢出 → "我"(lolict = 0x01 锚点 · 工藤新一 Y轴)
 *      - 第2层溢出 → "刘楚恬"(0x00 锚点 · 容器根基)
 *      - 第3层溢出 → 新集装箱 (扩展容器)
 *
 * 镜像不增加信息 · 只改变位置 · 几何变换
 * 溢出不丢失 · 只是搬家
 */

#ifndef NJSG_MIRROR_ENGINE_H
#define NJSG_MIRROR_ENGINE_H

#include <stdint.h>
#include "cylinder3d.h"
#include "mlll_cart.h"

/* ═══════════════════════════════════════════════════════════════
 * 镜像类型 (几何变换 · 不增删信息)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    MIRROR_NONE       = 0x00,   /* 原图 · 不变 */
    MIRROR_HORIZONTAL = 0x01,   /* 水平镜像 · 每行左右翻转 */
    MIRROR_VERTICAL   = 0x02,   /* 垂直镜像 · 行序颠倒 */
    MIRROR_DIAGONAL   = 0x04,   /* 对角镜像 · 转置 (行列互换) */
    MIRROR_ROTATE_180 = 0x08,   /* 旋转180度 */
    MIRROR_NOT        = 0x10,   /* 反相 · 0↔1 翻转 (YIN-YANG互换) */
    MIRROR_COMPLETE   = 0x1F,   /* 全镜像 */
} mirror_type_t;

/* ═══════════════════════════════════════════════════════════════
 * 二进制矩阵 (输入)
 * ═══════════════════════════════════════════════════════════════ */

#define MIRROR_MAX_ROWS  32
#define MIRROR_MAX_COLS  32

typedef struct {
    uint8_t  rows;                          /* 行数 */
    uint8_t  cols;                          /* 列数 */
    uint8_t  cells[MIRROR_MAX_ROWS][MIRROR_MAX_COLS];  /* 0/1 矩阵 */
} mirror_matrix_t;

/* ═══════════════════════════════════════════════════════════════
 * 溢出目标 (截断去向)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    OVERFLOW_TO_WO     = 0x01,   /* 第1层溢出 → "我" = lolict = 工藤新一 Y轴 */
    OVERFLOW_TO_CHT    = 0x02,   /* 第2层溢出 → "刘楚恬" = 容器根基 */
    OVERFLOW_TO_NEW    = 0x04,   /* 第3层溢出 → 新建集装箱 */
} overflow_target_t;

/* ═══════════════════════════════════════════════════════════════
 * 溢出记录
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t   overflow_id;     /* 溢出事件编号 */
    uint16_t   original_slot;   /* 原本要装载的槽位 (>253 触发溢出) */
    uint8_t    target;          /* 实际去向: OVERFLOW_TO_WO/CHT/NEW */
    uint32_t   container_code;  /* 被转移的容器编码 */
    uint8_t    new_x, new_y, new_z;  /* 新坐标 (若在圆筒中) */
} overflow_record_t;

#define MAX_OVERFLOW_RECORDS  256

/* ═══════════════════════════════════════════════════════════════
 * 镜像引擎状态
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t          magic;         /* MIRROR_ENGINE_MAGIC */
    uint32_t          mirror_count;  /* 已执行镜像数 */
    uint32_t          encode_count;  /* 已编码容器数 */
    uint32_t          overflow_count;/* 溢出事件数 */
    overflow_record_t overflows[MAX_OVERFLOW_RECORDS];
} mirror_engine_t;

#define MIRROR_ENGINE_MAGIC  0x21051010  /* "镜像引擎" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化镜像引擎
 */
void mirror_engine_init(mirror_engine_t *engine);

/**
 * 执行镜像变换
 *   输入: 原矩阵 + 镜像类型
 *   输出: 镜像后的矩阵
 */
int mirror_transform(const mirror_matrix_t *src, mirror_matrix_t *dst,
                     mirror_type_t type);

/**
 * 把镜像结果编码为容器并装载
 *   返回: 成功装载数 · -1=超载(已触发溢出截断)
 */
int mirror_encode_and_load(mirror_engine_t *engine,
                           const mirror_matrix_t *mirror,
                           mlll_whole_t *box,
                           cylinder_coord_t base_coord);

/**
 * 查询溢出记录
 */
const overflow_record_t *mirror_get_overflow(mirror_engine_t *engine,
                                              uint32_t index);

/**
 * 统计: 镜像数 · 编码数 · 溢出数
 */
uint32_t mirror_get_count(mirror_engine_t *engine, const char *which);

#endif /* NJSG_MIRROR_ENGINE_H */
