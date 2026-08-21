# 满全法·镜像方阵·上下对偶·左右对偶
#
# 目标：将256个镜像编码排列成方阵
#   每行2个，形成上下对偶
#   每列2个，形成左右对偶
#   16个方阵 = 32×8 = 256
#
# 对偶规则：
#   左右对偶：标准镜像 = 逐字替换 + 反转
#   上下对偶：首字对调 + 二三字对调

import math, strutils, algorithm

# ============================================================
# 基础镜像操作
# ============================================================

const MIRROR_MAP*: array[4, char] = ['q', 'p', 'd', 'b']  # p→q, q→p, b→d, d→b

func mc*(c: char): char =
  case c
  of 'p': result = 'q'
  of 'q': result = 'p'
  of 'b': result = 'd'
  of 'd': result = 'b'
  else: result = c

func reflect*(s: string): string =
  ## 左右对偶：逐字镜像 + 反转
  result = new_string(s.len)
  for i in 0..<s.len:
    result[s.len - 1 - i] = mc(s[i])

func vreflect*(s: string): string =
  ## 上下对偶：首字对调位置1 + 二三字对调位置3，末字不动
  ## 位置变换：pos0→pos1, pos1→pos0, pos2→pos3, pos3→pos2
  result = new_string(4)
  result[0] = s[1]  # pos1的字移到pos0
  result[1] = s[0]  # pos0的字移到pos1
  result[2] = s[3]  # pos3的字移到pos2
  result[3] = s[2]  # pos2的字移到pos3

func vreflect_alt*(s: string): string =
  ## 另一种上下对偶：首字对调
  result = s
  let tmp = result[0]
  result[0] = result[1]
  result[1] = tmp

func is_self_mirror*(s: string): bool =
  result = (reflect(s) == s)

# ============================================================
# 生成全部256个（固定遍历顺序，保证镜像对称显示）
# ============================================================

func all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

# ============================================================
# 16个一组显示
# ============================================================

proc show_16*(start_idx: int; all: seq[string]) =
  ## 一次显示16个编码（8对上下对偶，每对2个左右对偶）
  let end_idx = min(start_idx + 16, all.len)
  let n_in_group = end_idx - start_idx

  echo "━━━ 第 ", (start_idx div 16) + 1, " 批（", start_idx, "-", end_idx - 1, "）━━━"
  echo ""

  for i in countup(start_idx, end_idx - 1, 2):
    let a = all[i]
    let b = reflect(a)
    let c = vreflect(a)
    let d = vreflect(b)
    echo "  ", a, " ─ ", b, " ─ ", c, " ─ ", d

  echo ""

# ============================================================
# 测试
# ============================================================

when is_main_module:
  let codes = all_codes()

  echo "━━━ 满全法·镜像方阵·256个 ━━━"
  echo ""

  echo "自镜像 16 个（标准 4×4 排列）："
  echo ""

  # 选取16个自镜像
  var self_mirrors: seq[string] = @[]
  for s in codes:
    if is_self_mirror(s):
      self_mirrors.add(s)
  self_mirrors.sort()
  echo self_mirrors.join("  ")

  echo ""
  echo ""
  echo "━━━ 镜像对 120 个（每行 2 对） ━━━"
  echo ""

  var sm_count = 0
  var pair_count = 0
  var line: seq[string] = @[]

  for s in codes:
    let r = reflect(s)
    if s == r:
      # 自镜像
      line.add(s)
      sm_count += 1
    elif s < r:  # 每对只取字典序较小者
      line.add(s & "↔" & r)
      pair_count += 1

    if line.len >= 8:  # 每行显示8项
      echo "  ", line.join("    ")
      line = @[]

  if line.len > 0:
    echo "  ", line.join("    ")

  echo ""
  echo "自镜像: ", sm_count, " 个"
  echo "镜像对: ", pair_count, " 对（共 ", pair_count * 2, " 个）"
  echo "总计: ", sm_count + pair_count * 2, " = 256"
