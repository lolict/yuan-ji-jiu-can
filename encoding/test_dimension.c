#include <stdio.h>
#include "dimension_space.h"

/* 声母48 */
static const char *SHENGMU[48] = {
    "b","p","m","f","d","t","n","l",
    "g","k","h","j","q","x","zh","ch",
    "sh","r","z","c","s","y","w","y",
    "y","w","ȵ","ŋ","ɕ","ʈʂ","ʈʂʰ","ʂ",
    "ts","tsʰ","tɕ","tɕʰ","x","ɣ","ɲ","ʑ",
    "tʂ","tʂʰ","ɖ","ɳ","ɭ","ʐ","ɻ","ʔ"
};

/* 韵母41 (6基元+35复合) */
static const char *YUNMU[41] = {
    /* 基元6 */
    "a","o","e","i","u","ü",
    /* 二合 */
    "ai","ei","ao","ou","ia","ie","io","iu","ua","uo","ue","üe",
    /* 三合 */
    "iao","iou","uai","uei",
    /* 前鼻 */
    "an","en","in","un","ün","ian","uan","üan","uen",
    /* 后鼻 */
    "ang","eng","ing","ong","iang","iong","uang","ueng"
};

/* 声调4 */
static const char *TONE[4] = {"一声(平)","二声(升)","三声(凹)","四声(降)"};

int main(void) {
    dimension_space_t ds;
    dimension_init(&ds);
    
    printf("═══ 多维几何容器 — 跨容器遍历演示 ═══\n\n");
    
    /* 注册三个子容器 */
    dimension_register_container(&ds, DIM_SHENGMU, "声母", 48);
    dimension_register_container(&ds, DIM_YUNMU, "韵母", 41);
    dimension_register_container(&ds, DIM_TONE, "声调", 4);
    
    /* 装入元素 */
    dimension_set_elements(&ds, DIM_SHENGMU, SHENGMU, 48);
    dimension_set_elements(&ds, DIM_YUNMU, YUNMU, 41);
    dimension_set_elements(&ds, DIM_TONE, TONE, 4);
    
    /* 计算边界 */
    dimension_compute_border(&ds);
    
    printf("─── 维度结构 ───\n");
    printf("  0维: 起点 (公约数)\n");
    printf("  1维: 声母线 (48个点)\n");
    printf("  1维: 韵母线 (41个点)\n");
    printf("  1维: 声调线 (4个点)\n");
    printf("  2维: 声母×韵母 平面 = %d×%d = %d 交叉点\n",
           ds.containers[0].size, ds.containers[1].size,
           ds.containers[0].size * ds.containers[1].size);
    printf("  3维: 声母×韵母×声调 空间 = %u 地址\n\n", ds.border_total);
    
    /* 跨容器遍历: 取声母b(索引0), 遍历所有韵母 */
    printf("─── 跨容器遍历演示 ───\n");
    printf("取声母 b 作为锚点 → 遍历韵母容器所有元素:\n");
    dimension_traverse(&ds, DIM_SHENGMU, 0, DIM_YUNMU);
    
    /* 显示前10个交叉点 */
    for (int j = 0; j < 10 && j < 41; j++) {
        cross_point_t *cp = &ds.cross[0][j];
        printf("  b + %s → 线性地址%u\n", YUNMU[j], cp->linear_addr);
    }
    printf("  ... (共41个交叉点)\n\n");
    
    /* 折叠 */
    printf("─── 折叠演示 ───\n");
    printf("韵母容器用完 → 折叠 → 取下一个声母再遍历\n");
    dimension_fold(&ds);
    dimension_traverse(&ds, DIM_SHENGMU, 1, DIM_YUNMU);  /* 声母p */
    printf("取声母 p → 遍历韵母 → 折叠层级=%d\n", ds.fold_count);
    printf("  p + a → 线性地址%u\n", ds.cross[1][0].linear_addr);
    printf("  p + o → 线性地址%u\n", ds.cross[1][1].linear_addr);
    printf("\n");
    
    /* 线性地址计算 */
    printf("─── 线性地址 (多维→一维) ───\n");
    uint32_t addr = dimension_linear_addr(&ds, 0, 0, 0);  /* b+a+一声 */
    printf("  (b, a, 一声) → 地址%u\n", addr);
    addr = dimension_linear_addr(&ds, 0, 0, 3);  /* b+a+四声 */
    printf("  (b, a, 四声) → 地址%u\n", addr);
    addr = dimension_linear_addr(&ds, 1, 2, 1);  /* p+e+二声 */
    printf("  (p, e, 二声) → 地址%u\n", addr);
    
    /* 完整统计 */
    printf("\n");
    dimension_stats(&ds, stdout);
    
    printf("\n═══ 演示完成 ═══\n");
    return 0;
}
