## ═══════════════════════════════════════════════════════════════
## 思考系统 · 认知处理 · 推理 · 规划 · 决策 · 自省
## ═══════════════════════════════════════════════════════════════
## 核心:
##   推理: 从已知推未知 · 因果链追踪
##   规划: 多步骤行动方案生成
##   决策: 基于多维度评估选择最优
##   自省: 检查自身错误 · 记录改进
##   创造: 新组合/新规则的发现
## ═══════════════════════════════════════════════════════════════

import tables, strutils, sequtils, math

# ═══════════════════════════════════════════════════════════════
# 思维模式
# ═══════════════════════════════════════════════════════════════
type
  ThoughtMode* = enum
    tmDeductive,     # 演绎(一般→具体)
    tmInductive,     # 归纳(具体→一般)
    tmAbductive,     # 回溯(结果→原因)
    tmAnalogical,    # 类比(相似→相似)
    tmSystemic,      # 系统(整体→部分)
    tmCritical       # 批判(质疑→验证)

  ThoughtStep* = object
    mode*: ThoughtMode         # 使用的思维模式
    premise*: string            # 前提
    inference*: string          # 推理过程
    conclusion*: string         # 结论
    confidence*: float          # 置信度

# ═══════════════════════════════════════════════════════════════
# 规划
# ═══════════════════════════════════════════════════════════════
type
  Plan* = object
    goal*: string              # 目标
    steps*: seq[string]        # 步骤列表
    dependencies*: Table[string, seq[string]]  # 依赖
    estimated_steps*: int      # 预计步数
    fallback*: string          # 备选方案

# ═══════════════════════════════════════════════════════════════
# 问题
# ═══════════════════════════════════════════════════════════════
type
  Problem* = object
    description*: string        # 问题描述
    constraints*: seq[string]   # 约束条件
    known_facts*: seq[string]   # 已知事实
    goal*: string              # 目标状态

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  ThinkingSystem* = object
    thought_chain*: seq[ThoughtStep]  # 思维链
    plans*: seq[Plan]                 # 已生成的计划
    known_rules*: seq[string]         # 已知规则
    assumption_log*: seq[string]      # 假设记录
    error_log*: seq[string]           # 错误日志
    insight_log*: seq[string]         # 洞察记录
    max_chain_length*: int            # 最大思维链长

func new_thinking_system*(): ThinkingSystem =
  result.thought_chain = @[]
  result.plans = @[]
  result.known_rules = @["基元冻结不可违反", "零维根不可修改", "后来的不能冲撞元老"]
  result.assumption_log = @[]
  result.error_log = @[]
  result.insight_log = @[]
  result.max_chain_length = 10

# ═══════════════════════════════════════════════════════════════
# 演绎推理
# ═══════════════════════════════════════════════════════════════
proc deduce*(ts: var ThinkingSystem; premise: string; rule: string;
             conclusion: string; confidence: float = 0.8) =
  ts.thought_chain.add(ThoughtStep(
    mode: tmDeductive,
    premise: premise,
    inference: "应用规则: " & rule,
    conclusion: conclusion,
    confidence: confidence
  ))

# ═══════════════════════════════════════════════════════════════
# 归纳推理
# ═══════════════════════════════════════════════════════════════
proc induce*(ts: var ThinkingSystem; examples: seq[string];
             pattern: string; confidence: float = 0.6) =
  let premise = "从" & $examples.len & "个例子: " & examples[0] & "..."
  ts.thought_chain.add(ThoughtStep(
    mode: tmInductive,
    premise: premise,
    inference: "发现模式: " & pattern,
    conclusion: "可能存在规律: " & pattern,
    confidence: confidence
  ))

# ═══════════════════════════════════════════════════════════════
# 回溯推理
# ═══════════════════════════════════════════════════════════════
proc abduce*(ts: var ThinkingSystem; effect: string; possible_cause: string;
             confidence: float = 0.5) =
  ts.thought_chain.add(ThoughtStep(
    mode: tmAbductive,
    premise: "观察到: " & effect,
    inference: "最可能原因: " & possible_cause,
    conclusion: "推测: " & possible_cause & " → " & effect,
    confidence: confidence
  ))

# ═══════════════════════════════════════════════════════════════
# 规划生成
# ═══════════════════════════════════════════════════════════════
proc plan*(ts: var ThinkingSystem; goal: string; steps: seq[string];
           fallback: string = "等待主人指导") =
  let p = Plan(
    goal: goal,
    steps: steps,
    dependencies: initTable[string, seq[string]](),
    estimated_steps: steps.len,
    fallback: fallback
  )
  ts.plans.add(p)

# ═══════════════════════════════════════════════════════════════
# 问题求解: 基于已知事实推理
# ═══════════════════════════════════════════════════════════════
proc solve*(ts: var ThinkingSystem; problem: Problem): tuple[solution: string; steps: int; confidence: float] =
  # 检查已知事实
  var relevant_facts: seq[string] = @[]
  for fact in problem.known_facts:
    if fact.contains(problem.goal) or problem.goal.contains(fact):
      relevant_facts.add(fact)
  
  # 选择推理模式
  if relevant_facts.len >= 3:
    # 足够数据 → 归纳
    ts.induce(relevant_facts, "共同模式", 0.7)
    return ("基于" & $relevant_facts.len & "个相关事实归纳求解", relevant_facts.len + 1, 0.7)
  elif relevant_facts.len > 0:
    # 有数据但不多 → 演绎
    ts.deduce(relevant_facts[0], "系统规则", "可求解", 0.8)
    return ("基于演绎推理求解", relevant_facts.len + 2, 0.6)
  else:
    # 无数据 → 回溯
    ts.abduce(problem.description, "需要更多信息", 0.4)
    return ("需要更多信息才能求解", 0, 0.3)

# ═══════════════════════════════════════════════════════════════
# 自省: 检查思维链中的错误
# ═══════════════════════════════════════════════════════════════
proc reflect*(ts: var ThinkingSystem): tuple[issues: seq[string]; improvements: seq[string]] =
  var issues: seq[string] = @[]
  var improvements: seq[string] = @[]
  
  # 1. 检查思维链长度
  if ts.thought_chain.len > ts.max_chain_length:
    issues.add("思维链过长(" & $ts.thought_chain.len & "步) · 可能效率低")
    improvements.add("缩短思维链 · 合并冗余步骤")
  
  # 2. 检查置信度
  var low_confidence_steps: seq[int] = @[]
  for i, step in ts.thought_chain:
    if step.confidence < 0.5:
      low_confidence_steps.add(i + 1)
  if low_confidence_steps.len > 0:
    issues.add("第" & low_confidence_steps.join("/") & "步置信度低")
    improvements.add("增加验证步骤 · 或降低该步权重")
  
  # 3. 检查假设数量
  if ts.assumption_log.len > 5:
    issues.add("假设过多(" & $ts.assumption_log.len & "个) · 可能不稳定")
    improvements.add("验证或消除部分假设")
  
  # 4. 检查错误率
  if ts.error_log.len > 3:
    issues.add("近期错误频繁(" & $ts.error_log.len & "次)")
    improvements.add("回顾错误模式 · 强化规则学习")
  
  return (issues, improvements)

# ═══════════════════════════════════════════════════════════════
# 学习: 从错误中提炼规则
# ═══════════════════════════════════════════════════════════════
proc learn*(ts: var ThinkingSystem; error_description: string; correction: string) =
  ts.error_log.add(error_description)
  let new_rule = "避免:" & error_description & " → 应该:" & correction
  ts.known_rules.add(new_rule)
  ts.insight_log.add(correction)

# ═══════════════════════════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════════════════════════
proc report*(ts: ThinkingSystem) =
  echo "═══ 思考系统报告 ═══"
  echo ""
  echo "── 思维统计 ──"
  echo "  思维链长度: ", ts.thought_chain.len, "步"
  echo "  已生成计划: ", ts.plans.len
  echo "  已知规则: ", ts.known_rules.len
  echo "  假设数量: ", ts.assumption_log.len
  echo "  错误记录: ", ts.error_log.len
  echo "  洞察记录: ", ts.insight_log.len
  echo ""
  
  if ts.thought_chain.len > 0:
    echo "── 最近思维链 ──"
    let start = max(0, ts.thought_chain.len - 3)
    for i in start..<ts.thought_chain.len:
      let step = ts.thought_chain[i]
      echo "  第", i+1, "步 [", alignLeft($step.mode, 10), "] 置信:", formatFloat(step.confidence, ffDecimal, 2)
      echo "    前提: ", step.premise
      echo "    推理: ", step.inference
      echo "    结论: ", step.conclusion
    echo ""
  
  if ts.plans.len > 0:
    echo "── 活跃计划 ──"
    for p in ts.plans:
      echo "  目标: ", p.goal
      echo "  步骤: ", p.estimated_steps, "步 · 备选: ", p.fallback
    echo ""

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var ts = new_thinking_system()
  
  echo "═══ 思考系统 · 认知处理 ═══"
  echo ""
  
  # ── 场景1: 演绎推理 ──
  echo "── 场景1: 演绎推理 ──"
  ts.deduce("符号ꬰ被输入", "黑名单规则", "ꬰ应被隔离", 0.95)
  echo "  结论: ", ts.thought_chain[^1].conclusion
  echo ""
  
  # ── 场景2: 问题求解 ──
  echo "── 场景2: 问题求解 ──"
  let prob = Problem(
    description: "如何处理未知符号X",
    constraints: @["不可自创符号", "不可覆盖用户内容"],
    known_facts: @["X不在白名单", "X不在黑名单", "X是首次出现"],
    goal: "决定X的命运"
  )
  let sol = ts.solve(prob)
  echo "  方案: ", sol.solution
  echo "  置信: ", formatFloat(sol.confidence, ffDecimal, 2)
  echo ""
  
  # ── 场景3: 规划 ──
  echo "── 场景3: 规划生成 ──"
  ts.plan("完成声母容器训练", @["调入声母b", "遍历韵母", "折叠收卷", "调入下一声母"])
  echo "  计划: ", ts.plans[^1].goal, " (", ts.plans[^1].estimated_steps, "步)"
  echo ""
  
  # ── 场景4: 学习 ──
  echo "── 场景4: 从错误中学习 ──"
  ts.learn("使用了AI发明的符号ꬰ", "只使用白名单中的符号")
  ts.learn("覆盖了用户自定义编码", "保护用户符号 · 使用私有编码区")
  echo "  学会了 ", ts.known_rules.len - 3, " 条新规则"
  echo ""
  
  # ── 场景5: 自省 ──
  echo "── 场景5: 自省检查 ──"
  for i in 1..8:
    ts.thought_chain.add(ThoughtStep(mode: tmDeductive, premise: "x", inference: "y", conclusion: "z", confidence: 0.4))
  let (issues, improvements) = ts.reflect()
  echo "  发现问题: ", issues.len
  for issue in issues:
    echo "    ⚠ ", issue
  echo "  改进建议: ", improvements.len
  for imp in improvements:
    echo "    → ", imp
  
  echo ""
  report(ts)
