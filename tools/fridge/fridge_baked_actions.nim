## 预烘焙动作序列：编译时生成完整动作流
## 概念：把一连串状态转移的动作预先"烘焙"成不可变序列
## 运行时直接迭代执行，无需判断/查找/计算

import std/[sequtils, strutils, tables]
import fridge_types, fridge

# ═══════════════════════════════════════════════════
#  动作序列器：在编译时把路径烘焙成动作列表
# ═══════════════════════════════════════════════════

type
  BakedStep* = object
    action*: Action       ## 执行动作
    target*: StateSymbol  ## 目标状态
    annotation*: string   ## 人类可读注释

  BakedRecipe* = object
    name*: string
    steps*: seq[BakedStep]
    initialState*: StateSymbol
    finalState*: StateSymbol
    polarityTrace*: seq[Polarity]  ## 极性轨迹

# ── 烘焙函数（编译时调用） ──────────────────────────────

func bakePath*(name: string; plate: StewedPlate;
               path: seq[(StateSymbol, Event)]): BakedRecipe =
  ## 给定路径烘焙成不可变动作序列
  result.name = name
  result.steps = newSeq[BakedStep]()
  result.polarityTrace = newSeq[Polarity]()

  if path.len == 0: return

  result.initialState = path[0][0]

  var currentState = path[0][0]
  for (state, event) in path:
    let trans = plate.serve(state, event)
    var step: BakedStep
    step.action = trans.action
    step.target = trans.toState
    step.annotation = "在 " & $state & " 遇 " & $event & " → " & $trans.action
    result.steps.add(step)
    result.polarityTrace.add(trans.polarity)
    currentState = trans.toState

  result.finalState = currentState

# ── 执行预烘焙序列 ──────────────────────────────────────

proc executeBaked*(recipe: BakedRecipe) =
  ## 顺序执行预烘焙动作序列（无分支、无判断、纯遍历）
  echo "━━━ 执行预烘焙序列：", recipe.name, "━━━"
  echo "  起始: ", $recipe.initialState
  echo "  步骤数: ", recipe.steps.len
  echo ""

  for i, step in recipe.steps:
    # 直接执行动作 — 无查找、无判断、无计算
    echo "  [步骤 ", i+1, "] ", step.annotation
    echo "    → 执行: ", step.action
    echo "    → 进入: ", $step.target

  echo ""
  echo "  结束状态: ", $recipe.finalState
  echo "  极性轨迹: ", recipe.polarityTrace.mapIt($it).join(" → ")
  echo "━━━ 执行完毕 ━━━"

# ═══════════════════════════════════════════════════
#  演示：两条不同路径的烘焙
# ═══════════════════════════════════════════════════

const NJSG_RECIPE: Recipe = Recipe(rules: @[
  Transition(fromState: ssBag,      event: evOpen,   action: acAccept,  toState: ssYinYang,  polarity: polYin),
  Transition(fromState: ssYinYang,  event: evTouch,  action: acAccept,  toState: ssAdd,      polarity: polBoth),
  Transition(fromState: ssAdd,      event: evJudge,  action: acPass,    toState: ssJudge,    polarity: polYang),
  Transition(fromState: ssJudge,    event: evFold,   action: acAccept,  toState: ssNegative, polarity: polYin),
  Transition(fromState: ssNegative, event: evJudge,  action: acRestore, toState: ssBag,      polarity: polBoth),
  Transition(fromState: ssAdd,      event: evOpen,   action: acAccept,  toState: ssPhoenix,  polarity: polBoth),
  Transition(fromState: ssPhoenix,  event: evTouch,  action: acAccept,  toState: ssBlessing, polarity: polBoth),
  Transition(fromState: ssBlessing, event: evOpen,   action: acHold,    toState: ssBlessing, polarity: polYang),
  Transition(fromState: ssJudge,    event: evClose,  action: acAccept,  toState: ssPhoenix,  polarity: polBoth),
  Transition(fromState: ssPhoenix,  event: evClose,  action: acHold,    toState: ssRest,     polarity: polYin),
  Transition(fromState: ssRest,     event: evPeek,   action: acAccept,  toState: ssSee,      polarity: polYang),
])

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════╗"
  echo "║   预烘焙动作序列演示                      ║"
  echo "║   Baked Actions from NJSG FSM            ║"
  echo "╚══════════════════════════════════════════╝"
  echo ""

  let plate = stew(NJSG_RECIPE)

  # 路径1：袋 → 加法 → 审判 → 负样 → 复位
  let path1 = @[
    (ssBag, evOpen),
    (ssYinYang, evTouch),
    (ssAdd, evJudge),
    (ssJudge, evFold),
    (ssNegative, evJudge),
  ]
  let baked1 = bakePath("囡囝相知", plate, path1)
  executeBaked(baked1)

  echo ""

  # 路径2：袋 → 加法 → 凤凰 → 福禄 → 栖
  let path2 = @[
    (ssBag, evOpen),
    (ssYinYang, evTouch),
    (ssAdd, evOpen),
    (ssPhoenix, evTouch),
    (ssBlessing, evOpen),
  ]
  let baked2 = bakePath("凤栖悟道", plate, path2)
  executeBaked(baked2)

  echo ""

  # 路径3：从审判 → 凤凰 → 栖居 → 觉察（循环路径）
  let path3 = @[
    (ssJudge, evClose),
    (ssPhoenix, evClose),
    (ssRest, evPeek),
  ]
  let baked3 = bakePath("闭环回归", plate, path3)
  executeBaked(baked3)

  echo "╔══════════════════════════════════════════╗"
  echo "║   三条路径全部预烘焙完成                  ║"
  echo "║   每条路径在执行期只做遍历               ║"
  echo "╚══════════════════════════════════════════╝"
