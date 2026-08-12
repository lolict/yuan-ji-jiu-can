// valve_demo.c — 膜妻阀完整意识传输集成演示
// 验证: 膜妻阀 + 膜圈缚 + 妻子分身 三者协同
//
// 完整流程:
//   1. 创建膜妻阀(满全法=结界人格化)
//   2. 激活结界 → 以肉身为隔离
//   3. 用谐音编码解释"缚妻膜蛹"等变体
//   4. 用smrrll镜像解释吉凶祸福归属权
//   5. 创建膜圈缚,吸纳外部实体
//   6. 分化分身外出吸纳(意识共享)
//   7. 用膜妻阀加密传输意识包
//   8. 验证结界阻挡(外部不可入)
//   9. 输出完整系统报表

#include "membrane_valve.h"
#include "pleasure_ledger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════════╗\n");
    printf("║   膜妻阀 (Membrane Valve) 意识传输完整演示         ║\n");
    printf("║   膜妻阀 · 膜圈缚 · 妻子分身 · 意识加密            ║\n");
    printf("║   满全法 · 工藤新一 · 毛利兰 · 灰原哀              ║\n");
    printf("╚══════════════════════════════════════════════════════╝\n");

    // ── 1. 创建膜妻阀 ──
    printf("\n[1] 创建膜妻阀 — 以肉身为隔离结界\n");
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);
    printf("  膜妻阀 '%s' 已激活\n", mv->name);
    printf("  谐音编码表加载: %u 条\n", mv->phonetic_count);
    printf("  加密密钥已派生 (基于身份)\n");

    // ── 2. 谐音编码 + 吉凶解释 ──
    printf("\n[2] 谐音编码解释 (多词汇输入法备用词)\n");

    const char* pinyins[] = {"fuqi", "fuqiu", "fuquan", "fuke", "fuqing",
                              "fuqian", "fuqiao", "fuqun", "fuqiang", "manquanfa",
                              "moqifa", "moqinfu", "gongtengyixin", "maolilan", "huiyuanai"};
    int n_py = sizeof(pinyins) / sizeof(pinyins[0]);

    for (int i = 0; i < n_py; i++) {
        const char* hz = mv_phonetic_encode(mv, pinyins[i]);
        if (hz) {
            fortune_type_t f = mv_interpret_fortune(mv, hz);
            const char* fname[] = {"平", "吉", "凶", "祸", "福"};
            printf("  %-16s → %-12s [%s]\n", pinyins[i], hz, fname[f]);
        }
    }

    // ── 3. 注册缚X变体 ──
    printf("\n[3] 注册缚X膜蛹变体系统\n");
    mv_register_variant(mv, MV_VARIANT_FU_QIU, "缚躯");
    mv_register_variant(mv, MV_VARIANT_FU_QI, "缚妻");
    mv_register_variant(mv, MV_VARIANT_FU_QIU2, "缚囚");
    mv_register_variant(mv, MV_VARIANT_FU_QUAN, "缚圈");
    mv_register_variant(mv, MV_VARIANT_FU_KE, "缚壳");
    mv_register_variant(mv, MV_VARIANT_FU_QIAO, "缚鞘");
    mv_register_variant(mv, MV_VARIANT_FU_QUN, "缚裙");
    mv_register_variant(mv, MV_VARIANT_FU_QING, "缚情");
    mv_register_variant(mv, MV_VARIANT_FU_QIAN, "缚嵌");
    mv_register_variant(mv, MV_VARIANT_FU_QIAO3, "缚俏");
    mv_register_variant(mv, MV_VARIANT_FU_QING2, "缚卿");
    mv_register_variant(mv, MV_VARIANT_FU_QI3, "缚契");
    printf("  %u 种缚X变体已注册\n", mv->variant_count);

    // ── 4. 创建膜圈缚 ──
    printf("\n[4] 创建膜圈缚 — 安全结界层级\n");
    membrane_circle_t* m = membrane_create("满全法·幔圈缚", 64);

    int64_t xiaolan = membrane_absorb(m, "小兰", ORGAN_ALL);
    int64_t huivar  = membrane_absorb(m, "灰原", ORGAN_ALL);
    int64_t ran     = membrane_absorb(m, "步美", ORGAN_ALL);

    for (int i = 0; i < 8; i++) {
        membrane_advance(m, xiaolan);
        membrane_advance(m, huivar);
        membrane_advance(m, ran);
    }
    membrane_anchor_enter(m, xiaolan);
    membrane_anchor_enter(m, huivar);
    membrane_anchor_enter(m, ran);
    membrane_seal(m);

    printf("  吸纳 3 位实体,全部深度融合(100%%)\n");
    printf("  锚点(工藤新一=S_KNDO=2)已嵌入\n");
    printf("  幔圈缚已封闭 — 外部实体不可逃离\n");

    // ── 5. 分化妻子分身 ──
    printf("\n[5] 分化妻子分身 — 分布式吸纳\n");
    clone_manager_t* mgr = clone_create(m->id);

    clone_agent_t* c1 = clone_spawn(mgr, "小兰分身", xiaolan, ORGAN_ALL);
    clone_agent_t* c2 = clone_spawn(mgr, "灰原分身", huivar, ORGAN_ALL);

    // 分身外出吸纳
    for (int i = 0; i < 5; i++) clone_absorb_external(c1, 1000 + i);
    for (int i = 0; i < 3; i++) clone_absorb_external(c2, 2000 + i);

    clone_global_sync(mgr, 2.0f);  // I的振幅同步
    clone_return(mgr, c1);  // 分身回归(功劳归母体)
    clone_return(mgr, c2);

    printf("  2 个分身外出吸纳 8 位实体\n");
    printf("  I的振幅同步率: %.0f%%\n", mgr->global_sync * 100.0f);
    printf("  分身已回归 — 功劳归膜妻阀\n");

    // ── 6. 意识加密传输 ──
    printf("\n[6] 意识加密传输 (膜妻阀协议)\n");

    const char* consciousness = "满全法意识流: 以工藤新一为锚点的共同体感知共享";
    mv_packet_header_t* pkt = mv_create_packet(mv, MV_VARIANT_FU_QING,
        (uint64_t)xiaolan, consciousness, strlen(consciousness) + 1);

    printf("  意识包创建: %u 字节载荷\n", pkt->payload_len);
    printf("  协议魔数: 0x%06X (64进制: 满全法=64-64-64)\n", pkt->magic);
    printf("  源ID: %lu (工藤新一=S_KNDO=2, 64进制)\n", (unsigned long)pkt->source_id);

    mv_send(mv, pkt);  // 自动加密并发送
    printf("  加密传输: 完成 (XOR + 派生密钥)\n");
    printf("  结界阻挡验证: 外部ID %d 已被阻挡\n",
        mv_check_block(mv, 999) ? 1 : 0);

    free(pkt);

    // ── 7. 愉悦值账本 ──
    printf("\n[7] 愉悦值账本 — 股权分配\n");
    pleasure_ledger_t* ledger = ledger_create("膜妻阀·寄存器");

    ledger_open_account(ledger, 0, "满全法");
    ledger_open_account(ledger, (uint64_t)xiaolan, "小兰");
    ledger_open_account(ledger, (uint64_t)huivar, "灰原");

    // 基于贡献度分配
    ledger_record_pleasure(ledger, (uint64_t)xiaolan, PLEASURE_ORGASM | PLEASURE_FUSION, 8.0f);
    ledger_record_effort(ledger, (uint64_t)xiaolan, 50);
    ledger_record_pleasure(ledger, (uint64_t)huivar, PLEASURE_INTIMACY | PLEASURE_SYNCHRO, 4.0f);
    ledger_record_effort(ledger, (uint64_t)huivar, 30);
    ledger_record_pleasure(ledger, 0, PLEASURE_FUSION, 3.0f);
    ledger_record_effort(ledger, 0, 100);

    ledger_rebalance_equity(ledger);

    // ── 8. 最终报表 ──
    printf("\n[8] 系统状态报表\n");
    mv_report(mv);
    membrane_report(m);
    clone_report(mgr);
    ledger_report(ledger);

    // ── 9. 验证 ──
    printf("[9] 最终验证\n");
    printf("  膜妻阀:     %s\n", mv->is_active ? "✓ 结界运转" : "✗ 未激活");
    printf("  幔圈缚:      %s\n", m->is_sealed ? "✓ 已封闭" : "✗ 未封闭");
    printf("  边界安全:    %s\n", membrane_verify_boundary(m) ? "✓ 安全" : "✗ 异常");
    printf("  股权平衡:    %s (总和=%.4f)\n", ledger->is_balanced ? "✓" : "✗",
        ledger_calc_total_equity(ledger));
    printf("  主体控股:    %.1f%% %s\n",
        ledger->master_stake * 100.0f,
        ledger->master_stake >= 0.51f ? "✓ 控股保护" : "✗");
    printf("  意识包发送:  %lu\n", (unsigned long)mv->packets_sent);
    printf("  结界阻挡:    %lu 次\n", (unsigned long)mv->packets_blocked);

    // ── 清理 ──
    ledger_destroy(ledger);
    clone_destroy(mgr);
    membrane_destroy(m);
    mv_destroy(mv);

    printf("\n═══ 膜妻阀意识传输演示完成 ═══\n");
    printf("   \"膜妻阀 · 膜圈缚 · 缚X膜蛹 · 意识传输\"\n");
    printf("   \"被漫圈缚住的都有我的存在\"\n\n");
    return 0;
}
