/**
 * hanzi_geometry.c — 汉字几何容器实现
 *
 * 汉字高维空间的统一地址系统:
 *   部首进制 × 结构进制 × 笔画进制 = 唯一地址
 *
 * 边界: 214部首 × 8结构 × 50笔画 = 85600 可寻址空间
 * 实际: 常用字约3500个, 覆盖日常使用
 */

#include "hanzi_geometry.h"
#include "global_interlock.h"
#include <string.h>
#include "symbol_chain.h"

/* 结构名称 */
static const char *STRUCT_NAMES[] = {
    "?", "左右", "上下", "全包围", "半包围", "左中右", "上中下", "品字", "独体"
};

/* 笔画名称 */
static const char *STROKE_NAMES[] = {"横","竖","撇","捺","折"};

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void hanzi_geom_init(hanzi_geometry_t *hg) {
    memset(hg, 0, sizeof(*hg));
    hg->magic = HANZI_GEOMETRY_MAGIC;
    hg->version = 1;
    hg->entry_count = 0;
    hg->stroke_type_count = 5;
    hg->structure_count = STRUCT_SINGLE;
    hg->radical_count = MAX_RADICALS;
    hg->border_total = MAX_RADICALS * MAX_STRUCTURES * MAX_STROKES;
}

int hanzi_geom_set_radicals(hanzi_geometry_t *hg, uint16_t count) {
    if (!hg || hg->magic != HANZI_GEOMETRY_MAGIC) return -1;
    hg->radical_count = count;
    hg->border_total = count * hg->structure_count * MAX_STROKES;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 添加汉字条目
 * ═══════════════════════════════════════════════════════════════ */

int hanzi_geom_add_entry(hanzi_geometry_t *hg, uint16_t char_code,
                           uint16_t radical1, uint16_t radical2,
                           structure_t structure, uint8_t stroke_count,
                           const uint8_t *strokes) {
    if (!hg || hg->magic != HANZI_GEOMETRY_MAGIC) return -1;
    if (hg->entry_count >= MAX_CHAR_ENTRIES) return -1;
    if (stroke_count > MAX_STROKES) return -1;
    
    hanzi_geom_entry_t *e = &hg->entries[hg->entry_count];
    memset(e, 0, sizeof(*e));
    
    e->char_code = char_code;
    e->radical_primary = radical1;
    e->radical_secondary = radical2;
    e->structure = (uint8_t)structure;
    e->stroke_count = stroke_count;
    
    if (strokes && stroke_count > 0) {
        memcpy(e->stroke_sequence, strokes, stroke_count);
    }
    
    /* 紧凑位编码 */
    uint32_t bits = 0;
    uint8_t pos = 0;
    
    /* 部首: 8 bits (0-213) */
    bits |= ((uint32_t)(radical1 & 0xFF)) << pos;
    pos += 8;
    
    /* 结构: 3 bits (1-8) */
    bits |= ((uint32_t)(structure & 0x07)) << pos;
    pos += 3;
    
    /* 笔画数: 6 bits (1-50) */
    bits |= ((uint32_t)(stroke_count & 0x3F)) << pos;
    pos += 6;
    
    /* 笔顺: 3 bits × stroke_count */
    for (uint8_t i = 0; i < stroke_count; i++) {
        bits |= ((uint32_t)(strokes[i] & 0x07)) << pos;
        pos += 3;
    }
    
    e->packed_bits = bits;
    e->bit_length = pos;
    
    hg->entry_count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 获取紧凑编码
 * ═══════════════════════════════════════════════════════════════ */

uint32_t hanzi_geom_packed(const hanzi_geometry_t *hg, uint16_t idx) {
    if (!hg || hg->magic != HANZI_GEOMETRY_MAGIC) return 0;
    if (idx >= hg->entry_count) return 0;
    return hg->entries[idx].packed_bits;
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

void hanzi_geom_stats(const hanzi_geometry_t *hg, FILE *out) {
    if (!hg || hg->magic != HANZI_GEOMETRY_MAGIC) return;
    
    fprintf(out, "─── 汉字几何容器 ───\n");
    fprintf(out, "条目数: %d / %d\n", hg->entry_count, MAX_CHAR_ENTRIES);
    fprintf(out, "部首进制: %d 符号\n", hg->radical_count);
    fprintf(out, "结构进制: %d 符号\n", hg->structure_count);
    fprintf(out, "笔画进制: %d 符号\n", hg->stroke_type_count);
    fprintf(out, "边界: %u 可寻址\n", hg->border_total);
    
    if (hg->entry_count > 0) {
        fprintf(out, "\n─── 汉字条目 ───\n");
        for (uint16_t i = 0; i < hg->entry_count; i++) {
            const hanzi_geom_entry_t *e = &hg->entries[i];
            fprintf(out, "  [%2d] U+%04X 部首%d 结构%s 画%d %dbit\n",
                    i, e->char_code, e->radical_primary,
                    STRUCT_NAMES[e->structure], e->stroke_count, e->bit_length);
        }
    }
}
