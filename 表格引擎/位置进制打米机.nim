# 位置进制 · 打米机 — 囡囝共生 · 位置是脚下的存在
# 框架来源：POSITIONAL_BASE_MILL.md

import math

# =============================================================================
# 角度 (384° 制式)
# =============================================================================

type
  Angle* = object
    degrees*: int32

const
  FullCircle* = 384  # 满全法角度制式
  HalfCircle* = FullCircle div 2  # 192° = 对冲

proc newAngle*(degrees: int32): Angle =
  ## 创建角度 (自动模 384)
  result.degrees = (degrees mod FullCircle + FullCircle) mod FullCircle

proc `+`*(a, b: Angle): Angle =
  result = newAngle(a.degrees + b.degrees)

proc `-`*(a, b: Angle): Angle =
  result = newAngle(a.degrees - b.degrees)

proc `$`*(a: Angle): string =
  result = $a.degrees & "°"

proc opposite*(a: Angle): Angle =
  ## 返回对冲角度 (相差 192°)
  result = newAngle(a.degrees + HalfCircle)

# =============================================================================
# 位置 (Pos) — 三维圆柱坐标
# =============================================================================

type
  Pos* = object
    level*: int32    # 层 (可负=阴面)
    ring*: uint32    # 环 (非负)
    angle*: Angle    # 角 (384° 制式)

const
  DefaultRingsPerLevel* = 8  # 每层默认 8 环

proc newPos*(level: int32, ring: uint32, degrees: int32): Pos =
  ## 创建位置
  result.level = level
  result.ring = ring
  result.angle = newAngle(degrees)

proc origin*(): Pos =
  ## 原点位置
  result = newPos(0, 0, 0)

# =============================================================================
# 行走 = 加法 (不直接写坐标)
# =============================================================================

proc descend*(p: var Pos, n: uint32) =
  ## 下沉 N 层 (level += n)
  p.level += int32(n)

proc ascend*(p: var Pos, n: uint32) =
  ## 上升 N 层 (level -= n)
  p.level -= int32(n)

proc expand*(p: var Pos, n: uint32) =
  ## 外扩 N 环 (ring += n)
  p.ring += n

proc contract*(p: var Pos, n: uint32) =
  ## 内缩 N 环 (ring -= n, saturating at 0)
  if n <= p.ring:
    p.ring -= n
  else:
    p.ring = 0

proc rotateCW*(p: var Pos, a: Angle) =
  ## 顺时针旋转
  p.angle = p.angle + a

proc rotateCCW*(p: var Pos, a: Angle) =
  ## 逆时针旋转
  p.angle = p.angle - a

proc walkAdd*(p: Pos, other: Pos): Pos =
  ## 行走加法：从原点出发，走 p 的步数，再走 other 的步数
  result = origin()
  
  # 走 p 的步数
  if p.level > 0:
    result.descend(uint32(p.level))
  elif p.level < 0:
    result.ascend(uint32(-p.level))
  
  if p.ring > 0:
    result.expand(p.ring)
  
  if p.angle.degrees != 0:
    result.rotateCW(p.angle)
  
  # 走 other 的步数
  if other.level > 0:
    result.descend(uint32(other.level))
  elif other.level < 0:
    result.ascend(uint32(-other.level))
  
  if other.ring > 0:
    result.expand(other.ring)
  
  if other.angle.degrees != 0:
    result.rotateCW(other.angle)

proc `+`*(p, other: Pos): Pos =
  ## 位置加法 (行走合成)
  result = p.walkAdd(other)

# =============================================================================
# 盲盒拆解 (unpack) — 只在输出给外部时调用
# =============================================================================

proc perRing*(): uint64 =
  ## 每环容量 = 满角 384°
  FullCircle.uint64

proc perLevel*(ringsPerLevel: int = DefaultRingsPerLevel): uint64 =
  ## 每层容量 = 默认 8 环 × 384°
  ringsPerLevel.uint64 * FullCircle.uint64

proc unpack*(p: Pos, ringsPerLevel: int = DefaultRingsPerLevel): uint64 =
  ## 位置 → 整数 (开盲盒)
  ## unpack = level_abs × (384 × 8) + ring × 384 + angle
  let levelAbs = abs(p.level).uint64
  let angleVal = p.angle.degrees.uint64
  
  result = levelAbs * perLevel(ringsPerLevel) + 
           p.ring.uint64 * perRing() + 
           angleVal

proc unpackSigned*(p: Pos, ringsPerLevel: int = DefaultRingsPerLevel): int64 =
  ## 位置 → 有符号整数 (正层=阳面，负层=阴面)
  let unsigned = p.unpack(ringsPerLevel)
  if p.level >= 0:
    result = int64(unsigned)
  else:
    result = -int64(unsigned)

proc fromUnpacked*(v: uint64, ringsPerLevel: int = DefaultRingsPerLevel): Pos =
  ## 整数 → 位置 (逆运算)
  let perLevelVal = perLevel(ringsPerLevel)
  let perRingVal = perRing()
  
  let levelAbs = v div perLevelVal
  let remainder = v mod perLevelVal
  let ring = remainder div perRingVal
  let angle = remainder mod perRingVal
  
  result.level = int32(levelAbs)
  result.ring = uint32(ring)
  result.angle.degrees = int32(angle)

proc fromUnpackedSigned*(v: int64, ringsPerLevel: int = DefaultRingsPerLevel): Pos =
  ## 有符号整数 → 位置
  if v >= 0:
    result = fromUnpacked(v.uint64, ringsPerLevel)
  else:
    result = fromUnpacked((-v).uint64, ringsPerLevel)
    result.level = -result.level

# =============================================================================
# 对冲与镜像
# =============================================================================

proc opposes*(p, other: Pos): bool =
  ## 两位置对冲：角相差 192°、层相同、环相同
  let angleDiff = (p.angle.degrees - other.angle.degrees + FullCircle) mod FullCircle
  result = (p.level == other.level) and
           (p.ring == other.ring) and
           (angleDiff == HalfCircle)

proc mirror*(p: Pos): Pos =
  ## 返回三维镜像位置 (角度取反 192°, 层环不变)
  result.level = p.level
  result.ring = p.ring
  result.angle = p.angle.opposite()

# =============================================================================
# 敬畏制度 (Convention) — 乘法开关
# =============================================================================

type
  Convention* = object
    multiplierEnabled*: bool

proc newConvention*(enabled: bool = false): Convention =
  result.multiplierEnabled = enabled

proc `$`*(c: Convention): string =
  if c.multiplierEnabled:
    "乘法已启用"
  else:
    "乘法已禁用 (敬畏制度)"

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 位置进制 · 打米机测试 ==="
  
  echo "\n--- 角度测试 ---"
  let a1 = newAngle(100)
  let a2 = newAngle(200)
  echo "a1 = ", a1
  echo "a2 = ", a2
  echo "a1 + a2 = ", a1 + a2
  echo "a1 对冲 = ", a1.opposite()
  
  echo "\n--- 位置创建 ---"
  let p1 = newPos(1, 2, 96)
  echo "p1: 层=", p1.level, " 环=", p1.ring, " 角=", p1.angle
  
  echo "\n--- 行走加法 ---"
  var p2 = origin()
  p2.descend(1)
  p2.expand(2)
  p2.rotateCW(newAngle(96))
  echo "p2 (行走): 层=", p2.level, " 环=", p2.ring, " 角=", p2.angle
  echo "p1 == p2: ", p1 == p2
  
  echo "\n--- 位置加法 ---"
  let p3 = p1 + newPos(0, 1, 48)
  echo "p1 + (0,1,48°): 层=", p3.level, " 环=", p3.ring, " 角=", p3.angle
  
  echo "\n--- 盲盒拆解 ---"
  let v = p1.unpack()
  echo "p1.unpack() = ", v
  let p4 = fromUnpacked(v)
  echo "fromUnpacked(", v, ") = 层=", p4.level, " 环=", p4.ring, " 角=", p4.angle
  echo "往返一致：", p1 == p4
  
  echo "\n--- 有符号拆解 ---"
  let p5 = newPos(-2, 3, 192)
  let vSigned = p5.unpackSigned()
  echo "p5 (负层): 层=", p5.level, " unpackSigned = ", vSigned
  let p6 = fromUnpackedSigned(vSigned)
  echo "fromUnpackedSigned = 层=", p6.level, " 环=", p6.ring, " 角=", p6.angle
  
  echo "\n--- 对冲测试 ---"
  let p7 = newPos(1, 2, 0)
  let p8 = newPos(1, 2, 192)
  echo "p7: 角=", p7.angle
  echo "p8: 角=", p8.angle
  echo "p7 opposes p8: ", p7.opposes(p8)
  
  echo "\n--- 镜像测试 ---"
  let p9 = newPos(1, 2, 48)
  let p9m = p9.mirror()
  echo "p9: 角=", p9.angle
  echo "p9.mirror(): 角=", p9m.angle
  echo "镜像后对冲：", p9.opposes(p9m)
  
  echo "\n--- 敬畏制度 ---"
  var conv = newConvention(false)
  echo "默认：", conv
  conv.multiplierEnabled = true
  echo "启用乘法：", conv
  
  echo "\n=== 测试完成 ==="
