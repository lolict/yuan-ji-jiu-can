## ═══════════════════════════════════════════════════════════════
## master_registry.nim — 满全法全部容器元索引
## ═══════════════════════════════════════════════════════════════
##
## 所有容器(音调/韵母/声母/分身/备用/复韵/鼻音/辅音/颜色/空间/镜像/音乐/时钟)的统一索引
## 跨容器关系 · 进制对照 · 版本选择 · 去中心化验证
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm]

type
  RegistryEntry* = object
    id*: string
    name*: string
    base*: int
    kind*: string         # 音调/韵母/声母/分身/备用/复韵母/鼻韵母/辅音/颜色/空间/镜像/音乐/时钟
    symbol_count*: int
    version*: string
    container_file*: string

  MasterRegistry* = object
    entries*: seq[RegistryEntry]
    total_symbols*: int
    total_containers*: int
    decentralization*: float

# ═══════════════════════════════════════════════════════════════
## 全部容器注册表 (27个容器)
## ═══════════════════════════════════════════════════════════════

proc buildRegistry*(): MasterRegistry =
  result.total_containers = 0
  result.total_symbols = 0

  # ── 音调类型 (2) ──
  result.entries.add(RegistryEntry(
    id: "FQ-TONE60", name: "音调类型-60", base: 60, kind: "音调",
    symbol_count: 60, version: "v1.0", container_file: "symbol_classifier.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "FQ-ATONE50", name: "高级音调-100", base: 100, kind: "音调",
    symbol_count: 100, version: "v2.0", container_file: "symbol_classifier.nim",
  ))

  # ── 韵母类型 (7) ──
  const ym_entries = [
    ("FQ-YM-a", "a韵母族", 28), ("FQ-YM-o", "o韵母族", 28),
    ("FQ-YM-e", "e韵母族", 26), ("FQ-YM-i", "i韵母族", 26),
    ("FQ-YM-u", "u韵母族", 26), ("FQ-YM-ü", "ü韵母族", 7),
    ("FQ-YM-CMPD", "复韵母", 49),
  ]
  for tup in ym_entries:
    let bid = tup[0]
    let bname = tup[1]
    let bcount = tup[2]
    result.entries.add(RegistryEntry(
      id: bid, name: bname, base: bcount, kind: "韵母",
      symbol_count: bcount, version: "v1.0", container_file: "symbol_classifier.nim",
    ))

  # ── 声母 (3) ──
  result.entries.add(RegistryEntry(
    id: "FQ-SM48", name: "声母-48", base: 48, kind: "声母",
    symbol_count: 48, version: "v1.5", container_file: "encoding_container.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "FQ-SM50P", name: "声母-50扩展", base: 50, kind: "声母",
    symbol_count: 50, version: "v2.0", container_file: "symbol_classifier.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "FQ-SPCON", name: "特殊辅音", base: 60, kind: "辅音",
    symbol_count: 60, version: "v1.0", container_file: "symbol_classifier.nim",
  ))

  # ── 鼻韵母 (1) ──
  result.entries.add(RegistryEntry(
    id: "FQ-NASAL", name: "鼻韵母", base: 85, kind: "鼻韵母",
    symbol_count: 85, version: "v1.0", container_file: "container_special.nim",
  ))

  # ── 分身 (1) ──
  result.entries.add(RegistryEntry(
    id: "FQ-CLONE", name: "分身容器", base: 11, kind: "分身",
    symbol_count: 11, version: "v1.0", container_file: "symbol_classifier.nim",
  ))

  # ── 备用 (3) ──
  const spare_entries = [
    ("FQ-SPARE1", "备用集1-鼻音", 70),
    ("FQ-SPARE2", "备用集2-复韵", 29),
    ("FQ-SPARE3", "备用集3-特殊", 31),
  ]
  for tup in spare_entries:
    let sid = tup[0]
    let sname = tup[1]
    let scount = tup[2]
    result.entries.add(RegistryEntry(
      id: sid, name: sname, base: scount, kind: "备用",
      symbol_count: scount, version: "v1.x", container_file: "symbol_classifier.nim",
    ))

  # ── 七层系统 (7) ──
  result.entries.add(RegistryEntry(
    id: "Clock-60", name: "时钟层-60", base: 60, kind: "时钟",
    symbol_count: 60, version: "v1.0", container_file: "clock_layer.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "Color-42", name: "颜色层-42", base: 42, kind: "颜色",
    symbol_count: 42, version: "v1.0", container_file: "color42.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "Space-18", name: "空间层-18", base: 18, kind: "空间",
    symbol_count: 18, version: "v1.0", container_file: "space_phase.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "Mirror-10", name: "镜像层-10", base: 10, kind: "镜像",
    symbol_count: 10, version: "v1.0", container_file: "mirror_2d.nim",
  ))
  result.entries.add(RegistryEntry(
    id: "Music-3", name: "音乐层-3", base: 3, kind: "音乐",
    symbol_count: 3, version: "v1.0", container_file: "music_tier.nim",
  ))

  # ── 统计 ──
  result.total_containers = result.entries.len
  for e in result.entries:
    result.total_symbols += e.symbol_count
  result.decentralization = 0.857  # 从 fairness_check 验证得到

# ═══════════════════════════════════════════════════════════════
## 注册表报告
## ═══════════════════════════════════════════════════════════════

proc registryReport*(reg: MasterRegistry): string =
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════")
  lines.add("  满全法全容器元索引")
  lines.add("═══════════════════════════════════════════════════")
  lines.add("  容器总数: " & $reg.total_containers)
  lines.add("  符号总数: " & $reg.total_symbols)
  lines.add("  去中心化: " & $reg.decentralization)
  lines.add("")

  # 按kind分组
  var byKind: seq[(string, seq[RegistryEntry])] = @[]
  for e in reg.entries:
    var found = false
    for i, (k, es) in byKind:
      if k == e.kind:
        byKind[i] = (k, es & @[e])
        found = true
    if not found:
      byKind.add((e.kind, @[e]))

  for (kind, es) in byKind:
    var total_kind: int
    for e in es: total_kind += e.symbol_count
    lines.add("── " & kind & " (" & $es.len & "容器, " & $total_kind & "符号) ──")
    for e in es:
      lines.add("  " & e.id & "  base=" & $e.base & "  v=" & e.version &
        "  sym=" & $e.symbol_count & "  ← " & e.container_file)
    lines.add("")

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
when isMainModule:
  let reg = buildRegistry()
  echo registryReport(reg)