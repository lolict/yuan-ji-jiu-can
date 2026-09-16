## RISC-V RV32I 编码/解码器 — 字体符号双向映射
## 位域严格按 RISC-V Unprivileged Spec 20191213

import std/strutils

const
  OP_LOAD   = 0b0000011'u32
  OP_STORE  = 0b0100011'u32
  OP_BRANCH = 0b1100011'u32
  OP_JALR   = 0b1100111'u32
  OP_JAL    = 0b1101111'u32
  OP_OP_IMM = 0b0010011'u32
  OP_OP     = 0b0110011'u32
  OP_LUI    = 0b0110111'u32
  OP_AUIPC  = 0b0010111'u32

type
  InstrKind* = enum ikR, ikI, ikS, ikB, ikU, ikJ
  InstrRecord* = object
    name*: string
    kind*: InstrKind
    opcode*: uint32
    funct3*: uint32
    funct7*: uint32
    sym_codepoint*: uint32

const SYM_BASE* = 0xF000'u32

const INSTR_TABLE*: array[19, InstrRecord] = [
  InstrRecord(name: "add",  kind: ikR, opcode: OP_OP,     funct3: 0b000, funct7: 0b0000000, sym_codepoint: SYM_BASE + 0),
  InstrRecord(name: "sub",  kind: ikR, opcode: OP_OP,     funct3: 0b000, funct7: 0b0100000, sym_codepoint: SYM_BASE + 1),
  InstrRecord(name: "xor",  kind: ikR, opcode: OP_OP,     funct3: 0b100, funct7: 0b0000000, sym_codepoint: SYM_BASE + 2),
  InstrRecord(name: "or",   kind: ikR, opcode: OP_OP,     funct3: 0b110, funct7: 0b0000000, sym_codepoint: SYM_BASE + 3),
  InstrRecord(name: "and",  kind: ikR, opcode: OP_OP,     funct3: 0b111, funct7: 0b0000000, sym_codepoint: SYM_BASE + 4),
  InstrRecord(name: "sll",  kind: ikR, opcode: OP_OP,     funct3: 0b001, funct7: 0b0000000, sym_codepoint: SYM_BASE + 5),
  InstrRecord(name: "srl",  kind: ikR, opcode: OP_OP,     funct3: 0b101, funct7: 0b0000000, sym_codepoint: SYM_BASE + 6),
  InstrRecord(name: "sra",  kind: ikR, opcode: OP_OP,     funct3: 0b101, funct7: 0b0100000, sym_codepoint: SYM_BASE + 7),
  InstrRecord(name: "addi", kind: ikI, opcode: OP_OP_IMM, funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 8),
  InstrRecord(name: "slti", kind: ikI, opcode: OP_OP_IMM, funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 9),
  InstrRecord(name: "andi", kind: ikI, opcode: OP_OP_IMM, funct3: 0b111, funct7: 0,          sym_codepoint: SYM_BASE + 10),
  InstrRecord(name: "lw",   kind: ikI, opcode: OP_LOAD,   funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 11),
  InstrRecord(name: "sw",   kind: ikS, opcode: OP_STORE,  funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 12),
  InstrRecord(name: "beq",  kind: ikB, opcode: OP_BRANCH, funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 13),
  InstrRecord(name: "bne",  kind: ikB, opcode: OP_BRANCH, funct3: 0b001, funct7: 0,          sym_codepoint: SYM_BASE + 14),
  InstrRecord(name: "jal",  kind: ikJ, opcode: OP_JAL,    funct3: 0,     funct7: 0,          sym_codepoint: SYM_BASE + 15),
  InstrRecord(name: "jalr", kind: ikI, opcode: OP_JALR,   funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 16),
  InstrRecord(name: "lui",  kind: ikU, opcode: OP_LUI,    funct3: 0,     funct7: 0,          sym_codepoint: SYM_BASE + 17),
  InstrRecord(name: "auipc", kind: ikU, opcode: OP_AUIPC, funct3: 0, funct7: 0,              sym_codepoint: SYM_BASE + 18),
]

proc encode_r(funct7, rs2, rs1, funct3, rd, opcode: uint32): uint32 =
  (funct7 shl 25) or (rs2 shl 20) or (rs1 shl 15) or (funct3 shl 12) or (rd shl 7) or opcode

proc encode_i(imm, rs1, funct3, rd, opcode: uint32): uint32 =
  let imm12 = imm and 0xFFF
  (imm12 shl 20) or (rs1 shl 15) or (funct3 shl 12) or (rd shl 7) or opcode

proc encode_s(imm, rs2, rs1, funct3, opcode: uint32): uint32 =
  let imm12 = imm and 0xFFF
  ((imm12 shr 5) and 0x7F) shl 25 or (rs2 shl 20) or (rs1 shl 15) or
  (funct3 shl 12) or ((imm12 and 0x1F) shl 7) or opcode

proc encode_b(imm, rs2, rs1, funct3, opcode: uint32): uint32 =
  let imm13 = imm and 0x1FFF
  let b12 = (imm13 shr 12) and 0x1
  let b10_5 = (imm13 shr 5) and 0x3F
  let b4_1 = (imm13 shr 1) and 0xF
  let b11 = (imm13 shr 11) and 0x1
  ((b12 shl 6) or b10_5) shl 25 or (rs2 shl 20) or (rs1 shl 15) or
  (funct3 shl 12) or ((b4_1 shl 1) or b11) shl 7 or opcode

proc encode_u(imm, rd, opcode: uint32): uint32 =
  (imm and 0xFFFFF000'u32) or (rd shl 7) or opcode

proc encode_j(imm, rd, opcode: uint32): uint32 =
  let imm21 = imm and 0x1FFFFF'u32
  let b20 = (imm21 shr 20) and 0x1
  let b10_1 = (imm21 shr 1) and 0x3FF
  let b11 = (imm21 shr 11) and 0x1
  let b19_12 = (imm21 shr 12) and 0xFF
  (b20 shl 31) or (b10_1 shl 21) or (b11 shl 20) or (b19_12 shl 12) or (rd shl 7) or opcode

proc encode_instr_stub*(idx: int): uint32 =
  let rec = INSTR_TABLE[idx]
  case rec.kind
  of ikR: encode_r(rec.funct7, 0, 0, rec.funct3, 0, rec.opcode)
  of ikI: encode_i(0, 0, rec.funct3, 0, rec.opcode)
  of ikS: encode_s(0, 0, 0, rec.funct3, rec.opcode)
  of ikB: encode_b(0, 0, 0, rec.funct3, rec.opcode)
  of ikU: encode_u(0, 0, rec.opcode)
  of ikJ: encode_j(0, 0, rec.opcode)

proc sym_to_instr*(cp: uint32): int =
  for i in 0 ..< INSTR_TABLE.len:
    if INSTR_TABLE[i].sym_codepoint == cp: return i
  return -1

proc instr_to_sym*(idx: int): uint32 =
  if idx >= 0 and idx < INSTR_TABLE.len: INSTR_TABLE[idx].sym_codepoint
  else: 0
