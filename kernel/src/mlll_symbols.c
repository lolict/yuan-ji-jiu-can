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
    SYM_PHASE,          /* 相位角度 */
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

/* === 各维度辅助函数 === */
static int tone_return_frequency(int tone_idx, int level) { return (tone_idx + 1) * (level + 1); }

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

/* === 维度2：声调符号枚举 === */
static void mlll_enumerate_tones(void) {
    static const char* tone_names[8] = {
        "tone_yin_ping",   /* 阴平 */
        "tone_yang_ping",  /* 阳平 */
        "tone_shang",      /* 上声 */
        "tone_qu",         /* 去声 */
        "tone_yin_ru",     /* 阴入 */
        "tone_yang_ru",    /* 阳入 */
        "tone_zhong_yin",  /* 中阴 */
        "tone_zhong_yang",  /* 中阳 */
    };
    for (int i = 0; i < 8; i++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_TONE, i + 1, 0);
        e->dim         = SYM_TONE;
        e->property_a  = i + 1; /* 声调级数 */
        e->property_b  = tone_return_frequency(i, 0); /* 基础频率 */
        snprintf(e->name, sizeof(e->name), "%s", tone_names[i]);
        REG.count++;
    }
}

/* === 维度4：康熙部首枚举（141 个代表性部首）=== */
static void mlll_enumerate_radicals(void) {
    static const char* demo_radicals[] = {
        "一","丶","丨","丿","乙","亅","二","亠","人","儿",
        "入","八","冂","冖","冫","几","凵","刀","刃","力",
        "勹","匕","匚","十","卜","卩","厂","又","口","囗",
        "土","夂","夕","女","子","宀","寸","小","山","川",
        "工","己","已","巾","干","广","弋","弓","彐","彡",
        "彳","心","戸","扌","攴","斤","方","无","日","曰",
        "月","木","欠","止","歹","毋","比","毛","氏","气",
        "水","火","爪","片","牙","牛","犬","玄","玉","瓜",
        "瓦","甘","生","用","田","疋","目","矛","矢","石",
        "示","禾","穴","立","竹","米","糸","网","羊","羽",
        "老","而","耳","聿","肉","臣","自","至","臼","舌",
        "舟","艮","色","虫","血","行","衣","見","角","言",
        "谷","豕","貝","赤","走","足","身","車","辛","辰",
        "邑","酉","里","金","長","門","阜","隹","雨","非",
        "面","革","韋","韭","音","頁","風","飛","食","首",
        "香","馬","骨","高","鬥","鬼","魚","鳥","鹿","麥",
        "麻","黃","黍","黑","鼎","鼓","鼠","鼻","齊","齒",
        "龍","龜"
    };
    for (int i = 0; i < 141; i++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_RADICAL, i + 1, 0);
        e->dim         = SYM_RADICAL;
        e->property_a  = i + 1; /* 部首序号 */
        e->property_b  = (int64_t)strlen(demo_radicals[i]); /* UTF-8 字节长 */
        snprintf(e->name, sizeof(e->name), "rad_%s", demo_radicals[i]);
        REG.count++;
    }
}

/* === 维度3：相位角度枚举（0-360，步长1°）=== */
static void mlll_enumerate_phases(int step) {
    int angle = 0;
    while (angle <= 360 && REG.count < MLLL_SYMBOLS_MAX) {
        if (angle == 0 || angle == 90 || angle == 180 || angle == 270 || angle == 360 || angle % step == 0) {
            mlll_symbol_t *e = &REG.entries[REG.count];
            e->id          = symbol_hash(SYM_PHASE, angle, 0);
            e->dim         = SYM_PHASE;
            e->property_a  = angle;
            e->property_b  = angle % 360;
            snprintf(e->name, sizeof(e->name), "phase_%d", angle);
            REG.count++;
        }
        angle++;
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
    mlll_enumerate_geometry(100, 100, 200, 200);
    /* 声调维度：8 个声调 */
    mlll_enumerate_tones();
    /* 康熙部首维度（141 个代表性部首） */
    mlll_enumerate_radicals();
    /* 相位维度：0-360 每 30° 一个符号 */
    mlll_enumerate_phases(30);
    mlll_assign_offsets(delta);
}

void mlll_print_stats(void) {
    printf("=== mlll Symbol Registry ===\n");
    printf("Total symbols: %u\n", REG.count);
    printf("First 10 symbols (head of timeline):\n");
    for (uint32_t i = 0; i < 10 && i < REG.count; i++) {
        mlll_symbol_t *e = &REG.entries[i];
        double offset = (double)e->offset_num / (double)MLLL_OFFSET_PRECISION;
        printf("  [%u] id=%08x dim=%d a=%lld b=%lld | offset=%.4f | %s\n",
               i, e->id, e->dim,
               (long long)e->property_a, (long long)e->property_b,
               offset, e->name);
    }

    /* 按维度统计 */
    uint32_t dim_counts[SYM_DIMENSION_COUNT];
    memset(dim_counts, 0, sizeof(dim_counts));
    for (uint32_t i = 0; i < REG.count; i++) {
        if (REG.entries[i].dim < SYM_DIMENSION_COUNT) dim_counts[REG.entries[i].dim]++;
    }
    printf("\nDimension breakdown:\n");
    const char* dim_names[] = {"Geometry", "Tone", "Radical", "Color", "Phase"};
    for (int d = 0; d < SYM_DIMENSION_COUNT; d++) {
        printf("  %s: %u symbols\n", dim_names[d], dim_counts[d]);
    }

    /* 打印最后 20 个符号（包含声调和相位） */
    printf("\nLast 20 symbols (tail of timeline, includes tones+phases):\n");
    uint32_t start = REG.count > 20 ? REG.count - 20 : 0;
    for (uint32_t i = start; i < REG.count; i++) {
        mlll_symbol_t *e = &REG.entries[i];
        double offset = (double)e->offset_num / (double)MLLL_OFFSET_PRECISION;
        printf("  [%u] id=%08x dim=%d a=%lld b=%lld | offset=%.4f | %s\n",
               i, e->id, e->dim,
               (long long)e->property_a, (long long)e->property_b,
               offset, e->name);
    }

    printf("\nTone frequency table (level=0):\n");
    for (int t = 0; t < 8; t++) {
        printf("  tone[%d] freq=%lld", t, (long long)mlll_tone_frequency(t, 0));
    }
    printf("\nTone frequency table (level=1):\n");
    for (int t = 0; t < 8; t++) {
        printf("  tone[%d] freq=%lld", t, (long long)mlll_tone_frequency(t, 1));
    }
    printf("\n");
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
