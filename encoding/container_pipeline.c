/**
 * container_pipeline.c — 容器流水线实现
 *
 * 完整通路:
 *   UTF-8 → UCS-2 → 拼音查询 → 感知场路由 → 容器化 → 流水线排水
 */

#include "container_pipeline.h"
#include "mccp_encdec.h"
#include "shengmu.h"
#include "yunmu.h"
#include <string.h>

/* ═══════════════════════════════════════════════════════════════
 * UTF-8 → UCS-2 转换 (不用 zlib/iconv · 自行实现)
 *
 * UTF-8规则:
 *   0xxxxxxx                    → 1字节 · ASCII
 *   110xxxxx 10xxxxxx           → 2字节
 *   1110xxxx 10xxxxxx 10xxxxxx  → 3字节 (汉字在此)
 *   11110xxx 10xxxxxx 10xxxxxx 10xxxxxx → 4字节
 * ═══════════════════════════════════════════════════════════════ */

static int utf8_to_ucs2(const char *utf8, uint16_t *out_ucs2) {
    if (!utf8 || !out_ucs2) return -1;

    unsigned char c0 = (unsigned char)utf8[0];

    if (c0 < 0x80) {
        /* ASCII */
        *out_ucs2 = c0;
        return 1;  /* 消耗1字节 */
    } else if ((c0 & 0xE0) == 0xC0) {
        /* 2字节 */
        unsigned char c1 = (unsigned char)utf8[1];
        if ((c1 & 0xC0) != 0x80) return -1;
        *out_ucs2 = ((c0 & 0x1F) << 6) | (c1 & 0x3F);
        return 2;
    } else if ((c0 & 0xF0) == 0xE0) {
        /* 3字节 · 汉字最常见 */
        unsigned char c1 = (unsigned char)utf8[1];
        unsigned char c2 = (unsigned char)utf8[2];
        if ((c1 & 0xC0) != 0x80) return -1;
        if ((c2 & 0xC0) != 0x80) return -1;
        *out_ucs2 = ((c0 & 0x0F) << 12) | ((c1 & 0x3F) << 6) | (c2 & 0x3F);
        return 3;
    } else if ((c0 & 0xF8) == 0xF0) {
        /* 4字节 · UTF-16代理对 · 暂不支持 */
        return -1;
    }
    return -1;  /* 非法UTF-8 */
}

/* ═══════════════════════════════════════════════════════════════
 * UCS-2 → 拼音查询
 *
 * 使用 hanzi_table 的公开 API 查找, 不直接访问私有家族门表
 * ═══════════════════════════════════════════════════════════════ */

static int ucs2_to_pinyin(uint16_t ucs2, uint8_t *sm_idx, uint8_t *ym_idx, uint8_t *tone) {
    return hanzi_ucs2_to_pinyin(ucs2, sm_idx, ym_idx, tone);
}

/* ═══════════════════════════════════════════════════════════════
 * 主通路: UTF-8汉字 → 容器编码
 * ═══════════════════════════════════════════════════════════════ */

pipeline_result_t container_pipeline_process_utf8(const char *utf8) {
    pipeline_result_t result;
    memset(&result, 0, sizeof(result));
    result.status = PATH_OK;

    /* [1] UTF-8 → UCS-2 */
    uint16_t ucs2;
    int consumed = utf8_to_ucs2(utf8, &ucs2);
    if (consumed < 0) {
        result.status = PATH_ERR_INVALID_UTF8;
        return result;
    }
    result.ucs2 = ucs2;

    /* [2] UCS-2 → 拼音查询 (逐一镜像 · 不用哈希) */
    uint8_t sm_idx, ym_idx, tone;
    if (ucs2_to_pinyin(ucs2, &sm_idx, &ym_idx, &tone) < 0) {
        result.status = PATH_ERR_UNKNOWN_HANZI;
        return result;
    }

    /* [3] 感知场路由 (完整流程) */
    uint32_t code = hanzi_route(ucs2, sm_idx, ym_idx, tone);
    if (code == 0) {
        result.status = PATH_ERR_JUDGE_REJECT;
        return result;
    }

    result.container_code = code;
    result.family = CONTAINER_GET_FAMILY(code);
    result.polarity = CONTAINER_GET_POLARITY(code);

    return result;
}

/* ═══════════════════════════════════════════════════════════════
 * 直接通路: 已知 UCS-2 + 拼音
 * ═══════════════════════════════════════════════════════════════ */

pipeline_result_t container_pipeline_route_direct(uint16_t ucs2,
                                                  uint8_t sm_idx,
                                                  uint8_t ym_idx,
                                                  uint8_t tone) {
    pipeline_result_t result;
    memset(&result, 0, sizeof(result));
    result.status = PATH_OK;
    result.ucs2 = ucs2;

    /* 先确认这个字在表里 · 区分"不存在"和"被裁判拒绝" */
    uint8_t sm_lookup, ym_lookup, tone_lookup;
    if (hanzi_ucs2_to_pinyin(ucs2, &sm_lookup, &ym_lookup, &tone_lookup) < 0) {
        result.status = PATH_ERR_UNKNOWN_HANZI;
        return result;
    }

    uint32_t code = hanzi_route(ucs2, sm_idx, ym_idx, tone);
    if (code == 0) {
        result.status = PATH_ERR_JUDGE_REJECT;
        return result;
    }

    result.container_code = code;
    result.family = CONTAINER_GET_FAMILY(code);
    result.polarity = CONTAINER_GET_POLARITY(code);

    return result;
}

/* ═══════════════════════════════════════════════════════════════
 * 流水线排水位置查询
 * ═══════════════════════════════════════════════════════════════ */

uint32_t pipeline_get_depth(void) {
    /* 流水线深度由 PIPELINE_DEPTH 常量决定 · 当前排水深度通过 hanzi_get_judge_stats 近似 */
    return 0;  /* TODO: 未来可通过 hanzi_table 公开 API 获取 */
}

/* ═══════════════════════════════════════════════════════════════
 * 进制表查询
 * ═══════════════════════════════════════════════════════════════ */

const base_declaration_t* base_decl_get(uint32_t idx) {
    if (idx >= BASE_DECL_COUNT) return NULL;
    return &BASE_DECLS[idx];
}

uint32_t base_decl_count(void) {
    return BASE_DECL_COUNT;
}

/* ═══════════════════════════════════════════════════════════════
 * 层级公约数查询
 *
 * L0: 1 (原子 · 不可再拆)
 * L1: 48 (容器 = 一声母约束下可装载的实例数)
 * L2: 1024 (流水线 = 同时排队的容器数)
 * L3: 1 (产品 = 最终产出 · 一元)
 * ═══════════════════════════════════════════════════════════════ */

uint32_t layer_get_divisor(layer_level_t level) {
    switch (level) {
    case LAYER_L0_ATOMIC:     return 1;
    case LAYER_L1_CONTAINER:  return 48;    /* 一声母下约48容器 */
    case LAYER_L2_PIPELINE:   return 1024;  /* 流水线深度 */
    case LAYER_L3_PRODUCT:    return 1;     /* 最终一元 */
    default:                  return 1;
    }
}
