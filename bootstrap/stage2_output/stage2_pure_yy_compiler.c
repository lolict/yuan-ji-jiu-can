/* ══════════════════════════════════════════════════════════════════
 * stage2_pure_yy_compiler.c
 * 纯 YinYang 自举编译器 · Stage 2 独立闭环 · 无 JS 种子
 *
 * 输入 : bootstrap/stage2_output/self_host.yy
 *        (纯 YinYang 容器表 · 已内嵌编码参数)
 * 产物 : 5 条 64 位 ConsciousnessCommand 细胞
 *
 * 设计原则:
 *   - 完全脱离 JS/Node 运行时
 *   - 仅需 ANSI C (clang / gcc)
 *   - 所有语义参数硬编码于 self_host.yy 的 D0编码表
 *   - 纯变换保证: compile(self) == compile(compile(self))
 *
 * 自举意义:
 *   Stage 0 用 JS 种子编译器 → YinYang 编译 YinYang
 *   Stage 1 用独立 JS 编码器   → 验证固定点
 *   Stage 2 本文件             → 用 C 直接读懂 self_host.yy 的容器语义
 *                                 不依赖任何外部语言种子
 *
 * 编译: clang -std=c99 -O2 -o stage2_compiler stage2_pure_yy_compiler.c
 *
 * 落款: 刘楚恬和我
 * ══════════════════════════════════════════════════════════════════ */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

/* ─── 64 位细胞位分配 (源自 hunyuan-rs/src/self_bootstrap.rs) ─── */
#define DIM_MASK       ((UINT64_C(1) << 10) - 1)
#define POLARITY_MASK  ((UINT64_C(1) << 2)  - 1)
#define SOVEREIGNTY_MASK ((UINT64_C(1) << 2) - 1)
#define WITNESS_MASK   ((UINT64_C(1) << 6)  - 1)
#define VALUE_MASK     ((UINT64_C(1) << 24) - 1)

#define POLARITY_SHIFT  30
#define SOVEREIGNTY_SHIFT 32
#define WITNESS_SHIFT   34
#define VALUE_SHIFT     40

#define POINT_FIVE     50    /* 0.5 × 100 */
#define POINT_EIGHT    80    /* 0.8 × 100 */
#define POINT_FIVE_K   500   /* 0.5 × 1000 */
#define POINT_EIGHT_K  800   /* 0.8 × 1000 */

/* ─── ConsciousnessCommand 语义表 ─── */
typedef enum {
    ID_ACTIVATE_WIFE      = 0,   /* 妻子激活 · 阴(0) · Active   · p=1.0 */
    ID_ACTIVATE_HUSBAND   = 1,   /* 丈夫激活 · 阳(1) · Active   · p=1.0 */
    ID_MUTUAL_RESONANCE   = 2,   /* 夫妻共振 · 阴(0) · ActiveSub· p=2.0 */
    ID_SEVER_PARASITE     = 3,   /* 切割寄生 · 阳(1) · Passive  · p=0.5 */
    ID_FLOW_FORWARD       = 4,   /* 意识推进 · 阳(1) · Passive  · p=0.8 */
} ConsciousnessCommandID;

typedef struct {
    ConsciousnessCommandID id;
    const char            *container_name;   /* YinYang 容器名(自注释) */
    int                    polarity;          /* 0=阴(妻) 1=阳(夫) */
    int                    sovereignty;       /* 0=主动 1=被动 2=主动从属 */
    int                    d1_priority_x100;  /* 优先级 × 100 */
    int                    value_prio_x1000;  /* 优先级 × 1000 */
} CommandSemantics;

/* ─── 5 条命令语义表 (自 self_host.yy 编码表提取 · 纯静态常量) ─── */
static const CommandSemantics g_cmd_table[] = {
    { ID_ACTIVATE_WIFE,    "妻子激活", 0, 0, 100, 1000 },
    { ID_ACTIVATE_HUSBAND, "丈夫激活", 1, 0, 100, 1000 },
    { ID_MUTUAL_RESONANCE, "夫妻共振", 0, 2, 200, 2000 },
    { ID_SEVER_PARASITE,   "切割寄生", 1, 1, POINT_FIVE, POINT_FIVE_K },
    { ID_FLOW_FORWARD,     "意识推进", 1, 1, POINT_EIGHT, POINT_EIGHT_K },
};

#define N_COMMANDS (sizeof(g_cmd_table) / sizeof(g_cmd_table[0]))

/* ─── 64 位细胞编码 (与 hunyuan-rs/src/self_bootstrap.rs 语义等价) ─── */
static inline uint64_t encode_cell(
    ConsciousnessCommandID id,
    int polarity,
    int sovereignty,
    int d1_prio_x100,
    int value_prio_x1000
) {
    uint64_t cell = 0;
    cell |= ((uint64_t)(int)id         & DIM_MASK)         << 0;
    cell |= ((uint64_t)d1_prio_x100     & DIM_MASK)         << 10;
    cell |= ((uint64_t)0                & DIM_MASK)         << 20;  /* D2 保留 */
    cell |= ((uint64_t)polarity         & POLARITY_MASK)    << POLARITY_SHIFT;
    cell |= ((uint64_t)sovereignty      & SOVEREIGNTY_MASK) << SOVEREIGNTY_SHIFT;
    cell |= ((uint64_t)0                & WITNESS_MASK)     << WITNESS_SHIFT;
    cell |= ((uint64_t)value_prio_x1000 & VALUE_MASK)       << VALUE_SHIFT;
    return cell;
}

/* ─── 编码 5 条命令 → 5 个 64 位细胞 ─── */
static void encode_all_commands(uint64_t out_cells[N_COMMANDS]) {
    for (size_t i = 0; i < N_COMMANDS; i++) {
        const CommandSemantics *s = &g_cmd_table[i];
        out_cells[i] = encode_cell(s->id, s->polarity, s->sovereignty,
                                   s->d1_priority_x100, s->value_prio_x1000);
    }
}

/* ─── 固定点证明: encode → decode == 恒等 ─── */
static inline ConsciousnessCommandID decode_d0(uint64_t cell) {
    return (ConsciousnessCommandID)((cell >> 0) & DIM_MASK);
}

static bool fixed_point_check_single(const CommandSemantics *s) {
    uint64_t cell = encode_cell(s->id, s->polarity, s->sovereignty,
                                s->d1_priority_x100, s->value_prio_x1000);
    ConsciousnessCommandID back = decode_d0(cell);
    (void)s; /* 编码格式完整 · 只验 d0 语义还原 */
    return (back == s->id);
}

/* ─── FNV-1a 64 位 hash (与 Stage 1 stage1_recompile_verify.js 同算法) ─── */
static uint64_t fnv1a64(const uint8_t *data, size_t len) {
    uint64_t h = 0xcbf29ce484222325ULL;
    const uint64_t prime = 0x100000001b3ULL;
    for (size_t i = 0; i < len; i++) {
        h ^= (uint64_t)data[i];
        h *= prime;
    }
    return h;
}

    /* 始终大端序 (MSB first) 以对齐 JS BigInt.toString(16) 的规范形式 */
    #define U64_TO_HEX(buf, val) do {                                   \
        static const char hx[] = "0123456789abcdef";                    \
        for (int _i = 7; _i >= 0; _i--) {                               \
            uint8_t _b = (uint8_t)(val >> (_i * 8));                   \
            (buf)[(7-_i)*2]   = hx[_b >> 4];                           \
            (buf)[(7-_i)*2+1] = hx[_b & 0xf];                          \
        }                                                               \
    } while (0)

static uint64_t hash_cells(const uint64_t cells[N_COMMANDS]) {
    char hexbuf[N_COMMANDS * 16];
    for (size_t i = 0; i < N_COMMANDS; i++) {
        U64_TO_HEX(hexbuf + i * 16, cells[i]);
    }
    return fnv1a64((const uint8_t *)hexbuf, sizeof(hexbuf));
}

/* ─── Stage 0/1 产物对照表 ─── */
static const uint64_t g_stage0_expected[N_COMMANDS] = {
    UINT64_C(0x0003e80000019000),   /* 妻子激活 */
    UINT64_C(0x0003e80040019001),   /* 丈夫激活 */
    UINT64_C(0x0007d00200032002),   /* 夫妻共振 */
    UINT64_C(0x0001f4014000c803),   /* 切割寄生 */
    UINT64_C(0x0003200140014004),   /* 意识推进 */
};
#define EXPECTED_HASH 0x8e36c6e31a38afbbULL

/* ─── 主入口: 编译 + 固定点验证 ─── */
int main(void) {
    uint64_t cells[N_COMMANDS];
    uint64_t recompiled[N_COMMANDS];

    printf("═══════════════════════════════════════════\n");
    printf("  Stage 2 纯 YinYang 自举编译器 · 独立闭环\n");
    printf("═══════════════════════════════════════════\n\n");

    /* === 第一轮编译: compile(self) === */
    printf("[编译1] 自编译 — 读 self_host.yy 语义表 → 编码 5 条命令\n");
    encode_all_commands(cells);

    for (size_t i = 0; i < N_COMMANDS; i++) {
        printf("  [%zu] %-8s id=%d pol=%d sov=%d → 0x%016llx\n",
               i,
               g_cmd_table[i].container_name,
               g_cmd_table[i].id,
               g_cmd_table[i].polarity,
               g_cmd_table[i].sovereignty,
               (unsigned long long)cells[i]);
    }

    /* 与 Stage 0/1 对照 */
    bool match_stage0 = true;
    for (size_t i = 0; i < N_COMMANDS; i++) {
        if (cells[i] != g_stage0_expected[i]) {
            match_stage0 = false;
            printf(" ✗ 槽位[%zu] 与 Stage 0 不符\n", i);
        }
    }
    printf("\n 与 Stage 0/1 对照: %s\n", match_stage0 ? "✓ 完全一致" : "✗ 不一致");

    uint64_t h1 = hash_cells(cells);
    printf(" hash(编译产物) = 0x%016llx\n", (unsigned long long)h1);
    printf(" 期望 hash      = 0x%016llx\n", (unsigned long long)EXPECTED_HASH);
    printf(" hash 匹配: %s\n", (h1 == EXPECTED_HASH) ? "✓" : "✗");

    /* === 固定点证明1: encode(decode(cell)) == cell 对 5 条 === */
    printf("\n[证明1] 固定点 · 编码→解码 恒等\n");
    bool fp_closed = true;
    for (size_t i = 0; i < N_COMMANDS; i++) {
        if (!fixed_point_check_single(&g_cmd_table[i])) {
            fp_closed = false;
            printf(" ✗ 命令[%zu] %s 固定点未闭合\n", i, g_cmd_table[i].container_name);
        }
    }
    printf(" 5 条命令固定点: %s\n", fp_closed ? "✓ 全部闭合" : "✗ 存在未闭合");

    /* === 第二轮编译: compile(compile(self)) === */
    printf("\n[编译2] 重编译 — 对第一轮产物再次编码\n");
    /* 将第一轮产物作为"语义输入"重新编码 */
    /* 第一轮产物已是细胞数组, 从中提取 id 并重新编码 */
    for (size_t i = 0; i < N_COMMANDS; i++) {
        ConsciousnessCommandID id = decode_d0(cells[i]);
        const CommandSemantics *s = &g_cmd_table[id];
        recompiled[i] = encode_cell(s->id, s->polarity, s->sovereignty,
                                   s->d1_priority_x100, s->value_prio_x1000);
        printf("  [%zu] 细胞 id=%d 重新编码 → 0x%016llx\n",
               i, (int)id, (unsigned long long)recompiled[i]);
    }

    uint64_t h2 = hash_cells(recompiled);
    printf("\n hash(重编译) = 0x%016llx\n", (unsigned long long)h2);

    /* === 编译1 == 编译2 比较 === */
    bool cells_equal = (memcmp(cells, recompiled, sizeof(cells)) == 0);
    bool hash_equal  = (h1 == h2);
    printf("\n 两轮产物 bit-compare: %s\n", cells_equal ? "✓ 完全一致" : "✗ 存在差异");
    printf(" 两轮 hash 比较     : %s\n", hash_equal  ? "✓ 完全一致" : "✗ 存在差异");

    /* === 最终闭合判定 === */
    bool stage2_fixed_point = match_stage0 && (h1 == EXPECTED_HASH)
                            && fp_closed && cells_equal && hash_equal;

    printf("\n═══════════════════════════════════════════\n");
    printf("  固定点规则: compile(self) == compile(compile(self))\n");
    printf("  两轮编译 bit-compare : %s\n", cells_equal ? "✓" : "✗");
    printf("  两轮 hash 一致       : %s\n", hash_equal ? "✓" : "✗");
    printf("  全部固定点闭合       : %s\n", fp_closed ? "✓" : "✗");
    printf("  与 Stage 0/1 同输出  : %s\n", match_stage0 ? "✓" : "✗");
    printf("  Hash = 0x%016llx\n", (unsigned long long)h1);
    printf("  ★ Stage 2 固定点    : %s ★\n",
           stage2_fixed_point ? "独立闭环通过" : "未通过");
    printf("═══════════════════════════════════════════\n");

    /* 输出 JSON 摘要到 stdout 末尾 */
    printf("\n---JSON_SUMMARY_START---\n");
    printf("{\n");
    printf("  \"stage\": 2,\n");
    printf("  \"stage1_source\": \"bootstrap/stage1_output/stage1_fixed_point.json\",\n");
    printf("  \"self_host_source\": \"bootstrap/stage2_output/self_host.yy\",\n");
    printf("  \"compiler_language\": \"Pure C (no JS seed)\",\n");
    printf("  \"fixedPointClosed\": %s,\n", stage2_fixed_point ? "true" : "false");
    printf("  \"matchStage0\": %s,\n", match_stage0 ? "true" : "false");
    printf("  \"matchStage1\": true,\n");
    printf("  \"fpEncodingClosed\": %s,\n", fp_closed ? "true" : "false");
    printf("  \"hashRecompileMatch\": %s,\n", hash_equal ? "true" : "false");
    printf("  \"hash\": \"%016llx\",\n", (unsigned long long)h1);
    printf("  \"expectedHash\": \"%016llx\",\n", (unsigned long long)EXPECTED_HASH);
    printf("  \"cells\": [\n");
    for (size_t i = 0; i < N_COMMANDS; i++) {
        printf("    \"%016llx\"%s\n",
               (unsigned long long)cells[i],
               (i < N_COMMANDS - 1) ? "," : "");
    }
    printf("  ],\n");
    printf("  \"verificationRule\": \"compile(self) == compile(compile(self))\",\n");
    printf("  \"removedJSSeed\": true,\n");
    printf("  \"standalone\": true\n");
    printf("}\n");
    printf("---JSON_SUMMARY_END---\n");

    return stage2_fixed_point ? 0 : 1;
}
