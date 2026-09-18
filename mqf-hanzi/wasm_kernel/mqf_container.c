/*****************************************************************************
 *  mqf_container.c   容器物理运算核心
 *  ──────────────────────────────────────────────────────────────────────────
 *  同一源码 · 分别编译为 mqf_container_32.o / mqf_container_64.o
 *  实现了 SMRL 引擎的容器裁判（坐标·极性·主权·价值）+ 物理公理层（测距/吃/合一）
 *****************************************************************************/

#include "mqf_wasm.h"

/* wasm 裸机模式时无 string.h，提供最小 memset 替代 */
#ifdef __wasm__
static void mqf_memset(void* dst, int val, mqf_cell_t n) {
    unsigned char* d = (unsigned char*)dst;
    for (mqf_cell_t i = 0; i < n; i++) d[i] = (unsigned char)val;
}
#else
#include <string.h>
#define mqf_memset memset
#endif

/* ============================================================
 *  容器初始化
 * ============================================================ */
MQF_EXPORT("mqf_container_init")
mqf_container_t mqf_container_init(mqf_cell_t coord, uint8_t polarity) {
    mqf_container_t c;
    c.coord    = coord;
    c.polarity = (polarity != 0) ? MQF_POLARITY_YANG : MQF_POLARITY_YIN;
    c.phase    = MQF_COORD_PHASE(coord);
    c.layer    = MQF_COORD_LAYER(coord);
    c.reserved = 0;
    c.valence  = 0;
    c.energy   = 0;
    return c;
}

/* ============================================================
 *  物理公理层  测距 → 吃 → 合一 → 沉淀 → 驱动
 * ============================================================ */
MQF_EXPORT("mqf_distance")
mqf_cell_t mqf_distance(const mqf_container_t* a, const mqf_container_t* b) {
    mqf_signed_t d = (mqf_signed_t)(a->coord) - (mqf_signed_t)(b->coord);
    if (d < 0) d = -d;
    return (mqf_cell_t)d;
}

MQF_EXPORT("mqf_consume")
void mqf_consume(mqf_container_t* self, mqf_cell_t amount) {
    /* 吃 = 消除空间 = 物质减少 = self 能量按 amount/10 消耗 */
    mqf_cell_t cost = amount / MQF_CONSUME_BASE;
    if (cost == 0) cost = 1;
    if (self->energy > cost)
        self->energy -= cost;
    else
        self->energy = 0;
}

MQF_EXPORT("mqf_merge")
uint8_t mqf_merge(mqf_container_t* a, const mqf_container_t* b) {
    /* 合一 = 距离归零 = 时间最小化 */
    mqf_cell_t d = mqf_distance(a, b);
    if (d <= 1) {
        /* 距离足够小：检查极性相容 */
        uint8_t gap = MQF_POLARITY_GAP(a, b);
        if (gap == MQF_ATTRACT) {
            /* 异性相吸：能量叠加，坐标取平均 */
            a->energy += b->energy;
            a->valence += b->valence;
            a->phase = (a->phase + b->phase) % MQF_PHASE_COUNT;
            return 0; /* 合一成功 */
        } else {
            /* 同性相斥：能量对冲 */
            if (a->energy >= b->energy)
                a->energy -= b->energy / 2;
            else
                a->energy = 0;
            return 1; /* 斥·未合一 */
        }
    }
    /* 距离过远：坐标靠拢一步 */
    if (a->coord < b->coord)
        a->coord += 1;
    else if (a->coord > b->coord)
        a->coord -= 1;
    return 2; /* 趋近中 */
}

MQF_EXPORT("mqf_settle")
void mqf_settle(mqf_container_t* self) {
    /* 沉淀 = 静止刻度 = 公约数 = 持久度增加 */
    if (self->valence > 0) {
        mqf_cell_t surplus = self->valence / 10;   /* 余数沉淀 */
        if (surplus == 0) surplus = 1;
        self->energy += surplus;
        self->valence -= surplus;
    }
}

MQF_EXPORT("mqf_drive")
void mqf_drive(mqf_container_t* self) {
    /* 驱动 = 公倍数推动力 = 能量转换为坐标进位量 */
    if (self->energy >= self->valence / 2) {
        mqf_cell_t stride = self->layer;    /* 层级越高越远 */
        if (stride == 0) stride = 1;
        self->coord += stride;
        self->energy -= stride;
    }
}

MQF_EXPORT("mqf_cycle")
uint8_t mqf_cycle(mqf_container_t* self) {
    mqf_settle(self);
    mqf_drive(self);
    return self->energy > 0 ? 1 : 0;  /* 1=持续 0=静止 */
}

/* ============================================================
 *  极性判决
 * ============================================================ */
MQF_EXPORT("mqf_judge_polarity")
uint8_t mqf_judge_polarity(const mqf_container_t* a, const mqf_container_t* b) {
    uint8_t gap = MQF_POLARITY_GAP(a, b);   /* 0=斥(同性) 1=吸(异性) */
    return MQF_JUDGE_GAP(gap);             /* 返回保留值判断 */
}

/* ============================================================
 *  容器查询
 * ============================================================ */
MQF_EXPORT("mqf_container_energy")
mqf_cell_t mqf_container_energy(const mqf_container_t* c) {
    return c->energy;
}

MQF_EXPORT("mqf_container_coord")
mqf_cell_t mqf_container_coord(const mqf_container_t* c) {
    return c->coord;
}

MQF_EXPORT("mqf_container_phase")
uint8_t mqf_container_phase(const mqf_container_t* c) {
    return c->phase;
}

MQF_EXPORT("mqf_container_layer")
uint8_t mqf_container_layer(const mqf_container_t* c) {
    return c->layer;
}

MQF_EXPORT("mqf_container_valid")
uint8_t mqf_container_valid(const mqf_container_t* c) {
    return MQF_IS_VALID(c) ? 1 : 0;
}

/* ============================================================
 *  性能测试入口（本地运行，不导出为 wasm 符号）
 * ============================================================ */
#ifndef __wasm__
#if !defined(MQF_NO_MAIN)
int main(void) {
    /* 阳容器：相位0，第1层，行0列0，极性阳 */
    mqf_cell_t coord_a = MQF_COORD_PACK(0, 1, 0, 0);
    mqf_container_t a = mqf_container_init(coord_a, MQF_POLARITY_YANG);
    a.valence = 50;
    a.energy = 30;

    /* 阴容器：相位0，第1层，行8列0，极性阴 */
    mqf_cell_t coord_b = MQF_COORD_PACK(0, 1, 8, 0);
    mqf_container_t b = mqf_container_init(coord_b, MQF_POLARITY_YIN);
    b.valence = 40;
    b.energy = 20;

    /* 测距（坐标分别为 row0=0×256=0，row8=8×256=2048） */
    mqf_cell_t d = mqf_distance(&a, &b);

    /* 极性判决 */
    uint8_t judgment = MQF_JUDGE_GAP(MQF_POLARITY_GAP(&a, &b));

    /* 推进周期 */
    uint8_t active = mqf_cycle(&a);

    /* 合一（同相位不同极性 → 吸引 → 合一） */
    uint8_t merge_result = mqf_merge(&a, &b);

    return 0;
}
#endif
#endif
