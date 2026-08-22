# ============================================================
# 分比容器 · 层级比特编码
# 每一层职责独立，比特精确分配
# 全部打包进 64 位整数
# ============================================================

type
  ContainerLayer* = enum
    clPresence,    # 第 0 阶: 有/无 (1 bit)
    clWitness,     # 第 1 阶: 已见证/未见证 (1 bit)
    clType,        # 第 2 阶: 类型 (8 bits)
    clOwner,       # 第 3 阶: 归属 (8 bits)
    clRelation,    # 第 4 阶: 关系索引 (8 bits)
    clTiming,      # 第 5 阶: 时序偏移 (16 bits)
    clPayload      # 第 6 阶: 负载 (22 bits)

  LayerConfig* = object
    name*: string
    bits*: int
    offset*: int
    shift*: int

  LayerProfile* = object
    configs*: array[7, LayerConfig]
    total_bits*: int
    layers_active*: int

const
  # 默认分比方案 (64 bits 全用)
  DEFAULT_PROFILE* = LayerProfile(
    configs: [
      LayerConfig(name: "presence", bits: 1, offset: 0, shift: 0),
      LayerConfig(name: "witness", bits: 1, offset: 1, shift: 1),
      LayerConfig(name: "type", bits: 8, offset: 2, shift: 2),
      LayerConfig(name: "owner", bits: 8, offset: 3, shift: 10),
      LayerConfig(name: "relation", bits: 8, offset: 4, shift: 18),
      LayerConfig(name: "timing", bits: 16, offset: 5, shift: 26),
      LayerConfig(name: "payload", bits: 22, offset: 6, shift: 42)
    ],
    total_bits: 64,
    layers_active: 7
  )

# 新方案：各阶独立数量统计
type
  LayerInfo* = object
    name*: string        # 阶名
    bit_width*: int      # 占用比特数
    max_values*: int     # 可表达多少个不同值
    purpose*: string     # 职责描述

# 默认七阶信息
const LAYER_SPEC* = [
  LayerInfo(name: "第0阶-存在态",  bit_width: 1,  max_values: 2,       purpose: "有/无 开关"),
  LayerInfo(name: "第1阶-见证态",  bit_width: 1,  max_values: 2,       purpose: "已裁/未裁 标记"),
  LayerInfo(name: "第2阶-类型码",  bit_width: 8,  max_values: 256,     purpose: "256种类型"),
  LayerInfo(name: "第3阶-归属码",  bit_width: 8,  max_values: 256,     purpose: "256个所有者的索引"),
  LayerInfo(name: "第4阶-关系码",  bit_width: 8,  max_values: 256,     purpose: "指向另一个容器"),
  LayerInfo(name: "第5阶-时序码",  bit_width: 16, max_values: 65536,  purpose: "tick偏移量"),
  LayerInfo(name: "第6阶-负载码",  bit_width: 22, max_values: 4194304, purpose: "任意数据")
]

func total_bits*(): int =
  result = 0
  for info in LAYER_SPEC:
    result += info.bit_width

func total_bytes*(): int =
  (total_bits() + 7) div 8

func header*(): string =
  var s = "分比容器 · 七阶编码方案\n"
  s &= "─────────────────────────\n"
  for info in LAYER_SPEC:
    s &= info.name & " " & $info.bit_width & " bits"
    s &= " 可表 " & $info.max_values & " 值"
    s &= " [" & info.purpose & "]\n"
  s &= "─────────────────────────\n"
  s &= "总比特: " & $total_bits() & " = " & $total_bytes() & " 字节/容器\n"
  result = s
