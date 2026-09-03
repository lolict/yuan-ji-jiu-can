# isa-ref — 五架构机器码对齐参考

> 工具编号: #85 (编码层)
> 状态: 初始素材 · 持续核对官方手册扩充
> 路径: `tools/isa-ref/`

## 用途

把 ARM / x86-64 / RISC-V / MIPS32 / MTK(ARM) 的机器码按功能对齐到同一张表，
让跨平台编译同一语义时能快速检索各架构的编码差异。

## 文件结构

```
isa-ref/
├── SKILL.md          ← 本文件
├── README.md         ← 交叉索引入口
├── arm64.md          ← AArch64 指令编码
├── x86-64.md         ← x86-64 指令编码
├── riscv.md          ← RV32I/RV64I 指令编码
├── mips.md           ← MIPS32 指令编码
└── mtk.md            ← 联发科基于 ARM 的扩展
```

## 收录原则

- 只收基础整数指令集（不覆盖 SIMD/FP/特权）
- 二进制位域标注 opcode/operands 位置
- 汇编使用 GNU as / ARM asm 统一语法
- 每条标注字节数、对齐要求、典型用途
