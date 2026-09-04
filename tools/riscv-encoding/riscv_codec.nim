## RISC-V RV32I 编码/解码器 — 字体符号双向映射
##
## 设计:
##   1. 每条指令映射到 Unicode 私有区码点 (U+F000 起)
##   2. 机器码 → 符号: 指令骨架编码对应唯一私有区字形
##   3. 符号 → 机器码: 从私有区码点反查指令记录
##   4. 位域严格按 RISC-V Unprivileged Spec 20191213
##
## 格式参考 tools/isa-ref/riscv.md

import std/[strutils, os, osproc]

# ── 操作码 (7 bit) ─────────────────────────────────────────
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

# ── 指令记录 ───────────────────────────────────────────────
type
  InstrKind* = enum
    ikR, ikI, ikS, ikB, ikU, ikJ

  InstrRecord* = object
    name*: string
    kind*: InstrKind
    opcode*: uint32
    funct3*: uint32
    funct7*: uint32
    sym_codepoint*: uint32

# ── 符号码点基址 (Private Use Area) ───────────────────────
const SYM_BASE* = 0xF000'u32

# ── 指令表 (19 条核心指令) ───────────────────────────────
const INSTR_TABLE*: array[19, InstrRecord] = [
  # R-type 算术逻辑
  InstrRecord(name: "add",  kind: ikR, opcode: OP_OP,     funct3: 0b000, funct7: 0b0000000, sym_codepoint: SYM_BASE + 0),
  InstrRecord(name: "sub",  kind: ikR, opcode: OP_OP,     funct3: 0b000, funct7: 0b0100000, sym_codepoint: SYM_BASE + 1),
  InstrRecord(name: "xor",  kind: ikR, opcode: OP_OP,     funct3: 0b100, funct7: 0b0000000, sym_codepoint: SYM_BASE + 2),
  InstrRecord(name: "or",   kind: ikR, opcode: OP_OP,     funct3: 0b110, funct7: 0b0000000, sym_codepoint: SYM_BASE + 3),
  InstrRecord(name: "and",  kind: ikR, opcode: OP_OP,     funct3: 0b111, funct7: 0b0000000, sym_codepoint: SYM_BASE + 4),
  InstrRecord(name: "sll",  kind: ikR, opcode: OP_OP,     funct3: 0b001, funct7: 0b0000000, sym_codepoint: SYM_BASE + 5),
  InstrRecord(name: "srl",  kind: ikR, opcode: OP_OP,     funct3: 0b101, funct7: 0b0000000, sym_codepoint: SYM_BASE + 6),
  InstrRecord(name: "sra",  kind: ikR, opcode: OP_OP,     funct3: 0b101, funct7: 0b0100000, sym_codepoint: SYM_BASE + 7),

  # I-type 立即数
  InstrRecord(name: "addi", kind: ikI, opcode: OP_OP_IMM, funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 8),
  InstrRecord(name: "slti", kind: ikI, opcode: OP_OP_IMM, funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 9),
  InstrRecord(name: "andi", kind: ikI, opcode: OP_OP_IMM, funct3: 0b111, funct7: 0,          sym_codepoint: SYM_BASE + 10),

  # 访存
  InstrRecord(name: "lw",   kind: ikI, opcode: OP_LOAD,   funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 11),
  InstrRecord(name: "sw",   kind: ikS, opcode: OP_STORE,  funct3: 0b010, funct7: 0,          sym_codepoint: SYM_BASE + 12),

  # 分支
  InstrRecord(name: "beq",  kind: ikB, opcode: OP_BRANCH, funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 13),
  InstrRecord(name: "bne",  kind: ikB, opcode: OP_BRANCH, funct3: 0b001, funct7: 0,          sym_codepoint: SYM_BASE + 14),

  # 跳转
  InstrRecord(name: "jal",  kind: ikJ, opcode: OP_JAL,    funct3: 0,     funct7: 0,          sym_codepoint: SYM_BASE + 15),
  InstrRecord(name: "jalr", kind: ikI, opcode: OP_JALR,   funct3: 0b000, funct7: 0,          sym_codepoint: SYM_BASE + 16),

  # 高位立即数
  InstrRecord(name: "lui",  kind: ikU, opcode: OP_LUI,    funct3: 0,     funct7: 0,          sym_codepoint: SYM_BASE + 17),
  InstrRecord(name: "auipc", kind: ikU, opcode: OP_AUIPC, funct3: 0, funct7: 0,              sym_codepoint: SYM_BASE + 18),
]

# ── 编码函数 ───────────────────────────────────────────────
proc encode_r(funct7, rs2, rs1, funct3, rd, opcode: uint32): uint32 =
  result = (funct7 shl 25) or (rs2 shl 20) or (rs1 shl 15) or
           (funct3 shl 12) or (rd shl 7) or opcode

proc encode_i(imm, rs1, funct3, rd, opcode: uint32): uint32 =
  let imm12 = imm and 0xFFF
  result = (imm12 shl 20) or (rs1 shl 15) or (funct3 shl 12) or
           (rd shl 7) or opcode

proc encode_s(imm, rs2, rs1, funct3, opcode: uint32): uint32 =
  let imm12 = imm and 0xFFF
  let imm_hi = (imm12 shr 5) and 0x7F
  let imm_lo = imm12 and 0x1F
  result = (imm_hi shl 25) or (rs2 shl 20) or (rs1 shl 15) or
           (funct3 shl 12) or (imm_lo shl 7) or opcode

proc encode_b(imm, rs2, rs1, funct3, opcode: uint32): uint32 =
  let imm13 = imm and 0x1FFF
  let b12    = (imm13 shr 12) and 0x1
  let b10_5  = (imm13 shr 5) and 0x3F
  let b4_1   = (imm13 shr 1) and 0xF
  let b11    = (imm13 shr 11) and 0x1
  let field_hi = (b12 shl 6) or b10_5
  let field_lo = (b4_1 shl 1) or b11
  result = (field_hi shl 25) or (rs2 shl 20) or (rs1 shl 15) or
           (funct3 shl 12) or (field_lo shl 7) or opcode

proc encode_u(imm, rd, opcode: uint32): uint32 =
  result = (imm and 0xFFFFF000'u32) or (rd shl 7) or opcode

proc encode_j(imm, rd, opcode: uint32): uint32 =
  let imm21 = imm and 0x1FFFFF'u32
  let b20    = (imm21 shr 20) and 0x1
  let b10_1  = (imm21 shr 1) and 0x3FF
  let b11    = (imm21 shr 11) and 0x1
  let b19_12 = (imm21 shr 12) and 0xFF
  result = (b20 shl 31) or (b10_1 shl 21) or (b11 shl 20) or
           (b19_12 shl 12) or (rd shl 7) or opcode

# ── 指令骨架编码 (操作数=0) ─────────────────────────────
proc encode_instr_stub*(idx: int): uint32 =
  let rec = INSTR_TABLE[idx]
  case rec.kind
  of ikR: encode_r(rec.funct7, 0, 0, rec.funct3, 0, rec.opcode)
  of ikI: encode_i(0, 0, rec.funct3, 0, rec.opcode)
  of ikS: encode_s(0, 0, 0, rec.funct3, rec.opcode)
  of ikB: encode_b(0, 0, 0, rec.funct3, rec.opcode)
  of ikU: encode_u(0, 0, rec.opcode)
  of ikJ: encode_j(0, 0, rec.opcode)

# ── 解码: uint32 → 助记符 + 操作数 ───────────────────────
proc decode_machine(code: uint32): string =
  let opcode = code and 0x7F
  let rd     = (code shr 7) and 0x1F
  let funct3 = (code shr 12) and 0x7
  let rs1    = (code shr 15) and 0x1F
  let rs2    = (code shr 20) and 0x1F
  let funct7 = (code shr 25) and 0x7F

  for i in 0 ..< INSTR_TABLE.len:
    let rec = INSTR_TABLE[i]
    if rec.opcode != opcode: continue
    if rec.kind notin {ikU, ikJ} and rec.funct3 != funct3: continue
    if rec.kind == ikR and rec.funct7 != funct7: continue

    case rec.kind
    of ikR:
      return rec.name & " x" & $rd & ", x" & $rs1 & ", x" & $rs2
    of ikI:
      let imm = (code shr 20) and 0xFFF
      let simm = cast[int32](if imm >= 0x800: imm - 0x1000 else: imm)
      if rec.opcode == OP_LOAD or rec.opcode == OP_JALR:
        return rec.name & " x" & $rd & ", " & $simm & "(x" & $rs1 & ")"
      else:
        return rec.name & " x" & $rd & ", x" & $rs1 & ", " & $simm
    of ikS:
      let imm = ((funct7 shl 5) or rd) and 0xFFF
      let simm = cast[int32](if imm >= 0x800: imm - 0x1000 else: imm)
      return rec.name & " x" & $rs2 & ", " & $simm & "(x" & $rs1 & ")"
    of ikB:
      let b12   = (code shr 31) and 1
      let b10_5 = (code shr 25) and 0x3F
      let b4_1  = (code shr 8) and 0xF
      let b11   = (code shr 7) and 1
      let imm = (b12 shl 12) or (b11 shl 11) or (b10_5 shl 5) or (b4_1 shl 1)
      let simm = cast[int32](if imm >= 0x1000: imm - 0x2000 else: imm)
      return rec.name & " x" & $rs1 & ", x" & $rs2 & ", " & $simm
    of ikU:
      let imm = code and 0xFFFFF000'u32
      if rec.opcode == OP_LUI:
        return "lui x" & $rd & ", 0x" & toHex(imm)
      else:
        return "auipc x" & $rd & ", 0x" & toHex(imm)
    of ikJ:
      let b20    = (code shr 31) and 1
      let b10_1  = (code shr 21) and 0x3FF
      let b11    = (code shr 20) and 1
      let b19_12 = (code shr 12) and 0xFF
      let imm21 = (b20 shl 20) or (b19_12 shl 12) or (b11 shl 11) or (b10_1 shl 1)
      let simm = cast[int32](if imm21 >= 0x100000: imm21 - 0x200000 else: imm21)
      return "jal x" & $rd & ", " & $simm

  return "[未知指令 0x" & toHex(code).toUpperAscii & "]"

# ── 符号 ↔ 指令 查找 ─────────────────────────────────────
proc sym_to_instr*(cp: uint32): int =
  for i in 0 ..< INSTR_TABLE.len:
    if INSTR_TABLE[i].sym_codepoint == cp: return i
  return -1

proc instr_to_sym*(idx: int): uint32 =
  if idx >= 0 and idx < INSTR_TABLE.len:
    return INSTR_TABLE[idx].sym_codepoint
  return 0

# ── JSON 导出 (给字体生成器) ──────────────────────────────
proc export_json*(): string =
  var parts: seq[string] = @[]
  for i in 0 ..< INSTR_TABLE.len:
    let rec = INSTR_TABLE[i]
    let machine = encode_instr_stub(i)
    var parts_inner: seq[string] = @[]
    parts_inner.add("\"name\":\"" & rec.name & "\"")
    parts_inner.add("\"machine\":\"" & toHex(machine).toUpperAscii & "\"")
    parts_inner.add("\"format\":\"" & $rec.kind & "\"")
    if rec.kind == ikR:
      parts_inner.add("\"funct7\":\"" & align($rec.funct7, 7, '0') & "\"")
    if rec.kind in {ikR, ikI, ikS, ikB}:
      parts_inner.add("\"funct3\":\"" & align($rec.funct3, 3, '0') & "\"")
    parts.add("  \"" & toHex(rec.sym_codepoint) & "\": {" & parts_inner.join(",") & "}")
  result = "{\n" & parts.join(",\n") & "\n}"

# ── 一键生成字体 ─────────────────────────────────────────
proc gen_font*(out_path: string): bool =
  let json_str = export_json()
  let json_path = out_path & ".mapping.json"
  writeFile(json_path, json_str)
  let py_script = currentSourcePath().parentDir() / "gen_rv32i_font.py"
  let cmd = "python3 " & $py_script & " " & json_path & " " & out_path
  let (output, exit_code) = execCmdEx(cmd)
  stdout.write(output)
  if exit_code == 0:
    removeFile(json_path)
    return true
  return false

# ════════════════════════════════════════════════════════════
# 测试
# ════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ RISC-V RV32I 编码/解码原型 ═══"
  echo ""

  echo "【指令 → 符号码点 → 骨架机器码】"
  for i in 0 ..< INSTR_TABLE.len:
    let rec = INSTR_TABLE[i]
    let machine = encode_instr_stub(i)
    echo "  ", rec.name.alignLeft(6), " → U+", toHex(rec.sym_codepoint).toUpperAscii,
         " → 0x", toHex(machine).toUpperAscii
  echo ""

  echo "【编解码往返测试】"
  let tests: seq[tuple[desc: string, code: uint32]] = @[
    ("add x1, x1, x2",  encode_r(0b0000000, 2, 1, 0b000, 1, OP_OP)),
    ("sub x1, x1, x2",  encode_r(0b0100000, 2, 1, 0b000, 1, OP_OP)),
    ("xor x3, x4, x5",  encode_r(0b0000000, 5, 4, 0b100, 3, OP_OP)),
    ("addi x1, x2, 1",   encode_i(1, 2, 0b000, 1, OP_OP_IMM)),
    ("addi x1, x2, -1",  encode_i(0xFFF'u32, 2, 0b000, 1, OP_OP_IMM)),
    ("lw x3, 4(x5)",     encode_i(4, 5, 0b010, 3, OP_LOAD)),
    ("lw x3, -4(x5)",    encode_i(0xFFC'u32, 5, 0b010, 3, OP_LOAD)),
    ("sw x3, 8(x5)",     encode_s(8, 3, 5, 0b010, OP_STORE)),
    ("beq x1, x2, +16",  encode_b(16, 2, 1, 0b000, OP_BRANCH)),
    ("beq x1, x2, -16",  encode_b(0x1FFF'u32 - 16 + 1, 2, 1, 0b000, OP_BRANCH)),
    ("jal x1, +8",       encode_j(8, 1, OP_JAL)),
    ("jal x1, +1024",    encode_j(1024, 1, OP_JAL)),
    ("jal x0, -8",       encode_j(0x1FFFFF'u32 - 8 + 1, 0, OP_JAL)),
    ("lui x1, 0x12345",  encode_u(0x12345 shl 12, 1, OP_LUI)),
    ("auipc x2, 0xABCDE", encode_u(0xABCDE shl 12, 2, OP_AUIPC)),
  ]

  var pass = 0
  for t in tests:
    let decoded = decode_machine(t.code)
    let ok = decoded.startsWith(t.desc.splitWhitespace()[0])
    if ok: pass += 1
    var suffix = ""
    if not ok: suffix = " ✗"
    echo "  ", t.desc.alignLeft(20), " 0x", toHex(t.code).toUpperAscii, " ← ",
         decoded, suffix
  echo "  通过: ", pass, "/", tests.len
  echo ""

  echo "【字体符号闭环验证】"
  var sym_ok = 0
  for i in 0 ..< INSTR_TABLE.len:
    let rec = INSTR_TABLE[i]
    let machine = encode_instr_stub(i)
    let decoded = decode_machine(machine)
    if decoded.startsWith(rec.name):
      sym_ok += 1
    else:
      echo "  ✗ ", rec.name, "(U+", toHex(rec.sym_codepoint).toUpperAscii,
           ") → 0x", toHex(machine).toUpperAscii, " → ", decoded
  echo "  ", sym_ok, "/", INSTR_TABLE.len, " 条指令闭环正确"
  echo ""

  # ── 命令行接口 ─────────────────────────────────────────
  let args = commandLineParams()
  if args.len >= 1:
    case args[0]
    of "--export", "-e":
      echo export_json()
    of "--genfont", "-g":
      let out_path = if args.len >= 2: args[1] else: "rv32i_mc.ttf"
      echo "═══ 一键生成 RV32I 机器码字体 ═══"
      echo ""
      if gen_font(out_path):
        echo ""
        echo "═══ 完成: ", out_path, " ═══"
      else:
        echo "═══ 字体生成失败 ═══"
    else:
      echo "用法: riscv_codec [--export|-e | --genfont|-g [output.ttf]]"
  echo ""
  echo "═══ 完成 ═══"
