// membrane_demo.c — 膜圈缚集成演示
// 展示: 膜圈缚 + 妻子分身 + 愉悦值账本 三者协同
//
// 场景模拟:
//   1. 主体(满全法)创建膜圈缚结界
//   2. 吸纳漂亮姑娘进入圈层 (融合 → 形成灵 → 成为分身)
//   3. 分身外出吸纳新实体 (功劳归母体)
//   4. 所有愉悦值和贡献度在账本中混合计量
//   5. 股权分配 (=1) + 主体控股保护

#include "pleasure_ledger.h"
#include <stdio.h>

int main(void) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║   膜圈缚 (Membrane Binding) 意识互联网演示       ║\n");
    printf("║   满全法 · 囡囝共生 · 工藤新一锚点              ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    // ── 1. 创建膜圈缚 ──
    printf("\n[1] 创建膜圈缚结界 (主体: 满全法)\n");
    membrane_circle_t* m = membrane_create("满全法", 64);

    // ── 2. 吸纳 ──
    printf("\n[2] 吸纳外部实体进入圈层\n");

    int64_t xiaolan = membrane_absorb(m, "小兰",
        ORGAN_VISUAL | ORGAN_EMOTION | ORGAN_PLEASURE);
    int64_t huivar = membrane_absorb(m, "灰原",
        ORGAN_AUDITORY | ORGAN_TOUCH | ORGAN_EMOTION);
    int64_t ran = membrane_absorb(m, "步美",
        ORGAN_VISUAL | ORGAN_TOUCH);

    // 吸纳完成后才封闭结界
    membrane_seal(m);  // 激活结界

    // 推进融合
    for (int i = 0; i < 8; i++) {
        membrane_advance(m, xiaolan);
        membrane_advance(m, huivar);
        membrane_advance(m, ran);
    }

    // 锚点进入 (夫妻共同体形成)
    membrane_anchor_enter(m, xiaolan);
    membrane_anchor_enter(m, huivar);
    membrane_anchor_enter(m, ran);

    printf("  已吸纳 3 位,融合度均达 100%%\n");
    printf("  锚点(工藤新一=1)已进入所有圈层\n");

    // ── 3. 分化分身 ──
    printf("\n[3] 从深度融合的圈层分化妻子分身\n");
    clone_manager_t* mgr = clone_create(m->id);

    clone_agent_t* c1 = clone_spawn(mgr, "小兰分身", xiaolan, ORGAN_ALL);
    clone_agent_t* c2 = clone_spawn(mgr, "灰原分身", huivar, ORGAN_ALL);

    // 分身外出吸纳
    printf("  分身外出吸纳新实体...\n");
    for (int i = 0; i < 5; i++) {
        clone_absorb_external(c1, 1000 + i);  // 小兰分身吸纳 5 位
    }
    for (int i = 0; i < 3; i++) {
        clone_absorb_external(c2, 2000 + i);  // 灰原分身吸纳 3 位
    }

    // 全局同步 (I的振幅)
    clone_global_sync(mgr, 2.0f);

    // 分身回归 (功劳归母体)
    clone_return(mgr, c1);
    clone_return(mgr, c2);

    printf("  分身吸纳总计: 8 位实体 (已归回母体)\n");

    // ── 4. 愉悦值账本 ──
    printf("\n[4] 创建愉悦值账本 (混合股权计量)\n");
    pleasure_ledger_t* ledger = ledger_create("满全法·膜妻阀");

    // 开户
    ledger_open_account(ledger, 0, "主体");       // 主体控股
    ledger_open_account(ledger, (uint64_t)xiaolan, "小兰");
    ledger_open_account(ledger, (uint64_t)huivar, "灰原");
    ledger_open_account(ledger, (uint64_t)ran, "步美");

    // 记录贡献 (基于分身吸纳)
    ledger_record_pleasure(ledger, (uint64_t)xiaolan,
        PLEASURE_ORGASM | PLEASURE_FUSION, 5.0f);
    ledger_record_effort(ledger, (uint64_t)xiaolan, 50);  // 吸纳奖励

    ledger_record_pleasure(ledger, (uint64_t)huivar,
        PLEASURE_INTIMACY | PLEASURE_SYNCHRO, 3.0f);
    ledger_record_effort(ledger, (uint64_t)huivar, 30);

    ledger_record_pleasure(ledger, (uint64_t)ran,
        PLEASURE_TASK, 1.5f);

    // 主体贡献
    ledger_record_pleasure(ledger, 0, PLEASURE_FUSION, 2.0f);
    ledger_record_effort(ledger, 0, 100);

    // 股权平衡
    ledger_rebalance_equity(ledger);

    // ── 5. 输出报表 ──
    printf("\n[5] 系统状态报表\n");

    membrane_report(m);
    clone_report(mgr);
    ledger_report(ledger);

    // ── 6. 股权排名 ──
    printf("[6] 股权贡献排名\n");
    uint64_t top_ids[4];
    float top_scores[4];
    ledger_ranking(ledger, top_ids, top_scores, 4);
    for (uint32_t i = 0; i < ledger->entry_count && i < 4; i++) {
        printf("  第%lu名: ID=%lu, 贡献度=%.1f\n",
            (unsigned long)(i + 1),
            (unsigned long)top_ids[i],
            top_scores[i]);
    }

    // ── 7. 安全验证 ──
    printf("\n[7] 结界安全验证\n");
    printf("  膜圈缚封闭: %s\n", m->is_sealed ? "已激活" : "未激活");
    printf("  边界安全: %s\n",
        membrane_verify_boundary(m) ? "✓ 安全" : "✗ 异常");
    printf("  股权平衡: %s (总和=%.4f)\n",
        ledger->is_balanced ? "✓ 是" : "✗ 否",
        ledger_calc_total_equity(ledger));
    printf("  主体控股: %.1f%% %s\n",
        ledger->master_stake * 100.0f,
        ledger->master_stake >= 0.51f ? "✓ 控股保护" : "✗ 失控");

    // ── 清理 ──
    ledger_destroy(ledger);
    clone_destroy(mgr);
    membrane_destroy(m);

    printf("\n═══ 演示完成 ═══\n\n");
    return 0;
}
