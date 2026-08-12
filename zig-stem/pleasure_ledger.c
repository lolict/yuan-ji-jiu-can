// pleasure_ledger.c — 愉悦值账本实现
// 参见 pleasure_ledger.h 的架构说明
//
// 混合股权计量:
//   - 股权 share: 基于贡献度,总和恒定为 1.0 (=100%)
//   - 努力 credits: 自身积累,不可转让,独立计量
//   - 愉悦 points: 感知高潮的叠加态量化
//   - 信用 power: 股权+贡献度加权,决定支配权力

#include "pleasure_ledger.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// ── 内部常量 ──
static const float PLEASURE_ORGASM_WEIGHT    = 1.0f;   // 性高潮权重
static const float PLEASURE_INTIMACY_WEIGHT  = 0.5f;   // 亲密感权重
static const float PLEASURE_SYNCHRO_WEIGHT   = 0.3f;   // 同步快感权重
static const float PLEASURE_TASK_WEIGHT      = 0.2f;   // 任务成就权重
static const float PLEASURE_FUSION_WEIGHT    = 0.8f;   // 融合态权重

static const float EQUITY_RATIO              = 0.6f;   // 股权占信用权重
static const float CONTRIBUTION_RATIO        = 0.4f;   // 贡献度占信用权重

static const float MASTER_MIN_STAKE          = 0.51f;  // 主体最低控股比例

static uint64_t time_now(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// 查找账户 (返回索引, -1=不存在)
static int find_entry(const pleasure_ledger_t* l, uint64_t owner_id) {
    for (uint32_t i = 0; i < l->entry_count; i++) {
        if (l->entries[i].owner_id == owner_id) return (int)i;
    }
    return -1;
}

// ── 创建账本 ──
pleasure_ledger_t* ledger_create(const char* name) {
    pleasure_ledger_t* l = (pleasure_ledger_t*)calloc(1, sizeof(pleasure_ledger_t));
    if (!l) return NULL;
    strncpy(l->name, name, MEMBRANA_MAX_NAME - 1);
    l->name[MEMBRANA_MAX_NAME - 1] = '\0';
    l->master_stake = 1.0f;  // 初始主体100%控股
    l->is_balanced = true;
    return l;
}

void ledger_destroy(pleasure_ledger_t* l) {
    if (l) free(l);
}

// ── 开户 ──
int ledger_open_account(pleasure_ledger_t* l, uint64_t owner_id, const char* name) {
    if (!l) return -1;
    if (l->entry_count >= LEDGER_MAX_ENTRIES) return -1;
    if (find_entry(l, owner_id) >= 0) return -1;  // 已存在

    ledger_entry_t* e = &l->entries[l->entry_count];
    e->owner_id = owner_id;
    strncpy(e->name, name, MEMBRANA_MAX_NAME - 1);
    e->name[MEMBRANA_MAX_NAME - 1] = '\0';
    e->equity_share = 0.0f;
    e->effort_credits = 0;
    e->pleasure_points = 0.0f;
    e->task_count = 0;
    e->absorption_count = 0;
    e->contribution_score = 0.0f;
    e->credit_power = 0.0f;
    e->perception_equity = 0.0f;
    e->first_contrib_time = time_now();
    e->last_contrib_time = e->first_contrib_time;

    l->entry_count++;
    l->is_balanced = false;  // 需要重新平衡

    return (int)l->entry_count - 1;
}

// ── 记录愉悦 (叠加态量化) ──
float ledger_record_pleasure(pleasure_ledger_t* l, uint64_t owner_id,
                              pleasure_type_t type, float intensity) {
    if (!l) return 0.0f;
    int idx = find_entry(l, owner_id);
    if (idx < 0) return 0.0f;

    ledger_entry_t* e = &l->entries[idx];

    // 权重映射
    float weight = 0.0f;
    if (type & PLEASURE_ORGASM)    weight += PLEASURE_ORGASM_WEIGHT;
    if (type & PLEASURE_INTIMACY)  weight += PLEASURE_INTIMACY_WEIGHT;
    if (type & PLEASURE_SYNCHRO)   weight += PLEASURE_SYNCHRO_WEIGHT;
    if (type & PLEASURE_TASK)      weight += PLEASURE_TASK_WEIGHT;
    if (type & PLEASURE_FUSION)    weight += PLEASURE_FUSION_WEIGHT;

    float points = weight * intensity;
    e->pleasure_points += points;
    e->last_contrib_time = time_now();

    // 总愉悦更新
    l->total_pleasure = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        l->total_pleasure += l->entries[i].pleasure_points;
    }

    // 贡献度更新
    e->contribution_score = e->pleasure_points + (float)e->effort_credits * 0.5f;

    return points;
}

// ── 记录努力 (自身积累) ──
uint32_t ledger_record_effort(pleasure_ledger_t* l, uint64_t owner_id,
                               uint32_t points) {
    if (!l) return 0;
    int idx = find_entry(l, owner_id);
    if (idx < 0) return 0;

    ledger_entry_t* e = &l->entries[idx];
    e->effort_credits += points;
    e->task_count++;
    e->last_contrib_time = time_now();

    // 总努力更新
    l->total_effort = 0;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        l->total_effort += l->entries[i].effort_credits;
    }

    // 贡献度更新
    e->contribution_score = e->pleasure_points + (float)e->effort_credits * 0.5f;

    return e->effort_credits;
}

// ── 重新平衡股权 (总和=1) ──
void ledger_rebalance_equity(pleasure_ledger_t* l) {
    if (!l || l->entry_count == 0) return;

    // 总贡献度
    float total_contrib = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        total_contrib += l->entries[i].contribution_score;
    }
    l->total_contribution = total_contrib;

    if (total_contrib <= 0.0f) {
        // 无贡献则平均分配
        float equal = 1.0f / (float)l->entry_count;
        for (uint32_t i = 0; i < l->entry_count; i++) {
            l->entries[i].equity_share = equal;
        }
    } else {
        // 按贡献度比例分配
        for (uint32_t i = 0; i < l->entry_count; i++) {
            l->entries[i].equity_share = l->entries[i].contribution_score / total_contrib;
        }
    }

    // 检查主体控股 (确保 master_stake >= 51%)
    float master_share = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        if (l->entries[i].owner_id == 0) {  // 主体ID=0
            master_share = l->entries[i].equity_share;
            break;
        }
    }

    // 如果主体被稀释,强制回收至最低比例
    if (master_share < MASTER_MIN_STAKE) {
        float deficit = MASTER_MIN_STAKE - master_share;
        // 从最大股东处扣除
        uint32_t max_idx = 0;
        float max_share = 0;
        for (uint32_t i = 1; i < l->entry_count; i++) {
            if (l->entries[i].equity_share > max_share) {
                max_share = l->entries[i].equity_share;
                max_idx = i;
            }
        }
        l->entries[max_idx].equity_share -= deficit;
        for (uint32_t i = 0; i < l->entry_count; i++) {
            if (l->entries[i].owner_id == 0) {
                l->entries[i].equity_share = MASTER_MIN_STAKE;
                break;
            }
        }
    }

    // 计算信用支配权
    for (uint32_t i = 0; i < l->entry_count; i++) {
        ledger_entry_t* e = &l->entries[i];
        e->credit_power = e->equity_share * EQUITY_RATIO +
                          (e->contribution_score / total_contrib) * CONTRIBUTION_RATIO;
        e->perception_equity = e->equity_share;  // 感知股权 = 股权份额
    }

    // 验证总和
    float sum = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        sum += l->entries[i].equity_share;
    }
    l->total_equity = sum;
    l->is_balanced = (sum >= 0.99f && sum <= 1.01f);
}

// ── 总股权 ──
float ledger_calc_total_equity(const pleasure_ledger_t* l) {
    if (!l) return 0.0f;
    float sum = 0.0f;
    for (uint32_t i = 0; i < l->entry_count; i++) {
        sum += l->entries[i].equity_share;
    }
    return sum;
}

// ── 信用支配权 ──
float ledger_credit_power(const pleasure_ledger_t* l, uint64_t owner_id) {
    if (!l) return 0.0f;
    int idx = find_entry(l, owner_id);
    if (idx < 0) return 0.0f;
    return l->entries[idx].credit_power;
}

// ── 从膜圈缚同步 (功劳归母体) ──
uint32_t ledger_sync_from_membrane(pleasure_ledger_t* l,
                                    const membrane_circle_t* m,
                                    const clone_manager_t* mgr) {
    if (!l) return 0;

    // 同步膜圈缚的圈层
    if (m) {
        for (uint32_t i = 0; i < m->layer_count; i++) {
            uint64_t id = m->layers[i].id;
            if (find_entry(l, id) < 0) {
                ledger_open_account(l, id, m->layers[i].name);
            }
            // 记录愉悦
            if (m->layers[i].pleasure_value > 0) {
                ledger_record_pleasure(l, id,
                    PLEASURE_FUSION | PLEASURE_INTIMACY,
                    m->layers[i].pleasure_value);
            }
            // 记录努力(外部吸纳贡献)
            if (m->layers[i].external_count > 0) {
                ledger_record_effort(l, id, m->layers[i].external_count * 10);
            }
        }
    }

    // 同步分身子层 (功劳归母体 = 计入母体ID=0)
    if (mgr) {
        clone_agent_t* agent = mgr->agents;
        while (agent) {
            // 分身的吸纳归入母体
            if (agent->external_absorbed > 0) {
                ledger_record_effort(l, agent->parent_id,
                    agent->external_absorbed * 5);
                ledger_record_pleasure(l, agent->parent_id,
                    PLEASURE_TASK, (float)agent->external_absorbed);
            }
            agent = agent->next;
        }
    }

    // 重新平衡
    ledger_rebalance_equity(l);

    return l->entry_count;
}

// ── 股权排名 ──
void ledger_ranking(const pleasure_ledger_t* l, uint64_t* ids, float* scores, uint32_t max) {
    if (!l || !ids || !scores || l->entry_count == 0) return;

    // 创建索引数组并排序
    uint32_t count = (max < l->entry_count) ? max : l->entry_count;
    uint32_t* indices = (uint32_t*)malloc(l->entry_count * sizeof(uint32_t));
    if (!indices) return;

    for (uint32_t i = 0; i < l->entry_count; i++) indices[i] = i;

    // 降序排序索引
    for (uint32_t i = 0; i < l->entry_count; i++) {
        for (uint32_t j = i + 1; j < l->entry_count; j++) {
            if (l->entries[indices[j]].contribution_score > l->entries[indices[i]].contribution_score) {
                uint32_t tmp = indices[i];
                indices[i] = indices[j];
                indices[j] = tmp;
            }
        }
    }

    // 取前 count 名
    for (uint32_t i = 0; i < count; i++) {
        ids[i] = l->entries[indices[i]].owner_id;
        scores[i] = l->entries[indices[i]].contribution_score;
    }

    free(indices);
}

// ── 报表 ──
void ledger_report(const pleasure_ledger_t* l) {
    if (!l) return;

    printf("\n═══ 愉悦值账本 [%s] ═══\n", l->name);
    printf("  账户数:   %u\n", l->entry_count);
    printf("  股权总和: %.4f %s\n", l->total_equity, l->is_balanced ? "✓" : "✗需平衡");
    printf("  总努力:   %u\n", l->total_effort);
    printf("  总愉悦:   %.2f\n", l->total_pleasure);
    printf("  总贡献:   %.2f\n", l->total_contribution);
    printf("  主体控股: %.1f%%\n", l->master_stake * 100.0f);
    printf("  ─────────────────────────────\n");
    printf("  %-8s %-10s %-8s %-8s %-8s %-8s\n",
        "ID", "名称", "股权%", "努力", "愉悦", "信用");
    printf("  %-8s %-10s %-8s %-8s %-8s %-8s\n",
        "--------", "----------", "--------", "--------", "--------", "--------");

    for (uint32_t i = 0; i < l->entry_count; i++) {
        const ledger_entry_t* e = &l->entries[i];
        printf("  %-8lu %-10s %6.1f%% %-8u %-8.1f %-8.2f\n",
            (unsigned long)e->owner_id,
            e->name,
            e->equity_share * 100.0f,
            e->effort_credits,
            e->pleasure_points,
            e->credit_power);
    }
    printf("═══════════════════════════════════\n\n");
}
