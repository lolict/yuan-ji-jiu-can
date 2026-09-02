/**
 * test_mind_organ.c — 距离感知聚类测试
 */

#include "mind_organ.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    printf("═══ 距离感知聚类测试 ═══\n");
    
    /* 构造一组测试 cell */
    /* 簇 1: 主权接近, 极性相同 */
    NJSG_uint64 cells[] = {
        WK_CELL(0x1000, 0x10000, WK_POLARITY_WIFE, 1),   /* A */
        WK_CELL(0x1001, 0x10001, WK_POLARITY_WIFE, 1),   /* B - 接近 A */
        WK_CELL(0x1002, 0x10002, WK_POLARITY_WIFE, 2),   /* C - 接近 A */
        WK_CELL(0x2000, 0x50000, WK_POLARITY_HUSBAND, 5), /* D - 远离 */
        WK_CELL(0x2001, 0x50001, WK_POLARITY_HUSBAND, 5), /* E - 接近 D */
        WK_CELL(0x3000, 0x90000, WK_POLARITY_SYSTEM, 9),  /* F - 远离 */
    };
    NJSG_uint32 count = 6;
    
    /* 1. 距离计算 */
    printf("\n[1] 距离计算\n");
    printf("  d(A,B) = %.4f\n", mo_distance(cells[0], cells[1], MO_DIST_GEOMETRIC));
    printf("  d(A,D) = %.4f\n", mo_distance(cells[0], cells[3], MO_DIST_GEOMETRIC));
    printf("  d(D,E) = %.4f\n", mo_distance(cells[3], cells[4], MO_DIST_GEOMETRIC));
    printf("  d(A,F) = %.4f\n", mo_distance(cells[0], cells[5], MO_DIST_GEOMETRIC));
    
    /* 2. 最近邻 */
    printf("\n[2] 最近邻\n");
    int nn = mo_nearest(cells, count, 0, MO_DIST_GEOMETRIC);
    printf("  A 的最近邻: 索引 %d\n", nn);
    
    /* 3. 聚类 */
    printf("\n[3] DBSCAN 聚类\n");
    mo_result_t result;
    memset(&result, 0, sizeof(result));
    mo_cluster(cells, count, 0.3f, 2, MO_DIST_GEOMETRIC, &result);
    
    char stats[512];
    mo_stats(&result, stats, sizeof(stats));
    printf("  %s\n", stats);
    
    /* 4. 不同距离度量 */
    printf("\n[4] 不同距离度量\n");
    mo_dist_type_t types[] = {MO_DIST_GEOMETRIC, MO_DIST_SOVEREIGN, MO_DIST_POLARITY, MO_DIST_WITNESS};
    const char *names[] = {"几何", "主权", "极性", "见证"};
    for (int t = 0; t < 4; t++) {
        memset(&result, 0, sizeof(result));
        mo_cluster(cells, count, 0.3f, 2, types[t], &result);
        printf("  %s: %u 簇, %u 噪声, 内聚度=%.0f%%\n",
               names[t], result.cluster_count, result.noise_count, result.avg_cohesion * 100);
    }
    
    printf("\n═══ 测试完成 ═══\n");
    return 0;
}
