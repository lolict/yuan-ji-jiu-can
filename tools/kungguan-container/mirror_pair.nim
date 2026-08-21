# 满全法·镜像对·自镜像·bpqd编码系统
#
# 核心发现：
#   p ↔ q （镜像）
#   b ↔ d （镜像）
#
# 操作定义：
#   逐字镜像 + 整体反转 = 完整镜像
#
#   wxyz 是自镜像 ⟺ w=M(z) 且 x=M(y)
#   形式：w x M(x) M(w)
#
# 4^4 = 256 组合
#   自镜像：16个（绝对存在）
#   镜像对：120对（关系存在）

import math, strutils, algorithm

type
  # 字母只有这4个是镜像对
  MirrorChar* = enum
    mcP  # 0 p ↔ q
    mcQ  # 1 q ↔ p
    mcB  # 2 b ↔ d
    mcD  # 3 d ↔ b

  # 镜像字符
  MirrorGlyph* = object
    ch*: char           # 实际字符
    mc*: MirrorChar     # 镜像类别

  # 4字母编码组合
  MirrorCode* = object
    glyphs*: array[4, char]   # 4个字母
    is_self*: bool            # 是否自镜像
    pair_id*: int             # 配对ID（自镜像=自身ID）

  # 镜像对
  MirrorPair* = object
    id*: int
    a*: string                # 组合A
    b*: string                # 组合B（A的镜像）
    is_self*: bool            # 是否自镜像对

# ============================================================
# 基础映射
# ============================================================

# 字符 → 镜像类别
func to_mirror_char*(c: char): MirrorChar =
  case c
  of 'p': result = mcP
  of 'q': result = mcQ
  of 'b': result = mcB
  of 'd': result = mcD
  else: result = mcP

# 镜像类别 → 镜像字符
func mirror_of*(mc: MirrorChar): char =
  case mc
  of mcP: result = 'q'
  of mcQ: result = 'p'
  of mcB: result = 'd'
  of mcD: result = 'b'

# 字符的镜像
func mirror_char*(c: char): char =
  result = mirror_of(to_mirror_char(c))

# ============================================================
# 核心操作：逐字镜像 + 反转
# ============================================================

func reflect*(s: string): string =
  ## 完整镜像操作：逐字替换 + 反转
  result = new_string(s.len)
  for i in 0..<s.len:
    result[s.len - 1 - i] = mirror_char(s[i])

# 简化版
func reflect*(s: array[4, char]): array[4, char] =
  for i in 0..3:
    result[3 - i] = mirror_char(s[i])

# ============================================================
# 自镜像检测
# ============================================================

func is_self_mirror*(s: string): bool =
  ## 自镜像: reflect(s) == s
  result = (reflect(s) == s)

func is_self_mirror*(arr: array[4, char]): bool =
  ## 快速检测：w=M(z) 且 x=M(y)
  result = (mirror_char(arr[0]) == arr[3]) and (mirror_char(arr[1]) == arr[2])

# ============================================================
# 生成全部256个组合
# ============================================================

const MIRROR_CHARS*: array[4, char] = ['p', 'q', 'b', 'd']

func generate_all*(): seq[string] =
  ## 生成 4^4 = 256 个组合
  for a in MIRROR_CHARS:
    for b in MIRROR_CHARS:
      for c in MIRROR_CHARS:
        for d in MIRROR_CHARS:
          result.add($a & $b & $c & $d)

func classify*(): tuple[selves: seq[string], pairs: seq[tuple[a, b: string]]] =
  ## 分类：自镜像 vs 镜像对
  let all = generate_all()
  var seen: seq[string] = @[]
  for s in all:
    if s in seen:
      continue
    let r = reflect(s)
    if s == r:
      result.selves.add(s)
      seen.add(s)
    else:
      result.pairs.add((s, r))
      seen.add(s)
      seen.add(r)

# ============================================================
# 自镜像的数学形式：w x M(x) M(w)
# ============================================================

func generate_self_mirrors*(): seq[string] =
  ## 按公式 w x M(x) M(w) 生成16个自镜像
  for w in MIRROR_CHARS:
    for x in MIRROR_CHARS:
      let mw = mirror_char(w)
      let mx = mirror_char(x)
      result.add($w & $x & $mx & $mw)

# ============================================================
# 配对查找
# ============================================================

func find_pair*(s: string): string =
  ## 查找s的镜像配对（如果是自镜像则返回自身）
  result = reflect(s)

func pair_id*(s: string): int =
  ## 计算配对ID
  let r = reflect(s)
  let repr = min(s, r)
  var h = 0
  for c in repr:
    var v = 0
    case c
    of 'p': v = 0
    of 'q': v = 1
    of 'b': v = 2
    of 'd': v = 3
    else: v = 0
    h = h * 4 + v
  result = h

# ============================================================
# 自镜像的语义（满全法映射）
# ============================================================

func self_mirror_meaning*(s: string): string =
  ## 自镜像编码的语义注释
  case s
  of "ppqq": result = "纯阳双生（全p全q，无杂质）"
  of "pqpq": result = "阳动交替（p-q-p-q，波动）"
  of "qpqp": result = "阴动交替（q-p-q-p，逆波动）"
  of "qqpp": result = "纯阴双生（全q全p，逆纯）"
  of "bbdd": result = "地双生（全b全d，水平扩展）"
  of "bdbd": result = "地交替（b-d-b-d，潮汐）"
  of "dbdb": result = "地逆交替（d-b-d-b，逆潮汐）"
  of "ddbb": result = "地逆双生（全d全b，逆水平）"
  of "bpqd": result = "水波完整（b→p→q→d，顺时针）"
  of "bqpd": result = "水波逆完整（b→q→p→d，逆时针）"
  of "pbdq": result = "升沉完整（p→b→d→q，螺旋升）"
  of "pdbq": result = "升沉逆完整（p→d→b→q，螺旋降）"
  of "dpqb": result = "逆升完整（d→p→q→b，逆螺旋升）"
  of "dqpb": result = "逆升逆完整（d→q→p→b，逆螺旋降）"
  else: result = "自镜像（绝对存在）"

# 修正注释：上面代码里有中文混入，重写如下
func meaning*(s: string): string =
  case s
  of "ppqq": result = "pure_yang_twin"
  of "pqpq": result = "yang_wave"
  of "qpqp": result = "yin_wave"
  of "qqpp": result = "pure_yin_twin"
  of "bbdd": result = "earth_twin"
  of "bdbd": result = "tide"
  of "dbdb": result = "reverse_tide"
  of "ddbb": result = "earth_inverse_twin"
  of "bpqd": result = "wave_clockwise"
  of "bqpd": result = "wave_counter"
  of "pbdq": result = "spiral_up"
  of "pdbq": result = "spiral_down"
  of "dpqb": result = "spiral_inverse_up"
  of "dqpb": result = "spiral_inverse_down"
  else: result = "self_mirror_absolute"

# ============================================================
# 配对对的语义（满全法映射）
# ============================================================

func pair_meaning*(a, b: string): string =
  ## 镜像对的关系语义
  let pair_repr = min(a, b)
  case pair_repr
  of "bbbp": result = "水波起手（b-b-b-p ↔ q-d-d-d）"
  of "bbbb": result = "大地四隅（b-b-b-b ↔ d-d-d-d）"
  of "pppp": result = "天穹四正（p-p-p-p ↔ q-q-q-q）"
  else:
    result = "mirror_pair_" & pair_repr

# ============================================================
# 与汉字相位的桥梁：镜像编码 → 汉字
# ============================================================

func mirror_to_hanzi*(code: string): string =
  ## 将4字母镜像编码映射到汉字相位表的一个字
  ## 涌现规则：编码的自镜像性 → 极性，首字母 → 密度修正
  if is_self_mirror(code):
    # 自镜像 = 绝对存在 = 由结构类型决定
    case code[0]
    of 'p', 'q': result = "天"  # 天系 = 表面层
    of 'b', 'd': result = "地"  # 地系 = 骨架层
    else: result = "中"
  else:
    # 需要配对 = 关系存在 = 由关系极性决定
    case code[0]
    of 'p': result = "直"
    of 'q': result = "曲"
    of 'b': result = "柔"
    of 'd': result = "刚"
    else: result = "中"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 满全法·镜像对·bpqd编码系统 ==="
  echo ""

  # 1. 基础操作验证
  echo "【镜像操作验证】"
  for s in ["bpqd", "pdbq", "bqbq", "pdpd", "ppqq", "bbdd"]:
    let r = reflect(s)
    echo "  reflect(", s, ") = ", r, " ", (if s == r: "✓ 自镜像" else: "↔ 配对")
  echo ""

  # 2. 自镜像完整列表
  echo "【自镜像组合（16个）- 绝对存在】"
  let selves = generate_self_mirrors()
  var sorted_selves = selves
  sorted_selves.sort()
  for i, s in sorted_selves:
    echo "  ", align($(i+1), 2), ". ", s, "  (极性=绝对, meaning=", meaning(s), ")"
  echo ""

  # 3. 验证自镜像数量
  echo "【分类统计】"
  let (all_selves, all_pairs) = classify()
  echo "  自镜像: ", all_selves.len, " 个"
  echo "  镜像对: ", all_pairs.len, " 对"
  echo "  总计: ", all_selves.len + all_pairs.len * 2, " = 256"
  echo ""

  # 4. 全部镜像对
  echo "【全部镜像对（120对）- 关系存在】"
  for i in 0..<all_pairs.len:
    let pr = all_pairs[i]
    echo "  ", align($(i+1), 3), ". ", pr.a, " ↔ ", pr.b
  echo ""

  # 5. 编码→汉字映射示例
  echo "【镜像编码 → 汉字相位】"
  for code in ["ppqq", "pqpq", "pdbq", "bpqd", "bbbb", "pppq"]:
    let hanzi = mirror_to_hanzi(code)
    echo "  ", code, " → ", hanzi
  echo ""

  # 6. 与满全法的极性对应
  echo "【极性对应】"
  echo "  自镜像 (16个): 绝对存在 = 主权独立 = 不需要外部承认"
  echo "  镜像对 (120对): 关系存在 = 需要配对 = 主权从属于关系"
  echo ""

  # 7. 数学性质
  echo "【数学性质】"
  echo "  字母池: {p, q, b, d}"
  echo "  编码长度: 4"
  echo "  总组合: 4^4 = 256"
  echo "  自镜像: 4×4 = 16 (w x M(x) M(w) 形式)"
  echo "  配对: (256-16)/2 = 120 对"
  echo ""

  echo "【用途】"
  echo "  bpqd编码 = 最小的镜像封闭字母系统"
  echo "  自镜像 = 主权证书（自身即完整）"
  echo "  镜像对 = 关系合约（双方互相定义）"
  echo "  可用来做：方向编码、对称性判定、产权边界、身份认证"
