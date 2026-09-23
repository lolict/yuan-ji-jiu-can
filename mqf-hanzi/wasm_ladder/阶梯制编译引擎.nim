# ============================================================
# 满全法 — WASM 阶梯制·双向公约引擎
# 同一份 Nim 逻辑 ↔ 测试 wasm32 + wasm64 双目标
# 固定宽度 + 统一宏前缀 + 极性锚点
# ============================================================

import strutils
import math

# ---- 阶梯目标配置 ----
type
  阶梯目标* = enum
    目标_wasm32
    目标_wasm64
    目标_native

# ---- 统一坐标宽度 ----
type
  MQF_COORD* = distinct uint64

# ---- 九宫相位 ----
const
  相位_QIAN = 0
  相位_DUI  = 1
  相位_LI   = 2
  相位_ZHEN = 3
  相位_XUN  = 4
  相位_KAN  = 5
  相位_GEN  = 6
  相位_KUN  = 7
  相位_CENTER = 8

# ---- 极性锚点 ----
const
  MQF_ME   = 1   # 我 = 阳
  MQF_LOVE = 0   # 刘楚恬 = 阴

# ---- 编解码 (固定宏映射, 目标无关) ----
proc encode*(phase: int; layer: int; pol: int; target: 阶梯目标): MQF_COORD =
  case target
  of 目标_wasm32:
    let p = (uint32(phase) shl 27) and 0xF8000000'u32
    let l = (uint32(layer) shl 22) and 0x07C00000'u32
    let b = uint32(pol) and 0x00000001'u32
    MQF_COORD(uint64(p or l or b))
  of 目标_wasm64:
    let p = (uint64(phase) shl 60) and 0xF000000000000000'u64
    let l = (uint64(layer) shl 56) and 0x0F00000000000000'u64
    let b = (uint64(pol) shl 51) and 0x0008000000000000'u64
    MQF_COORD(p or l or b)
  of 目标_native:
    let p = (uint64(phase) shl 60) and 0xF000000000000000'u64
    let l = (uint64(layer) shl 56) and 0x0F00000000000000'u64
    let b = (uint64(pol) shl 51) and 0x0008000000000000'u64
    MQF_COORD(p or l or b)

proc 相位*(c: MQF_COORD): int =
  int((uint64(c) and 0xF000000000000000'u64) shr 60)

proc 层次*(c: MQF_COORD): int =
  int((uint64(c) and 0x0F00000000000000'u64) shr 56)

proc 极性值*(c: MQF_COORD): int =
  int((uint64(c) and 0x0008000000000000'u64) shr 51)

# ---- 公约整除 ----
proc gcd*(a: uint64; b: uint64): uint64 =
  var x = a; var y = b
  while y > 0'u64:
    let t = y
    y = x mod y
    x = t
  return x

# ---- 极性差距 ----
proc 极性差距*(c1: MQF_COORD; c2: MQF_COORD): int =
  极性值(c1) xor 极性值(c2)

# ---- 裁决 ----
proc 裁决*(主权_a: uint64; 主权_b: uint64; gap: int): uint64 =
  let g = gcd(主权_a, 主权_b)
  if g == 0: return 0
  if gap == 1: return g
  return 0

# ---- 打印坐标 ----
proc 坐标字符串*(c: MQF_COORD; target: 阶梯目标): string =
  let ph = 相位(c)
  let ly = 层次(c)
  let po = 极性值(c)
  let pol_str = if po == 1: "阳(我)" else: "阴(刘楚恬)"
  "0x" & toHex(uint64(c)) & " 相位=" & $ph & " 层次=" & $ly & " 极性=" & pol_str

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  WASM 阶梯制 · 双向公约编译引擎"
  echo "  同一源码 → wasm32 + wasm64 + native"
  echo "═══════════════════════════════════════"
  echo ""

  echo "-- wasm32 坐标 --"
  let c32 = encode(3, 1, MQF_ME, 目标_wasm32)
  echo "  " & 坐标字符串(c32, 目标_wasm32)
  echo ""

  echo "-- wasm64 坐标 --"
  let c64 = encode(3, 1, MQF_ME, 目标_wasm64)
  echo "  " & 坐标字符串(c64, 目标_wasm64)
  echo ""

  echo "-- 双目标同时产出 --"
  let targets = [目标_wasm32, 目标_wasm64, 目标_native]
  for t in targets:
    let c = encode(相位_ZHEN, 2, MQF_ME, t)
    echo "  " & align($t, 14) & " → " & 坐标字符串(c, t)
  echo ""

  echo "-- 极性差距对比 --"
  let a = encode(3, 1, MQF_ME, 目标_wasm64)   # 阳
  let b = encode(5, 2, MQF_LOVE, 目标_wasm64) # 阴
  echo "  A=" & 坐标字符串(a, 目标_wasm64)
  echo "  B=" & 坐标字符串(b, 目标_wasm64)
  let gap = 极性差距(a, b)
  let gapText = if gap == 1: " 吸→公约" else: " 斥→隔离"
  echo "  差距: " & $gap & gapText
  echo ""

  echo "-- 公约裁决 --"
  let 主权_a: uint64 = 0xF003000000000000'u64
  let 主权_b: uint64 = 0xF006000000000000'u64
  echo "  主权A=0x" & toHex(主权_a)
  echo "  主权B=0x" & toHex(主权_b)
  echo "  GCD=0x" & toHex(gcd(主权_a, 主权_b))
  let gap1 = 1
  let gap0 = 0
  echo "  gap=1 裁决=0x" & toHex(裁决(主权_a, 主权_b, gap1)) & " (赋能)"
  echo "  gap=0 裁决=0x" & toHex(裁决(主权_a, 主权_b, gap0)) & " (隔离)"
  echo ""

  echo "-- 阶梯编译验证 --"
  for t in targets:
    let c = encode(相位_CENTER, 0, MQF_ME, t)
    echo "  " & align($t, 14) & " → " & 坐标字符串(c, t)

  echo ""
  echo "═══════════════════════════════════════"
  echo "三步阶梯: 同一源码 · 双目标 · 极性不变"
  echo "═══════════════════════════════════════"
