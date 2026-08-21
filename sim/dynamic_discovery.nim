## ═══════════════════════════════════════════════════════════════
## dynamic_discovery.nim — 容器自主发现跨容器同构映射
## ═══════════════════════════════════════════════════════════════
##
## 核心命题: 容器不需要被告知彼此的关系
## 它们通过自主探索结构来发现跨容器同构
## 每个容器发出"探测信号" · 其他容器回应 · 匹配即桥接
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, formatfloat]
import encoding_container
import categorical_bridge

# ═══════════════════════════════════════════════════════════════
## 探测信号 — 容器向其他容器发出的结构指纹
## ═══════════════════════════════════════════════════════════════

type
  ProbeSignal* = object
    source_id*: string     ## 哪个容器发出的
    fingerprint*: string   ## 结构指纹 (紧凑表示)
    dimension*: int        ## 探测哪个维度 (符号/家族/位置/极性)
    strength*: float       ## 信号强度

  DiscoveryResult* = object
    from_container*, to_container*: string   ## 发现关系的两个容器
    isomorphism*: float    ## 同构度
    evidence*: seq[string]  ## 证据链 (n个证据支持这个发现)
    confidence*: float     ## 置信度

  DiscoveryLog* = object
    results*: seq[DiscoveryResult]

# ═══════════════════════════════════════════════════════════════
## 维度枚举 — 容器可以从多个维度探测其他容器
## ═══════════════════════════════════════════════════════════════

type
  DiscoveryDimension* = enum
    dimSymbol      ## 符号重复维度
    dimFamily      ## 家族结构维度
    dimPosPattern  ## 位置模式维度
    dimPolarity    ## 极性分布维度
    dimSubseq      ## 子序列同构维度

# ═══════════════════════════════════════════════════════════════
## 生成探测指纹 — 容器从某个维度生成自己的结构指纹
## ═══════════════════════════════════════════════════════════════

proc fingerprint_symbol*(c: ContainerEncoding): string =
  ## 符号指纹: 取符号表前一半 · 用Base64表示
  let half = c.grammar.symbols.len div 2
  for i in 0 ..< half:
    result &= c.grammar.symbols[i].ch

proc fingerprint_family*(c: ContainerEncoding): string =
  ## 家族指纹: 各家族取首字
  for fam in c.grammar.families:
    if fam.len > 0:
      result &= fam[0]

proc fingerprint_position*(c: ContainerEncoding): string =
  ## 位置指纹: 位置按进制分桶后的分布
  let bucket_count = min(c.grammar.base, 10)
  var buckets = newSeq[int](bucket_count)
  for i in 0 ..< c.grammar.symbols.len:
    let b = i mod bucket_count
    buckets[b] += 1
  for b in buckets:
    result &= $b & ","

proc fingerprint_subseq*(c: ContainerEncoding, length: int = 3): string =
  ## 子序列指纹: 提取所有连续子序列的哈希
  var hashes: seq[int] = @[]
  for i in 0 ..< c.grammar.symbols.len - length:
    var h = 0
    for j in i ..< i + length:
      h = h * 31 + ord(c.grammar.symbols[j].ch[0])
    hashes.add(h)
  hashes.sort()
  result = hashes.map(proc(x: int): string = $x).join("|")

# ═══════════════════════════════════════════════════════════════
## 自主发现 — 两个容器在指定维度上相互探测 · 匹配即桥接
## ═══════════════════════════════════════════════════════════════

proc discover_dimension*(ca, cb: ContainerEncoding, dim: DiscoveryDimension): DiscoveryResult =
  result.from_container = ca.id
  result.to_container = cb.id
  var evidence: seq[string] = @[]
  var score = 0.0

  case dim
  of dimSymbol:
    ## 符号维度: 两个字面符号相同的多少
    var shared = 0
    for sa in ca.grammar.symbols:
      for sb in cb.grammar.symbols:
        if sa.ch == sb.ch:
          shared += 1
          evidence.add("共享符号 '" & sa.ch & "'")
    let ratio = float(shared) / float(min(ca.grammar.symbols.len, cb.grammar.symbols.len))
    score = ratio * 0.9

  of dimFamily:
    ## 家族维度: 家族分类的相似度
    for fa in ca.grammar.families:
      for fb in cb.grammar.families:
        if fa == fb:
          score += 0.3
          evidence.add("共享家族 '" & fa & "'")
        elif fa.len > 0 and fb.len > 0 and fa[0] == fb[0]:
          score += 0.15
          evidence.add("家族首字 '" & fa[0] & "'")

  of dimPosPattern:
    ## 位置模式: 按进制分桶后分布是否相似
    let bucket_n = min(min(ca.grammar.base, cb.grammar.base), 10)
    var buckets_a = newSeq[int](bucket_n)
    var buckets_b = newSeq[int](bucket_n)
    for i in 0 ..< ca.grammar.symbols.len: buckets_a[i mod bucket_n] += 1
    for i in 0 ..< cb.grammar.symbols.len: buckets_b[i mod bucket_n] += 1
    var diff = 0
    for i in 0 ..< bucket_n: diff += abs(buckets_a[i] - buckets_b[i])
    let max_diff = max(ca.grammar.symbols.len, cb.grammar.symbols.len)
    if max_diff > 0:
      score = 0.5 * (1.0 - float(diff) / float(max_diff))
    if score > 0.1: evidence.add("位置分桶相似度=" & $score)

  of dimPolarity:
    ## 极性分布: 阴阳中性分布是否相似
    var yang_a, yin_a, neu_a = 0
    var yang_b, yin_b, neu_b = 0
    for s in ca.grammar.symbols:
      let p = ca.grammar.polarity_rule(s)
      if p == '1': yang_a += 1
      elif p == '0': yin_a += 1
      else: neu_a += 1
    for s in cb.grammar.symbols:
      let p = cb.grammar.polarity_rule(s)
      if p == '1': yang_b += 1
      elif p == '0': yin_b += 1
      else: neu_b += 1
    let total_a = yang_a + yin_a + neu_a
    let total_b = yang_b + yin_b + neu_b
    if total_a > 0 and total_b > 0:
      let ratio_yang = abs(float(yang_a)/float(total_a) - float(yang_b)/float(total_b))
      let ratio_yin = abs(float(yin_a)/float(total_a) - float(yin_b)/float(total_b))
      score = 0.15 * (1.0 - ratio_yang) * (1.0 - ratio_yin)
    if score > 0.05: evidence.add("极性分布相似")

  of dimSubseq:
    ## 子序列维度: 是否有长度为3的相同子序列模式
    for i in 0 ..< ca.grammar.symbols.len - 3:
      let sub_a = ca.grammar.symbols[i].ch & ca.grammar.symbols[i+1].ch & ca.grammar.symbols[i+2].ch
      for j in 0 ..< cb.grammar.symbols.len - 3:
        let sub_b = cb.grammar.symbols[j].ch & cb.grammar.symbols[j+1].ch & cb.grammar.symbols[j+2].ch
        if sub_a == sub_b:
          score += 0.25
          evidence.add("子序列'" & sub_a & "'出现在两容器中")

  result.isomorphism = min(score, 1.0)
  result.evidence = evidence
  result.confidence = min(float(evidence.len) * 0.2 + score * 0.5, 1.0)

# ═══════════════════════════════════════════════════════════════
## 全维度扫描 — 对一对容器从所有五个维度扫描
## ═══════════════════════════════════════════════════════════════

proc fullScan*(ca, cb: ContainerEncoding): seq[DiscoveryResult] =
  for dim in [dimSymbol, dimFamily, dimPosPattern, dimPolarity, dimSubseq]:
    let r = discover_dimension(ca, cb, dim)
    if r.confidence > 0.1:
      result.add(r)

  proc cmp(a, b: DiscoveryResult): int = cmp(b.confidence, a.confidence)
  result.sort(cmp)

# ═══════════════════════════════════════════════════════════════
## 自主发现日志 — 对所有容器对做全维度扫描
## ═══════════════════════════════════════════════════════════════

proc autonomousDiscovery*(containers: seq[ContainerEncoding]): DiscoveryLog =
  for i in 0 ..< containers.len:
    for j in i + 1 ..< containers.len:
      let scans = fullScan(containers[i], containers[j])
      for scan in scans:
        if scan.confidence > 0.15:
          result.results.add(scan)

  proc cmp(a, b: DiscoveryResult): int = cmp(b.confidence, a.confidence)
  result.results.sort(cmp)

# ═══════════════════════════════════════════════════════════════
## 发现报告 — 人类可读的自主发现报告
## ═══════════════════════════════════════════════════════════════

proc discoveryReport*(log: DiscoveryLog): string =
  var lines: seq[string] = @[]
  lines.add("═══ 自主发现报告 · 容器间自发同构扫描 ═══")
  lines.add("发现总数: " & $log.results.len)
  lines.add("")
  for r in log.results:
    lines.add("  " & r.from_container & " → " & r.to_container & "  同构=" &
      formatFloat(r.isomorphism, ffDecimal, 3) &
      "  置信=" & formatFloat(r.confidence, ffDecimal, 3) &
      "  证据数=" & $r.evidence.len)
    for e in r.evidence:
      lines.add("    · " & e)
    lines.add("")
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 自主发现 · 容器自发扫描维度 ═══"
  echo ""
  echo "── 单个维度探测示例 ──"
  let c_sm = makeSm48()
  let c_ym = makeYm66()
  let c_mir = makeMir10()
  let c_mus = makeMus3()

  echo "SM48 → MIR10 符号探测:"
  let sym_r = discover_dimension(c_sm, c_mir, dimSymbol)
  echo "  同构=" & $sym_r.isomorphism & "  置信=" & $sym_r.confidence
  echo "  证据: " & sym_r.evidence.join("; ")

  echo ""
  echo "── 全维度扫描 SM48 ↔ MIR10 ──"
  let scans = fullScan(c_sm, c_mir)
  for s in scans:
    if s.confidence > 0.1:
      echo "  同构=" & $s.isomorphism & "  置信=" & $s.confidence & "  证据=" & $s.evidence.len

  echo ""
  echo "── 全网络自主发现 ──"
  let log = autonomousDiscovery(allContainers())
  echo log