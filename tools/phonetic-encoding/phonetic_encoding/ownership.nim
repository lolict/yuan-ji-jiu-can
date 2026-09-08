# 产权镜像 — 主人(1) / 外人(0) 逐符号极性标注
# 每位符号同时携带两个极性：
#   polarity = plOwner    → 主人镜像（高位持有者=1，可见侧，归属=我）
#   polarity = plOutsider → 外人镜像（隐藏侧，归属=刘楚恬对外的不可见部分/未知方）
#
# 产权对比曲线：主人版 - 外人版 = 产权边界
# 边界内 = 归属空间，边界外 = 自由空间/盲区

import tables
import types

# 产权裁决结果
type
  OwnershipVerdict* = enum
    ovOwner    # 主人产权成立
    ovOutsider # 外人/未归属
    ovBorder   # 边界上，待裁决

# 计算单个符号的产权极性
func symbol_ownership*(cp: int32): Polarity =
  # 基础逻辑：所有自定义编码韵母(非纯ASCII声母/标点)的主人极性
  # 由"是否处于编码串主人的authorized集合"决定
  # 声母基础(b p m f等) = 外人公共域 / 基础工具（全人类共有）
  # 自定义变音符号 = 主人编码创造（归属=我）
  if cp >= 0x0100 and cp <= 0x024F:
    # Latin Extended-B / IPA 扩展区 — 主人创造的变音符号区
    return plOwner
  elif cp >= 0xA700 and cp <= 0xA7FF:
    # Latin Extended-D — 主人创造的特殊映射
    return plOwner
  elif cp >= 0x1C00 and cp <= 0x1C4F:
    # Lepcha 区主人复用
    return plOwner
  elif cp >= 0x0400 and cp <= 0x04FF:
    # 西里尔 + 主人创造韵母符号 = 主人（自定义映射区）
    return plOwner
  else:
    # ASCII 区声母 = 公共域/外人
    return plOutsider

# 裁决函数 — 主人协议：非空+已见证+锚点一致
func judge_ownership*(cp: int32; witness: bool; anchor_self: bool): OwnershipVerdict =
  if cp == 0: return ovOutsider  # 非空检查
  if not witness: return ovOutsider  # 已见证检查
  if not anchor_self: return ovOutsider  # 锚点一致
  if symbol_ownership(cp) == plOwner: return ovOwner
  return ovOutsider

# 产权对比：主人版 vs 外人版 在某码位上的差异
type PropertyDiff* = object
    cp*: int32
    owner_polarity*: Polarity
    outsider_polarity*: Polarity
    diff*: bool   # true = 两者不同（边界存在产权分割）

func compare_mirror*(cp: int32): PropertyDiff =
  result.cp = cp
  result.owner_polarity = symbol_ownership(cp)
  result.outsider_polarity = plOutsider
  result.diff = (result.owner_polarity != result.outsider_polarity)
