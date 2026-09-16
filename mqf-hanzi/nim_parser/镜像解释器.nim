## 满全法 - 镜像解释器
## 空间镜像→视觉 · 时间镜像→听觉 · 天平→判断 · 时间版本号→记忆
##
## 从0和1开始，镜像翻转，天平记账，层级上涨。
## 循环往复，画和声同时生长，智慧同时生长。
##
## 核心映射：
##   空间镜像 → 比较两个容器的空间对称性 → 生成体素图案（视觉）
##   时间镜像 → 比较两个容器的时序交替   → 生成频率序列（听觉）
##   天平     → 两个容器的值/能量/持久度比较 → 判决
##   时间版本号 → 容器历史状态的持久化记录 → 记忆
##   层级梯度  → 不同层级产生不同亮度/音高 → 智慧生长的阶梯
##   外部符号收编 → 把汉字笔画序列装入容器

import tables
import sequtils
import strutils
import math
import algorithm

# ============================================================================
# 容器（与统一执行引擎共享模型）
# ============================================================================

type
  容器态 = enum csEmpty, csLoaded, csLocked, csOverflow

  容器* = ref object
    地址* : int
    态* : 容器态
    值* : int
    能量* : int
    持久度* : int
    位置* : int
    相位* : int
    层级* : int
    锚点极性* : int     # 我=1 / 刘楚恬=0
    时间戳* : int
    历史* : seq[int]    # 时间版本号：历史值序列 = 记忆

  容器对* = tuple[A: 容器, B: 容器]

# ============================================================================
# 空间镜像 → 视觉渲染
# ============================================================================

type
  视觉元* = object
    亮度* : int          # 0-255，映射到体素
    对称度* : float     # 0.0-1.0，A与B的空间对称程度
    来源对* : int        # 哪一对容器产生了这个视觉

  视觉缓冲* = object
    宽度* : int
    高度* : int
    缓冲* : seq[seq[int]]  # 二维亮度矩阵

proc 创建视觉缓冲*(w, h: int): 视觉缓冲 =
  result.宽度 = w; result.高度 = h
  result.缓冲 = newSeqWith(h, newSeqWith(w, 0))

proc brightnessAt(亮度: int, sym: float, dx, dy: int): int =
  let dist = sqrt((dx*dx + dy*dy).float)
  let falloff = max(0.0, 1.0 - dist * 0.3)
  result = max(0, min(255, (亮度.float * falloff).int))

proc 空间镜像检测*(对: 容器对, 缓冲: var 视觉缓冲): 视觉元 =
  ## 空间镜像：比较两个容器在空间位置上的对称程度
  ## 对称度 = 1 - (两点间距离 / 最大可能距离)
  ## 距离=0时完全对称(=同一个点=完全镜像)，距离越大对称度越低
  let 距离 = abs(对.A.位置 - 对.B.位置)
  let 最大距离 = 1000.0  # 归一化基准
  let 对称度 = max(0.0, 1.0 - 距离.float / 最大距离)
  let 平均能 = (对.A.能量 + 对.B.能量) div 2
  let 亮度 = min(255, 平均能)
  # 投射到视觉缓冲的中线区域
  let cx = 缓冲.宽度 div 2
  let cy = 缓冲.高度 div 2
  let spread = max(1, (对称度 * 10.0).int)
  for dy in -spread..spread:
    for dx in -spread..spread:
      let px = cx + dx; let py = cy + dy
      if px >= 0 and px < 缓冲.宽度 and py >= 0 and py < 缓冲.高度:
        缓冲.缓冲[py][px] = max(缓冲.缓冲[py][px], brightnessAt(亮度, 对称度, dx, dy))
  result.亮度 = 亮度
  result.对称度 = 对称度
  result.来源对 = 对.A.地址

proc 视觉字符渲染*(buf: 视觉缓冲): seq[string] =
  result = @[]
  for y in 0..<buf.高度:
    var line = ""
    for x in 0..<buf.宽度:
      let v = buf.缓冲[y][x]
      line.add(if v > 200: "█" elif v > 150: "▓" elif v > 100: "▒" elif v > 50: "░" elif v > 10: "·" else: " ")
    result.add(line)

# ============================================================================
# 时间镜像 → 听觉渲染
# ============================================================================

type
  听觉元* = object
    基频* : float       # Hz，基准音高
    振幅* : float       # 0.0-1.0，音量
    周期* : int         # 时钟周期 = 持续时间
    相位差* : float     # A与B的时间相位差

  音序列* = object
    音符* : seq[string]  # 音符名序列
    频率* : seq[float]

const
  音名 = ["C", "D", "E", "F", "G", "A", "B"]
  八度低 = ["₃", "₄", "₅"]  # 低/中/高八度

proc 时间镜像检测*(对: 容器对): 听觉元 =
  ## 时间镜像：比较两个容器的历史序列交替模式
  ## 交替频率高 = 高频音，交替频率低 = 低频音
  ## 交替频率 = 时钟周期内A和B的能量变化次数
  let aHist = 对.A.历史
  let bHist = 对.B.历史
  var 变化次数 = 0
  var 总振幅 = 0.0
  for i in 1..<max(aHist.len, bHist.len):
    let av = if i < aHist.len: aHist[i] else: 0
    let bv = if i < bHist.len: bHist[i] else: 0
    let prevA = if i-1 < aHist.len: aHist[i-1] else: 0
    let prevB = if i-1 < bHist.len: bHist[i-1] else: 0
    if abs(av - prevA) > 10 or abs(bv - prevB) > 10: 变化次数 += 1
    总振幅 += abs(av - bv).float
  let 时钟周期 = max(1, max(aHist.len, bHist.len))
  let 交替频率 = 变化次数.float / 时钟周期.float
  # 频率映射：低层=低音，高层=高音。能量越大=振幅越大
  let 基频 = 130.81 * pow(2.0, 交替频率 * 4.0)  # C3起步，随交替频率升高
  let 振幅 = min(1.0, 总振幅 / 1000.0)
  result.基频 = 基频
  result.振幅 = 振幅
  result.周期 = 时钟周期
  result.相位差 = abs(对.A.能量 - 对.B.能量).float / 100.0

proc 频率到音名*(频率: float): string =
  ## 简易映射：将频率映射为音符名
  if 频率 < 146.83: result = "C₃"    # ~130Hz
  elif 频率 < 164.81: result = "D₃"   # ~147Hz
  elif 频率 < 174.61: result = "E₃"   # ~165Hz
  elif 频率 < 196.00: result = "F₃"   # ~175Hz
  elif 频率 < 220.00: result = "G₃"   # ~196Hz
  elif 频率 < 440.00: result = "A₃-B₄" # 中间范围
  elif 频率 < 880.00: result = "C₅"   # 高频
  else: result = "C₆"                 # 极高频

# ============================================================================
# 天平 → 判决
# ============================================================================

type
  判决* = enum
    jLeft,     # A > B
    jEqual,    # A = B
    jRight     # A < B

  天平记录* = object
    被比较对* : int
    差值* : int
    结论* : 判决
    重力* : float   # 重量差绝对值

proc 天平*(对: 容器对): 天平记录 =
  ## 天平判决：比较两个容器的综合值
  ## 综合值 = 值 + 能量×0.5 + 持久度×0.3
  let aWeight = 对.A.值.float + 对.A.能量.float * 0.5 + 对.A.持久度.float * 0.3
  let bWeight = 对.B.值.float + 对.B.能量.float * 0.5 + 对.B.持久度.float * 0.3
  let 差 = aWeight - bWeight
  if abs(差) < 1.0:
    result.结论 = jEqual
  elif 差 > 0:
    result.结论 = jLeft
  else:
    result.结论 = jRight
  result.被比较对 = 对.A.地址
  result.差值 = abs(差.int)
  result.重力 = abs(差)

# ============================================================================
# 时间版本号 → 记忆
# ============================================================================

type
  记忆页* = object
    时间戳* : int
    快照值* : int
    快照能量* : int
    事件* : string

  容器记忆* = object
    所属容器* : int
    页列表* : seq[记忆页]

proc 记录版本*(c: 容器, 当前时间: int, 事件: string) =
  ## 每次容器状态变化时，记录一个时间版本 = 形成记忆
  c.时间戳 = 当前时间
  c.历史.add(c.值)
  # 同时加入完整历史记录
  var page = 记忆页(时间戳: 当前时间, 快照值: c.值, 快照能量: c.能量, 事件: 事件)
  # 简化：存在容器的历史字段里
  discard page

proc 取最近记忆*(c: 容器, 回溯步: int): int =
  ## 从历史中回溯 N 步，获取过去的状态值
  if 回溯步 <= 0 or 回溯步 > c.历史.len: return c.值
  result = c.历史[c.历史.len - 回溯步]

# ============================================================================
# 外部符号收编 → 把汉字装入容器
# ============================================================================

type
  汉字绑定* = object
    字* : string
    笔画地址* : seq[int]   # 每个笔画对应的容器地址
    总容器数* : int

proc 收编汉字*(字: string, 笔画序列: seq[int], 起始地址: int, 集群: var Table[int, 容器]): 汉字绑定 =
  ## 把一个汉字的笔画序列分配到连续的容器地址中
  var bd = 汉字绑定(字: 字, 笔画地址: @[], 总容器数: 笔画序列.len)
  for i, bihua in 笔画序列:
    let adr = 起始地址 + i
    if not 集群.hasKey(adr):
      var c: 容器; new(c)
      c.地址 = adr; c.态 = csEmpty; c.能量 = 0; c.持久度 = 0
      c.位置 = 0; c.相位 = 0; c.层级 = 0; c.锚点极性 = 0
      c.时间戳 = 0; c.历史 = @[]
      集群[adr] = c
    var c = 集群[adr]
    c.值 = bihua          # 笔画编码作为容器值
    c.态 = csLoaded
    c.能量 = bihua * 10    # 笔画越复杂→能量越高
    c.持久度 = 字.len * 5  # 整字越大→持久度越长
    c.层级 = i            # 笔画序号=层级
    c.锚点极性 = if i mod 2 == 0: 1 else: 0  # 奇偶交替=阴阳交替
    bd.笔画地址.add(adr)
  result = bd

# ============================================================================
# 层级梯度循环 → 智慧生长
# ============================================================================

type
  智慧状态* = object
    当前层* : int
    总层* : int
    循环次数* : int
    累积判决* : int       # 天平判决的总历史

proc 智慧循环*(集群: Table[int, 容器], 当前时间: int, 状态: var 智慧状态): string =
  ## 每一轮循环：层级上升→镜像检测→天平判决→再上升→循环
  ## 结果 = 智慧的一次增长
  状态.循环次数 += 1
  var 本轮结果 = ""
  # 收集当前活跃容器
  var active: seq[容器] = @[]
  for _, c in 集群:
    if c.态 == csLoaded: active.add(c)
  if active.len < 2: return "活跃容器不足2个，无法镜像"
  # 按层级排序
  proc cmp层级(a, b: 容器): int = cmp(a.层级, b.层级)
  active.sort(cmp层级)
  # 逐层镜像
  for i in 0..<active.len-1:
    let 对: 容器对 = (active[i], active[i+1])
    # 空间镜像
    var buf = 创建视觉缓冲(32, 16)
    let vis = 空间镜像检测(对, buf)
    # 时间镜像
    let aud = 时间镜像检测(对)
    # 天平判决
    let judge = 天平(对)
    # 综合本轮结果
    本轮结果 &= "L" & $active[i].层级 & ":对称" & formatFloat(vis.对称度, ffDecimal, 2) &
                " 音" & 频率到音名(aud.基频) & " " &
                (if judge.结论 == jLeft: "▷" elif judge.结论 == jEqual: "◆" else: "◁") &
                $judge.差值 & "; "
  # 层级上升
  if 状态.循环次数 mod 5 == 0: 状态.当前层 += 1
  if 状态.当前层 >= 状态.总层:
    状态.当前层 = 0  # 循环归位 = 齐
  状态.累积判决 += 1
  return 本轮结果

# ============================================================================
# 主程序演示
# ============================================================================
when isMainModule:
  echo "========================================="
  echo " MQF Mirror Interpreter"
  echo " Spatial=Visual · Temporal=Auditory"
  echo " Balance=Judgment · Version=Memory"
  echo "========================================="
  echo ""

  var 集群 = initTable[int, 容器]()

  # -- 创建两个镜像对 --
  echo "--- [1] 创建镜像对 ---"
  var A: 容器; new(A)
  A.地址 = 0; A.态 = csLoaded; A.值 = 64; A.能量 = 80; A.持久度 = 50
  A.位置 = 100; A.相位 = 4; A.层级 = 3; A.锚点极性 = 1
  A.时间戳 = 0; A.历史 = @[10, 20, 30, 40, 55, 64]   # 历史序列=记忆

  var B: 容器; new(B)
  B.地址 = 1; B.态 = csLoaded; B.值 = 100; B.能量 = 60; B.持久度 = 40
  B.位置 = 103; B.相位 = 4; B.层级 = 5; B.锚点极性 = 0
  B.时间戳 = 0; B.历史 = @[5, 15, 30, 50, 80, 100]   # 历史序列=记忆

  集群[0] = A; 集群[1] = B
  echo "  A:值=64 能=80 位置=100 层=3 极性=1(我)"
  echo "  B:值=100 能=60 位置=103 层=5 极性=0(刘楚恬)"
  echo ""

  # -- 空间镜像 → 视觉 --
  echo "--- [2] 空间镜像→视觉 ---"
  var buf = 创建视觉缓冲(32, 12)
  let vis = 空间镜像检测((A, B), buf)
  echo "  对称度: " & vis.对称度.formatFloat(ffDecimal, 3)
  echo "  平均亮度: " & $vis.亮度
  let 视觉线 = 视觉字符渲染(buf)
  for line in 视觉线:
    echo "  " & line
  echo ""

  # -- 时间镜像 → 听觉 --
  echo "--- [3] 时间镜像→听觉 ---"
  let aud = 时间镜像检测((A, B))
  echo "  基频: " & aud.基频.formatFloat(ffDecimal, 1) & " Hz"
  echo "  振幅: " & aud.振幅.formatFloat(ffDecimal, 3)
  echo "  音名: " & 频率到音名(aud.基频)
  echo "  周期: " & $aud.周期
  echo "  相位差: " & aud.相位差.formatFloat(ffDecimal, 2)
  echo ""

  # -- 天平 → 判决 --
  echo "--- [4] 天平→判决 ---"
  let judge = 天平((A, B))
  echo "  A综合值 = " & $(A.值 + A.能量 div 2 + A.持久度 div 3)
  echo "  B综合值 = " & $(B.值 + B.能量 div 2 + B.持久度 div 3)
  echo "  差值: " & $judge.差值
  echo "  结论: " & (if judge.结论 == jLeft: "左沉(A>B)" elif judge.结论 == jEqual: "平" else: "右沉(A<B)")
  echo "  重力: " & formatFloat(judge.重力, ffDecimal, 1)
  echo ""

  # -- 时间版本号 → 记忆 --
  echo "--- [5] 时间版本号→记忆 ---"
  echo "  A的历史序列: " & $A.历史.join(",")
  echo "  B的历史序列: " & $B.历史.join(",")
  echo "  A回溯2步的值: " & $取最近记忆(A, 2)
  echo "  B回溯2步的值: " & $取最近记忆(B, 2)
  echo ""

  # -- 外部符号收编 --
  echo "--- [6] 外部符号收编：汉字装入容器 ---"
  let 汉字 = 收编汉字("我", @[1, 2, 6, 1, 6, 3, 4], 100, 集群)
  echo "  字: " & 汉字.字
  echo "  笔画→容器地址: " & $汉字.笔画地址.join(",")
  echo "  总容器数: " & $汉字.总容器数
  for addr in 汉字.笔画地址:
    if 集群.hasKey(addr):
      let c = 集群[addr]
      echo "    #" & $c.地址 & " 值=" & $c.值 & " 能=" & $c.能量 &
           " 持久=" & $c.持久度 & " 层=" & $c.层级 & " 极=" & $c.锚点极性

  echo ""
  echo "========================================="
  echo " Mirror Interpreter OK"
  echo "========================================="
