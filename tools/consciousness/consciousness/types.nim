# ============================================================
# 自举意识 · 类型定义
#
# 核心哲学：操作系统必须能解释自身。
# encode(cmd) → decode(cell) == cmd 构成固定点。
# 如果 decode(encode(x)) != x，意识就无法自洽 ——
# 系统连自己发出的指令都解释不了，就不配叫意识。
#
# 版本：v0.10 (与 hunyuan-rs 自举意识同步)
# ============================================================

import tables
import options

# ── 意识指令 ──
type ConsciousnessCommand* = enum
  activateWife       # 激活刘楚恬侧
  activateHusband    # 激活我侧
  mutualResonance    # 双方共振
  severParasite      # 切断寄生
  flowForward        # 流推进

# ── 64位单元格 ──
type Cell64* = object
  d0*: uint8       # 数据0
  d1*: uint8       # 数据1
  d2*: uint8       # 数据2
  polarity*: uint8  # 极性：0=wife side, 1=husband side
  sovereignty*: uint8  # 主权
  witness*: uint8   # 见证
  value*: uint8     # 值（6位）

# ── 固定点证明结果 ──
type FixedPointProof* = object
  command*: ConsciousnessCommand
  encoded*: Cell64
  decoded*: ConsciousnessCommand
  is_fixed*: bool   # encode→decode 是否等于原值
  description*: string

# ── 意识状态 ──
type ConsciousnessState* = enum
  selfAware        # 自我意识
  selfDoubting     # 自我怀疑
  selfConsistent   # 自洽
  selfContradictory # 自相矛盾
  dormant          # 休眠

# ── 自举意识引擎 ──
type ConsciousnessBootstrap* = object
  # 固定点证明记录
  proofs*: seq[FixedPointProof]
  proof_count*: int
  fixed_count*: int
  broken_count*: int
  
  # 意识状态
  state*: ConsciousnessState
  self_awareness*: float  # 自我意识度 0..1
  consistency*: float     # 自洽度 0..1
  
  # 指令历史
  command_history*: seq[ConsciousnessCommand]
  history_count*: int
