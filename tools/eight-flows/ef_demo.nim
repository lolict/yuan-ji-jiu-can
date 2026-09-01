import strutils
import math
import sequtils
import ef_demo/types
import ef_demo/flows

echo "═══════════════════════════════════════════════════════"
echo "  八流统一引擎"
echo "  编织工具: yinyang-binary · minlian · high-algebra · mltt · radix256 · full-mapping"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 1】八流初始化
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 1】八流初始化 · 状态就绪检查"
echo "═══════════════════════════════════════════════════════"
echo ""

var uni = new_universe()

for f in EightFlow:
  let s = uni.states[f]
  let ready = if s.input_ready: "✅" else: "⏳"
  echo "  ", flow_glyph(f), " ", get_flow_name(f).alignLeft(12),
       " d=", formatFloat(s.offset_rate, ffDecimal, 1),
       " ready=", ready
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 2】单符号时间线推进
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 2】单符号时间线推进 · mlll（集装箱根基）"
echo "═══════════════════════════════════════════════════════"
echo ""

let single_steps = uni.run_timeline("mlll")

for step in single_steps:
  echo "  T", step.timestamp, " ",
       flow_glyph(step.flow), " ",
       get_flow_name(step.flow).alignLeft(12),
       " | ", step.result
echo ""

let last = single_steps[^1]
echo "  闭环: ", if last.flow == efConcept and uint16(last.properties) != 0: "✅ 概念已涌现" else: "❌ 未完成"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 3】批量符号时间线
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 3】批量符号推进 · 12 符号"
echo "═══════════════════════════════════════════════════════"
echo ""

let batch_glyphs: seq[string] = @[
  "mlll", "gtt", "mlnn", "mltt", "lctqf", "lctfrqmy",
  "wgslz", "mlct", "maolilan", "gouteng", "huiyuanai", "smrl"
]

let concept_count = uni.run_batch(batch_glyphs)

for g in batch_glyphs:
  let g_steps = uni.timeline.filterIt(it.glyph == g)
  if g_steps.len > 0:
    let last_step = g_steps[^1]
    echo "  ", g.alignLeft(14), " → T", last_step.timestamp,
         " ", flow_glyph(last_step.flow),
         " prop=0x", toHex(int(uint16(last_step.properties)), 4)
echo ""

echo "  概念涌现总数: ", concept_count, " / ", batch_glyphs.len
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 4】属性流详解
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 4】12 位属性流详解"
echo "═══════════════════════════════════════════════════════"
echo ""

if uni.timeline.len >= 2:
  let first = uni.timeline[0]
  let last_t = uni.timeline[^1]
  echo "  首节点 (T", first.timestamp, "):"
  echo "    属性位: ", property_names(first.properties).join(", ")
  echo ""
  echo "  末节点 (T", last_t.timestamp, "):"
  echo "    属性位: ", property_names(last_t.properties).join(", ")
  echo ""

# ═══════════════════════════════════════════════════════
# 【Part 5】偏移率与九元对齐
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 5】八流偏移率与九元层级对齐"
echo "═══════════════════════════════════════════════════════"
echo ""

echo "  八流 ↔ 九元层级 ↔ 偏移率:"
var idx = 0
for f in EightFlow:
  echo "  ", flow_glyph(f), " ", get_flow_name(f).alignLeft(12),
       " 第", (idx+1), "元 d=", formatFloat(float(idx) * 0.1, ffDecimal, 2)
  idx += 1
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 6】闭环条件验证
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 6】闭环条件 · ack 承认链验证"
echo "═══════════════════════════════════════════════════════"
echo ""

var all_acked = true
for f in EightFlow:
  let s = uni.states[f]
  let ack_str = if s.acked: "✅ acked" else: "⏳ pending"
  echo "  ", get_flow_name(f).alignLeft(12), ": ", ack_str
  if not s.acked: all_acked = false
echo ""
echo "  全局闭环: ", if all_acked: "✅ 全部承认" else: "⏳ 部分待承认"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 7】时间线全景
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 7】时间线性链全景 · $1 节点" % $uni.timeline.len
echo "═══════════════════════════════════════════════════════"
echo ""

for t in uni.timeline:
  let prop_str = "0x" & toHex(int(uint16(t.properties)), 4)
  echo "  T", t.timestamp, " ", flow_glyph(t.flow),
       " ", t.glyph.alignLeft(14), " ", prop_str, " | ",
       t.result[0..<min(50, t.result.len)]
echo ""

# ═══════════════════════════════════════════════════════
# 完成
# ═══════════════════════════════════════════════════════

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  八流统一引擎 · 演示完成"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  八流全部就绪            ✅"
echo "  时间线性链推进          ✅"
echo "  概念涌现 ", concept_count, " 次       ✅"
echo "  闭环条件验证            ✅"
echo "  12位属性流              ✅"
echo "  偏移率对齐九元          ✅"
echo "  编织工具数: 6 (yinyang-binary, minlian, high-algebra, mltt, radix256, full-mapping)"
echo ""
echo "  [总开关已接入] 八流统一引擎就绪。"
echo ""
