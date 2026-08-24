## ═══════════════════════════════════════════════════════════════
## 全局链式传导总开关 · 容器互锁 · 永不遗忘 · 跨容器对接
## ═══════════════════════════════════════════════════════════════
## 核心规则:
##   1. 容器用完 → 不是重启 → 去下一个容器拿符号
##   2. 同时训练0维和1维宽度(单字符→双字符)
##   3. 按先后顺序排队 · 一次一个 · 倒计时训练
##   4. 历史符号永不丢弃 · 全部进全局记忆
##   5. 产权 = 多容器混合排序后的关系
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, sets

# ═══════════════════════════════════════════════════════════════
# 全局记忆库(永不遗忘 · 所有符号的历史)
# ═══════════════════════════════════════════════════════════════
type
  MemoryEntry* = object
    symbol*: string
    source*: string       # 来自哪个容器
    timestamp*: int       # 加入时间(顺序)
    used*: bool           # 是否被遍历过
    trained_with*: seq[string]  # 被哪些其他容器的符号训练过

  GlobalMemory* = object
    entries*: Table[string, MemoryEntry]
    order*: seq[string]         # 加入顺序(历史链条)
    container_order*: seq[string]  # 容器排队顺序

var global_memory = GlobalMemory()

proc init_global_memory*() =
  global_memory.entries = initTable[string, MemoryEntry]()
  global_memory.order = @[]
  global_memory.container_order = @[]

proc register_symbol*(symbol, source: string) =
  ## 注册符号进全局记忆 · 永不丢弃
  if not global_memory.entries.hasKey(symbol):
    global_memory.entries[symbol] = MemoryEntry(
      symbol: symbol,
      source: source,
      timestamp: global_memory.order.len,
      used: false,
      trained_with: @[],
    )
    global_memory.order.add(symbol)

proc mark_used*(symbol: string) =
  if global_memory.entries.hasKey(symbol):
    global_memory.entries[symbol].used = true

proc record_training*(target, trainer: string) =
  ## 记录: target被trainer训练过
  if global_memory.entries.hasKey(target):
    global_memory.entries[target].trained_with.add(trainer)

# ═══════════════════════════════════════════════════════════════
# 容器定义(每个容器独立 · 但通过全局记忆互锁)
# ═══════════════════════════════════════════════════════════════
type
  Container* = object
    name*: string
    symbols*: seq[string]
    current_index*: int      # 当前遍历到哪个符号
    exhausted*: bool         # 当前容器是否用完

func new_container*(name: string; symbols: seq[string]): Container =
  result.name = name
  result.symbols = symbols
  result.current_index = 0
  result.exhausted = false

func current_symbol*(c: Container): string =
  if c.current_index < c.symbols.len:
    return c.symbols[c.current_index]
  return ""

func advance*(c: var Container): bool =
  ## 前进到下一个符号 · 返回是否还有
  c.current_index += 1
  if c.current_index >= c.symbols.len:
    c.exhausted = true
    return false
  return true

# ═══════════════════════════════════════════════════════════════
# 全局开关: 容器调度器
# ═══════════════════════════════════════════════════════════════
type
  GlobalSwitch* = object
    containers*: seq[Container]
    current_container_idx*: int
    training_log*: seq[string]  # 训练历史(永不丢失)

func new_global_switch*(): GlobalSwitch =
  result.containers = @[]
  result.current_container_idx = 0
  result.training_log = @[]

proc add_switch_container*(sw: var GlobalSwitch; c: Container) =
  sw.containers.add(c)
  global_memory.container_order.add(c.name)
  # 注册所有符号进全局记忆
  for s in c.symbols:
    register_symbol(s, c.name)

func current_container*(sw: GlobalSwitch): Container =
  if sw.current_container_idx < sw.containers.len:
    return sw.containers[sw.current_container_idx]
  return Container()

func next_container*(sw: var GlobalSwitch): bool =
  ## 当前容器用完 · 切换到下一个容器
  sw.current_container_idx += 1
  if sw.current_container_idx >= sw.containers.len:
    return false
  return true

# ═══════════════════════════════════════════════════════════════
# 核心协议: 0维→1维训练(单字符→双字符宽度)
# ═══════════════════════════════════════════════════════════════
proc train_0d_to_1d*(sw: var GlobalSwitch; trainer_sym: string; target_sym: string) =
  ## 同时训练0维和1维:
  ##   0维 = 单字符宽度(起点)
  ##   1维 = 双字符宽度(终点)
  let log_entry = trainer_sym & " → " & target_sym & " (0维+1维训练)"
  sw.training_log.add(log_entry)
  record_training(target_sym, trainer_sym)
  mark_used(trainer_sym)
  mark_used(target_sym)

# ═══════════════════════════════════════════════════════════════
# 核心协议: 倒计时遍历训练
# ═══════════════════════════════════════════════════════════════
proc countdown_training*(sw: var GlobalSwitch) =
  ## 按先后顺序 · 一次一个 · 倒计时训练
  echo "═══ 全局开关: 倒计时训练 ═══"
  echo ""
  
  # 遍历所有容器对
  for i in 0..<sw.containers.len:
    for j in (i+1)..<sw.containers.len:
      let trainer_name = sw.containers[j].name
      let target_name = sw.containers[i].name
      
      echo "── 容器对: ", trainer_name, " → ", target_name, " ──"
      
      # 从trainer容器一个一个取符号
      for trainer_sym in sw.containers[j].symbols:
        # 训练target容器的所有符号(0维+1维)
        for target_sym in sw.containers[i].symbols:
          train_0d_to_1d(sw, trainer_sym, target_sym)
          echo "  ", trainer_sym, " 训练 ", target_sym
      
      echo ""

# ═══════════════════════════════════════════════════════════════
# 全局记忆报告
# ═══════════════════════════════════════════════════════════════
proc memory_report*() =
  echo "═══ 全局记忆报告 ═══"
  echo ""
  echo "── 符号历史(按加入顺序) ──"
  for i, sym in global_memory.order:
    let entry = global_memory.entries[sym]
    let status = if entry.used: "已遍历" else: "待遍历"
    echo "  #", i, " ", sym.alignLeft(6), " 来源:", entry.source.alignLeft(8), " ", status
  
  echo ""
  echo "── 容器排队顺序 ──"
  for i, name in global_memory.container_order:
    echo "  #", i, " ", name
  
  echo ""
  echo "── 训练关系 ──"
  for sym, entry in global_memory.entries:
    if entry.trained_with.len > 0:
      echo "  ", sym, " ← ", entry.trained_with.join(", ")

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  init_global_memory()
  var sw = new_global_switch()
  
  # 创建容器(按先后顺序排队)
  let yunmu_container = new_container("韵母", @[
    "a", "o", "e", "i", "u", "ü", "ai", "ei", "ao", "ou"
  ])
  let shengmu_container = new_container("声母", @[
    "b", "p", "m", "f", "d", "t", "n", "l", "g", "k"
  ])
  
  # 加入全局开关(自动注册所有符号到全局记忆)
  sw.add_switch_container(yunmu_container)
  sw.add_switch_container(shengmu_container)
  
  echo "── 初始状态 ──"
  echo "  容器1: ", sw.containers[0].name, " (", sw.containers[0].symbols.len, "符号)"
  echo "  容器2: ", sw.containers[1].name, " (", sw.containers[1].symbols.len, "符号)"
  echo ""
  
  # 执行倒计时训练
  countdown_training(sw)
  
  # 输出全局记忆
  memory_report()
