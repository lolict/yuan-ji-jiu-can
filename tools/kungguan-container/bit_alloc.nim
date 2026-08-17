# 满全法·容器比特分配表
# 每个容器分配固定比特宽度，加法统计到层级
# 层级 = 比特流 → 二进制 → 符号 → 容器 → 范畴

import tables
import strutils

type
  BitTier* = enum
    btBitStream    # 比特流层（原始比特）
    btBinary       # 二进制层（8位字节）
    btSymbol       # 符号层（MCCP 码点）
    btContainer    # 容器层（容器索引）
    btCategory     # 范畴层（跨容器关系）

  # 容器比特分配定义
  ContainerBitAlloc* = object
    name*: string       # 容器名
    base*: int          # 进制基数
    bit_width*: int     # 分配的比特数
    tier*: BitTier      # 所属层级
    count*: int         # 可编码项数量

# ============================================================
# 分配原则：
# - 比特宽度 = ceil(log2(进制基数))
# - 同层级容器用加法统计
# - 层级之间用加法串联（非乘法）
# ============================================================

# 计算所需比特数
func calc_bits*(base: int): int =
  if base <= 1: return 0
  var v = base - 1
  while v > 0:
    result += 1
    v = v shr 1

# ============================================================
# 容器分配表
# ============================================================
const CONTAINER_ALLOC*: array[10, ContainerBitAlloc] = [
  # --- 比特流层（最底层，原始比特操作） ---
  ContainerBitAlloc(name:"声母",     base:48,  bit_width:6,  tier:btBitStream, count:48),
  ContainerBitAlloc(name:"变音小写", base:50,  bit_width:6,  tier:btBitStream, count:50),
  ContainerBitAlloc(name:"变音大写", base:50,  bit_width:6,  tier:btBitStream, count:50),

  # --- 二进制层（8位字节对齐） ---
  ContainerBitAlloc(name:"声调",     base:60,  bit_width:6,  tier:btBinary, count:60),
  ContainerBitAlloc(name:"韵母",     base:66,  bit_width:7,  tier:btBinary, count:66),
  ContainerBitAlloc(name:"颜色",     base:42,  bit_width:6,  tier:btBinary, count:42),

  # --- 符号层（MCCP 码点） ---
  ContainerBitAlloc(name:"声音",     base:100, bit_width:7,  tier:btSymbol, count:100),
  ContainerBitAlloc(name:"空间相位", base:18,  bit_width:5,  tier:btSymbol, count:18),

  # --- 容器层（容器索引） ---
  ContainerBitAlloc(name:"镜像对齐", base:10,  bit_width:4,  tier:btContainer, count:10),
  ContainerBitAlloc(name:"音乐层",   base:3,   bit_width:2,  tier:btContainer, count:3),
]

# ============================================================
# 层级统计
# ============================================================

# 某层级总比特
func tier_total*(tier: BitTier): int =
  result = 0
  for c in CONTAINER_ALLOC:
    if c.tier == tier:
      result += c.bit_width

# 某层级可编码项总数
func tier_capacity*(tier: BitTier): int =
  result = 0
  for c in CONTAINER_ALLOC:
    if c.tier == tier:
      result += c.count

# 加法统计：同层容器占用总比特
func tier_bits_used*(tier: BitTier): int =
  result = 0
  for c in CONTAINER_ALLOC:
    if c.tier == tier:
      result += c.bit_width

# ============================================================
# 查询
# ============================================================

func find_container*(name: string): int =
  for i, c in CONTAINER_ALLOC:
    if c.name == name: return i
  return -1

func container_bits*(name: string): int =
  let idx = find_container(name)
  if idx >= 0: return CONTAINER_ALLOC[idx].bit_width
  return -1

func container_tier*(name: string): BitTier =
  let idx = find_container(name)
  if idx >= 0: return CONTAINER_ALLOC[idx].tier
  return btBitStream

when is_main_module:
  echo "=== 满全法·容器比特分配表 ==="
  echo ""
  echo "【按层级统计】"
  for tier in BitTier:
    let names = block:
      var ns: seq[string] = @[]
      for c in CONTAINER_ALLOC:
        if c.tier == tier: ns.add(c.name)
      ns.join(", ")
    echo "  ", $tier, ": ", tier_total(tier), " bits | 可编码 ", tier_capacity(tier), " 项 | ", names

  echo ""
  echo "【容器明细】"
  for c in CONTAINER_ALLOC:
    echo "  ", c.name, " | ", $c.tier, " | ", c.bit_width, " bits | 基数 ", c.base, " | ", c.count, " 项"

  echo ""
  echo "【层级总览】"
  var grand_total = 0
  for tier in BitTier:
    let t = tier_total(tier)
    grand_total += t
    echo "  ", $tier, ": ", t, " bits"
  echo "  ─────────────────"
  echo "  总计: ", grand_total, " bits = ", grand_total / 8, " bytes"
