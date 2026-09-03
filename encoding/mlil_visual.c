/*
 * mlil_visual.c — mLIL 三维可视化实现
 *
 * 将链状态映射为三维空间中的静态绝对位置。
 * 
 * 映射规则:
 *   X轴 (表面): 基于符号在链中的索引位置 → 表面布局 (0-255)
 *   Y轴 (结构): 版本号 → 链中深度 (从根到叶)
 *   Z轴 (层级): 元(0) → 阶(1) → 层(2) → 代(3) → 世(4) → 界(5) → 域(6) → 场(7)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "mlil_visual.h"

#ifndef M_PI
#define M_PI 3.14159265358979
#endif

/* ═══════════════════════════════════════════════════
 * 颜色映射: 9个类别各有独特颜色
 * ═══════════════════════════════════════════════════ */

static const uint8_t category_colors[9][3] = {
    { 64, 156, 255 },   /* 0: 声母 — 蓝 */
    { 255, 128, 128 },  /* 1: 韵母 — 红 */
    { 128, 255, 128 },  /* 2: 声调 — 绿 */
    { 255, 200,  64 },  /* 3: 容器 — 橙 */
    { 200, 128, 255 },  /* 4: 操作 — 紫 */
    { 255, 255, 128 },  /* 5: 极性 — 黄 */
    { 128, 255, 200 },  /* 6: 层级 — 青 */
    { 255, 160, 200 },  /* 7: 审判 — 粉 */
    { 180, 180, 180 },  /* 8: 产权 — 灰 */
};

uint32_t 类别颜色(uint8_t category) {
    if (category > 8) category = 8;
    return ((uint32_t)category_colors[category][0] << 16) |
           ((uint32_t)category_colors[category][1] << 8) |
           (uint32_t)category_colors[category][2];
}

/* ═══════════════════════════════════════════════════
 * 链 → 3D场景
 * ═══════════════════════════════════════════════════ */

int 链转场景(const chain_t* chain, uint32_t chain_id, mlil_scene_t* scene) {
    if (!chain || !scene) return -1;
    
    for (uint64_t i = 0; i < chain->count; i++) {
        if (scene->count >= MLIL_MAX_POINTS) break;
        
        const chain_node_t* node = &chain->nodes[i];
        mlil_point_t* p = &scene->points[scene->count];
        
        /* X: 表面 — 基于索引的极坐标展开 */
        float angle = (float)i / (float)(chain->count + 1) * 2.0f * M_PI;
        float radius = 30.0f + chain_id * 12.0f;
        p->x = 128.0f + radius * cosf(angle);
        p->y = 128.0f + radius * sinf(angle);
        
        /* Y: 结构 — 版本深度 */
        p->y = (float)node->version * 8.0f;
        
        /* Z: 层级 — 直接映射 */
        p->z = (float)(node->level % 8) * 15.0f;
        
        /* 颜色 */
        uint8_t cat = (uint8_t)(chain->category % 9);
        p->r = category_colors[cat][0];
        p->g = category_colors[cat][1];
        p->b = category_colors[cat][2];
        
        /* 标签 */
        strncpy(p->label, node->content, 31);
        p->label[31] = '\0';
        
        p->chain_id = chain_id;
        p->version = (uint32_t)node->version;
        p->level = node->level;
        
        scene->count++;
    }
    
    return (int)scene->count;
}

int 多链场景(const chain_t* chains, uint32_t count, mlil_scene_t* scene) {
    if (!chains || !scene) return -1;
    
    scene->count = 0;
    scene->view_angle_x = 30.0f;
    scene->view_angle_y = 45.0f;
    scene->zoom = 1.0f;
    strncpy(scene->title, "夫妻共同体 · 三维编码链", 127);
    
    for (uint32_t c = 0; c < count; c++) {
        链转场景(&chains[c], c, scene);
    }
    
    return (int)scene->count;
}

/* ═══════════════════════════════════════════════════
 * ASCII 投影视图
 * ═══════════════════════════════════════════════════ */

void 投影视图(const mlil_scene_t* scene) {
    if (!scene || scene->count == 0) return;
    
    /* 画布 */
    #define W   72
    #define H   24
    char canvas[H][W];
    
    /* 清空 */
    for (int y = 0; y < H; y++)
        for (int x = 0; x < W; x++)
            canvas[y][x] = ' ';
    
    /* 3D → 2D 投影 */
    float angle_y = scene->view_angle_y * M_PI / 180.0f;
    
    /* 颜色字符映射 */
    const char* level_chars = "元阶层代世界域场";
    
    for (uint32_t i = 0; i < scene->count; i++) {
        const mlil_point_t* p = &scene->points[i];
        
        /* 绕Y轴旋转 (俯视) */
        float rx = p->x * cosf(angle_y) - p->z * sinf(angle_y);
        float ry = p->y * 0.5f + p->x * sinf(angle_y) * 0.3f + p->z * cosf(angle_y) * 0.2f;
        
        /* 映射到画布 */
        int sx = (int)(rx / 4.0f + W / 2);
        int sy = H - 1 - (int)(ry / 20.0f + H / 4);
        
        if (sx >= 0 && sx < W && sy >= 0 && sy < H) {
            /* 用层级字符标记 */
            char ch = level_chars[p->level < 8 ? p->level : 7];
            
            /* 根节点用星 */
            if (p->version <= 1) ch = '*';
            
            canvas[sy][sx] = ch;
        }
    }
    
    /* 绘制坐标轴 */
    int axis_y = H - 2;
    for (int x = 2; x < W - 2; x++) canvas[axis_y][x] = '-';
    canvas[axis_y][W-2] = '>';
    int axis_x = 2;
    for (int y = 2; y < H - 2; y++) canvas[y][axis_x] = '|';
    canvas[1][axis_x] = 'Z';
    canvas[axis_y][W-1] = 'X';
    
    /* 输出 */
    printf("\n");
    printf("  ╔══════════════════════════════════════════════════════════════╗\n");
    printf("  ║  %-58s║\n", scene->title);
    printf("  ╠══════════════════════════════════════════════════════════════╣\n");
    
    for (int y = 0; y < H; y++) {
        printf("  ║ ");
        for (int x = 0; x < W; x++) {
            putchar(canvas[y][x]);
        }
        printf(" ║\n");
    }
    
    printf("  ╠══════════════════════════════════════════════════════════════╣\n");
    printf("  ║  ★根节点 元阶层代世界域场 = 8层                              ║\n");
    printf("  ║  X:表面 Y:结构(版本) Z:层级                                 ║\n");
    printf("  ╚══════════════════════════════════════════════════════════════╝\n");
    
    /* 点信息 */
    printf("\n  三维坐标 (前10节点):\n");
    uint32_t show = scene->count < 10 ? scene->count : 10;
    for (uint32_t i = 0; i < show; i++) {
        const mlil_point_t* p = &scene->points[i];
        const char* lvl[] = {"元","阶","层","代","世","界","域","场"};
        printf("    [%u] %-8s → (%5.1f, %5.1f, %5.1f) %s\n",
               p->version, p->label,
               p->x, p->y, p->z,
               lvl[p->level < 8 ? p->level : 7]);
    }
    
    if (scene->count > 10) {
        printf("    ... 共 %u 节点\n", scene->count);
    }
}

/* ═══════════════════════════════════════════════════
 * JSON导出 (供外部3D渲染)
 * ═══════════════════════════════════════════════════ */

void JSON导出(const mlil_scene_t* scene, const char* path) {
    if (!scene || !path) return;
    
    FILE* f = fopen(path, "w");
    if (!f) return;
    
    fprintf(f, "{\n");
    fprintf(f, "  \"schema\": \"njsg-mlil-3d\",\n");
    fprintf(f, "  \"version\": \"v0.3.0\",\n");
    fprintf(f, "  \"title\": \"%s\",\n", scene->title);
    fprintf(f, "  \"count\": %u,\n", scene->count);
    fprintf(f, "  \"view\": {\"ax\": %.1f, \"ay\": %.1f, \"zoom\": %.1f},\n",
            scene->view_angle_x, scene->view_angle_y, scene->zoom);
    fprintf(f, "  \"points\": [\n");
    
    for (uint32_t i = 0; i < scene->count; i++) {
        const mlil_point_t* p = &scene->points[i];
        fprintf(f, "    {\n");
        fprintf(f, "      \"label\": \"%s\",\n", p->label);
        fprintf(f, "      \"coord\": [%.2f, %.2f, %.2f],\n", p->x, p->y, p->z);
        fprintf(f, "      \"color\": [%u, %u, %u],\n", p->r, p->g, p->b);
        fprintf(f, "      \"version\": %u,\n", p->version);
        fprintf(f, "      \"level\": %u,\n", p->level);
        fprintf(f, "      \"chain\": %u\n", p->chain_id);
        fprintf(f, "    }%s\n", (i < scene->count - 1) ? "," : "");
    }
    
    fprintf(f, "  ]\n");
    fprintf(f, "}\n");
    
    fclose(f);
}
