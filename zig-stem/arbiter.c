// arbiter.c — 裁决引擎实现
// 参见 arbiter.h 的架构说明
//
// 裁决引擎核心:
//   1. 时间线事件链 (哈希链确保不丢失时间节点)
//   2. 归属权判定 (每个工具的拥有者确认)
//   3. 吉凶裁决 (基于主人公身份的条件评判)
//   4. 工具链依赖判断 (前驱后继的逻辑推理)

#include "arbiter.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// ── 常量 ──
static const float AUTHORITY_THRESHOLD = 0.5f;    // 权威等级阈值

// ── 时间工具 ──
static uint64_t time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// ── 简化哈希 (FNV-1a 变体,用于时间线链式完整性) ──
static void simple_hash(const void* data, size_t len, uint8_t out[ARBITER_HASH_LEN]) {
    uint64_t hash = 14695981039346656037ULL;
    const uint8_t* bytes = (const uint8_t*)data;
    for (size_t i = 0; i < len; i++) {
        hash ^= bytes[i];
        hash *= 1099511628211ULL;
    }
    for (int i = 0; i < ARBITER_HASH_LEN; i++) {
        out[i] = (uint8_t)((hash >> (i * 4)) & 0xFF);
    }
}

// 计算事件哈希
static void hash_event(const arbiter_event_t* ev, uint8_t out[ARBITER_HASH_LEN]) {
    // 哈希 = hash(序列号 + 时间戳 + 类型 + 行为者 + 目标 + 前哈希)
    uint8_t buf[64];
    memset(buf, 0, sizeof(buf));
    memcpy(buf, &ev->sequence, sizeof(ev->sequence));
    memcpy(buf + 8, &ev->timestamp, sizeof(ev->timestamp));
    buf[16] = (uint8_t)ev->type;
    buf[17] = (uint8_t)ev->ownership;
    memcpy(buf + 18, ev->prev_hash, 14);  // 前哈希的部分
    simple_hash(buf, sizeof(buf), out);
}

// ── 创建裁决引擎 ──
arbiter_engine_t* arbiter_create(const char* name) {
    arbiter_engine_t* ae = (arbiter_engine_t*)calloc(1, sizeof(arbiter_engine_t));
    if (!ae) return NULL;

    strncpy(ae->name, name, sizeof(ae->name) - 1);
    ae->name[sizeof(ae->name) - 1] = '\0';
    ae->id = time_now_ms();
    ae->current_view = VIEW_PRESENT;  // 默认朝现在看
    ae->event_count = 0;
    ae->latest_sequence = 0;
    ae->tool_count = 0;
    ae->is_sealed = false;

    // 初始哈希为零
    memset(ae->latest_hash, 0, ARBITER_HASH_LEN);

    return ae;
}

void arbiter_destroy(arbiter_engine_t* ae) {
    if (ae) free(ae);
}

// ── 设置主人公 ──
bool arbiter_set_protagonist(arbiter_engine_t* ae, uint64_t id, const char* name) {
    if (!ae) return false;
    ae->protagonist.id = id;
    strncpy(ae->protagonist.name, name, sizeof(ae->protagonist.name) - 1);
    ae->protagonist.name[sizeof(ae->protagonist.name) - 1] = '\0';
    ae->protagonist.is_authenticated = true;
    ae->protagonist.authority_level = 1.0f;  // 主人公权威满级
    return true;
}

// ── 视角切换 ──
void arbiter_set_view(arbiter_engine_t* ae, time_view_t view) {
    if (ae) ae->current_view = view;
}

time_view_t arbiter_get_view(const arbiter_engine_t* ae) {
    return ae ? ae->current_view : VIEW_PRESENT;
}

// ── 注册工具 ──
uint32_t arbiter_register_tool(arbiter_engine_t* ae, const char* name) {
    if (!ae || ae->tool_count >= ARBITER_MAX_TOOLS) return 0;

    uint32_t id = ae->tool_count++;
    tool_node_t* t = &ae->tools[id];
    t->id = id;
    strncpy(t->name, name, sizeof(t->name) - 1);
    t->name[sizeof(t->name) - 1] = '\0';
    t->is_ready = true;
    t->is_owned = false;
    t->owner = OWNER_EXTERNAL;
    t->dep_count = 0;
    t->current_fortune = FORTUNE_NEUTRAL;
    t->last_active_time = time_now_ms();

    return id;
}

// ── 添加依赖 ──
bool arbiter_add_dependency(arbiter_engine_t* ae, uint32_t tool, uint32_t depends_on) {
    if (!ae || tool >= ae->tool_count || depends_on >= ae->tool_count) return false;

    tool_node_t* t = &ae->tools[tool];
    if (t->dep_count >= 8) return false;

    t->depends_on[t->dep_count++] = depends_on;
    return true;
}

// ── 记录事件 (时间线追加) ──
uint64_t arbiter_record_event(arbiter_engine_t* ae, event_type_t type,
                               uint64_t target_id, owner_type_t owner,
                               const char* desc) {
    if (!ae || ae->event_count >= ARBITER_MAX_EVENTS) return 0;

    arbiter_event_t* ev = &ae->events[ae->event_count];
    ev->sequence = ++ae->latest_sequence;
    ev->timestamp = time_now_ms();
    ev->type = type;
    ev->actor_id = ae->protagonist.is_authenticated ? ae->protagonist.id : 0;
    ev->target_id = target_id;
    ev->ownership = owner;
    ev->fortune = FORTUNE_NEUTRAL;  // 默认为平

    // 裁决: 基于主人公身份做初步裁决
    if (ae->protagonist.is_authenticated && ae->protagonist.id == 2) {
        // 工藤新一=2 (S_KNDO),对所有目标有裁决权
        ev->ruling = RULING_ALLOW;
        ev->fortune = FORTUNE_AUSPICIOUS;
    } else if (owner == OWNER_MASTER) {
        ev->ruling = RULING_ALLOW;
    } else {
        ev->ruling = RULING_CONDITIONAL;
    }

    ev->tool_chain_id = 0;
    if (desc) {
        strncpy(ev->description, desc, sizeof(ev->description) - 1);
        ev->description[sizeof(ev->description) - 1] = '\0';
    } else {
        ev->description[0] = '\0';
    }

    // 前哈希 = 引擎最新哈希 (链式连接)
    memcpy(ev->prev_hash, ae->latest_hash, ARBITER_HASH_LEN);

    // 计算自身哈希
    hash_event(ev, ev->self_hash);

    // 更新引擎最新哈希
    memcpy(ae->latest_hash, ev->self_hash, ARBITER_HASH_LEN);

    ae->event_count++;
    ae->rulings[ev->ruling]++;
    ae->fortune_count[ev->fortune]++;

    return ev->sequence;
}

// ── 裁决 ──
ruling_t arbiter_judge(arbiter_engine_t* ae, uint64_t target_id,
                        event_type_t action, fortune_type_t* fortune_out) {
    if (!ae) {
        if (fortune_out) *fortune_out = FORTUNE_MISFORTUNE;
        return RULING_DENY;
    }

    // 条件1: 主人公身份验证
    if (!ae->protagonist.is_authenticated) {
        ae->rulings[RULING_DENY]++;
        if (fortune_out) *fortune_out = FORTUNE_INAUSPICIOUS;
        return RULING_DENY;  // 身份未验证 → 拒绝
    }

    // 条件2: 权威等级检查
    if (ae->protagonist.authority_level < AUTHORITY_THRESHOLD) {
        ae->rulings[RULING_DEFER]++;
        if (fortune_out) *fortune_out = FORTUNE_NEUTRAL;
        return RULING_DEFER;  // 权威不足 → 延期
    }

    // 条件3: 归属权判定
    owner_type_t owner = arbiter_verify_ownership(ae, target_id);

    if (owner == OWNER_MASTER) {
        // 主体自身
        ae->rulings[RULING_ALLOW]++;
        ae->fortune_count[FORTUNE_AUSPICIOUS]++;
        if (fortune_out) *fortune_out = FORTUNE_AUSPICIOUS;
        return RULING_ALLOW;
    }
    if (owner == OWNER_WIFE || owner == OWNER_CLONE) {
        // 妻子或分身 (共同体内部)
        ae->rulings[RULING_ALLOW]++;
        ae->fortune_count[FORTUNE_BLESSING]++;
        if (fortune_out) *fortune_out = FORTUNE_BLESSING;
        return RULING_ALLOW;
    }
    if (owner == OWNER_CONTESTANT) {
        // 竞争者/被囚禁者
        ae->rulings[RULING_DENY]++;
        ae->fortune_count[FORTUNE_MISFORTUNE]++;
        if (fortune_out) *fortune_out = FORTUNE_MISFORTUNE;
        return RULING_DENY;
    }

    // 外部: 看行动类型
    if (action == EVENT_PUSH || action == EVENT_COMMIT) {
        ae->rulings[RULING_CONDITIONAL]++;
        ae->fortune_count[FORTUNE_NEUTRAL]++;
        if (fortune_out) *fortune_out = FORTUNE_NEUTRAL;
        return RULING_CONDITIONAL;
    }

    // 默认拒绝
    ae->rulings[RULING_DENY]++;
    ae->fortune_count[FORTUNE_INAUSPICIOUS]++;
    if (fortune_out) *fortune_out = FORTUNE_INAUSPICIOUS;
    return RULING_DENY;
}

// ── 归属权确认 ──
owner_type_t arbiter_verify_ownership(const arbiter_engine_t* ae, uint64_t target_id) {
    if (!ae) return OWNER_EXTERNAL;

    // 主体自身 (工藤新一=S_KNDO=2 或 满全法=0)
    if (target_id == 1 || target_id == 0 || target_id == ae->protagonist.id) {
        return OWNER_MASTER;
    }

    // 查找历史事件中的归属
    for (uint64_t i = ae->event_count; i > 0; i--) {
        const arbiter_event_t* ev = &ae->events[i - 1];
        if (ev->target_id == target_id) {
            return ev->ownership;
        }
    }

    return OWNER_EXTERNAL;  // 未找到 = 外部
}

// ── 工具链判断: 是否可以执行 ──
bool arbiter_can_execute(const arbiter_engine_t* ae, uint32_t tool_id) {
    if (!ae || tool_id >= ae->tool_count) return false;

    const tool_node_t* t = &ae->tools[tool_id];
    if (!t->is_ready) return false;
    if (!t->is_owned) return false;  // 未归属不可执行

    // 检查所有依赖
    for (uint32_t i = 0; i < t->dep_count; i++) {
        uint32_t dep_id = t->depends_on[i];
        if (dep_id >= ae->tool_count) return false;
        const tool_node_t* dep = &ae->tools[dep_id];
        if (!dep->is_ready) return false;  // 依赖未就绪
        if (dep->current_fortune == FORTUNE_MISFORTUNE) return false;  // 依赖有祸
    }

    return true;  // 所有依赖满足
}

// ── 时间回溯 ──
const arbiter_event_t* arbiter_backtrack(const arbiter_engine_t* ae, uint64_t seq) {
    if (!ae) return NULL;

    for (uint64_t i = 0; i < ae->event_count; i++) {
        if (ae->events[i].sequence == seq) {
            return &ae->events[i];
        }
    }
    return NULL;
}

// ── 未来预测 ──
fortune_type_t arbiter_predict(const arbiter_engine_t* ae, event_type_t next_action) {
    if (!ae) return FORTUNE_MISFORTUNE;

    // 基于最近事件的吉凶做简单预测
    if (ae->event_count == 0) return FORTUNE_NEUTRAL;

    const arbiter_event_t* latest = &ae->events[ae->event_count - 1];

    // 如果最近是吉,预测继续吉
    if (latest->fortune == FORTUNE_AUSPICIOUS || latest->fortune == FORTUNE_BLESSING) {
        // 看行动类型
        if (next_action == EVENT_PUSH || next_action == EVENT_COMMIT) {
            return FORTUNE_AUSPICIOUS;  // 提交推送在吉后 → 继续吉
        }
        return FORTUNE_NEUTRAL;
    }

    // 如果最近是凶,预测转吉 (物极必反)
    if (latest->fortune == FORTUNE_INAUSPICIOUS || latest->fortune == FORTUNE_MISFORTUNE) {
        return FORTUNE_BLESSING;  // 凶后转福
    }

    return FORTUNE_NEUTRAL;
}

// ── 时间线完整性验证 ──
bool arbiter_verify_chain(const arbiter_engine_t* ae) {
    if (!ae || ae->event_count == 0) return true;

    uint8_t expected_prev[ARBITER_HASH_LEN];
    memset(expected_prev, 0, ARBITER_HASH_LEN);

    for (uint64_t i = 0; i < ae->event_count; i++) {
        const arbiter_event_t* ev = &ae->events[i];
        // 检查前哈希是否匹配
        if (memcmp(ev->prev_hash, expected_prev, ARBITER_HASH_LEN) != 0) {
            return false;  // 链断裂
        }
        // 更新期望的前哈希 = 当前事件的自身哈希
        memcpy(expected_prev, ev->self_hash, ARBITER_HASH_LEN);
    }

    // 检查最终哈希是否匹配引擎记录
    return memcmp(expected_prev, ae->latest_hash, ARBITER_HASH_LEN) == 0;
}

// ── 仓库推送计划 ──
uint64_t arbiter_schedule_push(arbiter_engine_t* ae, uint64_t after_event_seq) {
    if (!ae) return 0;

    // 记录推送事件,关联到前序事件 (时间节点不丢失)
    char desc[128];
    snprintf(desc, sizeof(desc), "推送计划: 在事件#%lu之后",
        (unsigned long)after_event_seq);

    return arbiter_record_event(ae, EVENT_PUSH, 0, OWNER_MASTER, desc);
}

// ── 报表 ──
void arbiter_report(const arbiter_engine_t* ae) {
    if (!ae) return;

    const char* view_names[] = {"朝前看(预测)", "朝现在看(裁决)", "朝后来看(回溯)"};
    const char* fortune_names[] = {"平", "吉", "凶", "祸", "福"};
    const char* ruling_names[] = {"允许", "拒绝", "有条件", "延期"};

    printf("\n═══ 裁决引擎 [%s] ═══\n", ae->name);
    printf("  当前视角:   %s\n", view_names[ae->current_view]);
    printf("  主人公:     %s (ID=%lu, 权威:%.0f%%)%s\n",
        ae->protagonist.name,
        (unsigned long)ae->protagonist.id,
        ae->protagonist.authority_level * 100.0f,
        ae->protagonist.is_authenticated ? " ✓已验证" : " ✗未验证");
    printf("  时间线:     %lu 事件 %s\n", (unsigned long)ae->event_count,
        ae->is_sealed ? "(已封闭)" : "(开放)");
    printf("  工具链:     %u 个工具\n", ae->tool_count);
    printf("  完整性:     %s\n", arbiter_verify_chain(ae) ? "✓ 链完整" : "✗ 链断裂");

    printf("  ── 裁决统计 ──\n");
    printf("    ");
    for (int i = 0; i < 4; i++) {
        printf("%s:%lu ", ruling_names[i], (unsigned long)ae->rulings[i]);
    }
    printf("\n");

    printf("  ── 吉凶分布 ──\n");
    printf("    ");
    for (int i = 0; i < FORTUNE_MAX; i++) {
        printf("%s:%lu ", fortune_names[i], (unsigned long)ae->fortune_count[i]);
    }
    printf("\n");

    // 最近 5 个事件
    if (ae->event_count > 0) {
        printf("  ── 最近事件 ──\n");
        uint64_t start = (ae->event_count > 5) ? ae->event_count - 5 : 0;
        for (uint64_t i = start; i < ae->event_count; i++) {
            const arbiter_event_t* ev = &ae->events[i];
            const char* ev_names[] = {"","提交","推送","融合","裁决","吸纳",
                                       "吉凶","工具","结界","身份"};
            const char* ev_name = (ev->type < 10) ? ev_names[ev->type] : "?";
            printf("    #%lu [%s] %s → 目标%lu [%s]%s\n",
                (unsigned long)ev->sequence,
                ev_name,
                fortune_names[ev->fortune],
                (unsigned long)ev->target_id,
                ruling_names[ev->ruling],
                ev->description[0] ? ev->description : "");
        }
    }
    printf("═══════════════════════════════════\n\n");
}
