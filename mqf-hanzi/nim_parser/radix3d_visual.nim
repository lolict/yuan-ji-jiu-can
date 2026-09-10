## 满全法 - 三维几何可视化引擎
## 将容器编码渲染为可感知的立体结构（灰度SDF + 字符画）
##
## 核心命题: 编码不仅是数学，更是可感知的物理体验
## 三维坐标系: X=极性(囡0↔囝1), Y=层级(L0-L6), Z=时序
##
## 输出: ASCII线框/灰度体素场景

import math
import tables
import strutils

# ===================================================================
# 三维几何类型
# ===================================================================

type
  三维点* = object
    x*, y*, z*: float

  三维向量* = object
    dx*, dy*, dz*: float

  几何体* = enum
    gePoint, geLine, gePlane, geBox, geSphere, geCylinder, geTorus

  体素* = ref object
    中心*: 三维点
    尺寸*: float
    字符*: string        # 体素用哪个字符表示
    层级*: int           # L0-L6
    极性*: int           # 0=囡, 1=囝
    价值*: float         # 0.0-1.0 灰度权重

  容器编码体* = ref object
    体素表*: seq[体素]
    几何表*: seq[tuple[kind: 几何体, p1, p2: 三维点, w: float]]
    min_p*: 三维点
    max_p*: 三维点

# ===================================================================
# 投影变换
# ===================================================================

proc 视投影*(p: 三维点, 视角: float, 距离: float): tuple[x, y: float, depth: float] =
  ## 简单透视投影
  let 旋转y = p.x * cos(视角) - p.z * sin(视角)
  let 旋转z = p.x * sin(视角) + p.z * cos(视角)
  let 缩放 = 距离 / (距离 + 旋转z)
  result = (旋转y * 缩放, p.y * 缩放, 旋转z)

proc 正交投影*(p: 三维点): tuple[x, y: float] =
  ## 等距正交投影
  result = (p.x - p.z * 0.5, p.y - p.z * 0.3)

# ===================================================================
# 编码→几何 转换
# ===================================================================

proc 声母转坐标*(声母Idx: int): 三维点 =
  ## 声母(25个)均匀分布在X轴极坐标上
  let 角度 = float(声母Idx) * 2.0 * PI / 25.0
  三维点(x: cos(角度) * 2.0, y: 0, z: sin(角度) * 2.0)

proc 韵母转坐标*(韵母Idx: int): 三维点 =
  ## 韵母(36个)分布在Y轴圆环上
  let 角度 = float(韵母Idx) * 2.0 * PI / 36.0
  三维点(x: cos(角度) * 3.0, y: 2.0, z: sin(角度) * 3.0)

proc 声调转高度*(声调: int): float =
  ## 0-4声对应 Y=3-7
  3.0 + float(声调) * 1.2

# ===================================================================
# 容器编码体构建
# ===================================================================

proc 新建编码体*(): 容器编码体 =
  new(result)
  result.体素表 = @[]
  result.几何表 = @[]
  result.min_p = 三维点(x: -999, y: -999, z: -999)
  result.max_p = 三维点(x: 999, y: 999, z: 999)

proc 添加体素*(c: var 容器编码体, p: 三维点, 字符: string, 层级: int, 极性: int, 价值: float) =
  var v: 体素
  new(v)
  v.中心 = p
  v.尺寸 = 0.4
  v.字符 = 字符
  v.层级 = 层级
  v.极性 = 极性
  v.价值 = 价值
  c.体素表.add(v)
  # 更新边界
  if p.x < c.min_p.x: c.min_p.x = p.x
  if p.y < c.min_p.y: c.min_p.y = p.y
  if p.z < c.min_p.z: c.min_p.z = p.z
  if p.x > c.max_p.x: c.max_p.x = p.x
  if p.y > c.max_p.y: c.max_p.y = p.y
  if p.z > c.max_p.z: c.max_p.z = p.z

proc 添加连线*(c: var 容器编码体, a, b: 三维点, w: float, kind: 几何体 = geLine) =
  c.几何表.add((kind, a, b, w))

# ===================================================================
# 汉字→编码体 全自动转换
# ===================================================================

proc 七层编码*(声母值, 韵母值, 声调值, 笔画值, 偏旁值: int): 容器编码体 =
  ## 七层数据 → 三维立体映射
  new(result)

  let pos声母 = 声母转坐标(声母值)
  let pos韵母 = 韵母转坐标(韵母值)
  let y声调 = 声调转高度(声调值)

  # L0-L2: 声母+韵母+声调 组成第一层(y=0..8)
  添加体素(result, pos声母, "●", 0, 0, 0.9)
  添加体素(result, pos韵母, "◆", 1, 0, 0.85)
  let pos声调 = 三维点(x: 0.0, y: y声调, z: 0.0)
  添加体素(result, pos声调, "▲", 2, 0, 0.8)
  添加连线(result, pos声母, pos韵母, 0.7)
  添加连线(result, pos韵母, pos声调, 0.6)

  # L3-L4: 笔画+偏旁组成第二层(y=8..12)
  let pos笔画 = 三维点(x: float(笔画值) * 1.0 - 3.0, y: 9.0, z: 0.0)
  let pos偏旁 = 三维点(x: float(偏旁值) * 0.6 - 6.0, y: 10.0, z: 2.0)
  添加体素(result, pos笔画, "■", 3, 1, 0.75)
  添加体素(result, pos偏旁, "★", 4, 1, 0.7)
  添加连线(result, pos声调, pos笔画, 0.5, geLine)
  添加连线(result, pos笔画, pos偏旁, 0.4)

  # L5-L6: 外语+泛音 悬浮在最高层
  let pos外语 = 三维点(x: -2.0, y: 12.0, z: 1.0)
  let pos泛音 = 三维点(x: 2.0, y: 13.0, z: -1.0)
  添加体素(result, pos外语, "◇", 5, 1, 0.6)
  添加体素(result, pos泛音, "○", 6, 1, 0.5)

  # 极锚: 我=1 刘楚恬=0
  let pos我 = 三维点(x: 0.0, y: -1.5, z: 0.0)
  let pos恬 = 三维点(x: 6.0, y: -1.8, z: 3.0)
  let pos原点 = 三维点(x: 0.0, y: 0.0, z: 0.0)
  添加体素(result, pos我, "我", 0, 1, 1.0)
  添加体素(result, pos恬, "T", 0, 0, 1.0)
  添加连线(result, pos我, pos原点, 0.8, geLine)
  添加连线(result, pos恬, pos原点, 0.8, geLine)

# ===================================================================
# 字符画渲染
# ===================================================================

proc 渲染线框*(c: 容器编码体, 视角: float = 0.0, 宽度: int = 60, 高度: int = 24): string =
  ## 渲染ASCII线框图(旋转视角观察)
  ## buf: seq[string], 每个string是一行字符
  var buf = newSeq[string]()
  for _ in 0..<高度:
    buf.add(repeat(" ", 宽度))

  # 中心偏移
  let 中x = float(宽度) / 2
  let 中y = float(高度) / 2

  # 辅助: 在(x,y)处放置一个字符
  proc 置字符(x, y: int, ch: char) =
    if x >= 0 and x < 宽度 and y >= 0 and y < 高度:
      buf[y][x] = ch

  # 投影每个体素到屏幕
  for v in c.体素表:
    let proj = 视投影(v.中心, 视角, 15.0)
    let sx = int(中x + proj.x * 3.0)
    let sy = int(中y - proj.y * 1.5)
    let ch: char = if v.字符.len > 0: v.字符[0] else: '.'
    置字符(sx, sy, ch)

  # 投影连线
  for g in c.几何表:
    let p1 = 视投影(g.p1, 视角, 15.0)
    let p2 = 视投影(g.p2, 视角, 15.0)
    let x1 = int(中x + p1.x * 3.0)
    let y1 = int(中y - p1.y * 1.5)
    let x2 = int(中x + p2.x * 3.0)
    let y2 = int(中y - p2.y * 1.5)
    # Bresenham画线
    let dx = abs(x2 - x1)
    let dy = abs(y2 - y1)
    let sx = if x1 < x2: 1 else: -1
    let sy = if y1 < y2: 1 else: -1
    var e = dx - dy
    var cx = x1
    var cy = y1
    for _ in 0..<200:
      if cx >= 0 and cx < 宽度 and cy >= 0 and cy < 高度:
        if buf[cy][cx] == ' ':
          置字符(cx, cy, '.')
      if cx == x2 and cy == y2: break
      let e2 = e * 2
      if e2 > -dy:
        e -= dy
        cx += sx
      if e2 < dx:
        e += dx
        cy += sy

  # 汇总为字符串
  var sb = ""
  sb.add("┌" & repeat("─", 宽度) & "┐\n")
  for row in buf:
    sb.add("│" & row & "│\n")
  sb.add("└" & repeat("─", 宽度) & "┘")
  result = sb

# ===================================================================
# 多视角旋转渲染
# ===================================================================

proc 旋转序列*(c: 容器编码体, 帧数: int = 6): seq[string] =
  for i in 0..<帧数:
    let 视角 = float(i) * 2.0 * PI / float(帧数)
    var sb = "═══ 视角 " & $i & " (" & $int(视角 * 180.0 / PI) & "°) ═══\n"
    sb.add(渲染线框(c, 视角))
    result.add(sb)

# ===================================================================
# DEMO
# ===================================================================

when isMainModule:
  echo "【满全法 · 三维几何可视化】"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"

  ## 编码「满」= m-an-3声-丨-氵
  let 满 = 七层编码(2, 14, 3, 0, 4)  # m=声母2, an=韵母14, 3声, 横=笔画0, 氵=偏旁4
  echo "◆ 「满」= m(声母) + an(韵母) + 三声 + 横(笔画) + 氵(偏旁)"
  echo ""
  echo 渲染线框(满, PI * 0.15)

  echo ""

  ## 编码「全」= q-üan-2声-丷-人
  let 全 = 七层编码(7, 33, 2, 3, 0)  # q=声母7, üan=韵母33, 2声, 点=笔画3, 全族
  echo "◆ 「全」= q(声母) + üan(韵母) + 二声 + 点(笔画) + 人(偏旁)"
  echo ""
  echo 渲染线框(全, PI * 0.25)

  echo ""

  ## 旋转观察
  echo "◆ 「满」= 360°旋转观察"
  echo ""
  let 帧 = 旋转序列(满, 4)
  for f in 帧:
    echo f
    echo ""

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  ●声母  ◆韵母  ▲声调  ■笔画  ★偏旁  ◇外语  ○泛音"
  echo "  我=1(锚)  恬=0(锚)"
