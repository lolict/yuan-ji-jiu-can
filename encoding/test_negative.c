#include <stdio.h>
#include "negative_sample.h"

int main(void) {
    negative_sample_t ns;
    negative_init(&ns);
    negative_set_core(&ns, 100, 10);
    
    printf("═══ 负样本容器 — 免疫记忆库演示 ═══\n\n");
    
    /* 模拟审判过程 */
    printf("─── 模拟审判流 ───\n");
    
    /* 第一轮: 新组合进来 */
    uint32_t pairs[][2] = {
        {0x0061, 0x00E1},   /* a vs á → 跨层级 */
        {0x0061, 0x0041},   /* a vs A → 通过 */
        {0x0061, 0x006F},   /* a vs o → 跨锚点 */
        {0x0069, 0x0061},   /* i vs a → 需审判 */
        {0x0061, 0x00E1},   /* a vs á → 第二次出现 */
    };
    const char *pair_names[] = {"a vs á", "a vs A", "a vs o", "i vs a", "a vs á(重复)"};
    
    for (int i = 0; i < 5; i++) {
        reject_reason_t reason;
        verdict_t v = negative_judge(&ns, pairs[i][0], pairs[i][1], &reason);
        
        const char *verdict_names[] = {"通过", "已知负样本", "新拒绝", "申诉"};
        printf("  %s → %s", pair_names[i], verdict_names[v]);
        
        if (v == VERDICT_KNOWN_NEG) {
            printf(" (原因: %s, 第%d次)\n", 
                   reason == REASON_CROSS_LEVEL ? "跨层级" : "其他",
                   i == 4 ? 2 : 1);  /* 第二次出现 */
        } else if (v == VERDICT_NEW_REJECT) {
            /* 模拟外部审判结果 */
            reject_reason_t reject_reason = (i == 0 || i == 2) ? REASON_CROSS_LEVEL : REASON_CROSS_ANCHOR;
            negative_writeback(&ns, pairs[i][0], pairs[i][1], reject_reason);
            printf(" → 审判后回写记忆库 (%s)\n",
                   reject_reason == REASON_CROSS_LEVEL ? "跨层级" : "跨锚点");
        } else {
            printf("\n");
        }
    }
    
    /* 判定树演示 */
    printf("\n─── 判定树: vA + vB == 2·v_core ───\n");
    printf("  v_core = 100, tolerance = 10\n");
    
    struct { int32_t a, b; } law_tests[] = {
        {100, 100},  /* 200 == 200 → 通过 */
        {95, 105},   /* 200 == 200 → 通过 */
        {80, 120},   /* 200 == 200 → 通过 */
        {50, 50},    /* 100 != 200 → 拒绝 */
        {150, 150},  /* 300 != 200 → 拒绝 */
    };
    
    for (int i = 0; i < 5; i++) {
        int pass = negative_law_judge(&ns, law_tests[i].a, law_tests[i].b);
        printf("  %d + %d = %d → %s\n",
               law_tests[i].a, law_tests[i].b,
               law_tests[i].a + law_tests[i].b,
               pass ? "通过" : "负样本");
    }
    
    /* 完整统计 */
    printf("\n");
    negative_stats(&ns, stdout);
    
    printf("\n═══ 演示完成 ═══\n");
    return 0;
}
