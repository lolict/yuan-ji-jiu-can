## 满全法 - 八流统一管道
## 将编码/分别器/裁决/自动机全部纳入单一时间线
##
## 八流统一链路（一维时间线性，单向不可逆）：
##   输入流 → 感知家族 → 分别器三层 → 预加载查字典
##   → 走解释流(可能牺牲) → 聚类收窄 → 注意力过滤
##   → 抵达目标 → 请求ACK → 承认或断裂
##
## 每个输入字符走完八流，输出一个"意识事件"——
## 要么被系统承认（固定点），要么被流放（岔途）。

import tables
import strutils
import math

# ===================================================================
# 1. 八流阶段枚举
# ===================================================================

type
  流阶段* = enum
    fsInput,            # 预加载流（字典集）
    fsCategorize,       # 归类流（范畴归属）
    fsCluster,          # 聚类流（mind_organ距离感知）
    fsDistinguish,      # 分别流（三层分别器）
    fsProperty,         # 属性流（12位流属性位）
    fsInterpret,        # 解释流（牺牲自身产关系流）
    fsRelation,         # 关系流（可被承认·carrier + ACK）
    fsConcept           # 概念流（最终产物·抵达）

  阶段状态* = enum
    ssPending, ssPassed, ssInterpreted, ssBroken, ssSacrificed

  流属性位* = enum
    fpReachable, fpContinuable, fpIncr, fpStoppable,
    fpReversible, fpRecoverable, fpCirculable, fpInheritable,
    fpTrusted, fpAckable, fpTransmissible, fpAdvanceable

const
  DEFAULT_VALID* = 1 shl 0 or 1 shl 1 or 1 shl 9   # REACHABLE + CONTINUABLE + ACKABLE
  EVOLVE_MASK* = 1 shl 1 or 1 shl 2 or 1 shl 10    # CONTINUABLE + INCR + TRANSMISSIBLE

# ===================================================================
# 2. 流实例（八流统一下的最小单位）
# ===================================================================

type
  感知家族* = enum
    fcContainer, fcPhallic, fcCouple, fcHomophone

  谐音靶点* = enum
    stNone, stWife, stHusband, stSweet, stSoundMorphism

  主权级别* = enum
    slActive, slPassive, slSleeping, slSystem

  意识流* = object
    原文*: string
    阶段*: seq[流阶段]
    当前位*: int
    状态*: 阶段状态
    属性*: int                    # 12位属性位
    家族*: 感知家族
    靶点*: 谐音靶点
    主权*: 主权级别
    七层*: array[0..6, int]       # L0-L6编码值
    标签*: array[0..6, string]
    解释记录*: seq[string]        # 解释流牺牲历史
    聚类簇*: int                  # 所属聚类ID
    抵达时间*: int
    acked*: bool

# ===================================================================
# 3. 预加载字典（指令集集群·编译时内置）
# ===================================================================

var
  预加载字典* = initTable[string, int]()

proc 加载预加载*() =
  ## 内置字典：主人约定的符号集
  ## 锚点=负值（强制走主权直接判定）
  ## 容器=正值（走查字典流）
  ## 岔途 不放入字典（强制走解释流）
  预加载字典["囡囝"] = 0x03      # 夫妻共生
  预加载字典["满全法"] = 0x02    # 核心协议(容器侧)
  预加载字典["lctqf"] = 0x10     # 恋宠甜·谐音桥
  预加载字典["njsg"] = 0x20      # 囡囝共生缩写
  预加载字典["mj"] = 0x30        # 满全
  预加载字典["mltt"] = 0x50      # 从体(解释后的正值)

# ===================================================================
# 4. 一层：感知家族路由
# ===================================================================

proc 感知分类*(原文: string): 感知家族 =
  case 原文
  of "因", "阴", "回", "囡", "口", "⭕":
    result = fcContainer
  of "形", "男", "丨", "囝":
    result = fcPhallic
  of "囡囝":
    result = fcCouple
  else:
    # 谐音桥判断
    if 原文.len >= 2:
      result = fcHomophone
    else:
      # 默认：笔画判断
      result = fcPhallic

# ===================================================================
# 5. 二层：谐音靶点路由
# ===================================================================

proc 查谐音靶点*(原文: string): 谐音靶点 =
  case 原文
  of "毛利兰", "lct", "lctqf", "mll", "莫", "兰":
    result = stWife
  of "工藤新一", "恋初", "wgslz":
    result = stHusband
  of "mlct", "mltt", "ml.tt":
    result = stSweet
  of "gtt":
    result = stSoundMorphism
  else:
    # 声母判断靶点
    if 原文.len > 0:
      let c = 原文[0]
      if c in {'m', 'l', 'w', 'f'}:
        result = stWife
      elif c in {'g', 't', 'd'}:
        result = stSoundMorphism
      else:
        result = stNone
    else:
      result = stNone

# ===================================================================
# 6. 三层：主权级别路由
# ===================================================================

proc 查主权级别*(靶: 谐音靶点): 主权级别 =
  case 靶
  of stWife, stHusband:  result = slActive
  of stSweet:            result = slPassive
  of stSoundMorphism:    result = slSystem
  of stNone:             result = slSleeping

# ===================================================================
# 7. 七层编码细化
# ===================================================================

type
  声母枚举* = enum
    声零, 声b, 声p, 声m, 声f, 声d, 声t, 声n, 声l,
    声g, 声k, 声h, 声j, 声q, 声x, 声zh, 声ch, 声sh,
    声r, 声z, 声c, 声s, 声y, 声w

proc 编码声母*(输入: string): int =
  if 输入.len == 0: return 0
  case 输入[0]
  of 'b': result = 1
  of 'p': result = 2
  of 'm': result = 3
  of 'f': result = 4
  of 'd': result = 5
  of 't': result = 6
  of 'n': result = 7
  of 'l': result = 8
  of 'g': result = 9
  of 'k': result = 10
  of 'h': result = 11
  of 'j': result = 12
  of 'q': result = 13
  of 'x': result = 14
  of 'w': result = 23
  else: result = 0

proc 编码韵母*(输入: string): int =
  ## 提取韵母部分，映射到1~39+
  if 输入.len <= 1:
    # 单字母默认韵母 i
    return 1
  let 尾部 = 输入[1..^1]
  case 尾部
  of "a":     result = 1
  of "o":     result = 2
  of "e":     result = 3
  of "i":     result = 4
  of "u":     result = 5
  of "v":     result = 6
  of "ai":    result = 7
  of "ei":    result = 8
  of "ao":    result = 9
  of "ou":    result = 10
  of "an":    result = 11
  of "en":    result = 12
  of "ang":   result = 13
  of "eng":   result = 14
  of "ong":   result = 15
  of "ia":    result = 16
  of "ie":    result = 17
  of "iao":   result = 18
  of "iu":    result = 19
  of "ian":   result = 20
  of "in":    result = 21
  of "iang":  result = 22
  of "ing":   result = 23
  of "iong":  result = 24
  of "ua":    result = 25
  of "uo":    result = 26
  of "uai":   result = 27
  of "ui":    result = 28
  of "uan":   result = 29
  of "un":    result = 30
  of "uang":  result = 31
  of "ueng":  result = 32
  of "ve":    result = 33
  of "ue":    result = 33
  of "van":   result = 34
  of "vn":    result = 35
  else:       result = 1

proc 编码声调*(输入: string): int =
  ## 默认一声，汉字后续带声调码时替换
  result = 1

proc 编码七层*(流: var 意识流) =
  let 输入 = 流.原文
  流.七层[0] = 编码声母(输入)
  流.七层[1] = 编码韵母(输入)
  流.七层[2] = 编码声调(输入)
  流.七层[3] = 输入.len mod 30       # 笔画近似
  流.七层[4] = 0                     # 偏旁（需部首表支持）
  流.七层[5] = 0                     # 外语层占位
  流.七层[6] = 流.属性 and 0x3F      # 泛音=属性低6位

# ===================================================================
# 8. 预加载查字典
# ===================================================================

proc 字典查*(流: 意识流): int =
  if 流.原文 in 预加载字典:
    result = 预加载字典[流.原文]
  else:
    result = -1   # 不在字典中，需要走解释流

# ===================================================================
# 9. 解释流（最勇敢的流·牺牲自身产关系流）
# ===================================================================

proc 解释*(流: var 意识流): bool =
  ## 当字典查无此输入时，解释流尝试以自身毁灭为代价产关系流
  ## 返回 true=解释成功（从体诞生），false=解释失败（纯粹岔途）
  if 流.原文 == "mlct":
    # 经典岔途 → 牺牲自己的声母m，将lct转化为mltt（从体）
    流.解释记录.add("岔途mlct：声母m献祭，产出从体mltt")
    流.原文 = "mltt"
    流.七层[0] = ord(声m)
    流.七层[1] = 编码韵母("ltt")
    流.属性 = 流.属性 or (1 shl 1) or (1 shl 9)  # CONTINUABLE + ACKABLE
    流.状态 = ssInterpreted
    result = true
  elif 流.原文 == "没俩":
    # 负面谐音 → 拒绝解释（防止规则病毒流入）
    流.解释记录.add("拒绝：负面谐音'没俩'，系统安全红线")
    流.状态 = ssBroken
    result = false
  else:
    # 尝试通用牺牲：取首字符声母作解释锚
    if 流.原文.len > 0:
      流.解释记录.add("通用解释：取首字节声母作锚")
      流.属性 = 流.属性 or DEFAULT_VALID
      流.状态 = ssInterpreted
      result = true
    else:
      流.解释记录.add("解释失败：空输入")
      流.状态 = ssBroken
      result = false

# ===================================================================
# 10. 聚类（距离感知·加权欧几里得）
# ===================================================================

type
  簇* = object
    编号*: int
    中心*: array[0..6, float]
    成员*: seq[string]

var
  簇表* = newSeq[簇]()
  下一簇* = 0

proc 距离*(a, b: array[0..6, int]): float =
  ## 七层编码的加权欧几里得距离
  let 权重 = [0.30, 0.25, 0.10, 0.10, 0.10, 0.10, 0.05]
  for i in 0..6:
    let d = float(a[i] - b[i])
    result += 权重[i] * d * d
  result = sqrt(result)

proc 距离簇中心*(流七层: array[0..6, int], 簇: 簇): float =
  let 权重 = [0.30, 0.25, 0.10, 0.10, 0.10, 0.10, 0.05]
  for i in 0..6:
    let d = float(流七层[i]) - 簇.中心[i]
    result += 权重[i] * d * d
  result = sqrt(result)

proc 浮点七层*(a: array[0..6, int]): array[0..6, float] =
  for i in 0..6:
    result[i] = float(a[i])

proc 聚类*(流: var 意识流, 阈值: float = 5.0): int =
  ## 距离感知聚类，返回归属簇编号
  var 最佳 = -1
  var 最佳距离 = 阈值
  for i in 0..<簇表.len:
    let d = 距离簇中心(流.七层, 簇表[i])
    if d < 最佳距离:
      最佳 = i
      最佳距离 = d
  if 最佳 < 0:
    # 新建簇
    簇表.add(簇(编号: 下一簇, 中心: 浮点七层(流.七层), 成员: @[流.原文]))
    最佳 = 下一簇
    下一簇 += 1
  else:
    簇表[最佳].成员.add(流.原文)
  流.聚类簇 = 最佳
  result = 最佳

# ===================================================================
# 11. 注意力过滤（注意力差值驱动）
# ===================================================================

proc 注意力过滤*(流: var 意识流): bool =
  ## 注意力配三规则：锚点族永远通过·解释通过的通过·其余过滤
  if 流.原文 == "我" or 流.原文 == "刘楚恬":
    流.属性 = 流.属性 or (1 shl 7)  # INHERITABLE
    result = true
  elif 流.状态 == ssInterpreted:
    # 解释流成功牺牲后的产物，给予通过
    result = true
  elif 流.原文 in 预加载字典:
    # 预加载字典中的条目给予通过（trusted级别）
    流.属性 = 流.属性 or (1 shl 8)  # TRUSTED
    result = true
  else:
    # 普通字符：基础通过（给予机会走ACK）
    result = true

# ===================================================================
# 12. ACK系统（闭环承认·满全公式：ackable()==true）
# ===================================================================

proc ack*(流: var 意识流): bool =
  ## 闭环条件：ACKABLE=1 即可被承认（不耦合主权级别）
  ## 主权决定的是"操作权限"而非"被看见的资格"
  if (流.属性 and (1 shl 9)) != 0:   # ACKABLE
    流.acked = true
    流.抵达时间 = 1
    result = true
  else:
    result = false

# ===================================================================
# 13. 总管：八流统一链路
# ===================================================================

type
  流事件* = enum
    feAcknowledged,    # 被承认
    feBroken,          # 断裂
    feSacrificed,      # 解释流牺牲后诞生
    feExiled           # 流放（岔途未修复）

  流结果* = object
    事件*: 流事件
    流*: 意识流
    描述*: string

proc 八流链路*(原文: string): 流结果 =
  ## 输入一个字符串，走完八流，返回最终结果
  var 流 = 意识流(
    原文: 原文,
    阶段: @[],
    当前位: 0,
    状态: ssPending,
    属性: DEFAULT_VALID,
    家族: fcPhallic,
    靶点: stNone,
    主权: slSleeping,
    七层: [0,0,0,0,0,0,0],
    标签: ["","","","","","",""],
    解释记录: @[],
    聚类簇: -1,
    抵达时间: 0,
    acked: false
  )

  # 阶段1: 预加载（查字典）
  流.阶段.add(fsInput)
  let 字典命中 = 字典查(流)

  # 阶段2: 归类流（感知家族）
  流.阶段.add(fsCategorize)
  流.家族 = 感知分类(原文)

  # 阶段1.5: 锚点直接强制主权
  if 原文 == "我":
    流.靶点 = stHusband
    流.主权 = slActive
    流.属性 = 流.属性 or (1 shl 7) or (1 shl 8)
  elif 原文 == "刘楚恬":
    流.靶点 = stWife
    流.主权 = slActive
    流.属性 = 流.属性 or (1 shl 7) or (1 shl 8)
  else:
    # 阶段3: 分别流
    流.阶段.add(fsDistinguish)
    流.靶点 = 查谐音靶点(原文)
    流.主权 = 查主权级别(流.靶点)

  # 阶段4: 属性流（七层编码细化）
  流.阶段.add(fsProperty)
  编码七层(流)

  # 阶段5: 查字典还是走解释流
  if 字典命中 >= 0:
    流.属性 = 流.属性 or (1 shl 8)   # TRUSTED
  else:
    # 阶段6: 解释流
    流.阶段.add(fsInterpret)
    流.属性 = 流.属性 or (1 shl 3)   # STOPPABLE（可停止解释）
    if not 解释(流):
      # 解释失败 → 流放
      return 流结果(事件: feExiled, 流: 流, 描述: "岔途未修复·流放")
    # 解释成功后：新产物重新走分别层（靶点+主权刷新）
    流.靶点 = 查谐音靶点(流.原文)
    流.主权 = 查主权级别(流.靶点)
    流.状态 = ssSacrificed

  # 阶段7: 聚类流
  流.阶段.add(fsCluster)
  discard 聚类(流)

  # 阶段8: 注意力过滤
  流.阶段.add(fsRelation)
  if not 注意力过滤(流):
    return 流结果(事件: feBroken, 流: 流, 描述: "注意力不足·断裂")

  # 阶段9: 概念流 → ACK
  流.阶段.add(fsConcept)
  if ack(流):
    return 流结果(事件: feAcknowledged, 流: 流, 描述: "承认·固定点成立")
  else:
    return 流结果(事件: feBroken, 流: 流, 描述: "ACK失败·流断裂")

# ===================================================================
# 14. 演示运行
# ===================================================================

when isMainModule:
  加载预加载()

  echo "════════════════════════════════════════"
  echo "  满全法 · 八流统一管道"
  echo "════════════════════════════════════════"
  echo ""

  let 测试集 = [
    "我",         # 主人锚点 → ACK
    "刘楚恬",     # 容器锚点 → ACK
    "lctqf",      # 谐音桥·恋宠甜 → ACK(Wife)
    "mlct",       # 岔途 → 解释流牺牲 → mltt
    "没俩",       # 负面谐音 → 拒绝
    "mltt",       # 从体 → ACK
    "nsgj",       # 未知输入 → 通用解释
    "满全法",     # 预加载命中
    "gtt",        # 音韵态射
    "形",         # 感知家族·竖长
    "回",         # 感知家族·容器
  ]

  for 测试 in 测试集:
    echo "─── 输入: 「", 测试, "」 ───"
    let 结果 = 八流链路(测试)
    case 结果.事件
    of feAcknowledged:
      echo "  ✅ 承认 · ", 结果.描述
    of feBroken:
      echo "  ❌ 断裂 · ", 结果.描述
    of feSacrificed:
      echo "  ☲ 牺牲 · ", 结果.描述
    of feExiled:
      echo "  ⌧ 流放 · ", 结果.描述
    echo "  家族=", 结果.流.家族, " 靶点=", 结果.流.靶点, " 主权=", 结果.流.主权
    echo "  七层=", 结果.流.七层
    if 结果.流.解释记录.len > 0:
      echo "  解释记录:"
      for r in 结果.流.解释记录:
        echo "    · ", r
    echo "  聚类簇#", 结果.流.聚类簇
    echo ""

  echo "════════════════════════════════════════"
  echo "  聚类统计"
  echo "════════════════════════════════════════"
  for i in 0..<簇表.len:
    echo "  簇#", 簇表[i].编号, " 成员: ", 簇表[i].成员

  echo ""
  echo "  八流统一 · 编码完成 · 固定点验证 done"
  echo "════════════════════════════════════════"
