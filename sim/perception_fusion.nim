## ═══════════════════════════════════════════════════════════════
## perception_fusion.nim — 感知一体化 · 七种感知融合
## ═══════════════════════════════════════════════════════════════
##
## 核心: 看/听/触/味/嗅/心/身 · 七种感知 · 一体化处理
##       每种感知是一个维度 · 七维数据 = 一个感知实体
##       感知即存储 · 存储即记忆 · 记忆即心算

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  PerceptionType* = enum
    ptSight = 0,    # 看
    ptSound = 1,    # 听
    ptTouch = 2,    # 触
    ptTaste = 3,    # 味
    ptSmell = 4,    # 嗅
    ptMind = 5,     # 心
    ptBody = 6      # 身

  PerceptionData* = object
    kind*: PerceptionType
    value*: float       # 感知强度 0-10
    source*: string     # 来源符号
    description*: string

  PerceptionEntity* = object
    perceptions*: array[7, PerceptionData]   # 七种感知
    fused_value*: float                      # 融合后的值
    entity_name*: string

  PerceptionFusion* = object
    entities*: seq[PerceptionEntity]
    index*: Table[string, int]
    name*: string

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newPerceptionFusion*(): PerceptionFusion =
  result.entities = @[]
  result.index = init_table[string, int]()
  result.name = "感知一体化"

# ═══════════════════════════════════════════════════════════════
## 创建感知实体
# ═══════════════════════════════════════════════════════════════

proc create_entity*(pf: var PerceptionFusion, name: string): int =
  var entity: PerceptionEntity
  entity.entity_name = name
  entity.fused_value = 0.0
  
  for i in 0..6:
    var pd: PerceptionData
    pd.kind = PerceptionType(i)
    pd.value = 0.0
    pd.source = ""
    pd.description = ""
    entity.perceptions[i] = pd
  
  pf.entities.add(entity)
  pf.index[name] = pf.entities.high
  result = pf.entities.high

# ═══════════════════════════════════════════════════════════════
## 录入感知
# ═══════════════════════════════════════════════════════════════

proc input_perception*(pf: var PerceptionFusion, entity_name: string, kind: PerceptionType, value: float, source, description: string) =
  if not pf.index.has_key(entity_name):
    discard pf.create_entity(entity_name)
  
  let idx = pf.index[entity_name]
  pf.entities[idx].perceptions[ord(kind)] = PerceptionData(
    kind: kind,
    value: value,
    source: source,
    description: description
  )

# ═══════════════════════════════════════════════════════════════
## 融合计算
# ═══════════════════════════════════════════════════════════════

proc fuse*(pf: var PerceptionFusion, entity_name: string): float =
  if not pf.index.has_key(entity_name):
    return 0.0
  
  let idx = pf.index[entity_name]
  var total = 0.0
  var active_count = 0
  
  for i in 0..6:
    let p = pf.entities[idx].perceptions[i]
    if p.value > 0:
      total += p.value
      active_count += 1
  
  if active_count > 0:
    pf.entities[idx].fused_value = total / float(active_count)
  else:
    pf.entities[idx].fused_value = 0.0
  
  result = pf.entities[idx].fused_value

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc perception_name*(pt: PerceptionType): string =
  case pt
  of ptSight: "看"
  of ptSound: "听"
  of ptTouch: "触"
  of ptTaste: "味"
  of ptSmell: "嗅"
  of ptMind: "心"
  of ptBody: "身"

proc report*(pf: PerceptionFusion): string =
  var lines: seq[string] = @[]
  lines.add("═══ 感知一体化 · 七种感知融合 ═══")
  lines.add("  实体数: " & $pf.entities.len)
  lines.add("")
  
  for entity in pf.entities:
    lines.add("── " & entity.entity_name & " (融合值=" & formatFloat(entity.fused_value, ffDecimal, 2) & ") ──")
    for i in 0..6:
      let p = entity.perceptions[i]
      if p.value > 0:
        lines.add("  " & perception_name(p.kind) & ": " & $p.value & " (" & p.description & ")")
    lines.add("")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var pf = newPerceptionFusion()
  
  echo "═══ 感知一体化验证 ═══"
  echo ""
  
  # 录入"水"的七种感知
  pf.input_perception("水", ptSight, 8.0, "水", "看起来透明")
  pf.input_perception("水", ptSound, 6.0, "水", "听起来流动")
  pf.input_perception("水", ptTouch, 9.0, "水", "摸起来凉爽")
  pf.input_perception("水", ptTaste, 3.0, "水", "尝起来无味")
  pf.input_perception("水", ptSmell, 2.0, "水", "闻起来无臭")
  pf.input_perception("水", ptMind, 7.0, "水", "想到生命之源")
  pf.input_perception("水", ptBody, 5.0, "水", "身体需要水")
  
  # 录入"火"的七种感知
  pf.input_perception("火", ptSight, 9.0, "火", "看起来明亮")
  pf.input_perception("火", ptSound, 7.0, "火", "听起来噼啪")
  pf.input_perception("火", ptTouch, 10.0, "火", "摸起来灼热")
  pf.input_perception("火", ptTaste, 0.0, "火", "不能尝")
  pf.input_perception("火", ptSmell, 6.0, "火", "闻起来焦味")
  pf.input_perception("火", ptMind, 8.0, "火", "想到毁灭与温暖")
  pf.input_perception("火", ptBody, 4.0, "火", "身体畏惧火")
  
  # 融合
  discard pf.fuse("水")
  discard pf.fuse("火")
  
  echo pf.report()