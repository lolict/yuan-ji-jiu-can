/*
 * mlll_pinyin.h — 汉字拼音知识库 interface
 *
 * 提供：
 *   - 汉字 → 拼音/声母/韵母/声调 映射
 *   - 拼音 → 汉字 反查（按声母+韵母+调）
 *   - 基于拼音属性的 夫妻对偶 / 命定伴侣 发现
 *
 * 知识库内嵌 ~300 常用字，含拼音标注 + 笔画数。
 * MLLL 九维坐标 由拼音字段自动算得（无需手动维护坐标表）。
 */
#ifndef MLLL_PINYIN_H
#define MLLL_PINYIN_H

#include "mlll_symbols.h"

/* 拼音条汉字：用 char_prop_t + 字符本身 */
typedef struct {
    char utf8[4];        /* UTF-8 编码，最多 3 字节 + \0 */
    char pinyin[8];      /* 带声调拼音，如 "zhōng" */
    char_prop_t prop;    /* MLLL 九维属性（由拼音+额外数据合成） */
} pinyin_entry_t;

/* 知识库大小 */
int  mlll_pinyin_count(void);

/* 索引访问 */
const pinyin_entry_t* mlll_pinyin_get(int idx);

/* 按汉字（UTF-8）查找，返回 NULL 若不存在 */
const pinyin_entry_t* mlll_pinyin_lookup_by_char(const char* utf8_char);

/* 按声母/韵母/声调查找所有匹配字符，返回匹配数；out 为结果索引数组 */
int  mlll_pinyin_find_by_pinyin(int initial_idx, int vowel_idx, int tone_idx, int* out, int max_out);

/* 计算 MLLL 属性（从 pinyin 字段拆解声母/韵母/声调，配合笔画数） */
void mlll_pinyin_compute_prop(const char* pinyin, int stroke_count, int radical_idx, int structure_idx, char_prop_t* out);

/* 拼音字符串 → 声母索引 (0..23)，无效返回 -1 */
int  mlll_pinyin_initial(const char* pinyin);

/* 拼音字符串 → 韵母索引 (0..38)，无效返回 -1 */
int  mlll_pinyin_vowel(const char* pinyin);

/* 拼音字符串 → 声调索引 (0..4，阴平/阳平/上声/去声/轻声) */
int  mlll_pinyin_tone(const char* pinyin);

/* 夫妻对偶搜索：找与 utf8_char MLLL 距离最远的字，返回匹配数 */
int  mlll_find_spouse(const char* utf8_char, int* out_indices, int max_out);

#endif /* MLLL_PINYIN_H */
