# ============================================================
# 颜色进制 · 相位编码
# 每个单元格存储 64 进制相位值 (0-63)
# 运算仅允许加法(安装)和减法(拆解)，禁用乘除
# ============================================================

import strutils

type
  PhaseLevel* = enum
    plL1,   # 基元本体 (base element)
    plL2,   # 修饰部件 (modifier component)
    plL3    # 装配成品 (assembled finished product)

  PhaseCell* = object
    phase*: int           # 相位编码 0-63 (64进制)
    weight*: float        # 视觉权重 0.0-1.0 (胖瘦)
    level*: PhaseLevel    # 层级
    owner*: string        # 归属权 SELF/EXTERNAL
    parent*: seq[string]  # 母体来源 ID 列表
    present*: bool        # 是否有像素

const
  PHASE_BASE* = 64
  EMPTY_CELL* = PhaseCell(present: false)

# ------------------------------------------------------------
# 创建空单元格
# ------------------------------------------------------------
func init_cell*(): PhaseCell =
  result = PhaseCell(
    phase: 0,
    weight: 0.0,
    level: plL1,
    owner: "SELF",
    parent: @[],
    present: false
  )

# ------------------------------------------------------------
# 创建有值的单元格
# ------------------------------------------------------------
func make_cell*(phase: int, weight: float, level: PhaseLevel = plL1,
                owner: string = "SELF", parent: seq[string] = @[]): PhaseCell =
  result = PhaseCell(
    phase: ((phase mod PHASE_BASE) + PHASE_BASE) mod PHASE_BASE,
    weight: weight,
    level: level,
    owner: owner,
    parent: parent,
    present: true
  )

# ------------------------------------------------------------
# 相位 → RGB 映射 (HSV 色相环)
# phase 0-63 → hue 0-360°, weight → value/亮度
# ------------------------------------------------------------
func phase_to_rgb*(phase: int, weight: float): tuple[r, g, b: int] =
  let h = (float(phase) / float(PHASE_BASE)) * 360.0
  let s = 0.85
  let v = max(0.2, min(1.0, weight))
  
  let hi = int(h / 60.0) mod 6
  let f = (h / 60.0) - float(int(h / 60.0))
  let p = v * (1.0 - s)
  let q = v * (1.0 - f * s)
  let t = v * (1.0 - (1.0 - f) * s)
  
  var r, g, b: float
  case hi
  of 0: r = v; g = t; b = p
  of 1: r = q; g = v; b = p
  of 2: r = p; g = v; b = t
  of 3: r = p; g = q; b = v
  of 4: r = t; g = p; b = v
  else: r = v; g = p; b = q
  
  result = (
    r: int(r * 255.0),
    g: int(g * 255.0),
    b: int(b * 255.0)
  )

# ------------------------------------------------------------
# 相位加法 (仅用于数值运算，不用于像素操作)
# 结果对 64 取模
# ------------------------------------------------------------
func phase_add*(a, b: int): int =
  ((a + b) mod PHASE_BASE + PHASE_BASE) mod PHASE_BASE

# ------------------------------------------------------------
# 相位减法
# ------------------------------------------------------------
func phase_sub*(a, b: int): int =
  ((a - b) mod PHASE_BASE + PHASE_BASE) mod PHASE_BASE

# ------------------------------------------------------------
# 层级名称
# ------------------------------------------------------------
func level_name*(l: PhaseLevel): string =
  case l
  of plL1: "L1-基元"
  of plL2: "L2-部件"
  of plL3: "L3-成品"

# ------------------------------------------------------------
# 单元格摘要
# ------------------------------------------------------------
func cell_summary*(c: PhaseCell): string =
  if not c.present:
    return "·"
  var s = $c.phase
  if c.weight > 0.0:
    s &= "(" & formatFloat(c.weight, ffDecimal, 1) & ")"
  s &= "-" & level_name(c.level)[0..2]
  result = s
