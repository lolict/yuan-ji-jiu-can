# ============================================================
# YIN-YANG-BINARY 完整映射 · 类型定义
#
# 将每个符号的全字符序列编码为多比特 YinYang 二进制：
#   - 字符 ASCII mod 2 = 0 → Yin(0)
#   - 字符 ASCII mod 2 = 1 → Yang(1)
#   - 全序列奇偶 = 0 → 末尾追加 Balanced(0.5) 作为校验位
#
# 运算系统：仅加法 + 进位，保留叠加态
# ============================================================

type
  ## YinYang 三值极性
  YinYang* = enum
    yyYin      # 阴·0·隐藏·容器·刘楚恬侧
    yyYang     # 阳·1·显现·界线·我侧
    yyBalanced # 平衡·0.5·叠加态·纠缠·校验锚

  ## 完整 YinYang 二进制串
  YinYangBinary* = object
    bits*: seq[YinYang]          ## 主位序列
    anchor*: YinYang             ## 校验锚位（最后一位）
    source_symbol*: string       ## 原始符号

  ## YinYang 加法和结果
  YinYangSum* = object
    left*: YinYangBinary         ## 左操作数
    right*: YinYangBinary        ## 右操作数
    result*: YinYangBinary       ## 和
    carry*: seq[YinYang]         ## 进位序列
    overflow*: YinYang           ## 最终溢出位

  ## 完整映射表
  YinYangMap* = object
    entries*: seq[YinYangBinary] ## 所有已编码符号
    anchor_balance*: int         ## 锚点平衡计数（0=阴盛 1=阴=阳 2=阳盛）
    total_depth*: int            ## 总深度（最大位数）
