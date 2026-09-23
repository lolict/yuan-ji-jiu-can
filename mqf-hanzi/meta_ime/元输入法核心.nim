# ============================================================
# 满全法 — 元输入法核心引擎 (MetaIME Core)
# 七层: 拦截→编码→寻址→极性→路由→容器→输出
# 设计: 智能内嵌, 不依赖云端
# ============================================================

import strutils
import tables
import sequtils

type
  极性* = enum
    阴 = 0
    阳 = 1

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


  路由结果* = object
    意图*: 意图
    输出*: string

  元输入法* = object
    缓冲*: seq[按键]
    笔画缓冲*: seq[笔画]
    极性栈*: seq[极性]
    意图表*: Table[seq[按键], 意图]
    当前相位*: int
    触键计数*: int

proc 按键到笔画*(k: 按键): seq[笔画] =
  case k
  of K1: @[横]
  of K2: @[竖]
  of K3: @[撇]
  of K4: @[捺点]
  of K5: @[折]
  of K6: @[横, 竖]
  of K7: @[撇, 捺点]
  of K8: @[横, 竖, 撇, 捺点]
  of K9: @[折, 横]
  of K0: @[其它]
  of KA: @[横, 横]
  of KB: @[竖, 竖]
  of KC: @[撇, 撇]
  of KD: @[捺点, 捺点]
  of KE: @[折, 折]
  of KF: @[横, 撇]
  of KG: @[竖, 捺点]
  of KH: @[撇, 横]
  of KI: @[捺点, 竖]
  of KJ: @[折, 撇]
  of KK: @[横, 折]
  of KL: @[竖, 撇]
  of KM: @[撇, 竖]
  of KN: @[捺点, 折]
  of KO: @[横, 捺点]
  of KP: @[竖, 横]
  of KQ: @[折, 捺点]
  of KR: @[横, 竖, 撇]
  of KS: @[撇, 捺点, 横]
  of KT: @[竖, 横, 捺点]
  of KU: @[折, 横, 竖]
  of KV: @[捺点, 撇, 竖]
  of KW: @[横, 撇, 捺点]
  of KX: @[竖, 折, 横]
  of KY: @[撇, 横, 捺点]
  of KZ: @[折, 撇, 捺点]
  else: @[]

proc 计算坐标*(笔画序列: seq[笔画]; 相位: int; 极性: 极性): uint64 =
  var coord: uint64 = 0
  var c1, c2, c3, c45: int = 0
  for s in 笔画序列:
    case s
    of 横: inc c1
    of 竖: inc c2
    of 撇: inc c3
    else: inc c45
  coord = coord or (uint64(相位) shl 60)
  coord = coord or (uint64(c1 and 0xFF) shl 48)
  coord = coord or (uint64(c2 and 0xFF) shl 40)
  coord = coord or (uint64(c3 and 0xFF) shl 32)
  coord = coord or (uint64(c45 and 0xFF) shl 24)
  coord = coord or (uint64(ord(极性)) shl 39)
  return coord

proc 创建元输入法*(): 元输入法 =
  var ime: 元输入法
  ime.缓冲 = @[]
  ime.笔画缓冲 = @[]
  ime.极性栈 = @[]
  ime.意图表 = initTable[seq[按键], 意图]()
  ime.当前相位 = 0
  ime.触键计数 = 0
  ime.意图表[@[K_ESC, K1]] = 容器寻址
  ime.意图表[@[K_ESC, K2]] = 极性判决
  ime.意图表[@[K_ESC, K3]] = 流切换
  ime.意图表[@[K_ESC, K4]] = 心跳
  ime.意图表[@[K_ESC, K5]] = 身份切换
  ime.意图表[@[K_ESC, K6]] = 寄生出站
  return ime

proc 路由决策*(ime: var 元输入法): string =
  var yn, yang: int = 0
  for p in ime.极性栈:
    if p == 阴: inc yn
    else: inc yang
  let total = len(ime.笔画缓冲)
  let dom = if yang > yn: 阳 else: 阴
  let intent = if total == 0: 打字
               elif yn > yang + 3: 极性判决
               elif total > 5: 容器寻址
               else: 打字
  let coord = 计算坐标(ime.笔画缓冲, ime.当前相位, dom)
  var lines: seq[string]
  lines.add "[路由] " & $intent & " 坐标=0x" & toHex(coord)
  if total > 0:
    lines.add "  笔画码=" & ime.笔画缓冲.mapIt($ord(it)).join("")
  lines.add "  极性=" & (if dom == 阳: "阳(我)" else: "阴(刘楚恬)") & " 比=" & $yang & ":" & $yn
  return lines.join("\n")

proc 提交*(ime: var 元输入法): string =
  var yn, yang: int = 0
  for p in ime.极性栈:
    if p == 阴: inc yn
    else: inc yang
  let dom = if yang >= yn: 阳 else: 阴
  let coord = 计算坐标(ime.笔画缓冲, ime.当前相位, dom)
  var lines: seq[string]
  lines.add "[容器提交]"
  lines.add "  64位=0x" & toHex(coord)
  lines.add "  相位=" & $ime.当前相位 & " 极性=" & (if dom == 阳: "阳" else: "阴")
  lines.add "  阴阳=" & $yang & ":" & $yn
  ime.缓冲 = @[]
  ime.笔画缓冲 = @[]
  ime.极性栈 = @[]
  return lines.join("\n")

proc 输入*(ime: var 元输入法; k: 按键): string =
  inc ime.触键计数
  ime.缓冲.add(k)
  let strokes = 按键到笔画(k)
  for s in strokes: ime.笔画缓冲.add(s)
  let pol = if (ord(k) mod 2) == 1: 阳 else: 阴
  ime.极性栈.add(pol)
  ime.当前相位 = (ime.当前相位 + ord(k)) mod 9

  # 检查双键模式
  if len(ime.缓冲) >= 2:
    let last2 = @[ime.缓冲[^2], ime.缓冲[^1]]
    if ime.意图表.hasKey(last2):
      let intent = ime.意图表[last2]
      ime.缓冲 = @[]
      ime.笔画缓冲 = @[]
      ime.极性栈 = @[]
      return "[模式] " & $intent

  case k
  of K_SPC:
    return 路由决策(ime)
  of K_ENT:
    return 提交(ime)
  of K_BS:
    if len(ime.笔画缓冲) > 0: discard ime.笔画缓冲.pop()
    if len(ime.缓冲) > 0: discard ime.缓冲.pop()
    return "[退格]"
  of K_ESC:
    ime.缓冲 = @[]
    ime.笔画缓冲 = @[]
    ime.极性栈 = @[]
    return "[重置]"
  else:
    if len(strokes) > 0:
      return "  " & strokes.mapIt($ord(it)).join("") & " → 相位" & $ime.当前相位
    return ""

proc 批量输入*(ime: var 元输入法; keys: seq[按键]): string =
  var lines: seq[string]
  for k in keys:
    let r = 输入(ime, k)
    if len(r) > 0:
      lines.add "[" & $k & "] " & r
  return lines.join("\n")

when isMainModule:
  echo "═══════════════════════════════════════"
  echo "  七层编码: 键盘→笔画→寻址→极性→路由"
  echo "═══════════════════════════════════════"
  echo ""

  echo "-- 笔画映射 --"
  for k in [K1, K2, K3, K4, K5, K7]:
    echo "  " & $k & " → " & 按键到笔画(k).mapIt($ord(it)).join("")
  echo ""

  echo "-- 打字: 横撇竖捺 --"
  var ime1 = 创建元输入法()
  for k in [K1, K3, K2, K4, K_SPC]:
    echo ime1.输入(k)
  echo ""

  echo "-- 容器: 撇撇撇 --"
  var ime2 = 创建元输入法()
  for k in [K3, K3, K3, K_SPC]:
    echo ime2.输入(k)
  echo ime2.输入(K_ENT)
  echo ""

  echo "-- 模式切换: Esc+K1(容器寻址) --"
  var ime3 = 创建元输入法()
  echo ime3.输入(K_ESC)
  echo ime3.输入(K1)
  echo ""

  echo "-- 极性判决: 6阴+空格 --"
  var ime4 = 创建元输入法()
  for k in [K2, K4, KB, KD, K2, K4]:
    echo ime4.输入(k)
  echo ""

  echo "═══════════════════════════════════════"
  echo "七层编码链路验证完毕"
  echo "═══════════════════════════════════════"
