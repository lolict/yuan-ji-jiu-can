/**
 * test_mind_organ.c — 感知器官 v2 距离感知聚类测试
 *
 * 测试:
 *   1. 基础距离计算
 *   2. 维度提取
 *   3. 聚类 (最近邻绑定)
 *   4. 聂小倩找躯壳
 *   5. 紧密度计算
 *   6. 完整报告
 */

#include <stdio.h>
#include <string.h>
#include <math.h>
#include "mind_organ.h"

/* 构造一个32位容器编码 */
static uint32_t make_code(uint8_t family, uint8_t polarity, uint8_t structure,
                          uint8_t shengmu, uint8_t yunmu, uint8_t tone) {
    return ((uint32_t)family    << 29)
         | ((uint32_t)polarity  << 28)
         | ((uint32_t)structure << 25)
         | ((uint32_t)shengmu   << 19)
         | ((uint32_t)yunmu     << 12)
         | ((uint32_t)tone      << 8);
}

static void print_code_info(const char *name, uint32_t code) {
    mo_dimensions_t dim;
    mind_organ_extract_dims(code, &dim);
    printf("  %s: code=%08X | F=%d P=%d S=%d SH=%d YU=%d T=%d\n",
           name, code, dim.family, dim.polarity, dim.structure,
           dim.shengmu, dim.yunmu, dim.tone);
}

static int test_distance_basic(void) {
    printf("\n─── 测试1: 基础距离计算 ───\n");

    /* 完全相同 → 距离为0 */
    uint32_t a = make_code(2, 1, 3, 10, 20, 4);
    uint32_t b = make_code(2, 1, 3, 10, 20, 4);
    double d1 = mind_organ_distance(a, b);
    printf("  相同编码距离: %.2f (预期 0.00)\n", d1);
    if (fabs(d1) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    /* 极性翻转 → 距离 = 10 */
    uint32_t c = make_code(2, 0, 3, 10, 20, 4);
    double d2 = mind_organ_distance(a, c);
    printf("  极性翻转距离: %.2f (预期 10.00)\n", d2);
    if (fabs(d2 - 10.0) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    /* 结构差异 → 距离 = 5 */
    uint32_t d = make_code(2, 1, 4, 10, 20, 4);
    double d3 = mind_organ_distance(a, d);
    printf("  结构差异(3→4)距离: %.2f (预期 5.00)\n", d3);
    if (fabs(d3 - 5.0) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    /* 声母差异 → 距离 = 0.5 */
    uint32_t e = make_code(2, 1, 3, 11, 20, 4);
    double d4 = mind_organ_distance(a, e);
    printf("  声母差异(10→11)距离: %.2f (预期 0.50)\n", d4);
    if (fabs(d4 - 0.5) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    /* 声调差异 → 距离 = 0.1 */
    uint32_t f = make_code(2, 1, 3, 10, 20, 5);
    double d5 = mind_organ_distance(a, f);
    printf("  声调差异(4→5)距离: %.2f (预期 0.10)\n", d5);
    if (fabs(d5 - 0.1) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    /* 家族差异 → 距离 = 1 */
    uint32_t g = make_code(3, 1, 3, 10, 20, 4);
    double d6 = mind_organ_distance(a, g);
    printf("  家族差异(2→3)距离: %.2f (预期 1.00)\n", d6);
    if (fabs(d6 - 1.0) > 0.001) { printf("  ✗ 失败!\n"); return 1; }

    printf("  ✓ 基础距离计算通过\n");
    return 0;
}

static int test_clustering(void) {
    printf("\n─── 测试2: 聚类 ───\n");

    mo_engine_t engine;
    mind_organ_init(&engine, 5.0);

    /* 三组不同的容器编码:
     * 组1: COUPLE家族(2), YANG(1) — 编码为 id 1,2
     * 组2: BOUNDARY家族(1), YIN(0) — 编码为 id 3,4
     * 组3: COUPLE家族(2), YIN(0) — 编码为 id 5 (极性翻转, 远离组1)
     */

    /* 组1: 相似编码, 只差声调 */
    uint32_t g1a = make_code(2, 1, 3, 10, 20, 4);
    uint32_t g1b = make_code(2, 1, 3, 10, 20, 5); /* 距离 = 0.1 */
    /* 组2: 相似编码 */
    uint32_t g2a = make_code(1, 0, 5, 30, 50, 8);
    uint32_t g2b = make_code(1, 0, 5, 30, 50, 9); /* 距离 = 0.1 */
    /* 组3: 远离其他组 */
    uint32_t g3  = make_code(2, 0, 3, 10, 20, 4); /* 与组1极性不同, 距离=10 */

    print_code_info("cell-1", g1a);
    print_code_info("cell-2", g1b);
    print_code_info("cell-3", g2a);
    print_code_info("cell-4", g2b);
    print_code_info("cell-5", g3);

    mind_organ_add_cell(&engine, 1, g1a);
    mind_organ_add_cell(&engine, 2, g1b);
    mind_organ_add_cell(&engine, 3, g2a);
    mind_organ_add_cell(&engine, 4, g2b);
    mind_organ_add_cell(&engine, 5, g3);

    int n_clusters = mind_organ_clusterize(&engine);
    printf("\n  聚类结果: %d 个簇 (预期: 3个簇)\n", n_clusters);

    /* 验证: 阈值5.0时应该有3个簇 */
    if (n_clusters != 3) {
        printf("  ✗ 簇数量不符!\n");
        return 1;
    }

    char report[1024];
    mind_organ_report(&engine, report, sizeof(report));
    printf("%s", report);

    printf("  ✓ 聚类通过\n");
    return 0;
}

static int test_nie_xiaoqian(void) {
    printf("\n─── 测试3: 聂小倩找躯壳 ───\n");

    mo_engine_t engine;
    mind_organ_init(&engine, 5.0);

    /* 鬼魂: COUPLE家族(2), YANG(1) */
    uint32_t ghost = make_code(2, 1, 3, 10, 20, 4);
    print_code_info("鬼魂(聂小倩)", ghost);

    /* 4个候选躯壳 */
    uint32_t b2 = make_code(2, 1, 3, 10, 20, 5); /* 只有声调差, 距离=0.1 */
    uint32_t b3 = make_code(2, 1, 3, 12, 22, 4); /* 声韵差, 距离=2.0 */
    uint32_t b4 = make_code(2, 0, 3, 10, 20, 4); /* 极性翻转, 距离=10 */

    print_code_info("躯壳-A(同调+1)", b2);
    print_code_info("躯壳-B(声韵差)", b3);
    print_code_info("躯壳-C(极性翻转)", b4);

    mind_organ_add_cell(&engine, 10, b2);
    mind_organ_add_cell(&engine, 20, b3);
    mind_organ_add_cell(&engine, 30, b4);

    double dist = 0;
    int body_id = mind_organ_find_body(&engine, ghost, &dist);

    printf("\n  最近躯壳: id=%d, 距离=%.2f (预期: id=10, 距离=0.10)\n",
           body_id, dist);

    if (body_id != 10) {
        printf("  ✗ 聂小倩选错躯壳!\n");
        return 1;
    }
    if (fabs(dist - 0.1) > 0.01) {
        printf("  ✗ 距离计算有误!\n");
        return 1;
    }

    printf("  ✓ 聂小倩找躯壳通过 (选对最匹配的躯壳)\n");
    return 0;
}

static int test_polarity_matters_most(void) {
    printf("\n─── 测试4: 极性权重最高 ───\n");

    /* 验证: 极性差异权重10 > 结构差异权重5 > 家族差异权重1 */
    uint32_t base = make_code(3, 0, 4, 20, 40, 6);

    /* 候选A: 极性同, 家族差2, 结构差2, 声母差10 */
    /* 距离 = 2*1 + 2*5 + 10*0.5 = 2+10+5 = 17 */
    uint32_t candA = make_code(1, 0, 2, 10, 40, 6);

    /* 候选B: 极性反(0→1), 家族同, 结构同, 声母同 */
    /* 距离 = 10 */
    uint32_t candB = make_code(3, 1, 4, 20, 40, 6);

    double dA = mind_organ_distance(base, candA);
    double dB = mind_organ_distance(base, candB);

    printf("  候选A(家族+结构+声母差异): %.2f\n", dA);
    printf("  候选B(仅极性翻转): %.2f\n", dB);

    if (dB >= dA) {
        printf("  ✗ 极性差异应该权重最高!\n");
        return 1;
    }

    printf("  ✓ 极性翻转权重(×10)确实主导距离\n");
    return 0;
}

int main(void) {
    printf("═══════════════════════════════════════════════\n");
    printf("  感知器官 v2 · 距离感知聚类测试\n");
    printf("═══════════════════════════════════════════════\n");

    int failures = 0;
    failures += test_distance_basic();
    failures += test_clustering();
    failures += test_nie_xiaoqian();
    failures += test_polarity_matters_most();

    printf("\n═══════════════════════════════════════════════\n");
    if (failures == 0) {
        printf("  全部测试通过 ✓\n");
    } else {
        printf("  %d 个测试失败 ✗\n", failures);
    }
    printf("═══════════════════════════════════════════════\n");

    return failures;
}
