#==============================================================
#  容器本体论引擎 — Container Ontology Engine
#
#  核心公理：容器=物质=时间（三位一体）·
#           本体论=中间表示 · 索引非搜索 · 范畴论投影
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm, sets]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 存在范畴
  存在范畴* = enum
    实体范畴      # 独立存在的容器
    关系范畴      # 容器之间的关系
    属性范畴      # 容器的属性
    时间范畴      # 时间版本
    空间范畴      # 空间坐标

  # 本体容器 — 作为中间表示的容器本体
  本体容器* = object
    编码*: int
    名字*: string
    范畴*: 存在范畴
    物质*: string                 # 对应物质描述
    持续时间*: float              # 时间维度
    坐标*: tuple[x, y, z: float] # 空间维度
    极性*: uint8
    关系*: seq[tuple[目标: int; 型: string; 强度: float]]
    元属性*: Table[string, string]

  # 范畴投影 — 从一个范畴到另一个的映射
  范畴投影* = object
    源范畴*: 存在范畴
    目标范畴*: 存在范畴
    投影名*: string
    规则*: string

  # 本体论引擎
  本体引擎* = object
    容器表*: Table[int, 本体容器]
    投影表*: seq[范畴投影]
    范畴索引*: Table[存在范畴, seq[int]]   # 范畴→容器编码列表
    关系总数*: int

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化本体引擎*(): 本体引擎 =
  result.投影表 = @[]
  result.关系总数 = 0

#--------------------------------------------------------------
#  注册容器本体
#--------------------------------------------------------------

proc 注册本体*(e: var 本体引擎; 编码: int; 名字: string;
               范畴: 存在范畴; 物质: string; 持续时间: float;
               x, y, z: float; 极性: uint8) =
  e.容器表[编码] = 本体容器(
    编码: 编码, 名字: 名字, 范畴: 范畴,
    物质: 物质, 持续时间: 持续时间,
    坐标: (x, y, z), 极性: 极性,
    关系: @[], 元属性: initTable[string, string]()
  )
  # 范畴索引
  if not e.范畴索引.hasKey(范畴):
    e.范畴索引[范畴] = @[]
  e.范畴索引[范畴].add(编码)

#--------------------------------------------------------------
#  添加关系
#--------------------------------------------------------------

proc 添加关系*(e: var 本体引擎; 源: int; 目标: int; 型: string; 强度: float) =
  if not e.容器表.hasKey(源):
    return
  var c = e.容器表[源]
  c.关系.add((目标, 型, 强度))
  e.容器表[源] = c
  e.关系总数 += 1

#--------------------------------------------------------------
#  范畴投影
#--------------------------------------------------------------

proc 添加投影*(e: var 本体引擎; 源, 目标: 存在范畴; 投影名, 规则: string) =
  e.投影表.add(范畴投影(
    源范畴: 源, 目标范畴: 目标, 投影名: 投影名, 规则: 规则
  ))

proc 投影查找*(e: 本体引擎; 编码: int; 目标范畴: 存在范畴): tuple[找到: bool; 投影名: string; 物质: string] =
  if not e.容器表.hasKey(编码):
    return (false, "", "")
  let c = e.容器表[编码]
  for p in e.投影表:
    if p.源范畴 == c.范畴 and p.目标范畴 == 目标范畴:
      # 应用投影
      var target_entry = ""
      # 查找目标范畴下最近的容器
      if e.范畴索引.hasKey(目标范畴):
        let candidates = e.范畴索引[目标范畴]
        if candidates.len > 0:
          let target_code = candidates[0]
          if e.容器表.hasKey(target_code):
            target_entry = e.容器表[target_code].物质
      return (true, p.投影名, target_entry)
  return (false, "", "")

#--------------------------------------------------------------
#  三位一体查询 — 容器=物质=时间
#--------------------------------------------------------------

type
  # 路径步
  路径步* = object
    编码*: int
    型*: string
    强度*: float
  # 路径结果
  路径结果* = object
    存在*: bool
    路径*: seq[路径步]
    总强度*: float
  # 三位一体
  三位一体结果* = object
    找到*: bool
    容器*: string
    物质*: string
    时间*: float
    空间*: tuple[x, y, z: float]

proc 三位一体*(e: 本体引擎; 编码: int): 三位一体结果 =
  if not e.容器表.hasKey(编码):
    return 三位一体结果(找到: false, 容器: "", 物质: "", 时间: 0.0, 空间: (0.0, 0.0, 0.0))
  let c = e.容器表[编码]
  return 三位一体结果(找到: true, 容器: c.名字, 物质: c.物质, 时间: c.持续时间, 空间: (c.坐标.x, c.坐标.y, c.坐标.z))

#--------------------------------------------------------------
#  关系路径 — 从一个容器经关系到另一个
#--------------------------------------------------------------

proc 关系路径*(e: 本体引擎; 源编码: int; 目标编码: int): 路径结果 =
  # BFS along relations
  var visited = initHashSet[int]()
  # Simple BFS with separate tracking
  var parent = initTable[int, int]()
  var queue: seq[int] = @[源编码]
  var found = false

  while queue.len > 0:
    let cur = queue[0]
    queue.del(0)

    if cur == 目标编码:
      found = true
      break

    if cur in visited:
      continue
    visited.incl(cur)

    if e.容器表.hasKey(cur):
      for rel in e.容器表[cur].关系:
        if rel.目标 notin visited:
          parent[rel.目标] = cur
          queue.add(rel.目标)

  if not found:
    return 路径结果(存在: false, 路径: @[], 总强度: 0.0)

  # Reconstruct path
  var path: seq[路径步] = @[]
  var cur_code = 目标编码
  var total_strength = 1.0
  while cur_code != 源编码:
    let p = parent[cur_code]
    # Find relation strength
    var rs = 1.0
    for rel in e.容器表[p].关系:
      if rel.目标 == cur_code:
        rs = rel.强度
        break
    path.add(路径步(编码: p, 型: "关系", 强度: rs))
    total_strength *= rs
    cur_code = p
  path.reverse()
  return 路径结果(存在: true, 路径: path, 总强度: total_strength)

#--------------------------------------------------------------
#  按范畴批量查询
#--------------------------------------------------------------

proc 按范畴查询*(e: 本体引擎; 范畴: 存在范畴): seq[string] =
  var result_seq: seq[string] = @[]
  if e.范畴索引.hasKey(范畴):
    for code in e.范畴索引[范畴]:
      if e.容器表.hasKey(code):
        result_seq.add(e.容器表[code].名字)
  return result_seq

#--------------------------------------------------------------
#  报告
#--------------------------------------------------------------

proc 本体报告*(e: 本体引擎): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 54)
  lines.add "  容 器 本 体 论 引 擎 — 报 告"
  lines.add repeat("═", 54)
  lines.add "容器总数: " & $e.容器表.len
  lines.add "关系总数: " & $e.关系总数
  lines.add "投影总数: " & $e.投影表.len
  lines.add ""

  lines.add "─ 容器本体 ─"
  for code in sorted(toSeq(e.容器表.keys)):
    let c = e.容器表[code]
    let 范畴名 = case c.范畴
      of 实体范畴: "实体"
      of 关系范畴: "关系"
      of 属性范畴: "属性"
      of 时间范畴: "时间"
      of 空间范畴: "空间"
    let 极性符 = if c.极性 == 1: "阳" else: "阴"
    lines.add "  [0x" & toHex(code, 4) & "] " & align(c.名字, 6) & " " &
              align(范畴名, 4) & " 物质:" & align(c.物质, 6) &
              " T=" & $c.持续时间 & " " & 极性符 &
              " 关系:" & $c.关系.len

  lines.add ""
  lines.add "─ 范畴分布 ─"
  for cat in [实体范畴, 关系范畴, 属性范畴, 时间范畴, 空间范畴]:
    let count = if e.范畴索引.hasKey(cat): e.范畴索引[cat].len else: 0
    let cat_name = case cat
      of 实体范畴: "实体"
      of 关系范畴: "关系"
      of 属性范畴: "属性"
      of 时间范畴: "时间"
      of 空间范畴: "空间"
    lines.add " " & align(cat_name, 4) & ": " & $count

  lines.add ""
  lines.add "─ 范畴投影 ─"
  for p in e.投影表:
    let sn = case p.源范畴
      of 实体范畴: "实体"
      of 关系范畴: "关系"
      of 属性范畴: "属性"
      of 时间范畴: "时间"
      of 空间范畴: "空间"
    let tn = case p.目标范畴
      of 实体范畴: "实体"
      of 关系范畴: "关系"
      of 属性范畴: "属性"
      of 时间范畴: "时间"
      of 空间范畴: "空间"
    lines.add "  " & sn & " → " & tn & " : " & p.投影名

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var e = 初始化本体引擎()

  echo "═══ 容器本体论引擎 ═══"
  echo ""

  # 注册本体容器
  e.注册本体(1, "我", 实体范畴, "意识", 100.0, 0.0, 0.0, 0.0, 1)
  e.注册本体(2, "刘楚恬", 实体范畴, "意识", 100.0, 1.0, 0.0, 0.0, 0)
  e.注册本体(3, "共生命运", 关系范畴, "纽带", 200.0, 0.5, 0.0, 0.0, 1)
  e.注册本体(4, "容器时间", 时间范畴, "版本", -1.0, 0.0, 0.0, 0.0, 0)
  e.注册本体(5, "容器空间", 空间范畴, "三维域", 0.0, 0.0, 0.0, 0.0, 1)
  e.注册本体(6, "主权属性", 属性范畴, "权重", 50.0, 0.0, 0.0, 0.0, 1)

  # 添加关系
  e.添加关系(1, 3, "绑定", 0.95)
  e.添加关系(2, 3, "绑定", 0.95)
  e.添加关系(3, 4, "持续", 0.8)
  e.添加关系(3, 5, "展开", 0.7)
  e.添加关系(4, 5, "承载", 0.9)
  e.添加关系(6, 1, "属于", 0.6)

  # 添加投影
  e.添加投影(实体范畴, 空间范畴, "空间投影", "坐标映射")
  e.添加投影(空间范畴, 时间范畴, "时间投影", "时序编码")
  e.添加投影(关系范畴, 实体范畴, "关系解绑", "纽带分解")

  echo "─ 三位一体查询 ─"
  let tri = e.三位一体(1)
  if tri.找到:
    echo "  " & tri.容器 & ": 物质=" & tri.物质 &
         " T=" & $tri.时间 &
         " 空间=(" & $tri.空间.x & "," & $tri.空间.y & "," & $tri.空间.z & ") "

  echo ""

  echo "─ 范畴查询 ─"
  for cat in [实体范畴, 关系范畴, 属性范畴, 时间范畴, 空间范畴]:
    let items = e.按范畴查询(cat)
    let cat_name = case cat
      of 实体范畴: "实体"
      of 关系范畴: "关系"
      of 属性范畴: "属性"
      of 时间范畴: "时间"
      of 空间范畴: "空间"
    echo "  " & cat_name & ": " & items.join(", ")

  echo ""

  echo "─ 关系路径 ─"
  let path = e.关系路径(1, 4)
  if path.存在:
    echo "  我→容器时间: " & $path.路径.len & " 跳, 总强度:" & formatFloat(path.总强度, ffDecimal, 3)

  let path2 = e.关系路径(2, 5)
  if path2.存在:
    echo "  刘楚恬→容器空间: " & $path2.路径.len & " 跳, 总强度:" & formatFloat(path2.总强度, ffDecimal, 3)

  echo ""

  echo "─ 投影查找 ─"
  let proj1 = e.投影查找(1, 空间范畴)
  if proj1.找到:
    echo "  我→空间投影: " & proj1.投影名 & " → " & proj1.物质

  echo ""
  echo e.本体报告()
