/**
 * meta_ime_jni.h — Android IME JNI 桥接层 (头文件规格)
 *
 * 架构: Kotlin 薄壳 + Rust 核心 (此处为 C 核心)
 *   满足 InputMethodService 框架要求 — 不脱离 Android IME 规范
 *   但所有智能内嵌于自身 — 不依赖云端
 *
 * JNI 函数命名规则 (Java 侧 → C 核心):
 *   Java_com_njsg_metaime_<ClassName>_<method>  →  njsg_metaime_<class>_<method>
 *
 * 七层编码流经 JNI:
 *   Kotlin KeyEvent → 声母韵母解析 → NJSG_uint16 syllable → 喂入引擎 → 匹配 → 自动输出
 *
 * 活码编译器接口:
 *   起点(钥匙) → 激活可变编码 → 固定后按排列顺序确定权限和数值
 *
 * 此头文件定义 JNI 桥接规格 — 编译期约束 · 不生成任何外部依赖
 */

#ifndef NJSG_META_IME_JNI_H
#define NJSG_META_IME_JNI_H

#include "meta_ime.h"
#include "wasm_port.h"

/* ── JNI 版本 ────────────────────────────────────────────── */

#define JNI_VERSION_NJSG 0x00010001  /* JNI 1.6 兼容 */

/* ── 钟表动作 (AI 生命周期) ───────────────────────────────
 *
 * 结束即开始 · 记忆跟随结束而继续
 * 每个 tick 驱动一次思考-动作循环
 */
typedef enum {
    MI_TICK_KEYDOWN = 0x01,   /* 按键按下 · 进入处理 */
    MI_TICK_MATCHED = 0x02,   /* 匹配成功 · 进入裁决 */
    MI_TICK_FIRED   = 0x03,   /* 动作触发 · 输出文字 */
    MI_TICK_COMMIT  = 0x04,   /* 提交完成 · 缓冲清空 */
    MI_TICK_IDLE    = 0x05,   /* 空闲 · 等待下一次 */
} mi_clock_action_t;

/* ──  JNI 实现桩 (核心实现链接到 meta_ime_jni.c) ───────── */

NJSG_EXPORT int metaime_jni_create(void **out_engine);
NJSG_EXPORT int metaime_jni_destroy(void *engine);
NJSG_EXPORT int metaime_jni_reset(void *engine);
NJSG_EXPORT int metaime_jni_load_rules(void *engine, const NJSG_uint8 *data, NJSG_size len);
NJSG_EXPORT int metaime_jni_feed(void *engine, NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone);
NJSG_EXPORT int metaime_jni_get_match(void *engine, NJSG_uint8 *confidence,
                                       char *out_text, NJSG_size buf_size, NJSG_uint64 *out_cell);
NJSG_EXPORT int metaime_jni_clock(void *engine, mi_clock_action_t action);

/* ──  JNI 方法表 ───────────────────────────────────────────
 *
 * 对应 Kotlin 侧 MetaIMEBridge 的方法签名
 * C 核心实现后通过 RegisterNatives 绑定
 */

/* 引擎生命周期 */
typedef int (*njsg_mi_create_fn)(void **out_engine);
typedef int (*njsg_mi_destroy_fn)(void *engine);
typedef int (*njsg_mi_reset_fn)(void *engine);

/* 规则加载 (从私有格式字节流 · 非 JSON/XML) */
typedef int (*njsg_mi_load_rules_fn)(void *engine,
                                      const NJSG_uint8 *data,
                                      NJSG_size len);

/* 单音节输入 */
typedef int (*njsg_mi_feed_fn)(void *engine,
                                NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone);

/* 获取匹配结果 */
typedef int (*njsg_mi_get_match_fn)(void *engine,
                                     NJSG_uint8 *confidence,
                                     char *out_text, NJSG_size buf_size,
                                     NJSG_uint64 *out_cell);

/* 时钟驱动 */
typedef int (*njsg_mi_clock_tick_fn)(void *engine, mi_clock_action_t action);

/* JNI 方法表结构 (供 RegisterNatives 使用) */
typedef struct {
    njsg_mi_create_fn    create;
    njsg_mi_destroy_fn   destroy;
    njsg_mi_reset_fn     reset;
    njsg_mi_load_rules_fn load_rules;
    njsg_mi_feed_fn      feed;
    njsg_mi_get_match_fn get_match;
    njsg_mi_clock_tick_fn clock_tick;
} njsg_mi_jni_table_t;

/* ── 获取 JNI 方法表 (唯一导出入口) ────────────────────── */

/* 前向声明 (实现在 meta_ime_jni.c) */
NJSG_EXPORT int metaime_jni_create(void **out_engine);
NJSG_EXPORT int metaime_jni_destroy(void *engine);
NJSG_EXPORT int metaime_jni_reset(void *engine);
NJSG_EXPORT int metaime_jni_load_rules(void *engine, const NJSG_uint8 *data, NJSG_size len);
NJSG_EXPORT int metaime_jni_feed(void *engine, NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone);
NJSG_EXPORT int metaime_jni_get_match(void *engine, NJSG_uint8 *confidence,
                                       char *out_text, NJSG_size buf_size, NJSG_uint64 *out_cell);
NJSG_EXPORT int metaime_jni_clock(void *engine, mi_clock_action_t action);

/**
 * 获取 JNI 方法表
 * Kotlin: MetaIMEBridge.nativeGetMethodTable() → Long (指针)
 * 返回值: 指向静态方法表的指针 (不需要释放)
 */
static inline const njsg_mi_jni_table_t *njsg_mi_get_jni_table(void) {
    static const njsg_mi_jni_table_t table = {
        .create     = (njsg_mi_create_fn)    metaime_jni_create,
        .destroy    = (njsg_mi_destroy_fn)   metaime_jni_destroy,
        .reset      = (njsg_mi_reset_fn)     metaime_jni_reset,
        .load_rules = (njsg_mi_load_rules_fn)metaime_jni_load_rules,
        .feed       = (njsg_mi_feed_fn)      metaime_jni_feed,
        .get_match  = (njsg_mi_get_match_fn) metaime_jni_get_match,
        .clock_tick = (njsg_mi_clock_tick_fn)metaime_jni_clock,
    };
    return &table;
}

/* ── 活码编译器 (起点激活器) ─────────────────────────────
 *
 * 起点(钥匙) → 激活可变编码 → 固定后按排列确定权限
 * 未固定: 位置任意活动
 * 固定后: 顺序决定加法权限
 */

typedef struct {
    NJSG_uint64  key;        /* 起点钥匙 */
    NJSG_uint8   fixed;      /* 是否已固定 */
    NJSG_uint8   order;      /* 排列顺序 (决定加法权限) */
    NJSG_uint32  value;      /* 当前值 */
} mi_live_code_t;

/**
 * 活码激活: 用钥匙激活一段编码
 * 未固定时: value = 任意值 (高熵)
 * 固定后: value = order 确定的加法累加
 */
NJSG_EXPORT int mi_live_activate(mi_live_code_t *lc, NJSG_uint64 key);

/**
 * 活码锁定: 固定位置 · 确定加法权限
 */
NJSG_EXPORT int mi_live_fix(mi_live_code_t *lc, NJSG_uint8 order);

/**
 * 活码累加: 按排列顺序加法统计
 */
NJSG_EXPORT int mi_live_accumulate(mi_live_code_t *lc, NJSG_uint32 delta);

#endif /* NJSG_META_IME_JNI_H */
