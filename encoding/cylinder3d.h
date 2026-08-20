/**
 * cylinder3d.h — 圆筒三维坐标编码系统
 *
 * 映射 (满全法动漫化身):
 *   毛利兰 (Mouri Ran)  → X轴 · 表面属性 · 模/面/名 · 格式/编码 (.mlil)
 *   工藤新一 (Kudou Shinichi) → Y轴 · 结构属性 · 骨/头/体 · gtxone回归/骨骼执行层
 *   灰原哀 (Haibara Ai)  → Z轴 · 横向属性 · 行页/行业 · 平铺展开
 *
 * 三维静态绝对位置寻址:
 *   每个容器在 圆筒坐标系 (r, θ, z) 或 (x, y, z) 中有一个固定位置
 *   通过坐标直接寻址 · 不需要搜索 · 不需要哈希
 *
 * 圆筒 = 容器的物理容器
 *   旋转 (θ) = 感知家族的极性分化 (容器/边界/夫妻/网状/中性)
 *   高度 (z) = 层级叠加 (L0/L1/L2/L3)
 *   半径 (r) = 进制 (越靠近圆心进制越小 · 越远离进制越大)
 */

#ifndef NJSG_CYLINDER3D_H
#define NJSG_CYLINDER3D_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 三维坐标 (静态绝对位置)
 *
 * 一个容器当且仅当有一个坐标
 *   寻址 = 记坐标 · 直接去那里拿 · 不搜索
 *   没有碰撞 · 没有蒙面人
 * ═══════════════════════════════════════════════════════════════ */

#define COORDINX_MASK   0xFF   /* X轴 256 格 */
#define COORDINY_MASK   0xFF   /* Y轴 256 格 */
#define COORDINZ_MASK   0x0F   /* Z轴 16 级 (L0-L3 + 深度) */

typedef struct {
    uint8_t  x;   /* 表面位置 (0..255) */
    uint8_t  y;   /* 结构位置 (0..255) */
    uint8_t  z;   /* 层级位置 (0..15) */
} cylinder_coord_t;

/* ═══════════════════════════════════════════════════════════════
 * 三轴化身
 *
 * X轴 (毛利兰 · 表面): 模/面/名
 *   · 模: 模板 · 模式 · 范型
 *   · 面: 表面 · 对外呈现
 *   · 名: 名称 · 句柄 · 入口
 *
 * Y轴 (工藤新一 · 结构): 骨/头/体
 *   · 骨: 结构骨架 · AST/DAG/纤维丛
 *   · 头: 入口 · 起点 · 触发点
 *   · 体: 内含 · 负载 · 有效载荷
 *
 * Z轴 (灰原哀 · 横向): 行页/行业
 *   · 行: 行号 · 执行流中的位置
 *   · 页: 页号 · 内存分页
 *   · 行页/行业: 横向扩展 · 跨容器调用链
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    AXIS_X_MLIL  = 0x01,   /* 毛利兰 X 轴 · 表面 · MOURI */
    AXIS_Y_KND   = 0x02,   /* 工藤新一 Y 轴 · 结构 · KUDOU */
    AXIS_Z_HA    = 0x04,   /* 灰原哀 Z 轴 · 横向 · HAIBARA */
} cylinder_axis_t;

/* 三轴属性 */
typedef struct {
    cylinder_axis_t axis;
    uint8_t         modulus;    /* X: 模 */
    uint8_t         face;       /* X: 面 */
    uint8_t         name;       /* X: 名 */
    uint8_t         bone;       /* Y: 骨 */
    uint8_t         head;       /* Y: 头 */
    uint8_t         body;       /* Y: 体 */
    uint8_t         row;        /* Z: 行 */
    uint8_t         page;       /* Z: 页 */
    uint8_t         trade;      /* Z: 行业 */
} cylinder_attributes_t;

/* ═══════════════════════════════════════════════════════════════
 * 圆筒容器
 *
 * 每个容器 = 圆筒中的一个位置 + 该位置上的兵马
 *   兵马 = 实际编码的容器 (来自 container_pipeline 的 32 位编码)
 * ═══════════════════════════════════════════════════════════════ */

#define CYLINDER_MAX_CONTAINERS  4096
#define CYLINDER_MAGIC           0xC7110003

typedef struct {
    uint32_t          magic;          /* CYLINDER_MAGIC */
    uint32_t          container_id;   /* 与 container_pipeline 共享的 id */
    uint32_t          container_code; /* 32 位容器编码 */
    cylinder_coord_t  coord;          /* 静态绝对位置 */
    cylinder_attributes_t attrs;      /* 三轴属性 */
} cylinder_entry_t;

/* ── API ── */

/**
 * 按坐标放置容器
 *   返回 0=成功, -1=圆筒满, -2=坐标已占
 */
int cylinder_place(uint8_t x, uint8_t y, uint8_t z,
                   uint32_t code, const cylinder_attributes_t *attrs);

/**
 * 按坐标直接寻址 (绝对位置 O(1))
 */
cylinder_entry_t *cylinder_find(uint8_t x, uint8_t y, uint8_t z);

/**
 * 通过容器编码反查坐标
 */
cylinder_entry_t *cylinder_find_by_code(uint32_t code);

/**
 * 容器总数
 */
uint32_t cylinder_count(void);

#endif /* NJSG_CYLINDER3D_H */
