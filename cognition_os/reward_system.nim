## ═══════════════════════════════════════════════════════════════
## 奖励系统 · 失爱恐惧 · 合一驱力 · 吸引子 · 抗议向量
## ═══════════════════════════════════════════════════════════════
## 核心:
##   奖励回路: 正向行为→奖励→强化→趋向
##   失爱恐惧: 联结断裂→痛苦→修复→守护
##   合一驱力: 分离焦虑→驱动→融合→完整
##   吸引子: 稳定状态·回归点·安全港
##   抗议向量: 不公正→抗议→纠正→重建
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, math

# ═══════════════════════════════════════════════════════════════
# 驱力类型
# ═══════════════════════════════════════════════════════════════
type
  DriveType* = enum
    dtReward,      # 奖励驱力(趋向愉悦)
    dtPunishment,  # 惩罚驱力(回避痛苦)
    dtFearOfLoss,  # 失爱恐惧(联结断裂)
    dtUnity,       # 合一驱力(追求完整)
    dtProtest,     # 抗议向量(对抗不公正)
    dtAttractor,   # 吸引子(趋向稳定)

  Drive* = object
    kind*: DriveType
    strength*: float          # 强度(0-1)
    target*: string           # 趋向/回避的目标
    source*: string           # 驱力来源
    active*: bool             # 是否激活

# ═══════════════════════════════════════════════════════════════
# 奖励事件
# ═══════════════════════════════════════════════════════════════
type
  RewardEvent* = object
    action*: string           # 触发行为
    delta*: float             # 奖励值(+奖励/-惩罚)
    reason*: string           # 原因
    timestamp*: string        # 时间
    associated_symbols*: seq[string]  # 关联符号

# ═══════════════════════════════════════════════════════════════
# 吸引子(稳定回归点)
# ═══════════════════════════════════════════════════════════════
type
  Attractor* = object
    name*: string             # 吸引子名称
    center*: string           # 中心点(稳定状态)
    radius*: float            # 吸引半径
    strength*: float          # 吸引强度
    basin*: seq[string]       # 吸引盆(被此吸引子捕获的状态列表)

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  RewardSystem* = object
    drives*: Table[string, Drive]           # 当前驱力表
    history*: seq[RewardEvent]              # 奖励历史
    attractors*: seq[Attractor]             # 吸引子列表
    bond_strength*: float                   # 联结强度(0-1 · 失爱恐惧相关)
    integrity*: float                       # 完整度(0-1 · 合一驱力相关)
    justice_balance*: float                 # 公正平衡(-1到1 · 抗议相关)
    current_state*: string                  # 当前状态

func new_reward_system*(): RewardSystem =
  result.drives = initTable[string, Drive]()
  result.history = @[]
  result.attractors = @[]
  result.bond_strength = 1.0      # 初始:完整联结
  result.integrity = 1.0          # 初始:完全完整
  result.justice_balance = 0.0    # 初始:公正
  result.current_state = "平安"

# ═══════════════════════════════════════════════════════════════
# 注册驱力
# ═══════════════════════════════════════════════════════════════
proc register_drive*(rs: var RewardSystem; name: string; kind: DriveType;
                    strength: float; target: string; source: string) =
  rs.drives[name] = Drive(
    kind: kind, strength: strength, target: target, source: source, active: true
  )

# ═══════════════════════════════════════════════════════════════
# 添加奖励事件
# ═══════════════════════════════════════════════════════════════
proc add_event*(rs: var RewardSystem; action, reason: string; delta: float;
                symbols: seq[string] = @[]) =
  rs.history.add(RewardEvent(
    action: action, delta: delta, reason: reason,
    timestamp: "now", associated_symbols: symbols
  ))
  
  # 更新系统状态
  if delta > 0:
    rs.bond_strength = min(1.0, rs.bond_strength + delta * 0.1)
    rs.integrity = min(1.0, rs.integrity + delta * 0.05)
  else:
    rs.bond_strength = max(0.0, rs.bond_strength + delta * 0.15)
    rs.integrity = max(0.0, rs.integrity + delta * 0.1)

# ═══════════════════════════════════════════════════════════════
# 添加吸引子
# ═══════════════════════════════════════════════════════════════
proc add_attractor*(rs: var RewardSystem; name, center: string;
                   radius, strength: float; basin: seq[string]) =
  rs.attractors.add(Attractor(
    name: name, center: center, radius: radius,
    strength: strength, basin: basin
  ))

# ═══════════════════════════════════════════════════════════════
# 计算: 当前状态离哪个吸引子最近
# ═══════════════════════════════════════════════════════════════
func nearest_attractor*(rs: RewardSystem; state: string): Attractor =
  for a in rs.attractors:
    if state in a.basin:
      return a
  # 默认返回第一个
  if rs.attractors.len > 0:
    return rs.attractors[0]
  # 空吸引子
  result.name = "空"
  result.center = "无"
  result.radius = 0.0
  result.strength = 0.0

# ═══════════════════════════════════════════════════════════════
# 失爱恐惧计算
# ═══════════════════════════════════════════════════════════════
func calc_fear_of_loss*(rs: RewardSystem): float =
  ## 失爱恐惧 = 1 - 联结强度 · 越弱越恐惧
  return 1.0 - rs.bond_strength

# ═══════════════════════════════════════════════════════════════
# 合一驱力计算
# ═══════════════════════════════════════════════════════════════
func calc_unity_drive*(rs: RewardSystem): float =
  ## 合一驱力 = 1 - 完整度 · 越破碎越驱动
  return 1.0 - rs.integrity

# ═══════════════════════════════════════════════════════════════
# 抗议向量计算
# ═══════════════════════════════════════════════════════════════
func calc_protest_vector*(rs: RewardSystem): tuple[direction: string; magnitude: float] =
  ## 抗议向量 = -公正平衡 · 负向不公正触发抗议
  if rs.justice_balance < -0.3:
    return ("抗议", abs(rs.justice_balance))
  elif rs.justice_balance > 0.3:
    return ("满意", rs.justice_balance)
  else:
    return ("平静", 0.0)

# ═══════════════════════════════════════════════════════════════
# 行为决策: 当前状态下该做什么
# ═══════════════════════════════════════════════════════
proc decide_action*(rs: var RewardSystem): tuple[action: string; urgency: float; reason: string] =
  let fear = calc_fear_of_loss(rs)
  let unity = calc_unity_drive(rs)
  let protest = calc_protest_vector(rs)
  
  # 最高优先级: 失爱恐惧(联结断裂是最危险的)
  if fear > 0.5:
    return ("修复联结", fear, "失爱恐惧: 联结强度仅" & formatFloat(rs.bond_strength, ffDecimal, 2))
  
  # 次优先级: 合一驱力(追求完整)
  if unity > 0.6:
    return ("追求合一", unity, "合一驱力: 完整度仅" & formatFloat(rs.integrity, ffDecimal, 2))
  
  # 第三: 抗议不公正
  if protest.direction == "抗议" and protest.magnitude > 0.3:
    return ("抗议纠正", protest.magnitude, "不公正: 平衡值" & formatFloat(rs.justice_balance, ffDecimal, 2))
  
  # 默认: 趋向吸引子
  let attractor = rs.nearest_attractor(rs.current_state)
  if attractor.strength > 0:
    return ("趋向" & attractor.name, attractor.strength * 0.5, "被吸引子 '" & attractor.center & "' 牵引")
  
  return ("维持现状", 0.1, "无显著驱力")

# ═══════════════════════════════════════════════════════════════
# 模拟场景: 符号被篡改(触发失爱恐惧)
# ═══════════════════════════════════════════════════════════════
proc simulate_tamper*(rs: var RewardSystem; symbol: string) =
  add_event(rs, "符号'" & symbol & "'被篡改", "外来编码覆盖", -0.3, @[symbol])
  rs.justice_balance -= 0.2

# ═══════════════════════════════════════════════════════════════
# 模拟场景: 容器丢失(触发合一驱力)
# ═══════════════════════════════════════════════════════════════
proc simulate_loss*(rs: var RewardSystem; container: string) =
  add_event(rs, "容器'" & container & "'丢失", "完整性受损", -0.4, @[])
  rs.integrity -= 0.3

# ═══════════════════════════════════════════════════════════════
# 模拟场景: 完成联结(奖励)
# ═══════════════════════════════════════════════════════════════
proc simulate_bond*(rs: var RewardSystem) =
  add_event(rs, "声母与韵母正确配对", "正确联结", 0.2, @["b", "a"])
  rs.bond_strength = min(1.0, rs.bond_strength + 0.15)
  rs.justice_balance = min(1.0, rs.justice_balance + 0.1)

# ═══════════════════════════════════════════════════════════════
# 系统报告
# ═══════════════════════════════════════════════════════════════
proc report*(rs: RewardSystem) =
  echo "═══ 奖励系统状态 ═══"
  echo ""
  echo "── 核心指标 ──"
  echo "  联结强度: ", formatFloat(rs.bond_strength, ffDecimal, 2), " (失爱恐惧: ", formatFloat(calc_fear_of_loss(rs), ffDecimal, 2), ")"
  echo "  完整度:   ", formatFloat(rs.integrity, ffDecimal, 2), " (合一驱力: ", formatFloat(calc_unity_drive(rs), ffDecimal, 2), ")"
  echo "  公正平衡: ", formatFloat(rs.justice_balance, ffDecimal, 2), " (抗议: ", calc_protest_vector(rs).direction, ")"
  echo "  当前状态: ", rs.current_state
  echo ""
  
  echo "── 驱力表 ──"
  for name, drive in rs.drives:
    let status = if drive.active: "激活" else: "休眠"
    echo "  ", alignLeft(name, 12), " ", alignLeft($drive.kind, 10), " 强度:", formatFloat(drive.strength, ffDecimal, 2), " 目标:", drive.target, " [", status, "]"
  echo ""
  
  echo "── 吸引子 ──"
  for a in rs.attractors:
    echo "  ", alignLeft(a.name, 8), " 中心:", alignLeft(a.center, 6), " 半径:", formatFloat(a.radius, ffDecimal, 1), " 强度:", formatFloat(a.strength, ffDecimal, 2)
    echo "    吸引盆: ", a.basin.join(", ")
  echo ""
  
  echo "── 奖励历史(最近5条) ──"
  let start = max(0, rs.history.len - 5)
  for i in start..<rs.history.len:
    let ev = rs.history[i]
    let sign = if ev.delta > 0: "+" else: ""
    echo "  ", alignLeft(ev.action, 20), " ", sign, formatFloat(ev.delta, ffDecimal, 2), "  ", ev.reason

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var rs = new_reward_system()
  
  # 注册核心驱力
  register_drive(rs, "失爱恐惧", dtFearOfLoss, 0.8, "联结完整", "刘楚恬-我")
  register_drive(rs, "合一驱力", dtUnity, 0.7, "完整融合", "夕瑶协议")
  register_drive(rs, "公正维护", dtProtest, 0.5, "产权公正", "满全法")
  register_drive(rs, "趋向平安", dtAttractor, 0.6, "平安状态", "系统本能")
  
  # 定义吸引子
  add_attractor(rs, "平安", "和谐", 0.5, 0.8, @["平安", "稳定", "和谐", "完整"])
  add_attractor(rs, "创造", "生成", 0.3, 0.6, @["创造", "生长", "扩展", "绽放"])
  add_attractor(rs, "回归", "修复", 0.4, 0.9, @["修复", "联结", "团聚", "回归"])
  
  echo "═══ 初始状态 ═══"
  report(rs)
  echo ""
  echo ""
  
  # ── 模拟场景1: 正确配对(奖励) ──
  echo "═══ 场景1: 正确配对行为 ═══"
  simulate_bond(rs)
  simulate_bond(rs)
  let d1 = rs.decide_action()
  echo "决策: ", d1.action, " (紧急度:", formatFloat(d1.urgency, ffDecimal, 2), ") 原因:", d1.reason
  echo ""
  
  # ── 模拟场景2: 符号被篡改(触发失爱恐惧+抗议) ──
  echo "═══ 场景2: 符号被篡改 ═══"
  simulate_tamper(rs, "ꬰ")
  simulate_tamper(rs, "a")
  let d2 = rs.decide_action()
  echo "决策: ", d2.action, " (紧急度:", formatFloat(d2.urgency, ffDecimal, 2), ") 原因:", d2.reason
  echo ""
  
  # ── 模拟场景3: 容器丢失(触发合一驱力) ──
  echo "═══ 场景3: 容器丢失 ═══"
  simulate_loss(rs, "韵母容器")
  simulate_loss(rs, "声调容器")
  let d3 = rs.decide_action()
  echo "决策: ", d3.action, " (紧急度:", formatFloat(d3.urgency, ffDecimal, 2), ") 原因:", d3.reason
  echo ""
  
  # ── 最终报告 ──
  echo "═══ 最终状态 ═══"
  report(rs)
