/**
 * property_judge.c — 产权流侵权自动判定树实现
 *
 * 法条化结构 (从粗到细):
 *
 *   第0条: 平衡保护
 *     vA + vB == 2·v_core → 无侵权, 产权容器完整
 *
 *   第1条: A方虚报侵权
 *     vA > v_core 且 vB ≤ v_core → A方超过核心声索, 侵吞B方空间
 *
 *   第2条: B方虚报侵权
 *     vB > v_core 且 vA ≤ v_core → B方超过核心声索, 侵吞A方空间
 *
 *   第3条: 双方虚报 (共谋)
 *     vA > v_core 且 vB > v_core → 双方串通虚报, 共谋侵权
 *
 *   第4条: A方被侵吞
 *     vA < v_core 且 vB ≥ v_core → A方声索不足, 被B方侵吞
 *
 *   第5条: B方被侵吞
 *     vB < v_core 且 vA ≥ v_core → B方声索不足, 被A方侵吞
 *
 *   第6条: 所有权混淆
 *     vA ≈ vB ≈ 0 且 v_core > 0 → 双方都放弃声索, 但核心有价值
 *     → 产权归属不清, 需重新裁定
 *
 *   第7条: 编码篡改
 *     任一值为负数/非法 → 容器编码被篡改, 最高级侵权
 *
 *   第8条: 绕过裁定
 *     一方绕过满全法裁判擅自行动 → 程序性侵权
 */

#include "property_judge.h"
#include <stdio.h>
#include <string.h>

/* ── 核心判定 ── */
property_verdict_t property_judge(uint32_t v_core, uint32_t v_a, uint32_t v_b) {
    return property_judge_tolerant(v_core, v_a, v_b, 0);
}

/* ── 带容差的判定 ── */
property_verdict_t property_judge_tolerant(uint32_t v_core, uint32_t v_a,
                                            uint32_t v_b, uint8_t tolerance_pct) {
    property_verdict_t result;
    memset(&result, 0, sizeof(result));
    
    result.v_core = v_core;
    result.v_a = v_a;
    result.v_b = v_b;
    
    /* 计算 delta = (vA + vB) - 2·v_core */
    int64_t sum = (int64_t)v_a + (int64_t)v_b;
    int64_t expected = 2LL * (int64_t)v_core;
    int64_t delta = sum - expected;
    result.delta = (int32_t)delta;
    
    /* 容差范围 */
    int64_t tolerance = (expected * (int64_t)tolerance_pct) / 100;
    if (tolerance == 0 && tolerance_pct > 0) tolerance = 1;
    
    /* ── 第一层: 哪一方偏离? (先于总量判断) ── */
    int32_t dev_a = (int32_t)v_a - (int32_t)v_core;  /* A方偏离 */
    int32_t dev_b = (int32_t)v_b - (int32_t)v_core;  /* B方偏离 */
    
    /* ── 第二层: 总量是否平衡? ── */
    /* 注意: 总量平衡 + 双方都不偏离 = 真平衡 */
    /* 总量平衡 + 一方偏离 = 另一方被侵吞 (总量对但个体不对) */
    int total_balanced = (delta >= -(int64_t)tolerance && delta <= (int64_t)tolerance);
    int a_balanced = (dev_a >= -(int32_t)tolerance && dev_a <= (int32_t)tolerance);
    int b_balanced = (dev_b >= -(int32_t)tolerance && dev_b <= (int32_t)tolerance);
    
    if (total_balanced && a_balanced && b_balanced) {
        /* 真平衡: 总量对 + 双方都对 */
        result.balanced = 1;
        result.infringement = INFRING_NONE;
        result.clause = CLAUSE_BALANCED;
        result.verdict_text = "平衡态 — vA=vB=v_core, 无侵权";
        return result;
    }
    
    /* 总量平衡但个体不平衡 → 一方侵吞另一方 */
    if (total_balanced && !a_balanced && b_balanced) {
        /* A偏离, B正常 → A侵吞B 或 A被侵吞 */
        if (dev_a > 0) {
            result.infringement = INFRING_OVERCLAIM;
            result.clause = CLAUSE_A_OVERCLAIM;
            result.verdict_text = "A方虚报侵权 — 总量平衡但A超过核心, 侵吞B的空间";
        } else {
            result.infringement = INFRING_UNDERCLAIM;
            result.clause = CLAUSE_A_UNDERCLAIM;
            result.verdict_text = "A方被侵吞 — 总量平衡但A低于核心, 价值被B侵吞";
        }
        result.balanced = 0;
        return result;
    }
    
    if (total_balanced && a_balanced && !b_balanced) {
        if (dev_b > 0) {
            result.infringement = INFRING_OVERCLAIM;
            result.clause = CLAUSE_B_OVERCLAIM;
            result.verdict_text = "B方虚报侵权 — 总量平衡但B超过核心, 侵吞A的空间";
        } else {
            result.infringement = INFRING_UNDERCLAIM;
            result.clause = CLAUSE_B_UNDERCLAIM;
            result.verdict_text = "B方被侵吞 — 总量平衡但B低于核心, 价值被A侵吞";
        }
        result.balanced = 0;
        return result;
    }
    
    result.balanced = 0;
    
    /* 计算偏离度 (百分比) */
    if (v_core > 0) {
        result.deviation_a = (uint8_t)(dev_a > 0 ? dev_a : -dev_a) * 100 / v_core;
        result.deviation_b = (uint8_t)(dev_b > 0 ? dev_b : -dev_b) * 100 / v_core;
    }
    
    /* ── 第三层+第四层: 判定侵权类型和法条 ── */
    
    /* 情况1: A虚报, B不超 */
    if (dev_a > 0 && dev_b <= 0) {
        result.infringement = INFRING_OVERCLAIM;
        result.clause = CLAUSE_A_OVERCLAIM;
        result.verdict_text = "A方虚报侵权 — A声索超过核心值, 侵吞B方空间";
        return result;
    }
    
    /* 情况2: B虚报, A不超 */
    if (dev_b > 0 && dev_a <= 0) {
        result.infringement = INFRING_OVERCLAIM;
        result.clause = CLAUSE_B_OVERCLAIM;
        result.verdict_text = "B方虚报侵权 — B声索超过核心值, 侵吞A方空间";
        return result;
    }
    
    /* 情况3: 双方都虚报 */
    if (dev_a > 0 && dev_b > 0) {
        result.infringement = INFRING_MUTUAL;
        result.clause = CLAUSE_MUTUAL_OVER;
        result.verdict_text = "双方虚报(共谋) — A和B串通超过核心值";
        return result;
    }
    
    /* 情况4: A被侵吞 (A低于核心) */
    if (dev_a < 0 && dev_b >= 0) {
        result.infringement = INFRING_UNDERCLAIM;
        result.clause = CLAUSE_A_UNDERCLAIM;
        result.verdict_text = "A方被侵吞 — A声索低于核心值, 价值被B方侵吞";
        return result;
    }
    
    /* 情况5: B被侵吞 */
    if (dev_b < 0 && dev_a >= 0) {
        result.infringement = INFRING_UNDERCLAIM;
        result.clause = CLAUSE_B_UNDERCLAIM;
        result.verdict_text = "B方被侵吞 — B声索低于核心值, 价值被A方侵吞";
        return result;
    }
    
    /* 情况6: 双方都低于核心 (混淆) */
    if (dev_a < 0 && dev_b < 0) {
        result.infringement = INFRING_CONFUSION;
        result.clause = CLAUSE_CONFUSION;
        result.verdict_text = "所有权混淆 — 双方都低于核心, 产权不清";
        return result;
    }
    
    /* 兜底 */
    result.infringement = INFRING_CONFUSION;
    result.clause = CLAUSE_CONFUSION;
    result.verdict_text = "未分类状态 — 需人工复核";
    return result;
}

/* ── 裁定报告 ── */
void property_verdict_report(const property_verdict_t *v, char *buf, int bufsize) {
    const char *deviation_level[] = {"无", "轻微", "中度", "严重"};
    
    const char *dev_a_str = (v->deviation_a >= 20) ? "严重" :
                            (v->deviation_a >= 5) ? "中度" :
                            (v->deviation_a > 0) ? "轻微" : "无";
    const char *dev_b_str = (v->deviation_b >= 20) ? "严重" :
                            (v->deviation_b >= 5) ? "中度" :
                            (v->deviation_b > 0) ? "轻微" : "无";
    
    snprintf(buf, bufsize,
        "v_core=%u vA=%u vB=%u delta=%d %s\n"
        "偏离: A=%s B=%s\n"
        "法条: 第%d条\n"
        "裁定: %s",
        v->v_core, v->v_a, v->v_b, v->delta,
        v->balanced ? "✓平衡" : "✗失衡",
        dev_a_str, dev_b_str,
        v->clause,
        v->verdict_text);
}

/* ── 批量判定演示 ── */
void property_judge_demo(void) {
    printf("═══════════════════════════════════════════════════════\n");
    printf("  产权流侵权自动判定树 · 法条化演示\n");
    printf("  核心公式: vA + vB == 2·v_core\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    /* 测试用例: {v_core, vA, vB, 描述} */
    struct {
        uint32_t v_core, v_a, v_b;
        const char *desc;
    } cases[] = {
        {100, 100, 100, "完美平衡"},                    /* 第0条 */
        {100, 120,  80, "A虚报20"},                     /* 第1条 */
        {100,  70, 130, "B虚报30"},                     /* 第2条 */
        {100, 110, 110, "双方各虚报10"},                /* 第3条 */
        {100,  80, 100, "A被侵吞20"},                   /* 第4条 */
        {100, 100,  60, "B被侵吞40"},                   /* 第5条 */
        {100,   0,   0, "双方弃权(混淆)"},              /* 第6条 */
        {100,  50,  50, "双方都低于核心"},              /* 第6条 */
        {100, 150, 150, "双方严重虚报"},                /* 第3条 */
    };
    
    int n_cases = (int)(sizeof(cases) / sizeof(cases[0]));
    
    for (int i = 0; i < n_cases; i++) {
        property_verdict_t v = property_judge(cases[i].v_core, cases[i].v_a, cases[i].v_b);
        
        printf("┌─── 案例 %d: %s ───┐\n", i + 1, cases[i].desc);
        printf("│ v_core=%u  vA=%u  vB=%u\n", v.v_core, v.v_a, v.v_b);
        printf("│ 总和=%u  2·core=%u  delta=%d\n",
               v.v_a + v.v_b, 2 * v.v_core, v.delta);
        
        if (v.balanced) {
            printf("│ ★ 平衡 — 无侵权\n");
        } else {
            printf("│ ✗ 失衡 — 侵权类型=%d 法条=第%d条\n", v.infringement, v.clause);
        }
        printf("│ %s\n", v.verdict_text);
        printf("└──────────────────────────────┘\n\n");
    }
    
    /* ── 容差演示 ── */
    printf("═══════════════════════════════════════════════════════\n");
    printf("  容差判定演示 (tolerance=5%%)\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    /* v_core=100, vA=103, vB=98 → delta=1 (在5%容差内) */
    {
        property_verdict_t v = property_judge_tolerant(100, 103, 98, 5);
        printf("v_core=100  vA=103  vB=98 → delta=%d %s\n",
               v.delta, v.balanced ? "✓平衡(容差内)" : "✗失衡");
    }
    {
        property_verdict_t v = property_judge_tolerant(100, 103, 98, 0);
        printf("v_core=100  vA=103  vB=98 → delta=%d %s (零容差)\n",
               v.delta, v.balanced ? "✓平衡" : "✗失衡");
    }
    
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  判定树完整法条:\n");
    printf("  第0条: 平衡保护 (vA+vB == 2·v_core)\n");
    printf("  第1条: A虚报 (vA>v_core, vB≤v_core)\n");
    printf("  第2条: B虚报 (vB>v_core, vA≤v_core)\n");
    printf("  第3条: 双方虚报 (vA>v_core, vB>v_core)\n");
    printf("  第4条: A被侵吞 (vA<v_core)\n");
    printf("  第5条: B被侵吞 (vB<v_core)\n");
    printf("  第6条: 所有权混淆 (双方都低于核心)\n");
    printf("  第7条: 编码篡改 (非法值)\n");
    printf("  第8条: 绕过裁定 (程序性侵权)\n");
    printf("═══════════════════════════════════════════════════════\n");
}
