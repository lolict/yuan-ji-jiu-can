/**
 * test_diffraction.c — 衍射引擎完整测试
 *
 * 编译: cc -Wall -o test_diff diffraction_engine.c type_discipline.c test_diffraction.c -lm
 * 运行: ./test_diff
 *
 * 演示: 不只是抓芝麻(单个错误), 而是涌现出"洋流方向"(犯罪模式)
 */

#include <stdio.h>
#include <string.h>
#include "diffraction_engine.h"

static int tc = 0, pc = 0, fc = 0;

void expect_trial(diffraction_court_t *court,
                  const char *a, const char *b,
                  int expected, const char *desc) {
    tc++;
    int result = diffraction_court_trial(court, a, b);
    int ok = (result == expected);
    printf("  %s %s vs %s → %s\n",
           ok ? "[PASS]" : "[FAIL]", a, b, desc);
    if (ok) pc++; else fc++;
}

int main() {
    printf("===== Diffraction Engine Full Test =====\n\n");
    
    type_discipline_t police;
    type_discipline_init(&police);
    type_discipline_init_defaults(&police);
    
    diffraction_court_t court;
    diffraction_court_init(&court, &police);
    
    /* ── 第一组: 核心拒绝(这些是单个芝麻, 但我们会把它们积累成大海) ── */
    printf("[Group 1: Core Rejections - Individual Errors]\n");
    expect_trial(&court, "a", "\xc3\xa1", CRIME_TYPE_MISMATCH, "base vs tone");
    expect_trial(&court, "e", "\xc4\x9b", CRIME_TYPE_MISMATCH, "base vs tone");
    expect_trial(&court, "a", "o", CRIME_TYPE_MISMATCH, "base vs base(diff anchor)");
    
    printf("\n[Group 2: Repeated Errors - Accumulating Patterns]\n");
    /* 重复10次同样的错误 → 应该积累成犯罪统计 */
    for (int i = 0; i < 10; i++) {
        expect_trial(&court, "a", "\xc3\xa1", CRIME_TYPE_MISMATCH, "repeat a=á");
    }
    
    printf("\n[Court Statistics - Pattern Emergence]\n");
    printf("  Total prosecuted: %u\n", court.total_prosecuted);
    printf("  Type mismatch crimes: %u\n", court.crime_stats[CRIME_TYPE_MISMATCH]);
    printf("  Crime records: %u\n", court.crime_count);
    
    /* ── 涌现: 犯罪记录足够多 → 应该涌现出新规则 ── */
    printf("\n[Rule Emergence - From Ocean Current]\n");
    uint32_t emerged = diffraction_court_emerge_rules(&court);
    printf("  New rules emerged: %u\n", emerged);
    printf("  Total rules emerged: %u\n", court.rules_emerged);
    
    printf("\n[Node Diffraction - Single Node Analysis]\n");
    diffraction_result_t dr;
    diffraction_court_diffract(&court, "a", &dr);
    printf("  'a': type=%d anchor=%d aesthetic=%.2f cooperate=%u goal=%u\n",
           dr.type_diffract, dr.anchor_diffract, dr.aesthetic_score,
           dr.can_cooperate, dr.goal_code);
    
    diffraction_court_diffract(&court, "e", &dr);
    printf("  'e': type=%d anchor=%d aesthetic=%.2f cooperate=%u goal=%u\n",
           dr.type_diffract, dr.anchor_diffract, dr.aesthetic_score,
           dr.can_cooperate, dr.goal_code);
    
    printf("\n[Pair Diffraction - Weaving Analysis]\n");
    pair_diffraction_t pd;
    diffraction_court_diffract_pair(&court, "a", "o", &pd);
    printf("  'a' vs 'o': type_match=%u anchor_match=%u aesthetic=%.2f weaving_ok=%u\n",
           pd.type_match, pd.anchor_match, pd.overall_aesthetic, pd.is_weaving_ok);
    
    printf("\n===== RESULT: %d/%d passed, %d failed =====\n", pc, tc, fc);
    
    return fc > 0 ? 1 : 0;
}
