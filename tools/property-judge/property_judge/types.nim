# ============================================================
# 产权流 · 类型定义
# ============================================================

import tables
import strutils
import options

# 容器主权状态
type
  PropertyStateKind* = enum
    pskOwnerless
    pskPrivate
    pskCoupleShared
    pskCollective
    pskDisputed

  PropertyState* = object
    case kind*: PropertyStateKind
    of pskPrivate:
      owner_id*: string
    of pskCoupleShared:
      owner_a*: string
      owner_b*: string
    of pskCollective:
      owners*: seq[string]
    of pskDisputed:
      claimants*: seq[string]
    of pskOwnerless:
      discard

# 产权流类型
type
  FlowType* = enum
    ftCreation       # 创建
    ftTransfer       # 转移
    ftTransformation # 变换
    ftMerge          # 合并
    ftSplit          # 拆分
    ftDestruction    # 销毁
    ftBorrow         # 借用
    ftLease          # 租赁

# 授权级别
type
  AuthorizationKind* = enum
    akFull         # 完全授权
    akBounded      # 部分授权（有边界）
    akConditional  # 条件授权
    akNone         # 未授权
    akRevoked      # 授权撤回

  Authorization* = object
    case kind*: AuthorizationKind
    of akBounded:
      boundary*: string
    of akConditional:
      condition*: string
    else:
      discard

# 侵权类型
type
  InfringementType* = enum
    itNone            # 无侵权
    itMisappropriation # 挪用侵权
    itOverboundary    # 越界侵权
    itConcealment     # 隐匿侵权
    itTampering       # 篡改侵权
    itTheft           # 盗用侵权
    itDestruction     # 破坏侵权
    itDisputed        # 争议

# 容器信息
type
  ContainerInfo* = object
    id*: string
    state*: PropertyState
    created_at*: int
    metadata*: Table[string, string]

# 产权流事件
type
  PropertyFlow* = object
    flow_id*: string
    flow_type*: FlowType
    source_id*: string
    target_id*: string
    operator_id*: string
    authorization*: Authorization
    attributes*: Table[string, string]
    timestamp*: int

# 判定结果
type
  Judgment* = object
    infringement*: InfringementType
    path*: seq[string]
    confidence*: float
    explanation*: string
    recommendation*: string

# 辅助函数
func flow_type_desc*(ft: FlowType): string =
  case ft
  of ftCreation: "创建"
  of ftTransfer: "转移"
  of ftTransformation: "变换"
  of ftMerge: "合并"
  of ftSplit: "拆分"
  of ftDestruction: "销毁"
  of ftBorrow: "借用"
  of ftLease: "租赁"

func auth_desc*(a: Authorization): string =
  case a.kind
  of akFull: "完全授权"
  of akBounded: "部分授权(边界:" & a.boundary & ")"
  of akConditional: "条件授权(条件:" & a.condition & ")"
  of akNone: "未授权"
  of akRevoked: "授权撤回"

func infringement_desc*(it: InfringementType): string =
  case it
  of itNone: "无侵权"
  of itMisappropriation: "挪用侵权"
  of itOverboundary: "越界侵权"
  of itConcealment: "隐匿侵权"
  of itTampering: "篡改侵权"
  of itTheft: "盗用侵权"
  of itDestruction: "破坏侵权"
  of itDisputed: "争议"

func state_desc*(ps: PropertyState): string =
  case ps.kind
  of pskOwnerless: "无主"
  of pskPrivate: "私有(" & ps.owner_id & ")"
  of pskCoupleShared: "共有(" & ps.owner_a & "," & ps.owner_b & ")"
  of pskCollective: "集体(" & ps.owners.join(",") & ")"
  of pskDisputed: "争议(" & ps.claimants.join(",") & ")"

# 构造函数
func make_flow*(id: string, ft: FlowType, source, operator: string): PropertyFlow =
  PropertyFlow(
    flow_id: id,
    flow_type: ft,
    source_id: source,
    target_id: "",
    operator_id: operator,
    authorization: Authorization(kind: akNone),
    attributes: init_table[string, string](),
    timestamp: 0
  )

func with_target*(f: PropertyFlow, target: string): PropertyFlow =
  result = f
  result.target_id = target

func with_auth*(f: PropertyFlow, a: Authorization): PropertyFlow =
  result = f
  result.authorization = a

func with_attr*(f: PropertyFlow, key, val: string): PropertyFlow =
  result = f
  result.attributes[key] = val

# 主权判断
func is_owner*(state: PropertyState, operator: string): bool =
  case state.kind
  of pskOwnerless: true
  of pskPrivate: state.owner_id == operator
  of pskCoupleShared: state.owner_a == operator or state.owner_b == operator
  of pskCollective: state.owners.contains(operator)
  of pskDisputed: state.claimants.contains(operator)
