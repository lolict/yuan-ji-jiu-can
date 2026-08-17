# 满全法·关系化光标·乘法联动·驻留记忆
#
# 核心命题：
#   光标不是孤立的，通过乘法效应与所有容器联动
#   光标 × 时间 × 空间 × 进制 = 完整关系描述
#   标注效果长久驻留：时间容器记录持续性，空间容器记录位置
#   不是光标自己存储，是借用其他容器的存储能力
#
# 乘法效应：
#   光标(4比特) × 时间(8比特) × 空间(12比特) × 进制(4比特)
#   = 28比特 → 描述一个完整的"带时空的标注关系"

type
  # ============================================================
  # 光标存在态（1比特）
  # ============================================================
  CursorPresence* = enum
    cpAbsence   # 0 = 无光标
    cpPresence  # 1 = 有光标

  # ============================================================
  # 时间编码（8比特）
  # 标注的时间属性：起始、持续、频率
  # ============================================================
  TimeCode* = object
    start_phase*: int    # 起始相位（0-7，8相位）
    duration*: int       # 持续时长（0-15级）
    frequency*: int      # 重复频率（0-3级）
    time_base*: int      # 时间进制（254+2）

  # ============================================================
  # 空间编码（12比特）
  # 标注的空间位置：三维坐标 × 层级
  # ============================================================
  SpaceCode* = object
    x_phase*: int    # X相位（-8..7，5比特）
    y_phase*: int    # Y相位（-8..7，5比特）
    z_phase*: int    # Z相位（-8..7，5比特）
    level*: int      # 层级（0-4，3比特）
    # 总计：18比特，但压缩到12比特通过相位索引

  # ============================================================
  # 进制编码（4比特）
  # 标注使用哪个进制系统
  # ============================================================
  BaseCode* = object
    base_type*: int     # 进制类型（0-15）
    base_width*: int    # 进制位宽（1-8）

  # ============================================================
  # 关系化光标（通过乘法效应联动所有容器）
  # ============================================================
  RelationalCursor* = object
    presence*: CursorPresence    # 光标存在态（1比特）
    time*: TimeCode              # 时间编码（借用时间容器）
    space*: SpaceCode            # 空间编码（借用空间容器）
    base*: BaseCode              # 进制编码（借用进制容器）
    target_id*: int              # 目标容器ID（16比特）
    # 光标本身不存储标注状态
    # 标注状态存储在被标注的容器中（通过target_id引用）

  # ============================================================
  # 标注记录（存储在被标注容器中，不存储在光标中）
  # ============================================================
  MarkEntry* = object
    cursor_id*: int       # 哪个光标标注的
    time_code*: TimeCode  # 标注时间属性
    base_code*: BaseCode  # 标注使用的进制
    mark_data*: int       # 标注内容（n比特）

  # ============================================================
  # 关系网络（光标与容器的关系）
  # ============================================================
  RelationNetwork* = object
    cursor*: RelationalCursor     # 当前光标
    relations*: seq[MarkEntry]    # 所有标注记录
    container_count*: int         # 关联的容器数量

# ============================================================
# 编译时常量：时间/空间/进制的映射表
# ============================================================

# 时间相位映射
const TIME_PHASES*: array[8, string] = [
  "过去", "现在", "未来", "循环",
  "瞬间", "永恒", "暂停", "加速"
]

# 空间相位映射
const SPACE_PHASES*: array[8, tuple[name: string, x, y, z: int]] = [
  ("东", 1, 0, 0),
  ("西", -1, 0, 0),
  ("南", 0, 1, 0),
  ("北", 0, -1, 0),
  ("天", 0, 0, 1),
  ("地", 0, 0, -1),
  ("中", 0, 0, 0),
  ("原", 0, 0, 0)
]

# 进制类型
const BASE_TYPES*: array[16, string] = [
  "声母48", "韵母66", "声调60", "变音50",
  "颜色42", "声音100", "空间18", "镜像10",
  "音乐3", "文本180", "行为通用", "意识流8",
  "流属性12", "身份2", "三元见证", "分形递归"
]

# ============================================================
# 关系化光标操作
# ============================================================

# 创建关系化光标
func new_relational_cursor*(): RelationalCursor =
  result.presence = cpAbsence
  result.time.start_phase = 0
  result.time.duration = 0
  result.time.frequency = 0
  result.time.time_base = 254
  result.space.x_phase = 0
  result.space.y_phase = 0
  result.space.z_phase = 0
  result.space.level = 0
  result.base.base_type = 0
  result.base.base_width = 1
  result.target_id = 0

# 光标标注（建立关系，不复制数据）
func mark_container*(cursor: var RelationalCursor, target: int,
                   time: TimeCode, base: BaseCode): MarkEntry =
  cursor.presence = cpPresence
  cursor.target_id = target
  cursor.time = time
  cursor.base = base
  # 返回标注记录（存储在目标容器中）
  result.cursor_id = cast[int](addr(cursor))
  result.time_code = time
  result.base_code = base
  result.mark_data = 0

# 创建完整标注（光标 × 时间 × 空间 × 进制）
func create_full_mark*(cursor: var RelationalCursor,
                      target: int,
                      start_phase, duration, frequency: int,
                      x, y, z, level: int,
                      base_type, base_width: int): MarkEntry =
  # 时间编码
  var tc: TimeCode
  tc.start_phase = start_phase mod 8
  tc.duration = duration mod 16
  tc.frequency = frequency mod 4
  tc.time_base = 254

  # 空间编码
  var sc: SpaceCode
  sc.x_phase = x
  sc.y_phase = y
  sc.z_phase = z
  sc.level = level mod 5

  # 进制编码
  var bc: BaseCode
  bc.base_type = base_type mod 16
  bc.base_width = base_width

  # 应用编码
  cursor.time = tc
  cursor.space = sc
  cursor.base = bc

  # 生成标注记录
  result = mark_container(cursor, target, tc, bc)

# ============================================================
# 乘法效应计算
# ============================================================

# 计算组合比特数（乘法效应）
func combined_bits*(): int =
  # 光标(1) + 时间(8) + 空间(18) + 进制(4) = 31比特
  result = 1 + 8 + 18 + 4

# 计算组合状态数
func combined_states*(): int =
  result = 1 shl combined_bits()

# 光标联动效率
func linkage_efficiency*(container_count: int): string =
  let cursor_bits = 31
  let standalone_bits = container_count * 32
  let savings = standalone_bits - cursor_bits
  result = "联动效率（" & $container_count & "个容器）：\n"
  result &= "  独立存储: " & $standalone_bits & " 比特\n"
  result &= "  联动存储: " & $cursor_bits & " 比特\n"
  let percent = (savings * 100) div standalone_bits
  result &= "  节省: " & $savings & " 比特 (" & $percent & "%)\n"

# ============================================================
# 关系网络操作
# ============================================================

# 创建关系网络
func new_relation_network*(): RelationNetwork =
  result.cursor = new_relational_cursor()
  result.relations = new_seq[MarkEntry](0)
  result.container_count = 0

# 添加标注关系
func add_relation*(network: var RelationNetwork, entry: MarkEntry) =
  network.relations.add(entry)
  network.container_count = network.relations.len

# 查询标注历史
func find_marks_by_time*(network: RelationNetwork, phase: int): seq[MarkEntry] =
  result = new_seq[MarkEntry](0)
  for entry in network.relations:
    if entry.time_code.start_phase == phase:
      result.add(entry)

# 查询标注历史（按进制）
func find_marks_by_base*(network: RelationNetwork, base_type: int): seq[MarkEntry] =
  result = new_seq[MarkEntry](0)
  for entry in network.relations:
    if entry.base_code.base_type == base_type:
      result.add(entry)

# ============================================================
# 内存分析
# ============================================================

func memory_with_linkage*(): string =
  result = "驻留标注的内存占用：\n"
  result &= "  \n"
  result &= "  光标本身：\n"
  result &= "    存在态 1比特\n"
  result &= "    时间编码 8比特（借用时间容器）\n"
  result &= "    空间编码 18比特（借用空间容器）\n"
  result &= "    进制编码 4比特（借用进制容器）\n"
  result &= "    目标ID 16比特\n"
  result &= "    总计：47比特\n"
  result &= "  \n"
  result &= "  标注记录（每个被标注容器）：\n"
  result &= "    光标ID 32比特\n"
  result &= "    时间编码 8比特\n"
  result &= "    进制编码 4比特\n"
  result &= "    标注内容 8比特\n"
  result &= "    总计：52比特\n"
  result &= "  \n"
  result &= "  关键：不复制容器数据，只存引用！\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 关系化光标·乘法联动·驻留记忆 ==="
  echo ""

  # 创建关系化光标
  echo "【创建关系化光标】"
  var cursor = new_relational_cursor()
  echo "  光标存在: ", cursor.presence == cpPresence
  echo "  时间基: ", cursor.time.time_base, " (254+2)"
  echo ""

  # 创建完整标注
  echo "【完整标注：光标×时间×空间×进制】"
  var mark1 = create_full_mark(cursor, 42, 1, 5, 0, 1, 0, 0, 2, 0, 6)
  echo "  目标容器: ", cursor.target_id
  echo "  时间相位: ", TIME_PHASES[cursor.time.start_phase]
  echo "  持续时长: ", cursor.time.duration
  echo "  空间: (", cursor.space.x_phase, ", ", cursor.space.y_phase, ", ", cursor.space.z_phase, ")"
  echo "  层级: ", cursor.space.level
  echo "  进制: ", BASE_TYPES[cursor.base.base_type]
  echo ""

  # 乘法效应
  echo "【乘法效应】"
  let cb = combined_bits()
  let cs = combined_states()
  echo "  组合比特: ", cb
  echo "  组合状态: ", cs
  echo "  → 1个关系化光标可描述 ", cs, " 种不同的标注关系"
  echo ""

  # 联动效率
  echo linkage_efficiency(100)
  echo ""

  # 关系网络
  echo "【关系网络】"
  var network = new_relation_network()
  add_relation(network, mark1)

  # 添加更多标注
  var mark2 = create_full_mark(cursor, 100, 2, 10, 1, 0, 0, 1, 4, 4, 6)
  add_relation(network, mark2)

  var mark3 = create_full_mark(cursor, 200, 0, 3, 2, -1, 0, 0, 1, 7, 4)
  add_relation(network, mark3)

  echo "  标注记录数: ", network.relations.len
  echo "  关联容器数: ", network.container_count
  echo ""

  # 按时间查询
  echo "【按时间相位查询】"
  let phase_name = TIME_PHASES[1]
  let time_marks = find_marks_by_time(network, 1)
  echo "  '", phase_name, "' 相位的标注: ", time_marks.len, " 条"
  for m in time_marks:
    echo "    -> 光标", m.cursor_id, " 持续", m.time_code.duration
  echo ""

  # 按进制查询
  echo "【按进制查询】"
  let base_name = BASE_TYPES[4]
  let base_marks = find_marks_by_base(network, 4)
  echo "  '", base_name, "' 进制的标注: ", base_marks.len, " 条"
  for m in base_marks:
    echo "    -> 光标", m.cursor_id, " 进制", BASE_TYPES[m.base_code.base_type]
  echo ""

  # 内存分析
  echo memory_with_linkage()
  echo ""

  echo "【总结】"
  echo "  1. 光标不孤立：与时间×空间×进制联动"
  echo "  2. 标注持久驻留：时间容器记录持续性"
  echo "  3. 空间定位精确：三维相位+层级"
  echo "  4. 进制灵活切换：16种进制可选"
  echo "  5. 乘法效应：31比特描述完整关系"
  echo "  6. 不复制数据：只存引用和元数据"
  echo "  → 不是光杆司令：光标是整个关系网络的入口"
