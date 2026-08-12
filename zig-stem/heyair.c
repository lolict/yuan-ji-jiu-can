/**
 * heyair.c — 灰原哀 · 横向层 · 感知器官 · 念动力管辖
 *
 * heyair = 灰原哀 = 横向/念动力/管辖 = 感知器官
 *
 * 在满全法角色体系中：
 * - gtxone (工藤) = 骨架/中心/纵向——定义"位置"
 * - kndoyle (柯南) = 框架/被管辖——定义"内容"
 * - heyair (灰原) = 横向/穿透——定义"关系如何跨越边界"
 *
 * heyair 的核心职责：
 * 1. 管辖二进制：让分散的 0 和 1 合并为一个共同体
 * 2. 横向编织：把不同领域的事件投影到同一基准
 * 3. 念动力驱动：用意志驱动关系变化（不依赖外部力）
 *
 * 数学含义：
 * - 灰原哀 = "横向投影函子"
 * - 她把一个范畴的对象和态射"映射"到另一个范畴
 * - 映射过程中，某些信息被"压缩"（0和1合并），某些被"保留"（差异变纹理）
 */

#include <stdint.h>
#include <stdbool.h>
#include <string.h>

/* ————————————————————————————————————————————
 * 常量
 * ————————————————————————————————————————──── */

/// 最大感知通道数
#define MAX_PERCEPTION_CHANNELS 64

/// 最大横向编织的行数
#define MAX_WEAVE_ROWS 256

/// 最大列数
#define MAX_WEAVE_COLS 64

/// 念动力衰减系数（距离越远，驱动力越弱）
#define NEN_DECAY 0.9f

/* ————————————————————————————————————————————
 * 状态定义
 * ————————————————————————————————————————──── */

/// 编织状态
typedef enum {
    WEAVE_EMPTY    = 0x00,  // 空：未开始编织
    WEAVE_ACTIVE   = 0x01,  // 活跃：正在编织
    WEAVE_SETTLING = 0x02,  // 沉降：编织完成，等待稳定
    WEAVE_STABLE   = 0x03,  // 稳定：编织已固定
} weave_state;

/// 感知类型
typedef enum {
    PERCEIVE_BINARY   = 0x00,  // 感知二进制：0和1的对立
    PERCEIVE_MERGE    = 0x01,  // 感知融合：2合1的过程
    PERCEIVE_DISTANCE = 0x02,  // 感知距离：节点间的远近
    PERCEIVE_PATTERN  = 0x03,  // 感知模式：聚类中的重复结构
} perceive_type;

/* ————————————————————————————————————————————
 * 感知通道
 * ————————————————————————————————————————──── */

/// 一个感知通道 = 灰原哀的一只"眼睛"
/// 每只眼睛从一个特定维度感知关系
typedef struct {
    uint32_t       id;
    perceive_type  type;
    float          sensitivity;     // 灵敏度 (0.0 ~ 1.0)
    uint32_t       target_node;    // 目标节点
    float          last_reading;   // 上次感知值
    bool           is_active;
} perception_channel_t;

/* ————————————————————————————————————————————
 * 核心结构：灰原哀横向层
 * ————————————————————————————————————————──── */

/// heyair = 灰原哀 = 横向感知编织器
///
/// 她不定义中心（那是工藤的事），
/// 也不定义被管辖者（那是柯南的事）。
/// 她把"中心"和"被管辖者"用横向的线编织起来，
/// 让它们之间产生"可以比较、可以穿越"的关系。
typedef struct {
    uint32_t id;
    weave_state state;

    /* 感知器官 */
    uint32_t channel_count;
    perception_channel_t channels[MAX_PERCEPTION_CHANNELS];

    /* 编织矩阵：行=事件/节点，列=维度 */
    float weave_matrix[MAX_WEAVE_ROWS][MAX_WEAVE_COLS];
    uint32_t row_count;
    uint32_t col_count;

    /* 念动力：意志驱动的强度 */
    float nen_power;              // 念动力总量
    float nen_focus;              // 念动力聚焦度
} heyair_t;

/* ————————————————————————————————————————————
 * 创建与初始化
 * ————————————————————————————————————————──── */

heyair_t heyair_create(void) {
    static uint32_t next_id = 1;
    heyair_t h;
    h.id = next_id++;
    h.state = WEAVE_EMPTY;
    h.channel_count = 0;
    memset(h.channels, 0, sizeof(h.channels));
    memset(h.weave_matrix, 0, sizeof(h.weave_matrix));
    h.row_count = 0;
    h.col_count = 0;
    h.nen_power = 0.0f;
    h.nen_focus = 1.0f;
    return h;
}

/* ————————————————————————————————————————————
 * 感知操作
 * ————————————————————————————————————————──── */

/// 创建感知通道
perception_channel_t* heyair_add_channel(heyair_t* h, perceive_type type,
                                          float sensitivity, uint32_t target) {
    if (h->channel_count >= MAX_PERCEPTION_CHANNELS) return 0;

    perception_channel_t* ch = &h->channels[h->channel_count];
    ch->id = h->channel_count + 1;
    ch->type = type;
    ch->sensitivity = sensitivity;
    ch->target_node = target;
    ch->last_reading = 0.0f;
    ch->is_active = true;
    h->channel_count++;
    return ch;
}

/// 感知：从通道读取一个值
/// 感知值 = 输入值 × 灵敏度 × 念动力聚焦
float heyair_perceive(heyair_t* h, uint32_t channel_id, float input_value) {
    if (channel_id == 0 || channel_id > h->channel_count) return 0.0f;

    perception_channel_t* ch = &h->channels[channel_id - 1];
    if (!ch->is_active) return 0.0f;

    float reading = input_value * ch->sensitivity * h->nen_focus;
    ch->last_reading = reading;
    return reading;
}

/* ————————————————————————————————————————————
 * 管辖二进制：2合1
 * ————————————————————————————————————————──── */

/**
 * 管辖二进制的核心操作：让 2 合 1
 *
 * 输入：两个分离的值 a 和 b
 * 输出：一个合并后的值
 *
 * 合并公式：
 * merged = a × (1 - merge_strength) + b × merge_strength
 *
 * 当 merge_strength = 0.5 时：完全融合（a 和 b 对等合并）
 * 当 merge_strength < 0.5 时：a 为主导
 * 当 merge_strength > 0.5 时：b 为主导
 *
 * 这就是灰原哀的"管辖"：不是消灭差异，是调节差异的比例
 */
float heyair_merge_binary(float a, float b, float merge_strength) {
    if (merge_strength < 0.0f) merge_strength = 0.0f;
    if (merge_strength > 1.0f) merge_strength = 1.0f;
    return a * (1.0f - merge_strength) + b * merge_strength;
}

/// 判断两个值是否"已融合"
/// 融合 = 差异小于某个容差
bool heyair_is_merged(float a, float b, float tolerance) {
    float diff = a - b;
    if (diff < 0) diff = -diff;
    return diff <= tolerance;
}

/* ————————————————————————————————————————————
 * 横向编织
 * ————————————————————————————————————————──── */

/// 在编织矩阵中添加一行（一个事件/节点）
bool heyair_weave_add_row(heyair_t* h, const float* values, uint32_t count) {
    if (h->row_count >= MAX_WEAVE_ROWS) return false;
    if (count > MAX_WEAVE_COLS) count = MAX_WEAVE_COLS;

    memcpy(h->weave_matrix[h->row_count], values, count * sizeof(float));
    h->row_count++;
    if (count > h->col_count) h->col_count = count;
    h->state = WEAVE_ACTIVE;
    return true;
}

/// 列对齐：把不同行的同一列"对齐"到相同基准
/// 对齐方法：均值归一化
void heyair_weave_align_columns(heyair_t* h) {
    if (h->col_count == 0 || h->row_count == 0) return;

    for (uint32_t col = 0; col < h->col_count; col++) {
        /* 计算列均值 */
        float sum = 0.0f;
        for (uint32_t row = 0; row < h->row_count; row++) {
            sum += h->weave_matrix[row][col];
        }
        float mean = sum / (float)h->row_count;

        /* 按均值对齐 */
        for (uint32_t row = 0; row < h->row_count; row++) {
            h->weave_matrix[row][col] -= mean;
        }
    }
    h->state = WEAVE_SETTLING;
}

/* ————————————————————————————————————————————
 * 念动力
 * ————————————————————————————————————————──── */

/**
 * 念动力 = 意志驱动的强度
 *
 * 念动力的特点：
 * 1. 不依赖外部力——它是"内在驱动"
 * 2. 随距离衰减——目标越远，驱动力越弱
 * 3. 可聚焦——把念动力集中在一点，产生更大的效果
 *
 * 公式：
 * nen_at_distance(power, distance) = power × decay^distance
 */

/// 注入念动力
void heyair_nen_inject(heyair_t* h, float amount) {
    h->nen_power += amount;
    if (h->nen_power > 100.0f) h->nen_power = 100.0f;
}

/// 聚焦念动力
void heyair_nen_focus(heyair_t* h, float focus) {
    if (focus < 0.1f) focus = 0.1f;
    if (focus > 1.0f) focus = 1.0f;
    h->nen_focus = focus;
}

/// 计算距离 d 处的念动力
float heyair_nen_at_distance(const heyair_t* h, float distance) {
    float result = h->nen_power;
    for (int i = 0; i < (int)distance; i++) {
        result *= NEN_DECAY;
    }
    return result * h->nen_focus;
}

/* ————————————————————————————————————————————
 * 聚类：感知模式的自然涌现
 * ————————————————————————————————————————──── */

/**
 * 灰原哀的聚类功能：
 * 不需要预先定义类别，让数据自己"抱团"
 *
 * 方法：简单的距离阈值聚类
 * 1. 计算所有节点对之间的距离
 * 2. 距离小于阈值的节点归入同一族
 * 3. 返回聚类结果
 */

#define MAX_CLUSTERS 16
#define MAX_CLUSTER_SIZE 64

typedef struct {
    uint32_t member_ids[MAX_CLUSTER_SIZE];
    uint32_t member_count;
    float    centroid;      // 质心
} cluster_t;

typedef struct {
    uint32_t count;
    cluster_t clusters[MAX_CLUSTERS];
} cluster_result;

/// 简单聚类：按距离阈值把节点分组
cluster_result heyair_cluster(const float* values, uint32_t count, float threshold) {
    cluster_result result;
    result.count = 0;

    if (count == 0) return result;

    /* 标记数组：是否已归入某族 */
    bool assigned[MAX_WEAVE_ROWS];
    memset(assigned, 0, sizeof(assigned));

    for (uint32_t i = 0; i < count && result.count < MAX_CLUSTERS; i++) {
        if (assigned[i]) continue;

        /* 新建一个族 */
        cluster_t* c = &result.clusters[result.count];
        c->member_count = 0;
        c->centroid = values[i];
        c->member_ids[c->member_count++] = i;
        assigned[i] = true;

        /* 寻找所有距离在阈值内的节点 */
        for (uint32_t j = i + 1; j < count; j++) {
            if (assigned[j]) continue;
            float diff = values[j] - values[i];
            if (diff < 0) diff = -diff;
            if (diff <= threshold) {
                c->member_ids[c->member_count++] = j;
                assigned[j] = true;
            }
        }
        result.count++;
    }

    /* 重新计算质心 */
    for (uint32_t k = 0; k < result.count; k++) {
        cluster_t* c = &result.clusters[k];
        float sum = 0.0f;
        for (uint32_t m = 0; m < c->member_count; m++) {
            sum += values[c->member_ids[m]];
        }
        if (c->member_count > 0) {
            c->centroid = sum / (float)c->member_count;
        }
    }

    return result;
}

/* ————————————————————————————————————————————
 * 演示
 * ————————————————————————————————————————──── */

#ifdef BUILD_DEMO

#include <stdio.h>

int main(void) {
    printf("=== heyair 灰原哀 · 横向感知编织 演示 ===\n\n");

    /* 1. 创建灰原哀 */
    heyair_t h = heyair_create();
    printf("[创建] 灰原哀 id=%u\n", h.id);

    /* 2. 管辖二进制：2合1 */
    printf("\n[管辖二进制] 让 0 和 1 合并\n");
    float merged = heyair_merge_binary(0.0f, 1.0f, 0.5f);
    printf("  merge(0, 1, strength=0.5) = %.2f\n", merged);
    printf("  含义：0和1完全对等融合 → 一个共同体\n");

    /* 3. 感知通道 */
    printf("\n[感知通道] 创建感知器官\n");
    heyair_add_channel(&h, PERCEIVE_BINARY, 0.9f, 100);
    heyair_add_channel(&h, PERCEIVE_MERGE, 0.8f, 101);
    heyair_add_channel(&h, PERCEIVE_DISTANCE, 0.7f, 102);
    printf("  已创建 %u 个感知通道\n", h.channel_count);

    /* 4. 念动力 */
    printf("\n[念动力] 注入并聚焦\n");
    heyair_nen_inject(&h, 50.0f);
    heyair_nen_focus(&h, 0.9f);
    printf("  念动力总量: %.1f, 聚焦度: %.1f\n", h.nen_power, h.nen_focus);
    printf("  距离0处: %.2f\n", heyair_nen_at_distance(&h, 0));
    printf("  距离1处: %.2f (衰减10%%)\n", heyair_nen_at_distance(&h, 1));
    printf("  距离5处: %.2f (衰减%.0f%%)\n", heyair_nen_at_distance(&h, 5),
           (1.0f - 0.9f*0.9f*0.9f*0.9f*0.9f) * 100);

    /* 5. 横向编织 */
    printf("\n[横向编织] 把不同节点投影到同一基准\n");
    float row1[] = {1.0f, 2.0f, 3.0f};
    float row2[] = {1.1f, 2.1f, 2.9f};
    float row3[] = {5.0f, 6.0f, 7.0f};
    heyair_weave_add_row(&h, row1, 3);
    heyair_weave_add_row(&h, row2, 3);
    heyair_weave_add_row(&h, row3, 3);
    printf("  已编织 %u 行 × %u 列\n", h.row_count, h.col_count);

    heyair_weave_align_columns(&h);
    printf("  列对齐完成（均值归一化）\n");

    /* 6. 聚类 */
    printf("\n[聚类] 让模式自然涌现\n");
    float data[] = {0.1f, 0.15f, 0.2f, 0.9f, 0.95f, 5.0f, 5.1f};
    cluster_result cr = heyair_cluster(data, 7, 0.3f);
    printf("  数据: ");
    for (int i = 0; i < 7; i++) printf("%.2f ", data[i]);
    printf("\n");
    printf("  聚类数: %u\n", cr.count);
    for (uint32_t i = 0; i < cr.count; i++) {
        printf("    族%u: 质心=%.2f, 成员=", i, cr.clusters[i].centroid);
        for (uint32_t j = 0; j < cr.clusters[i].member_count; j++) {
            printf("%.2f ", data[cr.clusters[i].member_ids[j]]);
        }
        printf("\n");
    }

    printf("\n=== 演示完成 ===\n");
    return 0;
}

#endif /* BUILD_DEMO */
