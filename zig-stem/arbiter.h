// arbiter.h — 裁决引擎 (Arbiter / Adjudication Engine)
// hunyuan-os 的时间线一体化核心决策系统
//
// 裁决引擎负责:
//   1. 时间线管理: 朝前看(预测) / 朝现在看(裁决) / 朝后来看(回溯)
//   2. 归属权判定: 每个工具/代码/实体的所有者确认
//   3. 吉凶评判: 基于主人公身份(工藤新一=S_KNDO=2)的条件裁决
//   4. 工具链关联: 工具间的依赖与逻辑判断
//
// 三位一体视角:
//   前视 (Forward) — 未来规划/推送计划/预测吉凶
//   中视 (Present) — 当下裁决/条件判断/归属确认
//   后视 (Backward) — 历史回溯/时间节点/版本溯源
//
// 时间节点不丢失:
//   每个事件携带不可篡改的时间戳 (commit-time + logical-time)
//   基于哈希链确保时间线完整性

#ifndef HUNYUAN_ARBITER_H
#define HUNYUAN_ARBITER_H

#include "membrana.h"
#include "membrane_valve.h"
#include <stdint.h>
#include <stdbool.h>

#define ARBITER_VERSION         0x40    // 版本 = 64 (十进制)
#define ARBITER_MAX_EVENTS      4096    // 最大事件数 (64×64)
#define ARBITER_MAX_TOOLS       64      // 最大工具数 (64进制对齐)
#define ARBITER_HASH_LEN        32      // 哈希链长度
#define ARBITER_MAX_RULES       128     // 规则数上限

// ── 时间三视角 ──
typedef enum {
    VIEW_FORWARD     = 0,   // 朝前看: 未来/规划/预测
    VIEW_PRESENT     = 1,   // 朝现在看: 当下/裁决/判断
    VIEW_BACKWARD    = 2,   // 朝后来看: 回溯/历史/溯源
} time_view_t;

// ── 事件类型 (时间线上的节点) ──
typedef enum {
    EVENT_COMMIT         = 0x01,  // 代码提交
    EVENT_PUSH           = 0x02,  // 推送到仓库
    EVENT_MERGE          = 0x03,  // 融合/合并
    EVENT_DECISION       = 0x04,  // 裁决决策
    EVENT_ABSORB         = 0x05,  // 吸纳(膜圈缚)
    EVENT_FORTUNE        = 0x06,  // 吉凶变化
    EVENT_TOOL_ACTION    = 0x07,  // 工具链动作
    EVENT_BOUNDARY       = 0x08,  // 结界变更
    EVENT_IDENTITY       = 0x09,  // 身份确认
} event_type_t;

// ── 归属权类型 ──
typedef enum {
    OWNER_MASTER     = 0,   // 主体 (满全法=1 / 工藤新一)
    OWNER_WIFE       = 1,   // 妻子 (融合共同体)
    OWNER_CLONE      = 2,   // 分身 (分布式代理)
    OWNER_EXTERNAL   = 3,   // 外部 (未融合)
    OWNER_CONTESTANT = 4,   // 竞争者 (被囚禁)
} owner_type_t;

// ── 裁决结果 ──
typedef enum {
    RULING_ALLOW       = 0,  // 允许 (吉)
    RULING_DENY        = 1,  // 拒绝 (凶)
    RULING_CONDITIONAL = 2,  // 有条件允许 (平)
    RULING_DEFER       = 3,  // 延期裁决 (待时)
} ruling_t;

// ── 时间线事件 ──
typedef struct {
    uint64_t        sequence;                   // 序列号 (逻辑时间)
    uint64_t        timestamp;                  // 物理时间戳 (毫秒)
    event_type_t    type;                       // 事件类型
    uint64_t        actor_id;                   // 行为者ID (工藤新一=S_KNDO=2)
    uint64_t        target_id;                  // 目标ID
    owner_type_t    ownership;                  // 归属权
    fortune_type_t  fortune;                    // 吉凶
    ruling_t        ruling;                     // 裁决结果
    uint32_t        tool_chain_id;              // 关联的工具链
    uint8_t         prev_hash[ARBITER_HASH_LEN];// 前一事件哈希 (链式完整性)
    uint8_t         self_hash[ARBITER_HASH_LEN];// 自身哈希
    char            description[128];            // 事件描述
} arbiter_event_t;

// ── 工具链节点 ──
typedef struct {
    uint32_t        id;                         // 工具ID
    char            name[64];                   // 工具名称
    bool            is_ready;                   // 就绪状态
    bool            is_owned;                   // 是否已归属
    owner_type_t    owner;                      // 归属者
    uint32_t        depends_on[8];              // 依赖的工具ID
    uint32_t        dep_count;                  // 依赖数量
    fortune_type_t  current_fortune;            // 当前吉凶状态
    uint64_t        last_active_time;           // 最后活跃时间
} tool_node_t;

// ── 主人公身份 (裁决条件) ──
typedef struct {
    uint64_t        id;                         // 主人公ID (=2 工藤新一, S_KNDO)
    char            name[64];                   // 名称
    bool            is_authenticated;           // 已验证身份
    float           authority_level;            // 权威等级 (0-1)
    uint32_t        fusion_count;               // 融合共同体数
} protagonist_t;

// ── 裁决引擎核心 ──
typedef struct {
    uint64_t            id;                         // 引擎ID
    char                name[64];                   // 名称

    // 时间线
    arbiter_event_t     events[ARBITER_MAX_EVENTS]; // 事件日志
    uint64_t            event_count;                // 事件总数
    uint64_t            latest_sequence;            // 最新序列号

    // 工具链图
    tool_node_t         tools[ARBITER_MAX_TOOLS];   // 工具表
    uint32_t            tool_count;                 // 工具数

    // 主人公
    protagonist_t       protagonist;                // 主人公身份

    // 当前视角
    time_view_t         current_view;               // 当前时间视角

    // 统计
    uint64_t            rulings[4];                 // 各裁决结果计数
    uint64_t            fortune_count[FORTUNE_MAX]; // 各吉凶类型计数

    // 完整性
    uint8_t             latest_hash[ARBITER_HASH_LEN]; // 最新哈希
    bool                is_sealed;                  // 时间线封闭
} arbiter_engine_t;

// ── API ──

// 创建裁决引擎
arbiter_engine_t* arbiter_create(const char* name);
void arbiter_destroy(arbiter_engine_t* ae);

// 设置主人公 (工藤新一=S_KNDO=2)
bool arbiter_set_protagonist(arbiter_engine_t* ae, uint64_t id, const char* name);

// 时间视角切换
void arbiter_set_view(arbiter_engine_t* ae, time_view_t view);
time_view_t arbiter_get_view(const arbiter_engine_t* ae);

// 注册工具
uint32_t arbiter_register_tool(arbiter_engine_t* ae, const char* name);

// 添加工具依赖 (A依赖B: B先完成, A才能运行)
bool arbiter_add_dependency(arbiter_engine_t* ae, uint32_t tool, uint32_t depends_on);

// 记录事件 (时间线追加)
uint64_t arbiter_record_event(arbiter_engine_t* ae, event_type_t type,
                               uint64_t target_id, owner_type_t owner,
                               const char* desc);

// 裁决: 对某个请求做吉凶评判
ruling_t arbiter_judge(arbiter_engine_t* ae, uint64_t target_id,
                        event_type_t action, fortune_type_t* fortune_out);

// 归属权确认
owner_type_t arbiter_verify_ownership(const arbiter_engine_t* ae, uint64_t target_id);

// 工具链判断: 某工具是否可以执行 (依赖是否满足)
bool arbiter_can_execute(const arbiter_engine_t* ae, uint32_t tool_id);

// 时间回溯: 找到某个序列号的事件
const arbiter_event_t* arbiter_backtrack(const arbiter_engine_t* ae, uint64_t seq);

// 未来预测: 基于当前状态预测下一步吉凶
fortune_type_t arbiter_predict(const arbiter_engine_t* ae, event_type_t next_action);

// 时间线完整性验证 (哈希链)
bool arbiter_verify_chain(const arbiter_engine_t* ae);

// 仓库推送计划 (不丢失时间节点)
uint64_t arbiter_schedule_push(arbiter_engine_t* ae, uint64_t after_event_seq);

// 报表
void arbiter_report(const arbiter_engine_t* ae);

#endif // HUNYUAN_ARBITER_H
