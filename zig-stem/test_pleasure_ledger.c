// test_pleasure_ledger.c — 愉悦值账本单元测试
#include "pleasure_ledger.h"
#include <stdio.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建账本
static int test_create(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    if (!l) return 0;
    if (strcmp(l->name, "满全法") != 0) return 0;
    if (l->entry_count != 0) return 0;
    if (l->master_stake != 1.0f) return 0;
    ledger_destroy(l);
    return 1;
}

// 测试: 开户
static int test_open_account(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    int idx = ledger_open_account(l, 100, "小兰");
    if (idx < 0) return 0;
    if (l->entry_count != 1) return 0;
    if (strcmp(l->entries[0].name, "小兰") != 0) return 0;
    // 不可重复开
    if (ledger_open_account(l, 100, "小兰2") >= 0) return 0;
    ledger_destroy(l);
    return 1;
}

// 测试: 记录愉悦
static int test_record_pleasure(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 100, "小兰");

    float pts = ledger_record_pleasure(l, 100, PLEASURE_ORGASM, 1.0f);
    if (pts <= 0.0f) return 0;
    if (l->entries[0].pleasure_points <= 0.0f) return 0;

    // 多重叠加
    ledger_record_pleasure(l, 100, PLEASURE_INTIMACY, 0.5f);
    ledger_record_pleasure(l, 100, PLEASURE_SYNCHRO, 0.3f);
    if (l->total_pleasure <= 0.0f) return 0;

    ledger_destroy(l);
    return 1;
}

// 测试: 记录努力
static int test_record_effort(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 100, "小兰");

    uint32_t eff = ledger_record_effort(l, 100, 50);
    if (eff != 50) return 0;
    if (l->entries[0].task_count != 1) return 0;

   eff = ledger_record_effort(l, 100, 30);
    if (eff != 80) return 0;

    ledger_destroy(l);
    return 1;
}

// 测试: 股权平衡 (=1)
static int test_rebalance(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 0, "主体");   // 主体
    ledger_open_account(l, 1, "小兰");
    ledger_open_account(l, 2, "灰原");

    // 贡献差异化
    ledger_record_pleasure(l, 0, PLEASURE_FUSION, 2.0f);
    ledger_record_pleasure(l, 1, PLEASURE_ORGASM, 1.0f);
    ledger_record_effort(l, 1, 100);
    ledger_record_pleasure(l, 2, PLEASURE_INTIMACY, 0.5f);

    ledger_rebalance_equity(l);

    float total = ledger_calc_total_equity(l);
    // 总和应在 0.99-1.01 之间
    if (total < 0.99f || total > 1.01f) return 0;
    if (!l->is_balanced) return 0;

    ledger_destroy(l);
    return 1;
}

// 测试: 信用支配权
static int test_credit_power(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 100, "小兰");
    ledger_record_pleasure(l, 100, PLEASURE_ORGASM, 2.0f);
    ledger_record_effort(l, 100, 200);
    ledger_rebalance_equity(l);

    float cp = ledger_credit_power(l, 100);
    if (cp <= 0.0f) return 0;

    // 不存在账户
    if (ledger_credit_power(l, 999) != 0.0f) return 0;

    ledger_destroy(l);
    return 1;
}

// 测试: 从膜圈缚同步
static int test_sync_from_membrane(void) {
    pleasure_ledger_t* l = ledger_create("满全法");

    membrane_circle_t* m = membrane_create("测试", 10);
    int64_t id1 = membrane_absorb(m, "小兰", ORGAN_ALL);
    for (int i = 0; i < 10; i++) membrane_advance(m, id1);

    uint32_t count = ledger_sync_from_membrane(l, m, NULL);
    if (count < 1) return 0;

    // 应有对应账户
    int found = 0;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        if (l->entries[i].owner_id == (uint64_t)id1) { found = 1; break; }
    }
    if (!found) return 0;

    membrane_destroy(m);
    ledger_destroy(l);
    return 1;
}

// 测试: 股权排名
static int test_ranking(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 1, "A");
    ledger_open_account(l, 2, "B");
    ledger_open_account(l, 3, "C");

    ledger_record_effort(l, 1, 50);
    ledger_record_effort(l, 2, 100);
    ledger_record_effort(l, 3, 30);

    uint64_t ids[3];
    float scores[3];
    ledger_ranking(l, ids, scores, 3);

    // 排名: B(100) > A(50) > C(30)
    if (ids[0] != 2) return 0;
    if (ids[1] != 1) return 0;
    if (ids[2] != 3) return 0;

    ledger_destroy(l);
    return 1;
}

// 测试: 主体控股保护
static int test_master_stake_protection(void) {
    pleasure_ledger_t* l = ledger_create("满全法");
    ledger_open_account(l, 0, "主体");
    ledger_open_account(l, 1, "小兰");

    // 小兰大量贡献
    ledger_record_pleasure(l, 1, PLEASURE_ORGASM, 10.0f);
    ledger_record_effort(l, 1, 1000);
    // 主体最少贡献
    ledger_record_pleasure(l, 0, PLEASURE_FUSION, 0.1f);

    ledger_rebalance_equity(l);

    // 查找主体股份
    float master_share = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        if (l->entries[i].owner_id == 0) {
            master_share = l->entries[i].equity_share;
            break;
        }
    }

    // 主体应保有 >= 51% 控股权
    if (master_share < 0.51f) return 0;

    ledger_destroy(l);
    return 1;
}

int main(void) {
    printf("\n═══ 愉悦值账本 (Pleasure Ledger) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_open_account);
    TEST(test_record_pleasure);
    TEST(test_record_effort);
    TEST(test_rebalance);
    TEST(test_credit_power);
    TEST(test_sync_from_membrane);
    TEST(test_ranking);
    TEST(test_master_stake_protection);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
