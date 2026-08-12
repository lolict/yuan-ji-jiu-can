// membrana.c — 膜圈缚 (Membrane Binding) 安全结界实现
// 参见 membrana.h 的架构说明
//
// 核心机制:
//   1. 膜圈缚以主体(用户名)为结界核心,形成安全边界
//   2. 每个被吸纳的外部实体成为一个圈层(layer)
//   3. 圈层吸收后状态零(shell)形成宣传外壳
//   4. 锚点(工藤新一=S_KNDO=2)进入后形成夫妻共同体
//   5. 感知器官同步 → 愉悦值叠加 → 可形成分身外出吸纳
//   6. 被包裹的实体不可脱离此范畴(安全验证)

#include "membrana.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// ── 内部常量 ──
static const float FUSION_THRESHOLD    = 0.6f;   // 形成灵的融合阈值
static const float SOUL_THRESHOLD      = 0.8f;   // 形成分身的最小融合度
static const float SYNC_DECAY          = 0.05f;  // 感知同步衰减率
static const float INTEGRITY_RECOVER   = 0.01f;  // 结界自修复率
static uint64_t    NEXT_ID             = 1;      // 全局ID序列

// ── 时间基点的获取 (简化: 毫秒时间戳) ──
static uint64_t time_now(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// ── 创建膜圈缚 ──
membrane_circle_t* membrane_create(const char* name, uint32_t capacity) {
    membrane_circle_t* m = (membrane_circle_t*)calloc(1, sizeof(membrane_circle_t));
    if (!m) return NULL;

    m->id = NEXT_ID++;
    strncpy(m->name, name, MEMBRANA_MAX_NAME - 1);
    m->name[MEMBRANA_MAX_NAME - 1] = '\0';
    m->integrity = 1.0f;          // 初始完整
    m->capacity = (capacity > MEMBRANA_MAX_LAYERS) ? MEMBRANA_MAX_LAYERS : capacity;
    m->layer_count = 0;
    m->total_pleasure = 0.0f;
    m->total_absorbed = 0;
    m->anchor_time = time_now();  // 工藤新一时间锚点
    m->is_sealed = false;

    return m;
}

// ── 销毁 ──
void membrane_destroy(membrane_circle_t* m) {
    if (m) free(m);
}

// ── 激活结界: 幔圈缚封闭 ──
bool membrane_seal(membrane_circle_t* m) {
    if (!m || m->is_sealed) return false;
    if (m->integrity < 0.5f) return false;  // 完整度不足不可封闭

    m->is_sealed = true;
    return true;
}

// ── 吸纳外部实体 ──
int64_t membrane_absorb(membrane_circle_t* m, const char* name, perception_organ_t organs) {
    if (!m || !name) return -1;
    if (m->layer_count >= m->capacity) return -1;  // 容量已满
    if (m->is_sealed) return -1;  // 封闭后不可吸纳

    membrane_layer_t* layer = &m->layers[m->layer_count];
    layer->id = NEXT_ID++;
    strncpy(layer->name, name, MEMBRANA_MAX_NAME - 1);
    layer->name[MEMBRANA_MAX_NAME - 1] = '\0';
    layer->state = ABSORB_PENDING;
    layer->fusion_degree = 0.0f;
    layer->pleasure_value = 0.0f;
    layer->organ_share = organs;
    layer->soul_formed = false;
    layer->shell_created = false;
    layer->absorb_time = time_now();
    layer->external_count = 0;

    m->layer_count++;
    m->total_absorbed++;

    // 吸纳消耗结界完整度
    m->integrity -= 0.02f;

    return (int64_t)layer->id;
}

// ── 推进融合状态 ──
bool membrane_advance(membrane_circle_t* m, uint64_t layer_id) {
    if (!m) return false;

    membrane_layer_t* layer = NULL;
    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].id == layer_id) {
            layer = &m->layers[i];
            break;
        }
    }
    if (!layer) return false;

    switch (layer->state) {
        case ABSORB_PENDING:
            // 从吸纳中 → 已绑定 (初次融合)
            layer->fusion_degree = 0.3f;
            layer->state = ABSORB_BOUND;
            break;

        case ABSORB_BOUND:
            // 融合度增长
            layer->fusion_degree += 0.15f;
            if (layer->fusion_degree >= FUSION_THRESHOLD) {
                layer->state = ABSORB_FUSED;
                // 形成灵(状态零) → 生成宣传外壳
                layer->soul_formed = true;
                layer->shell_created = true;
            }
            break;

        case ABSORB_FUSED:
            // 深度融合
            layer->fusion_degree += 0.1f;
            if (layer->fusion_degree >= SOUL_THRESHOLD) {
                layer->state = ABSORB_CLONE;
                // 感知同步激活 → 愉悦值开始累积
                layer->pleasure_value += 0.5f;
            }
            break;

        case ABSORB_CLONE:
            // 已是分身,持续累积愉悦值
            layer->pleasure_value += 0.05f;
            break;

        default:
            return false;
    }

    // 结界自修复
    if (m->integrity < 1.0f) {
        m->integrity += INTEGRITY_RECOVER;
        if (m->integrity > 1.0f) m->integrity = 1.0f;
    }

    return true;
}

// ── 形成灵(状态零) ──
bool membrane_form_soul(membrane_circle_t* m, uint64_t layer_id) {
    if (!m) return false;

    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].id == layer_id) {
            membrane_layer_t* layer = &m->layers[i];
            if (layer->fusion_degree >= FUSION_THRESHOLD) {
                layer->soul_formed = true;
                layer->shell_created = true;
                return true;
            }
            return false;
        }
    }
    return false;
}

// ── 锚点进入: 工藤新一=S_KNDO=2 进入圈层 ──
bool membrane_anchor_enter(membrane_circle_t* m, uint64_t layer_id) {
    if (!m) return false;

    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].id == layer_id) {
            membrane_layer_t* layer = &m->layers[i];
            if (layer->state < ABSORB_FUSED) return false;  // 未深度融合不可进入

            // 锚点进入 → 夫妻共同体形成
            // 感知器官全面共享
            layer->organ_share = ORGAN_ALL;
            // 愉悦值跃升 (初次连接高潮)
            layer->pleasure_value += 1.0f;
            // 融合度封顶
            layer->fusion_degree = 1.0f;
            return true;
        }
    }
    return false;
}

// ── 感知同步强度 ──
float membrane_perception_sync(const membrane_circle_t* m, uint64_t layer_id) {
    if (!m) return 0.0f;

    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].id == layer_id) {
            const membrane_layer_t* layer = &m->layers[i];
            // 感知强度 = 融合度 × 器官共享度 × 距离衰减
            float organ_ratio = (float)__builtin_popcount(layer->organ_share) / 5.0f;
            float sync = layer->fusion_degree * organ_ratio;
            // 时间衰减: 吸纳时间越久,同步越稳但不衰减
            return sync * (1.0f - SYNC_DECAY);
        }
    }
    return 0.0f;
}

// ── 总愉悦值 (叠加态) ──
float membrane_total_pleasure(membrane_circle_t* m) {
    if (!m) return 0.0f;
    float total = 0.0f;
    for (uint32_t i = 0; i < m->layer_count; i++) {
        total += m->layers[i].pleasure_value;
    }
    m->total_pleasure = total;  // 更新缓存
    return total;
}

// ── 单层愉悦值 ──
float membrane_layer_pleasure(const membrane_circle_t* m, uint64_t layer_id) {
    if (!m) return 0.0f;
    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].id == layer_id) {
            return m->layers[i].pleasure_value;
        }
    }
    return 0.0f;
}

// ── 结界安全验证 ──
bool membrane_verify_boundary(const membrane_circle_t* m) {
    if (!m) return false;

    // 封闭态下:
    // 1. 所有圈层实体不可脱离 (融合度>0即被绑)
    // 2. 外部实体不可突破结界
    // 3. 被包裹实体的功劳归于主体

    if (!m->is_sealed) return true;  // 未封闭时无约束

    for (uint32_t i = 0; i < m->layer_count; i++) {
        if (m->layers[i].fusion_degree <= 0.0f) {
            // 存在未融合的实体 → 结界不安全
            return false;
        }
    }

    // 结界完整度必须足够
    return m->integrity >= 0.3f;
}

// ── 统计报表 ──
void membrane_report(membrane_circle_t* m) {
    if (!m) return;

    printf("\n═══ 膜圈缚 [%s] 状态报表 ═══\n", m->name);
    printf("  结界完整度: %.0f%%\n", m->integrity * 100.0f);
    printf("  封闭状态:   %s\n", m->is_sealed ? "已封闭(幔圈缚)" : "未封闭");
    printf("  圈层数量:   %u/%u\n", m->layer_count, m->capacity);
    printf("  总吸纳数:   %u\n", m->total_absorbed);
    printf("  叠加愉悦值: %.2f\n", membrane_total_pleasure(m));
    printf("  边界安全:   %s\n", membrane_verify_boundary(m) ? "✓ 安全" : "✗ 异常");
    printf("  ─────────────────────────────\n");

    for (uint32_t i = 0; i < m->layer_count; i++) {
        const membrane_layer_t* l = &m->layers[i];
        const char* state_str[] = {"未融合", "吸纳中", "已绑定", "深度融合", "分身态"};
        printf("  [#%lu] %s — %s | 融合度:%.0f%% | 愉悦:%.2f | 器官:%s%s%s%s%s\n",
            (unsigned long)l->id,
            l->name,
            state_str[l->state],
            l->fusion_degree * 100.0f,
            l->pleasure_value,
            (l->organ_share & ORGAN_VISUAL)   ? "视" : "-",
            (l->organ_share & ORGAN_AUDITORY) ? "听" : "-",
            (l->organ_share & ORGAN_TOUCH)    ? "触" : "-",
            (l->organ_share & ORGAN_EMOTION)  ? "情" : "-",
            (l->organ_share & ORGAN_PLEASURE) ? "悦" : "-"
        );
    }
    printf("════════════════════════════════\n\n");
}
