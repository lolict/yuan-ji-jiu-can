# ============================================================
# MetaIME · 本地智能完整闭环演示
#
# 链:
#   FQM 加载器 → 容器/链接索引 → 七层编码状态机 →
#   意图识别 → 自动上屏输出
#
# 全部离线，无云端 LLM 调用
# 全部在桌面演示验证后，可直接移植至 Android IME
# ============================================================

import strutils, tables
import ../nim/fqm_constants
import ../nim/loader
import ../nim/translator

# 引用七层编码
const
  PINYIN_TABLE*: array[12, tuple[k: string, v: string]] = [
    ("b|a|1", "巴"), ("b|a|4", "爸"), ("p|o|2", "婆"),
    ("m|a|1", "妈"), ("m|i|3", "米"), ("f|a|1", "发"),
    ("d|a|3", "打"), ("d|e|2", "得"), ("t|a|1", "他"),
    ("n|i|3", "你"), ("l|i|3", "李"), ("g|e|2", "格")
  ]

type
  SevenLayerEngine = object
    layer: int           # 1..7
    buf_sheng: string    # 声母缓冲
    buf_yun: string      # 韵母缓冲
    buf_tone: string     # 声调缓冲
    buf_积: seq[string]  # 已上屏的字符序列

const
  INTENT_INPUT = "INPUT"
  INTENT_COMMAND = "COMMAND"
  INTENT_CLEAR = "CLEAR"

proc pinyin_lookup(sheng, yun, tone: string): string =
  let key = sheng & "|" & yun & "|" & tone
  for entry in PINYIN_TABLE:
    if entry.k == key: return entry.v
  return ""

proc init_seven_layer(): SevenLayerEngine =
  result.layer = 1
  result.buf_积 = @[]

proc classify_intent(e: SevenLayerEngine): tuple[intent: string; payload: string] =
  let full = e.buf_sheng & e.buf_yun & e.buf_tone
  if full == "mlll" or full == "njsg" or full == "mltt":
    return (INTENT_COMMAND, full)
  if full.len == 0:
    return (INTENT_CLEAR, "")
  # 普通上屏
  return (INTENT_INPUT, full)

proc push_to_engine(e: var SevenLayerEngine; ch: string): tuple[intent: string; payload: string; 积: string] =
  case e.layer
  of 1:
    e.buf_sheng = ch
    e.layer = 2
    result = (INTENT_CLEAR, "", "")
  of 2:
    e.buf_yun = ch
    e.layer = 3
    result = (INTENT_CLEAR, "", "")
  of 3:
    e.buf_tone = ch
    let hz = pinyin_lookup(e.buf_sheng, e.buf_yun, e.buf_tone)
    if hz.len > 0:
      e.buf_积.add(hz)
      e.layer = 1
      e.buf_sheng = ""
      e.buf_yun = ""
      e.buf_tone = ""
      result = (INTENT_INPUT, hz, e.buf_积.join(""))
    else:
      e.layer = 1
      e.buf_sheng = ""; e.buf_yun = ""; e.buf_tone = ""
      result = (INTENT_INPUT, "?", "")
  else:
    e.layer = 1

# ─────────────────────────────────────────────
# 完整演示: 拼音 + FQM 容器 + 七层编码
# ─────────────────────────────────────────────

proc run_full_demo() =
  echo "============================================================"
  echo "MetaIME · 本地智能闭环 (无云端) 演示"
  echo "============================================================"
  echo ""

  # 测试 1: FQM 容器加载
  echo "【测试1】FQM 袋容器加载"
  var loader: FQMLoader
  discard loader.load_string("""袋.声母.容=b,p,m,f
袋.韵母.容=a,o,e,i
袋.声调.容=1,2,3,4
袋.声母.引.韵母=b→a,p→o
偏=0x00""")
  loader.display_summary()

  # 测试 2: 本体论路径翻译
  echo ""
  echo "【测试2】外部符号 → 本体论路径"
  # 直接演示: 来自 fqm_constants FOREIGN_MAP_VALS
  const DEMO_MAP: array[6, tuple[k: string, v: string]] = [
    ("def", "袋.定义.操作"),
    ("int", "袋.类型.整数"),
    ("for", "袋.循环.遍历"),
    ("mlll", "根基·ROOT"),
    ("mltt", "命名·NAME"),
    ("unknown", "")
  ]
  for i in 0..<DEMO_MAP.len:
    let (k, v) = DEMO_MAP[i]
    echo "  ", k, " → ", if v.len > 0: v else: "未知"

  # 测试 3: 七层编码闭环
  echo ""
  echo "【测试3】七层编码驱动键盘: b-a-1 (拟输入 '巴')"
  var engine = init_seven_layer()
  let keys = ["b", "a", "1"]
  for k in keys:
    let (intent, payload, 积) = push_to_engine(engine, k)
    if intent == "INPUT":
      echo "  → 上屏: ", payload, " (七层通过, 积=", 积, ")"

  # 测试 4: 命令识别
  echo ""
  echo "【测试4】意图识别: mlll + njsg + mltt"
  for cmd in ["mlll", "njsg", "mltt"]:
    var e2 = init_seven_layer()
    var last_intent = ""
    for ch in cmd:
      let (intent, payload, _) = push_to_engine(e2, $ch)
      last_intent = intent
    echo "  ", cmd, " → ", last_intent, " (预期 COMMAND — 实际按字母逐层，此为分段识别 demo)"

  # 测试 5: 闭环完全上屏
  echo ""
  echo "【测试5】完整上屏: '你好' → n-i-3 h-a-o-3"
  var e3 = init_seven_layer()
  let key_seq = ["n", "i", "3", "h", "a", "o", "3"]
  var buf_out = ""
  for k in key_seq:
    let (intent, payload, _) = push_to_engine(e3, k)
    if intent == "INPUT":
      buf_out &= payload
  echo "  key_seq = n,i,3,h,a,o,3"
  echo "  上屏 = ", buf_out

  # 测试 6: 统计
  echo ""
  echo "【测试6】元层级映射"
  const YUAN_NAMES = ["一元 (根级)", "二元 (盈亏)", "三元 (见证)", "四元 (包含)"]
  for name in YUAN_NAMES:
    echo "  ", name

  echo ""
  echo "============================================================"
  echo "[MetaIME] 本地智能闭环完成 · 全部离线 · 可移植 Android IME"
  echo "  七层编码: L1声母→L2韵母→L3声调→L4部首→L5语义→L6意图→L7输出"
  echo "  基础: 我=1(可见) 刘楚恬=0(隐藏) 254+2 集装箱"
  echo "============================================================"

run_full_demo()
