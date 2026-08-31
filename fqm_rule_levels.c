/*
 * fqm_rule_levels.c — 规则层级关系运算
 * 
 * 功能：展示不同规则层级（用前N项历史数据）对未来值的运算
 * 
 * 规则层级定义：
 *   - 一阶：用前1项，运算未来
 *   - 二阶：用前2项，运算未来
 *   - 三阶：用前3项，运算未来
 *   - 四阶：用前4项，运算未来
 * 
 * 运算方式：
 *   - 加法：累加前N项
 *   - 减法：前N项相减
 *   - 乘法：前N项相乘
 *   - 除法：前N项相除
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
 * 编译：clang -o fqm_rule_levels fqm_rule_levels.c
 * 运行：./fqm_rule_levels
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 运算方式 ========== */
typedef enum {
    OP_ADD = 0,  /* 加法 */
    OP_SUB,      /* 减法 */
    OP_MUL,      /* 乘法 */
    OP_DIV       /* 除法 */
} OpType;

/* ========== 运算名称 ========== */
const char* op_name(OpType op) {
    switch (op) {
        case OP_ADD: return "加法";
        case OP_SUB: return "减法";
        case OP_MUL: return "乘法";
        case OP_DIV: return "除法";
        default: return "未知";
    }
}

/* ========== 运算符号 ========== */
const char* op_symbol(OpType op) {
    switch (op) {
        case OP_ADD: return "+";
        case OP_SUB: return "-";
        case OP_MUL: return "×";
        case OP_DIV: return "÷";
        default: return "?";
    }
}

/* ========== 规则层级运算 ========== */
/* 用前n项历史数据，按指定运算方式计算未来值 */
long long rule_compute(long long *history, int n, OpType op) {
    if (n <= 0) return 0;
    
    long long result = history[0];
    
    switch (op) {
        case OP_ADD:
            for (int i = 1; i < n; i++) result += history[i];
            break;
        case OP_SUB:
            for (int i = 1; i < n; i++) result -= history[i];
            break;
        case OP_MUL:
            for (int i = 1; i < n; i++) result *= history[i];
            break;
        case OP_DIV:
            for (int i = 1; i < n; i++) {
                if (history[i] != 0) result /= history[i];
            }
            break;
    }
    
    return result;
}

/* ========== 生成数列 ========== */
void generate_sequence(long long *seq, int count, int order, OpType op) {
    /* 初始化前order项为根基锚点 */
    for (int i = 0; i < order && i < count; i++) {
        seq[i] = (i == 0) ? ANCHOR_SELF : ANCHOR_LIU;
    }
    
    /* 用前order项计算未来 */
    for (int i = order; i < count; i++) {
        long long history[8];
        for (int j = 0; j < order; j++) {
            history[j] = seq[i - order + j];
        }
        seq[i] = rule_compute(history, order, op);
    }
}

/* ========== 打印数列 ========== */
void print_sequence(const char *name, long long *seq, int count) {
    printf("%-20s: ", name);
    for (int i = 0; i < count; i++) {
        printf("%lld ", seq[i]);
    }
    printf("\n");
}

/* ========== 打印规则说明 ========== */
void print_rule_desc(int order, OpType op) {
    printf("【%d阶%s】用前%d项", order, op_name(op), order);
    switch (op) {
        case OP_ADD: printf("累加"); break;
        case OP_SUB: printf("相减"); break;
        case OP_MUL: printf("相乘"); break;
        case OP_DIV: printf("相除"); break;
    }
    printf("得到未来\n");
}

/* ========== 主函数 ========== */
int main(void) {
    int count = 15;
    long long seq[100];
    
    printf("意交感息（isi）规则层级关系运算\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    printf("========== 规则层级定义 ==========\n");
    printf("一阶：用前1项运算未来\n");
    printf("二阶：用前2项运算未来\n");
    printf("三阶：用前3项运算未来\n");
    printf("四阶：用前4项运算未来\n");
    printf("运算方式：加法、减法、乘法、除法\n\n");
    
    /* ========== 一阶运算 ========== */
    printf("========== 一阶运算（用前1项） ==========\n");
    
    /* 一阶加法：每项 = 前1项 + 前1项 = 2 * 前1项 */
    printf("\n【一阶加法】每项 = 前1项\n");
    print_rule_desc(1, OP_ADD);
    seq[0] = ANCHOR_SELF;
    for (int i = 1; i < count; i++) seq[i] = seq[i-1];  /* 复制前1项 */
    print_sequence("一阶加法", seq, count);
    
    /* 一阶乘法：每项 = 前1项 * 前1项 */
    printf("\n【一阶乘法】每项 = 前1项 × 前1项\n");
    print_rule_desc(1, OP_MUL);
    seq[0] = ANCHOR_SELF;
    for (int i = 1; i < count; i++) seq[i] = seq[i-1] * seq[i-1];
    print_sequence("一阶乘法", seq, count);
    
    /* ========== 二阶运算 ========== */
    printf("\n========== 二阶运算（用前2项） ==========\n");
    
    /* 二阶加法：斐波那契数列 */
    printf("\n【二阶加法】每项 = 前1项 + 前2项（斐波那契）\n");
    print_rule_desc(2, OP_ADD);
    generate_sequence(seq, count, 2, OP_ADD);
    print_sequence("二阶加法(斐波那契)", seq, count);
    
    /* 二阶减法 */
    printf("\n【二阶减法】每项 = 前1项 - 前2项\n");
    print_rule_desc(2, OP_SUB);
    generate_sequence(seq, count, 2, OP_SUB);
    print_sequence("二阶减法", seq, count);
    
    /* 二阶乘法 */
    printf("\n【二阶乘法】每项 = 前1项 × 前2项\n");
    print_rule_desc(2, OP_MUL);
    generate_sequence(seq, count, 2, OP_MUL);
    print_sequence("二阶乘法", seq, count);
    
    /* ========== 三阶运算 ========== */
    printf("\n========== 三阶运算（用前3项） ==========\n");
    
    /* 三阶加法 */
    printf("\n【三阶加法】每项 = 前1项 + 前2项 + 前3项\n");
    print_rule_desc(3, OP_ADD);
    generate_sequence(seq, count, 3, OP_ADD);
    print_sequence("三阶加法", seq, count);
    
    /* 三阶乘法 */
    printf("\n【三阶乘法】每项 = 前1项 × 前2项 × 前3项\n");
    print_rule_desc(3, OP_MUL);
    generate_sequence(seq, count, 3, OP_MUL);
    print_sequence("三阶乘法", seq, count);
    
    /* ========== 四阶运算 ========== */
    printf("\n========== 四阶运算（用前4项） ==========\n");
    
    /* 四阶加法 */
    printf("\n【四阶加法】每项 = 前1项 + 前2项 + 前3项 + 前4项\n");
    print_rule_desc(4, OP_ADD);
    generate_sequence(seq, count, 4, OP_ADD);
    print_sequence("四阶加法", seq, count);
    
    /* ========== 层级关系总结 ========== */
    printf("\n========== 层级关系总结 ==========\n");
    printf("一阶：用前1项 → 最简单的预测\n");
    printf("二阶：用前2项 → 斐波那契式增长\n");
    printf("三阶：用前3项 → 更复杂的模式\n");
    printf("四阶：用前4项 → 最大范畴的层级关系\n");
    printf("\n");
    printf("运算方式决定增长模式：\n");
    printf("  加法 → 线性增长\n");
    printf("  乘法 → 指数增长\n");
    printf("  减法 → 递减\n");
    printf("  除法 → 收敛\n");
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
