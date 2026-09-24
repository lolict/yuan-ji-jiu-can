#==============================================================
#  聂小倩匹配引擎  —  满全法核心匹配机制
#  距离感知 · 加权距离 · 聚类分析 · 谐音路由
#  本质匹配: 聂小倩=NiuShengZhi+ZhuYin 全部封装
#==============================================================

import std/[tables, strutils, sets, math, algorithm, sequtils]

#==============================================================
#  类型
#=============================================================

type
  极性* = enum
    阳极性 = 1
    阴极性 = 0

  特征点* = object
    名字*: string
    坐标*: array[4, float]   # 4维特征: 声母/韵母/声调/极性
    频率*: float
    极性*: 极性

  权重表* = object
    声母权重*: float
    韵母权重*: float
    声调权重*: float
    极性权重*: float

  聚类* = object
    中心*: array[4, float]
    成员*: seq[string]
    名称*: string

  谐音路由* = object
    源音节*: string
    目标容器*: string
    距离*: float

  匹配引擎* = object
    特征表*: Table[string, 特征点]
    聚类表*: seq[聚类]
    权重*: 权重表
    路由表*: seq[谐音路由]
    最小距离*: float

#==============================================================
#  基础
#==============================================================

proc 默认权重*: 权重表 =
  result = 权重表(声母权重: 1.5, 韵母权重: 1.0, 声调权重: 0.5, 极性权重: 2.0)

proc 创建引擎*: 匹配引擎 =
  result = 匹配引擎(
    特征表: initTable[string, 特征点](),
    聚类表: @[],
    权重: 默认权重(),
    路由表: @[],
    最小距离: 3.0
  )

proc 注册特征*(e: var 匹配引擎; 名字: string; 声母, 韵母, 声调: float; 极性: 极性; 频率: float =
 1.0) =
  e.特征表[名字] = 特征点(
    名字: 名字,
    坐标: [声母, 韵母, 声调, float(ord(极性))],
    频率: 频率,
    极性: 极性
  )

#==============================================================
#  加权距离
#==============================================================

proc 加权距离*(e: 匹配引擎; a, b: 特征点): float =
  let dx = (a.坐标[0] - b.坐标[0]) * e.权重.声母权重
  let dy = (a.坐标[1] - b.坐标[1]) * e.权重.韵母权重
  let dz = (a.坐标[2] - b.坐标[2]) * e.权重.声调权重
  let dp = (a.坐标[3] - b.坐标[3]) * e.权重.极性权重
  result = sqrt(dx*dx + dy*dy + dz*dz + dp*dp)

#==============================================================
#  DBSCAN 距离感知聚类
#==============================================================

proc 区域点*(e: 匹配引擎; 中心: 特征点; 半径: float): seq[string] =
  for name, p in e.特征表:
    if e.加权距离(中心, p) <= 半径:
      result.add(name)

proc 聚类分析*(e: var 匹配引擎; 半径: float; 最小点数: int) =
  var 已访问 = initHashSet[string]()
  e.聚类表 = @[]

  for name, p in e.特征表:
    if name in 已访问:
      continue
    let neighbors = e.区域点(p, 半径)
    if neighbors.len < 最小点数:
      continue  # 噪声点
    var cluster = 聚类(中心: p.坐标, 成员: @[], 名称: "C" & $e.聚类表.len)
    var toProcess = neighbors
    while toProcess.len > 0:
      let current = toProcess.pop()
      if current in 已访问:
        continue
      已访问.incl(current)
      if not e.特征表.hasKey(current):
        continue
      cluster.成员.add(current)
      if cluster.成员.len == 1:
        cluster.中心 = e.特征表[current].坐标
      let curP = e.特征表[current]
      let curNeighbors = e.区域点(curP, 半径)
      if curNeighbors.len >= 最小点数:
        for cn in curNeighbors:
          if cn notin 已访问:
            toProcess.add(cn)
    e.聚类表.add(cluster)

#==============================================================
#  最近邻匹配
#==============================================================

proc 最近邻*(e: 匹配引擎; 查询: 特征点): tuple[名字: string; 距离: float] =
  var bestName = ""
  var bestDist = Inf
  for name, p in e.特征表:
    let d = e.加权距离(查询, p)
    if d < bestDist and d > 0:
      bestDist = d
      bestName = name
  return (名字: bestName, 距离: bestDist)

proc k近邻*(e: 匹配引擎; 查询: 特征点; k: int): seq[tuple[名字: string; 距离: float]] =
  var dists: seq[tuple[名字: string; 距离: float]] = @[]
  for name, p in e.特征表:
    let d = e.加权距离(查询, p)
    if d > 0:
      dists.add((名字: name, 距离: d))
  dists.sort(proc (a, b: tuple[名字: string; 距离: float]): int = cmp(a.距离, b.距离))
  if dists.len > k:
    dists.setLen(k)
  return dists

#==============================================================
#  谐音路由
#==============================================================

proc 谐音距离*(a, b: 特征点): float =
  # 仅基于声母+韵母 (忽略声调)
  let dx = a.坐标[0] - b.坐标[0]
  let dy = a.坐标[1] - b.坐标[1]
  result = sqrt(dx*dx + dy*dy)

proc 谐音匹配*(e: var 匹配引擎; 源名字: string): string =
  if not e.特征表.hasKey(源名字):
    return "未找到"
  let src = e.特征表[源名字]
  var bestName = ""
  var bestDist = Inf
  for name, p in e.特征表:
    if name == 源名字:
      continue
    let d = 谐音距离(src, p)
    if d < bestDist:
      bestDist = d
      bestName = name
  if bestName != "":
    e.路由表.add(谐音路由(源音节: 源名字, 目标容器: bestName, 距离: bestDist))
  return bestName

#==============================================================
#  声母遍历韵母 (聂小倩核心算法)
#==============================================================

proc 声母遍历韵母*(e: 匹配引擎; 声母值: float): seq[string] =
  # 遍历所有特征点，找出声母匹配的所有变体
  for name, p in e.特征表:
    if abs(p.坐标[0] - 声母值) < 0.01:
      result.add(name)

#==============================================================
#  报告
#==============================================================

proc 特征摘要*(e: 匹配引擎): string =
  var lines: seq[string] = @[]
  lines.add "特征点   声母 韵母 声调 极性"
  lines.add "──────── ──── ──── ──── ────"
  for name in e.特征表.keys:
    let p = e.特征表[name]
    lines.add align(name, 8) & " " &
             align($(p.坐标[0].int), 4) & " " &
             align($(p.坐标[1].int), 4) & " " &
             align($(p.坐标[2].int), 4) & " " &
             (if p.极性 == 阳极性: "阳" else: "阴")
  return lines.join("\n")

proc 聚类报告*(e: 匹配引擎): string =
  var lines: seq[string] = @[]
  lines.add "聚类分析结果 (DBSCAN):"
  lines.add "═══════════════════"
  if e.聚类表.len == 0:
    lines.add "  (尚未执行聚类)"
  for c in e.聚类表:
    lines.add "  " & c.名称 & " [" & $c.成员.len & "]: " & c.成员.join(", ")
  return lines.join("\n")

proc 路由报告*(e: 匹配引擎): string =
  var lines: seq[string] = @[]
  lines.add "谐音路由记录:"
  for r in e.路由表:
    lines.add "  " & r.源音节 & " → " & r.目标容器 & " (谐音距离=" & formatFloat(r.距离, ffDecimal, 3) & ")"
  return lines.join("\n")

#==============================================================
#  主程序
#==============================================================

when isMainModule:
  echo "════════════════════════════════════════"
  echo "  聂 小 倩 匹 配 引 擎"
  echo "  距离感知 · 加权距离 · 聚类 · 谐音路由"
  echo "════════════════════════════════════════"

  var e = 创建引擎()

  echo "\n-- 注册特征点 (满全法核心音节) --"
  # 坐标: [声母序, 韵母序, 声调序, 极性]
  e.注册特征("满", 2, 10, 3, 阳极性)    # mǎn
  e.注册特征("漫", 2, 10, 3, 阳极性)    # màn (谐音)
  e.注册特征("曼", 2, 10, 1, 阳极性)    # màn (谐音)
  e.注册特征("全", 12, 19, 1, 阴极性)   # quán
  e.注册特征("权", 12, 19, 1, 阳极性)   # quán (谐音)
  e.注册特征("泉", 12, 19, 1, 阴极性)   # quán (谐音)
  e.注册特征("法", 3, 0, 3, 阳极性)     # fà
  e.注册特征("发", 3, 0, 0, 阳极性)     # fā (谐音)
  e.注册特征("囡", 6, 10, 1, 阴极性)    # nān
  e.注册特征("囝", 11, 19, 0, 阳极性)   # jiān
  e.注册特征("生", 13, 15, 1, 阴极性)   # shēng
  e.注册特征("声", 13, 15, 1, 阳极性)   # shēng (谐音)

  echo "  已注册 ", e.特征表.len, " 个特征点"
  echo "  权重: 声母=", e.权重.声母权重,
       " 韵母=", e.权重.韵母权重,
       " 声调=", e.权重.声调权重,
       " 极性=", e.权重.极性权重

  echo "\n-- 特征空间 --"
  echo e.特征摘要()

  echo "\n-- 加权距离矩阵 (部分) --"
  let names = ["满", "全", "法", "囡", "囝", "生"]
  for i, a in names:
    for j, b in names:
      if i < j and e.特征表.hasKey(a) and e.特征表.hasKey(b):
        let d = e.加权距离(e.特征表[a], e.特征表[b])
        if d < 5.0:
          echo "  ", align(a, 4), " → ", align(b, 4),
               " d=", formatFloat(d, ffDecimal, 3)

  echo "\n-- 距离感知聚类 --"
  e.聚类分析(2.5, 2)
  echo e.聚类报告()

  echo "\n-- 最近邻匹配 --"
  var query = 特征点(名字: "检索", 坐标: [2, 10, 2, 1], 频率: 1.0, 极性: 阳极性)
  let nn = e.最近邻(query)
  echo "  查询 [m-ǎn-2-阳] → 最近邻: ", nn.名字, " (距离=", formatFloat(nn.距离, ffDecimal, 3), ")"

  let knn = e.k近邻(query, 3)
  echo "  Top-3:"
  for k in knn:
    echo "    ", align(k.名字, 8), " d=", formatFloat(k.距离, ffDecimal, 3)

  echo "\n-- 谐音路由 --"
  let hm1 = e.谐音匹配("满")
  echo "  满 → 谐音: " & hm1
  let hm2 = e.谐音匹配("全")
  echo "  全 → 谐音: " & hm2
  let hm3 = e.谐音匹配("法")
  echo "  法 → 谐音: " & hm3
  let hm4 = e.谐音匹配("生")
  echo "  生 → 谐音: " & hm4

  echo "\n-- 声母遍历韵母 --"
  let variants = e.声母遍历韵母(2.0)
  echo "  声母=m (序=2) 的所有变体: ", variants.join(", ")

  echo "\n-- 路由记录 --"
  echo e.路由报告()

  echo "\n════════════════════════════════════════"
  echo " 聂小倩匹配 · 本质匹配 · 谐音路由 · 距离感知"
  echo "════════════════════════════════════════"
