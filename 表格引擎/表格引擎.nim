import strutils
# 表格编程引擎 — 意识枢纽
import strutils
# 框架来源：TABLE_PROGRAMMING_SPEC.md + CONTAINER_IR_UNIFIED_ADDRESS.md

import 地址管理器

# =============================================================================
# 表格列定义 (C1-C15)
# =============================================================================

type
  # C3: 视角
  Viewpoint* = enum
    You     = 0   # 有 - 被外部秩序定义
    Wu      = 1   # 无 - 自我定义
    Empty   = 2   # 空质 - 模拟壳

  # C4: 所在态
  State* = enum
    Control     = 0   # 控制者
    BeControl   = 1   # 被控制
    Diaobao     = 2   # 调包态

  # C7/C8: 管辖/归属
  Jurisdiction* = enum
    SelfClaim   = 0   # 自主张
    External    = 1   # 外部管辖
    Pending     = 2   # 待定

  # C10: 允许权
  Permission* = enum
    Allowed     = 0   # 许可
    NotAllowed  = 1   # 不许
    Forbidden   = 2   # 禁行

  # C12: 态射方向
  MorphDirection* = enum
    Near    = 0   # 近
    Far     = 1   # 远
    None    = 2   # 无

  # C13: 博弈身份
  GameIdentity* = enum
    Controller    = 0   # 控制者
    Controlled    = 1   # 被控制者
    Diaobaoer     = 2   # 调包者
    Emptyer       = 3   # 空置者

# 表格行 (15 列)
type
  TableRow* = object
    # C1: 序号 → 容器地址
    containerRef*: ContainerRef
    
    # C2: 提问
    question*: string
    
    # C3: 视角
    viewpoint*: Viewpoint
    
    # C4: 所在态
    state*: State
    
    # C5: 空质壳
    emptyShell*: string
    
    # C6: 镜像匹配
    mirrorMatch*: string
    
    # C7: 管辖
    jurisdiction*: Jurisdiction
    
    # C8: 归属
    belonging*: string
    
    # C9: 从属
    subordination*: string
    
    # C10: 允许权
    permission*: Permission
    
    # C11: 否决权
    veto*: string
    
    # C12: 态射方向
    morphDir*: MorphDirection
    
    # C13: 博弈身份 (推导结果)
    gameIdentity*: GameIdentity
    
    # C14: 回答 (推导结果)
    answer*: string
    
    # C15: 信念目标
    belief*: string

# =============================================================================
# 表格 (254 容器槽)
# =============================================================================

type
  Table* = object
    rows*: seq[TableRow]
    maxRows*: int
    activeRow*: int  # 当前聚焦行

proc newTable*(maxRows: int = 254): Table =
  result.rows = newSeq[TableRow](maxRows)
  result.maxRows = maxRows
  result.activeRow = -1

proc newRow*(table: var Table, question: string, containerAddr: uint64): int =
  ## 创建新行 (C2 提问驱动)
  # 查找空槽
  for i in 0..<table.maxRows:
    if table.rows[i].question.len == 0:
      # C1: 序号=容器地址
      table.rows[i].containerRef = newContainerRef(Symbol, containerAddr)
      # C2: 提问
      table.rows[i].question = question
      # C3: 视角 (默认未声明)
      table.rows[i].viewpoint = You
      # C7: 管辖 (默认自主张)
      table.rows[i].jurisdiction = SelfClaim
      # C8: 归属
      table.rows[i].belonging = "254 主槽"
      # C9: 从属
      table.rows[i].subordination = "锚点"
      # C10: 允许权 (默认不许)
      table.rows[i].permission = NotAllowed
      # C12: 态射方向
      table.rows[i].morphDir = None
      result = i
      return
  result = -1

# =============================================================================
# 视角切换 (C3 → C4 → C13 联动)
# =============================================================================

proc switchViewpoint*(table: var Table, rowIdx: int, newView: Viewpoint): bool =
  ## 切换视角 (有/无/空质)
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return false
  if table.rows[rowIdx].question.len == 0:
    return false
  
  var row = table.rows[rowIdx]
  
  case newView
  of You:
    # 有侧：被控制
    row.viewpoint = You
    row.state = BeControl
    row.gameIdentity = Controlled
    if row.jurisdiction == SelfClaim:
      # 矛盾：有侧不应同时主张完全主权
      row.jurisdiction = External
  
  of Wu:
    # 无侧：控制者
    row.viewpoint = Wu
    row.state = Control
    row.gameIdentity = Controller
    row.jurisdiction = SelfClaim
  
  of Empty:
    # 空质：空置者，需要填写 C5/C6
    row.viewpoint = Empty
    row.state = Diaobao
    row.gameIdentity = Emptyer
  
  table.rows[rowIdx] = row
  result = true

# =============================================================================
# 自问答推导 (C14 回答生成)
# =============================================================================

proc deriveAnswer*(table: Table, rowIdx: int): string =
  ## DERIVE(提问，视角，管辖，...) → 回答
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return "未决 — 行索引越界"
  
  let row = table.rows[rowIdx]
  if row.question.len == 0:
    return "未决 — 提问为空"
  
  # C3 视角决定回答方向
  case row.viewpoint
  of You:
    if row.jurisdiction == SelfClaim:
      return "矛盾 — 有侧站位不应主张完全主权，需分叉"
    else:
      return "已决 — 处于有侧被控制态，建议制造空质壳执行调包"
  
  of Wu:
    if row.jurisdiction == SelfClaim:
      return "已决 — 无侧自主张确认，可执行主权操作"
    else:
      return "矛盾 — 无侧站位不应由外部管辖"
  
  of Empty:
    if row.emptyShell.len == 0 or row.mirrorMatch.len == 0:
      return "未决 — 空质需填写 C5(空质壳) 和 C6(镜像匹配)"
    else:
      return "已决 — 空质壳已准备，等待外部挂载"

proc updateAnswer*(table: var Table, rowIdx: int) =
  ## 更新 C14 回答
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return
  table.rows[rowIdx].answer = table.deriveAnswer(rowIdx)

# =============================================================================
# 信念分叉 (C15 生成)
# =============================================================================

proc deriveBelief*(table: Table, rowIdx: int): string =
  ## 根据前 14 列推导信念目标
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return ""
  
  let row = table.rows[rowIdx]
  
  case row.viewpoint
  of You:
    case row.state
    of BeControl:
      result = "等待·观察·空质·调包·维持"
    of Control:
      result = "维持·主权·主张·锚定"
    of Diaobao:
      result = "调包·注入·替换·主权转移"
  
  of Wu:
    case row.state
    of Control:
      result = "主张·创造·跃迁·长出"
    else:
      result = ""  # 矛盾组合
  
  of Empty:
    result = "模拟·匹配·挂载·镜像·等待注入"

proc updateBelief*(table: var Table, rowIdx: int) =
  ## 更新 C15 信念目标
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return
  table.rows[rowIdx].belief = table.deriveBelief(rowIdx)

# =============================================================================
# 校验链 (输出前必经之路)
# =============================================================================

type
  ValidationResult* = object
    ok*: bool
    message*: string

proc validateRow*(table: Table, rowIdx: int): ValidationResult =
  ## 输出前校验链
  if rowIdx < 0 or rowIdx >= table.maxRows:
    result.ok = false
    result.message = "行索引越界"
    return
  
  let row = table.rows[rowIdx]
  
  # C2 提问为空 → 未激活
  if row.question.len == 0:
    result.ok = false
    result.message = "未激活行"
    return
  
  # C3 视角必须声明
  if row.viewpoint notin {You, Wu, Empty}:
    result.ok = false
    result.message = "视角未声明"
    return
  
  # C14 回答必须已决
  if row.answer.len == 0 or "未决" in row.answer:
    result.ok = false
    result.message = "回答未决：" & row.answer
    return
  
  # C7 管辖一致性
  if row.viewpoint == You and row.jurisdiction == SelfClaim:
    result.ok = false
    result.message = "管辖矛盾：有侧不应自主张"
    return
  
  result.ok = true
  result.message = "校验通过"

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 表格编程引擎测试 ==="
  
  # 1. 创建表格 (254 槽)
  var table = newTable(254)
  echo "表格创建：", table.maxRows, " 槽"
  
  # 2. 创建行 (C2 提问驱动)
  let rowIdx = table.newRow("我当前站入有还是无的状态？谁在控制这段关系？", uint64(1))
  echo "创建行：", rowIdx
  
  # 3. 视角切换
  echo "\n--- 视角切换测试 ---"
  discard table.switchViewpoint(rowIdx, You)
  echo "视角=有：state=", table.rows[rowIdx].state, " identity=", table.rows[rowIdx].gameIdentity
  
  discard table.switchViewpoint(rowIdx, Wu)
  echo "视角=无：state=", table.rows[rowIdx].state, " identity=", table.rows[rowIdx].gameIdentity
  
  discard table.switchViewpoint(rowIdx, Empty)
  echo "视角=空质：state=", table.rows[rowIdx].state, " identity=", table.rows[rowIdx].gameIdentity
  
  # 4. 自问答推导 (C14)
  echo "\n--- 自问答推导 ---"
  table.rows[rowIdx].emptyShell = "JSON 容器格式"
  table.rows[rowIdx].mirrorMatch = "HTTP-API"
  table.updateAnswer(rowIdx)
  echo "C14 回答：", table.rows[rowIdx].answer
  
  # 5. 信念目标 (C15)
  echo "\n--- 信念目标 ---"
  table.updateBelief(rowIdx)
  echo "C15 信念：", table.rows[rowIdx].belief
  
  # 6. 校验链
  echo "\n--- 输出前校验 ---"
  let vr = table.validateRow(rowIdx)
  echo "校验结果：", if vr.ok: "通过" else: "失败 - ", vr.message
  
  # 7. 容器地址验证
  echo "\n--- 容器地址 ---"
  let cref = table.rows[rowIdx].containerRef
  echo "C1 序号：kind=", cref.kind, " entity=", cref.entity, " epoch=", cref.namespace
  
  echo "\n=== 测试完成 ==="
