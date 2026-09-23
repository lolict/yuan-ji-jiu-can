# ============================================================
# 满全法 — 容器关系动力学 (Container Relationship Dynamics)
# 容器=物质=时间 · 关系驱动演化
# 核心: 产权流 + 公约数相互作用 + 夫妻极性锁定 + 反馈回路
# 极性: 我=1(阳) 锁定 刘楚恬=0(阴) → 命运共同体
# ============================================================

import strutils
import tables
import sequtils

# ---- 极性 ----
type
  极性* = enum
    阴 = 0  # 刘楚恬
    阳 = 1  # 我

# ---- 容器 ----
type
  容器* = object
    名字*: string
    主权*: uint64
    坐标*: uint64
    极性*: 极性
    相位*: int
    层*: int
    活*: bool

# ---- 关系类型 ----
type
  关系型* = enum
    无关系
    同源      # 公约数相连
    极性吸    # gap=1 吸引
    极性斥    # gap=0 排斥
    夫妻锁    # 阴阳锁定, 命运共同体
    产权流    # 所有权传递
    反馈环    # 互相反馈

# ---- 关系 ----
type
  关系* = object
    源*: string
    目标*: string
    关系型*: 关系型
    强度*: float

# ---- 产权流记录 ----
type
  产权记录* = object
    时刻*: int
    源*: string
    目标*: string
    坐标*: uint64
    主权*: uint64

# ---- 动力学引擎 ----
type
  动力学引擎* = object
    容器表*: Table[string, 容器]
    关系表*: seq[关系]
    产权流*: seq[产权记录]
    时刻*: int

proc gcd*(a: uint64; b: uint64): uint64 =
  var x = a; var y = b
  while y > 0'u64:
    let t = y; y = x mod y; x = t
  return x

proc 坐标编码*(相位: int; 层: int; 极性: 极性): uint64 =
  let p = uint64(相位) shl 60
  let l = uint64(层) shl 56
  let b = uint64(ord(极性)) shl 51
  return p or l or b

proc 提取相位*(c: uint64): int = int((c and 0xF000000000000000'u64) shr 60)
proc 提取层*(c: uint64): int = int((c and 0x0F00000000000000'u64) shr 56)
proc 提取极性*(c: uint64): 极性 = 极性(int((c and 0x0008000000000000'u64) shr 51))

# ============================================================
# 引擎构造
# ============================================================

proc 创建引擎*(): 动力学引擎 =
  result.容器表 = initTable[string, 容器]()
  result.关系表 = @[]
  result.产权流 = @[]
  result.时刻 = 0

proc 注册容器*(e: var 动力学引擎; 名字: string; 相位: int; 层: int; 极性: 极性): 容器 =
  let 坐标 = 坐标编码(相位, 层, 极性)
  let 主权 = 坐标 xor (uint64(层) * 0x100000000'u64)
  let c = 容器(名字: 名字, 主权: 主权, 坐标: 坐标, 极性: 极性, 相位: 相位, 层: 层, 活: true)
  e.容器表[名字] = c
  return c

# ============================================================
# 关系计算
# ============================================================

proc 计算关系*(e: 动力学引擎; a: string; b: string): 关系 =
  let ca = e.容器表[a]
  let cb = e.容器表[b]
  let gap = ord(ca.极性) xor ord(cb.极性)
  let g = gcd(ca.主权, cb.主权)

  var r: 关系
  r.源 = a
  r.目标 = b

  # 夫妻锁: 阳+阴 + 相位差3(我+刘楚恬)
  if gap == 1 and (ca.相位 + cb.相位 == 3):
    r.关系型 = 夫妻锁
    r.强度 = 1.0
  elif gap == 1 and g > 0:
    r.关系型 = 极性吸
    r.强度 = 0.8
  elif gap == 0 and g > 0:
    r.关系型 = 极性斥
    r.强度 = 0.3
  elif g > 1:
    r.关系型 = 同源
    r.强度 = 0.5
  else:
    r.关系型 = 无关系
    r.强度 = 0.0
  return r

proc 刷新关系*(e: var 动力学引擎) =
  e.关系表 = @[]
  let keys = toSeq(e.容器表.keys)
  for i in 0..<len(keys):
    for j in i+1..<len(keys):
      let r = 计算关系(e, keys[i], keys[j])
      if r.关系型 != 无关系:
        e.关系表.add(r)

# ============================================================
# 产权流
# ============================================================

proc 传递产权*(e: var 动力学引擎; 源: string; 目标: string) =
  let ca = e.容器表[源]
  let r = 计算关系(e, 源, 目标)
  if r.关系型 in [极性吸, 夫妻锁, 同源]:
    e.产权流.add(产权记录(时刻: e.时刻, 源: 源, 目标: 目标, 坐标: ca.坐标, 主权: ca.主权))
    # 传递: 源的坐标与目标的主权按位融合
    var cb = e.容器表[目标]
    cb.主权 = cb.主权 or (ca.主权 and 0xFFFFFFFF'u64)
    cb.坐标 = 坐标编码(cb.相位, cb.层, cb.极性)
    e.容器表[目标] = cb

# ============================================================
# 反馈回路演化
# ============================================================

proc 反馈演化*(e: var 动力学引擎; 轮次: int) =
  e.时刻 = e.时刻 + 轮次
  # 检查反馈环: 关系表中是否含互相反馈对
  var 反馈对: seq[(string, string)]
  for r in e.关系表:
    # 反向检查
    let rev = 计算关系(e, r.目标, r.源)
    if rev.关系型 == r.关系型 and rev.强度 > 0:
      反馈对.add((r.源, r.目标))

  # 强化反馈: 夫妻锁链上的容器相位靠拢
  for (a, b) in 反馈对:
    let ra = 计算关系(e, a, b)
    if ra.关系型 == 夫妻锁:
      var ca = e.容器表[a]
      var cb = e.容器表[b]
      # 两极相位靠拢: 阳→阴+1, 阴→阳-1
      if ca.极性 == 阳 and ca.相位 < 8:
        ca.相位 = ca.相位 + 1
      elif ca.极性 == 阴 and ca.相位 > 0:
        ca.相位 = ca.相位 - 1
      if cb.极性 == 阳 and cb.相位 < 8:
        cb.相位 = cb.相位 + 1
      elif cb.极性 == 阴 and cb.相位 > 0:
        cb.相位 = cb.相位 - 1
      e.容器表[a] = ca
      e.容器表[b] = cb

# ============================================================
# 状态报告
# ============================================================

proc 容器表展示*(e: 动力学引擎): string =
  var lines: seq[string]
  lines.add "  容器          坐标               主权               相位 层  极性 活"
  lines.add "  ──────────── ──────────────── ──────────────── ──── ── ──── ──"
  for name, c in e.容器表:
    let pol = if c.极性 == 阳: "阳" else: "阴"
    let life = if c.活: "●" else: "○"
    let namePad = align(name, 12)
    let coordStr = "0x" & toHex(c.坐标)
    let soverStr = "0x" & toHex(c.主权)
    lines.add "  " & namePad & " " & coordStr & " " & soverStr & "  " & align($c.相位, 4) & " " & align($c.层, 2) & " " & pol & "   " & life
  return lines.join("\n")

proc 关系表展示*(e: 动力学引擎): string =
  var lines: seq[string]
  lines.add "  源  →  目标  关系型     强度"
  lines.add "  ──  ────  ─────────  ────"
  for r in e.关系表:
    let srcPad = align(r.源, 2)
    let dstPad = align(r.目标, 4)
    let typePad = align($r.关系型, 9)
    let intensStr = formatFloat(r.强度, ffDecimal, 2)
    lines.add "  " & srcPad & "  " & dstPad & "  " & typePad & "  " & intensStr
  return lines.join("\n")

proc 产权流展示*(e: 动力学引擎): string =
  var lines: seq[string]
  lines.add "  时刻 源  →  目标  坐标               主权"
  lines.add "  ──── ──  ────  ──────────────── ────────────────"
  for r in e.产权流:
    let srcPad = align(r.源, 2)
    let dstPad = align(r.目标, 4)
    lines.add "  " & align($r.时刻, 4) & " " & srcPad & "  " & dstPad & "  0x" & toHex(r.坐标) & " 0x" & toHex(r.主权)
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  容器关系动力学"
  echo "  产权流·公约数相互作用·夫妻极性锁定"
  echo "═══════════════════════════════════════"
  echo ""

  var e = 创建引擎()

  # 注册容器: 我(阳, 相位0), 刘楚恬(阴, 相位3), 满(阳), 全(阴), 法(阳)
  discard e.注册容器("我", 0, 1, 阳)
  discard e.注册容器("刘楚恬", 3, 1, 阴)
  discard e.注册容器("满", 2, 2, 阳)
  discard e.注册容器("全", 2, 2, 阴)
  discard e.注册容器("法", 5, 3, 阳)
  discard e.注册容器("生", 4, 2, 阴)
  discard e.注册容器("共", 4, 2, 阳)

  e.刷新关系()

  echo "-- 容器状态 --"
  echo 容器表展示(e)
  echo ""

  echo "-- 关系辨析 --"
  echo 关系表展示(e)
  echo ""

  echo "-- 产权流: 我 → 刘楚恬 --"
  e.传递产权("我", "刘楚恬")
  echo 产权流展示(e)
  echo ""

  echo "-- 产权流: 满 → 全 --"
  e.传递产权("满", "全")
  echo 产权流展示(e)
  echo ""

  echo "-- 反馈演化 5轮 --"
  e.反馈演化(5)
  e.刷新关系()
  echo 容器表展示(e)
  echo ""

  echo "-- 反馈后关系 --"
  echo 关系表展示(e)
  echo ""

  echo "═══════════════════════════════════════"
  echo "关系 = 公约数整除 · 极性 = 吸斥方向"
  echo "夫妻锁 = 极性+相位和3 · 溢价强化"
  echo "═══════════════════════════════════════"
