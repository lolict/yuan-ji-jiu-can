# ============================================================
# 天庭空间 · 类型定义
#
# 四模型复合：
#   吸引子 (Attractor)   — 系统趋向的点
#   叠加态 (Superposition) — 多态同时存在
#   纤维丛 (Fiber Bundle) — 空间随底空间变化
#   Topos (Topos)        — 层范畴，亚对象分类器
#
# 版本：v0.15 (与 hunyuan-rs 天庭空间同步)
# ============================================================

import tables
import options
import math

# ── 相空间坐标 ──
type PhaseCoord* = object
  x*: float       # 实部（容器侧）
  y*: float       # 虚部（界线侧）
  z*: float       # 高维偏移（平行世界层级）
  phase*: float   # 相位角 0..2π

# ── 吸引子 ──
type Attractor* = object
  name*: string
  position*: PhaseCoord    # 吸引子位置
  strength*: float         # 吸引强度 0..1
  radius*: float           # 吸引半径
  polarity*: float         # 极性：-1=阴(容器), 1=阳(界线), 0=平衡

# ── 叠加态 ──
type Superposition* = object
  states*: seq[float]      # 各态的概率幅
  phases*: seq[float]      # 各态的相位
  coherence*: float        # 相干度 0..1（1=完全相干）
  count*: int              # 态数量

# ── 纤维丛 ──
type FiberBundle* = object
  base_point*: PhaseCoord      # 底空间点
  fiber*: seq[float]           # 纤维（该点上的所有可能态）
  connection*: float           # 联络（纤维间的变换率）
  curvature*: float            # 曲率（空间弯曲程度）

# ── Topos（亚对象分类器）──
type Topos* = object
  truth_values*: Table[string, float]  # 真值表（非二元：0..1）
  subobject_classifier*: float         # 亚对象分类器 Ω
  pullbacks*: int                      # 拉回计数
  pushouts*: int                       # 推出计数

# ── 天庭空间 ──
type HeavenSpace* = object
  attractors*: seq[Attractor]          # 吸引子列表
  superpositions*: seq[Superposition]  # 叠加态列表
  bundles*: seq[FiberBundle]           # 纤维丛列表
  topos*: Topos                        # Topos 结构
  
  # 空间维度
  dimension*: int                      # 维度数
  volume*: float                       # 空间体积
  
  # 统计
  attractor_count*: int
  superposition_count*: int
  bundle_count*: int
