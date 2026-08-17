# 满全法·框架容器·几何边界·产权空间
#
# 核心命题：
#   一个可自定义几边形的容器空间
#   预加载的框架：边数、节点数、角度、长宽高
#   光标移动框架 → 放置在目标范畴 → 即完成产权包含
#   框架容器 ≠ 传统容器
#   传统容器 = 包含比特流/二进制/编码的容器
#   框架容器 = 只划定边界的容器（内归我，外自由）
#   框架容器的包含资格 ≠ 传统容器的包含资格
#   框架容器有权限范畴：只能管辖自身边界内的产权

type
  # ============================================================
  # 传统容器（包含所有比特流/二进制/编码的容器）
  # ============================================================
  TraditionalContainer* = object
    all_bits*: int64         # 所有比特流
    all_binary*: int64       # 所有二进制
    all_encoding*: seq[int]  # 所有编码
    all_bases*: seq[int]     # 所有进制

  # ============================================================
  # 框架容器（只划定边界的容器，不存储内容）
  # ============================================================
  ShellContainer* = object
    edge_count*: int         # 边数（几边形）
    node_count*: int         # 节点数
    node_angles*: seq[int]   # 每个节点的角度
    width*: int              # X轴长度（相位单位）
    height*: int             # Y轴长度（层级单位）
    depth*: int              # Z轴长度（进制单位）
    vertices*: seq[int]      # 每个顶点对应的码位ID

  # ============================================================
  # 产权权限（框架容器独有的权限）
  # ============================================================
  PropertyAuth* = object
    can_govern_traditional*: bool  # 能否管辖传统容器内容
    can_govern_boundary*: bool     # 能否管辖边界内产权
    can_extend_outward*: bool      # 能否扩展管辖范围
    govern_depth*: int             # 管辖深度（层级数）
    govern_base*: int              # 管辖进制范围

  # ============================================================
  # 权限范畴（框架容器能管什么）
  # ============================================================
  GovernScope* = object
    scope_name*: string
    allowed_bases*: seq[int]       # 允许管辖的进制
    allowed_levels*: seq[int]      # 允许管辖的层级
    allowed_phases*: seq[int]      # 允许管辖的相位

  # ============================================================
  # 预加载框架模板（编译时常量，不占运行时内存）
  # ============================================================
  ShellTemplate* = object
    label*: string
    edges*: int
    nodes*: int
    angles*: seq[int]
    size*: tuple[w, h, d: int]

# ============================================================
# 编译时常量：预加载框架模板
# ============================================================

const PRESET_SHELLS*: array[5, ShellTemplate] = [
  ShellTemplate(label: "三边形", edges: 3, nodes: 3,
                angles: @[60, 60, 60], size: (4, 4, 4)),
  ShellTemplate(label: "四边形", edges: 4, nodes: 4,
                angles: @[90, 90, 90, 90], size: (8, 8, 4)),
  ShellTemplate(label: "五边形", edges: 5, nodes: 5,
                angles: @[72, 72, 72, 72, 72], size: (10, 10, 5)),
  ShellTemplate(label: "六边形", edges: 6, nodes: 6,
                angles: @[60, 60, 60, 60, 60, 60], size: (12, 12, 6)),
  ShellTemplate(label: "八边形", edges: 8, nodes: 8,
                angles: @[45, 45, 45, 45, 45, 45, 45, 45], size: (16, 16, 8))
]

# ============================================================
# 框架容器操作
# ============================================================

# 创建框架容器（从模板实例化）
func new_shell*(src: ShellTemplate): ShellContainer =
  result.edge_count = src.edges
  result.node_count = src.nodes
  result.node_angles = src.angles
  result.width = src.size.w
  result.height = src.size.h
  result.depth = src.size.d
  result.vertices = new_seq[int](src.nodes)
  for i in 0..<src.nodes:
    result.vertices[i] = i

# 自定义框架容器（手动设定参数）
func custom_shell*(edges: int; nodes: int; angles: seq[int];
                 w: int; h: int; d: int): ShellContainer =
  result.edge_count = edges
  result.node_count = nodes
  result.node_angles = angles
  result.width = w
  result.height = h
  result.depth = d
  result.vertices = new_seq[int](nodes)

# ============================================================
# 框架容器的空间体积（管辖范围大小）
# ============================================================

# 计算框架的管辖体积（宽×高×深）
func govern_volume*(shell: ShellContainer): int =
  result = shell.width * shell.height * shell.depth

# 计算框架的节点密度（每单位体积的节点数）
func node_density*(shell: ShellContainer): float =
  if govern_volume(shell) > 0:
    result = shell.node_count.float / govern_volume(shell).float

# ============================================================
# 权限区分：框架容器 vs 传统容器
# ============================================================

# 创建权限（默认：不能扩展管辖，只能管边界内）
func default_auth*(): PropertyAuth =
  result.can_govern_traditional = false  # 不管辖传统容器内容
  result.can_govern_boundary = true      # 管辖边界内产权
  result.can_extend_outward = false      # 不能扩展范围
  result.govern_depth = 1               # 管辖深度1层
  result.govern_base = 1                # 管辖1种进制

# 超级权限（能管所有内容）
func super_auth*(): PropertyAuth =
  result.can_govern_traditional = true
  result.can_govern_boundary = true
  result.can_extend_outward = true
  result.govern_depth = 999
  result.govern_base = 999

# 判断框架容器能否管辖传统容器
func can_govern*(auth: PropertyAuth; target: TraditionalContainer): bool =
  result = auth.can_govern_traditional

# ============================================================
# 权限范畴检查
# ============================================================

# 创建权限范畴
func new_scope*(name: string; bases: seq[int]; levels: seq[int]; phases: seq[int]): GovernScope =
  result.scope_name = name
  result.allowed_bases = bases
  result.allowed_levels = levels
  result.allowed_phases = phases

# 检查进制是否在权限范围内
func allow_base*(scope: GovernScope; base_type: int): bool =
  for b in scope.allowed_bases:
    if b == base_type:
      result = true
      return
  result = false

# 检查层级是否在权限范围内
func allow_level*(scope: GovernScope; level: int): bool =
  for l in scope.allowed_levels:
    if l == level:
      result = true
      return
  result = false

# ============================================================
# 管辖判定（核心函数）
# ============================================================

# 框架是否管辖该坐标点
func is_governed*(shell: ShellContainer; auth: PropertyAuth;
                 x: int; y: int; z: int): bool =
  # 1. 边界检查：点在框架边界内
  let in_boundary = (x >= 0) and (x < shell.width) and
                    (y >= 0) and (y < shell.height) and
                    (z >= 0) and (z < shell.depth)
  # 2. 权限检查：框架有管辖权限
  let has_auth = auth.can_govern_boundary
  result = in_boundary and has_auth

# ============================================================
# 内存：框架容器只占用边界参数，不存储内容
# ============================================================

func shell_memory*(): string =
  result = "框架容器内存占用：\n"
  result &= "  边数+节点数：2 x 16比特 = 32比特\n"
  result &= "  角度序列：N x 8比特（N=节点数）\n"
  result &= "  宽×高×深：3 x 16比特 = 48比特\n"
  result &= "  顶点码位：N x 32比特\n"
  result &= "  \n"
  result &= "  不存储！不管辖传统容器的内容\n"
  result &= "  只存储边界参数和顶点码位引用\n"
  result &= "  → 框架是边界定义器，不是内容存储器\n"

func auth_memory*(): string =
  result = "权限区分：\n"
  result &= "  传统容器 = 包含比特流/二进制/编码\n"
  result &= "  框架容器 = 只划定边界（产权空间）\n"
  result &= "  \n"
  result &= "  框架容器的权限：\n"
  result &= "    can_govern_traditional: 是否管内容\n"
  result &= "    can_govern_boundary: 是否管边界内\n"
  result &= "    can_extend_outward: 是否可外扩\n"
  result &= "  \n"
  result &= "  默认：只管边界内，不管传统内容\n"
  result &= "  超级权限：两者都管\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 框架容器·几何边界·产权空间 ==="
  echo ""

  # 预加载框架
  echo "【预加载框架模板】"
  for s in PRESET_SHELLS:
    let vol = s.size.w * s.size.h * s.size.d
    echo "  ", s.label, ": ", s.edges, "边 ", s.nodes, "节点 体积=", vol
  echo ""

  # 创建框架容器
  echo "【创建四边形框架容器】"
  let quad_shell = new_shell(PRESET_SHELLS[1])
  echo "  边数: ", quad_shell.edge_count
  echo "  节点数: ", quad_shell.node_count
  echo "  角度: ", quad_shell.node_angles
  echo "  尺寸: ", quad_shell.width, "x", quad_shell.height, "x", quad_shell.depth
  echo "  管辖体积: ", govern_volume(quad_shell)
  echo ""

  # 自定义框架
  echo "【自定义七边形框架】"
  let hept_shell = custom_shell(7, 7, @[51,51,51,51,51,51,7], 14, 14, 7)
  echo "  边数: ", hept_shell.edge_count
  echo "  节点数: ", hept_shell.node_count
  echo "  尺寸: ", hept_shell.width, "x", hept_shell.height, "x", hept_shell.depth
  echo "  管辖体积: ", govern_volume(hept_shell)
  echo ""

  # 权限区分
  echo "【权限区分】"
  let basic = default_auth()
  let super = super_auth()
  echo "  默认权限：管边界=", basic.can_govern_boundary, " 管内容=", basic.can_govern_traditional
  echo "  超级权限：管边界=", super.can_govern_boundary, " 管内容=", super.can_govern_traditional
  echo ""

  # 管辖判定
  echo "【管辖判定】"
  echo "  框架尺寸: 8x8x4"
  echo "  (0,0,0)管辖: ", is_governed(quad_shell, basic, 0, 0, 0)
  echo "  (4,4,2)管辖: ", is_governed(quad_shell, basic, 4, 4, 2)
  echo "  (7,7,3)管辖: ", is_governed(quad_shell, basic, 7, 7, 3)
  echo "  (8,8,4)管辖: ", is_governed(quad_shell, basic, 8, 8, 4)
  echo "  (-1,0,0)管辖: ", is_governed(quad_shell, basic, -1, 0, 0)
  echo ""

  # 权限范畴
  echo "【权限范畴】"
  let scope = new_scope("基础管辖", @[0,1,2], @[0,1,2], @[0,1,2,3,4,5])
  echo "  范畴名: ", scope.scope_name
  echo "  进制0允许: ", allow_base(scope, 0)
  echo "  进制5允许: ", allow_base(scope, 5)
  echo "  层级1允许: ", allow_level(scope, 1)
  echo "  层级4允许: ", allow_level(scope, 4)
  echo ""

  # 传统容器 vs 框架容器
  echo "【框架容器 vs 传统容器】"
  var trad: TraditionalContainer
  trad.all_bits = 0
  trad.all_binary = 0
  trad.all_encoding = @[]
  trad.all_bases = @[]
  echo "  传统容器包含：比特流、二进制、编码"
  echo "  框架容器：只划定边界，不存储内容"
  echo "  默认权限下框架能管传统容器: ", can_govern(basic, trad)
  echo "  超级权限下框架能管传统容器: ", can_govern(super, trad)
  echo ""

  echo shell_memory()
  echo ""

  echo auth_memory()
  echo ""

  echo "【总结】"
  echo "  1. 框架容器 = 几何边界定义器（自定义几边形）"
  echo "  2. 预加载模板：三边/四边/五边/六边/八边形"
  echo "  3. 光标移动框架 → 放置在目标 → 完成产权包含"
  echo "  4. 框架 ≠ 传统容器（框架不管内容，只管边界）"
  echo "  5. 权限分两层：边界内产权 / 边界外自由"
  echo "  6. 默认只管边界内，超级权限管所有内容"
  echo "  → 一个框架就是一个产权空间的几何定义"
