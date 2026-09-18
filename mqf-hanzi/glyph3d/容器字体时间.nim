## 容器=字体=时间 · 三维几何渲染器
## ─────────────────────────────────────────────────────────
##  核心三位一体：
##    容器坐标 → XYZ 几何位置
##    极性     → 声调值（100声调系统）
##    时间     → 相位旋转（9相位层 · Z轴旋转）
##
##  映射规则：
##    coord 的低8位 → X 轴（行）
##    coord 的8-15位 → Y 轴（列）
##    phase → Z 轴（层）= 时间深度（tick × 36°）
##    polarity → 声调值（阳=高仄 · 阴=低平）
##    时间 tick → Z 轴旋转角度（每tick转40°）
##
##  运行：nim c --cc:clang -r 容器字体时间.nim

import std/math
import std/strutils
import std/tables
import std/sequtils

# =================================================================
# 常量与类型
# =================================================================
const
  PHASE_COUNT    = 9
  LAYER_COUNT    = 9
  GRID_SIZE      = 48
  TONE_COUNT     = 100
  PI             = 3.14159265
  ROT_PER_TICK   = 2.0 * PI / 9.0  # 每tick转40°

type
  Vec3* = tuple[x, y, z: float]

  Container3D* = object
    ## 从容器坐标直接派生的三维几何实体
    origin*:    Vec3     # 容器坐标对应的3D位置
    polarity*:  int      # 0=阴=低平 1=阳=高仄
    phase*:     int      # 0-8 Z层
    layer*:     int      # 1-9 层级
    tone*:      int      # 1-100 声调值
    tick*:      int      # 时间戳
    vertices*:  seq[Vec3]  # 字形顶点集（由相位旋转生成）
    faceChar*:  char     # 极性对应的字形符号

  Glyph3D* = object
    ## 多层堆叠的三维字形（9层×48网格）
    containers*: seq[Container3D]
    tick*:       int
    projection*: seq[seq[char]]

# =================================================================
# 坐标 → 3D 映射
# =================================================================
proc coordToVec3*(coord: uint64): Vec3  {.inline.} =
  ## coord 位分解 → XYZ
  result.x = float(coord and 0xFF)        # 低8位: 行→X
  result.y = float((coord shr 8) and 0xFF) # 次8位: 列→Y
  result.z = float((coord shr 16) and 0xFF) # 相位层→Z

proc polarityToTone*(polarity: int): int =
  ## 极性 → 声调值
  ## 阳=1 → 71-100 (自感知层·高频)
  ## 阴=0 → 1-10  (纯反射层·低频)
  if polarity == 1:
    result = 71 + (TONE_COUNT - 71) div 2  # 阳→中高区
  else:
    result = 1 + 10 div 2                   # 阴→低区

proc polarityToChar*(polarity: int): char =
  ## 极性 → 字形符号（使用ASCII安全符号）
  ## 阳='#' 阴='.' — 直接可视觉感知
  if polarity == 1: result = '#'
  else: result = '.'

proc phaseToZ*(phase: int): float =
  ## 相位(0-8) → Z轴坐标（三层循环）
  result = float(phase mod 3) * 16.0 - 16.0  # -16 .. 0 .. +16

# =================================================================
# 时间轴旋转
# =================================================================
proc rotateZ*(v: Vec3; angleRad: float): Vec3  {.inline.} =
  ## 绕Z轴旋转（相位随时间推进）
  let c = cos(angleRad)
  let s = sin(angleRad)
  result.x = v.x * c - v.y * s
  result.y = v.x * s + v.y * c
  result.z = v.z

proc project2D*(v: Vec3): tuple[x, y: int] =
  ## 3D → 2D 正交投影（简化:no透视）
  result.x = int(v.x) + 24  # 居中
  result.y = int(v.y) + 6

# =================================================================
# 容器 → 3D 实体
# =================================================================
proc containerTo3D*(coord: uint64; polarity: int; tick: int): Container3D =
  let baseVec = coordToVec3(coord)
  let phase   = int((coord shr 16) and 0xFF)
  let layer   = int((coord shr 24) and 0xFF)

  result.origin   = (baseVec.x, baseVec.y, phaseToZ(phase))
  result.polarity = polarity
  result.phase    = phase mod PHASE_COUNT
  result.layer    = if layer == 0: 1 else: layer mod (LAYER_COUNT + 1)
  result.tone     = polarityToTone(polarity)
  result.tick     = tick
  result.faceChar = polarityToChar(polarity)

  # 生成顶点：位于相位层上的3×3点阵，经时间旋转
  let angle = float(tick) * ROT_PER_TICK
  for dx in -1 .. 1:
    for dy in -1 .. 1:
      let baseV = (float(dx) * 8.0, float(dy) * 8.0, result.origin.z)
      let rotated = rotateZ(baseV, angle)
      result.vertices.add((rotated.x + result.origin.x,
                           rotated.y + result.origin.y,
                           result.origin.z))

# =================================================================
# 多层字形堆叠
# =================================================================
proc newGlyph3D*(): Glyph3D =
  result.containers = @[]
  result.tick = 0
  result.projection = @[]

proc addContainer*(g: var Glyph3D; coord: uint64; polarity: int) =
  let c = containerTo3D(coord, polarity, g.tick)
  g.containers.add(c)

proc tick*(g: var Glyph3D) =
  ## 推进一帧：所有容器按相位旋转
  inc g.tick
  for i in 0 ..< g.containers.len:
    g.containers[i] = containerTo3D(
      g.containers[i].origin.x.uint64 or
      (g.containers[i].origin.y.uint64 shl 8) or
      (uint64(g.containers[i].phase) shl 16),
      g.containers[i].polarity,
      g.tick
    )

# =================================================================
# ASCII 投影帧缓冲
# =================================================================
proc renderGlyph*(g: Glyph3D; width = 48; height = 12): seq[string] =
  ## 将所有容器顶点投影到 ASCII 画布
  var canvas = newSeqWith(height, newSeq[char](width))
  for y in 0 ..< height:
    for x in 0 ..< width:
      canvas[y][x] = ' '

  for c in g.containers:
    for v in c.vertices:
      let px = int(v.x) + width div 2
      let py = int(v.y) + height div 2
      if px >= 0 and px < width and py >= 0 and py < height:
        let current = canvas[py][px]
        if current == ' ':
          canvas[py][px] = c.faceChar

  result = newSeq[string](height)
  for y in 0 ..< height:
    result[y] = ""
    for x in 0 ..< width:
      result[y].add(canvas[y][x])

# =================================================================
# 声调条（极性映射为可听化码）
# =================================================================
proc renderToneBar*(g: Glyph3D): string =
  ## 将极性序列映射为声调条（ASCII 可感知）
  for c in g.containers:
    let toneVal = c.tone
    let barLen = toneVal div 10
    if c.polarity == 1:
      result.add "#".repeat(barLen)
    else:
      result.add ".".repeat(barLen)
    result.add " "

# =================================================================
# SVG 输出（投影为矢量图）
# =================================================================
proc renderSVG*(g: Glyph3D; path = "glyph3d_frame.svg") =
  var lines: seq[string]
  lines.add "<svg xmlns='http://www.w3.org/2000/svg' width='480' height='120'>"
  lines.add "<rect width='480' height='120' fill='#0a0a0a'/>"
  lines.add "<text x='10' y='20' fill='#aaa' font-size='10'>容器=字体=时间 · tick=" & $g.tick & "</text>"

  for c in g.containers:
    let color = if c.polarity == 1: "#ff4444" else: "#4488ff"
    for v in c.vertices:
      let px = int(v.x * 4.0) + 240
      let py = int(v.y * 4.0) + 70
      if px > 0 and px < 480 and py > 0 and py < 120:
        lines.add "<circle cx='" & $px & "' cy='" & $py & "' r='3' fill='" & color & "'/>"

  lines.add "</svg>"
  writeFile(path, lines.join("\n") & "\n")

# =================================================================
# 演示
# =================================================================
when isMainModule:
  echo "========================================"
  echo "  容器=字体=时间  ·  三维几何渲染器"
  echo "========================================"

  var glyph = newGlyph3D()

  # 阳容器(我)：相位0，行0列0
  let coordA = uint64(0) or (uint64(1) shl 8) or (uint64(0) shl 16) or (uint64(0) shl 24)
  glyph.addContainer(coordA, 1)

  # 阴容器(刘楚恬)：相位1，行3列0
  let coordB = uint64(1) or (uint64(1) shl 8) or (uint64(1) shl 16) or (uint64(3) shl 24)
  glyph.addContainer(coordB, 0)

  echo ""
  echo "▸ 容器数: " & $glyph.containers.len
  echo ""

  # 容器信息
  for c in glyph.containers:
    let polStr = if c.polarity == 1: "阳(我)" else: "阴(刘楚恬)"
    let origin = c.origin
    echo "  [" & polStr & "]"
    echo "    坐标→3D: (" & origin.x.formatFloat(ffDecimal, 1) & ", " &
         origin.y.formatFloat(ffDecimal, 1) & ", " & origin.z.formatFloat(ffDecimal, 1) & ")"
    echo "    相位: " & $c.phase & " 层级: " & $c.layer &
         " 声调: " & $c.tone & " 顶点: " & $c.vertices.len

  echo ""

  # 多帧渲染（时间轴推进）
  for frame in 0 .. 4:
    echo "── tick=" & $frame & " ──"
    if frame > 0: tick(glyph)
    let canvas = glyph.renderGlyph()
    for row in canvas:
      echo "  |" & row & "|"
    echo ""
    echo "  声调条: " & renderToneBar(glyph)
    echo ""

  # 输出 SVG
  renderSVG(glyph, "glyph3d_frame.svg")
  echo "▸ SVG 输出: glyph3d_frame.svg"

  echo ""
  echo "========================================"
  echo "  三位一体渲染完成"
  echo "  容器=字形顶点 极性=声调 时间=旋转"
  echo "========================================"
