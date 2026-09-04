## 冰箱引擎：预计算状态机
## 编译焖煮，运行时只取用
##
## 使用说明：
##   1. 定义配方（recipe）
##   2. 调用 stew() 焖煮成 plate
##   3. 运行时调用 serve() 查表，无需任何计算

import std/[tables]
import fridge_types

# ── 焖煮结果 ────────────────────────────────────────────

type
  ## 焖煮结果：编译时生成，运行时只读
  StewedPlate* = object
    ## 核心查表：从 (state * event) 到 transition 的映射
    lut*: Table[(StateSymbol, Event), Transition]
    ## 默认转移（未匹配时）
    defaultTransition*: Transition
    ## 元信息
    ruleCount*: int
    stateCount*: int
    isSealed*: bool  ## 封印后不可修改

# ── 配方验证错误 ────────────────────────────────────────

type
  FridgeError* = object of CatchableError
  ConflictingRuleError* = object of FridgeError
  UnreachableStateError* = object of FridgeError

# ── 焖煮过程 ────────────────────────────────────────────

func validateRecipe(recipe: Recipe): seq[string] =
  ## 编译时验证配方：检测冲突和死状态
  var warnings: seq[string] = @[]
  var seen: Table[(StateSymbol, Event), Transition]

  for rule in recipe.rules:
    let key = (rule.fromState, rule.event)
    if seen.hasKey(key):
      warnings.add(
        "冲突规则: [" & $rule.fromState & " × " & $rule.event & "] " &
        "已有(" & ($seen[key].action) & " → " & ($seen[key].toState) & ") " &
        "重复定义(" & ($rule.action) & " → " & ($rule.toState) & ")"
      )
    else:
      seen[key] = rule

  # 检查可达性：除 ssNone 外，所有状态必须被某条规则指向
  var reachable: set[StateSymbol] = {}
  for rule in recipe.rules:
    reachable.incl(rule.toState)
  for state in StateSymbol.low .. StateSymbol.high:
    if state == ssNone: continue
    if state notin reachable and state != ssBag:
      # 检查是否有从此状态出发的转移
      var hasOut = false
      for rule in recipe.rules:
        if rule.fromState == state:
          hasOut = true
          break
      if not hasOut:
        warnings.add("死状态: [" & $state & "] 无转入且无转出")

  result = warnings

proc validateToStdout*(recipe: Recipe): seq[string] =
  ## 打印验证警告到 stdout 并返回
  for w in validateRecipe(recipe):
    echo w
  result = validateRecipe(recipe)

func stew*(recipe: Recipe): StewedPlate =
  ## 焖煮：将配方编译成可查表的 plate
  ## 无副作用（不执行 IO）

  result.ruleCount = recipe.rules.len
  result.isSealed = true
  result.defaultTransition = Transition(
    fromState: ssNone, event: evNone,
    action: acHold, toState: ssNone, polarity: polBoth
  )

  var states: set[StateSymbol] = {}
  for rule in recipe.rules:
    result.lut[(rule.fromState, rule.event)] = rule
    states.incl(rule.fromState)
    states.incl(rule.toState)

  result.stateCount = states.len

# ── 运行时取用 ──────────────────────────────────────────

func serve*(plate: StewedPlate; state: StateSymbol; event: Event): Transition =
  ## 上菜：给定当前状态和事件，直接查表返回转移
  ## 无任何计算，纯 O(1) 查表
  let key = (state, event)
  if plate.lut.hasKey(key):
    result = plate.lut[key]
  else:
    result = plate.defaultTransition

func serveBatch*(plate: StewedPlate; path: seq[(StateSymbol, Event)]): seq[Transition] =
  ## 批量上菜：给定状态-事件路径，返回转移序列
  result = newSeq[Transition](path.len)
  for i, (state, event) in path:
    result[i] = plate.serve(state, event)

# ── 惰性焖煮模板 ────────────────────────────────────────
## 用户可在编译期定义完整配方，通过 stewNim() 模板生成常量

# ── 打印辅助 ────────────────────────────────────────────

proc printPlate*(plate: StewedPlate; recipe: Recipe) =
  ## 打印焖煮结果概览
  echo "═══════════════════════════════════════════════"
  echo "  冰箱焖煮结果"
  echo "═══════════════════════════════════════════════"
  echo "  规则数:  ", plate.ruleCount
  echo "  状态数:  ", plate.stateCount
  echo "  封印态:  ", if plate.isSealed: "已封印" else: "未封印"
  echo "───────────────────────────────────────────────"
  echo "  状态    事件      →  动作    目标状态  极性"
  echo "───────────────────────────────────────────────"
  for trans in recipe.rules:
    echo "  ", $trans.fromState, " × ", trans.event, " → ", trans.action, " → ", trans.toState, " (", trans.polarity, ")"
  echo "═══════════════════════════════════════════════"
