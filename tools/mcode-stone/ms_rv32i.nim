## ms_rv32i — RISC-V RV32I 精确编解码器
##
## 编码 Operands → 32位机器码（小端序4字节）
## 解码 32位机器码 → Operands
##
## 满全法规则：运算仅加减（移位立即数掩码等操作视为"位移位域"而非算术）
## 所有 opcode / funct3 / funct7 常量与 RISC-V Unprivileged Spec 20191213 完全一致

import std/[sequtils, strutils, tables]
import ms_types

# ═══════════════════════════════════════════════════
#  指令格式枚举
# ═══════════════════════════════════════════════════

type
  InsnFormat* = enum
    ifR, ifI, ifS, ifB, ifU, ifJ

# ═══════════════════════════════════════════════════
#  RV32I 指令常量表
#
#  opcode(bits[6:0]) funct3(bits[14:12]) funct7(bits[31:25])
#  LUI/AUIPC/JAL/JALR/ECALL/EBREAK: funct3 = 0 (无)
# ═══════════════════════════════════════════════════

type
  RV32IDef* = object
    semId*: string          ## 语义 ID
    opcode*: uint8          ## 7-bit opcode
    funct3*: uint8          ## 3-bit funct3
    funct7*: uint8          ## 7-bit funct7 (可能包含 shamt 位)
    fmt*: InsnFormat        ## 指令格式

# ── 完整 RV32I 指令定义 ──

const RV32I_DEFS*: seq[RV32IDef] = @[
  # ── R-type arithmetic (opcode=0110011) ──
  RV32IDef(semId: "ADD",    opcode: 0b0110011, funct3: 0b000, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "SUB",    opcode: 0b0110011, funct3: 0b000, funct7: 0b0100000, fmt: ifR),
  RV32IDef(semId: "SLL",    opcode: 0b0110011, funct3: 0b001, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "SLT",    opcode: 0b0110011, funct3: 0b010, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "SLTU",   opcode: 0b0110011, funct3: 0b011, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "XOR",    opcode: 0b0110011, funct3: 0b100, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "SRL",    opcode: 0b0110011, funct3: 0b101, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "SRA",    opcode: 0b0110011, funct3: 0b101, funct7: 0b0100000, fmt: ifR),
  RV32IDef(semId: "OR",     opcode: 0b0110011, funct3: 0b110, funct7: 0b0000000, fmt: ifR),
  RV32IDef(semId: "AND",    opcode: 0b0110011, funct3: 0b111, funct7: 0b0000000, fmt: ifR),

  # ── I-type ALU immediate (opcode=0010011) ──
  RV32IDef(semId: "ADDI",   opcode: 0b0010011, funct3: 0b000, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "SLTI",   opcode: 0b0010011, funct3: 0b010, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "SLTIU",  opcode: 0b0010011, funct3: 0b011, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "XORI",   opcode: 0b0010011, funct3: 0b100, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "ORI",    opcode: 0b0010011, funct3: 0b110, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "ANDI",   opcode: 0b0010011, funct3: 0b111, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "SLLI",   opcode: 0b0010011, funct3: 0b001, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "SRLI",   opcode: 0b0010011, funct3: 0b101, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "SRAI",   opcode: 0b0010011, funct3: 0b101, funct7: 0b0100000, fmt: ifI),

  # ── Load (opcode=0000011) ──
  RV32IDef(semId: "LB",     opcode: 0b0000011, funct3: 0b000, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "LH",     opcode: 0b0000011, funct3: 0b001, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "LW",     opcode: 0b0000011, funct3: 0b010, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "LBU",    opcode: 0b0000011, funct3: 0b100, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "LHU",    opcode: 0b0000011, funct3: 0b101, funct7: 0b0000000, fmt: ifI),

  # ── Store (opcode=0100011) ──
  RV32IDef(semId: "SB",     opcode: 0b0100011, funct3: 0b000, funct7: 0b0000000, fmt: ifS),
  RV32IDef(semId: "SH",     opcode: 0b0100011, funct3: 0b001, funct7: 0b0000000, fmt: ifS),
  RV32IDef(semId: "SW",     opcode: 0b0100011, funct3: 0b010, funct7: 0b0000000, fmt: ifS),

  # ── Branch (opcode=1100011) ──
  RV32IDef(semId: "BEQ",    opcode: 0b1100011, funct3: 0b000, funct7: 0b0000000, fmt: ifB),
  RV32IDef(semId: "BNE",    opcode: 0b1100011, funct3: 0b001, funct7: 0b0000000, fmt: ifB),
  RV32IDef(semId: "BLT",    opcode: 0b1100011, funct3: 0b100, funct7: 0b0000000, fmt: ifB),
  RV32IDef(semId: "BGE",    opcode: 0b1100011, funct3: 0b101, funct7: 0b0000000, fmt: ifB),
  RV32IDef(semId: "BLTU",   opcode: 0b1100011, funct3: 0b110, funct7: 0b0000000, fmt: ifB),
  RV32IDef(semId: "BGEU",   opcode: 0b1100011, funct3: 0b111, funct7: 0b0000000, fmt: ifB),

  # ── Jump (opcode=1101111 / 1100111) ──
  RV32IDef(semId: "JAL",    opcode: 0b1101111, funct3: 0b000, funct7: 0b0000000, fmt: ifJ),
  RV32IDef(semId: "JALR",   opcode: 0b1100111, funct3: 0b000, funct7: 0b0000000, fmt: ifI),

  # ── Upper immediate (opcode=0110111 / 0010111) ──
  RV32IDef(semId: "LUI",    opcode: 0b0110111, funct3: 0b000, funct7: 0b0000000, fmt: ifU),
  RV32IDef(semId: "AUIPC",  opcode: 0b0010111, funct3: 0b000, funct7: 0b0000000, fmt: ifU),

  # ── System (opcode=1110011) ──
  RV32IDef(semId: "ECALL",  opcode: 0b1110011, funct3: 0b000, funct7: 0b0000000, fmt: ifI),
  RV32IDef(semId: "EBREAK", opcode: 0b1110011, funct3: 0b000, funct7: 0b0000001, fmt: ifI),
]

# ═══════════════════════════════════════════════════
#  查找表（运行时构建一次）
# ═══════════════════════════════════════════════════

var encodeLUT* = initTable[string, RV32IDef]()
var decodeLUT* = initTable[uint32, string]()

proc initLuts*() =
  ## 初始化查找表（全部 var 初始化一次即可）
  if encodeLUT.len > 0: return
  for d in RV32I_DEFS:
    encodeLUT[d.semId] = d
    # 解码键：opcode(7) | funct3(3) | funct7(7) = 17 bit 唯一键
    let key = (uint32(d.opcode) shl 10) or (uint32(d.funct3) shl 7) or uint32(d.funct7)
    decodeLUT[key] = d.semId

# ═══════════════════════════════════════════════════
#  立即数位压缩：把 Python int 转成 RV32I 位域展开
# ═══════════════════════════════════════════════════

## B-type 立即数 [13-bit signed, 12|10:5|4:1|11]
## 硬件要求：bit[0] 恒为 0（半字对齐）
func encodeBImm*(imm: int32): uint32 =
  let v = uint32(imm)
  let imm12  = (v shr 12) and 0x1
  let imm10_5 = (v shr 5) and 0x3F
  let imm4_1  = (v shr 1) and 0xF
  let imm11   = (v shr 11) and 0x1
  result = (imm12 shl 31) or (imm10_5 shl 25) or (imm4_1 shl 8) or (imm11 shl 7)

func decodeBImm*(bits: uint32): int32 =
  let imm12   = (bits shr 31) and 0x1
  let imm10_5 = (bits shr 25) and 0x3F
  let imm4_1  = (bits shr 8) and 0xF
  let imm11   = (bits shr 7) and 0x1
  let v = (imm12 shl 12) or (imm11 shl 11) or (imm10_5 shl 5) or (imm4_1 shl 1)
  # 符号扩展：bit 12
  if (v and (1 shl 12)) != 0:
    result = cast[int32](v or (0xFFFFFFFF'u32 shl 13))
  else:
    result = cast[int32](v)

## J-type 立即数 [21-bit signed, 20|10:1|11|19:12]
## 硬件要求：bit[0] 恒为 0（半字对齐）
func encodeJImm*(imm: int32): uint32 =
  let v = uint32(imm)
  let imm20    = (v shr 20) and 0x1
  let imm10_1  = (v shr 1) and 0x3FF
  let imm11    = (v shr 11) and 0x1
  let imm19_12 = (v shr 12) and 0xFF
  result = (imm20 shl 31) or (imm19_12 shl 12) or (imm11 shl 20) or (imm10_1 shl 21)

func decodeJImm*(bits: uint32): int32 =
  let imm20    = (bits shr 31) and 0x1
  let imm19_12 = (bits shr 12) and 0xFF
  let imm11    = (bits shr 20) and 0x1
  let imm10_1  = (bits shr 21) and 0x3FF
  let v = (imm20 shl 20) or (imm19_12 shl 12) or (imm11 shl 11) or (imm10_1 shl 1)
  if (v and (1 shl 20)) != 0:
    result = cast[int32](v or (0xFFFFFFFF'u32 shl 21))
  else:
    result = cast[int32](v)

## I-type 立即数：12-bit signed [-2048, 2047]
func encodeIImm*(imm: int32): uint32 =
  result = uint32(imm) and 0xFFF

func decodeIImm*(bits: uint32): int32 =
  # 符号扩展：bit 11 — 使用 cast 做位 cast（避免 RangeDefect）
  if (bits and 0x800) != 0:
    result = cast[int32](bits or (0xFFFFFFFF'u32 shl 12))
  else:
    result = cast[int32](bits and 0xFFF)

## U-type 立即数：20-bit (bits [31:12] = imm[31:12])
func encodeUImm*(imm: int32): uint32 =
  result = uint32(imm) and 0xFFFFF

func decodeUImm*(bits: uint32): int32 =
  result = cast[int32](bits shl 12)

## S-type 立即数（Store），12-bit signed，拆分 [11:5][4:0]
func encodeSImm*(imm: int32): array[2, uint32] =
  let v = uint32(imm) and 0xFFF
  result[0] = (v shr 5) and 0x7F   ## bits[31:25]
  result[1] = v and 0x1F            ## bits[11:7]

func decodeSImm*(hi: uint32; lo: uint32): int32 =
  let v = ((hi and 0x7F) shl 5) or (lo and 0x1F)
  # 符号扩展：bit 11
  if (v and 0x800) != 0:
    result = cast[int32](v or (0xFFFFFFFF'u32 shl 12))
  else:
    result = cast[int32](v)

# ═══════════════════════════════════════════════════
#  encode：semId + Operands → 32-bit 机器码（小端序4字节）
# ═══════════════════════════════════════════════════

proc encodeRV32I*(semId: string; op: Operands): EncResult =
  ## 将语义 ID + 操作数编码为 RV32I 机器码（4字节小端序）
  initLuts()
  let sid = semId.toUpperAscii()
  if sid notin encodeLUT:
    return EncResult(ok: false, error: ceUnknownSemId, msg: "未知语义 ID: " & semId)

  let def = encodeLUT[sid]
  var dw: uint32 = 0

  case def.fmt
  of ifR:
    # funct7[31:25] rs2[24:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
    dw = (uint32(def.funct7) shl 25) or (uint32(op.rs2) shl 20) or
         (uint32(op.rs1) shl 15) or (uint32(def.funct3) shl 12) or
         (uint32(op.rd) shl 7) or uint32(def.opcode)
  of ifI:
    # 特殊：移位指令 SLLI/SRLI/SRAI 的 funct7 空间存 shamt
    if sid in ["SLLI", "SRLI", "SRAI"]:
      # 移位立即数: funct7(0x00 for SLLI/SRLI, 0x20 for SRAI) 在 [31:25]
      let f7 = if sid == "SRAI": 0x20'u32 else: 0'u32
      dw = (f7 shl 25) or (uint32(op.shamt) shl 20) or (uint32(op.rs1) shl 15) or
           (uint32(def.funct3) shl 12) or (uint32(op.rd) shl 7) or uint32(def.opcode)
    # 特殊：ECALL/EBREAK 的 imm 字段存系统调用号
    elif sid == "ECALL":
      dw = uint32(def.opcode)  # imm=0, rs1=0, funct3=0, rd=0
    elif sid == "EBREAK":
      dw = (1'u32 shl 20) or uint32(def.opcode)  # imm=1
    else:
      let iBits = encodeIImm(op.imm)
      dw = (iBits shl 20) or (uint32(op.rs1) shl 15) or
           (uint32(def.funct3) shl 12) or (uint32(op.rd) shl 7) or uint32(def.opcode)
  of ifS:
    let sBits = encodeSImm(op.imm)
    dw = (sBits[0] shl 25) or (uint32(op.rs2) shl 20) or
         (uint32(op.rs1) shl 15) or (uint32(def.funct3) shl 12) or
         (sBits[1] shl 7) or uint32(def.opcode)
  of ifB:
    let bBits = encodeBImm(op.imm)
    dw = (bBits shl 20) or (uint32(op.rs2) shl 20) or
         (uint32(op.rs1) shl 15) or (uint32(def.funct3) shl 12) or
         uint32(def.opcode)
    # B-type 的立即数已经含 bit[31] 和 bit[7]，所以需要直接叠加：
    dw = (uint32(def.funct3) shl 12) or (uint32(op.rs1) shl 15) or
         (uint32(op.rs2) shl 20) or uint32(def.opcode)
    dw = dw or encodeBImm(op.imm)
  of ifU:
    let uBits = encodeUImm(op.imm)
    dw = (uBits shl 12) or (uint32(op.rd) shl 7) or uint32(def.opcode)
  of ifJ:
    let jBits = encodeJImm(op.imm)
    dw = jBits or (uint32(op.rd) shl 7) or uint32(def.opcode)

  # 小端序输出
  var code = newSeq[uint8](4)
  code[0] = uint8(dw and 0xFF)
  code[1] = uint8((dw shr 8) and 0xFF)
  code[2] = uint8((dw shr 16) and 0xFF)
  code[3] = uint8((dw shr 24) and 0xFF)
  EncResult(ok: true, encoded: EncodedInsn(semId: sid, code: code,
                                           byteLen: 4, bitLen: 32))

# ═══════════════════════════════════════════════════
#  decode：32-bit 机器码（小端序4字节） → semId + Operands
# ═══════════════════════════════════════════════════

proc findDefByBits(opcode, funct3: uint8; dw: uint32): string =
  ## 根据 opcode + funct3 + 指令中的歧义位定位语义 ID。
  ## 注意：U-type (LUI=0x37 / AUIPC=0x17) 的 funct3 位是立即数的一部分，
  ##        不是真实 funct3，必须跳过 funct3 匹配。
  let funct7Field = uint8((dw shr 25) and 0x7F)
  let immField12  = (dw shr 20) and 0xFFF
  for d in RV32I_DEFS:
    if d.opcode != opcode: continue
    # U-type: funct3 位是 imm 的高 3 位，按 opcode 唯一匹配
    if opcode in [0b0110111'u8, 0b0010111'u8]:
      return d.semId
    if d.funct3 != funct3: continue
    case opcode
    of 0b0110011:  # R-type: funct7 是真实的
      if d.funct7 == funct7Field: return d.semId
    of 0b0010011:  # I-type: 区分 SLLI/SRLI/SRAI 用 funct7(位[31:25])，ECALL/EBREAK 用 imm
      if d.semId == "SLLI" and funct7Field == 0b0000000: return "SLLI"
      if d.semId == "SRLI" and funct7Field == 0b0000000: return "SRLI"
      if d.semId == "SRAI" and funct7Field == 0b0100000: return "SRAI"
      if d.semId == "ECALL" and immField12 == 0: return "ECALL"
      if d.semId == "EBREAK" and immField12 == 1: return "EBREAK"
      if d.semId notin ["SLLI", "SRLI", "SRAI", "ECALL", "EBREAK"]: return d.semId
    of 0b1110011:  # SYSTEM: ECALL / EBREAK
      if d.semId == "ECALL" and immField12 == 0: return "ECALL"
      if d.semId == "EBREAK" and immField12 == 1: return "EBREAK"
    else:  # 其他 opcode+funct3 唯一对应一条指令
      return d.semId
  return ""

proc decodeRV32I*(code: seq[uint8]): DecResult =
  ## 从 4 字节小端序 RV32I 机器码反推语义 ID + 操作数
  if code.len < 4:
    return DecResult(ok: false, error: ceTruncatedCode, msg: "需要4字节机器码")

  let dw = uint32(code[0]) or (uint32(code[1]) shl 8) or
           (uint32(code[2]) shl 16) or (uint32(code[3]) shl 24)

  let opcode = uint8(dw and 0x7F)
  let funct3 = uint8((dw shr 12) and 0x7)

  let semId = findDefByBits(opcode, funct3, dw)
  if semId.len == 0:
    return DecResult(ok: false, error: ceUnknownOpcode,
                     msg: "未知 opcode=" & toHex(opcode, 2) & " funct3=" & $funct3)

  # 按格式相关字段提取操作数
  var op: Operands

  case opcode
  of 0b0110011:  # R-type: funct7[31:25] rs2[24:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.rs1 = uint8((dw shr 15) and 0x1F)
    op.rs2 = uint8((dw shr 20) and 0x1F)
  of 0b0010011:  # I-type ALU: imm[11:0][31:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.rs1 = uint8((dw shr 15) and 0x1F)
    if semId in ["SLLI", "SRLI", "SRAI"]:
      op.shamt = uint8((dw shr 20) and 0x1F)
    else:
      op.imm = decodeIImm(dw shr 20)
  of 0b0000011:  # Load: imm[11:0][31:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.rs1 = uint8((dw shr 15) and 0x1F)
    op.imm = decodeIImm(dw shr 20)
  of 0b0100011:  # Store: imm[11:5][31:25] rs2[24:20] rs1[19:15] funct3[14:12] imm[4:0][11:7] opcode[6:0]
    op.rs1 = uint8((dw shr 15) and 0x1F)
    op.rs2 = uint8((dw shr 20) and 0x1F)
    let sHi = uint32(dw shr 25) and 0x7F
    let sLo = uint32(dw shr 7) and 0x1F
    op.imm = decodeSImm(sHi, sLo)
  of 0b1100011:  # Branch: rs2[24:20] rs1[19:15] imm 在 [31:25]/[11:7] funct3[14:12]
    op.rs1 = uint8((dw shr 15) and 0x1F)
    op.rs2 = uint8((dw shr 20) and 0x1F)
    op.imm = decodeBImm(dw)
  of 0b1100111:  # JALR: imm[11:0][31:20] rs1[19:15] funct3[14:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.rs1 = uint8((dw shr 15) and 0x1F)
    op.imm = decodeIImm(dw shr 20)
  of 0b1101111:  # JAL: imm[20|10:1|11|19:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.imm = decodeJImm(dw)
  of 0b0110111, 0b0010111:  # LUI / AUIPC: imm[31:12] rd[11:7] opcode[6:0]
    op.rd  = uint8((dw shr 7) and 0x1F)
    op.imm = cast[int32](dw shr 12)    # 20-bit label
  of 0b1110011:  # ECALL / EBREAK (imm 字段存 0/1，不拆回 Operands)
    discard
  else:
    discard

  DecResult(ok: true, decoded: DecodedInsn(semId: semId, operands: op, byteLen: 4))

# ═══════════════════════════════════════════════════
#  Round-trip：encode → decode → 对比
# ═══════════════════════════════════════════════════

proc roundtripRV32I*(semId: string; op: Operands): tuple[ok: bool; original: Operands; recovered: EncodedInsn; decoded: Operands] =
  ## encode 然后 decode，返回原操作数与反推操作数
  let enc = encodeRV32I(semId, op)
  if not enc.ok: return (ok: false, original: op, recovered: EncodedInsn(), decoded: Operands())
  let dec = decodeRV32I(enc.encoded.code)
  if not dec.ok: return (ok: false, original: op, recovered: enc.encoded, decoded: Operands())
  result = (ok: true, original: op, recovered: enc.encoded, decoded: dec.decoded.operands)
  result.ok = (dec.decoded.semId.toUpperAscii() == semId.toUpperAscii()) and result.ok

# ═══════════════════════════════════════════════════
#  元信息查询
# ═══════════════════════════════════════════════════

proc rv32iInsnCount*(): int = RV32I_DEFS.len

proc listSemIds*(): seq[string] =
  ## 全部注册的语义 ID 列表
  result = newSeq[string](RV32I_DEFS.len)
  for i, d in RV32I_DEFS: result[i] = d.semId

proc lookupDef*(semId: string): RV32IDef =
  ## 查询单个指令定义
  initLuts()
  let sid = semId.toUpperAscii()
  if sid in encodeLUT:
    return encodeLUT[sid]
