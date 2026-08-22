## ═══════════════════════════════════════════════════════════════
## container_dna.nim — 容器进化血统与家族树
## ═══════════════════════════════════════════════════════════════
##
## 每个容器有自己的DNA: 父容器、突变历史、版本进化、家族关系
## 没有中央编码统治 — 每个容器通过血统自然关联
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm]

type
  ContainerDNA* = object
    id*: string
    name*: string
    generation*: int         # 第几代 (0=原始, 1=第一代子, ...)
    parent_id*: string       # 父容器ID
    children*: seq[string]   # 子容器列表
    birth_version*: string   # 诞生版本
    current_version*: string
    mutations*: seq[string]  # 突变历史
    family*: string          # 所属家族
    sibling*: seq[string]    # 同级容器 (同父)

  FamilyTree* = object
    containers*: seq[ContainerDNA]
    root*: string            # 始祖容器ID

# ═══════════════════════════════════════════════════════════════
## 全部容器的DNA定义
## ═══════════════════════════════════════════════════════════════

proc buildDNA*(): FamilyTree =
  result.root = "FQ-ROOT"

  # ═══════════════ 第0代: 始祖 ═══════════════
  result.containers.add(ContainerDNA(
    id: "FQ-ROOT", name: "满全法根容器", generation: 0,
    parent_id: "NONE", children: @["FQ-TONE60", "FQ-SM48", "FQ-YM66", "Clock-60"],
    birth_version: "v0.1", current_version: "v0.1",
    mutations: @["创世: 定义基础编码规则"],
    family: "ROOT", sibling: @[],
  ))

  # ═══════════════ 第1代: 四大基础容器 ═══════════════
  result.containers.add(ContainerDNA(
    id: "FQ-TONE60", name: "音调类型-60进制", generation: 1,
    parent_id: "FQ-ROOT", children: @["FQ-ATONE50"],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: 6音×5声调=30小写+30大写=60"],
    family: "TONE", sibling: @["FQ-SM48", "FQ-YM66", "Clock-60"],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-SM48", name: "声母层-48进制", generation: 1,
    parent_id: "FQ-ROOT", children: @["FQ-SM50P", "FQ-SPCON"],
    birth_version: "v1.0", current_version: "v1.5",
    mutations: @["v1.0: 24声母", "v1.5: +24大写=48"],
    family: "SHENG", sibling: @["FQ-TONE60", "FQ-YM66", "Clock-60"],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-YM66", name: "韵母层-66进制", generation: 1,
    parent_id: "FQ-ROOT", children: @["FQ-YM100", "FQ-YM-a", "FQ-YM-o", "FQ-YM-e", "FQ-YM-i", "FQ-YM-u", "FQ-YM-ü"],
    birth_version: "v1.0", current_version: "v1.5",
    mutations: @["v1.0: 66符号", "v1.5: 优化符号表"],
    family: "YUNMU", sibling: @["FQ-TONE60", "FQ-SM48", "Clock-60"],
  ))

  result.containers.add(ContainerDNA(
    id: "Clock-60", name: "时钟层-60进制", generation: 1,
    parent_id: "FQ-ROOT", children: @[],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: 60秒/分"],
    family: "CLOCK", sibling: @["FQ-TONE60", "FQ-SM48", "FQ-YM66"],
  ))

  # ═══════════════ 第2代: 扩展容器 ═══════════════
  result.containers.add(ContainerDNA(
    id: "FQ-ATONE50", name: "高级音调-100进制", generation: 2,
    parent_id: "FQ-TONE60", children: @[],
    birth_version: "v1.0", current_version: "v2.0",
    mutations: @["v1.0: 5层×10=50小写", "v2.0: +50大写=100"],
    family: "TONE", sibling: @[],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-SM50P", name: "声母扩展-50进制", generation: 2,
    parent_id: "FQ-SM48", children: @[],
    birth_version: "v2.0", current_version: "v2.0",
    mutations: @["v2.0: 48+2(v/V)=50"],
    family: "SHENG", sibling: @["FQ-SPCON"],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-YM100", name: "韵母-100进制", generation: 2,
    parent_id: "FQ-YM66", children: @["FQ-YM-CMPD", "FQ-NASAL"],
    birth_version: "v2.0", current_version: "v2.0",
    mutations: @["v2.0: 50小写+50大写=100"],
    family: "YUNMU", sibling: @["FQ-YM-a", "FQ-YM-o", "FQ-YM-e", "FQ-YM-i", "FQ-YM-u", "FQ-YM-ü"],
  ))

  # ═══════════════ 第2代: 韵母六族 ═══════════════
  const base_ids = [("a","FQ-YM-a"), ("o","FQ-YM-o"), ("e","FQ-YM-e"), ("i","FQ-YM-i"), ("u","FQ-YM-u"), ("ü","FQ-YM-ü")]
  for tup in base_ids:
    let base = tup[0]
    let cid = tup[1]
    result.containers.add(ContainerDNA(
      id: cid, name: base & "韵母族", generation: 2,
      parent_id: "FQ-YM66", children: @[],
      birth_version: "v1.0", current_version: "v1.0",
      mutations: @["v1.0: 从YM66拆分, 显式枚举" & base & "全部变体"],
      family: "YUNMU", sibling: @["FQ-YM-a","FQ-YM-o","FQ-YM-e","FQ-YM-i","FQ-YM-u","FQ-YM-ü"],
    ))

  # ═══════════════ 第3代: 组合容器 ═══════════════
  result.containers.add(ContainerDNA(
    id: "FQ-YM-CMPD", name: "复韵母", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: 10组复韵母×变体=49符号"],
    family: "YUNMU", sibling: @["FQ-NASAL"],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-NASAL", name: "鼻韵母", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: 14组鼻韵母=85符号"],
    family: "YUNMU", sibling: @["FQ-YM-CMPD"],
  ))

  result.containers.add(ContainerDNA(
    id: "FQ-SPCON", name: "特殊辅音", generation: 2,
    parent_id: "FQ-SM48", children: @[],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: ⴀ-ⴿ+独立辅音=60符号"],
    family: "SHENG", sibling: @["FQ-SM50P"],
  ))

  # ═══════════════ 第3代: 分身+备用 ═══════════════
  result.containers.add(ContainerDNA(
    id: "FQ-CLONE", name: "分身容器", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.0", current_version: "v1.0",
    mutations: @["v1.0: 同形符号独立化=11符号"],
    family: "CLONE", sibling: @["FQ-SPARE1","FQ-SPARE2","FQ-SPARE3"],
  ))

  # ═══ 备用容器 (显式添加3个) ═══
  result.containers.add(ContainerDNA(
    id: "FQ-SPARE1", name: "备用集1", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.1", current_version: "v1.1",
    mutations: @["v1.1: 富余符号·视觉匹配·备用集1"],
    family: "SPARE", sibling: @["FQ-CLONE","FQ-SPARE2","FQ-SPARE3"],
  ))
  result.containers.add(ContainerDNA(
    id: "FQ-SPARE2", name: "备用集2", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.2", current_version: "v1.2",
    mutations: @["v1.2: 富余符号·视觉匹配·备用集2"],
    family: "SPARE", sibling: @["FQ-CLONE","FQ-SPARE1","FQ-SPARE3"],
  ))
  result.containers.add(ContainerDNA(
    id: "FQ-SPARE3", name: "备用集3", generation: 3,
    parent_id: "FQ-YM100", children: @[],
    birth_version: "v1.3", current_version: "v1.3",
    mutations: @["v1.3: 富余符号·视觉匹配·备用集3"],
    family: "SPARE", sibling: @["FQ-CLONE","FQ-SPARE1","FQ-SPARE2"],
  ))

# ═══════════════════════════════════════════════════════════════
## 家族树报告
## ═══════════════════════════════════════════════════════════════

proc dnaReport*(tree: FamilyTree): string =
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════")
  lines.add("  容器DNA · 进化血统与家族树")
  lines.add("═══════════════════════════════════════════════════")
  lines.add("")

  # 按代排序
  var sorted = tree.containers
  proc cmp(a, b: ContainerDNA): int = cmp(a.generation, b.generation)
  sorted.sort(cmp)

  var current_gen = -1
  for c in sorted:
    if c.generation != current_gen:
      current_gen = c.generation
      lines.add("── 第" & $current_gen & "代 ──")

    lines.add("  " & c.id & " (" & c.name & ")")
    lines.add("    父: " & c.parent_id & "  子: " & c.children.join(", "))
    lines.add("    版本: " & c.birth_version & " → " & c.current_version)
    lines.add("    突变: " & c.mutations.join(" | "))
    lines.add("    家族: " & c.family & "  同级: " & c.sibling.join(", "))
    lines.add("")

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
## ═══════════════════════════════════════════════════════════════

when isMainModule:
  let tree = buildDNA()
  echo dnaReport(tree)