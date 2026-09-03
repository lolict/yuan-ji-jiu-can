/**
 * container_ontology_bridge.c — 容器本体论统一桥接实现
 *
 * 一句话: 五个子容器, 一个出口。
 * 任何外部模块只需 include 这个头, 调用 init, 然后
 * 按统一接口操作全部能力。
 *
 * 这是"自举"的一步: 系统用自己的规则长出工具,
 * 而不是用他人规则(REST/MVC/继承)长工具。
 */

#include "container_ontology_bridge.h"
#include "encoding_bridge.h"
#include "global_control.h"
#include "property_judge.h"
#include "radix_star.h"
#include <string.h>
#include <stdio.h>

/* ── 子容器实例 ── */
static global_control_t       gc_control;
static rs_star_map_t          gc_star_map;
static int                    initialized = 0;

/* ── 一次性初始化全部子容器 ── */
void container_ontology_init(void) {
    if (initialized) return;
    initialized = 1;

    /* 子容器1: 编码系统 */
    encoding_init();

    /* 子容器2: 全局控制 */
    global_control_init(&gc_control);
    gc_register_container(&gc_control, 0x10, "声母", GC_POLARITY_YANG, 48);
    gc_register_container(&gc_control, 0x20, "韵母", GC_POLARITY_YIN, 66);
    gc_register_container(&gc_control, 0x30, "声调", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc_control, 0x40, "家族", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc_control, 0x50, "结构", GC_POLARITY_NEUTRAL, 8);
    gc_add_traversal(&gc_control, 0x10, 0x20, 1);  /* 声母→韵母 */
    gc_add_traversal(&gc_control, 0x30, 0x10, 2);  /* 声调→声母 */
    gc_add_traversal(&gc_control, 0x40, 0x20, 3);  /* 家族→韵母 */
    gc_set_judgment(&gc_control, GC_MATCH_POSITION, 1, 0, 10);

    /* 子容器3: 产权流 — 无状态, 纯函数, 无需额外初始化 */

    /* 子容器4: 部首星图 */
    rs_init(&gc_star_map);
    rs_register_center(&gc_star_map, 0x6C34, 81, 4, "水·流动");
    rs_register_center(&gc_star_map, 0x706B, 82, 4, "火·热烈");
    rs_register_center(&gc_star_map, 0x8A00, 140, 7, "言·言语");

    /* 水部常见字 */
    rs_add_star(&gc_star_map, 0, 0x6C34, 18, 0, 0, 0, 0);  /* 水 */
    rs_add_star(&gc_star_map, 0, 0x6C5F, 7, 0, 0, 0, 0);   /* 江 */
    rs_add_star(&gc_star_map, 0, 0x6CB3, 10, 0, 0, 0, 0);  /* 河 */
    rs_add_star(&gc_star_map, 0, 0x6D77, 10, 0, 0, 0, 0);  /* 海 */
    rs_add_star(&gc_star_map, 0, 0x6CC9, 4, 0, 0, 0, 0);   /* 泉 */
    rs_add_star(&gc_star_map, 0, 0x6CD5, 3, 0, 0, 0, 0);   /* 法 */
    rs_add_star(&gc_star_map, 0, 0x6D1E, 5, 0, 0, 0, 0);   /* 洞 */
    rs_add_star(&gc_star_map, 0, 0x6D41, 8, 0, 0, 0, 0);   /* 流 */
    rs_add_star(&gc_star_map, 0, 0x6E05, 4, 0, 0, 0, 0);   /* 清 */
    rs_add_star(&gc_star_map, 0, 0x6DE1, 5, 0, 0, 0, 0);   /* 淡 */

    /* 火部常见字 */
    rs_add_star(&gc_star_map, 1, 0x706B, 10, 0, 0, 0, 0);  /* 火 */
    rs_add_star(&gc_star_map, 1, 0x708E, 15, 0, 0, 0, 0);  /* 炎 */
    rs_add_star(&gc_star_map, 1, 0x70C8, 8, 0, 0, 0, 0);   /* 烈 */
    rs_add_star(&gc_star_map, 1, 0x70E7, 13, 0, 0, 0, 0);  /* 烧 */
    rs_add_star(&gc_star_map, 1, 0x70B9, 5, 0, 0, 0, 0);   /* 点 */
    rs_add_star(&gc_star_map, 1, 0x70DB, 5, 0, 0, 0, 0);   /* 灯 */

    /* 言部常见字 */
    rs_add_star(&gc_star_map, 2, 0x8A00, 15, 0, 0, 0, 0);  /* 言 */
    rs_add_star(&gc_star_map, 2, 0x8BDD, 10, 0, 0, 0, 0);  /* 话 */
    rs_add_star(&gc_star_map, 2, 0x8BED, 15, 0, 0, 0, 0);  /* 语 */
    rs_add_star(&gc_star_map, 2, 0x8BB0, 7, 0, 0, 0, 0);   /* 记 */
    rs_add_star(&gc_star_map, 2, 0x8BA4, 12, 0, 0, 0, 0);  /* 认 */
    rs_add_star(&gc_star_map, 2, 0x8BF4, 13, 0, 0, 0, 0);  /* 说 */
    rs_add_star(&gc_star_map, 2, 0x8BBA, 8, 0, 0, 0, 0);   /* 论 */
    rs_add_star(&gc_star_map, 2, 0x8BCD, 2, 0, 0, 0, 0);   /* 词 */

    /* 子容器5: 免疫记忆 — 简化版, 详细实现见 negative_sample.c */
}

/* ═══════════════════════════════════════════════════════════
 * 子容器1: 编码系统包装
 * ═══════════════════════════════════════════════════════════ */

uint32_t container_encode_utf8(const char *utf8) {
    return encoding_utf8_to_container(utf8);
}

int container_get_family(uint32_t code) {
    return (int)((code >> 29) & 0x7);
}

int container_get_polarity(uint32_t code) {
    return (int)((code >> 28) & 1);
}

int container_hanzi_count(void) {
    return encoding_hanzi_count();
}

/* ═══════════════════════════════════════════════════════════
 * 子容器2: 全局控制包装
 * ═══════════════════════════════════════════════════════════ */

int container_traversal_advance(void) {
    return gc_advance_traversal(&gc_control);
}

int container_nie_xiaoqian(uint32_t ghost_code, uint32_t *body_out) {
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        gc_register_container(&gc_control, 0x20, "韵母", GC_POLARITY_YIN, 66);
        gc_set_judgment(&gc_control, GC_MATCH_POSITION, 1, 0, 10);
    }
    /* 从韵母容器中找匹配 */
    for (uint16_t yunmu = 0; yunmu < 66; yunmu++) {
        uint32_t candidate = ((uint32_t)0x20 << 8) | yunmu;
        if (gc_judge_pair(&gc_control, ghost_code, candidate)) {
            if (body_out) *body_out = candidate;
            return (int)yunmu;
        }
    }
    gc_recover(&gc_control);
    return -1;
}

int container_recover(void) {
    return gc_recover(&gc_control);
}

/* ═══════════════════════════════════════════════════════════
 * 子容器3: 产权流包装
 * ═══════════════════════════════════════════════════════════ */

int container_property_judge(uint32_t v_core, uint32_t v_a, uint32_t v_b) {
    property_verdict_t v = property_judge(v_core, v_a, v_b);
    return (int)v.infringement;
}

/* ═══════════════════════════════════════════════════════════
 * 子容器4: 部首星图包装
 * ═══════════════════════════════════════════════════════════ */

int container_star_count(void) {
    return (int)gc_star_map.count;
}

int container_star_group_size(uint8_t radical_idx) {
    if (radical_idx >= gc_star_map.count) return 0;
    return (int)gc_star_map.centers[radical_idx].star_count;
}

/* ═══════════════════════════════════════════════════════════
 * 子容器5: 免疫记忆包装
 * ═══════════════════════════════════════════════════════════ */

int container_record_failure(uint32_t pattern, const char *reason) {
    (void)pattern; (void)reason;
    return 0;
}

int container_is_known_failure(uint32_t pattern) {
    (void)pattern;
    return 0;
}

/* ═══════════════════════════════════════════════════════════
 * 子容器6: 感知器官 (距离感知聚类 · 聂小倩匹配)
 * ═══════════════════════════════════════════════════════════ */

static mo_engine_t mo_organ;

void container_mind_organ_init(double threshold) {
    mind_organ_init(&mo_organ, threshold);
}

int container_mo_add_cell(int id, uint32_t container_code) {
    return mind_organ_add_cell(&mo_organ, id, container_code);
}

double container_mo_distance(uint32_t code_a, uint32_t code_b) {
    return mind_organ_distance(code_a, code_b);
}

int container_mo_clusterize(void) {
    return mind_organ_clusterize(&mo_organ);
}

int container_mo_find_body(uint32_t ghost_code, double *dist_out) {
    return mind_organ_find_body(&mo_organ, ghost_code, dist_out);
}

int container_mo_cluster_count(void) {
    return mo_organ.cluster_count;
}

void container_mo_report(char *buf, int bufsize) {
    mind_organ_report(&mo_organ, buf, bufsize);
}

/* 容器本体论初始化时同时初始化感知器官 (默认阈值 5.0) */
__attribute__((constructor))
static void container_mind_organ_auto_init(void) {
    /* 注意: 实际在 container_ontology_init() 调用时也可手动初始化 */
    mind_organ_init(&mo_organ, 5.0);
}
