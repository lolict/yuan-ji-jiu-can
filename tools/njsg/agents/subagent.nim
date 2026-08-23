# ============================================================
# njsg-swarm · 子代理 (分身)
# 每个子代理是一个独立执行单元
# ============================================================

import ../swarm/coordinator
import strutils

type
  SubAgent* = object
    id*: AgentId
    name*: string
    specialty*: string      # 专长领域
    polarity*: int          # 极性
    task_count*: int        # 已完成任务数
    status*: string         # 当前状态描述

# 创建子代理
proc new_sub_agent*(id: AgentId, name, specialty: string, polarity: int = 0): SubAgent =
  result = SubAgent(
    id: id,
    name: name,
    specialty: specialty,
    polarity: polarity,
    task_count: 0,
    status: "就绪"
  )

# 执行任务 (模拟)
proc execute*(a: var SubAgent, task: Task): string =
  a.status = "执行: " & task.name
  a.task_count += 1
  var s = "[代理 " & a.name & "] 完成任务 #" & $task.id & " " & task.name
  if task.payload > 0:
    s &= " (payload: 0x" & to_hex(task.payload) & ")"
  result = s

# 代理间通信
proc communicate*(a: SubAgent, b: SubAgent, msg: string): string =
  var s = "[通信] " & a.name & " → " & b.name & ": " & msg
  if a.polarity == b.polarity:
    s &= " (同极性, 直接传递)"
  else:
    s &= " (跨极性, 需见证)"
  result = s

# 子代理报告
proc agent_report*(a: SubAgent): string =
  var s = "子代理 #" & $a.id & " " & a.name & "\n"
  s &= "  专长: " & a.specialty & "\n"
  s &= "  极性: " & (if a.polarity > 0: "阳" elif a.polarity < 0: "阴" else: "中") & "\n"
  s &= "  已完成: " & $a.task_count & " 任务\n"
  s &= "  状态: " & a.status & "\n"
  result = s
