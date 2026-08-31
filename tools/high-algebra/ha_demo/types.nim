# ============================================================
# 高维代数 · 类型定义
#
# 实现：(n-1)×0.1 偏移率体系、跨层级静态回归、俘获规则、阶和
# ============================================================

type
  ## 代数层级（平行世界）
  AlgebraLevel* = object
    generation*: int              ## 代数层级（1 = 初代数, 2 = 二次代数, ...）
    offset_rate*: float           ## 偏移量 δ = (n-1) × 0.1
    multiplier*: float            ## 乘数 M = 1 + δ
    base_radix*: int              ## 进制基数（该层符号数量）
    is_source*: bool              ## 是否为源头（偏移量最小 = 初代数）

  ## 平行世界中的数值
  ParallelValue* = object
    level*: AlgebraLevel          ## 所在层级
    raw_value*: float             ## 原始数值
    symbol_ref*: string           ## 符号引用

  ## 跨层运算结果
  ConversionResult* = object
    original*: ParallelValue      ## 原始值
    converted*: float             ## 折算后数值
    conv_method*: string          ## 转换方法：static_regression / capture / tier_sum
    captured*: bool               ## 是否被俘获

  ## 阶和叠加态
  TierSumState* = object
    worlds*: seq[ParallelValue]   ## 参与叠加的平行世界
    sharpness*: float             ## 锐度（偏移量差异产生的感知差异）
    can_perceive*: bool           ## 是否可感知

  ## 2²⁵⁶ 高位对齐状态
  HighBitAlignment* = object
    tiers*: int                   ## 当前层数
    address_space*: float         ## 寻址空间 2²⁵⁶
    alignment_id*: string         ## 对齐标识
