/**
 * hanzi_geometry.h — 汉字几何容器
 *
 * 用三个独立进制坐标系描述同一个汉字:
 *   部首进制 (214个符号) — 字的"材料"
 *   结构进制 (8个符号)   — 字的"空间排列"
 *   笔画进制 (5个符号)    — 字的"书写序列"
 *
 * 三个坐标唯一确定一个0维容器 (汉字本体)
 *
 * 与声音容器的关系:
 *   声音容器坍缩到1维 (时间序列)
 *   汉字几何容器坍缩到0维 (方块字)
 *   交汇层 = 声音地址 ↔ 汉字地址 的映射
 */

#ifndef NJSG_HANZI_GEOMETRY_H
#define NJSG_HANZI_GEOMETRY_H

#include <stdint.h>
#include <stdio.h>
/* global_interlock.h 在 enum 定义之后包含 · 避免宏展开破坏枚举 */

#define HANZI_GEOMETRY_MAGIC    0x48474D54  /* "HGMT" */
#define MAX_RADICALS            214
#define MAX_STROKES             50
#define MAX_STRUCTURES          8
#define MAX_CHAR_ENTRIES        254

/* 结构类型 (结构进制的8个符号) */
typedef enum {
    STRUCT_INVALID  = 0,
    STRUCT_LEFT_RIGHT   = 1,    /* 左右 */
    STRUCT_TOP_BOTTOM   = 2,    /* 上下 */
    STRUCT_SURROUND     = 3,    /* 全包围 */
    STRUCT_SEMI         = 4,    /* 半包围 */
    STRUCT_LCR          = 5,    /* 左中右 */
    STRUCT_TMB          = 6,    /* 上中下 */
    STRUCT_PINZI         = 7,    /* 品字 */
    STRUCT_SINGLE       = 8,    /* 独体 */
} structure_t;

/* 笔画类型 (笔画进制的5个符号) */
typedef enum {
    STROKE_HENG  = 0,   /* 横 */
    STROKE_SHU   = 1,   /* 竖 */
    STROKE_PIE   = 2,   /* 撇 */
    STROKE_NA    = 3,   /* 捺 */
    STROKE_ZHE   = 4,   /* 折 */
} stroke_t;

/* ── 全局互锁 (在所有 enum 定义之后 · 避免宏展开破坏枚举) ── */
#include "global_interlock.h"
#include "symbol_chain.h"

/* 一个汉字的几何描述 */
typedef struct {
    uint16_t    char_code;          /* Unicode或内部编码 */
    uint16_t    radical_primary;     /* 主部首索引 (0-213) */
    uint16_t    radical_secondary;   /* 次部首索引 (可选) */
    uint8_t     structure;          /* 结构类型 (1-8) */
    uint8_t     stroke_count;       /* 总笔画数 */
    uint8_t     stroke_sequence[MAX_STROKES]; /* 笔顺 (5进制序列) */
    
    /* 紧凑位编码: 部首(8bit) + 结构(3bit) + 笔画数(6bit) + 笔顺(3bit×n) */
    uint32_t    packed_bits;
    uint8_t     bit_length;
} hanzi_geom_entry_t;

/* 汉字几何容器 */
typedef struct {
    uint32_t    magic;
    uint16_t    version;
    uint16_t    entry_count;
    
    /* 三个子进制的符号集大小 */
    uint16_t    radical_count;      /* 实际使用的部首数 */
    uint8_t     structure_count;    /* 实际使用的结构数 */
    uint8_t     stroke_type_count;  /* 笔画类型数 (固定5) */
    
    /* 边界 = 部首数 × 结构数 × 最大笔画数 */
    uint32_t    border_total;
    
    /* 汉字条目表 */
    hanzi_geom_entry_t entries[MAX_CHAR_ENTRIES];
} hanzi_geometry_t;

/* ── API ───────────────────────────────────────────────────── */

void    hanzi_geom_init(hanzi_geometry_t *hg);
int     hanzi_geom_set_radicals(hanzi_geometry_t *hg, uint16_t count);
int     hanzi_geom_add_entry(hanzi_geometry_t *hg, uint16_t char_code,
                              uint16_t radical1, uint16_t radical2,
                              structure_t structure, uint8_t stroke_count,
                              const uint8_t *strokes);
uint32_t hanzi_geom_packed(const hanzi_geometry_t *hg, uint16_t idx);
void    hanzi_geom_stats(const hanzi_geometry_t *hg, FILE *out);

#endif /* NJSG_HANZI_GEOMETRY_H */
