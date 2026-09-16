## 语音路由: 汉字拼音 → ISA 语义符号
##
## 路由表:
##   汉字(关键词) → 声母+韵母分解 → 语义 ID → 字体符号(U+F100+) → 机器码
##
## 路由规则按声母自然分组:
##   b/p/m/f      → 算术类 (加减)
##   d/t/n/l      → 数据类 (加载/存储)
##   g/k/h        → 分支类 (比较/跳转)
##   j/q/x        → 逻辑类 (与或异或)
##   zh/ch/sh/z/c/s/r/y/w → 特殊 (高位/位移)

import std/[strutils, tables, sequtils]

import cross_isa_map

# ── 常用汇编关键词路由表 ─────────────────────────────────
type
  RouteEntry* = object
    hanzi*: string        # 汉字/关键词
    pinyin*: string       # 拼音
    initial*: string      # 声母
    final*: string        # 韵母
    sem*: SemKind         # 目标语义
    desc*: string         # 说明

const ROUTE_TABLE: seq[RouteEntry] = @[
  # 算术
  RouteEntry(hanzi: "加", pinyin: "jia",   initial: "j",  final: "ia",  sem: semAdd,      desc: "加法 (add)"),
  RouteEntry(hanzi: "减", pinyin: "jian",  initial: "j",  final: "ian", sem: semSub,      desc: "减法 (sub)"),
  RouteEntry(hanzi: "和", pinyin: "he",   initial: "h",  final: "e",   sem: semAdd,      desc: "求和 (add)"),

  # 逻辑
  RouteEntry(hanzi: "与", pinyin: "yu",    initial: "y",  final: "u",   sem: semAnd,      desc: "按位与 (and)"),
  RouteEntry(hanzi: "或", pinyin: "huo",   initial: "h",  final: "uo",  sem: semOr,       desc: "按位或 (or)"),
  RouteEntry(hanzi: "异", pinyin: "yi",    initial: "y",  final: "i",   sem: semXor,      desc: "异或 (xor)"),
  RouteEntry(hanzi: "移", pinyin: "yi",    initial: "y",  final: "i",   sem: semSll,      desc: "移位 (sll/srl)"),

  # 数据
  RouteEntry(hanzi: "读", pinyin: "du",    initial: "d",  final: "u",   sem: semLoad,     desc: "加载 (load)"),
  RouteEntry(hanzi: "写", pinyin: "xie",   initial: "x",  final: "ie",  sem: semStore,    desc: "存储 (store)"),
  RouteEntry(hanzi: "取", pinyin: "qu",    initial: "q",  final: "u",   sem: semLoad,     desc: "取数 (load)"),
  RouteEntry(hanzi: "存", pinyin: "cun",   initial: "c",  final: "un",  sem: semStore,    desc: "存数 (store)"),

  # 控制流
  RouteEntry(hanzi: "跳", pinyin: "tiao",  initial: "t",  final: "iao", sem: semJump,     desc: "跳转 (jump)"),
  RouteEntry(hanzi: "等", pinyin: "deng",  initial: "d",  final: "eng", sem: semBranchEq, desc: "相等跳转 (beq)"),
  RouteEntry(hanzi: "比", pinyin: "bi",    initial: "b",  final: "i",   sem: semSlt,      desc: "比较 (slt)"),
  RouteEntry(hanzi: "返", pinyin: "fan",   initial: "f",  final: "an",  sem: semJumpReg,  desc: "返回 (jalr)"),

  # 立即数/高位
  RouteEntry(hanzi: "数", pinyin: "shu",   initial: "sh", final: "u",   sem: semLui,      desc: "立即数 (lui)"),
]

# ── 声母 → 语义类别 映射 ─────────────────────────────────
const INITIAL_TO_CATEGORY: Table[string, string] = {
  # 唇音: 算术
  "b": "arith", "p": "arith", "m": "arith", "f": "arith",
  # 舌尖: 数据
  "d": "data",  "t": "data",  "n": "data",  "l": "data",
  # 舌根: 控制
  "g": "ctrl",  "k": "ctrl",  "h": "ctrl",
  # 舌面: 逻辑
  "j": "logic", "q": "logic", "x": "logic",
  # 翘舌: 特殊
  "zh": "spec", "ch": "spec", "sh": "spec", "z": "spec", "c": "spec",
  # 零声母/半元音
  "r": "spec",  "y": "logic", "w": "data",
}.toTable

# ── 路由查找 ─────────────────────────────────────────────
proc find_route*(hanzi: string): int =
  for i in 0 ..< ROUTE_TABLE.len:
    if ROUTE_TABLE[i].hanzi == hanzi:
      return i
  return -1

proc find_route_by_pinyin*(py: string): int =
  for i in 0 ..< ROUTE_TABLE.len:
    if ROUTE_TABLE[i].pinyin == py:
      return i
  return -1

# ── 端到端路由: 汉字 → 机器码 ────────────────────────────
proc hanzi_to_machine*(hanzi: string; encs: seq[MachineEncoding];
                       isa: IsaKind): string =
  let idx = find_route(hanzi)
  if idx < 0: return "[未路由: " & hanzi & "]"
  let rt = ROUTE_TABLE[idx]
  for e in encs:
    if e.sem == rt.sem and e.isa == isa:
      return e.machine_hex
  return "[ISA " & $isa & " 不支援 " & $rt.sem & "]"

# ════════════════════════════════════════════════════════════
# 演示: 汉字拼音完整路由可视化
# ════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ 语音路由: 汉字 → 拼音 → 语义 → 字体符号 → 机器码 ═══"
  echo ""

  let encs = build_cross_isa_map()

  # 路由表示例
  echo "【路由表】(汉字 → 拼音 → 声母韵母 → 语义 → 字体符号)"
  echo "  ─────┬──────┬────────┬───────┬────────────┬────────"
  echo "  汉字 │ 拼音 │ 声母/韵 │ 语义  │ 字体符号   │ 描述"
  echo "  ─────┼──────┼────────┼───────┼────────────┼────────"
  for rt in ROUTE_TABLE:
    let cp = sem_to_sym(rt.sem)
    echo "  ", rt.hanzi.alignLeft(4), " │ ",
        rt.pinyin.alignLeft(5), " │ ",
        (rt.initial & "/" & rt.final).alignLeft(6), " │ ",
        ($rt.sem)[3..^1].alignLeft(5), " │ U+",
        toHex(cp).toUpperAscii.alignLeft(9), " │ ", rt.desc
  echo ""

  # 端到端例: 加 → 加法
  echo "【端到端】'加' (加法) → 4 个 ISA 机器码:"
  echo "  ┌──────────┬──────────────┬─────────────────────────────┐"
  echo "  │   ISA    │   机器码     │ 字形来源                     │"
  echo "  ├──────────┼──────────────┼─────────────────────────────┤"
  for isa in IsaKind:
    let mc = hanzi_to_machine("加", encs, isa)
    echo "  │ ", ($isa).alignLeft(8), " │ ",
        mc.alignLeft(12), " │ U+F100 (＋ 字形跨平台统一)  │"
  echo "  └──────────┴──────────────┴─────────────────────────────┘"
  echo ""

  # 路由器分类展示
  echo "【声母类别】"
  for init_name, cat in INITIAL_TO_CATEGORY:
    # 找出该声母下的所有汉字
    var chars: seq[string] = @[]
    for rt in ROUTE_TABLE:
      if rt.initial == init_name or rt.initial.startsWith(init_name):
        chars.add(rt.hanzi)
    if chars.len > 0:
      echo "  ", init_name.alignLeft(4), " [", cat.alignLeft(6), "] → ", chars.join("、")
  echo ""

  # 完整"加法程序"的多 ISA 表示
  echo "【加法程序】'加写返' (a=b+c, store, return)"
  echo "  加(读) → semLoad   → 从内存取操作数"
  echo "  加(加) → semAdd    ← a = b + c"
  echo "  写(存) → semStore  → 结果写回"
  echo "  返(回) → semJumpReg → jalr x0, 0(x1) 返回"
  echo ""
  echo "  RISC-V:  加→0080A003(lw) 加→00208033(add) 写→0020A423(sw) 返→00008067(jalr)"
  echo "  ARM64:   加→F9400420(ldr) 加→8B020020(add) 写→F9000420(str) 返→--"
  echo ""
  echo "═══ 完成: 汉字拼音 ISA 语义 路由验证 ═══"
