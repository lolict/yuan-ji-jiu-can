/**
 * mccp_encdec.h — MCCP 编解码函数 (UTF-8 ↔ MCCP 双向)
 * 位置: encoding/mccp_encdec.h
 * 配套: mccp_encdec.c (实现)
 *
 * 接口设计:
 *   mccp_encode()  — UTF-8 字符串 → MCCP token 序列
 *   mccp_decode()  — MCCP token 序列 → UTF-8 字符串
 *   mccp_matched() — 判断字符串是否匹配 MCCP token
 */
#ifndef NJSG_MCCP_ENCDEC_H
#define NJSG_MCCP_ENCDEC_H

#include "mccp.h"
#include "shengmu.h"
#include "yunmu.h"

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── 最大编码能力 ────────────────────────────────────── */
#define MCCP_ENC_MAX_TOKENS  512    /* 单次编码最大 token 数 */
#define MCCP_DEC_MAX_BYTES   2048   /* 单次解码最大 UTF-8 字节 */

/* ── 拼音到 MCCP 的映射结果 ─────────────────────────── */
typedef struct {
    uint32_t initial;     /* 声母 MCCP token (kind=INITIAL) */
    uint32_t final;      /* 韵母 MCCP token (kind=FINAL) */
    uint32_t hanzi;      /* 汉字 MCCP token (kind=HANZI) */
    uint8_t  tone;      /* 声调 0..4 */
    int      valid;     /* 是否成功解析 */
} mccp_pinyin_t;

/* ── 初始韵母拆分结果 ────────────────────────────────── */
typedef struct {
    char initial[4];    /* 声母缓冲 ("b","zh",...) */
    char final[8];      /* 韵母缓冲 ("a","iong",...) */
    uint8_t tone;       /* 声调 */
    int parsed;         /* 是否成功 */
} pinyin_split_t;

/* ── 公开 API ────────────────────────────────────────── */

/**
 * mccp_encdec_init — 初始化编码系统的查找表
 *   首次调用 encode/decode 前必须调用一次
 */
void mccp_encdec_init(void);

/**
 * mccp_encode — 将 UTF-8 拼音字符串 (如 "zhong4") 编码为 MCCP token
 * @param utf8     UTF-8 拼音字符串 (声调以数字 0-4 附加)
 * @param tokens   输出 token 缓冲
 * @param max_cap  缓冲容量 (tokens)
 * @param used     实际写入 token 数
 * @return 0 成功, -1 失败
 */
int mccp_encode(const char *utf8, uint32_t *tokens, size_t max_cap, size_t *used);

/**
 * mccp_decode — 将 MCCP token 序列解码为 UTF-8 字符串
 * @param tokens   MCCP token 数组
 * @param count    token 数量
 * @param utf8     输出 UTF-8 缓冲
 * @param max_cap  缓冲容量 (bytes)
 * @param used     实际写入字节数
 * @return 0 成功, -1 失败
 */
int mccp_decode(const uint32_t *tokens, size_t count, char *utf8, size_t max_cap, size_t *used);

/**
 * mccp_make_init — 构造声母 MCCP token
 * @param name  声母名 (如 "b", "zh")
 * @param tone  声调 0..4
 * @return MCCP token
 */
uint32_t mccp_make_initial(const char *name, uint8_t tone);

/**
 * mccp_make_final — 构造韵母 MCCP token
 * @param name  韵母名 (如 "a", "iong")
 * @param tone  声调 0..4
 * @return MCCP token
 */
uint32_t mccp_make_final(const char *name, uint8_t tone);

/**
 * mccp_make_hanzi — 构造汉字 MCCP token
 * @param index 汉字索引 0..4095
 * @param tone  声调 0..4
 * @param fsm   FSM 状态
 * @return MCCP token
 */
uint32_t mccp_make_hanzi(uint16_t index, uint8_t tone, uint8_t fsm);

/**
 * mccp_split_pinyin — 将拼音拆分为声母+韵母+声调
 * @param pinyin  输入拼音 (如 "zhong4", "a1")
 * @param result  输出拆分结果
 * @return 0 成功, -1 失败
 */
int mccp_split_pinyin(const char *pinyin, pinyin_split_t *result);

#ifdef __cplusplus
}
#endif

#endif /* NSSG_MCCP_ENCDEC_H */
