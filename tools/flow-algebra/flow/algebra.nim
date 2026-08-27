# ============================================================
# 流代数 · 核心引擎
#
# 12位流属性操作 + 八流体系 + 解释流牺牲机制 + 一维时间线性
# ============================================================

import tables
import strutils
import math
import options
import ./types

# ── 属性位操作 ──

# 获取属性对应的位索引
proc prop_bit*(p: FlowProperty): int =
  case p
  of reachable: 0
  of continuable: 1
  of incr: 2
  of stoppable: 3
  of reversible: 4
  of recoverable: 5
  of circulable: 6
  of inheritable: 7
  of trusted: 8
  of ackable: 9
  of transmissible: 10
  of advanceable: 11

# 设置属性位
proc set_prop*(props: var uint16, p: FlowProperty) =
  props = props or uint16(1 shl prop_bit(p))

# 清除属性位
proc clear_prop*(props: var uint16, p: FlowProperty) =
  props = props and (not uint16(1 shl prop_bit(p)))

# 检查属性位
proc has_prop*(props: uint16, p: FlowProperty): bool =
  return (props and uint16(1 shl prop_bit(p))) != 0

# 属性名称
proc prop_name*(p: FlowProperty): string =
  case p
  of reachable: "REACHABLE"
  of continuable: "CONTINUABLE"
  of incr: "INCR"
  of stoppable: "STOPPABLE"
  of reversible: "REVERSIBLE"
  of recoverable: "RECOVERABLE"
  of circulable: "CIRCULABLE"
  of inheritable: "INHERITABLE"
  of trusted: "TRUSTED"
  of ackable: "ACKABLE"
  of transmissible: "TRANSMISSIBLE"
  of advanceable: "ADVANCEABLE"

# ── 流类型名称 ──
proc flow_type_name*(t: FlowType): string =
  case t
  of preloadFlow: "预加载流"
  of categoryFlow: "范畴流"
  of clusterFlow: "聚类流"
  of classifyFlow: "分类流"
  of propertyFlow: "属性流"
  of interpretiveFlow: "解释流"
  of relationFlow: "关系流"
  of conceptFlow: "概念流"

# ── 流状态名称 ──
proc flow_state_name*(s: FlowState): string =
  case s
  of flowActive: "活跃"
  of flowBlocked: "阻塞"
  of flowSacrificing: "牺牲中"
  of flowAcked: "已承认"
  of flowBroken: "断裂"
  of flowDormant: "休眠"

# ── 时间步名称 ──
proc step_name*(s: TimeStep): string =
  case s
  of stepInput: "输入"
  of stepPerception: "感知"
  of stepDistinguish: "分别"
  of stepLookup: "查字典"
  of stepTraverse: "走解释流"
  of stepCluster: "聚类"
  of stepAttention: "注意力"
  of stepArrival: "抵达"

# ── 创建流代数引擎 ──
proc new_flow_algebra*(): FlowAlgebra =
  return FlowAlgebra(
    flows: @[],
    flow_count: 0,
    sacrifices: @[],
    sacrifice_count: 0,
    type_index: init_table[FlowType, seq[int]](),
    state_index: init_table[FlowState, seq[int]]()
  )

# ── 添加流 ──
proc add_flow*(fa: var FlowAlgebra, entry: FlowEntry): int =
  let idx = fa.flows.len
  fa.flows.add(entry)
  fa.flow_count += 1
  
  # 更新类型索引
  if not fa.type_index.hasKey(entry.flow_type):
    fa.type_index[entry.flow_type] = @[]
  fa.type_index[entry.flow_type].add(idx)
  
  # 更新状态索引
  if not fa.state_index.hasKey(entry.state):
    fa.state_index[entry.state] = @[]
  fa.state_index[entry.state].add(idx)
  
  return idx

# ── 创建标准流 ──
proc create_flow*(fa: var FlowAlgebra, name: string, ft: FlowType, source, target: string, props: seq[FlowProperty]): int =
  var prop_bits: uint16 = 0
  for p in props:
    set_prop(prop_bits, p)
  
  let ackable = has_prop(prop_bits, ackable) and has_prop(prop_bits, reachable)
  
  let entry = FlowEntry(
    name: name,
    flow_type: ft,
    state: flowActive,
    properties: prop_bits,
    source: source,
    target: target,
    carrier: "",
    ackable: ackable,
    description: flow_type_name(ft) & ": " & source & " → " & target
  )
  
  return fa.add_flow(entry)

# ── 解释流牺牲：mlct → mltt ──
proc sacrifice*(fa: var FlowAlgebra, mlct_name: string, mltt_name: string, cost: float): SacrificeRecord =
  # 找到对应的流
  var mlct_idx = -1
  var mltt_idx = -1
  
  for i in 0..<fa.flows.len:
    if fa.flows[i].name == mlct_name:
      mlct_idx = i
    if fa.flows[i].name == mltt_name:
      mltt_idx = i
  
  var success = false
  if mlct_idx >= 0 and mltt_idx >= 0:
    # 解释流牺牲自身（mlct → flowBroken）
    fa.flows[mlct_idx].state = flowBroken
    # 关系流被激活（mltt → flowAcked）
    fa.flows[mltt_idx].state = flowAcked
    fa.flows[mltt_idx].ackable = true
    success = true
  
  let record = SacrificeRecord(
    mlct: mlct_name,
    mltt: mltt_name,
    cost: clamp(cost, 0.0, 1.0),
    success: success,
    timestamp: fa.sacrifice_count
  )
  
  fa.sacrifices.add(record)
  fa.sacrifice_count += 1
  return record

# ── 判断流能否被路由 ──
proc can_route*(fa: FlowAlgebra, idx: int): bool =
  if idx < 0 or idx >= fa.flows.len:
    return false
  let f = fa.flows[idx]
  return has_prop(f.properties, reachable) and has_prop(f.properties, ackable)

# ── 判断流能否构成演化 ──
proc can_evolve*(fa: FlowAlgebra, idx: int): bool =
  if idx < 0 or idx >= fa.flows.len:
    return false
  let f = fa.flows[idx]
  return has_prop(f.properties, continuable) and 
         has_prop(f.properties, incr) and 
         has_prop(f.properties, transmissible)

# ── 判断流是否是活的 ──
proc is_alive*(fa: FlowAlgebra, idx: int): bool =
  if idx < 0 or idx >= fa.flows.len:
    return false
  let f = fa.flows[idx]
  return has_prop(f.properties, reachable) and 
         has_prop(f.properties, continuable) and 
         has_prop(f.properties, ackable)

# ── 一维时间线性推进 ──
proc time_step_name*(step: TimeStep): string =
  case step
  of stepInput: "Input"
  of stepPerception: "Perception"
  of stepDistinguish: "Distinguish"
  of stepLookup: "Lookup"
  of stepTraverse: "Traverse"
  of stepCluster: "Cluster"
  of stepAttention: "Attention"
  of stepArrival: "Arrival"

# ── 按类型获取流 ──
proc flows_by_type*(fa: FlowAlgebra, ft: FlowType): seq[FlowEntry] =
  result = @[]
  if fa.type_index.hasKey(ft):
    for idx in fa.type_index[ft]:
      if idx < fa.flows.len:
        result.add(fa.flows[idx])

# ── 按状态获取流 ──
proc flows_by_state*(fa: FlowAlgebra, fs: FlowState): seq[FlowEntry] =
  result = @[]
  if fa.state_index.hasKey(fs):
    for idx in fa.state_index[fs]:
      if idx < fa.flows.len:
        result.add(fa.flows[idx])

# ── 统计 ──
proc stats*(fa: FlowAlgebra): string =
  var s = "═══ 流代数统计 ═══\n"
  s &= "流总数: " & $fa.flow_count & "\n"
  s &= "牺牲次数: " & $fa.sacrifice_count & "\n"
  
  # 按类型统计
  s &= "\n按类型:\n"
  for ft in FlowType:
    let count = fa.flows_by_type(ft).len
    if count > 0:
      s &= "  " & flow_type_name(ft) & ": " & $count & "\n"
  
  # 按状态统计
  s &= "\n按状态:\n"
  for fs in FlowState:
    let count = fa.flows_by_state(fs).len
    if count > 0:
      s &= "  " & flow_state_name(fs) & ": " & $count & "\n"
  
  return s

# ── 流属性报告 ──
proc flow_report*(fa: FlowAlgebra, idx: int): string =
  if idx < 0 or idx >= fa.flows.len:
    return "流不存在"
  
  let f = fa.flows[idx]
  var s = "═══ 流报告 ═══\n"
  s &= "名称: " & f.name & "\n"
  s &= "类型: " & flow_type_name(f.flow_type) & "\n"
  s &= "状态: " & flow_state_name(f.state) & "\n"
  s &= "源: " & f.source & " → 目标: " & f.target & "\n"
  s &= "可路由: " & (if fa.can_route(idx): "✅" else: "❌") & "\n"
  s &= "可演化: " & (if fa.can_evolve(idx): "✅" else: "❌") & "\n"
  s &= "活的: " & (if fa.is_alive(idx): "✅" else: "❌") & "\n"
  s &= "属性位: " & $f.properties & "\n"
  s &= "激活属性:\n"
  for p in FlowProperty:
    if has_prop(f.properties, p):
      s &= "  ✅ " & prop_name(p) & "\n"
  
  return s
