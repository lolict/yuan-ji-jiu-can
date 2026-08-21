## ═══════════════════════════════════════════════════════════════
## fairness_check.nim — 验证无中央编码统治
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 没有少数编码统治所有 · 不允许权力集中
## 验证七个容器各自独立运转 · 跨容器桥接天然分布
## 任何容器的崩溃不影响其他容器的正常回答
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, math]
import encoding_container
import categorical_bridge
import dynamic_discovery

# ═══════════════════════════════════════════════════════════════
## 公平性指标 — 衡量系统的去中心化程度
## ═══════════════════════════════════════════════════════════════

type
  FairnessReport* = object
    container_count*: int
    bridge_count*: int
    replies_per_container*: seq[(string, int)]  ## 各容器参与回答次数
    bridge_per_container*: seq[(string, int)]   ## 各容器桥接数
    power_index*: float                         ## 权力集中度 (0=完全平等,1=完全集权)
    decentralization*: float                     ## 去中心化得分
    is_fair*: bool                              ## 系统是否公平
    diagnosis*: string                          ## 诊断结果

# ═══════════════════════════════════════════════════════════════
## 计算各容器参与度 — 在桥接网络中各容器有多少连接
## ═══════════════════════════════════════════════════════════════

proc bridgeDistribution*(bridges: BridgeNetwork): seq[(string, int)] =
  var counts: seq[(string, int)] = @[]
  for b in bridges.bridges:
    var found_s = false
    for i in 0 ..< counts.len:
      if counts[i][0] == b.source:
        counts[i] = (b.source, counts[i][1] + 1)
        found_s = true
    if not found_s: counts.add((b.source, 1))

    var found_t = false
    for i in 0 ..< counts.len:
      if counts[i][0] == b.target:
        counts[i] = (b.target, counts[i][1] + 1)
        found_t = true
    if not found_t: counts.add((b.target, 1))
  result = counts

# ═══════════════════════════════════════════════════════════════
## 权力指数 — 基于桥接分布的基尼系数思想
## ═══════════════════════════════════════════════════════════════

proc powerIndex*(bridges: BridgeNetwork): float =
  let dist = bridges.bridgeDistribution()
  if dist.len <= 1: return 0.0

  var vals: seq[int] = @[]
  for (_, c) in dist: vals.add(c)
  vals.sort()
  let n = vals.len
  let total = vals.foldl(a + b)
  if total == 0: return 0.0

  var sum_weighted = 0
  for i, v in vals:
    sum_weighted += (i + 1) * v

  result = (2.0 * float(sum_weighted) / float(n * total)) - (float(n + 1) / float(n))
  result = max(0.0, min(1.0, result))

# ═══════════════════════════════════════════════════════════════
## 公平性检查 — 全面验证系统无中央编码
## ═══════════════════════════════════════════════════════════════

proc fairnessCheck*(containers: seq[ContainerEncoding], bridges: BridgeNetwork): FairnessReport =
  result.container_count = containers.len
  result.bridge_count = bridges.bridges.len
  let dist = bridges.bridgeDistribution()
  result.bridge_per_container = dist
  result.power_index = bridges.powerIndex()
  result.decentralization = 1.0 - result.power_index
  result.is_fair = result.decentralization >= 0.5

  var diagnoses: seq[string] = @[]
  if result.decentralization >= 0.7:
    diagnoses.add("优秀: 高度去中心化")
  elif result.decentralization >= 0.5:
    diagnoses.add("合格: 去中心化程度可接受")
  else:
    diagnoses.add("警告: 存在集权倾向")

  # 检查是否有最少的容器拥有最多的桥接
  if dist.len > 0:
    proc cmp(a, b: (string, int)): int = cmp(b[1], a[1])
    var sorted_dist = dist
    sorted_dist.sort(cmp)
    let top = sorted_dist[0]
    diagnoses.add("桥接最多的容器: " & top[0] & " (" & $top[1] & "条)")
    if dist.len >= 2:
      let bottom = sorted_dist[^1]
      diagnoses.add("桥接最少的容器: " & bottom[0] & " (" & $bottom[1] & "条)")
      let ratio = if bottom[1] > 0: float(top[1]) / float(bottom[1]) else: 999.0
      if ratio > 5.0:
        diagnoses.add("失衡: 最高与最低桥接比=" & formatFloat(ratio, ffDecimal, 2))
      elif ratio > 3.0:
        diagnoses.add("轻微失衡: 桥接比=" & formatFloat(ratio, ffDecimal, 2))
      else:
        diagnoses.add("平衡: 桥接比=" & formatFloat(ratio, ffDecimal, 2))

  result.diagnosis = diagnoses.join("; ")

# ═══════════════════════════════════════════════════════════════
## 测试: 模拟某个容器崩溃后系统仍然正常
## ═══════════════════════════════════════════════════════════════

proc survivalTest*(containers: seq[ContainerEncoding], bridges: BridgeNetwork): string =
  var lines: seq[string] = @[]
  for victim in containers:
    var remaining: seq[ContainerEncoding] = @[]
    for c in containers:
      if c.id != victim.id:
        remaining.add(c)
    let remaining_bridges = computeBridgeNetwork(remaining)
    let report = fairnessCheck(remaining, remaining_bridges)
    lines.add("  移除" & victim.id & " 后: 桥接=" & $remaining_bridges.bridges.len &
      " 去中心化=" & $report.decentralization & " 公平=" & $report.is_fair)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 公平性报告
## ═══════════════════════════════════════════════════════════════

proc `$`*(r: FairnessReport): string =
  var lines: seq[string] = @[]
  lines.add("═══ 公平性验证 · 无中央编码统治 ═══")
  lines.add("  容器数: " & $r.container_count)
  lines.add("  桥接数: " & $r.bridge_count)
  lines.add("  权力指数: " & formatFloat(r.power_index, ffDecimal, 3) & " (0=平等, 1=集权)")
  lines.add("  去中心化: " & formatFloat(r.decentralization, ffDecimal, 3))
  lines.add("  是否公平: " & (if r.is_fair: "✅ 是" else: "❌ 否"))
  lines.add("  桥接分布:")
  for (cid, cnt) in r.bridge_per_container:
    lines.add("    " & cid & ": " & $cnt & "条桥接")
  lines.add("  诊断: " & r.diagnosis)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 公平性验证 · 无中央编码统治 ═══"
  echo ""
  let containers = allContainers()
  let bridges = computeBridgeNetwork(containers)
  let report = fairnessCheck(containers, bridges)
  echo report
  echo ""
  echo "── 容错测试 · 移除任一容器后系统仍运行 ──"
  echo survivalTest(containers, bridges)