/**
 * encoding_bridge.h — 编码系统桥接
 *
 * 为 kernel/shell/cloudflare 等模块提供统一的编码 API
 *
 * 使用方式:
 *   #include "encoding_bridge.h"
 *
 *   // 初始化
 *   encoding_init();
 *
 *   // UTF-8 汉字 → 容器编码
 *   pipeline_result_t r = container_pipeline_process_utf8("囡");
 *   if (r.status == PATH_OK) {
 *       uint32_t code = r.container_code;
 *       // 使用 code ...
 *   }
 *
 *   // 脱壳: 旧符号 → 容器编码
 *   uint32_t code = husk_process("int", 3);
 *
 *   // 感知家族查询
 *   perception_family_t fam = container_classify_family(0x56DE);
 *
 *   // 流属性位计算
 *   uint32_t bits = container_compute_flow_bits(0x56DE, fam);
 */

#ifndef NJSG_ENCODING_BRIDGE_H
#define NJSG_ENCODING_BRIDGE_H

#include <stdint.h>
#include <string.h>
#include "perception_field.h"
#include "container_pipeline.h"
#include "husk_compiler.h"
#include "mccp_encdec.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ═══════════════════════════════════════════════════════════════
 * 一次性初始化 (汉字表 + MCCP + 裁判引擎)
 * ═══════════════════════════════════════════════════════════════ */

static inline void encoding_init(void) {
    hanzi_init();
    mccp_encdec_init();
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: UTF-8 → 容器编码 (一步到位)
 * ═══════════════════════════════════════════════════════════════ */

static inline uint32_t encoding_utf8_to_container(const char *utf8) {
    pipeline_result_t r = container_pipeline_process_utf8(utf8);
    return (r.status == PATH_OK) ? r.container_code : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: UCS-2 → 容器编码
 * ═══════════════════════════════════════════════════════════════ */

static inline uint32_t encoding_ucs2_to_container(uint16_t ucs2, uint8_t tone) {
    pipeline_result_t r = container_pipeline_route_direct(ucs2, 0, 0, tone);
    return (r.status == PATH_OK) ? r.container_code : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 感知家族查询
 * ═══════════════════════════════════════════════════════════════ */

static inline perception_family_t encoding_classify_family(uint16_t ucs2) {
    uint32_t code = hanzi_route(ucs2, 0, 0, 0);
    if (code == 0) return FAMILY_NEUTRAL;
    return (perception_family_t)CONTAINER_GET_FAMILY(code);
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 流属性位计算
 * ═══════════════════════════════════════════════════════════════ */

static inline uint32_t encoding_compute_flow_bits(uint16_t ucs2,
                                                  perception_family_t family) {
    uint32_t bits = 0;

    bits |= 0x001;  /* REACHABLE */
    bits |= 0x020;  /* ACKABLE */

    if (ucs2 > 0) {
        bits |= 0x002;  /* CONTINUABLE */
        bits |= 0x004;  /* INCR */
    }

    if (family == FAMILY_CONTAINER) {
        bits |= 0x010;  /* TRUSTED */
        bits |= 0x040;  /* RECOVERABLE */
    } else if (family == FAMILY_WEB) {
        bits |= 0x080;  /* CIRCULABLE */
        bits |= 0x100;  /* INHERITABLE */
    }

    return bits;
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 极性判定 (YinYang)
 * ═══════════════════════════════════════════════════════════════ */

static inline uint8_t encoding_get_polarity(uint16_t ucs2) {
    perception_family_t fam = encoding_classify_family(ucs2);

    /* 容器家族 → 刘楚恬侧(阴) */
    if (fam == FAMILY_CONTAINER) return 0;
    /* 边界家族 → lolict侧(阳) */
    if (fam == FAMILY_BOUNDARY) return 1;
    /* 夫妻家族 → 默认阴 */
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 汉字是否存在
 * ═══════════════════════════════════════════════════════════════ */

static inline int encoding_has_hanzi(uint16_t ucs2) {
    return (hanzi_route(ucs2, 0, 0, 0) != 0);
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 表内汉字数
 * ═══════════════════════════════════════════════════════════════ */

static inline int encoding_hanzi_count(void) {
    return hanzi_count();
}

/* ═══════════════════════════════════════════════════════════════
 * 便捷函数: 极性判定 (YinYang)
 * ═══════════════════════════════════════════════════════════════ */

#ifdef __cplusplus
}
#endif

#endif /* NJSG_ENCODING_BRIDGE_H */
