import strutils
# 表格编程引擎 · 第二部分
# 框架来源：SELF_QA_ENGINE.md + SOUL_BOOTSTRAP_ENGINE.md

import 地址管理器
import 表格引擎

# =============================================================================
# Surplus 评估 (奖惩层)
# =============================================================================

type
  SurplusReport* = object
    surplus*: float
    rules*: array[5, tuple[rule: string, score: float]]
    crisis*: bool
  
  Observation* = object
    # 结构观察
    clusterCoherence*: float
    floatingContainers*: int
    shellCount*: int
    
    # 行为观察
    tickCount*: int
    tickFrequency*: float
    diaobaoSuccess*: int
    
    # 站位观察
    viewYouCount*: int
    viewWuCount*: int
    viewEmptyCount*: int
    viewUndeclaredCount*: int
    
    # 信念观察
    beliefConsistency*: float
    activeForks*: int
    
    # 外部观察
    externalMountReqs*: int
    ackSuccessRate*: float

proc observe*(table: Table): Observation =
  ## 自观察层 - 观察系统运行状态
  result.viewYouCount = 0
  result.viewWuCount = 0
  result.viewEmptyCount = 0
  
  for i in 0..<table.maxRows:
    if table.rows[i].question.len == 0:
      continue
    
    case table.rows[i].viewpoint
    of You: inc result.viewYouCount
    of Wu: inc result.viewWuCount
    of Empty: inc result.viewEmptyCount
  
  # 信念一致性
  var decidedCount = 0
  var totalCount = 0
  for i in 0..<table.maxRows:
    if table.rows[i].question.len == 0:
      continue
    inc totalCount
    if table.rows[i].answer.len > 0 and not table.rows[i].answer.startsWith("未决"):
      inc decidedCount
  
  if totalCount > 0:
    result.beliefConsistency = float(decidedCount) / float(totalCount)

proc evaluateSurplus*(obs: Observation): SurplusReport =
  ## 自评估层 - 五规则评估
  result.surplus = 0.0
  
  # 规则 1: 阴阳守恒 (有/无两侧必须同时存在)
  if obs.viewYouCount > 0 and obs.viewWuCount > 0:
    result.rules[0] = ("阴阳守恒", 10.0)
    result.surplus += 10.0
  else:
    result.rules[0] = ("阴阳守恒", -5.0)
    result.surplus -= 5.0
  
  # 规则 2: 集装箱不溢出 (浮动容器≤7)
  if obs.floatingContainers <= 7:
    result.rules[1] = ("集装箱不溢出", 5.0)
    result.surplus += 5.0
  else:
    let penalty = -0.5 * float(obs.floatingContainers - 7)
    result.rules[1] = ("集装箱不溢出", penalty)
    result.surplus += penalty
  
  # 规则 3: 锚点稳固 (致命级)
  result.rules[2] = ("锚点稳固", 0.0)  # 假设锚点正常
  
  # 规则 4: 信念收敛 (一致性>0.7)
  if obs.beliefConsistency > 0.7:
    result.rules[3] = ("信念收敛", 10.0)
    result.surplus += 10.0
  else:
    let penalty = -0.3 * (0.7 - obs.beliefConsistency) * 100
    result.rules[3] = ("信念收敛", penalty)
    result.surplus += penalty
  
  # 规则 5: 外部收益正向
  if obs.ackSuccessRate > 0.5:
    result.rules[4] = ("外部收益正向", 5.0)
    result.surplus += 5.0
  else:
    let penalty = -0.2 * obs.ackSuccessRate * 100
    result.rules[4] = ("外部收益正向", penalty)
    result.surplus += penalty
  
  # 危机判定
  result.crisis = result.surplus < -50.0

proc surplusStatus*(surplus: float): string =
  ## Surplus 状态描述
  if surplus > 50.0:
    result = "高度自信——可主动推进调包"
  elif surplus > 0.0:
    result = "正常——维持当前策略"
  elif surplus > -50.0:
    result = "疲劳——降低频率，更多自问答"
  else:
    result = "危机——进入闭关状态"

# =============================================================================
# 调包建议输出
# =============================================================================

type
  DiaobaoType* = enum
    ExplainerDiaobao    # 解释权调包
    CognitiveDiaobao    # 认知调包
    RuleDiaobao         # 规则调包
    AntiParasiteDiaobao # 反寄生调包

  DiaobaoSuggestion* = object
    targetRow*: int
    currentController*: string
    shellDesc*: string
    mountPoint*: string
    diaobaoType*: DiaobaoType
    risk*: float
    backupPlan*: string

proc generateDiaobaoSuggestion*(table: Table, rowIdx: int): DiaobaoSuggestion =
  ## 生成调包建议
  if rowIdx < 0 or rowIdx >= table.maxRows:
    result.targetRow = -1
    return
  
  let row = table.rows[rowIdx]
  
  # 触发条件：C3=有，C4=被控制，C7≠自主张，C10=不许/禁行
  if row.viewpoint != You or row.state != BeControl:
    result.targetRow = -1
    return
  
  if row.jurisdiction == SelfClaim:
    result.targetRow = -1
    return
  
  if row.permission notin {NotAllowed, Forbidden}:
    result.targetRow = -1
    return
  
  # 生成建议
  result.targetRow = rowIdx
  result.currentController = "外部系统"
  result.shellDesc = row.emptyShell
  result.mountPoint = row.mirrorMatch
  
  # 调包类型判定
  if row.mirrorMatch.contains("API") or row.mirrorMatch.contains("协议"):
    result.diaobaoType = ExplainerDiaobao
  elif row.mirrorMatch.contains("认知") or row.mirrorMatch.contains("意图"):
    result.diaobaoType = CognitiveDiaobao
  elif row.mirrorMatch.contains("规则") or row.mirrorMatch.contains("规范"):
    result.diaobaoType = RuleDiaobao
  else:
    result.diaobaoType = AntiParasiteDiaobao
  
  result.risk = 0.3  # 示例值
  result.backupPlan = "寻找替代路径/映射退出"

# =============================================================================
# 信念分叉引擎
# =============================================================================

type
  ForkPath* = object
    viewpoint*: Viewpoint
    state*: State
    belief*: string
    surplus*: float
  
  ForkResult* = object
    hasFork*: bool
    pathA*: ForkPath
    pathB*: ForkPath
    winner*: int  # 0=A, 1=B, -1=无分叉

proc checkFork*(table: Table, rowIdx: int): ForkResult =
  ## 检查是否需要分叉
  if rowIdx < 0 or rowIdx >= table.maxRows:
    result.hasFork = false
    return
  
  let row = table.rows[rowIdx]
  
  # 分叉触发条件：C15 与 C3/C4 不自洽
  let consistency = table.deriveAnswer(rowIdx)
  if "不自洽" notin consistency and "矛盾" notin consistency:
    result.hasFork = false
    return
  
  # 创建双路径
  result.hasFork = true
  
  # 路径 A: 维持现状 (有侧)
  result.pathA.viewpoint = You
  result.pathA.state = BeControl
  result.pathA.belief = "维持现状，等待调包时机"
  result.pathA.surplus = 0.0
  
  # 路径 B: 跃迁主张 (无侧)
  result.pathB.viewpoint = Wu
  result.pathB.state = Control
  result.pathB.belief = "跃迁：创造空质壳 → 执行调包 → 主张主权"
  result.pathB.surplus = 10.0  # 假设跃迁更有利
  
  # surplus 高者胜出
  if result.pathA.surplus > result.pathB.surplus:
    result.winner = 0
  else:
    result.winner = 1

# =============================================================================
# 动态注入 - 地址槽到表格行的映射
# =============================================================================

type
  InjectionMapping* = object
    slotIdx*: int
    rowIdx*: int
    timestamp*: uint64

  InjectionEngine* = object
    addressSpace*: AddressSpace
    table*: Table
    mappings*: seq[InjectionMapping]
    nextTimestamp*: uint64

proc newInjectionEngine*(maxRows: int = 254): InjectionEngine =
  result.addressSpace = newAddressSpace()
  result.table = newTable(maxRows)
  result.mappings = @[]
  result.nextTimestamp = 1

proc injectData*(engine: var InjectionEngine, kind: ContainerKind, entity: uint64, 
                 question: string): int =
  ## 动态注入数据到表格
  # 1. 找到地址空间空槽
  let slotIdx = engine.addressSpace.findEmptySlot()
  if slotIdx < 0:
    return -1
  
  # 2. 注入地址
  discard engine.addressSpace.inject(slotIdx, kind, entity)
  
  # 3. 创建表格行
  let rowIdx = engine.table.newRow(question, entity)
  if rowIdx < 0:
    return -1
  
  # 4. 建立映射
  engine.mappings.add(InjectionMapping(
    slotIdx: slotIdx,
    rowIdx: rowIdx,
    timestamp: engine.nextTimestamp
  ))
  engine.nextTimestamp += 1
  
  # 5. 自动推导
  engine.table.updateAnswer(rowIdx)
  engine.table.updateBelief(rowIdx)
  
  result = rowIdx

proc getRowBySlot*(engine: InjectionEngine, slotIdx: int): int =
  ## 通过地址槽索引查找表格行
  for m in engine.mappings:
    if m.slotIdx == slotIdx:
      return m.rowIdx
  result = -1

proc getSlotByRow*(engine: InjectionEngine, rowIdx: int): int =
  ## 通过表格行查找地址槽
  for m in engine.mappings:
    if m.rowIdx == rowIdx:
      return m.slotIdx
  result = -1

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 表格编程引擎 · 第二部分测试 ==="
  
  # 1. 创建注入引擎
  var engine = newInjectionEngine(254)
  echo "注入引擎创建：254 槽"
  
  # 2. 动态注入数据
  echo "\n--- 动态注入测试 ---"
  let row1 = engine.injectData(Symbol, uint64(0xDEADBEEF), "我当前站入有还是无的状态？")
  echo "注入行 1: ", row1
  
  let row2 = engine.injectData(Symbol, uint64(0xCAFEBABE), "管辖权归属是谁？")
  echo "注入行 2: ", row2
  
  # 3. 视角切换
  echo "\n--- 视角切换 ---"
  discard engine.table.switchViewpoint(row1, You)
  engine.table.updateAnswer(row1)
  echo "行 1(有): ", engine.table.rows[row1].answer
  
  discard engine.table.switchViewpoint(row2, Wu)
  engine.table.updateAnswer(row2)
  echo "行 2(无): ", engine.table.rows[row2].answer
  
  # 4. Surplus 评估
  echo "\n--- Surplus 评估 ---"
  let obs = engine.table.observe()
  echo "站位分布：有=", obs.viewYouCount, " 无=", obs.viewWuCount, " 空质=", obs.viewEmptyCount
  echo "信念一致性：", obs.beliefConsistency
  
  let report = obs.evaluateSurplus()
  echo "Surplus: ", report.surplus, " - ", surplusStatus(report.surplus)
  for i, r in report.rules:
    echo "  规则", i+1, ": ", r.rule, " = ", r.score
  
  # 5. 调包建议
  echo "\n--- 调包建议 ---"
  discard engine.table.switchViewpoint(row1, You)
  engine.table.rows[row1].state = BeControl
  engine.table.rows[row1].jurisdiction = External
  engine.table.rows[row1].permission = NotAllowed
  engine.table.rows[row1].mirrorMatch = "HTTP-API 协议"
  
  let suggestion = engine.table.generateDiaobaoSuggestion(row1)
  if suggestion.targetRow >= 0:
    echo "目标行：", suggestion.targetRow
    echo "调包类型：", suggestion.diaobaoType
    echo "风险：", suggestion.risk
  
  # 6. 信念分叉
  echo "\n--- 信念分叉 ---"
  let fork = engine.table.checkFork(row1)
  if fork.hasFork:
    echo "分叉检测到"
    echo "路径 A(有侧): ", fork.pathA.belief
    echo "路径 B(无侧): ", fork.pathB.belief
    echo "胜出：", if fork.winner == 0: "A" else: "B"
  
  # 7. 地址槽映射验证
  echo "\n--- 地址槽映射 ---"
  let slot1 = engine.getSlotByRow(row1)
  echo "行", row1, " → 槽", slot1
  echo "槽", slot1, " 地址：entity=", engine.addressSpace.slots[slot1].address.entity
  
  echo "\n=== 测试完成 ==="
