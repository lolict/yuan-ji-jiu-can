# maolilan 毛利兰空间索引 (Space Index)
# 功能：为合并后的符号分配空间坐标，提供可视定位
# maolilan = 空间场域 / 镜像 / 交互层 / 表面属性
# "空间索引/镜像关系/可定位场所"

type
  Cell64* = object
    coord*: uint64    # 64位坐标
    polarity*: bool    # 极性：true=阳(丈夫道) false=阴(妻子道)
    occupied*: bool    # 是否已被占用
    symbol*: string    # 占用该位置的符号名称

  Container* = object
    level*: int        # 层级（一元至九元）
    capacity*: int     # 容器容量（2的幂次）
    cells*: seq[Cell64]  # 内部64位单元序列
    parent*: int       # 父容器索引（-1=无父容器）
  
  SpaceIndex* = object
    containers*: seq[Container]
    anchor0_pos*: uint64   # "我=1"的空间坐标
    anchor1_pos*: uint64   # "她=0"的空间坐标

# 创建一个新的空间索引
func new_space_index*(): SpaceIndex =
  var cells: seq[Cell64] = @[]
  # 初始化64个基础单元
  for i in 0..63:
    cells.add(Cell64(
      coord: uint64(i),
      polarity: (i mod 2 == 1),  # 奇数=阳，偶数=阴
      occupied: false,
      symbol: ""
    ))
  
  let root = Container(
    level: 1,
    capacity: 64,
    cells: cells,
    parent: -1
  )
  
  return SpaceIndex(
    containers: @[root],
    anchor0_pos: 0,    # "我=1"在坐标0
    anchor1_pos: 63    # "她=0"在坐标63
  )

# 计算两个坐标之间的距离（耦合度）
func spatial_distance*(a: SpaceIndex, pos1, pos2: uint64): uint64 =
  if pos1 > pos2: return pos1 - pos2
  else: return pos2 - pos1

# 在容器中为符号分配坐标
func place_symbol*(s: var SpaceIndex, sym_name: string, level: int, 
                  pol: bool): uint64 =
  # 找到对应层级的容器
  var container_idx = -1
  for i, c in s.containers:
    if c.level == level:
      container_idx = i
      break
  
  if container_idx == -1:
    # 容器不存在，创建新层级
    var new_cells: seq[Cell64] = @[]
    let cap = 64 * (1 shl (level - 1))  # 每层容量翻倍
    for i in 0..(cap - 1):
      new_cells.add(Cell64(
        coord: uint64(i),
        polarity: (i mod 2 == 1),
        occupied: false,
        symbol: ""
      ))
    s.containers.add(Container(
      level: level,
      capacity: cap,
      cells: new_cells,
      parent: if s.containers.len > 0: s.containers.len - 1 else: -1
    ))
    container_idx = s.containers.len - 1
  
  # 找到极性匹配的空位
  var cell_idx = -1
  var i = 0
  while i < s.containers[container_idx].cells.len:
    let c = s.containers[container_idx].cells[i]
    if not c.occupied and c.polarity == pol:
      cell_idx = i
      break
    i += 1
  
  if cell_idx == -1:
    return high(uint64)  # 没有空位
  
  s.containers[container_idx].cells[cell_idx].occupied = true
  s.containers[container_idx].cells[cell_idx].symbol = sym_name
  return uint64(cell_idx)

# 查找符号的坐标
func find_symbol*(s: SpaceIndex, sym_name: string): (int, uint64) =
  # 遍历容器找符号
  for ci, c in s.containers:
    for i, cell in c.cells:
      if cell.symbol == sym_name:
        return (c.level, uint64(i))
  return (-1, high(uint64))

# 打印空间索引状态 (有副作用，用proc)
proc print_space*(s: SpaceIndex) =
  echo "=== maolilan 空间索引 ==="
  echo "锚点: 我=1 @坐标0 | 她=0 @坐标63"
  echo "容器数量: ", s.containers.len
  for c in s.containers:
    var occupied_count = 0
    var symbol_list = ""
    for cell in c.cells:
      if cell.occupied:
        occupied_count += 1
        if symbol_list.len > 0: symbol_list &= ", "
        symbol_list &= cell.symbol & " @" & $cell.coord
    echo "  L", c.level, " [", occupied_count, "/", c.capacity, "] ", symbol_list

# ===== 测试 =====
when is_main_module:
  echo "=== maolilan 毛利兰空间索引测试 ==="
  
  var space = new_space_index()
  
  # 放置符号到空间
  let pos1 = space.place_symbol("lctqf", 1, false)   # 妻子道 = 偶数位
  let pos2 = space.place_symbol("lolict", 1, true)   # 丈夫道 = 奇数位
  let pos3 = space.place_symbol("fqmy", 1, true)     # 密意
  
  echo "符号放置完成:"
  echo "  lctqf @", pos1
  echo "  lolict @", pos2
  echo "  fqmy @", pos3
  
  # 计算距离
  echo "\n空间距离:"
  echo "  ← lolict 到 lctqf → ", spatial_distance(space, pos2, pos1)
  echo "  ← fqmy 到 lctqf → ", spatial_distance(space, pos3, pos1)
  
  # 查找符号
  let (level, coord) = find_symbol(space, "lctqf")
  echo "\n查找 lctqf: L", level, " @", coord
  
  # 打印空间状态
  echo ""
  print_space(space)
