# 满全法 66进制韵母编码容器
# 33韵母 × 2(大小写) = 66符号 = 66进制
# 规则：只用明确配对符号，每符号有发音、有大小写、有归属

import tables

const SYMBOLS66* = [
  # --- #0-9: 单韵母 (基础元音) ---
  "a",    # 0  a       基础拉丁小写
  "A",    # 1  A       基础拉丁大写
  "ȍ",    # 2  o       拉丁O+短音
  "Ȍ",    # 3  O       拉丁O+短音大写
  "ӗ",    # 4  e       西里尔E+短音小写
  "Ӗ",    # 5  E       西里尔E+短音大写
  "i",    # 6  i       基础拉丁小写
  "I",    # 7  I       基础拉丁大写
  "u",    # 8  u       基础拉丁小写
  "U",    # 9  U       基础拉丁大写

  # --- #10-19: 复韵母 ---
  "ü",    # 10 ü       拉丁U+分音小写
  "Ü",    # 11 Ü       拉丁U+分音大写
  "ꞟ",    # 12 ai      拉丁连体变体小写
  "Ꞟ",    # 13 AI      拉丁连体变体大写
  "ɘ",    # 14 ei      音标反转E小写
  "Ǝ",    # 15 EI      拉丁E镜像大写
  "ɒ",    # 16 ao      拉丁alpha变体小写
  "Ɒ",    # 17 AO      拉丁alpha变体大写
  "౪",    # 18 ou      泰卢固OU连字小写
  "ȣ",    # 19 OU      西里尔UK连体(作大写)

  # --- #20-29: 复合韵母 ---
  "ɛ",    # 20 er      拉丁epsilon小写
  "Ɛ",    # 21 ER      拉丁epsilon大写
  "ѧ",    # 22 ia      古西里尔I+A小写
  "Ѧ",    # 23 IA      古西里尔I+A大写
  "ɤ",    # 24 ie      音标ram小写
  "ɣ",    # 25 IE      音标gamma小写(作IE大写)
  "ʉ",    # 26 ua      拉丁U杠小写
  "Ʉ",    # 27 UA      拉丁U杠大写
  "ꞛ",    # 28 uo      拉丁连体UK小写
  "Ꞛ",    # 29 UO      拉丁连体UK大写

  # --- #30-39: 鼻韵母-an/en/in/ün/ang ---
  "ã",    # 30 an      拉丁A+波浪小写
  "Ã",    # 31 AN      拉丁A+波浪大写
  "ẽ",    # 32 en      拉丁E+波浪小写
  "Ẽ",    # 33 EN      拉丁E+波浪大写
  "ĩ",    # 34 in      拉丁I+波浪小写
  "Ĩ",    # 35 IN      拉丁I+波浪大写
  "ʯ",    # 36 ün      拉丁U+钩小写
  "ʮ",    # 37 ÜN      拉丁U+钩(作大写)
  "ą",    # 38 ang     拉丁A+反尾小写
  "Ą",    # 39 ANG     拉丁A+反尾大写

  # --- #40-49: 鼻韵母-eng/ing/ong/iao/uai ---
  "ę",    # 40 eng     拉丁E+反尾小写
  "Ę",    # 41 ENG     拉丁E+反尾大写
  "į",    # 42 ing     拉丁I+反尾小写
  "Į",    # 43 ING     拉丁I+反尾大写
  "๓",    # 44 ong     泰文数字3(作小写)
  "ɷ",    # 45 ONG     拉丁闭合omega小写(作大写)
  "ⱄ",    # 46 iao     古西里尔I+A+O小写
  "Ⱋ",    # 47 IAO     格拉哥里I+A+O大写
  "⊰",    # 48 uai     数学U+A+I小写
  "ɞ",    # 49 UAI     拉丁反转open O(作大写)

  # --- #50-59: 复合鼻韵母-ian/uan/üan/iang/uang ---
  "ʌ",    # 50 ian     拉丁V反转小写
  "Ʌ",    # 51 IAN     拉丁Lambda反转大写
  "ɔ",    # 52 uan     拉丁open O小写
  "Ɔ",    # 53 UAN     拉丁open O大写
  "и",    # 54 üan     西里尔i(作小写)
  "И",    # 55 ÜAN     西里尔I大写
  "ǡ",    # 56 iang    拉丁A+分音+波浪小写
  "Ǡ",    # 57 IANG    拉丁A+分音+波浪大写
  "ѽ",    # 58 uang    西里尔omega变体小写
  "Ѽ",    # 59 UANG    西里尔omega变体大写

  # --- #60-65: 特殊韵母-iong/üe/iu ---
  "ѻ",    # 60 iong    西里尔ON小写
  "Ѻ",    # 61 IONG    西里尔ON大写
  "ÿ",    # 62 üe      拉丁y分音小写
  "Ÿ",    # 63 ÜE      拉丁Y分音大写
  "ȳ",    # 64 iu      拉丁Y+沉音小写
  "Ȳ",    # 65 IU      拉丁Y+沉音大写
]

# ============================================================
# 韵母到编码索引的映射（多对一，冲突=等价）
# ============================================================
const YUNMU_CODES* = {
  # --- 单韵母 ---
  "a":        @[0, 1],
  "o":        @[2, 3],
  "e":        @[4, 5],
  "i":        @[6, 7],
  "u":        @[8, 9],
  "ü":        @[10, 11],

  # --- 复韵母 ---
  "ai":       @[12, 13],
  "ei":       @[14, 15],
  "ao":       @[16, 17],
  "ou":       @[18, 19],

  # --- 复合韵母 ---
  "er":       @[20, 21],
  "ia":       @[22, 23],
  "ie":       @[24, 25],
  "ua":       @[26, 27],
  "uo":       @[28, 29],

  # --- 鼻韵母 ---
  "an":       @[30, 31],
  "en":       @[32, 33],
  "in":       @[34, 35],
  "ün":       @[36, 37],
  "ang":      @[38, 39],
  "eng":      @[40, 41],
  "ing":      @[42, 43],
  "ong":      @[44, 45],

  # --- 复合鼻韵母 ---
  "iao":      @[46, 47],
  "uai":      @[48, 49],
  "ian":      @[50, 51],
  "uan":      @[52, 53],
  "üan":      @[54, 55],
  "iang":     @[56, 57],
  "uang":     @[58, 59],

  # --- 特殊韵母 ---
  "iong":     @[60, 61],
  "üe":       @[62, 63],
  "iu":       @[64, 65],
}.toTable

# ============================================================
# 容器节点
# ============================================================
type
  YunmuNode* = object
    yunmu*: string
    codes*: seq[int]
    symbols*: seq[string]

  RongQi66* = object
    name*: string
    symbol_to_idx*: Table[string, int]
    nodes*: seq[YunmuNode]

func new_rongqi66*(name: string): RongQi66 =
  result.name = name
  result.symbol_to_idx = initTable[string, int]()
  for i, sym in SYMBOLS66:
    result.symbol_to_idx[sym] = i

func symbol_value*(r: RongQi66, sym: string): int =
  if r.symbol_to_idx.hasKey(sym): return r.symbol_to_idx[sym]
  return -1

func is_equivalent*(a: string, b: string): bool =
  let r = new_rongqi66("tmp")
  let va = r.symbol_value(a)
  let vb = r.symbol_value(b)
  if va >= 0 and vb >= 0: return va == vb
  return a == b

proc add_yunmu*(r: var RongQi66, name: string) =
  var node: YunmuNode
  node.yunmu = name
  if YUNMU_CODES.hasKey(name):
    node.codes = YUNMU_CODES[name]
    for idx in node.codes:
      if idx < SYMBOLS66.len:
        node.symbols.add(SYMBOLS66[idx])
  r.nodes.add(node)

func primary_code*(node: YunmuNode): int =
  if node.codes.len > 0: return node.codes[0]
  return -1

func is_yunmu_equiv*(a: YunmuNode, b: YunmuNode): bool =
  for ca in a.codes:
    for cb in b.codes:
      if ca == cb: return true
  return false

# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 满全法 66进制韵母编码 ==="
  echo "码元总数: ", SYMBOLS66.len, " (33韵母 × 2)"
  echo ""

  var r = new_rongqi66("满全法韵母")

  var yunmu_list = @[
    "a", "o", "e", "i", "u", "ü",
    "ai", "ei", "ao", "ou",
    "er", "ia", "ie", "ua", "uo",
    "an", "en", "in", "ün", "ang", "eng", "ing", "ong",
    "iao", "uai", "ian", "uan", "üan", "iang", "uang",
    "iong", "üe", "iu"
  ]

  for ym in yunmu_list:
    r.add_yunmu(ym)

  echo "已装载韵母: ", r.nodes.len
  echo "---"

  echo "【韵母编码表】"
  for node in r.nodes:
    let c = primary_code(node)
    var syms = ""
    for s in node.symbols:
      if syms.len > 0: syms &= ","
      syms &= s
    echo "  ", node.yunmu, " -> #", c, " [", syms, "]"

  echo ""
  echo "韵母:", r.nodes.len, " | 码元:", SYMBOLS66.len, " | 进制: 66"
