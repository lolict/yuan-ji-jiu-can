# 满全法·镜像3D·立方体编码
#
# 从2D方阵扩展到3D立方体
# 三个操作轴互相独立，生成8个顶点：
#
#   (0,0,0) = A           (1,0,0) = reflect(A)
#   (0,1,0) = rot_left(A) (1,1,0) = reflect(rot_left(A))
#   (0,0,1) = flip_v(A)   (1,0,1) = reflect(flip_v(A))
#   (0,1,1) = rot_left(flip_v(A)) (1,1,1) = reflect(rot_left(flip_v(A)))

import strutils

# ============================================================
# 基础镜像操作
# ============================================================

func mc*(c: char): char =
  case c
  of 'p': result = 'q'
  of 'q': result = 'p'
  of 'b': result = 'd'
  of 'd': result = 'b'
  else: result = c

func reflect*(s: string): string =
  ## 镜像轴（水平镜像 + 字符互换）
  result = new_string(4)
  for i in 0..3: result[3 - i] = mc(s[i])

func rot_left*(s: string): string =
  ## 旋转轴（循环左移）
  result = s[1] & s[2] & s[3] & s[0]

func flip_v*(s: string): string =
  ## 垂直翻转轴（交换上下：位置0↔2）
  result = $s[2] & s[1] & s[0] & s[3]

func flip_h*(s: string): string =
  ## 水平翻转轴（交换左右：位置1↔3）
  result = s[0] & s[3] & s[2] & s[1]

func swap_diag*(s: string): string =
  ## 对角交换（0↔3）
  result = $s[3] & s[1] & s[2] & s[0]

func invert_all*(s: string): string =
  ## 全镜像（每个字符都取镜像）
  result = new_string(4)
  for i in 0..3: result[i] = mc(s[i])

# ============================================================
# 3D立方体 = 8个顶点
# ============================================================

type
  Cube3D* = object
    id*: int
    vx*: array[8, string]  # 8个顶点编码
    distinct_count*: int      # 不同编码数量 (8=proper, <8=degenerate)

  Grid2D* = object
    id*: int
    a*, b*, c*, d*: string

proc make_cube*(a: string; op1, op2, op3: proc(s: string): string): Cube3D =
  ## 从起始编码A + 三个操作，生成立方体8个顶点
  let v0 = a
  let v1 = op1(a)
  let v2 = op2(a)
  let v3 = op3(a)
  let v4 = op1(op2(a))  # op1∘op2
  let v5 = op1(op3(a))  # op1∘op3
  let v6 = op2(op3(a))  # op2∘op3
  let v7 = op1(op2(op3(a))) # op1∘op2∘op3
  
  result.vx = [v0, v1, v2, v3, v4, v5, v6, v7]
  
  # 统计不同编码数量
  var seen: seq[string] = @[]
  for v in result.vx:
    if v notin seen: seen.add(v)
  result.distinct_count = seen.len

func all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

# ============================================================
# 比较：2D vs 3D 的 proper 结构数量
# ============================================================

func all_ops*(): seq[tuple[name: string; fn: proc(s: string): string]] =
  result = @[
    ("reflect", reflect),
    ("rot_left", rot_left),
    ("flip_v", flip_v),
    ("flip_h", flip_h),
    ("swap_diag", swap_diag),
    ("invert_all", invert_all),
  ]

proc count_cubes*(op1, op2, op3: proc(s: string): string): tuple[proper, partial, degenerate: int] =
  ## 统计 8-proper / partial (<8但>1) / degenerate (=1)
  let all = all_codes()
  var seen_repr: seq[string] = @[]
  
  for s in all:
    let cube = make_cube(s, op1, op2, op3)
    
    # 找到规范代表（最小值）
    var min_repr = cube.vx[0]
    for v in cube.vx:
      if v < min_repr: min_repr = v
    if min_repr in seen_repr: continue
    seen_repr.add(min_repr)
    
    if cube.distinct_count == 8:
      result.proper += 1
    elif cube.distinct_count == 1:
      result.degenerate += 1
    else:
      result.partial += 1

proc test_all_triplets*() =
  let ops = all_ops()
  
  echo "━━━ 3D立方体·三操作组合测试 ━━━"
  echo ""
  echo "━" .repeat(65)
  echo "  操作三元组                          proper  partial  degenerate"
  echo "━" .repeat(65)
  
  var best_count = 0
  var best_name = ""
  
  for i in 0..<ops.len:
    for j in (i+1)..<ops.len:
      for k in (j+1)..<ops.len:
        let name = ops[i].name & " × " & ops[j].name & " × " & ops[k].name
        let (proper, partial, degen) = count_cubes(ops[i].fn, ops[j].fn, ops[k].fn)
        
        echo "  ", align(name, 35), "  ", align($proper, 6), "  ", align($partial, 7), "  ", align($degen, 8)
        
        if proper > best_count:
          best_count = proper
          best_name = name
  
  echo "━" .repeat(65)
  echo ""
  echo "  ★ 最大: ", best_name, " = ", best_count, " proper 立方体"
  
  # 与2D比较
  echo ""
  echo "━━━ 对比：2D方阵 vs 3D立方体 ━━━"
  echo ""
  echo "  2D (reflect × rot_left):          79 proper 方阵"
  echo "  3D (最佳组合):                    ", best_count, " proper 立方体"
  echo ""
  echo "  2D: 每方阵 4编码 × 79 = 316 编码位置"
  echo "  3D: 每立方 8编码 × ", best_count, " = ", best_count * 8, " 编码位置"
  echo ""

# ============================================================
# 渲染3D立方体
# ============================================================

func cube_face_glyphs*(c: Cube3D): string =
  ## 将立方体的三个可见面渲染为方块字
  ## 正面(v0 v1 v2 v3) 右侧面(v1 v5 v3 v7) 顶面(v0 v2 v4 v6)
  func g(ch: char): string =
    case ch
    of 'p': result = "█"
    of 'q': result = "░"
    of 'b': result = "▓"
    of 'd': result = "▒"
    else: result = " "
  
  # 每个顶点是一个4字母编码，取首字母作为方块字代表
  let f0 = g(c.vx[0][0])  # 正面四个角的代表
  let f1 = g(c.vx[1][0])
  let f2 = g(c.vx[2][0])
  let f3 = g(c.vx[3][0])
  let f4 = g(c.vx[4][0])
  let f5 = g(c.vx[5][0])
  let f6 = g(c.vx[6][0])
  let f7 = g(c.vx[7][0])
  
  result = "  正面(z=0):      右侧(x=1):      顶(y=0):\n"
  result &= "  ┌─┬─┐        ┌─┬─┐        ┌─┬─┐\n"
  result &= "  │" & f0 & "│" & f1 & "│       │" & f1 & "│" & f5 & "│       │" & f0 & "│" & f2 & "│\n"
  result &= "  ├─┼─┤        ├─┼─┤        ├─┼─┤\n"
  result &= "  │" & f2 & "│" & f3 & "│       │" & f3 & "│" & f7 & "│       │" & f4 & "│" & f6 & "│\n"
  result &= "  └─┴─┘        └─┴─┘        └─┴─┘"
  

# ============================================================
# 主函数
# ============================================================

when is_main_module:
  test_all_triplets()
  
  echo "━━━ 立方体示例 (reflect × rot_left × flip_v) ━━━"
  echo ""
  
  let all = all_codes()
  var cubes_shown = 0
  var seen: seq[string] = @[]
  
  for s in all:
    let cube = make_cube(s, reflect, rot_left, flip_v)
    if cube.distinct_count != 8: continue
    
    var min_repr = cube.vx[0]
    for v in cube.vx:
      if v < min_repr: min_repr = v
    if min_repr in seen: continue
    seen.add(min_repr)
    
    cubes_shown += 1
    if cubes_shown > 3: break  # 只显示前3个
    
    echo "  立方体 #", cubes_shown, ":"
    echo "    8顶点: ", cube.vx[0], " ", cube.vx[1], " ", cube.vx[2], " ", cube.vx[3],
         " | ", cube.vx[4], " ", cube.vx[5], " ", cube.vx[6], " ", cube.vx[7]
    echo cube_face_glyphs(cube)
    echo ""
