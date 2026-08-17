# 满全法·最小比特容器
# 用 2-4 bits 的容器头，包裹任意大的数据
# 容器头 = 类型标识 + 层级 + 引用方式
# 容器体 = 引用到外部大数据（不占用容器自身空间）

import tables

# ============================================================
# 容器类型（2 bits 可标识 4 种基础类型）
# ============================================================
type
  MiniType* = enum
    mtScalar    # 00 = 标量（直接值，容器内存储）
    mtRef       # 01 = 引用（指向外部表）
    mtStream    # 10 = 流（指向连续数据块）
    mtCategory  # 11 = 范畴（指向跨容器关系）

# ============================================================
# 层级（2 bits 可标识 4 层）
# ============================================================
type
  MiniTier* = enum
    mtBitStream  # 00 = 比特流层
    mtBinary     # 01 = 二进制层
    mtSymbol     # 10 = 符号层
    mtContainer  # 11 = 容器层

# ============================================================
# 最小容器头 = 4 bits
# 类型(2) + 层级(2) = 4 bits
# ============================================================
type
  MiniHeader* = object
    ctype*: MiniType    # 2 bits
    tier*: MiniTier     # 2 bits

# ============================================================
# 容器体（根据类型不同，占用不同）
# ============================================================
type
  MiniContainer* = object
    header*: MiniHeader          # 4 bits 头
    case kind*: MiniType
    of mtScalar:
      direct_value*: int         # 直接值（小数据，< 256 用 8 bits）
    of mtRef:
      table_id*: int             # 表编号（4 bits = 16 张表）
      item_index*: int           # 表内索引（16 bits = 65536 项）
    of mtStream:
      stream_offset*: int        # 流偏移（32 bits = 4GB 范围）
      stream_length*: int        # 流长度（16 bits = 64KB）
    of mtCategory:
      cross_ref*: int            # 跨容器引用（24 bits = 16M 关系）

# ============================================================
# 大数据表（被容器引用，不占用容器空间）
# ============================================================
type
  BigDataTable* = object
    name*: string
    items*: seq[string]          # 大数据项（诗、音乐、编码表...）

# ============================================================
# 全局表注册表
# ============================================================
var TABLES*: seq[BigDataTable] = @[]

proc register_table*(name: string, items: seq[string]): int =
  result = TABLES.len
  TABLES.add(BigDataTable(name: name, items: items))

proc get_item*(table_id, item_idx: int): string =
  if table_id >= 0 and table_id < TABLES.len:
    if item_idx >= 0 and item_idx < TABLES[table_id].items.len:
      return TABLES[table_id].items[item_idx]
  return ""

# ============================================================
# 创建最小容器
# ============================================================

# 标量容器（4 bits 头 + 8 bits 值 = 12 bits = 1.5 bytes）
func make_scalar*(tier: MiniTier, value: int): MiniContainer =
  result.header = MiniHeader(ctype: mtScalar, tier: tier)
  result.kind = mtScalar
  result.direct_value = value

# 引用容器（4 bits 头 + 4 bits 表号 + 16 bits 索引 = 24 bits = 3 bytes）
func make_ref*(tier: MiniTier, table_id, item_idx: int): MiniContainer =
  result.header = MiniHeader(ctype: mtRef, tier: tier)
  result.kind = mtRef
  result.table_id = table_id
  result.item_index = item_idx

# 流容器（4 bits 头 + 32 bits 偏移 + 16 bits 长度 = 52 bits = 6.5 bytes）
func make_stream*(tier: MiniTier, offset, length: int): MiniContainer =
  result.header = MiniHeader(ctype: mtStream, tier: tier)
  result.kind = mtStream
  result.stream_offset = offset
  result.stream_length = length

# ============================================================
# 容器占用比特统计
# ============================================================

func header_bits*(): int = 4  # 类型2 + 层级2

func container_bits*(c: MiniContainer): int =
  result = header_bits()
  case c.kind
  of mtScalar:   result += 8    # 直接值 8 bits
  of mtRef:      result += 4 + 16  # 表号4 + 索引16
  of mtStream:   result += 32 + 16 # 偏移32 + 长度16
  of mtCategory: result += 24   # 跨容器引用

# ============================================================
# 演示：用最小容器包裹最大数据
# ============================================================
when is_main_module:
  echo "=== 最小比特容器 ==="
  echo ""

  # 注册大数据表（这些表可以非常大，但不占用容器空间）
  let poem_a_id = register_table("诗A", @[
    "第一行：春眠不觉晓",
    "第二行：处处闻啼鸟",
    "第三行：夜来风雨声",
    "第四行：花落知多少",
  ])

  let poem_b_id = register_table("诗B", @[
    "第一行：白日依山尽",
    "第二行：黄河入海流",
    "第三行：欲穷千里目",
    "第四行：更上一层楼",
  ])

  let music_id = register_table("音乐表", @[
    "C4-480-80-激情", "E4-480-70-宁静", "G4-480-75-快乐", "C5-960-90-神秘",
    "D4-240-85-紧张", "F4-240-75-柔和", "A4-480-80-明亮", "B4-480-70-忧郁",
  ])

  let encoding_id = register_table("编码表", @[
    "a-开口呼-0", "o-开口呼-1", "e-开口呼-2",
    "i-齐齿呼-0", "u-合口呼-0", "ü-撮口呼-0",
    "ai-复韵母-0", "ei-复韵母-1", "ao-复韵母-2", "ou-复韵母-3",
  ])

  echo "【大数据表已注册】"
  echo "  诗A: 4 行"
  echo "  诗B: 4 行"
  echo "  音乐: 8 音符"
  echo "  编码: 10 项"
  echo ""

  echo "【最小容器占用】"
  echo "  容器头: ", header_bits(), " bits"
  echo ""

  # 创建容器
  let c_scalar = make_scalar(mtBinary, 42)
  let c_poem_a  = make_ref(mtSymbol, poem_a_id, 0)   # 指向诗A第0行
  let c_poem_b  = make_ref(mtSymbol, poem_b_id, 2)   # 指向诗B第2行
  let c_music   = make_ref(mtSymbol, music_id, 3)    # 指向音乐第3音符
  let c_encoding = make_ref(mtSymbol, encoding_id, 5) # 指向编码第5项

  echo "标量容器(值=42):     ", container_bits(c_scalar), " bits"
  echo "诗A引用(第0行):      ", container_bits(c_poem_a), " bits"
  echo "诗B引用(第2行):      ", container_bits(c_poem_b), " bits"
  echo "音乐引用(第3音符):   ", container_bits(c_music), " bits"
  echo "编码引用(第5项):     ", container_bits(c_encoding), " bits"
  echo ""

  echo "【容器解引用】"
  echo "  标量值: ", c_scalar.direct_value
  echo "  诗A第0行: ", get_item(c_poem_a.table_id, c_poem_a.item_index)
  echo "  诗B第2行: ", get_item(c_poem_b.table_id, c_poem_b.item_index)
  echo "  音乐第3音符: ", get_item(c_music.table_id, c_music.item_index)
  echo "  编码第5项: ", get_item(c_encoding.table_id, c_encoding.item_index)
  echo ""

  echo "【核心思想】"
  echo "  容器只占 ", container_bits(c_poem_a), " bits (3 bytes)"
  echo "  但指向的数据可以无限大 (整首诗、整段音乐、整张编码表)"
  echo "  这就是：最小内存的容器，包裹最大内存的内容"
