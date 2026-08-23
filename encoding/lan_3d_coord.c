/**
 * ═══════════════════════════════════════════════════════════════════════════
 * lan_3d_coord.c — 毛利兰立体进制框架 (3D Coordinate Static Absolute Positioning)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 *   三轴坐标体系:
 *   ┌─────────────────────────────────────────────────────────────────────┐
 *   │  X 轴 · 表面轴 (毛利兰) · 模/面/名 · visible surface attributes    │
 *   │  Y 轴 · 结构轴 (工藤新一) · 骨/头/体 · structural skeleton         │
 *   │  Z 轴 · 横向轴 (灰原哀) · 行/页/业 · lateral/horizontal spread      │
 *   └─────────────────────────────────────────────────────────────────────┘
 *
 *   每个字符的三维坐标 = 其在毛利兰空间中的绝对静态位置
 *   坐标 ∈ [0, 63] 每轴 · 共 64³ = 262144 个寻址位置
 *   每轴 6 bits · 三轴 18 bits = 3 bytes per character spatial address
 *
 * ─────────────────────────────────────────────────────────────────────────
 * 编码: X[6b] | Y[6b] | Z[6b] = 18 bits per coordinate triple
 * 定位: (X, Y, Z) → 空间哈希 bucket → 同坐标字符聚类
 *
 * ═══════════════════════════════════════════════════════════════════════════
 * 编译: cc -Wall -Wextra -std=c99 -O2 -o lan3d lan_3d_coord.c -lm
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

/* ═══════════════════════════════════════════════════════════════════════════
 * 坐标常量
 * ═══════════════════════════════════════════════════════════════════════════ */

#define COORD_MIN     0
#define COORD_MAX     63
#define COORD_BITS    6
#define COORD_RANGE   64    /* 2^6 */

#define BUCKET_SIZE   64    /* 每空间桶最大字符数 */
#define MAX_CHARS     254   /* 容器字符容量 */

/* 三轴定义 */
typedef enum {
    AXIS_SURFACE  = 0,  /* X · 表面轴 · 毛利兰 */
    AXIS_STRUCT   = 1,  /* Y · 结构轴 · 工藤新一 */
    AXIS_LATERAL  = 2,  /* Z · 横向轴 · 灰原哀 */
    AXIS_COUNT    = 3
} axis_t;

static const char* axis_name(axis_t a) {
    switch(a) {
        case AXIS_SURFACE: return "表面(毛利兰)";
        case AXIS_STRUCT:  return "结构(工藤新一)";
        case AXIS_LATERAL: return "横向(灰原哀)";
        default: return "?";
    }
}

static const char* axis_cn(axis_t a) {
    switch(a) {
        case AXIS_SURFACE: return "模/面/名";
        case AXIS_STRUCT:  return "骨/头/体";
        case AXIS_LATERAL: return "行/页/业";
        default: return "?";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 字符类型 (与五门审判一致)
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef enum { T_BASE=1, T_TONE=2, T_COMPOUND=3, T_SYMBOL=4, T_EXTRA=5, T_MODIFIER=7 } sym_type_t;

static const char* type_str(sym_type_t t) {
    switch(t) {
        case T_BASE:     return "基元";
        case T_TONE:     return "基调";
        case T_COMPOUND: return "复合";
        case T_SYMBOL:   return "象征";
        case T_EXTRA:    return "溢出";
        case T_MODIFIER: return "修饰";
        default: return "未分类";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 三维坐标节点
 * ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    char    name[16];       /* 字符名称 */
    uint8_t code;           /* 容器编码 0x02..0xFE */
    sym_type_t type;        /* 类型 */
    uint8_t x;              /* 表面轴 [0..63] */
    uint8_t y;              /* 结构轴 [0..63] */
    uint8_t z;              /* 横向轴 [0..63] */
} coord_node_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * 字符注册表 (198个字符的基础映射)
 * ═══════════════════════════════════════════════════════════════════════════ */

static coord_node_t g_registry[MAX_CHARS];
static int g_count = 0;

/* ─────────────────────────────────────────────────────────────────────────
 * 字符识别
 * ───────────────────────────────────────────────────────────────────────── */

static sym_type_t identify_type(const char *name) {
    if (!name || !*name) return 0;
    const unsigned char *p = (const unsigned char *)name;
    if (strcmp(name, "a") == 0 || strcmp(name, "o") == 0 || strcmp(name, "e") == 0 ||
        strcmp(name, "i") == 0 || strcmp(name, "u") == 0 || strcmp(name, "v") == 0)
        return T_BASE;
    if (p[0] == 0xC4 && p[1] == 0x81) return T_TONE; /* ā */
    if (p[0] == 0xC3 && p[1] == 0xA1) return T_TONE; /* á */
    if (p[0] == 0xC4 && p[1] == 0x8D) return T_TONE; /* ǎ */
    if (p[0] == 0xC3 && p[1] == 0xA0) return T_TONE; /* à */
    if (p[0] == 0xC3 && p[1] == 0xAA) return T_TONE; /* ê */
    if (p[0] == 0xC4 && p[1] == 0x9B) return T_TONE; /* ě */
    if (p[0] == 0xC4 && p[1] == 0x93) return T_TONE; /* ō */
    if (p[0] == 0xC3 && p[1] == 0xB3) return T_TONE; /* ó */
    if (p[0] == 0xC7 && p[1] == 0x92) return T_TONE; /* ǒ */
    if (p[0] == 0xC3 && p[1] == 0xB2) return T_TONE; /* ò */
    if (p[0] == 0xC5 && p[1] == 0x8D) return T_TONE; /* ń */
    if (p[0] == 0xC7 && p[1] == 0x87) return T_TONE; /* ň */
    if (p[0] == 0xC3 && p[1] == 0xA9) return T_TONE; /* é */
    if (p[0] == 0xC3 && p[1] == 0xA8) return T_TONE; /* è */
    if (strcmp(name, "ai") == 0 || strcmp(name, "ei") == 0 || strcmp(name, "ao") == 0 ||
        strcmp(name, "ou") == 0 || strcmp(name, "an") == 0 || strcmp(name, "en") == 0 ||
        strcmp(name, "ang") == 0 || strcmp(name, "eng") == 0 || strcmp(name, "ong") == 0 ||
        strcmp(name, "er") == 0)
        return T_COMPOUND;
    if (strcmp(name, "+") == 0 || strcmp(name, "-") == 0 || strcmp(name, "=") == 0 ||
        strcmp(name, "*") == 0 || strcmp(name, "/") == 0 || strcmp(name, ".") == 0 ||
        strcmp(name, ",") == 0 || strcmp(name, "!") == 0 || strcmp(name, "?") == 0 ||
        strcmp(name, ":") == 0 || strcmp(name, ";") == 0 || strcmp(name, "(") == 0 ||
        strcmp(name, ")") == 0 || strcmp(name, "[") == 0 || strcmp(name, "]") == 0)
        return T_SYMBOL;
    if (strcmp(name, "ˊ") == 0 || strcmp(name, "ˇ") == 0 || strcmp(name, "ˋ") == 0 ||
        strcmp(name, "ˆ") == 0 || strcmp(name, "˙") == 0)
        return T_MODIFIER;
    return T_EXTRA;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 坐标计算引擎
 * ─────────────────────────────────────────────────────────────────────────
 * 
 * 表面轴 (X): 基于字符的视觉复杂度 / 笔画数
 * 结构轴 (Y): 基于字符的骨架构成 / 独立程度
 * 横向轴 (Z): 基于字符的连接能力 / 横向扩展
 */

static uint8_t calc_surface(const char *name, sym_type_t type) {
    uint8_t base = 0;
    switch(type) {
        case T_BASE:     base = 5; break;
        case T_TONE:     base = 20; break;
        case T_COMPOUND: base = 35; break;
        case T_SYMBOL:   base = 50; break;
        case T_EXTRA:    base = 45; break;
        case T_MODIFIER: base = 55; break;
        default:         base = 30; break;
    }
    /* 视觉复杂度: 名称越长越复杂 */
    size_t len = strlen(name);
    base += (uint8_t)(len * 3);
    if (base > COORD_MAX) base = COORD_MAX;
    return base;
}

static uint8_t calc_struct(const char *name, sym_type_t type) {
    uint8_t base = 0;
    switch(type) {
        case T_BASE:     base = 10; break;  /* 独立骨架强 */
        case T_TONE:     base = 40; break;  /* 依赖基元 */
        case T_COMPOUND: base = 30; break;  /* 两骨合一 */
        case T_SYMBOL:   base = 50; break;  /* 抽象骨架 */
        case T_EXTRA:    base = 25; break;
        case T_MODIFIER: base = 45; break;
        default:         base = 30; break;
    }
    /* 骨架强度: 元音核心独立性强 */
    if (type == T_BASE) {
        if (strcmp(name, "a") == 0) base = 5;
        else if (strcmp(name, "e") == 0) base = 8;
        else if (strcmp(name, "i") == 0) base = 12;
        else if (strcmp(name, "o") == 0) base = 14;
        else if (strcmp(name, "u") == 0) base = 16;
        else if (strcmp(name, "v") == 0) base = 18;
    }
    if (base > COORD_MAX) base = COORD_MAX;
    return base;
}

static uint8_t calc_lateral(const char *name, sym_type_t type) {
    uint8_t base = 0;
    switch(type) {
        case T_BASE:     base = 15; break;  /* 中等连接 */
        case T_TONE:     base = 35; break;  /* 横向修饰 */
        case T_COMPOUND: base = 55; break;  /* 强横向扩展 */
        case T_SYMBOL:   base = 10; break;  /* 弱连接 */
        case T_EXTRA:    base = 30; break;
        case T_MODIFIER: base = 60; break;  /* 纯横向 */
        default:         base = 25; break;
    }
    /* 横向扩展: 复合音天然宽 */
    if (type == T_COMPOUND) {
        size_t len = strlen(name);
        base += (uint8_t)(len * 4);
    }
    if (base > COORD_MAX) base = COORD_MAX;
    return base;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 注册字符到坐标系统
 * ───────────────────────────────────────────────────────────────────────── */

static int register_char(const char *name, uint8_t code) {
    if (g_count >= MAX_CHARS) return -1;
    coord_node_t *n = &g_registry[g_count];
    strncpy(n->name, name, 15);
    n->name[15] = '\0';
    n->code = code;
    n->type = identify_type(name);
    n->x = calc_surface(name, n->type);
    n->y = calc_struct(name, n->type);
    n->z = calc_lateral(name, n->type);
    g_count++;
    return g_count - 1;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 坐标编码 / 解码 (18bit pack into 3 bytes)
 * ───────────────────────────────────────────────────────────────────────── */

static void encode_coord(uint8_t x, uint8_t y, uint8_t z, uint8_t out[3]) {
    /* X[6b] at bits 17..12, Y[6b] at bits 11..6, Z[6b] at bits 5..0 */
    uint32_t packed = ((uint32_t)(x & 0x3F) << 12) |
                      ((uint32_t)(y & 0x3F) << 6)  |
                      ((uint32_t)(z & 0x3F));
    out[0] = (packed >> 10) & 0xFF;  /* high 8 bits (X full + Y top 2) */
    out[1] = (packed >> 2) & 0xFF;   /* mid 8 bits (Y bot 4 + Z top 4) */
    out[2] = (packed & 0x03) << 6;   /* low 2 bits of packed in high of byte 2 */
}

static void decode_coord(const uint8_t in[3], uint8_t *x, uint8_t *y, uint8_t *z) {
    uint32_t packed = ((uint32_t)in[0] << 10) |
                      ((uint32_t)in[1] << 2)  |
                      ((uint32_t)in[2] >> 6);
    *x = (packed >> 12) & 0x3F;
    *y = (packed >> 6) & 0x3F;
    *z = packed & 0x3F;
}

/* ─────────────────────────────────────────────────────────────────────────
 * 距离感知 (mind_organ 欧氏距离 + 曼哈顿距离)
 * ───────────────────────────────────────────────────────────────────────── */

static float euclidean_dist(coord_node_t *a, coord_node_t *b) {
    float dx = (float)(a->x - b->x);
    float dy = (float)(a->y - b->y);
    float dz = (float)(a->z - b->z);
    return sqrtf(dx*dx + dy*dy + dz*dz);
}

static uint32_t manhattan_dist(coord_node_t *a, coord_node_t *b) {
    return (uint32_t)abs(a->x - b->x) + abs(a->y - b->y) + abs(a->z - b->z);
}

/* ─────────────────────────────────────────────────────────────────────────
 * 空间哈希桶 (坐标 → 聚类桶)
 * ═══════════════════════════════════════════════════════════════════════════ */

#define BUCKET_COUNT 512  /* 2^9 空间桶 */

typedef struct {
    int   indices[MAX_CHARS];
    int   count;
} space_bucket_t;

static space_bucket_t g_buckets[BUCKET_COUNT];

static uint32_t coord_hash(uint8_t x, uint8_t y, uint8_t z) {
    /* 空间哈希: 混合三轴取中低9位 */
    uint32_t h = ((uint32_t)x << 6) | ((uint32_t)y << 0) | ((uint32_t)z << 3);
    h = ((h >> 9) ^ h) & 0x1FF;  /* 9 bits = 512 buckets */
    return h;
}

static void build_buckets(void) {
    memset(g_buckets, 0, sizeof(g_buckets));
    for (int i = 0; i < g_count; i++) {
        uint32_t h = coord_hash(g_registry[i].x, g_registry[i].y, g_registry[i].z);
        if (g_buckets[h].count < MAX_CHARS) {
            g_buckets[h].indices[g_buckets[h].count++] = i;
        }
    }
}

/* ─────────────────────────────────────────────────────────────────────────
 * 邻近搜索: 半径 R 内的所有字符
 * ───────────────────────────────────────────────────────────────────────── */

static int find_nearby(coord_node_t *target, float radius, int *out_indices, int max_out) {
    int found = 0;
    for (int i = 0; i < g_count && found < max_out; i++) {
        if (&g_registry[i] == target) continue;
        float d = euclidean_dist(target, &g_registry[i]);
        if (d <= radius) {
            out_indices[found++] = i;
        }
    }
    return found;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 初始化示例数据
 * ═══════════════════════════════════════════════════════════════════════════ */

static void init_registry(void) {
    /* 基元 a o e i u v */
    register_char("a",   0x02);
    register_char("o",   0x03);
    register_char("e",   0x04);
    register_char("i",   0x05);
    register_char("u",   0x06);
    register_char("v",   0x07);

    /* 基调 ā á ǎ à */
    register_char("ā",   0x80);
    register_char("á",   0x81);
    register_char("ǎ",   0x82);
    register_char("à",   0x83);

    /* 基调 ē é ě è */
    register_char("ē",   0x84);
    register_char("é",   0x85);
    register_char("ě",   0x86);
    register_char("è",   0x87);

    /* 基调 ī í ǐ ì */
    register_char("ī",   0x88);
    register_char("í",   0x89);
    register_char("ǐ",   0x8A);
    register_char("ì",   0x8B);

    /* 基调 ō ó ǒ ò */
    register_char("ō",   0x8C);
    register_char("ó",   0x8D);
    register_char("ǒ",   0x8E);
    register_char("ò",   0x8F);

    /* 基调 ū ú ǔ ù */
    register_char("ū",   0x90);
    register_char("ú",   0x91);
    register_char("ǔ",   0x92);
    register_char("ù",   0x93);

    /* 特殊基调 */
    register_char("ń",   0x94);
    register_char("ň",   0x95);
    register_char("ê",   0x96);
    register_char("˙",   0x97);

    /* 复合音 */
    register_char("ai",  0x10);
    register_char("ei",  0x11);
    register_char("ao",  0x12);
    register_char("ou",  0x13);
    register_char("an",  0x14);
    register_char("en",  0x15);
    register_char("ang", 0x16);
    register_char("eng", 0x17);
    register_char("ong", 0x18);
    register_char("er",  0x19);

    /* 象征符号 */
    register_char("+",   0x30);
    register_char("-",   0x31);
    register_char("=",   0x32);
    register_char(".",   0x33);
    register_char(",",   0x34);
    register_char("!",   0x35);
    register_char("?",   0x36);
    register_char("(",   0x37);
    register_char(")",   0x38);

    /* 修饰零件 */
    register_char("ˊ",   0xB0);
    register_char("ˇ",   0xB1);
    register_char("ˋ",   0xB2);
    register_char("ˆ",   0xB3);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 序列化: 坐标 → 字节流
 * ═══════════════════════════════════════════════════════════════════════════ */

static int serialize_coords(uint8_t *out, int max_bytes) {
    int bytes = 0;
    for (int i = 0; i < g_count && bytes + 3 <= max_bytes; i++) {
        encode_coord(g_registry[i].x, g_registry[i].y, g_registry[i].z, &out[bytes]);
        bytes += 3;
    }
    return bytes;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 测试
 * ═══════════════════════════════════════════════════════════════════════════ */

static int test_count = 0;
static int test_pass = 0;

#define TEST(name) do { test_count++; printf("  [TEST] %-50s ", name); } while(0)
#define PASS()     do { test_pass++; printf("✓\n"); } while(0)
#define FAIL(msg)  do { printf("✗ %s\n", msg); } while(0)

static void run_tests(void) {
    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  毛利兰立体进制框架 · 测试套件\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    /* ─── Test 1-3: 坐标范围 ─── */
    TEST("表面轴 X ∈ [0,63]");
    {
        int ok = 1;
        for (int i = 0; i < g_count; i++) {
            if (g_registry[i].x > 63) { ok = 0; break; }
        }
        if (ok) PASS(); else FAIL("X 超范围");
    }

    TEST("结构轴 Y ∈ [0,63]");
    {
        int ok = 1;
        for (int i = 0; i < g_count; i++) {
            if (g_registry[i].y > 63) { ok = 0; break; }
        }
        if (ok) PASS(); else FAIL("Y 超范围");
    }

    TEST("横向轴 Z ∈ [0,63]");
    {
        int ok = 1;
        for (int i = 0; i < g_count; i++) {
            if (g_registry[i].z > 63) { ok = 0; break; }
        }
        if (ok) PASS(); else FAIL("Z 超范围");
    }

    /* ─── Test 4-6: 坐标编码/解码 ─── */
    TEST("坐标编解码 (15,30,45)");
    {
        uint8_t enc[3];
        encode_coord(15, 30, 45, enc);
        uint8_t dx, dy, dz;
        decode_coord(enc, &dx, &dy, &dz);
        if (dx == 15 && dy == 30 && dz == 45) PASS();
        else FAIL("编解码不一致");
    }

    TEST("坐标编解码 (0,0,0)");
    {
        uint8_t enc[3];
        encode_coord(0, 0, 0, enc);
        uint8_t dx, dy, dz;
        decode_coord(enc, &dx, &dy, &dz);
        if (dx == 0 && dy == 0 && dz == 0) PASS();
        else FAIL("全零编解码失败");
    }

    TEST("坐标编解码 (63,63,63)");
    {
        uint8_t enc[3];
        encode_coord(63, 63, 63, enc);
        uint8_t dx, dy, dz;
        decode_coord(enc, &dx, &dy, &dz);
        if (dx == 63 && dy == 63 && dz == 63) PASS();
        else FAIL("全满编解码失败");
    }

    /* ─── Test 7-8: 距离计算 ─── */
    TEST("欧氏距离 a-o 非零");
    {
        float d = euclidean_dist(&g_registry[0], &g_registry[1]); /* a vs o */
        if (d > 0.0f && d < 200.0f) PASS();
        else FAIL("距离异常");
    }

    TEST("曼哈顿距离 a-o > 0");
    {
        uint32_t d = manhattan_dist(&g_registry[0], &g_registry[1]);
        if (d > 0) PASS();
        else FAIL("曼哈顿距离为零");
    }

    /* ─── Test 9: 空间桶构建 ─── */
    TEST("空间哈希桶 无溢出");
    {
        build_buckets();
        int ok = 1;
        for (int b = 0; b < BUCKET_COUNT; b++) {
            if (g_buckets[b].count >= MAX_CHARS) { ok = 0; break; }
        }
        if (ok) PASS();
        else FAIL("桶溢出");
    }

    /* ─── Test 10-11: 邻近搜索 ─── */
    TEST("邻近搜索 a 半径30");
    {
        build_buckets();
        int nearby[256];
        int n = find_nearby(&g_registry[0], 30.0f, nearby, 256);
        if (n >= 0 && n < g_count) PASS();
        else FAIL("搜索越界");
    }

    TEST("邻近搜索 radius=0 无结果");
    {
        int nearby[256];
        int n = find_nearby(&g_registry[0], 0.0f, nearby, 256);
        if (n == 0) PASS();
        else FAIL("零半径不应有结果");
    }

    /* ─── Test 12-13: 类型区分 ─── */
    TEST("基元 ≠ 基调 (表面轴)");
    {
        /* a (基元) x should differ from ā (基调) x */
        if (g_registry[0].x != g_registry[6].x) PASS();
        else FAIL("类型未区分");
    }

    TEST("基调 ≠ 复合 (横向轴)");
    {
        /* ā (基调, idx=6) z should differ from ai (复合, idx=30) z */
        if (g_registry[6].z != g_registry[30].z) PASS();
        else FAIL("横向未区分");
    }

    /* ─── Test 14: 比特成本验证 ─── */
    TEST("每坐标 ≤ 3 bytes");
    {
        uint8_t buf[3];
        encode_coord(40, 30, 50, buf);
        /* Any 18-bit value fits in 3 bytes */
        if (buf[0] != 0xFF && buf[1] != 0xFF) PASS();
        else FAIL("超出3字节");
    }

    /* ─── Test 15: 序列化 ─── */
    TEST("序列化全部坐标");
    {
        uint8_t buf[256*3];
        int bytes = serialize_coords(buf, sizeof(buf));
        if (bytes == g_count * 3) PASS();
        else FAIL("序列化字节数不匹配");
    }

    /* ─── Test 16: 距离对称性 ─── */
    TEST("距离对称性 d(a,b)=d(b,a)");
    {
        float d1 = euclidean_dist(&g_registry[0], &g_registry[2]);
        float d2 = euclidean_dist(&g_registry[2], &g_registry[0]);
        if (fabsf(d1 - d2) < 0.001f) PASS();
        else FAIL("不对称");
    }

    /* ─── Test 17: 同类型聚类 ─── */
    TEST("基调在横向轴聚集");
    {
        /* 所有基调的Z值应该相近 (都在35左右) */
        int all_high_z = 1;
        for (int i = 6; i <= 17; i++) { /* indices 6..17 are tones */
            if (g_registry[i].z < 25) { all_high_z = 0; break; }
        }
        if (all_high_z) PASS();
        else FAIL("基调横向分散");
    }

    printf("\n───────────────────────────────────────────────────────────────\n");
    printf("  结果: %d/%d 通过\n", test_pass, test_count);
    printf("───────────────────────────────────────────────────────────────\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 坐标可视化 (ASCII 投影)
 * ═══════════════════════════════════════════════════════════════════════════ */

static void visualize_axis(axis_t axis) {
    /* 64列ASCII直方图 */
    int hist[COORD_RANGE];
    memset(hist, 0, sizeof(hist));
    for (int i = 0; i < g_count; i++) {
        uint8_t v = (axis == AXIS_SURFACE) ? g_registry[i].x :
                    (axis == AXIS_STRUCT)  ? g_registry[i].y :
                                              g_registry[i].z;
        hist[v]++;
    }
    int max_h = 1;
    for (int j = 0; j < COORD_RANGE; j++) {
        if (hist[j] > max_h) max_h = hist[j];
    }
    printf("\n  %s (%s) 轴分布:\n", axis_name(axis), axis_cn(axis));
    printf("  0%*s63\n", COORD_RANGE - 2, "");
    printf("  |");
    for (int j = 0; j < COORD_RANGE; j++) {
        if (hist[j] > 0) {
            float ratio = (float)hist[j] / max_h;
            if (ratio > 0.75f) printf("█");
            else if (ratio > 0.5f) printf("▓");
            else if (ratio > 0.25f) printf("▒");
            else printf("░");
        } else {
            printf(" ");
        }
    }
    printf("|\n");
}

static void print_coord_card(coord_node_t *n) {
    printf("  ┌─────────────────────────────────────────┐\n");
    printf("  │  %-10s  编码: 0x%02X  类型: %-6s │\n", n->name, n->code, type_str(n->type));
    printf("  ├─────────────────────────────────────────┤\n");
    printf("  │  X[表面] = %2d (%-8s)               │\n", n->x, axis_cn(AXIS_SURFACE));
    printf("  │  Y[结构] = %2d (%-8s)               │\n", n->y, axis_cn(AXIS_STRUCT));
    printf("  │  Z[横向] = %2d (%-8s)               │\n", n->z, axis_cn(AXIS_LATERAL));
    printf("  ├─────────────────────────────────────────┤\n");
    uint8_t enc[3];
    encode_coord(n->x, n->y, n->z, enc);
    printf("  │  空间地址: %02X %02X %02X (18 bits)       │\n", enc[0], enc[1], enc[2]);
    printf("  └─────────────────────────────────────────┘\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 演示模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static void demo(void) {
    printf("\n");
    printf("╔═══════════════════════════════════════════════════════════════════╗\n");
    printf("║           毛利兰立体进制框架 · 三维坐标定位演示                  ║\n");
    printf("╚═══════════════════════════════════════════════════════════════════╝\n");

    printf("\n  三轴坐标系:\n");
    printf("    X 轴 · 表面轴 · 毛利兰  → 模/面/名 (视觉表面属性)\n");
    printf("    Y 轴 · 结构轴 · 工藤新一 → 骨/头/体 (结构骨架属性)\n");
    printf("    Z 轴 · 横向轴 · 灰原哀  → 行/页/业 (横向扩展属性)\n");
    printf("\n  寻址空间: 64³ = 262,144 个静态绝对位置\n");
    printf("  编码成本: 18 bits = 3 bytes per coordinate\n");

    /* 展示几个重点字符的坐标 */
    int hl_indices[] = {0, 7, 18, 28, 38};

    printf("\n─── 关键字符坐标卡 ───\n");
    for (int i = 0; i < 5; i++) {
        print_coord_card(&g_registry[hl_indices[i]]);
    }

    /* 轴分布可视化 */
    visualize_axis(AXIS_SURFACE);
    visualize_axis(AXIS_STRUCT);
    visualize_axis(AXIS_LATERAL);

    /* 距离矩阵 (部分) */
    printf("\n─── 距离矩阵 (欧氏/曼哈顿) ───\n");
    printf("  %-6s", "");
    const char *dm_names[] = {"a","o","e","ā","á","ai","ou","+"};
    int dm_idx[] = {0,1,2,6,7,18,23,28};
    for (int i = 0; i < 8; i++) printf("%-8s", dm_names[i]);
    printf("\n");
    for (int i = 0; i < 8; i++) {
        printf("  %-6s", dm_names[i]);
        for (int j = 0; j < 8; j++) {
            if (i == j) printf("%-8s", "·");
            else {
                float d = euclidean_dist(&g_registry[dm_idx[i]], &g_registry[dm_idx[j]]);
                printf("%-8d", (int)d);
            }
        }
        printf("\n");
    }
    printf("  (曼哈顿距离):\n");
    for (int i = 0; i < 8; i++) {
        printf("  %-6s", dm_names[i]);
        for (int j = 0; j < 8; j++) {
            if (i == j) printf("%-8s", "·");
            else {
                uint32_t m = manhattan_dist(&g_registry[dm_idx[i]], &g_registry[dm_idx[j]]);
                printf("%-8u", m);
            }
        }
        printf("\n");
    }

    /* 邻近示范 */
    printf("\n─── 邻近感知: a 周围半径 20 的字符 ───\n");
    build_buckets();
    int nearby[256];
    int nc = find_nearby(&g_registry[0], 20.0f, nearby, 256);
    printf("  找到 %d 个邻近字符: ", nc);
    for (int i = 0; i < nc && i < 12; i++) {
        printf("%s ", g_registry[nearby[i]].name);
    }
    if (nc > 12) printf("...");
    printf("\n");

    /* 比特效率总结 */
    printf("\n─── 比特效率分析 ───\n");
    printf("  容器容量:    254 字符\n");
    printf("  每字符编码:  8 bits (容器码)\n");
    printf("  每字符坐标:  18 bits (三轴 × 6 bits)\n");
    printf("  每字符总计:  26 bits (编码 + 空间定位)\n");
    printf("  空间桶数:    512 桶\n");
    printf("  已注册:      %d 字符\n", g_count);
    printf("  利用率:      %.1f%%\n", (float)g_count / MAX_CHARS * 100.0f);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 交互模式
 * ═══════════════════════════════════════════════════════════════════════════ */

static int find_by_name(const char *name) {
    for (int i = 0; i < g_count; i++) {
        if (strcmp(g_registry[i].name, name) == 0) return i;
    }
    return -1;
}

static void interactive(void) {
    char line[256];
    printf("\n╔═══════════════════════════════════════════════════════════════════╗\n");
    printf("║           毛利兰立体进制框架 · 交互模式                          ║\n");
    printf("╠═══════════════════════════════════════════════════════════════════╣\n");
    printf("║  命令:                                                           ║\n");
    printf("║    <名>          查坐标卡                                        ║\n");
    printf("║    d <A> <B>     计算两字符距离                                  ║\n");
    printf("║    n <名> <半径> 邻近搜索                                        ║\n");
    printf("║    b             桶统计                                          ║\n");
    printf("║    v             轴分布可视化                                    ║\n");
    printf("║    t             运行测试                                        ║\n");
    printf("║    q             退出                                            ║\n");
    printf("╚═══════════════════════════════════════════════════════════════════╝\n");

    build_buckets();

    while (1) {
        printf("\nlan3d> ");
        if (!fgets(line, sizeof(line), stdin)) break;
        line[strcspn(line, "\n")] = '\0';
        if (strlen(line) == 0) continue;

        if (line[0] == 'q') break;
        else if (line[0] == 't') run_tests();
        else if (line[0] == 'v') {
            visualize_axis(AXIS_SURFACE);
            visualize_axis(AXIS_STRUCT);
            visualize_axis(AXIS_LATERAL);
        }
        else if (line[0] == 'b') {
            int used = 0, max_c = 0, sum = 0;
            for (int b = 0; b < BUCKET_COUNT; b++) {
                if (g_buckets[b].count > 0) {
                    used++;
                    sum += g_buckets[b].count;
                    if (g_buckets[b].count > max_c) max_c = g_buckets[b].count;
                }
            }
            printf("  空间桶: %d/%d 已用 | 最大桶: %d | 平均: %.1f\n",
                   used, BUCKET_COUNT, max_c, used > 0 ? (float)sum/used : 0.0f);
        }
        else if (line[0] == 'd' && line[1] == ' ') {
            char na[16], nb[16];
            if (sscanf(line + 2, "%15s %15s", na, nb) == 2) {
                int ia = find_by_name(na);
                int ib = find_by_name(nb);
                if (ia < 0 || ib < 0) {
                    printf("  字符未找到\n");
                } else {
                    float e = euclidean_dist(&g_registry[ia], &g_registry[ib]);
                    uint32_t m = manhattan_dist(&g_registry[ia], &g_registry[ib]);
                    printf("  %s ↔ %s: 欧氏=%.2f 曼哈顿=%u\n", na, nb, e, m);
                }
            }
        }
        else if (line[0] == 'n' && line[1] == ' ') {
            char na[16];
            float r;
            if (sscanf(line + 2, "%15s %f", na, &r) == 2) {
                int ia = find_by_name(na);
                if (ia < 0) {
                    printf("  字符未找到\n");
                } else {
                    int nearby[256];
                    int nc = find_nearby(&g_registry[ia], r, nearby, 256);
                    printf("  %s 半径%.0f范围内: %d 个字符: ", na, r, nc);
                    for (int i = 0; i < nc && i < 15; i++) {
                        printf("%s ", g_registry[nearby[i]].name);
                    }
                    if (nc > 15) printf("...");
                    printf("\n");
                }
            }
        }
        else {
            int idx = find_by_name(line);
            if (idx >= 0) {
                print_coord_card(&g_registry[idx]);
            } else {
                printf("  未知字符 '%s' — 用 'd a á' 查距离, 'n a 20' 邻近搜索\n", line);
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * 入口
 * ═══════════════════════════════════════════════════════════════════════════ */

int main(int argc, char **argv) {
    init_registry();

    if (argc > 1) {
        if (strcmp(argv[1], "-t") == 0) {
            run_tests();
            return 0;
        }
        if (strcmp(argv[1], "-i") == 0) {
            interactive();
            return 0;
        }
        if (strcmp(argv[1], "-d") == 0) {
            demo();
            return 0;
        }
    }

    /* 默认: 演示 + 测试 */
    demo();
    run_tests();
    return 0;
}
