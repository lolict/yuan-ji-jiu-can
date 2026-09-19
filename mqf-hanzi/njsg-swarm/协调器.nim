# ============================================================
# 囡囝共生分布式智能体 — 协调器
# 协调器 = 容器调度的中枢，管理分身注册、任务分配、冲突裁决
# 设计原则: 不直接执行，只裁决和分配
# ============================================================

import sequtils
import strutils

# ---- 协调器状态 ----

type
  协调器* = object
    分身注册表*: seq[分身条目]
    任务池*: seq[协调任务]
    裁决历史*: seq[string]
    轮次*: int

  分身条目* = object
    名字*: string
    极性*: int
    主权*: uint64
    已分配*: seq[string]    # 任务标识
    状态*: string

  协调任务* = object
    标识*: string
    描述*: string
    所需极性*: int
    已分配*: bool
    分配给了*: string

# ---- 分身注册 ----

proc 注册分身*(c: var 协调器; 名字: string; 极性: int; 主权: uint64): string =
  # 检查重复注册
  for entry in c.分身注册表:
    if entry.名字 == 名字:
      return "[拒绝] 分身'" & 名字 & "'已存在"
    if entry.主权 == 主权:
      return "[拒绝] 主权'" & $主权 & "'已被" & entry.名字 & "占用"

  c.分身注册表.add(分身条目(
    名字: 名字,
    极性: 极性,
    主权: 主权,
    已分配: @[],
    状态: "就绪"
  ))
  return "[注册] " & 名字 & "(极=" & $极性 & ",主=" & $主权 & ")"

# ---- 任务分配 ----

proc 分配任务*(c: var 协调器; 任务标识, 描述: string; 所需极性: int): string =
  c.任务池.add(协调任务(
    标识: 任务标识,
    描述: 描述,
    所需极性: 所需极性,
    已分配: false,
    分配给了: ""
  ))
  let idx = len(c.任务池) - 1

  # 找极性相反(吸)的空闲分身
  var 最优: int = -1
  var 最优主权: uint64 = 0'u64

  for i, entry in c.分身注册表:
    if entry.状态 != "就绪":
      continue
    let gap = abs(entry.极性 - 所需极性)
    if gap == 1:  # 异极 = 吸 = 协作
      if 最优 < 0 or entry.主权 > 最优主权:
        最优 = i
        最优主权 = entry.主权

  if 最优 >= 0:
    c.任务池[idx].已分配 = true
    c.任务池[idx].分配给了 = c.分身注册表[最优].名字
    c.分身注册表[最优].已分配.add(任务标识)
    c.分身注册表[最优].状态 = "忙碌"
    return "[分配] '" & 任务标识 & "' → " & c.分身注册表[最优].名字 & "(吸,公约优先)"
  else:
    return "[等待] '" & 任务标识 & "' 无匹配分身(需要异极空闲)"

# ---- 分身释放 ----

proc 释放分身*(c: var 协调器; 名字: string): string =
  for i, entry in c.分身注册表:
    if entry.名字 == 名字:
      c.分身注册表[i].状态 = "就绪"
      return "[释放] " & 名字
  return "[找不到] " & 名字

# ---- 状态报告 ----

proc 协调器报告*(c: 协调器): string =
  var lines: seq[string]
  lines.add "═══ 协调器状态 [轮次=" & $c.轮次 & "] ═══"
  lines.add "分身数: " & $len(c.分身注册表) & " | 任务数: " & $len(c.任务池)
  lines.add ""
  for entry in c.分身注册表:
    lines.add "  " & entry.名字 & "|极=" & $entry.极性 & "|主=" & $entry.主权 & "|" & entry.状态 & "|任务:" & entry.已分配.join(",")
  lines.add ""
  var 已分 = 0
  var 待分 = 0
  for t in c.任务池:
    if t.已分配: inc 已分
    else: inc 待分
  lines.add "任务: 已分=" & $已分 & " 待分=" & $待分
  return lines.join("\n")

# ---- 运行一轮 ----

proc 运行轮次*(c: var 协调器): string =
  ## 完成所有忙碌分身，使他们回到就绪状态
  inc c.轮次
  var 释放列表: seq[string]
  for entry in c.分身注册表:
    if entry.状态 == "忙碌":
      释放列表.add(entry.名字)
  for name in 释放列表:
    discard c.释放分身(name)
  return "[轮次" & $c.轮次 & "] 释放: " & 释放列表.join(", ")

# ============================================================
# 单元测试
# ============================================================
when isMainModule:
  echo "=== 协调器单元测试 ==="
  var c: 协调器

  echo c.注册分身("震甲", 1, 0x01010101'u64)
  echo c.注册分身("巽乙", 0, 0x02020202'u64)
  echo c.注册分身("艮丙", 1, 0x03030303'u64)
  echo ""
  echo c.分配任务("NS_SCAN", "扫描命名空间", 0)
  echo c.分配任务("POL判决", "极性判决", 1)
  echo c.分配任务("SYM_CLEAN", "符号清洗", 0)
  echo ""
  echo 协调器报告(c)
  echo ""
  echo 运行轮次(c)
  echo ""
  echo 协调器报告(c)
