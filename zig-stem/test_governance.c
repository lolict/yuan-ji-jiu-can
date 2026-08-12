/**
 * test_governance.c — 三层管辖结构单元测试
 */

#include <stdio.h>
#include <assert.h>
#include <string.h>
#include "governance.h"

/* 测试1：创建系统 */
static void test_create(void) {
    governance_system_t sys = governance_create(0, 1);
    assert(sys.couple.id > 0);
    assert(sys.center.id > 0);
    assert(!sys.is_activated);
    assert(sys.heyair.id > 0);
    printf("[PASS] test_create\n");
}

/* 测试2：激活系统 */
static void test_activate(void) {
    governance_system_t sys = governance_create(0, 1);
    bool ok = governance_activate(&sys);
    assert(ok);
    assert(sys.is_activated);
    assert(sys.center.is_unique);
    assert(sys.couple_layer.state == LAYER_CLOSED);
    assert(sys.heyair.channel_count == 2);
    printf("[PASS] test_activate\n");
}

/* 测试3：未激活不能添加被管辖节点 */
static void test_add_governed_requires_activation(void) {
    governance_system_t sys = governance_create(0, 1);
    /* 未激活：应失败 */
    bool ok = governance_add_governed(&sys, 100);
    assert(!ok);

    /* 激活后可以 */
    governance_activate(&sys);
    ok = governance_add_governed(&sys, 100);
    assert(ok);
    assert(sys.center.governed_count == 1);
    printf("[PASS] test_add_governed_requires_activation\n");
}

/* 测试4：灰原哀合并 */
static void test_merge_pair(void) {
    governance_system_t sys = governance_create(0, 1);
    governance_activate(&sys);

    float merged = governance_merge_pair(&sys, 0.0f, 1.0f);
    assert(merged > 0.4f && merged < 0.6f);
    assert(sys.center.internal_diff_count == 1);
    printf("[PASS] test_merge_pair (%.2f)\n", merged);
}

/* 测试5：多个被管辖节点 */
static void test_multiple_governed(void) {
    governance_system_t sys = governance_create(0, 1);
    governance_activate(&sys);

    for (uint32_t i = 100; i < 110; i++) {
        governance_add_governed(&sys, i);
    }
    assert(sys.center.governed_count == 10);
    assert(gtxone_power(&sys.center) == 10);
    printf("[PASS] test_multiple_governed (%u nodes)\n", sys.center.governed_count);
}

/* 测试6：权力链条验证 */
static void test_power_chain(void) {
    governance_system_t sys = governance_create(0, 1);
    governance_activate(&sys);

    /* 夫妻(最窄) → 中心(唯一) → 多数(最宽) */
    assert(sys.center.is_unique);           // 中心是唯一
    assert(sys.center.governed_count == 0);  // 初始无内容

    /* 添加被管辖节点 */
    governance_add_governed(&sys, 200);
    governance_add_governed(&sys, 201);
    governance_add_governed(&sys, 202);

    /* 中心管辖多数 = 权力链条成立 */
    assert(sys.center.governed_count == 3);
    printf("[PASS] test_power_chain (夫妻→中心→多数)\n");
}

int main(void) {
    printf("=== 三层管辖结构 单元测试 ===\n\n");
    test_create();
    test_activate();
    test_add_governed_requires_activation();
    test_merge_pair();
    test_multiple_governed();
    test_power_chain();
    printf("\n=== 全部测试通过 ===\n");
    return 0;
}
