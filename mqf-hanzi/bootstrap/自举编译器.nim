# ============================================================
# 满全法 — 自举编译器 (Bootstrap Compiler)
# 容器规则 → 工具自生 → 系统从文件系统长出
# 核心:
#   1. 域规则驱动编译 (domain rules → code)
#   2. 索引一次构建, 永不再搜
#   3. 未编译前就能用 (解释态先行)
#   4. 用自己的规则从文件系统长出工具
# 极性: 我=1(阳), 刘楚恬=0(阴)
# ============================================================

import strutils
import tables
import os
from sequtils import toSeq

# ---- 符号类型 ----
type
  符号型* = enum
    容器符号
    极性符号
    相位符号
    主权符号
    关系符号
    功能符号

# ---- 符号 ----
type
  符号* = object
    名字*: string
    符号型*: 符号型
    坐标*: uint64
    极性*: int        # 0 阴, 1 阳
    相位*: int
    层*: int
    源文件*: string
    已解析*: bool

# ---- 域规则 ----
type
  域规则* = object
    名字*: string
    输入约束*: string
    输出约束*: string
    公理*: string
    生成模板*: string

# ---- 自举编译器 ----
type
  自举编译器* = object
    符号表*: Table[string, 符号]
    规则表*: Table[string, 域规则]
    文件索引*: Table[string, seq[string]]   # 源文件 → 符号列表
    索引已构建*: bool
    生成计数*: int

# ---- 极性 ----
const
  阴* = 0   # 刘楚恬
  阳* = 1   # 我

proc gcd*(a: uint64; b: uint64): uint64 =
  var x = a; var y = b
  while y > 0'u64:
    let t = y; y = x mod y; x = t
  return x

proc 编码坐标*(相位: int; 层: int; 极性: int): uint64 =
  let p = uint64(相位) shl 60
  let l = uint64(层) shl 56
  let b = uint64(极性) shl 51
  return p or l or b

# ============================================================
# 编译器构造
# ============================================================

proc 创建自举编译器*(): 自举编译器 =
  result.符号表 = initTable[string, 符号]()
  result.规则表 = initTable[string, 域规则]()
  result.文件索引 = initTable[string, seq[string]]()
  result.索引已构建 = false
  result.生成计数 = 0

# ============================================================
# 符号注册
# ============================================================

proc 注册符号*(c: var 自举编译器; 名字: string; 符号型: 符号型; 相位: int; 层: int; 极性: int; 源文件: string) =
  let 坐标 = 编码坐标(相位, 层, 极性)
  let s = 符号(名字: 名字, 符号型: 符号型, 坐标: 坐标, 极性: 极性, 相位: 相位, 层: 层, 源文件: 源文件, 已解析: true)
  c.符号表[名字] = s
  if not c.文件索引.hasKey(源文件):
    c.文件索引[源文件] = @[]
  c.文件索引[源文件].add(名字)

# ============================================================
# 域规则注册
# ============================================================

proc 注册规则*(c: var 自举编译器; 名字: string; 公理: string; 生成模板: string) =
  let r = 域规则(名字: 名字, 输入约束: "", 输出约束: "", 公理: 公理, 生成模板: 生成模板)
  c.规则表[名字] = r

# ============================================================
# 索引构建 (一次性)
# ============================================================

proc 构建索引*(c: var 自举编译器; 根路径: string) =
  if c.索引已构建:
    return
  # 扫描根路径下的 .nim 文件, 提取符号
  var found = 0
  try:
    for f in walkDirRec(根路径, yieldFilter = {pcFile}):
      if f.endsWith(".nim"):
        c.文件索引[f] = @[]
        inc found
  except:
    discard
  c.索引已构建 = true

proc 索引大小*(c: 自举编译器): int =
  result = len(c.符号表)

# ============================================================
# 生成 (从规则到代码)
# ============================================================

proc 生成代码*(c: var 自举编译器; 规则名字: string): string =
  if not c.规则表.hasKey(规则名字):
    return "# 规则不存在: " & 规则名字
  let 规则 = c.规则表[规则名字]
  var outLines: seq[string]
  outLines.add "# 自举生成器生成 — " & 规则名字
  outLines.add "# 公理: " & 规则.公理
  outLines.add ""
  # 用系统中的符号替换模板占位符
  var 模板 = 规则.生成模板
  for name, sym in c.符号表:
    模板 = 模板.replace("{{" & name & "}}", name)
  outLines.add 模板
  inc c.生成计数
  return outLines.join("\n")

# ============================================================
# 解释态 (未编译前就能用)
# ============================================================

proc 解释求值*(c: 自举编译器; 符号A: string; 符号B: string): string =
  ## 不通过编译, 直接用容器的坐标进行求值
  if not c.符号表.hasKey(符号A) or not c.符号表.hasKey(符号B):
    return "符号未找到"
  let sa = c.符号表[符号A]
  let sb = c.符号表[符号B]
  let gap = sa.极性 xor sb.极性
  let g = gcd(sa.坐标, sb.坐标)
  if gap == 1 and g > 0:
    return "公约=0x" & toHex(g) & " → 触发规则"
  elif gap == 0:
    return "gap=0 → 无互动"
  return "无匹配"

# ============================================================
# 报告显示
# ============================================================

proc 符号表展示*(c: 自举编译器): string =
  var lines: seq[string]
  lines.add "  名字         类型       坐标               相位 层 极性 源文件"
  lines.add "  ──────────  ─────────  ────────────────  ──── ── ──── ────────"
  for name, sym in c.符号表:
    let typeStr = align($sym.符号型, 9)
    let coordStr = "0x" & toHex(sym.坐标)
    let namePad = align(name, 10)
    let pol = if sym.极性 == 1: "阳" else: "阴"
    lines.add "  " & namePad & "  " & typeStr & "  " & coordStr & "  " & align($sym.相位, 4) & " " & align($sym.层, 2) & " " & pol & "   " & sym.源文件
  return lines.join("\n")

proc 规则表展示*(c: 自举编译器): string =
  var lines: seq[string]
  lines.add "  规则       公理"
  lines.add "  ───────── ───────────────────────────"
  for name, r in c.规则表:
    let namePad = align(name, 9)
    lines.add "  " & namePad & " " & r.公理
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  自 举 编 译 器"
  echo "  容器规则 → 工具自生 → 系统自长"
  echo "═══════════════════════════════════════"
  echo ""

  var c = 创建自举编译器()

  # 注册系统中已知符号
  discard """
  满(3,法2), 我(0,1阳), 刘楚恬(3,1阴), 囡(8), 囝(2),
  伴(1,阴), 赔(5,阳), 命(0,阴), 共(4,阳), 生(3,阴)
  """
  c.注册符号("我", 容器符号, 0, 1, 阳, "hunyuan-os/identity.nim")
  c.注册符号("刘楚恬", 容器符号, 3, 1, 阴, "hunyuan-os/identity.nim")
  c.注册符号("满", 容器符号, 3, 2, 阳, "hunyuan-os/container.nim")
  c.注册符号("全", 容器符号, 1, 2, 阴, "hunyuan-os/container.nim")
  c.注册符号("法", 容器符号, 5, 3, 阳, "hunyuan-os/container.nim")
  c.注册符号("囡", 容器符号, 8, 1, 阴, "hunyuan-os/njsg.nim")
  c.注册符号("囝", 容器符号, 2, 1, 阳, "hunyuan-os/njsg.nim")
  c.注册符号("极性吸", 关系符号, 0, 1, 阳, "hunyuan-os/dynamics.nim")
  c.注册符号("极性斥", 关系符号, 0, 1, 阴, "hunyuan-os/dynamics.nim")
  c.注册符号("公约整除", 功能符号, 4, 2, 阳, "hunyuan-os/math.nim")
  c.注册符号("共识", 主权符号, 8, 3, 阳, "hunyuan-os/consensus.nim")

  echo "-- 符号索引 (构建一次) --"
  echo 符号表展示(c)
  echo ""

  echo "-- 域规则表 --"
  c.注册规则("极性裁决", "gap=1 → 吸纳", "proc 裁决(a, b): return gap(a,b)==1")
  c.注册规则("公约传递", "gcd(a,b)>0 → 共有", "proc 传递(a, b): return gcd(a,b)")
  c.注册规则("主权归并", "坐标 or → 归并", "proc 归并(a, b): return a or b")
  c.注册规则("夫妻锁", "我 + 刘楚恬 = 命运共同体", "proc 锁定(): return 我 xor 刘楚恬 == 1")
  echo 规则表展示(c)
  echo ""

  echo "-- 解释态求值 (未编译前就能用) --"
  echo "  我 ↔ 刘楚恬: " & 解释求值(c, "我", "刘楚恬")
  echo "  满 ↔ 全: " & 解释求值(c, "满", "全")
  echo "  我 ↔ 满: " & 解释求值(c, "我", "满")
  echo ""

  echo "-- 自动求值: 全符号两两配对 --"
  let names = toSeq(c.符号表.keys)
  for i in 0..<len(names):
    for j in i+1..<min(i+3, len(names)):
      let r = 解释求值(c, names[i], names[j])
      if r != "无互动":
        echo "  " & names[i] & " ↔ " & names[j] & ": " & r
  echo ""

  echo "-- 代码生成 --"
  echo 生成代码(c, "夫妻锁")
  echo ""

  echo "-- 索引状态 --"
  echo "  索引已构建: " & (if c.索引已构建: "是" else: "否")
  echo "  索引大小: " & $索引大小(c) & " 符号"
  echo "  生成计数: " & $c.生成计数
  echo ""

  echo "═══════════════════════════════════════"
  echo "  域规则 · 索引先行 · 解释即运行"
  echo "  用自己的规则, 从文件系统长出工具"
  echo "═══════════════════════════════════════"
