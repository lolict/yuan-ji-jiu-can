# ============================================================
# MetaIME · 七层编码驱动的键盘拦截 + 意图识别 演示
#
# 注: 假设在真实 Android 中由 InputMethodService 拦截按键事件，
# 此处用 stdin 模拟事件流，逻辑层完全可移植到 IME。
#
# 七层编码:
#   L1 声母    b p m f d t n l ...
#   L2 韵母    a o e i u v ai ei ...
#   L3 声调    1 2 3 4
#   L4 结构部首  氵木亻口...
#   L5 语义映射  FQM 本体路径
#   L6 意图识别  INPUT | COMMAND | TRANSFORM
#   L7 输出动作  上屏 / 触发命令 / 转换
#
# 设计原则:
#   全部查表驱动，无云端，栈分配（编译时常量）
#   七层是过滤漏斗：每一层缩小候选集，L7 命中唯一下游
# ============================================================

import strutils, os

const
  # L1-L2-L3: 拼音→汉字 最小演示集 (真实系统扩展至全字库)
  # 格式: "声母|韵母|声调" => "汉字"
  PINYIN_TABLE*: array[12, tuple[key: string, val: string]] = [
    ("b|a|1", "巴"), ("b|a|4", "爸"), ("p|o|2", "婆"),
    ("m|a|1",  "妈"), ("m|i|3", "米"), ("f|a|1", "发"),
    ("d|a|3",  "打"), ("d|e|2", "得"), ("t|a|1", "他"),
    ("n|i|3",  "你"), ("l|i|3", "李"), ("g|e|2", "格")
  ]

  # L4 结构部首映射: 汉字→[部首, 结构码(mlil表面名/mlnnn仅加减码)]
  RADIX_TABLE*: array[8, tuple[key: string, radical: string, struct_code: string]] = [
    ("妈", "女", "mlil:Xing"),   ("咪", "口", "mlil:Kou"),
    ("米", "米", "mlnnn:only-add"), ("李", "木", "mlnnn:sub-only"),
    ("他", "亻", "mlil:Ren"),    ("巴", "巳", "mlnnn:only-add"),
    ("发", "又", "mlil:You"),    ("格", "木", "mlnnn:sub-only")
  ]

  # L5 语义: FQM 本体路径查询关键字（同 FOREIGN_MAP_KEYS 的子集）
  SEMANTIC_TABLE*: array[6, tuple[key: string, path: string]] = [
    ("int",    "袋.类型.整数"),
    ("char",   "袋.类型.字符"),
    ("let",    "袋.定义.绑定"),
    ("match",  "袋.判断.匹配"),
    ("for",    "袋.循环.遍历"),
    ("return", "袋.输出.结果")
  ]

  # L6 意图: 关键字 → 类别
  INTENT_TABLE*: array[5, tuple[key: string, intent: string]] = [
    ("mlll",  "COMMAND:根基"),
    ("njsg",  "COMMAND:共生"),
    ("mltt",  "TRANSFORM:命名"),
    ("ROOT",  "COMMAND:根基根"),
    ("exit",  "COMMAND:退出")
  ]

# ═══════════════════════════════════════════
# L1-L2-L3 拼音→汉字
# ═══════════════════════════════════════════
proc pinyin_to_hz*(sheng: string; yun: string; tone: string): string =
  let key = sheng & "|" & yun & "|" & tone
  for entry in PINYIN_TABLE:
    if entry.key == key:
      return entry.val
  return ""

proc match_pinyin_prefix*(sheng: string; yun: string): seq[string] =
  let prefix = sheng & "|" & yun & "|"
  for entry in PINYIN_TABLE:
    if entry.key.startsWith(prefix):
      result.add(entry.val)

# ═══════════════════════════════════════════
# L4 查部首/结构码
# ═══════════════════════════════════════════
proc hz_to_radix*(hz: string): tuple[radical: string; struct_code: string] =
  for entry in RADIX_TABLE:
    if entry.key == hz:
      return (entry.radical, entry.struct_code)
  return ("?", "mlnnn:default")

# ═══════════════════════════════════════════
# L5 语义 FQM 本体路径
# ═══════════════════════════════════════════
proc semantic_path*(word: string): string =
  for entry in SEMANTIC_TABLE:
    if entry.key == word:
      return entry.path
  return "袋.未知"

# ═══════════════════════════════════════════
# L6 意图识别
# ═══════════════════════════════════════════
proc detect_intent*(input: string): string =
  for entry in INTENT_TABLE:
    if entry.key == input:
      return entry.intent
  return "INPUT:普通输入"

# ═══════════════════════════════════════════
# L7 输出动作
# ═══════════════════════════════════════════
proc execute_output*(intent: string; payload: string): string =
  case intent
  of "INPUT:普通输入":
    result = "上屏: " & payload
  of "COMMAND:根基":
    result = "激活: 根基模式 (mlll-ROOT)"
  of "COMMAND:共生":
    result = "激活: 囡囝共生模式 (njsg)"
  of "COMMAND:根基根":
    result = "锚定: 根基根"
  of "COMMAND:退出":
    result = "关闭输入法"
  of "TRANSFORM:命名":
    result = "转换命名模式 (mltt-NAME)"
  else:
    if intent.startsWith("COMMAND:"):
      result = "命令: " & intent[8..^1]
    else:
      result = "动作: " & intent

# ═══════════════════════════════════════════
# 演示: 模拟按键序列
# ═══════════════════════════════════════════
proc run_interactive*() =
  echo ""
  echo "============================================================"
  echo "MetaIME · 七层编码键盘驱动演示 (stdin 模拟 Android IME 事件)"
  echo "============================================================"
  echo ""
  echo "测试 1: 拼音输入 b a 1"
  echo "  L1 声母=b"
  echo "  L2 韵母=a"
  echo "  L3 声调=1"
  let hz1 = pinyin_to_hz("b", "a", "1")
  if hz1.len > 0:
    echo "  → 汉字: " & hz1
    let (rad, code) = hz_to_radix(hz1)
    echo "  L4 部首=" & rad & " 结构码=" & code
    echo "  L5 语义=" & semantic_path(hz1)
    echo "  L6 意图=" & detect_intent(hz1)
    echo "  L7 输出=" & execute_output(detect_intent(hz1), hz1)
  echo ""
  echo "测试 2: 拼音输入 n i 3"
  let hz2 = pinyin_to_hz("n", "i", "3")
  if hz2.len > 0:
    echo "  L1-L3: " & hz2
    let (rad2, code2) = hz_to_radix(hz2)
    echo "  L4: 部首=" & rad2 & " 结构码=" & code2
    echo "  L5: 语义=" & semantic_path(hz2)
    echo "  L6: 意图=" & detect_intent(hz2)
    echo "  L7: 输出=" & execute_output(detect_intent(hz2), hz2)
  echo ""
  echo "测试 3: 意图识别 mlll (根基命令)"
  echo "  L6: " & detect_intent("mlll")
  echo "  L7: " & execute_output(detect_intent("mlll"), "mlll")
  echo ""
  echo "测试 4: 意图识别 for"
  echo "  L5: " & semantic_path("for")
  echo "  L6: " & detect_intent("for")
  echo "  L7: " & execute_output(detect_intent("for"), "for")
  echo ""
  echo "测试 5: 拼音 b a 缺省声调 → 候选列表"
  for hz in match_pinyin_prefix("b", "a"):
    echo "  候选: " & hz
  echo ""
  echo "[MetaIME] 七层编码驱动闭环完成"
  echo "  Android 部署: 套 InputMethodService 拦截 onKeyDown"
  echo "  本层逻辑: 可直接移植至 IME 事件循环"

# 允许被 main 模块调用
when isMainModule:
  run_interactive()
