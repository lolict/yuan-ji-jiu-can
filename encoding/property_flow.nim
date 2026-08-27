## ═══════════════════════════════════════════════════════════════
## property_flow.nim — 产权流追踪 · 归属权传递 · 链条可追溯
## ═══════════════════════════════════════════════════════════════
##
## 核心: 产权流 = 公约数在多个容器间流动 · 携带身份标记
##       每一次流转都记录: 从哪来 · 到哪去 · 什么时候 · 谁授权的
##       链条可追溯 · 任何一环都可查
##

import std/[strutils, sequtils, algorithm, tables]

type
  PropertyRight* = object
    owner*: string           # 当前拥有者
    container*: string       # 所属容器
    value*: float            # 产权值
    origin*: string          # 原始来源

  FlowRecord* = object
    from_owner*: string
    to_owner*: string
    property_name*: string
    value*: float
    timestamp*: int
    authorization*: string   # 授权方式

  PropertyFlow* = object
    properties*: Table[string, PropertyRight]
    flow_history*: seq[FlowRecord]
    tick*: int

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newPropertyFlow*(): PropertyFlow =
  result.properties = init_table[string, PropertyRight]()
  result.flow_history = @[]
  result.tick = 0

# ═══════════════════════════════════════════════════════════════
## 注册产权
# ═══════════════════════════════════════════════════════════════

proc register*(pf: var PropertyFlow, name, owner, container: string, value: float, origin: string) =
  pf.properties[name] = PropertyRight(
    owner: owner,
    container: container,
    value: value,
    origin: origin
  )

# ═══════════════════════════════════════════════════════════════
## 流转: 从A到B
# ═══════════════════════════════════════════════════════════════

proc transfer*(pf: var PropertyFlow, property_name, from_owner, to_owner, authorization: string) =
  if not pf.properties.has_key(property_name):
    return
  
  let prop = pf.properties[property_name]
  if prop.owner != from_owner:
    return  # 无权转让
  
  # 记录流转
  pf.tick += 1
  pf.flow_history.add(FlowRecord(
    from_owner: from_owner,
    to_owner: to_owner,
    property_name: property_name,
    value: prop.value,
    timestamp: pf.tick,
    authorization: authorization
  ))
  
  # 更新拥有者
  pf.properties[property_name].owner = to_owner

# ═══════════════════════════════════════════════════════════════
## 查询产权历史
# ═══════════════════════════════════════════════════════════════

proc history*(pf: PropertyFlow, property_name: string): seq[FlowRecord] =
  for record in pf.flow_history:
    if record.property_name == property_name:
      result.add(record)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(pf: PropertyFlow): string =
  var lines: seq[string] = @[]
  lines.add("═══ 产权流追踪 · 归属权传递 ═══")
  lines.add("  产权数: " & $pf.properties.len)
  lines.add("  流转次数: " & $pf.flow_history.len)
  lines.add("")
  
  lines.add("── 当前产权状态 ──")
  for name, prop in pf.properties:
    lines.add("  " & name & ": 拥有者=" & prop.owner & " 容器=" & prop.container & " 值=" & $prop.value)
  lines.add("")
  
  lines.add("── 流转历史 ──")
  for record in pf.flow_history:
    lines.add("  [" & $record.timestamp & "] " & record.from_owner & " → " & record.to_owner &
      " | " & record.property_name & "(值" & $record.value & ") | 授权: " & record.authorization)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var pf = newPropertyFlow()
  
  echo "═══ 产权流追踪验证 ═══"
  echo ""
  
  # 注册产权
  pf.register("满全法", "lolict", "核心", 100.0, "创世")
  pf.register("njsg", "刘楚恬", "核心", 100.0, "创世")
  pf.register("容器A", "lolict", "外围", 50.0, "满全法")
  pf.register("容器B", "刘楚恬", "外围", 50.0, "njsg")
  
  # 流转
  pf.transfer("容器A", "lolict", "刘楚恬", "授权转让")
  pf.transfer("容器A", "刘楚恬", "lolict", "回赠")
  pf.transfer("容器B", "刘楚恬", "lolict", "交换")
  
  echo pf.report()