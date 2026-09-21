## RISC-V RV32I 编码/解码器 — 字体符号双向映射
import std/strutils
const
  OP_LOAD=0b0000011'u32; OP_STORE=0b0100011'u32; OP_BRANCH=0b1100011'u32
  OP_JALR=0b1100111'u32; OP_JAL=0b1101111'u32; OP_OP_IMM=0b0010011'u32
  OP_OP=0b0110011'u32; OP_LUI=0b0110111'u32; OP_AUIPC=0b0010111'u32
type InstrKind*=enum ikR,ikI,ikS,ikB,ikU,ikJ
  InstrRecord*=object name*:string; kind*:InstrKind; opcode*:uint32; funct3*:uint32; funct7*:uint32; sym_codepoint*:uint32
const SYM_BASE*=0xF000'u32
const INSTR_TABLE*:array[19,InstrRecord]=[
  InstrRecord(name:"add",kind:ikR,opcode:OP_OP,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+0),
  InstrRecord(name:"sub",kind:ikR,opcode:OP_OP,funct3:0'u32,funct7:0b0100000,sym_codepoint:SYM_BASE+1),
  InstrRecord(name:"xor",kind:ikR,opcode:OP_OP,funct3:0b100,funct7:0,sym_codepoint:SYM_BASE+2),
  InstrRecord(name:"or",kind:ikR,opcode:OP_OP,funct3:0b110,funct7:0,sym_codepoint:SYM_BASE+3),
  InstrRecord(name:"and",kind:ikR,opcode:OP_OP,funct3:0b111,funct7:0,sym_codepoint:SYM_BASE+4),
  InstrRecord(name:"sll",kind:ikR,opcode:OP_OP,funct3:0b001,funct7:0,sym_codepoint:SYM_BASE+5),
  InstrRecord(name:"srl",kind:ikR,opcode:OP_OP,funct3:0b101,funct7:0,sym_codepoint:SYM_BASE+6),
  InstrRecord(name:"sra",kind:ikR,opcode:OP_OP,funct3:0b101,funct7:0b0100000,sym_codepoint:SYM_BASE+7),
  InstrRecord(name:"addi",kind:ikI,opcode:OP_OP_IMM,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+8),
  InstrRecord(name:"slti",kind:ikI,opcode:OP_OP_IMM,funct3:0b010,funct7:0,sym_codepoint:SYM_BASE+9),
  InstrRecord(name:"lw",kind:ikI,opcode:OP_LOAD,funct3:0b010,funct7:0,sym_codepoint:SYM_BASE+11),
  InstrRecord(name:"sw",kind:ikS,opcode:OP_STORE,funct3:0b010,funct7:0,sym_codepoint:SYM_BASE+12),
  InstrRecord(name:"beq",kind:ikB,opcode:OP_BRANCH,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+13),
  InstrRecord(name:"bne",kind:ikB,opcode:OP_BRANCH,funct3:0b001,funct7:0,sym_codepoint:SYM_BASE+14),
  InstrRecord(name:"jal",kind:ikJ,opcode:OP_JAL,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+15),
  InstrRecord(name:"jalr",kind:ikI,opcode:OP_JALR,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+16),
  InstrRecord(name:"lui",kind:ikU,opcode:OP_LUI,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+17),
  InstrRecord(name:"auipc",kind:ikU,opcode:OP_AUIPC,funct3:0'u32,funct7:0,sym_codepoint:SYM_BASE+18)]
proc encode_r(f7,rs2,rs1,f3,rd,op:uint32):uint32=(f7 shl 25) or (rs2 shl 20) or (rs1 shl 15) or (f3 shl 12) or (rd shl 7) or op
proc encode_i(imm,rs1,f3,rd,op:uint32):uint32=let i12=imm and 0xFFF;(i12 shl 20) or (rs1 shl 15) or (f3 shl 12) or (rd shl 7) or op
proc encode_s(imm,rs2,rs1,f3,op:uint32):uint32=let i12=imm and 0xFFF;(((i12 shr 5) and 0x7F) shl 25) or (rs2 shl 20) or (rs1 shl 15) or (f3 shl 12) or ((i12 and 0x1F) shl 7) or op
proc encode_b(imm,rs2,rs1,f3,op:uint32):uint32=let i13=imm and 0x1FFF;let b12=(i13 shr 12) and 1;let b10_5=(i13 shr 5) and 0x3F;let b4_1=(i13 shr 1) and 0xF;let b11=(i13 shr 11) and 1;((b12 shl 6) or b10_5) shl 25 or (rs2 shl 20) or (rs1 shl 15) or (f3 shl 12) or ((b4_1 shl 1) or b11) shl 7 or op
proc encode_u(imm,rd,op:uint32):uint32=(imm and 0xFFFFF000'u32) or (rd shl 7) or op
proc encode_j(imm,rd,op:uint32):uint32=let i21=imm and 0x1FFFFF'u32;let b20=(i21 shr 20) and 1;let b10_1=(i21 shr 1) and 0x3FF;let b11=(i21 shr 11) and 1;let b19_12=(i21 shr 12) and 0xFF;(b20 shl 31) or (b10_1 shl 21) or (b11 shl 20) or (b19_12 shl 12) or (rd shl 7) or op
proc encode_instr_stub*(idx:int):uint32=let r=INSTR_TABLE[idx];case r.kind of ikR:encode_r(r.funct7,0,0,r.funct3,0,r.opcode) of ikI:encode_i(0,0,r.funct3,0,r.opcode) of ikS:encode_s(0,0,0,r.funct3,r.opcode) of ikB:encode_b(0,0,0,r.funct3,r.opcode) of ikU:encode_u(0,0,r.opcode) of ikJ:encode_j(0,0,r.opcode)
proc sym_to_instr*(cp:uint32):int=for i in 0..<INSTR_TABLE.len:if INSTR_TABLE[i].sym_codepoint==cp:return i;return -1
