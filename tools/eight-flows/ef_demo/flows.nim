# ============================================================
# 八流统一 · 核心引擎
#
# 实现一维时间线性链，八流按序推进
# 闭环条件: target.ackable() == true
# ============================================================

import math
import strutils
import sequtils
import types

const FLOW_NAMES: array[EightFlow, string] = [
  "预加载流", "范畴流", "聚类流", "分类流",
  "属性流", "解释流", "关系流", "概念流"
]

const FLOW_GLYPHS: array[EightFlow, string] = [
  "📚", "🔷", "🎯", "🔍", "⚡", "💡", "🔗", "🧠"
]

proc new_universe*(): FlowUniverse =
  var uni = FlowUniverse(
    current: efPreload,
    glyph_registry: @[],
    concept_count: 0
  )
  for f in EightFlow:
    uni.states[f] = FlowState(
      phase: f,
      name: FLOW_NAMES[f],
      input_ready: f == efPreload,
      output_ready: false,
      carry: @[],
      polarity: 0,
      offset_rate: float(ord(f)) * 0.1,
      acked: false
    )
  result = uni

proc do_preload(uni: var FlowUniverse; input: string): string =
  var bits: seq[char] = @[]
  for c in input:
    bits.add(if int(c) mod 2 == 0: '0' else: '1')
  uni.glyph_registry.add(input)
  result = "字典已加载: " & input & " → [" & join(bits, "") & "]"
  uni.states[efPreload].output_ready = true

proc do_category(uni: var FlowUniverse; input: string): string =
  var cat_count = 0
  for c in input:
    discard case c
      of 'a'..'z': "小写"
      of '0'..'9': "数字"
      else: "符号"
    cat_count += 1
  result = "范畴映射: " & $cat_count & " 类型"
  uni.states[efCategory].output_ready = true

proc do_cluster(uni: var FlowUniverse; input: string): string =
  var k = max(1, int(sqrt(float(input.len))))
  result = "聚类 K=" & $k & " · 文档=" & $input.len & "字符"
  uni.states[efCluster].output_ready = true

proc do_classify(uni: var FlowUniverse; input: string): string =
  var family = "未知"
  if input.len > 0:
    let fc = int(input[0])
    family = case fc mod 4
      of 0: "容器族(模=0)"
      of 1: "操作族(模=1)"
      of 2: "极性族(模=2)"
      else: "产权族(模=3)"
  result = "感知家族: " & family
  uni.states[efClassify].output_ready = true

proc do_attribute(uni: var FlowUniverse; input: string): string =
  var bits: uint16 = 0
  if input.len > 0:   bits = bits or 0x0001
  if input.len > 4:   bits = bits or 0x0002
  if input.len > 8:   bits = bits or 0x0004
  if input.len < 20:  bits = bits or 0x0008
  if input.len mod 2 == 0: bits = bits or 0x0010
  if input.len mod 3 == 0: bits = bits or 0x0020
  if input.len mod 4 == 0: bits = bits or 0x0040
  result = "属性=0x" & toHex(int(bits), 4)
  uni.states[efAttribute].output_ready = true

proc do_interpret(uni: var FlowUniverse; input: string): string =
  let own_encoded = "mlct:" & $int(input.len)
  let target = "mltt:" & input
  uni.states[efInterpret].carry = @[byte(input.len)]
  result = "解释流牺牲自身(" & own_encoded & ")→产关系流→路由目标=" & target
  uni.states[efInterpret].output_ready = true

proc do_relational(uni: var FlowUniverse; input: string): string =
  let carried = uni.states[efInterpret].carry
  let ackable = carried.len > 0
  if ackable:
    uni.states[efRelational].acked = true
    result = "路由至目标 → ack=true ✅ 闭环成立"
  else:
    result = "路由失败 → ack=false ❌ 闭环断裂"
  uni.states[efRelational].output_ready = ackable

proc do_concept(uni: var FlowUniverse; input: string): string =
  if uni.states[efCategory].output_ready and
     uni.states[efCluster].output_ready and
     uni.states[efInterpret].output_ready:
    uni.concept_count += 1
    result = "概念涌现 #" & $uni.concept_count & " ← 范畴×聚类×解释"
    uni.states[efConcept].output_ready = true
  else:
    result = "条件不足，无法产生概念"

proc run_timeline*(uni: var FlowUniverse; glyph: string): seq[TimeNode] =
  var steps: seq[TimeNode] = @[]
  var data = glyph
  let r1 = uni.do_preload(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efPreload, glyph: glyph, result: r1, properties: 0x0001.FlowPropertyBits))
  data = glyph
  let r2 = uni.do_category(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efCategory, glyph: glyph, result: r2, properties: 0x0003.FlowPropertyBits))
  let r3 = uni.do_cluster(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efCluster, glyph: glyph, result: r3, properties: 0x0007.FlowPropertyBits))
  let r4 = uni.do_classify(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efClassify, glyph: glyph, result: r4, properties: 0x000F.FlowPropertyBits))
  let r5 = uni.do_attribute(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efAttribute, glyph: glyph, result: r5, properties: 0x00FF.FlowPropertyBits))
  let r6 = uni.do_interpret(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efInterpret, glyph: glyph, result: r6, properties: 0x01FF.FlowPropertyBits))
  data = "sacrificed(" & glyph & ")"
  let r7 = uni.do_relational(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efRelational, glyph: glyph, result: r7, properties: 0x03FF.FlowPropertyBits))
  let r8 = uni.do_concept(data)
  steps.add(TimeNode(timestamp: len(steps), flow: efConcept, glyph: glyph, result: r8, properties: 0x0FFF.FlowPropertyBits))
  uni.timeline = uni.timeline & steps
  result = steps

proc run_batch*(universe: var FlowUniverse; glyphs: seq[string]): int =
  result = 0
  for g in glyphs:
    let steps = universe.run_timeline(g)
    if steps.len > 0 and steps[^1].flow == efConcept:
      result += 1

proc `$`*(f: EightFlow): string = FLOW_NAMES[f]
proc flow_glyph*(f: EightFlow): string = FLOW_GLYPHS[f]
proc get_flow_name*(f: EightFlow): string = FLOW_NAMES[f]

proc property_names*(bits: FlowPropertyBits): seq[string] =
  var b = uint16(bits)
  if (b and 0x0001) != 0: result.add("REACHABLE")
  if (b and 0x0002) != 0: result.add("CONTINUABLE")
  if (b and 0x0004) != 0: result.add("INCR")
  if (b and 0x0008) != 0: result.add("STOPPABLE")
  if (b and 0x0010) != 0: result.add("REVERSIBLE")
  if (b and 0x0020) != 0: result.add("RECOVERABLE")
  if (b and 0x0040) != 0: result.add("CIRCULABLE")
  if (b and 0x0080) != 0: result.add("INHERITABLE")
  if (b and 0x0100) != 0: result.add("TRUSTED")
  if (b and 0x0200) != 0: result.add("ACKABLE")
  if (b and 0x0400) != 0: result.add("TRANSMISSIBLE")
  if (b and 0x0800) != 0: result.add("ADVANCEABLE")
