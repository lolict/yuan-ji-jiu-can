## ═══════════════════════════════════════════════════════════════
## 满全法编码器 · 拼音 ↔ 符号 双向转换
## ═══════════════════════════════════════════════════════════════
## 使用: nim c -r mqfncoder.nim "你要编码的拼音"
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, tables, parseutils, os]

# ═══════════════════════════════════════════════════════════════
# 核心编码表
# ═══════════════════════════════════════════════════════════════

# 声母表 27个 (含翘舌音 zh ch sh)
const SHENGMU = [
  "b", "p", "m", "f", "d", "t", "n", "l",
  "g", "k", "h", "w", "j", "q", "x", "y",
  "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
  "zh", "ch", "sh",
]

# 韵母核心映射 (拼音 → 满全法符号) 2026-08-22 定稿
const YUNMU = {
  "er":   "ə",
  "ai":   "æ",
  "ei":   "ɘ",
  "ao":   "ɒ",
  "ou":   "ɵ",
  "ia":   "ѧ",
  "ua":   "ʉ",
  "iao":  "ᴥ",
  "iou":  "ø",
  "iu":   "ø",
  "in":   "ɤ",
  "uo":   "œ",
  "uai":  "ɞ",
  "uei":  "ʚ",
  "ui":   "ʚ",
  "ue":   "ʊ",
  "an":   "ã",
  "en":   "ẽ",
  "vn":   "ʯ",
  "ang":  "ą",
  "eng":  "ę",
  "ing":  "į",
  "ong":  "ɷ",
  "ian":  "ʌ",
  "uan":  "๑",
  "van":  "ʮ",
  "uang": "ѽ",
  "ueng": "ɔ",
  "iang": "ѫ",
  "iong": "ѻ",
  "ii":   "ĳ",
  "io":   "ᵻ",
  "ie":   "ᵼ",
  "en2":  "ĩ",
  "iai":  "ɚ",
  "v":    "ü",
  "ve":   "ʊ",
  "vn2":  "ʯ",
}.toTable

# 声调映射
const T1 = {'a': "ā", 'e': "ē", 'i': "ī", 'o': "ō", 'u': "ū"}.toTable
const T2 = {'a': "á", 'e': "é", 'i': "í", 'o': "ó", 'u': "ú"}.toTable
const T3 = {'a': "ǎ", 'e': "ě", 'i': "ǐ", 'o': "ǒ", 'u': "ǔ"}.toTable
const T4 = {'a': "à", 'e': "è", 'i': "ì", 'o': "ò", 'u': "ù"}.toTable
const TONES = [T1, T2, T3, T4]

# ──────────────────────────────────────────────────────────────
# 编码函数
# ──────────────────────────────────────────────────────────────

proc encode_yunmu*(py: string): string =
  let lower = py.toLower
  if lower in YUNMU: return YUNMU[lower]
  # 单个拉丁字母韵母直接保留
  if lower.len == 1 and lower[0] in {'a', 'e', 'i', 'o', 'u', 'v'}: return lower
  # fallback: 尝试前缀匹配
  for key in YUNMU.keys:
    if lower.startsWith(key) and key.len > 1:
      return YUNMU[key]
  return "?" & py & "?"

proc apply_tone*(s: string, tone: int): string =
  if tone < 1 or tone > 4: return s
  let tbl = TONES[tone - 1]
  for c in ['a', 'e', 'i', 'o', 'u']:
    if s.contains(c):
      return s.replace($c, tbl[c])
  return s

proc encode_syllable*(py: string): string =
  let lower = py.toLower.strip
  var tone = 0
  var body = lower
  if lower.len > 0 and lower[^1] in '1'..'4':
    tone = parseInt($lower[^1])
    body = lower[0 ..< ^1]

  # 提取声母
  var sm = ""
  var ym_body = body
  for s in SHENGMU:
    if body.startsWith(s) and s.len > sm.len:
      sm = s
      ym_body = body[s.len ..^ 1]

  if sm == "":
    let base = encode_yunmu(ym_body)
    if tone > 0:
      return apply_tone(base, tone)
    return base

  let ym_sym = encode_yunmu(ym_body)
  if tone > 0:
    return sm & apply_tone(ym_sym, tone)
  return sm & ym_sym

proc encode*(input: string): string =
  var parts: seq[string] = @[]
  var cur = ""
  for ch in input:
    if ch == ' ' or ch == '\t' or ch == '\n':
      if cur.len > 0: parts.add(cur); cur = ""
    else:
      cur.add(ch)
  if cur.len > 0: parts.add(cur)
  var syms: seq[string] = @[]
  for p in parts:
    syms.add(encode_syllable(p))
  result = syms.join(" ")

# ──────────────────────────────────────────────────────────────
# 主程序
# ──────────────────────────────────────────────────────────────

when isMainModule:
  echo "═══ 满全法编码器 ═══"
  echo "声母: ", SHENGMU.len, " 个"
  echo "韵母映射: ", YUNMU.len, " 条"
  echo ""

  if paramCount() >= 1:
    let inp = paramStr(1)
    echo "拼音: ", inp
    echo "编码: ", encode(inp)
  else:
    echo "═══ 测试 ═══"
    for t in ["zhong guo", "man quan fa", "liu chu tian", "ai ei ao ou an en ang eng"]:
      echo "─"
      echo "拼音: ", t
      echo "编码: ", encode(t)
