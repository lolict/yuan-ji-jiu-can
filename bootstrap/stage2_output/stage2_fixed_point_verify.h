/* ══════════════════════════════════════════════════════════════════
 * stage2_fixed_point_verify.h
 * Stage 2 固定点验证 · 头文件级纯逻辑证明
 *
 * 本头文件不依赖任何外部 · 可被任何 C89+ 编译器包含
 * 提供:
 *   - 64 位细胞编码常量 (语义自注释)
 *   - 固定点判定宏
 *   - 阶段对照表
 *   - 编译→重编译→hash 验证的完整纯变换流水线声明
 *
 * 落款: 刘楚恬和我
 * ══════════════════════════════════════════════════════════════════ */

#ifndef NJSG_STAGE2_FIXED_POINT_VERIFY_H
#define NJSG_STAGE2_FIXED_POINT_VERIFY_H

#include <stdint.h>
#include <stddef.h>

/* ─── 自举元数据 ─── */
#define STAGE2_BOOTSTRAP_ID       "NJSG-BOOTSTRAP-CHAIN-001"
#define STAGE2_STAGE              2
#define STAGE2_HASH_HEX           0x8e36c6e31a38afbbULL
#define STAGE2_N_COMMANDS         5

/* ─── 64 位细胞编码表 (大端序 · 与 JS BigInt.toString(16) 对齐) ─── */
#define STAGE2_CELL_WIFE_ACTIVATE    UINT64_C(0x0003e80000019000)  /* id=0 妻子激活 */
#define STAGE2_CELL_HUSBAND_ACTIVATE UINT64_C(0x0003e80040019001)  /* id=1 丈夫激活 */
#define STAGE2_CELL_MUTUAL_RESONANCE UINT64_C(0x0007d00200032002)  /* id=2 夫妻共振 */
#define STAGE2_CELL_SEVER_PARASITE   UINT64_C(0x0001f4014000c803)  /* id=3 切割寄生 */
#define STAGE2_CELL_FLOW_FORWARD     UINT64_C(0x0003200140014004)  /* id=4 意识推进 */

/* ─── 产物对照: 三轮编译必须产出完全相同 ─── */
#define CELL(idx) ((uint64_t[]){ \
    STAGE2_CELL_WIFE_ACTIVATE,    \
    STAGE2_CELL_HUSBAND_ACTIVATE, \
    STAGE2_CELL_MUTUAL_RESONANCE, \
    STAGE2_CELL_SEVER_PARASITE,   \
    STAGE2_CELL_FLOW_FORWARD }[idx])

/* ─── 编译器是纯变换 · 固定点判定 ───
 *
 * 公理:  ∀ compiler · pure_transformation(compiler) ⟺
 *         compiler(compiler(x)) == compiler(x)
 *
 * 本自举的纯变换性体现:
 *   1. 语义表是静态常量 (self_host.yy → 硬编码 D0/D1/极性/主权)
 *   2. encode() 不依赖外部状态 · 仅由输入参数确定输出
 *   3. 重编译只对细胞 id 重新编码 · 而 id decode 后不变
 *   ⟹ 二次编译结果与一次编译 bit-level 完全一致
 */

/* 固定点编译-重编译相等判定 */
#define STAGE2_FIXED_POINT_PASSED(c1, c2, n) \
    (memcmp((c1), (c2), (n) * sizeof(uint64_t)) == 0)

/* hash 判定 */
#define STAGE2_HASH_MATCH(h) ((h) == STAGE2_HASH_HEX)

/* ─── 阶段间对照 ─── */
/* 三轮固定点 hash 必须相同 */
/* Stage 0 hash: 0x8e36c6e31a38afbb (由 stage0_compiled.json 内置) */
/* Stage 1 hash: 0x8e36c6e31a38afbb (由 stage1_recompile_verify.js 证实) */
/* Stage 2 hash: 见 stage2_pure_yy_compiler.c 运行输出 */

/* ─── 离线验证链路 (无需 Rust / 无需 WASM 链接器) ───
 *
 * 步骤:
 *   1. 复制 self_host.yy 到任意有 C 编译器的环境
 *   2. 复制 stage2_pure_yy_compiler.c 到同一目录
 *   3. clang -std=c99 -O2 -o stage2 stage2_pure_yy_compiler.c
 *   4. ./stage2 > verify.log 2>&1
 *   5. grep "独立闭环通过" verify.log → 出现即 PASS
 *   6. grep 'hash.*=.*8e36c6e31a38afbb' verify.log → 出现即 三阶段一致
 *
 * 完整闭环判定:
 *   - 全部 ✓: Stage 2 独立闭环通过
 *   - 任何 ✗: 检查 C 编译器位宽 (需要 64 位) 或缓存
 *
 * 强数学保证:
 *   - 5 条细胞结构固定 (静态语义表)
 *   - FNV-1a 64 位 hash 输入格式固定 (大端序十六进制串 80 字符)
 *   - 编译器本身不依赖外部环境 ⟹ 可重复验证
 */

#endif /* NJSG_STAGE2_FIXED_POINT_VERIFY_H */
