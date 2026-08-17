# 满全法 50进制变音韵母编码（大写独立文件）
# 50个大写变音符号，与 yunmu50_lower.nim 一一对应

import tables

const YUNMU50_UPPER* = [
  # --- 第1组：下点韵母大写 (A O E I U Ü) ---
  "Ạ",    # 0  A+下点
  "Ẹ",    # 1  E+下点
  "Ị",    # 2  I+下点
  "Ọ",    # 3  O+下点
  "Ụ",    # 4  U+下点
  "Ä",    # 5  A+分音
  "Ë",    # 6  E+分音
  "Ï",    # 7  I+分音
  "Ö",    # 8  O+分音
  "Ü",    # 9  U+分音

  # --- 第2组：倒短音 + 双重音 ---
  "Ȃ",    # 10 A+倒短音
  "Ȇ",    # 11 E+倒短音
  "Ȋ",    # 12 I+倒短音
  "Ȏ",    # 13 O+倒短音
  "Ȗ",    # 14 U+倒短音
  "A̋",    # 15 A+双重音
  "E̋",    # 16 E+双重音
  "I̋",    # 17 I+双重音
  "Ő",    # 18 O+双重音
  "Ű",    # 19 U+双重音

  # --- 第3组：上圈 + 反尾 ---
  "Å",    # 20 A+上圈
  "E̊",    # 21 E+上圈
  "I̊",    # 22 I+上圈
  "O̊",    # 23 O+上圈
  "Ů",    # 24 U+上圈
  "Ą",    # 25 A+反尾
  "Ę",    # 26 E+反尾
  "Į",    # 27 I+反尾
  "Ǫ",    # 28 O+反尾
  "Ų",    # 29 U+反尾

  # --- 第4组：波浪 + 帽子 ---
  "Ã",    # 30 A+波浪
  "Ẽ",    # 31 E+波浪
  "Ĩ",    # 32 I+波浪
  "Õ",    # 33 O+波浪
  "Ũ",    # 34 U+波浪
  "Â",    # 35 A+帽子
  "Ê",    # 36 E+帽子
  "Î",    # 37 I+帽子
  "Ô",    # 38 O+帽子
  "Û",    # 39 U+帽子

  # --- 第5组：短音 + 反钝音 ---
  "Ă",    # 40 A+短音
  "Ĕ",    # 41 E+短音
  "Ĭ",    # 42 I+短音
  "Ŏ",    # 43 O+短音
  "Ŭ",    # 44 U+短音
  "Ȁ",    # 45 A+反钝音
  "Ȅ",    # 46 E+反钝音
  "Ȉ",    # 47 I+反钝音
  "Ȍ",    # 48 O+反钝音
  "Ȕ",    # 49 U+反钝音
]

const GROUP_NAMES* = [
  "下点韵母",
  "倒短音+双重音",
  "上圈+反尾",
  "波浪+帽子",
  "短音+反钝音",
]

type
  YunmuNode* = object
    idx*: int
    symbol*: string
    group*: int
    mirror_lower*: int  # 对应小写位置

  YunmuTable* = object
    name*: string
    nodes*: seq[YunmuNode]
    symbol_to_idx*: Table[string, int]

func new_yunmu_upper*(name: string): YunmuTable =
  result.name = name
  result.nodes = @[]
  result.symbol_to_idx = initTable[string, int]()
  for i, sym in YUNMU50_UPPER:
    var node: YunmuNode
    node.idx = i
    node.symbol = sym
    node.group = i div 10
    node.mirror_lower = i
    result.nodes.add(node)
    result.symbol_to_idx[sym] = i

when is_main_module:
  echo "=== 满全法 50进制变音韵母（大写） ==="
  var yt = new_yunmu_upper("50进制大写韵母")
  echo "码元总数: ", YUNMU50_UPPER.len
  echo "---"

  for g in 0..4:
    echo "第", g+1, "组（", GROUP_NAMES[g], "）:"
    for i in 0..9:
      let idx = g * 10 + i
      if idx < YUNMU50_UPPER.len:
        echo "  #", idx, " ", YUNMU50_UPPER[idx]
