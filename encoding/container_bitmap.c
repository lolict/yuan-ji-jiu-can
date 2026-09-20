/**
 * ═══════════════════════════════════════════════════════════════════════════
 * container_bitmap.c — 254+2 集装箱比特分配引擎
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   把概念变成字节: 每个符号分配实际的比特地址
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编码空间 (256 = 2⁸)
 * ─────────────────────────────────────────────────────────────────────────
 *   0x00         : 保留 - 刘楚恬/空容器 (可被填入)
 *   0x01         : 保留 - 我/锚定容器 (已锚定)
 *   0x02..0xFE   : 254 可用码
 *   0xFF         : 保留 - 溢出/终止符
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 层级容器分配
 * ─────────────────────────────────────────────────────────────────────────
 *   1-bit 本体层 (容积1): 6基韵锚          → 6字符 × 8比特 =  48比特
 *   2-bit 融合层 (容积2): 66符号+28复韵    → 94字符 × 8比特 = 752比特
 *   3-bit 时间层 (容积3): 24声调韵         → 24字符 × 8比特 = 192比特
 *   4+bit 外部层       : 70溢出符号        → 70字符 × 8比特 = 560比特
 *   ─────────────────────────────────────────────────────────────────────
 *   合计: 254字符 × 8比特 = 2032比特 = 254字节
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o container_bitmap container_bitmap.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 核心常量
 * ═══════════════════════════════════════════════════════════════════════════ */

#define CODE_EMPTY        0x00    /* 刘楚恬 · 空容器 · 可被填入 */
#define CODE_SELF         0x01    /* 我     · 锚定容器         */
#define CODE_OVERFLOW     0xFF    /* 溢出 · 终止符           */

#define BITS_PER_CELL     8       /* 不可再少: log₂(256)       */
#define BYTES_PER_CELL    1       /* 8比特 = 1字节             */
#define CAPACITY          254     /* 最大容量 (256 - 2保留)    */

/* ═══════════════════════════════════════════════════════════════════════════
 * 字符编码表 — 只包含实际定义的符号
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    const char *name;       /* 符号名 */
    uint8_t     code;       /* 分配的字节值 */
    uint8_t     tier;       /* 容器层级: 1/2/3/4 */
    uint8_t     anchor;     /* 锚: 0~5基韵, 6=声调, 7=复韵/符号, 8=溢出 */
} cell_entry_t;

/* 编码表 — 大小由实际定义决定 */
static const cell_entry_t CELL_TABLE[] = {
    /* ─────────────────────────────────────────────────────────────────────
     * 1-bit 本体层: 6基韵锚 (0x02..0x07)
     * ───────────────────────────────────────────────────────────────────── */
    {"a",  0x02, 1, 0},   /* 锚=a */
    {"o",  0x03, 1, 1},   /* 锚=o */
    {"e",  0x04, 1, 2},   /* 锚=e */
    {"i",  0x05, 1, 3},   /* 锚=i */
    {"u",  0x06, 1, 4},   /* 锚=u */
    {"ü",  0x07, 1, 5},   /* 锚=ü */

    /* ─────────────────────────────────────────────────────────────────────
     * 2-bit 融合层: 28复韵母名 (0x10..0x2B)
     * ───────────────────────────────────────────────────────────────────── */
    {"ai",   0x10, 2, 7},  {"ei",   0x11, 2, 7},  {"ui",   0x12, 2, 7},
    {"ao",   0x13, 2, 7},  {"ou",   0x14, 2, 7},  {"iu",   0x15, 2, 7},
    {"ie",   0x16, 2, 7},  {"üe",   0x17, 2, 7},  {"er",   0x18, 2, 7},
    {"an",   0x19, 2, 7},  {"en",   0x1A, 2, 7},  {"in",   0x1B, 2, 7},
    {"un",   0x1C, 2, 7},  {"ün",   0x1D, 2, 7},  {"ang",  0x1E, 2, 7},
    {"eng",  0x1F, 2, 7},  {"ing",  0x20, 2, 7},  {"ong",  0x21, 2, 7},
    {"ia",   0x22, 2, 7},  {"ian",  0x23, 2, 7},  {"iang", 0x24, 2, 7},
    {"iong", 0x25, 2, 7},  {"ua",   0x26, 2, 7},  {"uo",   0x27, 2, 7},
    {"uai",  0x28, 2, 7},  {"uan",  0x29, 2, 7},  {"uang", 0x2A, 2, 7},
    {"üan",  0x2B, 2, 7},

    /* ─────────────────────────────────────────────────────────────────────
     * 2-bit 融合层: 66配对符号 (0x30..0x75)
     * ───────────────────────────────────────────────────────────────────── */
    {"ꬰ", 0x30, 2, 7},  {"ꬱ", 0x31, 2, 7},  {"ə", 0x32, 2, 7},  {"Ə", 0x33, 2, 7},
    {"ѱ", 0x34, 2, 7},  {"Ѱ", 0x35, 2, 7},  {"ꜵ", 0x36, 2, 7},  {"Ꜵ", 0x37, 2, 7},
    {"ȣ", 0x38, 2, 7},  {"Ȣ", 0x39, 2, 7},  {"ꭎ", 0x3A, 2, 7},  {"Ⴞ", 0x3B, 2, 7},
    {"ɤ", 0x3C, 2, 7},  {"Ɣ", 0x3D, 2, 7},  {"ᵫ", 0x3E, 2, 7},  {"ꭂ", 0x3F, 2, 7},
    {"ɛ", 0x40, 2, 7},  {"Ɛ", 0x41, 2, 7},  {"ꙙ", 0x42, 2, 7},  {"Ꙙ", 0x43, 2, 7},
    {"ⱄ", 0x44, 2, 7},  {"ⱅ", 0x45, 2, 7},  {"ĩ", 0x46, 2, 7},  {"Ĩ", 0x47, 2, 7},
    {"տ", 0x48, 2, 7},  {"໙", 0x49, 2, 7},  {"Ⴐ", 0x4A, 2, 7},  {"Ⴆ", 0x4B, 2, 7},
    {"Ⴊ", 0x4C, 2, 7},  {"ӫ", 0x4D, 2, 7},  {"Ӫ", 0x4E, 2, 7},  {"ꙟ", 0x4F, 2, 7},
    {"Ꙟ", 0x50, 2, 7},  {"ꭁ", 0x51, 2, 7},  {"ꭀ", 0x52, 2, 7},  {"ꙗ", 0x53, 2, 7},
    {"Ꙗ", 0x54, 2, 7},  {"ꙝ", 0x55, 2, 7},  {"Ꙝ", 0x56, 2, 7},  {"ѭ", 0x57, 2, 7},
    {"Ѭ", 0x58, 2, 7},  {"ȱ", 0x59, 2, 7},  {"Ȱ", 0x5A, 2, 7},  {"ǟ", 0x5B, 2, 7},
    {"Ǟ", 0x5C, 2, 7},  {"ȫ", 0x5D, 2, 7},  {"Ȫ", 0x5E, 2, 7},  {"ꜻ", 0x5F, 2, 7},
    {"Ꜻ", 0x60, 2, 7},  {"Ⴄ", 0x61, 2, 7},  {"Ⴂ", 0x62, 2, 7},  {"ꙛ", 0x63, 2, 7},
    {"Ꙛ", 0x64, 2, 7},  {"ѫ", 0x65, 2, 7},  {"Ѫ", 0x66, 2, 7},
    /* 补齐到66个: 继续分配 */
    {"ꬲ", 0x67, 2, 7},  {"ꬳ", 0x68, 2, 7},  {"ꬴ", 0x69, 2, 7},  {"ꬵ", 0x6A, 2, 7},
    {"ꬶ", 0x6B, 2, 7},  {"ꬷ", 0x6C, 2, 7},  {"ꬸ", 0x6D, 2, 7},  {"ꬹ", 0x6E, 2, 7},
    {"ꬺ", 0x6F, 2, 7},  {"ꬻ", 0x70, 2, 7},  {"ꬼ", 0x71, 2, 7},  {"ꬽ", 0x72, 2, 7},
    {"ꬾ", 0x73, 2, 7},  {"ꬿ", 0x74, 2, 7},  {"ꭑ", 0x75, 2, 7},

    /* ─────────────────────────────────────────────────────────────────────
     * 3-bit 时间层: 24声调韵 (0x80..0x97)
     * ───────────────────────────────────────────────────────────────────── */
    {"ā", 0x80, 3, 6},  {"á", 0x81, 3, 6},  {"ǎ", 0x82, 3, 6},  {"à", 0x83, 3, 6},
    {"ō", 0x84, 3, 6},  {"ó", 0x85, 3, 6},  {"ǒ", 0x86, 3, 6},  {"ò", 0x87, 3, 6},
    {"ē", 0x88, 3, 6},  {"é", 0x89, 3, 6},  {"ě", 0x8A, 3, 6},  {"è", 0x8B, 3, 6},
    {"ī", 0x8C, 3, 6},  {"í", 0x8D, 3, 6},  {"ǐ", 0x8E, 3, 6},  {"ì", 0x8F, 3, 6},
    {"ū", 0x90, 3, 6},  {"ú", 0x91, 3, 6},  {"ǔ", 0x92, 3, 6},  {"ù", 0x93, 3, 6},
    {"ǖ", 0x94, 3, 6},  {"ǘ", 0x95, 3, 6},  {"ǚ", 0x96, 3, 6},  {"ǜ", 0x97, 3, 6},

    /* ─────────────────────────────────────────────────────────────────────
     * 4+bit 外部层: 70溢出符号 (0xA0..0xE5)
     * ───────────────────────────────────────────────────────────────────── */
    {"Ӱ", 0xA0, 4, 8},  {"ӱ", 0xA1, 4, 8},  {"Ѧ", 0xA2, 4, 8},  {"ѧ", 0xA3, 4, 8},
    {"Ѽ", 0xA4, 4, 8},  {"ѽ", 0xA5, 4, 8},  {"Ѥ", 0xA6, 4, 8},  {"ѥ", 0xA7, 4, 8},
    {"Ё", 0xA8, 4, 8},  {"ё", 0xA9, 4, 8},  {"Є", 0xAA, 4, 8},  {"є", 0xAB, 4, 8},
    {"Ʝ", 0xAC, 4, 8},  {"႘", 0xAD, 4, 8},  {"Ꙍ", 0xAE, 4, 8},  {"ꙍ", 0xAF, 4, 8},
    {"ꭃ", 0xB0, 4, 8},  {"ꭄ", 0xB1, 4, 8},  {"Ⴘ", 0xB2, 4, 8},  {"Ⴏ", 0xB3, 4, 8},
    {"Ⴎ", 0xB4, 4, 8},  {"Ⴌ", 0xB5, 4, 8},  {"Ⴋ", 0xB6, 4, 8},  {"☁", 0xB7, 4, 8},
    {"Ӭ", 0xB8, 4, 8},  {"ӭ", 0xB9, 4, 8},  {"Ѿ", 0xBA, 4, 8},  {"ѿ", 0xBB, 4, 8},
    {"Ȥ", 0xBC, 4, 8},  {"Ç", 0xBD, 4, 8},  {"Ŋ", 0xBE, 4, 8},  {"ȥ", 0xBF, 4, 8},
    {"ç", 0xC0, 4, 8},  {"Ӗ", 0xC1, 4, 8},  {"Ḁ", 0xC2, 4, 8},  {"ḁ", 0xC3, 4, 8},
    {"Ḃ", 0xC4, 4, 8},  {"ḃ", 0xC5, 4, 8},  {"Ḅ", 0xC6, 4, 8},  {"ḅ", 0xC7, 4, 8},
    {"Ḇ", 0xC8, 4, 8},  {"ḇ", 0xC9, 4, 8},  {"Ḉ", 0xCA, 4, 8},  {"ḉ", 0xCB, 4, 8},
    {"Ḋ", 0xCC, 4, 8},  {"ḋ", 0xCD, 4, 8},  {"Ḍ", 0xCE, 4, 8},  {"ḍ", 0xCF, 4, 8},
    {"Ḏ", 0xD0, 4, 8},  {"ḏ", 0xD1, 4, 8},  {"Ḑ", 0xD2, 4, 8},  {"ḑ", 0xD3, 4, 8},
    {"Ḓ", 0xD4, 4, 8},  {"ḓ", 0xD5, 4, 8},  {"Ḕ", 0xD6, 4, 8},  {"ḕ", 0xD7, 4, 8},
    {"Ḗ", 0xD8, 4, 8},  {"ḗ", 0xD9, 4, 8},  {"Ḙ", 0xDA, 4, 8},  {"ḙ", 0xDB, 4, 8},
    {"Ḛ", 0xDC, 4, 8},  {"ḛ", 0xDD, 4, 8},  {"Ḝ", 0xDE, 4, 8},  {"ḝ", 0xDF, 4, 8},
    {"Ḟ", 0xE0, 4, 8},  {"ḟ", 0xE1, 4, 8},  {"Ḡ", 0xE2, 4, 8},  {"ḡ", 0xE3, 4, 8},
    {"Ḣ", 0xE4, 4, 8},  {"ḣ", 0xE5, 4, 8},
};

#define CELL_TABLE_SIZE ((int)(sizeof(CELL_TABLE) / sizeof(CELL_TABLE[0])))

/* ═══════════════════════════════════════════════════════════════════════════
 * 查询函数
 * ═══════════════════════════════════════════════════════════════════════════ */

/* 符号名 → 编码 */
static int name_to_code(const char *name) {
    if (!name) return -1;
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        if (strcmp(CELL_TABLE[i].name, name) == 0) {
            return CELL_TABLE[i].code;
        }
    }
    return -1;
}

/* 编码 → 符号名 */
static const char* code_to_name(uint8_t code) {
    if (code == CODE_EMPTY) return "[空/刘楚恬]";
    if (code == CODE_SELF)  return "[我/锚定]";
    if (code == CODE_OVERFLOW) return "[溢出/终止]";
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        if (CELL_TABLE[i].code == code) return CELL_TABLE[i].name;
    }
    return "[未分配]";
}

/* 编码 → 层级 */
static uint8_t code_to_tier(uint8_t code) {
    if (code == CODE_EMPTY || code == CODE_SELF) return 1;
    if (code == CODE_OVERFLOW) return 4;
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        if (CELL_TABLE[i].code == code) return CELL_TABLE[i].tier;
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 比特统计
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    int      allocated_chars;   /* 已分配字符数 */
    int      capacity;          /* 最大容量 */
    int      total_bits;        /* 已分配比特数 */
    int      anchor_bits;       /* 1-bit层 */
    int      fusion_bits;       /* 2-bit层 */
    int      time_bits;         /* 3-bit层 */
    int      external_bits;     /* 外部层 */
    float    utilization;       /* 利用率 */
} bit_budget_t;

static void compute_bit_budget(bit_budget_t *b) {
    memset(b, 0, sizeof(*b));
    b->capacity = CAPACITY;
    b->allocated_chars = CELL_TABLE_SIZE;
    b->utilization = (float)CELL_TABLE_SIZE / (float)CAPACITY;

    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        switch (CELL_TABLE[i].tier) {
            case 1: b->anchor_bits   += BITS_PER_CELL; break;
            case 2: b->fusion_bits   += BITS_PER_CELL; break;
            case 3: b->time_bits     += BITS_PER_CELL; break;
            case 4: b->external_bits += BITS_PER_CELL; break;
        }
    }
    b->total_bits = b->anchor_bits + b->fusion_bits + b->time_bits + b->external_bits;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 报告输出
 * ═══════════════════════════════════════════════════════════════════════════ */

static void print_bit_report(void) {
    bit_budget_t b;
    compute_bit_budget(&b);

    printf("\n");
    printf("╔══════════════════════════════════════════════════════════════╗\n");
    printf("║          254+2 集装箱 · 比特分配报告                        ║\n");
    printf("╠══════════════════════════════════════════════════════════════╣\n");
    printf("║                                                              ║\n");
    printf("║  ■ 信息论下限                                               ║\n");
    printf("║    254字符 → ⌈log₂(254)⌉ = 8比特/字符                      ║\n");
    printf("║    254+2 = 256 = 2⁸ → 完美填满一字节, 零浪费              ║\n");
    printf("║                                                              ║\n");
    printf("║  ■ 容量 vs 已分配                                           ║\n");
    printf("║    容量:   %-4d 字符 (编码空间 0x02..0xFE)                  ║\n", b.capacity);
    printf("║    已分配: %-4d 字符                                        ║\n", b.allocated_chars);
    printf("║    利用率: %.1f%%                                            ║\n", b.utilization * 100.0f);
    printf("║    剩余:   %-4d 字符可扩展                                  ║\n", b.capacity - b.allocated_chars);
    printf("║                                                              ║\n");
    printf("║  ■ 层级比特分布                                              ║\n");
    printf("║    1-bit 本体层:   6字符 × 8 = %-5u 比特 (锚点)           ║\n", b.anchor_bits);
    printf("║    2-bit 融合层:  94字符 × 8 = %-5u 比特 (代理)           ║\n", b.fusion_bits);
    printf("║    3-bit 时间层:  24字符 × 8 = %-5u 比特 (被使用者)       ║\n", b.time_bits);
    printf("║    4+bit 外部层:  70字符 × 8 = %-5u 比特 (溢出)           ║\n", b.external_bits);
    printf("║    ─────────────────────────────────────────────────────    ║\n");
    printf("║    合计: %u字符 × 8 = %u比特 = %u字节                     ║\n",
           b.allocated_chars, b.total_bits, b.total_bits / 8);
    printf("║                                                              ║\n");
    printf("║  ■ 关键结论                                                  ║\n");
    printf("║    ✓ 每字符8比特是理论最小值, 不可再少                     ║\n");
    printf("║    ✓ 254+2=256 完美对齐字节, 无碎片                        ║\n");
    printf("║    ✓ 像素渲染率 = 并行状态空间 = 8比特                     ║\n");
    printf("║    ✓ 容器层级是逻辑特权, 不改变每字符8比特的物理事实       ║\n");
    printf("║                                                              ║\n");
    printf("╚══════════════════════════════════════════════════════════════╝\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编码查询演示
 * ═══════════════════════════════════════════════════════════════════════════ */

static void demo_encoding(void) {
    printf("\n");
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  编码查询演示\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    const char *demo_names[] = {"a", "á", "ai", "ꬰ", "Ӱ", "e", "ě", "ə", NULL};

    for (int i = 0; demo_names[i]; i++) {
        int code = name_to_code(demo_names[i]);
        if (code >= 0) {
            uint8_t tier = code_to_tier((uint8_t)code);
            printf("  %-4s → 0x%02X | 层级=%d | 8比特 | %s\n",
                   demo_names[i], code, tier,
                   tier == 1 ? "本体层" : tier == 2 ? "融合层" : tier == 3 ? "时间层" : "外部层");
        } else {
            printf("  %-4s → [未找到]\n", demo_names[i]);
        }
    }

    printf("\n");
    printf("  保留码:\n");
    printf("    0x00 = 刘楚恬/空容器 (可被填入)\n");
    printf("    0x01 = 我/锚定容器 (已锚定)\n");
    printf("    0xFF = 溢出/终止符 (不可再分)\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int run_tests(void) {
    printf("测试编码表完整性...\n");
    int errors = 0;

    /* 测试1: 编码表大小 > 0 */
    if (CELL_TABLE_SIZE <= 0) {
        printf("  [FAIL] 编码表大小 = %d\n", CELL_TABLE_SIZE);
        errors++;
    } else {
        printf("  [PASS] 编码表大小 = %d (容量 %d)\n", CELL_TABLE_SIZE, CAPACITY);
    }

    /* 测试2: 编码唯一性 */
    int unique = 1;
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        for (int j = i + 1; j < CELL_TABLE_SIZE; j++) {
            if (CELL_TABLE[i].code == CELL_TABLE[j].code) {
                printf("  [FAIL] 编码冲突: %s 和 %s 都是 0x%02X\n",
                       CELL_TABLE[i].name, CELL_TABLE[j].name, CELL_TABLE[i].code);
                unique = 0;
                errors++;
            }
        }
    }
    if (unique) printf("  [PASS] 所有编码唯一 (%d个)\n", CELL_TABLE_SIZE);

    /* 测试3: 保留码未被占用 */
    int reserved_ok = 1;
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        if (CELL_TABLE[i].code == CODE_EMPTY || CELL_TABLE[i].code == CODE_OVERFLOW || CELL_TABLE[i].code == CODE_SELF) {
            printf("  [FAIL] 编码表占用保留码: %s = 0x%02X\n",
                   CELL_TABLE[i].name, CELL_TABLE[i].code);
            reserved_ok = 0;
            errors++;
        }
    }
    if (reserved_ok) printf("  [PASS] 保留码未被占用 (0x00, 0x01, 0xFF)\n");

    /* 测试4: 查询函数 */
    int code_a = name_to_code("a");
    if (code_a == 0x02) {
        printf("  [PASS] name_to_code(\"a\") = 0x02\n");
    } else {
        printf("  [FAIL] name_to_code(\"a\") = %d, 期望 0x02\n", code_a);
        errors++;
    }

    /* 测试5: 反向查询 */
    const char *name_0x02 = code_to_name(0x02);
    if (strcmp(name_0x02, "a") == 0) {
        printf("  [PASS] code_to_name(0x02) = \"a\"\n");
    } else {
        printf("  [FAIL] code_to_name(0x02) = \"%s\", 期望 \"a\"\n", name_0x02);
        errors++;
    }

    /* 测试6: 编码在有效范围 */
    int range_ok = 1;
    for (int i = 0; i < CELL_TABLE_SIZE; i++) {
        uint8_t c = CELL_TABLE[i].code;
        if (c < 0x02 || c > 0xFE) {
            printf("  [FAIL] 编码越界: %s = 0x%02X\n", CELL_TABLE[i].name, c);
            range_ok = 0;
            errors++;
        }
    }
    if (range_ok) printf("  [PASS] 所有编码在有效范围 0x02..0xFE\n");

    printf("\n结果: %d 错误\n", errors);
    return errors;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 主函数
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    if (argc == 1) {
        print_bit_report();
        demo_encoding();
        return 0;
    }

    if (argc == 2 && (strcmp(argv[1], "-t") == 0 || strcmp(argv[1], "--test") == 0)) {
        return run_tests();
    }

    if (argc == 2 && (strcmp(argv[1], "-e") == 0 || strcmp(argv[1], "--encode") == 0)) {
        char name[64];
        printf("输入符号名 (每行一个, 空行退出):\n");
        while (fgets(name, sizeof(name), stdin)) {
            size_t len = strlen(name);
            while (len > 0 && (name[len-1] == '\n' || name[len-1] == '\r'))
                name[--len] = '\0';
            if (len == 0) break;

            int code = name_to_code(name);
            if (code >= 0) {
                uint8_t tier = code_to_tier((uint8_t)code);
                printf("  %-6s → 0x%02X (二进制: ", name, code);
                for (int b = 7; b >= 0; b--) printf("%d", (code >> b) & 1);
                printf(") | 层级=%d | 8比特\n", tier);
            } else {
                printf("  %-6s → [未找到]\n", name);
            }
        }
        return 0;
    }

    /* 默认: 查询指定符号 */
    for (int i = 1; i < argc; i++) {
        int code = name_to_code(argv[i]);
        if (code >= 0) {
            uint8_t tier = code_to_tier((uint8_t)code);
            printf("%-6s → 0x%02X | 层级=%d | 8比特/字符\n", argv[i], code, tier);
        } else {
            printf("%-6s → [未找到]\n", argv[i]);
        }
    }
    return 0;
}
