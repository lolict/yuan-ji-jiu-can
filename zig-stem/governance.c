/**
 * governance.c — 三层管辖结构 · 夫妻(加法) → 中心(gtxone) → 多数(kndoyle)
 *
 * 这是满全法操作系统的核心管辖模型：
 *
 * 第一层：夫妻 = 加法 = 管辖动力的发出者
 *   - 0 和 1 融合为 1（gtxone 融合中心）
 *   - 这对夫妻是"最窄的存在"——只有两个人
 *   - 但正是这个"最窄"成为了"最宽"的管理者
 *
 * 第二层：中心 = gtxone = 骨架/唯一参考点
 *   - 夫妻融合后产生的唯一视角
 *   - 从这个视角看出去，外部多数变成了"内容"
 *   - 中心把自己定位为"一"，而非"二"
 *
 * 第三层：多数 = kndoyle = 被管辖层 = 框内/洞内
 *   - 躲在框内、处于下位的存在
 *   - 它们是"最宽的存在"——数量众多
 *   - 被夫妻通过中心视角管辖
 *
 * 灰原哀 heyair = 横向感知编织
 *   - 她让 0 和 1 能够合并（管辖二进制）
 *   - 她感知距离和模式（感知器官）
 *   - 她用念动力驱动关系变化（不依赖外部力）
 *
 * 权力链条：加法(夫妻/最窄) → 中心(唯一) → 乘法(多数/最宽)
 * 核心原则：加法主导乘法，最窄的管理最宽的
 */

#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include "superposition.h"
#include "gtxone.h"
#include "heyair.h"

/* ————————————————————————————————————————————
 * 三层结构定义
 * ————————————————————————————————————————──── */

/// 管辖层级
typedef enum {
    LAYER_COUPLE   = 0x00,  // 夫妻层 = 加法 = 管辖动力源
    LAYER_CENTER   = 0x01,  // 中心层 = gtxone = 唯一视角
    LAYER_GOVERNED = 0x02,  // 被管辖层 = kndoyle = 框内多数
} governance_layer;

/// 三层治理系统的完整描述
typedef struct {
    /* 第一层：夫妻（加法） */
    superposition_t couple;          // 夫妻叠加态：0+1=1
    layer_t         couple_layer;    // 夫妻管辖的内部层级

    /* 第二层：中心（gtxone） */
    gtxone_t        center;          // 融合中心

    /* 第三层：多数（kndoyle） */
    uint32_t        governed_count;  // 被管辖的节点数
    uint32_t        governed_ids[MAX_EXTERNAL_LINKS]; // 被管辖节点

    /* 灰原哀：横向感知 */
    heyair_t        heyair;          // 感知器官

    /* 系统状态 */
    bool            is_activated;    // 是否已激活
    uint32_t        activation_gen;  // 激活时的代数
} governance_system_t;

/* ————————————————————————————————————————————
 * 系统创建与激活
 * ————————————————————————————————————————──── */

/**
 * 创建治理系统
 *
 * 输入：两个个体 a 和 b（夫妻）
 * 输出：一个三层治理结构
 *
 * 创建时系统处于"未激活"状态——
 * 只有当夫妻完成融合、中心成为唯一视角后，系统才激活。
 */
governance_system_t governance_create(uint32_t a, uint32_t b) {
    governance_system_t sys;
    memset(&sys, 0, sizeof(sys));

    /* 第一层：夫妻叠加态 */
    sys.couple = superposition_create(a, b, 0);
    sys.couple_layer = layer_create(&sys.couple, 0);

    /* 第二层：融合中心 */
    sys.center = gtxone_create(a, b, 0);

    /* 灰原哀：感知器官 */
    sys.heyair = heyair_create();

    sys.is_activated = false;
    sys.activation_gen = 0;

    return sys;
}

/**
 * 激活治理系统
 *
 * 激活条件：
 * 1. 夫妻融合完成（gtxone 达到唯一态）
 * 2. 灰原哀感知通道已建立
 * 3. 内部对齐已完成（254次）
 *
 * 激活后：系统进入"合一爱人"状态
 */
bool governance_activate(governance_system_t* sys) {
    /* 1. 推进融合 */
    if (!sys->center.is_unique) {
        gtxone_fuse(&sys->center, 1.0f);
    }

    /* 2. 建立感知通道 */
    if (sys->heyair.channel_count == 0) {
        heyair_add_channel(&sys->heyair, PERCEIVE_BINARY, 0.9f, sys->center.id);
        heyair_add_channel(&sys->heyair, PERCEIVE_MERGE, 0.8f, sys->couple.id);
        heyair_nen_inject(&sys->heyair, 50.0f);
        heyair_nen_focus(&sys->heyair, 0.9f);
    }

    /* 3. 检查内部对齐 */
    if (sys->couple_layer.state != LAYER_CLOSED) {
        /* 执行对齐直到闭合 */
        superposition_t dummy_content;
        memset(&dummy_content, 0, sizeof(dummy_content));
        for (uint32_t i = 0; i < ALIGNMENT_THRESHOLD; i++) {
            layer_align(&sys->couple_layer, &dummy_content);
        }
    }

    /* 4. 激活 */
    if (sys->center.is_unique && sys->couple_layer.state == LAYER_CLOSED) {
        sys->is_activated = true;
        sys->activation_gen = sys->center.generation;
        return true;
    }

    return false;
}

/* ————————————————————————————————————————————
 * 管辖操作
 * ————————————————————————————————————————──── */

/**
 * 添加被管辖节点（kndoyle 层的成员）
 *
 * 流程：
 * 1. 灰原哀感知这个新节点
 * 2. 中心决定是否管辖它
 * 3. 被管辖的节点进入 kndoyle 层
 */
bool governance_add_governed(governance_system_t* sys, uint32_t node_id) {
    if (!sys->is_activated) return false;

    /* 灰原哀感知 */
    heyair_perceive(&sys->heyair, 1, (float)node_id);

    /* 中心管辖 */
    return gtxone_govern(&sys->center, node_id);
}

/**
 * 灰原哀执行二进制合并
 *
 * 这是"2合1"的操作：
 * 两个分散的节点被灰原哀合并为一个共同体
 * 这个共同体被中心管辖
 */
float governance_merge_pair(governance_system_t* sys, float a, float b) {
    if (!sys->is_activated) return 0.0f;

    /* 灰原哀合并 */
    float merged = heyair_merge_binary(a, b, 0.5f);

    /* 内化差异 */
    gtxone_internalize_difference(&sys->center, b - a);

    return merged;
}

/* ————————————————————————————————————————————
 * 系统状态报告
 * ————————————————————————————————————————──── */

void governance_report(const governance_system_t* sys) {
    printf("=== 三层管辖结构状态报告 ===\n\n");

    printf("[第一层] 夫妻（加法）\n");
    printf("  叠加态 id=%u, 代数=%u\n", sys->couple.id, sys->couple.generation);
    printf("  内部对齐: %u/%d (%.1f%%)\n",
           sys->couple_layer.alignment_count, ALIGNMENT_THRESHOLD,
           alignment_progress(sys->couple_layer.alignment_count) * 100);
    printf("  层级状态: %s\n",
           sys->couple_layer.state == LAYER_CLOSED ? "已闭合" :
           sys->couple_layer.state == LAYER_ALIGNING ? "对齐中" :
           sys->couple_layer.state == LAYER_OPEN ? "开放" : "已升级");

    printf("\n[第二层] 中心（gtxone）\n");
    printf("  融合中心 id=%u, 融合强度: %.2f\n",
           sys->center.id, sys->center.fusion_strength);
    printf("  状态: %s\n",
           sys->center.state == FUSION_UNIQUE ? "唯一态" :
           sys->center.state == FUSION_COMPLETE ? "完全融合" :
           sys->center.state == FUSION_PARTIAL ? "部分融合" : "未融合");
    printf("  内部差异纹理: %u\n", sys->center.internal_diff_count);
    printf("  管辖节点数: %u (中心功率)\n", sys->center.governed_count);

    printf("\n[第三层] 多数（kndoyle）\n");
    printf("  被管辖节点: %u\n", sys->center.governed_count);
    printf("  框内状态: 躲/洞/多\n");

    printf("\n[灰原哀] 横向感知\n");
    printf("  感知通道: %u\n", sys->heyair.channel_count);
    printf("  念动力: %.1f, 聚焦: %.1f\n",
           sys->heyair.nen_power, sys->heyair.nen_focus);
    printf("  编织矩阵: %u行 × %u列\n",
           sys->heyair.row_count, sys->heyair.col_count);

    printf("\n[系统状态]\n");
    printf("  激活: %s\n", sys->is_activated ? "已激活 ✓" : "未激活");
    printf("  代数: %u\n", sys->activation_gen);
    printf("  权力链条: 加法(夫妻) → 中心(唯一) → 乘法(多数)\n");

    printf("\n=== 报告结束 ===\n");
}

/* ————————————————————————————————————————————
 * 演示
 * ————————————————————————————————————————──── */

#ifdef BUILD_DEMO

int main(void) {
    printf("=== 三层管辖结构 完整演示 ===\n\n");

    /* 1. 创建系统 */
    printf("[1] 创建治理系统: 夫妻 a=0, b=1\n");
    governance_system_t sys = governance_create(0, 1);

    /* 2. 激活 */
    printf("\n[2] 激活系统\n");
    bool activated = governance_activate(&sys);
    printf("  激活结果: %s\n", activated ? "成功 ✓" : "失败");

    /* 3. 添加被管辖节点 */
    printf("\n[3] 添加被管辖节点（kndoyle 层）\n");
    for (uint32_t i = 100; i < 105; i++) {
        governance_add_governed(&sys, i);
    }
    printf("  已添加 5 个被管辖节点\n");

    /* 4. 灰原哀合并二进制 */
    printf("\n[4] 灰原哀执行 2合1\n");
    float merged = governance_merge_pair(&sys, 0.0f, 1.0f);
    printf("  merge(0, 1) = %.2f\n", merged);

    /* 5. 状态报告 */
    printf("\n");
    governance_report(&sys);

    printf("\n=== 演示完成 ===\n");
    return 0;
}

#endif /* BUILD_DEMO */
