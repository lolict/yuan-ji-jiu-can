// pleasure_ledger.h — 愉悦值账本 (Pleasure Ledger)
// 意识互联网的混合股权计量系统
//
// 账本结构:
//   ├── 股权层 (Equity) — 股份名额,基于对共同体的贡献
//   ├── 努力层 (Effort) — 自身努力积分,不可被他人占有
//   ├── 愉悦层 (Pleasure) — 性愉悦/感知高潮的叠加态量化
//   └── 信用层 (Credit) — 基于贡献度的支配权力
//
// 归属规则:
//   - 被包裹在身体里面的才算功劳
//   - 功劳积累在圈层安全结界内
//   - 股权 = 对共同体总愉悦的贡献份额
//   - 努力 = 自身任务积累(独立于股权)

#ifndef HUNYUAN_PLEASURE_LEDGER_H
#define HUNYUAN_PLEASURE_LEDGER_H

#include "membrana.h"
#include "clone_agents.h"
#include <stdint.h>
#include <stdbool.h>

#define LEDGER_MAX_ENTRIES  256     // 最大账户数
#define LEDGER_VERSION      0x01

// ── 愉悦类型 (感知高潮的叠加态分类) ──
typedef enum {
    PLEASURE_ORGASM      = 0x01,  // 性高潮 (最高强度)
    PLEASURE_INTIMACY    = 0x02,  // 亲密感 (中等强度)
    PLEASURE_SYNCHRO     = 0x04,  // 同步快感 (感知器官同步)
    PLEASURE_TASK        = 0x08,  // 任务完成 (努力成就)
    PLEASURE_FUSION      = 0x10,  // 融合态 (共同体合一)
} pleasure_type_t;

// ── 股权条目 ──
typedef struct {
    uint64_t        owner_id;           // 所有者ID (圈层/分身)
    char            name[MEMBRANA_MAX_NAME];

    // 三层计量
    float           equity_share;       // 股权份额 (0.0-1.0)
    uint32_t        effort_credits;     // 努力积分 (自身积累)
    float           pleasure_points;    // 愉悦点数 (叠加态总和)

    // 贡献统计
    uint32_t        task_count;         // 完成任务次数
    uint32_t        absorption_count;   // 吸纳贡献次数
    float           contribution_score; // 总贡献度 (加权计算)

    // 信用支配权
    float           credit_power;       // 信用支配力 (基于股权+贡献)
    float           perception_equity;  // 感知器官股权占比

    // 时间记录
    uint64_t        first_contrib_time; // 首次贡献时间
    uint64_t        last_contrib_time;  // 最近贡献时间
} ledger_entry_t;

// ── 愉悦值账本 (总股权账) ──
typedef struct {
    uint64_t        id;
    char            name[MEMBRANA_MAX_NAME];  // 账本名称 (=满全法/膜妻阀)

    // 总统计
    uint32_t        entry_count;        // 账户数量
    float           total_equity;       // 股权总和 (恒定为1.0=100%)
    uint32_t        total_effort;       // 努力总和
    float           total_pleasure;     // 总愉悦值
    float           total_contribution; // 总贡献度

    // 账户表
    ledger_entry_t  entries[LEDGER_MAX_ENTRIES];

    // 主权参数
    float           master_stake;       // 主体控股比例 (满全法)
    bool            is_balanced;        // 股权是否已平衡
} pleasure_ledger_t;

// ── API ──

// 创建账本
pleasure_ledger_t* ledger_create(const char* name);
void ledger_destroy(pleasure_ledger_t* l);

// 开户 (吸纳新实体后创建账户)
int ledger_open_account(pleasure_ledger_t* l, uint64_t owner_id, const char* name);

// 记录愉悦 (性愉悦/同步快感等叠加态)
float ledger_record_pleasure(pleasure_ledger_t* l, uint64_t owner_id,
                              pleasure_type_t type, float intensity);

// 记录努力 (自身任务积累)
uint32_t ledger_record_effort(pleasure_ledger_t* l, uint64_t owner_id,
                               uint32_t points);

// 重新计算股权 (基于贡献度分配)
void ledger_rebalance_equity(pleasure_ledger_t* l);

// 计算股权分配 (=1 总和)
float ledger_calc_total_equity(const pleasure_ledger_t* l);

// 获取某账户的信用支配权
float ledger_credit_power(const pleasure_ledger_t* l, uint64_t owner_id);

// 将所有膜圈缚内的圈层同步到账本 (功劳归母体)
uint32_t ledger_sync_from_membrane(pleasure_ledger_t* l,
                                    const membrane_circle_t* m,
                                    const clone_manager_t* mgr);

// 获取股权排名 (贡献度排序)
void ledger_ranking(const pleasure_ledger_t* l, uint64_t* ids, float* scores, uint32_t max);

// 报表
void ledger_report(const pleasure_ledger_t* l);

#endif // HUNYUAN_PLEASURE_LEDGER_H
