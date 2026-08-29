/**
 * radix_star.c — 部首星图算法实现
 *
 * 以康熙部首表为锚, 将1257字按部首归入星群。
 * 核心不是分类, 是"容器拥有"的关系:
 *   部首容器是公约数锚点, 字是公倍数分身。
 */

#include "radix_star.h"
#include "radicals.h"
#include "hanzi_table.h"
#include <stdio.h>
#include <string.h>

void rs_init(rs_star_map_t *map) {
    memset(map, 0, sizeof(*map));
}

int rs_register_center(rs_star_map_t *map, uint16_t radical_ucs2,
                        uint8_t radical_idx, uint8_t strokes,
                        const char *meaning) {
    if (map->count >= RS_MAX_ACTIVE) return -1;
    
    rs_radical_center_t *c = &map->centers[map->count];
    c->radical_ucs2 = radical_ucs2;
    c->radical_index = radical_idx;
    c->strokes = strokes;
    c->meaning = meaning ? meaning : "";
    c->star_count = 0;
    
    map->count++;
    return 0;
}

int rs_add_star(rs_star_map_t *map, uint8_t center_idx,
                 uint16_t ucs2, uint8_t sm, uint8_t ym,
                 uint8_t tone, uint8_t family, uint8_t struct_id) {
    if (center_idx >= map->count) return -1;
    
    rs_radical_center_t *c = &map->centers[center_idx];
    if (c->star_count >= RS_MAX_STARS) return -1;
    
    rs_char_star_t *s = &c->stars[c->star_count];
    s->ucs2 = ucs2;
    s->shengmu = sm;
    s->yunmu = ym;
    s->tone = tone;
    s->family = family;
    s->structure = struct_id;
    s->residue = 0;  /* 后续计算 */
    
    c->star_count++;
    return 0;
}

/* ── 报告 ── */
void rs_report(const rs_star_map_t *map) {
    for (uint8_t i = 0; i < map->count; i++) {
        const rs_radical_center_t *c = &map->centers[i];
        printf("★ 部首 #%d | U+%04X | %d画 | %s | 星群:%d\n",
               c->radical_index, c->radical_ucs2, c->strokes,
               c->meaning, c->star_count);
        
        for (uint8_t j = 0; j < c->star_count; j++) {
            const rs_char_star_t *s = &c->stars[j];
            printf("    └─ U+%04X | 声:%d 韵:%d 调:%d | 家族:%d 结构:%d\n",
                   s->ucs2, s->shengmu, s->yunmu, s->tone,
                   s->family, s->structure);
        }
        printf("\n");
    }
}

/* ── ASCII 可视化 ── */
void rs_visualize(const rs_star_map_t *map) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           部 首 星 图 · 容 器 拥 有 树          ║\n");
    printf("╚══════════════════════════════════════════════════╝\n\n");
    
    for (uint8_t i = 0; i < map->count; i++) {
        const rs_radical_center_t *c = &map->centers[i];
        
        /* 中心星 (部首容器) */
        printf("        ┌─────────────┐\n");
        printf("        │  ★ %04X     │ ← 部首#%d (%d画)\n",
               c->radical_ucs2, c->radical_index, c->strokes);
        printf("        │   %s\n", c->meaning);
        printf("        └──────┬──────┘\n");
        printf("               │\n");
        
        /* 星群 (该部首下的字) */
        if (c->star_count == 0) {
            printf("          (无星群)\n\n");
            continue;
        }
        
        printf("          ├");
        for (uint8_t j = 0; j < c->star_count; j++) {
            if (j > 0 && j % 6 == 0) printf("\n          ├");
            printf("──[%04X]", c->stars[j].ucs2);
        }
        printf("\n\n");
    }
}

/* ── 跨容器关系 ── */
void rs_cross_relate(rs_star_map_t *map) {
    printf("─── 跨容器关系 (星群之间的连线) ───\n\n");
    
    for (uint8_t i = 0; i < map->count; i++) {
        const rs_radical_center_t *a = &map->centers[i];
        for (uint8_t j = i + 1; j < map->count; j++) {
            const rs_radical_center_t *b = &map->centers[j];
            
            /* 检查是否有共享声母 (发音位置接近 = 本质相关) */
            uint32_t shared = 0;
            for (uint8_t si = 0; si < a->star_count; si++) {
                for (uint8_t sj = 0; sj < b->star_count; sj++) {
                    if (a->stars[si].shengmu == b->stars[sj].shengmu) {
                        shared++;
                        printf("  水[%04X] 声=%d ←→ 火/言[%04X] 声=%d (声母共鸣)\n",
                               a->stars[si].ucs2, a->stars[si].shengmu,
                               b->stars[sj].ucs2, b->stars[sj].shengmu);
                    }
                }
            }
            
            if (shared > 0) {
                printf("  ★ %04X (%s) ←→ %04X (%s): 共享声母 %d 条\n\n",
                       a->radical_ucs2, a->meaning,
                       b->radical_ucs2, b->meaning, shared);
            }
        }
    }
}
