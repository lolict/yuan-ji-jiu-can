/**
 * yunmu.h — 66进制韵母编码表 (C header)
 * 来源: tools/kungguan-container/yunmu66.nim
 *
 * 33韵母 × 2(大小写) = 66符号
 * 偶数 index = 小写, 奇数 index = 大写
 * 例: a=0 A=1 ai=10 AI=11 ong=44 ONG=45
 *
 * MCCP 用法:
 *   kind = MCCP_FINAL (0x2)
 *   index = 0..65 (由本表查找)
 */
#ifndef NJSG_YUNMU_H
#define NJSG_YUNMU_H

#include <stdint.h>
#include <string.h>

#define YUNMU_COUNT 66
#define YUNMU_UNIQUE 33

typedef struct {
    const char *name;   /* 小写韵母名 */
    uint8_t index;      /* 偶数 index (小写) */
} yunmu_entry_t;

static const yunmu_entry_t YUNMU_TABLE[YUNMU_UNIQUE] = {
    /* 单韵母 (index 0..11) */
    {"a",   0},  {"o",   2},  {"e",   4},  {"i",   6},
    {"u",   8},  {"ü",  18},
    /* 复韵母 (index 12..21) */
    {"ai", 10},  {"ei", 12},  {"ao", 14},  {"ou", 16},
    {"er", 20},
    /* 复合韵母-介音 (index 22..29) */
    {"ia", 22},  {"ie", 24},  {"ua", 26},  {"uo", 28},
    /* 鼻韵母-an/en/in/ün/ang/eng (index 30..45) */
    {"an", 30},  {"en", 32},  {"in", 34},  {"ün", 36},
    {"ang", 38}, {"eng", 40}, {"ing", 42}, {"ong", 44},
    /* 三合韵母-iao/uai/ian/uan/üan (index 46..55) */
    {"iao", 46}, {"uai", 48}, {"ian", 50}, {"uan", 52},
    {"üan", 54},
    /* 四合韵母-iang/uang/iong (index 56..61) */
    {"iang", 56}, {"uang", 58}, {"iong", 60},
    /* 特殊-üe/iu (index 62..65) */
    {"üe", 62},  {"iu", 64},
};

/**
 * 韵母名称 → MCCP index
 * @param name  韵母名 (如 "a", "ong", "IAO"), 支持 ü 或 v 输入
 * @return index 0..65 (偶=小写, 奇=大写), 未找到返回 -1
 */
static inline int yunmu_name_to_index(const char *name) {
    if (!name || !name[0]) return -1;

    int is_upper = (name[0] >= 'A' && name[0] <= 'Z');

    /* 构建标准化比较输入(转小写, ü→v) */
    char cmp[8];
    int len = 0;
    for (int i = 0; name[i] && len < 7; i++) {
        /* 大写转小写 */
        char c = name[i];
        if (c >= 'A' && c <= 'Z') c = c + 32;
        /* 多字节 UTF-8 ü (0xC3 0xBC) 和 Ü (0xC3 0x9C) → v */
        if ((uint8_t)c == 0xc3) {
            if ((uint8_t)name[i+1] == 0xbc || (uint8_t)name[i+1] == 0x9c) {
                cmp[len++] = 'v';
                i++; /* 跳过第二字节 */
                continue;
            }
        }
        /* 单字节 ü/Ü (Latin-1 兼容) */
        if ((uint8_t)c == 0xfc || (uint8_t)c == 0xdc) { c = 'v'; }
        cmp[len++] = c;
    }
    cmp[len] = '\0';

    /* 查表: 表名同样做 ü→v 归一后比较 */
    for (int i = 0; i < YUNMU_UNIQUE; i++) {
        const char *n = YUNMU_TABLE[i].name;
        if (!n) continue;  /* 防御: 不查空项 */
        /* 表名归一: 逐字节把 UTF-8 ü/Ü 换成 v 再比较 */
        const char *a = cmp;
        const char *b = n;
        int ok = 1;
        while (*a) {
            if ((uint8_t)*b == 0xc3 &&
                ((uint8_t)b[1] == 0xbc || (uint8_t)b[1] == 0x9c)) {
                if (*a != 'v') { ok = 0; break; }
                b += 2;
            } else {
                if (*a != *b) { ok = 0; break; }
                b++;
            }
            a++;
        }
        if (ok && *b == '\0') {
            return YUNMU_TABLE[i].index + (is_upper ? 1 : 0);
        }
    }

    return -1;
}

/**
 * MCCP index → 韵母名称
 * @param index  0..65
 * @return 韵母名 (小写), 越界返回 NULL
 */
static inline const char *yunmu_index_to_name(int index) {
    if (index < 0 || index >= YUNMU_COUNT) return NULL;
    /* 去掉大小写位 → 小写 index */
    int base = index & ~1;
    for (int i = 0; i < YUNMU_UNIQUE; i++) {
        if (YUNMU_TABLE[i].index == base) return YUNMU_TABLE[i].name;
    }
    return NULL;
}

#endif /* NYSG_YUNMU_H */
