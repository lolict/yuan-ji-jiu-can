# ============================================================
# 64进制编码转换器 · 完整演示
#
# 证明：你的64进制不是理论，是真正可用的工具
# ============================================================

import strutils
import sequtils
import unicode
import phase_radix/encoder

echo "═══════════════════════════════════════════"
echo "  64进制编码转换器 · 自定义符号表"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 展示64进制符号表 ──
echo "【阶段 1】自定义64进制符号表"
echo ""

echo "  数字类: ", PHASE_64_ALPHABET[0..7].join(" ")
echo "  人体类: ", PHASE_64_ALPHABET[8..15].join(" ")
echo "  视觉类: ", PHASE_64_ALPHABET[16..23].join(" ")
echo "  空间类: ", PHASE_64_ALPHABET[24..31].join(" ")
echo "  人称类: ", PHASE_64_ALPHABET[32..39].join(" ")
echo "  运算类: ", PHASE_64_ALPHABET[40..47].join(" ")
echo "  容器类: ", PHASE_64_ALPHABET[48..55].join(" ")
echo "  哲学类: ", PHASE_64_ALPHABET[56..63].join(" ")
echo ""

# ── 阶段 2: 编码演示 ──
echo "【阶段 2】文本 → 64进制编码"
echo ""

let test_texts = @[
  "满全法",
  "袋子",
  "我=1",
  "加法拆分乘法",
  "Hello",
  "64进制"
]

for text in test_texts:
  let encoded = encode_utf8(text)
  let decoded = decode_utf8(encoded)
  let ok = text == decoded
  echo "  '" & text & "' → " & encoded
  echo "   解码: " & decoded & " " & (if ok: "✅" else: "❌")
echo ""

# ── 阶段 3: 数值转换 ──
echo "【阶段 3】数值 ↔ 64进制"
echo ""

let test_numbers = @[0, 1, 63, 64, 128, 255, 1024, 4096, 2024, 65535]

for n in test_numbers:
  let phase = to_phase64(n)
  let back = from_phase64(phase)
  echo "  " & $n & " → " & phase & " → " & $back & " " & (if n == back: "✅" else: "❌")
echo ""

# ── 阶段 4: 编码报告 ──
echo "【阶段 4】详细编码报告"
echo ""

echo encode_report("满全法")
echo ""

# ── 阶段 5: 批量验证 ──
echo "【阶段 5】批量验证（100个Unicode字符）"
echo ""

var all_ok = true
var fail_count = 0
# 使用有效Unicode范围：CJK统一表意文字 0x4E00..0x4E63
for i in 0..<100:
  let ch = $Rune(0x4E00 + i)
  if not verify_utf8(ch):
    all_ok = false
    fail_count += 1
    if fail_count <= 5:
      echo "  ❌ 失败: U+" & toHex(0x4E00 + i) & " → " & ch

if all_ok:
  echo "  ✅ 100个Unicode字符全部编码/解码验证通过"
else:
  echo "  ❌ 失败: " & $fail_count & " / 100"
echo ""

# ── 阶段 6: 与传统base64对比 ──
echo "【阶段 6】与传统base64的对比"
echo ""

let sample = "满全法=袋子+加法"
let our_encoded = encode_utf8(sample)

echo "  原文: " & sample
echo "  64进制编码: " & our_encoded
echo "  编码符号数: " & $len(to_seq(our_encoded.runes())) & " 个"
echo "  每个符号: 有独立含义（满/全/袋/米/加/减...）"
echo "  传统base64: a-z A-Z 0-9 + / （无独立含义）"
echo ""

echo "═══════════════════════════════════════════"
echo "  你的64进制：每个符号都有含义"
echo "  不是乱码，是可读的符号系统"
echo "═══════════════════════════════════════════"
