/*
 * mlll_symbols.h — 七维符号系统公共头文件
 *
 * 七维编码：几何(边角面脊) × 声调(阴阳平上去入) × 部首(康熙) ×
 *          颜色(可见光谱) × 相位(360°) × 笔画(1-64) × 结构(8 种拓扑)
 *
 * 总符号数：841
 * 提供：跨维度共振、字符编码、距离度量、字形跃迁、字节 round-trip
 *
 * 用法：
 *   1. 独立运行：kernel/src/mlll_symbols.c 含 main()
 *   2. 库态编译：定义 MLLL_LIBRARY 宏后纳入其他 TU
 */

#ifndef MLLL_SYMBOLS_H
#define MLLL_SYMBOLS_H

#include <stdint.h>

#define MLLL_SYMBOLS_MAX       4096
#define MLLL_OFFSET_PRECISION  10000
#define MLLL_TOTAL_SYMBOLS     841  /* 七维当前总数 */

/* 符号类型 */
typedef enum {
    SYM_GEOMETRY = 0,   /* 几何边角面 */
    SYM_TONE,           /* 声调 */
    SYM_RADICAL,        /* 部首 */
    SYM_COLOR,          /* 颜色 */
    SYM_PHASE,          /* 相位角度 */
    SYM_STROKE,         /* 笔画数 */
    SYM_STRUCTURE,      /* 字形结构 */
    SYM_DIMENSION_COUNT /* 维度计数 */
} symbol_dim_t;

/* 单个符号 */
typedef struct {
    uint32_t     id;
    symbol_dim_t dim;
    int64_t      property_a;
    int64_t      property_b;
    uint32_t     sequence_pos;
    uint32_t     offset_num;
    char         name[64];
} mlll_symbol_t;

/* 符号库 */
typedef struct {
    mlll_symbol_t entries[MLLL_SYMBOLS_MAX];
    uint32_t      count;
    uint32_t      tone_table[8];
} mlll_registry_t;

/* 字符属性（用于字符串级编码） */
typedef struct {
    int stroke;
    int tone;
    int radical;
    int structure; /* 0=独体 1=左右 2=上下 3=半包围 4=全包围 5=品字 6=左中右 7=上中下 */
} char_prop_t;

/* ── 全局状态 ── */
extern mlll_registry_t REG;

/* ── 生命周期 ── */
void mlll_init(double delta);
void mlll_print_stats(void);

/* ── 查询 ── */
mlll_symbol_t* mlll_get_by_sequence(uint32_t pos);
mlll_symbol_t* mlll_cross_query(symbol_dim_t src_dim, int64_t prop, symbol_dim_t tgt_dim);

/* ── 编码 ── */
void     mlll_character_demo(int stroke, int tone, int radical_idx);
void     mlll_string_encode(const char* name, char_prop_t* chars, int len);
uint32_t mlll_encode_bytes(const uint8_t* data, int len, uint16_t* out_sym, int max_out);
uint32_t mlll_decode_bytes(const uint16_t* symbols, int n, uint8_t* out_data, int max_out);

/* ── 距离 + 检索 ── */
double mlll_char_distance(char_prop_t* a, char_prop_t* b);
int    mlll_char_nearest(char_prop_t* tgt, char_prop_t* cands, int n);
int    mlll_char_farthest(char_prop_t* tgt, char_prop_t* cands, int n);
int    mlll_char_leap(char_prop_t* origin, int shift, char_prop_t* cands, int n);
void   mlll_distance_decompose(char_prop_t* a, char_prop_t* b);
void   mlll_string_distance_matrix(const char* name, char_prop_t* chars, int len);

/* ── RGB 时分复用编码：文本 ↔ 像素矩阵 ── */
int  mlll_rgb_encode_text(const char *text, int text_len, uint8_t *rgb_buf, int buf_size);
int  mlll_rgb_decode_text(const uint8_t *rgb_buf, int pixel_pairs, char *text_out, int out_size);

#endif /* MLLL_SYMBOLS_H */
