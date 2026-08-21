## ═══════════════════════════════════════════════════════════════
## cross_query.nim — 多容器统一查询系统
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 没有中央查询器知道所有答案
## 每个容器独立回答 · 桥接层自然聚合
## 响应按同构度加权 · 每个容器的声音都被听见
##
## ═══════════════════════════════════════════════════════════════

import std/[sequtils, algorithm, strutils, math]
import encoding_container
import categorical_bridge
import dynamic_discovery

# ═══════════════════════════════════════════════════════════════
## 查询单元 — 每个容器独立回答的问题
## ═══════════════════════════════════════════════════════════════

type
  QueryUnit* = object
    container_id*: string  ## 哪个容器被问
    value*: string         ## 查询值 (如 "b", "a", "i")
    local_reply*: string   ## 本地回答 (容器自己的语言)
    local_score*: float    ## 本地置信度

  CrossQuery* = object
    question*: string      ## 全球问题
    replies*: seq[QueryUnit]  ## 各容器的本地回答
    bridges_used*: seq[StructuralIsomorphism]  ## 用于聚合的桥接
    global_answer*: string  ## 聚合后的全局回答
    decentralization*: float  ## 去中心化得分 (0=集权,1=完全去中心化)

# ═══════════════════════════════════════════════════════════════
## 容器本地回答 — 每个容器用自己的系统回答问题
## ═══════════════════════════════════════════════════════════════

proc localReply*(c: ContainerEncoding, question: string): QueryUnit =
  result.container_id = c.id
  result.value = question

  let v = c.symToInt(question)
  if v >= 0:
    let sym = c.symAt(v)
    let fam = c.familyOf(v)
    result.local_reply = c.id & " 中 '" & question & "' = 位" & $v &
      " 家族'" & fam & "' 极性'" & $c.grammar.polarity_rule(sym) & "'"
    result.local_score = 0.8
  else:
    result.local_reply = c.id & " 中 '" & question & "' 未找到 (返回默认位0)"
    result.local_score = 0.2

# ═══════════════════════════════════════════════════════════════
## 聚合权重 — 由桥接强度决定各容器的发言权重
## ═══════════════════════════════════════════════════════════════

proc aggregationWeights*(q: CrossQuery, bridges: BridgeNetwork): seq[(string, float)] =
  var weights: seq[(string, float)] = @[]
  for reply in q.replies:
    var w = 0.5  ## 基本权重
    for b in bridges.bridges:
      if b.source == reply.container_id or b.target == reply.container_id:
        w += b.score * 0.3
    weights.add((reply.container_id, w))
  # 归一化
  var total = 0.0
  for (_, w) in weights: total += w
  if total > 0:
    for i in 0 ..< weights.len:
      weights[i] = (weights[i][0], weights[i][1] / total)
  result = weights

# ═══════════════════════════════════════════════════════════════
## 去中心化得分 — 验证没有单一容器垄断答案
## ═══════════════════════════════════════════════════════════════

proc decentralizationScore*(q: CrossQuery, weights: seq[(string, float)]): float =
  if weights.len == 0: return 0.0
  # 计算权重分布的熵
  var entropy = 0.0
  for (_, w) in weights:
    if w > 0:
      entropy -= w * ln(w)
  let max_entropy = ln(float(weights.len))
  if max_entropy > 0:
    result = entropy / max_entropy
  else:
    result = 0.0

# ═══════════════════════════════════════════════════════════════
## 跨容器查询引擎
## ═══════════════════════════════════════════════════════════════

proc crossQuery*(containers: seq[ContainerEncoding],
                 bridges: BridgeNetwork,
                 question: string): CrossQuery =
  result.question = question
  for c in containers:
    result.replies.add(c.localReply(question))

  let weights = result.aggregationWeights(bridges)
  result.decentralization = result.decentralizationScore(weights)

  # 聚合全局回答
  var reply_parts: seq[string] = @[]
  for i, reply in result.replies:
    if reply.local_score > 0.3:
      let w = if i < weights.len: weights[i][1] else: 0.5
      reply_parts.add("[" & reply.container_id & ":" & $w & "] " & reply.local_reply)
  result.global_answer = reply_parts.join(" | ")

  # 收集用到的桥接
  for bl in bridges.bridges:
    var found_match = false
    for r in result.replies:
      if r.container_id == bl.source or r.container_id == bl.target:
        found_match = true; break
    if found_match:
      result.bridges_used.add(bl)

# ═══════════════════════════════════════════════════════════════
## 批量跨容器查询 — 对多个问题做跨容器回答
## ═══════════════════════════════════════════════════════════════

proc batchQuery*(containers: seq[ContainerEncoding],
                 bridges: BridgeNetwork,
                 questions: seq[string]): seq[CrossQuery] =
  for q in questions:
    result.add(crossQuery(containers, bridges, q))

# ═══════════════════════════════════════════════════════════════
## 查询报告
## ═══════════════════════════════════════════════════════════════

proc queryReport*(q: CrossQuery): string =
  var lines: seq[string] = @[]
  lines.add("── 问题: '" & q.question & "' ──")
  for reply in q.replies:
    lines.add("  " & reply.container_id & ": " & reply.local_reply)
  lines.add("  全局聚合: " & q.global_answer)
  lines.add("  去中心化: " & $q.decentralization)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 跨容器统一查询 · 无中央权威 ═══"
  echo ""
  let containers = allContainers()
  let bridges = computeBridgeNetwork(containers)
  let test_questions = @["a", "i", "H", "W"]
  for question in test_questions:
    echo crossQuery(containers, bridges, question)
    echo ""