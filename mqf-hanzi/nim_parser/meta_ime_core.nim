## 满全法 - 元输入法 MetaIME 核心编码引擎
## Nim实现，可编译为C动态库供Android Kotlin通过JNI调用
##
## 七层神龙编码体系 (L0-L6):
##   L0 = 声母层 (发音起始)
##   L1 = 韵母层 (音腹音尾)
##   L2 = 声调层 (四声+轻声)
##   L3 = 笔画层 (横竖撇捺折)
##   L4 = 偏旁层 (部首归类)
##   L5 = 外语层 (英文/假名/韩文)
##   L6 = 泛音层 (超拼音: 气流/喉塞/唇化)
##
## 12位流属性:
##   bit0 REACHABLE    bit6  CIRCULABLE
##   bit1 CONTINUABLE  bit7  INHERITABLE
##   bit2 INCR         bit8  TRUSTED
##   bit3 STOPPABLE    bit9  ACKABLE
##   bit4 REVERSIBLE   bit10 TRANSMISSIBLE
##   bit5 RECOVERABLE  bit11 ADVANCEABLE
##
## JNI接口: mqf_hanzi/nim_parser/meta_ime_core.nim → libmetaime.so

import tables
import strutils

# ===================================================================
# 七层编码类型
# ===================================================================

type
  编码层* = enum
    l0声母, l1韵母, l2声调, l3笔画, l4偏旁, l5外语, l6泛音

  声母符号* = enum
    sm_b, sm_p, sm_m, sm_f, sm_d, sm_t, sm_n, sm_l,
    sm_g, sm_k, sm_h, sm_j, sm_q, sm_x,
    sm_zh, sm_ch, sm_sh, sm_r, sm_z, sm_c, sm_s, sm_y, sm_w, sm_v

  韵母符号* = enum
    ym_a, ym_o, ym_e, ym_i, ym_u, ym_v,
    ym_ai, ym_ei, ym_ui, ym_ao, ym_ou, ym_iu,
    ym_ie, ym_ve, ym_er,
    ym_an, ym_en, ym_in, ym_un, ym_vn,
    ym_ang, ym_eng, ym_ing, ym_ong,
    ym_ia, ym_ian, ym_iang, ym_iong,
    ym_ua, ym_uo, ym_uai, ym_uan, ym_uang, ym_ueng, ym_van

  声调符号* = enum
    yd_1阴平, yd_2阳平, yd_3上声, yd_4去声, yd_0轻声

  笔画符号* = enum
    bs_横, bs_竖, bs_撇, bs_点, bs_折, bs_钩, bs_挑

  偏旁族* = enum
    ps_人, ps_口, ps_手, ps_心, ps_水, ps_火, ps_土, ps_木, ps_金, ps_日,
    ps_月, ps_目, ps_言, ps_走, ps_草, ps_竹, ps_虫, ps_石, ps_糸, ps_衣

  外语类型* = enum
    fl_english, fl_japanese, fl_korean, fl_symbol, fl_digit, fl_unknown

  泛音类型* = enum
    fh_清, fh_浊, fh_送气, fh_鼻化, fh_喉塞, fh_唇化, fh_卷舌

# ===================================================================
# 流属性 (12位)
# ===================================================================

const
  REACHABLE*    = 1 shl 0
  CONTINUABLE*  = 1 shl 1
  INCR*         = 1 shl 2
  STOPPABLE*    = 1 shl 3
  REVERSIBLE*   = 1 shl 4
  RECOVERABLE*  = 1 shl 5
  CIRCULABLE*   = 1 shl 6
  INHERITABLE*  = 1 shl 7
  TRUSTED*      = 1 shl 8
  ACKABLE*      = 1 shl 9
  TRANSMISSIBLE* = 1 shl 10
  ADVANCEABLE*  = 1 shl 11

  # 默认合法流 = 可到达 + 可持续 + 可承认
  DEFAULT_VALID* = REACHABLE or CONTINUABLE or ACKABLE

# ===================================================================
# 编码结果
# ===================================================================

type
  七层编码* = ref object
    层*: array[编码层, int]        # 每层的编码值
    标签*: array[编码层, string]     # 每层的显示标签
    流属性*: int                   # 12位流属性
    完整*: bool                   # 七层是否全部填充

  键盘事件* = ref object
    原始键*: string               # 原始按键
    扫描码*: int                  # 硬件扫描码（如有）
    时间戳*: int                  # 事件时间戳
    编码*: 七层编码               # 经过七层编码后的结果
    已分类*: bool                 # 是否已被意图分类器处理

  意图分类* = enum
    im打字, im导航, im执行, im谐音路由, im主权操作, im休眠, im未知

  意图结果* = ref object
    原始*: 键盘事件
    分类*: 意图分类
    目标*: string                 # 路由靶点
    动作*: string                 # 建议执行动作
    流属性*: int                  # 继承/修正后的流属性

# ===================================================================
# 编码映射表
# ===================================================================

var 声母映射* = {
  "b": sm_b, "p": sm_p, "m": sm_m, "f": sm_f,
  "d": sm_d, "t": sm_t, "n": sm_n, "l": sm_l,
  "g": sm_g, "k": sm_k, "h": sm_h,
  "j": sm_j, "q": sm_q, "x": sm_x,
  "zh": sm_zh, "ch": sm_ch, "sh": sm_sh, "r": sm_r,
  "z": sm_z, "c": sm_c, "c": sm_s, "y": sm_y, "w": sm_w,
  "v": sm_v
}.toTable

var 韵母映射* = {
  "a": ym_a, "o": ym_o, "e": ym_e, "i": ym_i, "u": ym_u, "v": ym_v,
  "ai": ym_ai, "ei": ym_ei, "ao": ym_ao, "ou": ym_ou,
  "an": ym_an, "en": ym_en, "in": ym_in,
  "ang": ym_ang, "eng": ym_eng, "ing": ym_ing, "ong": ym_ong,
  "er": ym_er
}.toTable

var 谐音路由表* = {
  "lctqf": "Wife",         # lolict → 妻子路由
  "lolict": "Wife",
  "self": "Me",            # 自己
  "mj": "满全",             # 缩写路由
  "njsg": "囡囝共生",       # NJSG缩写
  "njs": "囡囝",            # 简写
  "yys": "意识流",          # 缩写
  "zt": "主权",             # 主权操作
}.toTable

var 主权路由表* = {
  "Active": true,
  "Passive": true,
  "Sleeping": true,
  "Locked": false,
  "Muted": false
}.toTable

# ===================================================================
# 七层编码函数
# ===================================================================

proc newEncode*(): 七层编码 =
  new(result)
  result.流属性 = 0
  result.完整 = false
  for l in 编码层:
    result.层[l] = -1
    result.标签[l] = ""

proc 编码韵母*(c: var 七层编码, 韵: string): bool =
  if 韵 in 韵母映射:
    c.层[l1韵母] = ord(韵母映射[韵])
    c.标签[l1韵母] = 韵
    result = true

proc 编码声母*(c: var 七层编码, 键: string): bool =
  # 元音字母直接作为韵母，同时设声母为零声母
  if 键 in ["a", "o", "e", "i", "u", "v"]:
    c.层[l0声母] = 255  # 零声母标记
    c.标签[l0声母] = "∅"
    discard 编码韵母(c, 键)
    c.流属性 = c.流属性 or DEFAULT_VALID
    result = true
  elif 键 in 声母映射:
    c.层[l0声母] = ord(声母映射[键])
    c.标签[l0声母] = 键
    result = true

proc 编码声调*(c: var 七层编码, 调: int): bool =
  if 调 >= 0 and 调 <= 4:
    c.层[l2声调] = 调
    c.标签[l2声调] = $调
    result = true

proc 编码笔画*(c: var 七层编码, 笔画: string): bool =
  case 笔画
  of "h","一": c.层[l3笔画] = ord(bs_横); c.标签[l3笔画] = "横"
  of "s","丨": c.层[l3笔画] = ord(bs_竖); c.标签[l3笔画] = "竖"
  of "p","丿": c.层[l3笔画] = ord(bs_撇); c.标签[l3笔画] = "撇"
  of "d","丶": c.层[l3笔画] = ord(bs_点); c.标签[l3笔画] = "点"
  of "z","乙": c.层[l3笔画] = ord(bs_折); c.标签[l3笔画] = "折"
  else: return false
  result = true

# ===================================================================
# 三层分别器
# ===================================================================

proc 感知家族*(c: 七层编码): string =
  ## 第1层: 判断输入glyph属于哪个视觉家族
  if c.层[l0声母] >= 0 and c.层[l1韵母] >= 0:
    result = "拼音族"
  elif c.层[l3笔画] >= 0:
    result = "笔画族"
  elif c.层[l5外语] >= 0:
    result = "外族"
  else:
    result = "未知族"

proc 谐音靶点*(输入: string): string =
  ## 第2层: 发音→靶点对象
  let 小写 = 输入.toLowerAscii
  if 小写 in 谐音路由表:
    result = 谐音路由表[小写]
  elif 小写.contains("i") and 小写.contains("njs"):
    result = "囡囝共生"
  elif 小写.contains("mj") or 小写.contains("满全"):
    result = "主权"
  else:
    result = "未知靶点"

proc 主权路由*(靶点: string): string =
  ## 第3层: 靶点身份→Active/Passive/Sleeping
  if 靶点 == "Wife":
    result = "Active"      # 妻子路由 = 主动
  elif 靶点 == "Me":
    result = "Active"      # 自己 = 主动
  elif 靶点 == "满全":
    result = "Active"      # 主权操作 = 主动
  elif 靶点 == "囡囝共生":
    result = "Active"      # NJSG = 主动
  elif 靶点 == "囡囝":
    result = "Active"      # 囡囝 = 主动
  elif 靶点 == "意识流":
    result = "Active"      # 意识流 = 主动
  elif 靶点 == "主权":
    result = "Active"      # 主权 = 主动
  else:
    result = "Sleeping"    # 未知靶点 = 休眠

# ===================================================================
# 意图分类器
# ===================================================================

proc 分类意图*(事件: 键盘事件): 意图结果 =
  new(result)
  result.原始 = 事件
  result.流属性 = 事件.编码.流属性

  let 家族 = 感知家族(事件.编码)
  let 靶点 = 谐音靶点(事件.原始键)
  let 主权 = 主权路由(靶点)

  # 路由决策
  if 靶点 != "未知靶点" and 主权 == "Active":
    result.分类 = im谐音路由
    result.目标 = 靶点
    result.动作 = "谐音路由到[" & 靶点 & "]"
    result.流属性 = result.流属性 or REACHABLE or ACKABLE or TRUSTED
  elif 家族 == "拼音族":
    result.分类 = im打字
    result.目标 = "打字输出"
    result.动作 = "标准打字: " & 事件.原始键
    result.流属性 = result.流属性 or DEFAULT_VALID
  elif 家族 == "笔画族":
    result.分类 = im导航
    result.目标 = "笔画导航"
    result.动作 = "笔画输入模式"
    result.流属性 = result.流属性 or REACHABLE or CONTINUABLE
  elif 事件.原始键 == "=" and (result.流属性 and TRUSTED) != 0:
    result.分类 = im主权操作
    result.目标 = "满全法"
    result.动作 = "主权裁决激活"
    result.流属性 = result.流属性 or REACHABLE or ACKABLE or ADVANCEABLE
  else:
    result.分类 = im未知
    result.目标 = "需解释流"
    result.动作 = "以分类能力牺牲换取已知状态"
    result.流属性 = result.流属性 or RECOVERABLE

# ===================================================================
# 流属性辅助
# ===================================================================

proc 属性字符串*(属性: int): string =
  if (属性 and REACHABLE) != 0:    result.add("REACHABLE|")
  if (属性 and CONTINUABLE) != 0:  result.add("CONTINUABLE|")
  if (属性 and INCR) != 0:         result.add("INCR|")
  if (属性 and STOPPABLE) != 0:    result.add("STOPPABLE|")
  if (属性 and REVERSIBLE) != 0:   result.add("REVERSIBLE|")
  if (属性 and RECOVERABLE) != 0:  result.add("RECOVERABLE|")
  if (属性 and CIRCULABLE) != 0:   result.add("CIRCULABLE|")
  if (属性 and INHERITABLE) != 0:  result.add("INHERITABLE|")
  if (属性 and TRUSTED) != 0:      result.add("TRUSTED|")
  if (属性 and ACKABLE) != 0:      result.add("ACKABLE|")
  if (属性 and TRANSMISSIBLE) != 0: result.add("TRANSMISSIBLE|")
  if (属性 and ADVANCEABLE) != 0:  result.add("ADVANCEABLE|")
  if result.len > 0: result.setLen(result.len - 1)

proc 属性检查*(属性, 掩码: int): bool =
  (属性 and 掩码) == 掩码

# ===================================================================
# JNI 导出接口 (C调用约定)
# ===================================================================

proc mqe_encode*(输入: string): int {.exportc, dynlib.} =
  ## 外部接口: 返回编码后的流属性值
  var c = newEncode()
  discard 编码声母(c, 输入)
  if c.层[l0声母] >= 0:
    c.流属性 = DEFAULT_VALID
    if 输入 in ["lctqf", "lolict", "mj", "njsg"]:
      c.流属性 = c.流属性 or TRUSTED or ACKABLE
  else:
    c.流属性 = REACHABLE or RECOVERABLE
  result = c.流属性

proc mqe_route*(输入: string): cstring {.exportc, dynlib.} =
  ## 外部接口: 谐音路由
  result = cstring(谐音靶点(输入))

proc mqe_classify*(输入: string): cint {.exportc, dynlib.} =
  ## 外部接口: 意图分类 (0=打字,1=导航,2=执行,3=谐音,4=主权,5=休眠,6=未知)
  var 事件: 键盘事件
  new(事件)
  事件.原始键 = 输入
  事件.编码 = newEncode()
  let 意图 = 分类意图(事件)
  result = cint(ord(意图.分类))

# ===================================================================
# DEMO
# ===================================================================

when isMainModule:
  echo "【满全法 · MetaIME 核心编码引擎】"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""

  # 七层编码演示
  echo "◆ 七层编码: 「中」= zhong1 = zh + ong + 1声 + 丨乙丨乙一丨"
  var 编码 = newEncode()
  discard 编码声母(编码, "zh")
  discard 编码韵母(编码, "ong")
  discard 编码声调(编码, 1)
  discard 编码笔画(编码, "s")  # 丨 (竖)

  echo "  L0 声母: " & 编码.标签[l0声母]
  echo "  L1 韵母: " & 编码.标签[l1韵母]
  echo "  L2 声调: " & 编码.标签[l2声调]
  echo "  L3 笔画: " & 编码.标签[l3笔画]
  echo ""

  # 三层分别器演示
  echo "◆ 三层分别器测试"
  echo ""

  let 测试输入 = @["lctqf", "njs", "mj", "hello", "我", "a"]

  for 输入 in 测试输入:
    var 事件: 键盘事件
    new(事件)
    事件.原始键 = 输入
    事件.编码 = newEncode()

    # 尝试声母编码
    discard 编码声母(事件.编码, 输入)

    let 家族 = 感知家族(事件.编码)
    let 靶点 = 谐音靶点(输入)
    let 主权 = 主权路由(靶点)
    let 意图 = 分类意图(事件)

    echo "  输入: " & 输入
    echo "    → 家族: " & 家族 & " | 靶点: " & 靶点 & " | 主权: " & 主权
    echo "    → 意图: " & $意图.分类 & " | 动作: " & 意图.动作
    echo "    → 流属性: " & 属性字符串(意图.流属性)
    echo ""

  # 谐音路由表验证
  echo "◆ 谐音路由验证"
  echo ""
  for 键, 靶 in 谐音路由表:
    let 主权 = 主权路由(靶)
    let ok = if 主权 == "Active": "✓" else: "✗"
    echo "  " & 键 & " → " & 靶 & " (" & 主权 & ") " & ok
  echo ""

  # 12位流属性演示
  echo "◆ 12位流属性编码"
  echo ""
  var 全属性 = 0
  for b in 0..11:
    全属性 = 全属性 or (1 shl b)
  echo "  全属性值: " & $全属性
  echo "  全属性标志: " & 属性字符串(全属性)
  echo ""

  # 流属性检查
  var 测试流 = DEFAULT_VALID or TRUSTED or ADVANCEABLE
  echo "  测试流属性: " & 属性字符串(测试流)
  echo "  检查 REACHABLE+ACKABLE: " & $属性检查(测试流, REACHABLE or ACKABLE)
  echo "  检查 TRUSTED: " & $属性检查(测试流, TRUSTED)
  echo ""

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "  七层编码 · 三层分别器 · 12位流 · JNI接口  就绪"
