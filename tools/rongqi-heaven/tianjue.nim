# ============================================================
# 满全法 · 天觉感知系统
# ============================================================
# 天觉不看米粒，看袋子。
# 天觉看的是：这袋是谁的，值多少钱，脏不脏，坏没坏，要不要洗。
# 天觉看的是关系：袋子和袋子的关系，袋子和主人的关系，袋子和时间的关系。
# 天觉看的是层级：袋子包袋子，大袋套小袋，袋子成堆，堆成垛，垛成仓。
# ============================================================

import math
import strutils
import ./rongqi

type
  # --- 感知维度 ---
  TianJueDim* = enum
    tjdOwner,      # 主人
    tjdValue,      # 价值
    tjdState,      # 状态
    tjdQuality,    # 品质
    tjdRelation,   # 关系
    tjdHistory,    # 历史
    tjdNecessity,  # 必要性

  # --- 感知结果（天觉的观察）---
  TianJuePercept* = object
    target_id*: string          # 观察对象
    dim*: TianJueDim            # 哪个维度
    score*: float               # 评分 0.0 - 1.0
    label*: string              # 标签（"干净" / "破损" / "值钱"...）
    confidence*: float          # 确信度 0.0 - 1.0
    detail*: string             # 细节描述

  # --- 天觉主体（观察者）---
  TianJue* = object
    name*: string               # 天觉叫什么名字（"囡囝之眼"/"满全法眼"...）
    dimensions*: set[TianJueDim] # 开了哪些感知维度
    percepts*: seq[TianJuePercept]  # 当前看到的所有东西
    wisdom*: float              # 智慧（积累的感知经验）
    sight_range*: int           # 能一次看多少袋子

  # --- 感知判断（简单明了）---
  TianJueVerdict* = object
    bag_id*: string
    keep*: bool          # 留还是不留
    action*: string      # "保留" / "清洗" / "丢弃" / "修理" / "贩卖"
    worth*: float        # 当前价值评估
    reason*: string      # 为什么这样做


# ============================================================
# 构造
# ============================================================
func new_tianjue*(name: string): TianJue =
  result.name = name
  result.dimensions = {tjdOwner, tjdValue, tjdState, tjdQuality, tjdRelation, tjdHistory, tjdNecessity}
  result.percepts = @[]
  result.wisdom = 0.0
  result.sight_range = 100

func perceive_bag*(tj: var TianJue; bag: RongQi): seq[TianJuePercept] =
  ## 天觉看一袋子 — 不做数粒子的低级事，只看袋子级别
  var results: seq[TianJuePercept]

  # --- 主人维度 ---
  if tjdOwner in tj.dimensions:
    let owner_score = if bag.owner.len > 0: 1.0 else: 0.5
    results.add(TianJuePercept(
      target_id: bag.id,
      dim: tjdOwner,
      score: owner_score,
      label: if owner_score > 0.8: "有主" else: "失主",
      confidence: 0.9,
      detail: "主人是" & bag.owner
    ))

  # --- 价值维度 ---
  if tjdValue in tj.dimensions:
    let val_score = min(bag.value_coin / 100.0, 1.0)
    let vlabel = if bag.value_coin > 70: "值钱" elif bag.value_coin > 30: "中常" elif bag.value_coin > 0: "不值钱" else: "无价值"
    results.add(TianJuePercept(
      target_id: bag.id,
      dim: tjdValue,
      score: val_score,
      label: vlabel,
      confidence: 0.85,
      detail: "值 " & formatFloat(bag.value_coin, ffDecimal, 1) & " 钱"
    ))

  # --- 状态维度 ---
  if tjdState in tj.dimensions:
    let state_score = case bag.state
      of rqsNew, rqsClean, rqsStored: 1.0
      of rqsUsed, rqsDrying, rqsWashing: 0.7
      of rqsDirty: 0.3
      of rqsDamaged: 0.1
      of rqsDiscarded, rqsRecycled: 0.0
    results.add(TianJuePercept(
      target_id: bag.id,
      dim: tjdState,
      score: state_score,
      label: $bag.state,
      confidence: 0.9,
      detail: "袋子" & $bag.state
    ))

  # --- 品质维度 ---
  if tjdQuality in tj.dimensions:
    var q_score = 1.0
    if bag.quality.has_sand: q_score -= 0.3
    if bag.quality.has_insect: q_score -= 0.5
    if not bag.quality.rice_ok: q_score -= 0.8
    if bag.quality.needs_washing: q_score -= 0.2
    let qlabel = if q_score > 0.7: "好袋" elif q_score > 0.4: "能用" else: "坏袋"
    results.add(TianJuePercept(
      target_id: bag.id,
      dim: tjdQuality,
      score: max(q_score, 0.0),
      label: qlabel,
      confidence: 0.88,
      detail: "品质" & qlabel & (if bag.quality.has_insect: " [虫蛀]" else: "") & (if bag.quality.has_sand: " [含沙]" else: "")
    ))

  # --- 必要性维度（天觉最核心的判断：这袋子还该不该留）---
  if tjdNecessity in tj.dimensions:
    let n_score = if not bag.should_discard: 1.0 else: 0.0
    let nlabel = if n_score > 0.5: "该留" else: "该扔"
    results.add(TianJuePercept(
      target_id: bag.id,
      dim: tjdNecessity,
      score: n_score,
      label: nlabel,
      confidence: 0.92,
      detail: nlabel & (if bag.should_discard: "，因为" & bag.quality.discard_reason else: "")
    ))

  tj.percepts.add(results)
  return results


func verdict*(tj: TianJue; bag: RongQi): TianJueVerdict =
  ## 天觉的最后裁决：对一袋子的最终判定
  var keep = true
  var action = "保留"
  var worth = bag.value_coin

  if bag.should_discard:
    keep = false
    action = "丢弃"
    worth = 0.0
  elif bag.needs_washing:
    action = "清洗"
    worth = bag.value_coin * 0.9   # 洗一次掉价一成
  elif bag.needs_drying:
    action = "晾晒"
    worth = bag.value_coin * 0.95
  elif bag.state == rqsDamaged:
    action = "修理"
    worth = bag.value_coin * 0.7

  var reason = ""
  if not keep:
    reason = "该袋已" & bag.quality.discard_reason
  elif action != "保留":
    reason = "需要" & action
  else:
    reason = "状态良好，继续存储"

  return TianJueVerdict(
    bag_id: bag.id,
    keep: keep,
    action: action,
    worth: worth,
    reason: reason
  )


func perceive_batch*(tj: var TianJue; bags: seq[RongQi]): seq[TianJueVerdict] =
  ## 天觉一次看一批袋子（不数粒，只看袋）
  result = @[]
  for bag in bags:
    # 感知
    discard tj.perceive_bag(bag)
    # 裁决
    result.add(tj.verdict(bag))


func wisdom_level*(tj: TianJue): string =
  if tj.wisdom > 100: return "通天"
  elif tj.wisdom > 50: return "洞察"
  elif tj.wisdom > 20: return "明辨"
  elif tj.wisdom > 5: return "初觉"
  else: return "蒙昧"


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 天觉测试 ==="

  var tj = new_tianjue("囡囝之眼")
  echo "天觉:", tj.name, " 境界:", tj.wisdom_level

  # 好袋子
  var good = new_rongqi("g1", "张家的好米", "张三")
  good.value_coin = 80.0
  good.state = rqsStored
  good.add_content(ContentItem(name: "米", kind: "粮食", quantity: 50.0, is_edible: true))

  # 坏袋子
  var bad = new_rongqi("b1", "虫蛀米袋", "王五")
  bad.value_coin = 15.0
  bad.state = rqsDirty
  bad.quality.has_insect = true
  bad.quality.rice_ok = false
  bad.quality.discard_reason = "虫蛀严重"
  bad.add_content(ContentItem(name: "虫米", kind: "粮食", quantity: 20.0, is_edible: false))
  bad.add_content(ContentItem(name: "米虫", kind: "生物", quantity: 5.0, is_edible: false))

  # 要洗的袋子
  var wash = new_rongqi("w1", "沾沙米袋", "赵六")
  wash.value_coin = 45.0
  wash.state = rqsDirty
  wash.quality.has_sand = true
  wash.add_content(ContentItem(name: "米", kind: "粮食", quantity: 40.0, is_edible: true))
  wash.add_content(ContentItem(name: "沙子", kind: "杂质", quantity: 2.0, is_edible: false))

  let bags = @[good, bad, wash]
  let verdicts = tj.perceive_batch(bags)

  echo ""
  echo "【天觉裁决】"
  for v in verdicts:
    echo "  ", v.bag_id, " → ", v.action, " (", formatFloat(v.worth, ffDecimal, 1), "钱) ", v.reason

  echo ""
  echo "天觉智慧:", tj.wisdom, " 境界:", tj.wisdom_level
