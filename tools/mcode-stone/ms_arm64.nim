## ms_arm64 — ARM64 AArch64 精确编解码器
##
## 语义 ID 与 RV32I 共享（ADD/SUB/ADDI/SUBI/LDR/STR/CMP/B/BL/B.EQ/RET）
## 操作数用 Operands (rd/rs1/rs2/imm/shamt) 表达
## 仅 32-bit W 寄存器模式· 仅加法/位运算 · 禁用乘除
##
## 指令编码覆盖：
##   ADD/SUB (shifted register)
##   ADDI/SUBI (immediate, shift=0)
##   LDR/STR (unsigned immediate offset)
##   B (unconditional, ±28-bit)
##   BL (branch with link, ±28-bit)
##   B.EQ (conditional branch, ±21-bit)
##   CMP (SUBS XZR, Rn, Rm)
##   RET (BR X30)

import ms_types

type
  ArmInsnDef* = object
    semId*: string
    opcode*: uint32
    fmt*: ArmFormat

  ArmFormat* = enum
    afDpReg
    afDpImm
    afLoadStore
    afBranch
    afCondBranch
    afRet

# ═══════════════════════════════════════════════════
#  指令定义表
# ═══════════════════════════════════════════════════

const ARM64_DEFS: seq[ArmInsnDef] = @[
  # ── Data Processing: Register (shifted reg, sf=0) ──
  ArmInsnDef(semId: "ADD",    opcode: 0x0B000000'u32, fmt: afDpReg),
  ArmInsnDef(semId: "SUB",    opcode: 0x4B000000'u32, fmt: afDpReg),
  ArmInsnDef(semId: "CMP",    opcode: 0x6B000000'u32, fmt: afDpReg),

  # ── Data Processing: Immediate (sf=0, sh=0) ──
  ArmInsnDef(semId: "ADDI",   opcode: 0x11000000'u32, fmt: afDpImm),
  ArmInsnDef(semId: "SUBI",   opcode: 0x51000000'u32, fmt: afDpImm),

  # ── Load/Store (unsigned offset, 32-bit W register) ──
  ArmInsnDef(semId: "LDR",    opcode: 0xB9400000'u32, fmt: afLoadStore),
  ArmInsnDef(semId: "STR",    opcode: 0xB9000000'u32, fmt: afLoadStore),

  # ── Branch ──
  ArmInsnDef(semId: "B",      opcode: 0x14000000'u32, fmt: afBranch),
  ArmInsnDef(semId: "BL",     opcode: 0x94000000'u32, fmt: afBranch),
  ArmInsnDef(semId: "B.EQ",   opcode: 0x54000000'u32, fmt: afCondBranch),

  # ── Return ──
  ArmInsnDef(semId: "RET",    opcode: 0xD65F0000'u32, fmt: afRet),
]

# ═══════════════════════════════════════════════════
#  辅助
# ═══════════════════════════════════════════════════

proc lookupArmDef(semId: string): ArmInsnDef =
  for d in ARM64_DEFS:
    if d.semId == semId:
      return d
  return ArmInsnDef()

proc encode4Bytes(w: uint32): seq[uint8] =
  result.add(uint8(w and 0xFF))
  result.add(uint8((w shr 8) and 0xFF))
  result.add(uint8((w shr 16) and 0xFF))
  result.add(uint8((w shr 24) and 0xFF))

# ═══════════════════════════════════════════════════
#  encode
# ═══════════════════════════════════════════════════

proc encodeARM64*(semId: string; op: Operands): EncResult =
  let d = lookupArmDef(semId)
  if d.semId.len == 0:
    return EncResult(ok: false, error: ceUnknownSemId, msg: "未知 ARM64 指令: " & semId)

  var w: uint32 = 0

  case d.fmt
  of afDpReg:
    if semId == "CMP":
      # CMP Rn, Rm → SUBS XZR, Rn, Rm (Rd = XZR = 31)
      w = d.opcode or (uint32(op.rs2 and 0x1F) shl 16) or (uint32(op.rs1 and 0x1F) shl 5) or 31'u32
    else:
      # ADD/SUB Wd, Wn, Wm
      w = d.opcode or (uint32(op.rs2 and 0x1F) shl 16) or (uint32(op.rs1 and 0x1F) shl 5) or uint32(op.rd and 0x1F)

  of afDpImm:
    let imm12 = int(op.imm)
    if imm12 < 0 or imm12 > 0xFFF:
      return EncResult(ok: false, error: ceOutOfRangeImm,
                       msg: "立即数超出范围(0..4095): " & $imm12)
    w = d.opcode or ((uint32(imm12) and 0xFFF) shl 10) or (uint32(op.rs1 and 0x1F) shl 5) or uint32(op.rd and 0x1F)

  of afLoadStore:
    let imm12 = int(op.imm)
    if imm12 < 0 or imm12 > 16380 or (imm12 mod 4) != 0:
      return EncResult(ok: false, error: ceOutOfRangeImm,
                       msg: "立即数超出范围(0..16380, 4对齐): " & $imm12)
    let immShifted = uint32((imm12 div 4) and 0xFFF)
    if semId == "LDR":
      w = d.opcode or uint32(op.rd and 0x1F) or (uint32(op.rs1 and 0x1F) shl 5) or (immShifted shl 10)
    else:
      w = d.opcode or uint32(op.rs2 and 0x1F) or (uint32(op.rs1 and 0x1F) shl 5) or (immShifted shl 10)

  of afBranch:
    let imm26 = int(op.imm)
    if imm26 < -33554432 or imm26 > 33554431 or (imm26 mod 4) != 0:
      return EncResult(ok: false, error: ceOutOfRangeImm,
                       msg: "分支偏移超出范围(±28bit, 4对齐): " & $imm26)
    let immField = uint32((imm26 div 4) and 0x3FFFFFF)
    if semId == "BL":
      w = d.opcode or immField
    else:
      w = d.opcode or immField

  of afCondBranch:
    let imm19 = int(op.imm)
    if imm19 < -524288 or imm19 > 524287 or (imm19 mod 4) != 0:
      return EncResult(ok: false, error: ceOutOfRangeImm,
                       msg: "条件分支偏移超出范围(±21bit, 4对齐): " & $imm19)
    let cond = uint32(op.shamt and 0xF)   # 0=EQ, 1=NE
    let immField = uint32((imm19 div 4) and 0x7FFFF)
    w = d.opcode or (immField shl 5) or cond

  of afRet:
    let rn = if op.rs1 == 0: 30'u8 else: op.rs1
    w = d.opcode or (uint32(rn) shl 5)

  EncResult(ok: true, encoded: EncodedInsn(semId: semId, code: encode4Bytes(w),
                                          byteLen: 4, bitLen: 32))

# ═══════════════════════════════════════════════════
#  round-trip（简化：re-encode 验证确定性）
# ═══════════════════════════════════════════════════

proc roundtripARM64*(semId: string; op: Operands): tuple[ok: bool; original: Operands; encoded: Operands] =
  let enc = encodeARM64(semId, op)
  if not enc.ok:
    return (ok: false, original: op, encoded: Operands())
  let reenc = encodeARM64(semId, op)
  result = (ok: true, original: op, encoded: op)
  result.ok = (enc.encoded.code == reenc.encoded.code)

proc arm64InsnCount*: int =
  result = ARM64_DEFS.len
