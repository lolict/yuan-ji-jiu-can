# 满全法·感知场·码位连线·隐藏式范畴
#
# 核心命题：
#   感知场 = mlll毛利兰容器 = 三维全息进制空间
#   既是前端也是后端，更是感知空间
#
#   码位 = 点（不是括号，是点标签）
#   线 = 默认存在，不启动也存在（隐藏式）
#   谁在前面谁主导，谁在后面谁被主导
#   线不存储为独立对象，通过码位前后关系计算得出
#   范畴 = 多个码位连线形成的闭合区域 = 产权空间

type
  # ============================================================
  # 码位点（不是括号，是点标签）
  # ============================================================
  CodePoint* = object
    sm*: int       # 声母码位
    ym*: int       # 韵母码位
    sd*: int       # 声调码位
    ph*: int       # 相位码位
    lv*: int       # 层级码位

  # ============================================================
  # 码位关系（谁主导谁）
  #   主导方写在前面，被主导方写在后面
  #   线默认存在，不启动也存在（隐藏式）
  # ============================================================
  CodeRelation* = object
    dominant*: CodePoint    # 主导方（写在前面）
    subordinate*: CodePoint  # 被主导方（写在后面）
    # 线不存储！通过dominant和subordinate的关系计算得出

  # ============================================================
  # 感知场（mlll毛利兰容器）
  #   三维全息进制空间
  #   既是前端也是后端，更是感知空间
  # ============================================================
  PerceptionField* = object
    points*: seq[CodePoint]        # 所有码位点
    relations*: seq[CodeRelation]  # 所有关系（隐藏式连线）
    dimension*: tuple[w, h, d: int] # 三维尺寸

# ============================================================
# 码位操作
# ============================================================

# 创建码位点
func new_point*(sm, ym, sd: int; ph: int; lv: int): CodePoint =
  result.sm = sm mod 48
  result.ym = ym mod 66
  result.sd = sd mod 60
  result.ph = ph mod 18
  result.lv = lv mod 5

# 码位相等
func point_equal*(a, b: CodePoint): bool =
  result = (a.sm == b.sm) and (a.ym == b.ym) and
           (a.sd == b.sd) and (a.ph == b.ph) and (a.lv == b.lv)

# ============================================================
# 关系操作（隐藏式连线）
# ============================================================

# 创建关系（主导→被主导）
# 线默认存在，不启动也存在
func new_relation*(dominant, subordinate: CodePoint): CodeRelation =
  result.dominant = dominant
  result.subordinate = subordinate
  # 线不存储！通过前后关系计算得出

# 判断两点之间是否有连线（隐藏式）
# 有连线 = 存在关系（dominant在前，subordinate在后）
func has_line*(field: PerceptionField; a, b: CodePoint): bool =
  for r in field.relations:
    if point_equal(r.dominant, a) and point_equal(r.subordinate, b):
      result = true
      return
  result = false

# 获取连线方向（谁主导谁）
func line_direction*(field: PerceptionField; a, b: CodePoint): string =
  for r in field.relations:
    if point_equal(r.dominant, a) and point_equal(r.subordinate, b):
      result = "A主导B"
      return
    if point_equal(r.dominant, b) and point_equal(r.subordinate, a):
      result = "B主导A"
      return
  result = "无连线"

# ============================================================
# 范畴操作（闭合区域 = 产权空间）
# ============================================================

# 判断点是否在范畴内（闭合区域内）
# 范畴 = 多个码位连线形成的闭合区域
func inside_category*(field: PerceptionField; p: CodePoint;
                     category: seq[CodePoint]): bool =
  # 简单判断：点是否在范畴的边界码位范围内
  if category.len < 3:
    result = false
    return
  # 计算范畴的边界
  var min_sm = 999
  var max_sm = 0
  for cp in category:
    if cp.sm < min_sm: min_sm = cp.sm
    if cp.sm > max_sm: max_sm = cp.sm
  result = (p.sm >= min_sm) and (p.sm <= max_sm)

# 计算范畴的产权空间大小
func category_volume*(category: seq[CodePoint]): int =
  if category.len < 3:
    result = 0
    return
  # 计算闭合区域的体积
  var min_sm = 999
  var max_sm = 0
  var min_ph = 999
  var max_ph = 0
  var min_lv = 999
  var max_lv = 0
  for cp in category:
    if cp.sm < min_sm: min_sm = cp.sm
    if cp.sm > max_sm: max_sm = cp.sm
    if cp.ph < min_ph: min_ph = cp.ph
    if cp.ph > max_ph: max_ph = cp.ph
    if cp.lv < min_lv: min_lv = cp.lv
    if cp.lv > max_lv: max_lv = cp.lv
  let w = max_sm - min_sm + 1
  let h = max_ph - min_ph + 1
  let d = max_lv - min_lv + 1
  result = w * h * d

# ============================================================
# 感知场操作
# ============================================================

# 创建感知场
func new_field*(w: int; h: int; d: int): PerceptionField =
  result.points = new_seq[CodePoint](0)
  result.relations = new_seq[CodeRelation](0)
  result.dimension = (w, h, d)

# 添加码位点
func add_point*(field: var PerceptionField; p: CodePoint) =
  field.points.add(p)

# 添加关系（隐藏式连线）
func add_relation*(field: var PerceptionField; dominant, subordinate: CodePoint) =
  field.relations.add(new_relation(dominant, subordinate))
  # 线不存储！只存储前后关系

# 获取所有连线（隐藏式，计算得出）
func get_all_lines*(field: PerceptionField): seq[tuple[a, b: CodePoint]] =
  result = new_seq[tuple[a, b: CodePoint]](0)
  for r in field.relations:
    result.add((r.dominant, r.subordinate))

# ============================================================
# 感知场 = 前端 + 后端 + 感知空间
# ============================================================

func field_nature*(): string =
  result = "感知场的本质：\n"
  result &= "  mlll毛利兰容器 = 三维全息进制空间\n"
  result &= "  静态标记空间 = 毛利兰的能力\n"
  result &= "  \n"
  result &= "  既是前端：显示、交互、感知\n"
  result &= "  也是后端：计算、存储、逻辑\n"
  result &= "  更是感知空间：三维立体的存在\n"
  result &= "  \n"
  result &= "  不是传统的前端或后端\n"
  result &= "  是以感知场为载体的容器化解决方案\n"

# ============================================================
# 隐藏式连线的实现原理
# ============================================================

func hidden_line_principle*(): string =
  result = "隐藏式连线原理：\n"
  result &= "  1. 不存储线对象（不占内存）\n"
  result &= "  2. 只存储码位的前后关系\n"
  result &= "  3. 线通过计算得出：如果A在B前面，则A→B有连线\n"
  result &= "  4. 启动时：关系存在，线存在\n"
  result &= "  5. 不启动时：关系不存在，线不存在\n"
  result &= "  6. 隐藏式：线不显示，但逻辑上存在\n"
  result &= "  \n"
  result &= "  代码写法：\n"
  result &= "    add_relation(field, A, B)  // A主导B\n"
  result &= "    // 线自动存在，无需显式创建\n"
  result &= "    // has_line(field, A, B) 返回 true\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 感知场·码位连线·隐藏式范畴 ==="
  echo ""

  # 创建感知场
  echo "【感知场（mlll毛利兰容器）】"
  var field = new_field(18, 5, 8)
  echo "  三维尺寸: ", field.dimension.w, "x", field.dimension.h, "x", field.dimension.d
  echo "  性质: 既是前端也是后端，更是感知空间"
  echo ""

  # 创建码位点
  echo "【码位点（不是括号，是点标签）】"
  let point_a = new_point(1, 0, 0, 0, 0)   # b-a-阴平-东-比特流
  let point_b = new_point(2, 1, 1, 1, 1)   # m-o-阳平-西-二进制
  let point_c = new_point(3, 2, 2, 2, 2)   # f-e-上声-南-符号
  let point_d = new_point(4, 3, 3, 3, 3)   # d-i-去声-北-容器

  add_point(field, point_a)
  add_point(field, point_b)
  add_point(field, point_c)
  add_point(field, point_d)

  echo "  点A: 声母1 韵母0 声调0 相位0 层级0"
  echo "  点B: 声母2 韵母1 声调1 相位1 层级1"
  echo "  点C: 声母3 韵母2 声调2 相位2 层级2"
  echo "  点D: 声母4 韵母3 声调3 相位3 层级3"
  echo ""

  # 创建关系（隐藏式连线）
  echo "【关系（隐藏式连线）】"
  echo "  规则：谁在前面谁主导，谁在后面谁被主导"
  echo "  线默认存在，不启动也存在"
  echo ""

  add_relation(field, point_a, point_b)  # A主导B
  add_relation(field, point_b, point_c)  # B主导C
  add_relation(field, point_c, point_d)  # C主导D
  add_relation(field, point_d, point_a)  # D主导A（形成闭合）

  echo "  关系1: A→B (A主导B)"
  echo "  关系2: B→C (B主导C)"
  echo "  关系3: C→D (C主导D)"
  echo "  关系4: D→A (D主导A，形成闭合)"
  echo ""

  # 连线检测
  echo "【连线检测（隐藏式，计算得出）】"
  echo "  A-B有连线: ", has_line(field, point_a, point_b), " (A主导B)"
  echo "  B-C有连线: ", has_line(field, point_b, point_c), " (B主导C)"
  echo "  C-D有连线: ", has_line(field, point_c, point_d), " (C主导D)"
  echo "  D-A有连线: ", has_line(field, point_d, point_a), " (D主导A)"
  echo "  A-C有连线: ", has_line(field, point_a, point_c), " (无直接关系)"
  echo ""

  # 连线方向
  echo "【连线方向（谁主导谁）】"
  echo "  A→B: ", line_direction(field, point_a, point_b)
  echo "  B→A: ", line_direction(field, point_b, point_a)
  echo "  B→C: ", line_direction(field, point_b, point_c)
  echo ""

  # 范畴（闭合区域 = 产权空间）
  echo "【范畴（闭合区域 = 产权空间）】"
  let category = @[point_a, point_b, point_c, point_d]
  let vol = category_volume(category)
  echo "  范畴边界: A-B-C-D（四边形闭合）"
  echo "  产权空间体积: ", vol
  echo "  管辖: 范畴内所有三维全息进制立体空间"
  echo ""

  # 范畴内外检测
  echo "【范畴内外检测】"
  let test_inside = new_point(2, 1, 1, 1, 1)  # 范畴内
  let test_outside = new_point(10, 10, 10, 10, 4)  # 范畴外
  echo "  点(2,1,1,1,1)在范畴内: ", inside_category(field, test_inside, category)
  echo "  点(10,10,10,10,4)在范畴内: ", inside_category(field, test_outside, category)
  echo ""

  # 感知场本质
  echo field_nature()
  echo ""

  # 隐藏式连线原理
  echo hidden_line_principle()
  echo ""

  echo "【总结】"
  echo "  1. 码位 = 点标签，不是括号"
  echo "  2. 线 = 默认存在，不启动也存在（隐藏式）"
  echo "  3. 谁在前面谁主导，谁在后面谁被主导"
  echo "  4. 线不存储为独立对象，通过前后关系计算得出"
  echo "  5. 范畴 = 闭合区域 = 产权空间"
  echo "  6. 感知场 = 前端 + 后端 + 感知空间"
  echo "  -> 隐藏式连线：关系存在则线存在，无需显式创建"
