# ============================================================
# 机器码双向转换 · 字体 MCOD 表编解码器
#
# encode：语义 ID + 操作数 → 机器码字节数组
# decode：机器码字节数组 → 语义 ID + 操作数
# loadMCOD：从伪字体描述结构加载对齐表
#
# 不涉及运行时——只做位域拼接与拆解。
# 等 Nim 编译器与 wasi-sdk 就绪后，对接真实 .ttf 文件。
# ============================================================

import mc_types, mc_align

type
  Operands* = object
    rd*: uint8
    rs1*: uint8
    rs2*: uint8
    imm*: int32
    shamt*: uint8

proc fieldBits(f: Field): int = f.hi - f.lo + 1

proc encode*(enc: InsnEncoding, op: Operands): seq[uint8] =
  ## 将操作数按位域模板写入对应 ISA 编码模板
  if enc.byteLen == 0:
    return
  result = newSeq[uint8](enc.byteLen)
  var dw: uint32 = 0
  for f in enc.fields:
    let width = fieldBits(f)
    let mask = (1'u32 shl width) - 1
    case f.format
    of ffRegDst:  dw = dw or ((op.rd.uint32 and mask) shl f.lo)
    of ffRegSrc1: dw = dw or ((op.rs1.uint32 and mask) shl f.lo)
    of ffRegSrc2:
      if f.name == "rt" or f.name == "Rm":
        dw = dw or ((op.rs2.uint32 and mask) shl f.lo)
      else:
        dw = dw or ((op.rs2.uint32 and mask) shl f.lo)
    of ffShamt:   dw = dw or ((op.shamt.uint32 and mask) shl f.lo)
    of ffImmI:
      if f.hi > 19:  # RISC-V I-type imm extends sign
        let v = cast[int32](op.imm) and ((1 shl width) - 1)
        dw = dw or ((cast[uint32](v) and mask) shl f.lo)
      else:
        dw = dw or ((cast[uint32](op.imm) and mask) shl f.lo)
    of ffImmS:
      if f.name == "imm_hi":
        let v = (cast[uint32](op.imm) shr 5) and mask
        dw = dw or (v shl f.lo)
      else:
        let v = cast[uint32](op.imm) and mask
        dw = dw or (v shl f.lo)
    of ffImmU:
      let v = (cast[uint32](op.imm) shr 12) and mask
      dw = dw or (v shl f.lo)
    of ffImmB, ffImmJ:
      let v = cast[uint32](op.imm) and mask
      dw = dw or (v shl f.lo)
    of ffFunct3, ffFunct7, ffOpcode, ffSuffix:
      # 常量位在具体 ISA 模块中填；对齐表保留占位
      discard
  # 小端序写入
  for i in 0 ..< enc.byteLen:
    result[i] = uint8((dw shr (i * 8)) and 0xFF)

proc decode*(enc: InsnEncoding, code: seq[uint8]): Operands =
  ## 从机器码字节数组反推操作数
  if code.len < enc.byteLen:
    return
  var dw: uint32 = 0
  for i in 0 ..< enc.byteLen:
    dw = dw or (code[i].uint32 shl (i * 8))
  for f in enc.fields:
    let width = fieldBits(f)
    let mask = (1'u32 shl width) - 1
    let v = (dw shr f.lo) and mask
    case f.format
    of ffRegDst:  result.rd = uint8(v)
    of ffRegSrc1: result.rs1 = uint8(v)
    of ffRegSrc2: result.rs2 = uint8(v)
    of ffShamt:   result.shamt = uint8(v)
    of ffImmI:    result.imm = cast[int32](v)
    of ffImmS, ffImmB, ffImmJ, ffImmU:
      result.imm = cast[int32](v shl (32 - width))
    of ffFunct3, ffFunct7, ffOpcode, ffSuffix:
      discard

proc findAlignment*(semId: string): int =
  ## 在跨 ISA 对齐表中查找语义 ID 索引
  for i in 0 ..< ALIGN_TABLE.len:
    if ALIGN_TABLE[i].semId == semId:
      return i
  return -1

proc summary*(semId: string): string =
  ## 生成跨 ISA 对齐摘要文本
  let idx = findAlignment(semId)
  if idx < 0:
    return "[未找到: " & semId & "]"
  let e = ALIGN_TABLE[idx]
  var lines: seq[string] = @[]
  lines.add("═══ " & e.semId & " ═" & "═".repeat(40))
  lines.add("bit 宽度: " & $e.bitLen & " (" & $e.description & ")")
  lines.add("── 各 ISA 助记符对齐 ──")
  for kv in e.asm:
    let isaName = case kv.isa
      of isaRISCV32: "RV32I "
      of isaRISCV64: "RV64I "
      of isaARM64:   "ARM64 "
      of isaX86_64:  "x86-64"
      of isaMIPS32:  "MIPS32"
    lines.add("  " & isaName & " → " & kv.text)
  return lines.join("\n")
