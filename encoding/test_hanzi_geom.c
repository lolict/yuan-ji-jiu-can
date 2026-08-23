#include <stdio.h>
#include <string.h>
#include "hanzi_geometry.h"

int main(void) {
    hanzi_geometry_t hg;
    hanzi_geom_init(&hg);
    
    printf("═══ 汉字几何容器 — 演示 ═══\n\n");
    printf("边界: %u 可寻址空间\n\n", hg.border_total);
    
    /* 添加常用汉字 */
    /* 明: 日(72) + 月(74) + 左右 + 8画 */
    uint8_t ming_strokes[] = {2,1,2,1,1,1,1,1};  /* 竖-横折-横-横-撇-横折钩-横-横 */
    hanzi_geom_add_entry(&hg, 0x660E, 72, 74, STRUCT_LEFT_RIGHT, 8, ming_strokes);
    
    /* 林: 木(75) + 木(75) + 左右 + 8画 */
    uint8_t lin_strokes[] = {1,2,3,4,1,2,3,4};  /* 横-竖-撇-捺-横-竖-撇-捺 */
    hanzi_geom_add_entry(&hg, 0x6797, 75, 75, STRUCT_LEFT_RIGHT, 8, lin_strokes);
    
    /* 森: 木(75) + 木(75) + 品字 + 12画 */
    uint8_t sen_strokes[] = {1,2,3,4,1,2,3,4,1,2,3,4};
    hanzi_geom_add_entry(&hg, 0x68EE, 75, 75, STRUCT_PINZI, 12, sen_strokes);
    
    /* 国: 囗(31) + 玉(96) + 全包围 + 8画 */
    uint8_t guo_strokes[] = {2,1,2,1,1,1,1,1};
    hanzi_geom_add_entry(&hg, 0x56FD, 31, 96, STRUCT_SURROUND, 8, guo_strokes);
    
    /* 人: 人(9) + 独体 + 2画 */
    uint8_t ren_strokes[] = {3,4};  /* 撇-捺 */
    hanzi_geom_add_entry(&hg, 0x4EBA, 9, 0, STRUCT_SINGLE, 2, ren_strokes);
    
    /* 显示结果 */
    const char *struct_names[] = {"?","左右","上下","全包围","半包围","左中右","上中下","品字","独体"};
    printf("─── 汉字编码 ───\n");
    for (uint16_t i = 0; i < hg.entry_count; i++) {
        hanzi_geom_entry_t *e = &hg.entries[i];
        printf("  U+%04X: 部首%d + 结构%s + %d画 → %dbit (0x%06X)\n",
               e->char_code, e->radical_primary,
               struct_names[e->structure], e->stroke_count,
               e->bit_length, e->packed_bits);
    }
    
    printf("\n");
    hanzi_geom_stats(&hg, stdout);
    
    printf("\n═══ 演示完成 ═══\n");
    return 0;
}
