# ============================================================
# 边界分析引擎 · 核心逻辑
#
# 用加法拆分乘法边界做不到的事情
# 对比两个规则体系，判断容器是否允许存在
# ============================================================

import tables
import sequtils
import strutils
import options
import types

# 单个项目对比
func compare_item*(be: var BoundaryEngine, item: string, boundary_name: string): GapItem =
  inc be.comparison_count
  
  if not be.boundaries.hasKey(boundary_name):
    return GapItem(
      item: item,
      boundary: boundary_name,
      accommodation: acOutside,
      explanation: "边界不存在",
      recommendation: "需要定义新边界"
    )
  
  let boundary = be.boundaries[boundary_name]
  
  # 检查是否在边界内
  if boundary.items.contains(item):
    return GapItem(
      item: item,
      boundary: boundary_name,
      accommodation: acInside,
      explanation: "在乘法边界内",
      recommendation: "被规则覆盖"
    )
  
  # 检查是否被明确拒绝
  if boundary.rejects.contains(item):
    inc be.outside_count
    return GapItem(
      item: item,
      boundary: boundary_name,
      accommodation: acConflict,
      explanation: "被乘法边界明确拒绝",
      recommendation: "需要新边界来容纳"
    )
  
  # 部分匹配
  for i in boundary.items:
    if i.contains(item) or item.contains(i):
      return GapItem(
        item: item,
        boundary: boundary_name,
        accommodation: acPartially,
        explanation: "部分匹配乘法边界",
        recommendation: "补充加法细节"
      )
  
  # 完全在边界外
  inc be.outside_count
  return GapItem(
    item: item,
    boundary: boundary_name,
    accommodation: acOutside,
    explanation: "在乘法边界外 - 这是加法要做的事情",
    recommendation: "定义新规则容纳此项"
  )

# 批量对比：加法分类 vs 乘法边界
func compare_addition*(be: var BoundaryEngine, addition_name: string, boundary_name: string): seq[GapItem] =
  if not be.additions.hasKey(addition_name):
    return @[]
  
  let addition = be.additions[addition_name]
  var results: seq[GapItem] = @[]
  
  for item in addition.items:
    let gap = be.compare_item(item, boundary_name)
    results.add(gap)
  
  return results

# 寻找边界外的事情（乘法做不到的）
func find_gaps*(be: var BoundaryEngine): seq[GapItem] =
  var results: seq[GapItem] = @[]
  
  for add_name, addition in be.additions:
    for b_name, boundary in be.boundaries:
      for item in addition.items:
        let gap = be.compare_item(item, b_name)
        if gap.accommodation == acOutside or gap.accommodation == acConflict:
          results.add(gap)
  
  be.gaps = results
  return results

# 判断是否需要新边界
func needs_new_boundary*(be: BoundaryEngine): bool =
  return be.outside_count > 0

# 计算边界填充率
func fill_rate*(be: BoundaryEngine, addition_name: string, boundary_name: string): float =
  if not be.additions.hasKey(addition_name) or not be.boundaries.hasKey(boundary_name):
    return 0.0
  
  let addition = be.additions[addition_name]
  let boundary = be.boundaries[boundary_name]
  
  if boundary.capacity == 0:
    return 0.0
  
  var inside = 0
  for item in addition.items:
    if boundary.items.contains(item):
      inc inside
  
  return float(inside) / float(boundary.capacity)

# 判定分析结果
func accommodation_desc*(ac: Accommodation): string =
  case ac
  of acInside: "✅ 在乘法边界内（被覆盖）"
  of acOutside: "⚠️ 在乘法边界外（需加法补充）"
  of acPartially: "🔶 部分匹配（需细化）"
  of acConflict: "❌ 与乘法冲突（需新边界）"
  of acNewBoundary: "🆕 完全需要新边界"

# 生成报告
func gap_report*(be: var BoundaryEngine, addition_name: string, boundary_name: string): string =
  var s = "═══ 加法 vs 乘法 边界分析报告 ═══\n"
  s &= "加法分类: " & addition_name & "\n"
  s &= "乘法边界: " & boundary_name & "\n"
  s &= "════════════════════════════════\n"
  
  if not be.additions.hasKey(addition_name):
    return s & "加法分类不存在\n"
  
  if not be.boundaries.hasKey(boundary_name):
    return s & "乘法边界不存在\n"
  
  let addition = be.additions[addition_name]
  let boundary = be.boundaries[boundary_name]
  let results = be.compare_addition(addition_name, boundary_name)
  
  # 统计
  var inside = 0
  var outside = 0
  var partial = 0
  var conflict = 0
  
  for r in results:
    case r.accommodation
    of acInside: inc inside
    of acOutside: inc outside
    of acPartially: inc partial
    of acConflict: inc conflict
    of acNewBoundary: inc outside
  
  s &= "总项目: " & $results.len & "\n"
  s &= "边界内: " & $inside & "\n"
  s &= "边界外: " & $outside & "\n"
  s &= "部分匹配: " & $partial & "\n"
  s &= "冲突: " & $conflict & "\n"
  s &= "────────────────────────────────\n"
  s &= "边界容量: " & $boundary.capacity & "\n"
  s &= "已填充: " & $boundary.items.len & "\n"
  s &= "填充率: " & formatFloat(fill_rate(be, addition_name, boundary_name) * 100.0, ffDecimal, 1) & "%\n"
  s &= "════════════════════════════════\n"
  s &= "判定: "
  
  if conflict > 0:
    s &= "乘法拒绝加法 - 需要新边界\n"
  elif outside > 0:
    s &= "乘法不完全覆盖加法 - 需要加法补充\n"
  elif partial > 0:
    s &= "乘法部分覆盖 - 需要细化\n"
  else:
    s &= "乘法完全覆盖加法 - 容器允许存在\n"
  
  result = s

# 详细gap列表
func gap_details*(be: var BoundaryEngine, results: seq[GapItem]): string =
  var s = "═══ 边界外项目（乘法做不到的）═══\n"
  
  for r in results:
    if r.accommodation == acOutside or r.accommodation == acConflict:
      s &= "  [" & accommodation_desc(r.accommodation) & "] " & r.item & "\n"
      s &= "    " & r.explanation & "\n"
      s &= "    → " & r.recommendation & "\n"
  
  result = s
