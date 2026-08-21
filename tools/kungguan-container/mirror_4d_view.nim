# 满全法·4D超立方体·视觉渲染
#
# 6个proper 4D超立方体，每个16顶点全不同
# 用4×4矩阵展示四维结构

import strutils

func mc*(c: char): char =
  case c
  of 'p': result = 'q'
  of 'q': result = 'p'
  of 'b': result = 'd'
  of 'd': result = 'b'
  else: result = c

func reflect*(s: string): string =
  result = new_string(4)
  for i in 0..3: result[3 - i] = mc(s[i])

func rot_left*(s: string): string = s[1] & s[2] & s[3] & s[0]
func rot_right*(s: string): string = s[3] & s[0] & s[1] & s[2]
func rot_180*(s: string): string = s[2] & s[3] & s[0] & s[1]
func flip_v*(s: string): string = $s[2] & s[1] & s[0] & s[3]
func flip_h*(s: string): string = s[0] & s[3] & s[2] & s[1]
func swap_diag*(s: string): string = $s[3] & s[1] & s[2] & s[0]
func swap_anti*(s: string): string = s[0] & $s[2] & s[1] & s[3]
func invert_all*(s: string): string =
  result = new_string(4)
  for i in 0..3: result[i] = mc(s[i])

type
  Op = tuple[name: string; fn: proc(s: string): string]
  HC4D = object
    op_names: array[4, string]
    vertices: seq[string]
    base: string

const OPS: seq[Op] = @[
  ("refl", reflect),
  ("rotL", rot_left),
  ("rotR", rot_right),
  ("rot2", rot_180),
  ("fl_v", flip_v),
  ("fl_h", flip_h),
  ("sw_d", swap_diag),
  ("sw_a", swap_anti),
  ("invA", invert_all),
]

func all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

proc hypercube*(base: string; ops: seq[proc(s: string): string]): seq[string] =
  let d = ops.len
  let n = 1 shl d
  result = new_seq[string](n)
  for mask in 0..<n:
    var cur = base
    for bit in 0..<d:
      if (mask and (1 shl bit)) != 0:
        cur = ops[bit](cur)
    result[mask] = cur

proc count_distinct*(s: seq[string]): int =
  var seen: seq[string] = @[]
  for x in s:
    if x notin seen: seen.add(x)
  result = seen.len

func canonical*(s: seq[string]): string =
  var r = s[0]
  for x in s:
    if x < r: r = x
  result = r

proc render_hypercube*(hc: HC4D) =
  echo "  基准编码: ", hc.base
  echo "  操作: ", hc.op_names[0], " × ", hc.op_names[1], " × ",
                 hc.op_names[2], " × ", hc.op_names[3]
  echo ""
  echo "  4×4 超立方体（行=op0,op1, 列=op2,op3）:"
  echo ""
  echo "         col=00     col=01     col=10     col=11"
  echo "  ─────┬──────────┬──────────┬──────────┬──────────┐"
  
  for row in 0..3:
    let row_label = case row
      of 0: "00"
      of 1: "01"
      of 2: "10"
      else: "11"
    var line = "  " & row_label & "│"
    for col in 0..3:
      let mask = row + col * 4
      let code = hc.vertices[mask]
      line &= " " & code & "   │"
    echo line
    
    # 方块行
    var line2 = "  │"
    for col in 0..3:
      let mask = row + col * 4
      let code = hc.vertices[mask]
      func gx(c: char): string =
        case c
        of 'p': result = "█"
        of 'q': result = "░"
        of 'b': result = "▓"
        of 'd': result = "▒"
        else:   result = "·"
      let g = gx(code[0]) & gx(code[1]) & " " & gx(code[2]) & gx(code[3])
      line2 &= " " & g & " │"
    echo line2
    
    if row < 3:
      echo "  ├──────────┼──────────┼──────────┼──────────┤"
  
  echo "  ─────┴──────────┴──────────┴──────────┴──────────┘"
  echo ""
  echo "  16顶点: ", hc.vertices[0], " ", hc.vertices[1], " ",
       hc.vertices[2], " ", hc.vertices[3], " ... ",
       hc.vertices[14], " ", hc.vertices[15]
  echo ""

proc find_and_render*() =
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  满全法·4D超立方体·视觉渲染"
  echo "  编码空间: 256 (4字母 p/q/b/d)"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  
  # 最佳四操作组合（从mirror_nd的结果中提取）
  const BEST_NAMES = [
    ["refl", "rotL", "rot2", "fl_v"],
    ["refl", "rotR", "rot2", "fl_v"],
    ["refl", "rot2", "fl_v", "invA"],
    ["rotL", "rot2", "fl_v", "invA"],
    ["rotR", "rot2", "fl_v", "invA"],
    ["rot2", "sw_d", "sw_a", "invA"],
  ]
  
  let all = all_codes()
  var all_hc: seq[HC4D] = @[]
  
  for names in BEST_NAMES:
    # 查找对应操作函数
    var ops: seq[proc(s: string): string] = @[]
    for n in names:
      for op in OPS:
        if op.name == n:
          ops.add(op.fn)
          break
    
    if ops.len != 4: continue
    
    var seen: seq[string] = @[]
    for s in all:
      let hc = hypercube(s, ops)
      let canon = canonical(hc)
      if canon in seen: continue
      seen.add(canon)
      if count_distinct(hc) == 16:
        var entry: HC4D
        for i in 0..3: entry.op_names[i] = names[i]
        entry.vertices = hc
        entry.base = s
        all_hc.add(entry)
        break  # 只取每个组合的第一
  
  echo "  找到 ", all_hc.len, " 个 4D proper 超立方体"
  echo "  █=p(阳)  ░=q(阴)  ▓=b(刚)  ▒=d(柔)"
  echo ""
  
  for i, hc in all_hc:
    echo "  ═══════════════════════════════════════════════"
    echo "  超立方体 ", i+1, " / ", all_hc.len
    echo "  ═══════════════════════════════════════════════"
    echo ""
    render_hypercube(hc)
  
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  维度递增总结:"
  echo "  2D: 88 proper (4顶点, 352编码位置)"
  echo "  3D: 28 proper (8顶点, 224编码位置)"
  echo "  4D:  6 proper (16顶点, 96编码位置)"
  echo "  编码空间: 256 = 4⁴ (2⁸)"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

when is_main_module:
  find_and_render()
