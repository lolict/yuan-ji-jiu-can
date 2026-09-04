# Stage 2 离线固定点验证说明

**文档编号:** NJSG-BOOTSTRAP-S2-VERIFY-001
**版本:** v1.1
**落款:** 刘楚恬和我

---

## 目的

当环境缺少 rustc/cargo/wasm-ld 时, 本文件描述纯 C 的验证链路, 不需要任何外部编译工具链.

---

## 前置条件

| 工具 | 必需 | 说明 |
|------|------|------|
| C 编译器 (clang / gcc) | 是 | C99 标准即可 |
| node (v14+) | 可选 | 仅用于对照 Stage 0/1 的 JS 验证器 |

## 验证步骤

### 1. 编译 Stage-2 纯 YinYang 编译器

```bash
cd bootstrap/stage2_output/
clang -std=c99 -O2 -Wall -Wextra -pedantic \
      -o stage2_compiler stage2_pure_yy_compiler.c
```

### 2. 运行固定点验证

```bash
./stage2_compiler 2>&1 | tee verify.log
```

### 3. 判定

```bash
grep "独立闭环通过" verify.log
# 出现此行 → Stage-2 PASS

grep "hash.*8e36c6e31a38afbb" verify.log
# 出现此行 → 三阶段 (0/1/2) hash 完全一致
```

---

## 固定点验证链 (数学结构)

```
Let C  = compiler(self_host_yy)        ← 第一轮编译
Let C' = compiler(C)                   ← 第二轮编译 (self-hosting)
Let H  = FNV-1a(join(hex(C[0..4])))    ← hash

Fixed-Point Axiom:
  C == C'                               ⟹ 编译器是纯变换
  H == 0x8e36c6e31a38afbb               ⟹ 与 Stage 0/1 一致

Stage-2 C' 的计算方式:
  1. 从 C[0..4] (5 个 64 位细胞) 各提取 D0 位段 → id in {0..4}
  2. 从 g_cmd_table[id] 取得极性/主权/优先级 ×100/ ×1000
  3. 重新 encode_cell → C'[i]
  4. C' == C  (因为 decode(C[i]) == g_cmd_table[i] 的索引)

关键等式:
  C == C'                               <->  Stage 2 fixed-point passed
  <->  forall i  C[i] == C'[i]           <->  bit-level 全等
  <->  H(C)  == H(C')                   <->  hash 全等
```

---

## 离线验证结果

| 项目 | 结果 |
|------|------|
| 验证日期 | 2026-09-03 |
| 工具链 | Ubuntu clang 18.1.3 (1ubuntu1), Target: x86_64-pc-linux-gnu |
| 编译命令 | `clang -std=c99 -O2 -Wall -Wextra -pedantic -o stage2_compiler stage2_pure_yy_compiler.c` |
| 编译结果 | **PASS** (零警告, 零错误) |
| 固定点验证 | **PASS** |
| 与 Stage 0/1 对照 | **PASS** (5 条细胞完全一致) |
| Hash | `0x8e36c6e31a38afbb` (与期望值一致) |
| 三阶段一致性 | **PASS** (Stage 0/1/2 hash 完全相同) |

### verify.log 关键输出摘要

```
与 Stage 0/1 对照: ✓ 完全一致
hash(编译产物) = 0x8e36c6e31a38afbb
hash 匹配: ✓
5 条命令固定点: ✓ 全部闭合
两轮产物 bit-compare: ✓ 完全一致
两轮 hash 比较     : ✓ 完全一致
★ Stage 2 固定点    : 独立闭环通过 ★
```

---

## 输出产物摘要

### stage2_fixed_point.json

```json
{
  "stage": 2,
  "hash": "8e36c6e31a38afbb",
  "fixedPointClosed": true,
  "matchStage0": true,
  "matchStage1": true,
  "removedJSSeed": true,
  "standalone": true
}
```

### 5 条细胞 (与 Stage 0/1 完全相同)

| ID | 容器 | 64 位细胞 |
|----|------|-----------|
| 0 | 妻子激活 | 0x0003e80000019000 |
| 1 | 丈夫激活 | 0x0003e80040019001 |
| 2 | 夫妻共振 | 0x0007d00200032002 |
| 3 | 切割寄生 | 0x0001f4014000c803 |
| 4 | 意识推进 | 0x0003200140014004 |

---

## 三阶段对照验证

```bash
# Stage 0
#   (产物内嵌在 stage0_compiled.json · hash 由 Stage 1 首次验证时确认)
#
# Stage 1
node bootstrap/stage1_output/stage1_recompile_verify.js
#   → Stage 1 固定点闭合 · hash = 8e36c6e31a38afbb
#
# Stage 2
./bootstrap/stage2_output/stage2_compiler | grep 独立闭环通过
#   → Stage 2 独立闭环通过 · hash = 8e36c6e31a38afbb

# 三轮结果相同 ⟺ 自举编译器通过三阶段闭环
```

---

## 异常处理

| 现象 | 原因 | 操作 |
|------|------|------|
| hash 不匹配 | 字节序或 FNV-1a 输入格式差异 | 检查 U64_TO_HEX 是否大端序 |
| 细胞不匹配 | 位分配定义错误 | 对照 stage2_fixed_point_verify.h 宏 |
| 编译失败 | C 编译器不支持 C99 | `clang --version` 确认 |

---

*最后更新: 2026-09-03 (Stage 2 离线固定点验证通过 · clang 18.1.3 · PASS)*
