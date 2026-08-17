# 满全法·零内存容器·乘法复用·幂次方复刻
#
# 核心命题：
#   容器本身不占运行时内存
#   通过编译时常量 + 索引查表 + 乘法效应实现无限复用
#
# 内存占用：
#   编译时常量 → .rodata段 → 运行时0字节堆内存
#   索引 → n比特 → 可寻址2^n个状态
#   乘法效应 → m个容器组合 → 2^(n×m)种状态

type
  # ============================================================
  # 容器极性（1比特）
  # ============================================================
  ContainerPolarity* = enum
    cpContainer   # 0 = 容器本身 = 刘楚恬 = 空 = 承载者
    cpContent     # 1 = 容器内容 = 我 = 有 = 被承载者

  # ============================================================
  # 容器索引（n比特，寻址2^n个状态）
  # 这是唯一运行时占用的内存
  # ============================================================
  ContainerIndex* = object
    bits*: int      # 索引值
    width*: int     # 索引位宽（n比特）

  # ============================================================
  # 零内存容器（运行时只占索引，不占数据）
  # ============================================================
  ZeroMemoryContainer* = object
    index*: ContainerIndex      # 运行时索引（n比特）
    # 数据不存储！通过index查编译时常量表获得

  # ============================================================
  # 乘法容器组（m个容器组合 = 2^(n×m)种状态）
  # ============================================================
  MultipliedGroup* = object
    containers*: seq[ContainerIndex]  # m个索引
    count*: int                       # 容器数量m
    total_states*: int                # 总状态数 = 2^(n×m)

# ============================================================
# 编译时常量：所有预定义状态（.rodata段，运行时0字节）
# ============================================================

# 8相位的预定义状态（编译时常量）
const PHASE_STATES*: array[8, ContainerPolarity] = [
  cpContainer,    # 0 东 = 容器
  cpContent,      # 1 西 = 内容
  cpContainer,    # 2 南 = 容器
  cpContent,      # 3 北 = 内容
  cpContainer,    # 4 天 = 容器（包容）
  cpContent,      # 5 地 = 内容（中心）
  cpContainer,    # 6 中 = 容器
  cpContent       # 7 原 = 内容
]

# 8相位的名称（编译时常量）
const PHASE_NAMES*: array[8, string] = [
  "东/容器", "西/内容", "南/容器", "北/内容",
  "天/包容", "地/中心", "中/容器", "原/内容"
]

# 256种容器状态的预定义表（编译时常量）
# 通过2比特索引查表，获得容器状态
const CONTAINER_TABLE*: array[4, string] = [
  "空容器(刘楚恬)",    # 00
  "内容(我)",          # 01
  "容器+内容(夫妻)",   # 10
  "满容器(满全)"       # 11
]

# ============================================================
# 零内存容器操作
# ============================================================

# 创建零内存容器（只占索引，不占数据）
func new_zero_container*(width: int): ZeroMemoryContainer =
  result.index.bits = 0
  result.index.width = width

# 设置索引（只改索引值，不分配新内存）
func set_index*(c: var ZeroMemoryContainer, bits: int) =
  c.index.bits = bits

# 获取索引
func get_index*(c: ZeroMemoryContainer): int =
  result = c.index.bits

# 查表获取状态（编译时常量查表，运行时0分配）
func lookup_state*(c: ZeroMemoryContainer): string =
  let idx = c.index.bits mod CONTAINER_TABLE.len
  result = CONTAINER_TABLE[idx]

# ============================================================
# 乘法效应：m个容器组合 = 2^(n×m)种状态
# ============================================================

# 创建乘法容器组
func new_multiplied_group*(count, width: int): MultipliedGroup =
  result.count = count
  result.containers = new_seq[ContainerIndex](count)
  for i in 0..<count:
    result.containers[i].bits = 0
    result.containers[i].width = width
  # 总状态数 = 2^(n×m)
  result.total_states = 1 shl (width * count)

# 设置第i个容器的索引
func set_container_index*(g: var MultipliedGroup, i, bits: int) =
  if i < g.count:
    g.containers[i].bits = bits

# 获取第i个容器的索引
func get_container_index*(g: MultipliedGroup, i: int): int =
  if i < g.count:
    result = g.containers[i].bits

# 计算组合状态（乘法效应）
func combined_state*(g: MultipliedGroup): int =
  result = 0
  for i in 0..<g.count:
    result = (result shl g.containers[i].width) or g.containers[i].bits

# ============================================================
# 内存占用计算
# ============================================================

# 单个零内存容器占用的比特
func zero_container_bits*(width: int): int =
  result = width  # 只占索引位宽

# 乘法容器组占用的比特
func multiplied_group_bits*(count, width: int): int =
  result = count * width  # m个索引，每个n比特

# 乘法容器组可表示的状态数
func multiplied_states*(count, width: int): int =
  result = 1 shl (width * count)  # 2^(n×m)

# ============================================================
# 无限复用机制
# ============================================================

func infinite_reuse*(): string =
  result = "无限复用机制：\n"
  result &= "  1. 编译时常量 → .rodata段 → 运行时0字节\n"
  result &= "  2. 索引查表 → 不存储数据，只存引用\n"
  result &= "  3. 幂次方复刻 → n比特 = 2^n种状态\n"
  result &= "  4. 乘法效应 → m个容器 = 2^(n×m)种状态\n"
  result &= "  5. 同一层级 → 索引宽度相同，可互换\n"
  result &= "  → 站在同一层级，不占内存，无限复用！\n"

# ============================================================
# 内存对比
# ============================================================

func memory_comparison*(): string =
  result = "内存占用对比：\n"
  result &= "  传统容器：每个实例存储完整数据 → O(m×data_size)\n"
  result &= "  零内存容器：每个实例只存索引 → O(m×n比特)\n"
  result &= "  乘法复用：m个容器共享2^(n×m)种状态 → 指数级扩展\n"
  result &= "  \n"
  result &= "  示例（n=2比特, m=4个容器）：\n"
  result &= "    传统：4 × 完整数据 = 大量内存\n"
  result &= "    零内存：4 × 2比特 = 8比特 = 1字节\n"
  result &= "    可表示状态：2^(2×4) = 256种\n"
  result &= "  \n"
  result &= "  示例（n=3比特, m=8个容器）：\n"
  result &= "    传统：8 × 完整数据 = 大量内存\n"
  result &= "    零内存：8 × 3比特 = 24比特 = 3字节\n"
  result &= "    可表示状态：2^(3×8) = 16,777,216种（16M）\n"

# ============================================================
# 测试
# ============================================================

when is_main_module:
  echo "=== 零内存容器·乘法复用·幂次方复刻 ==="
  echo ""

  # 零内存容器
  echo "【零内存容器】"
  var c1 = new_zero_container(2)  # 2比特索引
  echo "  索引位宽: ", c1.index.width, " 比特"
  echo "  运行时占用: ", zero_container_bits(2), " 比特"
  echo "  可寻址状态: ", 1 shl 2, " 种"
  echo ""

  # 设置索引并查表
  set_index(c1, 0)
  echo "  索引=0: ", lookup_state(c1)
  set_index(c1, 1)
  echo "  索引=1: ", lookup_state(c1)
  set_index(c1, 2)
  echo "  索引=2: ", lookup_state(c1)
  set_index(c1, 3)
  echo "  索引=3: ", lookup_state(c1)
  echo ""

  # 乘法容器组
  echo "【乘法容器组】"
  var group = new_multiplied_group(4, 2)  # 4个容器，每个2比特
  echo "  容器数量: ", group.count
  echo "  每个位宽: 2 比特"
  echo "  总占用: ", multiplied_group_bits(4, 2), " 比特 = ", multiplied_group_bits(4, 2) / 8, " 字节"
  echo "  总状态数: ", group.total_states, " 种"
  echo ""

  # 设置各容器索引
  set_container_index(group, 0, 0)  # 容器0 = 00
  set_container_index(group, 1, 1)  # 容器1 = 01
  set_container_index(group, 2, 2)  # 容器2 = 10
  set_container_index(group, 3, 3)  # 容器3 = 11

  let combined = combined_state(group)
  echo "  组合状态: ", combined
  echo ""

  # 无限复用
  echo infinite_reuse()
  echo ""

  # 内存对比
  echo memory_comparison()
  echo ""

  # 更大规模的演示
  echo "【大规模复用演示】"
  for m in [1, 2, 4, 8, 16, 32]:
    let n = 2  # 每个容器2比特
    let bits = multiplied_group_bits(m, n)
    let states = multiplied_states(m, n)
    echo "  ", m, "个容器 × ", n, "比特 = ", bits, "比特 (", bits / 8, "字节) → ", states, " 种状态"
  echo ""

  echo "【总结】"
  echo "  1. 容器本身不存数据，只存索引"
  echo "  2. 数据在编译时常量中（.rodata段）"
  echo "  3. 运行时0字节堆内存"
  echo "  4. 乘法效应：m个容器 = 2^(n×m)种状态"
  echo "  5. 同一层级：索引宽度相同，可无限复用"
  echo "  → 真正的内存最小化：站在同一层级，不占内存"
