## ═══════════════════════════════════════════════════════════════
## 行为系统 · 行动决策 · 执行 · 反馈 · 学习
## ═══════════════════════════════════════════════════════════════
## 核心:
##   行为选择: 基于奖励/处罚/过滤结果选择行动
##   执行: 将决策转化为对容器/符号的具体操作
##   反馈: 执行后收集结果 · 更新奖励/处罚记录
##   学习: 从历史行为中优化未来决策
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json

# ═══════════════════════════════════════════════════════════════
# 行为类型
# ═══════════════════════════════════════════════════════════════
type
  ActionType* = enum
    atPair,         # 声母+韵母配对
    atFold,         # 折叠(用完收卷)
    atMove,         # 移动(容器间转移)
    atCreate,       # 创建新配对/组合
    atDelete,       # 删除(需许可)
    atQuery,        # 查询(状态/历史)
    atArchive,      # 归档
    atReactivate,   # 激活(唤醒休眠)
    atIsolate,      # 隔离(负样本)
    atDefer,        # 推迟(暂不执行)
    atObserve,      # 观察(灰度)

  Action* = object
    kind*: ActionType        # 行为类型
    target*: string          # 目标(符号/容器)
    parameters*: seq[string] # 参数列表
    priority*: float         # 优先级(0-1)
    reason*: string          # 为什么这样做
    requires_permit*: bool   # 是否需要主人许可
    irreversible*: bool       # 是否不可逆

# ═══════════════════════════════════════════════════════════════
# 行为结果
# ═══════════════════════════════════════════════════════════════
type
  ActionResult* = object
    action*: Action
    success*: bool
    actual_outcome*: string
    side_effects*: seq[string]
    timestamp*: string

# ═══════════════════════════════════════════════════════════════
# 行为规则: 什么情况下做什么
# ═══════════════════════════════════════════════════════════════
type
  BehaviorRule* = object
    name*: string
    condition*: string        # 条件描述
    action*: ActionType       # 对应行为
    priority*: float          # 规则优先级

# ═══════════════════════════════════════════════════════════════
# 行为历史
# ═══════════════════════════════════════════════════════════════
type
  BehavioralSystem* = object
    rules*: seq[BehaviorRule]
    history*: seq[ActionResult]
    pending*: seq[Action]           # 待执行队列
    success_count*: Table[string, int]
    failure_count*: Table[string, int]
    learning_rate*: float           # 学习速率

func new_behavioral_system*(): BehavioralSystem =
  result.rules = @[]
  result.history = @[]
  result.pending = @[]
  result.success_count = initTable[string, int]()
  result.failure_count = initTable[string, int]()
  result.learning_rate = 0.1
  
  # 默认行为规则
  result.rules.add(BehaviorRule(
    name: "白名单配对", condition: "符号在白名单",
    action: atPair, priority: 0.8
  ))
  result.rules.add(BehaviorRule(
    name: "黑名单隔离", condition: "符号在黑名单",
    action: atIsolate, priority: 1.0
  ))
  result.rules.add(BehaviorRule(
    name: "用完折叠", condition: "符号已遍历完",
    action: atFold, priority: 0.7
  ))
  result.rules.add(BehaviorRule(
    name: "灰度观察", condition: "符号未定义",
    action: atObserve, priority: 0.5
  ))
  result.rules.add(BehaviorRule(
    name: "越权推迟", condition: "需主人许可",
    action: atDefer, priority: 0.9
  ))
  result.rules.add(BehaviorRule(
    name: "损坏恢复", condition: "发现损坏符号",
    action: atReactivate, priority: 0.6
  ))

# ═══════════════════════════════════════════════════════════════
# 决策: 根据情境选择行为
# ═══════════════════════════════════════════════════════════════
proc decide*(bs: var BehavioralSystem; symbol: string;
             is_whitelist: bool; is_blacklist: bool;
             is_exhausted: bool; needs_permit: bool;
             is_undefined: bool): Action =
  ## 综合所有系统的裁决 · 选择一个行动
  
  # 最高优先级: 黑名单 → 隔离
  if is_blacklist:
    return Action(kind: atIsolate, target: symbol, priority: 1.0,
                  reason: "黑名单符号 · 立即隔离",
                  requires_permit: false, irreversible: false)
  
  # 需要许可 → 推迟
  if needs_permit:
    return Action(kind: atDefer, target: symbol, priority: 0.9,
                  reason: "需要主人许可 · 推迟执行",
                  requires_permit: true, irreversible: false)
  
  # 白名单 → 配对
  if is_whitelist:
    return Action(kind: atPair, target: symbol, priority: 0.8,
                  reason: "白名单基元 · 正常配对",
                  requires_permit: false, irreversible: false)
  
  # 用完 → 折叠
  if is_exhausted:
    return Action(kind: atFold, target: symbol, priority: 0.7,
                  reason: "符号用完 · 折叠收卷",
                  requires_permit: false, irreversible: false)
  
  # 未定义 → 观察
  if is_undefined:
    return Action(kind: atObserve, target: symbol, priority: 0.5,
                  reason: "未定义符号 · 灰度观察",
                  requires_permit: false, irreversible: false)
  
  # 默认: 推迟
  return Action(kind: atDefer, target: symbol, priority: 0.3,
                reason: "无匹配规则 · 推迟观察",
                requires_permit: false, irreversible: false)

# ═══════════════════════════════════════════════════════════════
# 执行行为
# ═══════════════════════════════════════════════════════════════
proc execute*(bs: var BehavioralSystem; action: Action;
              permit_given: bool = false): ActionResult =
  var result_action = action
  var success = true
  var outcome = ""
  var side_effects: seq[string] = @[]
  
  # 检查许可
  if action.requires_permit and not permit_given:
    success = false
    outcome = "缺少许可 · 执行被拒绝"
    bs.failure_count[action.target] = bs.failure_count.getOrDefault(action.target) + 1
  else:
    case action.kind:
    of atPair:
      outcome = "执行配对: " & action.target
      side_effects.add("产出新组合")
      bs.success_count["pair"] = bs.success_count.getOrDefault("pair") + 1
    of atIsolate:
      outcome = "隔离负样本: " & action.target
      side_effects.add("加入隔离区")
      side_effects.add("永不复用")
      bs.success_count["isolate"] = bs.success_count.getOrDefault("isolate") + 1
    of atFold:
      outcome = "折叠收卷: " & action.target
      side_effects.add("进入折叠层")
      bs.success_count["fold"] = bs.success_count.getOrDefault("fold") + 1
    of atObserve:
      outcome = "灰度观察: " & action.target
      side_effects.add("进入观察队列")
      bs.success_count["observe"] = bs.success_count.getOrDefault("observe") + 1
    of atDefer:
      outcome = "推迟执行: " & action.target
      bs.pending.add(action)
      bs.success_count["defer"] = bs.success_count.getOrDefault("defer") + 1
    of atReactivate:
      outcome = "激活: " & action.target
      side_effects.add("符号恢复活跃")
      bs.success_count["reactivate"] = bs.success_count.getOrDefault("reactivate") + 1
    of atArchive:
      outcome = "归档: " & action.target
      side_effects.add("进入历史容器")
      bs.success_count["archive"] = bs.success_count.getOrDefault("archive") + 1
    else:
      outcome = "未定义行为: " & $action.kind
      success = false
  
  return ActionResult(
    action: result_action,
    success: success,
    actual_outcome: outcome,
    side_effects: side_effects,
    timestamp: "now"
  )

# ═══════════════════════════════════════════════════════════════
# 学习和适应
# ═══════════════════════════════════════════════════════════════
proc learn*(bs: var BehavioralSystem; result: ActionResult) =
  ## 从行为结果中学习
  let key = $result.action.kind
  if result.success:
    bs.success_count[key] = bs.success_count.getOrDefault(key) + 1
    # 成功 → 提高同类行为优先级
    for i in 0..<bs.rules.len:
      if bs.rules[i].action == result.action.kind:
        bs.rules[i].priority = min(1.0, bs.rules[i].priority + bs.learning_rate * 0.1)
  
  else:
    bs.failure_count[key] = bs.failure_count.getOrDefault(key) + 1
    # 失败 → 降低同类行为优先级
    for i in 0..<bs.rules.len:
      if bs.rules[i].action == result.action.kind:
        bs.rules[i].priority = max(0.0, bs.rules[i].priority - bs.learning_rate * 0.2)

# ═══════════════════════════════════════════════════════════════
# 批量行为流水线
# ═══════════════════════════════════════════════════════════════
proc pipeline*(bs: var BehavioralSystem; decisions: seq[tuple[
  symbol: string; whitelist: bool; blacklist: bool;
  exhausted: bool; permit: bool; undefined: bool]]): seq[ActionResult] =
  for d in decisions:
    let action = bs.decide(d.symbol, d.whitelist, d.blacklist,
                           d.exhausted, d.permit, d.undefined)
    let r = bs.execute(action, true)  # 自动许可
    bs.learn(r)
    result.add(r)

# ═══════════════════════════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════════════════════════
proc report*(bs: BehavioralSystem) =
  echo "═══ 行为系统报告 ═══"
  echo ""
  echo "── 行为规则 ──"
  for rule in bs.rules:
    echo "  ", alignLeft(rule.name, 10), " ", alignLeft($rule.action, 8), " 优先级:", formatFloat(rule.priority, ffDecimal, 2), " | ", rule.condition
  echo ""
  echo "── 行为统计 ──"
  echo "  成功记录:"
  for k, v in bs.success_count:
    echo "    ", alignLeft(k, 8), ": ", v
  echo "  失败记录:"
  for k, v in bs.failure_count:
    echo "    ", alignLeft(k, 8), ": ", v
  echo "  待执行: ", bs.pending.len
  echo "  历史记录: ", bs.history.len
  echo "  学习速率: ", bs.learning_rate
  echo ""
  echo "── 行为历史(最近5条) ──"
  let start = max(0, bs.history.len - 5)
  for i in start..<bs.history.len:
    let h = bs.history[i]
    let status = if h.success: "✓" else: "✗"
    echo "  ", status, " ", alignLeft($h.action.kind, 8), " ", h.action.target, " → ", h.actual_outcome

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var bs = new_behavioral_system()
  
  echo "═══ 行为系统 · 决策演示 ═══"
  echo "规则数: ", bs.rules.len
  echo ""
  
  # ── 场景模拟 ──
  echo "── 场景1: 白名单符号 ──"
  let a1 = bs.decide("b", true, false, false, false, false)
  let r1 = bs.execute(a1)
  bs.learn(r1)
  bs.history.add(r1)
  echo "  决策: ", a1.kind, " ", a1.target, " 原因:", a1.reason
  echo "  结果: ", r1.actual_outcome
  
  echo ""
  echo "── 场景2: 黑名单符号 ──"
  let a2 = bs.decide("ꬰ", false, true, false, false, false)
  let r2 = bs.execute(a2)
  bs.learn(r2)
  bs.history.add(r2)
  echo "  决策: ", a2.kind, " ", a2.target, " 原因:", a2.reason
  echo "  结果: ", r2.actual_outcome
  echo "  副作用: ", r2.side_effects.join(", ")
  
  echo ""
  echo "── 场景3: 未定义符号 ──"
  let a3 = bs.decide("X", false, false, false, false, true)
  let r3 = bs.execute(a3)
  bs.learn(r3)
  bs.history.add(r3)
  echo "  决策: ", a3.kind, " ", a3.target, " 原因:", a3.reason
  echo "  结果: ", r3.actual_outcome
  
  echo ""
  echo "── 场景4: 需要许可 ──"
  let a4 = bs.decide("重要符号", true, false, false, true, false)
  let r4 = bs.execute(a4, false)  # 不给许可
  bs.learn(r4)
  bs.history.add(r4)
  echo "  决策: ", a4.kind, " ", a4.target, " 原因:", a4.reason
  echo "  结果: ", r4.actual_outcome, " (许可:", false, ")"
  
  echo ""
  echo "── 场景5: 用完折叠 ──"
  let a5 = bs.decide("a", true, false, true, false, false)
  let r5 = bs.execute(a5)
  bs.learn(r5)
  bs.history.add(r5)
  echo "  决策: ", a5.kind, " ", a5.target, " 原因:", a5.reason
  echo "  结果: ", r5.actual_outcome
  
  echo ""
  
  # ── 报告 ──
  report(bs)
