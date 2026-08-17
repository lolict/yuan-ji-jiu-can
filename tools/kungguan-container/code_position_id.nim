# 满全法·码位即ID·进制编码即定位
#
# 核心命题：
#   ID不擅自主张命名，ID = 进制的码位组合
#   声母码位(6比特) + 韵母码位(7比特) + 声调码位(6比特) + 相位码位(5比特) + 层级码位(3比特)
#   = 27比特 = 唯一ID

type
  # ============================================================
  # 码位类型（不额外命名ID，码位即ID）
  # ============================================================
  CodeCategory* = enum
    ccShengmu      # 声母48 = 6比特
    ccYunmu        # 韵母66 = 7比特
    ccShengdiao    # 声调60 = 6比特
    ccBianyin      # 变音50 = 6比特
    ccColor        # 颜色42 = 6比特
    ccPhase        # 空间相位18 = 5比特
    ccLevel        # 层级5 = 3比特
    ccDirection    # 镜像对齐10 = 4比特
    ccMusic        # 音乐3 = 2比特
    ccMark         # 标记色彩2 = 1比特

  # ============================================================
  # 复合ID（码位组合 = 唯一地址，不额外命名）
  # ============================================================
  CompoundID* = object
    shengmu*: int      # 6比特
    yunmu*: int        # 7比特
    shengdiao*: int    # 6比特
    phase*: int        # 5比特
    level*: int        # 3比特
    # 总计：27比特 = 唯一ID（1.3亿种组合）

  # ============================================================
  # 三维坐标ID（不额外命名，坐标即ID）
  # ============================================================
  CoordinateID* = object
    x_phase*: int      # X相位
    y_phase*: int      # Y相位
    z_phase*: int      # Z相位
    level*: int        # 层级（0-4）

  # ============================================================
  # 光标定位（指向码位组合，不指向擅自命名的ID）
  # ============================================================
  CodeCursor* = object
    presence*: int     # 1比特：有/无
    code*: CompoundID  # 码位组合 = 27比特ID
    # 没有target_id！码位组合就是地址

# ============================================================
# 码位位宽（编译时常量）
# ============================================================

const CODE_BIT_WIDTH*: array[10, int] = [
  6,   # 声母48
  7,   # 韵母66
  6,   # 声调60
  6,   # 变音50
  6,   # 颜色42
  5,   # 空间相位18
  3,   # 层级5
  4,   # 镜像对齐10
  2,   # 音乐3
  1    # 标记色彩2
]

# ============================================================
# 码位宽度和
# ============================================================

func total_code_bits*(): int =
  result = 0
  for w in CODE_BIT_WIDTH:
    result += w

# 复合ID的位宽
func compound_id_bits*(): int =
  result = 6 + 7 + 6 + 5 + 3  # = 27比特

# 复合ID的组合数
func compound_id_states*(): int =
  result = 1 shl compound_id_bits()  # 2^27 = 1.3亿

# ============================================================
# 创建复合ID（码位组合即ID）
# ============================================================

func make_compound_id*(sm, ym, sd: int; ph: int; lv: int): CompoundID =
  result.shengmu = sm mod 48
  result.yunmu = ym mod 66
  result.shengdiao = sd mod 60
  result.phase = ph mod 18
  result.level = lv mod 5

# 复合ID相等判断
func id_equal*(a, b: CompoundID): bool =
  result = (a.shengmu == b.shengmu) and
           (a.yunmu == b.yunmu) and
           (a.shengdiao == b.shengdiao) and
           (a.phase == b.phase) and
           (a.level == b.level)

# 复合ID转整数（唯一编码）
func id_to_int*(id: CompoundID): int =
  result = id.shengmu
  result = (result shl 7) or id.yunmu
  result = (result shl 6) or id.shengdiao
  result = (result shl 5) or id.phase
  result = (result shl 3) or id.level

# 整数转复合ID
func int_to_id*(code: int): CompoundID =
  let mask5 = 0b11111
  let mask6 = 0b111111
  let mask7 = 0b1111111
  result.level = code and 0b111
  result.phase = (code shr 3) and mask5
  result.shengdiao = (code shr 8) and mask6
  result.yunmu = (code shr 14) and mask7
  result.shengmu = (code shr 21) and mask6

# ============================================================
# 创建坐标ID（坐标即ID）
# ============================================================

func make_coord_id*(x, y, z: int; lv: int): CoordinateID =
  result.x_phase = ((x mod 16) + 16) mod 16
  result.y_phase = ((y mod 16) + 16) mod 16
  result.z_phase = ((z mod 16) + 16) mod 16
  result.level = lv mod 5

# ============================================================
# 创建码位光标（不擅自命名ID）
# ============================================================

func new_code_cursor*(): CodeCursor =
  result.presence = 0
  result.code = make_compound_id(0, 0, 0, 0, 0)

# 移动光标（改变码位组合）
func move_code_cursor*(cursor: var CodeCursor; sm, ym, sd: int; ph: int; lv: int) =
  cursor.presence = 1
  cursor.code = make_compound_id(sm, ym, sd, ph, lv)

# 光标定位判断
func is_at_code*(cursor: CodeCursor; sm, ym, sd: int; ph: int; lv: int): bool =
  let target = make_compound_id(sm, ym, sd, ph, lv)
  result = (cursor.presence == 1) and id_equal(cursor.code, target)

# ============================================================
# 查询光标地址
# ============================================================

func cursor_address*(cursor: CodeCursor): string =
  if cursor.presence == 0:
    result = "无光标"
  else:
    let id_val = id_to_int(cursor.code)
    result = "ID=" & $id_val
    result &= " 声母:" & $cursor.code.shengmu
    result &= " 韵母:" & $cursor.code.yunmu
    result &= " 声调:" & $cursor.code.shengdiao
    result &= " 相位:" & $cursor.code.phase
    result &= " 层级:" & $cursor.code.level

# ============================================================
# 内存占用分析
# ============================================================

func memory_with_code_id*(): string =
  result = "码位即ID的内存占用：\n"
  result &= "  光标本身：1比特(存在) + 27比特(码位组合) = 28比特\n"
  result &= "  其中码位：声母6 + 韵母7 + 声调6 + 相位5 + 层级3 = 27比特\n"
  result &= "  \n"
  result &= "  传统方式：光标 + target_id = 1 + 32 = 33比特\n"
  result &= "  码位方式：光标 + 码位组合 = 1 + 27 = 28比特\n"
  result &= "  节省：5比特，且ID有意义（不是随意数字）\n"
  result &= "  \n"
  result &= "  关键：ID = 码位组合，不擅自命名\n"
  result &= "  \n"
  result &= "  组合数：48x66x60x18x5 = 17,107,200 种唯一ID\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 码位即ID·进制编码即定位 ==="
  echo ""

  # 码位信息
  echo "【码位位宽】"
  echo "  声母: ", CODE_BIT_WIDTH[0], " 比特 (48种)"
  echo "  韵母: ", CODE_BIT_WIDTH[1], " 比特 (66种)"
  echo "  声调: ", CODE_BIT_WIDTH[2], " 比特 (60种)"
  echo "  相位: ", CODE_BIT_WIDTH[5], " 比特 (18种)"
  echo "  层级: ", CODE_BIT_WIDTH[6], " 比特 (5种)"
  echo "  总计: ", compound_id_bits(), " 比特"
  echo "  组合: ", compound_id_states(), " 种唯一ID"
  echo ""

  # 创建码位光标
  echo "【码位光标】"
  var cursor = new_code_cursor()
  echo "  初始: ", cursor_address(cursor)
  echo ""

  # 移动光标到码位组合
  move_code_cursor(cursor, 1, 0, 0, 0, 0)
  echo "  移动后: ", cursor_address(cursor)
  echo ""

  # 创建复合ID
  echo "【复合ID（码位组合即ID）】"
  let cid1 = make_compound_id(1, 0, 0, 0, 0)
  let cid2 = make_compound_id(2, 1, 1, 1, 1)
  let cid3 = make_compound_id(0, 0, 0, 4, 2)

  echo "  ID1: 声母1 韵母0 声调0 相位0 层级0 -> 整数: ", id_to_int(cid1)
  echo "  ID2: 声母2 韵母1 声调1 相位1 层级1 -> 整数: ", id_to_int(cid2)
  echo "  ID3: 声母0 韵母0 声调0 相位4 层级2 -> 整数: ", id_to_int(cid3)
  echo ""

  # 整数ID还原
  echo "【整数ID还原为码位】"
  let restored = int_to_id(id_to_int(cid2))
  echo "  整数: ", id_to_int(cid2)
  echo "  还原: 声母", restored.shengmu, " 韵母", restored.yunmu, " 声调", restored.shengdiao
  echo "        相位", restored.phase, " 层级", restored.level
  echo ""

  # 光标定位判断
  echo "【光标定位判断】"
  move_code_cursor(cursor, 1, 0, 0, 0, 0)
  echo "  光标在(1,0,0,0,0): ", is_at_code(cursor, 1, 0, 0, 0, 0)
  echo "  光标在(2,1,1,1,1): ", is_at_code(cursor, 2, 1, 1, 1, 1)
  echo "  光标在(0,0,0,4,2): ", is_at_code(cursor, 0, 0, 0, 4, 2)
  echo ""

  # 坐标ID
  echo "【坐标ID（坐标即ID）】"
  let coord1 = make_coord_id(1, 0, 0, 2)
  let coord2 = make_coord_id(0, 0, 1, 4)
  let coord3 = make_coord_id(-1, 0, 0, 1)
  echo "  (1,0,0,2) -> 东-符号层: X=", coord1.x_phase, " Y=", coord1.y_phase, " Z=", coord1.z_phase, " L=", coord1.level
  echo "  (0,0,1,4) -> 天-范畴层: X=", coord2.x_phase, " Y=", coord2.y_phase, " Z=", coord2.z_phase, " L=", coord2.level
  echo "  (-1,0,0,1) -> 西-二进制层: X=", coord3.x_phase, " Y=", coord3.y_phase, " Z=", coord3.z_phase, " L=", coord3.level
  echo ""

  # 内存占用
  echo memory_with_code_id()
  echo ""

  echo "【总结】"
  echo "  1. ID不擅自主张命名，ID = 进制的码位组合"
  echo "  2. 声母48 x 韵母66 x 声调60 x 相位18 x 层级5 = 1710万种唯一ID"
  echo "  3. 码位组合可编码为整数，也可从整数还原"
  echo "  4. 光标定位 = 指向码位组合，不是指向随意编号"
  echo "  5. 三维坐标 = 相位x层级x进制，坐标即地址"
  echo "  -> 不擅自命名：码位就是ID，进制编码就是定位"
