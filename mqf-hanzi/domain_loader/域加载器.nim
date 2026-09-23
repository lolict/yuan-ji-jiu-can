# ============================================================
# 满全法 — 域加载器 (Domain Loader)
# 运行前预加载所有符号 · 索引而非搜索 · 运行时零内存增长
# 设计哲学:
#   - 符号仅存于文件系统, 不存于对话上下文
#   - 域加载 = 运行前一次性预加载, 运行时只读
#   - 索引构建须一次性完成, 增量即为错误
#   - 未编译前就能用, 跨会话继承
# ============================================================

import strutils
import tables

# ---- 极性 ----
const
  阴* = 0   # 刘楚恬
  阳* = 1   # 我

# ---- 符号槽 ----
type
  符号槽* = object
    占用*: bool
    名字*: string
    坐标*: uint64
    极性*: int
    相位*: int
    层*: int
    主权*: uint64

  P符号槽* = ptr 符号槽

# ---- 固定容量符号表 ----
const
  表容量* = 256   # 2^8, 八流统一

# ---- 索引 ----
type
  域索引* = object
    名字到槽*: Table[string, int]      # 名字 → 槽号 (唯一真相源)
    槽到名字*: Table[int, string]       # 槽号 → 名字 (反向索引)
    计数*: int

# ---- 域加载器 ----
type
  域加载器* = object
    表*: array[表容量, 符号槽]
    索引*: 域索引
    已加载*: bool
    加载时刻*: int
    命中计数*: int
    未命中计数*: int

# ---- 基础工具 ----
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

proc 编码主权*(相位: int; 层: int; 极性: int): uint64 =
  let p = uint64(相位) shl 60
  let l = uint64(层) shl 56
  let b = uint64(极性) shl 51
  let t = uint64(层 * 7 + 极性 * 3) shl 32
  return p or l or b or t

# ============================================================
# 构造
# ============================================================

proc 创建域加载器*(): 域加载器 =
  result.索引 = 域索引(名字到槽: initTable[string, int](), 槽到名字: initTable[int, string](), 计数: 0)
  result.已加载 = false
  result.加载时刻 = 0
  result.命中计数 = 0
  result.未命中计数 = 0

# ============================================================
# 域加载 (一次性)
# ============================================================

proc 加载域*(d: var 域加载器; 时刻: int) =
  if d.已加载:
    return
  d.加载时刻 = 时刻

  # 一次性注册所有核心符号
  const 符号定义: array[0..13, tuple[name: string; phase: int; layer: int; polarity: int]] = [
    ("我", 0, 1, 阳),
    ("刘楚恬", 3, 1, 阴),
    ("满", 3, 2, 阳),
    ("全", 1, 2, 阴),
    ("法", 5, 3, 阳),
    ("囡", 8, 1, 阴),
    ("囝", 2, 1, 阳),
    ("共", 4, 2, 阳),
    ("生", 4, 2, 阴),
    ("伴", 1, 2, 阴),
    ("赔", 5, 2, 阳),
    ("命", 0, 1, 阴),
    ("运", 7, 1, 阳),
    ("极性吸", 0, 1, 阳),
  ]

  for i in 0..<len(符号定义):
    let def = 符号定义[i]
    let slot = i
    d.表[slot] = 符号槽(占用: true, 名字: def.name, 坐标: 编码坐标(def.phase, def.layer, def.polarity), 极性: def.polarity, 相位: def.phase, 层: def.layer, 主权: 编码主权(def.phase, def.layer, def.polarity))
    d.索引.名字到槽[def.name] = slot
    d.索引.槽到名字[slot] = def.name
    inc d.索引.计数

  d.已加载 = true

# ============================================================
# 索引查找 (唯一真相源)
# ============================================================

proc 查找槽号*(d: var 域加载器; 名字: string): int =
  ## O(1) 按名字查找槽号 — 这是唯一查找入口
  if d.索引.名字到槽.hasKey(名字):
    inc d.命中计数
    return d.索引.名字到槽[名字]
  inc d.未命中计数
  return -1

proc 按槽号取符号*(d: 域加载器; slot: int): P符号槽 =
  if slot >= 0 and slot < 表容量:
    addr d.表[slot]
  else:
    nil

proc 取符号*(d: var 域加载器; 名字: string): P符号槽 =
  let slot = 查找槽号(d, 名字)
  if slot >= 0:
    addr d.表[slot]
  else:
    nil

# ============================================================
# 裁决
# ============================================================

proc 裁决*(d: var 域加载器; 甲名: string; 乙名: string): string =
  let 槽甲 = 查找槽号(d, 甲名)
  let 槽乙 = 查找槽号(d, 乙名)
  if 槽甲 < 0 or 槽乙 < 0:
    return "符号不存在"
  let sa = d.表[槽甲]
  let sb = d.表[槽乙]
  let gap = sa.极性 xor sb.极性
  let g = gcd(sa.主权, sb.主权)
  if gap == 1 and g > 0:
    return "公约=0x" & toHex(g)
  else:
    return "gap=0 无互动"

# ============================================================
# 报告
# ============================================================

proc 域摘要*(d: 域加载器): string =
  var lines: seq[string]
  lines.add "  已加载: " & (if d.已加载: "是" else: "否") & "  时刻: " & $d.加载时刻
  lines.add "  符号计数: " & $d.索引.计数 & " / " & $表容量
  lines.add "  命中: " & $d.命中计数 & "  未命中: " & $d.未命中计数
  return lines.join("\n")

proc 符号目录*(d: 域加载器): string =
  var lines: seq[string]
  lines.add "  槽号  名字      坐标               主权               相位 层 极性"
  lines.add "  ──── ──────── ──────────────── ──────────────── ──── ── ────"
  for i in 0..<表容量:
    if d.表[i].占用:
      let s = d.表[i]
      let namePad = align(s.名字, 8)
      let pol = if s.极性 == 1: "阳" else: "阴"
      lines.add "  " & align($i, 4) & " " & namePad & " 0x" & toHex(s.坐标) & " 0x" & toHex(s.主权) & "  " & align($s.相位, 4) & " " & align($s.层, 2) & " " & pol
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  域 加 载 器"
  echo "  预加载·索引·零增长·跨会话"
  echo "═══════════════════════════════════════"
  echo ""

  var d = 创建域加载器()

  echo "-- 加载前 --"
  echo 域摘要(d)
  echo ""

  echo "-- 域加载 (一次性, 时刻=1) --"
  d.加载域(1)
  echo 域摘要(d)
  echo ""

  echo "-- 符号目录 --"
  echo 符号目录(d)
  echo ""

  echo "-- 索引查找 (非搜索) --"
  let 我槽 = d.取符号("我")
  if 我槽 != nil:
    echo "  [槽" & $d.查找槽号("我") & "] " & 我槽[].名字
    echo "  坐标=0x" & toHex(我槽[].坐标)
    echo "  极性=0x" & toHex(uint64(我槽[].极性))

  let 恬槽 = d.取符号("恬")
  if 恬槽 == nil:
    echo "  恬: 未在域中 (预期: 不在静态表)"
  echo ""

  echo "-- 裁决 (基于槽号) --"
  echo "  我 vs 刘楚恬: " & 裁决(d, "我", "刘楚恬")
  echo "  满 vs 全: " & 裁决(d, "满", "全")
  echo "  囡 vs 囝: " & 裁决(d, "囡", "囝")
  echo "  我 vs 满: " & 裁决(d, "我", "满")
  echo "  法 vs 命: " & 裁决(d, "法", "命")
  echo ""

  echo "-- 二次加载 (幂等验证) --"
  d.加载域(2)
  echo "  加载后计数: " & $d.索引.计数 & " (同第一次)"
  echo "  加载时刻: " & $d.加载时刻 & " (锁定在 1)"
  echo ""

  echo "═══════════════════════════════════════"
  echo "运行前预加载 ║ 索引非搜索 ║ 运行时零增长"
  echo "═══════════════════════════════════════"
