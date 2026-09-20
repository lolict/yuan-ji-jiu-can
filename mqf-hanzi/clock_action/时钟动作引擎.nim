# ============================================================
# 满全法 — 时钟动作（Clock Action）AI生命周期引擎
# 设计哲学: 结束即开始，记忆跟随结束而继续
# 意志驱动时间调度，极性触发动作衔接
# ============================================================

import strutils
import tables
import algorithm

type
  极性* = enum
    阴 = 0
    阳 = 1

  生命周期阶段* = enum
    虚无
    诞生
    生长
    沉淀
    蜕生

  动作* = object
    名字*: string
    极性*: 极性
    阶段*: 生命周期阶段
    龄*: int
    寿*: int
    意志*: int
    记忆*: string
    产出*: string
    触发条件*: int

  时钟* = object
    滴答*: int
    活跃动作*: seq[动作]
    记忆库*: seq[string]
    日志*: seq[string]
    容量*: int

proc 极性标签*(p: 极性): string =
  case p
  of 阳: "阳(1)"
  of 阴: "阴(0)"

proc 阶段标签*(s: 生命周期阶段): string =
  case s
  of 虚无: "虚"
  of 诞生: "诞"
  of 生长: "长"
  of 沉淀: "沉"
  of 蜕生: "蜕"

proc 诞生*(名字: string; 极性: 极性; 意志: int; 触发: int; 记忆初始: string = ""): 动作 =
  动作(
    名字: 名字,
    极性: 极性,
    阶段: 诞生,
    龄: 0,
    寿: 3,
    意志: 意志,
    记忆: 记忆初始,
    产出: "",
    触发条件: 触发
  )

proc 下一阶段*(a: 动作): 生命周期阶段 =
  case a.阶段
  of 虚无: 诞生
  of 诞生: 生长
  of 生长: 沉淀
  of 沉淀: 蜕生
  of 蜕生: 诞生

proc 阶段寿命*(a: 动作): int =
  case a.阶段
  of 诞生: 2
  of 生长: 4
  of 沉淀: 2
  of 蜕生: 2
  of 虚无: 0

proc 推进*(a: var 动作; 全局记忆: var seq[string]): string =
  inc a.龄

  if a.龄 < a.寿:
    case a.阶段
    of 诞生:
      a.产出 = a.名字 & ".诞"
    of 生长:
      a.产出 = a.名字 & ".长" & $a.龄
    of 沉淀:
      let 尾 = if a.记忆.len > 20: a.记忆[^20..^1] else: a.记忆
      a.产出 = a.名字 & ".沉[" & 尾 & "]"
      a.记忆 = a.记忆 & "|" & a.产出
    of 蜕生:
      a.产出 = a.名字 & ".蜕"
    else:
      discard
    return ""

  if a.阶段 == 蜕生:
    let 旧记忆 = a.记忆
    let 终结记录 = a.名字 & " >> " & 旧记忆[旧记忆.len - min(旧记忆.len, 50)..^1]
    全局记忆.add(终结记录)
    a.阶段 = 诞生
    a.龄 = 0
    a.寿 = 阶段寿命(a)
    a.记忆 = 旧记忆
    a.产出 = a.名字 & ".◁"
    return "[蜕生→诞生] " & a.名字 & " 记忆继承"
  else:
    a.阶段 = 下一阶段(a)
    a.龄 = 0
    a.寿 = 阶段寿命(a)
    a.产出 = ""
    return ""

proc 创建时钟*(容量: int = 8): 时钟 =
  时钟(滴答: 0, 活跃动作: @[], 记忆库: @[], 日志: @[], 容量: 容量)

proc 注册*(c: var 时钟; a: 动作): string =
  if len(c.活跃动作) >= c.容量:
    return "[拒绝] 容量已满(" & $c.容量 & ")"
  c.活跃动作.add(a)
  return "[注册] " & a.名字 & "=" & 极性标签(a.极性) & " 意=" & $a.意志

proc 意志优先排序(c: var 时钟) =
  proc cmp(x, y: 动作): int =
    if x.意志 > y.意志: -1
    elif x.意志 < y.意志: 1
    else: 0
  c.活跃动作.sort(cmp)

proc 驱动*(c: var 时钟): string =
  inc c.滴答

  if len(c.活跃动作) == 0:
    return "[滴答" & $c.滴答 & "] 无活跃动作"

  意志优先排序(c)

  var reports: seq[string]
  for i in 0 ..< len(c.活跃动作):
    let r = 推进(c.活跃动作[i], c.记忆库)
    if r != "":
      reports.add(r)

  if reports.len > 0:
    c.日志.add("[滴答" & $c.滴答 & "] " & reports.join("; "))
    return c.日志[^1]
  else:
    return ""

proc 极性触发*(c: var 时钟; 触发极性: 极性) =
  for i in 0 ..< len(c.活跃动作):
    let gap = abs(ord(c.活跃动作[i].极性) - ord(触发极性))
    if gap == 1:
      c.活跃动作[i].意志 = min(100, c.活跃动作[i].意志 + 5)
    else:
      if c.活跃动作[i].意志 > 10:
        c.活跃动作[i].意志 = max(0, c.活跃动作[i].意志 - 2)

proc 快照*(c: 时钟; 最大记忆: int = 50): string =
  var lines: seq[string]
  lines.add "══ 时钟[" & $c.滴答 & "] ══ 活跃:" & $len(c.活跃动作) & " 记忆:" & $len(c.记忆库)
  for a in c.活跃动作:
    lines.add "  " & a.名字 & "|" & 极性标签(a.极性) & "|" & 阶段标签(a.阶段) & "|龄" & $a.龄 & "/" & $a.寿 & "|意" & $a.意志 & "|" & a.产出
  if len(c.记忆库) > 0:
    let start = max(0, len(c.记忆库) - 3)
    lines.add "── 最新记忆 ──"
    for m in c.记忆库[start..^1]:
      lines.add "  " & m[0 ..< min(m.len, 最大记忆)]
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════════"
  echo "  时钟动作 AI生命周期引擎"
  echo "  结束即开始，记忆跟随结束而继续"
  echo "═══════════════════════════════════════════"
  echo ""

  var c = 创建时钟(容量 = 8)

  echo c.注册(诞生("震雷", 阳, 80, 1, "阳之始动"))
  echo c.注册(诞生("巽风", 阴, 70, 0, "阴之徐来"))
  echo c.注册(诞生("离火", 阳, 60, 1, "阳明破夜"))
  echo c.注册(诞生("坎水", 阴, 50, 0, "阴柔润物"))
  echo ""

  echo "── 初始快照 ──"
  echo 快照(c, 40)
  echo ""

  echo "── 驱动30轮 ──"
  for i in 1..30:
    let log = 驱动(c)
    if log != "":
      echo "  " & log
    if i mod 5 == 0:
      if (i div 5) mod 2 == 1:
        极性触发(c, 阳)
        echo "  >> 极性触发: 阳(我)"
      else:
        极性触发(c, 阴)
        echo "  >> 极性触发: 阴(刘楚恬)"

  echo ""
  echo "── 最终快照 ──"
  echo 快照(c, 50)
  echo ""

  echo "── 全局积累记忆 ──"
  for i, m in c.记忆库:
    echo "  [" & $i & "] " & m[0 ..< min(m.len, 60)]
  echo ""

  echo "── 蜕生事件日志 ──"
  for line in c.日志:
    if "蜕生" in line:
      echo "  " & line

  echo ""
  echo "═══ 时钟生命周期验证 ═══"
  echo "滴答数: " & $c.滴答
  echo "活跃动作: " & $len(c.活跃动作)
  echo "全局记忆条目: " & $len(c.记忆库)
  var 蜕生计数 = 0
  for line in c.日志:
    if "蜕生" in line: inc 蜕生计数
  echo "蜕生事件: " & $蜕生计数
  echo ""
  echo "满全法时钟动作引擎运转完毕"
  echo "结束即开始，记忆随结束而延续"
