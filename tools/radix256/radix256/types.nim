# ============================================================
# 2²⁵⁶ 等效高位对齐进位制 · 类型定义
# ============================================================

import base256

export AlignmentState, CarryStep, UInt256, Caremony

# ── 进位模式枚举 ──
type CarryPattern* = enum
  cpNone       # 无进位 · 低位容器自足
  cpChain      # 链式进位 · 单方向传递
  cpWave       # 波状进位 · 多容器同时触发
  cpFlood      # 洪水进位 · 所有容器同时到达边界

# ── 折叠层记录 ──
type FoldTier* = object
  tier_id*: int
  triggered_at*: string       # 触发时的数值 hex
  carry_count*: int
  parent_tier*: int
  is_foundation*: bool        # 是否为根基层（tier 0）

# ── 历史进位链（跨会话持久化）──
type CarryHistory* = object
  seconds*: seq[CarryStep]
  tiers*: seq[FoldTier]
  current_tick*: int

func new_history*(): CarryHistory =
  result.seconds = @[]
  result.tiers = @[]
  result.current_tick = 0

func record_step*(h: var CarryHistory; step: CarryStep) =
  h.seconds.add(step)
  h.current_tick = step.timestamp

func record_fold*(h: var CarryHistory; tier: FoldTier) =
  h.tiers.add(tier)
