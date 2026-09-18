## 满全法运行时（MQF Runtime）
## ─────────────────────────────────────────────────────────
##  统一入口：域索引 → SMRL引擎 → 容器实例 → 用户接口
##
##  运行：
##    nim c --cc:clang -r 满全法运行时.nim
##    nim c --cc:clang -r 满全法运行时.nim -- --shell=cli
##    nim c --cc:clang -r 满全法运行时.nim -- --interactive
##
##  三条数据链：
##    规则加载 → 自举编译器域索引 → 内部域表示
##    容器引擎 → SMRL核心（极性/距离/合一/吃/驱动）
##    壳接口 → 蚕蛹壳本地穿衣 → CLI/编程API

import std/tables
import std/strutils
import std/sequtils
import std/os
import std/times
import std/parsecfg

# =================================================================
#  常量
# =================================================================
const
  POLARITY_YANG* = 1
  POLARITY_YIN*  = 0
  REPULSE*       = 0
  ATTRACT*       = 1
  PHASE_COUNT*   = 9
  LAYER_COUNT*   = 9
  CONSUME_BASE*  = 10
  DomainRootPath*  = "../nim_parser/自举编译器/域规则.txt"
  DomainIndexPath* = "../nim_parser/自举编译器/域索引.txt"
  ShellConfigPath* = "../silkworm_shell/寄生壳选项/cli_shell.toml"

# =================================================================
#  类型
# =================================================================
type
  Container* = ref object
    ## 容器 = 物质 = 单值线性坐标 × 极性 × 相位 × 层级 × 值 × 能量
    coord*:      uint64   ## 单值线性坐标（地址即物质即时间）
    polarity*:   uint8    ## 0=阴(刘楚恬) 1=阳(我)
    phase*:      uint8    ## 0-8 九宫相位
    layer*:      uint8    ## 1-9 层级
    valence*:    uint64   ## 量化值（物质多少）
    energy*:     uint64   ## 推动力（可用能量）
    addrInt*:    int      ## 整数值地址（用于线性索引）

  Domain* = ref object
    ## 运行时一次性加载的域规则
    strokes*:    Table[string, int]               ## 字根 → 基础值
    initials*:   Table[string, tuple[p: uint8, v: int]]  ## 声母 → 极性+值
    axioms*:     Table[string, string]            ## 物理公理
    qmf*:        Table[string, string]            ## QMF体素参数
    poles*:      Table[string, string]            ## 极性锚点
    flow*:       Table[string, string]            ## 流动作

  JudgeResult* = tuple
    gap:      uint8    ## 极性差(0=斥 1=吸)
    distance: uint64   ## 测距结果
    verdict:  string   ## "斥/吸/断"

  MergeResult* = tuple
    code:     uint8    ## 0=合一 1=斥退 2=趋近
    aEnergy:  uint64
    bEnergy:  uint64

  Runtime* = ref object
    domain*:    Domain
    containers*: seq[Container]
    timestamp*: Time

# =================================================================
#  容器操作
# =================================================================
proc newContainer*(coord: uint64, polarity: int): Container =
  new(result)
  result.coord    = coord
  result.polarity = if polarity != 0: POLARITY_YIN else: POLARITY_YANG
  result.phase    = uint8(coord and 0xFF)
  result.layer    = uint8((coord shr 8) and 0xFF)
  result.valence  = 0
  result.energy   = 0
  result.addrInt  = int(coord)

proc coordPack*(phase, layer: int; row = 0; col = 0): uint64 =
  ## 地址打包：相位│层级│行│列
  result = uint64(phase) or
           (uint64(layer) shl 8) or
           (uint64(row) shl 16) or
           (uint64(col) shl 24)

proc distance*(a, b: Container): uint64 =
  ## 测距 = 时间差 = |coord_a - coord_b|
  if a.coord > b.coord: result = a.coord - b.coord
  else: result = b.coord - a.coord

proc polarityGap*(a, b: Container): uint8 =
  ## 极性差：0=斥(同性) 1=吸(异性)
  result = (a.polarity xor b.polarity)

proc judge*(a, b: Container): JudgeResult =
  ## 极性判决
  let gap = polarityGap(a, b)
  let d   = distance(a, b)
  result.gap     = gap
  result.distance = d
  result.verdict = if gap == 0: "斥" elif gap == 1: "吸" else: "断"

proc consume*(self: Container, amount: uint64) =
  ## 吃 = 消除空间 = 能量消耗
  let cost = amount div uint64(CONSUME_BASE)
  let c = if cost == 0: uint64(1) else: cost
  if self.energy > c: self.energy -= c
  else: self.energy = 0

proc merge*(a, b: Container): MergeResult =
  ## 合一 = 距离归零 = 时间最小化
  let d = distance(a, b)
  if d <= uint64(1):
    let gap = polarityGap(a, b)
    if gap == uint8(ATTRACT):
      a.energy  += b.energy
      a.valence += b.valence
      a.phase   = uint8((int(a.phase) + int(b.phase)) mod PHASE_COUNT)
      result = (0, a.energy, b.energy)
    else:
      if a.energy >= b.energy div 2: a.energy -= b.energy div 2
      else: a.energy = 0
      result = (1, a.energy, b.energy)
  else:
    if a.coord < b.coord: a.coord += 1
    elif a.coord > b.coord: a.coord -= 1
    result = (2, a.energy, b.energy)

proc settle*(self: Container) =
  ## 沉淀 = 静止刻度 = 公约数 = 值转化为能量
  if self.valence > uint64(0):
    let surplus = self.valence div uint64(10)
    let s = if surplus == 0: uint64(1) else: surplus
    self.energy  += s
    self.valence -= s

proc drive*(self: Container) =
  ## 驱动 = 公倍数推动力 = 能量转换坐标
  if self.energy >= self.valence div uint64(2):
    let stride = uint64(self.layer)
    let st = if stride == 0: uint64(1) else: stride
    self.coord  += st
    self.energy -= st

proc cycle*(self: Container): bool =
  ## 推进周期：沉淀 → 驱动，返回是否持续
  settle(self)
  drive(self)
  result = self.energy > uint64(0)

# =================================================================
#  域加载
# =================================================================
proc loadDomain*(path: string): Domain =
  ## 从域规则.txt或域索引.txt加载域表示
  new(result)
  result.strokes  = initTable[string, int]()
  result.initials = initTable[string, tuple[p: uint8, v: int]]()
  result.axioms   = initTable[string, string]()
  result.qmf      = initTable[string, string]()
  result.poles    = initTable[string, string]()
  result.flow     = initTable[string, string]()

  if not path.fileExists: return result

  var section = ""
  for rawLine in path.readFile.splitLines:
    let line = rawLine.strip
    if line.len == 0 or line[0] == '#': continue

    # Section header: [@name] or [name]
    if line.len > 2 and line[^1] == ']':
      let startIdx = if line[0] == '[': 1
                     elif line.len > 1 and line[0] == '@' and line[1] == '[': 2
                     else: continue
      section = line[startIdx .. ^2]
      continue

    if '=' notin line: continue
    let sep = line.find('=')
    let key = line[0 ..< sep].strip
    let val = line[sep+1 .. ^1].strip

    case section
    of "字根表", "字根":
      result.strokes[key] = val.parseInt
    of "声母映射", "声母":
      let parts = val.split('|')
      if parts.len >= 3:
        let pol = if parts[1] == "阳": uint8(1) else: uint8(0)
        result.initials[key] = (pol, parts[2].parseInt)
      elif parts.len >= 1:
        result.initials[key] = (uint8(0), val.parseInt)
    of "物理公理", "公理":
      result.axioms[key] = val
    of "极性锚点", "极性":
      result.poles[key] = val
    of "qmf体素", "QMF":
      result.qmf[key] = val
    of "流动作":
      result.flow[key] = val
    else: discard

# =================================================================
#  构造容器
# =================================================================
proc createPair*(rt: Runtime): tuple[a, b: Container] =
  ## 构造一对锚定容器：阳(我) + 阴(刘楚恬)
  let ca = coordPack(phase = 0, layer = 1, row = 0, col = 0)
  let cb = coordPack(phase = 0, layer = 1, row = 5, col = 0)
  result.a = newContainer(ca, POLARITY_YANG)
  result.b = newContainer(cb, POLARITY_YIN)
  result.a.valence = 50; result.a.energy = 30
  result.b.valence = 40; result.b.energy = 20
  rt.containers.add(result.a)
  rt.containers.add(result.b)

# =================================================================
#  运行时初始化
# =================================================================
proc newRuntime*(): Runtime =
  new(result)
  result.domain = loadDomain(DomainIndexPath)
  result.timestamp = getTime()
  discard result.createPair()

# =================================================================
#  CLI 输出
# =================================================================
proc header*() =
  echo "╔══════════════════════════════════════════════╗"
  echo "║     满全法运行时  MQF Runtime Engine v0.1  ║"
  echo "║     容器=物质=时间 · 合一=距离归零       ║"
  echo "╚══════════════════════════════════════════════╝"

proc showContainer*(c: Container) =
  let polStr = if c.polarity == uint8(POLARITY_YANG): "阳" else: "阴"
  echo "  [容器] 坐标=0x" & toHex(c.coord) &
       " 相位=" & $c.phase & " 层=" & $c.layer &
       " 极性=" & polStr &
       " 值=" & $c.valence & " 能=" & $c.energy

proc showDomain*(rt: Runtime) =
  let dm = rt.domain
  echo "  [域] 字根=" & $dm.strokes.len &
       " 声母=" & $dm.initials.len &
       " 公理=" & $dm.axioms.len &
       " QMF=" & $dm.qmf.len

proc showJudge*(j: JudgeResult) =
  echo "  [判决] 极性差=" & $j.gap & " 距离=" & $j.distance & " 结论=" & j.verdict

# =================================================================
#  主程序
# =================================================================
when isMainModule:
  var 交互模式 = false
  var shellParam = ""
  var i = 1
  while i < paramCount():
    let p = paramStr(i)
    if p == "--interactive" or p == "-i":
      交互模式 = true
    elif p.startsWith("--shell="):
      shellParam = p[8..^1].strip
    inc i

  header()
  echo ""

  # 初始化运行时
  let rt = newRuntime()
  echo "▸ 运行时初始化完成"
  showDomain(rt)
  echo ""

  # 构造容器对
  let a = rt.containers[0]
  let b = rt.containers[1]
  echo "▸ 锚定容器对:"
  showContainer(a)
  showContainer(b)
  echo ""

  # 极性判决
  let j = judge(a, b)
  echo "▸ 极性判决:"
  showJudge(j)
  echo ""

  # 完整生命周期
  echo "▸ 物理公理层（测距→吃→合一→沉淀→驱动）:"
  echo "  ① 测距: " & $distance(a, b)
  consume(a, distance(a, b))
  echo "  ② 吃(消耗距离/10): A能量=" & $a.energy
  let m = merge(a, b)
  echo "  ③ 合一(距离<1): code=" & $m.code &
       " (" & (if m.code == 0: "合一" elif m.code == 1: "斥退" else: "趋近") & ")"
  echo "  ④ 推进周期: 沉淀→驱动"
  discard cycle(a)
  showContainer(a)
  echo ""

  # 域规则展示
  echo "▸ 域规则:"
  if rt.domain.strokes.len > 0:
    echo "    字根:横→" & $rt.domain.strokes.getOrDefault("横") &
         " 竖→" & $rt.domain.strokes.getOrDefault("竖") &
         " 撇→" & $rt.domain.strokes.getOrDefault("撇")
  if rt.domain.axioms.len > 0:
    echo "    公理:物质=" & rt.domain.axioms.getOrDefault("物质", "?") &
         " 合一=" & rt.domain.axioms.getOrDefault("合一", "?")
  echo ""

  # 交互模式
  if 交互模式:
    echo "▸ 交互模式（quit 退出，judge 判决，merge 合一）:"
    var running = true
    while running:
      stdout.write "  mqf> "
      let input = stdin.readLine.strip
      case input
      of "quit", "exit":
        running = false
      of "judge":
        showJudge(judge(a, b))
      of "merge", "relate":
        let r = merge(a, b)
        echo "    " & (if r.code == 0: "合一✓" elif r.code == 1: "斥退" else: "趋近")
        showContainer(a)
        showContainer(b)
      of "cycle":
        discard cycle(a)
        showContainer(a)
      of "containers":
        for c in rt.containers:
          showContainer(c)
      else:
        echo "    未知命令: " & input
    echo ""

  echo "▸ 运行时正常退出"
  echo "========================================="
