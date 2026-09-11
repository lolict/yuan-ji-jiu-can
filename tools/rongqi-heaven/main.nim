# ============================================================
# 满全法 · 袋子系统 · 总入口
# ============================================================
# 不数米粒，只看袋子。
# 天庭看袋子，审判审袋子，历史记袋子，层级排袋子。
# ============================================================

import ./rongqi
import ./heavenly_court
import ./shenpan
import ./lishi
import ./cengji

import strutils
import tables


# ============================================================
# 系统总装
# ============================================================
type
  RongQiTianTi* = object
    registry*: RongQiRegistry   # 袋子注册
    heavenly_court*: HeavenlyCourt           # 天庭
    shenpan*: ShenPanEngine     # 审判
    lishi*: LiShiJiYi           # 历史
    hierarchy*: CengJiTree      # 层级
    system_name*: string


func new_system*(name: string): RongQiTianTi =
  result.registry = new_registry(name)
  result.heavenly_court = new_heavenly_court("囡囝之眼")
  result.shenpan = new_shenpan("满全法审判", "囡囝之眼")
  result.lishi = new_lishi("满全法记忆")
  result.hierarchy = new_cengji_tree(name & "·袋阵")
  result.system_name = name


# ============================================================
# 注册一袋子
# ============================================================
proc register_bag*(sys: var RongQiTianTi; bag: RongQi; level: CengJiLevel; parent_id: string = "") =
  sys.registry.bags[bag.id] = bag
  discard sys.hierarchy.add_to_tree(bag, level, parent_id)


# ============================================================
# 核心循环：感知 → 审判 → 记忆
# ============================================================
func process_one*(sys: var RongQiTianTi; bag: RongQi; tick: int): string =
  # 1. 天庭感知（不看米粒，只看袋）
  let percepts = sys.heavenly_court.perceive_bag(bag)
  let verdict = sys.heavenly_court.verdict(bag)

  # 2. 审判（对照真实命运，给奖惩）
  let pan = sys.shenpan.shenpan(bag, verdict, tick)

  # 3. 历史记忆（记住这一刻）
  sys.lishi.record(bag, verdict.action, pan.record.correct)

  # 4. 返回简洁描述
  var r = "[" & bag.name & "] "
  if pan.record.correct:
    r &= "✓ 判" & verdict.action & " → 功德+" & formatFloat(pan.record.merit, ffDecimal, 1)
  else:
    r &= "✗ 判" & verdict.action & "(应" & $pan.record.truth & ") → 业报" & formatFloat(pan.record.merit, ffDecimal, 1)
  return r


proc process_batch*(sys: var RongQiTianTi; bags: seq[RongQi]; tick: int) =
  for bag in bags:
    let r = sys.process_one(bag, tick)
    echo "  ", r
  # 学完一遍，历史总结规律
  sys.lishi.learn_patterns()
  sys.lishi.advance_tick()


# ============================================================
# 系统报告
# ============================================================
proc system_report*(sys: RongQiTianTi): string =
  var s = "═══ " & sys.system_name & " · 系统报告 ═══\n"
  s &= "\n【袋子注册】共 " & $sys.registry.bags.len & " 袋\n"

  s &= "\n【天庭感知】境界: " & sys.heavenly_court.wisdom_level &
       " 智慧: " & formatFloat(sys.heavenly_court.wisdom, ffDecimal, 1) & "\n"

  s &= "\n【审判】准确率: " & formatFloat(sys.shenpan.accuracy * 100, ffDecimal, 1) &
       "% 等级: " & sys.shenpan.judge_level & "\n"
  s &= "  功德: " & formatFloat(sys.shenpan.total_merit, ffDecimal, 1) &
       "  业报: " & formatFloat(sys.shenpan.total_demerit, ffDecimal, 1) &
       "  审判次数: " & $sys.shenpan.records.len & "\n"

  s &= "\n【历史记忆】经验: " & formatFloat(sys.lishi.experience, ffDecimal, 1) &
       "  已知袋: " & $sys.lishi.total_bags_ever &
       "  存活: " & $sys.lishi.total_bags_alive & "\n"

  s &= "\n" & sys.hierarchy.hierarchy_report()

  return s


# ============================================================
# 演示：张家的袋子系统
# ============================================================
when is_main_module:
  echo "══════════════════════════════════════"
  echo "   满全法 · 袋子天统"
  echo "   不数米粒，只看袋子"
  echo "══════════════════════════════════════"
  echo ""

  var sys = new_system("张家袋子天统")

  # --- 创建袋子 ---
  var bags: seq[RongQi] = @[]

  # 好袋：干净米袋
  var b1 = new_rongqi("b01", "甲袋好米", "张三")
  b1.value_coin = 50.0; b1.state = rqsStored
  b1.add_content(ContentItem(name: "米", kind: "粮食", quantity: 50.0, is_edible: true))
  bags.add(b1)

  # 含沙袋：要洗
  var b2 = new_rongqi("b02", "乙袋沙米", "张三")
  b2.value_coin = 30.0; b2.state = rqsDirty
  b2.quality.has_sand = true
  b2.add_content(ContentItem(name: "米", kind: "粮食", quantity: 40.0, is_edible: true))
  b2.add_content(ContentItem(name: "沙子", kind: "杂质", quantity: 3.0, is_edible: false))
  bags.add(b2)

  # 虫蛀袋：要扔
  var b3 = new_rongqi("b03", "丙袋虫米", "张三")
  b3.value_coin = 10.0; b3.state = rqsDirty
  b3.quality.rice_ok = false; b3.quality.has_insect = true
  b3.quality.discard_reason = "虫蛀发霉"
  b3.add_content(ContentItem(name: "虫蛀米", kind: "粮食", quantity: 20.0, is_edible: false))
  b3.add_content(ContentItem(name: "米虫", kind: "生物", quantity: 5.0, is_edible: false))
  bags.add(b3)

  # 新袋：刚买来
  var b4 = new_rongqi("b04", "丁袋新谷", "张三")
  b4.value_coin = 60.0; b4.state = rqsNew
  b4.add_content(ContentItem(name: "谷", kind: "粮食", quantity: 55.0, is_edible: true))
  bags.add(b4)

  # 破袋：要修
  var b5 = new_rongqi("b05", "戊袋破袋", "张三")
  b5.value_coin = 25.0; b5.state = rqsDamaged
  b5.add_content(ContentItem(name: "米", kind: "粮食", quantity: 35.0, is_edible: true))
  bags.add(b5)

  # --- 层级关系 ---
  sys.register_bag(b1, clZhongDai, parent_id="md1")
  sys.register_bag(b2, clZhongDai, parent_id="md1")
  sys.register_bag(b3, clXiaoDai, parent_id="zd1")
  sys.register_bag(b4, clDaDai, parent_id="md1")
  sys.register_bag(b5, clXiaoDai, parent_id="zd1")

  # 父袋们
  var md1 = new_rongqi("md1", "一号麻袋", "张三")
  md1.value_coin = 0.0; md1.state = rqsStored
  sys.register_bag(md1, clMaDai)

  var zd1 = new_rongqi("zd1", "甲中袋", "张三")
  zd1.value_coin = 0.0; zd1.state = rqsStored
  sys.register_bag(zd1, clZhongDai, parent_id="md1")

  sys.hierarchy.compute_depths()

  # --- 初始状态 ---
  echo "【初始袋子清单】"
  for b in bags:
    echo "  ", b.summary()

  echo ""

  # --- 第一轮审判 ---
  echo "【第一轮审判 · 天庭 · 审判 · 奖惩】"
  sys.process_batch(bags, tick = 0)

  echo ""

  # --- 处理后状态变化 ---
  echo "【处理后状态】"
  for b in bags:
    var st = b.summary()
    if b.should_discard:
      st &= " → 已被丢弃"
    elif b.needs_washing:
      st &= " → 待清洗"
    echo "  ", st

  echo ""

  # --- 第二轮审判（处理后的袋子）---
  echo "【第二轮审判】"
  sys.process_batch(bags, tick = 1)

  echo ""

  # --- 系统报告 ---
  echo system_report(sys)

  echo "══════════════════════════════════════"
  echo "   袋子系统运行完毕"
  echo "══════════════════════════════════════"
