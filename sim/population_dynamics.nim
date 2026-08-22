## ═══════════════════════════════════════════════════════════════
## 人口动态 · 容器生命周期 (出生/衰老/死亡/合并)
## ═══════════════════════════════════════════════════════════════
## 在七层统一社会模拟基础之上:
##   - 每个容器拥有活力值 (0.0~1.0)
##   - 活力随时钟衰减 · 受辐射强度补充
##   - 活力趋零 → 死亡(移除)
##   - 两容器辐射强度超阈 → 合并(生成新容器)
##   - 编码空位达到条件 → 出生(自发产生新容器)
##
## 核心规则:
##   活力变化 = 辐射收益 - 时间衰减 + 层级基数
##   死亡: 活力 < 0.05
##   合并: 辐射强度 > 0.6 且 两容器活力均 > 0.3
##   出生: 存活容器数 < 初始数×0.6 且 时钟_phase = "秒"
## ═══════════════════════════════════════════════════════════════

import unified_society
import std/[sets, math, strutils, random, sequtils, algorithm]

# ═══════════════════════════════════════════════════════════════
# 活力系统
# ═══════════════════════════════════════════════════════════════

type
  Vitality* = object
    energy*: float        # 当前活力 (0.0~1.0)
    age*: int             # 存活滴答数
    birth_tick*: int      # 出生时刻
    parent_a*: int        # 合并来源A (-1=自然诞生)
    parent_b*: int        # 合并来源B (-1=自然诞生)

  PopContainer* = object
    base*: Container      # 七层编码 (继承)
    vit*: Vitality        # 活力状态

  PopulationSociety* = object
    containers*: seq[PopContainer]
    state*: UnifiedState
    initial_count*: int   # 初始容器数
    birth_count*: int     # 总出生数
    death_count*: int     # 总死亡数
    merge_count*: int     # 总合并数
    generation*: int      # 世代数
    last_merge_tick*: int # 上次合并时刻 (冷却用)

# ═══════════════════════════════════════════════════════════════
# 从统一社会转换为人口社会
# ═══════════════════════════════════════════════════════════════
proc to_population*(society: Society): PopulationSociety =
  result.state = society.state
  result.initial_count = society.containers.len
  result.birth_count = 0
  result.death_count = 0
  result.merge_count = 0
  result.generation = 1
  result.last_merge_tick = -10
  for c in society.containers:
    var pc: PopContainer
    pc.base = c
    pc.vit = Vitality(
      energy: 0.3 + rand(0.5).float,  # 初始活力 0.3~0.8
      age: 0,
      birth_tick: 0,
      parent_a: -1,
      parent_b: -1,
    )
    result.containers.add(pc)

# ═══════════════════════════════════════════════════════════════
# 活力自然衰减
# ═══════════════════════════════════════════════════════════════
proc natural_decay*(pc: PopContainer): float =
  # 层级基数: 高层级容器更稳定
  let tier_base = case pc.base.tier
    of 1: 0.008
    of 2: 0.005
    of 3: 0.003
    else: 0.002
  # 年龄衰减: 越老越快 (指数加速)
  let age_factor = 1.0 + float(pc.vit.age) / 200.0 + float(pc.vit.age * pc.vit.age) / 50000.0
  result = tier_base * age_factor

# ═══════════════════════════════════════════════════════════════
# 辐射收益计算 (七层范畴辐射 → 活力补充)
# ═══════════════════════════════════════════════════════════════
proc radiation_gain*(pc: PopContainer, index: int, society: PopulationSociety): float =
  # 将 PopContainer 社会转回统一社会做辐射计算
  var us: Society
  us.state = society.state
  for c in society.containers:
    us.containers.add(c.base)
  let rays = category_radiation(index, us, 2)
  var total_gain = 0.0
  for ray in rays:
    total_gain += ray.strength * 0.002  # 每次辐射补充0.2%
  result = min(total_gain, 0.015)  # 单次上限1.5%

# ═══════════════════════════════════════════════════════════════
# 七层编码融合 (合并时生成新编码)
# ═══════════════════════════════════════════════════════════════
proc merge_code*(a: FQCode, b: FQCode, tick: int): FQCode =
  # 家族: 交替选择 (奇偶滴答决定)
  let fam_keep_a = (tick mod 2) == 0
  result.family = if fam_keep_a: a.family else: b.family
  # 极性: 互补优先 (阴阳合一→阳)
  result.polarity = if a.polarity != b.polarity: '1' else: a.polarity
  # 声母: 按滴答选
  result.shengmu = if (tick mod 3) == 0: a.shengmu else: b.shengmu
  # 韵母: 按滴答选
  result.yunmu = if (tick mod 5) == 0: b.yunmu else: a.yunmu
  # 声调: 取较高能级
  result.tone = if a.tone > b.tone: a.tone else: b.tone

# ═══════════════════════════════════════════════════════════════
# 合并两容器
# ═══════════════════════════════════════════════════════════════
proc merge_containers*(a, b: PopContainer, new_id: int, tick: int): PopContainer =
  let ca = a.base.code
  let cb = b.base.code
  let mc = merge_code(ca, cb, tick)
  var cc: Container
  cc.id = new_id
  cc.tier = max(a.base.tier, b.base.tier)  # 合并取高-tier
  cc.wall = a.base.wall
  cc.polarity = mc.polarity
  cc.code = mc
  cc.clock = a.base.clock
  cc.color = b.base.color  # 合并取B的颜色
  cc.space = a.base.space  # 合并取A的空间
  cc.mirror = b.base.mirror
  cc.music = a.base.music
  var desc = "M"  # M = 合并标记
  desc.add(mc.family)
  desc.add(mc.polarity)
  desc.add(mc.shengmu)
  desc.add(mc.yunmu)
  desc.add(mc.tone)
  cc.descriptor = desc
  result.base = cc
  result.vit = Vitality(
    energy: min((a.vit.energy + b.vit.energy) / 2.0 + 0.1, 0.85),  # 融合平均+奖励，上限0.85
    age: 0,
    birth_tick: tick,
    parent_a: a.base.id,
    parent_b: b.base.id,
  )

# ═══════════════════════════════════════════════════════════════
# 出生新容器 (编码空位自发产生)
# ═══════════════════════════════════════════════════════════════
proc birth_container*(new_id: int, tick: int): PopContainer =
  let sms = ["b", "p", "m", "f", "d", "t", "n", "l"]
  let yms = ["a", "ȍ", "ӗ", "i", "u", "ü"]
  let families = ['C', 'B', 'P']
  let polarities = ['1', '0']
  let tones: array[4, char] = ['1', '2', '3', '4']
  let seed = tick
  var code: FQCode
  code.family = families[seed mod 3]
  code.polarity = polarities[(seed div 3) mod 2]
  code.shengmu = sms[(seed div 7) mod sms.len]
  code.yunmu = yms[(seed div 11) mod yms.len]
  code.tone = tones[(seed div 13) mod 4]
  var cc: Container
  cc.id = new_id
  cc.tier = 1  # 新生总是 L1
  cc.wall = ['n', 'n']
  cc.polarity = code.polarity
  cc.code = code
  cc.clock = clock_from_ticks(tick mod 3600)
  cc.color = COLORS42[seed mod COLORS42.len]
  cc.space = SPACE18[(seed div 5) mod SPACE18.len]
  cc.mirror = MIRROR10[(seed div 9) mod MIRROR10.len]
  cc.music = MUSIC3[seed mod MUSIC3.len]
  var desc = "B"  # B = 出生标记
  desc.add(code.family)
  desc.add(code.polarity)
  desc.add(code.shengmu)
  desc.add(code.yunmu)
  desc.add(code.tone)
  cc.descriptor = desc
  result.base = cc
  result.vit = Vitality(
    energy: 0.8,  # 新生高活力
    age: 0,
    birth_tick: tick,
    parent_a: -1,
    parent_b: -1,
  )

# ═══════════════════════════════════════════════════════════════
# 人口动态演化一步
# ═══════════════════════════════════════════════════════════════
proc population_step*(soc: var PopulationSociety): string =
  var lines: seq[string] = @[]
  soc.state.tick += 1
  let tick = soc.state.tick
  # 更新世代
  let phase_val = tick mod 3600
  if phase_val < 60: soc.state.phase = "秒"
  elif phase_val < 3600: soc.state.phase = "分"
  else: soc.state.phase = "时"

  # 阶段1: 活力更新 (衰减 + 辐射补充)
  for i in 0..<soc.containers.len:
    var pc = soc.containers[i]
    pc.vit.age += 1
    let decay = natural_decay(pc)
    let gain = radiation_gain(pc, i, soc)
    pc.vit.energy = max(0.0, min(1.0, pc.vit.energy - decay + gain))
    soc.containers[i] = pc

  # 阶段2: 死亡判定
  var dead_indices: seq[int] = @[]
  for i in 0..<soc.containers.len:
    if soc.containers[i].vit.energy < 0.05:
      dead_indices.add(i)
      lines.add("  ☠ #" & $soc.containers[i].base.id &
        " " & soc.containers[i].base.descriptor &
        " 活力=" & formatFloat(soc.containers[i].vit.energy, ffDecimal, 3) &
        " 寿=" & $soc.containers[i].vit.age)
  # 移除死亡容器 (从后往前)
  if dead_indices.len > 0:
    for i in countdown(dead_indices.len-1, 0):
      soc.containers.delete(dead_indices[i])
  soc.death_count += dead_indices.len

  # 阶段3: 合并判定 (辐射超阈 → 融合)
  var merged: HashSet[int] = initHashSet[int]()
  var merge_happened = false
  for i in 0..<soc.containers.len:
    if i in merged: continue
    for j in i+1..<soc.containers.len:
      if j in merged: continue
      var us: Society
      us.state = soc.state
      for c in soc.containers:
        us.containers.add(c.base)
      # 合并冷却检查
      if tick - soc.last_merge_tick < 5:
        break  # 冷却期内跳过
      let rays = category_radiation(i, us, 1)
      for ray in rays:
        if ray.dst == j and ray.strength > 0.78:
          if soc.containers[i].vit.energy > 0.25 and soc.containers[j].vit.energy > 0.25:
            # 合并 i + j → new
            let new_id = soc.containers.len + tick
            let nc = merge_containers(soc.containers[i], soc.containers[j], new_id, tick)
            merged.incl(i)
            merged.incl(j)
            soc.containers.add(nc)
            soc.merge_count += 1
            soc.last_merge_tick = tick
            lines.add("  ⊕ #" & $soc.containers[i].base.id &
              " + #" & $soc.containers[j].base.id &
              " → #" & $new_id &
              " " & nc.base.descriptor &
              " 活=" & formatFloat(nc.vit.energy, ffDecimal, 3))
            merge_happened = true
            break
      if merge_happened: break
    if merge_happened: break
  # 移除被合并的容器 (从后往前)
  var merged_vec = toSeq(merged)
  merged_vec.sort()
  if merged_vec.len > 0:
    for i in countdown(merged_vec.len-1, 0):
      soc.containers.delete(merged_vec[i])

  # 阶段4: 出生判定 (种群低于阈值 → 自发新生)
  # 需要连续低种群才出生 (此处简化为阈值判定)
  let threshold = float(soc.initial_count) * 0.5
  if float(soc.containers.len) < threshold and soc.state.phase == "秒" and soc.birth_count < soc.initial_count * 2:
    let new_id = soc.containers.len + tick * 100
    let nc = birth_container(new_id, tick)
    soc.containers.add(nc)
    soc.birth_count += 1
    lines.add("  ✚ 新生 #" & $new_id &
      " " & nc.base.descriptor &
      " 活=" & formatFloat(nc.vit.energy, ffDecimal, 3))

  # 阶段5: 容器ID重排 (保持连续)
  for i in 0..<soc.containers.len:
    soc.containers[i].base.id = i

  # 每20滴答输出一次种群状态 (即使无事件)
  if soc.state.tick mod 20 == 0 and lines.len == 0:
    lines.add("  ── 种群=" & $soc.containers.len & " ──")
  if lines.len > 0:
    result = lines.join("\n")
  else:
    result = ""

# ═══════════════════════════════════════════════════════════════
# 运行人口模拟
# ═══════════════════════════════════════════════════════════════
proc run_population_simulation*(n_containers: int = 15, n_ticks: int = 100) =
  var us = create_society(n_containers)
  var soc = to_population(us)
  echo "═══════════════════════════════════════════════════════════════"
  echo "  人口动态模拟 · 七层统一 + 生命周期"
  echo "═══════════════════════════════════════════════════════════════"
  echo "初始容器: " & $soc.initial_count
  echo "模拟滴答: " & $n_ticks
  echo "死亡阈值: 活力 < 0.05"
  echo "合并阈值: 辐射 > 0.6 & 活力 > 0.3"
  echo "出生阈值: 存活 < 初始×0.6 & 相位=秒"
  echo ""
  echo "── 初始状态 ──"
  for c in soc.containers:
    echo "  #" & $c.base.id &
      " [" & $c.base.tier & "] " &
      c.base.descriptor &
      " 活=" & formatFloat(c.vit.energy, ffDecimal, 3)

  echo ""
  echo "── 演化过程 ──"
  for t in 1..n_ticks:
    let events = soc.population_step()
    if events.len > 0:
      echo ""
      echo "── 滴答 " & $t & " · 存活=" & $soc.containers.len & " ──"
      echo events

  echo ""
  echo "═══════════════════════════════════════════════════════════════"
  echo "  模拟结束 · 统计"
  echo "═══════════════════════════════════════════════════════════════"
  echo "世代数: " & $soc.generation
  echo "最终存活: " & $soc.containers.len
  echo "总出生: " & $soc.birth_count
  echo "总死亡: " & $soc.death_count
  echo "总合并: " & $soc.merge_count
  echo ""
  echo "── 最终容器 ──"
  for c in soc.containers:
    echo "  #" & $c.base.id &
      " [" & $c.base.tier & "] " &
      c.base.descriptor &
      " 活=" & formatFloat(c.vit.energy, ffDecimal, 3) &
      " 年=" & $c.vit.age &
      " 源=" & (if c.vit.parent_a >= 0: "合" else: "生")

# ═══════════════════════════════════════════════════════════════
# 多规模对比运行
# ═══════════════════════════════════════════════════════════════
run_population_simulation(31, 150)
echo "\n"
run_population_simulation(53, 200)
