# ============================================================
# 流代数 · 类型定义
#
# 12位流属性 + 八流体系 + 解释流牺牲机制
#
# 核心哲学：
#   流 = 容器之间的态射（转移/变换/交易）
#   解释流 = 牺牲自己产关系流（mlct→mltt）
#   关系流不是凭空产生的，是解释流用命换来的
#
# 版本：v0.12-v0.15 (与 hunyuan-rs 流代数同步)
# ============================================================

import tables
import options

# ── 12位流属性 ──
type FlowProperty* = enum
  reachable      # bit 0  可被到达
  continuable    # bit 1  可持续
  incr           # bit 2  可增量
  stoppable      # bit 3  可停止
  reversible     # bit 4  可逆
  recoverable    # bit 5  可恢复
  circulable     # bit 6  可循环
  inheritable    # bit 7  可继承
  trusted        # bit 8  已信任
  ackable        # bit 9  可被承认（闭环必需）
  transmissible  # bit 10 可传递
  advanceable    # bit 11 可推进

# ── 八流类型 ──
type FlowType* = enum
  preloadFlow     # 预加载流（字典级集 = 指令集集群）
  categoryFlow    # 范畴流（类型空间映射）
  clusterFlow     # 聚类流（clusterize）
  classifyFlow    # 分类流（感知家族路由）
  propertyFlow    # 属性流（12位二元属性）
  interpretiveFlow # 解释流（牺牲自己产关系流）
  relationFlow    # 关系流（路由后可被承认）
  conceptFlow     # 概念流（范畴+聚类+解释后产物）

# ── 流状态 ──
type FlowState* = enum
  flowActive      # 活跃
  flowBlocked     # 阻塞（遇到岔途）
  flowSacrificing # 牺牲中（解释流正在毁灭自身）
  flowAcked       # 已承认（闭环完成）
  flowBroken      # 断裂（对方不承认）
  flowDormant     # 休眠

# ── 流条目 ──
type FlowEntry* = object
  name*: string
  flow_type*: FlowType
  state*: FlowState
  properties*: uint16    # 12位属性位图
  source*: string         # 源容器
  target*: string         # 目标容器
  carrier*: string        # 载体（路由路径）
  ackable*: bool          # 是否可被承认
  description*: string

# ── 解释流牺牲记录 ──
type SacrificeRecord* = object
  mlct*: string           # 岔途（障碍）
  mltt*: string           # 从体（产出）
  cost*: float            # 牺牲代价 0..1
  success*: bool          # 是否成功转译
  timestamp*: int

# ── 一维时间线性（UnifyingFlow）──
type TimeStep* = enum
  stepInput        # 输入
  stepPerception   # 感知
  stepDistinguish  # 分别
  stepLookup       # 查字典
  stepTraverse     # 走解释流
  stepCluster      # 聚类
  stepAttention    # 注意力
  stepArrival      # 抵达

# ── 流代数引擎 ──
type FlowAlgebra* = object
  flows*: seq[FlowEntry]              # 流列表
  flow_count*: int
  sacrifices*: seq[SacrificeRecord]   # 牺牲记录
  sacrifice_count*: int
  type_index*: Table[FlowType, seq[int]]  # 按类型索引
  state_index*: Table[FlowState, seq[int]] # 按状态索引
