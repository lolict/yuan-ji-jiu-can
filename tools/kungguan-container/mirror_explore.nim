# 满全法·探索不同操作对的2D方阵生成效果

import math, strutils, algorithm, tables

# ============================================================
# 基础操作定义
# ============================================================

func mc*(c: char): char =
  case c
  of 'p': result = 'q'
  of 'q': result = 'p'
  of 'b': result = 'd'
  of 'd': result = 'b'
  else: result = c

# ---- 操作函数 ----
func op_reflect*(s: string): string =  # 逐字镜像 + 反转
  result = new_string(4)
  for i in 0..3: result[3 - i] = mc(s[i])

func op_reverse*(s: string): string =  # 纯反转
  result = new_string(4)
  for i in 0..3: result[3 - i] = s[i]

func op_mirror_only*(s: string): string =  # 仅逐字镜像（不反转）
  for c in s: result.add(mc(c))

func op_swap_01*(s: string): string = result = s; swap(result[0], result[1])
func op_swap_02*(s: string): string = result = s; swap(result[0], result[2])
func op_swap_03*(s: string): string = result = s; swap(result[0], result[3])
func op_swap_12*(s: string): string = result = s; swap(result[1], result[2])
func op_swap_13*(s: string): string = result = s; swap(result[1], result[3])
func op_swap_23*(s: string): string = result = s; swap(result[2], result[3])

func op_rot_left*(s: string): string =  # 左旋：abcd → bcda
  result = s[1] & s[2] & s[3] & s[0]

func op_rot_right*(s: string): string =  # 右旋：abcd → dabc
  result = s[3] & s[0] & s[1] & s[2]

func op_invert_pos*(s: string): string =  # 位置逆转：abcd → badc
  result = new_string(4)
  result[0] = s[1]; result[1] = s[0]
  result[2] = s[3]; result[3] = s[2]

func op_01swap_23swap*(s: string): string =  # 同时交换(0,1)和(2,3)
  result = s
  swap(result[0], result[1])
  swap(result[2], result[3])

func op_02swap_13swap*(s: string): string =  # 同时交换(0,2)和(1,3)
  result = s
  swap(result[0], result[2])
  swap(result[1], result[3])

# ============================================================
# 测试操作对的交换性
# ============================================================

proc all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

proc check_commute*(op1, op2: proc(s: string): string): bool =
  ## 检查两个操作是否可交换
  let all = all_codes()
  for s in all:
    if op1(op2(s)) != op2(op1(s)):
      return false
  return true

proc count_grids*(op1, op2: proc(s: string): string): tuple[proper, degenerate, total: int] =
  ## 统计完整方阵和退化方阵数量
  let all = all_codes()
  var seen: seq[string] = @[]
  for s in all:
    let a = s
    let b = op1(s)
    let c = op2(s)
    let d = op1(op2(s))  # 因为可交换，也等于 op2(op1(s))
    
    let repr = min(min(a, b), min(c, d))
    if repr in seen: continue
    seen.add(repr)
    
    # 统计不同代码数
    var distinct_count = 1
    if b != a: distinct_count += 1
    if c != a and c != b: distinct_count += 1
    if d != a and d != b and d != c: distinct_count += 1
    
    if distinct_count == 4:
      result.proper += 1
    else:
      result.degenerate += 1
    result.total += 1

# ============================================================
# 测试所有操作对
# ============================================================

type
  OpPair = tuple
    name1, name2: string
    op1, op2: proc(s: string): string

when is_main_module:
  echo "━━━ 满全法·探索不同操作对的2D方阵效果 ━━━"
  echo ""

  # 定义所有测试的操作对
  const pairs: seq[OpPair] = @[
    ("reflect", "reverse", op_reflect, op_reverse),
    ("reflect", "mirror_only", op_reflect, op_mirror_only),
    ("reflect", "swap_01", op_reflect, op_swap_01),
    ("reflect", "swap_03", op_reflect, op_swap_03),
    ("reflect", "swap_12", op_reflect, op_swap_12),
    ("reflect", "rot_left", op_reflect, op_rot_left),
    ("reflect", "invert_pos", op_reflect, op_invert_pos),
    ("reflect", "01swap+23swap", op_reflect, op_01swap_23swap),
    ("reverse", "mirror_only", op_reverse, op_mirror_only),
    ("reverse", "swap_01", op_reverse, op_swap_01),
    ("reverse", "swap_03", op_reverse, op_swap_03),
    ("reverse", "rot_left", op_reverse, op_rot_left),
    ("reverse", "invert_pos", op_reverse, op_invert_pos),
    ("mirror_only", "swap_01", op_mirror_only, op_swap_01),
    ("mirror_only", "swap_03", op_mirror_only, op_swap_03),
    ("mirror_only", "invert_pos", op_mirror_only, op_invert_pos),
    ("swap_01", "swap_23", op_swap_01, op_swap_23),
    ("swap_02", "swap_13", op_swap_02, op_swap_13),
    ("swap_03", "swap_12", op_swap_03, op_swap_12),
    ("rot_left", "rot_right", op_rot_left, op_rot_right),
    ("rot_left", "invert_pos", op_rot_left, op_invert_pos),
  ]

  echo "┌─────────────────────────────────────────────────────┐"
  echo "│  操作对                       交换性  完整  退化  总计 │"
  echo "├─────────────────────────────────────────────────────┤"

  for p in pairs:
    let commutes = check_commute(p.op1, p.op2)
    let (proper, degenerate, total) = count_grids(p.op1, p.op2)
    let commute_str = if commutes: "  ✓  " else: "  ✗  "
    let name_pair = align(p.name1 & " × " & p.name2, 30)
    echo "│ ", name_pair, commute_str,
        align($proper, 5), " ", align($degenerate, 5), " ", align($total, 5), "│"

  echo "└─────────────────────────────────────────────────────┘"
  echo ""
  echo "说明："
  echo "  ✓ = 操作可交换（形成有效群）"
  echo "  ✗ = 操作不可交换（结果依赖顺序）"
  echo "  完整 = 4个不同代码组成方阵"
  echo "  退化 = 有重复代码（1-3个不同代码）"
  echo "  总计 = 完整 + 退化（应≤64）"
