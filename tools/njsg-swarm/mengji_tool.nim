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
#   nim c -r mengji_tool.nim witness [--farm FID] [--subject SLOT] [--hash HASH]
#   nim c -r mengji_tool.nim verify [--subject SLOT]
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

# ── 三元见证状态 ──
type MengJiTriuneState* = enum
  mjTriuneEmpty      # 空:未出现
  mjTriuneSingle     # 单:仅一场持有
  mjTriuneDouble     # 双:两场摘要一致
  mjTriuneWitnessed  # 已见证:第三方背书
  mjTriuneRejected   # 已拒绝:摘要冲突

# ── 袋子背书记录 ──
type MengJiTriuneEntry* = object
  slot*: int
  subject*: string
  bagHash*: string
  triune*: MengJiTriuneState
  farm1*: string
  farm2*: string
  witness3rd*: string

# ── 盟机合引擎 ──
type MengJiEngine* = object
  nodes*: TableRef[string, MengJiNode]
  verdicts*: TableRef[string, MengJiVerdictEntry]
  triuneEntries*: TableRef[string, MengJiTriuneEntry]
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

proc triuneStateName*(s: MengJiTriuneState): string =
  case s
  of mjTriuneEmpty:     "空"
  of mjTriuneSingle:    "单"
  of mjTriuneDouble:    "双"
  of mjTriuneWitnessed: "已见证"
  of mjTriuneRejected:  "已拒绝"

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
  result.triuneEntries = newTable[string, MengJiTriuneEntry]()
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
  var h1 = 0x811c9dc5'i32
  var h2 = 0x0100193'i32
  for c in hashInput:
    h1 = ((h1 xor int32(c)) *% 0x01000193'i32)
    h2 = ((h2 xor (int32(c) shl 5)) *% 0x1000193'i32)
  let id = toHex(int32(h1)) & toHex(int32(h2))

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

# ── 裁定差异比对 ──
proc verdictDiff*(a: MengJiNode; b: MengJiNode): int =
  result = 0
  let minLen = min(a.verdictState.len, b.verdictState.len)
  for i in 0 ..< minLen:
    if a.verdictState[i] != b.verdictState[i]:
      result += 1

# ── Gossip PUSH ──
proc gossipPush*(engine: ref MengJiEngine; id: string) =
  if not engine.nodes.hasKey(id): return
  let node = engine.nodes[id]
  for nbId in node.neighborIds:
    if not engine.nodes.hasKey(nbId): continue
    let nb = engine.nodes[nbId]
    # 简单比较: 仅记计数
    engine.pushCount += 1
    discard verdictDiff(node, nb)

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
    let weight = (if w.anchor == engine.primaryAnchor: 2 else: 1)
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

# ── 里程碑二 · 分布式三元见证: 第三方背书 ──
#   状态流转: 空 → 单 → 双 → 已见证 / 已拒绝
proc triuneRegister*(engine: ref MengJiEngine; farmId: string;
                    slot: int; bagHash: string): MengJiTriuneState =
  if not engine.nodes.hasKey(farmId): return mjTriuneEmpty
  if bagHash.len != 16: return mjTriuneEmpty

  let key = "t_slot_" & $slot
  var entry: MengJiTriuneEntry
  entry.slot = slot
  entry.subject = "袋子_" & $slot
  entry.bagHash = bagHash
  entry.triune = mjTriuneEmpty
  entry.farm1 = ""
  entry.farm2 = ""
  entry.witness3rd = ""

  # 若已存在则读回(保留历史状态)
  if engine.triuneEntries.hasKey(key):
    entry = engine.triuneEntries[key]

  let farm = engine.nodes[farmId]
  let anchorStr = anchorName(farm.anchor)

  case entry.triune
  of mjTriuneEmpty:
    entry.farm1 = farmId
    entry.triune = mjTriuneSingle
    echo "[三元见证] 槽#", slot, ": 盟机场 ", farmId[0..7], "...(", anchorStr,
         ") 首次持有摘要=", bagHash, " → 单"
  of mjTriuneSingle:
    if farmId == entry.farm1:
      echo "[三元见证] 槽#", slot, ": 盟机场 ", farmId[0..7], " 重复,保持"
    elif entry.bagHash == bagHash:
      entry.farm2 = farmId
      entry.triune = mjTriuneDouble
      echo "[三元见证] 槽#", slot, ": 盟机场 ", farmId[0..7], "...(", anchorStr,
           ") 摘要一致(", bagHash, ") → 双"
    else:
      entry.triune = mjTriuneRejected
      echo "[三元见证] 槽#", slot, ": 盟机场 ", farmId[0..7],
           " 摘要不一致(", entry.bagHash, " vs ", bagHash, ") → 已拒绝"
  of mjTriuneDouble:
    if farmId != entry.farm1 and farmId != entry.witness3rd:
      if entry.bagHash == bagHash:
        entry.witness3rd = farmId
        entry.triune = mjTriuneWitnessed
        echo "[三元见证] 槽#", slot, ": 第三场 ", farmId[0..7], "...(", anchorStr,
             ") 背书成功(", bagHash, ") → 已见证"
      else:
        entry.triune = mjTriuneRejected
        echo "[三元见证] 槽#", slot, ": 第三场 ", farmId[0..7],
             " 摘要不一致(", entry.bagHash, " vs ", bagHash, ") → 已拒绝"
    else:
      echo "[三元见证] 槽#", slot, ": 盟机场 ", farmId[0..7], " 已参与过,忽略"
  else:
    echo "[三元见证] 槽#", slot, ": 已处于终态(", triuneStateName(entry.triune), "),不再更新"

  engine.triuneEntries[key] = entry
  result = entry.triune

# ── 里程碑二 · 分布式三元见证: 三条件验证 ──
#   条件一: 双场摘要一致  条件二: 第三方背书  条件三: 信任链相交 ≥30%
proc triuneVerify*(engine: ref MengJiEngine; slot: int): int =
  let key = "t_slot_" & $slot
  if not engine.triuneEntries.hasKey(key):
    echo "[三元验证] 槽#", slot, ": 无见证记录 → 条件一失败"
    return 1

  let entry = engine.triuneEntries[key]
  echo "[三元验证] 槽#", slot, " 状态=", triuneStateName(entry.triune)

  # 条件一: 双场摘要一致
  if entry.triune == mjTriuneEmpty:
    echo "[三元验证] 槽#", slot, " 条件一失败: 空"
    return 1
  if entry.triune == mjTriuneSingle:
    echo "[三元验证] 槽#", slot, " 条件一失败: 仅单场"
    return 1
  if entry.triune == mjTriuneRejected:
    echo "[三元验证] 槽#", slot, " 条件一失败: 已拒绝"
    return 1
  echo "[三元验证] 槽#", slot, " 条件一通过: 双场摘要一致"

  # 条件二: 第三方背书
  if entry.triune != mjTriuneWitnessed:
    echo "[三元验证] 槽#", slot, " 条件二失败: 无第三方背书"
    return 2
  if entry.witness3rd == "":
    echo "[三元验证] 槽#", slot, " 条件二失败: 第三方节点无效"
    return 2
  echo "[三元验证] 槽#", slot, " 条件二通过: 第三方(", entry.witness3rd[0..7], "...)已背书"

  # 条件三: 信任相交 = 取三方最低信任; 容差 30%
  var farmTrusts: seq[float] = @[]
  for fid in [entry.farm1, entry.farm2, entry.witness3rd]:
    if fid != "" and engine.nodes.hasKey(fid):
      farmTrusts.add(engine.nodes[fid].trust)

  if farmTrusts.len < 3:
    echo "[三元验证] 槽#", slot, " 条件三失败: 三方信任信息不全"
    return 3

  var minTrust = farmTrusts[0]
  for t in farmTrusts[1..^1]:
    if t < minTrust: minTrust = t

  echo "[三元验证] 槽#", slot, " 信任相交=", int(minTrust), "%"
  if minTrust < 30.0:
    echo "[三元验证] 槽#", slot, " 条件三失败: 信任相交 < 30%"
    return 3
  echo "[三元验证] 槽#", slot, " ★★★ 三条件全部通过 ★★★"
  return 0

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

  # M8: 三元见证(里程碑二)
  echo "\n─── 场景 M8: 三元见证 · 三方背书 ───"
  let n3 = registerNode(engine, "farm_node_C", mjFarm, mjLolict)
  bindNeighbor(engine, n2, n3)
  const bh = "deadbeefcafebabe"
  echo "[里程碑二] 三方对同一个袋子摘要=", bh, " 发起背书:"
  discard triuneRegister(engine, n0, 5, bh)
  discard triuneRegister(engine, n1, 5, bh)
  discard triuneRegister(engine, n3, 5, bh)
  let vr8 = triuneVerify(engine, 5)
  echo "[里程碑二] 槽#5 验证结果: ", (if vr8 == 0: "三条件通过 ★★★" else: "失败码=" & $vr8)

  # M9: 冲突袋子
  echo "\n─── 场景 M9: 三元见证 · 冲突拒绝 ───"
  discard triuneRegister(engine, n0, 6, "aaaaaaaaaaaaaaaa")
  discard triuneRegister(engine, n1, 6, "bbbbbbbbbbbbbbbb")
  let vr9 = triuneVerify(engine, 6)
  echo "[里程碑二] 槽#6 验证结果: ", (if vr9 == 0: "通过" else: "预期拒绝(码=" & $vr9 & ")")

  # M10: 全局概览(含见证计数)
  echo "\n─── 场景 M10: 里程碑二全局概览 ───"
  status(engine)
  echo "三元见证条目: ", engine.triuneEntries.len

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
    farmId = "sim_farm_A"
    bagHash = "deadbeefcafebabe"

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
      of "farm":    farmId = val
      of "hash":    bagHash = val
      of "help":
        echo "盟机合 CLI. 用法: mengji_tool.nim <cmd> [--opts]"
        echo "命令: register / gossip / consensus / conflict / witness / verify / status / demo"
        quit(0)
    of cmdEnd: discard

  if cmd == "":
    echo "盟机合 CLI. 用法: mengji_tool.nim <register|gossip|consensus|conflict|witness|verify|status|demo> [--opts]"
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

  of "witness":
    # 三方背书测试
    let n0 = registerNode(engine, "sim_hezhang", mjPhong, mjLolict)
    let n1 = registerNode(engine, "sim_farm_A", mjFarm, mjLiutiantian)
    let n2 = registerNode(engine, "sim_farm_B", mjFarm, mjLolict)
    echo "\n[三元见证] 三方对槽#", subjectSlot, " 摘要=", bagHash, " 背书:"
    discard triuneRegister(engine, n0, subjectSlot, bagHash)
    discard triuneRegister(engine, n1, subjectSlot, bagHash)
    discard triuneRegister(engine, n2, subjectSlot, bagHash)
    echo "\n[三元见证] 验证:"
    let vr = triuneVerify(engine, subjectSlot)
    echo "结果: ", (if vr == 0: "★★★ 三条件通过" else: "条件失败(" & $vr & ")")
    status(engine)

  of "verify":
    # 需要先有 witness 背书才能验证
    echo "注意: 先用 witness 命令背书后再 verify"
    let n0 = registerNode(engine, "sim_hezhang", mjPhong, mjLolict)
    let n1 = registerNode(engine, "sim_farm_A", mjFarm, mjLiutiantian)
    let n2 = registerNode(engine, "sim_farm_B", mjFarm, mjLolict)
    discard triuneRegister(engine, n0, subjectSlot, bagHash)
    discard triuneRegister(engine, n1, subjectSlot, bagHash)
    discard triuneRegister(engine, n2, subjectSlot, bagHash)
    let vr = triuneVerify(engine, subjectSlot)
    echo "验证结果: ", (if vr == 0: "★★★ 三条件通过 ★★★" else: "失败码=" & $vr)
    status(engine)

  of "status":
    echo "盟机合 CLI 已就绪(无持久节点). 用 demo 查看内置演示."

  of "demo":
    demo()

  else:
    echo "未知命令: ", cmd
