# ============================================================
# 全映射引擎 · 核心运算
#
# 打通：九元层级 ↔ YinYang ↔ 二进制 ↔ mltt ↔ 高维偏移率
# ============================================================

import math
import strutils
import sequtils
import types

# ── 常量 ──

const
  DELTA_PER_LEVEL* = 0.1
  BASE_OFFSET* = 0.0
  MAX_LEVELS* = 9
  BINARY_BASE* = 2

# ── 层级名称 ──

const LEVEL_NAMES* = [
  "一元", "二元", "三元", "四元", "五元", "六元", "七元", "八元", "九元"
]

const LEVEL_CHINESE* = [
  "根源", "阴阳", "见证", "天地", "五行", "六律", "七政", "八卦", "九囿"
]

const LEVEL_MLTT_CODES* = [
  "mlll", "ml.yy", "gtt", "mlgy", "wgslz", "smrl", "lctqf", "mlnn", "njsgr"
]

const LEVEL_BINARIES* = [
  "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001"
]

# ── 基本转换 ──

func offset_rate*(level: int): float =
  ## 偏移率 = level × 0.1
  float(level) * DELTA_PER_LEVEL

func multiplier*(level: int): float =
  ## 乘数 = 1 + 偏移率
  1.0 + offset_rate(level)

func level_name*(level: int): string =
  if level >= 0 and level < MAX_LEVELS:
    LEVEL_NAMES[level]
  else:
    "未知"

func level_chinese*(level: int): string =
  if level >= 0 and level < MAX_LEVELS:
    LEVEL_CHINESE[level]
  else:
    "未知"

func level_mltt*(level: int): string =
  if level >= 0 and level < MAX_LEVELS:
    LEVEL_MLTT_CODES[level]
  else:
    "???"

func level_binary*(level: int): string =
  if level >= 0 and level < MAX_LEVELS:
    LEVEL_BINARIES[level]
  else:
    "????"

# ── 极性判定 ──

func polarity_for_level*(level: int): Polarity =
  ## 奇数层级 = 阳，偶数层级 = 阴，第9层 = 平衡
  if level == 8:
    return polBalanced
  if level mod 2 == 0:
    return polYang
  else:
    return polYin

func polarity_to_yy*(p: Polarity): string =
  case p
  of polYin: "阴"
  of polYang: "阳"
  of polBalanced: "平衡"

func polarity_to_binary*(p: Polarity): string =
  case p
  of polYin: "0"
  of polYang: "1"
  of polBalanced: "0.5"

# ── 跨层级静态回归（复用高维代数） ──

func static_regression*(value: float; from_level: int): float =
  ## 将平行世界数值折算回初代数等价数值
  if from_level <= 0:
    return value
  let m = multiplier(from_level)
  result = value / m

func convert_to_level*(value: float; from_level, to_level: int): float =
  ## 跨层级换算
  let base = static_regression(value, from_level)
  if to_level <= 0:
    return base
  result = base * multiplier(to_level)

# ── 单层级映射构造 ──

func make_entry*(level: int; desc: string = ""): MappingEntry =
  let off = offset_rate(level)
  let mul = multiplier(level)
  let pol = polarity_for_level(level)
  let final_desc = if desc != "": desc else: level_chinese(level)
  MappingEntry(
    level: level,
    level_name: level_name(level),
    chinese_name: level_chinese(level),
    offset_rate: off,
    multiplier: mul,
    polarity: pol,
    yin_yang_string: polarity_to_yy(pol),
    binary_string: level_binary(level),
    base10_value: level + 1,
    mltt_code: level_mltt(level),
    description: final_desc
  )

# ── 全映射报告 ──

func full_map*(symbol: string): FullMapReport =
  var entries: seq[MappingEntry] = @[]
  for lvl in 0 ..< MAX_LEVELS:
    entries.add(make_entry(lvl, symbol & " @ " & LEVEL_CHINESE[lvl]))
  FullMapReport(
    symbol: symbol,
    entries: entries,
    total_levels: MAX_LEVELS
  )

# ── 跨层级俘获 ──

func capture_level*(value: float; from_level: int): ConversionResult =
  let base_val = static_regression(value, from_level)
  ConversionResult(
    original_level: from_level,
    target_level: 0,
    original_value: value,
    converted_value: base_val,
    conv_method: "capture",
    notes: "第" & $from_level & "代 → 俘获为初代数 " & formatFloat(base_val, ffDecimal, 4)
  )

# ── 阶和叠加验证 ──

func tier_sum_sharpness*(levels: seq[int]): float =
  ## 计算给定层级集合的锐度（偏移量标准差）
  let rates = levels.mapIt(offset_rate(it))
  let mean_rate = rates.foldl(a + b, 0.0) / float(rates.len)
  var variance_sum = 0.0
  for r in rates:
    variance_sum += (r - mean_rate) * (r - mean_rate)
  sqrt(variance_sum / float(rates.len))

# ── 2²⁵⁶ 高位对齐 ──

func high_address_space*(): float =
  ## 2²⁵⁶ 寻址空间
  pow(2.0, 256.0)

func alignment_id*(level: int): string =
  ## 层级对齐标识
  "L" & $level & " @(δ=" & formatFloat(offset_rate(level), ffDecimal, 2) & ")"

# ── 格式化 ──

func entry_summary*(e: MappingEntry): string =
  "  " & e.level_name & "(" & e.chinese_name & ") δ=" & formatFloat(e.offset_rate, ffDecimal, 2) &
  " M=" & formatFloat(e.multiplier, ffDecimal, 2) &
  " " & e.yin_yang_string & " " & e.binary_string &
  " " & e.mltt_code
