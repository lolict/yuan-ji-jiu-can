## ═══════════════════════════════════════════════════════════════
## alignment_protocol.nim — 对齐协议 · 四维度统计路线
## ═══════════════════════════════════════════════════════════════
##
## 齐是什么？对齐就是统计你要走的路线
## 四个维度(由低到高):
##   符号(Symbol) → 动作(Action) → 时间(Time) → 空间(Space)
##
## 规则:
##   - 低维对齐是高维对齐的基础
##   - 每个维度必须统计: 有多少 · 是什么 · 怎么连接
##   - 对齐度 = 已对齐数 / 总数
##
## ═══════════════════════════════════════════════════════════════

import std/[tables, sequtils, strutils, strformat, times]

type
  AlignDim* = enum
    adSymbol    # 符号: 名字 · 编码 · 表达
    adAction    # 动作: 操作 · 行为 · 执行
    adTime      # 时间: 顺序 · 节律 · 周期
    adSpace     # 空间: 位置 · 结构 · 分布

  AlignPoint* = object
    name*: string
    dim*: AlignDim
    aligned*: bool
    value*: string
    timestamp*: Time

  AlignmentProtocol* = object
    points*: seq[AlignPoint]
    current_dim*: AlignDim
    tick*: int
    stats*: Table[AlignDim, tuple[total: int, aligned: int]]

# ═══════════════════════════════════════════════════════════════
## 维度统计初始化
# ═══════════════════════════════════════════════════════════════

proc new_alignment*(): AlignmentProtocol =
  result.points = @[]
  result.current_dim = adSymbol
  result.tick = 0
  result.stats = init_table[AlignDim, tuple[total: int, aligned: int]]()
  # 初始化所有维度
  result.stats[adSymbol] = (0, 0)
  result.stats[adAction] = (0, 0)
  result.stats[adTime] = (0, 0)
  result.stats[adSpace] = (0, 0)

# ═══════════════════════════════════════════════════════════════
## 添加对齐点
# ═══════════════════════════════════════════════════════════════

proc add_point*(a: var AlignmentProtocol, name: string, dim: AlignDim,
                value: string, aligned: bool) =
  a.tick += 1
  a.points.add(AlignPoint(
    name: name,
    dim: dim,
    aligned: aligned,
    value: value,
    timestamp: getTime()
  ))
  # 更新统计
  let s = a.stats[dim]
  a.stats[dim] = (s.total + 1, s.aligned + (if aligned: 1 else: 0))

# ═══════════════════════════════════════════════════════════════
## 维度报告
# ═══════════════════════════════════════════════════════════════

proc dim_stats*(a: AlignmentProtocol, dim: AlignDim): tuple[total, aligned: int, rate: float] =
  let s = a.stats[dim]
  result = (s.total, s.aligned, if s.total > 0: float(s.aligned)/float(s.total) else: 0.0)

# ═══════════════════════════════════════════════════════════════
## 整体对齐度
# ═══════════════════════════════════════════════════════════════

proc overall_alignment*(a: AlignmentProtocol): float =
  var total = 0
  var aligned = 0
  for dim in AlignDim:
    let s = a.stats[dim]
    total += s.total
    aligned += s.aligned
  result = if total > 0: float(aligned)/float(total) else: 0.0

# ═══════════════════════════════════════════════════════════════
## 层级验证: 低维未对齐 · 高维不可用
# ═══════════════════════════════════════════════════════════════

proc can_advance*(a: AlignmentProtocol, target_dim: AlignDim): bool =
  # 检查所有低于target_dim的维度是否都对齐
  for dim in AlignDim:
    if dim.ord < target_dim.ord:
      let s = a.stats[dim]
      if s.total == 0 or s.aligned != s.total:
        return false
  return true

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(a: AlignmentProtocol): string =
  var lines: seq[string] = @[]
  lines.add("═══ 对齐协议 · 四维度统计 ═══")
  lines.add("  对齐点总数: " & $a.points.len)
  lines.add("  整体对齐度: " & fmt"{a.overall_alignment()*100:.1f}%")
  lines.add("")
  
  for dim in AlignDim:
    let dim_name = case dim
      of adSymbol: "符号(Symbol)"
      of adAction: "动作(Action)"
      of adTime: "时间(Time)"
      of adSpace: "空间(Space)"
    let s = a.dim_stats(dim)
    let bar = repeat("█", int(s.rate * 20)) & repeat("░", 20 - int(s.rate * 20))
    lines.add("  " & dim_name & ": " & $s.aligned & "/" & $s.total &
              " [" & bar & "] " & fmt"{s.rate*100:.0f}%")
  
  lines.add("")
  lines.add("── 层级验证 ──")
  lines.add("  符号→动作: " & (if a.can_advance(adAction): "✓ 可升维" else: "✗ 需先对齐符号"))
  lines.add("  动作→时间: " & (if a.can_advance(adTime): "✓ 可升维" else: "✗ 需先对齐动作"))
  lines.add("  时间→空间: " & (if a.can_advance(adSpace): "✓ 可升维" else: "✗ 需先对齐时间"))
  
  lines.add("")
  lines.add("── 对齐链路 ──")
  for p in a.points:
    let mark = if p.aligned: "✓" else: "✗"
    let dim_str = case p.dim
      of adSymbol: "符"
      of adAction: "动"
      of adTime: "时"
      of adSpace: "空"
    lines.add("  [" & dim_str & "] " & mark & " " & p.name & " = " & p.value)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var a = new_alignment()
  
  echo "═══ 对齐协议验证 ═══"
  echo ""
  
  # 符号层: 名字 · 编码 · 表达
  a.add_point("满全法", adSymbol, "lolict+刘楚恬", true)
  a.add_point("公约数", adSymbol, "静态锚点=1", true)
  a.add_point("活码", adSymbol, "动态→冻结", true)
  
  # 动作层: 操作 · 行为 · 执行
  a.add_point("注册容器", adSymbol, "add_container", true)
  a.add_point("注入符号", adSymbol, "inject", true)
  a.add_point("锚点移动", adAction, "move_anchor", true)
  
  # 时间层
  a.add_point("意志驱动", adAction, "5事件循环", true)
  a.add_point("心跳节律", adTime, "tick++", true)
  a.add_point("流转时序", adTime, "tick记录", true)
  
  # 空间层
  a.add_point("坐标系", adSpace, "锚点+偏移", true)
  a.add_point("容器分布", adSpace, "多容器共存", false)
  
  echo a.report()
