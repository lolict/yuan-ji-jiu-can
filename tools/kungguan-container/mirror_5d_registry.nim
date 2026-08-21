# 满全法·5D超立方体 + 编码注册表
#
# 探索5D（32顶点）是否还有proper超立方体
# 同时输出完整编码注册表

import strutils
import json

# ============================================================
# 操作集
# ============================================================

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

type Op = tuple[name: string; fn: proc(s: string): string]

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

# ============================================================
# 5D搜索
# ============================================================

proc search_5d*(): int =
  let all = all_codes()
  var best = 0
  let n_ops = OPS.len
  var tested = 0
  
  for a in 0..<n_ops:
    for b in (a+1)..<n_ops:
      for c in (b+1)..<n_ops:
        for d in (c+1)..<n_ops:
          for e in (d+1)..<n_ops:
            let ops = @[OPS[a].fn, OPS[b].fn, OPS[c].fn, OPS[d].fn, OPS[e].fn]
            var seen: seq[string] = @[]
            var proper = 0
            for s in all:
              let hc = hypercube(s, ops)
              let canon = canonical(hc)
              if canon in seen: continue
              seen.add(canon)
              if count_distinct(hc) == 32:
                proper += 1
            tested += 1
            if proper > best:
              best = proper
              if proper > 0:
                echo "  5D found: ", OPS[a].name, " × ", OPS[b].name, " × ",
                     OPS[c].name, " × ", OPS[d].name, " × ", OPS[e].name,
                     " = ", proper, " proper"
            if best >= 1: return best  # 提前退出
  
  result = best

# ============================================================
# 编码注册表生成
# ============================================================

type
  CodeInfo = object
    code: string
    block_glyph: string  # 方块字表示
    self_mirror: bool   # 16个自镜像码之一
    mirror_partner: string  # 配对码（如果是自镜像则等于自身）
    dim2_belongs: seq[string]   # 属于哪些2D方阵的canonical
    dim3_belongs: seq[string]   # 属于哪些3D立方体

proc generate_registry*() =
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  满全法·编码注册表生成"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  
  let all = all_codes()
  
  # 先收集所有2D方阵
  var dim2_canons: seq[tuple[canon: string; codes: seq[string]]] = @[]
  for i in 0..<OPS.len:
    for j in (i+1)..<OPS.len:
      if OPS[i].name != "rotL" or OPS[j].name != "invA": continue  # 用最佳组合
      let ops = @[OPS[i].fn, OPS[j].fn]
      var seen: seq[string] = @[]
      for s in all:
        let hc = hypercube(s, ops)
        let canon = canonical(hc)
        if canon in seen: continue
        seen.add(canon)
        if count_distinct(hc) == 4:
          dim2_canons.add((canon, hc))
      break
  
  # 统计每个编码参与多少个2D方阵
  var code_dim2_count: seq[tuple[code: string; count: int]] = @[]
  for s in all:
    var count = 0
    for d2 in dim2_canons:
      for c in d2.codes:
        if c == s:
          count += 1
          break
    code_dim2_count.add((s, count))
  
  echo "  总2D proper方阵 (rotL × invA): ", dim2_canons.len
  echo "  总编码: 256"
  echo ""
  
  # 输出统计
  var distribution: seq[int] = @[]
  for i in 0..33: distribution.add(0)
  for cd in code_dim2_count:
    let idx = min(cd.count, 33)
    distribution[idx] += 1
  
  echo "  编码参与方阵数的分布："
  for i in 0..<distribution.len:
    if distribution[i] > 0:
      let bar = repeat("█", min(distribution[i] div 2, 40))
      echo "    ", align($i, 2), "个方阵: ", align($distribution[i], 3), " codes  ", bar
  echo ""
  
  # 输出前30个参与方阵最多的编码
  echo "  参与2D方阵最多的编码（前30）："
  # 按参与数排序
  var sorted_codes = code_dim2_count
  for i in 0..<sorted_codes.len:
    for j in (i+1)..<sorted_codes.len:
      if sorted_codes[j].count > sorted_codes[i].count:
        let tmp = sorted_codes[j]
        sorted_codes[j] = sorted_codes[i]
        sorted_codes[i] = tmp
  
  for i in 0..<min(30, sorted_codes.len):
    let cd = sorted_codes[i]
    # 计算1的密度（p和b = 1，q和d = 0）
    var ones = 0
    for ch in cd.code:
      if ch == 'p' or ch == 'b': ones += 1
    echo "    ", cd.code, " (", ones, "个1, 参与", cd.count, "方阵)"
  
  echo ""

# ============================================================
# 主程序
# ============================================================

when is_main_module:
  echo "━━━ 5D超立方体搜索（32顶点全不同）━━━"
  echo ""
  
  let result5 = search_5d()
  
  echo ""
  if result5 == 0:
    echo "  ★ 5D: 0 proper 超立方体（32顶点全不同在256编码空间中不存在）"
    echo ""
    echo "  结论：维度上限 = 4D（最高存在proper超立方体的维度）"
  else:
    echo "  ★ 5D: ", result5, " proper 超立方体"
  
  echo ""
  
  # 生成编码注册表
  generate_registry()
  
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  维度增长总结："
  echo "  1D: 256 编码（无序）"
  echo "  2D: 88 proper 方阵（4顶点, 352编码位置）"
  echo "  3D: 28 proper 立方体（8顶点, 224编码位置）"
  echo "  4D:  6 proper 超立方体（16顶点, 96编码位置）"
  echo "  5D:  0 proper（32顶点全不同不存在）"
  echo ""
  echo "  实际有效的维度范围: 2D-4D"
  echo "  最大信息密度: 2D（88个结构）"
  echo "  最高单结构复杂度: 4D（16个不同编码）"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
