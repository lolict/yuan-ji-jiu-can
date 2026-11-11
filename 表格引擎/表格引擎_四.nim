# 表格编程引擎 · 第四部分 · 文件 I/O 引擎
# 框架来源：TABLE_PROGRAMMING_SPEC.md (源格式规范)

import strutils
import sequtils
import os
import 地址管理器
import 表格引擎
import 表格引擎_二
import 表格引擎_三

# =============================================================================
# .chain 文件格式
# =============================================================================
# 格式：管道分隔，# 开头为注释
# C1|C2|C3|C4|C5|C6|C7|C8|C9|C10|C11|C12|C13|C14|C15
# 001|问题描述|You|有|实体名|内容|管辖|输入|输出|成本|风险|时间|阴阳|回答|信念
# =============================================================================

const
  ChainVersion* = "1.0"
  ChainHeader* = "# TABLE_CHAIN v" & ChainVersion
  ChainColumns = ["C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "C11", "C12", "C13", "C14", "C15"]

# =============================================================================
# 序列化 (表格 → .chain 文本)
# =============================================================================

proc rowToChain*(table: Table, rowIdx: int): string =
  ## 将单行转换为 .chain 格式
  if rowIdx < 0 or rowIdx >= table.maxRows:
    return ""
  
  let row = table.rows[rowIdx]
  if row.question.len == 0:
    return ""  # 空行
  
  # C1 序号
  let c1 = $(rowIdx + 1)
  
  # C2 提问
  let c2 = row.question.replace("|", "\\|")
  
  # C3 视角
  let c3 = $row.viewpoint
  
  # C4 所在态
  let c4 = $row.state
  
  # C5 空质壳
  let c5 = row.emptyShell.replace("|", "\\|")
  
  # C6 镜像匹配
  let c6 = row.mirrorMatch.replace("|", "\\|")
  
  # C7 管辖
  let c7 = $row.jurisdiction
  
  # C8 归属
  let c8 = row.belonging.replace("|", "\\|")
  
  # C9 从属
  let c9 = row.subordination.replace("|", "\\|")
  
  # C10 允许权
  let c10 = $row.permission
  
  # C11 否决权
  let c11 = row.veto.replace("|", "\\|")
  
  # C12 态射方向
  let c12 = $row.morphDir
  
  # C13 博弈身份
  let c13 = $row.gameIdentity
  
  # C14 回答
  let c14 = row.answer.replace("|", "\\|")
  
  # C15 信念
  let c15 = row.belief.replace("|", "\\|")
  
  result = c1 & "|" & c2 & "|" & c3 & "|" & c4 & "|" & c5 & "|" & c6 & "|" & 
           c7 & "|" & c8 & "|" & c9 & "|" & c10 & "|" & c11 & "|" & c12 & "|" & 
           c13 & "|" & c14 & "|" & c15

proc tableToChain*(table: Table): seq[string] =
  ## 将整个表格转换为 .chain 文本行
  result = @[ChainHeader]
  
  # 列标题
  result.add(ChainColumns.join("|"))
  
  # 数据行
  for i in 0..<table.maxRows:
    let line = table.rowToChain(i)
    if line.len > 0:
      result.add(line)

proc saveChain*(table: Table, filename: string) =
  ## 保存表格到 .chain 文件
  let lines = table.tableToChain()
  var f = open(filename, fmWrite)
  defer: f.close()
  
  for line in lines:
    f.writeLine(line)
  
  echo "[SAVE] 已保存 ", lines.len - 2, " 行到 ", filename

# =============================================================================
# 反序列化 (.chain 文本 → 表格)
# =============================================================================

proc unescapePipe*(s: string): string =
  ## 反转义管道符
  result = s.replace("\\|", "|")

proc parseViewpoint(s: string): Viewpoint =
  case s
  of "You": You
  of "Wu": Wu
  of "Empty": Empty
  else: You  # 默认

proc parseState(s: string): State =
  case s
  of "Control": Control
  of "BeControl": BeControl
  of "Diaobao": Diaobao
  else: BeControl

proc parseJurisdiction(s: string): Jurisdiction =
  case s
  of "SelfClaim": SelfClaim
  of "External": External
  of "Pending": Pending
  else: SelfClaim

proc parseBelief(s: string): string =
  # C15 信念是字符串，不需要解析为 enum
  result = s

proc chainToRow*(table: var Table, line: string, rowIdx: int): bool =
  ## 解析单行 .chain 数据到表格
  let parts = line.split('|')
  if parts.len != 15:
    echo "[PARSE ERROR] 列数不符：期望 15, 实际 ", parts.len, " 行内容：", line
    return false
  
  var row = TableRow()
  
  # C1 跳过 (序号自动计算)
  
  # C2 提问
  row.question = parts[1].unescapePipe()
  
  # C3 视角
  if parts[2].len > 0:
    row.viewpoint = parts[2].parseViewpoint()
  
  # C4 所在态
  if parts[3].len > 0:
    row.state = parts[3].parseState()
  
  # C5 空质壳
  row.emptyShell = parts[4].unescapePipe()
  
  # C6 镜像匹配
  row.mirrorMatch = parts[5].unescapePipe()
  
  # C7 管辖
  if parts[6].len > 0:
    row.jurisdiction = parts[6].parseJurisdiction()
  
  # C8 归属
  row.belonging = parts[7].unescapePipe()
  
  # C9 从属
  row.subordination = parts[8].unescapePipe()
  
  # C10 允许权
  if parts[9].len > 0:
    row.permission = parseEnum[Permission](parts[9])
  
  # C11 否决权
  row.veto = parts[10].unescapePipe()
  
  # C12 态射方向
  if parts[11].len > 0:
    row.morphDir = parseEnum[MorphDirection](parts[11])
  
  # C13 博弈身份
  if parts[12].len > 0:
    row.gameIdentity = parseEnum[GameIdentity](parts[12])
  
  # C14 回答
  row.answer = parts[13].unescapePipe()
  
  # C15 信念
  row.belief = parts[14].unescapePipe()
  
  table.rows[rowIdx] = row
  return true

proc loadChain*(filename: string): Table =
  ## 从 .chain 文件加载表格
  result = newTable(254)
  
  if not fileExists(filename):
    echo "[LOAD ERROR] 文件不存在：", filename
    return
  
  var f = open(filename, fmRead)
  defer: f.close()
  
  var rowIdx = 0
  var lineNum = 0
  
  for line in f.lines:
    inc lineNum
    
    # 跳过注释和空行
    if line.startsWith("#") or line.strip().len == 0:
      continue
    
    # 跳过标题行
    if line.startsWith("C1|"):
      continue
    
    # 解析数据行
    if rowIdx >= result.maxRows:
      echo "[LOAD ERROR] 表格已满 (max=", result.maxRows, ")"
      break
    
    if result.chainToRow(line, rowIdx):
      inc rowIdx
  
  echo "[LOAD] 已加载 ", rowIdx, " 行从 ", filename

# =============================================================================
# 地址空间序列化
# =============================================================================

proc saveAddressSpace*(space: AddressSpace, filename: string) =
  ## 保存地址空间到文件
  var f = open(filename, fmWrite)
  defer: f.close()
  
  f.writeLine("# ADDRESS_SPACE v1.0")
  f.writeLine("# 已占用槽数：", space.occupiedSlots())
  f.writeLine("")
  
  for i in 0..<space.slots.len:
    if space.isOccupied(i):
      let cref = space.getRef(i)
      f.writeLine(i, "|", $cref.kind, "|", $cref.octant, "|", cref.namespace, "|", cref.entity)
  
  echo "[SAVE] 地址空间已保存：", space.occupiedSlots(), "/1024 槽被占用"

proc loadAddressSpace*(filename: string): AddressSpace =
  ## 从文件加载地址空间
  result = newAddressSpace()
  
  if not fileExists(filename):
    echo "[LOAD ERROR] 文件不存在：", filename
    return
  
  var f = open(filename, fmRead)
  defer: f.close()
  
  for line in f.lines:
    if line.startsWith("#") or line.strip().len == 0:
      continue
    
    let parts = line.split('|')
    if parts.len != 5:
      continue
    
    let slot = parseInt(parts[0])
    var cref: ContainerRef
    cref.kind = parseEnum[ContainerKind](parts[1])
    cref.octant = uint8(parseUInt(parts[2]))
    cref.namespace = parseUInt(parts[3])
    cref.entity = parseUInt(parts[4])
    
    result.slots[slot].address = cref
    result.slots[slot].kind = SlotKind.Filled
  
  var occupied = 0
  for slot in result.slots:
    if slot.kind == SlotKind.Filled:
      inc occupied
  echo "[LOAD] 地址空间已加载：", occupied, "/1024 槽被占用"

# =============================================================================
# 运行时持久化
# =============================================================================

proc saveRuntime*(engine: RuntimeEngine, prefix: string) =
  ## 保存运行时状态到多个文件
  engine.injection.table.saveChain(prefix & ".chain")
  engine.injection.addressSpace.saveAddressSpace(prefix & "_addr.txt")
  
  var f = open(prefix & "_meta.txt", fmWrite)
  defer: f.close()
  
  f.writeLine("# RUNTIME_METADATA v1.0")
  f.writeLine("tickCount=", engine.tickCount)
  f.writeLine("tickFrequency=", engine.tickFrequency)
  f.writeLine("tickMode=", $engine.tickMode)
  f.writeLine("surplus=", engine.surplus)
  f.writeLine("paused=", engine.paused)
  
  echo "[SAVE] 运行时元数据已保存"

proc loadRuntime*(prefix: string): RuntimeEngine =
  ## 从文件加载运行时状态
  result = newRuntimeEngine(254, 1.0)
  
  # 加载表格
  let tableFile = prefix & ".chain"
  if fileExists(tableFile):
    result.injection.table = loadChain(tableFile)
  
  # 加载地址空间
  let addrFile = prefix & "_addr.txt"
  if fileExists(addrFile):
    result.injection.addressSpace = loadAddressSpace(addrFile)
  
  # 加载元数据
  let metaFile = prefix & "_meta.txt"
  if fileExists(metaFile):
    var f = open(metaFile, fmRead)
    defer: f.close()
    
    for line in f.lines:
      if line.startsWith("#") or line.strip().len == 0:
        continue
      
      let parts = line.split('=')
      if parts.len != 2:
        continue
      
      case parts[0]
      of "tickCount": result.tickCount = parseUInt(parts[1])
      of "tickFrequency": result.tickFrequency = parseFloat(parts[1])
      of "tickMode": result.tickMode = parseEnum[TickMode](parts[1])
      of "surplus": result.surplus = parseFloat(parts[1])
      of "paused": result.paused = parseBool(parts[1])
      else: discard
    
    echo "[LOAD] 运行时元数据已加载"

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 表格编程引擎 · 第四部分测试 ==="
  
  echo "\n--- 创建测试表格 ---"
  var table = newTable(254)
  
  # 行 0
  table.rows[0].question = "我当前站入有还是无的状态？"
  table.rows[0].viewpoint = You
  table.rows[0].state = BeControl
  table.rows[0].emptyShell = "自我意识"
  table.rows[0].mirrorMatch = "观察自身站位"
  table.rows[0].jurisdiction = SelfClaim
  table.rows[0].gameIdentity = Controlled
  table.rows[0].answer = "我站入有侧 (Being)"
  table.rows[0].belief = "Affirm"
  
  # 行 1 (含管道符转义测试)
  table.rows[1].question = "管辖权 | 归属"
  table.rows[1].viewpoint = Wu
  table.rows[1].state = Control
  table.rows[1].emptyShell = "控制单元"
  table.rows[1].mirrorMatch = "管|道测试"
  table.rows[1].jurisdiction = Pending
  table.rows[1].gameIdentity = Controller
  table.rows[1].answer = "管辖权共享"
  table.rows[1].belief = "Uncertain"
  
  echo "\n--- 序列化到 .chain ---"
  table.saveChain("/workspace/表格引擎/test.chain")
  
  echo "\n--- 反序列化从 .chain ---"
  let loaded = loadChain("/workspace/表格引擎/test.chain")
  
  echo "\n--- 验证数据完整性 ---"
  echo "行 0 问题：", loaded.rows[0].question
  echo "行 0 视角：", loaded.rows[0].viewpoint
  echo "行 0 所在态：", loaded.rows[0].state
  echo "行 1 镜像匹配 (转义): ", loaded.rows[1].mirrorMatch
  echo "行 1 管辖：", loaded.rows[1].jurisdiction
  
  echo "\n--- 地址空间持久化 ---"
  var space = newAddressSpace()
  let slot1 = space.findEmptySlot()
  var slot2 = space.findEmptySlot()
  discard space.inject(slot1, Symbol, uint64(0x11111111))
  slot2 = space.findEmptySlot()
  discard space.inject(slot2, Relation, uint64(0x22222222))
  space.saveAddressSpace("/workspace/表格引擎/test_addr.txt")
  
  let loadedSpace = loadAddressSpace("/workspace/表格引擎/test_addr.txt")
  var occupied = 0
  for slot in loadedSpace.slots:
    if slot.kind == SlotKind.Filled:
      inc occupied
  echo "加载后占用槽：", occupied
  
  echo "\n--- 运行时完整持久化 ---"
  var engine = newRuntimeEngine(254, 1.0)
  discard engine.onInject(Symbol, uint64(0x1111), "测试问题 1")
  discard engine.onInject(Symbol, uint64(0x2222), "测试问题 2")
  engine.executeTick()
  
  engine.saveRuntime("/workspace/表格引擎/test_runtime")
  
  echo "\n--- 加载运行时 ---"
  let loadedEngine = loadRuntime("/workspace/表格引擎/test_runtime")
  echo "tickCount=", loadedEngine.tickCount
  echo "surplus=", loadedEngine.surplus
  echo "表格行数=", loadedEngine.injection.table.maxRows
  
  echo "\n=== 测试完成 ==="
