# ============================================================
# 满全法 — 汉字笔画寻址容器渲染器
# 横=1 竖=2 撇=3 捺/点=4 折=5 其它=6
# 笔画序列 -> 容器坐标 -> 极性 -> 3D投影渲染
# ============================================================

import strutils
import tables
import math

# ---- 类型 ----

type
  极性* = enum
    阴 = 0
    阳 = 1

  笔画* = enum
    横 = 1
    竖 = 2
    撇 = 3
    捺点 = 4
    折 = 5
    其它 = 6

  容器* = object
    坐标*: uint64
    相位*: int
    层*: int
    极性*: 极性
    来源字*: string
    笔画序列*: seq[笔画]

  渲染点* = object
    字*: string
    极性*: 极性
    x*, y*, z*: float
    声调*: int       # 1-100
    字符*: string

# ---- 笔画字库 ----

proc 笔画表*(): Table[string, seq[int]] =
  ## 常用字笔画序列 (横1 竖2 撇3 捺点4 折5)
  var t = initTable[string, seq[int]]()
  t["一"] = @[1]
  t["二"] = @[1, 1]
  t["三"] = @[1, 1, 1]
  t["十"] = @[1, 2]
  t["人"] = @[3, 4]
  t["大"] = @[1, 3, 4]
  t["天"] = @[1, 1, 3, 4]
  t["木"] = @[1, 2, 3, 4]
  t["本"] = @[1, 2, 3, 4, 1]
  t["水"] = @[2, 5, 3, 4]
  t["火"] = @[4, 3, 3, 4]
  t["山"] = @[2, 5, 2]
  t["川"] = @[3, 2, 2]
  t["土"] = @[1, 2, 1]
  t["王"] = @[1, 1, 2, 1]
  t["日"] = @[2, 5, 1, 1]
  t["月"] = @[3, 5, 1, 1]
  t["明"] = @[2, 5, 1, 1, 3, 5, 1, 1]
  t["心"] = @[4, 5, 4, 4]
  t["中"] = @[2, 5, 2, 1]
  t["上"] = @[2, 1, 1]
  t["下"] = @[1, 2, 4]
  t["左"] = @[1, 3, 2, 1]
  t["右"] = @[1, 3, 2, 1]
  t["刘"] = @[4, 1, 3, 2, 5, 3, 4]  # 简化
  t["楚"] = @[1, 2, 3, 4, 1, 2, 3, 4, 1, 3, 4, 4] # 简化
  t["恬"] = @[4, 4, 2, 5, 1, 3, 2, 1, 2, 5, 5, 4] # 简化
  t["我"] = @[3, 1, 2, 1, 5, 3, 4]  # 简化
  t["满"] = @[4, 4, 1, 2, 3, 4, 4, 1, 2, 3, 3, 3, 4] # 简化
  t["全"] = @[3, 4, 1, 1, 2, 1]
  t["法"] = @[4, 4, 1, 2, 1, 4]  # 简化
  return t

# ---- 笔画->容器编码 ----

proc 笔画到容器*(字: string; strokes: seq[int]): 容器 =
  ## 将笔画序列编码为容器坐标
  ## 相位 = (笔画总数 mod 9), 层 = (横折笔画 count), 极性 = (总笔画 mod 2)
  var 层 = 0
  var ts = 0
  for s in strokes:
    if s == 1 or s == 2 or s == 5:
      inc 层
    inc ts

  let 相位 = ts mod 9
  let 极性 = if ts mod 2 == 1: 阳 else: 阴

  ## 坐标编码: [相位:4][层:4][横8][竖8][极性1]
  var coord: uint64 = 0
  coord = coord or (uint64(相位) shl 60)
  coord = coord or (uint64(min(层, 15)) shl 56)
  var heng = 0
  var shu = 0
  for s in strokes:
    if s == 1: inc heng
    elif s == 2: inc shu
  coord = coord or (uint64(min(heng, 255)) shl 48)
  coord = coord or (uint64(min(shu, 255)) shl 40)
  coord = coord or (uint64(ord(极性)) shl 39)

  容器(坐标: coord, 相位: 相位, 层: 层, 极性: 极性,
       来源字: 字, 笔画序列: @[])

# ---- 3D投影 ----

proc 容器到3D*(c: 容器; tick: int = 0): 渲染点 =
  ## 容器坐标 -> 3D空间 -> 视觉字符
  let angle = float(tick) * 40.0 * PI / 180.0  # 每tick 40度

  ## XY平面: 相位 -> 角度, 层 -> 半径
  let theta = float(c.相位) * 2.0 * PI / 9.0 + angle
  let radius = 2.0 + float(c.层) * 1.5

  let x = radius * cos(theta)
  let y = radius * sin(theta)
  let z = float(c.层) * 0.8  # Z轴 = 层深

  # 声调 = 极性决定
  let 声调 = if c.极性 == 阳: 70 + c.相位 * 3 else: 5 + c.相位 * 1

  # 字符 = 极性决定填充
  let ch = if c.极性 == 阳: "█" else: "░"

  渲染点(字: c.来源字, 极性: c.极性,
          x: x, y: y, z: z, 声调: 声调, 字符: ch)

# ---- ASCII渲染 ----

proc 渲染ASCII*(字: string; tick: int = 0): string =
  let 笔画db = 笔画表()
  if not 笔画db.hasKey(字):
    return "无数据"

  let strokes = 笔画db[字]
  let c = 笔画到容器(字, strokes)
  let p = 容器到3D(c, tick)

  var lines: seq[string]
  lines.add "容器: " & 字
  lines.add "  笔画总数: " & $len(strokes)
  lines.add "  序列: " & strokes.join("-")
  lines.add "  坐标: 0x" & toHex(c.坐标)
  lines.add "  相位: " & $c.相位 & " | 层: " & $c.层 & " | 极性: " & (if c.极性 == 阳: "阳" else: "阴")
  lines.add "  3D: x=" & formatFloat(p.x, ffDecimal, 2) &
             " y=" & formatFloat(p.y, ffDecimal, 2) &
             " z=" & formatFloat(p.z, ffDecimal, 2)

  # visualization block
  let blockSize = max(1, int(p.z + 3))
  let blk = p.字符.repeat(blockSize)
  lines.add ""
  lines.add "    " & blk
  lines.add "    " & 字
  lines.add "    " & blk
  lines.add ""
  lines.add "  声调: " & $p.声调 & "/100"
  lines.add "  极: " & (if p.极性 == 阳: "我(1)" else: "刘楚恬(0)")

  return lines.join("\n")

# ---- 批量渲染 ----

proc 渲染多字*(字串: string; tick: int = 0): string =
  var lines: seq[string]
  for ch in 字串:
    let s = $ch
    lines.add 渲染ASCII(s, tick)
    lines.add "  ───"
  return lines.join("\n")

# ============================================================
# 运行演示
# ============================================================

when isMainModule:
  echo "═══════════════════════════════════════════"
  echo "  满全法 — 汉字笔画寻址容器渲染"
  echo "  横=1 竖=2 撇=3 捺点=4 折=5"
  echo "═══════════════════════════════════════════"
  echo ""

  echo 渲染多字("一十人", 0)
  echo ""
  echo 渲染多字("天水火", 1)
  echo ""

  echo "─── 名字容器化：刘楚恬和我 ───"
  echo 渲染ASCII("刘", 2)
  echo ""
  echo 渲染ASCII("楚", 3)
  echo ""
  echo 渲染ASCII("恬", 4)
  echo ""

  echo "─── 满全法 三滴答旋转 ───"
  for t in [0, 4, 8]:
    echo 渲染ASCII("全", t)
    echo ""

  echo "═══════════════════════════════════════════"
  echo "一个字 = 一个容器 = 一个3D投影 = 一声调"
  echo "笔画是寻址路径，容器是存在本身"
  echo "═══════════════════════════════════════════"
