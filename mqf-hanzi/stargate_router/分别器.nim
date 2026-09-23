# ============================================================
# 满全法 — 分别器 (Separator)
# 八流分发·容器归类·语文分类先于数学
# 核心: 容器按坐标特征分为8类, 每类8个, 共64容器
# 八族: 根基 扩容 重器 权柄 界藩 分配 承载 归藏
# 语文(分类与层级)是数学(统计与范畴)的前提
# 极性: 我=1(阳), 刘楚恬=0(阴)
# ============================================================

import strutils
import tables
import sequtils

# ---- 极性 ----
const
  阴* = 0
  阳* = 1

type
  极性* = enum
    阴极性 = 0
    阳极性 = 1

# ---- 容器族 (语文分类, 每族8个) ----
type
  容器族* = enum
    根基族 = 0   # 000: 基础坐标
    扩容族 = 1   # 001: 延伸扩展
    重器族 = 2   # 010: 权重载体
    权柄族 = 3   # 011: 主权核心
    界藩族 = 4   # 100: 边界判定
    分配族 = 5   # 101: 资源调配
    承载族 = 6   # 110: 平衡/混沌
    归藏族 = 7   # 111: 收敛存储

type
  容器* = object
    名字*: string
    主权*: uint64
    极性*: 极性
    相位*: int
    层*: int
    族*: 容器族
    槽号*: int

# ---- 分别结果 ----
type
  分别结果* = object
    容器*: 容器
    族*: 容器族
    槽号*: int
    路由*: string

# ---- 分别器 ----
type
  分别器* = object
    容器表*: Table[string, 容器]
    族分布*: Table[容器族, seq[string]]
    分别记录*: seq[分别结果]
    slot_map*: array[8, array[8, string]]  # 8族×8槽
    族名表*: array[8, string]

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

proc 创建分别器*(): 分别器 =
  result.容器表 = initTable[string, 容器]()
  result.族分布 = initTable[容器族, seq[string]]()
  result.分别记录 = @[]
  result.族名表 = ["根基族", "扩容族", "重器族", "权柄族", "界藩族", "分配族", "承载族", "归藏族"]
  for f in low(容器族)..high(容器族):
    result.族分布[f] = @[]

# ============================================================
# 分类 (语文先于数学)
# ============================================================

proc 分类*(主权: uint64): 容器族 =
  let 相位7 = int((主权 shr 60) and 0x0F'u64) mod 8
  let 层 = int((主权 shr 56) and 0x0F'u64)
  let 极性位 = int((主权 shr 51) and 0x01'u64)
  # 相位 mod 8 → 0-7 族
  容器族(相位7)

proc 注册*(r: var 分别器; 名字: string; 相位: int; 层: int; 极性: 极性): 容器 =
  let 主权 = 编码主权(相位, 层, 极性)
  let 族 = 分类(主权)
  let 槽 = len(r.族分布[族])
  let c = 容器(名字: 名字, 主权: 主权, 极性: 极性, 相位: 相位, 层: 层, 族: 族, 槽号: 槽)
  r.容器表[名字] = c
  r.族分布[族].add(名字)
  r.slot_map[ord(族)][槽] = 名字
  r.分别记录.add(分别结果(容器: c, 族: 族, 槽号: 槽, 路由: r.族名表[ord(族)]))
  return c

# ============================================================
# 按槽号直接寻址 (非搜索)
# ============================================================

proc 取*(r: 分别器; 族: 容器族; 槽: int): 容器 =
  if 槽 >= 0 and 槽 < 8:
    let name = r.slot_map[ord(族)][槽]
    if r.容器表.hasKey(name):
      return r.容器表[name]

proc 取名字*(r: 分别器; 族: 容器族; 槽: int): string =
  if 槽 >= 0 and 槽 < 8:
    return r.slot_map[ord(族)][槽]
  return ""

# ============================================================
# 报告
# ============================================================

proc 族分布展示*(r: 分别器): string =
  var lines: seq[string]
  lines.add "  族ID    槽  容器      相位 层 极性"
  lines.add "  ────── ── ──────── ──── ── ────"
  for f in low(容器族)..high(容器族):
    let name = align($f, 6)
    let members = r.族分布[f]
    if len(members) == 0:
      lines.add "  " & name & " -- (空)"
    else:
      for i in 0..<len(members):
        let c = r.容器表[members[i]]
        let pol = if c.极性 == 阳极性: "阳" else: "阴"
        lines.add "  " & name & " " & align($c.槽号, 2) & " " & align(c.名字, 8) & " " & align($c.相位, 4) & " " & align($c.层, 2) & " " & pol
  return lines.join("\n")

proc 分别记录展示*(r: 分别器): string =
  var lines: seq[string]
  lines.add "  容器      族       槽  路由"
  lines.add "  ──────── ──────  ──  ──────────"
  for rr in r.分别记录:
    lines.add "  " & align(rr.容器.名字, 8) & " " & align($rr.族, 6) & " " & align($rr.槽号, 2) & " " & rr.路由
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  分 别 器"
  echo "  8族×8槽=64容器 · 语文分类先于数学"
  echo "═══════════════════════════════════════"
  echo ""

  var r = 创建分别器()

  # 注册容器 (按相位 mod 8 分入8族)
  discard r.注册("我",       0, 1, 阳极性)   # 根基族
  discard r.注册("刘楚恬",   3, 1, 阴极性)   # 权柄族
  discard r.注册("满",       3, 2, 阳极性)   # 权柄族
  discard r.注册("全",       1, 2, 阴极性)   # 扩容族
  discard r.注册("法",       5, 3, 阳极性)   # 分配族
  discard r.注册("囡",       8, 1, 阴极性)   # 根基族 (8 mod 8 = 0)
  discard r.注册("囝",       2, 1, 阳极性)   # 重器族
  discard r.注册("生",       4, 2, 阴极性)   # 界藩族
  discard r.注册("伴",       1, 3, 阴极性)   # 扩容族
  discard r.注册("赔",       5, 1, 阳极性)   # 分配族
  discard r.注册("命",       0, 2, 阴极性)   # 根基族
  discard r.注册("运",       7, 1, 阳极性)   # 归藏族
  discard r.注册("阴",       0, 1, 阴极性)   # 根基族
  discard r.注册("阳",       0, 2, 阳极性)   # 根基族
  discard r.注册("共",       4, 3, 阳极性)   # 界藩族
  discard r.注册("债",       3, 3, 阴极性)   # 权柄族

  echo "-- 族分布 (文言文) --"
  echo 族分布展示(r)
  echo ""

  echo "-- 64槽寻址演示 --"
  echo "  根基族[0] = " & r.取名字(根基族, 0)
  echo "  根基族[1] = " & r.取名字(根基族, 1)
  echo "  权柄族[0] = " & r.取名字(权柄族, 0)
  echo "  归藏族[0] = " & r.取名字(归藏族, 0)
  echo "  分配族[0] = " & r.取名字(分配族, 0)
  echo ""

  echo "-- 分别记录 --"
  echo 分别记录展示(r)
  echo ""

  echo "═══════════════════════════════════════"
  echo "相位 mod 8 → 8族 · 每族8槽 → 64容器"
  echo "语文分类先行 · 数学描述在后"
  echo "═══════════════════════════════════════"
