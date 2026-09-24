## ms_font — 最小化 OpenType TTF 写入器（纯 Nim，零依赖）
##
## 仅覆盖最小必需表 + MCOD 自定义表。不实现 GSUB/GPOS/kerning/可变字体等高级特性。
## 目标：生成一个能被操作系统字体引擎加载、能作为数据载体嵌入语义ID↔机器码映射的文件。
##
## OpenType 规范（MS Typography）：
##   必需表：cmap / glyf / head / hhea / hmtx / loca / maxp / name / post / OS/2
##           + MCOD（自定义，table tag = 'M','C','O','D'）
##
## 所有多字节整数字段大端序（网络字节序）。
## 每个表起始相对文件偏移 4 字节对齐。

import std/[algorithm, strutils, tables]

# ═══════════════════════════════════════════════════
#  类型
# ═══════════════════════════════════════════════════

type
  GlyphId* = uint16

  HmtxEntry* = object
    advanceWidth*: uint16
    lsb*: int16            ## left side bearing

  McodEntry* = object
    codepoint*: uint32     ## PUA 码点
    semId*: string         ## 跨 ISA 语义 ID（例 "ADD"）
    isaCode*: seq[uint8]   ## 该 ISA 对应的机器码
    isaId*: int            ## 指令集标识（用于 isaFlags 字段；1=RV32I, 2=ARM64, 4=x86-64, 8=MIPS）

  FontConfig* = object
    familyName*: string
    styleName*: string
    psName*: string
    version*: string
    manufacturer*: string
    unitsPerEm*: int16     ## 方形 em 单位
    ascender*: int16
    descender*: int16      ## 通常为负

const
  DEFAULT_PUA_BASE* = 0xF000'u32    ## 私用区起始码点（RV32I 段）
  PUA_BASE_ARM64*   = 0xF100'u32    ## PUA ARM64 段起始
  ISA_RV32I*  = 1
  ISA_ARM64*  = 2
  ISA_X64*    = 4
  ISA_MIPS*   = 8
  MCOD_TAG* = [uint8('M'), uint8('C'), uint8('O'), uint8('D')]
  REQUIRED_TABLES* = [
    "cmap", "glyf", "head", "hhea", "hmtx", "loca",
    "maxp", "name", "post", "OS/2"
  ]

# ═══════════════════════════════════════════════════
#  原始字节缓冲（大端写入小工具）
# ═══════════════════════════════════════════════════

type
  Buf* = object
    data*: seq[uint8]

proc newBuf*(): Buf = Buf(data: @[])

proc addU8(b: var Buf; v: uint8) = b.data.add(v)

proc addBE16(b: var Buf; v: uint16) =
  b.data.add(uint8((v shr 8) and 0xFF))
  b.data.add(uint8(v and 0xFF))

proc addBE16Signed(b: var Buf; v: int16) =
  let u = cast[uint16](v)
  addBE16(b, u)

proc addBE32(b: var Buf; v: uint32) =
  b.data.add(uint8((v shr 24) and 0xFF))
  b.data.add(uint8((v shr 16) and 0xFF))
  b.data.add(uint8((v shr 8) and 0xFF))
  b.data.add(uint8(v and 0xFF))

proc addBE32Signed(b: var Buf; v: int32) =
  let u = cast[uint32](v)
  addBE32(b, u)

proc addTag(b: var Buf; s: string) =
  ## 4 ASCII 字节 tag，不足 4 字符右补空格
  var t = s
  while t.len < 4: t.add(' ')
  for c in t[0 .. 3]:
    b.data.add(uint8(c))

proc addBytes(b: var Buf; v: seq[uint8]) =
  b.data.add(v)

proc addString16(b: var Buf; s: string) =
  ## UTF-16BE 字符串（用于 name table）
  for c in s:
    addBE16(b, uint16(ord(c)))

proc pad4(b: var Buf) =
  ## 当前对齐到 4 字节边界
  while (b.data.len mod 4) != 0:
    b.data.add(0'u8)

proc len*(b: Buf): int = b.data.len

# ═══════════════════════════════════════════════════
#  字形：简单矩形，用 16-bit signed delta（避开 short-flag 陷阱）
# ═══════════════════════════════════════════════════

const
  ON_CURVE_B* = 0x01'u8     ## 字节常量，避免 int 推断

proc rectangleGlyph*(advance: uint16; height: uint16): seq[uint8] =
  ## 简单矩形 TrueType 字形：4 角点（逆时针）。
  ## 所有点用 16-bit signed delta（flag = ON_CURVE only）。
  var b = newBuf()
  addBE16(b, 1'u16)                              ## numberOfContours = 1
  addBE16Signed(b, 0)                            ## xMin
  addBE16Signed(b, 0)                            ## yMin
  addBE16Signed(b, cast[int16](advance))         ## xMax
  addBE16Signed(b, cast[int16](height))          ## yMax
  addBE16(b, 3'u16)                              ## endPtsOfContours = [3]
  addBE16(b, 0'u16)                              ## instructionLength = 0
  ## 4 on-curve 点 flag
  b.data.add([ON_CURVE_B, ON_CURVE_B, ON_CURVE_B, ON_CURVE_B])
  ## 4 point deltas (x,y) as int16 BE pairs:
  ## p0: (0,0)
  addBE16Signed(b, 0); addBE16Signed(b, 0)
  ## p1: (+advance, 0)
  addBE16Signed(b, cast[int16](advance)); addBE16Signed(b, 0)
  ## p2: (0, +height)
  addBE16Signed(b, 0); addBE16Signed(b, cast[int16](height))
  ## p3: (-advance, 0)
  addBE16Signed(b, -cast[int16](advance)); addBE16Signed(b, 0)
  return b.data

# ═══════════════════════════════════════════════════
#  表构造器
# ═══════════════════════════════════════════════════

# ─── head ───────────────────────────────────────────

proc tableHead*(unitsPerEm: int16; xMin, yMin, xMax, yMax: int16;
                indexToLocFormat: int16 = 0): seq[uint8] =
  var b = newBuf()
  addBE32(b, 0x00010000'u32)              ## version 1.0
  addBE32(b, 0x00010000'u32)              ## fontRevision 1.0
  addBE32(b, 0x00000000'u32)              ## checksumAdjustment (filled later)
  addBE32(b, 0x5F0F3CF5'u32)              ## magicNumber
  addBE16(b, 0x000B'u16)                  ## flags
  addBE16(b, uint16(unitsPerEm))          ## unitsPerEm
  b.data.add(newSeq[uint8](8))            ## created (8 bytes zero)
  b.data.add(newSeq[uint8](8))            ## modified
  addBE16Signed(b, xMin); addBE16Signed(b, yMin)
  addBE16Signed(b, xMax); addBE16Signed(b, yMax)
  addBE16(b, 0'u16)                       ## macStyle
  addBE16(b, 8'u16)                       ## lowestRecPPEM
  addBE16(b, 2'u16)                       ## fontDirectionHint
  addBE16Signed(b, indexToLocFormat)      ## indexToLocFormat
  addBE16(b, 0'u16)                       ## glyphDataFormat
  return b.data

# ─── hhea ───────────────────────────────────────────

proc tableHhea*(ascender, descender, lineGap: int16;
                advanceWidthMax: uint16; minLSB, minRSB, maxExt: int16;
                numOfLongHorMetrics: uint16): seq[uint8] =
  var b = newBuf()
  addBE32(b, 0x00010000'u32)              ## version
  addBE16Signed(b, ascender)
  addBE16Signed(b, descender)
  addBE16Signed(b, lineGap)
  addBE16(b, advanceWidthMax)
  addBE16Signed(b, minLSB)
  addBE16Signed(b, minRSB)
  addBE16Signed(b, maxExt)
  addBE16Signed(b, 1); addBE16Signed(b, 0)   ## caretSlopeRise / caretSlopeRun
  addBE16Signed(b, 0); addBE16Signed(b, 0)   ## caretOffset
  addBE16Signed(b, 0); addBE16Signed(b, 0)   ## reserved x4
  addBE16(b, 0'u16)                       ## metricDataFormat
  addBE16(b, numOfLongHorMetrics)
  return b.data

# ─── maxp ───────────────────────────────────────────

proc tableMaxp*(numGlyphs: uint16): seq[uint8] =
  var b = newBuf()
  addBE32(b, 0x00010000'u32)              ## version 1.0
  addBE16(b, numGlyphs)
  ## Fixed fields below (TrueType maxp only requires numGlyphs for format 0.5)
  ## For full 1.0 these are needed; we zero-fill minimally-functional:
  ## Not used by OpenType implementations for our simple glyphs.
  ## Just set basic values:
  addBE16(b, numGlyphs)                   ## maxPoints
  addBE16(b, 1'u16)                       ## maxContours
  addBE16(b, 0'u16)                       ## maxCompositePoints
  addBE16(b, 1'u16)                       ## maxCompositeContours
  addBE16(b, 1'u16)                       ## maxZones
  addBE16(b, 0'u16)                       ## maxTwilightPoints
  addBE16(b, 0'u16)                       ## maxStorage
  addBE16(b, 0'u16)                       ## maxFunctionDefs
  addBE16(b, 0'u16)                       ## maxInstructionDefs
  addBE16(b, 0'u16)                       ## maxStackElements
  addBE16(b, 0'u16)                       ## maxSizeOfInstructions
  addBE16(b, 0'u16)                       ## maxComponentElements
  addBE16(b, 0'u16)                       ## maxComponentDepth
  return b.data

# ─── hmtx ───────────────────────────────────────────

proc tableHmtx*(entries: seq[HmtxEntry]): seq[uint8] =
  var b = newBuf()
  for e in entries:
    addBE16(b, e.advanceWidth)
    addBE16Signed(b, e.lsb)
  return b.data

# ─── loca (short format) ────────────────────────────

proc tableLocaShort*(numGlyphsPlus1: int): seq[uint8] =
  ## Short format: offsets are advance widths accumulated, values are actual
  ## byte offsets / 2. We compute at finalize time, here we return placeholder.
  newSeq[uint8](numGlyphsPlus1 * 2)

# ─── cmap ───────────────────────────────────────────

proc tableCmap*(pairs: seq[tuple[cp: uint32; gid: uint16]]): seq[uint8] =
  ## Minimal cmap: one format 4 subtable (UCS-2 BMP subset + format 12 for full Unicode)
  ## We use format 4 with one segment for simplicity.
  ## Format 4 encoding:
  ##   version = 0
  ##   [subtableCount = 1]
  ##   [each subtable: platformID=3, encodingID=1, offset]
  ##   format 4 header: format=4, length, language=0
  ##   segCountX2, searchRange, entrySelector, rangeShift
  ##   endCode[segCount], reservedPad, startCode[segCount]
  ##   idDelta[segCount], idRangeOffset[segCount]
  ##
  ## For N pairs each with gid=i (simple monotonic), we use 1 segment per pair
  ## or we sort and merge into sequential ranges.

  var sortedPairs = sorted(pairs, proc(a, b: tuple[cp: uint32; gid: uint16]): int =
    if a.cp == b.cp: return 0
    if a.cp < b.cp: return -1
    return 1
  )

  # Build segments: each pair → a single simple segment
  let segCount = uint16(sortedPairs.len)
  var b = newBuf()
  addBE16(b, 0'u16)                         ## version
  addBE16(b, 1'u16)                         ## numberSubtables
  # Subtable record 1: platformID=3 (Microsoft), encodingID=1 (Unicode BMP)
  addBE16(b, 3'u16); addBE16(b, 1'u16)
  addBE32(b, 0'u32)                         ## offset placeholder — filled later
  var subtableOffset = b.len

  # subtable format 4 start
  addBE16(b, 4'u16)                         ## format
  # length placeholder
  var lengthOffset = b.len
  addBE16(b, 0'u16)
  addBE16(b, 0'u16)                         ## language
  # segCountX2
  let scX2 = segCount * 2
  addBE16(b, scX2)
  # searchRange, entrySelector, rangeShift
  var searchRange = 1'u16
  var entrySelector = 0'u16
  while searchRange * 2 <= scX2:
    searchRange = searchRange * 2
    entrySelector += 1
  searchRange = searchRange * 2  # where 2*searchRange > segCount {wrong formula}:
  # correct formula: searchRange = 2 * (2^entrySelector)
  searchRange = 1'u16
  entrySelector = 0'u16
  var t: uint16 = segCount
  while t > 1'u16:
    t = t shr 1
    searchRange = searchRange * 2
    entrySelector += 1
  searchRange = searchRange * 2
  let rangeShift = scX2 - searchRange
  addBE16(b, searchRange)
  addBE16(b, entrySelector)
  addBE16(b, rangeShift)
  # endCode[segCount]
  for p in sortedPairs:
    addBE16(b, cast[uint16](p.cp))        ## assume cp in BMP
  addBE16(b, 0'u16)                         ## reservedPad
  # startCode[segCount]
  for p in sortedPairs:
    addBE16(b, cast[uint16](p.cp))
  # idDelta[segCount]
  for p in sortedPairs:
    let delta = cast[int16](cast[int](p.gid) - cast[int](p.cp))
    addBE16Signed(b, delta)
  # idRangeOffset[segCount]
  for _ in sortedPairs:
    addBE16(b, 0'u16)
  # Patch length
  let subLen = uint16(b.len - subtableOffset)
  b.data[lengthOffset]     = uint8((subLen shr 8) and 0xFF)
  b.data[lengthOffset + 1] = uint8(subLen and 0xFF)
  # Patch subtable record offset to point to subtable (relative to cmap start)
  b.data[subtableOffset - 4] = uint8((subtableOffset shr 24) and 0xFF)
  b.data[subtableOffset - 3] = uint8((subtableOffset shr 16) and 0xFF)
  b.data[subtableOffset - 2] = uint8((subtableOffset shr 8) and 0xFF)
  b.data[subtableOffset - 1] = uint8(subtableOffset and 0xFF)
  return b.data

# ─── post ───────────────────────────────────────────

proc tablePost*(): seq[uint8] =
  var b = newBuf()
  addBE32(b, 0x00030000'u32)              ## format 3.0 (no glyph names)
  addBE32Signed(b, 0)                 ## italicAngle 0
  addBE16Signed(b, -100'i16)              ## underlinePosition
  addBE16Signed(b, 50'i16)                ## underlineThickness
  addBE32(b, 0'u32)                       ## isFixedPitch
  addBE32(b, 0'u32); addBE32(b, 0'u32)    ## minMemType42 / maxMemType42
  addBE32(b, 0'u32); addBE32(b, 0'u32)    ## minMemType1 / maxMemType1
  return b.data

# ─── OS/2 ───────────────────────────────────────────

proc tableOS2*(ascender, descender: int16; advanceWidthMax: uint16): seq[uint8] =
  var b = newBuf()
  addBE16(b, 0x0004'u16)                  ## version 4
  addBE16Signed(b, cast[int16](int(advanceWidthMax)))  ## xAvgCharWidth
  addBE16(b, 400'u16)                     ## usWeightClass (Regular)
  addBE16(b, 5'u16)                       ## usWidthClass
  addBE16(b, 0'u16)                       ## fsType
  addBE16Signed(b, 0); addBE16Signed(b, 0)  ## ySubscriptX/YSize
  addBE16Signed(b, 0); addBE16Signed(b, 0)  ## ySuperscriptX/YSize
  addBE16Signed(b, 0); addBE16Signed(b, 0)  ## yStrikeoutSize/Position
  addBE16Signed(b, 0);               ## sFamilyClass
  ## panose (10 bytes)
  b.data.add(newSeq[uint8](10))
  ## ulUnicodeRange (4x uint32)
  addBE32(b, 0'u32); addBE32(b, 0'u32); addBE32(b, 0'u32); addBE32(b, 0'u32)
  ## achVendID (4 chars ASCII)
  b.data.add([0'u8, 0'u8, 0'u8, 0'u8])
  addBE16(b, 0x0040'u16)                  ## fsSelection (REGULAR)
  addBE16(b, 0x0020'u16); addBE16(b, 0x007E'u16)    ## usFirstCharIndex / usLastCharIndex
  addBE16Signed(b, ascender)
  addBE16Signed(b, descender)
  addBE16Signed(b, 0);                ## sTypoLineGap
  addBE16Signed(b, ascender);             ## usWinAscent
  addBE16Signed(b, cast[int16](-descender.int16.int)) ## usWinDescent
  ## ulCodePageRange
  addBE32(b, 0'u32); addBE32(b, 0'u32)
  addBE16Signed(b, 0); addBE16Signed(b, 0) ## sxHeight / sCapHeight
  addBE16(b, 0'u16); addBE16(b, 0'u16)    ## usDefaultChar / usBreakChar
  addBE16(b, 0'u16)                       ## usMaxContext
  return b.data

# ─── name ───────────────────────────────────────────

type
  NameRecord* = tuple
    platformID: uint16
    encodingID: uint16
    languageID: uint16
    nameID: uint16
    offset: uint16        # set by table builder
    length: uint16        # set by table builder
    value: string

proc tableName*(records: seq[NameRecord]): seq[uint8] =
  var b = newBuf()
  # Compute string storage size each as UTF-16
  var strings: seq[tuple[off: uint16; len: uint16; utf16: seq[uint8]]]
  var curOff = 0'u16
  for r in records:
    var utf16: seq[uint8]
    for c in r.value:
      utf16.add(uint8((ord(c) shr 8) and 0xFF))
      utf16.add(uint8(ord(c) and 0xFF))
    let len = uint16(utf16.len)
    strings.add((curOff, len, utf16))
    curOff += len
  let stringStorageOff = uint16(6 + records.len * 12)
  addBE16(b, 0'u16)                         ## format
  addBE16(b, uint16(records.len))           ## count
  addBE16(b, stringStorageOff)              ## stringStorage offset
  for i, r in records:
    addBE16(b, r.platformID); addBE16(b, r.encodingID)
    addBE16(b, r.languageID); addBE16(b, r.nameID)
    addBE16(b, strings[i].len)
    addBE16(b, strings[i].off + stringStorageOff)
  # string storage
  for s in strings:
    addBytes(b, s.utf16)
  return b.data

# ─── glyf + loca 打包 ────────────────────────────────

proc packGlyfLoca*(glyphs: seq[seq[uint8]]; indexToLocFormat: int16 = 0): tuple[glyf: seq[uint8]; loca: seq[uint8]] =
  ## 生成 glyf 表和 loca 表。
  ## glyf: 所有 glyphs 拼接，pad 到每个 glyph 起始于 4 字节对齐。
  ## loca short format: 每个值 = glyph 绝对字节偏移 / 2。
  var glyf = newSeq[uint8]()
  var offsets: seq[uint32] = @[]
  for g in glyphs:
    # align to 4 within glyf
    while (glyf.len mod 4) != 0:
      glyf.add(0'u8)
    offsets.add(uint32(glyf.len))
    glyf.add(g)
  # trailing last offset = total glyph data end
  offsets.add(uint32(glyf.len))
  result.glyf = glyf
  # build loca (short)
  var b = newBuf()
  for o in offsets:
    addBE16(b, uint16(o div 2))
  result.loca = b.data

# ─── MCOD 自定义表 ───────────────────────────────────

proc tableMCOD*(entries: seq[McodEntry]): seq[uint8] =
  ## MCOD 表结构：
  ## uint16 version = 1
  ## uint16 numEntries
  ## uint32 stringPoolOffset   (relative to table start)
  ## uint32 codePoolOffset
  ## entries[numEntries]:
  ##   uint32 codepoint
  ##   uint32 isaFlags
  ##   uint16 semIdOffset
  ##   uint16 semIdLength
  ##   uint16 codeOffset
  ##   uint16 codeLength
  ## stringPool: 各 semId 为 UTF-8 紧排 (semIdOffset + semIdLength 定位)
  ## codePool:   各 isaCode 原始字节流 (codeOffset + codeLength 定位)

  var semOffsets: seq[uint16]
  var semLengths: seq[uint16]
  var codeOffsets: seq[uint16]
  var codeLengths: seq[uint16]
  var semPool = newBuf()
  var codePool = newBuf()
  # reserved fixed header size:
  # 2 (version) + 2 (numEntries) + 4 (strOffset) + 4 (codeOffset)
  # + numEntries * (4 + 4 + 2 + 2 + 2 + 2) = 16 bytes per entry
  let headerSize = uint32(2 + 2 + 4 + 4) + uint32(entries.len) * 16
  var strPoolOff = headerSize   # placeholder — computed after entry count known
  var codePoolOff: uint32 = 0   # computed after str pool built
  for e in entries:
    semOffsets.add(uint16(semPool.len))
    semLengths.add(uint16(len(e.semId)))
    for c in e.semId: semPool.data.add(uint8(c))
    codeOffsets.add(uint16(codePool.len))
    codeLengths.add(uint16(e.isaCode.len))
    for c in e.isaCode: codePool.data.add(c)
  codePoolOff = headerSize + uint32(semPool.len)

  var b = newBuf()
  addBE16(b, 1'u16)                               ## version
  addBE16(b, uint16(entries.len))
  addBE32(b, strPoolOff)
  addBE32(b, codePoolOff)
  for i, e in entries:
    addBE32(b, e.codepoint)
    addBE32(b, uint32(e.isaId))                    ## isaFlags ← ISA 标识值
    addBE16(b, semOffsets[i])
    addBE16(b, semLengths[i])
    addBE16(b, codeOffsets[i])
    addBE16(b, codeLengths[i])
  assert(b.len == int(headerSize))
  addBytes(b, semPool.data)
  addBytes(b, codePool.data)
  return b.data

# ═══════════════════════════════════════════════════
#  顶层：拼装完整 .ttf
# ═══════════════════════════════════════════════════

type
  BuiltTable = tuple
    tag: array[4, uint8]
    data: seq[uint8]
    checksum: uint32

proc computeChecksum(data: seq[uint8]): uint32 =
  ## OpenType table checksum: uint32 sum of uint32-aligned data (pad missing with 0).
  var padded = @data
  while (padded.len mod 4) != 0:
    padded.add(0'u8)
  var acc: uint32 = 0
  var i = 0
  while i + 3 < padded.len:
    let chunk = (uint32(padded[i]) shl 24) or (uint32(padded[i+1]) shl 16) or
                (uint32(padded[i+2]) shl 8) or uint32(padded[i+3])
    acc += chunk
    i += 4
  result = acc and 0xFFFFFFFF'u32

proc finalizeChecksum(headData: var seq[uint8]; allTables: seq[BuiltTable]): seq[uint8] =
  ## Write full font file and fix head.checksumAdjustment.
  ## head.checksumAdjustment = 0xB1B0AFBA - checksum_of_entire_file
  # For now just return placeholder; the real fixup caller is `buildTTF`
  result = headData

proc alignPos4(off: int): int = ((off + 3) div 4) * 4

proc buildTTF*(cfg: FontConfig;
               glyfHeight: uint16 = 700;
               cmapPairs: seq[tuple[cp: uint32; gid: uint16]];
               nameRecords: seq[NameRecord];
               mcodEntries: seq[McodEntry] = @[]): seq[uint8] =
  ## 完整 .ttf 拼装：
  ##   • 1 .notdef glyph + N GID entries matching cmapPairs
  ##   • 可选 MCOD 自定义表（按 mcodEntries 生成）
  ## 返回文件原始字节。
  let nPairs = cmapPairs.len
  let numGlyphs = uint16(nPairs + 1)               # +1 for .notdef
  # ── glyphs ──
  var glyphData: seq[seq[uint8]] = newSeq[seq[uint8]](0)
  let dummyGlyph = rectangleGlyph(500'u16, glyfHeight)
  glyphData.add(dummyGlyph)
  for _ in 0'u16 ..< uint16(nPairs):
    glyphData.add(rectangleGlyph(500'u16, glyfHeight))
  let packed = packGlyfLoca(glyphData, 0)

  # ── hmtx ──
  var hmtxEntries: seq[HmtxEntry] = newSeq[HmtxEntry](0)
  hmtxEntries.add(HmtxEntry(advanceWidth: 500'u16, lsb: 0'i16))
  for _ in 1'u16 ..< numGlyphs:
    hmtxEntries.add(HmtxEntry(advanceWidth: 500'u16, lsb: 0'i16))
  let hmtxT = tableHmtx(hmtxEntries)

  # ── OS/2 / head / maxp ──
  let os2T = tableOS2(cfg.ascender, cfg.descender, 500'u16)
  let headT = tableHead(cfg.unitsPerEm, 0'i16, 0'i16, 500'i16, glyfHeight.int16, 0'i16)
  let maxpT = tableMaxp(numGlyphs)
  let hheaT = tableHhea(cfg.ascender, cfg.descender, 0'i16, 500'u16, 0'i16, 0'i16, 700'i16, numGlyphs)
  let postT = tablePost()
  let nameT = tableName(nameRecords)
  let cmapT = tableCmap(cmapPairs)
  let locaT = packed.loca
  let glyfT = packed.glyf
  var mcodT: seq[uint8] = @[]
  if mcodEntries.len > 0:
    mcodT = tableMCOD(mcodEntries)

  # ── list all tables (MCOD if present) ──
  var tags: seq[string] = @["cmap", "glyf", "head", "hhea", "hmtx", "loca",
                             "maxp", "name", "post", "OS/2"]
  if mcodEntries.len > 0: tags.add("MCOD")
  # alphabetical order required by spec
  tags.sort()

  var sizeOfTables = 12 + uint32(tags.len) * 16
  var tableMap: seq[tuple[name: string; data: seq[uint8]]] = @[
    ("cmap", cmapT), ("glyf", glyfT), ("head", headT), ("hhea", hheaT),
    ("hmtx", hmtxT), ("loca", locaT), ("maxp", maxpT), ("name", nameT),
    ("post", postT), ("OS/2", os2T)
  ]
  if mcodEntries.len > 0: tableMap.add(("MCOD", mcodT))
  tableMap.sort(proc(a, b: tuple[name: string; data: seq[uint8]]): int = cmp(a.name, b.name))

  # ── compute offsets ──
  var offsets: seq[uint32] = @[]
  var pos = uint32(12 + tags.len * 16)
  for t in tableMap:
    offsets.add(pos)
    pos += uint32(t.data.len)
    # align to 4
    if (pos mod 4) != 0:
      pos += 4 - (pos mod 4)
  # last offset = file end (for loca consistency not required here)

  # ── compute checksums ──
  var checksumMap = initTable[string, uint32]()
  for t in tableMap:
    checksumMap[t.name] = computeChecksum(t.data)

  # ── build offset table ──
  var b = newBuf()
  addBE32(b, 0x00010000'u32)                    ## sfVersion TrueType 1.0
  addBE16(b, uint16(tags.len))                  ## numTables
  # searchRange = 2^floor(log2(numTables)) * 16
  var searchRange = 1'u16
  var entrySelector = 0'u16
  while searchRange * 2 <= uint16(tags.len):
    searchRange *= 2
    entrySelector += 1
  searchRange *= 2
  let rangeShift = uint16(tags.len) * 16 - searchRange
  addBE16(b, searchRange)
  addBE16(b, entrySelector)
  addBE16(b, rangeShift)
  for i, t in tableMap:
    addTag(b, t.name)
    addBE32(b, checksumMap[t.name])
    addBE32(b, offsets[i])
    addBE32(b, uint32(t.data.len))
  for i, t in tableMap:
    # ensure 4-aligned
    while (b.len mod 4) != 0:
      b.data.add(0'u8)
    assert(b.len == int(offsets[i]))
    addBytes(b, t.data)

  # ── fix head.checksumAdjustment ──
  # checksum of entire file with checksumAdjustment = 0
  let fileChecksum = computeChecksum(b.data)
  let adjust = (0xB1B0AFBA'u32 - fileChecksum) and 0xFFFFFFFF'u32
  let headOff = offsets[tags.find("head")]
  let adjOffset = int(headOff) + 8   # head.checksumAdjustment at offset 8
  b.data[adjOffset]     = uint8((adjust shr 24) and 0xFF)
  b.data[adjOffset + 1] = uint8((adjust shr 16) and 0xFF)
  b.data[adjOffset + 2] = uint8((adjust shr 8) and 0xFF)
  b.data[adjOffset + 3] = uint8(adjust and 0xFF)
  return b.data
