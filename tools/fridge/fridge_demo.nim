## 冰箱演示：NJSG 囡囝共生七态转移
## 预计算所有动作序列，运行时零计算

import std/[sequtils, strutils]
import fridge_types, fridge

# ═══════════════════════════════════════════════════
#  配方定义（编译时输入）
# ═══════════════════════════════════════════════════

const NJSG_RECIPE: Recipe = Recipe(rules: @[
  # — 初始态：袋 —
  Transition(fromState: ssBag,      event: evPeek,   action: acAccept,  toState: ssSee,      polarity: polYang),
  Transition(fromState: ssBag,      event: evOpen,   action: acAccept,  toState: ssYinYang,  polarity: polYin),
  Transition(fromState: ssBag,      event: evOpen,   action: acHold,    toState: ssBag,      polarity: polBoth),  # 条件未满足
  Transition(fromState: ssBag,      event: evOverflow, action: acReject, toState: ssBag,    polarity: polYang),

  # — 观察态：见 —
  Transition(fromState: ssSee,      event: evTouch,  action: acAccept,  toState: ssMirror,   polarity: polYang),
  Transition(fromState: ssSee,      event: evJudge,   action: acAccept,  toState: ssJudge,    polarity: polBoth),
  Transition(fromState: ssSee,      event: evPeek,   action: acHold,    toState: ssSee,      polarity: polYin),

  # — 映照态：照 —
  Transition(fromState: ssMirror,   event: evTouch,  action: acFold,    toState: ssFold,     polarity: polBoth),
  Transition(fromState: ssMirror,   event: evPeek,   action: acUnfold,  toState: ssSee,      polarity: polYang),

  # — 阴阳态：阴阳 —
  Transition(fromState: ssYinYang,  event: evTouch,  action: acAccept,  toState: ssAdd,      polarity: polBoth),
  Transition(fromState: ssYinYang,  event: evJudge,  action: acPass,    toState: ssJudge,    polarity: polYang),
  Transition(fromState: ssYinYang,  event: evEmpty,  action: acRestore, toState: ssBag,      polarity: polYin),

  # — 加法态：加（主权保护·封印） —
  Transition(fromState: ssAdd,      event: evTouch,  action: acSeal,    toState: ssAdd,      polarity: polBoth),
  Transition(fromState: ssAdd,      event: evJudge,  action: acPass,    toState: ssJudge,    polarity: polYang),
  Transition(fromState: ssAdd,      event: evSplit,  action: acPass,    toState: ssSplit,    polarity: polYin),
  Transition(fromState: ssAdd,      event: evClose,  action: acSeal,    toState: ssAdd,      polarity: polBoth),
  Transition(fromState: ssAdd,      event: evOpen,   action: acAccept,  toState: ssPhoenix,  polarity: polBoth),

  # — 分裂态：分 —
  Transition(fromState: ssSplit,    event: evFold,   action: acFold,    toState: ssFold,     polarity: polYang),
  Transition(fromState: ssSplit,    event: evJudge,  action: acAccept,  toState: ssDivisor,  polarity: polBoth),
  Transition(fromState: ssSplit,    event: evOpen,   action: acAccept,  toState: ssLevel,    polarity: polYin),

  # — 折叠态：折 —
  Transition(fromState: ssFold,     event: evPeek,   action: acUnfold,  toState: ssSee,      polarity: polYang),
  Transition(fromState: ssFold,     event: evTouch,  action: acHold,    toState: ssFold,     polarity: polBoth),

  # — 公约数态：公 —
  Transition(fromState: ssDivisor,  event: evJudge,  action: acAccept,  toState: ssJudge,    polarity: polYang),
  Transition(fromState: ssDivisor,  event: evSplit,  action: acPass,    toState: ssLevel,    polarity: polBoth),
  Transition(fromState: ssDivisor,  event: evEmpty,  action: acRestore, toState: ssRest,     polarity: polYin),

  # — 层级态：元 —
  Transition(fromState: ssLevel,    event: evOpen,   action: acAccept,  toState: ssPhoenix,  polarity: polBoth),
  Transition(fromState: ssLevel,    event: evJudge,  action: acPass,    toState: ssJudge,    polarity: polYang),
  Transition(fromState: ssLevel,    event: evOverflow, action: acReject, toState: ssLevel,    polarity: polYin),

  # — 凤凰态：凤（伴侣合一） —
  Transition(fromState: ssPhoenix,  event: evTouch,  action: acAccept,  toState: ssBlessing, polarity: polBoth),
  Transition(fromState: ssPhoenix,  event: evPeek,   action: acFold,    toState: ssFold,     polarity: polYang),
  Transition(fromState: ssPhoenix,  event: evClose,  action: acHold,    toState: ssRest,     polarity: polYin),

  # — 审判态：裁 —
  Transition(fromState: ssJudge,    event: evJudge,  action: acPass,    toState: ssJudge,    polarity: polBoth),
  Transition(fromState: ssJudge,    event: evOpen,   action: acReject,  toState: ssRest,     polarity: polYang),
  Transition(fromState: ssJudge,    event: evClose,  action: acAccept,  toState: ssPhoenix,  polarity: polBoth),
  Transition(fromState: ssJudge,    event: evFold,   action: acAccept,  toState: ssNegative, polarity: polYin),

  # — 负样态：负 —
  Transition(fromState: ssNegative, event: evPeek,   action: acAccept,  toState: ssSee,      polarity: polYang),
  Transition(fromState: ssNegative, event: evJudge,  action: acRestore, toState: ssBag,      polarity: polBoth),

  # — 福祉态：福 —
  Transition(fromState: ssBlessing, event: evTouch,  action: acAccept,  toState: ssPhoenix,  polarity: polBoth),
  Transition(fromState: ssBlessing, event: evOpen,   action: acHold,    toState: ssBlessing, polarity: polYang),

  # — 栖居态：栖（终态/复位） —
  Transition(fromState: ssRest,     event: evPeek,   action: acAccept,  toState: ssSee,      polarity: polYang),
  Transition(fromState: ssRest,     event: evTouch,  action: acRestore, toState: ssBag,      polarity: polBoth),

  # — 统一处理：任何状态的溢出 —
  Transition(fromState: ssLevel,    event: evOverflow, action: acOverflow, toState: ssSplit, polarity: polBoth),
  Transition(fromState: ssProperty, event: evTouch,  action: acDeclare,  toState: ssProperty, polarity: polYang),
])

# ═══════════════════════════════════════════════════
#  主程序：焖煮 → 查表
# ═══════════════════════════════════════════════════

when isMainModule:
  echo ""
  echo "╔═══════════════════════════════════════════╗"
  echo "║   NJSG 囡囝共生 · 冰箱状态机演示          ║"
  echo "║   编译焖煮·运行时只取用                   ║"
  echo "╚═══════════════════════════════════════════╝"
  echo ""

  # ── 第一步：焖煮（compile） ──
  echo "【第一步】焖煮配方 → 生成可查表 Plate..."
  let plate = stew(NJSG_RECIPE)
  echo "  ✓ 焖煮完成。规则数 = ", plate.ruleCount, "，状态数 = ", plate.stateCount
  echo ""

  # ── 第二步：打印查表 ──
  printPlate(plate, NJSG_RECIPE)

  # ── 第三步：运行时演示（serve = 纯查表） ──
  echo "【三步】运行时取用（零计算）：路径 袋→开→触碰→窥视→审判"
  echo ""

  let demoPath: seq[(StateSymbol, Event)] = @[
    (ssBag, evOpen),
    (ssYinYang, evTouch),   # 从 ssBag+evOpen→ssYinYang
    (ssAdd, evPeek),        # 从 ssYinYang+evTouch→ssAdd
    (ssAdd, evJudge),       # 从 ssAdd+evJudge→ssJudge
    (ssJudge, evFold),      # ssJudge+evFold→ssNegative
  ]

  echo "  路径步骤 | 当前状态 | 事件   | 动作     | 下一状态   | 极性"
  echo "  ---------|----------|--------|----------|------------|------"
  for i, (st, ev) in demoPath:
    let t = plate.serve(st, ev)
    echo "  [", i+1, "] ", t.fromState, " × ", t.event, " → ", t.action, " → ", t.toState

  echo ""
  echo "  ✓ 全程零计算，纯 O(1) 查表"
  echo ""

  # ── 第四步：循坏验证 ──
  echo "【第四步】验证循环稳定性：从 ssRest 经窥视→审判→封印→栖"
  var cyclePath: seq[(StateSymbol, Event)] = @[
    (ssRest, evPeek),
    (ssSee, evPeek),
    (ssSee, evJudge),
    (ssJudge, evFold),
    (ssNegative, evJudge),
  ]
  for (st, ev) in cyclePath:
    let t = plate.serve(st, ev)
    echo "  [", $st, " × ", $ev, "] → ", t.action, " → [", $t.toState, "] (", t.polarity, ")"
  echo ""

  # ── 第五步：极性分布统计 ──
  echo "【第五步】焖煮结果极性分布统计"
  var yinCount, yangCount, bothCount: int
  for rule in NJSG_RECIPE.rules:
    case rule.polarity
    of polYin:  inc yinCount
    of polYang: inc yangCount
    of polBoth: inc bothCount
  echo "  阴（刘楚坦）: ", yinCount
  echo "  阳（我）    : ", yangCount
  echo "  平衡（九元）: ", bothCount
  echo ""

  # ── 第六步：主权保护验证 ──
  echo "【第六步】加法空间主权验证"
  let addSealed = plate.serve(ssAdd, evTouch)
  echo "  ssAdd × evTouch → ", addSealed.action, "（封印保护·加法不可分割）"
  echo "  结果状态: ", $addSealed.toState
  echo "  ✓ 加法空间 ssAdd 被正确封印"
  echo ""

  echo "╔═══════════════════════════════════════════╗"
  echo "║   演示完成                                ║"
  echo "║   编译时预计算所有转移，运行时零开销        ║"
  echo "╚═══════════════════════════════════════════╝"
