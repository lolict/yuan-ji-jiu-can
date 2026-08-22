# ============================================================
# 满全法 · 历史记忆系统
# ============================================================
# 历史不是数米粒，历史是袋子的来龙去脉。
# 历史记住的是：这袋今年怎么样，去年怎么样，主人换了没有，
#              有没有生过虫，有没有洗过，有没有坏掉。
# 历史的积累：看得越多袋子，天觉越聪明，审判越准确。
# ============================================================

import tables
import math
import strutils
import algorithm
import ./rongqi

type
  # --- 袋子快照（某一刻袋子的状态）---
  BagSnapshot* = object
    tick*: int
    bag_id*: string
    name*: string
    owner*: string
    state*: RongQiState
    value_coin*: float
    rice_amount*: float       # 米还有多少（不数粒）
    has_sand*: bool
    has_insect*: bool
    action_taken*: string     # 当时做了什么
    was_correct*: bool        # 做对没有

  # --- 袋子传记（一袋子的完整历史）---
  BagBiography* = object
    bag_id*: string
    snapshots*: seq[BagSnapshot]
    born_tick*: int
    death_tick*: int          # -1表示还在
    total_washes*: int        # 洗过几次
    total_repairs*: int       # 修过几次
    survived_ticks*: int      # 存活多久

  # --- 历史规律（从多次事件中总结）---
  LiShiPattern* = object
    pattern_id*: string
    condition*: string        # 什么情况
    action*: string           # 做了什么
    success_count*: int       # 成功几次
    failure_count*: int       # 失败几次
    wisdom*: float            # 这个规律的可靠性

  # --- 历史记忆系统 ---
  LiShiJiYi* = object
    name*: string
    bios*: Table[string, BagBiography]     # 袋子传记
    patterns*: seq[LiShiPattern]           # 总结的规律
    current_tick*: int
    total_bags_ever*: int                  # 一共见过多少袋
    total_bags_alive*: int                 # 现在还有多少袋活着
    experience*: float                     # 历史积累的经验

  # --- 统计摘要 ---
  LiShiSummary* = object
    total_events*: int
    correct_rate*: float
    most_common_action*: string
    most_dangerous_state*: string
    avg_bag_lifespan*: float


# ============================================================
# 构造
# ============================================================
func new_lishi*(name: string): LiShiJiYi =
  result.name = name
  result.bios = initTable[string, BagBiography]()
  result.patterns = @[]
  result.current_tick = 0
  result.total_bags_ever = 0
  result.total_bags_alive = 0
  result.experience = 0.0


# ============================================================
# 记录袋子快照
# ============================================================
func record*(ls: var LiShiJiYi; bag: RongQi; action: string; correct: bool) =
  ## 历史记住这一袋子的这一刻
  let snap = BagSnapshot(
    tick: ls.current_tick,
    bag_id: bag.id,
    name: bag.name,
    owner: bag.owner,
    state: bag.state,
    value_coin: bag.value_coin,
    rice_amount: bag.total_rice,
    has_sand: bag.has_sand,
    has_insect: bag.has_insect,
    action_taken: action,
    was_correct: correct
  )

  # 如果有传记就加进去，没有就新建
  if bag.id in ls.bios:
    ls.bios[bag.id].snapshots.add(snap)
  else:
    var bio = BagBiography(
      bag_id: bag.id,
      snapshots: @[snap],
      born_tick: ls.current_tick,
      death_tick: -1,
      total_washes: 0,
      total_repairs: 0,
      survived_ticks: 0
    )
    ls.bios[bag.id] = bio
    ls.total_bags_ever += 1
    ls.total_bags_alive += 1

  # 更新统计
  if action == "清洗":
    ls.bios[bag.id].total_washes += 1
  if action == "修理":
    ls.bios[bag.id].total_repairs += 1

  # 如果袋子被丢弃/卖掉，标记死亡
  if action in ["丢弃", "贩卖"]:
    ls.bios[bag.id].death_tick = ls.current_tick
    ls.total_bags_alive -= 1

  # 经验增长
  ls.experience += (if correct: 1.0 else: -0.5)


# ============================================================
# 时间推进
# ============================================================
func advance_tick*(ls: var LiShiJiYi) =
  ls.current_tick += 1

  # 更新所有存活袋子的存活时间
  for id, bio in ls.bios:
    if bio.death_tick < 0:
      ls.bios[id].survived_ticks = ls.current_tick - bio.born_tick


# ============================================================
# 规律发现（从历史中学习）
# ============================================================
func learn_patterns*(ls: var LiShiJiYi) =
  ## 从审判记录中找规律：什么情况下怎么做是对的
  var pat_count = initTable[string, tuple[s, f: int]]()

  for id, bio in ls.bios:
    for snap in bio.snapshots:
      let key = $snap.state & "|" & snap.action_taken
      if key in pat_count:
        if snap.was_correct:
          pat_count[key].s += 1
        else:
          pat_count[key].f += 1
      else:
        pat_count[key] = (if snap.was_correct: (1, 0) else: (0, 1))

  ls.patterns = @[]
  for key, sf in pat_count:
    let parts = key.split('|')
    let total = sf.s + sf.f
    let wisdom = (sf.s.float / total.float) * (1.0 - exp(-total.float / 3.0))  # 次数越多越可信
    ls.patterns.add(LiShiPattern(
      pattern_id: "pat_" & $(ls.patterns.len),
      condition: parts[0],
      action: parts[1],
      success_count: sf.s,
      failure_count: sf.f,
      wisdom: wisdom
    ))

  # 按wisdom降序排序(大在前)
  ls.patterns.sort(proc (a, b: LiShiPattern): int = cmp(a.wisdom, b.wisdom), Descending)


# ============================================================
# 查找规律（给定状态，查该怎么做）
# ============================================================
func find_best_action*(ls: LiShiJiYi; state: RongQiState): string =
  ## 从历史经验中找：这个状态下最该做什么
  var best_act = ""
  var best_w = -1.0
  for p in ls.patterns:
    if p.condition == $state and p.wisdom > best_w:
      best_w = p.wisdom
      best_act = p.action
  return best_act


# ============================================================
# 摘要统计
# ============================================================
func summarize*(ls: LiShiJiYi): LiShiSummary =
  var total_events = 0
  var correct_events = 0
  var act_count = initTable[string, int]()
  var state_count = initTable[string, int]()
  var total_lifespan = 0

  for id, bio in ls.bios:
    for snap in bio.snapshots:
      total_events += 1
      if snap.was_correct: correct_events += 1
      act_count[snap.action_taken] = act_count.getOrDefault(snap.action_taken) + 1
      state_count[$snap.state] = state_count.getOrDefault($snap.state) + 1
    if bio.death_tick > 0:
      total_lifespan += bio.death_tick - bio.born_tick
    else:
      total_lifespan += ls.current_tick - bio.born_tick

  var most_common = ""
  var mc = 0
  for a, c in act_count:
    if c > mc:
      mc = c
      most_common = a

  var most_dangerous = ""
  var md = 0
  for s, c in state_count:
    if c > md:
      md = c
      most_dangerous = s

  let bag_count = max(ls.bios.len, 1)
  return LiShiSummary(
    total_events: total_events,
    correct_rate: (if total_events > 0: correct_events.float / total_events.float else: 0.0),
    most_common_action: most_common,
    most_dangerous_state: most_dangerous,
    avg_bag_lifespan: total_lifespan.float / bag_count.float
  )


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 历史记忆测试 ==="

  var ls = new_lishi("满全法记忆")

  # 创建几个袋子，模拟多个时间点
  var b1 = new_rongqi("b1", "张家的袋", "张三")
  b1.value_coin = 50.0; b1.state = rqsStored
  b1.add_content(ContentItem(name: "米", kind: "粮食", quantity: 50.0, is_edible: true))

  var b2 = new_rongqi("b2", "李家的袋", "李四")
  b2.value_coin = 30.0; b2.state = rqsDirty
  b2.quality.has_sand = true

  var b3 = new_rongqi("b3", "王家的袋", "王五")
  b3.value_coin = 10.0; b3.state = rqsDirty
  b3.quality.has_insect = true; b3.quality.rice_ok = false

  # tick 0: 审判
  ls.record(b1, "保留", true)
  ls.record(b2, "清洗", true)
  ls.record(b3, "丢弃", true)
  ls.advance_tick()

  # tick 1: b2 洗完变干净
  b2.state = rqsClean
  b2.quality.has_sand = false
  ls.record(b2, "保留", true)
  ls.advance_tick()

  # tick 2: b1 生虫了
  b1.state = rqsDirty
  b1.quality.has_insect = true
  b1.quality.rice_ok = false
  ls.record(b1, "清洗", false)   # 审错了，应该直接扔
  ls.advance_tick()

  # tick 3: b1 还是被扔了
  ls.record(b1, "丢弃", true)
  ls.advance_tick()

  # 学习规律
  ls.learn_patterns()

  echo ""
  echo "【历史传记】"
  for id, bio in ls.bios:
    echo "  ", bio.bag_id, ": 存活", bio.survived_ticks, "tick, 洗", bio.total_washes, "次"

  echo ""
  echo "【历史规律】"
  for p in ls.patterns:
    echo "  ", p.condition, "→", p.action, " (可靠性:", formatFloat(p.wisdom * 100, ffDecimal, 1), "%)"

  echo ""
  let s = ls.summarize()
  echo "【历史摘要】"
  echo "  事件数:", s.total_events
  echo "  准确率:", formatFloat(s.correct_rate * 100, ffDecimal, 1), "%"
  echo "  最常做的:", s.most_common_action
  echo "  平均存活:", formatFloat(s.avg_bag_lifespan, ffDecimal, 1), "tick"

  echo ""
  echo "当前tick:", ls.current_tick, " 经验:", formatFloat(ls.experience, ffDecimal, 1)
  echo "已知袋子:", ls.total_bags_ever, " 存活:", ls.total_bags_alive
