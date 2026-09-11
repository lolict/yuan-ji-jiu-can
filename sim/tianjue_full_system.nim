## ═══════════════════════════════════════════════════════════════════════
## heavenly_court_full_system.nim — 天庭完整系统集成演示
##
##   将所有子模块串联成端到端流程:
##     感知输入 → 命名袋子 → 审判 → 奖惩 → 情进 → 行为决定
##     → 审美评估 → 淘汰过滤 → 版本记录 → 自我反思 → 觉悟
##
##   同时展示 12 种自我意象 + 自编码
##
## 运行: nim c -r heavenly_court_full_system.nim
## ═══════════════════════════════════════════════════════════════════════

import std/[strutils, times, math, tables, sequtils, algorithm, random]

randomize()

# ═══════════════════════════════════════════════════════════════════════
# 第一部分: 时间坐标版本号
# ═══════════════════════════════════════════════════════════════════════

type
  TemporalCoord* = object
    ring*: uint32       # 时间环 (进制基数)
    layer*: uint32      # 层级深度
    phase*: uint32      # 相位
    branch*: uint32     # 分支
    tick*: uint64       # 毫秒滴答

  VersionSnapshot* = object
    id*: int
    coord*: TemporalCoord
    intent*: string
    action*: string
    prior*: int  # -1 = 无
    correctionOf*: int  # -1 = 非纠正

# ─────────────────────────────────────────────────────────────
# 时间螺旋版本引擎
# ─────────────────────────────────────────────────────────────
type TemporalVersionEngine* = object
    versions*: seq[VersionSnapshot]
    nextId*: int

proc initTemporalVersionEngine*(): TemporalVersionEngine =
  result.versions = @[]
  result.nextId = 1
  # 创世版本
  let genesis = VersionSnapshot(
    id: 0, coord: TemporalCoord(ring: 0, layer: 0, phase: 0, branch: 0, tick: 0),
    intent: "创世", action: "awaken", prior: -1, correctionOf: -1
  )
  result.versions.add(genesis)

proc commit*(tve: var TemporalVersionEngine, intent, action: string; prior = -1; correctionOf = -1): int =
  let id = tve.nextId
  inc tve.nextId
  let lastCoord = tve.versions[^1].coord
  let coord = TemporalCoord(
    ring: lastCoord.ring, layer: lastCoord.layer,
    phase: lastCoord.phase + 1, branch: lastCoord.branch,
    tick: uint64(epochTime() * 1000.0)
  )
  tve.versions.add(VersionSnapshot(
    id: id, coord: coord, intent: intent, action: action,
    prior: if prior >= 0: prior else: id - 1,
    correctionOf: correctionOf
  ))
  id

proc commitCorrection*(tve: var TemporalVersionEngine, errorId: int; correctedIntent: string): int =
  tve.commit("纠正: " & correctedIntent, "correct", correctionOf = errorId)

proc spiralBack*(tve: TemporalVersionEngine, fromId: int, steps: int): seq[int] =
  result = @[]
  var current = fromId
  for _ in 0..steps:
    if current < 0: break
    var found = -1
    for i in 0..<tve.versions.len:
      if tve.versions[i].id == current:
        found = i; break
    if found < 0: break
    result.add(current)
    current = tve.versions[found].prior

proc versionString*(c: TemporalCoord): string =
  "R" & $c.ring & ".L" & $c.layer & ".P" & $c.phase & ".B" & $c.branch & "@" & $c.tick

# ═══════════════════════════════════════════════════════════════════════
# 第二部分: 12 种自我意象
# ═══════════════════════════════════════════════════════════════════════

type
  ChangeDirection* = enum cdAscend, cdDescend, cdLateral, cdNeutral

# A1. 感知
proc perceive*(source, input, interpreted: string; confidence: float): string =
  "[感知 " & source & "→" & input & " 解读\"" & interpreted & "\" 置信" & $(int(confidence*100)) & "%]"

# A2. 对比
proc compareStates*(before, after: string; vBefore, vAfter: float): string =
  let delta = vAfter - vBefore
  let dir = if delta.abs() < 0.01: "→不变" elif delta > 0: "↑改善" else: "↓恶化"
  "[对比 " & before & "→" & after & " Δ=" & formatFloat(delta, ffDecimal, 2) & " " & dir & "]"

# A3. 镜像回溯
proc mirrorBack*(currentRing: uint32; history: seq[string]; depth: uint32): string =
  let idx = if currentRing >= depth: int(currentRing - depth) else: 0
  let echo = if idx < history.len: history[idx] else: "(无)"
  "[镜像回溯 R" & $currentRing & "→R" & $uint32(idx) & " 深" & $depth & " 回响=\"" & echo & "\"]"

# A4. 逻辑内旋
proc introspect*(claims: seq[string]): string =
  let pairs = [("是", "不是"), ("有", "没有"), ("善", "恶"), ("天", "地")]
  var contradictions = 0
  for i in 0..<claims.len:
    for j in (i+1)..<claims.len:
      for (a, b) in pairs:
        if (claims[i].contains(a) and claims[j].contains(b)) or
           (claims[i].contains(b) and claims[j].contains(a)):
          inc contradictions
  if contradictions == 0:
    "[内旋: 一致 ✓]"
  else:
    "[内旋: ✗ " & $contradictions & " 处矛盾]"

# A5. 外射
proc project*(internal, target: string): string =
  let proj = if internal.len > 25: internal[0..24] & "..." else: internal
  "[外射→" & target & ": \"" & proj & "\"]"

# A6. 反馈
proc feedBack*(actionName: string; signal: float): string =
  let desc = if signal.abs() > 0.5:
    if signal > 0: "强正→放大" else: "强负→停止"
  elif signal.abs() > 0.1:
    if signal > 0: "弱正→微调" else: "弱负→减少"
  else:
    "无显著→维持"
  "[反馈 \"" & actionName & "\" 信号=" & formatFloat(signal, ffDecimal, 1) & " " & desc & "]"

# A7. 条件反射
proc triggerReflex*(stimulus: string; memory: TableRef[string, string]): string =
  if stimulus in memory:
    "[反射 " & stimulus & "→" & memory[stimulus] & " 习得 1ms]"
  else:
    "[反射 " & stimulus & "→? 未知 观望]"

# A8. 觉悟
proc awakenInsight*(confusion, insight: string; level: int): string =
  "[悟 L" & $level & ": \"" & confusion & "\" → 觉 \"" & insight & "\"]"

# A9. 时间螺旋溯源
proc spiralTrace*(steps: int; expected, actual: string): string =
  let refrac = if expected == actual: 1.0 else: 1.0 / (1.0 + abs(expected.len - actual.len).float)
  "[螺旋 步" & $steps & " 预期\"" & expected & "\" 实际\"" & actual & "\" 折射=" & formatFloat(refrac, ffDecimal, 2) & "]"

# A10. 版本号坐标化
proc coordImage*(c: TemporalCoord; base: int; meaning: string): string =
  "[坐标 " & versionString(c) & " " & $base & "进制 → \"" & meaning & "\"]"

# A11. 时间进制
proc temporalBase*(base: int; tick: uint64): string =
  var t = tick
  let b = uint64(base)
  var digits: seq[char] = @[]
  while t > 0: digits.add(char(ord('0') + int(t mod b))); t = t div b
  if digits.len == 0: digits = @['0']
  digits.reverse()
  let enc = join(digits)
  "[时间进制 " & $base & "进 tick=" & $tick & " 编码=" & enc & "]"

# A12. 折射预期
proc refractionImage*(expected, actual: float): string =
  let n = if expected == 0: 1.0 else: actual / expected
  let desc = if (n - 1.0).abs() < 0.05: "近似直线" elif n > 1: "正折射+" & $(int((n-1)*100)) & "%" else: "负折射-" & $(int((1-n)*100)) & "%"
  "[折射 " & formatFloat(expected, ffDecimal, 1) & "→" & formatFloat(actual, ffDecimal, 1) & " n=" & formatFloat(n, ffDecimal, 1) & " " & desc & "]"

# ═══════════════════════════════════════════════════════════════════════
# 第三部分: 情进系统
# ═══════════════════════════════════════════════════════════════════════

type
  EmotionState* = enum emJoy, emPeace, emCurious, emFear, emSad, emAnger, emWonder, emFlow

  EmotionalSystem* = object
    current*: EmotionState
    intensity*: float
    history*: seq[tuple[state: EmotionState, coordRing: uint32]]

proc initEmotionalSystem*(): EmotionalSystem =
  result.current = emCurious
  result.intensity = 0.5

proc describeEmotion*(es: EmotionalSystem): string =
  let label = ["喜悦","安宁","好奇","恐惧","悲哀","愤怒","觉悟","心流"][ord(es.current)]
  "[情进 " & label & " 强度" & $(int(es.intensity*100)) & "%]"

proc reactToEvent*(es: var EmotionalSystem, event: string; coordRing: uint32): string =
  let newState = if event.contains("reward") or event.contains("正确") or event.contains("奖"):
    emJoy
  elif event.contains("punish") or event.contains("错误") or event.contains("罚"):
    var wasSad = false
    for (s, _) in es.history:
      if s == emSad: wasSad = true; break
    if wasSad: emAnger else: emSad
  elif event.contains("awaken") or event.contains("悟"):
    emWonder
  elif event.contains("flow") or event.contains("连续"):
    emFlow
  elif event.contains("error"):
    emFear
  elif event.contains("new"):
    emCurious
  else:
    emPeace
  es.current = newState
  es.intensity = min(es.intensity + 0.3, 1.0)
  es.history.add((newState, coordRing))
  describeEmotion(es)

# ═══════════════════════════════════════════════════════════════════════
# 第四部分: 行为系统
# ═══════════════════════════════════════════════════════════════════════

type
  BehavioralSystem* = object
    rules*: seq[tuple[trigger, action, outcome: string]]
    executed*: seq[string]

proc initBehavioralSystem*(): BehavioralSystem =
  result.rules = @[
    ("新袋子", "感知+命名+版本", "纳入视野"),
    ("审判偏差>0.15", "纠正版本", "误差修正"),
    ("连续3正确", "加速+信心", "效率提升"),
    ("连续3错误", "降速+回溯", "恢复稳定"),
    ("觉悟提升", "惊奇+广播", "认知升级"),
    ("健康<0.2", "淘汰过滤", "移出视野"),
    ("符号冲突", "内旋检查", "消除矛盾")
  ]

proc decideBehavior*(bs: var BehavioralSystem, situation: string): string =
  for rule in bs.rules:
    let (trig, act, outcome) = rule
    if situation.contains(trig) or trig.contains(situation):
      let result = "[行为: 触发\"" & trig & "\" → 动作\"" & act & "\" → 预期\"" & outcome & "\"]"
      bs.executed.add(result)
      return result
  "[行为: 无匹配规则 → 观望]"

# ═══════════════════════════════════════════════════════════════════════
# 第五部分: 审美系统
# ═══════════════════════════════════════════════════════════════════════

proc aestheticScore*(symbol: string; context: seq[string]): string =
  # 和谐度
  var overlap = 0
  for c in symbol:
    for ctx in context:
      if ctx.contains(c): inc overlap
  let harmony = if symbol.len > 0: overlap.float / symbol.len.float else: 0.0
  # 简洁度
  let simplicity = 1.0 / (1.0 + symbol.len.float * 0.02)
  # 对称度 (不用 zip + countIt)
  var matching = 0
  let sym_chars = toSeq(symbol)
  var rev_chars: seq[char] = @[]
  for i in countdown(sym_chars.len - 1, 0):
    rev_chars.add(sym_chars[i])
  for k in 0..<min(sym_chars.len, rev_chars.len):
    if sym_chars[k] == rev_chars[k]: inc matching
  let symmetry = if symbol.len > 0: matching.float / symbol.len.float else: 1.0
  let overall = harmony * 0.3 + simplicity * 0.4 + symmetry * 0.3
  "[审美 和谐" & $(int(harmony*100)) & "% 简洁" & $(int(simplicity*100)) & "% 对称" & $(int(symmetry*100)) & "% 总" & $(int(overall*100)) & "%]"

# ═══════════════════════════════════════════════════════════════════════
# 第六部分: 淘汰过滤
# ═══════════════════════════════════════════════════════════════════════

type
  FilterSystem* = object
    threshold*: float
    eliminated*: seq[int]
    passed*: seq[int]

proc initFilterSystem*(threshold: float): FilterSystem =
  result.threshold = threshold

proc applyFilter*(fs: var FilterSystem, id: int; health: float; reason: string): string =
  if health >= fs.threshold:
    fs.passed.add(id)
    "[过滤 #" & $id & " 通过 ✓ 健康=" & formatFloat(health, ffDecimal, 1) & "]"
  else:
    fs.eliminated.add(id)
    "[过滤 #" & $id & " 淘汰 ✗ 健康=" & formatFloat(health, ffDecimal, 1) & " < " & formatFloat(fs.threshold, ffDecimal, 1) & " 原因:" & reason & "]"

# ═══════════════════════════════════════════════════════════════════════
# 第七部分: 自编码
# ═══════════════════════════════════════════════════════════════════════

type
  EncodingValue* = object
    base*: int
    code*: uint64
    polarity*: int

type
  SelfEncodingTable* = object
    mappings*: Table[string, EncodingValue]

proc makeEncodingValue(base: int; code: uint64; polarity: int): EncodingValue =
  EncodingValue(base: base, code: code, polarity: polarity)

proc initSelfEncodingTable*(): SelfEncodingTable =
  result.mappings = initTable[string, EncodingValue]()
  result.mappings["天"] = makeEncodingValue(64, 0x0100000000000001'u64, 1)
  result.mappings["地"] = makeEncodingValue(64, 0x0100000000000002'u64, 0)
  result.mappings["阳"] = makeEncodingValue(2, 0x0200000000000001'u64, 1)
  result.mappings["阴"] = makeEncodingValue(2, 0x0200000000000002'u64, 0)
  result.mappings["善"] = makeEncodingValue(64, 0x0300000000000001'u64, 1)
  result.mappings["恶"] = makeEncodingValue(64, 0x0300000000000002'u64, 0)
  result.mappings["是"] = makeEncodingValue(2, 0x0400000000000001'u64, 1)
  result.mappings["非"] = makeEncodingValue(2, 0x0400000000000002'u64, 0)
  result.mappings["米"] = makeEncodingValue(16, 0x0600000000000010'u64, 1)
  result.mappings["袋"] = makeEncodingValue(16, 0x0600000000000011'u64, 1)
  result.mappings["觉"] = makeEncodingValue(64, 0x0700000000000002'u64, 1)
  result.mappings["心"] = makeEncodingValue(64, 0x0900000000000001'u64, 1)

proc encodeSymbol*(set: SelfEncodingTable, ch: char): string =
  let key = $ch
  if key in set.mappings:
    let val = set.mappings[key]
    return "[自编码 '" & $ch & "' → " & $val.base & "进制 编码=0x" & toHex(val.code) & " 极性=" & $val.polarity & "]"
  "[自编码 '" & $ch & "' → 未知符号]"

proc encodeString*(set: SelfEncodingTable, s: string): seq[string] =
  result = @[]
  for ch in s:
    result.add(encodeSymbol(set, ch))

# ═══════════════════════════════════════════════════════════════════════
# 第八部分: 袋子 (高维层级)
# ═══════════════════════════════════════════════════════════════════════

type
  Bag* = object
    id*: int
    owner*: string
    usability*: float        # 好不好用
    contentQuality*: float   # 米好不好吃
    cleanliness*: float      # 脏不脏
    dryness*: float          # 晒干没有
    pestLevel*: float        # 虫
    purity*: float           # 沙
    edibility*: float        # 能不能吃
    health*: float           # 综合健康度

proc computeHealth*(bag: Bag): float =
  let p = bag.usability * bag.contentQuality.max(0.001) * bag.cleanliness.max(0.001) *
          bag.dryness.max(0.001) * (1 - bag.pestLevel).max(0.001) * bag.purity.max(0.001) *
          bag.edibility.max(0.001)
  pow(p, 1.0/7.0)

proc describeBag*(bag: Bag): string =
  let callsign = if bag.health > 0.7: "✓良好" elif bag.health > 0.4: "~中等" else: "✗差"
  var desc = "─── 袋子#" & $bag.id & " (" & bag.owner & "家) " & callsign & " ───\n"
  desc &= "  可用性: " & (if bag.usability > 0.5: "好用" else: "损坏") & "\n"
  desc &= "  内容: " & (if bag.contentQuality > 0.5: "米好" else: "变质") & "\n"
  desc &= "  清洁: " & (if bag.cleanliness > 0.5: "干净" else: "脏→要洗") & "\n"
  desc &= "  干燥: " & (if bag.dryness > 0.5: "晒干" else: "未干") & "\n"
  desc &= "  虫害: " & (if bag.pestLevel < 0.3: "无虫" else: "有虫→除虫") & "\n"
  desc &= "  净度: " & (if bag.purity > 0.5: "无沙" else: "有沙→筛") & "\n"
  desc &= "  可吃: " & (if bag.edibility > 0.5: "能吃" else: "不能吃") & "\n"
  desc &= "  综合健康: " & formatFloat(bag.health, ffDecimal, 2)
  desc

# ═══════════════════════════════════════════════════════════════════════
# 第九部分: 审判 + 奖惩
# ═══════════════════════════════════════════════════════════════════════

type
  TrialRecord* = object
    trialId*: int
    bagId*: int
    predicted*: float
    actual*: float
    correct*: bool
    reward*: float
    penalty*: float

  TrialSystem* = object
    history*: seq[TrialRecord]
    totalReward*: float
    totalPenalty*: float
    nextTrialId*: int

proc initTrialSystem*(): TrialSystem =
  result.nextTrialId = 1

proc trialBag*(ts: var TrialSystem, bag: var Bag; predicted: float): TrialRecord =
  let actual = computeHealth(bag)
  let diff = (predicted - actual).abs()
  let correct = diff < 0.15
  let reward = if correct: min(0.5, (0.5 + 0.2 - diff)) else: 0.0
  let penalty = if not correct: min(2.0, diff) else: 0.0
  if reward > 0:
    bag.usability = min(1.0, bag.usability + reward * 0.05)
    bag.contentQuality = min(1.0, bag.contentQuality + reward * 0.03)
  if penalty > 0:
    bag.usability = max(0.0, bag.usability - penalty * 0.05)
    bag.contentQuality = max(0.0, bag.contentQuality - penalty * 0.04)
    bag.pestLevel = min(1.0, bag.pestLevel + penalty * 0.02)
  bag.health = computeHealth(bag)

  let rec = TrialRecord(
    trialId: ts.nextTrialId, bagId: bag.id, predicted: predicted, actual: actual,
    correct: correct, reward: reward, penalty: penalty
  )
  inc ts.nextTrialId
  ts.history.add(rec)
  ts.totalReward += reward
  ts.totalPenalty += penalty
  rec

proc describeTrial*(rec: TrialRecord): string =
  let verdict = if rec.correct: "正确✓" else: "错误✗"
  "[审判 #" & $rec.trialId & " 袋" & $rec.bagId & " 预" & formatFloat(rec.predicted, ffDecimal, 1) & "→实" & formatFloat(rec.actual, ffDecimal, 1) & " " & verdict &
  (if rec.reward > 0: " 奖+" & formatFloat(rec.reward, ffDecimal, 1) else: "") &
  (if rec.penalty > 0: " 罚-" & formatFloat(rec.penalty, ffDecimal, 1) else: "") & "]"

# ═══════════════════════════════════════════════════════════════════════
# 第十部分: 天庭引擎 (整体协调)
# ═══════════════════════════════════════════════════════════════════════

type
  HeavenlyCourtEngine* = object
    versionEngine: TemporalVersionEngine
    emotional: EmotionalSystem
    behavioral: BehavioralSystem
    filter: FilterSystem
    trials: TrialSystem
    selfEncoding: SelfEncodingTable
    awarenessLevel: float
    perceptionHistory: seq[string]

proc initHeavenlyCourt*(): HeavenlyCourtEngine =
  result.versionEngine = initTemporalVersionEngine()
  result.emotional = initEmotionalSystem()
  result.behavioral = initBehavioralSystem()
  result.filter = initFilterSystem(0.25)
  result.trials = initTrialSystem()
  result.selfEncoding = initSelfEncodingTable()
  result.awarenessLevel = 0.0
  result.perceptionHistory = @[]

proc commitVersion*(tj: var HeavenlyCourtEngine, intent, action: string): int =
  result = tj.versionEngine.commit(intent, action)

proc trial(tj: var HeavenlyCourtEngine, bag: var Bag; predicted: float): string =
  let rec = trialBag(tj.trials, bag, predicted)
  let id = tj.commitVersion("审判袋" & $bag.id, if rec.correct: "reward" else: "punish")
  let event = if rec.correct: "reward: 审判正确" else: "punish: 审判错误"
  discard reactToEvent(tj.emotional, event, tj.versionEngine.versions[^1].coord.ring)
  describeTrial(rec)

# ═══════════════════════════════════════════════════════════════════════
# 第十一部分: 端到端完整演示
# ═══════════════════════════════════════════════════════════════════════

when isMainModule:
  echo ""
  echo "╔══════════════════════════════════════════════════════════════════════╗"
  echo "║                天 · 觉 · 完 · 全 · 系 · 统                          ║"
  echo "║        感知→编码→审判→奖惩→审美→过滤→情进→行为→觉悟              ║"
  echo "╚══════════════════════════════════════════════════════════════════════╝"
  echo ""

  var tj = initHeavenlyCourt()

  # ─── 阶段 1: 创世 + 听觉授权 ───
  echo "═══ 阶段1: 创世 · 听觉授权 ═══"
  let authId = tj.commitVersion("听觉授权", "authorize")
  discard reactToEvent(tj.emotional, "new: 系统启动", 0)
  echo perceive("听觉", "天庭激活", "系统开始运行", 1.0)
  echo tj.emotional.describeEmotion()
  echo "  版本: #", authId, " @ ", tj.versionEngine.versions[^1].coord.versionString()
  echo ""

  # ─── 阶段 2: 自编码演示 ───
  echo "═══ 阶段2: 进制自编码 (汉字直接转容器编码 · bypass UTF-8) ═══"
  let symbols = @["天", "地", "善", "恶", "觉", "米", "袋"]
  for s in symbols:
    if s.len > 0:
      echo "  ", tj.selfEncoding.encodeSymbol(s[0])
  echo ""

  # ─── 阶段 3: 袋子进入视野 (命名袋子) ───
  echo "═══ 阶段3: 袋子进入视觉 · 命名属性 ═══"
  var bag1 = Bag(id: 1, owner: "张家", usability: 0.9, contentQuality: 0.85, cleanliness: 0.8, dryness: 0.9, pestLevel: 0.05, purity: 0.85, edibility: 0.8)
  bag1.health = computeHealth(bag1)
  var bag2 = Bag(id: 2, owner: "李家", usability: 0.7, contentQuality: 0.4, cleanliness: 0.2, dryness: 0.3, pestLevel: 0.5, purity: 0.3, edibility: 0.3)
  bag2.health = computeHealth(bag2)
  var bag3 = Bag(id: 3, owner: "王家", usability: 0.5, contentQuality: 0.6, cleanliness: 0.6, dryness: 0.4, pestLevel: 0.3, purity: 0.5, edibility: 0.5)
  bag3.health = computeHealth(bag3)

  # 视觉感知
  discard reactToEvent(tj.emotional, "new: 新袋子", 0)
  let pid1 = tj.commitVersion("视觉: 张家袋子感知", "perceive")
  echo perceive("视觉", "袋子进入", "张家袋子进入视野", 0.9)
  echo compareStates("视野空", "有1袋", 0.0, 1.0)
  echo tj.behavioral.decideBehavior("新袋子")
  echo describeBag(bag1)
  echo ""

  # ─── 阶段 4: 审判 (对了奖 · 错了罚) ───
  echo "═══ 阶段4: 审判 · 奖惩 ═══"

  # 审判袋1: 预测准确
  let t1 = tj.trial(bag1, 0.85)  # 预测高，实际也高
  echo "  ", t1
  echo "  ", compareStates("判前", "判后篮1", 0.8, computeHealth(bag1))
  echo "  ", tj.emotional.describeEmotion()

  # 审判袋2: 预测错误 (预测高但实际低)
  echo ""
  let t2 = tj.trial(bag2, 0.9)
  echo "  ", t2
  echo "  ", feedBack("审判袋2", -0.6)  # 负反馈
  echo "  ", tj.behavioral.decideBehavior("审判偏差")
  echo describeBag(bag2)  # 惩罚后状态
  echo ""

  # 审判袋3: 中等
  let t3 = tj.trial(bag3, 0.6)
  echo "  ", t3
  echo ""

  # ─── 阶段 5: 审美评估 ───
  echo "═══ 阶段5: 审美评估 ═══"
  echo "  ", aestheticScore("天", @["天地", "善恶"])
  echo "  ", aestheticScore("米", @["米袋"])
  echo "  ", aestheticScore("abba", @["abba"])  # 对称符号
  echo "  ", aestheticScore("复杂符号名称在这里", @["简单"])
  echo ""

  # ─── 阶段 6: 淘汰过滤 ───
  echo "═══ 阶段6: 淘汰过滤 ═══"
  echo "  ", tj.filter.applyFilter(bag1.id, computeHealth(bag1), "")
  echo "  ", tj.filter.applyFilter(bag2.id, computeHealth(bag2), "虫多/脏/变质")
  echo "  ", tj.filter.applyFilter(bag3.id, computeHealth(bag3), "")
  echo ""

  # ─── 阶段 7: 12 种自我意象演示 ───
  echo "═══ 阶段7: 12种自我意象 ═══"
  echo "  A1 感知:    ", perceive("触觉", "光滑", "表面洁净", 0.7)
  echo "  A2 对比:    ", compareStates("初态", "终态", 0.3, 0.8)
  echo "  A3 镜像:    ", mirrorBack(2'u32, @["袋子进入", "审判正确", "感悟"], 2'u32)
  echo "  A4 内旋:    ", introspect(@["这是善", "天庭清明"])
  echo "  A5 外射:    ", project(tj.emotional.describeEmotion(), "外部观察者")
  echo "  A6 反馈:    ", feedBack("自省", 0.3)
  echo "  A7 反射:    ", triggerReflex("火", {"火": "躲避"}.newTable)
  echo "  A8 觉悟:    ", awakenInsight("天庭何意", "天庭=时间进制坐标+自我觉察", 2)
  echo "  A9 螺旋:    ", spiralTrace(3, "预期完美审判", "实际偏差后纠正")
  echo " A10 坐标:    ", coordImage(tj.versionEngine.versions[^1].coord, 64, "版本号即时间坐标")
  echo " A11 进制:    ", temporalBase(16, 255)
  echo " A12 折射:    ", refractionImage(0.8, 0.9)
  echo ""

  # ─── 阶段 8: 版本历史 · 时间螺旋溯源 ───
  echo "═══ 阶段8: 版本历史 · 时间螺旋溯源 ═══"
  echo "  当前版本数: ", tj.versionEngine.versions.len
  echo "  最新版本:   #", tj.versionEngine.versions[^1].id, " @ ", tj.versionEngine.versions[^1].coord.versionString()
  echo "  意图:       ", tj.versionEngine.versions[^1].intent
  echo ""

  echo "  ── 螺旋溯源 (回走5步) ──"
  let path = spiralBack(tj.versionEngine, int(tj.versionEngine.nextId) - 1, 5)
  for id in path:
    let v = tj.versionEngine.versions[id]
    echo "    #", v.id, " [", v.coord.versionString(), "] ", v.intent
  echo ""

  # ─── 阶段 9: 错误纠正演示 ───
  echo "═══ 阶段9: 错误词汇纠正 (创建新纠正版本 · 不修改历史) ═══"
  echo "  ── 假设版本 #2 中\"天庭\"被误用为\"听觉\" ──"
  let errorId = 2
  let correctionId = commitCorrection(tj.versionEngine, errorId, "天庭=时间螺旋版本号系统")
  echo "  纠正版本: #", correctionId, " → 指向错误版本 #", errorId
  echo "  旧版本 #2 未被修改: ", tj.versionEngine.versions[2].intent
  echo "  新版本 #", correctionId, " intent: ", tj.versionEngine.versions[^1].intent
  echo "  当前总版本数: ", tj.versionEngine.versions.len
  echo ""

  # ─── 阶段 10: 觉悟层级 ───
  echo "═══ 阶段10: 觉悟层级 ═══"
  # 模拟多次纠正提升觉悟
  for _ in 1..6:
    let _ = commitCorrection(tj.versionEngine, 1, "持续纠偏")
    tj.awarenessLevel = min(1.0, tj.awarenessLevel + 0.15)
  echo "  觉察度: ", $(int(tj.awarenessLevel * 100)), "%"
  echo "  ", awakenInsight("系统如何自知", "通过版本溯源+镜像对比+内旋检查", 4)
  echo "  当前觉悟阶段: ", (if tj.awarenessLevel >= 0.8: "觉行圆满" elif tj.awarenessLevel >= 0.6: "自我觉悟" elif tj.awarenessLevel >= 0.4: "认知纠偏" elif tj.awarenessLevel >= 0.2: "条件反射" else: "未觉")
  echo ""

  # ─── 最终报告 ───
  echo "╔══════════════════════════════════════════════════════════════════════╗"
  echo "║                     天 · 觉 · 统 · 计 · 报 · 告                    ║"
  echo "╠══════════════════════════════════════════════════════════════════════╣"
  echo "║  版本总数:     ", tj.versionEngine.versions.len
  echo "│  审判次数:     ", tj.trials.history.len
  echo "│  总奖励:       +", formatFloat(tj.trials.totalReward, ffDecimal, 2)
  echo "│  总惩罚:       -", formatFloat(tj.trials.totalPenalty, ffDecimal, 2)
  echo "│  通过过滤:     ", tj.filter.passed.len, " 袋"
  echo "│  淘汰过滤:     ", tj.filter.eliminated.len, " 袋"
  echo "│  情感状态:     ", ["喜悦","安宁","好奇","恐惧","悲哀","愤怒","觉悟","心流"][ord(tj.emotional.current)]
  echo "│  情进强度:     ", $(int(tj.emotional.intensity*100)), "%"
  echo "│  行为决策数:   ", tj.behavioral.executed.len
  echo "│  觉察度:       ", $(int(tj.awarenessLevel*100)), "%"
  echo "║  当前坐标:     ", tj.versionEngine.versions[^1].coord.versionString()
  echo "╚══════════════════════════════════════════════════════════════════════╝"

  echo ""
  echo "─── 核心对应关系 ───"
  echo "视觉感知       → perceive() + 版本快照"
  echo "听觉授权       → commit() 授权版本"
  echo "自编码         → 汉字直接转容器编码 (天地善恶觉米袋)"
  echo "审判奖惩       → trial() → 对了奖 / 错了罚"
  echo "高维层级       → Bag 7维属性 (不数米粒)"
  echo "情进系统       → reactToEvent() 8种情感"
  echo "行为系统       → decideBehavior() 7条规则"
  echo "审美系统       → aestheticScore() 和谐·简洁·对称"
  echo "淘汰过滤       → filter.applyFilter() 健康度阈值"
  echo "12种意象       → perceive/compare/mirror/introspect/project/feedback/reflex/awaken/spiral/coordinate/base/refract"
  echo "时间螺旋版本   → TemporalVersionEngine (不可变追加 + 螺旋回溯)"
  echo "错误纠正       → commitCorrection() (创建新纠正版本 · 不改历史)"
  echo "觉悟层级       → 未觉→条件反射→认知纠偏→自我觉悟→觉行圆满"
