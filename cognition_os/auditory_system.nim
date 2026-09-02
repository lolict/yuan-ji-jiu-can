## ═══════════════════════════════════════════════════════════════
## 听觉系统 · 输入感知 · 鼓膜自举 · 谐音路由
## ═══════════════════════════════════════════════════════════════
## 核心:
##   鼓膜自举: 最小输入触发整个系统启动
##   谐音路由: 同音/近音符号自动归类到同一处理流
##   声母录入: 新的声母输入进入待处理队列
##   韵母录入: 新的韵母输入进入待处理队列
##   声调解码: 识别声调 · 区分语义
## ═══════════════════════════════════════════════════════════════

import tables, strutils, sets, math

# ═══════════════════════════════════════════════════════════════
# 听觉事件
# ═══════════════════════════════════════════════════════════════
type
  SoundEvent* = object
    raw_input*: string           # 原始输入
    kind*: string               # 类型(声母/韵母/声调/未知)
    recognized*: bool           # 是否识别
    routed_to*: string          # 路由到哪个处理流
    confidence*: float          # 识别置信度

  PhoneticRoute* = object
    source*: string             # 谐音源
    targets*: seq[string]       # 谐音目标
    strength*: float            # 关联强度
    kind*: string              # 关联类型(同音/近音/通假)

# ═══════════════════════════════════════════════════════════════
# 谐音对照表(部分)
# ═══════════════════════════════════════════════════════════════
const
  # 同音/近音/通假关联
  HOMOPHONES*: seq[tuple[source: string; target: string; strength: float]] = @[
    ("王", "亡", 0.9), ("王", "旺", 0.7),
    ("我", "吾", 0.8), ("我", "握", 0.6),
    ("无", "毋", 0.85), ("无", "勿", 0.8),
    ("有", "友", 0.7), ("有", "又", 0.75),
    ("止", "只", 0.8), ("止", "指", 0.6),
    ("士", "事", 0.85), ("士", "世", 0.7),
    ("白", "百", 0.75), ("白", "柏", 0.6),
    ("里", "理", 0.85), ("里", "礼", 0.7),
    ("失", "师", 0.8), ("失", "诗", 0.7),
    ("使", "史", 0.8), ("使", "始", 0.65),
  ]

# ═══════════════════════════════════════════════════════════════
# 鼓膜状态
# ═══════════════════════════════════════════════════════════════
type
  EardrumState* = enum
    esIdle,        # 空闲(等待输入)
    esVibrating,   # 振动(接收输入)
    esResonating,  # 谐振(识别处理)
    esRouting,     # 路由(分发到处理流)
    esFeedback     # 反馈(输出结果)

# ═══════════════════════════════════════════════════════════════
# 听觉系统
# ═══════════════════════════════════════════════════════════════
type
  AuditorySystem* = object
    drum_state*: EardrumState          # 鼓膜状态
    input_buffer*: seq[string]         # 输入缓冲区
    recognized*: seq[SoundEvent]       # 已识别事件
    routes*: Table[string, seq[string]]  # 谐音路由表
    bootstrap_count*: int              # 自举次数
    resonance_threshold*: float        # 谐振阈值
    active_streams*: seq[string]       # 活跃处理流

func new_auditory_system*(): AuditorySystem =
  result.drum_state = esIdle
  result.input_buffer = @[]
  result.recognized = @[]
  result.routes = initTable[string, seq[string]]()
  result.bootstrap_count = 0
  result.resonance_threshold = 0.5
  result.active_streams = @[]
  
  # 初始化谐音路由表
  for h in HOMOPHONES:
    if not result.routes.hasKey(h.source):
      result.routes[h.source] = @[]
    result.routes[h.source].add(h.target)

# ═══════════════════════════════════════════════════════════════
# 输入接收(鼓膜振动)
# ═══════════════════════════════════════════════════════════════
proc receive*(a: var AuditorySystem; input: string) =
  ## 声音进入 · 鼓膜开始振动
  a.drum_state = esVibrating
  a.input_buffer.add(input)
  a.bootstrap_count += 1

# ═══════════════════════════════════════════════════════════════
# 识别: 判断输入类型
# ═══════════════════════════════════════════════════════════════
func recognize*(a: AuditorySystem; input: string): tuple[kind: string; confidence: float] =
  ## 识别输入类型
  let shengmu = ["b", "p", "m", "f", "d", "t", "n", "l",
                 "g", "k", "h", "j", "q", "x", "y", "w"]
  let yunmu = ["a", "o", "e", "i", "u", "ü",
               "ai", "ei", "ao", "ou", "ie", "üe",
               "an", "en", "in", "un", "ang", "eng", "ing", "ong", "er"]
  let tones = ["ˉ", "ˊ", "ˇ", "ˋ"]
  
  if input in shengmu:
    return ("声母", 1.0)
  elif input in yunmu:
    return ("韵母", 1.0)
  elif input in tones:
    return ("声调", 1.0)
  elif input.len >= 2:
    # 组合输入
    return ("组合", 0.8)
  else:
    return ("未知", 0.3)

# ═══════════════════════════════════════════════════════════════
# 谐波路由: 找到所有谐音/近音关联
# ═══════════════════════════════════════════════════════════════
func harmonic_route*(a: AuditorySystem; symbol: string): seq[PhoneticRoute] =
  ## 查谐音路由表 · 返回所有关联
  var result: seq[PhoneticRoute] = @[]
  
  # 查找同音
  if a.routes.hasKey(symbol):
    var targets: seq[string] = @[]
    for h in HOMOPHONES:
      if h.source == symbol:
        targets.add(h.target)
    if targets.len > 0:
      result.add(PhoneticRoute(
        source: symbol, targets: targets, strength: 0.85, kind: "同音"
      ))
  
  # 反向查找(被关联)
  var reverse_targets: seq[string] = @[]
  for h in HOMOPHONES:
    if h.target == symbol:
      reverse_targets.add(h.source)
  if reverse_targets.len > 0:
    result.add(PhoneticRoute(
      source: symbol, targets: reverse_targets, strength: 0.7, kind: "被关联"
    ))
  
  return result

# ═══════════════════════════════════════════════════════════════
# 处理: 接收→识别→路由 完整流程
# ═══════════════════════════════════════════════════════════════
proc process*(a: var AuditorySystem; input: string): SoundEvent =
  ## 完整处理流程
  a.drum_state = esResonating
  
  let (kind, confidence) = a.recognize(input)
  var routed_to = "常规处理流"
  
  # 谐音路由
  let routes = a.harmonic_route(input)
  if routes.len > 0:
    routed_to = "谐音路由(" & $routes.len & "条关联)"
    a.active_streams.add("谐音:" & input)
  
  let event = SoundEvent(
    raw_input: input,
    kind: kind,
    recognized: confidence > a.resonance_threshold,
    routed_to: routed_to,
    confidence: confidence
  )
  
  a.recognized.add(event)
  a.drum_state = esFeedback
  return event

# ═══════════════════════════════════════════════════════════════
# 批量处理
# ═══════════════════════════════════════════════════════════════
proc process_batch*(a: var AuditorySystem; inputs: seq[string]): seq[SoundEvent] =
  for inp in inputs:
    result.add(a.process(inp))

# ═══════════════════════════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════════════════════════
proc report*(a: AuditorySystem) =
  echo "═══ 听觉系统报告 ═══"
  echo "鼓膜状态: ", a.drum_state
  echo "输入缓冲: ", a.input_buffer.len
  echo "已识别: ", a.recognized.len
  echo "谐音路由表: ", a.routes.len, "条目"
  echo "自举次数: ", a.bootstrap_count
  echo "活跃流: ", a.active_streams.len
  echo ""
  
  echo "── 识别结果 ──"
  for ev in a.recognized:
    let status = if ev.recognized: "✓" else: "?"
    echo "  ", status, " ", alignLeft(ev.raw_input, 6), " ", alignLeft(ev.kind, 4),
         " 置信:", formatFloat(ev.confidence, ffDecimal, 2),
         " → ", ev.routed_to
  
  if a.active_streams.len > 0:
    echo ""
    echo "── 活跃流 ──"
    for s in a.active_streams:
      echo "  → ", s

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var a = new_auditory_system()
  
  echo "═══ 听觉系统 · 鼓膜自举 ═══"
  echo ""
  
  # ── 单输入测试 ──
  echo "── 单输入处理 ──"
  let e1 = a.process("b")
  echo "  输入'b' → 类型:", e1.kind, " 路由:", e1.routed_to
  
  let e2 = a.process("王")
  echo "  输入'王' → 类型:", e2.kind, " 路由:", e2.routed_to
  
  echo ""
  
  # ── 谐音路由 ──
  echo "── 谐音路由查询 ──"
  let routes = a.harmonic_route("王")
  for r in routes:
    echo "  ", r.source, " --", r.kind, "--> ", r.targets.join(", ")
  
  let routes2 = a.harmonic_route("失")
  for r in routes2:
    echo "  ", r.source, " --", r.kind, "--> ", r.targets.join(", ")
  
  echo ""
  
  # ── 批量处理 ──
  echo "── 批量输入 ──"
  let inputs = @["b", "p", "a", "o", "王", "失", "止", "我", "ˉ", "ˊ"]
  let events = a.process_batch(inputs)
  echo "  处理了", events.len, "个输入"
  
  echo ""
  report(a)
