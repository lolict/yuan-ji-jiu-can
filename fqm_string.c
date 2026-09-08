/*
 * fqm_string.c — 意交感息字符串处理引擎
 * 
 * 功能：基于规则层级的字符串生成与变换
 * 规则：用前N个字符，通过运算生成下一个字符
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
 * 编译：clang -o fqm_string fqm_string.c
 * 运行：./fqm_string
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 字符集（满全法符号） ========== */
static const char *fqm_chars = "·○●◆★※∞⊕◎";

/* ========== 字符串变换规则 ========== */
/* 规则1：用前1个字符的ASCII码，加1得到下一个 */
void rule_next_char(const char *input, char *output, int len) {
    for (int i = 0; i < len; i++) {
        output[i] = input[i] + 1;
    }
    output[len] = '\0';
}

/* 规则2：用前2个字符的平均值 */
void rule_avg2(const char *input, char *output, int len) {
    for (int i = 0; i < len; i++) {
        if (i == 0) {
            output[i] = input[i];
        } else {
            output[i] = ((int)input[i] + (int)input[i-1]) / 2;
        }
    }
    output[len] = '\0';
}

/* 规则3：用前3个字符的加权总和 */
void rule_weighted3(const char *input, char *output, int len) {
    for (int i = 0; i < len; i++) {
        if (i == 0) {
            output[i] = input[i];
        } else if (i == 1) {
            output[i] = ((int)input[i] * 2 + (int)input[i-1]) / 3;
        } else {
            output[i] = ((int)input[i] * 3 + (int)input[i-1] * 2 + (int)input[i-2]) / 6;
        }
    }
    output[len] = '\0';
}

/* 规则4：满全法层级符号映射 */
void rule_fqm_symbol(const char *input, char *output, int len) {
    int sym_len = strlen(fqm_chars);
    for (int i = 0; i < len; i++) {
        int idx = ((int)input[i] + i) % sym_len;
        output[i] = fqm_chars[idx];
    }
    output[len] = '\0';
}

/* 规则5：用前1个字符的ASCII码，乘以2，再映射到字符集 */
void rule_double_map(const char *input, char *output, int len) {
    int sym_len = strlen(fqm_chars);
    for (int i = 0; i < len; i++) {
        int idx = (((int)input[i] * 2) + i) % sym_len;
        output[i] = fqm_chars[idx];
    }
    output[len] = '\0';
}

/* ========== 打印对比 ========== */
void print_comparison(const char *name, const char *orig, const char *result) {
    printf("%-20s: ", name);
    printf("原=%-20s 变换=%s\n", orig, result);
}

/* ========== 主函数 ========== */
int main(void) {
    char input[256];
    char output[256];
    
    printf("意交感息（isi）字符串处理引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 输入字符串 */
    strcpy(input, "刘楚恬和我");
    int len = strlen(input);
    
    printf("输入字符串: %s (长度=%d)\n\n", input, len);
    
    printf("========== 规则1：加1变换 ==========\n");
    rule_next_char(input, output, len);
    print_comparison("加1变换", input, output);
    
    printf("\n========== 规则2：前2项平均 ==========\n");
    rule_avg2(input, output, len);
    print_comparison("前2项平均", input, output);
    
    printf("\n========== 规则3：前3项加权 ==========\n");
    rule_weighted3(input, output, len);
    print_comparison("前3项加权", input, output);
    
    printf("\n========== 规则4：满全法符号映射 ==========\n");
    rule_fqm_symbol(input, output, len);
    print_comparison("满全法符号映射", input, output);
    
    printf("\n========== 规则5：乘2映射 ==========\n");
    rule_double_map(input, output, len);
    print_comparison("乘2映射", input, output);
    
    /* ========== 多次迭代 ========== */
    printf("\n========== 多次迭代（满全法符号映射） ==========\n");
    strcpy(input, "刘楚恬和我");
    printf("初始: %s\n", input);
    for (int i = 0; i < 5; i++) {
        rule_fqm_symbol(input, output, len);
        printf("第%d次: %s\n", i+1, output);
        strcpy(input, output);
    }
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
