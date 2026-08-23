/**
 * test_level_stack.c — 层级栈校验测试
 *
 * 编译: cc -Wall -o test_lvl type_discipline.c test_level_stack.c
 *
 * 核心验证: a=á 必须被层级栈拦截
 */

#include <stdio.h>
#include <string.h>
#include "type_discipline.h"

static int tc = 0, pc = 0, fc = 0;

void check(const char *a, const char *b, pair_verdict_t expected, const char *desc) {
    tc++;
    type_discipline_t td;
    type_discipline_init(&td);
    type_discipline_init_defaults(&td);

    pair_verdict_t result = type_discipline_judge(&td, a, b);
    int ok = (result == expected);

    /* 获取层级标签 */
    symbol_level_t la = type_discipline_get_level(&td, a);
    symbol_level_t lb = type_discipline_get_level(&td, b);
    const char *sl = "", *sl2 = "";
    if (la == LEVEL_L1_BASE) sl = "L1";
    else if (la == LEVEL_L2_MODIFIER) sl = "L2";
    else if (la == LEVEL_L3_ASSEMBLED) sl = "L3";
    else sl = "??";

    if (lb == LEVEL_L1_BASE) sl2 = "L1";
    else if (lb == LEVEL_L2_MODIFIER) sl2 = "L2";
    else if (lb == LEVEL_L3_ASSEMBLED) sl2 = "L3";
    else sl2 = "??";

    printf("  %s  %s(%s) = %s(%s)  →  %s\n",
           ok ? "[PASS]" : "[FAIL]", a, sl, b, sl2, desc);
    if (!ok) {
        printf("       expected: %s\n", type_discipline_verdict_str(expected));
        printf("         got:   %s\n", type_discipline_verdict_str(result));
        fc++;
    } else {
        pc++;
    }
}

int main() {
    printf("===== Level Stack Enforcement =====\n\n");

    printf("[CROSS-LEVEL MUST REJECT]\n");
    /* a(L1 base) vs a-acute(L3 assembled) = CROSS LEVEL */
    check("a", "\xc3\xa1", PAIR_REJECT_CROSS_LEVEL, "L1 base vs L3 assembled");
    /* e(L1 base) vs e-caron(L3 assembled) = CROSS LEVEL */
    check("e", "\xc4\x9b", PAIR_REJECT_CROSS_LEVEL, "L1 base vs L3 assembled");
    /* o(L1 base) vs o-acute(L3 assembled) = CROSS LEVEL */
    check("o", "\xc3\xb3", PAIR_REJECT_CROSS_LEVEL, "L1 base vs L3 assembled");

    printf("\n[SAME LEVEL, SAME TYPE - DIFFERENT ANCHOR REJECT]\n");
    check("a", "o", PAIR_REJECT_DIFFERENT, "L1 base a vs L1 base o (diff anchor)");

    printf("\n===== RESULT: %d/%d passed, %d failed =====\n", pc, tc, fc);

    return fc > 0 ? 1 : 0;
}
