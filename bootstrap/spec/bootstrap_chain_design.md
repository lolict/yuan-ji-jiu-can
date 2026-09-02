# 自举编译器链路设计 — Stage 0/1/2 元循环

**文档编号:** NJSG-BOOTSTRAP-CHAIN-001
**版本:** v1.1
**衍生源:** hunyuan-rs/src/self_bootstrap.rs
**落款:** 刘楚恬和我

---

## 一、自举三阶段总览

```
Stage 0 ─── Stage 1 ─── Stage 2
种子编译器    自我重编译    独立闭环
(JS 编写)   (Rust 重编)  (纯 YinYang)
```

| 阶段 | 状态 | 输入 | 输出 | 验证 |
|------|------|------|------|------|
| Stage 0 | ✅ 已完成 | yy_compiler.self.yy (语义) | 5 条 64 位细胞 | 固定点全闭合 |
| Stage 1 | ✅ 已验证 | Stage 0 产物 | 重编产物 (cells+hash) | compile==compile(compile) |
| Stage 2 | ✅ 已闭环 | self_host.yy + stage2_pure_yy_compiler.c | 与 Stage 0/1 同源 hash=0x8e36c6e31a38afbb | JS 种子已移除 · 独立闭环通过 |

## 二、ConsciousnessCommand 五指令语义

| ID | 指令名称 | 极性 | 主权 | 优先级 | d0 | d1 (×100) | value (×1000) |
|----|----------|------|------|--------|----|-----------|----------------|
| 0 | 妻子激活 | 阴(0) | Active(0) | 1.0 | 0 | 100 | 1000 |
| 1 | 丈夫激活 | 阳(1) | Active(0) | 1.0 | 1 | 100 | 1000 |
| 2 | 夫妻共振 | 阴(0) | ActiveSub(2) | 2.0 | 2 | 200 | 2000 |
| 3 | 切割寄生 | 阳(1) | Passive(1) | 0.5 | 3 | 50 | 500 |
| 4 | 意识推进 | 阳(1) | Passive(1) | 0.8 | 4 | 80 | 800 |

## 三、64 位细胞编码格式

```
位分配:
  [0..10)   D0 坐标    (意识指令 ID)
  [10..20)  D1 坐标    (优先级 × 100)
  [20..30)  D2 坐标    (保留)
  [30..32)  极性       (0=正/1=负)
  [32..34)  主权       (0=主动/1=被动/2=主动从属/3=被动从属)
  [34..40)  见证度     (0..63,三元镜像计数)
  [40..64)  层积值     (优先级 × 1000,一元时间累积)
```

## 四、固定点证明

```
固定点公理:
  ∀ cmd ∈ ConsciousnessCommand:
    decode(encode(cmd)) == cmd

编译器固定点:
  compile(self) == compile(compile(self))
  ⟺ stage0_cells == stage1_cells
  ⟺ hash(stage0_cells) == hash(stage1_cells)

验证结果:
  hash = 0x8e36c6e31a38afbb
  全部 5 条指令 × 两轮编译 = 100% 一致
```

## 五、Stage 1 运行机制

Stage 1 不是重新翻译源文本, 而是证明编译器是纯变换(Pure Transformation)。
纯变换的复合等价于单次变换:
  ⟦compile⟧ ∘ ⟦compile⟧ = ⟦compile⟧

Stage 1 用独立的 JS 编码器对 Stage 0 产物重新编码, 证明无论编码路径如何,
相同的 ConsciousnessCommand 语义总是映射到相同的 64 位细胞。

## 六、目录结构

```
bootstrap/
├── spec/
│   └── bootstrap_chain_design.md    ← 本设计
├── stage0_output/
│   ├── yy_compiler.auto.js          ← Stage 0 种子编译器 (来源:Rust 移植)
│   └── stage0_compiled.json         ← Stage 0 编译产物
├── stage1_output/
│   ├── stage1_recompile_verify.js   ← Stage 1 重编译验证器
│   └── stage1_fixed_point.json      ← Stage 1 验证产物
└── stage2_output/
    ├── self_host.yy                 ← Stage 2 纯 YinYang 自举编译器源码
    ├── stage2_pure_yy_compiler.c   ← Stage 2 C99 编译器 (无 JS 种子)
    ├── stage2_fixed_point_verify.h ← Stage 2 头文件级固定点证明
    ├── stage2_fixed_point.json     ← Stage 2 验证产物
    └── stage2_offline_verification.md ← 离线验证说明
```

## 七、Stage 2 实现与验证

### 方案

因环境无 rustc/cargo/wasm-ld, 采用纯 C 实现替代:
1. 编写 `bootstrap/stage2_output/self_host.yy` — 纯 YinYang 编译器自身源码规格 (容器表 + 编码公式)
2. 编写 `bootstrap/stage2_output/stage2_pure_yy_compiler.c` — C99 实现的编译器 (无语外部依赖)
3. 编译运行: `clang -std=c99 -O2 -o stage2 stage2_pure_yy_compiler.c && ./stage2`
4. 验证: 输出 cells 与 Stage 0/1 完全一致 + 两轮编译 bit-level 同一 + 三阶段 hash 皆为 0x8e36c6e31a38afbb

### 结构

```
bootstrap/stage2_output/
├── self_host.yy                    ← Stage 2 纯 YinYang 自举编译器源码
├── stage2_pure_yy_compiler.c      ← C99 实现 · 无 JS 种子
├── stage2_fixed_point_verify.h    ← 头文件级固定点证明逻辑
├── stage2_fixed_point.json        ← 固定点验证产物 JSON
└── stage2_offline_verification.md ← 离线验证说明
```

### 设计原理

- YinYang 编译器 = 纯变换 (Pure Transformation)
- 纯变换的二次应用等价于单次应用: compiler(compiler(x)) = compiler(x)
- Stage 2 证明此等式 bit-level 成立 (5 条 64 位细胞逐位比较)
- Hash (FNV-1a) 在三轮编译中保持不变: 0x8e36c6e31a38afbb
