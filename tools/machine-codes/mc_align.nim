# ============================================================
# 机器码跨 ISA 对齐表 · 语义 ID → 各 ISA 编码映射
#
# 这是你 DeepSeek 对话的核心需求：
# "如果跨平台的机器码使用同一个机器码来定义同一个功能，则对齐到同一个列表下"
#
# 实现：语义 ID 为公约数，各 ISA 位域为公倍数。
#      同一语义 ID 下，不同 ISA 并排记录汇编与位域模板。
# ============================================================

import mc_types

type
  AlignedEntry* = object
    semId*: string        # 跨 ISA 唯一语义 ID
    asm*: seq[tuple[isa: ISA, text: string]]   # 各 ISA 汇编助记符
    bitLen*: int          # 机器码 bit 宽度
    description*: cstring

const ALIGN_TABLE*: seq[AlignedEntry] = @[
  AlignedEntry(
    semId: "ADD",
    asm: @[(isaRISCV32, "add rd, rs1, rs2"),
           (isaARM64,   "add Rd, Rn, Rm"),
           (isaX86_64,  "add r/m64, r64"),
           (isaMIPS32,  "add rd, rs, rt")],
    bitLen: 32,
    description: "整数加法：rd = rs1 + rs2"
  ),
  AlignedEntry(
    semId: "SUB",
    asm: @[(isaRISCV32, "sub rd, rs1, rs2"),
           (isaARM64,   "sub Rd, Rn, Rm"),
           (isaX86_64,  "sub r/m64, r64"),
           (isaMIPS32,  "sub rd, rs, rt")],
    bitLen: 32,
    description: "整数减法：rd = rs1 - rs2"
  ),
  AlignedEntry(
    semId: "AND",
    asm: @[(isaRISCV32, "and rd, rs1, rs2"),
           (isaARM64,   "and Rd, Rn, Rm"),
           (isaX86_64,  "and r/m64, r64"),
           (isaMIPS32,  "and rd, rs, rt")],
    bitLen: 32,
    description: "按位与"
  ),
  AlignedEntry(
    semId: "OR",
    asm: @[(isaRISCV32, "or rd, rs1, rs2"),
           (isaARM64,   "orr Rd, Rn, Rm"),
           (isaX86_64,  "or r/m64, r64"),
           (isaMIPS32,  "or rd, rs, rt")],
    bitLen: 32,
    description: "按位或"
  ),
  AlignedEntry(
    semId: "XOR",
    asm: @[(isaRISCV32, "xor rd, rs1, rs2"),
           (isaARM64,   "eor Rd, Rn, Rm"),
           (isaX86_64,  "xor r/m64, r64"),
           (isaMIPS32,  "xor rd, rs, rt")],
    bitLen: 32,
    description: "按位异或"
  ),
  AlignedEntry(
    semId: "SLL",
    asm: @[(isaRISCV32, "sll rd, rs1, rs2"),
           (isaARM64,   "lsl Rd, Rn, Rm"),
           (isaX86_64,  "shl r/m64, cl"),
           (isaMIPS32,  "sll rd, rt, shamt")],
    bitLen: 32,
    description: "逻辑左移"
  ),
  AlignedEntry(
    semId: "SRL",
    asm: @[(isaRISCV32, "srl rd, rs1, rs2"),
           (isaARM64,   "lsr Rd, Rn, Rm"),
           (isaX86_64,  "shr r/m64, cl"),
           (isaMIPS32,  "srl rd, rt, shamt")],
    bitLen: 32,
    description: "逻辑右移"
  ),
  AlignedEntry(
    semId: "SRA",
    asm: @[(isaRISCV32, "sra rd, rs1, rs2"),
           (isaARM64,   "asr Rd, Rn, Rm"),
           (isaX86_64,  "sar r/m64, cl"),
           (isaMIPS32,  "sra rd, rt, shamt")],
    bitLen: 32,
    description: "算术右移（保留符号位）"
  ),
  AlignedEntry(
    semId: "SLT",
    asm: @[(isaRISCV32, "slt rd, rs1, rs2"),
           (isaARM64,   "cmp Rn, Rm; cset"),
           (isaX86_64,  "cmp r/m64, r64; setl"),
           (isaMIPS32,  "slt rd, rs, rt")],
    bitLen: 32,
    description: "有符号比较置位"
  ),
  AlignedEntry(
    semId: "BEQ",
    asm: @[(isaRISCV32, "beq rs1, rs2, label"),
           (isaARM64,   "b.eq label"),
           (isaX86_64,  "je label"),
           (isaMIPS32,  "beq rs, rt, label")],
    bitLen: 32,
    description: "相等则跳转"
  ),
  AlignedEntry(
    semId: "BNE",
    asm: @[(isaRISCV32, "bne rs1, rs2, label"),
           (isaARM64,   "b.ne label"),
           (isaX86_64,  "jne label"),
           (isaMIPS32,  "bne rs, rt, label")],
    bitLen: 32,
    description: "不等则跳转"
  ),
  AlignedEntry(
    semId: "BLT",
    asm: @[(isaRISCV32, "blt rs1, rs2, label"),
           (isaARM64,   "b.lt label"),
           (isaX86_64,  "jl label"),
           (isaMIPS32,  "bltz rs, label")],
    bitLen: 32,
    description: "小于则跳转"
  ),
  AlignedEntry(
    semId: "LW",
    asm: @[(isaRISCV32, "lw rd, offset(rs1)"),
           (isaARM64,   "ldr Rd, [Rn, #offset]"),
           (isaX86_64,  "mov r64, [r/m64]"),
           (isaMIPS32,  "lw rt, offset(base)")],
    bitLen: 32,
    description: "加载 32 位字"
  ),
  AlignedEntry(
    semId: "SW",
    asm: @[(isaRISCV32, "sw rs2, offset(rs1)"),
           (isaARM64,   "str Rt, [Rn, #offset]"),
           (isaX86_64,  "mov [r/m64], r64"),
           (isaMIPS32,  "sw rt, offset(base)")],
    bitLen: 32,
    description: "存储 32 位字"
  ),
  AlignedEntry(
    semId: "LH",
    asm: @[(isaRISCV32, "lh rd, offset(rs1)"),
           (isaARM64,   "ldrh Rt, [Rn, #offset]"),
           (isaX86_64,  "movzx r64, word ptr []"),
           (isaMIPS32,  "lh rt, offset(base)")],
    bitLen: 32,
    description: "加载 16 位半字（符号扩展）"
  ),
  AlignedEntry(
    semId: "SH",
    asm: @[(isaRISCV32, "sh rs2, offset(rs1)"),
           (isaARM64,   "strh Rt, [Rn, #offset]"),
           (isaX86_64,  "mov word ptr [], r16"),
           (isaMIPS32,  "sh rt, offset(base)")],
    bitLen: 32,
    description: "存储 16 位半字"
  ),
  AlignedEntry(
    semId: "LB",
    asm: @[(isaRISCV32, "lb rd, offset(rs1)"),
           (isaARM64,   "ldrb Rt, [Rn, #offset]"),
           (isaX86_64,  "movsx r64, byte ptr []"),
           (isaMIPS32,  "lb rt, offset(base)")],
    bitLen: 32,
    description: "加载 8 位字节（符号扩展）"
  ),
  AlignedEntry(
    semId: "SB",
    asm: @[(isaRISCV32, "sb rs2, offset(rs1)"),
           (isaARM64,   "strb Rt, [Rn, #offset]"),
           (isaX86_64,  "mov byte ptr [], r8"),
           (isaMIPS32,  "sb rt, offset(base)")],
    bitLen: 32,
    description: "存储 8 位字节"
  ),
  AlignedEntry(
    semId: "LBU",
    asm: @[(isaRISCV32, "lbu rd, offset(rs1)"),
           (isaARM64,   "ldrb Rt, [Rn, #offset]"),
           (isaX86_64,  "movzx r64, byte ptr []"),
           (isaMIPS32,  "lbu rt, offset(base)")],
    bitLen: 32,
    description: "加载 8 位字节（零扩展）"
  ),
  AlignedEntry(
    semId: "JMP",
    asm: @[(isaRISCV32, "j offset"),
           (isaARM64,   "b label"),
           (isaX86_64,  "jmp label"),
           (isaMIPS32,  "j target")],
    bitLen: 32,
    description: "无条件跳转"
  ),
  AlignedEntry(
    semId: "CALL",
    asm: @[(isaRISCV32, "jal ra, offset"),
           (isaARM64,   "bl label"),
           (isaX86_64,  "call label"),
           (isaMIPS32,  "jal target")],
    bitLen: 32,
    description: "调用子程序（保存返回地址）"
  ),
  AlignedEntry(
    semId: "RET",
    asm: @[(isaRISCV32, "ret"),
           (isaARM64,   "ret"),
           (isaX86_64,  "ret"),
           (isaMIPS32,  "jr ra")],
    bitLen: 32,
    description: "从子程序返回"
  ),
  AlignedEntry(
    semId: "ADDI",
    asm: @[(isaRISCV32, "addi rd, rs1, imm"),
           (isaARM64,   "add Rd, Rn, #imm"),
           (isaX86_64,  "add r/m64, imm"),
           (isaMIPS32,  "addi rt, rs, imm")],
    bitLen: 32,
    description: "立即数加法"
  ),
  AlignedEntry(
    semId: "CMP",
    asm: @[(isaRISCV32, "beq rs1, rs2, +4; ..."),
           (isaARM64,   "cmp Rn, Rm"),
           (isaX86_64,  "cmp r/m64, r64"),
           (isaMIPS32,  "beq rs, rt, label")],
    bitLen: 32,
    description: "比较（设标志位）"
  ),
  AlignedEntry(
    semId: "MOV",
    asm: @[(isaRISCV32, "addi rd, x0, 0"),
           (isaARM64,   "mov Rd, Rn"),
           (isaX86_64,  "mov r64, r/m64"),
           (isaMIPS32,  "or rd, rs, zero")],
    bitLen: 32,
    description: "寄存器间传送"
  ),
  AlignedEntry(
    semId: "MUL",
    asm: @[(isaRISCV32, "mul rd, rs1, rs2"),
           (isaARM64,   "mul Rd, Rn, Rm"),
           (isaX86_64,  "imul r64, r/m64"),
           (isaMIPS32,  "mul rd, rs, rt")],
    bitLen: 32,
    description: "整数乘法"
  ),
  AlignedEntry(
    semId: "DIV",
    asm: @[(isaRISCV32, "div rd, rs1, rs2"),
           (isaARM64,   "udiv Rd, Rn, Rm"),
           (isaX86_64,  "div r/m64"),
           (isaMIPS32,  "div rs, rt; mflo rd")],
    bitLen: 32,
    description: "整数除法"
  ),
  AlignedEntry(
    semId: "LUI",
    asm: @[(isaRISCV32, "lui rd, imm20"),
           (isaARM64,   "movz Rd, #imm"),
           (isaX86_64,  "mov r64, imm64"),
           (isaMIPS32,  "lui rt, imm")],
    bitLen: 32,
    description: "加载高位立即数"
  ),
  AlignedEntry(
    semId: "SYSCALL",
    asm: @[(isaRISCV32, "ecall"),
           (isaARM64,   "svc #0"),
           (isaX86_64,  "syscall"),
           (isaMIPS32,  "syscall")],
    bitLen: 32,
    description: "系统调用"
  ),
  AlignedEntry(
    semId: "NOP",
    asm: @[(isaRISCV32, "addi x0, x0, 0"),
           (isaARM64,   "nop"),
           (isaX86_64,  "nop"),
           (isaMIPS32,  "nop")],
    bitLen: 32,
    description: "空操作（延时槽填充）"
  ),
]
