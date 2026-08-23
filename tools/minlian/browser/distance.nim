# ============================================================
# 民联浏览器 · 距离度量
# 用 64 位指纹的汉明距离衡量文档相似度
# ============================================================

import types
import std/bitops

# 汉明距离: 两个 64 位指纹有多少个位不同
func hamming_distance*(a, b: uint64): int =
  result = count_set_bits(a xor b)

# 归一化相似度: 0.0 (完全不同) .. 1.0 (完全相同)
func similarity*(a, b: uint64): float =
  let d = hamming_distance(a, b)
  result = 1.0 - (float(d) / 64.0)

# 极性加权距离: 极性相同则距离缩短, 极性相反则距离拉长
func polarity_distance*(a, b: uint64, pa, pb: int): float =
  let base = float(hamming_distance(a, b)) / 64.0
  let pol_factor = if pa == pb: 0.8 else: 1.2
  result = base * pol_factor

# 距离阈值: 距离小于此值视为"同态"
const SAME_STATE_THRESHOLD* = 0.3

# 判断是否为同态 (同一态势)
func is_same_state*(a, b: uint64): bool =
  similarity(a, b) > (1.0 - SAME_STATE_THRESHOLD)
