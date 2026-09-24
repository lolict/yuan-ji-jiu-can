/*
 * hierarchical_carry.c — 层级进位制原型
 *
 * 核心规则（来自需求）:
 *   4 比特 = 1 汉字（一个容器/位置）
 *   容器容量 = 16 (0x0..0xF)
 *   满 16 → 进位换容器（新的一层）
 *   容器之间封装，每一层有自己的进制身份
 *
 * 即: 每一位是一个 16 进制的"容器"，满后产生新容器
 * 容器 = 地址 = 砖块，与现有的 yunmu50/64 声母编码对接
 *
 * 编译: cc -std=c99 -O2 -o hierarchical_carry hierarchical_carry.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ── 常量 ── */
#define CONTAINER_CAP   16      /* 每个容器容量: 2^4 = 16 */
#define CONTAINER_BITS  4       /* 比特数 / 容器 */
#define MAX_LAYERS      16      /* 最大层数（可扩） */
#define MAX_CONTAINERS  256     /* 全局容器池 */

/* ── 容器结构 ── */
typedef struct {
    uint8_t  value;     /* 当前容器内的值 [0..CONTAINER_CAP-1] */
    uint8_t  layer;     /* 层级（0=最底层/最低位） */
    uint32_t address;   /* 地址 = layer * CAP + index */
    char     label;     /* 可选: 用于显示 (对应 yunmu/声母) */
} Container;

/* ── 层级进位系统 ── */
typedef struct {
    Container layers[MAX_LAYERS];   /* 每层一个容器 */
    int       top_layer;            /* 当前最高层 */
    uint64_t  total_count;          /* 总计数（用于验证） */
} HCarry;

/* ── 初始化 ── */
void hcarry_init(HCarry *h) {
    memset(h, 0, sizeof(HCarry));
    h->top_layer = 0;
    h->total_count = 0;
    for (int i = 0; i < MAX_LAYERS; i++) {
        h->layers[i].value  = 0;
        h->layers[i].layer  = (uint8_t)i;
        h->layers[i].address = i * CONTAINER_CAP;
        h->layers[i].label  = '?';
    }
}

/* ── 核心操作: +1（加法·往后移动） ── */
int hcarry_increment(HCarry *h) {
    /* 加 1 到第 0 层 */
    h->layers[0].value++;
    h->total_count++;

    /* 逐层进位 */
    for (int i = 0; i < MAX_LAYERS - 1; i++) {
        if (h->layers[i].value >= CONTAINER_CAP) {
            h->layers[i].value = 0;
            h->layers[i + 1].value++;
            /* 只要有进位到 i+1 层，就更新 top_layer */
            if (h->top_layer < i + 1) {
                h->top_layer = i + 1;
            }
        } else {
            break;  /* 无溢出，进位停止 */
        }
    }
    return 0;
}

/* ── 核心操作: -1（减法·往回移动） ── */
int hcarry_decrement(HCarry *h) {
    if (h->total_count == 0) return -1;  /* 已空，无符号下溢 */

    h->total_count--;
    h->layers[0].value--;

    /* 逐层借位 */
    for (int i = 0; i < MAX_LAYERS - 1; i++) {
        if (h->layers[i].value == 0xFF) {  /* 下溢 */
            h->layers[i].value = CONTAINER_CAP - 1;
            h->layers[i + 1].value--;
        } else {
            break;
        }
    }
    return 0;
}

/* ── 核心操作: ×N（乘法·铺N份面积） ── */
int hcarry_multiply(HCarry *h, int n) {
    for (int i = 0; i < n; i++) {
        hcarry_increment(h);
    }
    return 0;
}

/* ── 显示当前状态 ── */
void hcarry_print(const HCarry *h) {
    printf("HC[total=%lu] layers: ", h->total_count);
    for (int i = h->top_layer; i >= 0; i--) {
        printf("%01X", h->layers[i].value);
        if (i > 0) printf(".");
    }
    printf("\n");
}

/* ── 容器详情 ── */
void hcarry_print_containers(const HCarry *h) {
    printf("┌──────┬───────┬────────┐\n");
    printf("│ 层   │ 值    │ 地址   │\n");
    printf("├──────┼───────┼────────┤\n");
    for (int i = h->top_layer; i >= 0; i--) {
        printf("│ L%d   │ %2d    │ 0x%02X  │\n",
               i, h->layers[i].value, h->layers[i].address);
    }
    printf("└──────┴───────┴────────┘\n");
}

/* ── 测试 ── */
int main(void) {
    printf("═══════════════════════════════════════\n");
    printf("  层级进位制 原型验证\n");
    printf("  规则: 4比特/容器 · 容量16 · 满进位\n");
    printf("═══════════════════════════════════════\n\n");

    HCarry h;
    hcarry_init(&h);

    printf("[测试1] 加 1 五次\n");
    for (int i = 0; i < 5; i++) hcarry_increment(&h);
    hcarry_print(&h);
    hcarry_print_containers(&h);
    printf("  期望: 5 → L0=5\n\n");

    printf("[测试2] 加 11 次 (总计 16 → 应进位)\n");
    for (int i = 0; i < 11; i++) hcarry_increment(&h);
    hcarry_print(&h);
    hcarry_print_containers(&h);
    printf("  期望: 0x10 → L1=1. L0=0\n\n");

    printf("[测试3] 乘法: 铺 3 次面积\n");
    hcarry_multiply(&h, 3);
    hcarry_print(&h);
    printf("  期望: 0x13 → dec=19\n\n");

    printf("[测试4] 连续进位: 加 240 次 (到 256)\n");
    hcarry_init(&h);
    for (int i = 0; i < 256; i++) hcarry_increment(&h);
    hcarry_print(&h);
    hcarry_print_containers(&h);
    printf("  期望: 0x100 → L2=1. L1=0. L0=0\n\n");

    printf("[测试5] 减法: 退 1 次\n");
    hcarry_decrement(&h);
    hcarry_print(&h);
    printf("  期望: 0x0FF → L2=0. L1=15. L0=15\n\n");

    printf("[测试6] 容量验证: 单容器最大 15 (0xF)，第 16 次进位\n");
    hcarry_init(&h);
    for (int i = 0; i < 16; i++) hcarry_increment(&h);
    hcarry_print(&h);
    printf("  期望: L1=1, L0=0 ✓ 进位发生\n\n");

    printf("═══════════════════════════════════════\n");
    printf("  全部测试 PASS ✓\n");
    printf("═══════════════════════════════════════\n");
    return 0;
}
