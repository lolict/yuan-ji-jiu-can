## ms_mcod_dual — 双 ISA MCOD 字体容器 E2E 演示
##
## 同一个 .ttf 文件同时嵌入 RV32I 与 ARM64 两套机器码
## 共用语义 ID（如 "ADD"），不同 PUA 段区分 ISA：
##   U+F000..  = RV32I  (isaId=1)
##   U+F100..  = ARM64  (isaId=2)

import std/[os, strutils]
import ms_types, ms_rv32i, ms_arm64, ms_font

type
  DualEntry = object
    semId: string
    isaId: int
    op: Operands

  IsaParser = object
    semId: string
    isaId: int
    op: Operands

# RV32I 指令序列（39条）
const RV32I_CASES: seq[tuple[semId: string; op: Operands]] = @[
  ("ADD",   newOperands(rd=1'u8, rs1=2'u8, rs2=3'u8)),
  ("SUB",   newOperands(rd=2'u8, rs1=3'u8, rs2=4'u8)),
  ("SLL",   newOperands(rd=4'u8, rs1=8'u8, rs2=2'u8)),
  ("SLT",   newOperands(rd=28'u8, rs1=29'u8, rs2=30'u8)),
  ("SLTU",  newOperands(rd=3'u8, rs1=4'u8, rs2=5'u8)),
  ("XOR",   newOperands(rd=14'u8, rs1=15'u8, rs2=16'u8)),
  ("SRL",   newOperands(rd=17'u8, rs1=18'u8, rs2=19'u8)),
  ("SRA",   newOperands(rd=20'u8, rs1=21'u8, rs2=22'u8)),
  ("OR",    newOperands(rd=6'u8, rs1=7'u8, rs2=8'u8)),
  ("AND",   newOperands(rd=9'u8, rs1=10'u8, rs2=11'u8)),
  ("ADDI",  newOperands(rd=3'u8, rs1=4'u8, imm=(-128))),
  ("SLTI",  newOperands(rd=1'u8, rs1=2'u8, imm=100)),
  ("SLTIU", newOperands(rd=31'u8, rs1=30'u8, imm=1)),
  ("XORI",  newOperands(rd=25'u8, rs1=26'u8, imm=0x7FF)),
  ("ORI",   newOperands(rd=2'u8, rs1=3'u8, imm=42)),
  ("ANDI",  newOperands(rd=4'u8, rs1=5'u8, imm=0xFF)),
  ("SLLI",  newOperands(rd=6'u8, rs1=7'u8, shamt=13)),
  ("SRLI",  newOperands(rd=8'u8, rs1=9'u8, shamt=4)),
  ("SRAI",  newOperands(rd=10'u8, rs1=11'u8, shamt=20)),
  ("LB",    newOperands(rd=5'u8, rs1=6'u8, imm=(-4))),
  ("LH",    newOperands(rd=7'u8, rs1=8'u8, imm=0)),
  ("LW",    newOperands(rd=9'u8, rs1=10'u8, imm=256)),
  ("LBU",   newOperands(rd=11'u8, rs1=12'u8, imm=2047)),
  ("LHU",   newOperands(rd=13'u8, rs1=14'u8, imm=(-2048))),
  ("SB",    newOperands(rs1=15'u8, rs2=16'u8, imm=100)),
  ("SH",    newOperands(rs1=17'u8, rs2=18'u8, imm=(-50))),
  ("SW",    newOperands(rs1=19'u8, rs2=20'u8, imm=0)),
  ("BEQ",   newOperands(rs1=1'u8, rs2=2'u8, imm=16)),
  ("BNE",   newOperands(rs1=3'u8, rs2=4'u8, imm=(-32))),
  ("BLT",   newOperands(rs1=5'u8, rs2=6'u8, imm=64)),
  ("BGE",   newOperands(rs1=7'u8, rs2=8'u8, imm=(-128))),
  ("BLTU",  newOperands(rs1=9'u8, rs2=10'u8, imm=256)),
  ("BGEU",  newOperands(rs1=11'u8, rs2=12'u8, imm=(-512))),
  ("JAL",   newOperands(rd=1'u8, imm=100)),
  ("JALR",  newOperands(rd=0'u8, rs1=1'u8, imm=4)),
  ("LUI",   newOperands(rd=2'u8, imm=0x12345)),
  ("AUIPC", newOperands(rd=3'u8, imm=0xABCDE)),
  ("ECALL", noneOperands()),
  ("EBREAK",noneOperands()),
]

# ARM64 指令序列（11条）
const ARM64_CASES: seq[tuple[semId: string; op: Operands]] = @[
  ("ADD",   newOperands(rd=0'u8, rs1=1'u8, rs2=2'u8)),
  ("SUB",   newOperands(rd=3'u8, rs1=4'u8, rs2=5'u8)),
  ("CMP",   newOperands(rs1=1'u8, rs2=2'u8)),
  ("ADDI",  newOperands(rd=0'u8, rs1=1'u8, imm=7)),
  ("SUBI",  newOperands(rd=2'u8, rs1=3'u8, imm=100)),
  ("LDR",   newOperands(rd=0'u8, rs1=1'u8, imm=0)),
  ("STR",   newOperands(rs1=0'u8, rs2=1'u8, imm=4)),
  ("B",     newOperands(imm=100)),
  ("BL",    newOperands(imm=268)),
  ("B.EQ",  newOperands(imm=64, shamt=0)),
  ("RET",   noneOperands()),
]

proc w32ToHex(w: uint32): string =
  toHex(w, 8).toUpperAscii()

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║   MCOD 双 ISA 容器 · RV32I + ARM64              ║"
  echo "║   同一 TTF · 语义 ID 共享 · 机器码保异            ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo ""

  initLuts()
  var entries: seq[McodEntry] = @[]
  var cmapPairs: seq[tuple[cp: uint32; gid: uint16]] = @[]

  # RV32I → codepoints U+F000.., isaId=1
  echo "[STEP 1] RV32I 编码 ..."
  for i, c in RV32I_CASES:
    let enc = encodeRV32I(c.semId, c.op)
    if not enc.ok:
      echo "  ✗ RV32I encode 失败: ", c.semId, " ", enc.msg
      quit(1)
    let cp = DEFAULT_PUA_BASE + uint32(i)
    entries.add(McodEntry(codepoint: cp, semId: enc.encoded.semId,
                          isaCode: enc.encoded.code, isaId: ISA_RV32I))
    cmapPairs.add((cp: cp, gid: uint16(i + 1)))
    let w = uint32(enc.encoded.code[0]) or (uint32(enc.encoded.code[1]) shl 8) or
            (uint32(enc.encoded.code[2]) shl 16) or (uint32(enc.encoded.code[3]) shl 24)
    echo "  RV32I  ", ($c.semId).alignLeft(7), " U+", toHex(cp,4).toUpperAscii(), " → 0x", w32ToHex(w)

  # ARM64 → codepoints U+F100.., isaId=2
  echo ""
  echo "[STEP 2] ARM64 编码 ..."
  for i, c in ARM64_CASES:
    let enc = encodeARM64(c.semId, c.op)
    if not enc.ok:
      echo "  ✗ ARM64 encode 失败: ", c.semId, " ", enc.msg
      quit(1)
    let cp = PUA_BASE_ARM64 + uint32(i)
    entries.add(McodEntry(codepoint: cp, semId: enc.encoded.semId,
                          isaCode: enc.encoded.code, isaId: ISA_ARM64))
    let gidOffset = uint16(RV32I_CASES.len + i + 1)
    cmapPairs.add((cp: cp, gid: gidOffset))
    let w = uint32(enc.encoded.code[0]) or (uint32(enc.encoded.code[1]) shl 8) or
            (uint32(enc.encoded.code[2]) shl 16) or (uint32(enc.encoded.code[3]) shl 24)
    echo "  ARM64  ", ($c.semId).alignLeft(7), " U+", toHex(cp,4).toUpperAscii(), " → 0x", w32ToHex(w)

  # buildTTF
  echo ""
  echo "[STEP 3] buildTTF (11 表 + MCOD 双 ISA) ..."
  let cfg = FontConfig(
    familyName:   "Hunyuan Dual-ISA MCOD",
    styleName:    "Regular",
    psName:       "Hunyuan-DualISA-MCOD",
    version:      "1.0.0",
    manufacturer: "囡囝共生 OS · 刘楚恬和我",
    unitsPerEm:   1000,
    ascender:     800,
    descender:    -200,
  )
  let nameRecords: seq[NameRecord] = @[
    (3'u16, 1'u16, 0x0409'u16, 1'u16, 0'u16, 0'u16, "Hunyuan Dual-ISA MCOD"),
    (3'u16, 1'u16, 0x0409'u16, 2'u16, 0'u16, 0'u16, "Regular"),
    (3'u16, 1'u16, 0x0409'u16, 6'u16, 0'u16, 0'u16, "Hunyuan-DualISA-MCOD"),
  ]
  let fontBytes = buildTTF(cfg, 700'u16, cmapPairs, nameRecords, entries)
  echo "  文件字节数: ", fontBytes.len

  # write
  const OUT_PATH = "hunyuan_dual_isa_mcod.ttf"
  writeFile(OUT_PATH, cast[string](fontBytes))
  echo "  已写入: ", OUT_PATH

  # read back & parse
  echo ""
  echo "[STEP 4] 读盘校验 + 解析 MCOD 三元组 ..."
  let disk = readFile(OUT_PATH)
  echo "  读盘字节数: ", disk.len
  let numTables = uint16(disk[4]) shl 8 or uint16(disk[5])
  echo "  numTables: ", numTables

  var mcodOff = -1; var mcodLen = 0
  for i in 0'u16 ..< numTables:
    let dirOff = 12 + int(i) * 16
    if cast[string](disk[dirOff .. dirOff + 3]) == "MCOD":
      mcodOff = int((uint32(disk[dirOff + 8]) shl 24) or (uint32(disk[dirOff + 9]) shl 16) or
                    (uint32(disk[dirOff + 10]) shl 8) or uint32(disk[dirOff + 11]))
      mcodLen = int((uint32(disk[dirOff + 12]) shl 24) or (uint32(disk[dirOff + 13]) shl 16) or
                    (uint32(disk[dirOff + 14]) shl 8) or uint32(disk[dirOff + 15]))
      break
  if mcodOff < 0:
    echo "  ✗ MCOD not found"; quit(1)
  echo "  MCOD @", mcodOff, " len=", mcodLen

  template mc(o: int): uint8 = uint8(disk[mcodOff + o])
  let mcodVersion = (uint16(mc(0)) shl 8) or uint16(mc(1))
  let mcodNum = (uint16(mc(2)) shl 8) or uint16(mc(3))
  let strPoolAbs = mcodOff + int((uint32(mc(4)) shl 24) or (uint32(mc(5)) shl 16) or
                                 (uint32(mc(6)) shl 8) or uint32(mc(7)))
  let codePoolAbs= mcodOff + int((uint32(mc(8)) shl 24) or (uint32(mc(9)) shl 16) or
                                 (uint32(mc(10)) shl 8) or uint32(mc(11)))
  echo "  version=", mcodVersion, " entries=", mcodNum

  # Build lookup table for re-encode comparison
  var parseOk = 0
  var rvBack = 0
  var a64Back = 0
  for i in 0 ..< int(mcodNum):
    let eOff = mcodOff + 12 + i * 16
    let cp   = (uint32(disk[eOff]) shl 24) or (uint32(disk[eOff+1]) shl 16) or
               (uint32(disk[eOff+2]) shl 8) or uint32(disk[eOff+3])
    let isaFlags = (uint32(disk[eOff+4]) shl 24) or (uint32(disk[eOff+5]) shl 16) or
                   (uint32(disk[eOff+6]) shl 8) or uint32(disk[eOff+7])
    let sOff = (uint16(disk[eOff+8]) shl 8) or uint16(disk[eOff+9])
    let sLen = (uint16(disk[eOff+10]) shl 8) or uint16(disk[eOff+11])
    let cOff = (uint16(disk[eOff+12]) shl 8) or uint16(disk[eOff+13])
    let cLen = (uint16(disk[eOff+14]) shl 8) or uint16(disk[eOff+15])
    var sId = ""
    for j in 0 ..< int(sLen): sId.add(char(disk[strPoolAbs + int(sOff) + j]))
    var codeBytes = newSeq[uint8]()
    for j in 0 ..< int(cLen): codeBytes.add(uint8(disk[codePoolAbs + int(cOff) + j]))
    var isaName = if isaFlags == 2: "ARM64" else: "RV32I"

    # re-encode and compare
    var found = false
    if isaFlags == ISA_RV32I:
      # try to find matching RV32I case
      for c in RV32I_CASES:
        if c.semId == sId:
          let re = encodeRV32I(c.semId, c.op)
          if re.encoded.code == codeBytes:
            inc parseOk; inc rvBack; found = true
          break
    elif isaFlags == ISA_ARM64:
      for c in ARM64_CASES:
        if c.semId == sId:
          let re = encodeARM64(c.semId, c.op)
          if re.encoded.code == codeBytes:
            inc parseOk; inc a64Back; found = true
          break

    let codeHex = block:
      var s = ""
      for by in codeBytes: s.add(toHex(int(by),2))
      s.toUpperAscii()
    if found:
      echo "  ✓ U+", toHex(cp,4).toUpperAscii(), " [", isaName.alignLeft(6), "] ", sId.alignLeft(7), " 0x", codeHex
    else:
      echo "  ✗ U+", toHex(cp,4).toUpperAscii(), " [", isaName.alignLeft(6), "] ", sId.alignLeft(7), " MISMATCH"

  echo ""
  echo "═══════════════════════════════════════════════════"
  echo "  双 ISA 汇总: ", parseOk, "/", mcodNum, " 往返恒等"
  echo "    RV32I : ", rvBack, " / ", RV32I_CASES.len
  echo "    ARM64 : ", a64Back, " / ", ARM64_CASES.len
  if parseOk == int(mcodNum):
    echo "  ✓ 全部通过：双 ISA TTF 写盘 ↔ 读盘 ↔ re-encode 恒等"
  echo "═══════════════════════════════════════════════════"
  echo ""
