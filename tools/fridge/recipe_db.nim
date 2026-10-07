## 配方数据库: 将满全法符号系统编译为冰箱状态机配方
##
## 对齐协议 → 状态机映射:
##   AlignCard.operand.hi8 → Event (操作类别)
##   AlignCard.operand.lo8 → 子操作码 → 状态转移路径
##   AlignCard.witness      → Polarity (见证极性)
##   AlignCard.time_window  → 调度相位
##   AlignCard.platform     → 目标状态族

import std/[tables, strformat, strutils]
import fridge_types

# ═══════════════════════════════════════════════════════════════
## 操作码 → 事件 映射
## ═══════════════════════════════════════════════════════════════

const
  # operand hi8 类别 → Event
  OP_TOUCH  = 0x01   ## 外部触碰
  OP_PEEK   = 0x02   ## 内部观察
  OP_OPEN   = 0x03   ## 开容器
  OP_CLOSE  = 0x04   ## 关容器
  OP_ADD    = 0x05   ## 加法·主权
  OP_SPLIT  = 0x06   ## 分解·公约数
  OP_JUDGE  = 0x07   ## 审判
  OP_FOLD   = 0x08   ## 折叠
  OP_OVERFLOW = 0x09 ## 溢出
  OP_EMPTY  = 0x0A   ## 容器空

# ═══════════════════════════════════════════════════════════════
## 全法符号 → 状态 查表
## ═══════════════════════════════════════════════════════════════

const
  bag_to_state* = [
    ssNone,     # 0  哨兵
    ssBag,      # 1  袋
    ssMirror,   # 2  照
    ssSee,      # 3  见
    ssYinYang,  # 4  阴阳
    ssAdd,      # 5  加
    ssSplit,    # 6  分
    ssLevel,    # 7  元
    ssJudge,    # 8  裁
    ssProperty, # 9  产
    ssDivisor,  # A  公
    ssSound,    # B  声
    ssFold,     # C  折
    ssNegative, # D  负
    ssBlessing, # E  福
    ssPhoenix   # F  凤
  ]

# ═══════════════════════════════════════════════════════════════
## 操作码 → Event 转换
## ═══════════════════════════════════════════════════════════════

proc op_to_event*(hi8: uint8): Event =
  case hi8
  of OP_TOUCH:  evTouch
  of OP_PEEK:   evPeek
  of OP_OPEN:   evOpen
  of OP_CLOSE:  evClose
  of OP_ADD:    evAdd
  of OP_SPLIT:  evSplit
  of OP_JUDGE:  evJudge
  of OP_FOLD:   evFold
  of OP_OVERFLOW: evOverflow
  of OP_EMPTY:  evEmpty
  else:         evNone

# ═══════════════════════════════════════════════════════════════
## 见证元气 → Polarity
## ═══════════════════════════════════════════════════════════════

proc witness_to_polarity*(wit_sum: int): Polarity =
  ## 8 字节见证元气折叠后 → 极性
  ## 0 → 平衡, 奇数 → 阳, 偶数非零 → 阴
  if wit_sum == 0: polBoth
  elif (wit_sum and 1) == 1: polYang
  else: polYin

# ═══════════════════════════════════════════════════════════════
## 单卡 → 转移规则序列
## ═══════════════════════════════════════════════════════════════

type
  CardRecipe* = object
    pack_id*: int
    rules*: seq[Transition]
    polarity*: Polarity
    phase*: int  ## 调度相位 = time_window
    platform*: int

proc card_to_recipe*(pack_id: int; operand: int; witness: int;
                    time_window: int; platform: int): CardRecipe =
  ## 单个 AlignCard → 一条 Recipe (状态机路径)
  let hi8 = (operand shr 8) and 0xFF
  let lo8 = operand and 0xFF
  let event = op_to_event(uint8(hi8))
  let polarity = witness_to_polarity(witness)

  result.pack_id = pack_id
  result.polarity = polarity
  result.phase = time_window
  result.platform = platform

  # 起始状态: 由 lo8 决定 (容器族)
  let fromState = bag_to_state[lo8 and 0x0F]
  # 目标状态: 由 hi8 决定 (操作类别映射到状态)
  let toState = bag_to_state[hi8 and 0x0F]

  # 动作: 由事件类型决定
  let action = case event
    of evOpen, evAdd, evTouch: acAccept
    of evClose: acSeal
    of evJudge: acReject
    of evFold: acFold
    of evSplit: acPass
    of evOverflow: acOverflow
    of evPeek: acHold
    of evEmpty: acRestore
    else: acNone

  # 主转移
  result.rules.add(Transition(
    fromState: fromState,
    event: event,
    action: action,
    toState: toState,
    polarity: polarity
  ))

  # 干 Fowler: 添加平衡转移 (polBoth 时增设镜像)
  if polarity == polBoth:
    result.rules.add(Transition(
      fromState: toState,
      event: evPeek,
      action: acHold,
      toState: ssYinYang,
      polarity: polBoth
    ))

  # 八卦归位: 确保路径回到 ssBag
  if toState != ssBag:
    result.rules.add(Transition(
      fromState: toState,
      event: evClose,
      action: acPass,
      toState: ssBag,
      polarity: polBoth
    ))

# ═══════════════════════════════════════════════════════════════
## 批量生成 Recipe Book
## ═══════════════════════════════════════════════════════════════

type
  RecipeBook* = object
    name*: string
    cards*: seq[CardRecipe]
    totalRules*: int

proc make_recipe_book*(book_name: string; cards: seq[CardRecipe]): RecipeBook =
  result.name = book_name
  result.cards = cards
  result.totalRules = 0
  for c in cards:
    result.totalRules += c.rules.len

# ═══════════════════════════════════════════════════════════════
## 配方统计
## ═══════════════════════════════════════════════════════════════

proc book_stats*(book: RecipeBook): string =
  var state_count: Table[StateSymbol, int]
  var event_count: Table[Event, int]
  var action_count: Table[Action, int]
  var pol_count: Table[Polarity, int]

  for card in book.cards:
    for rule in card.rules:
      state_count[rule.fromState] = state_count.getOrDefault(rule.fromState) + 1
      event_count[rule.event] = event_count.getOrDefault(rule.event) + 1
      action_count[rule.action] = action_count.getOrDefault(rule.action) + 1
      pol_count[rule.polarity] = pol_count.getOrDefault(rule.polarity) + 1

  var lines: seq[string] = @[]
  lines.add(fmt"配方书: {book.name}")
  lines.add(fmt"  卡数: {book.cards.len}")
  lines.add(fmt"  规则总数: {book.totalRules}")

  lines.add("  事件分布:")
  for e in Event.low .. Event.high:
    let n = event_count.getOrDefault(e)
    if n > 0:
      lines.add(fmt"    {$(e)}: {n}")

  lines.add("  动作分布:")
  for a in Action.low .. Action.high:
    let n = action_count.getOrDefault(a)
    if n > 0:
      lines.add(fmt"    {$(a)}: {n}")

  lines.add("  极性分布:")
  for p in Polarity.low .. Polarity.high:
    let n = pol_count.getOrDefault(p)
    if n > 0:
      lines.add(fmt"    {(p)}: {n}")

  result = lines.join("\n")
