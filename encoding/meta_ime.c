/**
 * meta_ime.c — 元输入法 MetaIME 七层编码引擎
 *
 * 智能来源内嵌于程序本身 · 依靠本地规则引擎和分类器
 * 不依赖云端大模型 · 不依赖外部服务器 · 不依赖积分制商业模式
 *
 * 七层编码:
 *   第0层: 声母 (6 bit) · 64 个
 *   第1层: 韵母 (8 bit) · 256 个
 *   第2层: 声调 (3 bit) · 5 个
 *   第3层: 极性 (3 bit) · 8 个
 *   第4层: 见证 (4 bit) · 16 个
 *   第5层: 流   (3 bit) · 8 个 (八流)
 *   第6层: 主权 (21 bit) · 容器标识
 *
 * 总和: 48 bit token · 装入 64-bit cell 与 WASM 内核对齐
 */

#include "meta_ime.h"
#include "shengmu.h"
#include "yunmu.h"
#include <string.h>
#include <stdio.h>

/* ── 内部辅助 ───────────────────────────────────────────── */

/* 音节匹配 (精确或通配) */
static int syllable_match(NJSG_uint16 rule_syl, NJSG_uint16 input_syl) {
    if (rule_syl == SYLLABLE_WILDCARD) return 1;       /* 通配 */
    return rule_syl == input_syl;                       /* 精确 */
}

/* 计算 pattern 与 input 的匹配度
 * 返回置信度 0-100
 */
static NJSG_uint8 calc_confidence(const metaime_pattern_t *pattern,
                                    const NJSG_uint16 *input, NJSG_uint8 input_len) {
    if (input_len == 0 || pattern->len == 0) return 0;
    
    /* input 长度必须 ≥ pattern 长度 */
    if (input_len < pattern->len) return 0;

    /* 从 input 缓冲区尾部对齐 pattern（后缀匹配 · 匹配最新输入） */
    NJSG_uint8 offset = input_len - pattern->len;

    /* 逐音节匹配 */
    NJSG_uint8 exact_count = 0;
    NJSG_uint8 wildcard_count = 0;

    for (NJSG_uint8 i = 0; i < pattern->len; i++) {
        NJSG_uint16 rs = pattern->syllables[i];
        if (rs == SYLLABLE_WILDCARD) {
            wildcard_count++;
        } else if (syllable_match(rs, input[offset + i])) {
            exact_count++;
        } else {
            return 0;  /* 失配 */
        }
    }
    
    /* 置信度: 精确匹配占主导 */
    if (exact_count == 0 && wildcard_count > 0) return 25;  /* 纯通配 · 低置信度 */
    
    /* 完全精确匹配 = 100 · 包含通配按比例降低 */
    NJSG_uint8 confidence = (NJSG_uint8)(exact_count * 100 / pattern->len);
    if (confidence > 100) confidence = 100;
    
    /* 长度完全匹配 +10 */
    if (input_len == pattern->len && confidence < 95) {
        confidence = (confidence < 90) ? confidence + 10 : confidence;
    }
    
    return confidence;
}

/* ═══════════════════════════════════════════════════════════════
 * API 实现
 * ═══════════════════════════════════════════════════════════════ */

void metaime_init(metaime_engine_t *engine) {
    if (!engine) return;
    memset(engine, 0, sizeof(*engine));
    engine->auto_fire = 70;     /* 默认置信度 ≥70% 自动触发 */
    engine->rule_count = 0;
    engine->input_len = 0;
}

int metaime_add_rule(metaime_engine_t *engine,
                      const metaime_pattern_t *pattern,
                      const metaime_action_t *action) {
    if (!engine || !pattern || !action) return -1;
    if (engine->rule_count >= METAIME_MAX_RULES) return -1;
    
    metaime_rule_t *rule = &engine->rules[engine->rule_count];
    memset(rule, 0, sizeof(*rule));
    
    rule->id = engine->rule_count;
    memcpy(&rule->pattern, pattern, sizeof(metaime_pattern_t));
    memcpy(&rule->action, action, sizeof(metaime_action_t));
    rule->active = 1;
    
    return engine->rule_count++;
}

int metaime_feed(metaime_engine_t *engine,
                  NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone) {
    if (!engine) return 0;
    if (engine->input_len >= METAIME_INPUT_BUF) {
        /* 缓冲区满 · 丢弃最旧一个 */
        memmove(engine->input_buf, engine->input_buf + 1,
                (METAIME_INPUT_BUF - 1) * sizeof(NJSG_uint16));
        engine->input_len = METAIME_INPUT_BUF - 1;
    }
    
    NJSG_uint16 syl = SYLLABLE(sm, ym, tone);
    engine->input_buf[engine->input_len++] = syl;
    engine->total_feeds++;
    
    /* 尝试匹配 */
    metaime_match_t match;
    if (metaime_try_match(engine, &match)) {
        engine->last_match = match;
        engine->total_matches++;
        return 1;
    }
    
    return 0;
}

int metaime_try_match(const metaime_engine_t *engine,
                       metaime_match_t *match) {
    if (!engine || !match) return 0;
    
    NJSG_uint8 best_confidence = 0;
    NJSG_uint8 best_rule_idx = 0;
    
    for (NJSG_uint16 i = 0; i < engine->rule_count; i++) {
        metaime_rule_t *rule = (metaime_rule_t *)&engine->rules[i];
        if (!rule->active) continue;
        
        NJSG_uint8 conf = calc_confidence(&rule->pattern,
                                           engine->input_buf,
                                           engine->input_len);
        if (conf > best_confidence) {
            best_confidence = conf;
            best_rule_idx = (NJSG_uint8)i;
        } else if (conf == best_confidence && conf > 0) {
            /* 相同置信度 · 取优先级更高者 */
            if (engine->rules[i].action.priority >
                engine->rules[best_rule_idx].action.priority) {
                best_rule_idx = (NJSG_uint8)i;
            }
        }
    }
    
    if (best_confidence == 0) return 0;
    
    metaime_rule_t *rule = (metaime_rule_t *)&engine->rules[best_rule_idx];
    match->rule_id = rule->id;
    match->rule_index = best_rule_idx;
    match->confidence = best_confidence;
    match->action = rule->action;
    rule->hit_count++;
    
    return 1;
}

int metaime_autofire(metaime_engine_t *engine,
                      char *out_text, NJSG_size buf_size,
                      NJSG_uint64 *cell) {
    if (!engine || !out_text || !cell) return 0;
    if (engine->last_match.confidence < engine->auto_fire) return 0;
    
    /* 触发 action */
    metaime_rule_t *rule = (metaime_rule_t *)&engine->rules[engine->last_match.rule_index];
    
    /* 复制输出文本 */
    NJSG_size tlen = strlen(rule->action.text);
    if (tlen >= buf_size) tlen = buf_size - 1;
    memcpy(out_text, rule->action.text, tlen);
    out_text[tlen] = '\0';
    
    /* 获得 cell */
    *cell = rule->action.cell;
    engine->total_autofires++;
    
    return 1;
}

void metaime_reset(metaime_engine_t *engine) {
    if (!engine) return;
    engine->input_len = 0;
    engine->last_match.confidence = 0;
}

/* ── 字符串辅助 ────────────────────────────────────────── */

#define MI_APPEND(s) do { \
    const char *_p = (s); \
    while (*_p && pos < buf_size - 1) buf[pos++] = *_p++; \
} while(0)

#define MI_UINT(v) do { \
    char n[12]; \
    int _k = 11; \
    NJSG_uint32 _v = (v); \
    if (_v == 0) { n[--_k] = '0'; } \
    else { while (_v > 0 && _k > 0) { n[--_k] = '0' + (_v % 10); _v /= 10; } } \
    for (int _j = _k; _j < 11; _j++) buf[pos++] = n[_j]; \
} while(0)

void metaime_stats(const metaime_engine_t *engine,
                    char *buf, NJSG_size buf_size) {
    if (!engine || !buf || buf_size == 0) return;
    
    NJSG_size pos = 0;
    
    MI_APPEND("═══ MetaIME 七层引擎 ═══\n");
    MI_APPEND("规则: "); MI_UINT(engine->rule_count);
    MI_APPEND(" / "); MI_UINT(METAIME_MAX_RULES); MI_APPEND("\n");
    MI_APPEND("输入缓冲: "); MI_UINT(engine->input_len); MI_APPEND(" 音节\n");
    MI_APPEND("总输入: "); MI_UINT(engine->total_feeds); MI_APPEND("\n");
    MI_APPEND("总匹配: "); MI_UINT(engine->total_matches); MI_APPEND("\n");
    MI_APPEND("自动触发: "); MI_UINT(engine->total_autofires); MI_APPEND("\n");
    MI_APPEND("自动阈值: "); MI_UINT(engine->auto_fire); MI_APPEND("%\n");
    
    if (engine->last_match.confidence > 0) {
        MI_APPEND("最近匹配: rule#"); MI_UINT(engine->last_match.rule_id);
        MI_APPEND(" 置信 "); MI_UINT(engine->last_match.confidence);
        MI_APPEND("%\n");
    }
    
    buf[pos] = '\0';
}

#undef MI_APPEND
#undef MI_UINT

/* ── 编解码 ────────────────────────────────────────────── */

NJSG_uint64 metaime_encode(NJSG_uint8 sm, NJSG_uint8 ym, NJSG_uint8 tone,
                            NJSG_uint8 pol, NJSG_uint8 wit, NJSG_uint8 flow,
                            NJSG_uint32 sovereignty) {
    return meta_ime_token(sm, ym, tone, pol, wit, flow, sovereignty);
}

void metaime_decode(NJSG_uint64 cell,
                     NJSG_uint8 *sm, NJSG_uint8 *ym, NJSG_uint8 *tone,
                     NJSG_uint8 *pol, NJSG_uint8 *wit, NJSG_uint8 *flow,
                     NJSG_uint32 *sovereignty) {
    if (sm)          *sm = METAIME_TOK_SM(cell);
    if (ym)          *ym = METAIME_TOK_YM(cell);
    if (tone)        *tone = METAIME_TOK_TONE(cell);
    if (pol)         *pol = METAIME_TOK_POL(cell);
    if (wit)         *wit = METAIME_TOK_WIT(cell);
    if (flow)        *flow = METAIME_TOK_FLOW(cell);
    if (sovereignty)  *sovereignty = METAIME_TOK_SOV(cell);
}
