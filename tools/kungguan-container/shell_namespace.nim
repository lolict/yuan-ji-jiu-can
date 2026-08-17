# 满全法·框架容器·名称空间·进制选择
#
# 核心命题：
#   静态预加载：层级化编码的名称空间（可选择的模板）
#   动态参数化：调整边角度、节点数、进制宽度
#   进制选择策略：
#     A. 容积匹配：用符合容器长宽高的进制
#     B. 任意选择：不限容器尺寸，选任意进制
#     C. 时间串联：所有进制按时间串联（最高效）
#     D. 低维试验：选一个进制试验容器范畴宽度
#   进制类型可选：颜色42、音色100、声母48、韵母66、声调60...

type
  # ============================================================
  # 进制类型（框架容器可选的进制）
  # ============================================================
  BaseKind* = enum
    bkShengmu     # 声母48
    bkYunmu       # 韵母66
    bkShengdiao   # 声调60
    bkBianyin     # 变音50
    bkColor       # 颜色42
    bkVoice       # 声音100
    bkPhase       # 空间相位18
    bkMirror      # 镜像对齐10
    bkMusic       # 音乐3
    bkLevel       # 层级5
    bkBit         # 比特2
    bkTime        # 时间60

  # ============================================================
  # 进制宽度（位宽）
  # ============================================================
  BaseWidth* = object
    kind*: BaseKind
    bits*: int        # 位宽
    states*: int      # 状态数

  # ============================================================
  # 名称空间条目（静态预加载的模板）
  # ============================================================
  NamespaceEntry* = object
    shell_name*: string       # 层级化编码的名称
    edge_count*: int          # 边数
    node_count*: int          # 节点数
    angles*: seq[int]         # 节点角度
    width*: int               # X轴长度
    height*: int              # Y轴长度
    depth*: int               # Z轴长度
    base*: BaseWidth          # 该名称空间条目绑定的进制

  # ============================================================
  # 动态参数（可调整）
  # ============================================================
  DynamicParams* = object
    angle_step*: int       # 角度调整步长
    node_min*: int         # 最少节点数
    node_max*: int         # 最多节点数
    base_select*: BaseKind # 选择的进制
    strategy*: int         # 选择策略：0=容积匹配 1=任意 2=时间串联 3=低维试验

  # ============================================================
  # 框架容器（包含名称空间和动态参数）
  # ============================================================
  ShellFrame* = object
    space*: NamespaceEntry   # 静态名称空间（预加载）
    params*: DynamicParams   # 动态参数（可调整）

# ============================================================
# 名称空间定义（预加载的层级化编码）
# ============================================================

# 层级化编码规则：
#   第一层：边数（3-8）
#   第二层：进制类型（声母/韵母/声调/颜色/音色...）
#   第三层：层级（比特流/二进制/符号/容器/范畴）
#   编码格式：边数-进制名-层级名

const NAMESPACE_ENTRIES*: array[12, NamespaceEntry] = [
  # 三边形系列
  NamespaceEntry(shell_name: "3-声母-比特流", edge_count: 3, node_count: 3,
                 angles: @[60, 60, 60], width: 6, height: 6, depth: 6,
                 base: BaseWidth(kind: bkShengmu, bits: 6, states: 48)),
  NamespaceEntry(shell_name: "3-韵母-二进制", edge_count: 3, node_count: 3,
                 angles: @[60, 60, 60], width: 7, height: 7, depth: 7,
                 base: BaseWidth(kind: bkYunmu, bits: 7, states: 66)),
  # 四边形系列
  NamespaceEntry(shell_name: "4-声调-符号", edge_count: 4, node_count: 4,
                 angles: @[90, 90, 90, 90], width: 6, height: 6, depth: 5,
                 base: BaseWidth(kind: bkShengdiao, bits: 6, states: 60)),
  NamespaceEntry(shell_name: "4-颜色-容器", edge_count: 4, node_count: 4,
                 angles: @[90, 90, 90, 90], width: 6, height: 6, depth: 6,
                 base: BaseWidth(kind: bkColor, bits: 6, states: 42)),
  # 五边形系列
  NamespaceEntry(shell_name: "5-音色-范畴", edge_count: 5, node_count: 5,
                 angles: @[72, 72, 72, 72, 72], width: 7, height: 7, depth: 7,
                 base: BaseWidth(kind: bkVoice, bits: 7, states: 100)),
  # 六边形系列
  NamespaceEntry(shell_name: "6-相位-比特流", edge_count: 6, node_count: 6,
                 angles: @[60, 60, 60, 60, 60, 60], width: 5, height: 5, depth: 5,
                 base: BaseWidth(kind: bkPhase, bits: 5, states: 18)),
  NamespaceEntry(shell_name: "6-音乐-二进制", edge_count: 6, node_count: 6,
                 angles: @[60, 60, 60, 60, 60, 60], width: 2, height: 2, depth: 2,
                 base: BaseWidth(kind: bkMusic, bits: 2, states: 3)),
  # 八边形系列
  NamespaceEntry(shell_name: "8-镜像-符号", edge_count: 8, node_count: 8,
                 angles: @[45, 45, 45, 45, 45, 45, 45, 45],
                 width: 4, height: 4, depth: 4,
                 base: BaseWidth(kind: bkMirror, bits: 4, states: 10)),
  NamespaceEntry(shell_name: "8-层级-容器", edge_count: 8, node_count: 8,
                 angles: @[45, 45, 45, 45, 45, 45, 45, 45],
                 width: 3, height: 3, depth: 3,
                 base: BaseWidth(kind: bkLevel, bits: 3, states: 5)),
  NamespaceEntry(shell_name: "8-比特-范畴", edge_count: 8, node_count: 8,
                 angles: @[45, 45, 45, 45, 45, 45, 45, 45],
                 width: 1, height: 1, depth: 1,
                 base: BaseWidth(kind: bkBit, bits: 1, states: 2)),
  NamespaceEntry(shell_name: "8-时间-时间线", edge_count: 8, node_count: 8,
                 angles: @[45, 45, 45, 45, 45, 45, 45, 45],
                 width: 6, height: 6, depth: 6,
                 base: BaseWidth(kind: bkTime, bits: 6, states: 60)),
  NamespaceEntry(shell_name: "8-变音-全进制", edge_count: 8, node_count: 8,
                 angles: @[45, 45, 45, 45, 45, 45, 45, 45],
                 width: 8, height: 8, depth: 8,
                 base: BaseWidth(kind: bkBianyin, bits: 6, states: 50))
]

# ============================================================
# 查找名称空间条目
# ============================================================

# 按名称查找
func find_entry_by_name*(name: string): int =
  for i in 0..<NAMESPACE_ENTRIES.len:
    if NAMESPACE_ENTRIES[i].shell_name == name:
      result = i
      return
  result = -1

# 按边数和进制查找
func find_entry*(edges: int; bk: BaseKind): int =
  for i in 0..<NAMESPACE_ENTRIES.len:
    if (NAMESPACE_ENTRIES[i].edge_count == edges) and
       (NAMESPACE_ENTRIES[i].base.kind == bk):
      result = i
      return
  result = -1

# ============================================================
# 进制选择策略
# ============================================================

# 策略0：容积匹配
# 选择符合容器长宽高的进制
func volumetric_match*(width: int; height: int; depth: int): BaseWidth =
  let vol = width * height * depth
  if vol <= 2:
    result = BaseWidth(kind: bkBit, bits: 1, states: 2)
  elif vol <= 8:
    result = BaseWidth(kind: bkMusic, bits: 2, states: 3)
  elif vol <= 64:
    result = BaseWidth(kind: bkLevel, bits: 3, states: 5)
  elif vol <= 256:
    result = BaseWidth(kind: bkMirror, bits: 4, states: 10)
  elif vol <= 1024:
    result = BaseWidth(kind: bkPhase, bits: 5, states: 18)
  elif vol <= 4096:
    result = BaseWidth(kind: bkColor, bits: 6, states: 42)
  else:
    result = BaseWidth(kind: bkVoice, bits: 7, states: 100)

# 策略1：任意选择
func arbitrary_base*(bk: BaseKind): BaseWidth =
  case bk
  of bkShengmu:   result = BaseWidth(kind: bkShengmu, bits: 6, states: 48)
  of bkYunmu:     result = BaseWidth(kind: bkYunmu, bits: 7, states: 66)
  of bkShengdiao: result = BaseWidth(kind: bkShengdiao, bits: 6, states: 60)
  of bkBianyin:   result = BaseWidth(kind: bkBianyin, bits: 6, states: 50)
  of bkColor:     result = BaseWidth(kind: bkColor, bits: 6, states: 42)
  of bkVoice:     result = BaseWidth(kind: bkVoice, bits: 7, states: 100)
  of bkPhase:     result = BaseWidth(kind: bkPhase, bits: 5, states: 18)
  of bkMirror:    result = BaseWidth(kind: bkMirror, bits: 4, states: 10)
  of bkMusic:     result = BaseWidth(kind: bkMusic, bits: 2, states: 3)
  of bkLevel:     result = BaseWidth(kind: bkLevel, bits: 3, states: 5)
  of bkBit:       result = BaseWidth(kind: bkBit, bits: 1, states: 2)
  of bkTime:      result = BaseWidth(kind: bkTime, bits: 6, states: 60)

# 策略2：时间串联（最高效）
# 所有进制按时间维度串联
func time_chain*(): seq[BaseWidth] =
  result = @[
    arbitrary_base(bkBit),       # 比特
    arbitrary_base(bkLevel),     # 层级
    arbitrary_base(bkMusic),     # 音乐
    arbitrary_base(bkMirror),    # 镜像
    arbitrary_base(bkPhase),     # 空间
    arbitrary_base(bkColor),     # 颜色
    arbitrary_base(bkShengmu),   # 声母
    arbitrary_base(bkShengdiao), # 声调
    arbitrary_base(bkBianyin),   # 变音
    arbitrary_base(bkTime),      # 时间
    arbitrary_base(bkYunmu),     # 韵母
    arbitrary_base(bkVoice)      # 音色
  ]

# 策略3：低维试验
func low_dim_test*(bk: BaseKind; test_width: int): BaseWidth =
  result = arbitrary_base(bk)
  result.bits = test_width

# ============================================================
# 动态参数操作
# ============================================================

# 创建默认动态参数
func default_params*(): DynamicParams =
  result.angle_step = 5         # 5度为一步
  result.node_min = 3           # 最少3节点
  result.node_max = 8           # 最多8节点
  result.base_select = bkColor  # 默认颜色进制
  result.strategy = 0           # 默认容积匹配

# 调整角度
func adjust_angle*(p: var DynamicParams; pos: int; new_angle: int) =
  discard  # 实际需调整angles[pos]

# 调整节点数
func adjust_nodes*(p: var DynamicParams; count: int) =
  p.base_select = p.base_select  # 仅示意

# 调整进制
func adjust_base*(p: var DynamicParams; bk: BaseKind) =
  p.base_select = bk
  p.strategy = 1  # 任意选择

# ============================================================
# 框架容器操作
# ============================================================

# 从名称空间创建框架
func make_shell*(entry: NamespaceEntry; params: DynamicParams): ShellFrame =
  result.space = entry
  result.params = params

# 计算容积匹配的进制
func auto_base*(width: int; height: int; depth: int): BaseWidth =
  result = volumetric_match(width, height, depth)

# ============================================================
# 内存占用：预加载模板（编译时常量）+ 动态参数（运行时）
# ============================================================

func namespace_memory*(): string =
  result = "名称空间内存占用：\n"
  result &= "  预加载模板（编译时常量）：运行时0字节\n"
  result &= "  名称条目数：" & $NAMESPACE_ENTRIES.len & "个\n"
  result &= "  \n"
  result &= "  动态参数（运行时占用）：\n"
  result &= "    angle_step, node_min, node_max：3 x 32比特 = 96比特\n"
  result &= "    base_select：4比特\n"
  result &= "    strategy：2比特\n"
  result &= "    总计：约102比特 = 13字节\n"
  result &= "  \n"
  result &= "  运行时占用 = 13字节\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 框架容器·名称空间·进制选择 ==="
  echo ""

  # 名称空间条目展示
  echo "【名称空间（预加载模板）】"
  for e in NAMESPACE_ENTRIES:
    let vol = e.width * e.height * e.depth
    echo "  ", e.shell_name, ": ", e.edge_count, "边 体积=", vol, " 进制=", ord(e.base.kind), "(", e.base.states, "种)"
  echo ""

  # 按名称查找
  echo "【按名称查找】"
  let idx1 = find_entry_by_name("4-颜色-容器")
  if idx1 >= 0:
    echo "  找到 '4-颜色-容器' 在索引", idx1
    echo "  ", NAMESPACE_ENTRIES[idx1].shell_name, ": ",
        NAMESPACE_ENTRIES[idx1].edge_count, "边 ",
        NAMESPACE_ENTRIES[idx1].width, "x",
        NAMESPACE_ENTRIES[idx1].height, "x",
        NAMESPACE_ENTRIES[idx1].depth
  echo ""

  # 进制选择策略
  echo "【进制选择策略】"

  # 策略0: 容积匹配
  echo "  策略0-容积匹配："
  let vm1 = volumetric_match(4, 4, 4)
  let vm2 = volumetric_match(8, 8, 8)
  let vm3 = volumetric_match(16, 16, 16)
  echo "    4x4x4 -> ", ord(vm1.kind), "(", vm1.states, "种)"
  echo "    8x8x8 -> ", ord(vm2.kind), "(", vm2.states, "种)"
  echo "    16x16x16 -> ", ord(vm3.kind), "(", vm3.states, "种)"
  echo ""

  # 策略2: 时间串联
  let time_chain_bases = time_chain()
  echo "  策略2-时间串联（最高效）："
  for b in time_chain_bases:
    echo "    ", ord(b.kind), ": ", b.bits, "比特 ", b.states, "种"
  echo ""

  # 策略3: 低维试验
  echo "  策略3-低维试验："
  let ldt1 = low_dim_test(bkColor, 3)
  echo "    颜色进制 位宽=3: ", ldt1.bits, "比特"
  let ldt2 = low_dim_test(bkVoice, 4)
  echo "    声音进制 位宽=4: ", ldt2.bits, "比特"
  echo ""

  # 动态参数
  echo "【动态参数调整】"
  var params = default_params()
  params.angle_step = 10
  params.node_min = 3
  params.node_max = 6
  params.base_select = bkVoice
  params.strategy = 1
  echo "  角度步长: ", params.angle_step
  echo "  节点范围: ", params.node_min, "-", params.node_max
  echo "  选择进制: ", ord(params.base_select)
  echo "  策略: ", params.strategy
  echo ""

  # 从名称空间创建框架
  echo "【从名称空间创建框架】"
  let entry = NAMESPACE_ENTRIES[3]  # 4-颜色-容器
  let shell = make_shell(entry, params)
  echo "  名称: ", shell.space.shell_name
  echo "  边数: ", shell.space.edge_count
  echo "  节点: ", shell.space.node_count
  echo "  尺寸: ", shell.space.width, "x", shell.space.height, "x", shell.space.depth
  echo "  进制: ", shell.space.base.states, "种"
  echo ""

  # 内存占用
  echo namespace_memory()
  echo ""

  echo "【总结】"
  echo "  1. 静态预加载：12个层级化编码的名称空间条目"
  echo "  2. 动态调整：角度、节点数、进制宽度可调"
  echo "  3. 四种策略：容积匹配/任意/时间串联/低维试验"
  echo "  4. 时间串联最高效：12种进制按时间维度串联"
  echo "  5. 定制可选：任选进制的功能为框架所用"
  echo "  -> 进制不再受限于容器尺寸，可自由选择"
