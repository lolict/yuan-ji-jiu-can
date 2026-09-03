/*
 * mlil_visual.h — mLIL 三维可视化投影
 *
 * 将链状态映射为三维几何坐标，输出可视化。
 * 
 * mLIL (毛利兰) 格式:
 *   X轴: 表面属性 (模/面/名) = 符号的呈现特征
 *   Y轴: 结构属性 (骨/头/体) = 链中的结构位置 (版本)
 *   Z轴: 横向属性 (行/页/行业) = 层级（元→场）
 *
 * 每个链节点在三维空间中有一个静态绝对位置：
 *   不需要搜索，不需要哈希 — 坐标即地址
 *
 * 输出格式:
 *   ASCII 3D 柱投影视图
 *   JSON 坐标导出 (供外部3D渲染)
 */

#ifndef NJSG_MLIL_VISUAL_H
#define NJSG_MLIL_VISUAL_H

#include <stdint.h>
#include "chain_block.h"

/* 三维空间中的节点坐标 */
typedef struct {
    float       x;          /* 表面: 0.0 ~ 255.0 (模/面/名) */
    float       y;          /* 结构: 版本链中的位置 */
    float       z;          /* 层级: 0(元) ~ 7(场) */
    
    /* 渲染属性 */
    uint8_t     r, g, b;    /* 颜色 (按类别) */
    char        label[32];  /* 显示标签 */
    uint32_t    chain_id;   /* 所属链 */
    uint32_t    version;    /* 版本号 */
    uint8_t     level;      /* 层级 0-7 */
} mlil_point_t;

/* 三维场景 */
#define MLIL_MAX_POINTS     4096

typedef struct {
    mlil_point_t    points[MLIL_MAX_POINTS];
    uint32_t        count;
    char            title[128];
    
    /* 视角 */
    float           view_angle_x;
    float           view_angle_y;
    float           zoom;
} mlil_scene_t;

/* 链 → 3D场景 */
int     链转场景(const chain_t* chain, uint32_t chain_id, mlil_scene_t* scene);
int     多链场景(const chain_t* chains, uint32_t count, mlil_scene_t* scene);

/* 可视化 */
void    投影视图(const mlil_scene_t* scene);           /* ASCII 投影 */
void    JSON导出(const mlil_scene_t* scene, const char* path); /* JSON输出 */

/* 颜色映射 */
uint32_t 类别颜色(uint8_t category);

#endif /* NJSG_MLIL_VISUAL_H */
