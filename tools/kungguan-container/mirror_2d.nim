# 满全法·镜像方阵·上下对偶·左右对偶·二维平面
#
# 二维方阵列：4个代码同时发生关系
#   左右对偶（水平镜像）= reflect = 逐字镜像 + 反转
#   上下对偶（垂直镜像）= reverse = 纯反转
#   两种操作不同但可交换
#
# 对于每个起始代码 A：
#   A ──────reflect────→ B
#   │                    │
#   reverse             reverse
#   ↓                    ↓
#   C ──────reflect────→ D
#
# 完整 2×2 方阵：56个（每阵4个不同代码）

import strutils

# ============================================================
# 基础操作
# ============================================================

func mc*(c: char): char =
  case c
  of 'p': result = 'q'
  of 'q': result = 'p'
  of 'b': result = 'd'
  of 'd': result = 'b'
  else: result = c

func reflect*(s: string): string =
  ## 左右对偶：逐字镜像 + 反转
  result = new_string(4)
  for i in 0..3:
    result[3 - i] = mc(s[i])

func reverse_str*(s: string): string =
  ## 上下对偶：纯反转
  result = new_string(4)
  for i in 0..3:
    result[3 - i] = s[i]

func is_self_mirror*(s: string): bool =
  result = (reflect(s) == s)

func is_palindrome*(s: string): bool =
  result = (reverse_str(s) == s)

# ============================================================
# 生成全部 256 个代码
# ============================================================

func all_codes*(): seq[string] =
  for a in ['p', 'q', 'b', 'd']:
    for b in ['p', 'q', 'b', 'd']:
      for c in ['p', 'q', 'b', 'd']:
        for d in ['p', 'q', 'b', 'd']:
          result.add($a & $b & $c & $d)

# ============================================================
# 生成 2×2 方阵
# ============================================================

type
  MirrorGrid2D* = object
    a*: string  # 左上
    b*: string  # 右上
    c*: string  # 左下
    d*: string  # 右下

func make_grid*(s: string): MirrorGrid2D =
  result.a = s
  result.b = reflect(s)
  result.c = reverse_str(s)
  result.d = reflect(reverse_str(s))

func is_proper_grid*(g: MirrorGrid2D): bool =
  ## 四个位置都不相同
  let codes = [g.a, g.b, g.c, g.d]
  for i in 0..3:
    for j in (i+1)..3:
      if codes[i] == codes[j]:
        return false
  return true

func all_proper_grids*(): seq[MirrorGrid2D] =
  let all = all_codes()
  var seen: seq[string] = @[]
  for s in all:
    let g = make_grid(s)
    if is_proper_grid(g):
      # 每4个码只取一个代表（选字典序最小的）
      let repr = min(min(g.a, g.b), min(g.c, g.d))
      if repr notin seen:
        seen.add(repr)
        # 重新从 repr 生成，保证输出一致
        result.add(make_grid(repr))
        # 注意：repr 作为左上时，需要 reflect(repr) = 右上 且 reverse(repr) = 左下
        # 但 repr 不一定是原 s，所以需要从 repr 重新生成网格

func all_proper_grids_by_repr*(): seq[MirrorGrid2D] =
  ## 按左上A为最小代表生成所有完整方阵
  let all = all_codes()
  var seen: seq[string] = @[]
  for s in all:
    let g = make_grid(s)
    if is_proper_grid(g):
      let repr = min(min(g.a, g.b), min(g.c, g.d))
      if repr notin seen:
        seen.add(repr)
        result.add(g)

# ============================================================
# 显示
# ============================================================

proc print_grid*(g: MirrorGrid2D; idx: int) =
  echo "  ┌──────┬──────┐"
  echo "  │ ", g.a, " │ ", g.b, " │"
  echo "  ├──────┼──────┤"
  echo "  │ ", g.c, " │ ", g.d, " │"
  echo "  └──────┴──────┘"
  echo "  reflect(左→右): ", g.a, "→", g.b, "  ", g.c, "→", g.d
  echo "  reverse(上→下): ", g.a, "→", g.c, "  ", g.b, "→", g.d
  echo ""

# ============================================================
# 测试
# ============================================================

when is_main_module:
  let all = all_codes()

  # 统计
  var self_mirror_count = 0
  var palindrome_count = 0
  var proper_grid_count = 0

  for s in all:
    if is_self_mirror(s): self_mirror_count += 1
    if is_palindrome(s): palindrome_count += 1

  # 统计完整网格
  var seen: seq[string] = @[]
  var proper_grids: seq[MirrorGrid2D] = @[]
  for s in all:
    let g = make_grid(s)
    if is_proper_grid(g):
      let repr = min(min(g.a, g.b), min(g.c, g.d))
      if repr notin seen:
        seen.add(repr)
        proper_grids.add(g)

  proper_grid_count = proper_grids.len

  echo "━━━ 满全法·镜像方阵·二维平面 ━━━"
  echo ""
  echo "自镜像: ", self_mirror_count, " 个"
  echo "回文:   ", palindrome_count, " 个"
  echo "完整 2×2 方阵: ", proper_grid_count, " 个"
  echo "总代码: ", all.len, " = 256"
  echo ""
  echo "━━━ 全部 ", proper_grid_count, " 个完整方阵 ━━━"
  echo ""

  for i, g in proper_grids:
    print_grid(g, i + 1)

  echo "━━━ 共 ", proper_grid_count, " 个 2×2 方阵 ━━━"
  echo "  每阵 4 个代码 = ", proper_grid_count * 4, " 个 = 256 全部分配"
