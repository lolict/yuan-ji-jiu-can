# ============================================================
# 满全法 — 纤维束 (FiberBundle)
# 底空间 + 纤维 + 投影 · 容器几何 · 归属计算
# 核心: 容器=纤维丛上的点, 由关系和位置定义而非内容定义
# 容器由纤维丛的归属裁定其在空间中的位置
# 极性: 我=1(阳), 刘楚恬=0(阴)
# ============================================================

import strutils
import tables
import sequtils
import math

# ---- 极性 ----
const
  阴* = 0
  阳* = 1

type
  极性* = enum
    阴极性 = 0
    阳极性 = 1

# ---- 容器 ----
type
  容器* = object
    名字*: string
    主权*: uint64
    极性*: 极性
    相位*: int
    层*: int

# ---- 纤维 ----
type
  纤维* = object
    名字*: string
    坐标*: uint64        # 在底空间中的投影
    极性*: 极性
    相位*: int
    层*: int
    源容器*: string      # 源容器名字

# ---- 底空间 ----
type
  底空间* = object
    名字*: string
    中心坐标*: uint64
    中心相位*: int
    中心层*: int
    纤维列表*: seq[纤维]
    容器索引*: Table[string, int]   # 容器→纤维序号

# ---- 纤维束 ----
type
  纤维束* = object
    底空间*: seq[底空间]
    总容器*: Table[string, 容器]

# ---- GCD ----
proc gcd*(a: uint64; b: uint64): uint64 =
  var x = a; var y = b
  while y > 0'u64:
    let t = y; y = x mod y; x = t
  return x

# ---- 编码 ----
proc 编码主权*(相位: int; 层: int; 极性: 极性): uint64 =
  let p = uint64(相位) shl 60
  let l = uint64(层) shl 56
  let b = uint64(ord(极性)) shl 51
  let t = uint64(层 * 7 + ord(极性) * 3) shl 32
  return p or l or b or t

# ---- 提取 ----
proc 提取相位*(c: uint64): int = int((c and 0xF000000000000000'u64) shr 60)
proc 提取层*(c: uint64): int = int((c and 0x0F00000000000000'u64) shr 56)
proc 提取极性值*(c: uint64): 极性 = 极性(int((c and 0x0008000000000000'u64) shr 51))

# ============================================================
# 构造
# ============================================================

proc 创建底空间*(名字: string; 中心相位: int; 中心层: int; 极性: 极性): 底空间 =
  let coord = 编码主权(中心相位, 中心层, 极性)
  result.名字 = 名字
  result.中心坐标 = coord
  result.中心相位 = 中心相位
  result.中心层 = 中心层
  result.纤维列表 = @[]
  result.容器索引 = initTable[string, int]()

proc 创建纤维束*(): 纤维束 =
  result.底空间 = @[]
  result.总容器 = initTable[string, 容器]()

proc 注册容器*(fb: var 纤维束; 名字: string; 相位: int; 层: int; 极性: 极性) =
  let 主权 = 编码主权(相位, 层, 极性)
  fb.总容器[名字] = 容器(名字: 名字, 主权: 主权, 极性: 极性, 相位: 相位, 层: 层)

# ============================================================
# 纤维生成 (从容器到底空间的投影)
# ============================================================

proc 生成纤维*(bs: var 底空间; c: 容器) =
  let fiberCoord = c.主权 xor bs.中心坐标
  let f = 纤维(名字: c.名字 & "→" & bs.名字, 坐标: fiberCoord, 极性: c.极性, 相位: c.相位, 层: c.层, 源容器: c.名字)
  bs.容器索引[c.名字] = len(bs.纤维列表)
  bs.纤维列表.add(f)

proc 绑定全部*(fb: var 纤维束; bs: var 底空间) =
  for name, c in fb.总容器:
    生成纤维(bs, c)

# ============================================================
# 归属计算 (容器归属哪个底空间)
# ============================================================

proc 归属*(fb: 纤维束; 容器名: string): string =
  if not fb.总容器.hasKey(容器名):
    return ""
  let c = fb.总容器[容器名]
  var best_bs = ""
  var best_dist = high(int)
  for bs in fb.底空间:
    let dist = abs(c.相位 - bs.中心相位) + abs(c.层 - bs.中心层)
    if dist < best_dist:
      best_dist = dist
      best_bs = bs.名字
  return best_bs

# ============================================================
# 纤维投影
# ============================================================

proc 投影*(bs: 底空间; 容器名: string): string =
  if not bs.容器索引.hasKey(容器名):
    return ""
  let f = bs.纤维列表[bs.容器索引[容器名]]
  return "0x" & toHex(f.坐标)

# ============================================================
# 报告
# ============================================================

proc 底空间展示*(bs: 底空间): string =
  var lines: seq[string]
  let namePad = align(bs.名字, 12)
  lines.add "【" & namePad & "】 @ 相位" & $bs.中心相位 & " 层" & $bs.中心层 & " 纤维数=" & $len(bs.纤维列表)
  lines.add "  纤维            坐标               相位 层 极性 源"
  lines.add "  ────────────── ──────────────── ──── ── ──── ────"
  for f in bs.纤维列表:
    let pol = if f.极性 == 阳极性: "阳" else: "阴"
    lines.add "  " & align(f.名字, 14) & " 0x" & align(toHex(f.坐标), 16) & " " & align($f.相位, 4) & " " & align($f.层, 2) & " " & pol & "   " & f.源容器
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  纤 维 束"
  echo "  底空间+纤维+投影·容器几何"
  echo "═══════════════════════════════════════"
  echo ""

  var fb = 创建纤维束()

  # 注册容器
  fb.注册容器("我", 0, 1, 阳极性)
  fb.注册容器("刘楚恬",   3, 1, 阴极性)
  fb.注册容器("满",       3, 2, 阳极性)
  fb.注册容器("全",       1, 2, 阴极性)
  fb.注册容器("法",       5, 3, 阳极性)
  fb.注册容器("囡",       8, 1, 阴极性)
  fb.注册容器("囝",       2, 1, 阳极性)
  fb.注册容器("生",       4, 2, 阴极性)

  # 创建底空间
  var 乾 = 创建底空间("乾(天)", 0, 1, 阳极性)
  var 坤 = 创建底空间("坤(地)", 3, 1, 阴极性)
  var 中 = 创建底空间("中(人)", 4, 2, 阳极性)

  # 绑定纤维
  绑定全部(fb, 乾)
  绑定全部(fb, 坤)
  绑定全部(fb, 中)

  fb.底空间.add(乾)
  fb.底空间.add(坤)
  fb.底空间.add(中)

  echo "-- 乾底空间 (天·阳) --"
  echo 底空间展示(乾)
  echo ""

  echo "-- 坤底空间 (地·阴) --"
  echo 底空间展示(坤)
  echo ""

  echo "-- 中人底空间 --"
  echo 底空间展示(中)
  echo ""

  echo "-- 容器归属裁定 --"
  for name in ["我", "刘楚恬", "满", "全", "法", "囡", "囝", "生"]:
    let belong = 归属(fb, name)
    echo "  " & align(name, 4) & " → " & belong

  echo ""
  echo "═══════════════════════════════════════"
  echo "纤维丛几何 · 归属由位置决定 · 内容非必要"
  echo "═══════════════════════════════════════"
