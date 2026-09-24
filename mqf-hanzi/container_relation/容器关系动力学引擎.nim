#==============================================================
#  容器关系动力学引擎 — Container Relation Dynamics
#
#  核心公理：公约数=通·无公约=断（斥）
#  纤维丛几何投影 · 间距感知拓扑 · 八家族极性关系图
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm, sets]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 容器节点（几何化）
  容器节点* = object
    编码*: int                    # 64位容器编码
    名字*: string
    x*, y*, z*: float             # 三维坐标
    极性*: uint8                  # 阳=1 阴=0
    家族*: int                    # 0-7 八族之一
    槽位*: int                    # 0-7 槽
    主权*: int                    # 主权值
    能量*: float                  # 当前能量

  # 关系边 — 两个容器之间的关系
  关系边* = object
    起点*: int                    # 容器编码
    终点*: int                    # 容器编码
    关系型*: string               # 通/断/吸/斥/合/续/遥/争
    权值*: float                  # 关系强度
    公约*: int                    # 公约数值
    间距*: float                  # 几何距离

  # 纤维丛投影
  纤维丛* = object
    底空间*: tuple[x, y, z: float]    # 三维底空间投影
    纤维*: float                      # 纤维维度的值
    截面*: string                     # 截面所属层

  # 拓扑统计
  拓扑统计* = object
    总节点*: int
    总边数*: int
    连通分量*: int
    平均度*: float
    核心节点*: seq[int]
    桥接边*: seq[seq[int]]

  # 容器关系动力学引擎
  容器关系动力学* = object
    节点表*: Table[int, 容器节点]      # 编码→节点
    边表*: seq[关系边]                 # 所有边
    纤维表*: Table[int, 纤维丛]       # 编码→纤维丛投影
    邻接表*: Table[int, seq[int]]     # 编码→邻居编码序列
    拓扑*: 拓扑统计

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化关系动力学*(): 容器关系动力学 =
  result.拓扑 = 拓扑统计(
    总节点: 0, 总边数: 0, 连通分量: 0,
    平均度: 0.0, 核心节点: @[], 桥接边: @[]
  )

#--------------------------------------------------------------
#  容器注册
#--------------------------------------------------------------

proc 注册容器*(c: var 容器关系动力学; 编码: int; 名字: string;
                x, y, z: float; 极性: uint8;
                家族, 槽位: int; 主权: int; 能量: float) =
  c.节点表[编码] = 容器节点(
    编码: 编码, 名字: 名字,
    x: x, y: y, z: z,
    极性: 极性, 家族: 家族, 槽位: 槽位,
    主权: 主权, 能量: 能量
  )
  c.邻接表[编码] = @[]

#--------------------------------------------------------------
#  公约数计算 — 关系根本
#--------------------------------------------------------------

proc 公约关系*(a, b: int): tuple[gcd_val: int; 通断: bool; 关系型: string] =
  if a == 0 or b == 0:
    return (0, false, "断")

  let g = gcd(a, b)
  if g > 1:
    # 有公约 = 通
    # 细分类：同家族（公约=8的倍数）→合，跨家族→续
    if g mod 8 == 0:
      return (g, true, "合")   # 同族合
    else:
      return (g, true, "续")   # 跨族续
  else:
    # 无公约 = 断 → 斥
    return (g, false, "斥")

#--------------------------------------------------------------
#  极性关系：吸/斥
#--------------------------------------------------------------

proc 极性关系*(p1, p2: uint8): string =
  if p1 != p2:
    return "吸"   # 阴阳相吸
  else:
    return "斥"   # 同性相斥

#--------------------------------------------------------------
#  间距计算与感知
#--------------------------------------------------------------

proc 间距*(c: 容器关系动力学; a, b: int): float =
  if not (c.节点表.hasKey(a) and c.节点表.hasKey(b)):
    return -1.0
  let na = c.节点表[a]
  let nb = c.节点表[b]
  let dx = na.x - nb.x
  let dy = na.y - nb.y
  let dz = na.z - nb.z
  return sqrt(dx * dx + dy * dy + dz * dz)

proc 间距感知*(dist: float): string =
  # 距离感知分级
  if dist < 5.0:
    return "近"
  elif dist < 20.0:
    return "中"
  else:
    return "远"

#--------------------------------------------------------------
#  建边 — 关系拓扑构建
#--------------------------------------------------------------

proc 构建关系*(c: var 容器关系动力学; 距离阈值: float = 50.0) =
  c.边表 = @[]
  let keys = toSeq(c.节点表.keys)
  for i in 0 ..< keys.len:
    for j in (i + 1) ..< keys.len:
      let a = keys[i]
      let b = keys[j]
      let dist = c.间距(a, b)

      # 超距离阈值不建边（间距感知）
      if dist > 距离阈值:
        continue

      # 公约数判断
      let cr = 公约关系(a, b)

      # 极性判断
      let pr = 极性关系(c.节点表[a].极性, c.节点表[b].极性)

      # 最终关系类型：取公约关系为主，极性为修正
      var final型 = cr.关系型
      var weight = float(cr.gcd_val)

      if pr == "斥" and cr.通断:
        # 同性相斥但有公约 → 争（竞争关系）
        final型 = "争"
        weight = weight * 0.5  # 斥降低权值
      elif pr == "吸" and cr.通断:
        # 异性相吸且有公约 → 增强
        weight = weight * 1.5
      elif pr == "吸":
        # 异性相吸但无公约 → 遥（远距离弱关系）
        final型 = "遥"
        weight = 0.1

      # 距离衰减：间距越远权值越低
      weight = weight / (1.0 + dist * 0.1)

      c.边表.add(关系边(
        起点: a, 终点: b,
        关系型: final型,
        权值: weight,
        公约: cr.gcd_val,
        间距: dist
      ))

      # 通断双方邻接表
      if cr.通断:
        c.邻接表[a].add(b)
        c.邻接表[b].add(a)

#--------------------------------------------------------------
#  纤维丛几何投影
#--------------------------------------------------------------

proc 纤维丛投影*(c: var 容器关系动力学) =
  for k, node in c.节点表:
    # 底空间：[x, y, z]
    let base = (node.x, node.y, node.z)

    # 纤维维 = 主权 + 能量编码（升维投影）
    let fiber = float(node.主权) * 0.1 + node.能量 * 0.01

    # 截面层：由家族+槽位决定
    var section = "根基"
    case node.家族
    of 0:
      section = "根基层"
    of 1:
      section = "扩容层"
    of 2:
      section = "重器层"
    of 3:
      section = "权柄层"
    of 4:
      section = "界藩层"
    of 5:
      section = "分配层"
    of 6:
      section = "承载层"
    of 7:
      section = "归藏层"
    else:
      section = "未定层"

    c.纤维表[k] = 纤维丛(
      底空间: base,
      纤维: fiber,
      截面: section
    )

#--------------------------------------------------------------
#  拓扑分析：度统计、核心节点、桥接
#--------------------------------------------------------------

proc 分析拓扑*(c: var 容器关系动力学) =
  c.拓扑.总节点 = c.节点表.len
  c.拓扑.总边数 = c.边表.len

  # 度统计
  var total_degree = 0
  var degree_table = initTable[int, int]()
  for k in c.邻接表.keys:
    let d = c.邻接表[k].len
    degree_table[k] = d
    total_degree += d

  if c.节点表.len > 0:
    c.拓扑.平均度 = float(total_degree) / float(c.节点表.len)

  # 核心节点：度 >= 平均度 * 1.5 的节点
  var hubs: seq[int] = @[]
  for k, d in degree_table:
    if float(d) >= c.拓扑.平均度 * 1.5:
      hubs.add(k)
  c.拓扑.核心节点 = hubs.sorted()

#--------------------------------------------------------------
#  寻路 — 经公约数通断的连接路径
#--------------------------------------------------------------

proc 寻路*(c: 容器关系动力学; 起点: int; 终点: int): seq[int] =
  var visited = initHashSet[int]()
  var queue: seq[seq[int]] = @[@[起点]]

  while queue.len > 0:
    let path = queue[0]
    queue.del(0)
    let cur = path[^1]

    if cur == 终点:
      return path

    if cur in visited:
      continue
    visited.incl(cur)

    if c.邻接表.hasKey(cur):
      for nb in c.邻接表[cur]:
        if nb notin visited:
          var new_path = path
          new_path.add(nb)
          queue.add(new_path)

#--------------------------------------------------------------
#  公约数封装 — 共享公约的容器组
#--------------------------------------------------------------

proc 公约封装*(c: 容器关系动力学; 最小公约: int): seq[seq[int]] =
  var groups: seq[seq[int]] = @[]
  var grouped = initHashSet[int]()

  let keys = toSeq(c.节点表.keys).sorted()
  for i in 0 ..< keys.len:
    if keys[i] in grouped:
      continue
    var group: seq[int] = @[keys[i]]
    grouped.incl(keys[i])

    for j in (i + 1) ..< keys.len:
      if keys[j] in grouped:
        continue
      let g = gcd(keys[i], keys[j])
      if g >= 最小公约:
        group.add(keys[j])
        grouped.incl(keys[j])

    if group.len > 1:
      groups.add(group)
  return groups

#--------------------------------------------------------------
#  关系图报告
#--------------------------------------------------------------

proc 关系图报告*(c: 容器关系动力学): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 54)
  lines.add "  容 器 关 系 动 力 学 引 擎 — 关 系 图"
  lines.add repeat("═", 54)
  lines.add "节点总数: " & $c.拓扑.总节点
  lines.add "关系边数: " & $c.拓扑.总边数
  lines.add "平均度: " & formatFloat(c.拓扑.平均度, ffDecimal, 2)
  lines.add "核心节点数: " & $c.拓扑.核心节点.len
  lines.add ""

  lines.add "─ 关系边清单 ─"
  for edge in c.边表:
    let na = c.节点表[edge.起点]
    let nb = c.节点表[edge.终点]
    lines.add "  " & na.名字 & "(" & $edge.起点 & ") " &
             edge.关系型 & " " &
             nb.名字 & "(" & $edge.终点 & ") " &
             "[公约:" & $edge.公约 & ", 间距:" &
             formatFloat(edge.间距, ffDecimal, 1) &
             ", 权值:" & formatFloat(edge.权值, ffDecimal, 2) & "]"

  lines.add ""
  lines.add "─ 纤维丛投影 ─"
  for k in sorted(toSeq(c.纤维表.keys)):
    let f = c.纤维表[k]
    let n = c.节点表[k]
    lines.add "  " & n.名字 & " → 截面:" & f.截面 &
             " 纤维:" & formatFloat(f.纤维, ffDecimal, 2) &
             " 底空间(" & formatFloat(f.底空间.x, ffDecimal, 1) & "," &
             formatFloat(f.底空间.y, ffDecimal, 1) & "," &
             formatFloat(f.底空间.z, ffDecimal, 1) & ") "

  lines.add ""
  lines.add "─ 八族拓扑 ─"
  for family in 0 .. 7:
    var members: seq[string] = @[]
    for k, n in c.节点表:
      if n.家族 == family:
        members.add(n.名字)
    if members.len > 0:
      let fanames = ["根基", "扩容", "重器", "权柄", "界藩", "分配", "承载", "归藏"]
      lines.add "  族" & $family & "(" & fanames[family] & "): " & members.join(", ")

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var c = 初始化关系动力学()

  # 注册满全法核心容器（按八族分配）
  c.注册容器(10, "根我", 0.0, 0.0, 0.0, 1, 0, 0, 100, 1.0)
  c.注册容器(20, "境恬", 5.0, 0.0, 0.0, 0, 1, 1, 90, 0.9)
  c.注册容器(30, "满", 10.0, 5.0, 0.0, 1, 0, 2, 85, 0.8)
  c.注册容器(40, "全", 15.0, 5.0, 0.0, 0, 1, 3, 84, 0.8)
  c.注册容器(50, "法", 20.0, 5.0, 0.0, 1, 0, 4, 70, 0.7)
  c.注册容器(60, "生", 25.0, 5.0, 0.0, 0, 1, 5, 60, 0.6)
  c.注册容器(70, "声", 30.0, 5.0, 0.0, 1, 0, 6, 50, 0.5)
  c.注册容器(80, "调", 35.0, 5.0, 0.0, 0, 1, 7, 40, 0.4)
  c.注册容器(16, "一", 40.0, 5.0, 0.0, 1, 2, 0, 30, 0.3)
  c.注册容器(24, "二", 45.0, 5.0, 0.0, 0, 3, 1, 24, 0.24)

  echo "═══ 构建关系拓扑 ═══"

  # 构建关系
  c.构建关系(40.0)

  # 纤维丛投影
  c.纤维丛投影()

  # 拓扑分析
  c.分析拓扑()

  echo "构建完成: " & $c.拓扑.总节点 & " 节点, " &
       $c.拓扑.总边数 & " 边"

  echo ""

  # 关系类型统计
  var rel_count = initTable[string, int]()
  for e in c.边表:
    if not rel_count.hasKey(e.关系型):
      rel_count[e.关系型] = 0
    rel_count[e.关系型] = rel_count[e.关系型] + 1

  echo "关系类型统计:"
  for r, cnt in rel_count:
    echo "  " & r & ": " & $cnt

  echo ""

  # 寻路测试
  echo "─ 寻路测试 ─"
  let path = c.寻路(10, 80)
  if path.len > 0:
    var path_names: seq[string] = @[]
    for p in path:
      path_names.add(c.节点表[p].名字)
    echo "  10→80: " & path_names.join(" → ")
  else:
    echo "  10→80: 无路径"

  let path2 = c.寻路(20, 50)
  if path2.len > 0:
    var pn2: seq[string] = @[]
    for p in path2:
      pn2.add(c.节点表[p].名字)
    echo "  20→50: " & pn2.join(" → ")
  else:
    echo "  20→50: 无路径"

  echo ""

  # 公约封装
  echo "─ 公约封装（最小公约>=10） ─"
  let groups = c.公约封装(10)
  for i in 0 ..< groups.len:
    let g = groups[i]
    var names: seq[string] = @[]
    for enc in g:
      names.add(c.节点表[enc].名字)
    echo "  组" & $i & ": " & names.join(", ")

  echo ""

  echo c.关系图报告()
