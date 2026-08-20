/**
 * husk_font.c — 字体脱壳编译器实现
 *
 * 核心: 生米 → 我们的锅 → 煮熟 → 碗扔掉
 *
 * 编译后的字形完全由我们的编码驱动 · 旧外壳32位/64位已无意义
 */

#include "husk_font.h"
#include <string.h>
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 内部: 从位图提取相位
 *
 * 输入: 外部渲染好的位图 (黑白点阵)
 * 输出: 我们的相位记录 (可以用于重建字形的纯视觉信息)
 * ═══════════════════════════════════════════════════════════════ */

static void Extract_Phase(const uint8_t *bitmap, uint8_t w, uint8_t h,
                          glyph_phase_record_t *rec) {
    /* 步骤1: 轮廓提取 (简化版 · 实际可用更复杂的轮廓跟踪) */
    memset(rec->contour, 0, GLYPH_COORD_STEPS);

    uint8_t step = (w > GLYPH_COORD_STEPS) ? (w / GLYPH_COORD_STEPS) : 1;
    for (uint8_t i = 0; i < GLYPH_COORD_STEPS && i * step < w; i++) {
        /* 沿水平扫描 · 记录每列最上方的黑像素位置 */
        uint8_t top = FQ_FULL_8;
        for (uint8_t y = 0; y < h; y++) {
            if (bitmap[y * w + i * step]) {
                top = y;
                break;
            }
        }
        rec->contour[i] = top;
    }

    /* 步骤2: 密度方向统计 (8方向) */
    uint8_t dir_count[8] = {0};
    for (uint8_t y = 1; y < h - 1; y++) {
        for (uint8_t x = 1; x < w - 1; x++) {
            if (!bitmap[y * w + x]) continue;

            /* 8邻域方向统计 */
            if (bitmap[(y-1)*w+x]) dir_count[0]++;  /* 上 */
            if (bitmap[(y-1)*w+(x+1)]) dir_count[1]++;/* 右上 */
            if (bitmap[y*w+(x+1)]) dir_count[2]++;    /* 右 */
            if (bitmap[(y+1)*w+(x+1)]) dir_count[3]++;/* 右下 */
            if (bitmap[(y+1)*w+x]) dir_count[4]++;    /* 下 */
            if (bitmap[(y+1)*w+(x-1)]) dir_count[5]++;/* 左下 */
            if (bitmap[y*w+(x-1)]) dir_count[6]++;    /* 左 */
            if (bitmap[(y-1)*w+(x-1)]) dir_count[7]++;/* 左上 */
        }
    }

    /* 找主方向 */
    uint8_t max_dir = 0;
    uint8_t max_count = 0;
    for (uint8_t d = 0; d < 8; d++) {
        if (dir_count[d] > max_count) {
            max_count = dir_count[d];
            max_dir = d;
        }
    }
    rec->density = max_dir;

    /* 笔画方向位图: 哪些方向有显著笔画 */
    static const uint8_t POW2_8[8] = {1, 2, 4, 8, 16, 32, 64, 128};
    rec->stroke_dirs = 0;
    for (uint8_t d = 0; d < 8; d++) {
        if (dir_count[d] > (max_count / 4)) {
            rec->stroke_dirs |= POW2_8[d];
        }
    }

    /* 步骤3: 拓扑不变量 (简化: 计算封闭区域) */
    rec->topology = 0;
    for (uint8_t y = 1; y < h - 1 && rec->topology < 8; y++) {
        for (uint8_t x = 1; x < w - 1; x++) {
            /* 检查是否是被白像素包围的黑像素（简化封闭区域检测） */
            if (bitmap[y*w+x] && !bitmap[(y-1)*w+x] &&
                !bitmap[(y+1)*w+x] && !bitmap[y*w+(x-1)]) {
                rec->topology++;
            }
        }
    }
    if (rec->topology > 8) rec->topology = 8;
}

/* ═══════════════════════════════════════════════════════════════
 * 内部: 相位 → 我们的容器编码
 *
 * 把相位信息编码到我们的32位容器 · 完全脱离旧编码
 *
 * 编码分配 (满全法自有):
 *   our_code[31:29] = 感知家族 (5家族)
 *   our_code[28]    = 极性 (阴/阳)
 *   our_code[25:22] = 主密度方向 (8方向)
 *   our_code[21:18] = 拓扑不变量
 *   our_code[17:10] = 轮廓均值 (形状概要)
 *   our_code[9:0]   = 笔画密度 (1024级)
 * ═══════════════════════════════════════════════════════════════ */

static uint32_t Phase_To_OurCode(const glyph_phase_record_t *rec) {
    uint32_t code = 0;

    /* 轮廓均值 (形状概要) */
    uint16_t contour_sum = 0;
    for (uint8_t i = 0; i < GLYPH_COORD_STEPS; i++) {
        if (rec->contour[i] != FQ_FULL_8) contour_sum += rec->contour[i];
    }
    uint8_t contour_avg = (uint8_t)(contour_sum / GLYPH_COORD_STEPS);

    /* 笔画密度 (非黑像素占比) */
    uint16_t stroke_density = (uint16_t)(rec->stroke_dirs * 100 / 8);

    /* 极性: 拓扑数多为阳(1) · 少为阴(0) */
    uint8_t polarity = (rec->topology > 3) ? 1 : 0;

    /* 拼接 — 用乘法替代位移 */
    code |= (2 * FQ_POW2_29);            /* 家族: COUPLE(2) · 字形是耦合 */
    code |= (polarity * FQ_POW2_28);     /* 极性 */
    code |= (rec->density * FQ_POW2_25); /* 密度方向 */
    code |= (rec->topology * FQ_POW2_18);/* 拓扑 */
    code |= (contour_avg * FQ_POW2_10);  /* 轮廓均值 */
    code |= (stroke_density & FQ_MASK_10);/* 笔画密度 */

    return code;
}

/* ═══════════════════════════════════════════════════════════════
 * ═══════════════════════════════════════════════════════════════ */

void husk_font_init(husk_font_compiler_t *comp, mlll_whole_t *box,
                    cylinder_coord_t base) {
    memset(comp, 0, sizeof(*comp));
    comp->magic      = HUSK_FONT_MAGIC;
    comp->target_box = box;
    comp->base_coord = base;
}

/* ═══════════════════════════════════════════════════════════════
 * 编译单个字符
 *
 * 流程:
 *   1. 接收旧编码 + 外渲位图 (生米)
 *   2. 提取相位 (去壳 · 取肉)
 *   3. 分配我们的编码 (煮熟)
 *   4. 装入我们的锅 (mlll字车)
 *   5. 旧编码仅记录溯源 · 不再使用 (碗已空)
 * ═══════════════════════════════════════════════════════════════ */

int husk_font_compile_one(husk_font_compiler_t *comp,
                          uint32_t old_code,
                          const uint8_t *bitmap,
                          uint8_t width, uint8_t height) {
    if (!comp || comp->magic != HUSK_FONT_MAGIC) return -1;
    if (comp->glyph_count >= 254) return -1;

    /* 获取当前记录槽 */
    glyph_phase_record_t *rec = &comp->records[comp->glyph_count];

    /* 步骤1: 保存旧外壳 (仅溯源 · 之后不再使用) */
    rec->old_shell       = old_code;
    rec->shell_was_32bit = (old_code > FQ_FULL_16) ? 1 : 0;

    /* 步骤2: 提取相位 (去壳 · 取肉 · 肉是字形 · 壳是编码) */
    if (bitmap && width > 0 && height > 0) {
        Extract_Phase(bitmap, width, height, rec);
    }

    /* 步骤3: 相位 → 我们的容器编码 (煮熟 · 变成我们的) */
    rec->our_code = Phase_To_OurCode(rec);

    /* 步骤4: 装入我们的锅 (mlll 254+2 字车) */
    int slot = mlll_load(comp->target_box, rec->our_code, CUNIT_32);

    if (slot < 0 || slot >= 254) {
        /* 溢出: 字车满了 · 截断到锚点 */
        return -2;
    }

    /* 步骤5: 圆筒三维坐标 · 字形放置到空间 */
    cylinder_coord_t coord;
    coord.x = (uint8_t)(comp->base_coord.x + (comp->glyph_count % 16));
    coord.y = (uint8_t)(comp->base_coord.y + (comp->glyph_count / 16));
    coord.z = comp->base_coord.z;

    cylinder_attributes_t attrs;
    memset(&attrs, 0, sizeof(attrs));
    attrs.axis   = AXIS_X_MLIL;     /* 表面属性 = 毛利兰 = 字形外观 */
    attrs.bone   = rec->density;    /* 主方向 = 骨 */
    attrs.head   = rec->topology;   /* 拓扑 = 头 */
    attrs.body   = rec->stroke_dirs;/* 笔画 = 体 */

    cylinder_place(coord.x, coord.y, coord.z, rec->our_code, &attrs);

    comp->glyph_count++;
    return slot;
}

/* ═══════════════════════════════════════════════════════════════
 * 批量编译
 * ═══════════════════════════════════════════════════════════════ */

int husk_font_compile_batch(husk_font_compiler_t *comp,
                            const uint32_t *old_codes,
                            const uint8_t **bitmaps,
                            const uint8_t *widths,
                            const uint8_t *heights,
                            int count) {
    if (!comp || !old_codes) return -1;

    int success = 0;
    for (int i = 0; i < count; i++) {
        const uint8_t *bmp = bitmaps ? bitmaps[i] : 0;
        uint8_t w = widths ? widths[i] : 0;
        uint8_t h = heights ? heights[i] : 0;

        int slot = husk_font_compile_one(comp, old_codes[i], bmp, w, h);
        if (slot >= 0) success++;
    }

    comp->batch_count++;
    return success;
}

/* ═══════════════════════════════════════════════════════════════
 * 翻译: 旧编码 → 我们的容器编码
 * ═══════════════════════════════════════════════════════════════ */

uint32_t husk_font_translate(husk_font_compiler_t *comp, uint32_t old_code) {
    if (!comp || comp->magic != HUSK_FONT_MAGIC) return 0;

    /* 线性查找 (可用哈希加速但满全法不需要 · 因为外壳不重要) */
    for (uint32_t i = 0; i < comp->glyph_count; i++) {
        if (comp->records[i].old_shell == old_code) {
            /* 返回我们的编码 · 旧外壳已脱 */
            return comp->records[i].our_code;
        }
    }

    return 0;  /* 未编译 · 生米 */
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

uint32_t husk_font_stats(husk_font_compiler_t *comp, const char *which) {
    if (!comp) return 0;
    if (which[0] == 'g') return comp->glyph_count;  /* glyphs */
    if (which[0] == 'b') return comp->batch_count;  /* batches */
    if (which[0] == 'r') return (254 - comp->glyph_count); /* remaining */
    return 0;
}
