# ============================================================
# 满全法 · 审判引擎
# ============================================================
# 审判不看米粒，看袋子。
# 审判对 → 奖励（功德+）
# 审判错 → 惩罚（业报-）
# 审判的核心：这袋子该不该留？该不该洗？该不该扔？
# ============================================================

import math
import strutils
import ./rongqi
import ./heavenly_court

type
  # --- 判决动作 ---
  PanAction* = enum
    paKeep,      # 保留
    paWash,      # 清洗
    paDry,       # 晾晒
    paRepair,    # 修理
    paDiscard,   # 丢弃
    paSell,      # 贩卖

  # --- 判决记录（每次审判一条）---
  ShenPanRecord* = object
    tick*: int              # 何时审判
    bag_id*: string         # 审的是哪袋
    action*: PanAction      # 判的是什么
    correct*: bool          # 审对了吗
    merit*: float           # 功德（正=奖，负=罚）
    reason*: string         # 为什么这样判
    truth*: PanAction       # 实际应该怎样（事后对照）

  # --- 审判引擎 ---
  ShenPanEngine* = object
    name*: string
    records*: seq[ShenPanRecord]   # 历史审判记录
    total_merit*: float            # 总功德
    total_demerit*: float          # 总业报（惩罚累计）
    correct_count*: int            # 审对次数
    wrong_count*: int              # 审错次数
    judge_name*: string            # 谁是审判者（天庭之名）
    strictness*: float             # 严格度（1.0=标准，越高越严）
    learning_rate*: float          # 学习速率（每次审判学多少）

  # --- 判决结果（包含奖惩）---
  PanResult* = object
    record*: ShenPanRecord
    verdict_desc*: string
    reward_desc*: string


# ============================================================
# 构造
# ============================================================
func new_shenpan*(name, judge: string): ShenPanEngine =
  result.name = name
  result.records = @[]
  result.total_merit = 0.0
  result.total_demerit = 0.0
  result.correct_count = 0
  result.wrong_count = 0
  result.judge_name = judge
  result.strictness = 1.0
  result.learning_rate = 0.05


# ============================================================
# 核心：判断一袋子的真实命运（该怎么做）
# ============================================================
func truth_action*(bag: RongQi): PanAction =
  ## 袋子的真实需要（不是天庭的判断，是事实）
  if bag.should_discard:
    return paDiscard
  if bag.needs_washing:
    return paWash
  if bag.needs_drying:
    return paDry
  if bag.state == rqsDamaged:
    return paRepair
  if bag.value_coin > 60 and bag.quality.rice_ok:
    return paSell   # 值钱的好袋可以卖掉
  return paKeep


func truth_desc*(a: PanAction): string =
  case a
  of paKeep: return "留"
  of paWash: return "洗"
  of paDry: return "晒"
  of paRepair: return "修"
  of paDiscard: return "扔"
  of paSell: return "卖"


# ============================================================
# 审判：天庭的判决 + 对错对照 + 奖惩
# ============================================================
func shenpan*(engine: var ShenPanEngine; bag: RongQi; heavenly_court_verdict: HeavenlyCourtVerdict; tick: int): PanResult =
  ## 把天庭的判断 和 真实命运 对照，给奖惩
  let truth = bag.truth_action()
  var action = paKeep

  # 把天庭的动作为 PanAction
  case heavenly_court_verdict.action
  of "保留": action = paKeep
  of "清洗": action = paWash
  of "晾晒": action = paDry
  of "修理": action = paRepair
  of "丢弃": action = paDiscard
  of "贩卖": action = paSell
  else: action = paKeep

  # 判断对错
  let correct = (action == truth)

  # 计算奖惩
  var merit = 0.0
  if correct:
    merit = 10.0 * engine.strictness
    engine.correct_count += 1
  else:
    merit = -15.0 * engine.strictness   # 惩罚比奖励重，因为是造业
    engine.wrong_count += 1

  engine.total_merit += (if merit > 0: merit else: 0)
  engine.total_demerit += (if merit < 0: abs(merit) else: 0)

  # 记录
  var rec = ShenPanRecord(
    tick: tick,
    bag_id: bag.id,
    action: action,
    correct: correct,
    merit: merit,
    reason: heavenly_court_verdict.reason,
    truth: truth
  )
  engine.records.add(rec)

  # 结果描述
  let verdict_str = "判" & truth_desc(action) & "，理应" & truth_desc(truth)
  var reward_str = ""
  if correct:
    reward_str = "✓ 对 → 功德+" & formatFloat(merit, ffDecimal, 1)
  else:
    reward_str = "✗ 错 → 业报" & formatFloat(merit, ffDecimal, 1)

  return PanResult(record: rec, verdict_desc: verdict_str, reward_desc: reward_str)


# ============================================================
# 精确率（审判有多准）
# ============================================================
func accuracy*(engine: ShenPanEngine): float =
  let total = engine.correct_count + engine.wrong_count
  if total == 0: return 0.0
  return engine.correct_count.float / total.float


# ============================================================
# 审判者等级
# ============================================================
func judge_level*(engine: ShenPanEngine): string =
  let acc = engine.accuracy * 100
  if acc >= 95: return "如镜"
  elif acc >= 85: return "明镜"
  elif acc >= 70: return "铜镜"
  elif acc >= 50: return "蒙尘"
  else: return "瞎判"


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 审判测试 ==="

  var sp = new_shenpan("满全法审判", "囡囝之眼")
  echo "审判者:", sp.judge_name, " 严格度:", sp.strictness
  echo ""

  # 袋子们
  var good = new_rongqi("g1", "好袋", "张三")
  good.value_coin = 50.0; good.state = rqsStored
  good.add_content(ContentItem(name: "米", kind: "粮食", quantity: 50.0, is_edible: true))

  var bad = new_rongqi("b1", "坏袋", "李四")
  bad.value_coin = 10.0; bad.state = rqsDirty
  bad.quality.rice_ok = false; bad.quality.has_insect = true
  bad.quality.discard_reason = "虫蛀"

  var wash = new_rongqi("w1", "要洗袋", "王五")
  wash.value_coin = 40.0; wash.state = rqsDirty
  wash.quality.has_sand = true

  var bags = @[good, bad, wash]
  var tj = new_heavenly_court("囡囝之眼")

  echo "【逐袋审判】"
  for i, bag in bags:
    let v = tj.verdict(bag)
    let r = sp.shenpan(bag, v, i)
    echo "  ", bag.name, ":", r.verdict_desc, " → ", r.reward_desc

  echo ""
  echo "【审判总览】"
  echo "  总功德:", formatFloat(sp.total_merit, ffDecimal, 1)
  echo "  总业报:", formatFloat(sp.total_demerit, ffDecimal, 1)
  echo "  准确率:", formatFloat(sp.accuracy * 100, ffDecimal, 1), "%"
  echo "  审判者等级:", sp.judge_level
