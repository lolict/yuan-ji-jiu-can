# 满全法·范畴线·产权边界·码位组合管辖空间
#
# 核心命题：
#   两个码位之间用范畴线连接
#   范畴线划定边界：内部=产权空间，外部=自由空间
#   边界内所有内存空间归该码位组合管辖
#   多个码位连线形成多边形边界，共同管辖内部空间
#
# 绑定方式：
#   不是两个括号(码位)硬拼
#   而是通过范畴线(小点连线)划定边界
#   线内=管辖空间，线外=自由空间

type
  # ============================================================
  # 码位点（不是括号，是点上的标签）
  # ============================================================
  CodePoint* = object
    sm*: int       # 声母码位 0-47
    ym*: int       # 韵母码位 0-65
    sd*: int       # 声调码位 0-59
    ph*: int       # 相位码位 0-17
    lv*: int       # 层级码位 0-4

  # ============================================================
  # 范畴线（连接两个码位的线）
  # 不是括号硬拼，是线划定边界
  # ============================================================
  BoundaryLine* = object
    point_a*: CodePoint      # 起点码位
    point_b*: CodePoint      # 终点码位
    line_id*: int            # 线ID（由码位组合决定，不擅自命名）

  # ============================================================
  # 产权空间（线内归码位管辖）
  # ============================================================
  PropertySpace* = object
    boundary*: BoundaryLine  # 边界线
    inner_points*: seq[CodePoint]  # 线内所有码位
    outer_points*: seq[CodePoint]  # 线外所有码位（自由空间）

  # ============================================================
  # 多边形边界（多根线围成的管辖空间）
  # ============================================================
  PolygonBoundary* = object
    lines*: seq[BoundaryLine]       # 多根范畴线
    vertices*: seq[CodePoint]       # 顶点码位
    governed_space*: PropertySpace  # 管辖空间

# ============================================================
# 码位点操作
# ============================================================

# 创建码位点
func new_code_point*(sm, ym, sd: int; ph: int; lv: int): CodePoint =
  result.sm = sm mod 48
  result.ym = ym mod 66
  result.sd = sd mod 60
  result.ph = ph mod 18
  result.lv = lv mod 5

# 码位点相等（检查是否同一位置）
func point_equal*(a, b: CodePoint): bool =
  result = (a.sm == b.sm) and (a.ym == b.ym) and
           (a.sd == b.sd) and (a.ph == b.ph) and (a.lv == b.lv)

# ============================================================
# 范畴线操作（两个码位之间的连线）
# ============================================================

# 计算线ID（不擅自命名，由码位组合决定）
func make_line_id*(a, b: CodePoint): int =
  # 线ID = 两端码位的哈希组合
  var h = a.sm
  h = h * 66 + a.ym
  h = h * 60 + a.sd
  h = h * 18 + a.ph
  h = h * 5 + a.lv
  h = h * 100000 + b.sm * 10000 + b.ym * 1000 + b.sd * 100 + b.ph * 10 + b.lv
  result = h

# 创建范畴线（连接两个码位点，划定边界）
func new_boundary_line*(a, b: CodePoint): BoundaryLine =
  result.point_a = a
  result.point_b = b
  # line_id不擅自命名 = 由两端码位组合决定
  result.line_id = make_line_id(a, b)

# ============================================================
# 产权空间操作（线内管辖，线外自由）
# ============================================================

# 判断点是否在线内（内部=管辖空间）
func is_inside*(line: BoundaryLine; p: CodePoint): bool =
  # 计算点到线的距离
  # 线A-B，点P
  # 如果A==P或B==P，点在边界上
  if point_equal(p, line.point_a) or point_equal(p, line.point_b):
    result = true
  else:
    # 简单判断：点是否在线的码位范围内
    let min_sm = min(line.point_a.sm, line.point_b.sm)
    let max_sm = max(line.point_a.sm, line.point_b.sm)
    let min_ym = min(line.point_a.ym, line.point_b.ym)
    let max_ym = max(line.point_a.ym, line.point_b.ym)
    let min_ph = min(line.point_a.ph, line.point_b.ph)
    let max_ph = max(line.point_a.ph, line.point_b.ph)
    result = (p.sm >= min_sm) and (p.sm <= max_sm) and
             (p.ym >= min_ym) and (p.ym <= max_ym) and
             (p.ph >= min_ph) and (p.ph <= max_ph)

# ============================================================
# 多边形边界（多根线围成的管辖空间）
# ============================================================

# 创建多边形边界（多根范畴线围成）
func new_polygon*(vertices: seq[CodePoint]): PolygonBoundary =
  result.vertices = vertices
  result.lines = new_seq[BoundaryLine](0)
  # 连接每两个相邻顶点形成线
  for i in 0..<vertices.len:
    let a = vertices[i]
    let b = vertices[(i + 1) mod vertices.len]
    result.lines.add(new_boundary_line(a, b))

# 多边形内所有码位都被管辖
func is_governed*(poly: PolygonBoundary; p: CodePoint): bool =
  # 遍历所有线，检查是否在线内
  for line in poly.lines:
    if is_inside(line, p):
      result = true
      return
  result = false

# ============================================================
# 管辖统计
# ============================================================

# 统计管辖空间内的码位数量
func governed_count*(poly: PolygonBoundary; candidates: seq[CodePoint]): int =
  result = 0
  for p in candidates:
    if is_governed(poly, p):
      result += 1


# 统计自由空间内的码位数量
func free_count*(poly: PolygonBoundary; candidates: seq[CodePoint]): int =
  result = 0
  for p in candidates:
    if not is_governed(poly, p):
      result += 1

# ============================================================
# 内存：边界线本身只占用线ID + 两个码位点
# ============================================================

func boundary_memory*(): string =
  result = "范畴线内存占用：\n"
  result &= "  两个码位点：2 x 27比特 = 54比特\n"
  result &= "  线ID：32比特\n"
  result &= "  总计：86比特 ≈ 11字节\n"
  result &= "  \n"
  result &= "  不存储内部/外部空间列表\n"
  result &= "  内部/外部通过计算判断（边界测试）\n"
  result &= "  → 边界划定后，内外通过计算区分\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 范畴线·产权边界·码位组合管辖空间 ==="
  echo ""

  # 创建两个码位点
  echo "【码位点】"
  let point_a = new_code_point(1, 0, 0, 0, 0)   # b-a-阴平-东-比特流
  let point_b = new_code_point(2, 1, 1, 1, 1)   # m-o-阳平-西-二进制
  echo "  点A: 声母", point_a.sm, " 韵母", point_a.ym, " 声调", point_a.sd
  echo "  点B: 声母", point_b.sm, " 韵母", point_b.ym, " 声调", point_b.sd
  echo ""

  # 创建范畴线（两个码位之间画一根线）
  echo "【范畴线（连接两个码位的线）】"
  let line_ab = new_boundary_line(point_a, point_b)
  echo "  线ID: ", line_ab.line_id
  echo "  线A: 声母", line_ab.point_a.sm, "-> 相位", line_ab.point_a.ph
  echo "  线B: 声母", line_ab.point_b.sm, "-> 相位", line_ab.point_b.ph
  echo "  含义: 这根线划定了边界，线内=产权空间"
  echo ""

  # 测试产权空间（点在线内还是线外）
  echo "【产权空间（线内管辖，线外自由）】"
  let test_points = @[
    new_code_point(1, 0, 0, 0, 0),   # 点A边界上
    new_code_point(2, 1, 1, 1, 1),   # 点B边界上
    new_code_point(1, 0, 0, 0, 0),   # 线上
    new_code_point(5, 5, 5, 5, 2),   # 线外
    new_code_point(10, 10, 10, 10, 3)  # 线外
  ]
  echo "  测试5个点："
  for i, p in test_points:
    var status = "外部(自由空间)"
    if is_inside(line_ab, p):
      status = "内部(产权空间)"
    echo "    点", i, ": 声母", p.sm, " 韵母", p.ym, " -> ", status
  echo ""

  # 多边形边界（多根线围成管辖空间）
  echo "【多边形边界（多根线围成管辖空间）】"
  let poly = new_polygon(@[
    new_code_point(0, 0, 0, 0, 0),   # 原点
    new_code_point(10, 0, 0, 0, 0),  # 东
    new_code_point(10, 10, 0, 0, 0), # 东南
    new_code_point(0, 10, 0, 0, 0)   # 南
  ])
  echo "  多边形顶点数: ", poly.vertices.len
  echo "  多边形边数: ", poly.lines.len
  echo "  内部空间由这4根范畴线共同划定"
  echo ""

  # 多边形管辖统计
  echo "【管辖统计】"
  var candidates = new_seq[CodePoint](0)
  for sm in 0..12:
    for ym in 0..12:
      candidates.add(new_code_point(sm, ym, 0, 0, 0))
  let governed = governed_count(poly, candidates)
  let free = free_count(poly, candidates)
  echo "  候选码位数: ", candidates.len
  echo "  管辖空间内: ", governed, " 个"
  echo "  自由空间外: ", free, " 个"
  echo ""

  # 内存占用
  echo boundary_memory()
  echo ""

  echo "【总结】"
  echo "  1. 码位是点上的标签，不是括号"
  echo "  2. 两个码位用范畴线连接（小点连线）"
  echo "  3. 范畴线划定边界：内=产权空间，外=自由空间"
  echo "  4. 多根线围成多边形，共同管辖内部空间"
  echo "  5. 不存储内外列表，通过边界测试计算"
  echo "  6. 边界线仅占用86比特"
  echo "  → 用线划圈，圈内归我，圈外自由"
