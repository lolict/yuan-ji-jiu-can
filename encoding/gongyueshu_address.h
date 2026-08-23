/**
 * gongyueshu_address.h — 公约数地址容器
 *
 * 用6个基韵锚 (公约数) 折叠所有韵母 · 缩写地址长度
 *   全长 "ai" (8+8=16bit) → 公约数路径 [a,i] (3+3=6bit)
 *   全长 "iang" (40bit) → [i,a,ng] (3+3+3=9bit)
 *
 * 是产权容器: 有自己的编码系统 (公约数路径编码)
 * 输出可喂给声音进制容器使用
 */

#ifndef NJSG_GONGYUESHU_ADDRESS_H
#define NJSG_GONGYUESHU_ADDRESS_H

#include <stdint.h>
#include <stdio.h>

#define GONGYUESHU_MAGIC        0x47594144  /* "GYAD" */
#define GONGYUESHU_MAX_ENTRIES  254         /* 与集装箱/声音一致 */

/* 路径中的特殊标记 */
#define GY_MARK_N       6   /* 前鼻音 -n */
#define GY_MARK_NG      7   /* 后鼻音 -ng */

typedef struct {
    uint8_t     shengmu_idx;    /* 声母索引 (0-47, 0xFF=零声母) */
    uint8_t     gy_path[4];     /* 公约数路径 (每个元素 0-7) */
    uint8_t     path_len;       /* 路径长度 1-4 */
    uint8_t     tone;           /* 声调 1-4 */
    uint32_t    packed_bits;    /* 紧凑位编码 */
    uint8_t     bit_length;     /* 实际位长度 */
    
    /* 直接可用的声音参数 (与声音进制共享) */
    uint16_t    frequency;      /* 基频 (×0.1Hz) */
    uint8_t     contour;        /* 轮廓 (0=平 1=升 2=降 3=凹) */
    uint16_t    duration;       /* 时长 (ms) */
    uint8_t     amplitude;      /* 强度 (0-255) */
} gongyueshu_entry_t;

typedef struct {
    uint32_t    magic;
    uint16_t    version;
    uint16_t    entry_count;
    uint16_t    reserved;
    gongyueshu_entry_t entries[GONGYUESHU_MAX_ENTRIES];
} gongyueshu_addr_t;

/* ── API ───────────────────────────────────────────────────── */

void    gongyueshu_init(gongyueshu_addr_t *ga);
int     gongyueshu_encode(gongyueshu_addr_t *ga, const char *shengmu,
                           const char *yunmu, uint8_t tone);
int     gongyueshu_decode(const gongyueshu_addr_t *ga, uint16_t idx,
                           char *shengmu_out, char *yunmu_out,
                           uint8_t *tone_out, uint16_t *freq_out);
uint32_t gongyueshu_get_packed(const gongyueshu_addr_t *ga, uint16_t idx);
uint8_t gongyueshu_get_bit_length(const gongyueshu_addr_t *ga, uint16_t idx);
uint16_t gongyueshu_freq(uint8_t gongyueshu_idx);
void    gongyueshu_stats(const gongyueshu_addr_t *ga, FILE *out);

#endif /* NJSG_GONGYUESHU_ADDRESS_H */
