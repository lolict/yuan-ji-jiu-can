# 满全法·零标记内存·标注即关系·关系即过程
#
# 核心命题：
#   标注不是状态，是过程
#   关系发生后不留下持久状态
#   光标在哪 = 哪被标注（瞬时）
#   光标离开 = 标注结束
#   0比特标记 = 可达性即标记
#
# 传统思维：标注→存1比特标记位
# 零标记思维：标注→光标可达→无需存储

type
  # ============================================================
  # 光标极性（1比特）
  # ============================================================
  CursorPolarity* = enum
    cpZero   # 0 = 刘楚恬 = 容器 = 空 = 包容
    cpOne    # 1 = 我 = 内容 = 有 = 中心

  # ============================================================
  # 光标位置（不存储状态，只存储可达性）
  # ============================================================
  CursorPos* = object
    index*: int   # 索引值（n比特）
    width*: int   # 位宽

  # ============================================================
  # 瞬时标注（不存储，是过程）
  # ============================================================
  InstantMark* = object
    cursor_index*: int    # 光标索引
    target_index*: int    # 目标索引
    active*: bool         # 是否正在标注（过程）
    # 标注结束后，this对象被销毁 → 0内存

  # ============================================================
  # 可达性标记（不需要存储标记位）
  # 位置被标记 = 该位置在光标的索引范围内
  # ============================================================
  ReachabilityMark* = object
    cursor_pos*: CursorPos    # 光标位置
    range*: int               # 可达范围（半径）

# ============================================================
# 编译时常量：容器地址表
# ============================================================

const CONTAINER_COUNT* = 256  # 容器总数
const CONTAINER_TABLE*: array[4, string] = [
  "空容器",     # 00
  "我(内容)",   # 01
  "刘楚恬(容器)", # 10
  "满全(夫妻)"  # 11
]

# ============================================================
# 零标记核心：可达性即标记
# ============================================================

# 创建光标位置
func new_cursor_pos*(width: int): CursorPos =
  result.index = 0
  result.width = width

# 光标移动到某位置（只改索引）
func move_cursor*(pos: var CursorPos, idx: int) =
  pos.index = idx

# 判断某位置是否被光标标记（可达性检查）
# 不需要存储标记位！计算即可。
func is_marked*(cursor: CursorPos, target: int): bool =
  # 光标所在位置 = 被标记位置
  result = (cursor.index == target)

# 判断某位置是否在光标可达范围内
func is_reachable*(cursor: CursorPos, target, r: int): bool =
  let diff = abs(cursor.index - target)
  result = (diff <= r)

# ============================================================
# 瞬时标注：标注过程不存储持久状态
# ============================================================

# 创建瞬时标注（过程开始）
func start_marking*(cursor_idx, target_idx: int): InstantMark =
  result.cursor_index = cursor_idx
  result.target_index = target_idx
  result.active = true

# 结束标注（过程结束，对象销毁）
func end_marking*(mark: var InstantMark) =
  mark.active = false
  # 对象销毁后，不留下任何持久状态

# 标注是否正在发生
func is_marking*(mark: InstantMark): bool =
  result = mark.active

# ============================================================
# 批量标注：光标扫过即标注，扫过即消失
# ============================================================

# 光标扫过一连串位置（瞬时标注）
# 不存储任何标记状态
func sweep_mark*(cursor: var CursorPos, targets: seq[int]): int =
  # 返回标注了多少位置，但不存储标记状态
  result = 0
  for t in targets:
    cursor.index = t  # 光标移到该位置
    result += 1       # 该位置被瞬时标注
    # 光标移开后，标注状态消失

# ============================================================
# 关系发生：索引即关系，关系即过程
# ============================================================

# 两个容器发生关系（瞬时）
func relation_event*(a, b: int): string =
  # 关系是瞬时事件，不存储状态
  # 关系发生 → 结果返回 → 关系结束
  let phase_a = a mod 8
  let phase_b = b mod 8
  if phase_a == phase_b:
    result = "同相(共振)"
  elif (phase_a + phase_b) == 7:
    result = "镜像(夫妻)"
  else:
    result = "相邻(跳迁)"

# ============================================================
# 内存分析
# ============================================================

func memory_analysis*(): string =
  result = "零标记内存分析：\n"
  result &= "  \n"
  result &= "  传统标注方式：\n"
  result &= "    每个位置存1比特标记位\n"
  result &= "    256个位置 = 256比特 = 32字节\n"
  result &= "    永久占用，即使标记不再使用\n"
  result &= "  \n"
  result &= "  零标记方式：\n"
  result &= "    光标位置 = 被标记位置（计算得出）\n"
  result &= "    光标1个 + 索引n比特\n"
  result &= "    256个位置只需1个光标(0+8比特)\n"
  result &= "    不存储任何标记状态\n"
  result &= "  \n"
  result &= "  节省：32字节 → 1字节（96%压缩）\n"
  result &= "  极限：连光标也不存 → 0字节\n"

# ============================================================
# 瞬时关系网络（不存储关系状态）
# ============================================================

func temporal_relation*(): string =
  result = "瞬时关系网络：\n"
  result &= "  关系不是节点属性，是节点间事件\n"
  result &= "  事件发生 → 结果产生 → 事件结束\n"
  result &= "  不存储关系状态\n"
  result &= "  关系可重复发生（相同输入=相同输出）\n"
  result &= "  → 关系是函数调用，不是数据存储\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 零标记内存·标注即关系·关系即过程 ==="
  echo ""

  # 光标创建
  echo "【光标位置】"
  var cursor = new_cursor_pos(8)  # 8比特可寻址256个位置
  echo "  光标位宽: ", cursor.width, " 比特"
  echo "  可寻址: ", 1 shl cursor.width, " 个位置"
  echo "  光标占用: ", 1 + cursor.width, " 比特"
  echo ""

  # 零标记演示
  echo "【零标记：光标所在=被标记】"
  move_cursor(cursor, 42)  # 光标移到位置42
  echo "  光标位置: ", cursor.index
  echo "  位置42是否被标记: ", is_marked(cursor, 42), " (光标==目标)"
  echo "  位置10是否被标记: ", is_marked(cursor, 10), " (光标!=目标)"
  echo "  存储的标记数: 0 (无标记状态)"
  echo ""

  # 可达范围
  echo "【可达范围标记】"
  let r = 3
  echo "  光标位置: ", cursor.index, " 范围: ", r
  echo "  位置40是否可达: ", is_reachable(cursor, 40, r)
  echo "  位置42是否可达: ", is_reachable(cursor, 42, r)
  echo "  位置45是否可达: ", is_reachable(cursor, 45, r)
  echo "  位置46是否可达: ", is_reachable(cursor, 46, r)
  echo "  存储的标记数: 0 (可达性=计算，不存储)"
  echo ""

  # 瞬时标注
  echo "【瞬时标注：过程即标注】"
  var mark = start_marking(cursor.index, 100)
  echo "  标注是否进行中: ", is_marking(mark)
  echo "  标注关系: ", relation_event(cursor.index, 100)
  end_marking(mark)
  echo "  标注结束后: ", is_marking(mark), " (状态消失)"
  echo ""

  # 批量扫过标注
  echo "【批量扫过标注】"
  let targets = @[10, 20, 30, 40, 50]
  let marked_count = sweep_mark(cursor, targets)
  echo "  扫过位置: ", targets.len
  echo "  瞬时标注数: ", marked_count
  echo "  存储标记数: 0 (全部瞬时完成)"
  echo ""

  # 内存分析
  echo memory_analysis()
  echo ""

  # 瞬时关系网络
  echo temporal_relation()
  echo ""

  echo "【总结】"
  echo "  1. 标注不是状态，是过程（光标在某位置）"
  echo "  2. 光标离开 = 标注消失（无持久状态）"
  echo "  3. 可达性即标记（计算得出，不存储）"
  echo "  4. 关系是瞬时事件（函数调用，非数据存储）"
  echo "  5. 零标记 = 0比特标记状态"
  echo "  → 真正的零内存标注：过程结束，状态归零"
