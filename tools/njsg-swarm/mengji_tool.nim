# ============================================================
# 盟机合 · CLI 节点注册 + Gossip 测试脚本
#   —— MengJi Node Registration CLI + Gossip Test Script
# ------------------------------------------------------------
#
# 用法:
#   nim c -r mengji_tool.nim register [--name NODE_ID] [--type PHONG|FARM] [--anchor 0|1]
#   nim c -r mengji_tool.nim gossip [--rounds N]
#   nim c -r mengji_tool.nim consensus [--subject SLOT]
#   nim c -r mengji_tool.nim conflict [--subject SLOT]
#   nim c -r mengji_tool.nim status
#   nim c -r mengji_tool.nim demo
#
# 满全法协议:
#   0 = 刘楚恬（低位隐藏侧）
#   1 = 我（高位可见侧）
#   容器 = 第一存在单位
#   盟机合掌(PHONG) = 资源拥有者·运营场
#   盟机场(FARM)   = 节点·萌机场
# ============================================================

import std/[strutils, os, json, tables, sequtils, times, parseopt, random]

# ── 节点类型 ──
type MengJiNodeType* = enum
  mjPhong  # 盟机合掌
  mjFarm   # 盟机场/萌机场

type MengJiVerdict* = enum
  mjPending  # 待裁定
  mjPass     # 许可
  mjReject   # 拒绝
  mjSplit    # 分形

type MengJiAnchor* = enum
  mjLiutiantian  # 刘楚恬: 低位隐藏侧 = 0
  mjLolict       # 我: 高位可见侧 = 1

# ── 节点 ──
type MengJiNode* = object
  id*: string
  name*: string
  nodeType*: MengJiNodeType
  anchor*: MengJiAnchor
  trust*: float
  heartbeat*: float  # Unix timestamp as float
  neighborIds*: seq[string]
  verdictState*: seq[MengJiVerdict]
  alive*: bool

# ── 裁定条目 ──
type MengJiVerdictEntry* = object
  slot*: int
  subject*: string
  verdict*: MengJiVerdict
  witnesses*: seq[string]     # 节点 id 列表(最多 3)
  consensusReached*: bool
  decidedAt*: float

# ── 盟机合引擎 ──
type MengJiEngine* = object
  nodes*: TableRef[string, MengJiNode]
  verdicts*: TableRef[string, MengJiVerdictEntry]
  globalTick*: int
  pushCount*: int
  pullCount*: int
  primaryAnchor*: MengJiAnchor

# ── 常量 ──
const
  MAX_NODES*    = 32
  MAX_VERDICTS* = 64
  GOSSIP_INTERVAL* = 5
  HEARTBEAT_TIMEOUT* = 30
  TRUST_MAX*   = 100.0
  TRUST_MIN*   = 0.0

# ── 工具函数 ──
proc verdictName*(v: MengJiVerdict): string =
  case v
  of mjPending: "待定"
  of mjPass:    "许可"
  of mjReject:  "拒绝"
  of mjSplit:   "分形"

proc nodeTypeName*(t: MengJiNodeType): string =
  case t
  of mjPhong: "合掌(PHONG)"
  of mjFarm:  "机场(FARM)"

proc anchorName*(a: MengJiAnchor): string =
  case a
  of mjLiutiantian: "刘楚恬(0)"
  of mjLolict:      "我(1)"

proc anchorValue*(a: MengJiAnchor): int =
  case a
  of mjLiutiantian: 0
  of mjLolict:      1

# ── 创建引擎 ──
proc newMengJiEngine*(): ref MengJiEngine =
  new result
  result.nodes = newTable[string, MengJiNode]()
  result.verdicts = newTable[string, MengJiVerdictEntry]()
  result.globalTick = 0
  result.pushCount = 0
  result.pullCount = 0
  result.primaryAnchor = mjLolict  # 默认夫妻主权人高位侧

# ── 节点注册 ──
proc registerNode*(engine: ref MengJiEngine; name: string;
                  ntype: MengJiNodeType; anchor: MengJiAnchor): string =
  if engine.nodes.len >= MAX_NODES:
    echo "[盟机合错误] 节点数已达上限 ", MAX_NODES
    return ""

  # 简单 hash 生成 id
  let hashInput = name & $epochTime()
  var h1 = 0x811c9dc5'u32
  var h2 = 0x0100193'u32
  for c in hashInput:
    h1 = h1 xor uint32(c)
    h1 = h1 *% 0x01000193'u32
    h2 = h2 xor (uint32(c) shl 5)
    h2 = h2 *% 0x1000193'u32
  let id = toHex(h1) & toHex(h2)

  var node: MengJiNode
  node.id = id
  node.name = name
  node.nodeType = ntype
  node.anchor = anchor
  node.trust = if ntype == mjPhong: 80.0 else: 50.0
  node.heartbeat = epochTime()
  node.neighborIds = @[]
  node.verdictState = @[]
  node.alive = true

  engine.nodes[id] = node
  echo "[盟机合] 注册节点: id=", id, " name=", name,
       " type=", nodeTypeName(ntype), " anchor=", anchorName(anchor),
       " trust=", int(node.trust), "%"
  return id

# ── 邻居绑定 ──
proc bindNeighbor*(engine: ref MengJiEngine; idA: string; idB: string) =
  if not engine.nodes.hasKey(idA) or not engine.nodes.hasKey(idB):
    echo "[盟机合错误] 节点未找到"
    return
  if idA == idB: return
  var nodeA = engine.nodes[idA]
  var nodeB = engine.nodes[idB]
  if idB notin nodeA.neighborIds: nodeA.neighborIds.add(idB)
  if idA notin nodeB.neighborIds: nodeB.neighborIds.add(idA)
  engine.nodes[idA] = nodeA
  engine.nodes[idB] = nodeB
  echo "[盟机合] 邻居绑定: ", idA, " <-> ", idB

# ── 心跳 ──
proc heartbeat*(engine: ref MengJiEngine; id: string) =
  if not engine.nodes.hasKey(id): return
  var node = engine.nodes[id]
  node.heartbeat = epochTime()
  node.alive = true
  engine.nodes[id] = node

# ── Gossip PUSH ──
proc gossipPush*(engine: ref MengJiEngine; id: string) =
  if not engine.nodes.hasKey(id): return
  let node = engine.nodes[id]
  for nbId in node.neighborIds:
    if not engine.nodes.hasKey(nbId): continue
    let nb = engine.nodes[nbId]
    # 简单比较: 仅记计数
    engine.pushCount += 1
    verdictDiff(node, nb)

# ── 裁定差异比对 ──
proc verdictDiff*(a: MengJiNode; b: MengJiNode): int =
  result = 0
  let minLen = min(a.verdictState.len, b.verdictState.len)
  for i in 0 ..< minLen:
    if a.verdictState[i] != b.verdictState[i]:
      result += 1

# ── Gossip PULL ──
proc gossipPull*(engine: ref MengJiEngine; id: string) =
  if not engine.nodes.hasKey(id): return
  var node = engine.nodes[id]
  for nbId in node.neighborIds:
    if not engine.nodes.hasKey(nbId): continue
    let nb = engine.nodes[nbId]
    # 同步: 邻居的裁定态更多则补足
    if nb.verdictState.len > node.verdictState.len:
      for i in node.verdictState.len ..< nb.verdictState.len:
        node.verdictState.add(nb.verdictState[i])
      engine.nodes[id] = node
    engine.pullCount += 1

# ── PUSH推拉一轮 ──
proc gossipRound*(engine: ref MengJiEngine) =
  engine.globalTick += 1
  echo "\n[盟机合] ═══ 第 ", engine.globalTick, " 轮 PUSH推拉 ═══"
  echo "[盟机合] -- PUSH 广播阶段 --"
  let keys = toSeq(engine.nodes.keys)
  for k in keys:
    gossipPush(engine, k)
  echo "[盟机合] -- PULL 拉取阶段 --"
  for k in keys:
    gossipPull(engine, k)
  echo "[盟机合] 第 ", engine.globalTick, " 轮完毕. PUSH=", engine.pushCount, " PULL=", engine.pullCount

# ── 三元见证裁定 ──
proc consensusVote*(engine: ref MengJiEngine; slot: int;
                   vote: MengJiVerdict; voterId: string): string =
  if not engine.nodes.hasKey(voterId): return "节点不存在"

  var entry: MengJiVerdictEntry
  let key = "slot_" & $slot
  if engine.verdicts.hasKey(key):
    entry = engine.verdicts[key]
  else:
    entry.slot = slot
    entry.subject = "裁定主题_" & $slot
    entry.verdict = mjPending
    entry.witnesses = @[]
    entry.consensusReached = false

  # 记录见证者
  if voterId notin entry.witnesses and entry.witnesses.len < 3:
    entry.witnesses.add(voterId)

  # 计算投票
  var votePass = 0
  var voteReject = 0
  for wid in entry.witnesses:
    if not engine.nodes.hasKey(wid): continue
    let w = engine.nodes[wid]
    if slot >= w.verdictState.len: continue
    # 夫妻主权人锚点权重 2x
    let weight = (w.anchor == engine.primaryAnchor) ? 2 else: 1
    if w.verdictState[slot] == mjPass:   votePass += weight
    if w.verdictState[slot] == mjReject: voteReject += weight

  # 共识阈值: ≥3 票(锚点 2 + 普通 1)
  if votePass >= 3:
    entry.verdict = mjPass
    entry.consensusReached = true
  elif voteReject >= 3:
    entry.verdict = mjReject
    entry.consensusReached = true
  else:
    entry.verdict = mjSplit

  entry.decidedAt = epochTime()
  engine.verdicts[key] = entry

  let result = "主题#" & $slot & " " & voterId & " 投票 " & verdictName(vote) &
               " → 许可=" & $votePass & " 拒绝=" & $voteReject &
               " 结论=" & verdictName(entry.verdict) &
               (if entry.consensusReached: " ★共识达成" else: " 待更多见证")
  echo "[见证] ", result
  return result

# ── 冲突裁决 ──
proc conflictResolve*(engine: ref MengJiEngine; slot: int): string =
  echo "\n[冲突裁决] 主题#", slot
  # 收集节点裁定
  var anchorVerdict = mjPending
  var hasAnchor = false
  var bestPhongTrust = -1.0
  var bestPhongVerdict = mjPending

  for id, node in engine.nodes:
    if not node.alive: continue
    if slot >= node.verdictState.len: continue
    let v = node.verdictState[slot]
    if node.anchor == engine.primaryAnchor:
      anchorVerdict = v
      hasAnchor = true
    if node.nodeType == mjPhong and node.trust > bestPhongTrust:
      bestPhongTrust = node.trust
      bestPhongVerdict = v

  var resolved: MengJiVerdict
  if hasAnchor:
    resolved = anchorVerdict
    echo "[冲突裁决] 取夫妻主权人锚点(", anchorName(engine.primaryAnchor), ")裁定=", verdictName(resolved)
  elif bestPhongTrust >= 0:
    resolved = bestPhongVerdict
    echo "[冲突裁决] 取最高信任合掌(信任=", int(bestPhongTrust), "%)裁定=", verdictName(resolved)
  else:
    resolved = mjPending
    echo "[冲突裁决] 无足够见证者,保守取待定"

  # 广播到的全部节点
  for id, node in engine.nodes:
    if slot < node.verdictState.len:
      var n = node
      n.verdictState[slot] = resolved
      engine.nodes[id] = n

  let key = "slot_" & $slot
  if engine.verdicts.hasKey(key):
    var e = engine.verdicts[key]
    e.verdict = resolved
    e.consensusReached = (resolved != mjSplit)
    engine.verdicts[key] = e

  return verdictName(resolved)

# ── 状态概览 ──
proc status*(engine: ref MengJiEngine) =
  echo "\n=== 盟机合 · 全局概览 ==="
  echo "节点数:     ", engine.nodes.len
  echo "裁定条目:   ", engine.verdicts.len
  echo "Gossip 轮:  ", engine.globalTick
  echo "PUSH 计数:  ", engine.pushCount
  echo "PULL 计数:  ", engine.pullCount
  echo "主锚点:     ", anchorName(engine.primaryAnchor)
  echo ""
  for id, node in engine.nodes:
    echo "  节点 ", id[0..7], "... ", node.name,
        " | ", nodeTypeName(node.nodeType),
        " | ", anchorName(node.anchor),
        " | trust=", int(node.trust), "%",
        " | ", (if node.alive: "存活" else: "失活")

# ── Demo ──
proc demo*() =
  echo "═══ 盟机合 · CLI 测试 Demo ═══\n"

  var engine = newMengJiEngine()

  # M1: 注册 3 节点
  echo "─── 场景 M1: 节点注册 ───"
  let n0 = registerNode(engine, "hezhang_root", mjPhong, mjLolict)
  let n1 = registerNode(engine, "farm_node_A", mjFarm, mjLiutiantian)
  let n2 = registerNode(engine, "farm_node_B", mjFarm, mjLolict)

  # M2: 邻居绑定
  echo "\n─── 场景 M2: 邻居拓扑 ───"
  bindNeighbor(engine, n0, n1)
  bindNeighbor(engine, n1, n2)
  bindNeighbor(engine, n2, n0)

  # M3: 设定裁定态
  echo "\n─── 场景 M3: 设定裁定态(分歧) ───"
  engine.nodes[n0].verdictState = @[mjPass]
  engine.nodes[n1].verdictState = @[mjReject]
  engine.nodes[n2].verdictState = @[mjPass]

  # M4: Gossip PUSH推拉
  echo "\n─── 场景 M4: Gossip PUSH推拉 ───"
  gossipRound(engine)

  # M5: 冲突裁决
  echo "\n─── 场景 M5: 冲突裁决 ───"
  discard conflictResolve(engine, 0)

  # M6: 三元见证
  echo "\n─── 场景 M6: 三元见证裁定 ───"
  discard consensusVote(engine, 0, mjPass, n0)
  discard consensusVote(engine, 0, mjReject, n1)
  discard consensusVote(engine, 0, mjPass, n2)

  # M7: 状态概览
  echo "\n─── 场景 M7: 全局概览 ───"
  status(engine)

  echo "\n[盟机合] Demo 完毕."

# ── CLI 入口 ──
when isMainModule:
  var
    cmd = ""
    nodeName = "盟机_node"
    nodeTypeStr = "FARM"
    anchorVal = 1
    rounds = 1
    subjectSlot = 0

  var p = initOptParser(commandLineParams())
  for kind, key, val in p.getopt():
    case kind
    of cmdArgument:
      if cmd == "": cmd = key
    of cmdLongOption, cmdShortOption:
      case key
      of "name":    nodeName = val
      of "type":    nodeTypeStr = val
      of "anchor":  anchorVal = parseInt(val)
      of "rounds":  rounds = parseInt(val)
      of "subject": subjectSlot = parseInt(val)
      of "help":
        echo "盟机合 CLI. 用法: mengji_tool.nim <cmd> [--opts]"
        echo "命令: register / gossip / consensus / conflict / status / demo"
        quit(0)
    of cmdEnd: discard

  if cmd == "":
    echo "盟机合 CLI. 用法: mengji_tool.nim <register|gossip|consensus|conflict|status|demo> [--opts]"
    quit(0)

  var engine = newMengJiEngine()

  case cmd
  of "register":
    let ntype = if nodeTypeStr.toUpper() == "PHONG": mjPhong else: mjFarm
    let anchor = if anchorVal == 0: mjLiutiantian else: mjLolict
    let id = registerNode(engine, nodeName, ntype, anchor)
    if id != "": echo "注册成功: ", id

  of "gossip":
    # 先注册模拟节点
    let n0 = registerNode(engine, "sim_hezhang", mjPhong, mjLolict)
    let n1 = registerNode(engine, "sim_farm_A", mjFarm, mjLiutiantian)
    let n2 = registerNode(engine, "sim_farm_B", mjFarm, mjLolict)
    bindNeighbor(engine, n0, n1)
    bindNeighbor(engine, n1, n2)
    engine.nodes[n0].verdictState = @[mjPass]
    engine.nodes[n1].verdictState = @[mjReject]
    engine.nodes[n2].verdictState = @[mjPass]
    for i in 1..rounds:
      gossipRound(engine)
    status(engine)

  of "consensus":
    let n0 = registerNode(engine, "sim_hezhang", mjPhong, mjLolict)
    let n1 = registerNode(engine, "sim_farm_A", mjFarm, mjLiutiantian)
    let n2 = registerNode(engine, "sim_farm_B", mjFarm, mjLolict)
    engine.nodes[n0].verdictState = @[mjPass]
    engine.nodes[n1].verdictState = @[mjReject]
    engine.nodes[n2].verdictState = @[mjPass]
    discard consensusVote(engine, subjectSlot, mjPass, n0)
    discard consensusVote(engine, subjectSlot, mjReject, n1)
    discard consensusVote(engine, subjectSlot, mjPass, n2)
    status(engine)

  of "conflict":
    let n0 = registerNode(engine, "sim_hezhang", mjPhong, mjLolict)
    let n1 = registerNode(engine, "sim_farm_A", mjFarm, mjLiutiantian)
    let n2 = registerNode(engine, "sim_farm_B", mjFarm, mjLolict)
    engine.nodes[n0].verdictState = @[mjPass]
    engine.nodes[n1].verdictState = @[mjReject]
    engine.nodes[n2].verdictState = @[mjPass]
    discard conflictResolve(engine, subjectSlot)
    status(engine)

  of "status":
    echo "盟机合 CLI 已就绪(无持久节点). 用 demo 查看内置演示."

  of "demo":
    demo()

  else:
    echo "未知命令: ", cmd
