/**
 * global_control_bridge_kernel.c — 全局控制容器 → 内核桥接
 *
 * 让内核和 Rust 核心通过全局控制容器来协调编码秩序。
 * 核心命题:
 *   符号用完后不重复重启, 从外部容器取符号遍历。
 *   匹配不靠编码关系, 靠发音位置和本质匹配。
 *
 * 这就是"聂小倩借尸还魂"的代码化:
 *   为鬼魂(符号本质)找到匹配的躯壳(另一容器中的符号),
 *   而不是按编码关系(大小写)找到有毒的蛇。
 */

#include <stdio.h>
#include <string.h>
#include "global_control_bridge_kernel.h"
#include "../../encoding/global_control.h"

/* ── 全局控制容器静态实例 ── */
static global_control_t gc;
static int initialized = 0;

/* ── 初始化(一次性) ── */
void global_control_bridge_init(void) {
    if (initialized) return;
    initialized = 1;

    global_control_init(&gc);

    /* 注册五个核心容器 */
    gc_register_container(&gc, 0x10, "声母", GC_POLARITY_YANG, 48);
    gc_register_container(&gc, 0x20, "韵母", GC_POLARITY_YIN, 66);
    gc_register_container(&gc, 0x30, "声调", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc, 0x40, "家族", GC_POLARITY_NEUTRAL, 5);
    gc_register_container(&gc, 0x50, "结构", GC_POLARITY_NEUTRAL, 8);

    /* 遍历规则: 声母→韵母 (优先级1) */
    gc_add_traversal(&gc, 0x10, 0x20, 1);
    /* 遍历规则: 声调→声母 (优先级2) */
    gc_add_traversal(&gc, 0x30, 0x10, 2);
    /* 遍历规则: 家族→韵母 (优先级3) */
    gc_add_traversal(&gc, 0x40, 0x20, 3);

    /* 判断规则: 发音位置匹配, 需要极性, 容差10 */
    gc_set_judgment(&gc, GC_MATCH_POSITION, 1, 0, 10);
}

/* ── 包装器: 转发到底层全局控制容器 ── */

int gcb_register_container(uint8_t id, const char *name,
                           uint8_t polarity, uint32_t boundary) {
    global_control_bridge_init();
    return gc_register_container(&gc, id, name, polarity, boundary);
}

int gcb_add_traversal(uint8_t src, uint8_t dst, uint8_t priority) {
    global_control_bridge_init();
    return gc_add_traversal(&gc, src, dst, priority);
}

int gcb_get_next_traversal(uint8_t *src_out, uint8_t *dst_out) {
    global_control_bridge_init();
    return gc_get_next_traversal(&gc, src_out, dst_out);
}

int gcb_advance_traversal(void) {
    global_control_bridge_init();
    return gc_advance_traversal(&gc);
}

void gcb_set_judgment(uint8_t match_type, uint8_t require_pol,
                       uint8_t require_aes, uint8_t tol) {
    global_control_bridge_init();
    gc_set_judgment(&gc, match_type, require_pol, require_aes, tol);
}

int gcb_judge_pair(uint32_t sym_a, uint32_t sym_b) {
    global_control_bridge_init();
    return gc_judge_pair(&gc, sym_a, sym_b);
}

/* ── 聂小倩: 为鬼魂找躯壳 ── */
int gcb_nie_xiaoqian(uint32_t ghost_code, uint32_t *body_out) {
    global_control_bridge_init();

    /* 从韵母容器(0x20)中找匹配的躯壳 */
    for (uint16_t yunmu = 0; yunmu < 66; yunmu++) {
        uint32_t candidate = ((uint32_t)0x20 << 8) | yunmu;
        int match = gc_judge_pair(&gc, ghost_code, candidate);
        if (match) {
            if (body_out) *body_out = candidate;
            return (int)yunmu;  /* 返回躯壳索引 */
        }
    }

    /* 没找到 → 进入恢复流程 */
    gc_recover(&gc);
    return -1;
}

/* ── 决策树: 恢复 ── */
int gcb_recover(void) {
    global_control_bridge_init();
    return gc_recover(&gc);
}

/* ── 独立运行入口 ── */
#ifdef BRIDGE_TEST
int main(void) {
    global_control_bridge_test();
    return 0;
}
#endif

/* ── 完整演示 ── */
void global_control_bridge_test(void) {
    global_control_bridge_init();

    printf("═══ 全局控制容器 · 内核桥接演示 ═══\n\n");

    /* 容器地图 */
    printf("─── 容器地图 ───\n");
    gc_report(&gc, stdout);

    /* 聂小倩 */
    printf("\n─── 聂小倩: 为鬼魂找躯壳 ───\n");
    uint32_t ghost = 0x102040;   /* 声母+韵母+声调 */
    uint32_t body = 0;
    int result = gcb_nie_xiaoqian(ghost, &body);
    if (result >= 0) {
        printf("  鬼魂(0x%06X) 找到躯壳: 韵母[%d] (0x%06X)\n",
               ghost, result, body);
        printf("  借尸还魂成功!\n\n");
    } else {
        printf("  未找到匹配躯壳, 进入恢复流程\n\n");
    }

    /* 遍历演示 */
    printf("─── 跨容器遍历: 声母→韵母 ───\n");
    uint8_t src, dst;
    int next = gcb_get_next_traversal(&src, &dst);
    if (next >= 0) {
        printf("  当前遍历规则: 0x%02X → 0x%02X\n", src, dst);
        printf("  推进遍历...\n");
        gcb_advance_traversal();
        printf("  推进完成\n\n");
    }

    /* 最终状态 */
    printf("─── 最终状态 ───\n");
    gc_report(&gc, stdout);
    printf("\n═══ 演示完成 ═══\n");
}
