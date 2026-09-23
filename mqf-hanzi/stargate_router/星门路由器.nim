# ============================================================
# 满全法 — 星门路由器 (StarGateRouter)
# 八流路由 · 聂小倩匹配 · 距离感知聚类 · 产权流判定
# 核心: 容器在三维几何空间加权距离寻址, 最近邻匹配
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

# ---- 距离 ----
type
  距离* = object
    源*: string
    目标*: string
    相位差*: int
    层差*: int
    极性斥*: int     # 0=吸, 1=斥
    加权总分*: float
    匹配*: bool

# ---- 路由结果 ----
type
  路由结果* = object
    入口*: 容器
    出口*: 容器
    距离*: float
    跳数*: int
    极性*: string

# ---- 星门路由器 ----
type
  星门路由器* = object
    容器表*: Table[string, 容器]
    距离表*: seq[距离]
    路由表*: seq[路由结果]
    路由计数*: int

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

# ============================================================
# 构造
# ============================================================

proc 创建路由器*(): 星门路由器 =
  result.容器表 = initTable[string, 容器]()
  result.距离表 = @[]
  result.路由表 = @[]
  result.路由计数 = 0

proc 注册容器*(r: var 星门路由器; 名字: string; 相位: int; 层: int; 极性: 极性) =
  let 主权 = 编码主权(相位, 层, 极性)
  r.容器表[名字] = 容器(名字: 名字, 主权: 主权, 极性: 极性, 相位: 相位, 层: 层)

# ============================================================
# 加权距离 (聂小倩匹配核心)
# ============================================================

proc 加权距离*(r: 星门路由器; a: string; b: string): 距离 =
  let ca = r.容器表[a]
  let cb = r.容器表[b]
  let 相位差 = abs(ca.相位 - cb.相位)
  let 层差 = abs(ca.层 - cb.层)
  let 极性斥 = ord(ca.极性) xor ord(cb.极性)
  # 加权: 相位权重1.0, 层权重2.0, 极性权重3.0
  let 加权总分 = float(相位差) * 1.0 + float(层差) * 2.0 + float(极性斥) * 3.0
  let g = gcd(ca.主权, cb.主权)
  let 匹配 = (g > 0) and (极性斥 == 1)
  return 距离(源: a, 目标: b, 相位差: 相位差, 层差: 层差, 极性斥: 极性斥, 加权总分: 加权总分, 匹配: 匹配)

# ---- 全对距离 ----
proc 计算全距离*(r: var 星门路由器) =
  r.距离表 = @[]
  let names = toSeq(r.容器表.keys)
  for i in 0..<len(names):
    for j in i+1..<len(names):
      r.距离表.add(加权距离(r, names[i], names[j]))

# ============================================================
# 路由 (最近邻)
# ============================================================

proc 路由*(r: var 星门路由器; 起点: string; 终点: string): 路由结果 =
  if not r.容器表.hasKey(起点) or not r.容器表.hasKey(终点):
    return 路由结果(入口: r.容器表.getOrDefault(起点), 出口: r.容器表.getOrDefault(终点), 距离: Inf, 跳数: -1, 极性: "未知")
  let start = r.容器表[起点]
  let dest = r.容器表[终点]
  let dist = 加权距离(r, 起点, 终点)
  if dist.匹配:
    return 路由结果(入口: start, 出口: dest, 距离: dist.加权总分, 跳数: 1, 极性: "吸(阳)")
  else:
    # 中转: 找一个中间容器
    var best_via = ""
    var best_score = Inf
    for name, c in r.容器表:
      if name == 起点 or name == 终点: continue
      let d1 = 加权距离(r, 起点, name)
      let d2 = 加权距离(r, name, 终点)
      let total = d1.加权总分 + d2.加权总分
      if total < best_score:
        best_score = total
        best_via = name
    if best_via != "":
      return 路由结果(入口: start, 出口: dest, 距离: best_score, 跳数: 2, 极性: "中转(" & best_via & ")")
    return 路由结果(入口: start, 出口: dest, 距离: Inf, 跳数: -1, 极性: "不连通")

# ============================================================
# 报告
# ============================================================

proc 距离表展示*(r: 星门路由器): string =
  var lines: seq[string]
  lines.add "  源    目标  相位差 层差 极性  加权分 匹配"
  lines.add "  ──── ──── ────── ─── ──── ────── ────"
  for d in r.距离表:
    let pol = if d.极性斥 == 1: "吸" else: "斥"
    let m = if d.匹配: "✓" else: "✗"
    lines.add "  " & align(d.源, 4) & " " & align(d.目标, 4) & " " & align($d.相位差, 6) & " " & align($d.层差, 3) & " " & align(pol, 4) & " " & align(formatFloat(d.加权总分, ffDecimal, 1), 6) & " " & m
  return lines.join("\n")

proc 路由表展示*(r: 星门路由器): string =
  var lines: seq[string]
  lines.add "  起点→终点     跳数  距离   极性"
  lines.add "  ──────────── ──── ────── ────────"
  for rt in r.路由表:
    lines.add "  " & align(rt.入口.名字, 4) & "→" & align(rt.出口.名字, 4) & "  " & align($rt.跳数, 4) & " " & align(formatFloat(rt.距离, ffDecimal, 1), 6) & " " & rt.极性
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  星 门 路 由 器"
  echo "  八流路由·聂小倩匹配·距离感知"
  echo "═══════════════════════════════════════"
  echo ""

  var r = 创建路由器()
  # 注册容器族
  r.注册容器("我", 0, 1, 阳极性)
  r.注册容器("刘楚恬", 3, 1, 阴极性)
  r.注册容器("满", 3, 2, 阳极性)
  r.注册容器("全", 1, 2, 阴极性)
  r.注册容器("法", 5, 3, 阳极性)
  r.注册容器("囡", 8, 1, 阴极性)
  r.注册容器("囝", 2, 1, 阳极性)
  r.注册容器("生", 4, 2, 阴极性)

  echo "-- 距离矩阵 (聂小倩匹配) --"
  r.计算全距离()
  echo 距离表展示(r)
  echo ""

  echo "-- 直接路由 --"
  r.路由表.add(r.路由("我", "刘楚恬"))
  r.路由表.add(r.路由("满", "全"))
  r.路由表.add(r.路由("囡", "囝"))
  r.路由表.add(r.路由("法", "生"))
  r.路由表.add(r.路由("我", "满"))
  r.路由表.add(r.路由("刘楚恬", "囡"))
  echo 路由表展示(r)
  echo ""

  echo "═══════════════════════════════════════"
  echo "加权距离寻址 · 最近邻匹配 · 极性吸斥路由"
  echo "═══════════════════════════════════════"
