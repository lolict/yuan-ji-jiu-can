# ============================================================
# njsg-swarm · 协调器
# 管理任务分配、冲突裁决、子代理调度
# ============================================================

import std/[tables, sets, algorithm, sequtils]

type
  AgentId* = int
  
  TaskStatus* = enum
    tsPending,     # 待分配
    tsAssigned,    # 已分配
    tsRunning,     # 执行中
    tsCompleted,   # 已完成
    tsFailed,      # 失败
    tsConflict     # 冲突待裁决

  TaskPriority* = enum
    tpLow, tpNormal, tpHigh, tpCritical

  Task* = object
    id*: int
    name*: string
    status*: TaskStatus
    priority*: TaskPriority
    assignee*: AgentId          # 分配的代理 (-1=未分配)
    dependencies*: seq[int]     # 依赖的任务 ID
    conflict_with*: HashSet[int] # 与之冲突的任务
    payload*: uint64            # 携带数据 (可编码容器)

  Agent* = object
    id*: int
    name*: string
    active*: bool
    current_task*: int          # 当前执行的任务 (-1=空闲)
    completed*: seq[int]        # 已完成任务
    polarity*: int              # 代理极性 (影响裁决)
    capacity*: int              # 同时处理任务数

  Conflict* = object
    task_a*: int
    task_b*: int
    reason*: string
    resolution*: string

  Coordinator* = object
    tasks*: Table[int, Task]
    agents*: Table[int, Agent]
    conflict_log*: seq[Conflict]
    task_queue*: seq[int]       # 待分配任务队列
    next_task_id*: int
    agent_count*: int           # 代理计数 (Table 无 len)
    task_count*: int            # 任务计数
    tick*: int

# 创建协调器
proc new_coordinator*(): Coordinator =
  result = Coordinator(
    tasks: init_table[int, Task](),
    agents: init_table[int, Agent](),
    conflict_log: @[],
    task_queue: @[],
    next_task_id: 0,
    agent_count: 0,
    task_count: 0,
    tick: 0
  )

# 注册代理
proc register_agent*(c: var Coordinator, name: string, polarity: int = 0, capacity: int = 1): AgentId =
  let id = c.agent_count
  inc c.agent_count
  c.agents[id] = Agent(
    id: id,
    name: name,
    active: true,
    current_task: -1,
    completed: @[],
    polarity: polarity,
    capacity: capacity
  )
  result = id

# 提交任务
proc submit_task*(c: var Coordinator, name: string, priority: TaskPriority = tpNormal, deps: seq[int] = @[], payload: uint64 = 0): int =
  let id = c.next_task_id
  inc c.next_task_id
  c.tasks[id] = Task(
    id: id,
    name: name,
    status: tsPending,
    priority: priority,
    assignee: -1,
    dependencies: deps,
    conflict_with: init_hash_set[int](),
    payload: payload
  )
  c.task_queue.add(id)
  inc c.task_count
  result = id

# 检测冲突
proc detect_conflicts*(c: var Coordinator): seq[Conflict] =
  result = @[]
  let task_ids = to_seq(c.tasks.keys).sorted()
  for i in 0..<task_ids.len:
    for j in (i+1)..<task_ids.len:
      let a = task_ids[i]
      let b = task_ids[j]
      # 检查是否有共同依赖 (资源冲突)
      let da = c.tasks[a].dependencies
      let db = c.tasks[b].dependencies
      for d in da:
        if d in db:
          result.add(Conflict(
            task_a: a,
            task_b: b,
            reason: "共同依赖任务#" & $d,
            resolution: ""
          ))
          c.tasks[a].conflict_with.incl(b)
          c.tasks[b].conflict_with.incl(a)
          break

# 裁决冲突: 优先级高的先执行, 相同则极性=1(我)优先
proc resolve_conflict*(c: var Coordinator, conflict: Conflict): string =
  let ta = c.tasks[conflict.task_a]
  let tb = c.tasks[conflict.task_b]
  
  if ta.priority.int > tb.priority.int:
    result = "任务#" & $ta.id & "(高优先级)先执行"
    c.tasks[conflict.task_b].status = tsConflict
  elif ta.priority.int < tb.priority.int:
    result = "任务#" & $tb.id & "(高优先级)先执行"
    c.tasks[conflict.task_a].status = tsConflict
  else:
    # 同优先级: 极性裁决 (阳优先)
    let agent_a = if ta.assignee >= 0: c.agents[ta.assignee].polarity else: 0
    let agent_b = if tb.assignee >= 0: c.agents[tb.assignee].polarity else: 0
    if agent_a >= agent_b:
      result = "任务#" & $ta.id & "(阳极性)先执行"
      c.tasks[conflict.task_b].status = tsConflict
    else:
      result = "任务#" & $tb.id & "(阳极性)先执行"
      c.tasks[conflict.task_a].status = tsConflict
  
  c.conflict_log.add(Conflict(
    task_a: conflict.task_a,
    task_b: conflict.task_b,
    reason: conflict.reason,
    resolution: result
  ))

# 分配任务给空闲代理
proc dispatch*(c: var Coordinator): int =
  var dispatched = 0
  while c.task_queue.len > 0:
    let task_id = c.task_queue[0]
    # 检查依赖是否完成
    let deps = c.tasks[task_id].dependencies
    var deps_met = true
    for d in deps:
      if d in c.tasks:
        if c.tasks[d].status != tsCompleted:
          deps_met = false
          break
    if not deps_met:
      # 依赖未完成, 跳过 (放到队尾)
      c.task_queue.delete(0)
      c.task_queue.add(task_id)
      break
    
    # 找空闲代理
    var assigned = false
    for id, agent in c.agents:
      if agent.active and agent.current_task < 0:
        c.task_queue.delete(0)
        c.tasks[task_id].status = tsAssigned
        c.tasks[task_id].assignee = id
        c.agents[id].current_task = task_id
        assigned = true
        dispatched += 1
        break
    
    if not assigned:
      break  # 没有空闲代理
  
  result = dispatched

# 完成任务
proc complete_task*(c: var Coordinator, task_id: int) =
  if task_id in c.tasks:
    c.tasks[task_id].status = tsCompleted
    let agent_id = c.tasks[task_id].assignee
    if agent_id >= 0 and agent_id in c.agents:
      c.agents[agent_id].current_task = -1
      c.agents[agent_id].completed.add(task_id)
    # 解除与之冲突的任务的阻塞
    for conflict_id in c.tasks[task_id].conflict_with:
      if conflict_id in c.tasks:
        if c.tasks[conflict_id].status == tsConflict:
          c.tasks[conflict_id].status = tsPending
          c.task_queue.add(conflict_id)

# 推进 tick
proc tick*(c: var Coordinator) =
  inc c.tick

# 协调器状态报告
proc coordinator_report*(c: Coordinator): string =
  var s = "═══ 协调器状态 ═══\n"
  s &= "Tick: " & $c.tick & "\n"
  s &= "代理数: " & $c.agent_count & "\n"
  s &= "任务数: " & $c.task_count & "\n\n"
  
  s &= "  代理列表:\n"
  for id, agent in c.agents:
    var st = "空闲"
    if agent.current_task >= 0:
      st = "执行#" & $agent.current_task
    s &= "    #" & $id & " " & agent.name & " [" & st & "] 极性:" & $agent.polarity & "\n"
  
  s &= "\n  任务列表:\n"
  for id, task in c.tasks:
    var status_str = case task.status
      of tsPending: "待分配"
      of tsAssigned: "已分配"
      of tsRunning: "执行中"
      of tsCompleted: "已完成"
      of tsFailed: "失败"
      of tsConflict: "冲突"
    var assignee_str = "未分配"
    if task.assignee >= 0: assignee_str = "#" & $task.assignee
    s &= "    #" & $id & " " & task.name & " [" & status_str & "] → " & assignee_str & "\n"
  
  if c.conflict_log.len > 0:
    s &= "\n  冲突裁决:\n"
    for cf in c.conflict_log:
      s &= "    #" & $cf.task_a & " vs #" & $cf.task_b & " → " & cf.resolution & "\n"
  
  result = s
