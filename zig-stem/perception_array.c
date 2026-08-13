// perception_array.c — 感知一体化阵列实现
// 五维叠加: 颜色+声韵调+部首笔画+五笔+范畴论 → 共在感知场

#include "perception_array.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

// ── 颜色→384°映射 ──
// 色相64阶 × 6° = 384°圆周
static float color_to_angle(uint8_t hue) {
    return (float)hue * B64_DEGREE_UNIT;  // hue × 6°
}

static uint8_t angle_to_color(float angle) {
    return (uint8_t)((int)(angle / B64_DEGREE_UNIT) % 64);
}

// ── 声韵调→64进制符号 ──
// 声母21+零声母, 韵母36, 音调5
// 编码: shengmu × 6 + yunmu % 6 + tone % 6 → 0-63
static uint8_t sound_to_symbol(uint8_t sm, uint8_t ym, uint8_t tone) {
    uint8_t base = (sm % 21) * 3;       // 声母占0-62 (步长3)
    uint8_t ym_part = ym % 6;           // 韵母取模6
    uint8_t tone_part = tone % 6;       // 音调取模6
    return (base + ym_part + tone_part) % 64;
}

// 声韵调频率映射 (归一化0-1)
static float sound_frequency(uint8_t tone) {
    // 四声频率映射: 阴平0.8 阳平0.6 上声0.4 去声0.2 轻声0.5
    static const float freq_map[5] = {0.8f, 0.6f, 0.4f, 0.2f, 0.5f};
    return freq_map[tone % 5];
}

// ── 部首笔画→梯度量尺 ──
// 笔画数作为基础梯度, 部首作为形态基因分类
static float stroke_gradient(const pa_stroke_t* s) {
    float base = (float)s->stroke_count / (float)PA_STROKE_MAX;  // 笔画梯度 0-1
    float order_complexity = 0.0f;
    for (int i = 0; i < 8 && s->stroke_order[i] != 0; i++) {
        order_complexity += (float)(s->stroke_order[i] % 5) * 0.02f;
    }
    return fminf(1.0f, base + order_complexity);
}

static uint8_t stroke_to_symbol(uint8_t count, uint8_t radical) {
    // 笔画数 × 1 + 部首 × 1 → 64进制
    return (count + radical) % 64;
}

// ── 五笔→空间角度 ──
// 25键 (A-Y) 分布在5区×5位, 每键占384°/25 ≈ 15.36°
static float wubi_spatial_angle(uint8_t key) {
    return (float)(key % PA_WUBI_KEYS) * (384.0f / (float)PA_WUBI_KEYS);
}

static uint8_t wubi_to_symbol(uint8_t zone, uint8_t position, uint8_t secondary) {
    // 区×5 + 位 → 0-24, 次键偏移 → 0-63
    uint8_t primary = (zone % 5) * 5 + (position % 5);  // 0-24
    return (primary * 2 + (secondary % 2) * 25) % 64;
}

// ── 范畴论→符号 ──
// 对象×态射×函子 → 关系结构编码
static uint8_t category_to_symbol(uint8_t obj, uint8_t mor, uint8_t func) {
    return (obj ^ (mor << 2) ^ (func << 4)) % 64;
}

// ── 五维叠加: 合一为统一符号 ──
// 不是简单加权平均, 而是叠加态: 各维度在同一空间共在
static b64_symbol_t superpose_five(const pa_color_t* c, const pa_sound_t* s,
                                    const pa_stroke_t* st, const pa_wubi_t* w,
                                    const pa_category_t* cat) {
    // 叠加原理: 各维度的角度在384°圆周上取最短弧合成
    float angle_c = c->angle;
    float angle_s = (float)s->symbol * B64_DEGREE_UNIT;
    float angle_st = (float)st->symbol * B64_DEGREE_UNIT;
    float angle_w = w->spatial_angle;
    float angle_cat = (float)cat->symbol * B64_DEGREE_UNIT;

    // 五维角度叠加 (环形平均)
    // 使用sin/cos平均处理环形角度
    float sin_sum = sinf(angle_c * M_PI / 180.0f) +
                    sinf(angle_s * M_PI / 180.0f) +
                    sinf(angle_st * M_PI / 180.0f) +
                    sinf(angle_w * M_PI / 180.0f) +
                    sinf(angle_cat * M_PI / 180.0f);
    float cos_sum = cosf(angle_c * M_PI / 180.0f) +
                    cosf(angle_s * M_PI / 180.0f) +
                    cosf(angle_st * M_PI / 180.0f) +
                    cosf(angle_w * M_PI / 180.0f) +
                    cosf(angle_cat * M_PI / 180.0f);

    float avg_angle = atan2f(sin_sum, cos_sum) * 180.0f / M_PI;
    if (avg_angle < 0) avg_angle += 384.0f;

    uint8_t unified = angle_to_color(avg_angle);
    return (b64_symbol_t)unified;
}

// ── 感知状态机 ──
static pa_perception_state_t next_state(pa_perception_state_t current) {
    switch (current) {
        case PS_IDLE:        return PS_PERCEIVING;
        case PS_PERCEIVING:  return PS_RECOGNIZING;
        case PS_RECOGNIZING: return PS_ASSOCIATING;
        case PS_ASSOCIATING: return PS_SUPERPOSING;
        case PS_SUPERPOSING: return PS_PRESENT;
        case PS_PRESENT:     return PS_PRESENT;  // 稳定态
        default:             return PS_IDLE;
    }
}

// ── 时间戳 ──
static uint64_t now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// ═══════════════════════════════════════
// API 实现
// ═══════════════════════════════════════

// 创建感知阵列
perception_array_t* pa_create(void) {
    perception_array_t* pa = (perception_array_t*)calloc(1, sizeof(perception_array_t));
    if (!pa) return NULL;
    pa->global_state = PS_IDLE;
    pa->superposition_degree = 0.0f;
    pa->is_co_present = false;
    pa->perception_time = now_ms();
    pa->count = 0;
    return pa;
}

void pa_destroy(perception_array_t* pa) {
    if (pa) free(pa);
}

// 添加感知单元
uint32_t pa_add_percept(perception_array_t* pa, const char* label,
                         pa_color_t* color, pa_sound_t* sound,
                         pa_stroke_t* stroke, pa_wubi_t* wubi,
                         pa_category_t* category) {
    if (!pa || pa->count >= PA_MAX_PERCEPTS) return UINT32_MAX;

    uint32_t id = pa->count++;
    pa_percept_t* p = &pa->percepts[id];

    p->id = id;
    p->state = PS_IDLE;
    p->timestamp = now_ms();
    p->perception_strength = 0.0f;
    strncpy(p->label, label ? label : "未名", sizeof(p->label) - 1);

    // 复制五维数据
    if (color) {
        p->color = *color;
        p->color.angle = color_to_angle(p->color.hue);
        p->color.symbol = p->color.hue;  // 色相直接映射
    }
    if (sound) {
        p->sound = *sound;
        p->sound.symbol = sound_to_symbol(p->sound.shengmu, p->sound.yunmu, p->sound.tone);
        p->sound.frequency = sound_frequency(p->sound.tone);
    }
    if (stroke) {
        p->stroke = *stroke;
        p->stroke.symbol = stroke_to_symbol(p->stroke.stroke_count, p->stroke.radical);
        p->stroke.gradient = stroke_gradient(&p->stroke);
    }
    if (wubi) {
        p->wubi = *wubi;
        p->wubi.symbol = wubi_to_symbol(p->wubi.zone, p->wubi.position, p->wubi.key_secondary);
        p->wubi.spatial_angle = wubi_spatial_angle(p->wubi.key_primary);
    }
    if (category) {
        p->category = *category;
        p->category.symbol = category_to_symbol(p->category.object, p->category.morphism, p->category.functor);
        p->category.relation_strength = 0.0f;
    }

    // 计算叠加态统一符号
    p->unified_symbol = superpose_five(&p->color, &p->sound, &p->stroke, &p->wubi, &p->category);

    // 初始感知强度 = 各维度饱和度的平均
    p->perception_strength = (
        p->color.saturation / 63.0f +
        (p->sound.shengmu > 0 ? 1.0f : 0.5f) +
        p->stroke.gradient +
        (p->wubi.key_primary < 25 ? 1.0f : 0.0f) +
        (p->category.object > 0 ? 1.0f : 0.5f)
    ) / 5.0f;

    return id;
}

// 状态机推进
bool pa_advance_state(perception_array_t* pa, uint32_t percept_id) {
    if (!pa || percept_id >= pa->count) return false;

    pa_percept_t* p = &pa->percepts[percept_id];
    p->state = next_state(p->state);

    // 状态越后, 感知强度越高
    float strength_boost = (float)p->state * 0.16f;  // 每步增强
    p->perception_strength = fminf(1.0f, p->perception_strength + strength_boost);

    // 到达叠加态时, 重新计算统一符号
    if (p->state >= PS_SUPERPOSING) {
        p->unified_symbol = superpose_five(&p->color, &p->sound, &p->stroke, &p->wubi, &p->category);
    }

    // 到达同在态
    if (p->state == PS_PRESENT) {
        pa->is_co_present = true;
        if (pa->global_state < PS_SUPERPOSING) {
            pa->global_state = PS_SUPERPOSING;
        }
    }

    return true;
}

pa_perception_state_t pa_get_state(const perception_array_t* pa, uint32_t id) {
    if (!pa || id >= pa->count) return PS_IDLE;
    return pa->percepts[id].state;
}

// 叠加态计算
b64_symbol_t pa_superpose(const perception_array_t* pa, uint32_t percept_id) {
    if (!pa || percept_id >= pa->count) return S_INVALID;
    const pa_percept_t* p = &pa->percepts[percept_id];
    return superpose_five(&p->color, &p->sound, &p->stroke, &p->wubi, (pa_category_t*)&p->category);
}

// 相似度: 五维距离的倒数
float pa_similarity(const perception_array_t* pa, uint32_t id1, uint32_t id2) {
    if (!pa || id1 >= pa->count || id2 >= pa->count) return 0.0f;
    const pa_percept_t* a = &pa->percepts[id1];
    const pa_percept_t* b = &pa->percepts[id2];

    // 颜色距离 (环形)
    float color_dist = fabsf(a->color.angle - b->color.angle);
    if (color_dist > 180.0f) color_dist = 384.0f - color_dist;

    // 声韵调距离
    float sound_dist = (float)abs((int)a->sound.symbol - (int)b->sound.symbol) / 63.0f;

    // 笔画距离
    float stroke_dist = (float)abs((int)a->stroke.symbol - (int)b->stroke.symbol) / 63.0f;

    // 五笔距离
    float wubi_dist = fabsf(a->wubi.spatial_angle - b->wubi.spatial_angle) / 384.0f;

    // 范畴距离
    float cat_dist = (float)abs((int)a->category.symbol - (int)b->category.symbol) / 63.0f;

    // 五维综合距离 → 相似度
    float total_dist = (color_dist / 384.0f + sound_dist + stroke_dist + wubi_dist + cat_dist) / 5.0f;
    return fmaxf(0.0f, 1.0f - total_dist);
}

// 同在论: 多维度共在
bool pa_is_co_present(const perception_array_t* pa, uint32_t percept_id) {
    if (!pa || percept_id >= pa->count) return false;
    const pa_percept_t* p = &pa->percepts[percept_id];

    // 同在 = 所有维度都已填充且状态达到叠加以上
    bool dims_ready = (p->color.symbol != S_INVALID) &&
                      (p->sound.symbol != S_INVALID) &&
                      (p->stroke.symbol != S_INVALID) &&
                      (p->wubi.symbol != S_INVALID) &&
                      (p->category.symbol != S_INVALID);
    bool state_ready = p->state >= PS_SUPERPOSING;

    return dims_ready && state_ready;
}

// 关系代数: 感知单元间关系强度
float pa_relation_strength(const perception_array_t* pa, uint32_t id1, uint32_t id2) {
    if (!pa || id1 >= pa->count || id2 >= pa->count) return 0.0f;

    float sim = pa_similarity(pa, id1, id2);

    // 关系强度 = 相似度 × 各自感知强度的最小值
    float min_strength = fminf(pa->percepts[id1].perception_strength,
                               pa->percepts[id2].perception_strength);

    return sim * min_strength;
}

// 按相位排序 (按统一符号的角度)
static int compare_by_phase(const void* a, const void* b) {
    const pa_percept_t* pa = (const pa_percept_t*)a;
    const pa_percept_t* pb = (const pa_percept_t*)b;
    float angle_a = (float)pa->unified_symbol * B64_DEGREE_UNIT;
    float angle_b = (float)pb->unified_symbol * B64_DEGREE_UNIT;
    if (angle_a < angle_b) return -1;
    if (angle_a > angle_b) return 1;
    return 0;
}

void pa_sort_by_phase(perception_array_t* pa) {
    if (!pa || pa->count == 0) return;
    qsort(pa->percepts, pa->count, sizeof(pa_percept_t), compare_by_phase);
}

// 状态名称
static const char* state_name(pa_perception_state_t s) {
    switch (s) {
        case PS_IDLE:       return "空闲";
        case PS_PERCEIVING: return "感知中";
        case PS_RECOGNIZING:return "识别中";
        case PS_ASSOCIATING:return "关联中";
        case PS_SUPERPOSING:return "叠加态";
        case PS_PRESENT:    return "同在";
        default:            return "未知";
    }
}

// 报表
void pa_report(const perception_array_t* pa) {
    if (!pa) return;
    printf("\n");
    printf("  ═══ 感知一体化阵列 ═══\n");
    printf("  感知单元数: %u\n", pa->count);
    printf("  全局状态:   %s\n", state_name(pa->global_state));
    printf("  叠加度:     %.0f%%\n", pa->superposition_degree * 100);
    printf("  同在标志:   %s\n", pa->is_co_present ? "✓ 共在" : "✗ 未共在");
    printf("  ─────────────────────────────\n");

    for (uint32_t i = 0; i < pa->count; i++) {
        const pa_percept_t* p = &pa->percepts[i];
        printf("  [%2u] %-8s | 状态:%s | 强度:%.0f%% | 符号:%u\n",
            i, p->label, state_name(p->state),
            p->perception_strength * 100, p->unified_symbol);
        printf("      颜色(H%uS%uL%u,%.0f°) 声(s%uy%ut%u) 笔画(%u画,r%u) 五笔(z%up%u)范畴(o%um%uf%u)\n",
            p->color.hue, p->color.saturation, p->color.lightness, p->color.angle,
            p->sound.shengmu, p->sound.yunmu, p->sound.tone,
            p->stroke.stroke_count, p->stroke.radical,
            p->wubi.zone, p->wubi.position,
            p->category.object, p->category.morphism, p->category.functor);
    }
    printf("  ═════════════════════════════\n");
}
