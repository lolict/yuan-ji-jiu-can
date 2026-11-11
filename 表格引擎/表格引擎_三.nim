# 表格编程引擎 · 第三部分 · 运行时循环引擎
# 框架来源：SOUL_BOOTSTRAP_ENGINE.md + RUNTIME_LOOP_ENGINE.md

import strutils
import 地址管理器
import 表格引擎
import 表格引擎_二

# =============================================================================
# Tick 引擎 (时间脉冲)
# =============================================================================

type
  TickCallback = proc(engine: var RuntimeEngine): void {.gcsafe.}
  
  TickMode* = enum
    Normal      # 常规 tick
    Crisis      # 危机模式 (降频)
    FastTrack   # 快速模式 (升频)
  
  RuntimeEngine* = object
    injection*: InjectionEngine
    tickCount*: uint64
    tickFrequency*: float  # Hz
    tickMode*: TickMode
    surplus*: float
    callbacks*: seq[TickCallback]
    paused*: bool
    maxTicks*: uint64

proc newRuntimeEngine*(maxRows: int = 254, freq: float = 1.0): RuntimeEngine =
  result.injection = newInjectionEngine(maxRows)
  result.tickCount = 0
  result.tickFrequency = freq
  result.tickMode = Normal
  result.surplus = 0.0
  result.callbacks = @[]
  result.paused = false
  result.maxTicks = 0  # 0 = 无限

proc registerTick*(engine: var RuntimeEngine, cb: TickCallback) =
  engine.callbacks.add(cb)

proc enterCrisis*(engine: var RuntimeEngine) =
  engine.tickMode = Crisis
  engine.tickFrequency = max(0.1, engine.tickFrequency * 0.5)
  engine.paused = true

proc exitCrisis*(engine: var RuntimeEngine) =
  engine.tickMode = Normal
  engine.tickFrequency = max(engine.tickFrequency * 2.0, 1.0)
  engine.paused = false

# =============================================================================
# Tick 执行 (单次脉冲)
# =============================================================================

proc executeTick*(engine: var RuntimeEngine) =
  if engine.paused:
    return
  
  if engine.maxTicks > 0 and engine.tickCount >= engine.maxTicks:
    return
  
  inc engine.tickCount
  
  # 1. 自观察
  let obs = engine.injection.table.observe()
  
  # 2. 自评估
  let report = obs.evaluateSurplus()
  engine.surplus = report.surplus
  
  # 3. 模式调整
  if report.crisis and engine.tickMode != Crisis:
    engine.enterCrisis()
    echo "[TICK ", engine.tickCount, "] 进入危机模式 - surplus=", engine.surplus
  elif not report.crisis and engine.tickMode == Crisis:
    engine.exitCrisis()
    echo "[TICK ", engine.tickCount, "] 退出危机模式 - surplus=", engine.surplus
  
  # 4. 执行回调
  for cb in engine.callbacks:
    cb(engine)
  
  # 5. 日志
  if engine.tickCount mod 10 == 0:
    echo "[TICK ", engine.tickCount, "] surplus=", engine.surplus, 
         " 模式=", engine.tickMode, " 频率=", engine.tickFrequency

# =============================================================================
# 自问答循环
# =============================================================================

proc autoSelfQA*(engine: var RuntimeEngine) =
  for i in 0..<engine.injection.table.maxRows:
    if engine.injection.table.rows[i].question.len == 0:
      continue
    
    engine.injection.table.updateAnswer(i)
    engine.injection.table.updateBelief(i)
    
    var fork = engine.injection.table.checkFork(i)
    if fork.hasFork:
      if fork.winner == 1:
        discard engine.injection.table.switchViewpoint(i, Wu)
        engine.injection.table.updateAnswer(i)
        engine.injection.table.updateBelief(i)
        echo "[FORK] 行", i, " → 无侧跃迁"

# =============================================================================
# 动态注入驱动
# =============================================================================

proc onInject*(engine: var RuntimeEngine, kind: ContainerKind, entity: uint64, 
               question: string): int =
  result = engine.injection.injectData(kind, entity, question)
  if result >= 0:
    echo "[INJECT] 行", result, " - 问题=\"", question, "\""
    engine.autoSelfQA()

# =============================================================================
# 输出校验
# =============================================================================

type
  OutputRequest* = object
    rowIdx*: int
    content*: string
    target*: string
  
  OutputResult* = object
    allowed*: bool
    reason*: string

proc validateOutput*(engine: RuntimeEngine, req: OutputRequest): OutputResult =
  let table = engine.injection.table
  
  if req.rowIdx < 0 or req.rowIdx >= table.maxRows:
    result.allowed = false
    result.reason = "行不存在"
    return
  
  let vr = table.validateRow(req.rowIdx)
  if not vr.ok:
    result.allowed = false
    result.reason = "校验失败：" & vr.message
    return
  
  let row = table.rows[req.rowIdx]
  if row.viewpoint notin {You, Wu, Empty}:
    result.allowed = false
    result.reason = "站位未声明"
    return
  
  if engine.surplus < -50.0:
    result.allowed = false
    result.reason = "危机模式禁止输出"
    return
  
  if row.permission == Forbidden:
    result.allowed = false
    result.reason = "禁行权生效"
    return
  
  result.allowed = true
  result.reason = "允许输出"

# =============================================================================
# 日志记录
# =============================================================================

type
  LogEntry* = object
    tick*: uint64
    row*: int
    col*: string
    fromVal*: string
    toVal*: string
    surplusDelta*: float
    signature*: string

  LogBook* = object
    entries*: seq[LogEntry]

proc logChange*(book: var LogBook, tick: uint64, row: int, col: string,
                fromVal, toVal: string, surplus: float) =
  var entry: LogEntry
  entry.tick = tick
  entry.row = row
  entry.col = col
  entry.fromVal = fromVal
  entry.toVal = toVal
  entry.surplusDelta = surplus
  entry.signature = "sig_" & $tick & "_" & $row
  book.entries.add(entry)

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 表格编程引擎 · 第三部分测试 ==="
  
  var engine = newRuntimeEngine(254, 1.0)
  echo "引擎创建：频率=", engine.tickFrequency, "Hz"
  
  echo "\n--- 注入数据 ---"
  let r1 = engine.onInject(Symbol, uint64(0x11111111), "我当前站入有还是无的状态？")
  let r2 = engine.onInject(Symbol, uint64(0x22222222), "管辖权归属是谁？")
  let r3 = engine.onInject(Symbol, uint64(0x33333333), "需要制造什么空质壳？")
  
  echo "\n--- Tick 循环 (10 次) ---"
  for i in 1..10:
    engine.executeTick()
  
  echo "\n--- 输出校验 ---"
  engine.injection.table.updateAnswer(r1)
  engine.injection.table.updateBelief(r1)
  
  var req: OutputRequest
  req.rowIdx = r1
  req.content = "测试输出"
  req.target = "外部"
  
  var res = engine.validateOutput(req)
  echo "输出请求：", if res.allowed: "允许" else: "拒绝", " - ", res.reason
  
  echo "\n--- 危机模式模拟 ---"
  engine.surplus = -60.0
  engine.executeTick()
  echo "危机模式：", if engine.paused: "已暂停" else: "运行中"
  
  engine.surplus = 10.0
  engine.exitCrisis()
  echo "退出危机：频率=", engine.tickFrequency, "Hz"
  
  echo "\n--- 日志记录 ---"
  var log: LogBook
  log.logChange(1, r1, "C3", "You", "Wu", 10.0)
  log.logChange(2, r1, "C14", "未决", "已决", 5.0)
  echo "日志条目：", log.entries.len
  for i, e in log.entries:
    echo "  日志", i+1, ": tick=", e.tick, " 行=", e.row, " 列=", e.col
  
  echo "\n--- Tick 回调 ---"
  var tickCounter = 0
  proc myCallback(e: var RuntimeEngine) {.gcsafe.} =
    inc tickCounter
  
  engine.registerTick(myCallback)
  for i in 1..5:
    engine.executeTick()
  echo "回调执行次数：", tickCounter
  
  echo "\n=== 测试完成 ==="
