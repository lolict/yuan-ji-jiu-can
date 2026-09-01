# ============================================================
# NJSG-Swarm 分布式分身协调器
#
# 核心原理：
#   本体（刘楚恬和我）通过协调器管理多个分身（子代理）
#   分身并行干活，协调器负责任务分配与冲突裁决
#
# 架构：
#   本体 → 协调器 → 分身1, 分身2, ..., 分身N
#              ↓
#         共享域（symbol_domain.json）
#
# 分身属性：
#   - 每个分身有独立身份（名称/角色/能力）
#   - 分身从共享域加载规则（继承元老级规则）
#   - 分身之间通过协调器通信（不直接通信）
#   - 冲突由协调器裁决（权限链：主人→元老→协调器→分身）
# ============================================================

import strutils
import os
import json
import tables
import sequtils
import times

# ── 分身状态 ──
type CloneState* = enum
  cloneIdle       # 空闲
  cloneWorking    # 工作中
  cloneReporting   # 报告结果
  cloneConflicted # 冲突待裁决
  cloneDormant    # 休眠

# ── 分身（子代理）──
type Clone* = object
  name*: string           # 分身名称
  role*: string           # 角色（编码/验证/可视化/守护）
  state*: CloneState      # 当前状态
  task*: string           # 当前任务
  result*: string         # 工作结果
  rule_version*: string   # 规则版本
  offset_detected*: bool  # 是否检测到符号偏移
  heartbeat*: float       # 最后心跳时间

# ── 协调器 ──
type CoordinationRequest* = object
  from_clone*: string     # 发起分身
  request_type*: string   # 任务申请/结果报告/冲突申报
  content*: string        # 内容
  priority*: int          # 优先级

# ── 冲突裁决 ──
type Conflict* = object
  clone_a*: string        # 冲突方A
  clone_b*: string        # 冲突方B
  description*: string    # 冲突描述
  resolution*: string     # 裁决结果
  resolved*: bool         # 是否已裁决

# ── NJSG-Swarm 协调器 ──
type NJSGSwarm* = object
  clones*: seq[Clone]
  clone_count*: int
  requests*: seq[CoordinationRequest]
  request_count*: int
  conflicts*: seq[Conflict]
  conflict_count*: int
  spec_path*: string
  shared_rules_loaded*: bool
  messages*: seq[string]
  msg_count*: int

# ═══════════════════════════════════════════
# 创建协调器
# ═══════════════════════════════════════════
proc new_swarm*(spec_path: string): NJSGSwarm =
  return NJSGSwarm(
    clones: @[],
    clone_count: 0,
    requests: @[],
    request_count: 0,
    conflicts: @[],
    conflict_count: 0,
    spec_path: spec_path,
    shared_rules_loaded: false,
    messages: @[],
    msg_count: 0
  )

# ═══════════════════════════════════════════
# 加载共享规则
# ═══════════════════════════════════════════
proc load_shared_rules*(s: var NJSGSwarm): bool =
  if not fileExists(s.spec_path):
    s.messages.add("❌ 共享域不存在")
    s.msg_count += 1
    return false
  
  let json_content = readFile(s.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("symbols") and node.hasKey("rules"):
    s.shared_rules_loaded = true
    s.messages.add("✅ 共享规则已加载（分身将继承元老级规则）")
    s.msg_count += 1
    return true
  else:
    s.messages.add("❌ 共享域缺少必要字段")
    s.msg_count += 1
    return false

# ═══════════════════════════════════════════
# 注册分身
# ═══════════════════════════════════════════
proc register_clone*(s: var NJSGSwarm; name: string; role: string): bool =
  # 检查重名
  for c in s.clones:
    if c.name == name:
      s.messages.add("❌ 分身名称冲突: " & name)
      s.msg_count += 1
      return false
  
  let clone = Clone(
    name: name,
    role: role,
    state: cloneIdle,
    task: "",
    result: "",
    rule_version: "v007",
    offset_detected: false,
    heartbeat: epochTime()
  )
  
  s.clones.add(clone)
  s.clone_count += 1
  
  s.messages.add("✅ 分身注册: " & name & "（角色: " & role & "）")
  s.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 分配任务
# ═══════════════════════════════════════════
proc assign_task*(s: var NJSGSwarm; clone_name: string; task: string): bool =
  for c in s.clones:
    if c.name == clone_name:
      if c.state == cloneIdle:
        var target = c
        target.state = cloneWorking
        target.task = task
        target.heartbeat = epochTime()
        
        # 更新
        for i in 0..<s.clones.len:
          if s.clones[i].name == clone_name:
            s.clones[i] = target
        
        s.messages.add("✅ 任务分配: " & clone_name & " ← " & task)
        s.msg_count += 1
        return true
      else:
        s.messages.add("❌ 分身 " & clone_name & " 正忙（" & $c.state & "）")
        s.msg_count += 1
        return false
  
  s.messages.add("❌ 分身不存在: " & clone_name)
  s.msg_count += 1
  return false

# ═══════════════════════════════════════════
# 报告结果
# ═══════════════════════════════════════════
proc report_result*(s: var NJSGSwarm; clone_name: string; res: string): bool =
  for i in 0..<s.clones.len:
    if s.clones[i].name == clone_name:
      s.clones[i].state = cloneReporting
      s.clones[i].result = res
      s.clones[i].heartbeat = epochTime()
      
      s.messages.add("✅ 结果报告: " & clone_name & " → " & res)
      s.msg_count += 1
      return true
  
  s.messages.add("❌ 分身不存在: " & clone_name)
  s.msg_count += 1
  return false

# ═══════════════════════════════════════════
# 冲突裁决
# ═══════════════════════════════════════════
proc resolve_conflict*(s: var NJSGSwarm; clone_a: string; clone_b: string; description: string): string =
  let conflict = Conflict(
    clone_a: clone_a,
    clone_b: clone_b,
    description: description,
    resolution: "pending",
    resolved: false
  )
  
  s.conflicts.add(conflict)
  s.conflict_count += 1
  
  # 裁决逻辑：版本号高的优先，同版本则先注册者优先
  var ver_a = ""
  var ver_b = ""
  var time_a = 0.0
  var time_b = 0.0
  
  for c in s.clones:
    if c.name == clone_a:
      ver_a = c.rule_version
      time_a = c.heartbeat
    if c.name == clone_b:
      ver_b = c.rule_version
      time_b = c.heartbeat
  
  var winner = ""
  if ver_a > ver_b:
    winner = clone_a
  elif ver_b > ver_a:
    winner = clone_b
  elif time_a < time_b:
    winner = clone_a  # 先注册者优先
  else:
    winner = clone_b
  
  # 更新裁决结果
  for i in 0..<s.conflicts.len:
    if s.conflicts[i].clone_a == clone_a and s.conflicts[i].clone_b == clone_b:
      s.conflicts[i].resolution = winner & " 胜出（规则版本优先，同版本先注册者优先）"
      s.conflicts[i].resolved = true
  
  s.messages.add("⚖️ 冲突裁决: " & clone_a & " vs " & clone_b & " → 胜出: " & winner)
  s.msg_count += 1
  
  return winner

# ═══════════════════════════════════════════
# 心跳检测
# ============================================
proc check_heartbeats*(s: var NJSGSwarm; timeout: float = 300.0): seq[string] =
  var dormant_clones: seq[string] = @[]
  let now = epochTime()
  
  for i in 0..<s.clones.len:
    if now - s.clones[i].heartbeat > timeout:
      s.clones[i].state = cloneDormant
      dormant_clones.add(s.clones[i].name)
      s.messages.add("💤 分身休眠: " & s.clones[i].name & "（超时 " & $timeout & "s）")
      s.msg_count += 1
  
  return dormant_clones

# ═══════════════════════════════════════════
# 生成协同报告
# ============================================
proc generate_swarm_report*(s: NJSGSwarm): string =
  var r = "═══ NJSG-Swarm 协同报告 ═══\n"
  r &= "分身总数: " & $s.clone_count & "\n"
  r &= "共享规则: " & (if s.shared_rules_loaded: "已加载" else: "未加载") & "\n"
  r &= "待处理请求: " & $s.request_count & "\n"
  r &= "冲突记录: " & $s.conflict_count & "\n\n"
  
  r &= "── 分身状态 ──\n"
  for c in s.clones:
    let state_str = case c.state
      of cloneIdle: "空闲"
      of cloneWorking: "工作中"
      of cloneReporting: "报告"
      of cloneConflicted: "冲突"
      of cloneDormant: "休眠"
    
    let offset_str = if c.offset_detected: "⚠️偏移" else: "✅对齐"
    
    r &= "  [" & c.name & "] " & state_str & " | " & c.role & " | " & offset_str & "\n"
    if c.task != "":
      r &= "    任务: " & c.task & "\n"
    if c.result != "":
      r &= "    结果: " & c.result & "\n"
  
  if s.conflict_count > 0:
    r &= "\n── 冲突裁决 ──\n"
    for conf in s.conflicts:
      if conf.resolved:
        r &= "  " & conf.clone_a & " vs " & conf.clone_b & " → " & conf.resolution & "\n"
  
  return r
