## ms_mcod_demo — MCOD 字体容器 E2E 演示
##
## 1. 选取若干 RV32I 指令 + 操作数
## 2. ms_rv32i.encode 得到机器码
## 3. ms_font.buildTTF 组装成 .ttf 文件，内含 MCOD 自定义表
## 4. writeFile 写盘
## 5. 反读 .ttf 并解析 MCOD 表，验证 codepoint↔semId↔machineCode 三元组恒等

import std/[os, paths, strutils, streams]
import ms_types, ms_rv32i, ms_font

# ═══════════════════════════════════════════════════
#  1. 构造演示指令序列
# ═══════════════════════════════════════════════════

type
  DemoCase = tuple
    semId: string
    op: Operands

const CASES: seq[DemoCase] = @[
  ("ADD",    newOperands(rd = 1'u8, rs1 = 2'u8, rs2 = 3'u8)),
  ("SUB",    newOperands(rd = 2'u8, rs1 = 3'u8, rs2 = 4'u8)),
  ("ADDI",   newOperands(rd = 5'u8, rs1 = 0'u8, imm = 7)),
  ("LW",     newOperands(rd = 3'u8, rs1 = 4'u8, imm = 0)),
  ("SW",     newOperands(rs1 = 5'u8, rs2 = 6'u8, imm = 8)),
  ("BEQ",    newOperands(rs1 = 1'u8, rs2 = 2'u8, imm = 16)),
  ("JAL",    newOperands(rd = 1'u8, imm = 100)),
  ("LUI",    newOperands(rd = 2'u8, imm = 0x12345)),
  ("AUIPC",  newOperands(rd = 3'u8, imm = 0xABCDE)),
  ("ECALL",  noneOperands()),
]

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════════╗"
  echo "║   MCOD 字体容器 · Semantic ID ↔ Machine Code ║"
  echo "║   .ttf 内嵌 MCOD 自定义表                    ║"
  echo "╚══════════════════════════════════════════════╝"
  echo ""

  # ───────────── 编码每个指令（RV32I） ─────────────
  initLuts()
  var entries: seq[McodEntry] = @[]
  var cmapPairs: seq[tuple[cp: uint32; gid: uint16]] = @[]

  echo "[", CASES.len, " 条语义 ID]"
  for i, c in CASES:
    let enc = encodeRV32I(c.semId, c.op)
    if not enc.ok:
      echo "  ✗ encode 失败: ", c.semId, " ", enc.msg
      continue
    let cp = DEFAULT_PUA_BASE + uint32(i)       ## PUA 码点
    let gid = uint16(i + 1)                       ## glyph 0 = .notdef
    entries.add(McodEntry(codepoint: cp, semId: enc.encoded.semId,
                          isaCode: enc.encoded.code))
    cmapPairs.add((cp: cp, gid: gid))
    let code32 = if enc.encoded.code.len >= 4:
      (uint32(enc.encoded.code[3]) shl 24) or (uint32(enc.encoded.code[2]) shl 16) or
      (uint32(enc.encoded.code[1]) shl 8)  or  uint32(enc.encoded.code[0])
    else: 0'u32
    echo "  ", ($c.semId).alignLeft(7), " ", $c.op,
         " → 0x", toHex(code32, 8).toUpperAscii(),
         " codepoint=U+", toHex(cp, 4).toUpperAscii()

  # ───────────── buildTTF ─────────────
  let cfg = FontConfig(
    familyName:   "Hunyuan RV32I MCOD",
    styleName:    "Regular",
    psName:       "Hunyuan-RV32I-MCOD",
    version:      "1.0.0",
    manufacturer: "囡囝共生 OS · 刘楚恬和我",
    unitsPerEm:   1000,
    ascender:     800,
    descender:    -200,
  )

  let nameRecords: seq[NameRecord] = @[
    (3'u16, 1'u16, 0x0409'u16, 1'u16, 0'u16, 0'u16, "Hunyuan RV32I MCOD"),  ## nameID 1 family
    (3'u16, 1'u16, 0x0409'u16, 2'u16, 0'u16, 0'u16, "Regular"),
    (3'u16, 1'u16, 0x0409'u16, 3'u16, 0'u16, 0'u16, "Hunyuan-RV32I-MCOD"),
    (3'u16, 1'u16, 0x0409'u16, 4'u16, 0'u16, 0'u16, "Hunyuan RV32I MCOD Regular"),
    (3'u16, 1'u16, 0x0409'u16, 5'u16, 0'u16, 0'u16, "Version 1.0.0"),
    (3'u16, 1'u16, 0x0409'u16, 6'u16, 0'u16, 0'u16, "Hunyuan-RV32I-MCOD"),
  ]

  echo ""
  echo "─── buildTTF ──"
  let fontBytes = buildTTF(cfg, 700'u16, cmapPairs, nameRecords, entries)
  echo "  文件字节数: ", fontBytes.len

  # ───────────── 写盘 ─────────────
  const OUT_PATH = "hunyuan_rv32i_mcod.ttf"
  writeFile(OUT_PATH, cast[string](fontBytes))
  echo "  已写入: ", OUT_PATH

  # ───────────── 读盘并校验 head / MCOD ─────────────
  echo ""
  echo "─── 读盘校验 ──"
  let disk = readFile(OUT_PATH)
  echo "  读盘字节数: ", disk.len, " (应等于 ", fontBytes.len, ")"

  # head table offset from offset table
  let numTables = uint16(disk[4]) shl 8 or uint16(disk[5])
  echo "  numTables: ", numTables

  # locate MCOD table
  var mcodOffset = -1
  var mcodLength = 0
  for i in 0'u16 ..< numTables:
    let dirOff = 12 + int(i) * 16
    let tag = disk[dirOff .. dirOff + 3]
    if cast[string](tag) == "MCOD":
      let off = (uint32(disk[dirOff + 8]) shl 24) or (uint32(disk[dirOff + 9]) shl 16) or
                (uint32(disk[dirOff + 10]) shl 8) or uint32(disk[dirOff + 11])
      let ln  = (uint32(disk[dirOff + 12]) shl 24) or (uint32(disk[dirOff + 13]) shl 16) or
                (uint32(disk[dirOff + 14]) shl 8) or uint32(disk[dirOff + 15])
      mcodOffset = int(off)
      mcodLength = int(ln)
      break

  if mcodOffset < 0:
    echo "  ✗ MCOD 表未找到"
    quit(1)

  echo "  MCOD table found at offset=", mcodOffset, " len=", mcodLength

  # parse MCOD
  template mc(off: int): uint8 = uint8(disk[mcodOffset + off])
  let mcodVersion = (uint16(mc(0)) shl 8) or uint16(mc(1))
  let mcodNum     = (uint16(mc(2)) shl 8) or uint16(mc(3))
  let strPoolOffB = (uint32(mc(4)) shl 24) or (uint32(mc(5)) shl 16) or
                    (uint32(mc(6)) shl 8) or uint32(mc(7))
  let codePoolOffB= (uint32(mc(8)) shl 24) or (uint32(mc(9)) shl 16) or
                    (uint32(mc(10)) shl 8) or uint32(mc(11))
  let strPoolOff = mcodOffset + int(strPoolOffB)
  let codePoolOff= mcodOffset + int(codePoolOffB)
  echo "  MCOD version=", mcodVersion, " numEntries=", mcodNum,
       " strPool@=", strPoolOffB, " codePool@=", codePoolOffB

  echo ""
  echo "─── 解析 MCOD 三元组 [codepoint → semId → machine code] ──"
  var parseOk = 0
  for i in 0 ..< int(mcodNum):
    let eOff = mcodOffset + 12 + i * 16
    let cp   = (uint32(disk[eOff]) shl 24) or (uint32(disk[eOff + 1]) shl 16) or
               (uint32(disk[eOff + 2]) shl 8) or uint32(disk[eOff + 3])
    let isaFlags = (uint32(disk[eOff + 4]) shl 24) or (uint32(disk[eOff + 5]) shl 16) or
                   (uint32(disk[eOff + 6]) shl 8) or uint32(disk[eOff + 7])
    discard isaFlags
    let sOff = (uint16(disk[eOff + 8]) shl 8) or uint16(disk[eOff + 9])
    let sLen = (uint16(disk[eOff + 10]) shl 8) or uint16(disk[eOff + 11])
    let cOff = (uint16(disk[eOff + 12]) shl 8) or uint16(disk[eOff + 13])
    let cLen = (uint16(disk[eOff + 14]) shl 8) or uint16(disk[eOff + 15])
    var semIdStr = ""
    for j in 0 ..< int(sLen):
      semIdStr.add(char(disk[strPoolOff + int(sOff) + j]))
    var codeBytes = newSeq[uint8]()
    for j in 0 ..< int(cLen):
      codeBytes.add(uint8(disk[codePoolOff + int(cOff) + j]))
    let codeHex = block:
      var s = ""
      for by in codeBytes:
        s.add(toHex(int(by), 2))
      s.toUpperAscii()
    echo "  U+", toHex(cp, 4).toUpperAscii(),
         " → semId=", semIdStr.alignLeft(7),
         " → machine_code=0x", codeHex

    # 校验 codeBytes 与 ms_rv32i 对该 semId 的 encode 一致
    # 从三元重新 encode
    var found = false
    for j, c in CASES:
      if c.semId == semIdStr:
        let re = encodeRV32I(c.semId, c.op)
        if re.encoded.code == codeBytes:
          inc parseOk
          found = true
        break

  echo ""
  echo "MCOD E2E 校验: ", parseOk, "/", mcodNum, " 通过"
  if parseOk == int(mcodNum):
    echo "✓ 全部通过：.ttf 文件内 MCOD 自定义表与 Nim encode 结果恒等"
  echo ""
