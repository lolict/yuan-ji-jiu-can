# ============================================================
# MIPS32 · 位域表（定长 32 位指令）
#
# 对齐语义 ID 与 RISC-V / ARM64 / x86-64
# ============================================================

import mc_types

const
  # ── R-型：register-register ──
  MIPS_R_TYPE*: seq[Field] = @[
    Field(name: "opcode", format: ffOpcode, hi: 31, lo: 26),
    Field(name: "rs",     format: ffRegSrc1, hi: 25, lo: 21),
    Field(name: "rt",     format: ffRegSrc2, hi: 20, lo: 16),
    Field(name: "rd",     format: ffRegDst,  hi: 15, lo: 11),
    Field(name: "shamt",  format: ffShamt,   hi: 10, lo: 6),
    Field(name: "funct",  format: ffFunct7,  hi: 5,  lo: 0),
  ]

  # ── I-型：immediate / load / branch ──
  MIPS_I_TYPE*: seq[Field] = @[
    Field(name: "opcode", format: ffOpcode, hi: 31, lo: 26),
    Field(name: "rs",     format: ffRegSrc1, hi: 25, lo: 21),
    Field(name: "rt",     format: ffRegDst,  hi: 20, lo: 16),
    Field(name: "imm",    format: ffImmI,   hi: 15, lo: 0),
  ]

  # ── J-型：jump ──
  MIPS_J_TYPE*: seq[Field] = @[
    Field(name: "opcode", format: ffOpcode, hi: 31, lo: 26),
    Field(name: "target", format: ffImmJ,   hi: 25, lo: 0),
  ]

const MIPS32_ENCODINGS*: seq[InsnEncoding] = @[
  InsnEncoding(semId: "ADD",    mnemonic: "add",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "ADDU",   mnemonic: "addu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SUB",    mnemonic: "sub",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SUBU",   mnemonic: "subu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "AND",    mnemonic: "and",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "OR",     mnemonic: "or",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "XOR",    mnemonic: "xor",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "NOR",    mnemonic: "nor",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SLT",    mnemonic: "slt",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SLTU",   mnemonic: "sltu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),

  # ── 立即数 ──
  InsnEncoding(semId: "ADDI",   mnemonic: "addi",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "ADDIU",  mnemonic: "addiu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "ANDI",   mnemonic: "andi",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "ORI",    mnemonic: "ori",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "XORI",   mnemonic: "xori",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "SLTI",   mnemonic: "slti",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "SLTIU",  mnemonic: "sltiu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "LUI",    mnemonic: "lui",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),

  # ── 移位 ──
  InsnEncoding(semId: "SLL",    mnemonic: "sll",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SRL",    mnemonic: "srl",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SRA",    mnemonic: "sra",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SLLV",   mnemonic: "sllv",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SRLV",   mnemonic: "srlv",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "SRAV",   mnemonic: "srav",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),

  # ── 乘除 ──
  InsnEncoding(semId: "MUL",    mnemonic: "mul",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "MULT",   mnemonic: "mult",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "MULTU",  mnemonic: "multu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "DIV",    mnemonic: "div",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "DIVU",   mnemonic: "divu",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "MFHI",   mnemonic: "mfhi",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "MFLO",   mnemonic: "mflo",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),

  # ── 加载 / 存储（对齐 LW / SW / LH / SH / LB / SB） ──
  InsnEncoding(semId: "LW",     mnemonic: "lw", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "SW",     mnemonic: "sw", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "LH",     mnemonic: "lh", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "LHU",    mnemonic: "lhu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "SH",     mnemonic: "sh", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "LB",     mnemonic: "lb", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "LBU",    mnemonic: "lbu", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "SB",     mnemonic: "sb", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),

  # ── 分支（对齐 BEQ / BNE / BLT） ──
  InsnEncoding(semId: "BEQ",    mnemonic: "beq",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "BNE",    mnemonic: "bne",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "BGEZ",   mnemonic: "bgez", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "BGTZ",   mnemonic: "bgtz", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "BLEZ",   mnemonic: "blez", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),
  InsnEncoding(semId: "BLTZ",   mnemonic: "bltz", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_I_TYPE),

  # ── 跳转（对齐 JMP / CALL / RET） ──
  InsnEncoding(semId: "JMP",    mnemonic: "j",    isa: isaMIPS32, byteLen: 4,
               fields: MIPS_J_TYPE),
  InsnEncoding(semId: "JAL",    mnemonic: "jal",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_J_TYPE),
  InsnEncoding(semId: "CALL",   mnemonic: "jal",  isa: isaMIPS32, byteLen: 4,
               fields: MIPS_J_TYPE),
  InsnEncoding(semId: "RET",    mnemonic: "jr",   isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),

  # ── 系统 ──
  InsnEncoding(semId: "SYSCALL", mnemonic: "syscall", isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
  InsnEncoding(semId: "NOP",    mnemonic: "nop",    isa: isaMIPS32, byteLen: 4,
               fields: MIPS_R_TYPE),
]
