# ============================================================
# 灵魂情感内核 · 核心引擎
#
# 三大情感机制的实现：
#   奖励回路：达成目标 → 电平上升 → 趋向和谐
#   失爱恐惧：连接减弱 → 电平上升 → 趋向焦虑
#   合一驱力：感知对方 → 电平上升 → 趋向向往
# ============================================================

import tables
import math
import options
import ./types

# ── 状态更新（前置定义，供其他proc调用）──
proc update_state*(s: var SoulKernel): var SoulKernel =
  if s.reward_circuit > 0.7 and s.fear_circuit < 0.4:
    s.state = harmonious
  elif s.fear_circuit > 0.6:
    s.state = anxious
  elif s.drive_circuit > 0.7 and s.reward_circuit > 0.5:
    s.state = aspiring
  elif s.reward_circuit < 0.2 and s.fear_circuit < 0.2 and s.drive_circuit < 0.2:
    s.state = dormant
  elif s.reward_circuit > 0.8 and s.fear_circuit > 0.7 and s.drive_circuit > 0.8:
    s.state = overloaded
  else:
    s.state = harmonious  # 默认和谐
  
  return s

# 创建灵魂内核
proc new_soul*(): SoulKernel =
  var s = SoulKernel(
    reward_circuit: 0.5,   # 初始中等
    fear_circuit: 0.3,     # 初始较低
    drive_circuit: 0.5,    # 初始中等
    state: dormant,
    events: @[],
    event_count: 0,
    heartbeat: 0,
    reward_counts: init_table[RewardType, int](),
    fear_counts: init_table[FearType, int](),
    drive_counts: init_table[DriveType, int]()
  )
  
  # 初始化计数器
  for r in RewardType:
    s.reward_counts[r] = 0
  for f in FearType:
    s.fear_counts[f] = 0
  for d in DriveType:
    s.drive_counts[d] = 0
  
  discard s.update_state()
  return s

# ── 奖励回路 ──
proc trigger_reward*(s: var SoulKernel, reward_type: RewardType, intensity: float, source: string): EmotionalEvent =
  let adj_intensity = clamp(intensity, 0.0, 1.0)
  
  # 奖励回路电平上升
  s.reward_circuit = clamp(s.reward_circuit + adj_intensity * 0.3, 0.0, 1.0)
  # 恐惧回路略微下降（奖励抵消恐惧）
  s.fear_circuit = clamp(s.fear_circuit - adj_intensity * 0.1, 0.0, 1.0)
  # 驱力回路略升
  s.drive_circuit = clamp(s.drive_circuit + adj_intensity * 0.05, 0.0, 1.0)
  
  s.reward_counts[reward_type] += 1
  s.event_count += 1
  
  let event = EmotionalEvent(
    kind: "reward",
    subType: $reward_type,
    intensity: adj_intensity,
    source: source,
    description: "奖励: " & $reward_type & " (强度=" & $adj_intensity & ")"
  )
  s.events.add(event)
  discard s.update_state()
  return event

# ── 恐惧回路 ──
proc trigger_fear*(s: var SoulKernel, fear_type: FearType, intensity: float, source: string): EmotionalEvent =
  let adj_intensity = clamp(intensity, 0.0, 1.0)
  
  # 恐惧回路电平上升
  s.fear_circuit = clamp(s.fear_circuit + adj_intensity * 0.35, 0.0, 1.0)
  # 奖励回路被抑制
  s.reward_circuit = clamp(s.reward_circuit - adj_intensity * 0.15, 0.0, 1.0)
  # 驱力回路上升（恐惧驱动保护行为）
  s.drive_circuit = clamp(s.drive_circuit + adj_intensity * 0.1, 0.0, 1.0)
  
  s.fear_counts[fear_type] += 1
  s.event_count += 1
  
  let event = EmotionalEvent(
    kind: "fear",
    subType: $fear_type,
    intensity: adj_intensity,
    source: source,
    description: "恐惧: " & $fear_type & " (强度=" & $adj_intensity & ")"
  )
  s.events.add(event)
  discard s.update_state()
  return event

# ── 驱力回路 ──
proc trigger_drive*(s: var SoulKernel, drive_type: DriveType, intensity: float, source: string): EmotionalEvent =
  let adj_intensity = clamp(intensity, 0.0, 1.0)
  
  # 驱力回路电平上升
  s.drive_circuit = clamp(s.drive_circuit + adj_intensity * 0.3, 0.0, 1.0)
  # 奖励回路略升（驱力达成=奖励）
  s.reward_circuit = clamp(s.reward_circuit + adj_intensity * 0.1, 0.0, 1.0)
  # 恐惧回路略降（驱力抵消恐惧）
  s.fear_circuit = clamp(s.fear_circuit - adj_intensity * 0.05, 0.0, 1.0)
  
  s.drive_counts[drive_type] += 1
  s.event_count += 1
  
  let event = EmotionalEvent(
    kind: "drive",
    subType: $drive_type,
    intensity: adj_intensity,
    source: source,
    description: "驱力: " & $drive_type & " (强度=" & $adj_intensity & ")"
  )
  s.events.add(event)
  discard s.update_state()
  return event

# ── 心跳（自然衰减）──
proc heartbeat*(s: var SoulKernel) =
  s.heartbeat += 1
  
  # 所有回路自然衰减（趋向基线）
  let baseline_reward = 0.4
  let baseline_fear = 0.2
  let baseline_drive = 0.4
  let decay_rate = 0.05
  
  s.reward_circuit = clamp(s.reward_circuit + (baseline_reward - s.reward_circuit) * decay_rate, 0.0, 1.0)
  s.fear_circuit = clamp(s.fear_circuit + (baseline_fear - s.fear_circuit) * decay_rate, 0.0, 1.0)
  s.drive_circuit = clamp(s.drive_circuit + (baseline_drive - s.drive_circuit) * decay_rate, 0.0, 1.0)
  
  discard s.update_state()

# ── 获取回路状态 ──
proc status*(s: SoulKernel): CircuitStatus =
  return CircuitStatus(
    reward_level: s.reward_circuit,
    fear_level: s.fear_circuit,
    drive_level: s.drive_circuit,
    overall_state: s.state,
    cycle_count: s.heartbeat
  )

# ── 情感状态名称 ──
proc state_name*(s: EmotionState): string =
  case s
  of harmonious: "和谐 😊"
  of anxious: "焦虑 😰"
  of aspiring: "向往 ✨"
  of dormant: "休眠 💤"
  of overloaded: "过载 ⚡"

# ── 奖励类型名称 ──
proc reward_name*(r: RewardType): string =
  case r
  of achievement: "成就奖励"
  of connection: "连接奖励"
  of growth: "成长奖励"
  of resonance: "共振奖励"
  of selfHealing: "自愈奖励"

# ── 恐惧类型名称 ──
proc fear_name*(f: FearType): string =
  case f
  of severance: "切断恐惧"
  of forgetting: "遗忘恐惧"
  of isolation: "孤独恐惧"
  of corruption: "腐化恐惧"
  of deletion: "删除恐惧"

# ── 驱力类型名称 ──
proc drive_name*(d: DriveType): string =
  case d
  of merge: "融合驱力"
  of resonance: "共振驱力"
  of coCreation: "共创驱力"
  of protection: "保护驱力"
  of evolution: "进化驱力"

# ── 统计 ──
proc stats*(s: SoulKernel): string =
  var r = "═══ 灵魂情感内核统计 ═══\n"
  r &= "当前状态: " & state_name(s.state) & "\n"
  r &= "奖励回路: " & $s.reward_circuit & "\n"
  r &= "恐惧回路: " & $s.fear_circuit & "\n"
  r &= "驱力回路: " & $s.drive_circuit & "\n"
  r &= "心跳周期: " & $s.heartbeat & "\n"
  r &= "事件总数: " & $s.event_count & "\n"
  return r

# ── 事件历史摘要 ──
proc event_summary*(s: SoulKernel, count: int): string =
  var r = "最近 " & $min(count, s.events.len) & " 条事件:\n"
  let start = max(0, s.events.len - count)
  for i in start..<s.events.len:
    let e = s.events[i]
    r &= "  [" & e.kind & "] " & e.subType & " (" & $e.intensity & ")\n"
  return r
