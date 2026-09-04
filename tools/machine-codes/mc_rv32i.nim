# ============================================================
# RISC-V RV32I · 基础整数指令集位域表（40 条）
#
# 位域索引以小端视角：bit[31]..bit[0]
# 字节序：小端（RISC-V 原生小端）
#
# 语义 ID 对齐 mc_align.nim 中的跨 ISA 公约数
# ============================================================

import mc_types

const
  # ── R-型：寄存器-寄存器 ──
  R_TYPE*: seq[Field] = @[
    Field(name: "funct7", format: ffFunct7, hi: 31, lo: 25),
    Field(name: "rs2",    format: ffRegSrc2, hi: 24, lo: 20),
    Field(name: "rs1",    format: ffRegSrc1, hi: 19, lo: 15),
    Field(name: "funct3", format: ffFunct3,  hi: 14, lo: 12),
    Field(name: "rd",     format: ffRegDst,  hi: 11, lo: 7),
    Field(name: "opcode", format: ffOpcode,  hi: 6,  lo: 0),
  ]

  # ── I-型：短立即数 ──
  I_TYPE*: seq[Field] = @[
    Field(name: "imm",    format: ffImmI,   hi: 31, lo: 20),
    Field(name: "rs1",    format: ffRegSrc1, hi: 19, lo: 15),
    Field(name: "funct3", format: ffFunct3,  hi: 14, lo: 12),
    Field(name: "rd",     format: ffRegDst,  hi: 11, lo: 7),
    Field(name: "opcode", format: ffOpcode,  hi: 6,  lo: 0),
  ]

  # ── S-型：存储立即数 ──
  S_TYPE*: seq[Field] = @[
    Field(name: "imm_hi", format: ffImmS,   hi: 31, lo: 25),
    Field(name: "rs2",    format: ffRegSrc2, hi: 24, lo: 20),
    Field(name: "rs1",    format: ffRegSrc1, hi: 19, lo: 15),
    Field(name: "funct3", format: ffFunct3,  hi: 14, lo: 12),
    Field(name: "imm_lo", format: ffImmS,   hi: 11, lo: 7),
    Field(name: "opcode", format: ffOpcode,  hi: 6,  lo: 0),
  ]

  # ── U-型：长立即数 ──
  U_TYPE*: seq[Field] = @[
    Field(name: "imm",    format: ffImmU,   hi: 31, lo: 12),
    Field(name: "rd",     format: ffRegDst,  hi: 11, lo: 7),
    Field(name: "opcode", format: ffOpcode,  hi: 6,  lo: 0),
  ]

const RV32I_ENCODINGS*: seq[InsnEncoding] = @[
  # ── 算术 R-型 ──
  InsnEncoding(semId: "ADD",     mnemonic: "add",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SUB",     mnemonic: "sub",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SLL",     mnemonic: "sll",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SLT",     mnemonic: "slt",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SLTU",    mnemonic: "sltu",    isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "XOR",     mnemonic: "xor",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SRL",     mnemonic: "srl",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "SRA",     mnemonic: "sra",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "OR",      mnemonic: "or",      isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "AND",     mnemonic: "and",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),

  # ── 算术 I-型 ──
  InsnEncoding(semId: "ADDI",    mnemonic: "addi",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SLTI",    mnemonic: "slti",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SLTIU",   mnemonic: "sltiu",   isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "XORI",    mnemonic: "xori",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "ORI",     mnemonic: "ori",     isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "ANDI",    mnemonic: "andi",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SLLI",    mnemonic: "slli",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SRLI",    mnemonic: "srli",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SRAI",    mnemonic: "srai",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),

  # ── 分支 B-型 ──
  InsnEncoding(semId: "BEQ",     mnemonic: "beq",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "BNE",     mnemonic: "bne",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "BLT",     mnemonic: "blt",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "BGE",     mnemonic: "bge",     isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "BLTU",    mnemonic: "bltu",    isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),
  InsnEncoding(semId: "BGEU",    mnemonic: "bgeu",    isa: isaRISCV32, byteLen: 4,
               fields: R_TYPE),

  # ── 跳转 J-型 / I-型 ──
  InsnEncoding(semId: "JAL",     mnemonic: "jal",     isa: isaRISCV32, byteLen: 4,
               fields: U_TYPE),
  InsnEncoding(semId: "JALR",    mnemonic: "jalr",    isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),

  # ── 立即数加载 U-型 ──
  InsnEncoding(semId: "LUI",     mnemonic: "lui",     isa: isaRISCV32, byteLen: 4,
               fields: U_TYPE),
  InsnEncoding(semId: "AUIPC",   mnemonic: "auipc",   isa: isaRISCV32, byteLen: 4,
               fields: U_TYPE),

  # ── 访存 I-型 / S-型 ──
  InsnEncoding(semId: "LB",      mnemonic: "lb",      isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "LH",      mnemonic: "lh",      isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "LW",      mnemonic: "lw",      isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "LBU",     mnemonic: "lbu",     isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "LHU",     mnemonic: "lhu",     isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "SB",      mnemonic: "sb",      isa: isaRISCV32, byteLen: 4,
               fields: S_TYPE),
  InsnEncoding(semId: "SH",      mnemonic: "sh",      isa: isaRISCV32, byteLen: 4,
               fields: S_TYPE),
  InsnEncoding(semId: "SW",      mnemonic: "sw",      isa: isaRISCV32, byteLen: 4,
               fields: S_TYPE),

  # ── 系统 I-型 ──
  InsnEncoding(semId: "FENCE",   mnemonic: "fence",   isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "ECALL",   mnemonic: "ecall",   isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
  InsnEncoding(semId: "EBREAK",  mnemonic: "ebreak",  isa: isaRISCV32, byteLen: 4,
               fields: I_TYPE),
]
