/**
 * mccp_encdec.c — MCCP 编解码实现 (UTF-8 ↔ MCCP 双语)
 */
#include "mccp_encdec.h"
#include <string.h>
#include <ctype.h>

/* ── 内部状态 ────────────────────────────────────────── */
static int g_initialized = 0;

/* 声母名长度排序: 先匹配2字(zh/ch/sh/ri), 再匹配1字(b/p/...) */
static const char *SHENGMU_NAMES_2[] = {"zh", "ch", "sh", "ri", NULL};
static const char *SHENGMU_NAMES_1[] = {
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "w", "j", "q", "x", "y",
    "z", "c", "s", "r", NULL
};

void mccp_encdec_init(void) {
    g_initialized = 1;
}

/**
 * 拆分拼音: "zhong4" → initial="zh", final="ong", tone=4
 *            "a1"     → initial="",   final="a",   tone=1
 */
int mccp_split_pinyin(const char *pinyin, pinyin_split_t *result) {
    if (!pinyin || !result) return -1;
    memset(result, 0, sizeof(*result));

    int len = strlen(pinyin);
    if (len == 0) return -1;

    /* 提取尾部数字作为声调 */
    char last = pinyin[len - 1];
    if (last >= '0' && last <= '4') {
        result->tone = last - '0';
        len--;  /* 去掉声调字符 */
    } else if (last == '5') {
        result->tone = 0;  /*轻声也用0 */
        len--;
    }
    if (len == 0) return -1;

    /* 尝试匹配2字声母 */
    int matched = 0;
    for (int i = 0; SHENGMU_NAMES_2[i]; i++) {
        int nlen = strlen(SHENGMU_NAMES_2[i]);
        if (len >= nlen && strncmp(pinyin, SHENGMU_NAMES_2[i], nlen) == 0) {
            memcpy(result->initial, SHENGMU_NAMES_2[i], nlen);
            result->initial[nlen] = '\0';
            /* 剩余部分 = 韵母 */
            int flen = len - nlen;
            if (flen > 0 && flen < (int)sizeof(result->final)) {
                memcpy(result->final, pinyin + nlen, flen);
                result->final[flen] = '\0';
            }
            matched = 1;
            break;
        }
    }

    /* 尝试匹配1字声母 */
    if (!matched) {
        for (int i = 0; SHENGMU_NAMES_1[i]; i++) {
            int nlen = 1;
            if (len >= nlen && pinyin[0] == SHENGMU_NAMES_1[i][0]) {
                result->initial[0] = pinyin[0];
                result->initial[1] = '\0';
                /* 剩余 = 韵母 */
                int flen = len - nlen;
                if (flen > 0 && flen < (int)sizeof(result->final)) {
                    memcpy(result->final, pinyin + nlen, flen);
                    result->final[flen] = '\0';
                }
                matched = 1;
                break;
            }
        }
    }

    /* 零声母: 整个都是韵母 */
    if (!matched) {
        if (len < (int)sizeof(result->final)) {
            memcpy(result->final, pinyin, len);
            result->final[len] = '\0';
            result->initial[0] = '\0';
            matched = 1;
        }
    }

    if (!matched) return -1;
    result->parsed = 1;
    return 0;
}

uint32_t mccp_make_initial(const char *name, uint8_t tone) {
    int idx = shengmu_to_index(name);
    if (idx < 0) idx = 0;
    return MCP(MCCP_INITIAL, idx, tone & 0x0F, 0);
}

uint32_t mccp_make_final(const char *name, uint8_t tone) {
    /* 查韵母表找到 index */
    int idx = yunmu_name_to_index(name);
    if (idx < 0) idx = 0;
    return MCP(MCCP_FINAL, idx, tone & 0x0F, 0);
}

uint32_t mccp_make_hanzi(uint16_t index, uint8_t tone, uint8_t fsm) {
    return MCP(MCCP_HANZI, index & 0xFFF, tone & 0x0F, fsm & 0xFF);
}

/**
 * 编码: 拼音字符串 → MCCP tokens
 * 输入格式: 每行一个 "声母 韵母 tone\n" 或连续 "zhong4 guo2\n"
 * 简化版: 每个"词"以空白分隔, 每个词是拼音
 */
int mccp_encode(const char *utf8, uint32_t *tokens, size_t max_cap, size_t *used) {
    if (!utf8 || !tokens || !used || max_cap == 0) return -1;
    *used = 0;

    const char *p = utf8;
    size_t count = 0;

    while (*p && count < max_cap) {
        /* 跳过空白 */
        while (*p && (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r')) p++;
        if (!*p) break;

        /* 提取一个拼音词 */
        char word[16] = {0};
        int wlen = 0;
        while (*p && *p != ' ' && *p != '\t' && *p != '\n' && *p != '\r' && wlen < 15) {
            word[wlen++] = *p++;
        }
        word[wlen] = '\0';

        /* 拆分 */
        pinyin_split_t split;
        if (mccp_split_pinyin(word, &split) == 0 && split.parsed) {
            /* 声母 token */
            if (split.initial[0]) {
                int s_idx = shengmu_to_index(split.initial);
                if (s_idx < 0) s_idx = 0;
                tokens[count++] = MCP(MCCP_INITIAL, s_idx, split.tone, 0);
            }
            /* 韵母 token: 使用 yunmu 查找表 */
            if (split.final[0]) {
                int f_idx = yunmu_name_to_index(split.final);
                if (f_idx < 0) f_idx = 0;
                tokens[count++] = MCP(MCCP_FINAL, f_idx, split.tone, 0);
            }
        }
    }

    *used = count;
    return 0;
}

/**
 * 解码: MCCP tokens → UTF-8 拼音字符串
 */
int mccp_decode(const uint32_t *tokens, size_t count, char *utf8, size_t max_cap, size_t *used) {
    if (!tokens || !utf8 || !used || max_cap == 0) return -1;
    *used = 0;

    int pos = 0;
    utf8[0] = '\0';

    for (size_t i = 0; i < count; i++) {
        uint32_t tok = tokens[i];
        uint8_t kind = MCCP_KIND(tok);
        uint16_t idx = MCCP_INDEX(tok);
        uint8_t tone = MCCP_METAA(tok);

        const char *name = NULL;

        switch (kind) {
            case MCCP_INITIAL:
                name = shengmu_from_index(idx);
                break;
            case MCCP_FINAL:
                name = yunmu_index_to_name(idx);
                if (!name) name = "[?]";
                break;
            case MCCP_HANZI:
                /* TODO: 汉字反向查找 */
                name = "[汉字]";
                break;
            default:
                continue;
        }

        if (name) {
            int nlen = strlen(name);
            if (pos + nlen + 2 >= (int)max_cap) break;  /* +2 for tone + space */

            memcpy(utf8 + pos, name, nlen);
            pos += nlen;

            /* 追加声调数字 */
            if (tone > 0 && tone <= 4) {
                utf8[pos++] = '0' + tone;
            }

            /* 加空格分隔 */
            utf8[pos++] = ' ';
            utf8[pos] = '\0';
        }
    }

    /* 去掉尾部空格 */
    while (pos > 0 && utf8[pos-1] == ' ') {
        utf8[--pos] = '\0';
    }

    *used = pos;
    return 0;
}
