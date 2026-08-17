# 满全法 50进制变音韵母编码（小写独立文件）
# 50个小写变音符号，按音标类型分组排序
# 分组顺序：下点 → 倒短音 → 双重音 → 上圈反尾 → 波浪帽子 → 短音反钝音

import tables

const YUNMU50_LOWER* = [
  # --- 第1组：下点韵母 (a o e i u ü) ---
  "ạ",    # 0  a+下点
  "ẹ",    # 1  e+下点
  "ị",    # 2  i+下点
  "ọ",    # 3  o+下点
  "ụ",    # 4  u+下点
  "ä",    # 5  a+分音
  "ë",    # 6  e+分音
  "ï",    # 7  i+分音
  "ö",    # 8  o+分音
  "ü",    # 9  u+分音

  # --- 第2组：倒短音 + 双重音 ---
  "ȃ",    # 10 a+倒短音
  "ȇ",    # 11 e+倒短音
  "ȋ",    # 12 i+倒短音
  "ȏ",    # 13 o+倒短音
  "ȗ",    # 14 u+倒短音
  "a̋",    # 15 a+双重音
  "e̋",    # 16 e+双重音
  "i̋",    # 17 i+双重音
  "ő",    # 18 o+双重音
  "ű",    # 19 u+双重音

  # --- 第3组：上圈 + 反尾 ---
  "å",    # 20 a+上圈
  "e̊",    # 21 e+上圈
  "i̊",    # 22 i+上圈
  "o̊",    # 23 o+上圈
  "ů",    # 24 u+上圈
  "ą",    # 25 a+反尾
  "ę",    # 26 e+反尾
  "į",    # 27 i+反尾
  "ǫ",    # 28 o+反尾
  "ų",    # 29 u+反尾

  # --- 第4组：波浪 + 帽子 ---
  "ã",    # 30 a+波浪
  "ẽ",    # 31 e+波浪
  "ĩ",    # 32 i+波浪
  "õ",    # 33 o+波浪
  "ũ",    # 34 u+波浪
  "â",    # 35 a+帽子
  "ê",    # 36 e+帽子
  "î",    # 37 i+帽子
  "ô",    # 38 o+帽子
  "û",    # 39 u+帽子

  # --- 第5组：短音 + 反钝音 ---
  "ă",    # 40 a+短音
  "ĕ",    # 41 e+短音
  "ĭ",    # 42 i+短音
  "ŏ",    # 43 o+短音
  "ŭ",    # 44 u+短音
  "ȁ",    # 45 a+反钝音
  "ȅ",    # 46 e+反钝音
  "ȉ",    # 47 i+反钝音
  "ȍ",    # 48 o+反钝音
  "ȕ",    # 49 u+反钝音
]

# 分组名
const GROUP_NAMES* = [
  "下点韵母",     # 0-9
  "倒短音+双重音", # 10-19
  "上圈+反尾",    # 20-29
  "波浪+帽子",    # 30-39
  "短音+反钝音",  # 40-49
]

type
  YunmuNode* = object
    idx*: int
    symbol*: string
    group*: int        # 所属组(0-4)
    base_vowel*: char  # 基础元音(a/e/i/o/u)
    diacritic*: string # 变音类型

  YunmuTable* = object
    name*: string
    nodes*: seq[YunmuNode]
    symbol_to_idx*: Table[string, int]

func new_yunmu_lower*(name: string): YunmuTable =
  result.name = name
  result.nodes = @[]
  result.symbol_to_idx = initTable[string, int]()
  for i, sym in YUNMU50_LOWER:
    var node: YunmuNode
    node.idx = i
    node.symbol = sym
    node.group = i div 10
    result.nodes.add(node)
    result.symbol_to_idx[sym] = i

# 基础元音提取（简化）
func base_vowel*(idx: int): char =
  const vowels = ['a', 'e', 'i', 'o', 'u', 'a', 'e', 'i', 'o', 'u']
  return vowels[idx mod 10]

# 变音类型描述
func diacritic_type*(idx: int): string =
  case idx div 10
  of 0:
    if idx mod 10 < 5: return "下点"
    else: return "分音"
  of 1:
    if idx mod 10 < 5: return "倒短音"
    else: return "双重音"
  of 2:
    if idx mod 10 < 5: return "上圈"
    else: return "反尾"
  of 3:
    if idx mod 10 < 5: return "波浪"
    else: return "帽子"
  of 4:
    if idx mod 10 < 5: return "短音"
    else: return "反钝音"
  else: return "未知"

when is_main_module:
  echo "=== 满全法 50进制变音韵母（小写） ==="
  var yt = new_yunmu_lower("50进制小写韵母")
  echo "码元总数: ", YUNMU50_LOWER.len
  echo "分组数: 5组 × 10 = 50"
  echo "---"

  for g in 0..4:
    echo "第", g+1, "组（", GROUP_NAMES[g], "）:"
    for i in 0..9:
      let idx = g * 10 + i
      echo "  #", idx, " ", YUNMU50_LOWER[idx]
