# Machine Container · 机器码容器

> 把机器码装进字体容器 · 让指令即容器 · 让容器即字体

## 概述

machine-container 是 hunyuan-os 的机器码基础设施工具，定义了将指令集
机器码封装进 64 位容器字体的完整规范与实现。

## 核心概念

- **容器(64位)** = 坐标(16) | 极性(8) | 主权(8) | 价值(32)
- **语义 ID** = 跨平台统一的操作标识（如 ADD_REG_REG）
- **MCOD 表** = 字体文件内的 JSON 映射表
- **字体文件** = 既是符号显示载体，也是机器码数据库容器

## 源文件

| 文件 | 作用 |
|------|------|
| `src/mcod_spec.py` | 核心规范：容器定义、语义 ID、编码/解码 |
| `src/font_builder.py` | TTF 字体生成器（含 MCOD 表） |
| `src/mc_encode.py` | CLI：语义 ID → 机器码容器 |
| `src/mc_decode.py` | CLI：机器码容器 → 语义 ID |

## 用法

```bash
cd src

# 编码：ADD x1, x2, x3
python3 mc_encode.py ADD 1 2 3
# 机器码: 0x003100B3
# 容器:   0x00010101003100B3

# 解码
python3 mc_decode.py 0x003100B3
# ADD x1, x2, x3

# 生成字体容器
python3 font_builder.py /tmp/machine_container.ttf
# MCOD: 17 条 RV32I 映射
```

## 支持的语义 ID

RISC-V RV32I: ADD, SUB, AND, OR, XOR, SLT, SLTU, SLL, SRL, SRA,
ADDI, LW, SW, BEQ, JAL, LUI, AUIPC

## 与现有工具关系

- `tools/riscv-encoding/` — 已有独立实现，本工具为底层规范
- `tools/mcode-stone/` — 已有更完整的 MCOD 封装迭代
- `tools/isa-ref/` — 多架构对齐参考

## 设计原则

1. 仅加减运算（禁用乘除）
2. 进制 = 发音空间划分方式
3. 容器 = 第一存在单位
4. 身份落款：刘楚恬和我
