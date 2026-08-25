/**
 * ═══════════════════════════════════════════════════════════════════════════
 * fqm_base64.c — 自定义64进制加载器
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   不依赖系统ASCII/Unicode, 从文本文件读64个自定义符号。
 *   符号随时可增删, 不需重新编译核心。
 *
 *   两层结构:
 *     固定层(编译一次): 加载器 + 解析器
 *     可变层(动态加载): fqm.symbols 文件, 随时可改
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o fqm64 fqm_base64.c
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 核心常量
 * ═══════════════════════════════════════════════════════════════════════════ */

#define BASE64_COUNT  64
#define SYMBOL_LEN    16    /* 每个符号最大字节数 */
#define SYMBOLS_FILE  "fqm.symbols"

/* ═══════════════════════════════════════════════════════════════════════════
 * 符号表 (从文件动态加载)
 * ═══════════════════════════════════════════════════════════════════════════ */

static char g_symbols[BASE64_COUNT][SYMBOL_LEN];
static int  g_loaded = 0;

/* ═══════════════════════════════════════════════════════════════════════════
 * 加载符号表
 * ═══════════════════════════════════════════════════════════════════════════ */

static int load_symbols(const char *filename) {
    FILE *f = fopen(filename, "r");
    if (!f) {
        printf("  [ERROR] 无法打开符号文件: %s\n", filename);
        printf("  [INFO]  请创建该文件, 每行一个符号, 共64行\n");
        return -1;
    }
    
    int count = 0;
    while (count < BASE64_COUNT && fgets(g_symbols[count], SYMBOL_LEN, f)) {
        /* 去掉换行符 */
        size_t len = strlen(g_symbols[count]);
        while (len > 0 && (g_symbols[count][len-1] == '\n' || g_symbols[count][len-1] == '\r'))
            g_symbols[count][--len] = '\0';
        if (len > 0) count++;
    }
    fclose(f);
    
    g_loaded = (count == BASE64_COUNT);
    return count;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 编码: 字节 → 自定义64进制字符串
 * ═══════════════════════════════════════════════════════════════════════════ */

static int encode(const uint8_t *data, int len, char *out, int out_max) {
    if (!g_loaded) { strcpy(out, "[未加载符号表]"); return -1; }
    
    int pos = 0;
    int bits = 0;
    uint32_t buf = 0;
    
    for (int i = 0; i < len; i++) {
        buf = (buf << 8) | data[i];
        bits += 8;
        while (bits >= 6) {
            bits -= 6;
            int idx = (buf >> bits) & 0x3F;
            if (pos + (int)strlen(g_symbols[idx]) >= out_max) return pos;
            pos += sprintf(out + pos, "%s", g_symbols[idx]);
        }
    }
    if (bits > 0) {
        int idx = (buf << (6 - bits)) & 0x3F;
        if (pos + (int)strlen(g_symbols[idx]) < out_max)
            pos += sprintf(out + pos, "%s", g_symbols[idx]);
    }
    return pos;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 解码: 自定义64进制字符串 → 字节
 * ═══════════════════════════════════════════════════════════════════════════ */

static int decode(const char *str, uint8_t *out, int out_max) {
    if (!g_loaded) return -1;
    
    int pos = 0;
    int bits = 0;
    uint32_t buf = 0;
    int len = strlen(str);
    int i = 0;
    
    while (i < len && pos < out_max) {
        /* 尝试匹配每个符号 */
        int matched = -1;
        for (int s = 0; s < BASE64_COUNT; s++) {
            int slen = strlen(g_symbols[s]);
            if (strncmp(str + i, g_symbols[s], slen) == 0) {
                matched = s;
                i += slen;
                break;
            }
        }
        if (matched < 0) { i++; continue; } /* 跳过无法识别的字符 */
        
        buf = (buf << 6) | matched;
        bits += 6;
        if (bits >= 8) {
            bits -= 8;
            out[pos++] = (buf >> bits) & 0xFF;
        }
    }
    return pos;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 显示符号表
 * ═══════════════════════════════════════════════════════════════════════════ */

static void show_table(void) {
    printf("  自定义64进制符号表:\n");
    for (int i = 0; i < BASE64_COUNT; i++) {
        printf("    [%2d] %s\n", i, g_symbols[i]);
        if (i % 8 == 7) printf("\n");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int test_count = 0;
static int test_pass = 0;

#define TEST(name) do { test_count++; printf("  [TEST] %-40s ", name); } while(0)
#define PASS()     do { test_pass++; printf("✓\n"); } while(0)
#define FAIL(msg)  do { printf("✗ %s\n", msg); } while(0)

static void run_tests(void) {
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  自定义64进制加载器 · 测试套件\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    TEST("符号表已加载");
    if (g_loaded) PASS(); else FAIL("未加载");
    
    TEST("符号表数量=64");
    int count = 0;
    for (int i = 0; i < BASE64_COUNT; i++)
        if (strlen(g_symbols[i]) > 0) count++;
    if (count == 64) PASS(); else FAIL("数量不对");
    
    TEST("编码-解码往返");
    if (g_loaded) {
        uint8_t orig[] = {0x48, 0x65, 0x6C, 0x6C, 0x6F}; /* "Hello" */
        char encoded[256];
        uint8_t decoded[256];
        encode(orig, 5, encoded, sizeof(encoded));
        int dlen = decode(encoded, decoded, sizeof(decoded));
        if (dlen == 5 && memcmp(orig, decoded, 5) == 0) PASS();
        else FAIL("往返不一致");
    } else FAIL("未加载");
    
    TEST("编码输出非空");
    if (g_loaded) {
        uint8_t data[] = {0xFF};
        char encoded[256];
        encode(data, 1, encoded, sizeof(encoded));
        if (strlen(encoded) > 0) PASS(); else FAIL("输出为空");
    } else FAIL("未加载");
    
    printf("\n───────────────────────────────────────────────────────\n");
    printf("  结果: %d/%d 通过\n", test_pass, test_count);
    printf("───────────────────────────────────────────────────────\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void interactive(void) {
    char line[1024];
    printf("\n╔═══════════════════════════════════════════════════════╗\n");
    printf("║         自定义64进制加载器 · 交互模式                 ║\n");
    printf("╠═══════════════════════════════════════════════════════╣\n");
    printf("║  命令:                                               ║\n");
    printf("║    e <文本>          编码为自定义64进制               ║\n");
    printf("║    d <符号串>        解码为原文                       ║\n");
    printf("║    t                 显示符号表                      ║\n");
    printf("║    test              运行测试                        ║\n");
    printf("║    q                 退出                            ║\n");
    printf("╚═══════════════════════════════════════════════════════╝\n");
    
    while (1) {
        printf("\nfqm64> ");
        if (!fgets(line, sizeof(line), stdin)) break;
        line[strcspn(line, "\n")] = '\0';
        if (strlen(line) == 0) continue;
        
        if (line[0] == 'q') break;
        if (line[0] == 't') { show_table(); continue; }
        if (line[0] == 'e' && line[1] == ' ') {
            uint8_t data[512];
            int len = strlen(line + 2);
            memcpy(data, line + 2, len);
            char encoded[2048];
            encode(data, len, encoded, sizeof(encoded));
            printf("  编码: %s\n", encoded);
        }
        else if (line[0] == 'd' && line[1] == ' ') {
            uint8_t decoded[512];
            int dlen = decode(line + 2, decoded, sizeof(decoded));
            if (dlen > 0) {
                printf("  解码: ");
                for (int i = 0; i < dlen; i++) {
                    if (decoded[i] >= 32 && decoded[i] < 127)
                        putchar(decoded[i]);
                    else
                        printf("\\x%02X", decoded[i]);
                }
                printf("\n");
            } else {
                printf("  解码失败\n");
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 入口
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    printf("═══════════════════════════════════════════════════════\n");
    printf("  自定义64进制加载器\n");
    printf("═══════════════════════════════════════════════════════\n");
    
    /* 加载符号表 */
    int n = load_symbols(SYMBOLS_FILE);
    if (n < 0) {
        printf("\n  请先创建 %s 文件, 每行一个符号, 共64行。\n", SYMBOLS_FILE);
        printf("  示例:\n");
        printf("    者\n");
        printf("    这\n");
        printf("    做\n");
        printf("    ...(共64行)\n");
        return 1;
    }
    printf("  已加载 %d/%d 个符号\n", n, BASE64_COUNT);
    
    if (argc > 1) {
        if (strcmp(argv[1], "-t") == 0) { run_tests(); return 0; }
        if (strcmp(argv[1], "-i") == 0) { interactive(); return 0; }
    }
    
    /* 默认: 显示符号表 + 测试 */
    show_table();
    run_tests();
    return 0;
}
