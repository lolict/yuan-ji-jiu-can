# 满全法 60进制声调编码
# 结构：5声 × 6韵(a/o/e/i/u/ü) × 2套(大小写) = 60
# 五声：阴平(1) 阳平(2) 上声(3) 去声(4) 轻声(0)

import tables

const TONE60* = [
  # ========== 小写（#0-#29）==========
  # --- 第1声（阴平）---
  "ā",    # 0  a+1声
  "ō",    # 1  o+1声
  "ē",    # 2  e+1声
  "ī",    # 3  i+1声
  "ū",    # 4  u+1声
  "ǖ",    # 5  ü+1声

  # --- 第2声（阳平）---
  "á",    # 6  a+2声
  "ó",    # 7  o+2声
  "é",    # 8  e+2声
  "í",    # 9  i+2声
  "ú",    # 10 u+2声
  "ǘ",    # 11 ü+2声

  # --- 第3声（上声）---
  "ǎ",    # 12 a+3声
  "ǒ",    # 13 o+3声
  "ě",    # 14 e+3声
  "ǐ",    # 15 i+3声
  "ǔ",    # 16 u+3声
  "ǚ",    # 17 ü+3声

  # --- 第4声（去声）---
  "à",    # 18 a+4声
  "ò",    # 19 o+4声
  "è",    # 20 e+4声
  "ì",    # 21 i+4声
  "ù",    # 22 u+4声
  "ǜ",    # 23 ü+4声

  # --- 轻声（第0声·虚无底）---
  "a",    # 24 a+轻声
  "o",    # 25 o+轻声
  "e",    # 26 e+轻声
  "i",    # 27 i+轻声
  "u",    # 28 u+轻声
  "ü",    # 29 ü+轻声

  # ========== 大写（#30-#59）==========
  # --- 第1声（阴平）大写 ---
  "Ā",    # 30 A+1声
  "Ō",    # 31 O+1声
  "Ē",    # 32 E+1声
  "Ī",    # 33 I+1声
  "Ū",    # 34 U+1声
  "Ǖ",    # 35 Ü+1声

  # --- 第2声（阳平）大写 ---
  "Á",    # 36 A+2声
  "Ó",    # 37 O+2声
  "É",    # 38 E+2声
  "Í",    # 39 I+2声
  "Ú",    # 40 U+2声
  "Ǘ",    # 41 Ü+2声

  # --- 第3声（上声）大写 ---
  "Ǎ",    # 42 A+3声
  "Ǒ",    # 43 O+3声
  "Ě",    # 44 E+3声
  "Ǐ",    # 45 I+3声
  "Ǔ",    # 46 U+3声
  "Ǚ",    # 47 Ü+3声

  # --- 第4声（去声）大写 ---
  "À",    # 48 A+4声
  "Ò",    # 49 O+4声
  "È",    # 50 E+4声
  "Ì",    # 51 I+4声
  "Ù",    # 52 U+4声
  "Ǜ",    # 53 Ü+4声

  # --- 轻声（第0声·虚无底）大写 ---
  "A",    # 54 A+轻声
  "O",    # 55 O+轻声
  "E",    # 56 E+轻声
  "I",    # 57 I+轻声
  "U",    # 58 U+轻声
  "Ü",    # 59 Ü+轻声
]

# 声调名
const TONE_NAMES* = ["轻声", "阴平", "阳平", "上声", "去声"]

# 基础元音
const BASE_VOWELS* = ["a", "o", "e", "i", "u", "ü"]

# 五声编号（0=轻声, 1=阴平, 2=阳平, 3=上声, 4=去声）
const TONE_NUMS* = [1, 2, 3, 4, 0]

type
  ToneNode* = object
    idx*: int
    symbol*: string
    tone*: int        # 0=轻声, 1=阴平, 2=阳平, 3=上声, 4=去声
    vowel*: string    # a/o/e/i/u/ü
    is_upper*: bool   # 是否大写

  ToneTable* = object
    name*: string
    nodes*: seq[ToneNode]
    symbol_to_idx*: Table[string, int]

func new_tone60*(name: string): ToneTable =
  result.name = name
  result.nodes = @[]
  result.symbol_to_idx = initTable[string, int]()
  for i, sym in TONE60:
    var node: ToneNode
    node.idx = i
    node.symbol = sym
    let base_idx = i mod 30
    if base_idx < 24:
      # 有调：0-5=1声, 6-11=2声, 12-17=3声, 18-23=4声
      node.tone = (base_idx div 6) + 1
    else:
      # 轻声：24-29
      node.tone = 0
    node.vowel = BASE_VOWELS[base_idx mod 6]
    node.is_upper = (i >= 30)
    result.nodes.add(node)
    result.symbol_to_idx[sym] = i

# 获取声调编号（0=轻声, 1-4=四声）
func tone_num*(idx: int): int =
  let b = idx mod 30
  if b < 24: return (b div 6) + 1
  else: return 0

# 获取基础元音
func vowel*(idx: int): string =
  return BASE_VOWELS[(idx mod 30) mod 6]

# 是否大写
func is_upper*(idx: int): bool =
  return idx >= 30

# 大小写配对索引
func mirror*(idx: int): int =
  if idx < 30: return idx + 30
  else: return idx - 30

when is_main_module:
  echo "=== 满全法 60进制声调 ==="
  var tt = new_tone60("60进制声调")
  echo "码元总数: ", TONE60.len
  echo "结构: 5声 × 6韵 × 2套(大小写) = 60"
  echo "---"

  echo "【小写 #0-#29】"
  echo "  阴平: ", TONE60[0], " ", TONE60[1], " ", TONE60[2], " ", TONE60[3], " ", TONE60[4], " ", TONE60[5]
  echo "  阳平: ", TONE60[6], " ", TONE60[7], " ", TONE60[8], " ", TONE60[9], " ", TONE60[10], " ", TONE60[11]
  echo "  上声: ", TONE60[12], " ", TONE60[13], " ", TONE60[14], " ", TONE60[15], " ", TONE60[16], " ", TONE60[17]
  echo "  去声: ", TONE60[18], " ", TONE60[19], " ", TONE60[20], " ", TONE60[21], " ", TONE60[22], " ", TONE60[23]
  echo "  轻声: ", TONE60[24], " ", TONE60[25], " ", TONE60[26], " ", TONE60[27], " ", TONE60[28], " ", TONE60[29]

  echo ""
  echo "【大写 #30-#59】"
  echo "  阴平: ", TONE60[30], " ", TONE60[31], " ", TONE60[32], " ", TONE60[33], " ", TONE60[34], " ", TONE60[35]
  echo "  阳平: ", TONE60[36], " ", TONE60[37], " ", TONE60[38], " ", TONE60[39], " ", TONE60[40], " ", TONE60[41]
  echo "  上声: ", TONE60[42], " ", TONE60[43], " ", TONE60[44], " ", TONE60[45], " ", TONE60[46], " ", TONE60[47]
  echo "  去声: ", TONE60[48], " ", TONE60[49], " ", TONE60[50], " ", TONE60[51], " ", TONE60[52], " ", TONE60[53]
  echo "  轻声: ", TONE60[54], " ", TONE60[55], " ", TONE60[56], " ", TONE60[57], " ", TONE60[58], " ", TONE60[59]

  echo ""
  echo "【轻声验证——虚无底/源头】"
  for i in 0..5:
    echo "  #", 24+i, " ", TONE60[24+i], " ↔ #", 24+i+30, " ", TONE60[24+i+30]
