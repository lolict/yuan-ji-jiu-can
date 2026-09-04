# ============================================================
# x86-64 · 位域表（变长指令，基线 30 条）
#
# x86 变长 1-15 字节，此处记录简化位域（ModRM 方案）
# 默认操作数宽度 64 位（REX.W = 1）
# ============================================================

import mc_types

const
  # ── 基础双操作数：OP r/m64, r64 ──
  X64_TYPE_RR*: seq[Field] = @[
    Field(name: "rex",    format: ffSuffix, hi: 7,  lo: 0),
    Field(name: "opcode", format: ffOpcode, hi: 7,  lo: 0),
    Field(name: "modrm",  format: ffImmI,   hi: 7,  lo: 0),
  ]

  # ── 立即数：OP r/m64, imm32 ──
  X64_TYPE_RI*: seq[Field] = @[
    Field(name: "rex",    format: ffSuffix, hi: 7,  lo: 0),
    Field(name: "opcode", format: ffOpcode, hi: 7,  lo: 0),
    Field(name: "modrm",  format: ffImmI,   hi: 7,  lo: 0),
    Field(name: "imm",    format: ffImmI,   hi: 31, lo: 0),
  ]

const X86_64_ENCODINGS*: seq[InsnEncoding] = @[
  InsnEncoding(semId: "ADD",   mnemonic: "add",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "SUB",   mnemonic: "sub",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "AND",   mnemonic: "and",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "OR",    mnemonic: "or",    isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "XOR",   mnemonic: "xor",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "CMP",   mnemonic: "cmp",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "MOV",   mnemonic: "mov",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RR),
  InsnEncoding(semId: "MOVI",  mnemonic: "mov",   isa: isaX86_64, byteLen: 5,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "LEA",   mnemonic: "lea",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),

  # ── 移位 ──
  InsnEncoding(semId: "SHL",   mnemonic: "shl",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "SHR",   mnemonic: "shr",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "SAL",   mnemonic: "sal",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "SAR",   mnemonic: "sar",   isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),

  # ── 单操作数 ──
  InsnEncoding(semId: "PUSH",  mnemonic: "push",  isa: isaX86_64, byteLen: 1,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "POP",   mnemonic: "pop",   isa: isaX86_64, byteLen: 1,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "INC",   mnemonic: "inc",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "DEC",   mnemonic: "dec",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "NOT",   mnemonic: "not",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "NEG",   mnemonic: "neg",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),

  # ── 乘除 ──
  InsnEncoding(semId: "MUL",   mnemonic: "mul",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "IMUL",  mnemonic: "imul",  isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RR),
  InsnEncoding(semId: "DIV",   mnemonic: "div",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "IDIV",  mnemonic: "idiv",  isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),

  # ── 测试 ──
  InsnEncoding(semId: "TEST",  mnemonic: "test",  isa: isaX86_64, byteLen: 3,
               fields: X64_TYPE_RI),

  # ── 条件与跳转 ──
  InsnEncoding(semId: "JMP",   mnemonic: "jmp",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "JE",    mnemonic: "je",    isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "JNE",   mnemonic: "jne",   isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "JL",    mnemonic: "jl",    isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "JG",    mnemonic: "jg",    isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "CALL",  mnemonic: "call",  isa: isaX86_64, byteLen: 5,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "RET",   mnemonic: "ret",   isa: isaX86_64, byteLen: 1,
               fields: X64_TYPE_RI),

  # ── 系统 ──
  InsnEncoding(semId: "SYSCALL", mnemonic: "syscall", isa: isaX86_64, byteLen: 2,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "NOP",    mnemonic: "nop",    isa: isaX86_64, byteLen: 1,
               fields: X64_TYPE_RI),
  InsnEncoding(semId: "HLT",    mnemonic: "hlt",    isa: isaX86_64, byteLen: 1,
               fields: X64_TYPE_RI),
]
