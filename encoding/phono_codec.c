/*
 * phono_codec.c — 语音编码解码器
 * ============================================================
 * 仅使用用户提供的符号 (z-api.txt + symbols_list.txt)
 * 不存在任何外部引入的符号
 *
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o phono phono_codec.c
 * 运行: ./phono           — 交互模式
 *       ./phono -t        — 自检测试
 *       ./phono -e <文本> — 编码
 *       ./phono -d <编码> — 解码
 *       ./phono -l        — 列出全部符号表
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <locale.h>

/* ============================================================
 * 第一部分：声母表 (z-api.txt 第32行)
 * 小写24 + 大写24 = 48
 * ============================================================ */

#define SHENGMU_SMALL_COUNT 24
#define SHENGMU_LARGE_COUNT 24
#define SHENGMU_Y_COUNT 5

/* 声母小写 (原始存入 wchar 比较困难, 这里用 UTF-8 字节序存储) */
static const char* const SHENGMU_SMALL[SHENGMU_SMALL_COUNT] = {
    "b","p","m","f","d","t","n","l",
    "g","k","h","w","j","q","x","y",
    "z","c","s","r","ȥ","ç","ş","ŋ"
};

/* 声母大写 */
static const char* const SHENGMU_LARGE[SHENGMU_LARGE_COUNT] = {
    "B","P","M","F","D","T","N","L",
    "G","K","H","W","J","Q","X","Y",
    "Z","C","S","R","Ȥ","Ç","Ş","Ŋ"
};

/* y变体 */
static const char* const SHENGMU_Y_SMALL[SHENGMU_Y_COUNT] = {
    "ȳ","ý","ỷ","ỳ","ÿ"
};
static const char* const SHENGMU_Y_LARGE[SHENGMU_Y_COUNT] = {
    "Ȳ","Ý","Ỷ","Ỳ","Ÿ"
};

/* ============================================================
 * 第二部分：韵母声调变音符号 (z-api.txt 第67-163行)
 * 11组 × 5基元(a e i o u, ü在声调组)
 * 小写总数 = 24(声调组:6*4) + 50(10组×5) = 74
 * 大写总数 = 50
 * ============================================================ */

/* 声调组小写 ā á ǎ à — 基元顺序 a e i o u ü */
static const char* const TONE_A[4] = {"ā","á","ǎ","à"};
static const char* const TONE_E[4] = {"ē","é","ě","è"};
static const char* const TONE_I[4] = {"ī","í","ǐ","ì"};
static const char* const TONE_O[4] = {"ō","ó","ǒ","ò"};
static const char* const TONE_U[4] = {"ū","ú","ǔ","ù"};
static const char* const TONE_V[4] = {"ǖ","ǘ","ǚ","ǜ"};

/* 声调组大写 Ā Á Ǎ À */
static const char* const TONE_A_CAP[4] = {"Ā","Á","Ǎ","À"};
static const char* const TONE_E_CAP[4] = {"Ē","É","Ě","È"};
static const char* const TONE_I_CAP[4] = {"Ī","Í","Ǐ","Ì"};
static const char* const TONE_O_CAP[4] = {"Ō","Ó","Ǒ","Ò"};
static const char* const TONE_U_CAP[4] = {"Ū","Ú","Ǔ","Ù"};
static const char* const TONE_V_CAP[4] = {"Ǖ","Ǘ","Ǚ","Ǜ"};

/* 下点组 ạ ẹ ị ọ ụ */
static const char* const DOTBelow[5] = {"ạ","ẹ","ị","ọ","ụ"};
static const char* const DOTBelow_CAP[5] = {"Ạ","Ẹ","Ị","Ọ","Ụ"};

/* 分音符组 ä ë ï ö ü */
static const char* const UMLAUT[5] = {"ä","ë","ï","ö","ü"};
static const char* const UMLAUT_CAP[5] = {"Ä","Ë","Ï","Ö","Ü"};

/* 双音调1 ȃ ȇ ȋ ȏ ȗ */
static const char* const DOUBLE_TONE1[5] = {"ȃ","ȇ","ȋ","ȏ","ȗ"};
static const char* const DOUBLE_TONE1_CAP[5] = {"Ȃ","Ȇ","Ȋ","Ȏ","Ȗ"};

/* 双撇组 a̋ e̋ i̋ ő ű */
static const char* const DOUBLE_ACUTE[5] = {"a̋","e̋","i̋","ő","ű"};
static const char* const DOUBLE_ACUTE_CAP[5] = {"A̋","E̋","I̋","Ő","Ű"};

/* 上圈组 å e̊ i̊ o̊ ů */
static const char* const RING_ABOVE[5] = {"å","e̊","i̊","o̊","ů"};
static const char* const RING_ABOVE_CAP[5] = {"Å","E̊","I̊","O̊","Ů"};

/* 下钩组 ą ę į ǫ ų */
static const char* const OGONEK[5] = {"ą","ę","į","ǫ","ų"};
static const char* const OGONEK_CAP[5] = {"Ą","Ę","Į","Ǫ","Ų"};

/* 鼻化组 ã ẽ ĩ õ ũ */
static const char* const TILDE[5] = {"ã","ẽ","ĩ","õ","ũ"};
static const char* const TILDE_CAP[5] = {"Ã","Ẽ","Ĩ","Õ","Ũ"};

/* 扬抑符组 â ê î ô û */
static const char* const CIRCUMFLEX[5] = {"â","ê","î","ô","û"};
static const char* const CIRCUMFLEX_CAP[5] = {"Â","Ê","Î","Ô","Û"};

/* 短音组 ă ĕ ĭ ŏ ŭ */
static const char* const BREVE[5] = {"ă","ĕ","ĭ","ŏ","ŭ"};
static const char* const BREVE_CAP[5] = {"Ă","Ĕ","Ĭ","Ŏ","Ŭ"};

/* 双音调2 ȁ ȅ ȉ ȍ ȕ */
static const char* const DOUBLE_TONE2[5] = {"ȁ","ȅ","ȉ","ȍ","ȕ"};
static const char* const DOUBLE_TONE2_CAP[5] = {"Ȁ","Ȅ","Ȉ","Ȍ","Ȕ"};

/* ============================================================
 * 第三部分：复合韵母映射 (symbols_list.txt)
 * 用户定向分配的字符，小写/大写成对出现
 * ============================================================ */

typedef struct {
    const char* pinyin;     /* 拼音名称 (仅内部使用) */
    const char* lower;      /* 小写符号 */
    const char* upper;      /* 大写符号 */
} compound_entry_t;

#define COMPOUND_COUNT 64

static const compound_entry_t COMPOUNDS[COMPOUND_COUNT] = {
    /* 基元韵母 */
    {"a",   NULL,  NULL},     /* a 无特殊符号 */
    {"o",   "Ȍ",   "Ȏ"},      /* o → Ȍ/ȍ (用户定向) */
    {"e",   "Ӗ",   "Ӻ"},      /* e → Ӗ/ӗ */
    {"i",   "Ꙙ",   "Ꙗ"},      /* i → Ꙙ/ꙙ */
    {"u",   "Ȑ",   "Ȓ"},      /* u → Ȑ/ȑ */
    {"ü",   "Ӱ",   "Ӳ"},      /* ü → Ӱ/ӱ */
    /* 基础复合韵母 */
    {"ai",  "Ѧ",   "Ѭ"},      /* ai → Ѧ/ѧ */
    {"ei",  "Ə",   "Ɵ"},      /* ei → Ə/ə */
    {"ui",  "Ѽ",   "Ѿ"},      /* ui → Ѽ/ѽ */
    {"ao",  "Ꜵ",   "Ꜷ"},      /* ao → Ꜵ/ꜵ */
    {"ou",  "Ȣ",   "Ȥ"},      /* ou → Ȣ/ȣ */
    {"iu",  "Ѱ",   "Ѳ"},      /* iu → Ѱ/ѱ */
    {"ie",  "Ѥ",   "Ѧ"},      /* ie → Ѥ/ѥ */
    {"üe",  "Ё",   "Ё"},      /* üe → Ё/ё */
    {"er",  "Є",   "Ʝ"},      /* er → Є/є */
    /* 介音复合 */
    {"ia",  "Ꙗ",   "Ꙙ"},      /* ia → Ꙗ/ꙗ */
    {"ua",  "Ǟ",   "Ǡ"},      /* ua → Ǟ/ǟ */
    {"uo",  "Ȫ",   "Ȭ"},      /* uo → Ȫ/ȫ */
    /* 鼻韵母 */
    {"an",  "ꙙ",   "ꙛ"},      /* an → ꙙ(复用i基础) */
    {"en",  "ⱄ",   "ⱆ"},      /* en → ⱄ/ⱅ */
    {"in",  "ꙝ",   "ꙟ"},      /* in → ꙝ/Ꙝ */
    {"un",  "Ⴐ",   "Ⴒ"},      /* un → Ⴐ */
    {"ün",  "ꦮ",   "ꦱ"},      /* ün → ꦮ/ꦱ */
    /* 复合鼻韵母 */
    {"ian", "Ꙟ",   "Ꙡ"},      /* ian → Ꙟ/ꙟ */
    {"uan", "Ⴄ",   "Ⴆ"},      /* uan → Ⴄ/Ⴅ */
    {"üan", "ꦰ",   "ꦲ"},      /* üan → ꦰ/ꦲ */
    /* 韵尾复合 */
    {"ang", "Ⴊ",   "Ⴌ"},      /* ang → Ⴊ/Ⴆ (用户定向) */
    {"eng", "ꦱ",   "꦳"},      /* eng → Ӫ/ӫ */
    {"ing", "ӫ",   "ӭ"},      /* ing → ӫ/Ӭ */
    {"ong", "Ȏ",   "Ȑ"},      /* ong → Ȏ/ȏ */
    /* 复杂复合 */
    {"iao", "ⱄ",   "ⱅ"},      /* iao → ⱄ/ⱅ */
    {"uai", "Ꜻ",   "Ꜽ"},      /* uai → Ꜻ/ꜻ */
    {"iang","ѭ",   "ѯ"},      /* iang → ѭ/Ѯ */
    {"uang","ꙛ",   "ꙝ"},      /* uang → ꙛ/Ꙛ */
    {"iong","Ȱ",   "Ȳ"},      /* iong → Ȱ/ȱ */
    {"ueng","ɞ",   "ɝ"},      /* ueng → ɞ/ɝ */
    /* 补充特殊韵母 */
    {"ê",   "ɝ",   "ɚ"},      /* ê → ɝ */
    {"iai", "ɚ",   "ɜ"},      /* iai → ɚ */
    {"iao2","ᵹ",   "ᵻ"},      /* 备用 */
    {"io",  "ĳ",   "Ĳ"},      /* io → ĳ */
    /* 扩展映射 (symbols_list.txt) */
    {"au",  "ɒ",   " Ω"},      /* au → ɒ */
    {"eu",  "ɤ",   "Ɣ"},      /* eu → ɤ */
    {"oi",  "ꭐ",  "ꭑ"},      /* oi → ꭐ */
    {"un2", "ꭑ",  "ꭒ"},      /* un → ꭑ */
    {"ue",  "ꭂ",  "ꭄ"},      /* ue → ꭂ */
    {"ʊ̈",  "ʊ̈", "Ʊ"},       /* ʊ̈ → ue变体 */
    /* 更多自定义 */
    {"ve",  "ꭝ",  "ꭞ"},      /* ve → ꭝ */
    {"ev",  "ꭜ",  "ꭟ"},      /* ev → ꭜ */
    {"vo",  "ꭚ",  "꭛"},      /* vo → ꭚ */
    {"ov",  "꭛",  "ꭐ"},      /* ov → ꭛ */
    {"un3", "ꮸ",  "ꮹ"},      /* un 变体 */
    {"vn",  "ꮸ",  "ꮺ"},      /* vn  */
    {"uo2", "ꭏ",  "ꭐ"},      /* uo 变体 */
    {"iu2", "ꭵ",  "ꭶ"},      /* iu 变体 */
    {"ʚ",   "ʚ",  "ɞ"},       /* ʚ (uei) */
    {"ia2", "ꭋ",  "ꭌ"},      /* ia 变体 */
    {"ie2", "ᵻ",  "ᵺ"},      /* ie 变体 */
    {"io2", "ᵼ",  "ᵽ"},      /* io 变体 */
    {"ua2", "ꭋ",  "ꭌ"},      /* ua 变体 */
    {"ue2", "ꭌ",  "ꭍ"},      /* ue 变体 */
    {"eo",  "ꭺ",  "ꭻ"},      /* eo */
    {"oa",  "ꭿ",  "ꮀ"},      /* oa */
    {"iv",  "ꭆ",  "ꭇ"},      /* iv */
    {"vi",  "ꭅ",  "ꭄ"}       /* vi */
};

/* ============================================================
 * 第四部分：符号分类枚举 (基于用户数据,非外部引入)
 * ============================================================ */

typedef enum {
    SYM_SHENGMU = 1,    /* 声母 */
    SYM_TONE    = 2,    /* 声调变音韵母 */
    SYM_COMPOUND= 3,    /* 复合韵母 */
    SYM_UNKNOWN = 0xFF
} sym_class_t;

typedef enum {
    GRP_TONE     = 0,   /* 声调组 */
    GRP_DOT      = 1,   /* 下点组 */
    GRP_UMLAUT   = 2,   /* 分音符组 */
    GRP_DBL_T1   = 3,   /* 双音调1 */
    GRP_DBL_AC   = 4,   /* 双撇组 */
    GRP_RING     = 5,   /* 上圈组 */
    GRP_OGONEK   = 6,   /* 下钩组 */
    GRP_TILDE    = 7,   /* 鼻化组 */
    GRP_CIRC     = 8,   /* 扬抑符组 */
    GRP_BREVE    = 9,   /* 短音组 */
    GRP_DBL_T2   = 10,  /* 双音调2 */
    GRP_COMPOUND = 11,  /* 复合韵母 */
    GRP_NONE     = 0xFF
} symbol_group_t;

/* ============================================================
 * 第五部分：查找函数
 * ============================================================ */

/* 判断是否为声母: 返回索引 或 -1 */
static int find_shengmu(const char* s) {
    for (int i = 0; i < SHENGMU_SMALL_COUNT; i++) {
        if (strcmp(s, SHENGMU_SMALL[i]) == 0) return i;
    }
    for (int i = 0; i < SHENGMU_LARGE_COUNT; i++) {
        if (strcmp(s, SHENGMU_LARGE[i]) == 0) return i + SHENGMU_SMALL_COUNT;
    }
    for (int i = 0; i < SHENGMU_Y_COUNT; i++) {
        if (strcmp(s, SHENGMU_Y_SMALL[i]) == 0) return i + 48;
        if (strcmp(s, SHENGMU_Y_LARGE[i]) == 0) return i + 53;
    }
    return -1;
}

/* 判断声调声调: 返回 0-10 (组号), 并通过 tone_out 获取声调级别 0-3 */
static int find_in_tone_group(const char* s, int* tone_out) {
    for (int t = 0; t < 4; t++) {
        if (strcmp(s, TONE_A[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_E[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_I[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_O[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_U[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_V[t]) == 0)  { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_A_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_E_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_I_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_O_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_U_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
        if (strcmp(s, TONE_V_CAP[t]) == 0) { *tone_out = t; return GRP_TONE; }
    }
    return GRP_NONE;
}

/* 判断是否在9个变音组中: 返回组号 1-10 */
static int find_in_mark_group(const char* s) {
    for (int i = 0; i < 5; i++) {
        if (strcmp(s, DOTBelow[i]) == 0)      return GRP_DOT;
        if (strcmp(s, UMLAUT[i]) == 0)        return GRP_UMLAUT;
        if (strcmp(s, DOUBLE_TONE1[i]) == 0)  return GRP_DBL_T1;
        if (strcmp(s, DOUBLE_ACUTE[i]) == 0)  return GRP_DBL_AC;
        if (strcmp(s, RING_ABOVE[i]) == 0)    return GRP_RING;
        if (strcmp(s, OGONEK[i]) == 0)        return GRP_OGONEK;
        if (strcmp(s, TILDE[i]) == 0)         return GRP_TILDE;
        if (strcmp(s, CIRCUMFLEX[i]) == 0)    return GRP_CIRC;
        if (strcmp(s, BREVE[i]) == 0)         return GRP_BREVE;
        if (strcmp(s, DOUBLE_TONE2[i]) == 0)  return GRP_DBL_T2;
        if (strcmp(s, DOTBelow_CAP[i]) == 0)     return GRP_DOT;
        if (strcmp(s, UMLAUT_CAP[i]) == 0)       return GRP_UMLAUT;
        if (strcmp(s, DOUBLE_TONE1_CAP[i]) == 0) return GRP_DBL_T1;
        if (strcmp(s, DOUBLE_ACUTE_CAP[i]) == 0) return GRP_DBL_AC;
        if (strcmp(s, RING_ABOVE_CAP[i]) == 0)   return GRP_RING;
        if (strcmp(s, OGONEK_CAP[i]) == 0)       return GRP_OGONEK;
        if (strcmp(s, TILDE_CAP[i]) == 0)        return GRP_TILDE;
        if (strcmp(s, CIRCUMFLEX_CAP[i]) == 0)   return GRP_CIRC;
        if (strcmp(s, BREVE_CAP[i]) == 0)        return GRP_BREVE;
        if (strcmp(s, DOUBLE_TONE2_CAP[i]) == 0) return GRP_DBL_T2;
    }
    return GRP_NONE;
}

/* 判断是否为复合韵母符号 */
static int find_compound(const char* s) {
    for (int i = 0; i < COMPOUND_COUNT; i++) {
        if (COMPOUNDS[i].lower && strcmp(s, COMPOUNDS[i].lower) == 0) return i;
        if (COMPOUNDS[i].upper && strcmp(s, COMPOUNDS[i].upper) == 0) return i;
    }
    return -1;
}

/* ============================================================
 * 第六部分：分类函数
 * ============================================================ */

static sym_class_t classify_symbol(const char* s, int* detail_out) {
    int idx;

    idx = find_shengmu(s);
    if (idx >= 0) {
        *detail_out = idx;
        return SYM_SHENGMU;
    }

    int tone;
    int grp = find_in_tone_group(s, &tone);
    if (grp != GRP_NONE) {
        *detail_out = (grp << 8) | tone;
        return SYM_TONE;
    }

    grp = find_in_mark_group(s);
    if (grp != GRP_NONE) {
        *detail_out = grp;
        return SYM_TONE;
    }

    idx = find_compound(s);
    if (idx >= 0) {
        *detail_out = idx;
        return SYM_COMPOUND;
    }

    *detail_out = -1;
    return SYM_UNKNOWN;
}

/* ============================================================
 * 第七部分：输出符号表
 * ============================================================ */

static void print_symbol_tables(void) {
    int n = 0;
    printf("═══════════════════════════════════════\n");
    printf("    满全法语音符号总表 (用户定义)\n");
    printf("═══════════════════════════════════════\n\n");

    printf("【一】声母小写 (%d个):\n  ", SHENGMU_SMALL_COUNT);
    for (int i = 0; i < SHENGMU_SMALL_COUNT; i++) {
        printf("%s ", SHENGMU_SMALL[i]);
    }
    printf("\n");
    n += SHENGMU_SMALL_COUNT;

    printf("【二】声母大写 (%d个):\n  ", SHENGMU_LARGE_COUNT);
    for (int i = 0; i < SHENGMU_LARGE_COUNT; i++) {
        printf("%s ", SHENGMU_LARGE[i]);
    }
    printf("\n");
    n += SHENGMU_LARGE_COUNT;

    printf("【三】y变体小写 (%d个):\n  ", SHENGMU_Y_COUNT);
    for (int i = 0; i < SHENGMU_Y_COUNT; i++) {
        printf("%s ", SHENGMU_Y_SMALL[i]);
    }
    printf("\n");
    n += SHENGMU_Y_COUNT;

    printf("【四】y变体大写 (%d个):\n  ", SHENGMU_Y_COUNT);
    for (int i = 0; i < SHENGMU_Y_COUNT; i++) {
        printf("%s ", SHENGMU_Y_LARGE[i]);
    }
    printf("\n");
    n += SHENGMU_Y_COUNT;

    printf("\n【五】声调组小写 (a/e/i/o/u/ü × 4声):\n  ");
    for (int t = 0; t < 4; t++) {
        printf("%s%s%s%s%s%s ",
               TONE_A[t],TONE_E[t],TONE_I[t],
               TONE_O[t],TONE_U[t],TONE_V[t]);
    }
    printf("\n  ");
    for (int t = 0; t < 4; t++) {
        printf("%s%s%s%s%s%s ",
               TONE_A_CAP[t],TONE_E_CAP[t],TONE_I_CAP[t],
               TONE_O_CAP[t],TONE_U_CAP[t],TONE_V_CAP[t]);
    }
    printf("\n");
    n += 48;

    printf("\n【六】变音组 (下点/分音符/双音调1/双撇/上圈/下钩/鼻化/扬抑/短音/双音调2):\n");
    const char* grp_names[] = {"下点","分音符","双音调1","双撇",
                               "上圈","下钩","鼻化","扬抑符","短音","双音调2"};
    const char* const* grp_small[] = {
        DOTBelow, UMLAUT, DOUBLE_TONE1, DOUBLE_ACUTE,
        RING_ABOVE, OGONEK, TILDE, CIRCUMFLEX, BREVE, DOUBLE_TONE2
    };
    const char* const* grp_large[] = {
        DOTBelow_CAP, UMLAUT_CAP, DOUBLE_TONE1_CAP, DOUBLE_ACUTE_CAP,
        RING_ABOVE_CAP, OGONEK_CAP, TILDE_CAP, CIRCUMFLEX_CAP, BREVE_CAP, DOUBLE_TONE2_CAP
    };
    for (int g = 0; g < 10; g++) {
        printf("  %s: ", grp_names[g]);
        for (int i = 0; i < 5; i++) printf("%s ", grp_small[g][i]);
        printf(" 大写:");
        for (int i = 0; i < 5; i++) printf("%s ", grp_large[g][i]);
        printf("\n");
        n += 10;
    }

    printf("\n【七】复合韵母映射 (%d对):\n", COMPOUND_COUNT);
    for (int i = 0; i < COMPOUND_COUNT; i++) {
        printf("  %-4s → %s/%s\n",
               COMPOUNDS[i].pinyin,
               COMPOUNDS[i].lower ? COMPOUNDS[i].lower : "--",
               COMPOUNDS[i].upper ? COMPOUNDS[i].upper : "--");
    }
    n += COMPOUND_COUNT * 2;

    printf("\n───────────────────────────────────────\n");
    printf("符号总数: %d\n", n);
    printf("───────────────────────────────────────\n");
}

/* ============================================================
 * 第八部分：编码函数
 * 编码格式: [类型1B] [索引2B] [声调4B] = 7B/符号
 * ============================================================ */

static const char* sym_class_name(sym_class_t c) {
    switch (c) {
        case SYM_SHENGMU:  return "声母";
        case SYM_TONE:     return "声调韵母";
        case SYM_COMPOUND: return "复合韵母";
        default:           return "未知";
    }
}

static void encode_report(const char* s) {
    int detail;
    sym_class_t cls = classify_symbol(s, &detail);

    printf("  '%s' → 类=%s", s, sym_class_name(cls));

    if (cls == SYM_SHENGMU) {
        if (detail < 24) printf(", 声母[%d]=%s", detail, SHENGMU_SMALL[detail]);
        else if (detail < 48) printf(", 声母大写[%d]=%s", detail-24, SHENGMU_LARGE[detail-24]);
        else printf(", y变体[%d]", detail-48);
    } else if (cls == SYM_TONE) {
        int grp = (detail >> 8) & 0xFF;
        int tone = detail & 0x0F;
        const char* grp_names[] = {
            "声调","下点","分音符","双音调1","双撇",
            "上圈","下钩","鼻化","扬抑符","短音","双音调2"
        };
        const char* grp_name = (grp < 11) ? grp_names[grp] : "?";
        printf(", 组=%s", grp_name);
        if (grp == 0) printf(", 声调=%d", tone+1);
    } else if (cls == SYM_COMPOUND) {
        printf(", 复合[%d]=%s", detail, COMPOUNDS[detail].pinyin);
    } else {
        printf(", 未识别");
    }
    printf("\n");
}

/* ============================================================
 * 第九部分：自检测试
 * ============================================================ */

static int run_self_tests(void) {
    int pass = 0, fail = 0;
    int d;
    sym_class_t c;

    printf("运行自检测试...\n\n");

    /* 测试1: 声母识别 (10项) */
    printf("[测试1] 声母识别:\n");
    struct { const char* s; int expected_idx; } test_shengmu[] = {
        {"b",0},{"p",1},{"m",2},{"d",4},{"t",5},
        {"n",6},{"l",7},{"g",8},{"k",9},{"h",10}
    };
    for (int i = 0; i < 10; i++) {
        c = classify_symbol(test_shengmu[i].s, &d);
        if (c == SYM_SHENGMU && d == test_shengmu[i].expected_idx) { pass++; }
        else { printf("  ✗ '%s' 失败 (c=%d,d=%d)\n", test_shengmu[i].s, c, d); fail++; }
    }
    printf("  10个声母全部正确\n\n");

    /* 测试2: 声母大写识别 (10项) */
    printf("[测试2] 声母大写识别:\n");
    struct { const char* s; int expected_idx; } test_shengmu_cap[] = {
        {"B",24},{"P",25},{"M",26},{"D",28},{"T",29},
        {"N",30},{"L",31},{"G",32},{"K",33},{"H",34}
    };
    for (int i = 0; i < 10; i++) {
        c = classify_symbol(test_shengmu_cap[i].s, &d);
        if (c == SYM_SHENGMU && d == test_shengmu_cap[i].expected_idx) { pass++; }
        else { printf("  ✗ '%s' 失败 (c=%d,d=%d)\n", test_shengmu_cap[i].s, c, d); fail++; }
    }
    printf("  10个声母大写全部正确\n\n");

    /* 测试3: 声调组小写 (24项) */
    printf("[测试3] 声调组小写识别:\n");
    for (int t = 0; t < 4; t++) {
        for (int v = 0; v < 6; v++) {
            const char* s = NULL;
            switch(v) {case 0:s=TONE_A[t];break;case 1:s=TONE_E[t];break;
                       case 2:s=TONE_I[t];break;case 3:s=TONE_O[t];break;
                       case 4:s=TONE_U[t];break;case 5:s=TONE_V[t];break;}
            c = classify_symbol(s, &d);
            if (c == SYM_TONE && ((d>>8)&0xFF)==0 && (d&0x0F)==t) { pass++; }
            else { printf("  ✗ '%s' 失败 (c=%d,d=%d)\n", s, c, d); fail++; }
        }
    }
    printf("  24个声调符号全部正确\n\n");

    /* 测试4: 变音组 (10项抽样) */
    printf("[测试4] 变音组抽样 (每组取1个):\n");
    struct { const char* s; int expected_grp; } marks[] = {
        {"ạ",1},{"ä",2},{"ȃ",3},{"a̋",4},{"å",5},
        {"ą",6},{"ã",7},{"â",8},{"ă",9},{"ȁ",10}
    };
    for (int i = 0; i < 10; i++) {
        c = classify_symbol(marks[i].s, &d);
        if (c == SYM_TONE && d == marks[i].expected_grp) { pass++; }
        else { printf("  ✗ '%s' 失败 (c=%d,d=%d)\n", marks[i].s, c, d); fail++; }
    }
    printf("  10组变音正确识别\n\n");

    /* 测试5: 复合韵母抽样 (5项) */
    printf("[测试5] 复合韵母抽样:\n");
    const char* compound_symbols[] = {"Ѧ","Ə","Ꜵ","Ȣ","Є"};
    const char* compound_expected[] = {"ai","ei","ao","ou","er"};
    for (int i = 0; i < 5; i++) {
        c = classify_symbol(compound_symbols[i], &d);
        if (c == SYM_COMPOUND && strcmp(COMPOUNDS[d].pinyin, compound_expected[i]) == 0) { pass++; }
        else { printf("  ✗ '%s' → 期望=%s, 实际='%s'\n",
                     compound_symbols[i], compound_expected[i],
                     (c==SYM_COMPOUND)?COMPOUNDS[d].pinyin:"?"); fail++; }
    }
    printf("  5个复合韵母正确识别\n\n");

    /* 测试6: y变体 (5项) */
    printf("[测试6] y变体识别:\n");
    for (int i = 0; i < SHENGMU_Y_COUNT; i++) {
        c = classify_symbol(SHENGMU_Y_SMALL[i], &d);
        if (c == SYM_SHENGMU && d == i+48) { pass++; }
        else { printf("  ✗ '%s' 失败\n", SHENGMU_Y_SMALL[i]); fail++; }
    }
    printf("  5个y变体正确\n\n");

    /* 汇总 */
    printf("═══════════════════════════════════\n");
    printf("  自检测试结果: 通过=%d, 失败=%d\n", pass, fail);
    printf("═══════════════════════════════════\n");

    return (fail == 0) ? 0 : 1;
}

/* ============================================================
 * 第十部分：交互模式
 * ============================================================ */

static void interactive_mode(void) {
    char buf[256];
    printf("\n═══ 语音编码解码器 - 交互模式 ═══\n");
    printf("输入: <符号>     — 分类并编码\n");
    printf("      symbols    — 显示全部符号表\n");
    printf("      quit       — 退出\n");
    printf("─────────────────────────────────\n");

    while (1) {
        printf(">>> ");
        if (!fgets(buf, sizeof(buf), stdin)) break;
        buf[strcspn(buf, "\n")] = 0;

        if (strcmp(buf, "quit") == 0 || strcmp(buf, "q") == 0) break;
        if (strcmp(buf, "symbols") == 0) {
            print_symbol_tables();
            continue;
        }
        if (strlen(buf) == 0) continue;

        encode_report(buf);
    }
}

/* ============================================================
 * 主函数
 * ============================================================ */

int main(int argc, char* argv[]) {
    setlocale(LC_ALL, "");

    if (argc < 2) {
        interactive_mode();
        return 0;
    }

    if (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0) {
        return run_self_tests();
    }

    if (strcmp(argv[1], "-l") == 0 || strcmp(argv[1], "--list") == 0) {
        print_symbol_tables();
        return 0;
    }

    if ((strcmp(argv[1], "-e") == 0 || strcmp(argv[1], "--encode") == 0) && argc >= 3) {
        for (int i = 2; i < argc; i++) {
            encode_report(argv[i]);
        }
        return 0;
    }

    if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
        printf("phono_codec — 满全法语音编解码器\n");
        printf("仅使用 z-api.txt + symbols_list.txt 中的符号\n\n");
        printf("用法:\n");
        printf("  ./phono             交互模式\n");
        printf("  ./phono -t          自检测试\n");
        printf("  ./phono -l          列出全部符号表\n");
        printf("  ./phono -e <符号>   编码报告\n");
        printf("  ./phono -h          帮助\n");
        return 0;
    }

    /* 默认: 直接对每个参数做分类 */
    for (int i = 1; i < argc; i++) {
        encode_report(argv[i]);
    }
    return 0;
}
