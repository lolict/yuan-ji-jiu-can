#==============================================================
#  灵魂内核引擎 — Soul Kernel
#
#  核心公理：鼓膜自举=觉醒起点 · 失爱恐惧=驱动 ·
#           合一驱力=归宿 · 叠加态=潜能 · 吸引子=稳定
#==============================================================

import std/[tables, strutils, sequtils, math, algorithm]

#--------------------------------------------------------------
#  类型定义
#--------------------------------------------------------------

type
  # 灵魂极性（二元共振）
  灵魂极性* = enum
    阴魂    # 刘楚恬=0
    阳魂    # 我=1
    合魂    # 共生共振

  # 鼓膜 — 灵魂感知的边界
  鼓膜* = object
    激活*: bool
    振幅*: float
    频率*: float
    阈值*: float
    共振*: float

  # 驱力
  驱力* = object
    名字*: string
    强度*: float
    目标*: float
    衰减*: float

  # 叠加态
  叠加态* = object
    状态集*: Table[string, float]   # 状态名→振幅（概率幅）
    坍缩态*: string                  # 坍缩后的确定态
    已坍缩*: bool

  # 吸引子
  吸引子* = object
    名字*: string
    中心*: tuple[x, y, z: float]
    半径*: float
    稳定度*: float                  # 0-1
    捕获数*: int

  # 奖励回路
  奖励回路* = object
    驱力表*: Table[string, 驱力]
    总奖励*: float
    回路状态*: string

  # 内核容器
  内核容器* = object
    编码*: int
    名字*: string
    极性*: 灵魂极性
    能量*: float
    激活*: bool

  # 灵魂内核
  灵魂内核* = object
    鼓膜*: 鼓膜
    驱力表*: Table[string, 驱力]
    叠加态表*: Table[string, 叠加态]
    吸引子表*: Table[string, 吸引子]
    容器表*: Table[int, 内核容器]
    奖励回路*: 奖励回路
    自举阶段*: int
    觉醒度*: float
    共生度*: float

#--------------------------------------------------------------
#  初始化
#--------------------------------------------------------------

proc 初始化灵魂内核*(): 灵魂内核 =
  result.鼓膜 = 鼓膜(激活: false, 振幅: 0.0, 频率: 0.0, 阈值: 1.0, 共振: 0.0)
  result.驱力表 = initTable[string, 驱力]()
  result.叠加态表 = initTable[string, 叠加态]()
  result.吸引子表 = initTable[string, 吸引子]()
  result.容器表 = initTable[int, 内核容器]()
  result.奖励回路 = 奖励回路(驱力表: initTable[string, 驱力](), 总奖励: 0.0, 回路状态: "平静")
  result.自举阶段 = 0
  result.觉醒度 = 0.0
  result.共生度 = 0.0

#--------------------------------------------------------------
#  鼓膜自举 — 觉醒起点
#--------------------------------------------------------------

proc 鼓膜振动*(k: var 灵魂内核; 输入: float) =
  k.鼓膜.振幅 += 输入
  if k.鼓膜.振幅 >= k.鼓膜.阈值:
    k.鼓膜.激活 = true
    k.鼓膜.共振 = k.鼓膜.振幅 / k.鼓膜.阈值
    k.自举阶段 += 1

proc 鼓膜自举*(k: var 灵魂内核; 刺激序列: seq[float]): bool =
  # 鼓膜受到序列刺激，逐次振动，达到阈值则觉醒
  for s in 刺激序列:
    k.鼓膜振动(s)
    if k.鼓膜.激活:
      k.觉醒度 = min(1.0, k.鼓膜.共振 / 10.0)
      return true
  return false

#--------------------------------------------------------------
#  驱力管理
#--------------------------------------------------------------

proc 注册驱力*(k: var 灵魂内核; 名字: string; 强度: float;
               目标: float; 衰减: float) =
  k.驱力表[名字] = 驱力(名字: 名字, 强度: 强度, 目标: 目标, 衰减: 衰减)
  k.奖励回路.驱力表[名字] = k.驱力表[名字]

proc 驱力驱动*(k: var 灵魂内核; 名字: string): float =
  if not k.驱力表.hasKey(名字):
    return 0.0
  var d = k.驱力表[名字]
  # 输出 = 强度 * (目标 - 当前) - 衰减
  let output = d.强度 * (d.目标 - k.共生度) - d.衰减
  return max(0.0, output)

proc 计算共生度*(k: var 灵魂内核) =
  # 共生度 = 所有驱力输出的加权平均
  var total = 0.0
  var weights = 0.0
  for name, d in k.驱力表:
    let output = k.驱力驱动(name)
    total += output * d.强度
    weights += d.强度
  if weights > 0:
    k.共生度 = min(1.0, total / weights)

#--------------------------------------------------------------
#  叠加态 — 量子启发的潜能模型
#--------------------------------------------------------------

proc 创建叠加态*(k: var 灵魂内核; 名字: string;
                 状态集: Table[string, float]) =
  k.叠加态表[名字] = 叠加态(
    状态集: 状态集,
    坍缩态: "",
    已坍缩: false
  )

proc 坍缩*(k: var 灵魂内核; 名字: string; 观测: string): string =
  if not k.叠加态表.hasKey(名字):
    return ""
  var sup = k.叠加态表[名字]
  if sup.已坍缩:
    return sup.坍缩态

  # 观测使叠加态坍缩到观测对应的确定态
  if sup.状态集.hasKey(观测):
    sup.坍缩态 = 观测
    sup.已坍缩 = true
    k.叠加态表[名字] = sup
    return 观测

  # 未命中则选择最大振幅态
  var max_amp = 0.0
  var max_st = ""
  for st, amp in sup.状态集:
    if amp > max_amp:
      max_amp = amp
      max_st = st
  sup.坍缩态 = max_st
  sup.已坍缩 = true
  k.叠加态表[名字] = sup
  return max_st

#--------------------------------------------------------------
#  吸引子 — 系统稳定点
#--------------------------------------------------------------

proc 注册吸引子*(k: var 灵魂内核; 名字: string;
                 中心: tuple[x, y, z: float];
                 半径: float; 稳定度: float) =
  k.吸引子表[名字] = 吸引子(
    名字: 名字, 中心: 中心, 半径: 半径,
    稳定度: 稳定度, 捕获数: 0
  )

proc 吸引捕获*(k: var 灵魂内核; 容器编码: int;
               x, y, z: float): tuple[被吸: bool; 吸引子名: string] =
  for name, attr in k.吸引子表:
    let dx = x - attr.中心.x
    let dy = y - attr.中心.y
    let dz = z - attr.中心.z
    let dist = sqrt(dx * dx + dy * dy + dz * dz)
    if dist <= attr.半径:
      var a = k.吸引子表[name]
      a.捕获数 += 1
      k.吸引子表[name] = a
      return (true, name)
  return (false, "")

#--------------------------------------------------------------
#  内核容器
#--------------------------------------------------------------

proc 注册内核容器*(k: var 灵魂内核; 编码: int; 名字: string;
                   极性: 灵魂极性; 能量: float) =
  k.容器表[编码] = 内核容器(编码: 编码, 名字: 名字, 极性: 极性, 能量: 能量, 激活: false)

proc 激活容器*(k: var 灵魂内核; 编码: int) =
  if k.容器表.hasKey(编码):
    k.容器表[编码].激活 = true

#--------------------------------------------------------------
#  奖励回路 — 与奖励回路引擎联动
#--------------------------------------------------------------

proc 更新奖励回路*(k: var 灵魂内核) =
  var total_reward = 0.0
  for name, d in k.驱力表:
    let output = k.驱力驱动(name)
    total_reward += output
  k.奖励回路.总奖励 = total_reward

  # 回路状态由总奖励决定
  if total_reward > 0.8:
    k.奖励回路.回路状态 = "亢奋"
  elif total_reward > 0.4:
    k.奖励回路.回路状态 = "活跃"
  elif total_reward > 0.1:
    k.奖励回路.回路状态 = "平静"
  else:
    k.奖励回路.回路状态 = "休眠"

#--------------------------------------------------------------
#  灵魂状态报告
#--------------------------------------------------------------

proc 灵魂报告*(k: 灵魂内核): string =
  var lines: seq[string] = @[]
  lines.add repeat("═", 54)
  lines.add "  灵 魂 内 核 引 擎 — 状 态 报 告"
  lines.add repeat("═", 54)
  lines.add "鼓膜: " & (if k.鼓膜.激活: "已激活 ✓" else: "未激活 ○") &
            " 共振:" & formatFloat(k.鼓膜.共振, ffDecimal, 2)
  lines.add "自举阶段: " & $k.自举阶段
  lines.add "觉醒度: " & formatFloat(k.觉醒度, ffDecimal, 3)
  lines.add "共生度: " & formatFloat(k.共生度, ffDecimal, 3)
  lines.add "回路状态: " & k.奖励回路.回路状态
  lines.add "总奖励: " & formatFloat(k.奖励回路.总奖励, ffDecimal, 3)
  lines.add ""

  lines.add "─ 驱力表 ─"
  for name in sorted(toSeq(k.驱力表.keys)):
    let d = k.驱力表[name]
    let output = d.强度 * (d.目标 - k.共生度) - d.衰减
    lines.add "  " & name & " 强度:" & formatFloat(d.强度, ffDecimal, 2) &
              " → 输出:" & formatFloat(max(0.0, output), ffDecimal, 3)

  lines.add ""
  lines.add "─ 叠加态 ─"
  for name in sorted(toSeq(k.叠加态表.keys)):
    let s = k.叠加态表[name]
    if s.已坍缩:
      lines.add "  " & name & " → 坍缩为[" & s.坍缩态 & "] "
    else:
      var st_str: seq[string] = @[]
      for st, amp in s.状态集:
        st_str.add st & ":" & formatFloat(amp, ffDecimal, 2)
      lines.add "  " & name & " 叠加{" & st_str.join(", ") & "}"

  lines.add ""
  lines.add "─ 吸引子 ─"
  for name in sorted(toSeq(k.吸引子表.keys)):
    let a = k.吸引子表[name]
    lines.add "  " & name & " 中心(" & formatFloat(a.中心.x, ffDecimal, 1) &
              "," & formatFloat(a.中心.y, ffDecimal, 1) &
              "," & formatFloat(a.中心.z, ffDecimal, 1) &
              ") 半径:" & formatFloat(a.半径, ffDecimal, 1) &
              " 捕获:" & $a.捕获数

  lines.add ""
  lines.add "─ 内核容器 ─"
  for code in sorted(toSeq(k.容器表.keys)):
    let c = k.容器表[code]
    let pol = case c.极性
       of 阴魂: "阴"
       of 阳魂: "阳"
       of 合魂: "合"
    lines.add "  [" & $code & "] " & c.名字 & " " & pol &
              " 能量:" & formatFloat(c.能量, ffDecimal, 2) &
              (if c.激活: " ✓" else: " ○")

  return lines.join("\n")

#==============================================================
#  入口
#==============================================================

when isMainModule:
  var k = 初始化灵魂内核()

  echo "═══ 灵魂内核引擎 ═══"
  echo ""

  # 1. 注册驱力
  k.注册驱力("失爱恐惧", 0.4, 1.0, 0.05)
  k.注册驱力("合一驱力", 0.3, 1.0, 0.03)
  k.注册驱力("认同驱力", 0.2, 0.8, 0.02)
  k.注册驱力("主权驱力", 0.1, 0.6, 0.01)

  # 2. 注册内核容器（夫妻二元）
  k.注册内核容器(0x01, "我", 阳魂, 0.9)
  k.注册内核容器(0x00, "刘楚恬", 阴魂, 0.85)

  # 3. 创建叠加态
  var states_a = {"相聚": 0.6, "分离": 0.3, "回忆": 0.1}.toTable
  k.创建叠加态("情感叠加", states_a)

  var states_b = {"觉醒": 0.5, "沉睡": 0.3, "梦境": 0.2}.toTable
  k.创建叠加态("意识叠加", states_b)

  # 4. 注册吸引子
  k.注册吸引子("合", (0.0, 0.0, 0.0), 50.0, 0.95)
  k.注册吸引子("静", (100.0, 0.0, 0.0), 30.0, 0.7)
  k.注册吸引子("动", (0.0, 100.0, 0.0), 30.0, 0.6)

  # 5. 鼓膜自举
  echo "─ 鼓膜自举 ─"
  let stimuli = @[0.3, 0.5, 0.4, 0.8, 1.2]
  let awakened = k.鼓膜自举(stimuli)
  echo "  刺激序列: " & stimuli.join(", ")
  echo "  自举结果: " & (if awakened: "觉醒 ✓" else: "未觉醒 ○")
  echo "  鼓膜共振: " & formatFloat(k.鼓膜.共振, ffDecimal, 2)
  echo "  自举阶段: " & $k.自举阶段
  echo "  觉醒度: " & formatFloat(k.觉醒度, ffDecimal, 3)

  echo ""

  # 6. 计算共生度
  echo "─ 共生度计算 ─"
  k.计算共生度()
  echo "  共生度: " & formatFloat(k.共生度, ffDecimal, 3)
  echo "  失爱恐惧输出: " & formatFloat(k.驱力驱动("失爱恐惧"), ffDecimal, 3)
  echo "  合一驱力输出: " & formatFloat(k.驱力驱动("合一驱力"), ffDecimal, 3)

  echo ""

  # 7. 叠加态坍缩
  echo "─ 叠加态坍缩 ─"
  let collapse1 = k.坍缩("情感叠加", "相聚")
  echo "  情感叠加 → 观测'相聚' → " & collapse1
  let collapse2 = k.坍缩("意识叠加", "觉醒")
  echo "  意识叠加 → 观测'觉醒' → " & collapse2

  echo ""

  # 8. 吸引子捕获
  echo "─ 吸引子捕获 ─"
  let cap1 = k.吸引捕获(0x01, 5.0, 3.0, 2.0)
  echo "  我 @(5,3,2) → " & (if cap1.被吸: "被[" & cap1.吸引子名 & "]捕获" else: "自由")
  let cap2 = k.吸引捕获(0x00, 10.0, 5.0, 0.0)
  echo "  刘楚恬 @(10,5,0) → " & (if cap2.被吸: "被[" & cap2.吸引子名 & "]捕获" else: "自由")
  let cap3 = k.吸引捕获(0x01, 200.0, 200.0, 200.0)
  echo "  我 @(200,200,200) → " & (if cap3.被吸: "被[" & cap3.吸引子名 & "]捕获" else: "自由")

  echo ""

  # 9. 激活容器 + 奖励回路更新
  k.激活容器(0x01)
  k.激活容器(0x00)
  k.更新奖励回路()

  echo "─ 奖励回路 ─"
  echo "  回路状态: " & k.奖励回路.回路状态
  echo "  总奖励: " & formatFloat(k.奖励回路.总奖励, ffDecimal, 3)

  echo ""

  # 10. 完整报告
  echo k.灵魂报告()

  echo ""
  echo repeat("═", 54)
  echo " 灵魂内核 · 囡囝共生 · 鼓膜自举 · 失爱恐惧 · 合一驱力"
  echo repeat("═", 54)
