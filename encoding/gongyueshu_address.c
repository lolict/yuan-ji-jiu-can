/**
 * gongyueshu_address.c — 公约数地址容器
 *
 * 核心: 用6个基韵锚 (公约数) 折叠所有韵母
 *   全长韵母名 "ai" → 路径 [a,i] → 编码为 2×3bit = 6bit
 *   全长韵母名 "iang" → 路径 [i,a,ng] → 编码为 3×3bit = 9bit
 *
 * 地址结构 (紧凑编码):
 *   ┌────────┬───────────┬──────────┬────────┐
 *   │ 声母   │ 韵母路径  │ 路径长度 │ 声调   │
 *   │ 6 bit  │ 3bit×n    │ 2 bit    │ 2 bit  │
 *   └────────┴───────────┴──────────┴────────┘
 *   总计: 10-20 bit (2-4字节), 对比全长拼写 5-10字节
 *
 * 折叠表: 每个韵母 = 一个公约数序列
 *   单韵母: a→[a], o→[o], e→[e], i→[i], u→[u], ü→[ü]
 *   复韵母: ai→[a,i], ao→[a,o], ei→[e,i], ou→[o,u]...
 *   鼻韵母: an→[a,n标记], ang→[a,ng标记]...
 *
 * 产权: 本容器有自己的编码系统 (公约数路径), 不被外部编码统治
 */

#include "gongyueshu_address.h"
#include "global_interlock.h"
#include <string.h>
#include "symbol_chain.h"

/* ═══════════════════════════════════════════════════════════════
 * 公约数常量 (6个基韵锚) — 已统一到 global_interlock.h
 * 此处仅保留特殊路径标记
 * ═══════════════════════════════════════════════════════════════ */

/* 特殊路径标记 (占用 6,7 这两个码点) */
#define MARK_N      GY_MARK_N   /* 前鼻音 -n 结尾 */
#define MARK_NG     GY_MARK_NG  /* 后鼻音 -ng 结尾 */

static const char *GONGYUESHU_NAMES[6] = {"a","o","e","i","u","ü"};

/* ═══════════════════════════════════════════════════════════════
 * 声母索引表 (48声 → 0-47)
 * ═══════════════════════════════════════════════════════════════ */

static const char *SHENG_NAMES[48] = {
    "b","p","m","f","d","t","n","l",
    "g","k","h","j","q","x","zh","ch",
    "sh","r","z","c","s","y","w","y",
    "y","w","ȵ","ŋ","ɕ","ʈʂ","ʈʂʰ","ʂ",
    "ts","tsʰ","tɕ","tɕʰ","x","ɣ","ɲ","ʑ",
    "tʂ","tʂʰ","ɖ","ɳ","ɭ","ʐ","ɻ","ʔ"
};

static int lookup_shengmu(const char *name) {
    for (int i = 0; i < 48; i++) {
        if (strcmp(name, SHENG_NAMES[i]) == 0) return i;
    }
    return -1;  /* 未找到 */
}

static int lookup_gongyueshu(const char *name) {
    for (int i = 0; i < 6; i++) {
        if (strcmp(name, GONGYUESHU_NAMES[i]) == 0) return i;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 折叠表: 韵母 → 公约数路径
 * 这是"折叠"操作的核心: 把长的韵母符号折成短的公约数序列
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    const char *name;       /* 韵母原名 */
    uint8_t path[4];        /* 公约数路径 */
    uint8_t len;            /* 路径长度 1-4 */
} fold_entry_t;

static const fold_entry_t FOLD_TABLE[] = {
    /* 单韵母 (自身) */
    {"a",   {GY_a}, 1},
    {"o",   {GY_o}, 1},
    {"e",   {GY_e}, 1},
    {"i",   {GY_i}, 1},
    {"u",   {GY_u}, 1},
    {"ü",   {GY_uu}, 1},
    {"er",  {GY_e, GY_a}, 2},     /* 卷舌元音: e + a (近似) */
    
    /* 二合元音 */
    {"ai",  {GY_a, GY_i}, 2},
    {"ei",  {GY_e, GY_i}, 2},
    {"ao",  {GY_a, GY_u}, 2},     /* ao 实际发音 [au] */
    {"ou",  {GY_o, GY_u}, 2},
    {"ia",  {GY_i, GY_a}, 2},
    {"ie",  {GY_i, GY_e}, 2},
    {"io",  {GY_i, GY_o}, 2},
    {"iu",  {GY_i, GY_u}, 2},     /* iou 缩写 */
    {"ua",  {GY_u, GY_a}, 2},
    {"uo",  {GY_u, GY_o}, 2},
    {"ue",  {GY_u, GY_e}, 2},     /* üe 省略两点 */
    {"üe",  {GY_uu, GY_e}, 2},
    
    /* 三合元音 */
    {"iao", {GY_i, GY_a, GY_u}, 3},
    {"iou", {GY_i, GY_o, GY_u}, 3},  /* iu 全写 */
    {"uai", {GY_u, GY_a, GY_i}, 3},
    {"uei", {GY_u, GY_e, GY_i}, 3},  /* ui 全写 */
    
    /* 前鼻音 (-n) */
    {"an",  {GY_a, MARK_N}, 2},
    {"en",  {GY_e, MARK_N}, 2},
    {"in",  {GY_i, MARK_N}, 2},
    {"un",  {GY_u, MARK_N}, 2},
    {"ün",  {GY_uu, MARK_N}, 2},
    {"ian", {GY_i, GY_a, MARK_N}, 3},
    {"uan", {GY_u, GY_a, MARK_N}, 3},
    {"üan", {GY_uu, GY_a, MARK_N}, 3},
    {"uen", {GY_u, GY_e, MARK_N}, 3},  /* un 全写 */
    
    /* 后鼻音 (-ng) */
    {"ang", {GY_a, MARK_NG}, 2},
    {"eng", {GY_e, MARK_NG}, 2},
    {"ing", {GY_i, MARK_NG}, 2},
    {"ong", {GY_o, MARK_NG}, 2},
    {"iang",{GY_i, GY_a, MARK_NG}, 3},
    {"iong",{GY_i, GY_o, MARK_NG}, 3},
    {"uang",{GY_u, GY_a, MARK_NG}, 3},
    {"ueng",{GY_u, GY_e, MARK_NG}, 3},
    
    /* 特殊 */
    {"iong",{GY_i, GY_o, MARK_NG}, 3},
};

#define FOLD_TABLE_SIZE (sizeof(FOLD_TABLE) / sizeof(FOLD_TABLE[0]))

static int lookup_fold(const char *yunmu, uint8_t *path, uint8_t *len) {
    for (uint16_t i = 0; i < FOLD_TABLE_SIZE; i++) {
        if (strcmp(yunmu, FOLD_TABLE[i].name) == 0) {
            memcpy(path, FOLD_TABLE[i].path, FOLD_TABLE[i].len);
            *len = FOLD_TABLE[i].len;
            return 0;
        }
    }
    return -1;  /* 未找到折叠规则 */
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void gongyueshu_init(gongyueshu_addr_t *ga) {
    memset(ga, 0, sizeof(*ga));
    ga->magic = GONGYUESHU_MAGIC;
    ga->version = 1;
    ga->entry_count = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码: 声母+韵母+声调 → 紧凑地址
 * ═══════════════════════════════════════════════════════════════ */

int gongyueshu_encode(gongyueshu_addr_t *ga, const char *shengmu,
                       const char *yunmu, uint8_t tone) {
    if (!ga || ga->magic != GONGYUESHU_MAGIC) return -1;
    if (ga->entry_count >= GONGYUESHU_MAX_ENTRIES) return -1;
    if (tone < 1 || tone > 4) return -1;
    
    gongyueshu_entry_t *entry = &ga->entries[ga->entry_count];
    memset(entry, 0, sizeof(*entry));
    
    /* 声母 → 索引 */
    int sm = lookup_shengmu(shengmu);
    entry->shengmu_idx = (sm >= 0) ? (uint8_t)sm : 0xFF;  /* 0xFF = 零声母 */
    
    /* 韵母 → 折叠为公约数路径 */
    uint8_t path[4];
    uint8_t path_len;
    int fold_ok = lookup_fold(yunmu, path, &path_len);
    
    if (fold_ok == 0) {
        memcpy(entry->gy_path, path, path_len);
        entry->path_len = path_len;
    } else {
        /* 未在折叠表中: 尝试直接匹配单韵母 */
        int gy = lookup_gongyueshu(yunmu);
        if (gy >= 0) {
            entry->gy_path[0] = (uint8_t)gy;
            entry->path_len = 1;
        } else {
            return -1;  /* 无法编码 */
        }
    }
    
    /* 声调 */
    entry->tone = tone;
    
    /* 计算紧凑位编码 */
    /* 位布局: shengmu(6) | path_len(2) | path(3×n) | tone(2) */
    uint32_t bits = 0;
    uint8_t pos = 0;
    
    /* 声母: 6 bits */
    bits |= ((uint32_t)entry->shengmu_idx & 0x3F) << pos;
    pos += 6;
    
    /* 路径长度: 2 bits (0=1, 1=2, 2=3, 3=4) */
    bits |= ((uint32_t)(path_len - 1) & 0x03) << pos;
    pos += 2;
    
    /* 路径: 3 bits × path_len */
    for (uint8_t i = 0; i < path_len; i++) {
        bits |= ((uint32_t)entry->gy_path[i] & 0x07) << pos;
        pos += 3;
    }
    
    /* 声调: 2 bits */
    bits |= ((uint32_t)(tone - 1) & 0x03) << pos;
    pos += 2;
    
    entry->packed_bits = bits;
    entry->bit_length = pos;
    
    /* 同时存储完整参数 (供声音进制直接使用) */
    entry->frequency = gongyueshu_freq(path[0]);  /* 取路径第一个公约数的频率 */
    entry->contour = tone - 1;  /* 0=平 1=升 2=降 3=凹 */
    entry->duration = 200 + path_len * 20;  /* 基础200ms + 每路径点20ms */
    entry->amplitude = 200;
    
    ga->entry_count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 解码: 紧凑地址 → 完整参数
 * ═══════════════════════════════════════════════════════════════ */

int gongyueshu_decode(const gongyueshu_addr_t *ga, uint16_t idx,
                       char *shengmu_out, char *yunmu_out,
                       uint8_t *tone_out, uint16_t *freq_out) {
    if (!ga || ga->magic != GONGYUESHU_MAGIC) return -1;
    if (idx >= ga->entry_count) return -1;
    
    const gongyueshu_entry_t *entry = &ga->entries[idx];
    
    /* 解码声母 */
    if (entry->shengmu_idx < 48) {
        strcpy(shengmu_out, SHENG_NAMES[entry->shengmu_idx]);
    } else {
        strcpy(shengmu_out, "");  /* 零声母 */
    }
    
    /* 解码韵母: 从路径反推 (取第一个公约数作为代表) */
    if (entry->path_len > 0 && entry->gy_path[0] < 6) {
        strcpy(yunmu_out, GONGYUESHU_NAMES[entry->gy_path[0]]);
    } else {
        strcpy(yunmu_out, "?");
    }
    
    *tone_out = entry->tone;
    *freq_out = entry->frequency;
    
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 获取紧凑位表示
 * ═══════════════════════════════════════════════════════════════ */

uint32_t gongyueshu_get_packed(const gongyueshu_addr_t *ga, uint16_t idx) {
    if (!ga || ga->magic != GONGYUESHU_MAGIC) return 0;
    if (idx >= ga->entry_count) return 0;
    return ga->entries[idx].packed_bits;
}

uint8_t gongyueshu_get_bit_length(const gongyueshu_addr_t *ga, uint16_t idx) {
    if (!ga || ga->magic != GONGYUESHU_MAGIC) return 0;
    if (idx >= ga->entry_count) return 0;
    return ga->entries[idx].bit_length;
}

/* ═══════════════════════════════════════════════════════════════
 * 频率映射: 公约数 → 基频 (与声音进制共享)
 * ═══════════════════════════════════════════════════════════════ */

uint16_t gongyueshu_freq(uint8_t gongyueshu_idx) {
    static const uint16_t FREQ[6] = {
        850,   /* a  850Hz */
        450,   /* o  450Hz */
        300,   /* e  300Hz */
        2800,  /* i  2800Hz */
        350,   /* u  350Hz */
        2200,  /* ü  2200Hz */
    };
    if (gongyueshu_idx >= 6) return 500;
    return FREQ[gongyueshu_idx];
}

/* ═══════════════════════════════════════════════════════════════
 * 统计
 * ═══════════════════════════════════════════════════════════════ */

void gongyueshu_stats(const gongyueshu_addr_t *ga, FILE *out) {
    if (!ga || ga->magic != GONGYUESHU_MAGIC) return;
    
    fprintf(out, "─── 公约数地址容器 ───\n");
    fprintf(out, "条目数: %d / %d\n", ga->entry_count, GONGYUESHU_MAX_ENTRIES);
    fprintf(out, "编码密度: 10-20 bit/地址 (对比全长拼写 40-80 bit)\n");
    fprintf(out, "压缩率: ~50%%\n");
    
    if (ga->entry_count > 0) {
        fprintf(out, "\n─── 地址列表 ───\n");
        for (uint16_t i = 0; i < ga->entry_count; i++) {
            const gongyueshu_entry_t *e = &ga->entries[i];
            fprintf(out, "  [%2d] 声母%02d 路径[", i, e->shengmu_idx);
            for (uint8_t j = 0; j < e->path_len; j++) {
                uint8_t g = e->gy_path[j];
                if (g < 6) fprintf(out, "%s", GONGYUESHU_NAMES[g]);
                else if (g == MARK_N) fprintf(out, "-n");
                else if (g == MARK_NG) fprintf(out, "-ng");
            }
            fprintf(out, "] 声%d  freq=%dHz  %dbit\n",
                    e->tone, e->frequency, e->bit_length);
        }
    }
}
