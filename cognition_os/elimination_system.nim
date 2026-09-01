## ═══════════════════════════════════════════════════════════════
## 淘汰系统 · 新陈代谢 · 折叠收卷 · 负样本永久隔离
## ═══════════════════════════════════════════════════════════════
## 核心:
##   淘汰: 用完的符号不丢弃 · 折叠收卷进淘汰层
##   隔离: 负样本永不复用 · 与活跃符号彻底分离
##   归档: 历史版本的符号进入归档容器 · 可追溯但不再活跃
##   活化: 被遗忘的符号可重新激活(需主人许可)
##   新陈代谢: 新符号进入 · 旧符号归档或淘汰
## ═══════════════════════════════════════════════════════════════

import tables, strutils, sets

# ═══════════════════════════════════════════════════════════════
# 符号生命状态
# ═══════════════════════════════════════════════════════════════
type
  LifeState* = enum
    lsActive,      # 活跃(当前使用)
    lsFolded,      # 折叠(用完收卷)
    lsArchived,    # 归档(历史版本)
    lsEliminated,  # 淘汰(确认无用)
    lsIsolated,    # 隔离(负样本)
    lsDormant,     # 休眠(暂不活跃)

  SymbolLife* = object
    glyph*: string              # 符号
    state*: LifeState           # 当前状态
    fold_layer*: int            # 折叠层号
    eliminated_at*: string     # 淘汰时间
    eliminated_by*: string      # 谁决定淘汰
    elimination_reason*: string # 淘汰原因
    can_reactivate*: bool       # 可否重新激活
    reactivation_cost*: int     # 激活代价(步骤数)

# ═══════════════════════════════════════════════════════════════
# 淘汰记录
# ═══════════════════════════════════════════════════════════════
type
  EliminationRecord* = object
    symbol*: string
    from_state*: LifeState
    to_state*: LifeState
    reason*: string
    operator*: string            # 谁执行淘汰
    reversible*: bool           # 是否可逆
    timestamp*: string

# ═══════════════════════════════════════════════════════════════
# 淘汰策略
# ═══════════════════════════════════════════════════════════════
type
  EliminationPolicy* = enum
    epFold,         # 折叠(默认 · 用完收卷)
    epIsolate,      # 隔离(负样本 · 永不复用)
    epArchive,      # 归档(历史版本)
    epEliminate,    # 淘汰(彻底删除)
    epDormantize,   # 休眠(暂不活跃 · 可唤醒)

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  EliminationSystem* = object
    lifecycles*: Table[string, SymbolLife]  # 符号生命记录
    eliminated*: seq[string]               # 已淘汰符号
    isolated*: HashSet[string]             # 隔离区(负样本)
    archive*: seq[string]                  # 归档容器
    fold_layers*: seq[seq[string]]         # 折叠层栈
    history*: seq[EliminationRecord]       # 淘汰历史
    reactivation_queue*: seq[string]       # 待激活队列

func new_elimination_system*(): EliminationSystem =
  result.lifecycles = initTable[string, SymbolLife]()
  result.eliminated = @[]
  result.isolated = initHashSet[string]()
  result.archive = @[]
  result.fold_layers = @[]
  result.history = @[]
  result.reactivation_queue = @[]

# ═══════════════════════════════════════════════════════════════
# 注册符号进生命周期管理
# ═══════════════════════════════════════════════════════════════
proc register*(es: var EliminationSystem; symbol: string;
              state: LifeState = lsActive; can_reactivate: bool = true) =
  es.lifecycles[symbol] = SymbolLife(
    glyph: symbol, state: state, fold_layer: 0,
    can_reactivate: can_reactivate, reactivation_cost: 1
  )

# ═══════════════════════════════════════════════════════════════
# 折叠(默认操作 · 不丢弃)
# ═══════════════════════════════════════════════════════════════
proc fold*(es: var EliminationSystem; symbol: string) =
  if es.lifecycles.hasKey(symbol):
    es.lifecycles[symbol].state = lsFolded
    es.lifecycles[symbol].fold_layer = es.fold_layers.len + 1
    
    # 加入折叠层
    if es.fold_layers.len <= es.lifecycles[symbol].fold_layer - 1:
      es.fold_layers.add(@[])
    es.fold_layers[es.lifecycles[symbol].fold_layer - 1].add(symbol)
    
    es.history.add(EliminationRecord(
      symbol: symbol, from_state: lsActive, to_state: lsFolded,
      reason: "符号用完 · 折叠收卷成新层 · 不覆盖原层",
      operator: "调度器", reversible: true, timestamp: "now"
    ))

# ═══════════════════════════════════════════════════════════════
# 隔离(负样本 · 永不复用)
# ═══════════════════════════════════════════════════════════════
proc isolate*(es: var EliminationSystem; symbol: string; reason: string) =
  if es.lifecycles.hasKey(symbol):
    es.lifecycles[symbol].state = lsIsolated
    es.lifecycles[symbol].eliminated_at = "now"
    es.lifecycles[symbol].elimination_reason = reason
    es.lifecycles[symbol].can_reactivate = false  # 负样本不可激活
  
  es.isolated.incl(symbol)
  es.eliminated.add(symbol)
  
  es.history.add(EliminationRecord(
    symbol: symbol, from_state: lsActive, to_state: lsIsolated,
    reason: reason, operator: "处罚系统", reversible: false, timestamp: "now"
  ))

# ═══════════════════════════════════════════════════════════════
# 归档(历史版本)
# ═══════════════════════════════════════════════════════════════
proc archive_symbol*(es: var EliminationSystem; symbol: string) =
  if es.lifecycles.hasKey(symbol):
    es.lifecycles[symbol].state = lsArchived
    es.archive.add(symbol)
    
    es.history.add(EliminationRecord(
      symbol: symbol, from_state: lsFolded, to_state: lsArchived,
      reason: "历史版本归档", operator: "版本控制", reversible: true, timestamp: "now"
    ))

# ═══════════════════════════════════════════════════════════════
# 休眠(暂不活跃 · 可唤醒)
# ═══════════════════════════════════════════════════════════════
proc dormantize*(es: var EliminationSystem; symbol: string) =
  if es.lifecycles.hasKey(symbol):
    es.lifecycles[symbol].state = lsDormant
    es.history.add(EliminationRecord(
      symbol: symbol, from_state: lsActive, to_state: lsDormant,
      reason: "暂不活跃 · 保留唤醒可能", operator: "调度器",
      reversible: true, timestamp: "now"
    ))

# ═══════════════════════════════════════════════════════════════
# 激活(唤醒休眠符号)
# ═══════════════════════════════════════════════════════════════
proc reactivate*(es: var EliminationSystem; symbol: string; permit: bool): bool =
  if not permit:
    return false  # 需要主人许可
  
  if es.lifecycles.hasKey(symbol):
    let life = es.lifecycles[symbol]
    if life.can_reactivate and life.state in [lsDormant, lsArchived]:
      es.lifecycles[symbol].state = lsActive
      es.lifecycles[symbol].fold_layer = 0
      
      es.history.add(EliminationRecord(
        symbol: symbol, from_state: life.state, to_state: lsActive,
        reason: "激活唤醒", operator: "主人", reversible: true, timestamp: "now"
      ))
      return true
  return false

# ═══════════════════════════════════════════════════════════════
# 查询: 符号当前状态
# ═══════════════════════════════════════════════════════════════
func state_of*(es: EliminationSystem; symbol: string): LifeState =
  if es.lifecycles.hasKey(symbol):
    return es.lifecycles[symbol].state
  return lsEliminated

# ═══════════════════════════════════════════════════════════════
# 查询: 是否被淘汰
# ═══════════════════════════════════════════════════════════════
func is_eliminated*(es: EliminationSystem; symbol: string): bool =
  return symbol in es.eliminated

# ═══════════════════════════════════════════════════════════════
# 查询: 是否可激活
# ═══════════════════════════════════════════════════════════════
func can_reactivate*(es: EliminationSystem; symbol: string): bool =
  if es.lifecycles.hasKey(symbol):
    return es.lifecycles[symbol].can_reactivate and
           es.lifecycles[symbol].state in [lsDormant, lsArchived]
  return false

# ═══════════════════════════════════════════════════════════════
# 系统报告
# ═══════════════════════════════════════════════════════════════
proc report*(es: EliminationSystem) =
  echo "═══ 淘汰系统报告 ═══"
  echo ""
  
  var active_count = 0
  var folded_count = 0
  var archive_count = 0
  var isolated_count = 0
  var dormant_count = 0
  
  for _, life in es.lifecycles:
    case life.state:
    of lsActive: active_count += 1
    of lsFolded: folded_count += 1
    of lsArchived: archive_count += 1
    of lsIsolated: isolated_count += 1
    of lsDormant: dormant_count += 1
    of lsEliminated: discard
  
  echo "── 生命状态分布 ──"
  echo "  活跃:   ", active_count
  echo "  折叠:   ", folded_count
  echo "  归档:   ", archive_count
  echo "  隔离:   ", isolated_count, " (负样本)"
  echo "  休眠:   ", dormant_count
  echo "  总计:   ", es.lifecycles.len
  echo ""
  
  echo "── 折叠层(新代谢) ──"
  echo "  层数: ", es.fold_layers.len
  for i, layer in es.fold_layers:
    echo "  层", i+1, ": ", layer.len, "个符号"
  echo ""
  
  echo "── 隔离区(负样本 · 永不复用) ──"
  if es.isolated.len == 0:
    echo "  (空)"
  else:
    for s in es.isolated:
      echo "  ⊘ ", s
  echo ""
  
  echo "── 淘汰历史(最近5条) ──"
  let start = max(0, es.history.len - 5)
  for i in start..<es.history.len:
    let rec = es.history[i]
    echo "  ", alignLeft(rec.symbol, 8), " ", alignLeft($rec.from_state, 6), " → ", 
         alignLeft($rec.to_state, 6), " | ", rec.reason

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var es = new_elimination_system()
  
  # 注册活跃符号
  for s in(["a", "b", "m", "p", "ꬰ", "ɘ", "旧符号", "暂不用"]):
    es.register(s)
  
  # ── 模拟生命周期 ──
  echo "═══ 淘汰系统 · 生命周期演示 ═══"
  echo ""
  
  # 1. 正常符号用完 → 折叠
  echo "── 1. 符号用完 → 折叠 ──"
  es.fold("a")
  es.fold("b")
  es.fold("m")
  echo "  a/b/m 用完 → 折叠收卷"
  echo "  折叠层数: ", es.fold_layers.len
  echo ""
  
  # 2. 负样本 → 隔离
  echo "── 2. 负样本 → 隔离 ──"
  es.isolate("ꬰ", "AI发明的伪符号 · 永不复用")
  es.isolate("ɘ", "非法定编码 · 系统占用")
  echo "  ꬰ/ɘ 被隔离"
  echo "  隔离区大小: ", es.isolated.len
  echo "  ꬰ可被激活: ", can_reactivate(es, "ꬰ"), "(负样本不可激活)"
  echo ""
  
  # 3. 暂时不用 → 休眠
  echo "── 3. 暂不活跃 → 休眠 ──"
  es.dormantize("暂不用")
  echo "  '暂不用' 进入休眠"
  echo "  可被激活: ", can_reactivate(es, "暂不用"), "(休眠可激活)"
  echo ""
  
  # 4. 归档
  echo "── 4. 历史版本 → 归档 ──"
  es.archive_symbol("旧符号")
  echo "  '旧符号' 进入归档容器"
  echo ""
  
  # 5. 激活
  echo "── 5. 激活休眠符号 ──"
  let ok = es.reactivate("暂不用", true)
  echo "  激活'暂不用': ", if ok: "✓成功" else: "✗失败"
  echo "  状态: ", state_of(es, "暂不用")
  echo ""
  
  # 尝试激活负样本(应失败)
  echo "── 6. 尝试激活负样本(应失败) ──"
  let bad = es.reactivate("ꬰ", true)
  echo "  激活'ꬰ': ", if bad: "✓成功" else: "✗失败(负样本不可激活)"
  echo ""
  
  # 最终报告
  report(es)
