## ═══════════════════════════════════════════════════════════════
## multi_encoding_society.nim — 多编码容器社会模拟
## ═══════════════════════════════════════════════════════════════
##
## 核心演示: 七个编码容器各自独立运作
## 没有中央编码统治所有状态
## 跨容器关系由结构同构自然涌现
## 每个容器有自己的语法、自己的家族、自己的极性
##
## ═══════════════════════════════════════════════════════════════

import std/[algorithm, strutils, sequtils, sets, formatfloat]
import encoding_container
import categorical_bridge

# ═══════════════════════════════════════════════════════════════
## 多编码社会状态
## ═══════════════════════════════════════════════════════════════

type
  MultiEncodingSociety* = object
    containers*: seq[ContainerEncoding]
    bridges*: BridgeNetwork
    tick*: int

  CrossContainerMapping* = object
    from_container*, to_container*: string
    from_sym*, to_sym*: string
    confidence*: float
    via*: string

# ═══════════════════════════════════════════════════════════════
## 创建多编码社会
## ═══════════════════════════════════════════════════════════════

proc createMultiEncodingSociety*(): MultiEncodingSociety =
  result.containers = allContainers()
  result.bridges = computeBridgeNetwork(result.containers)
  result.tick = 0

# ═══════════════════════════════════════════════════════════════
## 跨容器符号映射 — 寻找两个容器之间的自然桥接点
## 由结构同构决定 — 不靠中央规则
## ═══════════════════════════════════════════════════════════════

proc findCrossMappings*(society: MultiEncodingSociety, from_id, to_id: string): seq[CrossContainerMapping] =
  var from_idx = -1
  var to_idx = -1
  for i, c in society.containers:
    if c.id == from_id: from_idx = i
    if c.id == to_id: to_idx = i
  if from_idx < 0 or to_idx < 0: return

  let from_c = society.containers[from_idx]
  let to_c = society.containers[to_idx]

  # 1. 符号字面相同的 → 直接映射 (结构同构的最强证据)
  for s1 in from_c.grammar.symbols:
    for s2 in to_c.grammar.symbols:
      if s1.ch == s2.ch:
        result.add(CrossContainerMapping(
          from_container: from_id,
          to_container: to_id,
          from_sym: s1.ch,
          to_sym: s2.ch,
          confidence: 0.9,
          via: "符号同构"
        ))

  # 2. 家族名相同的 → 范畴桥接
  for fam in from_c.grammar.families:
    if to_c.grammar.families.contains(fam):
      let idx1 = from_c.grammar.families.find(fam)
      let idx2 = to_c.grammar.families.find(fam)
      let s1 = from_c.symAt(idx1 mod from_c.grammar.symbols.len)
      let s2 = to_c.symAt(idx2 mod to_c.grammar.symbols.len)
      result.add(CrossContainerMapping(
        from_container: from_id,
        to_container: to_id,
        from_sym: s1.ch,
        to_sym: s2.ch,
        confidence: 0.7,
        via: "家族范畴桥接(" & fam & ")"
      ))

  # 3. 位置等比映射 — 按进制比例缩放位置
  for i in 0 ..< min(from_c.grammar.symbols.len, 5):
    let ratio = float(i) / float(from_c.grammar.symbols.len)
    let j = int(ratio * float(to_c.grammar.symbols.len))
    result.add(CrossContainerMapping(
      from_container: from_id,
      to_container: to_id,
      from_sym: from_c.symAt(i).ch,
      to_sym: to_c.symAt(j).ch,
      confidence: 0.5,
      via: "等比映射(" & formatFloat(ratio, ffDecimal, 2) & ")"
    ))

# ═══════════════════════════════════════════════════════════════
## 容器间乘法联动 — 七层的统一公式但各层独立
## 址 = 各层独立计算 · 跨层桥接由同构度加权
## ═══════════════════════════════════════════════════════════════

proc unifiedAddress*(society: MultiEncodingSociety, inputs: seq[(string, string, string)]): int =
  ## inputs: [(容器ID, 符号, 极性)]
  ## 每个容器根据 согла算自己那一份 · 最后用桥接网络加权聚合
  var total = 0
  var total_weight = 0.0

  for (cont_id, sym, _) in inputs:
    var idx = -1
    for i, x in society.containers:
      if x.id == cont_id: idx = i; break
    if idx < 0: continue
    let c = society.containers[idx]
    let v = c.symToInt(sym)
    total += v * c.grammar.base

  # 用桥接网络调整
  var bridge_adj = 0.0
  for b in society.bridges.bridges:
    bridge_adj += b.score * 10.0

  let target_idx = (total + int(bridge_adj)) mod society.containers.len
  if target_idx < 0: result = 0 else: result = target_idx

# ═══════════════════════════════════════════════════════════════
## 阶和使 — 容器间的使/从关系网络
## ═══════════════════════════════════════════════════════════════

proc shicongNetwork*(society: MultiEncodingSociety): string =
  var lines: seq[string] = @[]
  for c in society.containers:
    let cong = findCongTi(c, society.containers)
    let shi = findShiTi(c, society.containers)
    if cong.len > 0 or shi.len > 0:
      lines.add("  " & c.id & " (" & $c.grammar.base & "进制):")
      for (_, desc) in cong: lines.add("    从 ← " & desc)
      for (_, desc) in shi: lines.add("    使 → " & desc)
      lines.add("")
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 社会状态报告
## ═══════════════════════════════════════════════════════════════

proc `$`*(society: MultiEncodingSociety): string =
  var lines: seq[string] = @[]
  lines.add("═══ 多编码容器社会 · 七容器独立共存 ═══")
  lines.add("容器数: " & $society.containers.len)
  lines.add("桥接数: " & $society.bridges.bridges.len)
  lines.add("")
  for c in society.containers:
    lines.add("  " & c.id & " (" & $c.grammar.base & "进制) · " &
      $c.grammar.symbols.len & "符号 · 家族: " & c.grammar.families.join(", "))
  lines.add("")
  lines.add("── 桥接矩阵 ──")
  for b in society.bridges.bridges:
    lines.add("  " & b.source & " ↔ " & b.target & "  " &
      formatFloat(b.score, ffDecimal, 3) & "  [" & kindName(b.bridge_kind) & "]  " & b.details)
  lines.add("")
  lines.add("── 进制整除 ──")
  for p in @[(48, 66), (48, 60), (66, 60), (42, 10)]:
    lines.add("  " & divisorRelation(p[0], p[1]))
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证运行
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  let society = createMultiEncodingSociety()
  echo society

  echo ""
  echo "═══ 跨容器映射示例 ═══"
  echo ""
  echo "── SM48 → YM66 ──"
  let mappings_sm_ym = society.findCrossMappings(CONTAINER_SM, CONTAINER_YM)
  for m in mappings_sm_ym:
    echo "  " & m.from_sym & "(" & m.from_container & ") → " & m.to_sym & "(" & m.to_container & ")  [" & m.via & "]"

  echo ""
  echo "── Clock-60 → Color-42 ──"
  let mappings_clk_clr = society.findCrossMappings(CONTAINER_CLK, CONTAINER_CLR)
  for m in mappings_clk_clr:
    echo "  " & m.from_sym & "(" & m.from_container & ") → " & m.to_sym & "(" & m.to_container & ")  [" & m.via & "]"

  echo ""
  echo "═══ 阶和使 — 从/使关系网络 ═══"
  echo ""
  echo society.shicongNetwork()

  echo ""
  echo "═══ 七层统一地址 ═══"
  echo ""
  let addr1 = society.unifiedAddress(@[
    (CONTAINER_SM, "b", "阳"), (CONTAINER_YM, "a", "中"),
    (CONTAINER_CLK, "i", "中"), (CONTAINER_CLR, "ạ", "阴"),
    (CONTAINER_SPC, "E", "中"), (CONTAINER_MIR, "a", "阴"),
    (CONTAINER_MUS, "原子", "中")
  ])
  echo "  统一选址: b(a)-ạ-i-E-a-原子 → 容器#" & $addr1

  let addr2 = society.unifiedAddress(@[
    (CONTAINER_SM, "t", "阴"), (CONTAINER_YM, "ü", "中"),
    (CONTAINER_CLK, "ē", "中"), (CONTAINER_CLR, "ă", "阳"),
    (CONTAINER_SPC, "W", "中"), (CONTAINER_MIR, "U", "阴"),
    (CONTAINER_MUS, "段落", "中")
  ])
  echo "  统一选址: t(ü)-ă-W-U-段落 → 容器#" & $addr2