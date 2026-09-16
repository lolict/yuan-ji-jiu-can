# @name: stego_nim
# @type: steganography-core
# @owner: 五彩石
# @status: stable
# @priority: P1
# @description: 五彩石隐写核心 - Nim实现 (LSB/Unicode/HTML)

import std/[strutils, streams, base64, uri, json, tables]
import std/sequtils

type
  StegoMode* = enum
    smLSB, smUnicode, smHTML

# === Unicode 隐写：零宽字符编码 ===

const
  ZWSP = '\u200B'   # bit 0
  ZWNJ = '\u200C'   # bit 1
  ZWJ  = '\u200D'   # 分隔符
  BOM  = '\uFEFF'   # 结束符

proc unicodeEncode*(carrier: string, payload: string): StringSlice =
  ## 将 payload 以零宽字符编码嵌入 carrier
  var bits: seq[uint8] = @[]
  
  # 4字节长度头（大端序）
  let plen = payload.len
  bits.add uint8((plen shr 24) and 0xFF)
  bits.add uint8((plen shr 16) and 0xFF)
  bits.add uint8((plen shr 8) and 0xFF)
  bits.add uint8(plen and 0xFF)
  
  # payload 字节流
  for c in payload:
    bits.add uint8(c)
  
  # 转零宽字符序列
  var result = newStringOfCap(carrier.len + bits.len * 2 + 1)
  result.add(carrier)
  
  for byte in bits:
    for bitPos in countdown(7, 0):
      let bit = (byte shr bitPos) and 0x01
      if bit == 0:
        result.add(ZWSP)
      else:
        result.add(ZWNJ)
      result.add(ZWJ)
  result.add(BOM)
  return result

proc unicodeDecode*(data: string): tuple[ok: bool, payload: string] =
  ## 从 carrier 文本中解码零宽字符
  var zwChars: seq[char] = @[]
  for ch in data:
    if ch == BOM: break
    if ch in [ZWSP, ZWNJ, ZWJ]:
      zwChars.add(ch)
  
  # 提取 bits
  var bits: seq[uint8] = @[@0, @0, @0, @0]
  var bitIdx = 0
  var byteVal: uint8 = 0
  var bitCount = 0
  
  var extracted: seq[uint8] = @[]
  var i = 0
  while i < zwChars.high:
    if i + 1 <= zwChars.high and zwChars[i + 1] == ZWJ:
      let bit = if zwChars[i] == ZWSP: 0u8 else: 1u8
      byteVal = (byteVal shl 1) or bit
      inc bitCount
      if bitCount == 8:
        extracted.add(byteVal)
        byteVal = 0
        bitCount = 0
      inc i, 2
    else:
      inc i
  
  if extracted.len < 4:
    return (false, "")
  
  let plen = (extracted[0].int shl 24) or (extracted[1].int shl 16) or
               (extracted[2].int shl 8) or extracted[3].int
  
  if plen <= 0 or plen > extracted.len - 4:
    return (false, "")
  
  var payload = newStringOfCap(plen)
  for j in 0 ..< plen:
    payload.add char(extracted[4 + j])
  
  return (true, payload)


# === HTML 隐写 ===

proc htmlEncode*(htmlContent: string, payload: string): StringSlice =
  ## 在 HTML 不可见位置嵌入 base64 编码的 payload
  let encoded = encode(payload)  # std/base64
  let comment = "<!-- csl3:" & encoded & " -->"
  let hiddenSpan = "<span style=\"display:none;font-size:0;opacity:0\">" &
                   "<!--csl3-->" & encoded & "<!--/csl3--></span>"
  
  let headClose = htmlContent.find("</head>")
  if headClose >= 0:
    let (pre, post) = (htmlContent[0 ..< headClose], htmlContent[headClose .. ^1])
    return pre & "\n" & comment & "\n" & hiddenSpan & "\n" & post
  return htmlContent & "\n" & comment & "\n" & hiddenSpan & "\n"

proc htmlDecode*(htmlContent: string): tuple[ok: bool, payload: string] =
  ## 从 HTML 中解码 payload
  let patC = "<!-- csl3:"
  let patC2 = " -->"
  let startC = htmlContent.find(patC)
  if startC >= 0:
    let innerStart = startC + patC.len
    let innerEnd = htmlContent.find(patC2, innerStart)
    if innerEnd > innerStart:
      let encoded = htmlContent[innerStart ..< innerEnd]
      try:
        return (true, decode(encoded))
      except:
        discard
  
  let patS = "<!--csl3-->"
  let patS2 = "<!--/csl3-->"
  let startS = htmlContent.find(patS)
  if startS >= 0:
    let innerStart = startS + patS.len
    let innerEnd = htmlContent.find(patS2, innerStart)
    if innerEnd > innerStart:
      let encoded = htmlContent[innerStart ..< innerEnd]
      try:
        return (true, decode(encoded))
      except:
        discard
  
  return (false, "")


# === LSB PNG 隐写 ===

proc lsbEncode*(imagePath: string, payload: string, outputPath: string): bool =
  ## 将 payload 写入 PNG 图像最低有效位
  let f = open(imagePath, fmRead)
  defer: f.close()
  
  let pngBytes = newString(f.readAllLen)
  if pngBytes.len < 32:
    return false
  
  # 简单的 PPM/PAM 或原始 RGB 处理（此处为简化演示）
  # 实际应用需要 PNG 解码库
  
  # 头部格式：4字节长度 + payload + 4字节 CRC32
  var data = newStringOfCap(4 + payload.len + 4)
  let plen = payload.len
  data.add char((plen shr 24) and 0xFF)
  data.add char((plen shr 16) and 0xFF)
  data.add char((plen shr 8) and 0xFF)
  data.add char(plen and 0xFF)
  data.add(payload)
  
  # 计算 CRC32
  var crc: uint32 = 0xFFFFFFFF
  for c in payload:
    crc = crc xor uint32(c)
    for _ in 0 ..< 8:
      if (crc and 1) != 0:
        crc = (crc shr 1) xor 0xEDB88320'u32
      else:
        crc = crc shr 1
  crc = crc xor 0xFFFFFFFF'u32
  data.add char((crc shr 24) and 0xFF)
  data.add char((crc shr 16) and 0xFF)
  data.add char((crc shr 8) and 0xFF)
  data.add char(crc and 0xFF)
  
  # LSB 写入（简化版 - 实际应用需要完整 PNG 解析）
  var imgData = pngBytes
  if data.len * 8 > imgData.len:
    return false
  
  for i, b in data:
    for bitPos in countdown(7, 0):
      let bit = (uint8(b) shr uint8(bitPos)) and 0x01
      let pixIdx = i * 8 + (7 - bitPos)
      if pixIdx < imgData.len:
        imgData[pixIdx] = char((uint8(imgData[pixIdx]) and 0xFE) or bit)
  
  let outF = open(outputPath, fmWrite)
  defer: outF.close()
  outF.write(imgData)


# === CLI 入口 ===

when isMainModule:
  import std/os
  
  if paramCount() < 2:
    echo "stego_nim <encode|decode> <mode> <input> [output] [-p payload]"
    echo "  mode = unicode | html | lsb"
    quit(1)
  
  let action = paramStr(1)
  let mode = paramStr(2)
  
  if action == "encode":
    if paramCount() < 4:
      echo "需要 input output [-p payload]"
      quit(1)
    
    let input = paramStr(3)
    let output = paramStr(4)
    var payload = ""
    if paramCount() >= 6 and paramStr(5) == "-p":
      payload = paramStr(6)
    
    if mode == "unicode":
      let cin = if fileExists(input): readFile(input) else: input
      let result = unicodeEncode(cin, payload)
      writeFile(output, $result)
      echo "已编码 ", payload.len, " 字符到 ", output
    elif mode == "html":
      let cin = if fileExists(input): readFile(input) else: input
      let result = htmlEncode(cin, payload)
      writeFile(output, $result)
      echo "已编码 ", payload.len, " 字符到 ", output
    elif mode == "lsb":
      let ok = lsbEncode(input, payload, output)
      if ok:
        echo "已编码 ", payload.len, " 字节到图像"
      else:
        echo "编码失败"
        quit(1)
  
  elif action == "decode":
    if paramCount() < 3:
      echo "需要 input"
      quit(1)
    
    let input = paramStr(3)
    
    if mode == "unicode":
      let content = readFile(input)
      let (ok, payload) = unicodeDecode(content)
      if ok:
        stdout.write(payload)
      else:
        echo "解码失败"
        quit(1)
    elif mode == "html":
      let content = readFile(input)
      let (ok, payload) = htmlDecode(content)
      if ok:
        stdout.write(payload)
      else:
        echo "解码失败"
        quit(1)
  
  else:
    echo "未知操作: ", action
    quit(1)
