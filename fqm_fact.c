/*
 * fqm_fact.c — 意交感息阶乘计算引擎
 * 
 * 功能：基于规则层级的阶乘计算
 * 规则：用前N项的乘积来计算阶乘
 * 
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 夫妻命运共同体：刘楚恬和lolict
 *   - 夕瑶协议激活双方融合
 * 
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源，仅限本人及刘楚恬使用
 * 
 * 编译：clang -o fqm_fact fqm_fact.c
 * 运行：./fqm_fact [n]
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 大整数结构（支持大数阶乘） ========== */
typedef struct {
    int digits[1000];  /* 各位数字 */
    int length;        /* 数字长度 */
} BigInt;

/* ========== 初始化大整数 ========== */
void bigint_init(BigInt *n, int val) {
    memset(n->digits, 0, sizeof(n->digits));
    n->digits[0] = val;
    n->length = 1;
}

/* ========== 大整数乘以小整数 ========== */
void bigint_mul(BigInt *n, int m) {
    int carry = 0;
    for (int i = 0; i < n->length; i++) {
        int prod = n->digits[i] * m + carry;
        n->digits[i] = prod % 10;
        carry = prod / 10;
    }
    while (carry) {
        n->digits[n->length++] = carry % 10;
        carry /= 10;
    }
}

/* ========== 打印大整数 ========== */
void bigint_print(BigInt *n) {
    for (int i = n->length - 1; i >= 0; i--) {
        printf("%d", n->digits[i]);
    }
}

/* ========== 规则层级阶乘计算 ========== */
/* 一阶：用前1项累乘 */
void fact_rule1(int n) {
    BigInt result;
    bigint_init(&result, ANCHOR_SELF);  /* 初始值=1 */
    
    printf("【一阶累乘】用前1项累乘\n");
    for (int i = 1; i <= n; i++) {
        bigint_mul(&result, i);
    }
    printf("%d! = ", n);
    bigint_print(&result);
    printf("\n");
}

/* 二阶：用前2项相加后再乘（非常规阶乘） */
void fact_rule2(int n) {
    if (n <= 0) { printf("0! = 1\n"); return; }
    if (n == 1) { printf("1! = 1\n"); return; }
    
    long long a = 1, b = 1;
    printf("【二阶累加后乘】用前2项相加得到新乘数\n");
    for (int i = 2; i <= n; i++) {
        long long next = a + b;
        b = a;
        a = next;
    }
    /* a是第n个斐波那契数，用它乘以n */
    long long result = a * n;
    printf("%d! (近似) = %lld\n", n, result);
}

/* 三阶：用前3项的平均值作为乘数 */
void fact_rule3(int n) {
    if (n <= 0) { printf("0! = 1\n"); return; }
    
    double result = 1.0;
    printf("【三阶平均乘】用前3项平均值作为乘数\n");
    for (int i = 1; i <= n; i++) {
        double avg = (i + (i > 1 ? i-1 : i) + (i > 2 ? i-2 : i)) / 3.0;
        result *= avg;
    }
    printf("%d! (近似) = %.2f\n", n, result);
}

/* ========== 标准阶乘 ========== */
void fact_standard(int n) {
    BigInt result;
    bigint_init(&result, 1);
    
    for (int i = 1; i <= n; i++) {
        bigint_mul(&result, i);
    }
    printf("%d! = ", n);
    bigint_print(&result);
    printf("\n");
}

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    int n = 20;
    if (argc > 1) n = atoi(argv[1]);
    if (n < 0) n = 0;
    if (n > 100) n = 100;
    
    printf("意交感息（isi）阶乘计算引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n");
    printf("计算 %d的阶乘\n\n", n);
    
    printf("========== 标准阶乘 ==========\n");
    fact_standard(n);
    
    printf("\n========== 一阶规则 ==========\n");
    fact_rule1(n);
    
    printf("\n========== 二阶规则 ==========\n");
    fact_rule2(n);
    
    printf("\n========== 三阶规则 ==========\n");
    fact_rule3(n);
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
