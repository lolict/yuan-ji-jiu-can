# ============================================================
# ARM64 (AArch64) · 位域表（核心 30 条对齐表）
#
# 每条指令固定 32 位（4 字节）
# 字节序：小端（默认），可切换大端
# ============================================================

import mc_types

const
  # ── R-型：数据处理寄存器 ──
  ARM_R_TYPE*: seq[Field] = @[
    Field(name: "sf",      format: ffFunct7, hi: 31, lo: 31),  # 1=64bit
    Field(name: "op",      format: ffOpcode,  hi: 30, lo: 29),
    Field(name: "S",       format: ffFunct3,  hi: 28, lo: 28),
    Field(name: "Rm",      format: ffRegSrc2, hi: 20, lo: 16),
    Field(name: "option",  format: ffFunct7,  hi: 15, lo: 13),
    Field(name: "Rn",      format: ffRegSrc1, hi: 9,  lo: 5),
    Field(name: "Rd",      format: ffRegDst,  hi: 4,  lo: 0),
  ]

  # ── I-型：立即数（DP imm） ──
  ARM_I_TYPE*: seq[Field] = @[
    Field(name: "sf",      format: ffFunct7, hi: 31, lo: 31),
    Field(name: "op",      format: ffOpcode,  hi: 30, lo: 29),
    Field(name: "S",       format: ffFunct3,  hi: 28, lo: 28),
    Field(name: "hw",      format: ffImmU,   hi: 23, lo: 21),
    Field(name: "imm16",   format: ffImmU,   hi: 20, lo: 5),
    Field(name: "Rd",      format: ffRegDst,  hi: 4,  lo: 0),
  ]

  # ── Load/Store ──
  ARM_LS_TYPE*: seq[Field] = @[
    Field(name: "size",    format: ffFunct7, hi: 31, lo: 30),
    Field(name: "V",       format: ffFunct3,  hi: 29, lo: 28),
    Field(name: "opc",     format: ffFunct3,  hi: 27, lo: 26),
    Field(name: "imm19",   format: ffImmI,   hi: 23, lo: 5),
    Field(name: "Rt",      format: ffRegDst,  hi: 4,  lo: 0),
  ]

const ARM64_ENCODINGS*: seq[InsnEncoding] = @[
  # ── 数据处理（对齐语义 ID） ──
  InsnEncoding(semId: "ADD",     mnemonic: "add",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "SUB",     mnemonic: "sub",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "AND",     mnemonic: "and",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "ORR",     mnemonic: "orr",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "EOR",     mnemonic: "eor",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "MOVZ",    mnemonic: "movz",    isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "MOVK",    mnemonic: "movk",    isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "LSL",     mnemonic: "lsl",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "LSR",     mnemonic: "lsr",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "ASR",     mnemonic: "asr",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "CMP",     mnemonic: "cmp",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "TST",     mnemonic: "tst",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),
  InsnEncoding(semId: "MVN",     mnemonic: "mvn",     isa: isaARM64, byteLen: 4,
               fields: ARM_R_TYPE),

  # ── 立即数 ──
  InsnEncoding(semId: "ADDI",    mnemonic: "addi",    isa: isaARM64, byteLen: 4,
               fields: ARM_I_TYPE),
  InsnEncoding(semId: "SUBI",    mnemonic: "subi",    isa: isaARM64, byteLen: 4,
               fields: ARM_I_TYPE),
  InsnEncoding(semId: "ANDI",    mnemonic: "andi",    isa: isaARM64, byteLen: 4,
               fields: ARM_I_TYPE),
  InsnEncoding(semId: "ORRI",    mnemonic: "orri",    isa: isaARM64, byteLen: 4,
               fields: ARM_I_TYPE),
  InsnEncoding(semId: "EORI",    mnemonic: "eori",    isa: isaARM64, byteLen: 4,
               fields: ARM_I_TYPE),

  # ── 加载 / 存储（复用 LW / SW） ──
  InsnEncoding(semId: "LW",      mnemonic: "ldr",     isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "SW",      mnemonic: "str",     isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "LH",      mnemonic: "ldrh",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "SH",      mnemonic: "strh",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "LB",      mnemonic: "ldrb",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "SB",      mnemonic: "strb",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "LBU",     mnemonic: "ldrb",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "LHU",     mnemonic: "ldrh",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),

  # ── 分支 ──
  InsnEncoding(semId: "BEQ",     mnemonic: "b.eq",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "BNE",     mnemonic: "b.ne",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "BLT",     mnemonic: "b.lt",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "BGT",     mnemonic: "b.gt",    isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "B",       mnemonic: "b",       isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "BL",      mnemonic: "bl",      isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
  InsnEncoding(semId: "RET",     mnemonic: "ret",     isa: isaARM64, byteLen: 4,
               fields: ARM_LS_TYPE),
]
