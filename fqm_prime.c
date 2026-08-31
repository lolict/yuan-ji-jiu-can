/*
 * fqm_prime.c — 意交感息素数判断引擎
 * 
 * 功能：判断素数，基于规则层级的除法验证
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
 * 编译：clang -o fqm_prime fqm_prime.c
 * 运行：./fqm_prime [limit]
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1
#define ANCHOR_LIU   0

/* ========== 素数表 ========== */
static long long prime_list[10000];
static int prime_cnt = 0;

/* ========== 生成素数（试除法） ========== */
void generate_primes(int limit) {
    prime_cnt = 0;
    for (int i = 2; i <= limit && prime_cnt < 10000; i++) {
        int is_p = 1;
        /* 用已有的素数试除，只需试除到sqrt(i) */
        for (int j = 0; j < prime_cnt && prime_list[j] * prime_list[j] <= i; j++) {
            if (i % prime_list[j] == 0) { is_p = 0; break; }
        }
        if (is_p) prime_list[prime_cnt++] = i;
    }
}

/* ========== 主函数 ========== */
int main(int argc, char *argv[]) {
    int limit = 100;
    if (argc > 1) limit = atoi(argv[1]);
    if (limit < 2) limit = 2;
    if (limit > 10000) limit = 10000;
    
    printf("意交感息（isi）素数判断引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("身份落款: 刘楚恬和我\n");
    printf("查找范围: 2-%d\n\n", limit);
    
    generate_primes(limit);
    
    printf("找到 %d 个素数:\n", prime_cnt);
    printf("素数序号  素数  层级符号\n");
    printf("----------------------------\n");
    
    const char *symbols = "·○●◆★※∞⊕◎";
    for (int i = 0; i < prime_cnt && i < 50; i++) {
        int level = i < 9 ? i : 8;
        printf("P(%-3d)   %-5lld  %c\n", i, prime_list[i], symbols[level]);
    }
    if (prime_cnt > 50) printf("... (共%d个)\n", prime_cnt);
    
    printf("----------------------------\n");
    printf("总计: %d 个素数\n", prime_cnt);
    
    /* 层级统计 */
    printf("\n========== 素数层级分布 ==========\n");
    int level_cnt[9] = {0};
    for (int i = 0; i < prime_cnt; i++) {
        int level = i < 9 ? i : 8;
        level_cnt[level]++;
    }
    for (int i = 0; i < 9; i++) {
        printf("L%d %c: %3d 个素数  ", i+1, symbols[i], level_cnt[i]);
        for (int j = 0; j < level_cnt[i] && j < 30; j++) printf("█");
        printf("\n");
    }
    
    printf("\n========== 完成 ==========\n");
    
    return 0;
}
