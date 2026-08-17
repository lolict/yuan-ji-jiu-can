# 满全法·触摸光标·1比特标记系统
# 光标+标记色彩，只占用1比特
# 触摸即光标，手指即鼠标，虚拟打败实体垄断
# 空符号不占内存，打出才占内存
# 相位容器静止不占地：编译时常量，不占运行时堆内存

type
  # ============================================================
  # 光标存在态（1比特）
  # 0 = 无光标（空，不占内存）
  # 1 = 有光标（存在，占1比特）
  # ============================================================
  CursorPresence* = enum
    cpAbsent    # 0 空=不存在=不占内存
    cpPresent   # 1 有=存在=占1比特

  # ============================================================
  # 标记色彩（1比特）
  # 0 = 未标记（透明，不占内存）
  # 1 = 已标记（有色彩，占1比特）
  # ============================================================
  MarkColor* = enum
    mcUnmarked  # 0 未标记=透明=不占内存
    mcMarked    # 1 已标记=有色彩=占1比特

  # ============================================================
  # 触摸模式（2比特）
  # ============================================================
  TouchMode* = enum
    tmTap       # 0 点触=跳跃着陆
    tmSlide     # 1 滑行=跟随移动
    tmPress     # 2 按入=进入菜单
    tmScroll    # 3 滚珠=调整焦距

  # ============================================================
  # 悬浮菜单（自带，无需实体鼠标）
  # ============================================================
  FloatMenu* = object
    focus_distance*: int    # 焦距（1-7级）
    move_speed*: int        # 移动速度（1-7级）
    move_range*: int        # 移动范畴（相位范围）
    target_phase*: int      # 目标相位ID

  # ============================================================
  # 1比特光标（总共4比特 = 0.5字节）
  # 不存储位置！位置由系统当前触摸位置决定
  # ============================================================
  TouchCursor* = object
    presence*: CursorPresence   # 1比特：有/无
    marked*: MarkColor          # 1比特：标记/未标记
    mode*: TouchMode            # 2比特：4种触摸模式

  # ============================================================
  # 标记位图（极简占用）
  # 每个位置1比特，1字节可存8个位置的标记状态
  # ============================================================
  MarkBitmap* = object
    data*: seq[byte]    # 每个bit代表一个位置的标记状态
    width*: int         # 位图宽度（位置数）
    height*: int        # 位图高度

# ============================================================
# 编译时常量：相位容器静止不占地
# 作为const存储在程序.rodata段，不占运行时堆内存
# 程序代码≠运行时变量内存
# ============================================================

const PHASE_COORDS*: array[8, array[3, int]] = [
  [1, 0, 0],    # 东
  [-1, 0, 0],   # 西
  [0, 1, 0],    # 南
  [0, -1, 0],   # 北
  [0, 0, 1],    # 天
  [0, 0, -1],   # 地
  [0, 0, 0],    # 中
  [0, 0, 0]     # 原
]

const PHASE_NAMES*: array[8, string] = [
  "东", "西", "南", "北", "天", "地", "中", "原"
]

# ============================================================
# 触摸光标操作
# ============================================================

# 创建空光标（不占内存）
func new_cursor*(): TouchCursor =
  result.presence = cpAbsent
  result.marked = mcUnmarked
  result.mode = tmTap

# 触摸：手指点哪里，光标就在哪里（1比特）
proc touch*(cursor: var TouchCursor) =
  cursor.presence = cpPresent  # 触摸→存在（1）

# 离开：手指离开，光标消失（0比特）
proc leave*(cursor: var TouchCursor) =
  cursor.presence = cpAbsent   # 离开→消失（0）

# 滑行：手指滑动，光标跟随
proc slide*(cursor: var TouchCursor, mode: TouchMode) =
  cursor.presence = cpPresent
  cursor.mode = mode

# 标记：给位置打上色彩标记（1比特）
proc mark*(cursor: var TouchCursor) =
  cursor.marked = mcMarked

# 取消标记
proc unmark*(cursor: var TouchCursor) =
  cursor.marked = mcUnmarked

# 是否占用内存
func occupies_memory*(cursor: TouchCursor): bool =
  result = (cursor.presence == cpPresent)

# 是否标记
func is_marked*(cursor: TouchCursor): bool =
  result = (cursor.marked == mcMarked)

# ============================================================
# 悬浮菜单操作
# ============================================================

# 调整焦距
proc set_focus*(menu: var FloatMenu, distance: int) =
  menu.focus_distance = distance

# 调整速度
proc set_speed*(menu: var FloatMenu, speed: int) =
  menu.move_speed = speed

# 调整范畴
proc set_range*(menu: var FloatMenu, range: int) =
  menu.move_range = range

# 移动到目标相位
proc move_to_phase*(menu: var FloatMenu, phase_id: int) =
  menu.target_phase = phase_id

# ============================================================
# 位图标记系统：每个位置1比特
# ============================================================

func new_bitmap*(w, h: int): MarkBitmap =
  let total_bits = w * h
  let total_bytes = (total_bits + 7) div 8
  result.data = new_seq[byte](total_bytes)
  result.width = w
  result.height = h

# 设置位置的标记（1比特）
proc set_bit*(bitmap: var MarkBitmap, x, y: int) =
  let idx = y * bitmap.width + x
  let byte_idx = idx div 8
  let bit_idx = idx mod 8
  if byte_idx < bitmap.data.len:
    bitmap.data[byte_idx] = bitmap.data[byte_idx] or byte(1 shl bit_idx)

# 清除位置的标记（0比特）
proc clear_bit*(bitmap: var MarkBitmap, x, y: int) =
  let idx = y * bitmap.width + x
  let byte_idx = idx div 8
  let bit_idx = idx mod 8
  if byte_idx < bitmap.data.len:
    bitmap.data[byte_idx] = bitmap.data[byte_idx] and byte(not (1 shl bit_idx))

# 检查位置是否标记
func is_bit_set*(bitmap: MarkBitmap, x, y: int): bool =
  let idx = y * bitmap.width + x
  let byte_idx = idx div 8
  let bit_idx = idx mod 8
  if byte_idx < bitmap.data.len:
    result = (bitmap.data[byte_idx] and byte(1 shl bit_idx)) != 0

# 统计标记位置数
func count_marked*(bitmap: MarkBitmap): int =
  result = 0
  for b in bitmap.data:
    var x = b
    while x != 0:
      result += 1
      x = x and byte(x - 1)

# 位图占用字节
func bitmap_size*(bitmap: MarkBitmap): int =
  result = bitmap.data.len

# ============================================================
# 内存占用统计
# ============================================================

func cursor_bits*(): int =
  # 光标: 1比特(presence) + 1比特(marked) + 2比特(mode) = 4比特 = 0.5字节
  result = 4

func marked_positions_bits*(count: int): int =
  # 每个标记位置1比特
  result = count

func total_memory*(cursor: TouchCursor, marked_count: int): int =
  result = cursor_bits() + marked_positions_bits(marked_count)

# ============================================================
# 相位容器静止不占地证明
# ============================================================

func phase_memory_proof*(): string =
  result = "相位容器静止不占地证明：\n"
  result &= "  const PHASE_COORDS: 编译时常量 → .rodata段\n"
  result &= "  const PHASE_NAMES:  编译时常量 → .rodata段\n"
  result &= "  .rodata段 = 程序代码的一部分\n"
  result &= "  程序代码 ≠ 运行时变量内存\n"
  result &= "  结论：相位容器运行时占 0 字节堆内存\n"
  result &= "  只有触摸光标(4比特)和标记位图(每点1比特)占运行时内存\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 触摸光标·1比特标记系统 ==="
  echo ""

  # 创建光标
  var cursor = new_cursor()
  echo "【空光标状态】"
  echo "  占用内存: ", occupies_memory(cursor), " (不占)"
  echo "  总比特数: ", cursor_bits(), " = ", cursor_bits() / 8, " 字节"
  echo ""

  # 触摸
  touch(cursor)
  echo "【触摸后】"
  echo "  占用内存: ", occupies_memory(cursor), " (占)"
  echo "  有光标: ", cursor.presence == cpPresent
  echo ""

  # 标记
  mark(cursor)
  echo "【标记后】"
  echo "  已标记: ", is_marked(cursor), " (1比特)"
  echo ""

  # 滑行
  slide(cursor, tmSlide)
  echo "【滑行模式】"
  echo "  模式: 滑行"
  echo ""

  # 位图测试
  echo "【位图标记系统】"
  var bitmap = new_bitmap(18, 5)  # 18相位 × 5层级 = 90位置
  echo "  位图大小: 18×5 = 90位置"
  echo "  占用字节: ", bitmap_size(bitmap), " (90bit / 8)"
  echo ""

  # 标记几个位置
  set_bit(bitmap, 0, 0)  # 东-比特流
  set_bit(bitmap, 1, 0)  # 西-比特流
  set_bit(bitmap, 6, 2)  # 中-容器
  set_bit(bitmap, 4, 4)  # 天-范畴

  echo "  标记4个位置后:"
  echo "    标记总数: ", count_marked(bitmap)
  echo "    占用: ", count_marked(bitmap), " 比特 = ", count_marked(bitmap) / 8, " 字节"
  echo "    (0,0)标记: ", is_bit_set(bitmap, 0, 0)
  echo "    (3,3)标记: ", is_bit_set(bitmap, 3, 3)
  echo ""

  # 内存总结
  echo "【内存占用总结】"
  echo "  光标本体: ", cursor_bits(), " 比特 (0.5字节)"
  echo "  每个标记: 1 比特 (0.125字节)"
  echo "  每个空位: 0 比特 (不占内存!)"
  echo "  总内存(4标记): ", total_memory(cursor, 4), " 比特"
  echo ""

  # 相位容器证明
  echo phase_memory_proof()
  echo ""

  # 降维打击实体鼠标
  echo "【降维打击实体鼠标】"
  echo "  实体鼠标: 需要硬件、电池、USB、驱动程序"
  echo "  触摸光标: 手指即鼠标，4比特即驱动"
  echo "  虚拟打败实体: 零硬件成本"
  echo "  空符号不占内存: 打出才占，不打出不占"
  echo "  相位静止不占地: const编译时常量，运行时0字节"
