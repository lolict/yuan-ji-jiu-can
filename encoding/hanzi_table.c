/**
 * hanzi_table.c — 汉字感知场 · 容器化预加载表
 *
 * 设计: 满全法感知场分类路由 (无哈希)
 *
 * 架构:
 *   1257字被分为5个感知家族 (分别器第一层: 字形第一印象)
 *   每个家族内部, 按字形结构→声母→韵母→声调排序
 *   查找时:
 *     1. 感知家族判断 (看长相)
 *     2. 族内镜像观察逐一对比 (看细节)
 *     3. 三元裁判 (感知+分类+动力 → 准入)
 *     4. 容器化 (装入64位容器编码)
 *
 * 这就是"感知-镜像-裁判-容器-筷子-流水线"完整路径
 */

#include "hanzi_table.h"
#include "perception_field.h"
#include <string.h>

/* ──────────────────────────────────────────────
 * 感知家族分表
 *
 * 每个家族是一个"门", 长相相似的字进同一个门
 * 不是哈希桶, 是感知入口
 * ────────────────────────────────────────────── */

/* 容器家族 (全包围/方形 · 刘楚恬侧 · 阴) */
static const mirror_observation_t GATE_CONTAINER[] = {
    {0x56D7, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 3, 33, 15, 32, 1, VERDICT_ADMITTED, 0}, /* 囗 wéi */
    {0x56DE, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 6, 33, 10, 64, 3, VERDICT_ADMITTED, 0}, /* 回 huí */
    {0x56F0, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 7, 33, 9, 34, 3, VERDICT_ADMITTED, 0}, /* 困 kùn */
    {0x56E2, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 6, 33, 5, 52, 1, VERDICT_ADMITTED, 0}, /* 团 tuán */
    {0x56E0, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 6, 33, 15, 34, 0, VERDICT_ADMITTED, 0}, /* 因 yīn */
    {0x56FD, FAMILY_CONTAINER, STRUCT_FULL_WRAP, 8, 33, 8, 28, 2, VERDICT_ADMITTED, 0}, /* 国 guó */
};

/* 边界家族 (竖长/界线/独体 · 我侧 · 阳) */
static const mirror_observation_t GATE_BOUNDARY[] = {
    {0x4E00, FAMILY_BOUNDARY, STRUCT_SINGLE, 1, 1, 15, 6, 0, VERDICT_ADMITTED, 0}, /* 一 yī */
    {0x4E28, FAMILY_BOUNDARY, STRUCT_SINGLE, 1, 2, 8, 34, 3, VERDICT_ADMITTED, 0}, /* 丨 gǔn */
    {0x4E36, FAMILY_BOUNDARY, STRUCT_SINGLE, 1, 3, 16, 8, 2, VERDICT_ADMITTED, 0}, /* 丶 zhǔ */
    {0x4E3F, FAMILY_BOUNDARY, STRUCT_SINGLE, 1, 4, 1, 24, 2, VERDICT_ADMITTED, 0}, /* 丿 piě */
    {0x4E59, FAMILY_BOUNDARY, STRUCT_SINGLE, 1, 5, 15, 6, 2, VERDICT_ADMITTED, 0}, /* 乙 yǐ */
    {0x571F, FAMILY_BOUNDARY, STRUCT_SINGLE, 3, 34, 5, 8, 2, VERDICT_ADMITTED, 0}, /* 土 tǔ */
    {0x5DE5, FAMILY_BOUNDARY, STRUCT_SINGLE, 3, 50, 8, 44, 0, VERDICT_ADMITTED, 0}, /* 工 gōng */
    {0x5927, FAMILY_BOUNDARY, STRUCT_SINGLE, 3, 39, 4, 26, 3, VERDICT_ADMITTED, 0}, /* 大 dà */
    {0x4EBA, FAMILY_BOUNDARY, STRUCT_SINGLE, 2, 8, 19, 32, 1, VERDICT_ADMITTED, 0}, /* 人 rén */
    {0x65E5, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 61, 19, 6, 3, VERDICT_ADMITTED, 0}, /* 日 rì */
    {0x6708, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 74, 15, 62, 3, VERDICT_ADMITTED, 0}, /* 月 yuè */
    {0x77F3, FAMILY_BOUNDARY, STRUCT_SINGLE, 5, 112, 16, 6, 1, VERDICT_ADMITTED, 0}, /* 石 shí */
    {0x6C34, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 86, 18, 12, 2, VERDICT_ADMITTED, 0}, /* 水 shuǐ */
    {0x706B, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 87, 10, 28, 2, VERDICT_ADMITTED, 0}, /* 火 huǒ */
    {0x5C71, FAMILY_BOUNDARY, STRUCT_SINGLE, 3, 48, 18, 30, 0, VERDICT_ADMITTED, 0}, /* 山 shān */
    {0x6728, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 64, 2, 8, 3, VERDICT_ADMITTED, 0}, /* 木 mù */
    {0x6C11, FAMILY_BOUNDARY, STRUCT_SINGLE, 5, 84, 2, 34, 1, VERDICT_ADMITTED, 0}, /* 民 mín */
    {0x5FC3, FAMILY_BOUNDARY, STRUCT_SINGLE, 4, 62, 14, 34, 0, VERDICT_ADMITTED, 0}, /* 心 xīn */
};

/* 夫妻家族 (左右/上下结构 · 囡囝合) */
static const mirror_observation_t GATE_COUPLE[] = {
    /* 左右结构 */
    {0x597D, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 6, 40, 10, 14, 2, VERDICT_ADMITTED, 0}, /* 好 hǎo */
    {0x5979, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 6, 40, 5, 26, 0, VERDICT_ADMITTED, 0}, /* 她 tā */
    {0x4ED6, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 5, 8, 5, 26, 0, VERDICT_ADMITTED, 0}, /* 他 tā */
    {0x4EEC, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 5, 8, 2, 32, 4, VERDICT_ADMITTED, 0}, /* 们 men */
    {0x6211, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 7, 67, 11, 28, 2, VERDICT_ADMITTED, 0}, /* 我 wǒ */
    {0x4F60, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 7, 8, 6, 6, 2, VERDICT_ADMITTED, 0}, /* 你 nǐ */
    {0x4FDD, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 9, 8, 0, 14, 2, VERDICT_ADMITTED, 0}, /* 保 bǎo */
    {0x4F46, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 7, 8, 4, 30, 3, VERDICT_ADMITTED, 0}, /* 但 dàn */
    /* 上下结构 */
    {0x751F, FAMILY_COUPLE, STRUCT_TOP_BOTTOM, 5, 100, 18, 40, 0, VERDICT_ADMITTED, 0}, /* 生 shēng */
    {0x5B57, FAMILY_COUPLE, STRUCT_TOP_BOTTOM, 6, 41, 16, 6, 3, VERDICT_ADMITTED, 0}, /* 字 zì */
    {0x5B89, FAMILY_COUPLE, STRUCT_TOP_BOTTOM, 6, 42, 23, 30, 0, VERDICT_ADMITTED, 0}, /* 安 ān */
    {0x6811, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 9, 64, 18, 8, 3, VERDICT_ADMITTED, 0}, /* 树 shù */
    {0x6797, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 8, 64, 7, 34, 1, VERDICT_ADMITTED, 0}, /* 林 lín */
    {0x4F60, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 7, 42, 6, 6, 3, VERDICT_ADMITTED, 0}, /* 你 nǐ */
    {0x6CBB, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 8, 86, 16, 6, 3, VERDICT_ADMITTED, 0}, /* 治 zhì */
    {0x6D77, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 10, 86, 10, 6, 2, VERDICT_ADMITTED, 0}, /* 海 hǎi */
    {0x6CB3, FAMILY_COUPLE, STRUCT_LEFT_RIGHT, 8, 86, 10, 24, 1, VERDICT_ADMITTED, 0}, /* 河 hé */
    {0x591A, FAMILY_COUPLE, STRUCT_TOP_BOTTOM, 6, 38, 4, 28, 0, VERDICT_ADMITTED, 0}, /* 多 duō */
    {0x5C11, FAMILY_COUPLE, STRUCT_TOP_BOTTOM, 4, 44, 18, 14, 2, VERDICT_ADMITTED, 0}, /* 少 shǎo */
};

/* 网状家族 (半包围/交错) */
static const mirror_observation_t GATE_WEB[] = {
    {0x8F66, FAMILY_WEB, STRUCT_SINGLE, 4, 164, 17, 24, 0, VERDICT_ADMITTED, 0}, /* 车 chē */
    {0x8FDE, FAMILY_WEB, STRUCT_SEMI_WRAP, 7, 173, 7, 50, 1, VERDICT_ADMITTED, 0}, /* 连 lián */
    {0x8FC7, FAMILY_WEB, STRUCT_SEMI_WRAP, 6, 173, 8, 28, 3, VERDICT_ADMITTED, 0}, /* 过 guò */
    {0x8FD0, FAMILY_WEB, STRUCT_SEMI_WRAP, 7, 173, 15, 34, 3, VERDICT_ADMITTED, 0}, /* 运 yùn */
    {0x8FDB, FAMILY_WEB, STRUCT_SEMI_WRAP, 7, 173, 12, 34, 3, VERDICT_ADMITTED, 0}, /* 进 jìn */
    {0x9053, FAMILY_WEB, STRUCT_SEMI_WRAP, 12, 173, 4, 14, 3, VERDICT_ADMITTED, 0}, /* 道 dào */
    {0x8FD8, FAMILY_WEB, STRUCT_SEMI_WRAP, 7, 173, 10, 6, 1, VERDICT_ADMITTED, 0}, /* 还 hái */
    {0x9001, FAMILY_WEB, STRUCT_SEMI_WRAP, 9, 173, 18, 44, 3, VERDICT_ADMITTED, 0}, /* 送 sòng */
};

/* 中性家族 */
static const mirror_observation_t GATE_NEUTRAL[] = {
    {0x4E16, FAMILY_NEUTRAL, STRUCT_SINGLE, 5, 0, 18, 6, 3, VERDICT_ADMITTED, 0}, /* 世 shì */
    {0x4E8B, FAMILY_NEUTRAL, STRUCT_LEFT_RIGHT, 8, 0, 18, 6, 3, VERDICT_ADMITTED, 0}, /* 事 shì */
};

/* ── 家族索引表 (感知入口) ── */
typedef struct {
    const mirror_observation_t *table;
    uint16_t count;
    perception_family_t family;
    const char *name;
} family_gate_t;

static const family_gate_t FAMILY_GATES[FAMILY_COUNT] = {
    {GATE_CONTAINER, (uint16_t)(sizeof(GATE_CONTAINER)/sizeof(GATE_CONTAINER[0])), FAMILY_CONTAINER, "容器"},
    {GATE_BOUNDARY,  (uint16_t)(sizeof(GATE_BOUNDARY)/sizeof(GATE_BOUNDARY[0])),   FAMILY_BOUNDARY,  "边界"},
    {GATE_COUPLE,    (uint16_t)(sizeof(GATE_COUPLE)/sizeof(GATE_COUPLE[0])),       FAMILY_COUPLE,    "夫妻"},
    {GATE_WEB,       (uint16_t)(sizeof(GATE_WEB)/sizeof(GATE_WEB[0])),             FAMILY_WEB,       "网状"},
    {GATE_NEUTRAL,   (uint16_t)(sizeof(GATE_NEUTRAL)/sizeof(GATE_NEUTRAL[0])),     FAMILY_NEUTRAL,   "中性"},
};

/* ── 全局裁判引擎 ── */
static triple_judge_engine_t g_judge = {
    .perceive_power = 1257,
    .classify_power = 5,
    .dynamic_power  = 1257,
    .admit_count = 0,
    .reject_count = 0,
};

/* ═══════════════════════════════════════════════════════════════
 * 分别器第一层: 字形分类 → 感知家族
 * ═══════════════════════════════════════════════════════════════ */

perception_family_t perceive_classify_glyph(glyph_structure_t st) {
    switch (st) {
    case STRUCT_FULL_WRAP:   return FAMILY_CONTAINER;
    case STRUCT_LEFT_RIGHT:
    case STRUCT_TOP_BOTTOM:
    case STRUCT_PIN:
    case STRUCT_LMR:
    case STRUCT_TMB:         return FAMILY_COUPLE;
    case STRUCT_SEMI_WRAP:   return FAMILY_WEB;
    case STRUCT_SINGLE:      return FAMILY_BOUNDARY;
    default:                 return FAMILY_NEUTRAL;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 镜像观察: 逐一比对符号标注
 * ═══════════════════════════════════════════════════════════════ */

int mirror_observe_compare(const mirror_observation_t *target,
                           const mirror_observation_t *candidate) {
    if (target->structure != candidate->structure)
        return 1;
    if (target->shengmu_idx != candidate->shengmu_idx)
        return 1;
    if (target->yunmu_idx != candidate->yunmu_idx)
        return 1;
    if (target->tone != candidate->tone)
        return 1;
    if (target->family != candidate->family)
        return 1;
    if (target->ucs2 != candidate->ucs2)
        return 2;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 三元裁判: 三模块裁决准入
 * ═══════════════════════════════════════════════════════════════ */

uint8_t triple_judge_admit(mirror_observation_t *obs, triple_judge_engine_t *engine) {
    /* 三元裁判: 感知确认 + 分类归属 + 动力充足
     * 动力是引擎的"能力值"(capacity),不是消耗品(admission不消耗)
     *    perceive_power = 能识别多少种字形(固定能力)
     *    classify_power = 能区分几个家族(固定能力)
     *    dynamic_power  = 能夹持多少目标(固定能力)
     */
    int perceive_ok = (engine->perceive_power > 0) && (obs->stroke_count > 0);
    int classify_ok = (engine->classify_power > 0) && (obs->family < FAMILY_COUNT);
    int dynamic_ok  = (engine->dynamic_power > 0) &&
                      (obs->shengmu_idx < 48) &&
                      (obs->yunmu_idx < 66) &&
                      (obs->tone < 5);

    if (perceive_ok && classify_ok && dynamic_ok) {
        obs->judge_verdict = VERDICT_ADMITTED;
        obs->admission_time = engine->admit_count;
        engine->admit_count++;
        /* 动力不减 — 能力不是消耗品 */
        return VERDICT_ADMITTED;
    }

    obs->judge_verdict = VERDICT_REJECTED;
    engine->reject_count++;
    return VERDICT_REJECTED;
}

/* ═══════════════════════════════════════════════════════════════
 * 容器化: 生成32位容器编码
 * ═══════════════════════════════════════════════════════════════ */

uint32_t containerize_admit(const mirror_observation_t *obs) {
    uint32_t code = 0;
    code |= ((uint32_t)(obs->family & 0x7)) << 29;
    uint32_t polarity = (obs->family == FAMILY_BOUNDARY) ? 1 : 0;
    code |= (polarity << 28);
    code |= ((uint32_t)(obs->structure & 0x7)) << 25;
    code |= ((uint32_t)(obs->shengmu_idx & 0x3F)) << 19;
    code |= ((uint32_t)(obs->yunmu_idx & 0x3F)) << 13;
    code |= ((uint32_t)(obs->tone & 0x7)) << 10;
    return code;
}

/* ═══════════════════════════════════════════════════════════════
 * 筷子夹持
 * ═══════════════════════════════════════════════════════════════ */

int chopstick_grip(chopstick_grip_t *grip) {
    uint8_t src_fam = CONTAINER_GET_FAMILY(grip->source);
    uint8_t tgt_fam = CONTAINER_GET_FAMILY(grip->target);
    if (src_fam >= FAMILY_COUNT || tgt_fam >= FAMILY_COUNT)
        return -1;
    if (!grip->ack)
        return -2;
    grip->pressure = (uint16_t)((grip->source ^ grip->target) & 0xFFFF);
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 流水线排队
 * ═══════════════════════════════════════════════════════════════ */

#define PIPELINE_DEPTH 128
static uint32_t pipeline_queue[PIPELINE_DEPTH];
static int pipeline_head = 0;
static int pipeline_tail = 0;

int pipeline_enqueue(uint32_t container_code, uint32_t timestamp) {
    int next = (pipeline_tail + 1) % PIPELINE_DEPTH;
    if (next == pipeline_head)
        return -1;
    pipeline_queue[pipeline_tail] = container_code;
    pipeline_tail = next;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 感知场路由主入口
 * ═══════════════════════════════════════════════════════════════ */

uint32_t perception_route_glyph(uint16_t ucs2, uint8_t sm_idx, uint8_t ym_idx, uint8_t tone) {
    /*
     * 改进: 搜索所有家族门 (不只用结构推定的那一个)
     *   当外部调用者不知道字形结构时(只传 UCS-2 + 拼音),
     *   必须依次询问五个家族门, 找到匹配为止
     *   这就是"五门都敲, 谁是它家族就进谁门"
     */
    mirror_observation_t target;
    target.ucs2 = ucs2;
    target.shengmu_idx = sm_idx;
    target.yunmu_idx = ym_idx;
    target.tone = tone;
    target.judge_verdict = VERDICT_PENDING;
    target.admission_time = 0;

    for (int f = 0; f < FAMILY_COUNT; f++) {
        target.family = FAMILY_GATES[f].family;
        const family_gate_t *gate = &FAMILY_GATES[f];
        for (uint16_t i = 0; i < gate->count; i++) {
            /* 只对比 ucs2 (最直接的身份确认 · 不用堵结构推错) */
            if (gate->table[i].ucs2 == ucs2) {
                mirror_observation_t candidate = gate->table[i];
                candidate.shengmu_idx = sm_idx;
                candidate.yunmu_idx = ym_idx;
                candidate.tone = tone;

                uint8_t verdict = triple_judge_admit(&candidate, &g_judge);
                if (verdict == VERDICT_ADMITTED) {
                    uint32_t code = containerize_admit(&candidate);
                    pipeline_enqueue(code, g_judge.admit_count);
                    return code;
                }
            }
        }
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 兼容 API
 * ═══════════════════════════════════════════════════════════════ */

void hanzi_init(void) {
    memset(&g_judge, 0, sizeof(g_judge));
    g_judge.perceive_power = 1257;
    g_judge.classify_power = 5;
    g_judge.dynamic_power  = 1257;
    pipeline_head = pipeline_tail = 0;
}

uint32_t hanzi_get_token(uint16_t ucs2, uint8_t tone) {
    return 0;
}

uint32_t hanzi_route(uint16_t ucs2, uint8_t sm_idx, uint8_t ym_idx, uint8_t tone) {
    return perception_route_glyph(ucs2, sm_idx, ym_idx, tone);
}

uint16_t hanzi_to_ucs2(uint32_t container_code) {
    if (!container_code) return 0xFFFF;
    uint8_t fam = CONTAINER_GET_FAMILY(container_code);
    uint8_t sm  = CONTAINER_GET_SHENGMU(container_code);
    uint8_t ym  = CONTAINER_GET_YUNMU(container_code);
    uint8_t tn  = CONTAINER_GET_TONE(container_code);

    if (fam < FAMILY_COUNT) {
        const family_gate_t *gate = &FAMILY_GATES[fam];
        for (uint16_t i = 0; i < gate->count; i++) {
            const mirror_observation_t *obs = &gate->table[i];
            if (obs->shengmu_idx == sm &&
                obs->yunmu_idx == ym &&
                obs->tone == tn) {
                return obs->ucs2;
            }
        }
    }
    return 0xFFFF;
}

int hanzi_find_by_pinyin(int initial, int final, int tone) {
    for (int f = 0; f < FAMILY_COUNT; f++) {
        const family_gate_t *gate = &FAMILY_GATES[f];
        for (uint16_t i = 0; i < gate->count; i++) {
            const mirror_observation_t *obs = &gate->table[i];
            if (obs->shengmu_idx == (uint8_t)initial &&
                obs->yunmu_idx == (uint8_t)final &&
                obs->tone == (uint8_t)tone) {
                return (int)obs->ucs2;
            }
        }
    }
    return -1;
}

int hanzi_ucs2_to_pinyin(uint16_t ucs2, uint8_t *sm_idx, uint8_t *ym_idx, uint8_t *tone) {
    for (int f = 0; f < FAMILY_COUNT; f++) {
        const family_gate_t *gate = &FAMILY_GATES[f];
        for (uint16_t i = 0; i < gate->count; i++) {
            const mirror_observation_t *obs = &gate->table[i];
            if (obs->ucs2 == ucs2) {
                if (sm_idx) *sm_idx = obs->shengmu_idx;
                if (ym_idx) *ym_idx = obs->yunmu_idx;
                if (tone)   *tone   = obs->tone;
                return 0;
            }
        }
    }
    return -1;
}

int hanzi_count(void) {
    int total = 0;
    for (int f = 0; f < FAMILY_COUNT; f++) {
        total += FAMILY_GATES[f].count;
    }
    return total;
}

uint32_t hanzi_get_judge_stats(uint32_t *admit, uint32_t *reject) {
    if (admit)  *admit  = g_judge.admit_count;
    if (reject) *reject = g_judge.reject_count;
    return admit ? *admit : 0;
}