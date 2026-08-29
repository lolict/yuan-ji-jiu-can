/**
 * radix_star.h — 部首星图算法
 *
 * 核心命题:
 *   每个部首是一个星(容器), 以它造的字是环绕它的星群。
 *   星图 = 部首容器 + 它所拥有的字符星群 + 容器间的关系连线。
 *
 * 三部端到端演示:
 *   水部(氵): 水的容器, 所有带水旁的字归入此容器
 *   火部(灬): 火的容器, 所有带火旁/四点底的字归入此容器
 *   言部(讠): 言语的容器, 所有带言字旁的字归入此容器
 *
 * 这不是外来框架的"属性-值"结构, 是容器本体论:
 *   部首容器"拥有"它的字, 字是容器的分身(公倍数),
 *   部首是容器中心的公约数锚点。
 */

#ifndef NJSG_RADIX_STAR_H
#define NJSG_RADIX_STAR_H

#include <stdint.h>

/* ═══ 部首星定义 ═══ */

#define RS_MAX_STARS        32      /* 一个部首下最多挂多少字 */
#define RS_MAX_ACTIVE       8       /* 同时激活多少部首 */

/* 星群中的一个字符(环绕星) */
typedef struct {
    uint16_t    ucs2;               /* 字符编码 */
    uint8_t     shengmu;            /* 声母 */
    uint8_t     yunmu;              /* 韵母 */
    uint8_t     tone;               /* 声调 */
    uint8_t     family;             /* 感知家族 */
    uint8_t     structure;          /* 字形结构 */
    uint8_t     residue;            /* 剩余笔画 (去掉部首后) */
} rs_char_star_t;

/* 部首容器 (中心星) */
typedef struct {
    uint16_t        radical_ucs2;       /* 部首 UCS-2 */
    uint8_t         radical_index;      /* 康熙部首序号 */
    uint8_t         strokes;           /* 部首頁数 */
    uint32_t        container_code;    /* 32位容器编码 */
    const char     *meaning;           /* 部首含义 */
    rs_char_star_t  stars[RS_MAX_STARS]; /* 环绕它的字符星群 */
    uint8_t         star_count;
} rs_radical_center_t;

/* 星图 */
typedef struct {
    rs_radical_center_t centers[RS_MAX_ACTIVE];
    uint8_t             count;
} rs_star_map_t;

/* ═══ API ═══ */

void    rs_init(rs_star_map_t *map);
int     rs_register_center(rs_star_map_t *map, uint16_t radical_ucs2,
                            uint8_t radical_idx, uint8_t strokes,
                            const char *meaning);
int     rs_add_star(rs_star_map_t *map, uint8_t center_idx,
                     uint16_t ucs2, uint8_t sm, uint8_t ym,
                     uint8_t tone, uint8_t family, uint8_t struct_id);
void    rs_report(const rs_star_map_t *map);
void    rs_visualize(const rs_star_map_t *map);
void    rs_cross_relate(rs_star_map_t *map);

#endif /* NJSG_RADIX_STAR_H */
