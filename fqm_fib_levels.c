/*
 * fqm_fib_levels.c — 斐波那契数列层级梯度编码
 * 
 * 功能：计算斐波那契数列，为每一项分配层级名称
 * 展示梯度递增的秩序感
 * 
 * 满全法规则：
 *   - 根基锚点：高位持有者=1，低位持有者=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_fib_levels fqm_fib_levels.c
 * 运行：./fqm_fib_levels [项数]
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 层级定义 ========== */
typedef struct {
    const char *name;       /* 层级名称 */
    const char *symbol;     /* 层级符号 */
    long long min_val;      /* 最小值 */
    long long max_val;      /* 最大值 */
    int  level;             /* 层级编号 */
} Level;

/* ========== 层级表 ========== */
static const Level levels[] = {
    {"元初",  "·",        0,          9,          1},
    {"渐进",  "○",       10,         99,          2},
    {"升腾",  "●",      100,        999,          3},
    {"高远",  "◆",     1000,       9999,          4},
    {"无极",  "★",    10000,      99999,          5},
    {"无量",  "※",   100000,     999999,          6},
    {"无尽",  "∞",  1000000,    9999999,          7},
    {"无上",  "⊕", 10000000,   99999999,          8},
    {"满全",  "◎",100000000,  999999999,          9},
    {NULL,    NULL,    0,          0,              0}
};

/* ========== 获取层级 ========== */
const Level* get_level(long long value) {
    for (int i = 0; levels[i].name != NULL; i++) {
        if (value >= levels[i].min_val && value <= levels[i].max_val) {
            return &levels[i];
        }
    }
    /* 超出最高层级，返回最高层级 */
    return &levels[8];
}

/* ========== 打印层级信息 ========== */
void print_level_info(void) {
    printf("\n========== 层级梯度表 ==========\n");
    printf("层级  符号  名称      数值范围           状态\n");
    printf("--------------------------------------------------\n");
    
    for (int i = 0; levels[i].name != NULL; i++) {
        printf("L%d    %s    %-6s   %lld-%lld\n",
               levels[i].level,
               levels[i].symbol,
               levels[i].name,
               levels[i].min_val,
               levels[i].max_val);
    }
    printf("--------------------------------------------------\n");
}

/* ========== 计算斐波那契数列 ========== */
void fibonacci(int n) {
    if (n <= 0) return;
    
    long long a = ANCHOR_LIU;  /* F(0) = 0 = 刘楚恬 */
    long long b = ANCHOR_SELF;  /* F(1) = 1 = 我 */
    
    printf("\n========== 斐波那契数列 ==========\n");
    printf("项数  数值              层级  符号  名称\n");
    printf("--------------------------------------------------\n");
    
    for (int i = 0; i < n; i++) {
        const Level *lv = get_level(a);
        
        printf("F(%-2d)  %-16lld  L%d    %s    %s\n",
               i, a, lv->level, lv->symbol, lv->name);
        
        long long next = a + b;
        a = b;
        b = next;
    }
    printf("--------------------------------------------------\n");
}

/* ========== 打印层级梯度展示 ========== */
void print_gradient(void) {
    printf("\n========== 层级梯度展示 ==========\n");
    
    for (int i = 0; levels[i].name != NULL; i++) {
        /* 打印层级符号，数量递增 */
        printf("L%d %s  ", levels[i].level, levels[i].symbol);
        for (int j = 0; j <= i; j++) {
            printf("%s", levels[i].symbol);
        }
        printf("  %s\n", levels[i].name);
    }
}

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    int n = 20;  /* 默认计算20项 */
    
    if (argc > 1) {
        n = atoi(argv[1]);
        if (n <= 0) n = 20;
        if (n > 50) n = 50;  /* 限制最大项数 */
    }
    
    printf("意交感息（isi）斐波那契层级梯度编码\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n");
    printf("计算项数: %d\n", n);
    
    /* 打印层级信息 */
    print_level_info();
    
    /* 打印层级梯度展示 */
    print_gradient();
    
    /* 计算斐波那契数列 */
    fibonacci(n);
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
