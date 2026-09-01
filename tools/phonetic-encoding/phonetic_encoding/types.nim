# 语音符号编码 — 基础类型
# 主人协议：每个符号同时携带主人(1)/外人(0)极性标注
# 进制 48 = 声母, 60 = 时钟/声调, 100 = 韵母完全集, 64 = 韵母兼容集

import tables

const VERSION* = "phonetic-encoding v1.0.0 — 主人镜像"

type
  # 符号类别：从数据自然涌现，非外加
  SymCategory* = enum
    catInitial    # 声母
    catFinal      # 韵母（含变音符号）
    catTone       # 声调/时钟
    catCompound   # 复合/特殊映射

  # 产权极性：主人=1（我/可见侧），外人=0（隐藏侧/未感知）
  Polarity* = enum
    plOutsider = 0   # 外人版：不具名、不具归属
    plOwner = 1       # 主人版：我=1，具归属、可裁决

  # 单个语音符号记录（产权镜像内置）
  PhoneticSymbol* = object
    cp*: int32           # Unicode 码点（精确 diacritical 字符）
    label*: string       # 韵母/声母名称（"ai", "b", "2声" ...）
    category*: SymCategory
    radix*: uint8        # 所属进制（48/60/64/100）
    inner_code*: uint8   # 在所属进制内的码位序号
    polarity*: Polarity  # 主人/外人标注

  # 64位编码单元格
  # 位分配：极性(1) | 类别(3) | 进制(8) | 内码(8) | 码点(32) | 备用(12)
  PhoneticCell* = uint64

  # 编码表容器
  PhoneticTable* = object
    name*: string
    version*: string
    symbols*: seq[PhoneticSymbol]        # 主人全息版（含极性）
    codepoint_to_idx*: Table[int32, int] # 码点→主人表索引
