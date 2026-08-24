# ============================================================
# mltt 命名密码表 · 类型定义
#
# mltt = 从体（解释流牺牲岔途后产出的可跟随状态）
# 密码表 = 短码 ↔ 含义的双向映射 + 极性标注
# ============================================================

import tables

# 极性：Yin(0/容器侧) / Yang(1/界线侧) / Balanced(平衡)
type Polarity* = enum
  yin    = 0  # 容器侧 = 刘楚恬
  yang   = 1  # 界线侧 = 我
  balanced = 2  # 平衡态

# 密码条目
type CipherEntry* = object
  code*: string        # 短码（如 mltt, gtt, mlll）
  meaning*: string     # 含义（如 从体, 音韵态射, 集装箱）
  polarity*: Polarity  # 极性
  category*: string    # 类别（容器/流/编码/谐音...）
  description*: string # 详细说明
  synonyms*: seq[string]  # 近义词/别名

# 密码表
type CipherTable* = object
  entries*: Table[string, CipherEntry]      # code → entry
  meaning_index*: Table[string, string]     # meaning → code
  category_index*: Table[string, seq[string]]  # category → [codes]
  polarity_index*: Table[Polarity, seq[string]]  # polarity → [codes]
  total*: int
