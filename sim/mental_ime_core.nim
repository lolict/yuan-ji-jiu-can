## ═══════════════════════════════════════════════════════════════
## mental_ime_core.nim — 心算输入法核心 · 内部索引 · 零依赖
## ═══════════════════════════════════════════════════════════════
##
## 核心: 不查表 · 不依赖外部 · 纯心算生成编码
##       笔画类型 = 容器 · 笔画数 = 位置 · 结构 = 层级
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables]

type
  StrokeType* = enum
    stHeng = 0,  # 横
    stShu  = 1,  # 竖
    stPie  = 2,  # 撇
    stNa   = 3,  # 捺
    stZhe  = 4,  # 折

  Structure* = enum
    stSingle   = 0,  # 独体
    stLeftRight = 1,  # 左右
    stTopBottom = 2,  # 上下
    stSurround  = 3,  # 包围

  CodeUnit* = object
    strokes*: seq[StrokeType]
    structure*: Structure
    stroke_count*: int
    code*: string

  MentalIME* = object
    units*: Table[string, CodeUnit]
    stroke_names*: array[5, string]
    struct_names*: array[4, string]

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newMentalIME*(): MentalIME =
  result.units = init_table[string, CodeUnit]()
  result.stroke_names = ["横", "竖", "撇", "捺", "折"]
  result.struct_names = ["独体", "左右", "上下", "包围"]

# ═══════════════════════════════════════════════════════════════
## 心算编码: 纯计算 · 不查表
# ═══════════════════════════════════════════════════════════════

proc compute_code*(ime: MentalIME, ch: string, strokes: seq[StrokeType], structure: Structure): string =
  ## 心算生成编码
  ## 编码 = 首笔画 + 笔画数 + 结构码
  
  if strokes.len == 0:
    return "ERR"
  
  # 首笔画码
  let first_stroke = ord(strokes[0])
  let stroke_code = chr(ord('a') + first_stroke)
  
  # 笔画数
  let count_code = $strokes.len
  
  # 结构码
  let struct_code = chr(ord('0') + ord(structure))
  
  # 组合
  result = stroke_code & count_code & struct_code

proc register*(ime: var MentalIME, ch: string, strokes: seq[StrokeType], structure: Structure) =
  var unit: CodeUnit
  unit.strokes = strokes
  unit.structure = structure
  unit.stroke_count = strokes.len
  unit.code = ime.compute_code(ch, strokes, structure)
  ime.units[ch] = unit

# ═══════════════════════════════════════════════════════════════
## 内置字库: 心算生成 · 不依赖外部
# ═══════════════════════════════════════════════════════════════

proc load_builtin*(ime: var MentalIME) =
  # 横起笔
  ime.register("一", @[stHeng], stSingle)
  ime.register("二", @[stHeng, stHeng], stSingle)
  ime.register("三", @[stHeng, stHeng, stHeng], stSingle)
  ime.register("王", @[stHeng, stHeng, stShu, stHeng], stSingle)
  ime.register("木", @[stHeng, stShu, stPie, stNa], stSingle)
  
  # 竖起笔
  ime.register("丨", @[stShu], stSingle)
  ime.register("上", @[stShu, stHeng, stHeng], stSingle)
  ime.register("口", @[stShu, stZhe, stHeng], stSingle)
  ime.register("日", @[stShu, stZhe, stHeng, stHeng], stSingle)
  
  # 撇起笔
  ime.register("丿", @[stPie], stSingle)
  ime.register("人", @[stPie, stNa], stSingle)
  ime.register("入", @[stPie, stNa], stSingle)
  ime.register("八", @[stPie, stNa], stSingle)
  
  # 捺起笔
  ime.register("丶", @[stNa], stSingle)
  ime.register("义", @[stNa, stPie, stNa], stSingle)
  ime.register("文", @[stNa, stHeng, stPie, stNa], stSingle)
  
  # 折起笔
  ime.register("乙", @[stZhe], stSingle)
  ime.register("刀", @[stZhe, stPie], stSingle)
  ime.register("力", @[stZhe, stPie], stSingle)
  ime.register("了", @[stZhe, stShu], stSingle)
  ime.register("子", @[stZhe, stZhe, stShu], stSingle)
  ime.register("也", @[stZhe, stShu, stZhe], stSingle)
  
  # 左右结构
  ime.register("江", @[stNa, stNa, stHeng, stShu, stHeng, stShu, stHeng], stLeftRight)
  ime.register("河", @[stNa, stNa, stHeng, stShu, stZhe, stHeng, stShu, stHeng], stLeftRight)
  ime.register("湖", @[stNa, stNa, stHeng, stShu, stZhe, stHeng, stShu, stPie, stHeng, stShu, stZhe, stHeng], stLeftRight)
  ime.register("海", @[stNa, stNa, stHeng, stPie, stHeng, stPie, stZhe, stNa, stHeng, stPie, stNa], stLeftRight)
  ime.register("池", @[stNa, stNa, stHeng, stShu, stZhe, stShu, stZhe], stLeftRight)
  ime.register("汗", @[stNa, stNa, stHeng, stShu, stHeng, stHeng, stShu], stLeftRight)
  ime.register("汤", @[stNa, stNa, stHeng, stZhe, stPie, stZhe, stPie], stLeftRight)
  
  # 上下结构
  ime.register("炎", @[stNa, stHeng, stPie, stNa, stNa, stHeng, stPie, stNa], stTopBottom)
  ime.register("字", @[stNa, stZhe, stZhe, stZhe, stShu, stZhe, stShu], stTopBottom)
  ime.register("宅", @[stNa, stZhe, stZhe, stHeng, stZhe, stPie, stZhe], stTopBottom)

# ═══════════════════════════════════════════════════════════════
## 查询
# ═══════════════════════════════════════════════════════════════

proc lookup*(ime: MentalIME, ch: string): string =
  if ime.units.has_key(ch):
    let unit = ime.units[ch]
    result = unit.code
  else:
    result = "UNKNOWN"

proc get_unit*(ime: MentalIME, ch: string): CodeUnit =
  if ime.units.has_key(ch):
    result = ime.units[ch]

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(ime: MentalIME): string =
  var lines: seq[string] = @[]
  lines.add("═══ 心算输入法核心 ═══")
  lines.add("  字数: " & $ime.units.len)
  lines.add("")
  lines.add("  字 │ 编码  │ 笔画 │ 结构")
  lines.add("  ───┼───────┼──────┼─────")
  
  for ch, unit in ime.units:
    lines.add("  " & ch & " │ " & unit.code & " │ " & $unit.stroke_count & " │ " & $ord(unit.structure))
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var ime = newMentalIME()
  ime.load_builtin()
  
  echo "═══ 心算输入法核心验证 ═══"
  echo ""
  echo ime.report()
  echo ""
  
  echo "── 心算查询 ──"
  echo "  江: " & ime.lookup("江")
  echo "  河: " & ime.lookup("河")
  echo "  湖: " & ime.lookup("湖")
  echo "  海: " & ime.lookup("海")
  echo "  炎: " & ime.lookup("炎")
  echo "  字: " & ime.lookup("字")
  echo ""
  
  echo "── 编码规则验证 ──"
  let jiang = ime.get_unit("江")
  echo "  江: 首笔画=捺(d) 笔画数=7 结构=左右(1)"
  echo "  编码应为: d71"
  echo "  实际编码: " & jiang.code
  echo "  验证: " & (if jiang.code == "d71": "✓ 通过" else: "✗ 失败")