# 满全法·镜像方阵·reflect×rot_left·79个完整方阵
#
# 最佳组合：reflect（左右对偶） × rot_left（上下对偶）
# 79个完整方阵 + 14个退化 = 93总计

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

func rot_left*(s: string): string =
  result = s[1] & s[2] & s[3] & s[0]

func all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

type
  Grid2D* = object
    id*: int
    a*, b*, c*, d*: string

func make_grids*(): seq[Grid2D] =
  let all = all_codes()
  var seen: seq[string] = @[]
  var idx = 0
  for s in all:
    let ga = s
    let gb = reflect(s)
    let gc = rot_left(s)
    let gd = reflect(rot_left(s))
    
    let repr = min(min(ga, gb), min(gc, gd))
    if repr in seen: continue
    seen.add(repr)
    
    # 统计不同代码数
    var ds = 1
    if gb != ga: ds += 1
    if gc != ga and gc != gb: ds += 1
    if gd != ga and gd != gb and gd != gc: ds += 1
    
    if ds == 4:
      idx += 1
      result.add(Grid2D(id: idx, a: ga, b: gb, c: gc, d: gd))

func count_degenerate*(): int =
  let all = all_codes()
  var seen: seq[string] = @[]
  for s in all:
    let ga = s
    let gb = reflect(s)
    let gc = rot_left(s)
    let gd = reflect(rot_left(s))
    
    let repr = min(min(ga, gb), min(gc, gd))
    if repr in seen: continue
    seen.add(repr)
    
    var ds = 1
    if gb != ga: ds += 1
    if gc != ga and gc != gb: ds += 1
    if gd != ga and gd != gb and gd != gc: ds += 1
    
    if ds != 4:
      result += 1

when is_main_module:
  let grids = make_grids()
  let degen = count_degenerate()
  let total = grids.len * 4 + degen * 4

  echo "━━━ 满全法·reflect×rot_left·二维镜像方阵 ━━━"
  echo ""
  echo "完整方阵: ", grids.len, " 个"
  echo "退化方阵: ", degen, " 个"
  echo "总代码:   ", total, " (应 ≤ 256)"
  echo ""
  echo "━━━ 全部 ", grids.len, " 个完整方阵 ━━━"
  echo ""

  for g in grids:
    echo "  ┌──────┬──────┐"
    echo "  │ ", g.a, " │ ", g.b, " │"
    echo "  ├──────┼──────┤"
    echo "  │ ", g.c, " │ ", g.d, " │"
    echo "  └──────┴──────┘"
    echo ""

  echo "━━━ 共 ", grids.len, " 个 2×2 方阵 ━━━"
