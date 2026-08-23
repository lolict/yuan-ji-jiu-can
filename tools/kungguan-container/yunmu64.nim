# 满全法 64进制韵母编码（兼容模块）
# 实际实现与 yunmu66 相同：33韵母 × 2(大小写) = 66符号
# 此模块为 kungguan_container.nim 提供 yunmu64 接口
# 
# 注：64进制是早期命名，实际为 66 符号系统

import tables

const SYMBOLS66* = [
  "a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U",
  "ü", "Ü", "ꞟ", "Ꞟ", "ɘ", "Ǝ", "ɒ", "Ɒ", "౪", "ȣ",
  "ɛ", "Ɛ", "ѧ", "Ѧ", "ɤ", "ɣ", "ʉ", "Ʉ", "ꞛ", "Ꞛ",
  "ã", "Ã", "ẽ", "Ẽ", "ĩ", "Ĩ", "ʯ", "ʮ", "ą", "Ą",
  "ę", "Ę", "į", "Į", "๓", "ɷ", "ⱄ", "Ⱋ", "⊰", "ɞ",
  "ʌ", "Ʌ", "ɔ", "Ɔ", "и", "И", "ǡ", "Ǡ", "ѽ", "Ѽ",
  "ѻ", "Ѻ", "ÿ", "Ÿ", "ȳ", "Ȳ",
]

const YUNMU_CODES* = {
  "a": @[0, 1], "o": @[2, 3], "e": @[4, 5], "i": @[6, 7], "u": @[8, 9], "ü": @[10, 11],
  "ai": @[12, 13], "ei": @[14, 15], "ao": @[16, 17], "ou": @[18, 19],
  "er": @[20, 21], "ia": @[22, 23], "ie": @[24, 25], "ua": @[26, 27], "uo": @[28, 29],
  "an": @[30, 31], "en": @[32, 33], "in": @[34, 35], "ün": @[36, 37],
  "ang": @[38, 39], "eng": @[40, 41], "ing": @[42, 43], "ong": @[44, 45],
  "iao": @[46, 47], "uai": @[48, 49], "ian": @[50, 51], "uan": @[52, 53],
  "üan": @[54, 55], "iang": @[56, 57], "uang": @[58, 59],
  "iong": @[60, 61], "üe": @[62, 63], "iu": @[64, 65],
}.toTable

type
  YunmuNode* = object
    yunmu*: string
    codes*: seq[int]
    symbols*: seq[string]

  RongQi66* = object
    name*: string
    symbol_to_idx*: Table[string, int]
    nodes*: seq[YunmuNode]

func new_rongqi64*(name: string): RongQi66 =
  result.name = name
  result.symbol_to_idx = initTable[string, int]()
  for i, sym in SYMBOLS66:
    result.symbol_to_idx[sym] = i

func symbol_value*(r: RongQi66, sym: string): int =
  if r.symbol_to_idx.hasKey(sym): return r.symbol_to_idx[sym]
  return -1

func is_equivalent*(a: string, b: string): bool =
  let r = new_rongqi64("tmp")
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
