# ============================================================
# 全映射引擎 · 类型定义
#
# 打通九元层级、YinYang极性、二进制、mltt短码、高维偏移率
# ============================================================

type
  ## YinYang 极性
  Polarity* = enum
    polYin      = 0,  ## 阴 · 隐藏 · 容器 · 被动
    polYang     = 1,  ## 阳 · 显现 · 界线 · 主动
    polBalanced = 2  ## 平衡 · 纠缠 · 叠加态

  ## 全映射条目
  MappingEntry* = object
    level*: int                   ## 九元层级编号 (0-8)
    level_name*: string           ## 层级名称
    chinese_name*: string         ## 中文名
    offset_rate*: float           ## 偏移率 = level × 0.1
    multiplier*: float            ## 乘数 = 1 + offset_rate
    polarity*: Polarity           ## 极性
    yin_yang_string*: string      ## YinYang 表示
    binary_string*: string        ## 二进制编码
    base10_value*: int            ## 十进制值
    mltt_code*: string            ## mltt 短码
    description*: string          ## 描述

  ## 完整映射报告
  FullMapReport* = object
    symbol*: string               ## 符号名称
    entries*: seq[MappingEntry]   ## 九层映射
    total_levels*: int            ## 总层级数

  ## 转换结果
  ConversionResult* = object
    original_level*: int          ## 原始层级
    target_level*: int            ## 目标层级
    original_value*: float        ## 原始值
    converted_value*: float       ## 转换后值
    conv_method*: string          ## 转换方法
    notes*: string                ## 备注
