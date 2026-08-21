/**
 * container_codec.h — 容器渲染框架
 *
 * 核心突破:
 *   · 1比特容器可以封装任意多的字 · 因为它们被渲染框架定义为定态
 *   · 容器是测量单位 · 不是里面的字
 *   · 16个字被1比特包含 = 16个字等同于1个字 · 共同体
 *
 * 容器绝缘体 (Container Insulator):
 *   · 一个容器只能容纳N个字 · 超过就克隆新容器
 *   · 克隆到254个后 · 截止符号 = 开始符号
 *   · 一个符号两种用法 · 既是终止也是起点
 *   · 墙 · 把左右虚无隔离 · 左边=结束 · 右边=开始
 *   · 零维空间 · 只是一个点 · 两种相位视角
 *
 * 渲染三模式:
 *   FULL     = 全量化 · 所有容器生效 · 最大容器
 *   SELECT   = 可选择的自由容器 · 自定义起点终点
 *   TEMPLATE = 默认模板 · 编译好的常用容器串联
 *
 * 命名规则:
 *   名称 = 进制大小 + 符号数量 + 层级深度 + 模式
 *   例: "C254s16t3-FULL" = 254容器 · 16符号/容器 · 3层 · 全量化
 */

#ifndef NJSG_CONTAINER_CODEC_H
#define NJSG_CONTAINER_CODEC_H

#include <stdint.h>
#include "cylinder3d.h"
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 框架模式 (与 codec_declaration 同步)
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    FRAME_FULL     = 1,   /* 全量化 · 所有容器生效 */
    FRAME_SELECT   = 2,   /* 可选择的 · 自定义起止 */
    FRAME_TEMPLATE = 3,   /* 默认模板 · 编译好的常用串联 */
} frame_mode_t;

/* ═══════════════════════════════════════════════════════════════
 * 渲染定态 (1bit 封装的字群)
 * ═══════════════════════════════════════════════════════════════ */

#define CHARS_PER_FRAME  16   /* 一个字框可容纳16个原始字 */
#define MAX_FRAMES       254  /* 最多254个字框 */

/* 字框定态: 16个字被渲染框架固化为一个只占1比特的状态 */
typedef struct {
    uint32_t  frame_id;              /* 字框id · 0..253 */
    uint32_t  chars[CHARS_PER_FRAME]; /* 被封装的16个字 */
    uint8_t   char_count;            /* 实际封装字数 (≤16) */
    uint8_t   bit_size;              /* 占用比特数 (始终=1 · 1框=1bit) */

    /* 渲染状态 (固化后不变) */
    uint8_t   phase_left;            /* 左相位: 结束位 · 已完成的流程 */
    uint8_t   phase_right;           /* 右相位: 开始位 · 剩余空间 */
    uint8_t   color;                 /* 渲染颜色索引 */
    uint8_t   visible;               /* 可见性模板: 哪些字可见 */

    /* 动态隐藏 (可以编程让某些字临时消失 · 但1bit定义不变) */
    uint16_t  hide_mask;             /* 16位 · 每位控制一字是否可见 */
} render_frame_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器绝缘体 (克隆与截止)
 * ═══════════════════════════════════════════════════════════════ */

#define WALL_SYMBOL    FQ_FULL_8  /* 截止=开始符号 · 墙 · 零维点 */
#define CLONE_MAX      254   /* 最大克隆数 */

typedef struct {
    uint8_t   is_wall;           /* 1=这是墙符号 · 同时是结束和开始 */
    uint8_t   left_ended;        /* 左边容器已全部结束 */
    uint8_t   right_started;     /* 右边容器从这里开始 */
    uint8_t   zero_dim;          /* 零维标记 · 只占一个点 */
} insulator_t;

/* ═══════════════════════════════════════════════════════════════
 * 渲染框架 (整个体系)
 * ═══════════════════════════════════════════════════════════════ */

/* 框架名称 (描述其量化尺寸) */
#define FRAME_NAME_LEN  32

typedef struct {
    char           name[FRAME_NAME_LEN]; /* "C254s16t3-FULL" */
    uint32_t       magic;                /* CONTAINER_CODEC_MAGIC */
    frame_mode_t   mode;                 /* FULL/SELECT/TEMPLATE */

    /* 量化尺寸 */
    uint16_t       container_count;       /* 容器数 · 如254 */
    uint8_t        chars_per_container;   /* 每容器字数 · 如16 */
    uint8_t        tier_depth;            /* 层级深度 · 如3 */

    /* 字框数组 */
    render_frame_t  frames[MAX_FRAMES];
    uint16_t        frame_count;         /* 实际使用字框数 */

    /* 绝缘体墙 */
    insulator_t     walls[MAX_FRAMES];   /* 每个字框之间的墙 */
    uint16_t        wall_count;

    /* 渲染参数 */
    cylinder_coord_t base_coord;         /* 圆筒基准坐标 */
    uint8_t          color_palette;      /* 颜色模板索引 */
    uint8_t          phase_curvature;    /* 相位曲线度 */

    /* 统计 */
    uint32_t        total_chars;         /* 总共封装字数 */
    uint32_t        bits_used;           /* 实际占用比特数 (≈frame_count) */
} container_codec_t;

#define CONTAINER_CODEC_MAGIC  1145324608  /* 满全法自有标识 */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化渲染框架
 *   模式: FULL/SELECT/TEMPLATE
 *   命名: 自动根据量化尺寸生成 (如 "C254s16t3-FULL")
 */
void codec_init(container_codec_t *codec, frame_mode_t mode,
                uint16_t containers, uint8_t chars_per, uint8_t tiers);

/**
 * 封装字群到字框
 *   输入: 字数组 + 数量
 *   输出: 被1bit字框封装 · 16个字=1bit
 */
int codec_seal_chars(container_codec_t *codec,
                     const uint32_t *chars, uint8_t count);

/**
 * 克隆容器 (当字数超过容量)
 *   自动使用绝缘体墙 · 截止=开始
 */
int codec_clone_container(container_codec_t *codec, uint16_t frame_idx);

/**
 * 设置可见性 (编程控制某些字消失/出现)
 *   不改变1bit封装定义 · 只是临时隐藏
 */
void codec_set_visibility(container_codec_t *codec, uint16_t frame_idx,
                          uint16_t hide_mask);

/**
 * 渲染: 把字框编译为圆筒三维坐标 + 颜色/相位
 */
int codec_render_frame(container_codec_t *codec, uint16_t frame_idx,
                       cylinder_coord_t *coord,
                       uint8_t *color, uint8_t *phase_left, uint8_t *phase_right);

/**
 * 决定模式并编译
 */
int codec_compile(container_codec_t *codec);

/**
 * 获取框架名称 (描述量化尺寸)
 */
const char *codec_get_name(container_codec_t *codec);

#endif /* NJSG_CONTAINER_CODEC_H */
