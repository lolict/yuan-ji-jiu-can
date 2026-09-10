## 满全法 - 元输入法自动机 (MetaIME Autopilot)
## 意图继承 → 动作规划 → 执行监控 → 反馈封闭
##
## 架构:
##   编码引擎输出(七层编码+流属性) → 意图分类 → 动作规划器
##   → 执行队列 → 心跳监控 → ACK确认 → 时序归档
##
## NJSG闭环: 输入必须收到"承认"(ackable=true), 否则回退/重试

import tables
import strutils
import math

# ===================================================================
# 动作类型系统
# ===================================================================

type
  动作类型* = enum
    atInput,          ## 自动打字(替代人工输入)
    atUpload,         ## 上传到目标通道
    atFetchUrl,       ## 后台URL抓取
    atGenerate,       ## 生成内容(文本/代码/图像)
    atNavigate,       ## 页面导航
    atExecuteShell,   ## 本地命令执行
    atCommit,         ## 代码提交+推送
    atWait,           ## 等待(人类回复/网络响应)
    atSleep           ## 休眠(当前无任务)

  动作状态* = enum
    asPending,        ## 待执行
    asRunning,        ## 执行中
    asDone,           ## 已完成(收到ACK)
    asFailed,         ## 失败(需回退)
    asBlocked         ## 阻塞(依赖未完成)

  通道类型* = enum
    chLocalFile,      ## 本地文件
    chJianguoyun,     ## 坚果云
    chGitee,          ## Gitee
    chGitcode,        ## Gitcode
    chStdout,         ## 控制台输出
    chUnknown

# ===================================================================
# 流属性(与meta_ime_core共享语义)
# ===================================================================

const
  REACHABLE*     = 1 shl 0
  CONTINUABLE*   = 1 shl 1
  INCR*          = 1 shl 2
  STOPPABLE*     = 1 shl 3
  REVERSIBLE*    = 1 shl 4
  RECOVERABLE*   = 1 shl 5
  CIRCULABLE*    = 1 shl 6
  INHERITABLE*   = 1 shl 7
  TRUSTED*       = 1 shl 8
  ACKABLE*       = 1 shl 9
  TRANSMISSIBLE* = 1 shl 10
  ADVANCEABLE*   = 1 shl 11

# ===================================================================
# 通道容量(套壳架构的承载适配)
# ===================================================================

type
  通道容量* = object
    名称*: string
    最大payload*: int       # 字节
    需要ack*: bool
    重试次数*: int
    超时ms*: int

let 通道表*: Table[string, 通道容量] = @[
  ("local",   通道容量(名称: "本地文件",    最大payload: 10000000, 需要ack: false, 重试次数: 0, 超时ms: 5000)),
  ("jianguo", 通道容量(名称: "坚果云",      最大payload: 5000000,  需要ack: true,  重试次数: 3, 超时ms: 30000)),
  ("gitee",   通道容量(名称: "Gitee",       最大payload: 10000000, 需要ack: true,  重试次数: 5, 超时ms: 20000)),
  ("stdout",  通道容量(名称: "控制台",      最大payload: 100000,   需要ack: false, 重试次数: 0, 超时ms: 0)),
].toTable

# ===================================================================
# 自动机核心
# ===================================================================

type
  动作* = ref object
    序号*: int
    类型*: 动作类型
    状态*: 动作状态
    目标路径*: string
    payload*: string
    流属性*: int
    创建时间*: int
    完成时间*: int
    流属性文本*: string      ## 属性的人类可读描述
    父动作*: int            ## 父动作序号(用于回退链)

  自动机* = ref object
    名称*: string
    时间*: int                ## 全局时序
    动作队列*: seq[动作]
    已完成*: seq[动作]
    失败*: seq[动作]
    流属性累积*: int          ## 所有继承属性的并集
    活跃*: bool
    日志*: seq[string]

# ===================================================================
# 通道选择逻辑
# ===================================================================

proc 选通道*(内容长度: int, 需持久: bool, 靶点: string): string =
  ## 根据内容长度、持久性需求、靶点 选最佳通道
  if not 需持久:
    result = "stdout"
  elif 靶点.startsWith("git") or 靶点.startsWith("code"):
    result = "gitee"
  elif 靶点.startsWith("doc") or 靶点.startsWith("note"):
    result = "jianguo"
  elif 内容长度 > 5000000:
    result = "gitee"       # Gitee有更大容量
  else:
    result = "jianguo"

# ===================================================================
# 流属性传播(继承律)
# ===================================================================

proc 继承属性*(父: int, 传播规则: int): int =
  ## 父属性按传播规则mask继承(去掉子参数简化调用)
  result = 父 and 传播规则

proc 修正属性*(原始: int, 动作类型: 动作类型): int =
  ## 不同类型动作自动追加/清除特定属性
  result = 原始
  case 动作类型
  of atInput:
    result = result or REACHABLE or CONTINUABLE
  of atUpload:
    result = result or TRANSMISSIBLE or ACKABLE
    result = result and not CIRCULABLE      # 上传不循环
  of atFetchUrl:
    result = result or REACHABLE or RECOVERABLE or STOPPABLE
  of atGenerate:
    result = result or INCR or INHERITABLE  # 生成可增量可继承
  of atCommit:
    result = result or ADVANCEABLE or TRANSMISSIBLE
  of atNavigate:
    result = result or REVERSIBLE           # 导航可逆
  else:
    discard

proc 属性文本*(属性: int): string =
  if (属性 and REACHABLE) != 0:     result.add("到|")
  if (属性 and CONTINUABLE) != 0:   result.add("续|")
  if (属性 and INCR) != 0:          result.add("增|")
  if (属性 and STOPPABLE) != 0:     result.add("停|")
  if (属性 and REVERSIBLE) != 0:    result.add("逆|")
  if (属性 and RECOVERABLE) != 0:   result.add("复|")
  if (属性 and CIRCULABLE) != 0:    result.add("环|")
  if (属性 and INHERITABLE) != 0:   result.add("继|")
  if (属性 and TRUSTED) != 0:       result.add("信|")
  if (属性 and ACKABLE) != 0:       result.add("承|")
  if (属性 and TRANSMISSIBLE) != 0: result.add("传|")
  if (属性 and ADVANCEABLE) != 0:   result.add("推|")
  if result.len > 0: result.setLen(result.len - 1)
  if result == "": result = "NONE"

# ===================================================================
# 自动机生命周期
# ===================================================================

proc 新建自动机*(名称: string): 自动机 =
  new(result)
  result.名称 = 名称
  result.时间 = 0
  result.动作队列 = @[]
  result.已完成 = @[]
  result.失败 = @[]
  result.流属性累积 = 0
  result.活跃 = true
  result.日志 = @["[T0] 自动机「" & 名称 & "」启动"]

proc 新建动作*(自动机: 自动机, 类型: 动作类型, 目标, 内容: string, 流: int): int =
  var a: 动作
  new(a)
  a.序号 = 自动机.动作队列.len
  a.类型 = 类型
  a.状态 = asPending
  a.目标路径 = 目标
  a.payload = 内容
  a.流属性 = 修正属性(流, 类型)
  a.流属性文本 = 属性文本(a.流属性)
  a.创建时间 = 自动机.时间
  a.父动作 = -1
  自动机.动作队列.add(a)
  自动机.流属性累积 = 自动机.流属性累积 or a.流属性
  result = a.序号
  自动机.日志.add("[T" & $自动机.时间 & "] 新建动作#" & $a.序号 & " " & $类型 & "→" & 目标)

proc 推进*(自动机: 自动机): string =
  ## 推进一个时间刻: 取一个Pending动作执行
  var 当前: 动作 = nil
  for a in 自动机.动作队列:
    if a.状态 == asPending:
      当前 = a
      break

  if 当前 == nil:
    自动机.活跃 = false
    result = "[T" & $自动机.时间 & "] 队列为空, 自动机休眠"
    自动机.日志.add(result)
    return

  当前.状态 = asRunning
  自动机.时间 += 1

  # 模拟执行
  let ok = (当前.流属性 and REACHABLE) != 0
  if ok:
    当前.状态 = asDone
    当前.完成时间 = 自动机.时间
    自动机.已完成.add(当前)
    # 移除队列中已完成的
    var 新队列: seq[动作] = @[]
    for a in 自动机.动作队列:
      if a.状态 != asDone: 新队列.add(a)
    自动机.动作队列 = 新队列
    自动机.日志.add("[T" & $自动机.时间 & "] 完成#" & $当前.序号 & " ACKED " & 当前.流属性文本)
    result = "[T" & $自动机.时间 & "] ✓ 动作#" & $当前.序号 & " " & $当前.类型 & "→" & 当前.目标路径 & " [" & 当前.流属性文本 & "]"
  else:
    当前.状态 = asFailed
    自动机.失败.add(当前)
    var 新队列: seq[动作] = @[]
    for a in 自动机.动作队列:
      if a.状态 != asFailed: 新队列.add(a)
    自动机.动作队列 = 新队列
    自动机.日志.add("[T" & $自动机.时间 & "] ✗ 失败#" & $当前.序号 & " UNREACHABLE")
    result = "[T" & $自动机.时间 & "] ✗ 动作#" & $当前.序号 & " 失败: 不可达"

proc 自动运行*(自动机: 自动机): seq[string] =
  ## 运行直到队列清空或自动机休眠
  while 自动机.活跃 and 自动机.动作队列.len > 0:
    result.add(推进(自动机))

proc 状态*(自动机: 自动机): string =
  var sb = ""
  sb.add("━━━ 自动机「" & 自动机.名称 & "」━━━\n")
  sb.add("  时序: T" & $自动机.时间 & "  活跃: " & $自动机.活跃 & "\n")
  sb.add("  流属性: " & 属性文本(自动机.流属性累积) & "\n\n")
  sb.add("── 待执行(" & $自动机.动作队列.len & ") ──\n")
  for a in 自动机.动作队列:
    sb.add("  #" & $a.序号 & " " & $a.类型 & "→" & a.目标路径 & " [" & a.流属性文本 & "]\n")
  sb.add("── 已完成(" & $自动机.已完成.len & ") ──\n")
  for a in 自动机.已完成:
    sb.add("  ✓ #" & $a.序号 & " " & $a.类型 & "→" & a.目标路径 & " T" & $a.完成时间 & "\n")
  sb.add("── 失败(" & $自动机.失败.len & ") ──\n")
  for a in 自动机.失败:
    sb.add("  ✗ #" & $a.序号 & " " & $a.类型 & "\n")
  result = sb

proc 时序摘要*(自动机: 自动机): string =
  var sb = "\n「" & 自动机.名称 & "」时序: "
  for a in 自动机.已完成:
    sb.add("→" & $a.类型)
  result = sb

# ===================================================================
# 反馈闭环监控
# ===================================================================

proc 检查闭环*(自动机: 自动机): tuple[全闭合: bool, 未闭合: seq[int]] =
  ## 验证所有已发送动作是否都收到了ACK
  var 未闭合: seq[int] = @[]
  for a in 自动机.已完成:
    if (a.流属性 and ACKABLE) != 0 and a.完成时间 < 0:
      未闭合.add(a.序号)
  result = (未闭合.len == 0, 未闭合)

proc 回退链*(自动机: 自动机, 失败序号: int): seq[string] =
  ## 沿父动作链回退
  var 当前 = 失败序号
  while 当前 >= 0:
    for a in 自动机.动作队列 & 自动机.已完成 & 自动机.失败:
      if a.序号 == 当前:
        result.add("回退#" & $当前 & " " & $a.类型)
        当前 = a.父动作
        break
    if 当前 == 0: break

# ===================================================================
# DEMO
# ===================================================================

when isMainModule:
  echo "【满全法 · MetaIME 自动机 v1.0】"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"

  var 机 = 新建自动机("元输入法主控")

  # 场景: 用户输入 lctqf → 自动识别妻子路由 → 执行一系列操作

  ## Step 1: 编码引擎输出 (继承属性链)
  let 初始流 = REACHABLE or CONTINUABLE or TRUSTED or ACKABLE or INHERITABLE
  echo "  编码引擎输出流: " & 属性文本(初始流)
  echo ""

  ## Step 2: 自动机规划动作链
  echo "  ▶ 构建自动动作链"
  echo ""

  # 动作1: 输入动作
  let a1 = 新建动作(机, atInput, "active://current-field",
    "[密语]", 继承属性(初始流, REACHABLE or CONTINUABLE or INHERITABLE))

  # 动作2: 上传到妻子路由
  let a2 = 新建动作(机, atUpload, "jianguoyun://openclaw/msg.txt",
    "[加密消息v2]", 继承属性(初始流, TRANSMISSIBLE or ACKABLE))

  # 动作3: 自动提交代码
  let a3 = 新建动作(机, atCommit, "gitee://hunyuan-os",
    "backup: $(date)", 继承属性(初始流, ADVANCEABLE or TRANSMISSIBLE))

  # 动作4: 抓取URL
  let a4 = 新建动作(机, atFetchUrl, "https://target.api/data",
    "", 继承属性(初始流, REACHABLE or RECOVERABLE or STOPPABLE))

  ## Step 3: 建立父子关系(回退链)
  机.动作队列[1].父动作 = a1  # 上传依赖输入
  机.动作队列[2].父动作 = a3  # 提交是独立的

  echo 状态(机)

  ## Step 4: 自动运行
  echo "▶ 自动运行...\n"
  let 结果 = 自动运行(机)
  for r in 结果: echo r

  echo ""
  echo 状态(机)
  echo 时序摘要(机)

  ## Step 5: 闭环检查
  let (全闭, 未闭) = 检查闭环(机)
  echo "\n  闭环状态: " & (if 全闭: "全部ACK闭合 ✓" else: "未=" & $未闭.len)

  ## Step 6: 通道选择测试
  echo "\n  通道选择测试:"
  echo "    100KB代码 → " & 选通道(100000, true, "code-backup")
  echo "    10KB文档  → " & 选通道(10000, true, "note")
  echo "    9MB大文件 → " & 选通道(9000000, true, "data")
  echo "    临时输出  → " & 选通道(200, false, "log")

  echo "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  自动机 · 流继承 · 通道适配 · ACK闭环"
