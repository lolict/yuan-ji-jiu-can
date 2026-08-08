/*
 * mlll 符号生成器——几何维度的最小可行版本
 *
 * 编码流水线：类型产出 → 时间排队 → 量化位置 → 偏移存储
 * 本文件实现几何边角面维度的枚举 + 偏移分配
 *
 * 扩展其他维度只需增加枚举函数，存储和计算逻辑不变
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

#define MLLL_SYMBOLS_MAX 4096
#define MLLL_OFFSET_PRECISION 10000  /* 偏移率精度：万分之一 */

/* 符号类型——扩展时在此追加 */
typedef enum {
    SYM_GEOMETRY = 0,   /* 几何边角面 */
    SYM_TONE,           /* 声调 */
    SYM_RADICAL,        /* 部首 */
    SYM_COLOR,          /* 颜色 */
    SYM_DIMENSION_COUNT /* 维度计数 */
} symbol_dim_t;

/* 单个符号定义 */
typedef struct {
    uint32_t    id;             /* 唯一标识 */
    symbol_dim_t dim;           /* 所属维度 */
    int64_t     property_a;     /* 主要属性（如边数） */
    int64_t     property_b;     /* 次要属性（如面数） */
    uint32_t    sequence_pos;   /* 时间线上的位置 */
    uint32_t    offset_num;     /* 偏移率分子（除以 MLLL_OFFSET_PRECISION） */
    char        name[64];       /* 可读名（调试用，不进入编码） */
} mlll_symbol_t;

/* 符号库 */
typedef struct {
    mlll_symbol_t entries[MLLL_SYMBOLS_MAX];
    uint32_t      count;
    uint32_t      tone_table[8]; /* 声调坐标表，用于计算渐变频率根 */
} mlll_registry_t;

static mlll_registry_t REG;

/* 声调初始化——8个声调坐标映射 */
static void mlll_init_tones(void) {
    const int tones[8] = {1, 2, 3, 4, 5, 6, 7, 8}; /* 扩展至8声调 */
    for (int i = 0; i < 8; i++) {
        REG.tone_table[i] = tones[i];
    }
}

/* 符号哈希——生成唯一标识 */
static uint32_t symbol_hash(symbol_dim_t dim, int64_t a, int64_t b) {
    uint32_t h = (uint32_t)dim * 2654435761u;
    h ^= (uint32_t)(a * 40503) + (uint32_t)(b * 7919);
    return h;
}

/* === 维度1：几何边角面枚举 === */
/* 统计几何维度的枚举容量 */
static int geometry_count_sides(int max_sides) { return max_sides - 2; }
static int geometry_count_faces(int max_faces) { return max_faces - 3; }
static int geometry_count_ridges(int max_ridges) { return max_ridges - 5; }
static int geometry_count_internal_ridges(int max_internal) { return max_internal - 3; }

static void mlll_enumerate_geometry(int max_sides, int max_faces, int max_ridges, int max_internal) {
    int idx = 0;
    /* 边数枚举（3-max_sides） */
    for (int s = 3; s <= max_sides; s++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_GEOMETRY, s, 0);
        e->dim         = SYM_GEOMETRY;
        e->property_a  = s;
        e->property_b  = 0;
        snprintf(e->name, sizeof(e->name), "geom_edges_%d", s);
        REG.count++;
        idx++;
        (void)idx;
    }
    /* 面数枚举（4-max_faces） */
    for (int f = 4; f <= max_faces; f++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_GEOMETRY, 0, f);
        e->dim         = SYM_GEOMETRY;
        e->property_a  = 0;
        e->property_b  = f;
        snprintf(e->name, sizeof(e->name), "geom_faces_%d", f);
        REG.count++;
    }
    /* 脊数枚举（6-max_ridges） */
    for (int r = 6; r <= max_ridges; r++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_GEOMETRY, r, 1000);
        e->dim         = SYM_GEOMETRY;
        e->property_a  = r;
        e->property_b  = 1000;
        snprintf(e->name, sizeof(e->name), "geom_ridges_%d", r);
        REG.count++;
    }
    /* 内部脊数枚举（4-max_internal） */
    for (int ir = 4; ir <= max_internal; ir++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_GEOMETRY, 2000, ir);
        e->dim         = SYM_GEOMETRY;
        e->property_a  = 2000;
        e->property_b  = ir;
        snprintf(e->name, sizeof(e->name), "geom_internal_%d", ir);
        REG.count++;
    }
}

/* === 第二步：时间排队 + 分配偏移 === */
static void mlll_assign_offsets(double delta) {
    for (uint32_t i = 0; i < REG.count; i++) {
        REG.entries[i].sequence_pos = i;
        /* 偏移率 = (i+1) * delta，存为定点整数 */
        REG.entries[i].offset_num = (uint32_t)round((i + 1) * delta * MLLL_OFFSET_PRECISION);
    }
}

/* === 第三步：解码——通过位置获取符号 === */
static mlll_symbol_t* mlll_get_by_sequence(uint32_t pos) {
    if (pos >= REG.count) return NULL;
    return &REG.entries[pos];
}

/* === 第四步：静态回归——偏移坐标 → 真实值估算 === */
static double mlll_static_regression(uint32_t pos, double delta, uint32_t precision) {
    /* 真实值 ≈ 位置 × 偏移率 × 维度基数 */
    if (pos == 0) return 1.0; /* 第 0 次起源 = 精确 1 */
    double offset_rate = ((double)REG.entries[pos].offset_num) / precision;
    return (pos + 1) * offset_rate;
}

/* === 声调坐标运算 === */
/* 计算渐变频率的根——频率 = 声调坐标 × 层级 */
static int64_t mlll_tone_frequency(int tone_index, int level) {
    if (tone_index < 0 || tone_index >= 8) return -1;
    return REG.tone_table[tone_index] * (level + 1);
}

/* 主入口 */
void mlll_init(double delta) {
    memset(&REG, 0, sizeof(REG));
    mlll_init_tones();
    /* 几何维度：3-200 边，4-200 面，6-400 脊，4-400 内部脊 */
    mlll_enumerate_geometry(200, 200, 400, 400);
    mlll_assign_offsets(delta);
}

/* 导出统计 */
void mlll_print_stats(void) {
    printf("=== mlll Symbol Registry ===\n");
    printf("Total symbols: %u\n", REG.count);
    printf("First 20 symbols (head of timeline):\n");
    for (uint32_t i = 0; i < 20 && i < REG.count; i++) {
        mlll_symbol_t *e = &REG.entries[i];
        double offset = (double)e->offset_num / (double)MLLL_OFFSET_PRECISION;
        double static_val = mlll_static_regression(i, offset, MLLL_OFFSET_PRECISION);
        printf("  [%u] id=%08x dim=%d a=%lld b=%lld | offset=%.4f | static_regression=%.4f | %s\n",
               i, e->id, e->dim,
               (long long)e->property_a, (long long)e->property_b,
               offset, static_val, e->name);
    }
    printf("\nTone frequency table (level=0):\n");
    for (int t = 0; t < 8; t++) {
        printf("  tone[%d] frequency=%lld\n", t, (long long)mlll_tone_frequency(t, 0));
    }
    printf("\nTone frequency table (level=1):\n");
    for (int t = 0; t < 8; t++) {
        printf("  tone[%d] frequency=%lld\n", t, (long long)mlll_tone_frequency(t, 1));
    }
}

/* 演示 */
#ifndef MLLL_LIBRARY
int main(void) {
    /* 初始偏移率 delta = 0.1（二次起源代数） */
    mlll_init(0.1);
    mlll_print_stats();
    printf("\nTimeline lookup test:\n");
    mlll_symbol_t *s = mlll_get_by_sequence(254);
    if (s) {
        printf("  Symbol at position 254: %s (id=%08x)\n", s->name, s->id);
    }
    return 0;
}
#endif
