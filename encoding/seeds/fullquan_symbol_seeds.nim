## 满全法 · 符号种子注册表
## 来源:
##   - tools/kungguan-container/shengmu48.nim (声母48)
##   - tools/kungguan-container/yunmu66.nim (韵母66)
##   - docs/bootstrap/PROJECTION_PERCEPTION_CONSCIOUSNESS.md (感知体验编码)
##   - 满全法对话记录: 变音韵母100符号种子
##
## 用途: 社会模拟 · 意识编码 · 感知场符号集

import strutils

# ═══════════════════════════════════════════════════════════════
# 种子一: 变音韵母100符号 (感知体验层)
# 来源: 满全法对话记录 · 变音分组
# 下点=沉 · 倒短音=闪 · 双重音=穿透 · 上圈=轻 · 反尾=收
# 波浪=柔 · 帽子=飞扬 · 短音=急促 · 反钝音=沉重
# ═══════════════════════════════════════════════════════════════

const VOWEL_100_SEEDS* = [
  # --- #0-9: 下点韵母 (沉 · 深 · 郑重) ---
  "ạ", "ẹ", "ị", "ọ", "ụ",
  "ä", "ë", "ï", "ö", "ü",

  # --- #10-19: 倒短音 + 双重音 (闪 · 急促 · 穿透) ---
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ",
  "a̋", "e̋", "i̋", "ő", "ű",

  # --- #20-29: 上圈 + 反尾 (轻 · 收) ---
  "å", "e̊", "i̊", "o̊", "ů",
  "ą", "ę", "į", "ǫ", "ų",

  # --- #30-39: 波浪 + 帽子 (柔 · 摇曳 · 飞扬) ---
  "ã", "ẽ", "ĩ", "õ", "ũ",
  "â", "ê", "î", "ô", "û",

  # --- #40-49: 短音 + 反钝音 (急促 · 紧张 · 沉重 · 下降) ---
  "ă", "ĕ", "ĭ", "ŏ", "ŭ",
  "ȁ", "ȅ", "ȉ", "ȍ", "ȕ",
]

# ═══════════════════════════════════════════════════════════════
# 种子二: 变音韵母100 · 大写对应
# ═══════════════════════════════════════════════════════════════

const VOWEL_100_UPPER* = [
  "Ạ", "Ẹ", "Ị", "Ọ", "Ụ",
  "Ä", "Ë", "Ï", "Ö", "Ü",

  "Ȃ", "Ȇ", "Ȋ", "Ȏ", "Ȕ",
  "A̋", "E̋", "I̋", "Ő", "Ű",

  "Å", "E̊", "I̊", "O̊", "Ů",
  "Ą", "Ę", "Į", "Ǫ", "Ų",

  "Ã", "Ẽ", "Ĩ", "Õ", "Ũ",
  "Â", "Ê", "Î", "Ô", "Û",

  "Ă", "Ĕ", "Ĭ", "Ŏ", "Ŭ",
  "Ȁ", "Ȅ", "Ȉ", "Ȍ", "Ȕ",
]

# ═══════════════════════════════════════════════════════════════
# 种子三: 声调韵母60 (时钟层)
# a e i o u ü + 四声(平上去去) × 2
# =轻声6 + 4声×6 + 大写12 → 60符号
# ═══════════════════════════════════════════════════════════════

const TONE_60_SEEDS* = [
  # 轻声
  "a", "e", "i", "o", "u", "ü",
  "A", "E", "I", "O", "U", "Ü",
  # 1声(长音)
  "ā", "ē", "ī", "ō", "ū", "ǖ",
  "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ",
  # 2声(锐音)
  "á", "é", "í", "ó", "ú", "ǘ",
  "Á", "É", "Í", "Ó", "Ú", "Ǘ",
  # 3声(抑扬)
  "ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ",
  "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ",
  # 4声(钝音)
  "à", "è", "ì", "ò", "ù", "ǜ",
  "À", "È", "Ì", "Ò", "Ù", "Ǜ",
]

# ═══════════════════════════════════════════════════════════════
# 种子四: 声母48 (含扩展 v/V → 50)
# 来源: shengmu48.nim + 满全法对话记录
# ═══════════════════════════════════════════════════════════════

const SHENGMU48_SEEDS* = [
  # 唇音
  "b", "p", "m", "f",
  # 舌尖前音
  "d", "t", "n", "l",
  # 舌根音
  "g", "k", "h", "w",
  # 舌面音
  "j", "q", "x", "y",
  # 舌尖前音(平舌)
  "z", "c", "s", "r",
  # 卷舌音
  "ȥ", "ç", "ş", "ŋ",
  # v补位
  "v",
]


const SHENGMU48_UPPER_SEEDS* = [
  "B", "P", "M", "F",
  "D", "T", "N", "L",
  "G", "K", "H", "W",
  "J", "Q", "X", "Y",
  "Z", "C", "S", "R",
  "Ȥ", "Ç", "Ş", "Ŋ",
  "V",
]

# ═══════════════════════════════════════════════════════════════
# 种子五: 声母48 + 韵母66 基础对 (来自 yunmu66.nim · shengmu48.nim)
# 这是基础码元层 · 变音是叠加在这上面的体验层
# ═══════════════════════════════════════════════════════════════

const YUNMU66_SEEDS* = [
  # 单韵母
  "a", "A",         # 0-1
  "ȍ", "Ȍ",         # 2-3
  "ӗ", "Ӗ",         # 4-5
  "i", "I",         # 6-7
  "u", "U",         # 8-9
  "ü", "Ü",         # 10-11
  # 复韵母
  "ꞟ", "Ꞟ",         # 12-13 ai
  "ɘ", "Ǝ",         # 14-15 ei
  "ɒ", "Ɒ",         # 16-17 ao
  "౪", "ȣ",         # 18-19 ou
  # 复合韵母
  "ɛ", "Ɛ",         # 20-21 er
  "ѧ", "Ѧ",         # 22-23 ia
  "ɤ", "ɣ",         # 24-25 ie
  "ʉ", "Ʉ",         # 26-27 ua
  "ꞛ", "Ꞛ",         # 28-29 uo
  # 鼻韵母
  "ã", "Ã",         # 30-31 an
  "ẽ", "Ẽ",         # 32-33 en
  "ĩ", "Ĩ",         # 34-35 in
  "ʯ", "ʮ",         # 36-37 ün
  "ą", "Ą",         # 38-39 ang
  "ę", "Ę",         # 40-41 eng
  "į", "Į",         # 42-43 ing
  "๓", "ɷ",         # 44-45 ong
  # 三合
  "ⱄ", "Ⱋ",         # 46-47 iao
  "⊰", "ɞ",         # 48-49 uai
  "ʌ", "Ʌ",         # 50-51 ian
  "ɔ", "Ɔ",         # 52-53 uan
  "и", "И",         # 54-55 üan
  # 四合
  "ǡ", "Ǡ",         # 56-57 iang
  "ѽ", "Ѽ",         # 58-59 uang
  "ѻ", "Ѻ",         # 60-61 iong
  # 特殊
  "ÿ", "Ÿ",         # 62-63 üe
  "ȳ", "Ȳ",         # 64-65 iu
]

# ═══════════════════════════════════════════════════════════════
# 感知体验映射
# 来源: docs/bootstrap/PROJECTION_PERCEPTION_CONSCIOUSNESS.md
# ═══════════════════════════════════════════════════════════════

type QiZhi* = enum      # 体验质感
  qzDownDot     # 下点: 沉 · 深 · 郑重  (ạ ẹ ị ọ ụ)
  qzDiaeresis   # 分音: 郑重 · 仪式感   (ä ë ï ö ü)
  qzInvertBreve # 倒短音: 闪 · 急促     (ȃ ȇ ȋ ȏ ȗ)
  qzDoubleAcute # 双重音: 穿透 · 强调   (a̋ e̋ i̋ ő ű)
  qzRingAbove   # 上圈: 轻 · 飘         (å e̊ i̊ o̊ ů)
  qzOgonek      # 反尾: 收 · 稳         (ą ę į ǫ ų)
  qzTilde       # 波浪: 柔 · 摇曳 · 不确定 (ã ẽ ĩ õ ũ)
  qzCircumflex  # 帽子: 向上 · 飞扬 · 期待  (â ê î ô û)
  qzBreve       # 短音: 急促 · 紧张 · 轻快  (ă ĕ ĭ ŏ ŭ)
  qzDoubleGrave # 反钝音: 沉重 · 悲伤 · 下降 (ȁ ȅ ȉ ȍ ȕ)
  qzNone        # 无变音: 平静 · 中性

proc vowel_qz*(v: string): QiZhi =
  ## 判断韵母属于哪组体验质感
  if v.len == 0: return qzNone
  # 用字符串前缀匹配（支持 Unicode 复合字符）
  if v.startsWith("ạ") or v.startsWith("ẹ") or v.startsWith("ị") or v.startsWith("ọ") or v.startsWith("ụ"): return qzDownDot
  if v.startsWith("ä") or v.startsWith("ë") or v.startsWith("ï") or v.startsWith("ö") or v.startsWith("ü"): return qzDiaeresis
  if v.startsWith("ȃ") or v.startsWith("ȇ") or v.startsWith("ȋ") or v.startsWith("ȏ") or v.startsWith("ȗ"): return qzInvertBreve
  if v.startsWith("a̋") or v.startsWith("e̋") or v.startsWith("i̋") or v.startsWith("ő") or v.startsWith("ű"): return qzDoubleAcute
  if v.startsWith("å") or v.startsWith("e̊") or v.startsWith("i̊") or v.startsWith("o̊") or v.startsWith("ů"): return qzRingAbove
  if v.startsWith("ą") or v.startsWith("ę") or v.startsWith("į") or v.startsWith("ǫ") or v.startsWith("ų"): return qzOgonek
  if v.startsWith("ã") or v.startsWith("ẽ") or v.startsWith("ĩ") or v.startsWith("õ") or v.startsWith("ũ"): return qzTilde
  if v.startsWith("â") or v.startsWith("ê") or v.startsWith("î") or v.startsWith("ô") or v.startsWith("û"): return qzCircumflex
  if v.startsWith("ă") or v.startsWith("ĕ") or v.startsWith("ĭ") or v.startsWith("ŏ") or v.startsWith("ŭ"): return qzBreve
  if v.startsWith("ȁ") or v.startsWith("ȅ") or v.startsWith("ȉ") or v.startsWith("ȍ") or v.startsWith("ȕ"): return qzDoubleGrave
  return qzNone

when is_main_module:
  echo "═══ 满全法 · 符号种子注册 ═══"
  echo ""
  echo "变音韵母100种子: ", VOWEL_100_SEEDS.len, " + ", VOWEL_100_UPPER.len, " = ", VOWEL_100_SEEDS.len + VOWEL_100_UPPER.len
  echo "声调韵母60种子: ", TONE_60_SEEDS.len
  echo "声母50种子: ", SHENGMU48_SEEDS.len, " + ", SHENGMU48_UPPER_SEEDS.len, " = ", SHENGMU48_SEEDS.len + SHENGMU48_UPPER_SEEDS.len
  echo "韵母66基础种子: ", YUNMU66_SEEDS.len, " (shengmu48+66 基础对)"
  echo ""

  echo "── 变音韵母100 ──"
  echo "小写: "
  for i, v in VOWEL_100_SEEDS:
    if i > 0 and i mod 10 == 0: echo ""
    stdout.write v & " "
  echo ""
  echo ""
  echo "大写: "
  for i, v in VOWEL_100_UPPER:
    if i > 0 and i mod 10 == 0: echo ""
    stdout.write v & " "
  echo ""
