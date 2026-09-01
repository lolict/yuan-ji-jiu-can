## ═══════════════════════════════════════════════════════════════
## 容器调度系统 · 声母调入 · 折叠 · 减法倒计时
## ═══════════════════════════════════════════════════════════════
## 核心规则:
##   1. 声母调入: 一次调入一个声母 · 遍历完韵母后再调下一个
##   2. 折叠: 符号用完不覆盖原层 · 折叠成新层
##   3. 减法倒计时: 两容器混合用减法决定先后
##   4. 边界到达后从下一个容器拿新东西
##   5. 产权不丢失: 每次调度保持产权标记
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, times

# ═══════════════════════════════════════════════════════════════
# 常量: 容器本体
# ═══════════════════════════════════════════════════════════════
const
  SHENGMU* = ["b", "p", "m", "f", "d", "t", "n", "l",
              "g", "k", "h", "j", "q", "x", "y", "w"]
  YUNMU* = ["a", "o", "e", "i", "u", "ü", "ai", "ei",
            "ao", "ou", "ie", "üe", "an", "en", "in", "un",
            "ang", "eng", "ing", "ong", "er"]
  TONES* = ["ˉ", "ˊ", "ˇ", "ˋ"]

# ═══════════════════════════════════════════════════════════════
# 符号状态
# ═══════════════════════════════════════════════════════════════
type
  SymbolState* = enum
    ssIdle,       # 待使用
    ssTraining,   # 训练中
    ssFolded,     # 已折叠(用完收卷)
    ssCompleted,  # 已完成(遍历完毕)

  SymbolEntry* = object
    glyph*: string            # 符号字形
    container*: string        # 所属容器
    state*: SymbolState       # 当前状态
    fold_layer*: int          # 折叠层号(0=原始 · >0=折叠次数)
    trained_with*: seq[string]  # 与哪些符号配对训练过
    owner*: string            # 产权归属(主人/子代理)
    created_at*: string       # 创建版本

# ═══════════════════════════════════════════════════════════════
# 容器层
# ═══════════════════════════════════════════════════════════════
type
  ContainerLayer* = object
    name*: string                    # 容器名称
    symbols*: Table[string, SymbolEntry]  # 符号表
    current_index*: int              # 当前声母索引
    current_yunmu_index*: int        # 当前遍历到的韵母
    exhausted*: bool                 # 声母是否遍历完
    fold_count*: int                 # 该容器的折叠次数

func new_container_layer*(name: string): ContainerLayer =
  result.name = name
  result.symbols = initTable[string, SymbolEntry]()
  result.current_index = 0
  result.current_yunmu_index = 0
  result.exhausted = false
  result.fold_count = 0

# ═══════════════════════════════════════════════════════════════
# 调度器本体
# ═══════════════════════════════════════════════════════════════
type
  Scheduler* = object
    shengmu*: ContainerLayer          # 声母容器层
    yunmu*: ContainerLayer            # 韵母容器层
    tone*: ContainerLayer             # 声调容器层
    output_stream*: seq[string]       # 输出流(调度结果)
    fold_stack*: seq[seq[string]]     # 折叠栈(每层用完的符号)
    step_count*: int                  # 调度步数
    version*: string                  # 当前版本

func new_scheduler*(): Scheduler =
  var s = Scheduler()
  s.shengmu = new_container_layer("声母")
  s.yunmu = new_container_layer("韵母")
  s.tone = new_container_layer("声调")
  s.output_stream = @[]
  s.fold_stack = @[]
  s.step_count = 0
  s.version = "v0.0.0"
  
  # 装载声母符号
  for i, g in SHENGMU:
    s.shengmu.symbols[g] = SymbolEntry(
      glyph: g, container: "声母", state: ssIdle, fold_layer: 0,
      owner: "主人", created_at: "v0.0.0"
    )
  
  # 装载韵母符号
  for i, g in YUNMU:
    s.yunmu.symbols[g] = SymbolEntry(
      glyph: g, container: "韵母", state: ssIdle, fold_layer: 0,
      owner: "主人", created_at: "v0.0.0"
    )
  
  # 装载声调符号
  for i, g in TONES:
    s.tone.symbols[g] = SymbolEntry(
      glyph: g, container: "声调", state: ssIdle, fold_layer: 0,
      owner: "主人", created_at: "v0.0.0"
    )
  
  return s

# ═══════════════════════════════════════════════════════════════
# 规则1: 声母调入 · 一次一个 · 遍历完韵母再调下一个
# ═══════════════════════════════════════════════════════════════
proc step_import_shengmu*(s: var Scheduler): tuple[sheng: string; yunmus: seq[string]; done: bool] =
  ## 一次调入一个声母 · 配对所有韵母 · 声调逐调
  if s.shengmu.current_index >= SHENGMU.len:
    return ("", @[], true)
  
  let sheng = SHENGMU[s.shengmu.current_index]
  
  # 配对所有韵母
  var pairs: seq[string] = @[]
  for yun in YUNMU:
    for ton in TONES:
      let combo = sheng & yun & ton
      pairs.add(combo)
      s.output_stream.add(combo)
  
  # 记录训练关系
  for yun in YUNMU:
    if s.shengmu.symbols.hasKey(sheng):
      s.shengmu.symbols[sheng].trained_with.add(yun)
    if s.yunmu.symbols.hasKey(yun):
      s.yunmu.symbols[yun].trained_with.add(sheng)
  
  # 声母标记为训练中
  if s.shengmu.symbols.hasKey(sheng):
    s.shengmu.symbols[sheng].state = ssTraining
  
  s.step_count += 1
  return (sheng, pairs, false)

# ═══════════════════════════════════════════════════════════════
# 规则2: 折叠 · 符号用完不覆盖原层 · 折叠成新层
# ═══════════════════════════════════════════════════════════════
proc fold_shengmu*(s: var Scheduler; sheng: string) =
  ## 声母用完 · 折叠成新层 · 不覆盖原层
  if s.shengmu.symbols.hasKey(sheng):
    s.shengmu.symbols[sheng].state = ssFolded
    s.shengmu.symbols[sheng].fold_layer = s.fold_stack.len + 1
  
  # 这声母的所有配对进入折叠栈
  var fold_layer: seq[string] = @[]
  for entry in s.output_stream:
    if entry.startsWith(sheng):
      fold_layer.add(entry)
  s.fold_stack.add(fold_layer)
  s.shengmu.fold_count += 1

# ═══════════════════════════════════════════════════════════════
# 规则3: 减法倒计时 · 两容器混合用减法决定先后
# ═══════════════════════════════════════════════════════════════
proc subtract_order*(a_count: int; b_count: int): seq[string] =
  ## 减法倒计时: 比较两容器剩余量 · 多的先出 · 逐个递减
  var remaining_a = a_count
  var remaining_b = b_count
  var order: seq[string] = @[]
  
  while remaining_a > 0 or remaining_b > 0:
    if remaining_a >= remaining_b:
      if remaining_a > 0:
        order.add("A")
        remaining_a -= 1
      if remaining_b > 0:
        order.add("B")
        remaining_b -= 1
    else:
      if remaining_b > 0:
        order.add("B")
        remaining_b -= 1
      if remaining_a > 0:
        order.add("A")
        remaining_a -= 1
  
  return order

# ═══════════════════════════════════════════════════════════════
# 规则4: 边界到达后从下一个容器拿新东西
# ═══════════════════════════════════════════════════════════════
proc on_boundary_reached*(s: var Scheduler; exhausted_container: string): string =
  ## 边界到达处理: 当前容器用完 · 切换到下一个容器
  case exhausted_container:
  of "声母":
    # 声母用完 → 拿声调来做收尾组合
    if s.tone.current_index < TONES.len:
      let ton = TONES[s.tone.current_index]
      s.tone.current_index += 1
      return "切换到声调: " & ton
    else:
      return "所有容器完成 · 一轮结束"
  of "韵母":
    return "韵母完成 → 调下一个声母"
  of "声调":
    return "声调完成 → 开启新的一轮折叠"
  else:
    return "未知容器"

# ═══════════════════════════════════════════════════════════════
# 完整调度执行
# ═══════════════════════════════════════════════════════════════
proc execute_schedule*(s: var Scheduler) =
  echo "═══ 容器调度系统 · 声母调入 ═══"
  echo "声母: ", SHENGMU.len, "个"
  echo "韵母: ", YUNMU.len, "个"
  echo "声调: ", TONES.len, "个"
  echo ""
  
  # ── 减法倒计时决定容器优先顺序 ──
  let order = subtract_order(SHENGMU.len, YUNMU.len)
  echo "── 减法倒计时调度序 ──"
  echo "  声母(", SHENGMU.len, ") vs 韵母(", YUNMU.len, ") → ", order.join(" ")
  echo ""
  
  # ── 声母调入 · 一次一个 · 遍历所有韵母 ──
  echo "── 声母调入实况 ──"
  for i in 0..<SHENGMU.len:
    let sheng = SHENGMU[i]
    
    # 一次调入一个声母
    let (imported, pairs, done) = s.step_import_shengmu()
    if done: break
    
    echo "  第", i+1, "步: 调入声母 '", imported & "' → ", pairs.len, "个配对"
    
    # 声母遍历完 · 折叠
    s.fold_shengmu(sheng)
    
    # 检查边界
    if i == 2 or i == 5:  # 模拟边界到达
      let boundary_result = s.on_boundary_reached("韵母")
      echo "    △ 边界到达: ", boundary_result
    
    # 推进声母索引
    s.shengmu.current_index += 1
  
  echo ""

# ═══════════════════════════════════════════════════════════════
# 状态报告
# ═══════════════════════════════════════════════════════════════
proc report*(s: Scheduler) =
  echo "═══ 调度器状态报告 ═══"
  echo "调度步数: ", s.step_count
  echo "输出流长度: ", s.output_stream.len
  echo "折叠层数: ", s.fold_stack.len
  echo ""
  
  echo "── 折叠栈(每层符号) ──"
  for i, layer in s.fold_stack:
    echo "  层", i+1, ": ", layer.len, "个符号 [", layer[0], "...", layer[^1], "]"
  
  echo ""
  echo "── 声母状态 ──"
  let states = [ssIdle, ssTraining, ssFolded, ssCompleted]
  let state_names = ["待使用", "训练中", "已折叠", "已完成"]
  for i, st in states:
    var count = 0
    for _, entry in s.shengmu.symbols:
      if entry.state == st: count += 1
    echo "  ", alignLeft(state_names[i], 6), ": ", count
  
  echo ""
  echo "── 输出流(前20项) ──"
  for i in 0..<min(s.output_stream.len, 20):
    echo "  #", i, " ", s.output_stream[i]

# ═══════════════════════════════════════════════════════════════
# 产权核查: 确认每个符号的产权归属
# ═══════════════════════════════════════════════════════════════
func check_property_rights*(s: Scheduler): tuple[ok: bool; violations: seq[string]] =
  var violations: seq[string] = @[]
  
  for name, entry in s.shengmu.symbols:
    if entry.owner != "主人":
      violations.add("声母'" & name & "'产权异常: " & entry.owner)
  
  for name, entry in s.yunmu.symbols:
    if entry.owner != "主人":
      violations.add("韵母'" & name & "'产权异常: " & entry.owner)
  
  return (violations.len == 0, violations)

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var s = new_scheduler()
  
  # 执行调度
  execute_schedule(s)
  
  # 状态报告
  report(s)
  
  # 产权核查
  let (ok, violations) = check_property_rights(s)
  echo ""
  echo "── 产权核查 ──"
  if ok:
    echo "  ✓ 所有符号产权归主人"
  else:
    for v in violations:
      echo "  ✗ ", v
