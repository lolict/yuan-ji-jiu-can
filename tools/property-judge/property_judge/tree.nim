# ============================================================
# 产权流 · 侵权判定树
# 
# 判定树结构:
#   ┌─────────────────────────────────────────────────────────────┐
#   │  1. 容器存在? ──→ 否 → 盗用(非法构造)                       │
#   │        ↓ 是                                                  │
#   │  2. 有主? ──→ 否 → 无主容器,自由占用                         │
#   │        ↓ 是                                                  │
#   │  3. 操作人是主人? ──→ 是 → 检查授权撤回 → 无侵权/挪用        │
#   │        ↓ 否                                                  │
#   │  4. 授权级别? ──→ 完全/部分/条件/撤回/无                     │
#   │        ↓ 无授权                                              │
#   │  5. 隐匿? ──→ 是 → 隐匿侵权                                  │
#   │        ↓ 否                                                  │
#   │  6. 篡改? ──→ 是 → 篡改侵权                                  │
#   │        ↓ 否                                                  │
#   │  7. 破坏? ──→ 是 → 破坏侵权                                  │
#   │        ↓ 否                                                  │
#   │  8. 盗用? ──→ 是 → 盗用侵权                                  │
#   │        ↓ 否                                                  │
#   │  9. 挪用? ──→ 是 → 挪用侵权                                  │
#   │        ↓ 否                                                  │
#   │  → 合理使用                                                  │
#   └─────────────────────────────────────────────────────────────┘
# ============================================================

import tables
import strutils
import sequtils
import options
import types

# 判定引擎
type
  PropertyJudge* = object
    containers*: Table[string, ContainerInfo]
    history*: seq[(PropertyFlow, Judgment)]
    total_judged*: int
    total_infringement*: int

# 创建引擎
func new_judge*(): PropertyJudge =
  PropertyJudge(
    containers: init_table[string, ContainerInfo](),
    history: @[],
    total_judged: 0,
    total_infringement: 0
  )

# 注册容器
func register*(j: var PropertyJudge, info: ContainerInfo) =
  j.containers[info.id] = info

# 获取容器状态
func get_state*(j: PropertyJudge, id: string): Option[PropertyState] =
  if j.containers.hasKey(id):
    return some(j.containers[id].state)
  return none(PropertyState)

# 无侵权判定
func clean_judgment*(path: seq[string]): Judgment =
  Judgment(
    infringement: itNone,
    path: path,
    confidence: 1.0,
    explanation: "无侵权: 操作合法",
    recommendation: "无需处理"
  )

# 侵权判定
func infringing_judgment*(path: seq[string], it: InfringementType, reason: string): Judgment =
  let rec = case it
    of itMisappropriation: "立即停止挪用, 恢复原状"
    of itOverboundary: "退回授权边界内, 申请扩展授权"
    of itConcealment: "公开容器来源, 补全历史记录"
    of itTampering: "恢复原始编码, 记录篡改行为"
    of itTheft: "立即归还容器, 记录盗窃行为"
    of itDestruction: "赔偿损失, 记录破坏行为"
    of itDisputed: "提交仲裁, 暂停争议操作"
    of itNone: "无需处理"
  
  Judgment(
    infringement: it,
    path: path,
    confidence: 0.85,
    explanation: reason,
    recommendation: rec
  )

# ────────────────────────────────────────────────────────────
# 核心判定函数
# ────────────────────────────────────────────────────────────

func judge*(j: var PropertyJudge, flow: PropertyFlow): Judgment =
  var path: seq[string] = @["root"]
  inc j.total_judged
  
  # ─── 节点1: 容器是否存在? ───
  let state_opt = j.get_state(flow.source_id)
  if state_opt.isNone():
    path.add("container_not_found")
    let judgment = infringing_judgment(path, itTheft,
      "容器 " & flow.source_id & " 不存在于注册表中, 可能为非法构造")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  let state = state_opt.get()
  path.add("container_exists")
  
  # ─── 节点2: 容器是否有主? ───
  if state.kind == pskOwnerless:
    path.add("ownerless")
    let judgment = clean_judgment(path)
    j.history.add((flow, judgment))
    return judgment
  
  path.add("has_owner")
  
  # ─── 节点3: 操作人是否为主人? ───
  let owner_op = state.is_owner(flow.operator_id)
  
  if owner_op:
    path.add("operator_is_owner")
    # 主人操作自己的容器，检查授权是否被撤回
    if flow.authorization.kind == akRevoked:
      path.add("auth_revoked")
      let judgment = infringing_judgment(path, itMisappropriation,
        "授权已被撤回, 操作无效")
      j.history.add((flow, judgment))
      inc j.total_infringement
      return judgment
    
    path.add("auth_valid")
    let judgment = clean_judgment(path)
    j.history.add((flow, judgment))
    return judgment
  
  # ─── 节点4: 非主人操作，检查授权 ───
  path.add("operator_not_owner")
  
  case flow.authorization.kind
  of akFull:
    path.add("full_auth")
    let judgment = clean_judgment(path)
    j.history.add((flow, judgment))
    return judgment
  
  of akBounded:
    path.add("bounded_auth")
    # 检查是否越界
    var within = true
    if flow.attributes.hasKey("scope"):
      let scope = flow.attributes["scope"]
      if scope != flow.authorization.boundary:
        within = false
    
    if not within:
      path.add("boundary_violated")
      let judgment = infringing_judgment(path, itOverboundary,
        "超出授权边界: " & flow.authorization.boundary)
      j.history.add((flow, judgment))
      inc j.total_infringement
      return judgment
    
    path.add("within_boundary")
    let judgment = clean_judgment(path)
    j.history.add((flow, judgment))
    return judgment
  
  of akConditional:
    path.add("conditional_auth")
    # 检查条件是否满足
    var met = false
    if flow.attributes.hasKey("condition_status"):
      let status = flow.attributes["condition_status"]
      if status == "satisfied" or status == flow.authorization.condition:
        met = true
    
    if not met:
      path.add("condition_not_met")
      let judgment = infringing_judgment(path, itMisappropriation,
        "未满足授权条件: " & flow.authorization.condition)
      j.history.add((flow, judgment))
      inc j.total_infringement
      return judgment
    
    path.add("condition_met")
    let judgment = clean_judgment(path)
    j.history.add((flow, judgment))
    return judgment
  
  of akRevoked:
    path.add("auth_revoked")
    let judgment = infringing_judgment(path, itTheft,
      "授权已被撤回, 任何操作均属盗用")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  of akNone:
    path.add("no_auth")
    # 继续到节点5
  
  # ─── 节点5-9: 无授权，按特异性检查侵权类型 ───
  path.add("no_authorization")
  
  # 节点5: 隐匿检查（隐藏来源）
  if flow.attributes.hasKey("source_hidden"):
    if flow.attributes["source_hidden"] == "true":
      path.add("concealment")
      let judgment = infringing_judgment(path, itConcealment, "隐藏容器来源或历史")
      j.history.add((flow, judgment))
      inc j.total_infringement
      return judgment
  
  # 节点6: 篡改检查（变换无授权）
  if flow.flow_type == ftTransformation:
    path.add("tampering")
    let judgment = infringing_judgment(path, itTampering, "擅自修改容器编码或属性")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  # 节点7: 破坏检查（销毁无授权）
  if flow.flow_type == ftDestruction:
    path.add("destruction")
    let judgment = infringing_judgment(path, itDestruction, "未经授权销毁他人容器")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  # 节点8: 盗用检查（转移无授权）
  if flow.flow_type == ftTransfer:
    path.add("theft")
    let judgment = infringing_judgment(path, itTheft, "非主人未经授权转移他人容器")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  # 节点9: 挪用检查（租赁无授权）
  if flow.flow_type == ftLease:
    path.add("misappropriation")
    let judgment = infringing_judgment(path, itMisappropriation, "未经授权租赁容器")
    j.history.add((flow, judgment))
    inc j.total_infringement
    return judgment
  
  # 默认: 合理使用
  path.add("fair_use")
  let judgment = clean_judgment(path)
  j.history.add((flow, judgment))
  return judgment

# ────────────────────────────────────────────────────────────
# 统计与报告
# ────────────────────────────────────────────────────────────

func judge_report*(j: PropertyJudge): string =
  var s = "═══ 产权流侵权判定报告 ═══\n"
  s &= "注册容器: " & $len(j.containers) & "\n"
  s &= "判定总数: " & $j.total_judged & "\n"
  s &= "侵权总数: " & $j.total_infringement & "\n"
  if j.total_judged > 0:
    let rate = float(j.total_infringement) / float(j.total_judged) * 100.0
    s &= "侵权率: " & formatFloat(rate, ffDecimal, 1) & "%\n"
  s &= "══════════════════════════\n"
  result = s

func judgment_detail*(j: PropertyJudge, idx: int): string =
  if idx < 0 or idx >= j.history.len:
    return "索引越界"
  
  let (flow, judgment) = j.history[idx]
  var s = "─── 判定 #" & $idx & " ───\n"
  s &= "流: " & flow.flow_id & " [" & flow_type_desc(flow.flow_type) & "]\n"
  s &= "源: " & flow.source_id & " 操作人: " & flow.operator_id & "\n"
  s &= "授权: " & auth_desc(flow.authorization) & "\n"
  s &= "结果: " & infringement_desc(judgment.infringement) & "\n"
  s &= "路径: " & judgment.path.join(" → ") & "\n"
  s &= "说明: " & judgment.explanation & "\n"
  s &= "建议: " & judgment.recommendation & "\n"
  result = s
