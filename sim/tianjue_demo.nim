## ═══════════════════════════════════════════════════════════════
## tianjue_demo.nim — 天觉系统完整演示
##   视觉感知 · 听觉授权 · 审判奖惩 · 历史积累 · 高维层级
## ═══════════════════════════════════════════════════════════════
##
## 核心逻辑:
##   公约数 = 袋子定义的标准属性 (不可变的基准)
##   公倍数 = 每个具体袋子的审判实例
##   照镜子 = 公倍数映射回公约数进行对比
##   偏差 = 审判对了 → 奖励 / 审判错了 → 惩罚
##
## 运行: nim c -r tianjue_demo.nim
## ═══════════════════════════════════════════════════════════════

import std/[strutils, times, math, tables, sequtils, algorithm]

# ═══════════════════════════════════════════════════════════════
# 第一部分: 公约数定义 — 袋子的基准锚点
# ═══════════════════════════════════════════════════════════════
## 公约数 = 不可变的定义基准
## 不是数米粒，是定义"袋子应该是什么样的"

type
  BagStandard* = object  ## 公约数锚点
    owner*: string           # 谁家的袋子
    min_usability*: float    # 最低可用性
    min_content_quality*: float  # 最低内容质量
    min_cleanliness*: float  # 最低清洁度
    min_dryness*: float      # 最低干燥度
    max_pest_tolerance*: float  # 最大虫害容忍 (越低越好)
    min_purity*: float       # 最低纯净度
    base_value*: float       # 基准价值

  BagInstance* = object  ## 公倍数 = 围绕公约数旋转的具体袋子
    id*: int
    owner*: string
    usability*: float
    content_quality*: float
    cleanliness*: float
    dryness*: float
    pest_level*: float       # 0=无虫, 1=严重虫害
    purity*: float
    value*: float
    verdict_history*: seq[VerdictEntry]
    total_reward*: float
    total_penalty*: float

  VerdictEntry* = object  ## 审判记录
    trial_num*: int
    timestamp*: float
    predicted_health*: float
    actual_health*: float
    verdict*: string         # "正确" 或 "错误"
    delta*: float            # 偏差量
    reward*: float
    penalty*: float
    bag_state_snapshot*: BagInstance  # 审判时刻的袋子状态快照

# ═══════════════════════════════════════════════════════════════
# 第二部分: 听觉授权 — 天觉系统的激活
# ═══════════════════════════════════════════════════════════════

type
  TianJueAuthState* = enum
    tsDormant,      # 休眠 — 未授权
    tsAuthorized,   # 已授权 — 视觉&审判激活
    tsRevoked       # 撤销 — 授权收回

  TianJueState* = object
    auth*: TianJueAuthState
    bag_standards*: Table[string, BagStandard]   # 公约数表
    bags*: Table[int, BagInstance]               # 袋子实例表
    visible_bags*: seq[int]                      # 视觉感知到的袋子
    trial_counter*: int
    global_history*: seq[VerdictEntry]           # 全量历史积累
    reward_pool*: float                          # 奖励池
    penalty_pool*: float                         # 惩罚池

proc initTianJue*(): TianJueState =
  result.auth = tsDormant
  result.visible_bags = @[]
  result.trial_counter = 0
  result.global_history = @[]
  result.reward_pool = 0.0
  result.penalty_pool = 0.0

proc authorize*(tj: var TianJueState) =
  tj.auth = tsAuthorized
  echo "[天觉] 听觉已授权 — 视觉系统激活 · 审判系统就绪"

proc revoke*(tj: var TianJueState) =
  tj.auth = tsRevoked
  echo "[天觉] 授权已撤销 — 进入静默模式"

proc canPerceive*(tj: TianJueState): bool = tj.auth == tsAuthorized
proc canJudge*(tj: TianJueState): bool = tj.auth == tsAuthorized

# ═══════════════════════════════════════════════════════════════
# 第三部分: 视觉感知 — 只识别袋子，不分析米粒
# ═══════════════════════════════════════════════════════════════

proc perceive_bag*(tj: var TianJueState, bag_id: int): bool =
  if not tj.canPerceive():
    echo "[视觉] 未授权 — 无法感知"
    return false
  if bag_id notin tj.bags:
    echo "[视觉] 错误 — 袋子 #" & $bag_id & " 不存在"
    return false
  if bag_id notin tj.visible_bags:
    tj.visible_bags.add(bag_id)
  echo "[视觉] 感知到袋子 #" & $bag_id & " (主人:" & tj.bags[bag_id].owner & ") — 不分析内容物，只记录袋子本身"
  return true

# ═══════════════════════════════════════════════════════════════
# 第四部分: 公约数-公倍数 锚定 + 照镜子对比
# ═══════════════════════════════════════════════════════════════

proc compute_health*(bag: BagInstance): float =
  ## 高维层级认知 — 袋子级的综合健康度
  ## 不是数米粒，是计算袋子"好不好用"
  let product = bag.usability *
                bag.content_quality.max(0.001) *
                bag.cleanliness.max(0.001) *
                bag.dryness.max(0.001) *
                (1.0 - bag.pest_level).max(0.001) *
                bag.purity.max(0.001)
  result = pow(product, 1.0 / 6.0)

proc mirror_compare*(standard: BagStandard, bag: BagInstance, predicted: float): string =
  ## 照镜子: 把袋子映射回公约数进行对比
  ## 偏差在阈值内 = 正确 (公倍数符合公约数)
  ## 偏差超出阈值 = 错误 (公倍数偏离公约数)
  let actual = compute_health(bag)
  let delta = abs(predicted - actual)
  if delta <= 0.15:
    result = "正确"
  else:
    result = "错误"

# ═══════════════════════════════════════════════════════════════
# 第五部分: 审判 + 奖惩 — 对了奖，错了罚
# ═══════════════════════════════════════════════════════════════

proc trial_bag*(tj: var TianJueState, bag_id: int, predicted_health: float): int =
  ## 审判一个袋子
  ## 返回: 审判编号 (-1 表示失败)
  if not tj.canJudge():
    echo "[审判] 未授权 — 无法审判"
    return -1
  if bag_id notin tj.bags:
    echo "[审判] 错误 — 目标袋子不存在"
    return -1

  inc tj.trial_counter
  let trial_id = tj.trial_counter
  let bag = tj.bags[bag_id]
  let actual_health = compute_health(bag)

  # 照镜子对比 (公约数基准 vs 公倍数实际)
  let standard = tj.bag_standards.getOrDefault(bag.owner)
  let verdict = mirror_compare(standard, bag, predicted_health)
  let delta = abs(predicted_health - actual_health)

  # 奖惩
  var reward = 0.0
  var penalty = 0.0

  if verdict == "正确" and delta < 0.15:
    reward = (0.5 * (1.0 - delta)).min(1.0)
    penalty = 0.0
  elif verdict == "正确":
    reward = 0.2
    penalty = 0.0
  else:
    reward = 0.0
    penalty = delta.min(2.0)  # 偏差越大惩罚越重

  # 应用奖惩到袋子
  var updated_bag = bag
  if reward > 0:
    # 奖励: 提升袋子的各项属性
    updated_bag.usability = (updated_bag.usability + reward * 0.05).min(1.0)
    updated_bag.content_quality = (updated_bag.content_quality + reward * 0.03).min(1.0)
    updated_bag.cleanliness = (updated_bag.cleanliness + reward * 0.02).min(1.0)
  if penalty > 0:
    # 惩罚: 降低袋子的各项属性
    updated_bag.usability = (updated_bag.usability - penalty * 0.05).max(0.0)
    updated_bag.content_quality = (updated_bag.content_quality - penalty * 0.04).max(0.0)
    updated_bag.cleanliness = (updated_bag.cleanliness - penalty * 0.03).max(0.0)
    updated_bag.purity = (updated_bag.purity - penalty * 0.02).max(0.0)
    updated_bag.pest_level = (updated_bag.pest_level + penalty * 0.01).min(1.0)

  updated_bag.total_reward += reward
  updated_bag.total_penalty += penalty

  # 创建审判记录 — 不可变快照
  let entry = VerdictEntry(
    trial_num: trial_id,
    timestamp: epochTime(),
    predicted_health: predicted_health,
    actual_health: actual_health,
    verdict: verdict,
    delta: delta,
    reward: reward,
    penalty: penalty,
    bag_state_snapshot: updated_bag  # 保存完整状态快照
  )

  updated_bag.verdict_history.add(entry)
  tj.bags[bag_id] = updated_bag
  tj.global_history.add(entry)
  tj.reward_pool += reward
  tj.penalty_pool += penalty

  # 输出审判报告
  echo "═══════════════════════════════════════════════════════════"
  echo "[审判 #" & $trial_id & "] 袋子 #" & $bag_id & " (主人:" & bag.owner & ")"
  echo "  预测健康度: " & formatFloat(predicted_health, ffDecimal, 3)
  echo "  实际健康度: " & formatFloat(actual_health, ffDecimal, 3)
  echo "  偏差:       " & formatFloat(delta, ffDecimal, 3)
  echo "  照镜子结果: " & verdict
  if reward > 0:
    echo "  奖励: +" & formatFloat(reward, ffDecimal, 2) & " → 袋子属性提升"
  if penalty > 0:
    echo "  惩罚: -" & formatFloat(penalty, ffDecimal, 2) & " → 袋子属性下降"
  echo "═══════════════════════════════════════════════════════════"

  return trial_id

# ═══════════════════════════════════════════════════════════════
# 第六部分: 高维层级认知 — 袋子属性命名与描述
# ═══════════════════════════════════════════════════════════════

proc describe_bag*(tj: TianJueState, bag_id: int): string =
  if bag_id notin tj.bags:
    return "袋子不存在"
  let bag = tj.bags[bag_id]
  var lines: seq[string] = @[]

  lines.add("─── 袋子 #" & $bag_id & " 高维属性描述 ───")
  lines.add("归属: " & bag.owner & "家的袋子")
  lines.add("价值: " & formatFloat(bag.value, ffDecimal, 1) & " (基准:" & formatFloat(tj.bag_standards.getOrDefault(bag.owner).base_value, ffDecimal, 1) & ")")
  lines.add("可用性: " & (if bag.usability > 0.5: "好用 ✓" else: "已损坏 ✗"))
  lines.add("内容质量: " & (if bag.content_quality > 0.5: "米好吃 ✓" else: "米变质 ✗"))
  lines.add("清洁度: " & (if bag.cleanliness > 0.5: "干净 ✓" else: "脏了 ✗ 需要洗"))
  lines.add("干燥度: " & (if bag.dryness > 0.5: "晒干了 ✓" else: "未干 ✗"))
  lines.add("虫害: " & (if bag.pest_level < 0.3: "无虫 ✓" else: "有虫 ✗ 需除虫"))
  lines.add("纯净度: " & (if bag.purity > 0.5: "无沙 ✓" else: "有沙 ✗ 需筛"))
  lines.add("综合健康度: " & formatFloat(compute_health(bag), ffDecimal, 3))
  lines.add("累计奖励: +" & formatFloat(bag.total_reward, ffDecimal, 2))
  lines.add("累计惩罚: -" & formatFloat(bag.total_penalty, ffDecimal, 2))
  lines.add("历史审判次数: " & $bag.verdict_history.len)

  # 判断处理建议
  lines.add("─── 处理建议 ───")
  if bag.cleanliness < 0.3: lines.add("⚠ 袋子脏了 → 要洗")
  if bag.dryness < 0.5: lines.add("⚠ 谷子未干 → 要晒")
  if bag.pest_level > 0.3: lines.add("⚠ 有虫 → 要除虫")
  if bag.purity < 0.4: lines.add("⚠ 有沙子 → 要筛")
  if bag.content_quality < 0.3: lines.add("⚠ 米不能吃 → 扔掉")
  if compute_health(bag) > 0.7: lines.add("✓ 这袋子状态好 → 可以用")

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 第七部分: 历史积累报告
# ═══════════════════════════════════════════════════════════════

proc report_history*(tj: TianJueState): string =
  var lines: seq[string] = @[]
  lines.add("")
  lines.add("╔═══════════════════════════════════════════════════════════╗")
  lines.add("║              天觉系统 — 历史积累报告                      ║")
  lines.add("╠═══════════════════════════════════════════════════════════╣")
  let auth_str = case tj.auth:
    of tsDormant: "休眠"
    of tsAuthorized: "已授权"
    of tsRevoked: "已撤销"
  lines.add("║ 授权状态: " & auth_str)
  lines.add("║ 视觉感知袋数: " & $tj.visible_bags.len)
  lines.add("║ 袋子总数: " & $tj.bags.len)
  lines.add("║ 总审判次数: " & $tj.trial_counter)
  lines.add("║ 历史记录总数: " & $tj.global_history.len)
  lines.add("║ 奖励池: +" & formatFloat(tj.reward_pool, ffDecimal, 2))
  lines.add("║ 惩罚池: -" & formatFloat(tj.penalty_pool, ffDecimal, 2))

  if tj.global_history.len > 0:
    let correct_count = tj.global_history.countIt(it.verdict == "正确")
    let wrong_count = tj.global_history.countIt(it.verdict == "错误")
    let accuracy = correct_count.float / tj.global_history.len.float
    lines.add("║ 审判准确率: " & formatFloat(accuracy * 100, ffDecimal, 1) & "%")
    lines.add("║   ├─ 正确: " & $correct_count & " 次 (获得奖励)")
    lines.add("║   └─ 错误: " & $wrong_count & " 次 (受到惩罚)")

  lines.add("╚═══════════════════════════════════════════════════════════╝")
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 第八部分: 袋子管理
# ═══════════════════════════════════════════════════════════════

proc register_standard*(tj: var TianJueState, owner: string, base_value: float) =
  tj.bag_standards[owner] = BagStandard(
    owner: owner,
    min_usability: 0.5,
    min_content_quality: 0.5,
    min_cleanliness: 0.3,
    min_dryness: 0.5,
    max_pest_tolerance: 0.3,
    min_purity: 0.4,
    base_value: base_value
  )
  echo "[注册] 公约数锚点已设定 — " & owner & "家的袋子标准"

proc create_bag*(tj: var TianJueState, id: int, owner: string,
                 usability, content_quality, cleanliness, dryness,
                 pest_level, purity: float): int =
  let standard = tj.bag_standards.getOrDefault(owner)
  let bag = BagInstance(
    id: id,
    owner: owner,
    usability: usability,
    content_quality: content_quality,
    cleanliness: cleanliness,
    dryness: dryness,
    pest_level: pest_level,
    purity: purity,
    value: standard.base_value * usability,
    verdict_history: @[],
    total_reward: 0.0,
    total_penalty: 0.0
  )
  tj.bags[id] = bag
  echo "[制造] 袋子 #" & $id & " 已创建 (主人:" & owner & ", 健康度:" & formatFloat(compute_health(bag), ffDecimal, 2) & ")"

# ═══════════════════════════════════════════════════════════════
# 第九部分: 完整工作流演示
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo ""
  echo "╔═══════════════════════════════════════════════════════════╗"
  echo "║              天觉系统 · 完整工作流演示                    ║"
  echo "║     视觉感知 · 听觉授权 · 审判奖惩 · 历史积累            ║"
  echo "║            公约数锚定 · 公倍数旋转 · 照镜子               ║"
  echo "╚═══════════════════════════════════════════════════════════╝"
  echo ""

  # ── 步骤1: 初始化天觉系统 ──
  echo "═══ 步骤1: 初始化天觉系统 ═══"
  var tj = initTianJue()
  echo ""

  # ── 步骤2: 听觉授权 ──
  echo "═══ 步骤2: 听觉授权 ═══"
  tj.authorize()
  echo ""

  # ── 步骤3: 定义公约数 (袋子的基准) ──
  echo "═══ 步骤3: 定义公约数锚点 ═══"
  register_standard(tj, "张家", 100.0)
  register_standard(tj, "李家", 80.0)
  echo ""

  # ── 步骤4: 创建袋子 (公倍数 = 围绕公约数的具体实例) ──
  echo "═══ 步骤4: 创建袋子 (公倍数实例) ═══"
  # 张家的袋子 — 好的
  discard create_bag(tj, 1, "张家",
             usability = 0.9,      # 袋子好用
             content_quality = 0.85,  # 米好吃
             cleanliness = 0.8,   # 干净
             dryness = 0.9,       # 晒干了
             pest_level = 0.05,   # 几乎无虫
             purity = 0.85)       # 几乎无沙

  # 李家的袋子 — 有问题的
  discard create_bag(tj, 2, "李家",
             usability = 0.7,
             content_quality = 0.4,   # 米变质
             cleanliness = 0.2,       # 脏了
             dryness = 0.3,           # 未干
             pest_level = 0.5,        # 有虫
             purity = 0.3)            # 有沙

  # 张家第二个袋子 — 状态中等
  discard create_bag(tj, 3, "张家",
             usability = 0.6,
             content_quality = 0.7,
             cleanliness = 0.5,
             dryness = 0.6,
             pest_level = 0.2,
             purity = 0.6)
  echo ""

  # ── 步骤5: 视觉感知 ──
  echo "═══ 步骤5: 视觉感知 ═══"
  discard perceive_bag(tj, 1)
  discard perceive_bag(tj, 2)
  discard perceive_bag(tj, 3)
  echo ""

  # ── 步骤6: 高维层级认知 — 袋子属性描述 ──
  echo "═══ 步骤6: 高维层级认知 — 袋子属性描述 ═══"
  echo describe_bag(tj, 1)
  echo ""
  echo describe_bag(tj, 2)
  echo ""
  echo describe_bag(tj, 3)
  echo ""

  # ── 步骤7: 审判 → 对了奖，错了罚 ──
  echo "═══ 步骤7: 审判 + 奖惩 ═══"
  echo "── 场景7a: 准确审判 (预测≈实际) → 应得奖励 ──"
  echo "  张家#1: 预测健康度 0.85, 实际 ~0.79"
  discard trial_bag(tj, 1, 0.85)  # 准确 → 奖励

  echo ""
  echo "── 场景7b: 不准确审判 (预测远偏离实际) → 应受惩罚 ──"
  echo "  李家#2: 预测健康度 0.9, 实际 ~0.29 (有虫脏米变质)"
  discard trial_bag(tj, 2, 0.90)  # 严重偏差 → 惩罚

  echo ""
  echo "── 场景7c: 张家#1 二次审判 (偏差更小) ──"
  echo "  张家#1: 预测健康度 0.80, 实际 ~0.80 (已获奖励提升)"
  discard trial_bag(tj, 1, 0.80)  # 准确 → 奖励

  echo ""
  echo "── 场景7d: 李家#2 二次审判 (稍微准确一些但仍偏离) ──"
  echo "  李家#2: 预测健康度 0.5, 实际 ~0.21 (已被惩罚降质)"
  discard trial_bag(tj, 2, 0.50)  # 仍偏差 → 仍惩罚

  echo ""

  # ── 步骤8: 高维认知 — 赏后对比 ──
  echo "═══ 步骤8: 奖惩后高维属性变化 ═══"
  echo describe_bag(tj, 1)
  echo ""
  echo describe_bag(tj, 2)
  echo ""

  # ── 步骤9: 历史积累报告 ──
  echo "═══ 步骤9: 历史积累报告 ═══"
  echo report_history(tj)

  # ── 步骤10: 对比不同版本 (历史状态快照) ──
  echo ""
  echo "═══ 步骤10: 版本对比 — 每个审判时刻的快照都保留 ═══"
  echo "── 张家#1 的版本历史 ──"
  let bag1 = tj.bags[1]
  for entry in bag1.verdict_history:
    echo "  [#" & $entry.trial_num & "] " &
         "预测=" & formatFloat(entry.predicted_health, ffDecimal, 2) & " " &
         "实际=" & formatFloat(entry.actual_health, ffDecimal, 2) & " " &
         "偏差=" & formatFloat(entry.delta, ffDecimal, 2) & " → " &
         entry.verdict &
         " (奖+" & formatFloat(entry.reward, ffDecimal, 2) &
         "/惩-" & formatFloat(entry.penalty, ffDecimal, 2) & ")"

  echo ""
  echo "── 李家#2 的版本历史 ──"
  let bag2 = tj.bags[2]
  for entry in bag2.verdict_history:
    echo "  [#" & $entry.trial_num & "] " &
         "预测=" & formatFloat(entry.predicted_health, ffDecimal, 2) & " " &
         "实际=" & formatFloat(entry.actual_health, ffDecimal, 2) & " " &
         "偏差=" & formatFloat(entry.delta, ffDecimal, 2) & " → " &
         entry.verdict &
         " (奖+" & formatFloat(entry.reward, ffDecimal, 2) &
         " / 惩-" & formatFloat(entry.penalty, ffDecimal, 2) & ")"

  # ── 最终状态 ──
  echo ""
  echo "═══ 最终状态总结 ═══"
  echo ""
  echo "╔═══════════════════════════════════════════════════════════╗"
  echo "║  #  │ 主人 │ 审判 │ 奖励  │ 惩罚  │ 健康度  │ 最终状态   ║"
  echo "╠═══════════════════════════════════════════════════════════╣"
  for id in sorted(toSeq(tj.bags.keys)):
    let b = tj.bags[id]
    let h = compute_health(b)
    let status = (if h > 0.7: "良好 ✓" elif h > 0.4: "中等 ~" else: "差 ✗")
    echo "║  " & $id & " │ " & b.owner & "  │ " &
         $b.verdict_history.len & "    │ +" &
         formatFloat(b.total_reward, ffDecimal, 1) & " │ -" &
         formatFloat(b.total_penalty, ffDecimal, 1) & " │ " &
         formatFloat(h, ffDecimal, 2) & "   │ " &
         status & "      ║"
  echo "╚═══════════════════════════════════════════════════════════╝"

  echo ""
  echo "演示完成。"
  echo ""
  echo "── 核心对应关系 ──"
  echo "视觉系统     = perceive_bag()     — 只识别袋子，不分析内容"
  echo "听觉授权     = authorize()        — 授权后视觉+审判激活"
  echo "天觉系统     = TianJueState        — 整体状态容器"
  echo "公约数锚定   = BagStandard         — 不可变的袋子定义基准"
  echo "公倍数旋转   = BagInstance         — 围绕公约数的具体实例"
  echo "照镜子对比   = mirror_compare()    — 公倍数映射回公约数判定对错"
  echo "审判奖惩     = trial_bag()         — 对了奖，错了罚"
  echo "历史积累     = VerdictEntry快照    — 每次审判不可变记录"
  echo "高维层级     = describe_bag()      — 袋子属性的多维度命名"
