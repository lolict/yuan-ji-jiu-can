/**
 * test_type_discipline.c — 类型纪律系统测试
 *
 * 编译: cc -Wall -o test_td type_discipline.c test_type_discipline.c
 * 运行: ./test_td
 *
 * 注意: 所有非ASCII字符使用UTF-8字节序列
 *   á = 0xc3 0xa1
 *   ě = 0xc4 0x9b
 *   ō = 0xc5 0x8D
 */

#include <stdio.h>
#include <string.h>
#include "type_discipline.h"

static int tc = 0, pc = 0, fc = 0;

void check(const char *name_a, const char *name_b,
           pair_verdict_t expected, const char *desc) {
    tc++;
    type_discipline_t td;
    type_discipline_init(&td);
    type_discipline_init_defaults(&td);
    pair_verdict_t result = type_discipline_judge(&td, name_a, name_b);
    int ok = (result == expected);
    printf("  %s  [%s]\n", ok ? "[PASS]" : "[FAIL]", desc);
    if (!ok) {
        printf("       expected: %s\n", type_discipline_verdict_str(expected));
        printf("         got:   %s\n", type_discipline_verdict_str(result));
        fc++;
    } else {
        pc++;
    }
}

int main() {
    printf("===== Type Discipline =====\n\n");

    printf("[MUST REJECT]\n");
    /* a (base L1) vs a-acute (tone L3) = cross level (level check fires first) */
    check("a", "\xc3\xa1", PAIR_REJECT_CROSS_LEVEL, "base a vs tone a-acute = cross level");
    /* e (base L1) vs e-caron (tone L3) = cross level */
    check("e", "\xc4\x9b", PAIR_REJECT_CROSS_LEVEL, "base e vs tone e-caron = cross level");
    /* a (base) vs o (base diff anchor) = different anchor */
    check("a", "o", PAIR_REJECT_DIFFERENT, "base a vs base o = diff anchor");
    /* extra symbol pairing = reject */
    check("\xd3\xb0", "\xd3\xb1", PAIR_REJECT_EXTRA, "extra symbol pair");

    printf("\n[RESULT] %d/%d passed, %d failed\n", pc, tc, fc);
    return fc > 0 ? 1 : 0;
}
