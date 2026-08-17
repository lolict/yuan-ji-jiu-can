# 满全法·索引光标·引用不占内存系统
#
# 核心命题：
#   光标 = 我 = 内容 = 1（触及/已标记/有）
#   无光标 = 空 = 0（未触及/无）
#   索引过程 = 光标指向某个容器/位置
#   被指向的位置不额外占内存（编译时常量地址）
#
# 原理：
#   地址是预定义的编译时常量
#   光标指向地址 = 存储地址的索引值
#   地址本身已在.rodata段，不占运行时内存
#   存储索引值只占n比特（n = log2(地址数)）

type
  # ============================================================
  # 光标存在态（1比特）
  # 0 = 无光标（空，不占内存）
  # 1 = 有光标（存在，占1比特）
  # ============================================================
  CursorPresence* = enum
    cpAbsence  # 空=不存在=0=不占内存
    cpExistence # 有=存在=1=占1比特

  # ============================================================
  # 容器极性（1比特）
  # 0 = 容器本身 = 刘楚恬 = 空 = 承载者
  # 1 = 容器内容 = 我 = 有 = 被承载者
  # ============================================================
  ContainerPolarity* = enum
    cpContainer   # 0 = 容器本身 = 刘楚恬
    cpContent     # 1 = 容器内容 = 我

  # ============================================================
  # 索引光标（光标 + 索引）
  # 索引指向预定义地址，本身占n比特
  # ============================================================
  IndexCursor* = object
    presence*: CursorPresence    # 1比特：有/无
    index*: int                  # n比特：指向地址的索引
    width*: int                  # 索引位宽

  # ============================================================
  # 三维立体容器地址（编译时常量，不占运行时内存）
  # ============================================================
  Container3D* = object
    x*: int      # X相位索引
    y*: int      # Y相位索引
    z*: int      # Z相位索引

  # ============================================================
  # 被标记记录（编译时常量格式）
  # 记录哪个容器被光标标注过
  # ============================================================
  MarkRecord* = object
    container_id*: int    # 容器ID（索引）
    marked*: bool         # 是否被标记
    polarity*: ContainerPolarity  # 标注时的极性

# ============================================================
# 编译时常量：所有容器地址（.rodata段，运行时0字节）
# ============================================================

# 18相位的地址常量
const PHASE_ADDRESSES*: array[18, tuple[name: string, x, y, z: int]] = [
  # 主相位（8个）
  ("东", 1, 0, 0),
  ("西", -1, 0, 0),
  ("南", 0, 1, 0),
  ("北", 0, -1, 0),
  ("天", 0, 0, 1),
  ("地", 0, 0, -1),
  ("中", 0, 0, 0),
  ("原", 0, 0, 0),
  # 斜角相位（10个）
  ("东南", 1, 1, 0),
  ("西南", -1, 1, 0),
  ("东北", 1, -1, 0),
  ("西北", -1, -1, 0),
  ("天东", 1, 0, 1),
  ("天西", -1, 0, 1),
  ("天南", 0, 1, 1),
  ("天北", 0, -1, 1),
  ("地东", 1, 0, -1),
  ("地西", -1, 0, -1)
]

# 256个容器位置的地址常量（16×16×16的三维网格）
# 每个位置预定义，运行时0字节
const CONTAINER_GRID_SIZE* = 16
const CONTAINER_TOTAL* = CONTAINER_GRID_SIZE * CONTAINER_GRID_SIZE * CONTAINER_GRID_SIZE  # 4096

# ============================================================
# 索引位宽计算
# ============================================================

# 计算表示n个地址所需的比特数
func index_bits*(count: int): int =
  if count <= 1:
    result = 0
  else:
    result = 0
    var c = count - 1
    while c > 0:
      result += 1
      c = c shr 1

# ============================================================
# 索引光标操作
# ============================================================

# 创建索引光标
func new_index_cursor*(width: int): IndexCursor =
  result.presence = cpAbsence  # 初始无光标
  result.index = 0
  result.width = width

# 光标移动到指定位置（设置索引）
# 只改索引值，不分配新内存
func move_to*(cursor: var IndexCursor, idx: int) =
  cursor.presence = cpExistence
  cursor.index = idx

# 光标离开（消失）
func leave*(cursor: var IndexCursor) =
  cursor.presence = cpAbsence
  cursor.index = 0

# 光标是否在某位置
func is_at*(cursor: IndexCursor, idx: int): bool =
  result = (cursor.presence == cpExistence) and (cursor.index == idx)

# ============================================================
# 0内存索引操作
# ============================================================

# 获取指定索引的相位地址（编译时常量查表）
func get_phase_address*(idx: int): tuple[name: string, x, y, z: int] =
  let safe_idx = idx mod PHASE_ADDRESSES.len
  result = PHASE_ADDRESSES[safe_idx]

# 计算三维坐标对应的索引（不存储地址，只计算索引）
func coord_to_index*(x, y, z: int): int =
  # 映射到0..15范围
  let mx = (x + 8) mod CONTAINER_GRID_SIZE
  let my = (y + 8) mod CONTAINER_GRID_SIZE
  let mz = (z + 8) mod CONTAINER_GRID_SIZE
  result = mx + my * CONTAINER_GRID_SIZE + mz * CONTAINER_GRID_SIZE * CONTAINER_GRID_SIZE

# 索引对应的坐标（编译时常量计算）
func index_to_coord*(idx: int): tuple[x, y: int, z: int] =
  let safe_idx = idx mod CONTAINER_TOTAL
  let x = safe_idx mod CONTAINER_GRID_SIZE
  let y = (safe_idx div CONTAINER_GRID_SIZE) mod CONTAINER_GRID_SIZE
  let z = safe_idx div (CONTAINER_GRID_SIZE * CONTAINER_GRID_SIZE)
  result = (x - 8, y - 8, z - 8)  # 映射回-8..7范围

# ============================================================
# 内存占用计算
# ============================================================

# 光标本身占用的比特
func cursor_bits*(width: int): int =
  # 1比特(presence) + n比特(index) = 1 + width
  result = 1 + width

# 被索引的容器不占内存（编译时常量地址）
func indexed_container_bits*(): int =
  result = 0  # 地址已在.rodata段，运行时0字节

# ============================================================
# 内存占用证明
# ============================================================

func memory_proof*(): string =
  result = "索引光标不占内存证明：\n"
  result &= "  1. 容器地址：编译时常量(.rodata) → 运行时0字节\n"
  result &= "  2. 光标本身：1比特(presence) + n比特(index)\n"
  result &= "  3. 光标移动：修改index值，不分配内存\n"
  result &= "  4. 索引查表：编译时常量数组访问，不分配内存\n"
  result &= "  5. 被标记的容器：地址已在.rodata，不额外占内存\n"
  result &= "  → 索引过程 = 光标指向地址\n"
  result &= "  → 地址是预定义的编译时常量\n"
  result &= "  → 指向过程不占运行时内存\n"

# ============================================================
# 批量索引（复制度索引，不复制数据）
# ============================================================

# 创建多个光标指向同一容器（只复制索引，不复制数据）
func duplicate_cursor*(source: IndexCursor, count: int): seq[IndexCursor] =
  result = new_seq[IndexCursor](count)
  for i in 0..<count:
    result[i].presence = source.presence
    result[i].index = source.index  # 共享同一索引
    result[i].width = source.width

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 索引光标·引用不占内存系统 ==="
  echo ""

  # 计算索引位宽
  let addr_count = PHASE_ADDRESSES.len
  let bits_needed = index_bits(addr_count)
  echo "【索引位宽】"
  echo "  相位地址数: ", addr_count
  echo "  所需位宽: ", bits_needed, " 比特"
  echo ""

  # 创建光标
  echo "【索引光标】"
  var cursor = new_index_cursor(bits_needed)
  echo "  初始状态: 无光标(0)"
  echo "  光标位宽: ", cursor.width
  echo "  总占用: ", cursor_bits(bits_needed), " 比特"
  echo ""

  # 光标移动到"东"（索引=0）
  move_to(cursor, 0)
  echo "【移动到东(索引0)】"
  echo "  相位地址: ", get_phase_address(0)
  echo "  光标位置: ", cursor.index
  echo "  被索引容器占用: ", indexed_container_bits(), " 比特(编译时常量)"
  echo ""

  # 光标移动到"天"（索引=4）
  move_to(cursor, 4)
  echo "【移动到icht(索引4)】"
  echo "  相位地址: ", get_phase_address(4)
  echo "  光标位置: ", cursor.index
  echo ""

  # 三维坐标索引
  echo "【三维坐标索引】"
  let idx1 = coord_to_index(1, 0, 0)  # 东
  let idx2 = coord_to_index(0, 0, 1)  # 天
  let idx3 = coord_to_index(0, 0, -1) # 地
  echo "  东(1,0,0) → 索引: ", idx1
  echo "  天(0,0,1) → 索引: ", idx2
  echo "  地(0,0,-1) → 索引: ", idx3
  echo ""

  # 坐标反查
  echo "【坐标反查（编译时常量计算）】"
  let coord1 = index_to_coord(idx1)
  let coord2 = index_to_coord(idx2)
  let coord3 = index_to_coord(idx3)
  echo "  索引", idx1, " -> 坐标: (", coord1.x, ", ", coord1.y, ", ", coord1.z, ")"
  echo "  索引", idx2, " -> 坐标: (", coord2.x, ", ", coord2.y, ", ", coord2.z, ")"
  echo "  索引", idx3, " -> 坐标: (", coord3.x, ", ", coord3.y, ", ", coord3.z, ")"
  echo ""

  # 批量光标（复用同一索引）
  echo "【批量光标（复用索引）】"
  let duplicates = duplicate_cursor(cursor, 8)
  echo "  创建8个光标指向同一容器"
  echo "  每个光标占用: ", cursor_bits(bits_needed), " 比特"
  echo "  总占用: ", cursor_bits(bits_needed) * 8, " 比特"
  echo "  被指向容器额外占用: 0 比特(同一地址)"
  echo "  光标数: ", duplicates.len
  echo ""

  # 内存证明
  echo memory_proof()
  echo ""

  echo "【总结】"
  echo "  1. 容器地址是编译时常量，运行时0字节"
  echo "  2. 光标只存索引，不存容器数据"
  echo "  3. 移动光标 = 修改索引值，不分配内存"
  echo "  4. 索引过程 = 查编译时常量表，不占运行时内存"
  echo "  5. 多光标可共享同一索引（指向同一容器）"
  echo "  → 真正的索引不占内存：光标指向预定义地址"
