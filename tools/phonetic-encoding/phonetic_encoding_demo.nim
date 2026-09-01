# 语音符号编码 · 主人全息演示
# 证明：符号真正进入代码数据结构，不是 ASCII 占位符

import strutils
import unicode
import options
import phonetic_encoding/types
import phonetic_encoding/symbols
import phonetic_encoding/ownership
import phonetic_encoding/codec

proc hex4(cp: int32): string =
  var s = toHex(int(cp))
  while s.len < 4: s = "0" & s
  result = toUpperAscii(s)

# ── 阶段 1: 加载主人全息表 ──
echo "═══════════════════════════════════════════════════"
echo "  语音符号编码 v1 · 主人全息表"
echo "═══════════════════════════════════════════════════"
echo ""

let tbl = build_owner_table()
echo tbl.stats()
echo ""

# ── 阶段 2: 编码串实时读取证明 ──
echo "【阶段 2】编码串字符逐一验证（非猜测，直接读取）"
echo ""
let runes = FINAL_ENCODING_STRING.toRunes()
var real_count = 0
for r in runes:
  if r != Rune(0x7D):
    real_count += 1
echo "  编码串实际符号数: ", real_count
echo "  首字符: ", $runes[0], " (U+", hex4(int32(runes[0])), ")"
echo "  第二字符: ", $runes[1], " (U+", hex4(int32(runes[1])), ")"
echo ""

# ── 阶段 3: 逐符号编解码 ──
echo "【阶段 3】符号 <-> 64位单元格 编解码"
echo ""
let test_cps = @[
  0x020D'i32,
  0x0466'i32,
  0xA657'i32,
  0x011B'i32,
  0x0225'i32,
]
for cp in test_cps:
  let catg: SymCategory = (if cp >= 0x0100 and cp <= 0x024F: catFinal
      elif cp >= 0x0400 and cp <= 0x04FF: catFinal
      else: catInitial)
  let pol = symbol_ownership(cp)
  let cell = encode_cell(cp, catg, 64'u8, uint8(cp and 0xFF), pol)
  var d_cp: int32
  var d_cat: SymCategory
  var d_radix: uint8
  var d_inner: uint8
  var d_pol: Polarity
  decode_cell(cell, d_cp, d_cat, d_radix, d_inner, d_pol)
  let mark = if pol == plOwner: "主人" else: "外人"
  echo "  U+", hex4(cp), " ", $Rune(cp),
    " -> cell=0x", toUpperAscii(toHex(int(cell))),
    " 极性=", mark, " -> 译回U+", hex4(d_cp)
echo ""

# ── 阶段 4: 产权镜像对比 ──
echo "【阶段 4】产权镜像对比 (主人 vs 外人)"
echo ""
let owner_syms = tbl.filter_by_polarity(plOwner)
let outsider_syms = tbl.filter_by_polarity(plOutsider)
echo "  主人符号数: ", owner_syms.len
echo "  外人符号数: ", outsider_syms.len
echo ""
echo "  主人符号前五:"
for i in 0 .. min(4, owner_syms.len - 1):
  let s = owner_syms[i]
  echo "    U+", hex4(s.cp), " ", $Rune(s.cp), " [", s.label, "]"
echo "  外人符号前五:"
for i in 0 .. min(4, outsider_syms.len - 1):
  let s = outsider_syms[i]
  echo "    U+", hex4(s.cp), " ", $Rune(s.cp), " [", s.label, "]"
echo ""

# ── 阶段 5: 查询演示 ──
echo "【阶段 5】码点查询"
echo ""
let q1 = tbl.lookup(0x0466'i32)
if isSome(q1):
  let s = q1.get()
  let pol_str = if s.polarity == plOwner: "主人" else: "外人"
  echo "  查询 Ѧ(U+0466): 韵母=", s.label, " 进制=", s.radix,
    " 内码=", s.inner_code, " 极性=", pol_str

let q2 = tbl.lookup(0x0300'i32)
if q2.isNone(q2):
  echo "  查询 U+0300: 未收录（外人/自由空间）"
echo ""
echo "═══════════════════════════════════════════════════"
echo "  完成：符号真正进入代码（非 ASCII 占位符）"
echo "═══════════════════════════════════════════════════"
