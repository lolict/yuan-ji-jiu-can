# ============================================================
# 颜色进制 · 相位画布
# 二维相位单元网格，每个单元存储相位/权重/层级/归属
# ============================================================

import phase
import sequtils

type
  PhaseCanvas* = object
    width*: int
    height*: int
    cells*: seq[seq[PhaseCell]]
    tick*: int              # 操作计数
    op_log*: seq[string]    # 操作日志

# ------------------------------------------------------------
# 创建画布
# ------------------------------------------------------------
func new_phase_canvas*(w, h: int): PhaseCanvas =
  result.width = w
  result.height = h
  result.cells = newSeqWith(h, newSeqWith(w, init_cell()))
  result.tick = 0
  result.op_log = @[]

# ------------------------------------------------------------
# 获取/设置单元格
# ------------------------------------------------------------
func get*(c: PhaseCanvas, x, y: int): PhaseCell =
  if x < 0 or x >= c.width or y < 0 or y >= c.height:
    return EMPTY_CELL
  result = c.cells[y][x]

func set*(c: var PhaseCanvas, x, y: int, cell: PhaseCell) =
  if x < 0 or x >= c.width or y < 0 or y >= c.height:
    return
  c.cells[y][x] = cell

# ------------------------------------------------------------
# 加法运算：在 (dx, dy) 位置安装组件
# 组件的相位单元叠加到画布上
# ------------------------------------------------------------
func add*(c: var PhaseCanvas, comp: PhaseCanvas, dx, dy: int): int =
  var installed = 0
  for y in 0..<comp.height:
    for x in 0..<comp.width:
      let src = comp.cells[y][x]
      if not src.present:
        continue
      let tx = x + dx
      let ty = y + dy
      if tx < 0 or tx >= c.width or ty < 0 or ty >= c.height:
        continue
      c.cells[ty][tx] = src
      installed += 1
  c.tick += 1
  c.op_log.add("ADD@" & $dx & "," & $dy & " ×" & $installed)
  result = installed

# ------------------------------------------------------------
# 减法运算：挖除 (dx, dy) 开始 w×h 区域的像素
# ------------------------------------------------------------
func subtract*(c: var PhaseCanvas, dx, dy, w, h: int): int =
  var removed = 0
  for y in dy..<dy+h:
    for x in dx..<dx+w:
      if x < 0 or x >= c.width or y < 0 or y >= c.height:
        continue
      if c.cells[y][x].present:
        c.cells[y][x] = init_cell()
        removed += 1
  c.tick += 1
  c.op_log.add("SUB@" & $dx & "," & $dy & " ×" & $removed)
  result = removed

# ------------------------------------------------------------
# 统计画布信息
# ------------------------------------------------------------
func count_present*(c: PhaseCanvas): int =
  for row in c.cells:
    for cell in row:
      if cell.present:
        result += 1

func level_count*(c: PhaseCanvas): tuple[l1, l2, l3: int] =
  for row in c.cells:
    for cell in row:
      if cell.present:
        case cell.level
        of plL1: result.l1 += 1
        of plL2: result.l2 += 1
        of plL3: result.l3 += 1

# ------------------------------------------------------------
# 画布摘要
# ------------------------------------------------------------
func canvas_summary*(c: PhaseCanvas): string =
  let present = c.count_present()
  let total = c.width * c.height
  let lc = c.level_count()
  result = $c.width & "×" & $c.height &
           " 像素:" & $present & "/" & $total &
           " L1:" & $lc.l1 & " L2:" & $lc.l2 & " L3:" & $lc.l3 &
           " tick:" & $c.tick
