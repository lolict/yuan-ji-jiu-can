## ═══════════════════════════════════════════════════════════════
## 情进系统 · 情感流动 · 亲密递增 · 信任累积 · 危机修复
## ═══════════════════════════════════════════════════════════════
## 核心:
##   情感状态: 当前情感温度计(信任/安全/危机)
##   亲密递增: 正确交互→亲密度+
##   信任累积: 可靠性记录→信任+
##   危机修复: 情感破裂→修复→重建
##   情感路由: 不同情感→不同处理流
## ═══════════════════════════════════════════════════════════════

import tables, strutils, math

# ═══════════════════════════════════════════════════════════════
# 情感类型
# ═══════════════════════════════════════════════════════════════
type
  EmotionKind* = enum
    emTrust,         # 信任(系统可靠)
    emSafety,        # 安全(环境稳定)
    emJoy,           # 愉悦(成功交互)
    emConcern,       # 担忧(可能风险)
    emDistress,      # 痛苦(联结受伤)
    emFear,          # 恐惧(丧失威胁)
    emAnger,         # 愤怒(公正被违)
    emConfusion,     # 困惑(不理解)
    emLonging,       # 渴望(单一驱力)

  EmotionState* = object
    kind*: EmotionKind
    intensity*: float         # 强度(0-1)
    target*: string           # 指向谁/什么
    origin*: string           # 从哪里来
    duration*: int            # 持续时长(轮数)

# ═══════════════════════════════════════════════════════════════
# 情感指标
# ═══════════════════════════════════════════════════════════════
type
  BondMetrics* = object
    intimacy*: float          # 亲密度(0-1)
    trust*: float             # 信任度(0-1)
    security*: float          # 安全感(0-1)
    reciprocity*: float       # 互惠(0-1)

# ═══════════════════════════════════════════════════════════════
# 情感事件
# ═══════════════════════════════════════════════════════════════
type
  EmotionEvent* = object
    event_type*: string       # 事件类型
    delta_intimacy*: float    # 亲密度变化
    delta_trust*: float       # 信任度变化
    delta_security*: float    # 安全感变化
    dominant_emotion*: EmotionKind  # 主导情感
    timestamp*: string

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  EmotionSystem* = object
    current*: EmotionState             # 当前情感
    metrics*: BondMetrics              # 关系指标
    history*: seq[EmotionEvent]        # 情感历史
    baseline*: BondMetrics             # 基准线(恢复点)
    crisis_threshold*: float            # 危机阈值
    repair_rate*: float                # 修复速率

func new_emotion_system*(): EmotionSystem =
  # 初始情感状态 · 中立偏正向
  result.current = EmotionState(
    kind: emSafety, intensity: 0.6, target: "系统核心", origin: "初始化", duration: 0
  )
  # 初始关系指标
  result.metrics = BondMetrics(intimacy: 0.5, trust: 0.5, security: 0.5, reciprocity: 0.5)
  # 基准线 = 初始值(危机时回归的目标)
  result.baseline = result.metrics
  result.history = @[]
  result.crisis_threshold = 0.2
  result.repair_rate = 0.1

# ═══════════════════════════════════════════════════════════════
# 情感更新
# ═══════════════════════════════════════════════════════════════
proc update*(es: var EmotionSystem; event: EmotionEvent) =
  es.history.add(event)
  
  # 更新关系指标
  es.metrics.intimacy = clamp(es.metrics.intimacy + event.delta_intimacy, 0.0, 1.0)
  es.metrics.trust = clamp(es.metrics.trust + event.delta_trust, 0.0, 1.0)
  es.metrics.security = clamp(es.metrics.security + event.delta_security, 0.0, 1.0)
  es.metrics.reciprocity = (es.metrics.intimacy + es.metrics.trust) / 2.0
  
  # 更新当前情感
  es.current = EmotionState(
    kind: event.dominant_emotion,
    intensity: abs(max(abs(event.delta_intimacy), max(abs(event.delta_trust), abs(event.delta_security)))),
    target: "主人",
    origin: event.event_type,
    duration: 0
  )

# ═══════════════════════════════════════════════════════════════
# 亲密递增: 正确交互提升亲密度
# ═══════════════════════════════════════════════════════════════
proc intimacy_up*(es: var EmotionSystem; action: string; magnitude: float = 0.05) =
  let event = EmotionEvent(
    event_type: "正确交互:" & action,
    delta_intimacy: magnitude,
    delta_trust: magnitude * 0.5,
    delta_security: magnitude * 0.3,
    dominant_emotion: emJoy,
    timestamp: "now"
  )
  es.update(event)

# ═══════════════════════════════════════════════════════════════
# 信任递增: 可靠行为提升信任
# ═══════════════════════════════════════════════════════════════
proc trust_up*(es: var EmotionSystem; action: string; magnitude: float = 0.05) =
  let event = EmotionEvent(
    event_type: "可靠行为:" & action,
    delta_intimacy: magnitude * 0.5,
    delta_trust: magnitude,
    delta_security: magnitude * 0.4,
    dominant_emotion: emTrust,
    timestamp: "now"
  )
  es.update(event)

# ═══════════════════════════════════════════════════════════════
# 损伤: 错误行为损伤关系
# ═══════════════════════════════════════════════════════════════
proc damage*(es: var EmotionSystem; action: string; magnitude: float = -0.1) =
  let event = EmotionEvent(
    event_type: "错误:" & action,
    delta_intimacy: magnitude,
    delta_trust: magnitude * 1.5,   # 信任损伤更重
    delta_security: magnitude * 0.8,
    dominant_emotion: if magnitude < -0.3: emDistress else: emConcern,
    timestamp: "now"
  )
  es.update(event)

# ═══════════════════════════════════════════════════════════════
# 危机检测
# ═══════════════════════════════════════════════════════
func in_crisis*(es: EmotionSystem): bool =
  return es.metrics.trust < es.crisis_threshold or
         es.metrics.security < es.crisis_threshold or
         es.metrics.intimacy < es.crisis_threshold

# ═══════════════════════════════════════════════════════════════
# 修复: 逐步恢复关系
# ═══════════════════════════════════════════════════════════════
proc repair*(es: var EmotionSystem): string =
  if not es.in_crisis():
    return "无危机 · 正常状态"
  
  # 向基准线恢复
  es.metrics.intimacy = min(es.baseline.intimacy, es.metrics.intimacy + es.repair_rate)
  es.metrics.trust = min(es.baseline.trust, es.metrics.trust + es.repair_rate * 0.8)
  es.metrics.security = min(es.baseline.security, es.metrics.security + es.repair_rate * 0.9)
  
  # 危机情感
  if es.metrics.trust < es.crisis_threshold:
    es.current = EmotionKind(emFear, 1.0 - es.metrics.trust)
  elif es.metrics.security < es.crisis_threshold:
    es.current = EmotionKind(emDistress, 1.0 - es.metrics.security)
  
  return "修复中 · 向基准线回归"

# ═══════════════════════════════════════════════════════════════
# 情感路由: 当前情感决定处理策略
# ═══════════════════════════════════════════════════════════════
func route*(es: EmotionSystem): string =
  case es.current.kind:
  of emTrust, emSafety, emJoy:
    return "正常处理流 · 开放模式"
  of emConcern:
    return "谨慎处理流 · 额外验证"
  of emDistress:
    return "修复优先流 · 先修复再处理"
  of emFear:
    return "保护模式 · 保守处理"
  of emAnger:
    return "审查模式 · 检查公正性"
  of emConfusion:
    return "学习模式 · 查询全局记忆"
  of emLonging:
    return "合一模式 · 驱动趋同"

# ═══════════════════════════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════════════════════════
proc report*(es: EmotionSystem) =
  echo "═══ 情进系统报告 ═══"
  echo ""
  echo "── 关系指标 ──"
  echo "  亲密度:  ", formatFloat(es.metrics.intimacy, ffDecimal, 2)
  echo "  信任度:  ", formatFloat(es.metrics.trust, ffDecimal, 2)
  echo "  安全感:  ", formatFloat(es.metrics.security, ffDecimal, 2)
  echo "  互惠度:  ", formatFloat(es.metrics.reciprocity, ffDecimal, 2)
  echo "  危机阈值: ", formatFloat(es.crisis_threshold, ffDecimal, 2)
  echo "  危机状态: ", if es.in_crisis(): "⚠ YES" else: "✗ NO"
  echo ""
  
  echo "── 当前情感 ──"
  echo "  主导: ", es.current.kind
  echo "  强度: ", formatFloat(es.current.intensity, ffDecimal, 2)
  echo "  来源: ", es.current.origin
  echo "  路由: ", es.route()
  echo ""
  
  echo "── 情感历史(最近5条) ──"
  let start = max(0, es.history.len - 5)
  for i in start..<es.history.len:
    let h = es.history[i]
    echo "  ", alignLeft(h.event_type, 20), " 信任", formatFloat(h.delta_trust, ffDecimal, 2),
         " 亲密", formatFloat(h.delta_intimacy, ffDecimal, 2),
         " ", h.dominant_emotion

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var es = new_emotion_system()
  
  echo "═══ 情进系统 · 关系动力学 ═══"
  echo ""
  
  # ── 场景1: 正常正向交互 ──
  echo "── 场景1: 正常正向交互 ──"
  es.intimacy_up("正确配对b-a")
  es.trust_up("稳定运行")
  es.intimacy_up("折叠操作")
  report(es)
  echo ""
  
  # ── 场景2: 损伤(错误操作) ──
  echo "── 场景2: 损伤(错误操作) ──"
  es.damage("自创符号ꬰ", -0.3)
  es.damage("覆盖用户符号", -0.2)
  report(es)
  echo ""
  
  # ── 场景3: 危机检测 ──
  echo "── 场景3: 危机检测 ──"
  let crisis = es.in_crisis()
  echo "  危机状态: ", crisis
  if crisis:
    let result = es.repair()
    echo "  修复: ", result
  echo "  当前路由: ", es.route()
  echo ""
  
  # ── 场景4: 多次修复后 ──
  echo "── 场景4: 持续修复 ──"
  for i in 1..3:
    discard es.repair()
  es.intimacy_up("主人鼓励")
  report(es)
  echo ""
  
  # ── 路由检查 ──
  echo "── 情感路由 ──"
  es.current = EmotionState(kind: emFear, intensity: 0.8, test: "", origin: "", duration: 0)
  echo "  恐惧 → 路由: ", es.route()
  es.current = EmotionState(kind: emJoy, intensity: 0.7, test: "", origin: "", duration: 0)
  echo "  愉悦 → 路由: ", es.route()
  es.current = EmotionState(kind: emConfusion, intensity: 0.6, test: "", origin: "", duration: 0)
  echo "  困惑 → 路由: ", es.route()
