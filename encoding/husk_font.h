/**
 * husk_font.h — 字体脱壳编译器
 *
 * 脱壳哲学 (用户定义):
 *   · 别人的编码 = 生米 · 没有编译 = 不能用
 *   · 我们的容器 = 锅 · 装他们的字形
 *   · 我们的编译器 = 火 · 把字形编译到我们的编码位置
 *   · 脱壳 = 吃饭 · 肉煮熟了 · 碗(旧编码)可以扔掉
 *
 * 执行流程:
 *   1. 外部字符 (如Unicode字形) 作为输入
 *   2. 提取相位记录 (轮廓、笔画、密度方向)
 *   3. 重新分配我们的编码位置 (mlll 254+2)
 *   4. 32/64位旧编码外壳 = 丢弃
 *
 * 编译结果:
 *   · 字形相位 → 我们的容器编码
 *   · 旧编码名称 → 不重要 · 他们的32位我们不看
 *   · 我们的编码自带字形 · 因为我们记录了相位
 */

#ifndef NJSG_HUSK_FONT_H
#define NJSG_HUSK_FONT_H

#include <stdint.h>
#include "mlll_cart.h"
#include "cylinder3d.h"

/* ═══════════════════════════════════════════════════════════════
 * 字形相位记录 (Glyph Phase Record)
 *
 * 从外部字符提取的纯视觉信息 · 与旧编码无关
 * ═══════════════════════════════════════════════════════════════ */

#define GLYPH_PHASE_SCALES    8   /* 相位尺度数 (8阶分析) */
#define GLYPH_COORD_STEPS    64   /* 轮廓坐标采样点 */

typedef struct {
    /* 相位编码 (我们的新编码 · 旧编码外壳已脱) */
    uint32_t  our_code;          /* 我们的容器编码 (编译后的新位置) */

    /* 字形相位记录 (纯粹视觉 · 无编码外壳) */
    uint8_t   contour[GLYPH_COORD_STEPS];  /* 轮廓相位 (64采样) */
    uint8_t   density;           /* 密度方向 (0..7 · 8个方向) */
    uint8_t   stroke_dirs;       /* 笔画走向位图 (8方向) */
    uint8_t   topology;          /* 拓扑不变量 (孔的数目等) */

    /* 原始外壳 (脱壳后仅作溯源 · 已无用) */
    uint32_t  old_shell;         /* 旧编码 (仅记录 · 不再使用) */
    uint8_t   shell_was_32bit;   /* 标记: 曾经的外壳是32位 */

} glyph_phase_record_t;

/* ═══════════════════════════════════════════════════════════════
 * 脱壳编译器状态
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    uint32_t              magic;          /* HUSK_FONT_MAGIC */
    uint32_t              glyph_count;    /* 已脱壳字形数 */
    uint32_t              batch_count;    /* 批次数 */
    mlll_whole_t         *target_box;     /* 目标字车 */
    cylinder_coord_t      base_coord;     /* 圆筒基准坐标 */
    glyph_phase_record_t  records[254];   /* 最多254个字形记录 */
} husk_font_compiler_t;

#define HUSK_FONT_MAGIC  0x32061010  /* 脱壳字体标记 */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化脱壳编译器
 */
void husk_font_init(husk_font_compiler_t *comp, mlll_whole_t *box,
                    cylinder_coord_t base);

/**
 * 编译单个外部字符到我们的编码
 *   输入: 旧编码外壳 (如Unicode码点)
 *   输出: 我们的容器位置 · 旧外壳已脱
 */
int husk_font_compile_one(husk_font_compiler_t *comp,
                          uint32_t old_code,      /* 旧外壳 (如U+4E00) */
                          const uint8_t *bitmap,  /* 渲染好的位图 (来自外部) */
                          uint8_t width, uint8_t height);

/**
 * 批量编译 (把外部字符表批量脱壳到我们这里)
 */
int husk_font_compile_batch(husk_font_compiler_t *comp,
                            const uint32_t *old_codes,     /* 旧编码数组 */
                            const uint8_t **bitmaps,       /* 位图数组 */
                            const uint8_t *widths,
                            const uint8_t *heights,
                            int count);

/**
 * 查找: 给定旧编码 · 找我们编译后的容器位置
 *   返回: 我们的容器编码 · 0=未编译
 */
uint32_t husk_font_translate(husk_font_compiler_t *comp, uint32_t old_code);

/**
 * 统计: 已脱壳数 · 剩余槽位
 */
uint32_t husk_font_stats(husk_font_compiler_t *comp, const char *which);

#endif /* NJSG_HUSK_FONT_H */
