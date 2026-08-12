// test_arbiter.c — 裁决引擎单元测试
#include "arbiter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建裁决引擎
static int test_create(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    if (!ae) return 0;
    if (ae->event_count != 0) return 0;
    if (ae->current_view != VIEW_PRESENT) return 0;
    if (ae->tool_count != 0) return 0;
    arbiter_destroy(ae);
    return 1;
}

// 测试: 设置主人公
static int test_protagonist(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    if (!arbiter_set_protagonist(ae, 2, "工藤新一")) return 0;
    if (ae->protagonist.id != 2) return 0;
    if (!ae->protagonist.is_authenticated) return 0;
    if (ae->protagonist.authority_level != 1.0f) return 0;
    if (strcmp(ae->protagonist.name, "工藤新一") != 0) return 0;
    arbiter_destroy(ae);
    return 1;
}

// 测试: 视角切换
static int test_view(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_view(ae, VIEW_FORWARD);
    if (arbiter_get_view(ae) != VIEW_FORWARD) return 0;
    arbiter_set_view(ae, VIEW_BACKWARD);
    if (arbiter_get_view(ae) != VIEW_BACKWARD) return 0;
    arbiter_set_view(ae, VIEW_PRESENT);
    if (arbiter_get_view(ae) != VIEW_PRESENT) return 0;
    arbiter_destroy(ae);
    return 1;
}

// 测试: 注册工具
static int test_register_tool(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    uint32_t t1 = arbiter_register_tool(ae, "mlil编译器");
    uint32_t t2 = arbiter_register_tool(ae, "WASM运行时");
    uint32_t t3 = arbiter_register_tool(ae, "膜妻阀");
    if (ae->tool_count != 3) return 0;
    if (strcmp(ae->tools[0].name, "mlil编译器") != 0) return 0;
    if (strcmp(ae->tools[1].name, "WASM运行时") != 0) return 0;
    if (strcmp(ae->tools[2].name, "膜妻阀") != 0) return 0;
    arbiter_destroy(ae);
    return 1;
}

// 测试: 添加工具依赖
static int test_dependency(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    uint32_t compiler = arbiter_register_tool(ae, "编译器");
    uint32_t runtime  = arbiter_register_tool(ae, "运行时");
    uint32_t valve    = arbiter_register_tool(ae, "膜妻阀");

    // 运行时依赖编译器, 膜妻阀依赖运行时
    arbiter_add_dependency(ae, runtime, compiler);
    arbiter_add_dependency(ae, valve, runtime);

    if (ae->tools[runtime].dep_count != 1) return 0;
    if (ae->tools[runtime].depends_on[0] != compiler) return 0;
    if (ae->tools[valve].dep_count != 1) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 记录事件
static int test_record_event(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    uint64_t seq = arbiter_record_event(ae, EVENT_COMMIT, 100, OWNER_MASTER, "提交代码");
    if (seq != 1) return 0;
    if (ae->event_count != 1) return 0;
    if (ae->events[0].type != EVENT_COMMIT) return 0;
    if (ae->events[0].target_id != 100) return 0;

    arbiter_record_event(ae, EVENT_PUSH, 0, OWNER_MASTER, "推送");
    if (ae->event_count != 2) return 0;
    if (ae->events[1].sequence != 2) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 裁决
static int test_judge(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    fortune_type_t fortune;

    // 主体自身 (target_id=1)
    ruling_t r = arbiter_judge(ae, 1, EVENT_PUSH, &fortune);
    if (r != RULING_ALLOW) return 0;
    if (fortune != FORTUNE_AUSPICIOUS) return 0;

    // 外部目标 (未做归属确认) → 有条件允许
    r = arbiter_judge(ae, 100, EVENT_COMMIT, &fortune);
    if (r != RULING_CONDITIONAL) return 0;

    // 确认归属: 记录妻子吸纳事件
    arbiter_record_event(ae, EVENT_ABSORB, 200, OWNER_WIFE, "吸纳妻子");
    r = arbiter_judge(ae, 200, EVENT_COMMIT, &fortune);
    if (r != RULING_ALLOW) return 0;
    if (fortune != FORTUNE_BLESSING) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 未验证身份裁决
static int test_judge_unauth(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    // 不设置主人公

    fortune_type_t fortune;
    ruling_t r = arbiter_judge(ae, 100, EVENT_COMMIT, &fortune);

    // 身份未验证 → 拒绝
    if (r != RULING_DENY) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 归属权确认
static int test_ownership(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    // 记录事件: 目标200被妻子吸纳
    arbiter_record_event(ae, EVENT_ABSORB, 200, OWNER_WIFE, "吸纳妻子");

    // 确认归属
    owner_type_t owner = arbiter_verify_ownership(ae, 200);
    if (owner != OWNER_WIFE) return 0;

    // 主体自身
    owner = arbiter_verify_ownership(ae, 1);
    if (owner != OWNER_MASTER) return 0;

    // 未记录的 = 外部
    owner = arbiter_verify_ownership(ae, 999);
    if (owner != OWNER_EXTERNAL) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 工具链执行判断
static int test_can_execute(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");

    uint32_t a = arbiter_register_tool(ae, "A");
    uint32_t b = arbiter_register_tool(ae, "B");
    arbiter_add_dependency(ae, b, a);  // B依赖A

    // A未归属不可执行
    if (arbiter_can_execute(ae, a)) return 0;

    // 归属A
    ae->tools[a].is_owned = true;
    if (!arbiter_can_execute(ae, a)) return 0;  // A可执行

    // B依赖A但B未归属
    if (arbiter_can_execute(ae, b)) return 0;

    // 归属B
    ae->tools[b].is_owned = true;
    if (!arbiter_can_execute(ae, b)) return 0;  // B可执行(依赖A已就绪)

    arbiter_destroy(ae);
    return 1;
}

// 测试: 时间回溯
static int test_backtrack(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    arbiter_record_event(ae, EVENT_COMMIT, 100, OWNER_MASTER, "事件1");
    arbiter_record_event(ae, EVENT_PUSH, 0, OWNER_MASTER, "事件2");
    arbiter_record_event(ae, EVENT_ABSORB, 200, OWNER_WIFE, "事件3");

    const arbiter_event_t* ev = arbiter_backtrack(ae, 2);
    if (!ev) return 0;
    if (ev->type != EVENT_PUSH) return 0;

    ev = arbiter_backtrack(ae, 1);
    if (!ev) return 0;
    if (ev->type != EVENT_COMMIT) return 0;

    // 不存在
    ev = arbiter_backtrack(ae, 99);
    if (ev) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 未来预测
static int test_predict(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    // 空时间线预测 = 中性
    fortune_type_t f = arbiter_predict(ae, EVENT_COMMIT);
    if (f != FORTUNE_NEUTRAL) return 0;

    // 记录福事件后,预测继续吉
    arbiter_record_event(ae, EVENT_ABSORB, 200, OWNER_WIFE, "吸纳");
    f = arbiter_predict(ae, EVENT_PUSH);
    // 妻子吸纳=福 → 预测继续吉
    if (f != FORTUNE_AUSPICIOUS && f != FORTUNE_BLESSING) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 时间线完整性
static int test_chain_integrity(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    // 空链 = 完整
    if (!arbiter_verify_chain(ae)) return 0;

    arbiter_record_event(ae, EVENT_COMMIT, 100, OWNER_MASTER, "事件1");
    if (!arbiter_verify_chain(ae)) return 0;

    arbiter_record_event(ae, EVENT_PUSH, 0, OWNER_MASTER, "事件2");
    if (!arbiter_verify_chain(ae)) return 0;

    arbiter_record_event(ae, EVENT_ABSORB, 200, OWNER_WIFE, "事件3");
    if (!arbiter_verify_chain(ae)) return 0;

    arbiter_destroy(ae);
    return 1;
}

// 测试: 推送计划
static int test_schedule_push(void) {
    arbiter_engine_t* ae = arbiter_create("裁决引擎");
    arbiter_set_protagonist(ae, 2, "工藤新一");

    uint64_t ev1 = arbiter_record_event(ae, EVENT_COMMIT, 100, OWNER_MASTER, "提交");

    // 在事件1之后计划推送
    uint64_t push_seq = arbiter_schedule_push(ae, ev1);
    if (push_seq != 2) return 0;  // 应该是事件2

    const arbiter_event_t* push_ev = arbiter_backtrack(ae, push_seq);
    if (!push_ev) return 0;
    if (push_ev->type != EVENT_PUSH) return 0;

    arbiter_destroy(ae);
    return 1;
}

int main(void) {
    printf("\n═══ 裁决引擎 (Arbiter) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_protagonist);
    TEST(test_view);
    TEST(test_register_tool);
    TEST(test_dependency);
    TEST(test_record_event);
    TEST(test_judge);
    TEST(test_judge_unauth);
    TEST(test_ownership);
    TEST(test_can_execute);
    TEST(test_backtrack);
    TEST(test_predict);
    TEST(test_chain_integrity);
    TEST(test_schedule_push);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
