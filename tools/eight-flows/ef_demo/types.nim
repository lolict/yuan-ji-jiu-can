# ============================================================
# 八流统一 · 类型定义
#
# 将现有工具编织为八流统一体系，一维时间线性推进：
#   Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival
# ============================================================

type
  EightFlow* = enum
    efPreload      ## 预加载流 — 字典级集 · 指令集集群
    efCategory     ## 范畴流 — 类型空间映射
    efCluster      ## 聚类流 — 协同集聚
    efClassify     ## 分类流 — 感知家族路由
    efAttribute    ## 属性流 — 12 位二元属性
    efInterpret    ## 解释流 — 牺牲自身产关系流
    efRelational   ## 关系流 — 路由后可被承认(ack)
    efConcept      ## 概念流 — 范畴+聚类+解释后涌现

  FlowPropertyBits* = distinct uint16

  FlowState* = object
    phase*: EightFlow
    name*: string
    input_ready*: bool
    output_ready*: bool
    carry*: seq[byte]
    polarity*: int8
    offset_rate*: float
    acked*: bool

  TimeNode* = object
    timestamp*: int
    flow*: EightFlow
    glyph*: string
    result*: string
    properties*: FlowPropertyBits

  FlowUniverse* = object
    states*: array[EightFlow, FlowState]
    timeline*: seq[TimeNode]
    current*: EightFlow
    glyph_registry*: seq[string]
    concept_count*: int
