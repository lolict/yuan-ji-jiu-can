/*
 * perception_field.c — 感知场 · 镜像观察 · 三元裁判 · 容器化
 *
 * 系统前门: 输入符号 → 感知家族 → 镜像对比 → 三元裁判 → 容器化 → 筷子夹持
 *
 * 架构:
 *   输入符号 → 感知家族(字形分类) → 镜像对比(发音+形态) → 三元裁判 → 准入
 *   → 容器化(32位编码) → 筷子夹持(二元公倍数) → 投入一元时间线
 */

#include <stdio.h>
#include <string.h>
#include "perception_field.h"
#include "global_interlock.h"
#include "symbol_chain.h"

/* ═══════════════════════════════════════════════════════════════
 * 管线队列 (模拟的一元时间线)
 * ═══════════════════════════════════════════════════════════════ */

#define PIPELINE_MAX 256

static struct {
    uint32_t codes[PIPELINE_MAX];
    uint32_t timestamps[PIPELINE_MAX];
    uint16_t head;
    uint16_t count;
} pipeline;

/* ═══════════════════════════════════════════════════════════════
 * 分别器第一层: 字形分类 → 感知家族
 * 不是哈希 = "看长相, 进哪个门"
 * ═══════════════════════════════════════════════════════════════ */

perception_family_t perceive_classify_glyph(glyph_structure_t st) {
    switch (st) {
        case STRUCT_SINGLE:       /* 独体 → 边界型 (如 一 二 人) */
            return FAMILY_BOUNDARY;

        case STRUCT_LEFT_RIGHT:   /* 左右 → 夫妻型 (如 好 她 你) */
        case STRUCT_TOP_BOTTOM:   /* 上下 → 夫妻型 (如 吕 昌 )');
            return FAMILY_COUPLE;

        case STRUCT_SEMI_WRAP:    /* 半包围 → 网状型 (如 匡 匹 区) */
            return FAMILY_WEB;

        case STRUCT_FULL_WRAP:    /* 全包围 → 容器型 (如 国 围 回) */
            return FAMILY_CONTAINER;

        case STRUCT_PIN:          /* 品字 → 容器型 (如 品 森 众) */
            return FAMILY_CONTAINER;

        case STRUCT_LMR:          /* 左中右 → 网状型 (如 做 街 掰) */
        case STRUCT_TMB:          /* 上中下 → 网状型 (如 意 莫 葬) */
            return FAMILY_WEB;

        default:
            return FAMILY_NEUTRAL;
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 镜像观察: 逐一对比符号标注
 * 匹配不是编码关系 = 本质匹配 (发音位置 + 视觉外观)
 * ═══════════════════════════════════════════════════════════════ */

int mirror_observe_compare(const mirror_observation_t *target,
                          const mirror_observation_t *candidate) {
    if (!target || !candidate) return 1;

    /* 家族必须相同 (否则是两个不同容器的人, 不可能匹配) */
    if (target->family != candidate->family) return 1;

    /* 字形结构对比: 同结构=0, 不同结构=1, 需进一步=2 */
    if (target->structure == candidate->structure) return 0;

    /* 同家族不同结构: 需进一步镜像对比 (如 好 vs 她 都是左右) */
    if (target->family == FAMILY_COUPLE || target->family == FAMILY_WEB) {
        return 2;  /* 同家族不同子结构, 需深入 */
    }

    /* 家族内结构不同且非夫妻/网状 = 不匹配 */
    return 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 三元裁判: 三模块裁决准入
 *   ┌──────┬────────┬──────────────────────────────┐
 *   │ 模块  │ 动力   │ 检查                         │
 *   ├──────┼────────┼──────────────────────────────┤
 *   │ 感知  │ 视觉   │ 笔画 > 0 且 结构合法          │
 *   │ 分类  │ 归属   │ 家族 != NEUTRAL              │
 *   │ 动力  │ 操作   │ 能被筷子夹持(二进制表示有内容) │
 *   └──────┴────────┴──────────────────────────────┘
 * 三元齐全 = ADMITTED
 * ═══════════════════════════════════════════════════════════════ */

uint8_t triple_judge_admit(mirror_observation_t *obs, triple_judge_engine_t *engine) {
    if (!obs) return VERDICT_PENDING;

    uint8_t modules = 0;

    /* 模块1 PERCEIVE: 能看见它的长相吗? */
    if (obs->stroke_count > 0 && obs->structure <= STRUCT_TMB) {
        modules |= JUDGE_PERCEIVE;
        if (engine) engine->perceive_power++;
    }

    /* 模块2 CLASSIFY: 它属于哪个容器家族? */
    if (obs->family < FAMILY_NEUTRAL) {
        modules |= JUDGE_CLASSIFY;
        if (engine) engine->classify_power++;
    }

    /* 模块3 DYNAMIC: 能用筷子夹持它吗?
     * 筷子夹持条件: 符号非空 + 发音索引合法 */
    if (obs->shengmu_idx > 0 && obs->shengmu_idx < 64 &&
        obs->yunmu_idx > 0 && obs->yunmu_idx < 64) {
        modules |= JUDGE_DYNAMIC;
        if (engine) engine->dynamic_power++;
    }

    /* 三元齐全 */
    if ((modules & (JUDGE_PERCEIVE | JUDGE_CLASSIFY | JUDGE_DYNAMIC)) ==
        (JUDGE_PERCEIVE | JUDGE_CLASSIFY | JUDGE_DYNAMIC)) {
        obs->judge_verdict = VERDICT_ADMITTED;
        if (engine) engine->admit_count++;
        return VERDICT_ADMITTED;
    }

    /* 缺任何一元 */
    if (modules == 0) {
        obs->judge_verdict = VERDICT_REJECTED;
    } else {
        obs->judge_verdict = VERDICT_SUSPENDED;
    }
    if (engine) engine->reject_count++;
    return obs->judge_verdict;
}

/* ═══════════════════════════════════════════════════════════════
 * 容器化: 通过裁判后, 装入容器 (生成32位编码)
 *
 * 编码位域:
 *   family(3) | polarity(1) | structure(3) | shengmu(6) | yunmu(6) | tone(3) | reserved(2)
 *   ─────────┬─────────────┬─────────────┬────────────┬───────────┬─────────┬──────
 *     感知     主权侧       字形结构     声母索引   韵母索引   声调    保留
 * ═══════════════════════════════════════════════════════════════ */

uint32_t containerize_admit(const mirror_observation_t *obs) {
    if (!obs || obs->judge_verdict != VERDICT_ADMITTED) return 0;

    return CONTAINER_ENCODE(obs->family,
                           obs->family == FAMILY_BOUNDARY ? 1 : 0,
                           obs->structure,
                           obs->shengmu_idx & 0x3F,
                           obs->yunmu_idx & 0x3F,
                           obs->tone & 0x7);
}

/* ═══════════════════════════════════════════════════════════════
 * 筷子夹持: 二元容器 → 夹持目标
 * 两根筷子 = 起点(source) + 终点(target)
 * 夹持动作 = 缩小有和虚无之间的空间
 * ═══════════════════════════════════════════════════════════════ */

int chopstick_grip(chopstick_grip_t *grip) {
    if (!grip || grip->source == 0 || grip->target == 0) return -1;

    /* 夹持条件: source 和 target 必须不同 (不能自己夹自己) */
    if (grip->source == grip->target) return -1;

    /* 极性判定: 容器(0) 夹边界(1) → 合法 */
    uint8_t pol_s = (grip->source >> CONTAINER_POLARITY_SHIFT) & 1;
    uint8_t pol_t = (grip->target >> CONTAINER_POLARITY_SHIFT) & 1;

    if (pol_s == pol_t) return -1;  /* 同极性 → 夹不住 */

    /* 夹持压力: 基于两根筷子的家族距离 */
    uint8_t fam_s = (grip->source >> CONTAINER_FAMILY_SHIFT) & 0x7;
    uint8_t fam_t = (grip->target >> CONTAINER_FAMILY_SHIFT) & 0x7;
    uint8_t dist = (fam_s > fam_t) ? (fam_s - fam_t) : (fam_t - fam_s);
    grip->pressure = 100 - dist * 20;  /* 距离越远压力越小 */
    if (grip->pressure == 0) grip->pressure = 10;

    grip->ack = 1;  /* 目标承认被夹持 */
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 流程排队: 夹持成功后, 投入一元时间线
 * ═══════════════════════════════════════════════════════════════ */

int pipeline_enqueue(uint32_t container_code, uint32_t timestamp) {
    if (container_code == 0) return -1;
    if (pipeline.count >= PIPELINE_MAX) return -1;  /* 队列满 */

    uint16_t idx = (pipeline.head + pipeline.count) % PIPELINE_MAX;
    pipeline.codes[idx] = container_code;
    pipeline.timestamps[idx] = timestamp;
    pipeline.count++;

    return 0;
}

/**
 * 获取管线中已排队的编码数量
 */
uint16_t pipeline_count(void) {
    return pipeline.count;
}

/**
 * 获取管线中指定位置的编码
 */
uint32_t pipeline_peek(uint16_t index) {
    if (index >= pipeline.count) return 0;
    return pipeline.codes[(pipeline.head + index) % PIPELINE_MAX];
}

/**
 * 清空管线
 */
void pipeline_clear(void) {
    pipeline.head = 0;
    pipeline.count = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 感知场综合报告
 * ═══════════════════════════════════════════════════════════════ */

void perception_field_report(triple_judge_engine_t *engine) {
    printf("\n");
    printf("  ┌─ 感知场 ──────────────────────────┐\n");
    if (engine) {
        printf("  │ 感知动力: %u  分类动力: %u\n",
               engine->perceive_power, engine->classify_power);
        printf("  │ 夹持动力: %u\n", engine->dynamic_power);
        printf("  │ 准入: %u  拒绝: %u\n",
               engine->admit_count, engine->reject_count);
    }
    printf("  │ 管线队列: %u/%u\n", pipeline.count, (unsigned int)PIPELINE_MAX);
    printf("  └────────────────────────────────────┘\n");
}
