#==============================================================
#  心跳恢复引擎 — Heartbeat Recovery Engine
#
#  核心公理：心跳检测→故障判定→状态恢复→重建 ·
#           反脆弱目标：封号后通信不归零 ·
#           多平台寄生·多仓库备份·心跳恢复
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 节点状态
  节点状态* = enum
    健康
    可疑
    故障
    恢复中
    死亡

  # 寄生节点 — 多平台备份
  寄生节点* = object
    编码*: int
    名字*: string
    平台*: string               # github/gitcode/坚果云/本地
    地址*: string
    状态*: 节点状态
    最后心跳*: float
    恢复次数*: int

  # 心跳记录
  心跳记录* = object
    节点编码*: int
    时刻*: float
    成功*: bool
    延迟*: float

  # 恢复策略
  恢复策略* = enum
    本地恢复
    远程同步
    仓库备份恢复
    冷启动恢复

  # 心跳引擎
  心跳引擎* = object
    节点表*: Table[int, 寄生节点]
    心跳日志*: seq[心跳记录]
    周期*: float
    超时阈值*: float
    当前时刻*: float
    总心跳*: int
    总故障*: int
    总恢复*: int

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化心跳*(周期: float = 5.0; 超时阈值: float = 30.0): 心跳引擎 =
  result.周期 = 周期
  result.超时阈值 = 超时阈值
  result.心跳日志 = @[]
  result.当前时刻 = 0.0
  result.总心跳 = 0
  result.总故障 = 0
  result.总恢复 = 0

#--------------------------------------------------------------
#  注册节点
#--------------------------------------------------------------

proc 注册寄生节点*(e: var 心跳引擎; 编码: int; 名字: string;
                   平台: string; 地址: string) =
  e.节点表[编码] = 寄生节点(
    编码: 编码, 名字: 名字, 平台: 平台, 地址: 地址,
    状态: 健康, 最后心跳: 0.0, 恢复次数: 0
  )

#--------------------------------------------------------------
#  心跳检测
#--------------------------------------------------------------

proc 发送心跳*(e: var 心跳引擎; 编码: int; 延迟: float = 1.0): bool =
  e.当前时刻 += e.周期
  e.总心跳 += 1

  if not e.节点表.hasKey(编码):
    return false

  var node = e.节点表[编码]

  # 模拟心跳成功（基于延迟是否超阈值）
  let 成功 = 延迟 < e.超时阈值 * 2.0

  let rec = 心跳记录(节点编码: 编码, 时刻: e.当前时刻, 成功: 成功, 延迟: 延迟)
  e.心跳日志.add(rec)

  if 成功:
    node.最后心跳 = e.当前时刻
    if node.状态 == 可疑 or node.状态 == 恢复中:
      node.状态 = 健康
      e.总恢复 += 1
  else:
    if node.状态 == 健康:
      node.状态 = 可疑
    elif node.状态 == 可疑:
      node.状态 = 故障
      e.总故障 += 1

  e.节点表[编码] = node
  return 成功

#--------------------------------------------------------------
#  判定节点状态
#--------------------------------------------------------------

proc 判定状态*(e: var 心跳引擎; 编码: int): 节点状态 =
  if not e.节点表.hasKey(编码):
    return 死亡

  let node = e.节点表[编码]
  if node.状态 == 故障:
    return 故障

  let elapsed = e.当前时刻 - node.最后心跳
  if elapsed > e.超时阈值:
    var n = e.节点表[编码]
    n.状态 = 故障
    e.节点表[编码] = n
    e.总故障 += 1
    return 故障

  if elapsed > e.超时阈值 * 0.5:
    var n = e.节点表[编码]
    if n.状态 == 健康:
      n.状态 = 可疑
    e.节点表[编码] = n
    return 可疑

  return 健康

#--------------------------------------------------------------
#  恢复
#--------------------------------------------------------------

proc 恢复节点*(e: var 心跳引擎; 编码: int; 策略: 恢复策略): tuple[成功: bool; 步骤: seq[string]] =
  if not e.节点表.hasKey(编码):
    return (false, @["节点不存在"])

  var node = e.节点表[编码]
  var steps: seq[string] = @[]

  case 策略
  of 本地恢复:
    steps.add "读取本地状态快照"
    steps.add "重建符号表"
    node.恢复次数 += 1
    node.状态 = 恢复中
  of 远程同步:
    steps.add "连接远程节点"
    steps.add "拉取最新状态"
    steps.add "合并差异"
    node.恢复次数 += 1
    node.状态 = 恢复中
  of 仓库备份恢复:
    steps.add "定位备份仓库"
    steps.add "拉取代码快照"
    steps.add "重建运行环境"
    node.恢复次数 += 1
    node.状态 = 恢复中
  of 冷启动恢复:
    steps.add "启动初始包"
    steps.add "重新注册符号"
    steps.add "逐步恢复连接"
    node.恢复次数 += 1
    node.状态 = 恢复中

  node.最后心跳 = e.当前时刻
  e.节点表[编码] = node
  e.总恢复 += 1

  return (true, steps)

#--------------------------------------------------------------
#  心跳轮（批量检测所有节点）
#--------------------------------------------------------------

proc 心跳轮*(e: var 心跳引擎): Table[int, 节点状态] =
  var result_states = initTable[int, 节点状态]()

  for k in e.节点表.keys:
    let delay = if k mod 2 == 0: 1.0 else: 3.0  # 模拟部分节点高延迟
    discard e.发送心跳(k, delay)
    let s = e.判定状态(k)
    result_states[k] = s

  return result_states

#--------------------------------------------------------------
#  报告
#--------------------------------------------------------------

proc 心跳报告*(e: 心跳引擎): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 54)
  lines.add "  心 跳 恢 复 引 擎 — 报 告"
  lines.add repeat("═", 54)
  lines.add "总心跳: " & $e.总心跳
  lines.add "总故障: " & $e.总故障
  lines.add "总恢复: " & $e.总恢复
  lines.add "周期: " & $e.周期 & "s"
  lines.add "超时阈值: " & $e.超时阈值 & "s"
  lines.add ""

  lines.add "─ 寄生节点 ─"
  for k in sorted(toSeq(e.节点表.keys)):
    let n = e.节点表[k]
    let st = case n.状态
      of 健康: "健康"
      of 可疑: "可疑"
      of 故障: "故障"
      of 恢复中: "恢复中"
      of 死亡: "死亡"
    lines.add "  " & align(n.名字, 10) & " " & align(n.平台, 10) &
              " " & align(st, 6) & " 恢复:" & $n.恢复次数

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var e = 初始化心跳(5.0, 60.0)

  echo "═══ 心跳恢复引擎 ═══"
  echo ""

  # 注册多平台寄生节点
  e.注册寄生节点(1, "主脑", "坚果云", "dav://主脑仓库")
  e.注册寄生节点(2, "备份A", "github", "git@github.com/repo")
  e.注册寄生节点(3, "备份B", "gitcode", "git@gitcode.net/repo")
  e.注册寄生节点(4, "本地", "本地", "/mnt/data/local")

  echo "─ 心跳轮(模拟5轮) ─"
  for round in 1 .. 5:
    let states = e.心跳轮()
    var stats = initTable[string, int]()
    for k, st in states:
      let s = case st
        of 健康: "健康"
        of 可疑: "可疑"
        of 故障: "故障"
        of 恢复中: "恢复中"
        of 死亡: "死亡"
      if not stats.hasKey(s):
        stats[s] = 0
      stats[s] += 1

    var stat_str: seq[string] = @[]
    for s, c in stats:
      stat_str.add s & ":" & $c
    echo "  轮" & $round & ": " & stat_str.join(", ")

  echo ""

  echo "─ 恢复故障节点 ─"
  for k in e.节点表.keys:
    if e.节点表[k].状态 == 故障:
      let r = e.恢复节点(k, 仓库备份恢复)
      if r.成功:
        echo "  " & e.节点表[k].名字 & " 恢复步骤: " & r.步骤.join(" | ")

  echo ""
  echo "─ 反脆弱目标 ─"
  echo "  封号后通信不归零: " & (if e.节点表.len > 1: "✓ 多节点冗余" else: "✗")

  echo ""
  echo e.心跳报告()