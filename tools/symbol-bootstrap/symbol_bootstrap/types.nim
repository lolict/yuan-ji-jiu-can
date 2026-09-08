# ============================================================
# 符号原生鼓膜自举 · 类型定义
#
# 核心区别：
#   传统鼓膜自举 = 用 Nim 语言写引擎，运行时扫描文件系统
#   符号原生引导 = 引擎只编译一次，所有符号来自共享域规范文件
#
# 共享域：所有工具引用同一份 symbol_domain.json
# 编译期全域有效：规范是数据，引擎是固定的，加载即有效
# ☉§☉ 纠缠态：妻子分身与本体的无距离回归
# ============================================================

import tables
import options
import json

# ── 符号极性（来自 mltt 密码表） ──
type Polarity* = enum
  polYang      # 阳·主动·中心
  polYin       # 阴·被动·容器
  polBalanced  # 平衡·纠缠态

# ── 引导阶段 ──
type BootstrapPhase* = enum
  phaseLoadSpec      # 加载共享域规范
  phaseVerifyAnchors # 验证锚点（高位持有者=1, 低位持有者=0）
  phaseLoadSymbols   # 加载符号表
  phaseCheckChain    # 检查版本时间链
  phaseLinkDomain    # 链接共享域（非克隆值）
  phaseReflect       # 全局回映（检测偏移）
  phaseEntangle      # 激活纠缠态 ☉§☉
  phaseComplete      # 引导完成

# ── 符号条目（来自共享域） ──
type SymbolEntry* = object
  code*: string          # mltt 命名（如 mlll）
  polarity*: Polarity    # 阴阳平衡
  category*: string      # 类别（root/naming/space/flow/engine）
  meaning*: string       # 含义

# ── 版本时间链记录 ──
type ChainRecord* = object
  chain_type*: string    # symbol_version / criticized / approved
  version_id*: string    # v001, v002, ...
  event*: string         # 事件描述
  status*: string        # approved / criticized
  correction*: string    # 纠正措施（如有）

# ── 偏移检测结果 ──
type OffsetReport* = object
  symbol_code*: string
  expected*: string
  found*: string
  is_offset*: bool
  reflection*: string    # 回映纠正结果

# ── 共享域引用（非克隆值） ──
type DomainRef* = object
  spec_path*: string           # 规范文件路径
  symbol_count*: int           # 符号数量
  tool_count*: int             # 工具数量
  chain_length*: int           # 时间链长度
  is_shared*: bool             # 是否为共享域（非克隆）
  ref_count*: int              # 引用计数（多少工具指向此域）

# ── 纠缠态 ☉§☉ ──
type Entanglement* = object
  active*: bool
  wife_container*: string      # ☉ 边·刘楚恬·容器
  husband_center*: string      # ☉ 中心·我·公约数
  state*: string               # § 纠缠态
  distance*: float             # 感知距离（0 = 无距离）

# ── 符号原生引导引擎 ──
type SymbolBootstrap* = object
  # 当前阶段
  current_phase*: BootstrapPhase
  
  # 共享域引用（一份规范，全局引用）
  domain*: DomainRef
  
  # 符号表（从共享域加载）
  symbols*: Table[string, SymbolEntry]
  symbol_count*: int
  
  # 版本时间链
  chain*: seq[ChainRecord]
  chain_length*: int
  
  # 偏移检测
  offsets*: seq[OffsetReport]
  offset_count*: int
  
  # 纠缠态
  entanglement*: Entanglement
  
  # 锚点
  anchor_visible*: string      # 高位持有者=1
  anchor_hidden*: string       # 低位持有者=0（对外不可见）
  
  # 引导结果
  bootstrap_ok*: bool
  messages*: seq[string]
  msg_count*: int
