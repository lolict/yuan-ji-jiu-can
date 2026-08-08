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
#include <stdlib.h>
#include <math.h>
#include "mlll_symbols.h"

#define MLLL_SYMBOLS_MAX 4096
#define MLLL_OFFSET_PRECISION 10000  /* 偏移率精度：万分之一 */


mlll_registry_t REG;

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
        e->id          = symbol_hash(SYM_TONE, i, 0);
        e->dim         = SYM_TONE;
        e->property_a  = i; /* 声调索引 0=阴平, 1=阳平, ... */
        e->property_b  = tone_return_frequency(i, 0);
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

/* === 维度5：颜色光谱枚举（380-780nm，每20nm一个符号）=== */
static void mlll_enumerate_colors(void) {
    static const char* color_range_names[] = {
        "380nm", "400nm_violet", "420nm_indigo", "440nm_blue",
        "460nm_blue", "480nm_cyan", "490nm_cyan", "500nm_green_cyan",
        "510nm_green", "520nm_green", "530nm_green_yellow", "540nm_yellow_green",
        "550nm_yellow", "560nm_yellow_orange", "570nm_orange", "580nm_orange",
        "590nm_red_orange", "600nm_red", "620nm_red", "640nm_deep_red"
    };
    for (int i = 0; i < 20; i++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        int wavelength = 380 + i * 20;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_COLOR, wavelength, 0);
        e->dim         = SYM_COLOR;
        e->property_a  = wavelength;
        e->property_b  = (int64_t)(wavelength >= 380 && wavelength <= 780);
        snprintf(e->name, sizeof(e->name), "col_%dnm_%s", wavelength, color_range_names[i]);
        REG.count++;
    }
}

/* === 维度6：笔画数枚举（1-64画，汉字骨骼密度）=== */
static void mlll_enumerate_strokes(void) {
    for (int s = 1; s <= 64; s++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_STROKE, s, 0);
        e->dim         = SYM_STROKE;
        e->property_a  = s;          /* 笔画数本体 */
        e->property_b  = s * s;      /* 笔画数平方（结构复杂度代理） */
        snprintf(e->name, sizeof(e->name), "stroke_%d", s);
        REG.count++;
    }
}

/* === 维度7：字形结构枚举 ===
 * 汉字的组件空间排布方式——独体/左右/上下/半包围/全包围/品字/左中右/上中下
 */
static void mlll_enumerate_structures(void) {
    static const char* struct_names[] = {
        "single",       /* 独体 */
        "leftright",    /* 左右 */
        "topbottom",    /* 上下 */
        "halfwrap",     /* 半包围 */
        "fullwrap",     /* 全包围 */
        "pin",          /* 品字 */
        "lcr",          /* 左中右 */
        "tcb"           /* 上中下 */
    };
    int n = sizeof(struct_names) / sizeof(struct_names[0]);
    for (int i = 0; i < n; i++) {
        if (REG.count >= MLLL_SYMBOLS_MAX) return;
        mlll_symbol_t *e = &REG.entries[REG.count];
        e->id          = symbol_hash(SYM_STRUCTURE, i, 0);
        e->dim         = SYM_STRUCTURE;
        e->property_a  = i;   /* 结构类型号 */
        e->property_b  = (int64_t)(i + 1) * (i + 1);  /* 结构复杂度代理 */
        snprintf(e->name, sizeof(e->name), "struct_%s", struct_names[i]);
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
mlll_symbol_t* mlll_get_by_sequence(uint32_t pos) {
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

/* 计算渐变频率的根——频率 = 声调坐标 × 层级 */
static int64_t mlll_tone_frequency(int tone_index, int level) {
    if (tone_index < 0 || tone_index >= 8) return -1;
    return REG.tone_table[tone_index] * (level + 1);
}

/* === 跨维度查询：给定某维度的属性值，找到其他维度偏移率最近的符号 ===
 * 原理：所有维度共享同一条时间线，偏移率统一编码。
 *       当两个符号值在时间线上的偏移率接近，它们就是跨维度共振。
 *
 * 输入：source_dim 源维度，property 源属性值
 *       target_dim 目标维度
 * 返回：目标维度中偏移率最接近的符号指针
 */
mlll_symbol_t* mlll_cross_query(symbol_dim_t source_dim, int64_t property, symbol_dim_t target_dim) {
    /* 先找到源维度指定属性的符号，取得偏移率 */
    mlll_symbol_t *src = NULL;
    for (uint32_t i = 0; i < REG.count; i++) {
        if (REG.entries[i].dim == source_dim && REG.entries[i].property_a == property) {
            src = &REG.entries[i];
            break;
        }
    }
    if (!src) return NULL;

    uint32_t src_offset = src->offset_num;
    /* 在目标维度中线性扫描，差值最小者即共振符号 */
    mlll_symbol_t *best = NULL;
    uint32_t best_delta = UINT32_MAX;
    for (uint32_t i = 0; i < REG.count; i++) {
        if (REG.entries[i].dim != target_dim) continue;
        uint32_t d = (REG.entries[i].offset_num > src_offset)
                   ? REG.entries[i].offset_num - src_offset
                   : src_offset - REG.entries[i].offset_num;
        if (d < best_delta) {
            best_delta = d;
            best = &REG.entries[i];
        }
    }
    return best;
}

/* === 汉字编码演示：将汉字属性映射到全部六维坐标 ===
 * 任何汉字都可在时间线上找到唯一六维坐标。
 *
 * 输入：stroke=笔画数 t=声调序号(0=阴平,1=阳平,...)
 *        radical_idx=部首序号(1=一,2=丶,...141=龜)
 */
void mlll_character_demo(int stroke, int tone_idx, int radical_idx) {
    printf("\nCharacter encoding (stroke=%d, tone=%d, radical_idx=%d):\n",
           stroke, tone_idx, radical_idx);
    mlll_symbol_t *stroke_s = mlll_cross_query(SYM_STROKE, stroke, SYM_STROKE);
    mlll_symbol_t *radical_s = (radical_idx > 0 && radical_idx <= 141)
        ? &REG.entries[587 + 8 + radical_idx - 1] : NULL;
    mlll_symbol_t *tone_s    = (tone_idx >= 0 && tone_idx < 8)
        ? &REG.entries[587 + tone_idx] : NULL;
    printf("  Stroke : stroke_%d (offset=%.1f)\n", stroke,
           stroke_s ? (double)stroke_s->offset_num / MLLL_OFFSET_PRECISION : 0.0);
    printf("  Radical: %s (offset=%.1f)\n",
           radical_s ? radical_s->name : "null",
           radical_s ? (double)radical_s->offset_num / MLLL_OFFSET_PRECISION : 0.0);
    printf("  Tone   : %s (offset=%.1f)\n",
           tone_s ? tone_s->name : "null",
           tone_s ? (double)tone_s->offset_num / MLLL_OFFSET_PRECISION : 0.0);
    mlll_symbol_t *phase_v = mlll_cross_query(SYM_STROKE, stroke, SYM_PHASE);
    mlll_symbol_t *color_v = mlll_cross_query(SYM_STROKE, stroke, SYM_COLOR);
    mlll_symbol_t *geom_v  = mlll_cross_query(SYM_TONE, tone_idx, SYM_GEOMETRY);
    mlll_symbol_t *stroke_v = mlll_cross_query(SYM_RADICAL, radical_idx, SYM_STROKE);
    printf("  Phase (via stroke): %s\n", phase_v ? phase_v->name : "null");
    printf("  Color (via stroke): %s\n", color_v ? color_v->name : "null");
    printf("  Geom  (via tone)  : %s\n", geom_v ? geom_v->name : "null");
    printf("  Stroke (via rad)  : %s\n", stroke_v ? stroke_v->name : "null");
    printf("  Unified timeline encodes all 6 dims simultaneously\n");
}

/* === 字符串级编码：汉字串 → 时间线坐标序列 ===
 * 每个汉字是六维空间中的一个点，字符串就是点的轨迹。
 * 编码输出序列：轨迹 = 沿时间线排列的四维坐标链。
 */

void mlll_string_encode(const char* name, char_prop_t* chars, int len) {
    printf("\nString encoding: %s (%d chars)\n", name, len);
    double first_offset = 0, last_offset = 0;
    for (int i = 0; i < len; i++) {
        mlll_symbol_t *st = mlll_cross_query(SYM_STROKE, chars[i].stroke, SYM_STROKE);
        mlll_symbol_t *rd = (chars[i].radical > 0 && chars[i].radical <= 141)
            ? &REG.entries[587 + 8 + chars[i].radical - 1] : NULL;
        mlll_symbol_t *tn = (chars[i].tone >= 0 && chars[i].tone < 8)
            ? &REG.entries[587 + chars[i].tone] : NULL;
        double offset_st = st ? (double)st->offset_num / MLLL_OFFSET_PRECISION : 0;
        double offset_rd = rd ? (double)rd->offset_num / MLLL_OFFSET_PRECISION : 0;
        double offset_tn = tn ? (double)tn->offset_num / MLLL_OFFSET_PRECISION : 0;
        printf("  char[%d] s=%d t=%d r=%d | offsets st=%.1f/rd=%.1f/tn=%.1f | flow=%.2f\n",
               i, chars[i].stroke, chars[i].tone, chars[i].radical,
               offset_st, offset_rd, offset_tn,
               offset_st * 0.5 + offset_rd * 0.3 + offset_tn * 0.2);
        if (i == 0) first_offset = offset_st;
        if (i == len - 1) last_offset = offset_st;
    }
    printf("  Trajectory span: %.1f -> %.1f (flow length = %.1f)\n",
           first_offset, last_offset, last_offset - first_offset);
}

/* 字符距离：七维空间中的相似度（归一化到 [0,1]） */
double mlll_char_distance(char_prop_t* a, char_prop_t* b) {
    mlll_symbol_t *st_a = mlll_cross_query(SYM_STROKE, a->stroke, SYM_STROKE);
    mlll_symbol_t *rd_a = (a->radical > 0 && a->radical <= 141)
        ? &REG.entries[587 + 8 + a->radical - 1] : NULL;
    mlll_symbol_t *tn_a = (a->tone >= 0 && a->tone < 8)
        ? &REG.entries[587 + a->tone] : NULL;
    mlll_symbol_t *sx_a = (a->structure >= 0 && a->structure < 8)
        ? &REG.entries[587 + 8 + 141 + 20 + 13 + 64 + a->structure] : NULL;
    mlll_symbol_t *st_b = mlll_cross_query(SYM_STROKE, b->stroke, SYM_STROKE);
    mlll_symbol_t *rd_b = (b->radical > 0 && b->radical <= 141)
        ? &REG.entries[587 + 8 + b->radical - 1] : NULL;
    mlll_symbol_t *tn_b = (b->tone >= 0 && b->tone < 8)
        ? &REG.entries[587 + b->tone] : NULL;
    mlll_symbol_t *sx_b = (b->structure >= 0 && b->structure < 8)
        ? &REG.entries[587 + 8 + 141 + 20 + 13 + 64 + b->structure] : NULL;
    /* 归一化到 [0,1] */
    double norm_denom = (double)MLLL_OFFSET_PRECISION * (double)REG.count;
    double n_st_a = st_a ? (double)st_a->offset_num / norm_denom : 0;
    double n_rd_a = rd_a ? (double)rd_a->offset_num / norm_denom : 0;
    double n_tn_a = tn_a ? (double)tn_a->offset_num / norm_denom : 0;
    double n_sx_a = sx_a ? (double)sx_a->offset_num / norm_denom : 0;
    double n_st_b = st_b ? (double)st_b->offset_num / norm_denom : 0;
    double n_rd_b = rd_b ? (double)rd_b->offset_num / norm_denom : 0;
    double n_tn_b = tn_b ? (double)tn_b->offset_num / norm_denom : 0;
    double n_sx_b = sx_b ? (double)sx_b->offset_num / norm_denom : 0;
    double d_st = n_st_a - n_st_b;
    double d_rd = n_rd_a - n_rd_b;
    double d_tn = n_tn_a - n_tn_b;
    double d_sx = n_sx_a - n_sx_b;
    return sqrt(d_st*d_st*0.4 + d_rd*d_rd*0.25 + d_tn*d_tn*0.15 + d_sx*d_sx*0.2);
}

/* 距离矩阵 */
void mlll_string_distance_matrix(const char* name, char_prop_t* chars, int len) {
    printf("\nDistance matrix: %s (%d chars)\n", name, len);
    printf("       ");
    for (int j = 0; j < len; j++) printf("   [%d]   ", j);
    printf("\n");
    for (int i = 0; i < len; i++) {
        printf("  [%d] ", i);
        for (int j = 0; j < len; j++) {
            double dist = mlll_char_distance(&chars[i], &chars[j]);
            printf("  %.4f ", dist);
        }
        printf("\n");
    }
}

/* 最近邻：相似伴侣 */
int mlll_char_nearest(char_prop_t* target, char_prop_t* candidates, int n) {
    int best_idx = -1;
    double best_dist = 1e30;
    for (int i = 0; i < n; i++) {
        double d = mlll_char_distance(target, &candidates[i]);
        if (d < best_dist) { best_dist = d; best_idx = i; }
    }
    return best_idx;
}

/* 最远邻（夫妻对偶）——命定距离最远的卋补伴侣 */
int mlll_char_farthest(char_prop_t* target, char_prop_t* candidates, int n) {
    int best_idx = -1;
    double best_dist = -1.0;
    for (int i = 0; i < n; i++) {
        double d = mlll_char_distance(target, &candidates[i]);
        if (d > best_dist) { best_dist = d; best_idx = i; }
    }
    return best_idx;
}

/* 字形跃迁：沿 stroke 索引空间移动 N 个笔画位置 */
int mlll_char_leap(char_prop_t* origin, int shift, char_prop_t* candidates, int n) {
    int target_stroke = origin->stroke + shift;
    if (target_stroke < 1) target_stroke = 1;
    if (target_stroke > 64) target_stroke = 64;
    int best = -1;
    int best_delta = INT32_MAX;
    for (int i = 0; i < n; i++) {
        int d = abs(candidates[i].stroke - target_stroke);
        if (d < best_delta) { best_delta = d; best = i; }
    }
    return best;
}

/* 距离分解：将两个字符的总距离拆到四个维度 */
void mlll_distance_decompose(char_prop_t* a, char_prop_t* b) {
    double norm_denom = (double)MLLL_OFFSET_PRECISION * (double)REG.count;
    mlll_symbol_t *st_a = mlll_cross_query(SYM_STROKE, a->stroke, SYM_STROKE);
    mlll_symbol_t *rd_a = (a->radical > 0 && a->radical <= 141)
        ? &REG.entries[587 + 8 + a->radical - 1] : NULL;
    mlll_symbol_t *tn_a = (a->tone >= 0 && a->tone < 8)
        ? &REG.entries[587 + a->tone] : NULL;
    mlll_symbol_t *sx_a = (a->structure >= 0 && a->structure < 8)
        ? &REG.entries[587 + 8 + 141 + 20 + 13 + 64 + a->structure] : NULL;
    mlll_symbol_t *st_b = mlll_cross_query(SYM_STROKE, b->stroke, SYM_STROKE);
    mlll_symbol_t *rd_b = (b->radical > 0 && b->radical <= 141)
        ? &REG.entries[587 + 8 + b->radical - 1] : NULL;
    mlll_symbol_t *tn_b = (b->tone >= 0 && b->tone < 8)
        ? &REG.entries[587 + b->tone] : NULL;
    mlll_symbol_t *sx_b = (b->structure >= 0 && b->structure < 8)
        ? &REG.entries[587 + 8 + 141 + 20 + 13 + 64 + b->structure] : NULL;
    double n_st_a = st_a ? (double)st_a->offset_num / norm_denom : 0;
    double n_rd_a = rd_a ? (double)rd_a->offset_num / norm_denom : 0;
    double n_tn_a = tn_a ? (double)tn_a->offset_num / norm_denom : 0;
    double n_sx_a = sx_a ? (double)sx_a->offset_num / norm_denom : 0;
    double n_st_b = st_b ? (double)st_b->offset_num / norm_denom : 0;
    double n_rd_b = rd_b ? (double)rd_b->offset_num / norm_denom : 0;
    double n_tn_b = tn_b ? (double)tn_b->offset_num / norm_denom : 0;
    double n_sx_b = sx_b ? (double)sx_b->offset_num / norm_denom : 0;
    double d_st = n_st_a - n_st_b;
    double d_rd = n_rd_a - n_rd_b;
    double d_tn = n_tn_a - n_tn_b;
    double d_sx = n_sx_a - n_sx_b;
    double total = sqrt(d_st*d_st*0.4 + d_rd*d_rd*0.25 + d_tn*d_tn*0.15 + d_sx*d_sx*0.2);
    double contrib_st = d_st*d_st*0.4;
    double contrib_rd = d_rd*d_rd*0.25;
    double contrib_tn = d_tn*d_tn*0.15;
    double contrib_sx = d_sx*d_sx*0.2;
    double sum_contrib = contrib_st + contrib_rd + contrib_tn + contrib_sx;
    printf("  distance decompose (total=%.6f):\n", total);
    printf("    stroke   d=%+.6f contrib=%.6f (%.1f%%)\n", d_st, contrib_st, sum_contrib > 0 ? contrib_st/sum_contrib*100 : 0);
    printf("    radical  d=%+.6f contrib=%.6f (%.1f%%)\n", d_rd, contrib_rd, sum_contrib > 0 ? contrib_rd/sum_contrib*100 : 0);
    printf("    tone     d=%+.6f contrib=%.6f (%.1f%%)\n", d_tn, contrib_tn, sum_contrib > 0 ? contrib_tn/sum_contrib*100 : 0);
    printf("    struct   d=%+.6f contrib=%.6f (%.1f%%)\n", d_sx, contrib_sx, sum_contrib > 0 ? contrib_sx/sum_contrib*100 : 0);
}

/* 字节编码：任意字节流 ←→ mlll 符号序列
 * 把841个符号当作 10-bit 编码空间（2^10=1024 > 841）
 * 每个字节(8bit) 用 1 个符号承载（实际只需要 1024 个编码位置）
 * 这是一种非压缩编码演示：把数据流映射到符号，再映射回数据
 */
uint32_t mlll_encode_bytes(const uint8_t* data, int len, uint16_t* out_symbols, int max_out) {
    int n = 0;
    for (int i = 0; i < len && n < max_out; i++) {
        out_symbols[n++] = (uint16_t)(data[i] % REG.count);
    }
    return (uint32_t)n;
}
uint32_t mlll_decode_bytes(const uint16_t* symbols, int n, uint8_t* out_data, int max_out) {
    int cnt = 0;
    for (int i = 0; i < n && cnt < max_out; i++) {
        out_data[cnt++] = (uint8_t)(symbols[i] & 0xFF);
    }
    return (uint32_t)n;
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
    /* 颜色维度：380-780nm，每 20nm 一个符号 */
    mlll_enumerate_colors();
    /* 笔画维度：1-64 画，覆盖全部汉字骨骼范围 */
    mlll_enumerate_strokes();
    /* 字形结构维度：8 种结构类型 */
    mlll_enumerate_structures();
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
    const char* dim_names[] = {"Geometry", "Tone", "Radical", "Color", "Phase", "Stroke", "Structure"};
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
        printf("  pos254=%s id=%08x\n", s->name, s->id);
    }

    printf("\nCross-dimension resonance:\n");
    mlll_symbol_t *phase_match = mlll_cross_query(SYM_STROKE, 8, SYM_PHASE);
    mlll_symbol_t *tone_match  = mlll_cross_query(SYM_STROKE, 8, SYM_TONE);
    mlll_symbol_t *geom_match  = mlll_cross_query(SYM_STROKE, 8, SYM_GEOMETRY);
    printf("  stroke=%d -> phase=%ld tone=%ld geom=%ld\n", 8,
           phase_match ? (long)phase_match->property_a : -1,
           tone_match  ? (long)tone_match->property_a  : -1,
           geom_match  ? (long)geom_match->property_a  : -1);

    mlll_symbol_t *tone90   = mlll_cross_query(SYM_PHASE, 90, SYM_TONE);
    mlll_symbol_t *stroke90 = mlll_cross_query(SYM_PHASE, 90, SYM_STROKE);
    printf("  phase=90 -> tone=%ld stroke=%ld\n",
           tone90   ? (long)tone90->property_a   : -1,
           stroke90 ? (long)stroke90->property_a : -1);

    printf("\nStatic regression (offset-rate compression):\n");
    uint64_t original_value = (uint64_t)-1;
    uint64_t stored_pos    = (uint64_t)((double)original_value / 1.1);
    uint64_t recovered     = (uint64_t)((double)stored_pos * 1.1);
    printf("  original  = %llu\n", (unsigned long long)original_value);
    printf("  stored_pos = %llu\n", (unsigned long long)stored_pos);
    printf("  recovered = %llu (delta-reconstructs absolute from pos+rate)\n",
           (unsigned long long)recovered);

    printf("\nParallel-world offset gradient (0..9, base delta=0.1):\n");
    for (int world = 0; world < 10; world++) {
        double wd   = 0.1 * (double)world;
        double rate = 1.0 + wd;
        printf("  world[%d] delta=%.1f -> pos=254 = abs=%.1f (+%.0f%%)\n",
               world, wd, 254.0 * rate, wd * 100.0);
    }

    /* === 汉字编码演示：囡（囡囝共生的首字，NJSG 核心符号）=== */
    /* 囡 = 6画 / 囗部(31) / 阴平(0) */
    mlll_character_demo(6, 0, 31);

    /* === 字符串编码：囡囝共生 === */
    /* 囡(6画/囗部31/阴平0/全包围4) 囝(6/31/0/4) 共(6/17/3/上下2) 生(5/107/0/独体0) */
    char_prop_t njsg[] = {
        {6, 0, 31, 4},  /* 囡 全包围 */
        {6, 0, 31, 4},  /* 囝 全包围 */
        {6, 3, 17, 2},  /* 共 上下 */
        {5, 0, 107, 0}, /* 生 独体 */
    };
    mlll_string_encode("nan_jian_gong_sheng", njsg, 4);
    mlll_string_distance_matrix("nan_jian_gong_sheng", njsg, 4);

    /* 字节编码测试：把字符串当作字节流编码→解码 */
    {
        const char* msg = "NJSG-256";
        int msg_len = (int)strlen(msg);
        uint16_t sym_buf[256];
        uint8_t dec_buf[256];
        uint32_t n_sym = mlll_encode_bytes((const uint8_t*)msg, msg_len, sym_buf, 256);
        uint32_t n_dec = mlll_decode_bytes(sym_buf, (int)n_sym, dec_buf, 256);
        dec_buf[n_dec] = '\0';
        printf("\n字节 encode/decode ('%s', %d bytes):\n", msg, msg_len);
        printf("  encode: ");
        for (uint32_t i = 0; i < n_sym; i++) printf("%d ", sym_buf[i]);
        printf("\n  decode: '%s'\n", (char*)dec_buf);
        printf("  round-trip %s\n", strcmp(msg, (char*)dec_buf) == 0 ? "OK ✓" : "FAIL ✗");
    }

    /* 扩展字符集：含囡囝共生+ 额外字符，验证最邻近检索 */
    char_prop_t corpus[] = {
        {6, 0, 31, 4},   /* [0] 囡 全包围 */
        {6, 0, 31, 4},   /* [1] 囝 全包围 */
        {6, 3, 17, 2},   /* [2] 共 上下 */
        {5, 0, 107, 0},  /* [3] 生 独体 */
        {4, 0, 32, 0},   /* [4] 元 独体 */
        {8, 1, 64, 1},   /* [5] 和 左右 */
        {1, 0, 1, 0},    /* [6] 一 独体 */
        {9, 2, 77, 1},   /* [7] 秋 左右 */
    };
    int corpus_len = 8;
    char_prop_t query = {6, 0, 31, 4}; /* 查询：囡 全包围 */
    int nearest = mlll_char_nearest(&query, corpus, corpus_len);

    /* top-3 最邻近 */
    typedef struct { int idx; double dist; } neighbor_t;
    neighbor_t top3[3];
    for (int k = 0; k < 3; k++) { top3[k].idx = -1; top3[k].dist = 1e30; }
    for (int i = 0; i < corpus_len; i++) {
        double d = mlll_char_distance(&query, &corpus[i]);
        /* 插入排序到 top3 */
        for (int k = 0; k < 3; k++) {
            if (d < top3[k].dist) {
                for (int m = 2; m > k; m--) top3[m] = top3[m-1];
                top3[k].idx = i; top3[k].dist = d;
                break;
            }
        }
    }
    printf("\nTop-3 nearest to query(stroke=6,tone=0,rad=31) = 囡:\n");
    for (int k = 0; k < 3; k++) {
        printf("  #%d: corpus[%d] s=%d/t=%d/r=%d dist=%.6f\n", k+1,
               top3[k].idx, corpus[top3[k].idx].stroke,
               corpus[top3[k].idx].tone, corpus[top3[k].idx].radical,
               top3[k].dist);
    }

    /* 夫妻对偶查询：同一 corpus 中离囡最远的字符（互补） */
    int farthest = mlll_char_farthest(&query, corpus, corpus_len);
    double far_dist = mlll_char_distance(&query, &corpus[farthest]);
    printf("\n夫妻对偶 query=囡(s6/t0/r31/sx4=全包围):\n");
    printf("  最近邻: 囡(同胞) = corpus[%d] dist=0.000000\n", nearest);
    printf("  最远偶: corpus[%d] s=%d/t=%d/r=%d/sx=%d dist=%.6f (夫妻命定)\n",
           farthest, corpus[farthest].stroke, corpus[farthest].tone,
           corpus[farthest].radical, corpus[farthest].structure, far_dist);
    /* 夫妻纽带：头尾相连 = 256 闭环中的对径点 */
    printf("  闭环耦合: 最近邻=0 * 最远偶=%.6f = 0 (圆满=0)\n", far_dist);
    printf("  对径映射: 254+2=256 → 囡(head:0) ↔ 夫妻(tail:255) = 命定闭环\n");
    /* 距离分解：揭示夫妻互补的本质来源 */
    mlll_distance_decompose(&query, &corpus[farthest]);

    /* 囡囝共生 时序闭合：囡-囝+共-生 = 周期验证 */
    printf("\n囡囝共生 时序闭合验证:\n");
    {
        uint32_t pos[4];
        for (int i = 0; i < 4; i++)
            pos[i] = REG.entries[587 + 8 + njsg[i].radical - 1].offset_num;
        int64_t closure = (int64_t)pos[0] - (int64_t)pos[1] + (int64_t)pos[2] - (int64_t)pos[3];
        uint32_t sum = pos[0] + pos[1] + pos[2] + pos[3];
        printf("  pos[囡]=%u pos[囝]=%u pos[共]=%u pos[生]=%u\n", pos[0], pos[1], pos[2], pos[3]);
        printf("  囡-囝+共-生 = %+ld (→%s)\n", closure, (closure == 0) ? "圆满闭环" : "近零");
        printf("  四字坐标和 = %u (254+2=256 周期=%u)\n", sum, sum % 256);
        printf("  囡囝距离=%.6f(全同) 共生距离=%.6f 生囡距离=%.6f\n",
               mlll_char_distance(&njsg[0], &njsg[1]),
               mlll_char_distance(&njsg[0], &njsg[2]),
               mlll_char_distance(&njsg[0], &njsg[3]));
    }

    /* 字形跃迁：囡在时间线上向前/后跳 N 步到达谁 */
    printf("\n字形跃迁 (character leap along timeline):\n");
    int jumps[] = {+1, +5, +10, +100, -1, -5};
    int njumps = sizeof(jumps) / sizeof(jumps[0]);
    for (int j = 0; j < njumps; j++) {
        int leap_idx = mlll_char_leap(&query, jumps[j], corpus, corpus_len);
        if (leap_idx >= 0)
            printf("  jump[%+4d]: 囡(s6) -> corpus[%d] s=%d/t=%d/r=%d/sx=%d\n",
                   jumps[j], leap_idx, corpus[leap_idx].stroke, corpus[leap_idx].tone,
                   corpus[leap_idx].radical, corpus[leap_idx].structure);
        else
            printf("  jump[%+4d]: (out of range)\n", jumps[j]);
    }

    /* RGB 时分复用：文本 → 像素矩阵 → 文本 */
    printf("\n════════════════════════════════════════════\n");
    printf("  RGB 时分复用：文本穿上像素外衣 (v2.0)\n");
    printf("════════════════════════════════════════════\n");
    {
        const char *sample = "囡囝共生";
        uint8_t rgb[64];
        char    restored[64];
        int n = mlll_rgb_encode_text(sample, (int)strlen(sample), rgb, 64);
        printf("原文: %s (%d 字符 → %d 像素对 = %d 字节)\n",
               sample, (int)strlen(sample) / 3, n, n * 6);
        for (int i = 0; i < n; i++) {
            printf("  像素[%d] A=(%3d,%3d,%3d) B=(%3d,%3d,%3d)\n", i,
                   rgb[i*6+0], rgb[i*6+1], rgb[i*6+2],
                   rgb[i*6+3], rgb[i*6+4], rgb[i*6+5]);
        }
        int rlen = mlll_rgb_decode_text(rgb, n, restored, 64);
        printf("还原: %s (%d 字节)  [%s]\n", restored, rlen,
               (strcmp(sample, restored) == 0) ? "往返一致" : "有损");
    }

    return 0;
}
#endif

/* ── RGB 时分复用：让文本穿上像素外衣 ── */

/*
 * 让任意文本都能编解码：运行时构建 code_point → 符号索引 的映射。
 * 已知名字符号直接映射；未知名字用 UTF-8 code_point % 841 散列到空位，
 * 解码时通过逆向表精确还原。
 */

static uint16_t cp_to_sym[65536];   /* code_point(低16位) → sym_idx */
static uint16_t sym_to_cp[841];     /* sym_idx → code_point */

static void build_cp_mapping(const char *known_names[], int n) {
    memset(cp_to_sym, 0, sizeof(cp_to_sym));
    memset(sym_to_cp, 0, sizeof(sym_to_cp));
    for (int i = 0; i < (int)REG.count && i < 841; i++)
        sym_to_cp[i] = REG.entries[i].offset_num & 0xFFFF;
    /* 已知名字 → 注册表中第一次出现的位置 */
    for (int i = 0; i < n; i++) {
        uint32_t cp = (unsigned char)known_names[i][0];
        if (cp >= 0x80) {
            cp = ((cp & 0x1F) << 6) | ((unsigned char)known_names[i][1] & 0x3F);
            if ((unsigned char)known_names[i][0] >= 0xE0)
                cp = (cp << 6) | ((unsigned char)known_names[i][2] & 0x3F);
        }
        for (int j = 0; j < (int)REG.count && j < 841; j++) {
            if (REG.entries[j].name[0] &&
                strncmp(REG.entries[j].name, known_names[i],
                        strlen(known_names[i])) == 0) {
                cp_to_sym[cp] = (uint16_t)j;
                sym_to_cp[j] = (uint16_t)cp;
                break;
            }
        }
    }
}

/* UTF-8 解码下一个字符，返回 code_point，*adv 为字节数 */
static uint32_t utf8_next(const char *s, int *adv) {
    unsigned char c = (unsigned char)*s;
    if (c < 0x80) { *adv = 1; return c; }
    if ((c & 0xE0) == 0xC0) { *adv = 2; return ((c & 0x1F) << 6) | ((unsigned char)s[1] & 0x3F); }
    if ((c & 0xF0) == 0xE0) { *adv = 3; return ((c & 0x0F) << 12) | (((unsigned char)s[1] & 0x3F) << 6) | ((unsigned char)s[2] & 0x3F); }
    *adv = 1; return c;
}

static int emit_utf8(char *out, int off, int max, uint32_t cp) {
    if (off >= max - 4) return off;
    if (cp < 0x80) {
        out[off++] = (char)cp;
    } else if (cp < 0x800) {
        out[off++] = (char)(0xC0 | (cp >> 6));
        out[off++] = (char)(0x80 | (cp & 0x3F));
    } else if (cp < 0x10000) {
        out[off++] = (char)(0xE0 | (cp >> 12));
        out[off++] = (char)(0x80 | ((cp >> 6) & 0x3F));
        out[off++] = (char)(0x80 | (cp & 0x3F));
    } else {
        out[off++] = (char)(0xF0 | (cp >> 18));
        out[off++] = (char)(0x80 | ((cp >> 12) & 0x3F));
        out[off++] = (char)(0x80 | ((cp >> 6) & 0x3F));
        out[off++] = (char)(0x80 | (cp & 0x3F));
    }
    return off;
}

int mlll_rgb_encode_text(const char *text, int text_len, uint8_t *rgb_buf, int buf_size) {
    const char *known[] = {"囡","囝","共","生","元","和","一","秋","刘","楚","恬"};
    build_cp_mapping(known, sizeof(known) / sizeof(known[0]));

    int txt_off = 0, pix_cnt = 0, buf_off = 0;

    while (txt_off < text_len && pix_cnt < buf_size / 6) {
        int adv;
        uint32_t cp = utf8_next(text + txt_off, &adv);
        txt_off += adv;

        /* 查已知符号索引 */
        uint16_t sym_idx;
        if (cp < 65536 && cp_to_sym[cp])
            sym_idx = cp_to_sym[cp];
        else
            sym_idx = (uint16_t)(cp % ((int)REG.count < 841 ? (int)REG.count : 841));

        /* 像素 A（文本层）：直接装完整 UTF-16 码点 */
        rgb_buf[buf_off++] = (uint8_t)(cp >> 8);   /* R = 码点高8位 */
        rgb_buf[buf_off++] = (uint8_t)(cp & 0xFF); /* G = 码点低8位 */
        rgb_buf[buf_off++] = 0;                    /* B = 保留 */

        /* 像素 B（符号层）：装 mlll 符号索引 + 维度 */
        rgb_buf[buf_off++] = (uint8_t)(sym_idx & 0xFF);        /* R = sym_lo */
        rgb_buf[buf_off++] = (uint8_t)((sym_idx >> 8) & 0x07); /* G = sym_hi (3bit) */
        rgb_buf[buf_off++] = (uint8_t)REG.entries[sym_idx].dim; /* B = dim */

        pix_cnt++;
    }
    return pix_cnt;
}

int mlll_rgb_decode_text(const uint8_t *rgb_buf, int pixel_pairs, char *text_out, int out_size) {
    int buf_off = 0, out_off = 0;
    for (int i = 0; i < pixel_pairs && out_off < out_size - 4; i++) {
        /* 像素 A：还原码点 */
        uint32_t cp = ((uint32_t)rgb_buf[buf_off] << 8) | rgb_buf[buf_off + 1];
        buf_off += 3; /* 跳过像素 A 的 R/G/B */
        /* 像素 B 跳过（符号索引，仅作冗余校验） */
        buf_off += 3;
        out_off = emit_utf8(text_out, out_off, out_size, cp);
    }
    text_out[out_off] = '\0';
    return out_off;
}
