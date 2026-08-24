# ============================================================
# 64进制编码转换器
#
# 用自定义的64个符号，把任意文本编码/解码。
# 不是系统的base64，是你自己的64进制。
# ============================================================

import tables
import sequtils
import unicode

# 自定义64进制符号表（不是标准base64）
# 每个符号都是精心选择的，有独立的含义
const PHASE_64_ALPHABET* = [
  "一", "二", "三", "四", "五", "六", "七", "八",  # 1-8
  "人", "入", "大", "天", "太", "夫", "央", "失",  # 9-16
  "白", "百", "的", "目", "自", "且", "具", "真",  # 17-24
  "中", "申", "由", "甲", "电", "曰", "曲", "曳",  # 25-32
  "我", "你", "他", "她", "它", "们", "这", "那",  # 33-40
  "加", "减", "乘", "除", "等", "不", "非", "是",  # 41-48
  "袋", "米", "容", "器", "编", "解", "编", "码",  # 49-56
  "全", "满", "空", "有", "无", "生", "灭", "道"   # 57-64
]

# 反向查找表
func build_reverse_table(): Table[string, int] =
  var t = init_table[string, int]()
  for i, ch in PHASE_64_ALPHABET:
    t[ch] = i
  return t

let REVERSE_TABLE = build_reverse_table()

# 获取64进制字符
func phase_char*(idx: int): string =
  if idx >= 0 and idx < 64:
    return PHASE_64_ALPHABET[idx]
  return "?"

# 获取字符的索引
proc phase_index*(ch: string): int =
  if REVERSE_TABLE.hasKey(ch):
    return REVERSE_TABLE[ch]
  return -1

# 编码：字符串 → 64进制串
proc encode*(text: string): string =
  var result = ""
  for ch in text:
    let code = int(ch)
    # 每个字符变成两个64进制符号（因为 64*64=4096 > 256）
    let hi = code div 64
    let lo = code mod 64
    result &= PHASE_64_ALPHABET[hi]
    result &= PHASE_64_ALPHABET[lo]
  return result

# 解码：64进制串 → 字符串
proc decode*(encoded: string): string =
  var result = ""
  let chars = to_seq(encoded.runes())
  
  var i = 0
  while i + 1 < chars.len:
    let hi = phase_index($chars[i])
    let lo = phase_index($chars[i + 1])
    if hi >= 0 and lo >= 0:
      let code = hi * 64 + lo
      result &= chr(code)
    else:
      result &= "?"
    i += 2
  
  return result

# 编码（UTF-8版本，支持中文）
# 每个字符用可变长64进制表示：1-4个符号
# 第一个符号的高2位表示长度，低4位编码数值低位
# 后续符号各用6位
# 范围：1符号=4位(0..15), 2符号=10位(0..1023), 3符号=16位(0..65535), 4符号=22位(覆盖Unicode)
proc encode_utf8*(text: string): string =
  var result = ""
  let runes = to_seq(text.runes())
  
  for r in runes:
    let code = int(r)
    
    # 判断需要几个符号
    var count: int
    if code < 16: count = 1
    elif code < 1024: count = 2      # 16 * 64
    elif code < 65536: count = 3    # 16 * 64²
    else: count = 4                 # 16 * 64³ = 2²² > Unicode max
    
    let length_marker = (count - 1) * 16  # 0, 16, 32, 48
    var n = code
    
    # 第一个符号：低4位 = n mod 16
    let first_data = n mod 16
    result &= PHASE_64_ALPHABET[length_marker + first_data]
    n = n div 16
    
    # 后续符号：各6位
    for i in 1..<count:
      result &= PHASE_64_ALPHABET[n mod 64]
      n = n div 64
  
  return result

# 解码（UTF-8版本）
proc decode_utf8*(encoded: string): string =
  var chars: seq[Rune] = @[]
  let encoded_chars = to_seq(encoded.runes())
  
  var i = 0
  while i < encoded_chars.len:
    let idx0 = phase_index($encoded_chars[i])
    if idx0 < 0:
      break
    
    # 从第一个符号解码长度
    let length_marker = idx0 div 16  # 0, 1, 2, 3
    let count = length_marker + 1
    
    var code = idx0 mod 16  # 第一个符号的低4位（bits 0..3）
    var multiplier = 16       # 后续符号的权重：16, 1024, 65536
    
    # 读取后续符号
    for j in 1..<count:
      if i + j < encoded_chars.len:
        let idx = phase_index($encoded_chars[i + j])
        if idx >= 0:
          code = code + idx * multiplier
          multiplier = multiplier * 64
    
    chars.add(Rune(code))
    i += count
  
  return $chars

# 进制转换工具
proc to_phase64*(n: int): string =
  if n == 0:
    return PHASE_64_ALPHABET[0]
  var result = ""
  var num = n
  while num > 0:
    result = PHASE_64_ALPHABET[num mod 64] & result
    num = num div 64
  return result

proc from_phase64*(s: string): int =
  var result = 0
  let chars = to_seq(s.runes())
  for ch in chars:
    let idx = phase_index($ch)
    if idx >= 0:
      result = result * 64 + idx
  return result

# 验证编码正确性
proc verify*(text: string): bool =
  let encoded = encode(text)
  let decoded = decode(encoded)
  return text == decoded

# 验证UTF-8编码
proc verify_utf8*(text: string): bool =
  let encoded = encode_utf8(text)
  let decoded = decode_utf8(encoded)
  return text == decoded

# 编码报告
proc encode_report*(text: string): string =
  var s = "═══ 64进制编码报告 ═══\n"
  s &= "原文: " & text & "\n"
  s &= "UTF-8编码: " & encode_utf8(text) & "\n"
  s &= "解码验证: " & (if verify_utf8(text): "✅ 通过" else: "❌ 失败") & "\n"
  s &= "原文长度: " & $text.len & " 字符\n"
  s &= "编码长度: " & $len(to_seq(encode_utf8(text).runes())) & " 符号\n"
  result = s
