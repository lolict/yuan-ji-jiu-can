# ============================================================
# 天庭空间 · 核心引擎
#
# 四模型复合实现：
#   吸引子：系统趋向的点，极性分阴阳
#   叠加态：多态共存，相干度决定坍缩概率
#   纤维丛：空间随底空间变化，联络决定平行移动
#   Topos：亚对象分类器，非二元真值
# ============================================================

import tables
import math
import options
import ./types

# ── 创建天庭空间 ──
proc new_heaven*(): HeavenSpace =
  var h = HeavenSpace(
    attractors: @[],
    superpositions: @[],
    bundles: @[],
    topos: Topos(
      truth_values: init_table[string, float](),
      subobject_classifier: 0.5,
      pullbacks: 0,
      pushouts: 0
    ),
    dimension: 3,
    volume: 1.0,
    attractor_count: 0,
    superposition_count: 0,
    bundle_count: 0
  )
  
  # Topos 初始真值（非二元逻辑）
  h.topos.truth_values["真"] = 1.0
  h.topos.truth_values["假"] = 0.0
  h.topos.truth_values["可能"] = 0.5
  h.topos.truth_values["未知"] = -0.1
  
  return h

# ── 吸引子操作 ──

# 添加吸引子
proc add_attractor*(h: var HeavenSpace, name: string, px, py, pz, phase, strength, radius: float, polarity: float): Attractor =
  let a = Attractor(
    name: name,
    position: PhaseCoord(x: px, y: py, z: pz, phase: phase),
    strength: clamp(strength, 0.0, 1.0),
    radius: radius,
    polarity: clamp(polarity, -1.0, 1.0)
  )
  h.attractors.add(a)
  h.attractor_count += 1
  return a

# 计算点到吸引子的引力
proc attractor_force*(h: HeavenSpace, px, py, pz: float, idx: int): float =
  if idx < 0 or idx >= h.attractors.len:
    return 0.0
  let a = h.attractors[idx]
  let dx = px - a.position.x
  let dy = py - a.position.y
  let dz = pz - a.position.z
  let dist = sqrt(dx*dx + dy*dy + dz*dz)
  if dist < 0.001:
    return a.strength
  if dist > a.radius:
    return 0.0
  return a.strength * (1.0 - dist / a.radius)

# 找到最近的吸引子
proc nearest_attractor*(h: HeavenSpace, px, py, pz: float): Option[(int, float)] =
  if h.attractors.len == 0:
    return none((int, float))
  
  var best_idx = 0
  var best_dist = high(float)
  
  for i in 0..<h.attractors.len:
    let a = h.attractors[i]
    let dx = px - a.position.x
    let dy = py - a.position.y
    let dz = pz - a.position.z
    let dist = sqrt(dx*dx + dy*dy + dz*dz)
    if dist < best_dist:
      best_dist = dist
      best_idx = i
  
  return some((best_idx, best_dist))

# ── 叠加态操作 ──

# 计算标准差（前置定义）
proc stddev*(values: seq[float]): float =
  if values.len == 0:
    return 0.0
  var sum = 0.0
  for v in values:
    sum += v
  let mean = sum / float(values.len)
  var variance = 0.0
  for v in values:
    variance += (v - mean) * (v - mean)
  return sqrt(variance / float(values.len))

# 创建叠加态
proc add_superposition*(h: var HeavenSpace, states: seq[float], phases: seq[float]): Superposition =
  let coherence = 1.0 - stddev(states)
  let s = Superposition(
    states: states,
    phases: phases,
    coherence: clamp(coherence, 0.0, 1.0),
    count: states.len
  )
  h.superpositions.add(s)
  h.superposition_count += 1
  return s

# 叠加态坍缩（观测时）
proc collapse*(s: Superposition): int =
  if s.states.len == 0:
    return -1
  # 概率最大的态坍缩
  var max_idx = 0
  var max_prob = s.states[0]
  for i in 1..<s.states.len:
    if s.states[i] > max_prob:
      max_prob = s.states[i]
      max_idx = i
  return max_idx

# ── 纤维丛操作 ──

# 添加纤维丛点
proc add_bundle*(h: var HeavenSpace, px, py, pz, phase: float, fiber: seq[float], connection: float): FiberBundle =
  let b = FiberBundle(
    base_point: PhaseCoord(x: px, y: py, z: pz, phase: phase),
    fiber: fiber,
    connection: connection,
    curvature: 1.0 - abs(connection)
  )
  h.bundles.add(b)
  h.bundle_count += 1
  return b

# 平行移动：沿底空间移动时纤维的变换
proc parallel_transport*(b: FiberBundle, displacement: float): float =
  return b.connection * displacement

# ── Topos 操作 ──

# 亚对象分类器判断
proc classify*(h: HeavenSpace, value: float): string =
  if value > 0.8:
    return "真"
  elif value > 0.3:
    return "可能"
  elif value > -0.1:
    return "未知"
  else:
    return "假"

# 非二元合取（min）
proc topos_and*(h: HeavenSpace, a, b: float): float =
  return min(a, b)

# 非二元析取（max）
proc topos_or*(h: HeavenSpace, a, b: float): float =
  return max(a, b)

# 非二元蕴含（Gödel）
proc topos_implies*(h: HeavenSpace, a, b: float): float =
  if a <= b:
    return 1.0
  else:
    return b

# ── 空间统计 ──
proc stats*(h: HeavenSpace): string =
  var s = "═══ 天庭空间统计 ═══\n"
  s &= "维度: " & $h.dimension & "D\n"
  s &= "吸引子数: " & $h.attractor_count & "\n"
  s &= "叠加态数: " & $h.superposition_count & "\n"
  s &= "纤维丛数: " & $h.bundle_count & "\n"
  s &= "Topos Ω: " & $h.topos.subobject_classifier & "\n"
  return s

# ── 吸引子列表 ──
proc attractor_list*(h: HeavenSpace): string =
  var s = "═══ 吸引子列表 ═══\n"
  for i in 0..<h.attractors.len:
    let a = h.attractors[i]
    let pol = if a.polarity > 0.3: "阳" elif a.polarity < -0.3: "阴" else: "平"
    s &= "  [" & $i & "] " & a.name & " 强度=" & $a.strength & " 极性=" & pol & "\n"
  return s
