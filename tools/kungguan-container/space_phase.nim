# 满全法·空间相位容器
# 把方位词编码为三维空间中的可计算位置
# 相位 = 容器在空间中的站位（十字架/圆形/九宫格）
# 层级 = 加法线排布（同层多容器，层级间用加法统计）

type
  # ============================================================
  # 层级（加法线排布）
  # 同一层级可放置多个容器，用加法统计到比特流
  # ============================================================
  LevelTier* = enum
    ltBitStream    # 比特流（最底层）
    ltBinary       # 二进制流
    ltSymbol       # 符号编码
    ltContainer    # 容器层
    ltCategory     # 范畴层

  # ============================================================
  # 相位命名方式（三种可选）
  # ============================================================
  PhaseMode* = enum
    pmCross        # 十字架（6方向：东西南北天地）
    pmCircular     # 圆形（360度）
    pmGrid         # 九宫格（位置坐标）

  # ============================================================
  # 绝对空间（东南西北—绝对坐标系）
  # ============================================================
  AbsoluteDir* = enum
    adEast      # 东
    adWest      # 西
    adSouth     # 南
    adNorth     # 北
    adHeaven    # 天/上
    adEarth     # 地/下
    adCenter    # 中央/极心

  # ============================================================
  # 相对空间（前后左右里外—以观察者为锚点）
  # ============================================================
  RelativeDir* = enum
    rdFront     # 前
    rdBack      # 后
    rdLeft      # 左
    rdRight     # 右
    rdInside    # 里
    rdOutside   # 外
    rdMiddle    # 中

  # ============================================================
  # 人事定位（以人体为锚点—头肩腰脚侧）
  # ============================================================
  BodyAnchor* = enum
    baHead      # 头
    baShoulder  # 肩
    baChest     # 胸
    baWaist     # 腰
    baSide      # 侧（左）
    baHand      # 手（右）
    baFoot      # 脚
    baFront     # 正
    baBack      # 背

  # ============================================================
  # 三维空间坐标
  # ============================================================
  Space3D* = object
    x*: int     # 东西轴（东为正）
    y*: int     # 南北轴（南为正）
    z*: int     # 天地轴（天为上）

  # ============================================================
  # 相位节点（容器在空间中的站位）
  # ============================================================
  PhaseNode* = object
    name*: string           # 汉字名（东、前、头...）
    tier*: LevelTier        # 所在层级
    abs_dir*: AbsoluteDir   # 绝对方向（若有）
    rel_dir*: RelativeDir   # 相对方向（若有）
    body*: BodyAnchor       # 人体锚点（若有）
    coord*: Space3D         # 三维坐标
    symbol*: string         # 对应编码符号

  # ============================================================
  # 集装箱站位（十字架/斜线/九宫格）
  # ============================================================
  ContainerSlot* = enum
    csCenter     # 中央/极心（#0）
    csEast       # 东（十字架）
    csWest       # 西
    csSouth      # 南
    csNorth      # 北
    csHeaven     # 天/上
    csEarth     # 地/下
    csEastSouth  # 东南（斜线角）
    csWestSouth  # 西南
    csEastNorth  # 东北
    csWestNorth  # 西北
    csHeavenNorth # 天北角
    csEarthNorth  # 地北角
    csHeavenSouth # 天南角
    csEarthSouth  # 地南角
    csHeavenEast  # 天东角
    csEarthEast   # 地东角
    csHeavenWest  # 天西角
    csEarthWest   # 地西角

  # ============================================================
  # 容器定义
  # ============================================================
  SpaceContainer* = object
    slots*: seq[PhaseNode]      # 已占用的站位
    tier*: LevelTier            # 当前层级
    bit_width*: int             # 占用比特数（3/4/5...）
    phase_mode*: PhaseMode      # 当前相位模式

# 前向声明：Space3D 转字符串（必须在 echo 之前可用）
func `$`*(s: Space3D): string = "(" & $s.x & "," & $s.y & "," & $s.z & ")"

# ============================================================
# 加法层级统计
# ============================================================

# 同层容器用加法统计（非乘法）
func add_level_count*(containers: seq[SpaceContainer], tier: LevelTier): int =
  result = 0
  for c in containers:
    if c.tier == tier:
      result += c.bit_width

# 当前层剩余可容纳比特
func level_remaining*(tier: LevelTier, used: int): int =
  # 假设每层级总宽度为 64 比特（可配置）
  let total_width = 64
  result = total_width - used

# ============================================================
# 相位查找
# ============================================================

func find_slot*(c: SpaceContainer, name: string): int =
  for i, s in c.slots:
    if s.name == name: return i
  return -1

func is_slot_occupied*(c: SpaceContainer, slot: ContainerSlot): bool =
  for s in c.slots:
    case slot
    of csCenter:
      if s.coord.x == 0 and s.coord.y == 0 and s.coord.z == 0: return true
    of csEast:
      if s.coord.x > 0 and s.coord.y == 0 and s.coord.z == 0: return true
    of csWest:
      if s.coord.x < 0 and s.coord.y == 0 and s.coord.z == 0: return true
    of csSouth:
      if s.coord.y > 0 and s.coord.x == 0 and s.coord.z == 0: return true
    of csNorth:
      if s.coord.y < 0 and s.coord.x == 0 and s.coord.z == 0: return true
    of csHeaven:
      if s.coord.z > 0 and s.coord.x == 0 and s.coord.y == 0: return true
    of csEarth:
      if s.coord.z < 0 and s.coord.x == 0 and s.coord.y == 0: return true
    else:
      # 斜线简化判断
      discard
  return false

# ============================================================
# 方向 → 坐标映射
# ============================================================

func abs_to_coord*(d: AbsoluteDir): Space3D =
  case d
  of adEast:      result = Space3D(x: 1, y: 0, z: 0)
  of adWest:      result = Space3D(x: -1, y: 0, z: 0)
  of adSouth:     result = Space3D(x: 0, y: 1, z: 0)
  of adNorth:     result = Space3D(x: 0, y: -1, z: 0)
  of adHeaven:    result = Space3D(x: 0, y: 0, z: 1)
  of adEarth:     result = Space3D(x: 0, y: 0, z: -1)
  of adCenter:    result = Space3D(x: 0, y: 0, z: 0)

func rel_to_coord*(d: RelativeDir): Space3D =
  case d
  of rdFront:     result = Space3D(x: 0, y: 1, z: 0)   # 前=南
  of rdBack:      result = Space3D(x: 0, y: -1, z: 0)  # 后=北
  of rdLeft:      result = Space3D(x: -1, y: 0, z: 0)  # 左=西
  of rdRight:     result = Space3D(x: 1, y: 0, z: 0)   # 右=东
  of rdInside:    result = Space3D(x: 0, y: 0, z: 0)   # 里=中
  of rdOutside:   result = Space3D(x: 0, y: 0, z: 0)   # 外=_offset
  of rdMiddle:    result = Space3D(x: 0, y: 0, z: 0)   # 中=中央

func body_to_coord*(b: BodyAnchor): Space3D =
  case b
  of baHead:      result = Space3D(x: 0, y: 0, z: 2)   # 头=天之上
  of baShoulder:  result = Space3D(x: 0, y: 0, z: 1)   # 肩=天
  of baChest:     result = Space3D(x: 0, y: 0, z: 0)   # 胸=中央
  of baWaist:     result = Space3D(x: 0, y: 0, z: -1)  # 腰=地
  of baFoot:      result = Space3D(x: 0, y: 0, z: -2)  # 脚=地之下
  of baSide:      result = Space3D(x: -1, y: 0, z: 0)  # 侧=西
  of baHand:      result = Space3D(x: 1, y: 0, z: 0)   # 手=东
  of baFront:     result = Space3D(x: 0, y: 1, z: 0)   # 正=前（南）
  of baBack:      result = Space3D(x: 0, y: -1, z: 0)  # 后=北

# ============================================================
# 九宫格索引
# ============================================================

func nine_grid*(c: SpaceContainer): array[9, string] =
  # 九宫格位置：
  # 0 1 2  (西北 北 东北)
  # 3 4 5  (西  中 东 )
  # 6 7 8  (西南 南 东南)
  for i in 0..8: result[i] = ""
  for s in c.slots:
    let gx = s.coord.x + 1  # 0,1,2
    let gy = s.coord.y + 1  # 0,1,2
    let idx = gy * 3 + gx
    if idx >= 0 and idx < 9:
      result[idx] = s.name

when is_main_module:
  echo "=== 空间相位容器 ==="
  echo ""

  echo "绝对方向坐标："
  echo "  东: ", abs_to_coord(adEast)
  echo "  西: ", abs_to_coord(adWest)
  echo "  南: ", abs_to_coord(adSouth)
  echo "  北: ", abs_to_coord(adNorth)
  echo "  天: ", abs_to_coord(adHeaven)
  echo "  地: ", abs_to_coord(adEarth)
  echo ""

  echo "相对方向坐标："
  echo "  前: ", rel_to_coord(rdFront)
  echo "  后: ", rel_to_coord(rdBack)
  echo "  左: ", rel_to_coord(rdLeft)
  echo "  右: ", rel_to_coord(rdRight)
  echo ""

  echo "人体锚点坐标："
  echo "  头: ", body_to_coord(baHead)
  echo "  胸: ", body_to_coord(baChest)
  echo "  腰: ", body_to_coord(baWaist)
  echo "  脚: ", body_to_coord(baFoot)
  echo ""

  echo "加法层级统计："
  var containers = @[
    SpaceContainer(tier:ltBitStream, bit_width:3),
    SpaceContainer(tier:ltBitStream, bit_width:4),
    SpaceContainer(tier:ltBitStream, bit_width:5),
    SpaceContainer(tier:ltBinary, bit_width:8),
  ]
  let bitstream_used = add_level_count(containers, ltBitStream)
  let binary_used = add_level_count(containers, ltBinary)
  echo "  比特流层已用: ", bitstream_used, " bits (3+4+5)"
  echo "  二进制层已用: ", binary_used, " bits (8)"
  echo "  比特流层剩余: ", level_remaining(ltBitStream, bitstream_used), " bits"
