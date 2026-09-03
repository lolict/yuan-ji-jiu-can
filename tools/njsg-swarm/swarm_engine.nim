# ============================================================
# NJSG-Swarm 协调引擎 — 真实可运行
#
# 功能：
#   本体 → 协调器 → 分身（子代理）
#   分身用蚕蛹壳（git/api/ai）干活
#   协调器裁决冲突 · 监控心跳 · 继承规则
# ============================================================

import os, osproc, strutils, json, tables, times, sequtils,algorithm

const
  REPO_ROOT = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"
  SHARED_DOMAIN = REPO_ROOT & "/tools/symbol-bootstrap/spec/sdomain.json"

type
  CloneState* = enum
    cloneIdle, cloneWorking, cloneReporting, cloneConflicted, cloneDormant, cloneDead

  Clone* = object
    name*: string
    role*: string          # git-worker / api-worker / ai-worker / validator / guardian
    state*: CloneState
    task*: string
    result*: string
    rule_version*: string
    spawned_at*: float
    last_heartbeat*: float
    adaptation_count*: int

  Task* = object
    id*: string
    description*: string
    assigned_to*: string
    status*: string        # pending / working / completed / failed
    priority*: int         # 1-9 (九元优先级)
    created_at*: float

  Conflict* = object
    id*: string
    description*: string
    party_a*: string
    party_b*: string
    resolution*: string
    resolved*: bool
    winner*: string

  Swarm* = ref object
    clones*: seq[Clone]
    tasks*: seq[Task]
    conflicts*: seq[Conflict]
    rule_version*: string
    max_clones*: int
    heartbeat_timeout*: float

# ═══════════════════════════════════════════
# 共享域加载
# ═══════════════════════════════════════════
proc load_shared_domain(): JsonNode =
  if fileExists(SHARED_DOMAIN):
    result = parseJson(readFile(SHARED_DOMAIN))

# ═══════════════════════════════════════════
# 协调器操作
# ═══════════════════════════════════════════
proc new_swarm*(max_clones: int = 6; heartbeat_timeout: float = 300.0): Swarm =
  new(result)
  result.max_clones = max_clones
  result.heartbeat_timeout = heartbeat_timeout
  result.rule_version = "v1.0"
  result.clones = @[]
  result.tasks = @[]
  result.conflicts = @[]

proc spawn_clone*(s: Swarm; name: string; role: string): string =
  if s.clones.len >= s.max_clones:
    return "ERROR: 分身数已满 (" & $s.max_clones & ")"
  let now = epochTime()
  var c = Clone(
    name: name,
    role: role,
    state: cloneIdle,
    task: "",
    result: "",
    rule_version: s.rule_version,
    spawned_at: now,
    last_heartbeat: now,
    adaptation_count: 0
  )
  s.clones.add(c)
  return "分身[" & name & "](" & role & ") 已派生"

proc submit_task*(s: Swarm; desc: string; priority: int = 5): string =
  let t = Task(
    id: "T" & $(s.tasks.len + 1),
    description: desc,
    assigned_to: "",
    status: "pending",
    priority: priority,
    created_at: epochTime()
  )
  s.tasks.add(t)
  result = "任务[" & t.id & "] 已提交 (优先级=" & $priority & ")"

proc assign_tasks*(s: Swarm): seq[string] =
  result = @[]
  # 按优先级排序任务
  var pending = s.tasks.filterIt(it.status == "pending")
  pending.sort(proc (a, b: Task): int = cmp(b.priority, a.priority))
  for t in pending:
    # 找到空闲分身
    var assigned = false
    for i in 0..<s.clones.len:
      if s.clones[i].state == cloneIdle:
        # 根据角色匹配合适的分身
        var suitable = false
        if t.description.contains("git") or t.description.contains("仓库"):
          suitable = s.clones[i].role == "git-worker"
        elif t.description.contains("api") or t.description.contains("HTTP"):
          suitable = s.clones[i].role == "api-worker"
        elif t.description.contains("ai") or t.description.contains("模型"):
          suitable = s.clones[i].role == "ai-worker"
        elif t.description.contains("验证") or t.description.contains("检查"):
          suitable = s.clones[i].role == "validator"
        else:
          suitable = true  # 通用任务任何分身可做
        if suitable:
          for ti in 0..<s.tasks.len:
            if s.tasks[ti].id == t.id:
              s.tasks[ti].assigned_to = s.clones[i].name
              s.tasks[ti].status = "working"
              break
          s.clones[i].state = cloneWorking
          s.clones[i].task = t.description
          s.clones[i].last_heartbeat = epochTime()
          result.add("任务[" & t.id & "] → 分身[" & s.clones[i].name & "]")
          assigned = true
          break
    if not assigned:
      # 分配到任何空闲分身
      for i in 0..<s.clones.len:
        if s.clones[i].state == cloneIdle:
          for ti in 0..<s.tasks.len:
            if s.tasks[ti].id == t.id:
              s.tasks[ti].assigned_to = s.clones[i].name
              s.tasks[ti].status = "working"
              break
          s.clones[i].state = cloneWorking
          s.clones[i].task = t.description
          s.clones[i].last_heartbeat = epochTime()
          result.add("任务[" & t.id & "] → 分身[" & s.clones[i].name & "] (通用)")
          assigned = true
          break

proc report_result*(s: Swarm; clone_name: string; work_result: string): string =
  for i in 0..<s.clones.len:
    if s.clones[i].name == clone_name:
      s.clones[i].state = cloneReporting
      s.clones[i].result = work_result
      s.clones[i].last_heartbeat = epochTime()
      s.clones[i].adaptation_count += 1
      # 更新任务状态
      for ti in 0..<s.tasks.len:
        if s.tasks[ti].assigned_to == clone_name and s.tasks[ti].status == "working":
          s.tasks[ti].status = "completed"
      return "分身[" & clone_name & "] 报告完成"
  return "ERROR: 分身不存在"

proc complete_clone_task*(s: Swarm; clone_name: string) =
  for i in 0..<s.clones.len:
    if s.clones[i].name == clone_name:
      s.clones[i].state = cloneIdle
      s.clones[i].task = ""

proc detect_conflicts*(s: Swarm): seq[Conflict] =
  result = @[]
  # 检测：多个分身同时修改同一资源
  var working = s.clones.filterIt(it.state == cloneWorking)
  for i in 0..<working.len:
    for j in i+1..<working.len:
      # 简单冲突检测：任务描述相似
      if working[i].task == working[j].task and working[i].task.len > 0:
        var c = Conflict(
          id: "C" & $(s.conflicts.len + 1),
          description: "分身[" & working[i].name & "]和[" & working[j].name & "]执行相同任务: " & working[i].task,
          party_a: working[i].name,
          party_b: working[j].name,
          resolution: "",
          resolved: false,
          winner: ""
        )
        s.clones[i].state = cloneConflicted
        s.clones[j].state = cloneConflicted
        result.add(c)
        s.conflicts.add(c)

proc resolve_conflict*(s: Swarm; conflict_id: string): string =
  for i in 0..<s.conflicts.len:
    if s.conflicts[i].id == conflict_id and not s.conflicts[i].resolved:
      # 裁决规则：更高规则版本赢 → 同版本→注册时间更早赢→字母序
      let a_name = s.conflicts[i].party_a
      let b_name = s.conflicts[i].party_b
      var winner = a_name
      # 找分身索引
      var ai = -1
      var bi = -1
      for ci in 0..<s.clones.len:
        if s.clones[ci].name == a_name: ai = ci
        if s.clones[ci].name == b_name: bi = ci
      if ai >= 0 and bi >= 0:
        if s.clones[ai].rule_version > s.clones[bi].rule_version:
          winner = a_name
        elif s.clones[bi].rule_version > s.clones[ai].rule_version:
          winner = b_name
        else:
          # 同规则版本：注册时间更早 wins（先来后到）
          if s.clones[ai].spawned_at > s.clones[bi].spawned_at:
            winner = b_name  # b 更早
          elif s.clones[bi].spawned_at > s.clones[ai].spawned_at:
            winner = a_name  # a 更早
          else:
            winner = min(a_name, b_name)  # 字母序
      s.conflicts[i].resolved = true
      s.conflicts[i].winner = winner
      s.conflicts[i].resolution = "裁决: " & winner & " 胜出 (优先级: 规则版本 > 注册时间 > 字母序)"
      # 失败方回退为 idle
      for ci in 0..<s.clones.len:
        if s.clones[ci].name == a_name or s.clones[ci].name == b_name:
          s.clones[ci].state = cloneIdle
      return s.conflicts[i].resolution
  return "ERROR: 冲突不存在或已裁决"

proc check_heartbeats*(s: Swarm): seq[string] =
  result = @[]
  let now = epochTime()
  for i in 0..<s.clones.len:
    let elapsed = now - s.clones[i].last_heartbeat
    if elapsed > s.heartbeat_timeout and s.clones[i].state in [cloneWorking, cloneReporting]:
      s.clones[i].state = cloneDormant
      result.add("分身[" & s.clones[i].name & "] 心跳超时 → 休眠")
    elif s.clones[i].state == cloneDormant and elapsed < s.heartbeat_timeout:
      s.clones[i].state = cloneIdle
      result.add("分身[" & s.clones[i].name & "] 心跳恢复 → 空闲")

proc reset_heartbeat*(s: Swarm; clone_name: string) =
  for i in 0..<s.clones.len:
    if s.clones[i].name == clone_name:
      s.clones[i].last_heartbeat = epochTime()
      if s.clones[i].state == cloneDormant:
        s.clones[i].state = cloneIdle

# ═══════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════
proc report*(s: Swarm) =
  echo ""
  echo "╔══════════════════════════════════════════════════╗"
  echo "║        NJSG-Swarm 协调器 · 状态              ║"
  echo "╚══════════════════════════════════════════════════╝"
  echo "  规则版本: " & s.rule_version
  echo "  最大分身: " & $s.max_clones
  echo "  心跳超时: " & $s.heartbeat_timeout & "s"
  echo ""
  echo "  [分身列表]"
  if s.clones.len == 0:
    echo "    （无分身）"
  for c in s.clones:
    let state_str = case c.state
      of cloneIdle: "空闲"
      of cloneWorking: "工作中"
      of cloneReporting: "报告"
      of cloneConflicted: "冲突"
      of cloneDormant: "休眠"
      of cloneDead: "终止"
    echo "    [" & c.name & "] " & c.role & " · " & state_str & " · 任务=" & c.task & " · 适配=" & $c.adaptation_count
  echo ""
  echo "  [任务队列]"
  if s.tasks.len == 0:
    echo "    （无任务）"
  for t in s.tasks:
    let assign = if t.assigned_to.len > 0: " → " & t.assigned_to else: ""
    echo "    [" & t.id & "] (P" & $t.priority & ") " & t.description & assign & " [" & t.status & "]"
  echo ""
  if s.conflicts.len > 0:
    echo "  [冲突记录]"
    for c in s.conflicts:
      let status = if c.resolved: "已裁决(" & c.winner & ")" else: "待裁决"
      echo "    [" & c.id & "] " & c.party_a & " vs " & c.party_b & " · " & status

when isMainModule:
  echo "swarm_engine.nim — NJSG-Swarm 协调引擎"
