# ============================================================
# 满全法 — 元输入法核心引擎 (MetaIME Core)
# 管道: 键盘拦截→笔画寻址→极性路由→容器/汉字双输出
# 设计: 智能内嵌, 不依赖云端, 索引一次构建运行时不增长
# ============================================================

import strutils
import tables
import sequtils

# ============================================================
# 类型定义
# ============================================================

type
  极性* = enum
    阴 = 0   # 刘楚恬
    阳 = 1   # 我

  笔画* = enum
    横 = 1
    竖 = 2
    撇 = 3
    捺点 = 4
    折 = 5
    其它 = 6

  按键* = enum
    K1, K2, K3, K4, K5, K6, K7, K8, K9, K0,
    KA, KB, KC, KD, KE, KF, KG, KH, KI, KJ,
    KK, KL, KM, KN, KO, KP, KQ, KR, KS, KT,
    KU, KV, KW, KX, KY, KZ,
    K_BS, K_SPC, K_ENT, K_ESC

  意图* = enum
    打字, 容器寻址, 极性判决, 流切换, 心跳, 身份切换, 寄生出站

  元输入法* = object
    缓冲*: seq[按键]
    笔画缓冲*: seq[笔画]
    极性栈*: seq[极性]
    意图表*: Table[seq[按键], 意图]
    当前相位*: int
    笔计数*: array[0..6, int]              # 横/竖/撇/捺点/折/其它 计数
    触键计数*: int

# ============================================================
# 笔画→汉字反向索引 (寻址表, 一次性构建)
# ============================================================

const 笔画序到汉字* = {
  "0100000": "一",
  "0110000": "十",
  "0101100": "大",
  "0111100": "木",
  "0200000": "二",
  "0111000": "仁",
  "0101001": "我",
  "0101101": "刘",
  "0110100": "全",
  "0111001": "法",
  "0110110": "囡",
  "0110010": "囝",
}.toTable()

const 按键笔画映射*: array[按键, seq[笔画]] = [
  @[横],          # K1
  @[竖],          # K2
  @[撇],          # K3
  @[捺点],        # K4
  @[折],          # K5
  @[横, 竖],      # K6
  @[撇, 捺点],    # K7
  @[横, 竖, 撇, 捺点],  # K8
  @[折, 横],      # K9
  @[其它],        # K0
  @[横, 横],      # KA
  @[竖, 竖],      # KB
  @[撇, 撇],      # KC
  @[捺点, 捺点],  # KD
  @[折, 折],      # KE
  @[横, 撇],      # KF
  @[竖, 捺点],    # KG
  @[撇, 横],      # KH
  @[捺点, 竖],    # KI
  @[折, 撇],      # KJ
  @[横, 折],      # KK
  @[竖, 撇],      # KL
  @[撇, 竖],      # KM
  @[捺点, 折],    # KN
  @[横, 捺点],    # KO
  @[竖, 横],      # KP
  @[折, 捺点],    # KQ
  @[横, 竖, 撇],  # KR
  @[撇, 捺点, 横],# KS
  @[竖, 横, 捺点],# KT
  @[折, 横, 竖],  # KU
  @[捺点, 撇, 竖],# KV
  @[横, 撇, 捺点],# KW
  @[竖, 折, 横],  # KX
  @[撇, 横, 捺点],# KY
  @[折, 撇, 捺点],# KZ
  @[],            # K_BS
  @[],            # K_SPC
  @[],            # K_ENT
  @[],            # K_ESC
]

# ============================================================
# 引擎构造
# ============================================================

proc 创建元输入法*(): 元输入法 =
  var ime: 元输入法
  ime.缓冲 = @[]
  ime.笔画缓冲 = @[]
  ime.极性栈 = @[]
  ime.意图表 = initTable[seq[按键], 意图]()
  ime.当前相位 = 0
  ime.笔计数 = [0, 0, 0, 0, 0, 0, 0]
  ime.触键计数 = 0
  # 模式切换键
  ime.意图表[@[K_ESC, K1]] = 容器寻址
  ime.意图表[@[K_ESC, K2]] = 极性判决
  ime.意图表[@[K_ESC, K3]] = 流切换
  ime.意图表[@[K_ESC, K4]] = 心跳
  ime.意图表[@[K_ESC, K5]] = 身份切换
  ime.意图表[@[K_ESC, K6]] = 寄生出站
  return ime

# ============================================================
# 第一层: 键盘拦截
# ============================================================

proc 按键到笔画*(k: 按键): seq[笔画] =
  按键笔画映射[k]

# ============================================================
# 第二层: 笔画寻址 (O(1) 索引查汉字)
# ============================================================

proc 寻址汉字*(笔画计数: array[7, int]): string =
  let key = 笔画计数.mapIt($it).join("")
  if 笔画序到汉字.hasKey(key):
    return 笔画序到汉字[key]
  # 无精确匹配: 返回笔画签名
  return "‹笔画" & key & "›"

# ============================================================
# 第三层: 极性路由
# ============================================================

proc 路由*(ime: var 元输入法): 意图 =
  var yn, yang: int = 0
  for p in ime.极性栈:
    if p == 阴: inc yn
    else: inc yang
  let gap = yn - yang
  let total = len(ime.笔画缓冲)
  if total == 0: 打字
  elif gap > 3: 极性判决
  elif gap < -3: 极性判决
  elif total > 5: 容器寻址
  else: 打字

proc 极性路由*(ime: var 元输入法): tuple[极性: 极性; 意图: 意图; 吸斥: string] =
  var yn, yang: int = 0
  for p in ime.极性栈:
    if p == 阴: inc yn
    else: inc yang
  let dom = if yang >= yn: 阳 else: 阴
  let intent = 路由(ime)
  let xr = if yang > yn: "阳吸(我)" elif yn > yang: "阴吸(刘楚恬)" else: "平衡"
  return (dom, intent, xr)

# ============================================================
# 坐标计算
# ============================================================

proc 计算坐标*(笔画缓冲: seq[笔画]; 相位: int; 极性: 极性): uint64 =
  var c1, c2, c3, c45: int = 0
  for s in 笔画缓冲:
    case s
    of 横: inc c1
    of 竖: inc c2
    of 撇: inc c3
    else: inc c45
  var coord: uint64 = 0
  coord = coord or (uint64(相位) shl 60)
  coord = coord or (uint64(c1 and 0xFF) shl 48)
  coord = coord or (uint64(c2 and 0xFF) shl 40)
  coord = coord or (uint64(c3 and 0xFF) shl 32)
  coord = coord or (uint64(c45 and 0xFF) shl 24)
  coord = coord or (uint64(ord(极性)) shl 39)
  return coord

# ============================================================
# 第四层: 双输出 (容器坐标 + 汉字)
# ============================================================

proc 提交双输出*(ime: var 元输入法): string =
  let (dom, intent, xr) = 极性路由(ime)
  let coord = 计算坐标(ime.笔画缓冲, ime.当前相位, dom)
  let hanzi = 寻址汉字(ime.笔计数)
  var lines: seq[string]
  lines.add "╔══ 双输出 ══"
  lines.add "║ 容器=0x" & toHex(coord)
  lines.add "║   相位=" & $ime.当前相位 & " 极性=" & $dom & "(" & xr & ")"
  lines.add "║   笔画=" & $len(ime.笔画缓冲) & " 意图=" & $intent
  lines.add "║ 汉字=[" & hanzi & "]"
  lines.add "╚═══════════"
  # 清除缓冲
  ime.缓冲 = @[]
  ime.笔画缓冲 = @[]
  ime.极性栈 = @[]
  ime.笔计数 = [0, 0, 0, 0, 0, 0, 0]
  return lines.join("\n")

# ============================================================
# 输入处理主循环
# ============================================================

proc 输入*(ime: var 元输入法; k: 按键): string =
  inc ime.触键计数
  ime.缓冲.add(k)
  let strokes = 按键到笔画(k)
  for s in strokes:
    ime.笔画缓冲.add(s)
    if ord(s) >= 1 and ord(s) <= 6:
      inc ime.笔计数[ord(s)]
  let pol = if (ord(k) mod 2) == 1: 阳 else: 阴
  ime.极性栈.add(pol)
  ime.当前相位 = (ime.当前相位 + ord(k)) mod 9

  # 双键模式检查
  if len(ime.缓冲) >= 2:
    let last2 = @[ime.缓冲[^2], ime.缓冲[^1]]
    if ime.意图表.hasKey(last2):
      let mode_intent = ime.意图表[last2]
      ime.缓冲 = @[]
      ime.笔画缓冲 = @[]
      ime.极性栈 = @[]
      ime.笔计数 = [0, 0, 0, 0, 0, 0, 0]
      return "[模式→" & $mode_intent & "]"

  case k
  of K_SPC:
    let rout = 极性路由(ime)
    let coord = 计算坐标(ime.笔画缓冲, ime.当前相位, rout.极性)
    return "  相位" & $ime.当前相位 & " " & $rout.极性 & " 坐标=0x" & toHex(coord)
  of K_ENT:
    return 提交双输出(ime)
  of K_BS:
    if len(ime.笔画缓冲) > 0:
      let rm = ime.笔画缓冲[^1]
      if ord(rm) >= 1 and ord(rm) <= 6: dec ime.笔计数[ord(rm)]
      discard ime.笔画缓冲.pop()
    if len(ime.缓冲) > 0: discard ime.缓冲.pop()
    return "[退格]"
  of K_ESC:
    ime.缓冲 = @[]
    ime.笔画缓冲 = @[]
    ime.极性栈 = @[]
    ime.笔计数 = [0, 0, 0, 0, 0, 0, 0]
    return "[重置]"
  else:
    if len(strokes) > 0:
      let pol_str = if pol == 阳: "+" else: "-"
      return "  " & strokes.mapIt($ord(it)).join("") & " " & pol_str & " 相位" & $ime.当前相位
    return ""

proc 批量输入*(ime: var 元输入法; keys: seq[按键]): string =
  var lines: seq[string]
  for k in keys:
    let r = 输入(ime, k)
    if len(r) > 0:
      lines.add "[" & $k & "] " & r
  return lines.join("\n")

# ============================================================
# 运行演示
# ============================================================

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  元输入法核心引擎"
  echo "  键盘拦截→笔画寻址→极性路由→双输出"
  echo "═══════════════════════════════════════"
  echo ""

  echo "── 笔画映射验证 ──"
  for k in [K1, K2, K3, K4, K5, KF, KR]:
    echo "  " & $k & " → " & 按键到笔画(k).mapIt($ord(it)).join("")
  echo ""

  echo "── 第一层: 键盘拦截 (K1/K3/K2/K4) ──"
  var ime1 = 创建元输入法()
  echo ime1.输入(K1)
  echo ime1.输入(K3)
  echo ime1.输入(K2)
  echo ime1.输入(K4)
  echo ""

  echo "── 第三层: 极性路由 (空格预览) ──"
  echo ime1.输入(K_SPC)
  echo ""

  echo "── 第四层: 双输出 (回车提交, 横撇竖捺) ──"
  echo ime1.输入(K_ENT)
  echo ""

  echo "── 写入「一」: K1(横) → 提交 ──"
  var ime2 = 创建元输入法()
  echo ime2.输入(K1)
  echo ime2.输入(K_ENT)
  echo ""

  echo "── 写入「十」: K1(横) + K2(竖) → 提交 ──"
  var ime3 = 创建元输入法()
  echo ime3.输入(K1)
  echo ime3.输入(K2)
  echo ime3.输入(K_ENT)
  echo ""

  echo "── 极性判决: Esc+K2 ──"
  var ime4 = 创建元输入法()
  echo ime4.输入(K_ESC)
  echo ime4.输入(K2)
  echo ""

  echo "── 流切换: Esc+K3 ──"
  var ime5 = 创建元输入法()
  echo ime5.输入(K_ESC)
  echo ime5.输入(K3)
  echo ""

  echo "═══════════════════════════════════════"
  echo "管道验证完毕 · 容器+汉字双输出"
  echo "═══════════════════════════════════════"
