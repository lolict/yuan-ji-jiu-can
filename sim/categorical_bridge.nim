## ═══════════════════════════════════════════════════════════════
## categorical_bridge.nim — 跨容器同构映射与范畴桥接
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 跨容器关系不是中央指定的，而是结构同构自然涌现的
## 两个容器之间的桥接由它们自己决定 — 没有外部权威
## 范畴论精神: 只关心结构映射，不关心内容是什么
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm]
import encoding_container

# ═══════════════════════════════════════════════════════════════
## 结构同构度 — 衡量两个容器在结构上的亲和程度
## 完全由容器自身的属性计算 — 无需外部知识
## ═══════════════════════════════════════════════════════════════

type
  StructuralIsomorphism* = object
    source*, target*: string   ## 容器 ID
    score*: float              ## 同构度 [0,1]
    bridge_kind*: BridgeKind   ## 桥接类型
    details*: string           ## 桥接依据

  BridgeKind* = enum
    bkSharedFamily    ## 家族共享: 两个容器都有相似家族分类
    bkBaseCompatible   ## 进制兼容: 进制之间存在公约数关系
    bkSymbolOverlap    ## 符号重复: 有相同或相似符号
    bkAngleMatch       ## 角度匹配: 家族数/进制比例相近
    bkPolarLink       ## 极性互补: 阴阳极性形成互补
    bkNoLink          ## 无桥接

  BridgeNetwork* = object
    bridges*: seq[StructuralIsomorphism]

# ═══════════════════════════════════════════════════════════════
## 公约数 — 容器间的天然亲缘关系
## ═══════════════════════════════════════════════════════════════

proc gcd*(a, b: int): int =
  var x = a
  var y = b
  while y != 0:
    let t = y
    y = x mod y
    x = t
  result = x

# ═══════════════════════════════════════════════════════════════
## 公约数整除关系 — 你的整除层级原则
## ═══════════════════════════════════════════════════════════════

proc divisorRelation*(base_a, base_b: int): string =
  let g = gcd(base_a, base_b)
  if g == base_a: return $base_a & "|" & $base_b & " → 前者是后者的公约数 → 前者是后者的从体"
  if g == base_b: return $base_b & "|" & $base_a & " → 后者是前者的公约数 → 后者是前者的从体"
  if g > 1:     return $base_a & "与" & $base_b & "有公约数" & $g & " → 可约分化"
  return $base_a & "与" & $base_b & "互质 → 无公约数关系"

# ═══════════════════════════════════════════════════════════════
## 跨容器桥接计算 — 每个容器和其他所有容器之间自然涌现桥接
## 由容器属性决定 — 没有外部规则
## ═══════════════════════════════════════════════════════════════

proc computeBridge*(ca, cb: ContainerEncoding): StructuralIsomorphism =
  result.source = ca.id
  result.target = cb.id
  result.score = 0.0

  # 1. 家族共享 — 检查两个容器是否有相似家族分类
  var shared_fams = 0
  for fa in ca.grammar.families:
    for fb in cb.grammar.families:
      if fa == fb: shared_fams += 1
      # 部分匹配 (包含相同关键词)
      elif fa.len > 0 and fb.len > 0:
        for kw in ["系", "音", "域"]:
          if fa.contains(kw) and fb.contains(kw) and fa == fb:
            shared_fams += 1
  if shared_fams > 0:
    result.score += 0.15 * float(shared_fams)
    result.bridge_kind = bkSharedFamily
    result.details = "共享" & $shared_fams & "个家族"

  # 2. 进制兼容 — 检查公约数关系
  let g = gcd(ca.grammar.base, cb.grammar.base)
  if g == ca.grammar.base or g == cb.grammar.base:
    result.score += 0.10
    result.details.add(" · 进制整除关系(" & $ca.grammar.base & "/" & $cb.grammar.base & ")")
    if result.bridge_kind == bkNoLink: result.bridge_kind = bkBaseCompatible
  elif g > 1:
    result.score += 0.05
    result.details.add(" · 进制公约数" & $g)
    if result.bridge_kind == bkNoLink: result.bridge_kind = bkBaseCompatible

  # 3. 符号重叠 — 检查字面符号有多少重复
  var shared_syms = 0
  for sa in ca.grammar.symbols:
    for sb in cb.grammar.symbols:
      if sa.ch == sb.ch:
        shared_syms += 1
  if shared_syms > 0:
    let overlap_ratio = float(shared_syms) / float(min(ca.grammar.symbols.len, cb.grammar.symbols.len))
    result.score += 0.20 * overlap_ratio
    if result.bridge_kind == bkNoLink: result.bridge_kind = bkSymbolOverlap
    result.details.add(" · 共享" & $shared_syms & "个符号")

  # 4. 极性互补 — 阴阳互补加成分
  if ca.grammar.polarity_rule(ca.grammar.symbols[0]) !=
     cb.grammar.polarity_rule(cb.grammar.symbols[0]):
    result.score += 0.08
    if result.bridge_kind == bkNoLink: result.bridge_kind = bkPolarLink
    result.details.add(" · 极性互补")

  result.score = min(result.score, 1.0)

# ═══════════════════════════════════════════════════════════════
## 桥接网络 — 所有容器对之间的桥接关系
## ═══════════════════════════════════════════════════════════════

proc computeBridgeNetwork*(containers: seq[ContainerEncoding]): BridgeNetwork =
  for i in 0 .. containers.high:
    for j in i + 1 ..< containers.len:
      let b = computeBridge(containers[i], containers[j])
      if b.score > 0.05:
        result.bridges.add(b)

  # 按同构度排序
  proc cmp(a, b: StructuralIsomorphism): int = cmp(b.score, a.score)
  result.bridges.sort(cmp)

# ═══════════════════════════════════════════════════════════════
## 从群到体 — 给定容器 A, 找出它在各层级上的从/使关系
## ═══════════════════════════════════════════════════════════════

proc findCongTi*(container: ContainerEncoding, all: seq[ContainerEncoding]): seq[(string, string)] =
  ## 找到 container 作为从体(被包含)的所有容器
  for other in all:
    if other.id == container.id: continue
    let g = gcd(container.grammar.base, other.grammar.base)
    if g == container.grammar.base:
      result.add((other.id, container.id & "是" & other.id & "的从体(进制整除)"))

proc findShiTi*(container: ContainerEncoding, all: seq[ContainerEncoding]): seq[(string, string)] =
  ## 找到 container 作为使体(使他人从之)的所有容器
  for other in all:
    if other.id == container.id: continue
    let g = gcd(container.grammar.base, other.grammar.base)
    if g == other.grammar.base:
      result.add((other.id, container.id & "是" & other.id & "的使体(进制使从)"))

# ═══════════════════════════════════════════════════════════════
## 桥接报告
## ═══════════════════════════════════════════════════════════════

proc `$`*(b: StructuralIsomorphism): string =
  result = b.source & " ↔ " & b.target & " | " & $b.bridge_kind & " | " & $b.score & " | " & b.details

proc kindName*(bk: BridgeKind): string =
  case bk
  of bkSharedFamily: "家族共享"
  of bkBaseCompatible: "进制兼容"
  of bkSymbolOverlap: "符号重复"
  of bkAngleMatch: "角度匹配"
  of bkPolarLink: "极性互补"
  of bkNoLink: "无桥接"

proc printBridge*(b: StructuralIsomorphism): string =
  result = b.source & " ↔ " & b.target & "  同构度=" & $b.score.formatFloat(ffDecimal, 3) & "  桥接: " & kindName(b.bridge_kind) & "  (" & b.details & ")"

# ═══════════════════════════════════════════════════════════════
## 验证运行
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 跨容器 · 结构同构桥接网络 ═══"
  echo ""
  let containers = allContainers()
  let network = computeBridgeNetwork(containers)
  echo "桥接总数: " & $network.bridges.len
  echo "── 桥接排序 ──"
  for b in network.bridges:
    echo printBridge(b)
    echo ""

  echo "── 进制整除层级 ──"
  let pairs = @[(48, 66), (48, 60), (66, 60), (42, 10), (48, 42), (10, 18)]
  for p in pairs:
    echo "  " & divisorRelation(p[0], p[1])

  echo ""
  echo "── 容器的从/使关系 ──"
  for c in containers:
    let cong = findCongTi(c, containers)
    let shi = findShiTi(c, containers)
    if cong.len > 0 or shi.len > 0:
      echo "  " & c.id & ":"
      for (rel, desc) in cong: echo "    从: " & desc
      for (rel, desc) in shi: echo "    使: " & desc