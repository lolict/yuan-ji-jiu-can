# ============================================================
# 呼唤表 (Calling Table) + 四阶感知 (Four-Stage Perception)
# v0.16 容器关系动力学 · 代码骨架
# ============================================================

import std/[tables, times, strutils, json]

type
  Polarity* = enum
    polYang, polYin, polNeutral

  DiffMarker* = object
    timestamp*: float
    container_id*: uint64
    delta*: float
    pattern*: string

  CauseEffect* = object
    cause_id*: uint64
    effect_id*: uint64
    weight*: float
    polarity*: Polarity

  Container* = object
    id*: uint64
    label*: string
    polarity*: Polarity
    merit*: float

  PerceptionHistory* = object
    markers*: seq[DiffMarker]
    verdicts*: seq[string]

  Relation* = object
    caller*: uint64
    callee*: uint64
    created_at*: float
    feedback_count*: int
    ackable*: bool
    dissolved*: bool

  FeedbackKind* = enum
    fbDirect, fbCircuitous, fbOvershoot

  Feedback* = object
    origin*: uint64
    kind*: FeedbackKind
    payload*: float
    echo_pattern*: seq[float]

  FeedbackLoop* = object
    rounds*: int
    history*: seq[Feedback]

  CallFn* = proc(ctx: var Container, response: Container): Container {.closure.}

  CallEntry* = object
    index*: uint64
    path*: string
    logic*: CallFn

  CallingTable* = object
    entries*: Table[uint64, CallEntry]
    preload*: seq[string]

  PerceptionStage* = enum
    psReflect, psJudge, psEvaluate, psOrganize

  Perception* = object
    stage*: PerceptionStage
    history*: PerceptionHistory

proc new_calling_table*(): CallingTable =
  result.entries = init_table[uint64, CallEntry]()
  result.preload = @[]

proc register*(ct: var CallingTable, index: uint64, path: string, logic: CallFn) =
  ct.entries[index] = CallEntry(index: index, path: path, logic: logic)

proc call*(ct: var CallingTable, index: uint64,
           ctx: var Container, response: Container): Container =
  if index in ct.entries:
    let entry = ct.entries[index]
    entry.logic(ctx, response)
  else:
    Container(id: 0'u64, label: "reject", polarity: polNeutral, merit: 0.0)

proc push_feedback*(loop: var FeedbackLoop, fb: Feedback) =
  loop.history.add(fb)
  loop.rounds = loop.history.len

proc echo_shape*(loop: FeedbackLoop): seq[float] =
  if loop.history.len < 2:
    return @[]
  result = @[]
  for i in 1 ..< loop.history.len:
    let diff = loop.history[i].payload - loop.history[i-1].payload
    result.add(diff)

proc new_perception*(): Perception =
  result.stage = psReflect
  result.history = PerceptionHistory()

proc reflect*(p: var Perception, loop: FeedbackLoop): seq[DiffMarker] =
  p.stage = psReflect
  let shape = echo_shape(loop)
  result = @[]
  for i in 0 ..< shape.len:
    if abs(shape[i]) > 0.01:
      result.add(DiffMarker(
        timestamp: epoch_time(),
        container_id: uint64(i),
        delta: shape[i],
        pattern: if shape[i] > 0: "A型" else: "B型"
      ))
  for m in result:
    p.history.markers.add(m)

proc judge*(p: var Perception): seq[CauseEffect] =
  p.stage = psJudge
  result = @[]
  let markers = p.history.markers
  if markers.len >= 2:
    let cause = markers[markers.len - 2]
    let effect = markers[markers.len - 1]
    result.add(CauseEffect(
      cause_id: cause.container_id,
      effect_id: effect.container_id,
      weight: abs(cause.delta - effect.delta),
      polarity: if effect.delta > 0: polYang else: polYin
    ))

proc evaluate*(p: var Perception): float =
  p.stage = psEvaluate
  if p.history.verdicts.len == 0:
    result = 0.0
  else:
    result = float(p.history.markers.len) / float(p.history.verdicts.len)

proc organize*(p: var Perception, ct: CallingTable): seq[uint64] =
  p.stage = psOrganize
  let value = if p.history.markers.len > 0:
      float(p.history.markers.len) / 10.0
    else: 0.0
  if value > 0.5:
    result = @[0xF100'u64]
  elif value > 0.0:
    result = @[]
  else:
    result = @[0xF101'u64]

when is_main_module:
  echo "呼唤表 + 四阶感知 演示"
  echo "════════════════════════════════════════════════════════════"

  var ct = new_calling_table()

  var add_logic: CallFn = proc(ctx: var Container, response: Container): Container =
    ctx.merit += response.merit * 0.5
    if ctx.merit > 100.0: ctx.polarity = polYang
    result = ctx

  var sub_logic: CallFn = proc(ctx: var Container, response: Container): Container =
    ctx.merit -= response.merit * 0.3
    if ctx.merit < 0.0: ctx.polarity = polYin
    result = ctx

  ct.register(0xF100'u64, "SemAdd", add_logic)
  ct.register(0xF101'u64, "SemSub", sub_logic)

  var a = Container(id: 1'u64, label: "A", polarity: polYang, merit: 10.0)
  var b = Container(id: 2'u64, label: "B", polarity: polYin, merit: 5.0)

  echo "\n初始状态:"
  echo "  A: merit=", a.merit.format_float(ffDecimal, 2), " polarity=", a.polarity
  echo "  B: merit=", b.merit.format_float(ffDecimal, 2), " polarity=", b.polarity

  var loop: FeedbackLoop

  for i in 1..5:
    echo "\n━━━ 第 ", i, " 轮 ━━━"
    let response_b = Container(id: 2'u64, label: "B", polarity: polYin, merit: 5.0 + float(i))
    let result = ct.call(if i mod 2 == 1: 0xF100'u64 else: 0xF101'u64, a, response_b)
    echo "  A 呼唤 B → merit=", result.merit.format_float(ffDecimal, 2)

    var fb = Feedback(
      origin: b.id,
      kind: if i mod 3 == 0: fbCircuitous else: fbDirect,
      payload: b.merit + float(i) * 0.5
    )
    fb.echo_pattern = @[b.merit, b.merit * 0.5, b.merit * 0.25]
    loop.push_feedback(fb)

  let shape = echo_shape(loop)
  echo "\n涟漪形状:"
  for d in shape:
    echo "  Δ=", d.format_float(ffDecimal, 3)

  var per = new_perception()
  let diffs = per.reflect(loop)
  echo "\n阶段一 · 反射感知 → ", diffs.len, " 个差异"

  let verdicts = per.judge()
  echo "阶段二 · 判断之力 → ", verdicts.len, " 个因果对"

  let value = per.evaluate()
  echo "阶段三 · 评价之力 → 价值=", value.format_float(ffDecimal, 3)

  let new_strategy = per.organize(ct)
  echo "阶段四 · 逻辑组织 → 新呼唤=", new_strategy

  echo "\n最终感知阶段: ", per.stage
  echo "════════════════════════════════════════════════════════════"
