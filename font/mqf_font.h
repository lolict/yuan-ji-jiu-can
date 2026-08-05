/* 自动生成的 SDF 字形表 — 由 encoding/gen_sdf_font.py 生成 */
#ifndef NJSG_FONT_H
#define NJSG_FONT_H

#include <stdint.h>

#define NJSG_FONT_TILE      12
#define NJSG_FONT_ATLAS_W   224
#define NJSG_FONT_ATLAS_H   224
#define NJSG_FONT_PH_COUNT  1309

typedef struct {
    uint8_t atlas_ix, col, row;
    uint8_t adv;
    int8_t  bx, by;
    uint8_t pad;
};

/* extern const NJSG_FONT_ALIGN MqfGlyph NJSG_FONT_TABLE[1309]; */
#endif /* NJSG_FONT_H */
