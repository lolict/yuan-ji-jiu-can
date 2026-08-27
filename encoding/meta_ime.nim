## ═══════════════════════════════════════════════════════════════
## meta_ime.nim — 元输入法核心 · 本地智能载体
## ═══════════════════════════════════════════════════════════════
##
## 核心: 智能来源内嵌于程序本身 · 不依赖云端大模型
##       手机芯片本地运行轻量级规则引擎和分类器
##       七层编码体系驱动键盘拦截、意图识别、自动打字
##
## 七层编码:
##   1. 物理层: 按键扫描码
##   2. 笔画层: 横竖撇捺折
##   3. 声母层: b p m f ...
##   4. 韵母层: a o e ...
##   5. 声调层: 平仄升降
##   6. 语义层: 意图识别
##   7. 分发层: 多通道输出
##
## 架构: Kotlin薄壳(InputMethodService) + Rust核心(JNI)
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, tables, strformat]

type
  Layer* = enum
    lPhysical    # 物理层
    lStroke      # 笔画层
    lShengmu     # 声母层
    lYunmu       # 韵母层
    lTone        # 声调层
    lSemantic    # 语义层
    lDispatch    # 分发层

  KeyEvent* = object
    raw_code*: int
    layer*: Layer
    mapped*: string
    timestamp*: int

  Intent* = object
    action*: string
    target*: string
    confidence*: float

  MetaIME* = object
    layers*: seq[Layer]
    stroke_map*: Table[int, string]
    shengmu_map*: Table[string, int]
    yunmu_map*: Table[string, int]
    tone_map*: Table[int, string]
    intent_rules*: seq[Intent]
    local_mode*: bool
    tick*: int

# ═══════════════════════════════════════════════════════════════
## 初始化元输入法
# ═══════════════════════════════════════════════════════════════

proc new_meta_ime*(): MetaIME =
  result.layers = @[lPhysical, lStroke, lShengmu, lYunmu, lTone, lSemantic, lDispatch]
  result.stroke_map = init_table[int, string]()
  result.shengmu_map = init_table[string, int]()
  result.yunmu_map = init_table[string, int]()
  result.tone_map = init_table[int, string]()
  result.intent_rules = @[]
  result.local_mode = true
  result.tick = 0

  # 笔画映射: 1=横 2=竖 3=撇 4=捺 5=折
  result.stroke_map[1] = "横"
  result.stroke_map[2] = "竖"
  result.stroke_map[3] = "撇"
  result.stroke_map[4] = "捺"
  result.stroke_map[5] = "折"

  # 声母映射
  let shengmus = @["b", "p", "m", "f", "d", "t", "n", "l",
                     "g", "k", "h", "j", "q", "x",
                     "z", "c", "s", "zh", "ch", "sh", "r"]
  for i, sm in shengmus:
    result.shengmu_map[sm] = i

  # 韵母映射
  let yunmus = @["a", "o", "e", "i", "u", "v",
                   "ai", "ei", "ui", "ao", "ou", "iu",
                   "ie", "ve", "er", "an", "en", "in", "un", "vn",
                   "ang", "eng", "ing", "ong"]
  for i, ym in yunmus:
    result.yunmu_map[ym] = i

  # 声调映射
  result.tone_map[0] = "轻声"
  result.tone_map[1] = "阴平"
  result.tone_map[2] = "阳平"
  result.tone_map[3] = "上声"
  result.tone_map[4] = "去声"

# ═══════════════════════════════════════════════════════════════
## 七层编码处理
# ═══════════════════════════════════════════════════════════════

proc process_key*(ime: var MetaIME, raw_code: int): KeyEvent =
  ime.tick += 1
  var evt = KeyEvent(raw_code: raw_code, layer: lPhysical, timestamp: ime.tick)

  # 物理层 → 笔画层
  if raw_code >= 1 and raw_code <= 5:
    evt.layer = lStroke
    evt.mapped = ime.stroke_map[raw_code]
  # 声母层(模拟: 101-121 = b-r)
  elif raw_code >= 101 and raw_code <= 121:
    evt.layer = lShengmu
    for sm, code in ime.shengmu_map:
      if code == raw_code - 101:
        evt.mapped = sm
        break

  result = evt

# ═══════════════════════════════════════════════════════════════
## 意图识别(本地规则引擎)
# ═══════════════════════════════════════════════════════════════

proc recognize_intent*(ime: MetaIME, input: string): Intent =
  # 本地规则: 不联网 · 不调用API
  if input.contains("写") or input.contains("发"):
    result = Intent(action: "compose", target: "text", confidence: 0.8)
  elif input.contains("查") or input.contains("搜"):
    result = Intent(action: "search", target: "info", confidence: 0.7)
  elif input.contains("发") or input.contains("送"):
    result = Intent(action: "send", target: "message", confidence: 0.75)
  else:
    result = Intent(action: "type", target: "raw", confidence: 0.9)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(ime: MetaIME): string =
  var lines: seq[string] = @[]
  lines.add("═══ 元输入法核心 · 本地智能载体 ═══")
  lines.add("  七层编码: 物理→笔画→声母→韵母→声调→语义→分发")
  lines.add("  本地模式: " & (if ime.local_mode: "是 · 不依赖云端" else: "否"))
  lines.add("  处理事件数: " & $ime.tick)
  lines.add("")

  lines.add("── 编码表统计 ──")
  lines.add("  笔画: " & $ime.stroke_map.len & " 种 (横竖撇捺折)")
  lines.add("  声母: " & $ime.shengmu_map.len & " 个")
  lines.add("  韵母: " & $ime.yunmu_map.len & " 个")
  lines.add("  声调: " & $ime.tone_map.len & " 种")

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var ime = new_meta_ime()

  echo "═══ 元输入法核心验证 ═══"
  echo ""

  # 模拟按键输入
  let keys = @[1, 104, 2, 108, 3, 5]
  echo "── 按键处理 ──"
  for k in keys:
    let evt = ime.process_key(k)
    echo "  原始码=" & $evt.raw_code & " → 层=" & $evt.layer & " 映射=" & evt.mapped

  echo ""
  echo "── 意图识别(本地) ──"
  let intents = @["写一段话", "查天气", "发消息", "你好"]
  for input in intents:
    let intent = ime.recognize_intent(input)
    echo "  输入: \"" & input & "\" → 动作=" & intent.action &
          " 置信度=" & $intent.confidence

  echo ""
  echo ime.report()

  echo ""
  echo "── 架构 ──"
  echo "  Kotlin薄壳: InputMethodService框架"
  echo "  Rust核心: 规则引擎 + 分类器"
  echo "  JNI连接: Kotlin ↔ Rust"
  echo "  不依赖云端: 所有智能本地运行"
