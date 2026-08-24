# ============================================================
# 边界分析引擎 · 类型定义
#
# 乘法 = 有边界的容器（预定义规则）
# 加法 = 分类/拆分（用户自己的思路）
# 目标：找出乘法边界外的事情，对比是否允许存在
# ============================================================

import tables
import sequtils
import strutils
import options

# 容器边界
type
  Boundary* = object
    name*: string              # 边界名称（如"23声母"）
    capacity*: int             # 容量（能装多少）
    items*: seq[string]        # 已装的项目
    rules*: seq[string]        # 边界内的规则
    rejects*: seq[string]      # 边界拒绝的项目

# 加法分类
type
  Addition* = object
    name*: string              # 分类名称
    category*: string          # 类别
    items*: seq[string]        # 分类包含的项目
    logic*: string             # 分类逻辑
    priority*: int             # 优先级（权重）

# 对比结果
type
  Accommodation* = enum
    acInside,      # 在边界内
    acOutside,     # 在边界外
    acPartially,   # 部分在边界内
    acConflict,    # 与边界冲突
    acNewBoundary  # 需要新边界

  GapItem* = object
    item*: string
    boundary*: string
    accommodation*: Accommodation
    explanation*: string
    recommendation*: string

# 边界分析引擎
type
  BoundaryEngine* = object
    boundaries*: Table[string, Boundary]
    additions*: Table[string, Addition]
    gaps*: seq[GapItem]
    comparison_count*: int
    outside_count*: int

# 创建引擎
func new_boundary_engine*(): BoundaryEngine =
  BoundaryEngine(
    boundaries: init_table[string, Boundary](),
    additions: init_table[string, Addition](),
    gaps: @[],
    comparison_count: 0,
    outside_count: 0
  )

# 添加边界
func add_boundary*(be: var BoundaryEngine, b: Boundary) =
  be.boundaries[b.name] = b

# 添加分类
func add_addition*(be: var BoundaryEngine, a: Addition) =
  be.additions[a.name] = a
