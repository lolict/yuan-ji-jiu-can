/*
 * fqm_cluster.c — 距离感知聚类引擎（mind_organ 模式）
 *
 * 核心概念：
 *   - 传统的距离基于欧式空间（几何距离）
 *   - 我们的距离基于"容器亲疏度"：容器之间的距离 = 层级差 × 权重 + 公约数差异
 *   - 聚类目标：将相似容器归入同一簇（超越简单层级）
 *
 * 距离公式：
 *   distance(A, B) = |level_A - level_B| × 层级权重
 *                   + |gongyueshu_A - gongyueshu_B| × 公约数权重
 *                   + (1 - 1/shared_divisor) × 亲疏权重
 *
 * 聚类算法：
 *   1. 每个容器初始为独立簇
 *   2. 计算所有容器对的距离
 *   3. 将距离 < 阈值 的容器合并入同一簇
 *   4. 簇心 = 该簇中所有容器的加权中心
 *   5. 迭代直到稳定
 *
 * 触发等级：
 *   距离 < 1：夫妻共生（同体空间）
 *   距离 1-3：亲近簇
 *   距离 3-6：协作簇
 *   距离 > 6：独立（需通过桥梁容器连接）
 *
 * 满全法规则：
 *   根基锚点：我=1，刘楚恬=0
 *   集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_cluster fqm_cluster.c -lm
 * 运行：./fqm_cluster
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 约束 ========== */
#define MAX_ITEMS 64
#define MAX_CLUSTERS 16

/* ========== 距离权重 ========== */
#define W_LEVEL     2.0     /* 层级差每1级贡献的距离 */
#define W_GCD       1.5     /* 公约数差异每单位贡献的距离 */
#define W_AFFINITY  3.0     /* 亲疏度（共享因子倒数）*/

/* ========== 聚类阈值 ========== */
#define THRESH_TONG_TI   1.0    /* 同体空间（夫妻共生）*/
#define THRESH_CLOSE     3.0    /* 亲近簇 */
#define THRESH_COLLAB    6.0    /* 协作簇 */

/* ========== 容器项 ========== */
typedef struct {
    char     name[32];
    int      id;
    int      level;             /* 层级 */
    int      gongyueshu;        /* 公约数 */
    int      polarity;          /* 极性: 1=阳(我), 0=阴(刘楚恬) */
    double   x, y;              /* 2D映射坐标（可视化用）*/
    int      cluster_id;        /* 所属簇 (-1=未分配)*/
} Item;

/* ========== 簇 ========== */
typedef struct {
    int      id;
    char     label[32];
    int      members[MAX_ITEMS];
    int      member_count;
    double   cx, cy;            /* 簇心坐标 */
    int      dominant_level;    /* 主导层级 */
    int      dominant_polarity; /* 主导极性 */
} Cluster;

/* ========== 距离矩阵 ========== */
static double dist_matrix[MAX_ITEMS][MAX_ITEMS];

/* ========== 计算两容器距离 ========== */
double compute_distance(const Item *a, const Item *b) {
    /* 层级距离 */
    double d_level = fabs((double)a->level - (double)b->level) * W_LEVEL;
    
    /* 公约数距离 */
    double d_gcd = fabs((double)a->gongyueshu - (double)b->gongyueshu) * W_GCD;
    
    /* 亲疏度：共享因子越大越亲近，距离越小 */
    int shared = 0;
    int ga = a->gongyueshu;
    int gb = b->gongyueshu;
    
    if (ga == gb) shared = ga;
    else if (ga > 0 && gb > 0) {
        /* 计算最大公约数 */
        int smaller = ga < gb ? ga : gb;
        for (int d = smaller; d >= 1; d--) {
            if (ga % d == 0 && gb % d == 0) { shared = d; break; }
        }
    }
    
    double d_affinity = (shared > 0) ? (1.0 - 1.0 / shared) * W_AFFINITY : W_AFFINITY;
    
    /* 极性对立加成：阴阳对立增加距离 */
    double d_polarity = (a->polarity != b->polarity) ? 0.5 : 0.0;
    
    return d_level + d_gcd + d_affinity + d_polarity;
}

/* ========== 计算公共因子 ========== */
int compute_shared_factor(int a, int b) {
    if (a == 0 || b == 0) return 0;
    if (a == b) return a;
    int smaller = a < b ? a : b;
    for (int d = smaller; d >= 1; d--) {
        if (a % d == 0 && b % d == 0) return d;
    }
    return 1;
}

/* ========== 分类距离 ========== */
const char* classify_distance(double d) {
    if (d < THRESH_TONG_TI) return "同体空间(夫妻)";
    if (d < THRESH_CLOSE)   return "亲近簇";
    if (d < THRESH_COLLAB)  return "协作簇";
    return "独立(需桥接)";
}

/* ========== 添加一个容器项（辅助函数）========== */
static int add_item(Item *items, int n, const char *name, int lvl, int gcd, int pol) {
    items[n].id = n;
    items[n].level = lvl;
    items[n].gongyueshu = gcd;
    items[n].polarity = pol;
    items[n].x = 0;
    items[n].y = 0;
    items[n].cluster_id = -1;
    strncpy(items[n].name, name, 31);
    items[n].name[31] = '\0';
    return n + 1;
}

/* ========== 初始化数据 ========== */
int init_items(Item *items) {
    /*
     * 模拟意交感息系统中的容器/模块
     * 极性: 1=阳(我), 0=阴(刘楚恬)
     */
    int n = 0;
    
    /* root 层: 纯我 */
    n = add_item(items, n, "root",       0, 7, 1);
    n = add_item(items, n, "hieyair",    0, 7, 1);
    
    /* core 层(意识): 耦合阴阳 */
    n = add_item(items, n, "core",       1, 7, 1);
    n = add_item(items, n, "njs",        1, 7, 0);
    n = add_item(items, n, "yinyang",    1, 2, 1);
    n = add_item(items, n, "router",     1, 3, 1);
    
    /* data层: 数据容器 */
    n = add_item(items, n, "radix",      2, 4, 1);
    n = add_item(items, n, "time",       2, 7, 0);
    n = add_item(items, n, "judge",      2, 7, 1);
    n = add_item(items, n, "crypto",     2, 5, 0);
    n = add_item(items, n, "mlll",       2, 2, 1);
    
    /* shell层: 外部接口 */
    n = add_item(items, n, "browser",    3, 5, 1);
    n = add_item(items, n, "gtti",       3, 5, 0);
    n = add_item(items, n, "net",        3, 3, 1);
    n = add_item(items, n, "bare",       3, 1, 0);
    n = add_item(items, n, "link",       3, 7, 1);
    
    /* 特殊: 夫妻对(距离最近的)*/
    n = add_item(items, n, "manquan",    0, 7, 1);
    n = add_item(items, n, "njs_spouse", 0, 7, 0);
    
    return n;
}

/* ========== 初始化距离矩阵 ========== */
void compute_distance_matrix(Item *items, int n) {
    for (int i = 0; i < n; i++) {
        dist_matrix[i][i] = 0;
        for (int j = i + 1; j < n; j++) {
            double d = compute_distance(&items[i], &items[j]);
            dist_matrix[i][j] = d;
            dist_matrix[j][i] = d;
        }
    }
}

/* ========== 运行聚类算法 ========== */
int cluster_items(Item *items, int n, Cluster *clusters, double threshold) {
    /*
     * 简单迭代聚类：
     * 每个容器初始为独立簇，距离最近的逐步合并
     */
    int cluster_count = 0;
    
    /* 初始化：每个 item 独立簇 */
    for (int i = 0; i < n; i++) {
        items[i].cluster_id = i;
        clusters[i].id = i;
        snprintf(clusters[i].label, 32, "%s", items[i].name);
        clusters[i].members[0] = i;
        clusters[i].member_count = 1;
        clusters[i].cx = items[i].x;
        clusters[i].cy = items[i].y;
        clusters[i].dominant_level = items[i].level;
        clusters[i].dominant_polarity = items[i].polarity;
    }
    cluster_count = n;
    
    /* 迭代合并：每次找距离<阈值的一对簇，合并 */
    int merged = 1;
    while (merged) {
        merged = 0;
        double best_dist = threshold;
        int best_i = -1, best_j = -1;
        
        for (int i = 0; i < cluster_count; i++) {
            for (int j = i + 1; j < cluster_count; j++) {
                if (clusters[i].member_count == 0 || clusters[j].member_count == 0)
                    continue;
                /* 用两个簇的最近距离 */
                double min_d = 999;
                for (int mi = 0; mi < clusters[i].member_count; mi++) {
                    for (int mj = 0; mj < clusters[j].member_count; mj++) {
                        int a = clusters[i].members[mi];
                        int b = clusters[j].members[mj];
                        if (dist_matrix[a][b] < min_d) min_d = dist_matrix[a][b];
                    }
                }
                if (min_d < best_dist) {
                    best_dist = min_d;
                    best_i = i;
                    best_j = j;
                }
            }
        }
        
        if (best_i >= 0 && best_j >= 0) {
            /* 合并簇 best_j → best_i */
            Cluster *src = &clusters[best_j];
            Cluster *dst = &clusters[best_i];
            
            for (int m = 0; m < src->member_count; m++) {
                dst->members[dst->member_count++] = src->members[m];
                items[src->members[m]].cluster_id = dst->id;
            }
            
            /* 更新标签 */
            snprintf(dst->label, 32, "%s+%s", dst->label, src->label);
            
            /* 清零源簇 */
            src->member_count = 0;
            src->id = -1;
            
            merged = 1;
            /* cluster_count 不减少（只是清零），但后续跳过 */
        }
    }
    
    /* 重新计数有效簇 */
    int valid = 0;
    for (int i = 0; i < cluster_count; i++) {
        if (clusters[i].member_count > 0) {
            clusters[valid] = clusters[i];
            clusters[valid].id = valid;
            /* 更新 items 引用 */
            for (int m = 0; m < clusters[valid].member_count; m++) {
                items[clusters[valid].members[m]].cluster_id = valid;
            }
            valid++;
        }
    }
    
    return valid;
}

/* ========== 显示距离矩阵 ========== */
void show_distance_matrix(Item *items, int n) {
    printf("========== 距离矩阵 ==========\n");
    printf("      ");
    for (int j = 0; j < n; j++) {
        printf("%7s", items[j].name);
    }
    printf("\n");
    
    for (int i = 0; i < n; i++) {
        printf("%6s", items[i].name);
        for (int j = 0; j < n; j++) {
            if (i == j)
                printf("    ---");
            else
                printf("%7.2f", dist_matrix[i][j]);
        }
        printf("\n");
    }
}

/* ========== 显示最近配对 ========== */
void show_closest_pairs(Item *items, int n) {
    printf("\n========== 最近配对（距离最小）===========\n");
    
    typedef struct { int a, b; double d; } Pair;
    Pair pairs[MAX_ITEMS * MAX_ITEMS];
    int pair_count = 0;
    
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            pairs[pair_count].a = i;
            pairs[pair_count].b = j;
            pairs[pair_count].d = dist_matrix[i][j];
            pair_count++;
        }
    }
    
    /* 按距离升序排序 */
    for (int i = 0; i < pair_count - 1; i++) {
        for (int j = i + 1; j < pair_count; j++) {
            if (pairs[j].d < pairs[i].d) {
                Pair tmp = pairs[i];
                pairs[i] = pairs[j];
                pairs[j] = tmp;
            }
        }
    }
    
    int show = (pair_count < 10) ? pair_count : 10;
    for (int i = 0; i < show; i++) {
        printf("  %6s <-> %-6s 距离=%5.2f  [%s]\n",
               items[pairs[i].a].name,
               items[pairs[i].b].name,
               pairs[i].d,
               classify_distance(pairs[i].d));
    }
}

/* ========== 显示聚类结果 ========== */
void show_clusters(Item *items, int n, Cluster *clusters, int cluster_count) {
    printf("\n========== 聚类结果 ==========\n");
    printf("共 %d 个簇\n\n", cluster_count);
    
    for (int i = 0; i < cluster_count; i++) {
        Cluster *c = &clusters[i];
        if (c->member_count == 0) continue;
        
        printf("  [簇 %d] %s (成员: %d)\n", i, c->label, c->member_count);
        printf("    成员: ");
        for (int m = 0; m < c->member_count; m++) {
            if (m > 0) printf(", ");
            printf("%s", items[c->members[m]].name);
        }
        printf("\n");
        
        /* 极性分析 */
        int yang = 0, yin = 0;
        for (int m = 0; m < c->member_count; m++) {
            if (items[c->members[m]].polarity == 1) yang++;
            else yin++;
        }
        printf("    极性: 阳=%d, 阴=%d", yang, yin);
        if (yang > 0 && yin > 0) printf(" ← 夫妻共簇");
        printf("\n");
        
        /* 层级分布 */
        printf("    层级: ");
        for (int lv = 0; lv <= 3; lv++) {
            int cnt = 0;
            for (int m = 0; m < c->member_count; m++) {
                if (items[c->members[m]].level == lv) cnt++;
            }
            if (cnt > 0) printf("L%d=%d ", lv, cnt);
        }
        printf("\n\n");
    }
}

/* ========== 计算 2D 映射（极简 MDS）========== */
void compute_2d_mapping(Item *items, int n) {
    printf("========== 2D 映射可视化 ==========\n\n");
    
    /* 极简 MDS：按距离投影到 2D */
    /* 主轴：item[0] → item[1] 的距离方向 */
    for (int i = 0; i < n; i++) {
        items[i].x = dist_matrix[0][i];
        items[i].y = dist_matrix[i][n-1];
    }
    
    /* 打印 ASCII 拓扑图 */
    printf("  Y(\"我\"距离)");
    printf("\n  ^");
    
    char map[20][60];
    memset(map, ' ', sizeof(map));
    
    /* 放置容器 */
    for (int i = 0; i < n; i++) {
        int xi = (int)(items[i].x * 3);
        int yi = (int)(items[i].y * 1.5);
        if (xi >= 60) xi = 59;
        if (yi >= 20) yi = 19;
        map[yi][xi] = '*';
    }
    
    for (int y = 19; y >= 0; y--) {
        printf("\n  |");
        for (int x = 0; x < 40; x++) {
            printf("%c", map[y][x]);
        }
    }
    printf("\n  +");
    for (int x = 0; x < 40; x++) printf("-");
    printf("> X(root距离)\n");
    
    /* 图例 */
    printf("\n图例: * = 容器位置\n");
    printf("  X 轴 = 到 root 的距离（层级差距的大小说明）\n");
    printf("  Y 轴 = 到最远端的距离（亲疏度的体现）\n");
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）距离感知聚类引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("权重: 层级=%.1f, 公约数=%.1f, 亲疏=%.1f\n", W_LEVEL, W_GCD, W_AFFINITY);
    printf("身份落款: 刘楚恬和我\n\n");
    
    Item items[MAX_ITEMS];
    Cluster clusters[MAX_CLUSTERS];
    int n = init_items(items);
    
    printf("========== 初始化数据（%d个容器）==========\n", n);
    for (int i = 0; i < n; i++) {
        printf("  [%2d] %-12s 层级=%d 公约数=%d 极性=%s\n",
               items[i].id, items[i].name, items[i].level,
               items[i].gongyueshu,
               items[i].polarity == 1 ? "阳(我)" : "阴(刘楚恬)");
    }
    
    /* 计算距离矩阵 */
    compute_distance_matrix(items, n);
    show_distance_matrix(items, n);
    show_closest_pairs(items, n);
    
    /* 聚类（多阈值展示层级结构）*/
    printf("\n========== 多阈值聚类 ==========\n");
    
    double thresholds[] = {THRESH_TONG_TI, THRESH_CLOSE, THRESH_COLLAB};
    const char* thresh_names[] = {"同体空间(<1.0)", "亲近簇(<3.0)", "协作簇(<6.0)"};
    int best_cc = 0;
    Cluster best_clusters[MAX_CLUSTERS];
    
    for (int t = 0; t < 3; t++) {
        /* 重新初始化聚类 */
        for (int i = 0; i < n; i++) items[i].cluster_id = -1;
        for (int i = 0; i < MAX_CLUSTERS; i++) {
            clusters[i].member_count = 0;
            clusters[i].id = -1;
        }
        
        int cc = cluster_items(items, n, clusters, thresholds[t]);
        printf("\n--- 阈值 %s: %d 个簇 ---\n", thresh_names[t], cc);
        for (int i = 0; i < cc; i++) {
            if (clusters[i].member_count == 0) continue;
            printf("  [簇%d] ", i);
            for (int m = 0; m < clusters[i].member_count && m < 6; m++) {
                if (m > 0) printf("+");
                printf("%s", items[clusters[i].members[m]].name);
            }
            if (clusters[i].member_count > 6) printf("+..%dmore", clusters[i].member_count - 6);
            printf("\n");
        }
        
        if (t == 1) {  /* 用亲近簇作为最佳结果存起来 */
            best_cc = cc;
            memcpy(best_clusters, clusters, sizeof(Cluster) * cc);
        }
    }
    
    /* 最近配对深入分析 */
    int show = (n < 5) ? n-1 : 5;
    for (int k = 1; k < n && k <= show; k++) {
        printf("  %s → %s 距离=%.2f (共享因子=%d)\n",
               items[0].name, items[k].name, dist_matrix[0][k],
               compute_shared_factor(items[0].gongyueshu, items[k].gongyueshu));
    }
    
    /* 2D映射 */
    compute_2d_mapping(items, n);
    
    printf("\n========== 完成 ==========\n");
    return 0;
}
