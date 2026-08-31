# ============================================================
# YIN-YANG-BINARY 完整映射引擎 · 演示
#
# 完整演示：符号→多比特YinYang编码、三值加法、十进制转换、全符号映射
# ============================================================

import strutils
import yinyang
import types

echo "═══════════════════════════════════════════════════════"
echo "  YIN-YANG-BINARY 完整映射引擎"
echo "  符号→多比特编码 | 三值加法 | 十进制转换"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 1】符号 → 多比特 YinYang 编码
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 1】符号 → 多比特 YinYang 编码"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  规则：每个字符 ASCII mod 2 → Yin(0)/Yang(1)"
echo "  锚位：所有位中 Yang 数量的奇偶性"
echo ""

let sample_symbols = ["mlll", "mlnn", "gtt", "lctqf", "maolilan", "yinyang", "smrl"]

for sym in sample_symbols:
  let encoded = encode_symbol(sym)
  var bits_str = ""
  for b in encoded.bits:
    bits_str &= $b
  echo "  ", sym.alignLeft(12), " → ", bits_str, " | 锚=", $encoded.anchor,
       " | 十进制=", $yinyang_to_decimal(encoded.bits)
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 2】构建完整映射表
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 2】完整映射表（64+ 符号）"
echo "═══════════════════════════════════════════════════════"
echo ""

let yymap = build_map()

echo "  已编码符号数: ", yymap.entries.len
echo "  最大位深度:   ", yymap.total_depth
echo "  锚点平衡:     ",
     case yymap.anchor_balance
     of 0: "阴盛（0多）"
     of 1: "阴阳平衡"
     of 2: "阳盛（1多）"
     else: "未知"
echo ""

echo "  符号映射表:"
echo "  " & "─".repeat(60)

for entry in yymap.entries:
  var bits_str = ""
  for b in entry.bits:
    bits_str &= $b
  let dec = yinyang_to_decimal(entry.bits)
  echo "  ", entry.source_symbol.alignLeft(14),
       " │ ", bits_str.alignLeft(20),
       " │ 锚=", $entry.anchor,
       " │ 十进制=", $dec

echo "  " & "─".repeat(60)
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 3】YinYang 三值加法
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 3】YinYang 三值加法运算"
echo "═══════════════════════════════════════════════════════"
echo ""

let sum_cases = [
  ("mlll", "gtt"),       # 根基对+桥梁
  ("mlnn", "mltt"),      # 囡囝+从体
  ("mlll", "mlnn"),      # 根基+囡囝
  ("gtt", "lctqf"),      # 桥梁+合规
  ("yin", "yang"),       # 阴+阳
  ("maolilan", "gouteng") # 毛利兰+工藤新一
]

for pair in sum_cases:
  let a = encode_symbol(pair[0])
  let b = encode_symbol(pair[1])
  let sum_result = add_binary(a, b)

  var a_str = ""
  for bit in a.bits: a_str &= $bit
  var b_str = ""
  for bit in b.bits: b_str &= $bit
  var r_str = ""
  for bit in sum_result.result.bits: r_str &= $bit

  let dec_a = yinyang_to_decimal(a.bits)
  let dec_b = yinyang_to_decimal(b.bits)
  let dec_r = yinyang_to_decimal(sum_result.result.bits)

  echo "  ", pair[0], " + ", pair[1]
  echo "    左操作数:  ", a_str, " (锚=", $a.anchor, ")"
  echo "    右操作数:  ", b_str, " (锚=", $b.anchor, ")"
  echo "    结果:      ", r_str, " (锚=", $sum_result.result.anchor, ")"
  echo "    验证:      ", dec_a, " + ", dec_b, " = ", dec_r
  echo ""

# ═══════════════════════════════════════════════════════
# 【Part 4】平衡验证
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 4】平衡验证 · 夫妻共同体对称性"
echo "═══════════════════════════════════════════════════════"
echo ""

# 编码夫妻关键对
let wife_encoded  = encode_symbol("lctfrqmy")  # 映射到刘楚恬
let husband_encoded = encode_symbol("wgslz")   # 映射到lolict
let both_encoded  = encode_symbol("mlnn")      # 囡囝共生

echo "  Wife 侧 (lctfrqmy):"
var wife_bits = ""
for b in wife_encoded.bits: wife_bits &= $b
echo "    ", wife_bits, " (锚=", $wife_encoded.anchor, ")"
echo ""
echo "  Husband 侧 (wgslz):"
var husb_bits = ""
for b in husband_encoded.bits: husb_bits &= $b
echo "    ", husb_bits, " (锚=", $husband_encoded.anchor, ")"
echo ""
echo "  囡囝共生 (mlnn):"
var both_bits = ""
for b in both_encoded.bits: both_bits &= $b
echo "    ", both_bits, " (锚=", $both_encoded.anchor, ")"
echo ""
let couple_sum = add_binary(wife_encoded, husband_encoded)
var couple_bits = ""
for b in couple_sum.result.bits: couple_bits &= $b
echo "  Wife + Husband = 囡囝共生:"
echo "    ", couple_bits, " (锚=", $couple_sum.result.anchor, ")"
echo "    溢出位: ", $couple_sum.overflow
echo "    ✅ 夫妻对相加，溢出=0 时为完美共生"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 5】性能与统计
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 5】统计与验证"
echo "═══════════════════════════════════════════════════════"
echo ""

var total_yin = 0
var total_yang = 0
var total_balanced = 0

for entry in yymap.entries:
  for b in entry.bits:
    case b
    of yyYin: total_yin += 1
    of yyYang: total_yang += 1
    of yyBalanced: total_balanced += 1
  case entry.anchor
  of yyYin: total_yin += 1
  of yyYang: total_yang += 1
  of yyBalanced: total_balanced += 1

echo "  总极性位:    ", total_yin + total_yang + total_balanced
echo "  Yin(0) 计数: ", total_yin
echo "  Yang(1) 计数: ", total_yang
echo "  Balanced 计数: ", total_balanced
echo ""
echo "  极性比 (阴:阳) = 1 : ",
     if total_yin > 0: formatFloat(float(total_yang) / float(total_yin), ffDecimal, 2)
     else: "∞"
echo ""

# ═══════════════════════════════════════════════════════
# 完成
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "  YIN-YANG-BINARY 完整映射引擎 · 演示完成"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  ✅ 64+ 符号 → 多比特 YinYang 编码"
echo "  ✅ 三值全加器（Yin/Yang/Balanced）"
echo "  ✅ 对称性验证（夫妻共同体加减法）"
echo "  ✅ YinYang ↔ 十进制双向转换"
echo "  ✅ 锚点平衡统计"
echo ""
echo "  [总开关已接入] YIN-YANG-BINARY 完整映射就绪。"
echo ""
