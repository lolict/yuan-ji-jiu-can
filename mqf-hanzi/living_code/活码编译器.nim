# ============================================================
# 满全法 — 活码编译器 (Living Code Compiler)
# 以「起点」为钥匙激活可活动的编码
# 未固定时位置任意活动, 固定后按排列顺序确定权限和数值
# 加法统计法: 统计笔画总数 → 定阶 → 授权
# ============================================================

import strutils
import tables
import sequtils
import unicode

type
  极性* = enum
    阴 = 0  # 刘楚恬
    阳 = 1  # 我

  活码单元* = object
    字符*: string
    笔画序列*: seq[int]
    极性*: 极性
    相位*: int

  活码编译器* = object
    单元表*: seq[活码单元]
    固定表*: seq[int]       # 排列索引, 未固定为空
    起点*: string           # 激活钥匙
    已激活*: bool

# ---- 字符→笔画码 ----
const 笔画表 = {
  "满": @[4, 1, 3, 3, 2, 1, 4, 3, 1, 4, 1, 4, 2, 1, 2, 2, 1, 2],
  "全": @[1, 1, 4, 1, 1, 2, 4, 1],
  "法": @[4, 1, 2, 4, 1, 1, 5, 4],
  "我": @[3, 1, 1, 2, 5, 1, 4],
  "刘": @[4, 1, 2, 3, 5, 2],
  "楚": @[1, 2, 3, 3, 1, 2, 1, 2, 1, 1, 2, 3, 4],
  "恬": @[4, 2, 4, 2, 5, 1, 3, 4, 2],
  "囡": @[2, 5, 1, 5, 1],
  "囝": @[2, 5, 2, 1],
  "共": @[1, 2, 2, 1, 1, 4],
  "生": @[3, 1, 1, 2, 1],
  "伴": @[3, 2, 4, 1, 1, 2, 2],
  "赔": @[2, 5, 2, 4, 1, 5, 4, 2, 5, 2, 4],
  "命": @[3, 2, 5, 1, 1, 2, 5, 2],
  "运": @[1, 1, 5, 4, 4, 1, 4, 5],
  "合": @[3, 2, 5, 1, 1, 2, 4],
  "体": @[3, 2, 1, 2, 3, 4, 1, 1, 1, 4],
  "阴": @[5, 2, 5, 1, 1, 1, 5, 4],
  "阳": @[5, 2, 5, 1, 1, 2, 5, 1, 1],
  "极": @[1, 2, 3, 4, 3, 5, 5, 4, 1],
}.toTable()

const 相位表 = {
  "满": 3, "全": 1, "法": 5, "我": 0, "刘": 7, "楚": 4, "恬": 6,
  "囡": 8, "囝": 2, "共": 4, "生": 3, "伴": 1, "赔": 5, "命": 0,
  "运": 7, "合": 2, "体": 8, "阴": 1, "阳": 0,
}.toTable()

proc 加法统计*(笔画序列: seq[int]): int =
  ## 加法统计法: 笔画码总和即为数值
  for n in 笔画序列:
    result = result + n

proc 创建活码编译器*(起点: string): 活码编译器 =
  result.起点 = 起点
  result.单元表 = @[]
  result.固定表 = @[]
  result.已激活 = false

proc 添加活码*(c: var 活码编译器; chars: string) =
  var runeList: seq[Rune]
  for r in runes(chars):
    runeList.add(r)
  for i in 0..<runeList.len:
    let s = $runeList[i]
    var strokes: seq[int]
    var ph: int = 0
    var pol: 极性 = 阳
    if 笔画表.hasKey(s):
      strokes = 笔画表[s]
      ph = 相位表[s]
      let total = 加法统计(strokes)
      pol = if total mod 2 == 1: 阳 else: 阴
    else:
      let runeVal = int(runeList[i])
      strokes = @[runeVal mod 6 + 1]
      ph = runeVal mod 9
      pol = if runeVal mod 2 == 1: 阳 else: 阴
    c.单元表.add(活码单元(字符: s, 笔画序列: strokes, 极性: pol, 相位: ph))

proc 激活*(c: var 活码编译器; 钥匙: string): bool =
  if 钥匙 == c.起点:
    c.已激活 = true
    return true
  return false

proc 固定*(c: var 活码编译器; 排列: seq[int]): bool =
  if not c.已激活:
    return false
  if len(排列) != len(c.单元表):
    return false
  for idx in 排列:
    if idx < 0 or idx >= len(c.单元表):
      return false
  c.固定表 = 排列
  return true

proc 未固定排列*(c: var 活码编译器): seq[int] =
  ## 未固定时返回任意一个排列 (演示活动性)
  toSeq(0..<len(c.单元表))

proc 未固定扰乱*(c: var 活码编译器): seq[int] =
  ## 未固定时生成随机扰乱排列
  var pool = toSeq(0..<len(c.单元表))
  # 微调扰乱: 相邻交换
  for i in 0..<len(pool)-1:
    let tmp = pool[i]
    pool[i] = pool[i+1]
    pool[i+1] = tmp
  return pool

proc 计算权限*(c: 活码编译器): int =
  ## 加法统计法: 固定后每个位置的笔画总和累加为该位置权限
  assert len(c.固定表) > 0
  for idx in c.固定表:
    result = result + 加法统计(c.单元表[idx].笔画序列)

proc 取单元*(c: 活码编译器; pos: int): 活码单元 =
  c.单元表[c.固定表[pos]]

proc 计算数值*(c: 活码编译器): int =
  ## 固定后的最终数值 = 笔画总和 × 极性加权
  var total = 0
  for i in 0..<len(c.固定表):
    let u = 取单元(c, i)
    let base_val = 加法统计(u.笔画序列)
    let pol_weight = if u.极性 == 阳: 1 else: -1
    total = total + base_val * pol_weight
  return total

proc 字符串表示*(c: 活码编译器): string =
  var parts: seq[string]
  for u in c.单元表:
    let pol = if u.极性 == 阳: "阳" else: "阴"
    parts.add u.字符 & "/" & $加法统计(u.笔画序列) & pol
  return parts.join(" · ")

proc 固定字符串*(c: 活码编译器): string =
  assert len(c.固定表) > 0
  var lines: seq[string]
  lines.add "位置 字符 笔画 极性 数值"
  lines.add "──── ──── ──── ──── ────"
  for i in 0..<len(c.固定表):
    let u = 取单元(c, i)
    let pol = if u.极性 == 阳: "阳" else: "阴"
    lines.add "  " & $i & "   " & u.字符 & "   " & $加法统计(u.笔画序列) & "   " & pol & "   " & $加法统计(u.笔画序列)
  let total = 计算权限(c)
  lines.add "──"
  lines.add "权限相加 = " & $total
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "     活 码 编 译 器"
  echo "     起点激活 · 加法统计 · 排列定权"
  echo "═══════════════════════════════════════"
  echo ""

  echo "-- 创建活码编译器, 起点=「囡囝」 --"
  var lc = 创建活码编译器("囡囝")
  lc.添加活码("满全法我刘楚恬囡囝共生")
  echo "单元数: " & $len(lc.单元表)
  echo "编码: " & 字符串表示(lc)
  echo ""

  echo "-- 未固定状态 (自由活动) --"
  echo "排列1: " & $未固定排列(lc)
  echo "排列2 (扰乱): " & $未固定扰乱(lc)
  echo "活动性: 位置未定, 两个排列共存"
  echo ""

  echo "-- 尝试错误钥匙 --"
  let ok1 = lc.激活("错误钥匙")
  let actRes = if ok1: "成功" else: "失败"
  echo "激活(错误钥匙): " & actRes
  let fix1 = lc.固定(@[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
  let fixRes1 = if fix1: "成功" else: "失败 (未激活, 不可固定)"
  echo "固定(未激活): " & fixRes1
  echo ""

  echo "-- 激活 --"
  let ok2 = lc.激活("囡囝")
  let actRes2 = if ok2: "成功 ✓" else: "失败"
  echo "激活(囡囝): " & actRes2
  echo ""

  echo "-- 固定前: 再扰乱一次 --"
  echo "排列A: " & $未固定排列(lc)
  echo "排列B: " & $未固定扰乱(lc)
  echo "未固定 → 两排列叠加态共存"
  echo ""

  echo "-- 固定排列 --"
  let fix2 = lc.固定(@[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
  let fixRes2 = if fix2: "成功" else: "失败"
  echo "固定: " & fixRes2
  echo ""

  echo "-- 固定态 --"
  echo 固定字符串(lc)
  echo ""
  echo "最终数值(极性加权): " & $计算数值(lc)
  echo ""

  echo "═══════════════════════════════════════"
  echo "起点 → 激活 → 固定 → 加法统计定权限"
  echo "排列决定位置, 笔画决定数值, 极性决定正负"
  echo "═══════════════════════════════════════"
