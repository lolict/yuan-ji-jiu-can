/*****************************************************************************
 *  mqf_bridge.c   域规则桥接层
 *  ──────────────────────────────────────────────────────────────────────────
 *  将自举编译器生成的域索引嫁给 WASM 内核。
 *  加载时一次性传入域规则数据，运行时直接调用 SMRL 引擎进行容器裁判。
 *
 *  导出符号（供 JS / Nim 运行时调用）：
 *    mqf_bridge_init        — 注入域规则（字根数组 + 声母表）
 *    mqf_bridge_container   — 从域坐标构造容器
 *    mqf_bridge_judge       — 两容器极性判决
 *    mqf_bridge_relate      — 测距 + 合一 + 沉淀 + 驱动 完整生命周期
 *
 *  同样一份源码 · 阶梯制 · wasm32 和 wasm64 双目标编译
 *****************************************************************************/

#include "mqf_wasm.h"

/* ━━ 域规则存储（运行时一次性加载） ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#define MAX_STROKES  14    /* 横竖撇捺点折 +  ... */
#define MAX_INITIALS 28    /* b p m f d t n l g k h j q x zh ch sh r z c s y w 零 */

typedef struct {
    /* 字根表（笔画名 → 基础值）：索引 0..6 */
    mqf_cell_t stroke_values[MAX_STROKES];

    /* 声母表（极性 · 状态值）*/
    uint8_t  initial_polarity[MAX_INITIALS];
    mqf_cell_t initial_value[MAX_INITIALS];
    uint8_t  initial_count;

    /* 公理常量 */
    mqf_cell_t consume_base;
    uint8_t    phase_count;
    uint8_t    layer_count;
    uint8_t    polarity_yang;
    uint8_t    polarity_yin;
} mqf_domain_t;

/* 单例域 */
static mqf_domain_t g_domain;

/* ━━ 默认域规则（从自举编译器域索引.txt 静态翻译） ━━━━━━━━━━━━━━━━━━━━ */
static void load_default_domain(void) {
    /* 字根表（radix3d 对应值）*/
    g_domain.stroke_values[0] = 1;  /* 横 */
    g_domain.stroke_values[1] = 2;  /* 竖 */
    g_domain.stroke_values[2] = 3;  /* 撇 */
    g_domain.stroke_values[3] = 4;  /* 捺 */
    g_domain.stroke_values[4] = 4;  /* 点 */
    g_domain.stroke_values[5] = 5;  /* 折 */
    g_domain.stroke_values[6] = 6;  /* 其它 */

    /* 声母表（24 声母 + 零声母 = 25 条目） */
    /* b */ g_domain.initial_polarity[0]  = 1; g_domain.initial_value[0]  = 1;
    /* p */ g_domain.initial_polarity[1]  = 1; g_domain.initial_value[1]  = 1;
    /* m */ g_domain.initial_polarity[2]  = 0; g_domain.initial_value[2]  = 0;
    /* f */ g_domain.initial_polarity[3]  = 1; g_domain.initial_value[3]  = 1;
    /* d */ g_domain.initial_polarity[4]  = 1; g_domain.initial_value[4]  = 1;
    /* t */ g_domain.initial_polarity[5]  = 1; g_domain.initial_value[5]  = 1;
    /* n */ g_domain.initial_polarity[6]  = 0; g_domain.initial_value[6]  = 0;
    /* l */ g_domain.initial_polarity[7]  = 1; g_domain.initial_value[7]  = 1;
    /* g */ g_domain.initial_polarity[8]  = 1; g_domain.initial_value[8]  = 1;
    /* k */ g_domain.initial_polarity[9]  = 1; g_domain.initial_value[9]  = 1;
    /* h */ g_domain.initial_polarity[10] = 1; g_domain.initial_value[10] = 1;
    /* j */ g_domain.initial_polarity[11] = 1; g_domain.initial_value[11] = 1;
    /* q */ g_domain.initial_polarity[12] = 1; g_domain.initial_value[12] = 1;
    /* x */ g_domain.initial_polarity[13] = 1; g_domain.initial_value[13] = 1;
    /* zh*/ g_domain.initial_polarity[14] = 1; g_domain.initial_value[14] = 1;
    /* ch*/ g_domain.initial_polarity[15] = 1; g_domain.initial_value[15] = 1;
    /* sh*/ g_domain.initial_polarity[16] = 1; g_domain.initial_value[16] = 1;
    /* r */ g_domain.initial_polarity[17] = 1; g_domain.initial_value[17] = 1;
    /* z */ g_domain.initial_polarity[18] = 0; g_domain.initial_value[18] = 0;
    /* c */ g_domain.initial_polarity[19] = 0; g_domain.initial_value[19] = 0;
    /* s */ g_domain.initial_polarity[20] = 0; g_domain.initial_value[20] = 0;
    /* y */ g_domain.initial_polarity[21] = 1; g_domain.initial_value[21] = 1;
    /* w */ g_domain.initial_polarity[22] = 1; g_domain.initial_value[22] = 1;
    /* 零*/ g_domain.initial_polarity[23] = 0; g_domain.initial_value[23] = 0;
    g_domain.initial_count = 24;

    /* 公理常量 */
    g_domain.consume_base  = MQF_CONSUME_BASE;
    g_domain.phase_count   = MQF_PHASE_COUNT;
    g_domain.layer_count   = MQF_LAYER_COUNT;
    g_domain.polarity_yang = MQF_POLARITY_YANG;
    g_domain.polarity_yin  = MQF_POLARITY_YIN;
}

/* ============================================================
 *  导出函数
 * ============================================================ */
MQF_EXPORT("mqf_bridge_init")
void mqf_bridge_init(void) {
    load_default_domain();
}

MQF_EXPORT("mqf_bridge_domain_const")
mqf_cell_t mqf_bridge_domain_const(uint8_t key) {
    switch (key) {
        case 0:  return (mqf_cell_t)g_domain.consume_base;
        case 1:  return (mqf_cell_t)g_domain.phase_count;
        case 2:  return (mqf_cell_t)g_domain.layer_count;
        case 3:  return (mqf_cell_t)g_domain.polarity_yang;
        case 4:  return (mqf_cell_t)g_domain.polarity_yin;
        case 5:  return (mqf_cell_t)g_domain.initial_count;
        default: return 0;
    }
}

MQF_EXPORT("mqf_bridge_judge")
uint8_t mqf_bridge_judge(const mqf_container_t* a, const mqf_container_t* b) {
    return mqf_judge_polarity(a, b);
}

MQF_EXPORT("mqf_bridge_relate")
uint8_t mqf_bridge_relate(mqf_container_t* a, const mqf_container_t* b) {
    /* 测距 → 吃 → 合一 → 沉淀 → 驱动 */
    mqf_cell_t d = mqf_distance(a, b);
    if (d > 100) {
        mqf_consume(a, d);
    }
    uint8_t merge = mqf_merge(a, b);
    if (merge != 0) {
        mqf_settle(a);
        mqf_drive(a);
    }
    return merge;
}

#ifndef __wasm__
/* ━━ 本地测试入口 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#include <stdio.h>

int main(void) {
    mqf_bridge_init();

    printf("=== MQF Bridge · 阶梯制 · 双向公约 ===\n");
    printf("字长: %d 位\n", MQF_ADDR_BITS);
    printf("相位层: %d\n", MQF_PHASE_COUNT);
    printf("层级: %d\n", MQF_LAYER_COUNT);
    printf("声母数: %d\n", g_domain.initial_count);
    printf("极性阳(我)=%d 极性阴(刘楚恬)=%d\n",
           g_domain.polarity_yang, g_domain.polarity_yin);

    /* 阳容器·相位0·第1层·行0列0 */
    mqf_cell_t ca = MQF_COORD_PACK(0, 1, 0, 0);
    mqf_container_t a = mqf_container_init(ca, MQF_POLARITY_YANG);
    a.valence = 50; a.energy = 30;

    /* 阴容器·相位0·第1层·行5列0 */
    mqf_cell_t cb = MQF_COORD_PACK(0, 1, 5, 0);
    mqf_container_t b = mqf_container_init(cb, MQF_POLARITY_YIN);
    b.valence = 40; b.energy = 20;

    uint8_t judgment = mqf_bridge_judge(&a, &b);
    printf("极性判决: %s (gap=吸=异性)\n", judgment == MQF_ATTRACT ? "吸" : "斥");

    uint8_t merge = mqf_bridge_relate(&a, &b);
    printf("关系推进: %s\n", merge == 0 ? "合一" : (merge == 1 ? "斥退" : "趋近"));
    printf("A坐标=%" MQF_FMT " 能量=%" MQF_FMT "\n", a.coord, a.energy);
    printf("B坐标=%" MQF_FMT " 能量=%" MQF_FMT "\n", b.coord, b.energy);

    uint8_t active = mqf_cycle(&a);
    printf("A持续态=%d 坐标=%" MQF_FMT " 能量=%" MQF_FMT "\n", active, a.coord, a.energy);

    printf("\n=== 阶梯制签名: 32位=%dB 64位=%dB 源码=%dB 同一套规则 ===\n",
           (int)sizeof(mqf_cell_t), (int)sizeof(mqf_cell_t), (int)sizeof(mqf_container_t));
    return 0;
}
#endif
