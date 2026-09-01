# 编解码 — 符号 ↔ 64位单元格
# 位分配：[极性(1)][类别(3)][进制(8)][内码(8)][码点低32位(32)][备用(12)]
#
# 64进制单元格 = 七阶比特容器的语音特化版（对齐现有 yunmu64 容器协议）

import tables
import unicode
import options
import types
import symbols
import ownership

# 64 位单元格局常量
const
  POLARITY_SHIFT* = 63
  CATEGORY_SHIFT* = 60
  RADIX_SHIFT*   = 52
  INNER_SHIFT*   = 44
  CODEPOINT_MASK* = 0xFFFFFFFF'u32

# 从单个符号编一个单元格
func encode_cell*(cp: int32; category: SymCategory; radix: uint8;
                  inner_code: uint8; polarity: Polarity): PhoneticCell =
  let p = (uint64(polarity) shl POLARITY_SHIFT)
  let c = (uint64(category) shl CATEGORY_SHIFT)
  let r = (uint64(radix) shl RADIX_SHIFT)
  let i = (uint64(inner_code) shl INNER_SHIFT)
  let codep = uint64(cp) and CODEPOINT_MASK
  result = p or c or r or i or codep

# 解码一个单元格 → 各字段
func decode_cell*(cell: PhoneticCell; out_cp: var int32; out_cat: var SymCategory;
                  out_radix: var uint8; out_inner: var uint8; out_pol: var Polarity) =
  out_pol = Polarity((cell shr POLARITY_SHIFT) and 0x1)
  out_cat = SymCategory((cell shr CATEGORY_SHIFT) and 0x7)
  out_radix = uint8((cell shr RADIX_SHIFT) and 0xFF)
  out_inner = uint8((cell shr INNER_SHIFT) and 0xFF)
  out_cp = int32(cell and CODEPOINT_MASK)

# 主人表构建 — 从确认串+声母+声调一次性生成
func build_owner_table*(): PhoneticTable =
  result.name = "owner_phonetic_v1"
  result.version = VERSION
  result.symbols = newSeq[PhoneticSymbol]()
  result.codepoint_to_idx = initTable[int32, int]()

  # 从确认编码串加载 66 韵母对 = 132 符号
  var idx = 0
  let runes = FINAL_ENCODING_STRING.toRunes()
  var pair_idx = 0
  var i = 0
  while i < runes.len:
    if runes[i] == Rune(0x7D):  # '}'
      i += 1
      continue
    if i + 1 < runes.len and runes[i + 1] == Rune(0x7D):
      # 单字符（末位可能）
      let cp = int32(runes[i])
      var sym: PhoneticSymbol
      sym.cp = cp
      if pair_idx < YUNMU_NAMES.len:
        sym.label = YUNMU_NAMES[pair_idx]
      else:
        sym.label = "y" & $pair_idx
      sym.category = catFinal
      sym.radix = 64
      sym.inner_code = uint8(pair_idx)
      sym.polarity = symbol_ownership(cp)
      result.codepoint_to_idx[cp] = result.symbols.len
      result.symbols.add(sym)
      idx += 1
      i += 1
      pair_idx += 1
    else:
      # 大写+小写对
      let cp_upper = int32(runes[i])
      let cp_lower = int32(runes[i + 1])
      # 大写
      var sym_u: PhoneticSymbol
      sym_u.cp = cp_upper
      if pair_idx < YUNMU_NAMES.len:
        sym_u.label = YUNMU_NAMES[pair_idx] & "_大写"
      else:
        sym_u.label = "yu" & $pair_idx
      sym_u.category = catFinal
      sym_u.radix = 64
      sym_u.inner_code = uint8(pair_idx * 2)
      sym_u.polarity = symbol_ownership(cp_upper)
      result.codepoint_to_idx[cp_upper] = result.symbols.len
      result.symbols.add(sym_u)
      # 小写
      var sym_l: PhoneticSymbol
      sym_l.cp = cp_lower
      if pair_idx < YUNMU_NAMES.len:
        sym_l.label = YUNMU_NAMES[pair_idx] & "_小写"
      else:
        sym_l.label = "yl" & $pair_idx
      sym_l.category = catFinal
      sym_l.radix = 64
      sym_l.inner_code = uint8(pair_idx * 2 + 1)
      sym_l.polarity = symbol_ownership(cp_lower)
      result.codepoint_to_idx[cp_lower] = result.symbols.len
      result.symbols.add(sym_l)
      idx += 2
      i += 2
      pair_idx += 1
    i += 1

  # 追加声母 48
  for j in 0 ..< INITIALS_48.len:
    let cp = INITIALS_48[j][0]
    var sym: PhoneticSymbol
    sym.cp = cp
    sym.label = INITIALS_48[j][1]
    sym.category = catInitial
    sym.radix = 48
    sym.inner_code = uint8(j)
    sym.polarity = symbol_ownership(cp)
    result.codepoint_to_idx[cp] = result.symbols.len
    result.symbols.add(sym)

  # 追加声调 60
  for j in 0 ..< TONES_60.len:
    let cp = TONES_60[j][0]
    var sym: PhoneticSymbol
    sym.cp = cp
    sym.label = TONES_60[j][1]
    sym.category = catTone
    sym.radix = 60
    sym.inner_code = uint8(j)
    sym.polarity = symbol_ownership(cp)
    result.codepoint_to_idx[cp] = result.symbols.len
    result.symbols.add(sym)

# 查询：码点 → 符号
func lookup*(t: PhoneticTable; cp: int32): Option[PhoneticSymbol] =
  if t.codepoint_to_idx.hasKey(cp):
    let idx = t.codepoint_to_idx[cp]
    return some(t.symbols[idx])
  return none(PhoneticSymbol)

# 按极性过滤
func filter_by_polarity*(t: PhoneticTable; p: Polarity): seq[PhoneticSymbol] =
  result = newSeq[PhoneticSymbol]()
  for sym in t.symbols:
    if sym.polarity == p:
      result.add(sym)

# 统计
func stats*(t: PhoneticTable): string =
  var n_owner = 0
  var n_outsider = 0
  var n_initial = 0
  var n_final = 0
  var n_tone = 0
  for sym in t.symbols:
    if sym.polarity == plOwner: inc n_owner
    else: inc n_outsider
    case sym.category
    of catInitial: inc n_initial
    of catFinal: inc n_final
    of catTone: inc n_tone
    of catCompound: discard
  result = "主人全息表 v1 | 总数=" & $t.symbols.len &
    " 主人=" & $n_owner & " 外人=" & $n_outsider &
    " 声母=" & $n_initial & " 韵母=" & $n_final & " 声调=" & $n_tone
