/**
 * shengmu.h — 48进制声母编码表 (C header)
 * 来源: tools/kungguan-container/shengmu48.nim
 *
 * 24声母小写 + 24声母大写 = 48符号
 * 每对严格大小写配对: shengmu48[i] ↔ shengmu48_upper[i]
 *
 * 在 MCCP 中使用:
 *   kind = MCCP_INITIAL (0x1)
 *   index = 0..47 (由本表查找)
 *   meta_a = tone (由外部传入)
 *   meta_b = FSM state
 */
#ifndef NJSG_SHENGMU_H
#define NJSG_SHENGMU_H

#include <stdint.h>
#include <string.h>

#define SHENGMU_COUNT 48
#define SHENGMU_LOWER_COUNT 24

/* 声母名称 (拼音) — 索引 0..23 */
static const char * const SHENGMU_NAME[SHENGMU_LOWER_COUNT] = {
    "b", "p", "m", "f",   /* 唇音 */
    "d", "t", "n", "l",   /* 舌尖前音 */
    "g", "k", "h", "w",   /* 舌根音 */
    "j", "q", "x", "y",   /* 舌面音 */
    "z", "c", "s", "r",   /* 舌尖前音(平舌) */
    "zh", "ch", "sh", "ri", /* 扩展: 卷舌音 + 日 */
};

/* 声母 Unicode 小写符号 (索引 0..23) */
/* 直接使用 ASCII b,p,m,f,d,t,n,l,g,k,h,w,j,q,x,y,z,c,s,r */
/* 卷舌音用组合符号: zh=ts+ʐ变体, ch=tc+ʈ变体, sh=ᶊ, ri=ɻ */

/* 声母到索引的映射表 (查找用) */
typedef struct {
    const char *name;
    uint8_t index;
} shengmu_entry_t;

static const shengmu_entry_t SHENGMU_TABLE[SHENGMU_LOWER_COUNT] = {
    /* 唇音 */
    {"b",   0},  {"p",   1},  {"m",   2},  {"f",   3},
    /* 舌尖前音 */
    {"d",   4},  {"t",   5},  {"n",   6},  {"l",   7},
    /* 舌根音 */
    {"g",   8},  {"k",   9},  {"h",  10},  {"w",  11},
    /* 舌面音 */
    {"j",  12},  {"q",  13},  {"x",  14},  {"y",  15},
    /* 舌尖前音(平舌) */
    {"z",  16},  {"c",  17},  {"s",  18},  {"r",  19},
    /* 卷舌音 */
    {"zh", 20},  {"ch", 21},  {"sh", 22},  {"ri", 23},
};

/* 首字节判断: 是否为多字节声母 (zh/ch/sh/ri) */
static inline int shengmu_is_multi(uint8_t c) {
    return (c == 'z' || c == 'c' || c == 's' || c == 'r');
}

/**
 * 声母名称 → MCCP index (0..23 小写, 24..47 大写)
 * @param name  声母名 (如 "b", "zh", "B", "ZH")
 * @return index, 未找到返回 -1
 */
static inline int shengmu_to_index(const char *name) {
    if (!name || !name[0]) return -1;

    /* 检测大写: A-Z 或组合大写 */
    int is_upper = (name[0] >= 'A' && name[0] <= 'Z');
    int offset = is_upper ? 24 : 0;

    /* 单字符声母 (真正只有1字节) */
    if (name[1] == '\0') {
        char c = name[0];
        for (int i = 0; i < SHENGMU_LOWER_COUNT; i++) {
            if (SHENGMU_TABLE[i].name[0] == c && SHENGMU_TABLE[i].name[1] == '\0') {
                return SHENGMU_TABLE[i].index + offset;
            }
        }
    }

    /* 双字符声母 (zh/ch/sh/ri) 或双字符大写 (ZH/CH/SH/RI) */
    for (int i = 0; i < SHENGMU_LOWER_COUNT; i++) {
        const char *n = SHENGMU_TABLE[i].name;
        if (n[0] == name[0] && n[1] == name[1] && n[2] == '\0') {
            return i + offset;
        }
        /* 大写组合: ZH, CH, SH, RI */
        if (is_upper && n[0] == (name[0] + 32) && n[1] == (name[1] + 32) && n[2] == '\0') {
            return i + offset;
        }
    }
    return -1;
}

/**
 * MCCP index → 声母名称
 * @param index  0..47
 * @return 声母名 (如 "b", "zh", "B", "ZH"), 越界返回 NULL
 */
static inline const char *shengmu_from_index(int index) {
    if (index < 0 || index >= SHENGMU_COUNT) return NULL;
    if (index >= 24) index -= 24;
    return SHENGMU_TABLE[index].name;
}

#endif /* NJSG_SHENGMU_H */
