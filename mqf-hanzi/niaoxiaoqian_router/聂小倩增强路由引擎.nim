#==============================================================
#  聂小倩增强路由引擎 — Enhanced NiaoXiaoQian Router
#
#  核心：谐音路由 · 加权距离聚类 · 容器本体论感知 · 双向寻路
#  "情"不自己走 · 被声母 · 韵母 · 声调 · 极性指引方向
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm, sets]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 路由节点 — 拼音×极性的四维特征
  路由节点* = object
    编码*: int               # 64位容器编码
    名字*: string
    声母*: string
    韵母*: string
    声调*: int               # 1-5
    极性*: uint8             # 阳=1 阴=0
    三维*: tuple[x, y, z: float]
    权重*: float             # 路由权重

  # 路由路径
  路由路径* = object
    hop*: seq[int]          # 经过的节点编码序列
    总距离*: float
    总跳数*: int
    路径型*: string          # 直路/弯路/迂回路

  # 路由表项
  路由项* = object
    目标*: int
    下一跳*: int
    距离*: float
    跳数*: int

  # 聚类（加权距离DBSCAN增强）
  增强聚类* = object
    编号*: int
    中心*: tuple[x, y, z: float]
    成员*: seq[int]
    名称*: string
    极性纯度*: float         # 同极性占比

  # 聂小倩引擎
  聂小倩引擎* = object
    节点表*: Table[int, 路由节点]
    路由表*: Table[int, seq[路由项]]   # 源编码→路由项序列
    聚类表*: seq[增强聚类]
    半径*: float
    最小点数*: int
    路由计数*: int

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化聂小倩*(半径: float = 15.0; 最小点数: int = 2): 聂小倩引擎 =
  result.半径 = 半径
  result.最小点数 = 最小点数
  result.路由计数 = 0

#--------------------------------------------------------------
#  注册节点
#--------------------------------------------------------------

proc 注册节点*(e: var 聂小倩引擎; 编码: int; 名字, 声母, 韵母: string;
               声调: int; 极性: uint8;
               x, y, z: float; 权重: float) =
  e.节点表[编码] = 路由节点(
    编码: 编码, 名字: 名字,
    声母: 声母, 韵母: 韵母, 声调: 声调,
    极性: 极性,
    三维: (x, y, z),
    权重: 权重
  )

#--------------------------------------------------------------
#  特征距离 — 4D加权
#  声母1.5 / 韵母1.0 / 声调0.5 / 极性2.0
#--------------------------------------------------------------

proc 声母距离*(a, b: string): float =
  if a == b:
    return 0.0
  # 声母表序距离
  let 声母表 = @["b", "p", "m", "f", "d", "t", "n", "l",
                 "g", "k", "h", "j", "q", "x",
                 "zh", "ch", "sh", "r", "z", "c", "s",
                 "y", "w", "zero"]
  var ai = -1
  var bi = -1
  for i in 0 ..< 声母表.len:
    if 声母表[i] == a:
      ai = i
    if 声母表[i] == b:
      bi = i
  if ai >= 0 and bi >= 0:
    return float(abs(ai - bi)) / 24.0
  return 1.0

proc 韵母距离*(a, b: string): float =
  if a == b:
    return 0.0
  let 韵母表 = @["a", "o", "e", "i", "u", "v",
                 "ai", "ei", "ui", "ao", "ou", "iu",
                 "ie", "ve", "er", "an", "en", "in", "un", "vn",
                 "ang", "eng", "ing", "ong",
                 "ia", "iao", "ian", "iang", "iong",
                 "ua", "uo", "uai", "uan", "uang",
                 "ue", "un2"]
  var ai = -1
  var bi = -1
  for i in 0 ..< 韵母表.len:
    if 韵母表[i] == a:
      ai = i
    if 韵母表[i] == b:
      bi = i
  if ai >= 0 and bi >= 0:
    return float(abs(ai - bi)) / 30.0
  return 1.0

proc 特征距离*(e: 聂小倩引擎; a, b: int): float =
  if not (e.节点表.hasKey(a) and e.节点表.hasKey(b)):
    return -1.0
  let na = e.节点表[a]
  let nb = e.节点表[b]

  let 声距 = 声母距离(na.声母, nb.声母)
  let 韵距 = 韵母距离(na.韵母, nb.韵母)
  let 调距 = if na.声调 == nb.声调: 0.0 else: 0.3
  let 极距 = if na.极性 == nb.极性: 0.0 else: 1.0

  return 1.5 * 声距 + 1.0 * 韵距 + 0.5 * 调距 + 2.0 * 极距

proc 几何距离*(e: 聂小倩引擎; a, b: int): float =
  if not (e.节点表.hasKey(a) and e.节点表.hasKey(b)):
    return -1.0
  let na = e.节点表[a]
  let nb = e.节点表[b]
  let dx = na.三维.x - nb.三维.x
  let dy = na.三维.y - nb.三维.y
  let dz = na.三维.z - nb.三维.z
  return sqrt(dx * dx + dy * dy + dz * dz)

proc 综合距离*(e: 聂小倩引擎; a, b: int): float =
  let fd = e.特征距离(a, b)
  let gd = e.几何距离(a, b)
  if fd < 0 or gd < 0:
    return -1.0
  return 0.6 * fd + 0.4 * gd * 0.1

#--------------------------------------------------------------
#  谐音路由 — 同声母+同韵母（忽略声调）→ 路由通路
#--------------------------------------------------------------

proc 谐音寻路*(e: 聂小倩引擎; 源: int; 目标声母, 目标韵母: string): seq[int] =
  if not e.节点表.hasKey(源):
    return @[]
  var 候选: seq[int] = @[]
  for k, n in e.节点表:
    if k != 源 and n.声母 == 目标声母 and n.韵母 == 目标韵母:
      候选.add(k)
  return 候选

proc 谐音通鸣*(e: 聂小倩引擎; 声母, 韵母: string): seq[int] =
  var result_seq: seq[int] = @[]
  for k, n in e.节点表:
    if n.声母 == 声母 and n.韵母 == 韵母:
      result_seq.add(k)
  return result_seq

#--------------------------------------------------------------
#  加权距离DBSCAN聚类增强
#--------------------------------------------------------------

proc 区域点*(e: 聂小倩引擎; 中心点: int): seq[int] =
  var result_seq: seq[int] = @[]
  for k in e.节点表.keys:
    if k == 中心点:
      continue
    let d = e.综合距离(中心点, k)
    if d >= 0 and d <= e.半径:
      result_seq.add(k)
  return result_seq

proc 聚类分析*(e: var 聂小倩引擎) =
  e.聚类表 = @[]
  var visited = initHashSet[int]()

  for 编码 in e.节点表.keys:
    if 编码 in visited:
      continue
    let neighbors = e.区域点(编码)
    if neighbors.len < e.最小点数:
      continue

    var cluster = 增强聚类(
      编号: e.聚类表.len,
      成员: @[编码],
      名称: "群" & $e.聚类表.len,
      极性纯度: 0.0
    )
    var to_process = neighbors
    while to_process.len > 0:
      let cur = to_process.pop()
      if cur in visited:
        continue
      visited.incl(cur)
      cluster.成员.add(cur)
      let cur_neighbors = e.区域点(cur)
      if cur_neighbors.len >= e.最小点数:
        for cn in cur_neighbors:
          if cn notin visited:
            to_process.add(cn)

    e.聚类表.add(cluster)

  # 计算聚类中心和极性纯度
  for i in 0 ..< e.聚类表.len:
    var cx = 0.0
    var cy = 0.0
    var cz = 0.0
    var yang = 0
    for m in e.聚类表[i].成员:
      let n = e.节点表[m]
      cx += n.三维.x
      cy += n.三维.y
      cz += n.三维.z
      if n.极性 == 1:
        yang += 1
    let sz = float(e.聚类表[i].成员.len)
    e.聚类表[i].中心 = (cx / sz, cy / sz, cz / sz)
    e.聚类表[i].极性纯度 = float(yang) / sz

#--------------------------------------------------------------
#  路由构建 — 以谐音和特征距离构建路由表
#--------------------------------------------------------------

proc 构建路由*(e: var 聂小倩引擎) =
  for src in e.节点表.keys:
    var items: seq[路由项] = @[]
    for dst in e.节点表.keys:
      if src == dst:
        continue
      let d = e.综合距离(src, dst)
      if d < 0:
        continue
      # 谐音直连：同声母+同韵母 → 直路
      let ns = e.节点表[src]
      let nd = e.节点表[dst]
      var 直接 = false
      if ns.声母 == nd.声母 and ns.韵母 == nd.韵母:
        if d <= e.半径 * 2.0:
          直接 = true
      if 直接:
        items.add(路由项(目标: dst, 下一跳: dst, 距离: d, 跳数: 1))
    # 按距离排序
    items = sorted(items, proc(a, b: 路由项): int = cmp(a.距离, b.距离))
    e.路由表[src] = items
    e.路由计数 += items.len

#--------------------------------------------------------------
#  路由寻路 — 多跳
#--------------------------------------------------------------

proc 寻路*(e: 聂小倩引擎; 源: int; 目标: int): 路由路径 =
  var visited = initHashSet[int]()
  var queue: seq[tuple[path: seq[int], dist: float]] = @[(@[源], 0.0)]

  while queue.len > 0:
    let cur = queue[0]
    queue.del(0)
    let cur_node = cur.path[^1]

    if cur_node == 目标:
      var p型 = "直路"
      if cur.path.len > 2:
        p型 = "弯路"
      if cur.path.len > 4:
        p型 = "迂回路"
      return 路由路径(hop: cur.path, 总距离: cur.dist,
                       总跳数: cur.path.len - 1, 路径型: p型)

    if cur_node in visited:
      continue
    visited.incl(cur_node)

    if e.路由表.hasKey(cur_node):
      for rt in e.路由表[cur_node]:
        if rt.下一跳 notin visited:
          var new_path = cur.path
          new_path.add(rt.下一跳)
          queue.add((new_path, cur.dist + rt.距离))

  return 路由路径()

#--------------------------------------------------------------
#  报告
#--------------------------------------------------------------

proc 路由报告*(e: 聂小倩引擎): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 54)
  lines.add "  聂 小 倩 增 强 路 由 引 擎"
  lines.add repeat("═", 54)
  lines.add "节点数: " & $e.节点表.len
  lines.add "聚类数: " & $e.聚类表.len
  lines.add "路由数: " & $e.路由计数
  lines.add "聚类半径: " & $e.半径
  lines.add ""

  lines.add "─ 聚类结果 ─"
  for c in e.聚类表:
    var names: seq[string] = @[]
    for m in c.成员:
      names.add(e.节点表[m].名字)
    lines.add "  " & c.名称 & " (" & $c.成员.len & "): " &
             names.join(", ") &
             " [纯度:" & formatFloat(c.极性纯度, ffDecimal, 2) &
             " 中心(" & formatFloat(c.中心.x, ffDecimal, 1) & "," &
             formatFloat(c.中心.y, ffDecimal, 1) & "," &
             formatFloat(c.中心.z, ffDecimal, 1) & ")] "

  lines.add ""
  lines.add "─ 路由表（每源最近3跳） ─"
  for src in sorted(toSeq(e.路由表.keys)):
    let src_n = e.节点表[src]
    lines.add "  " & src_n.名字 & "(" & $src & "):"
    var shown = 0
    for rt in e.路由表[src]:
      if shown >= 3:
        break
      let dst_n = e.节点表[rt.目标]
      lines.add "    → " & dst_n.名字 & " 距:" &
               formatFloat(rt.距离, ffDecimal, 3) & " " & $rt.跳数 & "跳"
      shown += 1

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var e = 初始化聂小倩(12.0, 2)

  # 注册核心路由节点
  e.注册节点(0x22801, "满", "m", "an", 3, 1, 1.0, 2.0, 0.0, 1.0)
  e.注册节点(0xC4C01, "漫", "m", "an", 4, 0, 1.5, 2.5, 0.5, 0.9)
  e.注册节点(0xC4C010, "全", "q", "uan", 2, 0, 5.0, 5.0, 0.0, 1.0)
  e.注册节点(0xB4C03, "囝", "j", "ian", 3, 1, 5.5, 5.5, 0.5, 0.8)
  e.注册节点(0x6281E, "囡", "n", "an", 1, 0, 4.5, 4.5, 0.5, 0.85)
  e.注册节点(0x30031, "法", "f", "a", 3, 1, 10.0, 5.0, 0.0, 1.0)
  e.注册节点(0x30000, "发", "f", "a", 1, 0, 10.5, 5.5, 0.5, 0.7)
  e.注册节点(0x1A005, "声", "sh", "eng", 1, 1, 15.0, 5.0, 0.0, 0.9)
  e.注册节点(0x1A010, "生", "sh", "eng", 1, 1, 15.5, 5.5, 0.5, 0.85)
  e.注册节点(0x1A020, "升", "sh", "eng", 1, 1, 16.0, 6.0, 0.5, 0.8)

  echo "═══ 聂小倩增强路由 ═══"
  echo ""

  # 谐音通鸣
  echo "─ 谐音通鸣 ─"
  let fa = e.谐音通鸣("f", "a")
  var fa_names: seq[string] = @[]
  for c in fa:
    fa_names.add(e.节点表[c].名字)
  echo "  f+a → " & fa_names.join(", ")

  let sheng_yin = e.谐音通鸣("sh", "eng")
  var sy_names: seq[string] = @[]
  for c in sheng_yin:
    sy_names.add(e.节点表[c].名字)
  echo "  sh+eng → " & sy_names.join(", ")

  echo ""

  # 特征距离示例
  echo "─ 特征距离 ─"
  let d1 = e.特征距离(0x22801, 0xC4C01)
  echo "  满↔漫 特征距离: " & formatFloat(d1, ffDecimal, 4)
  let d2 = e.综合距离(0x22801, 0x30031)
  echo "  满↔法 综合距离: " & formatFloat(d2, ffDecimal, 4)

  echo ""

  # 聚类
  e.聚类分析()
  echo "─ 聚类结果 ─"
  for c in e.聚类表:
    var names: seq[string] = @[]
    for m in c.成员:
      names.add(e.节点表[m].名字)
    echo "  " & c.名称 & ": " & names.join(", ")

  echo ""

  # 路由构建与寻路
  e.构建路由()
  echo "─ 路由寻路 ─"
  let path1 = e.寻路(0x22801, 0x30031)
  if path1.hop.len > 0:
    var pn: seq[string] = @[]
    for h in path1.hop:
      pn.add(e.节点表[h].名字)
    echo "  满→法: " & pn.join(" → ") &
         " [" & path1.路径型 & ", " & $path1.总跳数 & "跳] "
  else:
    echo "  满→法: 无路由"

  let path2 = e.寻路(0x22801, 0x1A005)
  if path2.hop.len > 0:
    var pn: seq[string] = @[]
    for h in path2.hop:
      pn.add(e.节点表[h].名字)
    echo "  满→声: " & pn.join(" → ") &
         " [" & path2.路径型 & ", " & $path2.总跳数 & "跳] "
  else:
    echo "  满→声: 无路由"

  echo ""
  echo e.路由报告()
