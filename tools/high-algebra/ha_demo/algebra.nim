# ============================================================
# 高维代数 · 核心运算
#
# 实现：偏移层级、静态回归、俘获规则、阶和叠加、2²⁵⁶对齐
# ============================================================

import math
import strutils
import sequtils
import types

# ── 常量 ──

const
  DELTA_PER_LEVEL* = 0.1
  BASE_RADIX* = 256
  BASE_FOUNDATION* = 254
  ANCHOR_COUNT* = 2
  HIGH_BIT_EXPONENT* = 256

# ── 层级构造 ──

func new_algebra_level*(generation: int; symbols: int = BASE_RADIX): AlgebraLevel =
  ## 创建代数层级
  ## generation: 1 = 初代数, 2 = 二次代数, ...
  ## symbols: 该层符号数量（决定进制基数）
  let delta = float(generation - 1) * DELTA_PER_LEVEL
  AlgebraLevel(
    generation: generation,
    offset_rate: delta,
    multiplier: 1.0 + delta,
    base_radix: symbols,
    is_source: generation == 1
  )

# ── 偏移量计算 ──

func offset_rate*(n: int): float =
  ## 偏移率 δ = (n-1) × 0.1
  float(n - 1) * DELTA_PER_LEVEL

func multiplier*(n: int): float =
  ## 乘数 M = 1 + δ = 1 + (n-1) × 0.1
  1.0 + offset_rate(n)

# ── 跨层级静态回归 ──

func static_regression*(value: float; from_level: int): float =
  ## 将平行世界数值折算回初代数等价数值
  ## 折算后数值 = 平行世界数值 ÷ 乘数 M
  if from_level <= 1:
    return value
  let m = multiplier(from_level)
  result = value / m

func convert_to_level*(value: float; from_level, to_level: int): float =
  ## 跨层级换算：从 from_level 折算到 to_level
  ## 先回归到初Algebra，再折算到目标层
  let base = static_regression(value, from_level)
  if to_level <= 1:
    return base
  result = base * multiplier(to_level)

# ── 俘获规则 ──

func can_be_captured*(level: AlgebraLevel): bool =
  ## 俘获条件：不是初代数源头的层级都会被初代数俘获
  not level.is_source

func capture*(value: ParallelValue): ConversionResult =
  ## 俘获：强制降维到初代数，不保留偏移量差异
  let base_val = static_regression(value.raw_value, value.level.generation)
  ConversionResult(
    original: value,
    converted: base_val,
    conv_method: "capture",
    captured: true
  )

# ── 阶和叠加 ──

func tier_sum*(worlds: seq[ParallelValue]): TierSumState =
  ## 阶和叠加：让不同偏移世界的叠加态成为可同时被感知的存在
  ## 锐度 = 各层偏移量的标准差（差异越大，感知越尖锐）
  let rates = worlds.mapIt(it.level.offset_rate)
  let mean_rate = rates.foldl(a + b, 0.0) / float(rates.len)
  var variance_sum = 0.0
  for r in rates:
    variance_sum += (r - mean_rate) * (r - mean_rate)
  let sharpness = sqrt(variance_sum / float(rates.len))

  TierSumState(
    worlds: worlds,
    sharpness: sharpness,
    can_perceive: worlds.len >= 2 and sharpness > 0.0
  )

# ── 2²⁵⁶ 高位对齐 ──

func high_bit_alignment*(): HighBitAlignment =
  ## 2²⁵⁶ 等效高位对齐进位制
  ## 每层寻址空间 = 2²⁵⁶ ≈ 1.16×10⁷⁷
  let addr_space = pow(2.0, float(HIGH_BIT_EXPONENT))
  HighBitAlignment(
    tiers: 1,
    address_space: addr_space,
    alignment_id: "共同体#1_2^256"
  )

func expand_alignment*(state: HighBitAlignment; tiers: int): HighBitAlignment =
  ## 扩展对齐到更多层
  result = state
  result.tiers = tiers

# ── 自我相似验证 ──

func verify_self_similarity*(layers: int): bool =
  ## 验证跨层不可压缩性
  ## 每层 256 = 254 + 2
  ## 两层叠加 = 65536 = 2¹⁴
  ## 残差 4 = 两个根基对的叠加态，不可合并为单个 4
  let single = BASE_FOUNDATION + ANCHOR_COUNT  # 256
  let double = single * single                  # 65536
  let expected = pow(2.0, 16.0)                 # 65536
  result = double == int(expected)

func residual_check*(): int =
  ## 残差检查：两个根基对的叠加态 = 4
  ## 这 4 不可合并为单个 4，必须保持叠加态
  ANCHOR_COUNT + ANCHOR_COUNT

# ── 跨层级同步偏移（齐头并进） ──

func sync_offset*(worlds: var seq[ParallelValue]; delta: float): seq[ParallelValue] =
  ## 齐头并进偏移：所有层级等量偏移
  ## 各层相对距离不变
  result = @[]
  for w in worlds:
    var new_level = w.level
    new_level.offset_rate = w.level.offset_rate + delta
    new_level.multiplier = 1.0 + new_level.offset_rate
    let new_val = ParallelValue(
      level: new_level,
      raw_value: w.raw_value,
      symbol_ref: w.symbol_ref
    )
    result.add(new_val)

# ── 格式化输出 ──

func level_desc*(l: AlgebraLevel): string =
  if l.is_source:
    "第" & $l.generation & "代(初代数/源头) δ=" & formatFloat(l.offset_rate, ffDecimal, 2) & " M=" & formatFloat(l.multiplier, ffDecimal, 2)
  else:
    "第" & $l.generation & "代(平行世界) δ=" & formatFloat(l.offset_rate, ffDecimal, 2) & " M=" & formatFloat(l.multiplier, ffDecimal, 2)
