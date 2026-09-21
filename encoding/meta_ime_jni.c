/**
 * meta_ime_jni.c — JNI 桥接实现
 *
 * 活码编译器 + 时钟动作 + 七层编码引擎联动
 * 用 malloc/free 模拟 JNI 边界的生命周期
 */

#include "meta_ime_jni.h"
#include <stdlib.h>
#include <string.h>

/* ── JNI 方法实现 ────────────────────────────────────────── */

int metaime_jni_create(void **out_engine) {
    if (!out_engine) return -1;
    metaime_engine_t *eng = (metaime_engine_t *)malloc(sizeof(metaime_engine_t));
    if (!eng) return -1;
    metaime_init(eng);
    *out_engine = eng;
    return 0;
}

int metaime_jni_destroy(void *engine) {
    if (!engine) return -1;
    free(engine);
    return 0;
}

int metaime_jni_reset(void *engine) {
    metaime_engine_t *eng = (metaime_engine_t *)engine;
    if (!eng) return -1;
    metaime_reset(eng);
    return 0;
}

int metaime_jni_load_rules(void *engine, const NJSG_uint8 *data, NJSG_size len) {
    /* 此实现中规则通过 add_rule 手动加载
     * 字节流加载在 Android 侧由 parseFromBuffer 完成
     * 这里仅检查参数有效性 */
    if (!engine || !data || len == 0) return -1;
    return 0;
}

int metaime_jni_feed(void *engine, NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone) {
    metaime_engine_t *eng = (metaime_engine_t *)engine;
    if (!eng) return -1;
    return metaime_feed(eng, sm, ym, tone);
}

int metaime_jni_get_match(void *engine, NJSG_uint8 *confidence,
                           char *out_text, NJSG_size buf_size, NJSG_uint64 *out_cell) {
    metaime_engine_t *eng = (metaime_engine_t *)engine;
    if (!eng) return -1;

    int fired = metaime_autofire(eng, out_text, buf_size, out_cell);
    if (confidence) *confidence = eng->last_match.confidence;
    return fired;
}

int metaime_jni_clock(void *engine, mi_clock_action_t action) {
    metaime_engine_t *eng = (metaime_engine_t *)engine;
    if (!eng) return -1;

    switch (action) {
        case MI_TICK_KEYDOWN:
            /* 进入处理状态 — 重置临时缓冲 */
            break;
        case MI_TICK_MATCHED:
            /* 匹配成功 — 可在此插入观察 */
            break;
        case MI_TICK_FIRED:
            /* 动作触发 — 输出文字 */
            break;
        case MI_TICK_COMMIT:
            /* 提交完成 — 清空缓冲 */
            metaime_reset(eng);
            break;
        case MI_TICK_IDLE:
            /* 空闲 — 什么都不做 */
            break;
    }
    return 0;
}

/* ── 活码编译器 ─────────────────────────────────────────── */

int mi_live_activate(mi_live_code_t *lc, NJSG_uint64 key) {
    if (!lc) return -1;
    lc->key   = key;
    lc->fixed = 0;
    lc->order = 0;
    lc->value = 0x5A5A5A5A;  /* 高熵未固定值 */
    return 0;
}

int mi_live_fix(mi_live_code_t *lc, NJSG_uint8 order) {
    if (!lc) return -1;
    lc->fixed = 1;
    lc->order = order;
    lc->value = order * 100;  /* 加法统计起点 */
    return 0;
}

int mi_live_accumulate(mi_live_code_t *lc, NJSG_uint32 delta) {
    if (!lc) return -1;
    if (!lc->fixed) {
        /* 未固定: 值不累积 */
        return 0;
    }
    /* 固定后: 加法统计 */
    lc->value += delta * lc->order;
    return 0;
}
