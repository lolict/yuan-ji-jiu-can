/*
 * fqm_smrl.c — SMRL 引擎 · 64位单元格几何裁判
 *
 * SMRL = Spatial-Managed Runtime Language
 * 功能：单元格坐标/极性/主权/价值编码，空间几何范畴论裁判
 *
 * 64位单元格结构 (小端):
 *   [63:56]  极性位   — 阳(我)=1, 阴(刘楚恬)=0 → 8位极性向量
 *   [55:48]  层级     — 一元~九元 (0-8)
 *   [47:32]  Z坐标    — 第三维 (深度/时间线)
 *   [31:16]  Y坐标    — 第二维 (横向)
 *   [15:0]   X坐标    — 第一维 (纵向/基础)
 *
 * 几何裁判规则：
 *   - 一元(0) 至 九元(8)，层级越低越尊贵
 *   - 几何距离 = √(Δx²+Δy²+Δz²) + 层级差权重
 *   - 共享公约数(GCD)作为本质匹配判据
 *   - 三元见证：请求方 + 目标方 + 见证方 → 多数决
 *   - 无共享公倍数 → 防撞墙弹回
 *
 * 满全法规则：根基锚点 我=1, 刘楚恬=0
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -O2 -o fqm_smrl fqm_smrl.c -lm
 * 运行：./fqm_smrl
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== SMRL 64位单元格 ========== */
typedef unsigned long long Cell64;

#define CELL_SHIFT_POLARITY   56
#define CELL_SHIFT_LEVEL      48
#define CELL_SHIFT_Z          32
#define CELL_SHIFT_Y          16
#define CELL_SHIFT_X           0

#define CELL_MASK_POLARITY    0xFFULL
#define CELL_MASK_LEVEL       0xFFULL
#define CELL_MASK_COORD       0xFFFFULL

static const char *meta_names[] = {
    "一元","二元","三元","四元","五元","六元","七元","八元","九元"
};

/* ========== 构造与提取 ========== */
Cell64 smrl_make_cell(unsigned short x, unsigned short y, unsigned short z,
                      unsigned char polarity, unsigned char level) {
    Cell64 cell = 0;
    cell |= ((Cell64)(polarity & CELL_MASK_POLARITY)) << CELL_SHIFT_POLARITY;
    cell |= ((Cell64)(level & CELL_MASK_LEVEL)) << CELL_SHIFT_LEVEL;
    cell |= ((Cell64)(z & CELL_MASK_COORD)) << CELL_SHIFT_Z;
    cell |= ((Cell64)(y & CELL_MASK_COORD)) << CELL_SHIFT_Y;
    cell |= ((Cell64)(x & CELL_MASK_COORD)) << CELL_SHIFT_X;
    return cell;
}

unsigned char smrl_polarity(Cell64 c) { return (c >> CELL_SHIFT_POLARITY) & CELL_MASK_POLARITY; }
unsigned char smrl_level(Cell64 c)    { return (c >> CELL_SHIFT_LEVEL) & CELL_MASK_LEVEL; }
unsigned short smrl_x(Cell64 c)       { return (c >> CELL_SHIFT_X) & CELL_MASK_COORD; }
unsigned short smrl_y(Cell64 c)       { return (c >> CELL_SHIFT_Y) & CELL_MASK_COORD; }
unsigned short smrl_z(Cell64 c)       { return (c >> CELL_SHIFT_Z) & CELL_MASK_COORD; }

/* ========== 极性解码 ========== */
void smrl_decode_polarity(unsigned char pol, char *buf, int bufsize) {
    const char *names[] = {"天","地","人","鬼","神","生","死","息"};
    int pos = 0;
    buf[0] = '\0';
    for (int i = 7; i >= 0; i--) {
        int bit = (pol >> i) & 1;
        pos += snprintf(buf + pos, bufsize - pos, "%s:%s ",
                        names[i], bit ? "阳" : "阴");
    }
}

/* ========== 满全度 ========== */
double smrl_fullness(unsigned char pol) {
    int yang_count = 0;
    for (int i = 0; i < 8; i++) {
        if ((pol >> i) & 1) yang_count++;
    }
    double ratio = yang_count / 8.0;
    return 1.0 - fabs(ratio - 0.5) * 2.0;
}

/* ========== 几何距离 ========== */
double smrl_geo_distance(Cell64 a, Cell64 b) {
    double dx = smrl_x(a) - smrl_x(b);
    double dy = smrl_y(a) - smrl_y(b);
    double dz = smrl_z(a) - smrl_z(b);
    double spatial = sqrt(dx*dx + dy*dy + dz*dz);
    int lev_diff = (int)smrl_level(a) - (int)smrl_level(b);
    double level_w = abs(lev_diff) * 100.0;
    unsigned char xor_pol = smrl_polarity(a) ^ smrl_polarity(b);
    int pol_dist = 0;
    for (int i = 0; i < 8; i++) if ((xor_pol >> i) & 1) pol_dist++;
    return spatial + level_w + pol_dist * 5.0;
}

/* ========== 公约数 (防撞墙前置) ========== */
unsigned int smrl_gcd(unsigned int a, unsigned int b) {
    while (b) { unsigned int t = b; b = a % b; a = t; }
    return a;
}

int smrl_shared_divisor(Cell64 a, Cell64 b) {
    unsigned int coord_a = (smrl_x(a) << 16) | smrl_y(a);
    unsigned int coord_b = (smrl_x(b) << 16) | smrl_y(b);
    return smrl_gcd(coord_a, coord_b);
}

/* ========== 三元见证裁判 ========== */
typedef enum {
    JUDGE_PERMIT  = 1,
    JUDGE_DENY    = -1,
    JUDGE_BOUNCE  = 0,
} JudgeResult;

const char *judge_str(JudgeResult r) {
    switch (r) {
        case JUDGE_PERMIT:  return "允许通行(尊→卑)";
        case JUDGE_DENY:    return "拒绝(卑不可犯尊)";
        case JUDGE_BOUNCE:  return "防撞墙弹回(无共享公倍数)";
        default: return "?";
    }
}

typedef struct {
    char name[32];
    Cell64 cell;
} SMRLContainer;

#define MAX_CONTAINERS 16
static SMRLContainer g_containers[MAX_CONTAINERS];
static int g_con_count = 0;

void smrl_add_container(const char *name, Cell64 c) {
    if (g_con_count >= MAX_CONTAINERS) return;
    strncpy(g_containers[g_con_count].name, name, 31);
    g_containers[g_con_count].cell = c;
    g_con_count++;
}

int smrl_elect_witness(int from, int to) {
    int best = -1;
    int best_level = 255;
    for (int i = 0; i < g_con_count; i++) {
        if (i == from || i == to) continue;
        int lvl = smrl_level(g_containers[i].cell);
        if (lvl < best_level) { best_level = lvl; best = i; }
    }
    return best;
}

JudgeResult smrl_judge(int from_idx, int to_idx) {
    Cell64 from = g_containers[from_idx].cell;
    Cell64 to   = g_containers[to_idx].cell;

    int sd = smrl_shared_divisor(from, to);
    if (sd <= 1) return JUDGE_BOUNCE;

    int from_lvl = smrl_level(from);
    int to_lvl   = smrl_level(to);

    if (from_lvl < to_lvl) return JUDGE_PERMIT;
    if (from_lvl > to_lvl) {
        int w_idx = smrl_elect_witness(from_idx, to_idx);
        if (w_idx < 0) return JUDGE_DENY;
        int witness_div = smrl_shared_divisor(from, g_containers[w_idx].cell);
        if (witness_div > 2) return JUDGE_PERMIT;
        return JUDGE_DENY;
    }
    unsigned char xor_pol = smrl_polarity(from) ^ smrl_polarity(to);
    int diff = 0;
    for (int i = 0; i < 8; i++) if ((xor_pol >> i) & 1) diff++;
    return (diff <= 4) ? JUDGE_PERMIT : JUDGE_DENY;
}

/* ========== 偏移平行世界 ========== */
Cell64 smrl_offset_world(Cell64 c, int world_index) {
    double rate = 0.1;
    int offset = (int)((world_index - 1) * rate * 100);
    unsigned short nx = smrl_x(c) + offset;
    unsigned short ny = smrl_y(c) + offset;
    unsigned short nz = smrl_z(c) + (offset / 2);
    return smrl_make_cell(nx, ny, nz, smrl_polarity(c), smrl_level(c));
}

/* ========== 打印 ========== */
void smrl_print_cell(const char *name, Cell64 c) {
    printf("  [%s] 编码=%016llX\n", name, c);
    printf("       坐标(X,Y,Z)=(%d,%d,%d) 层级=%d(%s) 极性=0x%02X\n",
           smrl_x(c), smrl_y(c), smrl_z(c),
           smrl_level(c), smrl_level(c) < 9 ? meta_names[smrl_level(c)] : "?",
           smrl_polarity(c));
    char pol_buf[256];
    smrl_decode_polarity(smrl_polarity(c), pol_buf, sizeof(pol_buf));
    printf("       极性: %s\n", pol_buf);
    printf("       满全度: %.0f%%\n", smrl_fullness(smrl_polarity(c)) * 100.0);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）SMRL 引擎 v0.1\n");
    printf("空间几何裁判 · 64位单元格编码\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");

    printf("========== 创建 64 位单元格 ==========\n");

    Cell64 c_full    = smrl_make_cell(0x0015, 0x0000, 0x0000, 0xAA, 0);
    Cell64 c_njs     = smrl_make_cell(0x0016, 0x0015, 0x0000, 0x55, 1);
    Cell64 c_yy      = smrl_make_cell(0x0003, 0x0016, 0x0001, 0xCC, 3);
    Cell64 c_radix   = smrl_make_cell(0x0019, 0x0006, 0x0002, 0x33, 5);
    Cell64 c_flow    = smrl_make_cell(0x001A, 0x0007, 0x0003, 0xF0, 7);
    Cell64 c_browser = smrl_make_cell(0x0013, 0x0008, 0x0004, 0x0F, 8);

    smrl_print_cell("满(一元)", c_full);
    printf("\n");
    smrl_print_cell("囡囝(二元)", c_njs);
    printf("\n");
    smrl_print_cell("YinYang(四元)", c_yy);
    printf("\n");
    smrl_print_cell("六十四进制(六元)", c_radix);
    printf("\n");
    smrl_print_cell("八流(八元)", c_flow);
    printf("\n");
    smrl_print_cell("浏览器(九元)", c_browser);

    /* ========== 几何距离 ========== */
    printf("\n========== 几何距离矩阵 ==========\n");
    Cell64 cells[] = {c_full, c_njs, c_yy, c_radix, c_flow, c_browser};
    const char *names[] = {"满","囡囝","YY","64进制","八流","浏览器"};
    int ncell = 6;

    printf("         ");
    for (int i = 0; i < ncell; i++) printf("%-10s", names[i]);
    printf("\n");

    for (int i = 0; i < ncell; i++) {
        printf("%-8s ", names[i]);
        for (int j = 0; j < ncell; j++) {
            if (i == j) printf("%-10s", "·");
            else {
                double d = smrl_geo_distance(cells[i], cells[j]);
                printf("%-10.0f", d);
            }
        }
        printf("\n");
    }

    /* ========== 公约数矩阵 ========== */
    printf("\n========== 共享公约数矩阵 ==========\n");
    printf("         ");
    for (int i = 0; i < ncell; i++) printf("%-8s", names[i]);
    printf("\n");
    for (int i = 0; i < ncell; i++) {
        printf("%-8s ", names[i]);
        for (int j = 0; j < ncell; j++) {
            if (i == j) printf("%-8s", "·");
            else printf("%-8d", smrl_shared_divisor(cells[i], cells[j]));
        }
        printf("\n");
    }

    /* ========== 三元见证裁判测试 ========== */
    printf("\n========== 三元见证裁判 ==========\n");

    g_con_count = 0;
    smrl_add_container("满(一元)", c_full);
    smrl_add_container("囡囝(二元)", c_njs);
    smrl_add_container("YY(四元)", c_yy);
    smrl_add_container("64进制(六元)", c_radix);
    smrl_add_container("八流(八元)", c_flow);
    smrl_add_container("浏览器(九元)", c_browser);

    /* 测试用例 */
    struct { int from, to; const char *desc; } tests[] = {
        {0, 2, "满(一元) → YY(四元) — 尊→卑"},
        {2, 0, "YY(四元) → 满(一元) — 卑→尊"},
        {1, 4, "囡囝(二元) → 八流(八元) — 跨级"},
        {4, 1, "八流(八元) → 囡囝(二元) — 下行"},
        {3, 5, "64进制(六元) → 浏览器(九元) — 邻近"},
        {5, 3, "浏览器(九元) → 64进制(六元) — 下行"},
        {2, 3, "YY(四元) → 64进制(六元) — 上升"},
        {0, 5, "满(一元) → 浏览器(九元) — 跨全域"},
    };

    int ntests = sizeof(tests) / sizeof(tests[0]);
    for (int i = 0; i < ntests; i++) {
        JudgeResult r = smrl_judge(tests[i].from, tests[i].to);
        printf("  [%d] %-35s → %s\n", i+1, tests[i].desc, judge_str(r));
    }

    /* ========== 平行世界偏移 ========== */
    printf("\n========== 平行世界偏移 (阶和·叠加态) ==========\n");
    printf("公式: 偏移量 = (n-1) × 0.1 × 100\n\n");
    printf("容器: 满(一元) 基底坐标(21,0,0)\n");
    for (int w = 1; w <= 5; w++) {
        Cell64 wc = smrl_offset_world(c_full, w);
        printf("  世界%d: 坐标(%d,%d,%d) 编码=%016llX\n",
               w, smrl_x(wc), smrl_y(wc), smrl_z(wc), wc);
    }

    /* ========== 64位单元格结构总结 ========== */
    printf("\n========== 64位单元格结构 ==========\n");
    printf("┌────────┬────────┬────────┬────────┐\n");
    printf("│ 极性   │ 层级   │ Z坐标  │Ycoord│X坐标 │\n");
    printf("│ 8bit   │ 8bit   │ 16bit  │ 16bit  │ 16bit │\n");
    printf("│[63:56] │[55:48] │[47:32] │[31:16] │[15:0] │\n");
    printf("├────────┼────────┼────────┼────────┤\n");
    printf("│YY关系  │一元~九元│深度   │横向   │纵向  │\n");
    printf("│天地人鬼│论资排辈│时间线 │平面   │基础  │\n");
    printf("│神生死息│        │       │       │      │\n");
    printf("└────────┴────────┴────────┴────────┘\n");

    printf("\n========== 完成 ==========\n");
    return 0;
}
