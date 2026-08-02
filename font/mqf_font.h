/* 自动生成的 SDF 字形表 — 由 encoding/gen_sdf_font.py 生成 */
#ifndef MQF_FONT_H
#define MQF_FONT_H

#include <stdint.h>

#define MQF_FONT_TILE      12
#define MQF_FONT_ATLAS_W   224
#define MQF_FONT_ATLAS_H   224
#define MQF_FONT_PH_COUNT  1309

typedef struct {
    uint8_t atlas_ix, col, row;
    uint8_t adv;
    int8_t  bx, by;
    uint8_t pad;
};

/* extern const MQF_FONT_ALIGN MqfGlyph MQF_FONT_TABLE[1309]; */
#endif /* MQF_FONT_H */
