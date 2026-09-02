/**
 * meta_ime.h — 元输入法 MetaIME 七层编码引擎
 *
 * 基于七层编码体系驱动键盘拦截、意图识别和自动打字:
 *   第0层: 声母 (Shengmu)      6 bit  · 64 个 (实际用48)
 *   第1层: 韵母 (Yunmu)        8 bit  · 256 个 (实际用66)
 *   第2层: 声调 (Tone)         3 bit  · 5 个 (0=轻声, 1-4)
 *   第3层: 主权 (Sovereignty) 24 bit  · 容器标识
 *   第4层: 极性 (Polarity)     3 bit  · 妻/夫/系/观/仲/见/秘/空
 *   第5层: 见证 (Witness)     4 bit  · 三元裁判等级
 *   第6层: 流   (Flow)         3 bit  · 八流统一
 *
 * 单一 token = 48 bit 编码:
 *   47:42  shengmu  (初始识别)
 *   41:34  yunmu    (音色形状)
 *   33:31  tone     (振动模式)
 *   30:27  polarity (阴阳极性)
 *   26:24  witness  (见证等级)
 *   23:21  flow     (八流选择)
 *   20:0   sovereignty (21 bit 容器空间)
 *
 * 用法:
 *   1. init 引擎
 *   2. add_rule() 注册规则 (输入 pattern → 输出 cell + action)
 *   3. feed_syllable() 逐音节输入
 *   4. 自动匹配 → 返回最高置信度 rule
 *   5. autotype() 输出结果
 *
 * 红线和哲学:
 *   - 不依赖云端 · 不依赖外部服务 · 只依赖自有编码
 *   - 规则自主生长 · 可由 njsg-swarm 自动发现新规则
 *   - 输入一旦匹配成功 · 自动执行 action · 无需人工确认
 */

#ifndef NJSG_META_IME_H
#define NJSG_META_IME_H

#include "wasm_port.h"

/* ── 配置 ──────────────────────────────────────────────── */

#define METAIME_MAX_RULES       256     /* 规则表容量 */
#define METAIME_MAX_PATTERN     16      /* 最大规则 pattern 长度 (音节数) */
#define METAIME_MAX_OUTPUT      32      /* 单个 action 输出最长字符 */
#define METAIME_INPUT_BUF       32      /* 输入缓冲区 (音节) */

/* ── 七层编码 ──────────────────────────────────────────── */

/* 极性扩展 (超越 wife/husband/system) */
#define METAIME_POL_WIFE     0x0  /* 妻 容器包被包含者 */
#define METAIME_POL_HUSBAND  0x1  /* 夫 界线 */
#define METAIME_POL_SYSTEM   0x2  /* 系统 中性 */
#define METAIME_POL_OBSERVER 0x3  /* 观察 距离观测 */
#define METAIME_POL_ARBITER  0x4  /* 仲裁 见证升级 */
#define METAIME_POL_SECRET   0x5  /* 密 隐藏根基 */
#define METAIME_POL_EMPTY    0x6  /* 空 幂等 */

/* ── 单音节编码 (15 bit) ──────────────────────────────── */

#define SYLLABLE_SM_SHIFT   0
#define SYLLABLE_YM_SHIFT   6
#define SYLLABLE_TONE_SHIFT 12

#define SYLLABLE(sm, ym, tone) \
    (((uint16_t)((sm) & 0x3F) << SYLLABLE_SM_SHIFT) | \
     ((uint16_t)((ym) & 0x3F) << SYLLABLE_YM_SHIFT) | \
     ((uint16_t)((tone) & 0x7) << SYLLABLE_TONE_SHIFT))

#define SYLLABLE_SM(s)      (((s) >> SYLLABLE_SM_SHIFT) & 0x3F)
#define SYLLABLE_YM(s)      (((s) >> SYLLABLE_YM_SHIFT) & 0x3F)
#define SYLLABLE_TONE(s)    (((s) >> SYLLABLE_TONE_SHIFT) & 0x7)

/* 通配符: 匹配任意音节 */
#define SYLLABLE_WILDCARD   0xFFFF

/* ── 48-bit token 编码 ────────────────────────────────── */

#define METAIME_TOKEN_NULL  0xFFFFFFFFFFFFULL  /* 空 token */

static inline NJSG_uint64 meta_ime_token(NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone,
                                          NJSG_uint8 pol, NJSG_uint8 wit, NJSG_uint8 flow,
                                          NJSG_uint32 sov) {
    return ((NJSG_uint64)(sm & 0x3F) << 42) |
           ((NJSG_uint64)(ym & 0xFF) << 34) |
           ((NJSG_uint64)(tone & 0x7) << 31) |
           ((NJSG_uint64)(pol & 0x7) << 28) |
           ((NJSG_uint64)(wit & 0xF) << 24) |
           ((NJSG_uint64)(flow & 0x7) << 21) |
           ((NJSG_uint64)(sov & 0x1FFFFF));
}

/* Token 分解宏 */
#define METAIME_TOK_SM(t)     (NJSG_uint8)(((t) >> 42) & 0x3F)
#define METAIME_TOK_YM(t)     (NJSG_uint8)(((t) >> 34) & 0xFF)
#define METAIME_TOK_TONE(t)   (NJSG_uint8)(((t) >> 31) & 0x7)
#define METAIME_TOK_POL(t)    (NJSG_uint8)(((t) >> 28) & 0x7)
#define METAIME_TOK_WIT(t)    (NJSG_uint8)(((t) >> 24) & 0xF)
#define METAIME_TOK_FLOW(t)   (NJSG_uint8)(((t) >> 21) & 0x7)
#define METAIME_TOK_SOV(t)    (NJSG_uint32)((t) & 0x1FFFFF)

/* ── 规则结构 ──────────────────────────────────────────── */

/* Pattern: 一个 pattern 是若干音节编码组成的序列
 * 通配符 SYLLABLE_WILDCARD 匹配任意单个音节 */
typedef struct {
    NJSG_uint16 syllables[METAIME_MAX_PATTERN];  /* 音节序列 */
    NJSG_uint8  len;        /* pattern 长度 (音节数) */
    NJSG_uint8  wildcard_only; /* 纯通配 · 低优先级 */
} metaime_pattern_t;

/* Action: 匹配后执行的行动 */
typedef struct {
    char         text[METAIME_MAX_OUTPUT];     /* 输出文本 (UTF-8) */
    NJSG_uint64  cell;     /* WASM 内核 cell (主权/极性/见证编码) */
    NJSG_uint8   flow;     /* 触发哪一流 */
    NJSG_uint8   priority; /* 优先级 0-15 · 高优先 */
} metaime_action_t;

/* 规则 */
typedef struct {
    NJSG_uint16     id;
    metaime_pattern_t pattern;
    metaime_action_t  action;
    NJSG_uint32     hit_count;  /* 命中次数 (规则生长) */
    NJSG_uint8      active;     /* 是否启用 */
} metaime_rule_t;

/* ── 匹配结果 ──────────────────────────────────────────── */

typedef struct {
    NJSG_uint16 rule_id;
    NJSG_uint8  rule_index;/* 在规则表中的索引 */
    NJSG_uint8  confidence;/* 置信度 0-100 */
    metaime_action_t action;
} metaime_match_t;

/* ── 引擎状态 ──────────────────────────────────────────── */

typedef struct {
    /* 规则表 */
    metaime_rule_t  rules[METAIME_MAX_RULES];
    NJSG_uint16     rule_count;
    
    /* 输入缓冲区 */
    NJSG_uint16     input_buf[METAIME_INPUT_BUF];
    NJSG_uint8      input_len;
    
    /* 最近一次匹配 */
    metaime_match_t last_match;
    
    /* 配置 */
    NJSG_uint8      auto_fire;        /* ≥ 此置信度自动触发 */
    NJSG_uint8      case_sensitive;   /* 大小写敏感 */
    
    /* 统计 */
    NJSG_uint32     total_feeds;
    NJSG_uint32     total_matches;
    NJSG_uint32     total_autofires;
} metaime_engine_t;

/* ── API ────────────────────────────────────────────────── */

/**
 * 初始化引擎
 */
NJSG_EXPORT void metaime_init(metaime_engine_t *engine);

/**
 * 添加规则
 * @return rule index · -1 = 表满
 */
NJSG_EXPORT int metaime_add_rule(metaime_engine_t *engine,
                                  const metaime_pattern_t *pattern,
                                  const metaime_action_t *action);

/**
 * 输入一个音节 (声母+韵母+声调)
 * @return 1 = 有新匹配 · 0 = 无匹配
 */
NJSG_EXPORT int metaime_feed(metaime_engine_t *engine,
                              NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone);

/**
 * 尝试匹配当前输入缓冲
 * @param match  输出匹配结果
 * @return 1 = 有匹配 · 0 = 无
 */
NJSG_EXPORT int metaime_try_match(const metaime_engine_t *engine,
                                   metaime_match_t *match);

/**
 * 自动打字: 如果匹配置信度 ≥ auto_fire · 触发 action
 * @param out_text  输出文本
 * @param cell      输出 WASM cell
 * @return 1 = 已触发 · 0 = 未触发
 */
NJSG_EXPORT int metaime_autofire(metaime_engine_t *engine,
                                  char *out_text, NJSG_size buf_size,
                                  NJSG_uint64 *cell);

/**
 * 清除输入缓冲
 */
NJSG_EXPORT void metaime_reset(metaime_engine_t *engine);

/**
 * 获取统计信息文本
 */
NJSG_EXPORT void metaime_stats(const metaime_engine_t *engine,
                                char *buf, NJSG_size buf_size);

/**
 * 七层编码 → 64-bit cell (编码到 WASM 内核)
 */
NJSG_EXPORT NJSG_uint64 metaime_encode(NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone,
                                        NJSG_uint8 pol, NJSG_uint8 wit, NJSG_uint8 flow,
                                        NJSG_uint32 sovereignty);

/**
 * 64-bit cell → 七层解码
 */
NJSG_EXPORT void metaime_decode(NJSG_uint64 cell,
                                 NJSG_uint8 *sm, NJSG_uint8 *ym, NJSG_uint8 *tone,
                                 NJSG_uint8 *pol, NJSG_uint8 *wit, NJSG_uint8 *flow,
                                 NJSG_uint32 *sovereignty);

#endif /* NJSG_META_IME_H */
