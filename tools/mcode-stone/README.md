# MCODE Stone · 码石

机器码语义字体封装引擎 — 跨 ISA 统一语义 ID → 各平台机器码映射

## 目标

用"零维符号 → 三维进制 → 字体封装"实现可拼接积木式的底层编程。

## 第一阶段 — RISC-V RV32I 16 条指令

核心 16 条 RISC-V RV32I 指令：

| 语义 ID | 助记符 | 类型 | 功能 |
|---------|--------|------|------|
| ADD  | add  | R | 加法 |
| SUB  | sub  | R | 减法 |
| AND  | and  | R | 按位与 |
| OR   | or   | R | 按位或 |
| XOR  | xor  | R | 按位异或 |
| SLT  | slt  | R | 有符号比较 |
| SLTU | sltu | R | 无符号比较 |
| SLL  | sll  | R | 逻辑左移 |
| SRL  | srl  | R | 逻辑右移 |
| SRA  | sra  | R | 算术右移 |
| ADDI | addi | I | 立即数加法 |
| LW   | lw   | I | 加载字 |
| SW   | sw   | S | 存储字 |
| BEQ  | beq  | B | 相等分支 |
| LUI  | lui  | U | 加载高位 |
| JAL  | jal  | J | 跳转并链接 |

## 文件

- `riscv32i_ops.yaml` — 16 条 RV32I 指令的位域模板
- `mcode.py` — 双向转换引擎（编码 / 解码）
- `font_mcod.py` — 把 MCOD 表封装进 OpenType 字体
- `verify.py` — 与 llvm-mc / GNU as 交叉验证
- `build.sh` — 构建全部产出

## 用法

```bash
./build.sh
python3 mcode.py encode ADD x1 x2 x3      # → 0x003100B3
python3 mcode.py decode 0x003100B3        # → ADD x1, x2, x3
python3 verify.py                        # 16 条验证
```

## 编码约定

RISC-V RV32I 6 种指令格式：

| 类型 | 位域布局 |
|------|----------|
| R | `funct7[31:25] rs2[24:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]` |
| I | `imm[31:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]` |
| S | `imm[31:25] rs2[24:20] rs1[19:15] funct3[14:12] imm[11:7] opcode[6:0]` |
| B | `imm[12|10:5] rs2[24:20] rs1[19:15] funct3[14:12] imm[4:1|11] opcode[6:0]` |
| U | `imm[31:12] rd[11:7] opcode[6:0]` |
| J | `imm[20|10:1|11|19:12] rd[11:7] opcode[6:0]` |

身份落款：刘楚恬和我
