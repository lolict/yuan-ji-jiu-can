## ═══════════════════════════════════════════════════════════════
## stroke_radix_proper.nim — 笔画进制 · 统计 · 分类 · 聚类 · 索引
## ═══════════════════════════════════════════════════════════════
##
## 根: 统计每个容器里有多少 · 分类 · 聚类 · 索引
##       没有统计就没有进制
##       索引 = 一看就知道怎么找

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
    stSingle    = 0,  # 独体
    stLeftRight = 1,  # 左右
    stTopBottom = 2,  # 上下
    stSurround  = 3,  # 包围

  Character* = object
    ch*: string
    strokes*: seq[StrokeType]
    stroke_count*: int
    primary_stroke*: StrokeType
    structure*: Structure
    radical*: string

  # 统计容器
  Stats* = object
    total*: int
    by_stroke_type*: array[5, int]      # 各笔画类型数量
    by_stroke_count*: Table[int, int]   # 各笔画数数量
    by_structure*: array[4, int]        # 各结构数量
    by_radical*: Table[string, int]     # 各偏旁数量

  # 索引路径
  IndexPath* = object
    stroke_type*: string
    stroke_count*: int
    structure*: string
    position*: int
    radical*: string

  StrokeRadix* = object
    characters*: seq[Character]
    stats*: Stats
    index*: Table[string, IndexPath]   # 字 → 索引路径
    name*: string

# ═══════════════════════════════════════════════════════════════
## 数据
# ═══════════════════════════════════════════════════════════════

proc get_characters*(): seq[Character] =
  result = @[
    Character(ch: "一", strokes: @[stHeng], stroke_count: 1, primary_stroke: stHeng, structure: stSingle, radical: "一"),
    Character(ch: "二", strokes: @[stHeng, stHeng], stroke_count: 2, primary_stroke: stHeng, structure: stSingle, radical: "二"),
    Character(ch: "三", strokes: @[stHeng, stHeng, stHeng], stroke_count: 3, primary_stroke: stHeng, structure: stSingle, radical: "三"),
    Character(ch: "王", strokes: @[stHeng, stHeng, stShu, stHeng], stroke_count: 4, primary_stroke: stHeng, structure: stSingle, radical: "王"),
    Character(ch: "木", strokes: @[stHeng, stShu, stPie, stNa], stroke_count: 4, primary_stroke: stHeng, structure: stSingle, radical: "木"),
    Character(ch: "本", strokes: @[stHeng, stShu, stPie, stNa, stHeng], stroke_count: 5, primary_stroke: stHeng, structure: stSingle, radical: "木"),
    Character(ch: "朱", strokes: @[stPie, stHeng, stShu, stPie, stNa], stroke_count: 5, primary_stroke: stPie, structure: stSingle, radical: "木"),
    Character(ch: "江", strokes: @[stNa, stNa, stHeng, stShu, stHeng, stShu, stHeng], stroke_count: 7, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "河", strokes: @[stNa, stNa, stHeng, stShu, stZhe, stHeng, stShu, stHeng], stroke_count: 8, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "湖", strokes: @[stNa, stNa, stHeng, stShu, stZhe, stHeng, stShu, stPie, stHeng, stShu, stZhe, stHeng], stroke_count: 12, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "海", strokes: @[stNa, stNa, stHeng, stPie, stHeng, stPie, stZhe, stNa, stHeng, stPie, stNa], stroke_count: 11, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "池", strokes: @[stNa, stNa, stHeng, stShu, stZhe, stShu, stZhe], stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "汗", strokes: @[stNa, stNa, stHeng, stShu, stHeng, stHeng, stShu], stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "汤", strokes: @[stNa, stNa, stHeng, stZhe, stPie, stZhe, stPie], stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵"),
    Character(ch: "丨", strokes: @[stShu], stroke_count: 1, primary_stroke: stShu, structure: stSingle, radical: "丨"),
    Character(ch: "上", strokes: @[stShu, stHeng, stHeng], stroke_count: 3, primary_stroke: stShu, structure: stSingle, radical: "上"),
    Character(ch: "口", strokes: @[stShu, stZhe, stHeng], stroke_count: 3, primary_stroke: stShu, structure: stSingle, radical: "口"),
    Character(ch: "日", strokes: @[stShu, stZhe, stHeng, stHeng], stroke_count: 4, primary_stroke: stShu, structure: stSingle, radical: "日"),
    Character(ch: "目", strokes: @[stShu, stZhe, stHeng, stHeng, stHeng], stroke_count: 5, primary_stroke: stShu, structure: stSingle, radical: "目"),
    Character(ch: "田", strokes: @[stShu, stZhe, stHeng, stShu, stHeng], stroke_count: 5, primary_stroke: stShu, structure: stSingle, radical: "田"),
    Character(ch: "丿", strokes: @[stPie], stroke_count: 1, primary_stroke: stPie, structure: stSingle, radical: "丿"),
    Character(ch: "人", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "人"),
    Character(ch: "入", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "入"),
    Character(ch: "八", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "八"),
    Character(ch: "禾", strokes: @[stPie, stHeng, stShu, stPie, stNa], stroke_count: 5, primary_stroke: stPie, structure: stSingle, radical: "禾"),
    Character(ch: "千", strokes: @[stPie, stHeng, stShu], stroke_count: 3, primary_stroke: stPie, structure: stSingle, radical: "千"),
    Character(ch: "丶", strokes: @[stNa], stroke_count: 1, primary_stroke: stNa, structure: stSingle, radical: "丶"),
    Character(ch: "义", strokes: @[stNa, stPie, stNa], stroke_count: 3, primary_stroke: stNa, structure: stSingle, radical: "义"),
    Character(ch: "文", strokes: @[stNa, stHeng, stPie, stNa], stroke_count: 4, primary_stroke: stNa, structure: stSingle, radical: "文"),
    Character(ch: "方", strokes: @[stNa, stHeng, stZhe, stPie], stroke_count: 4, primary_stroke: stNa, structure: stSingle, radical: "方"),
    Character(ch: "六", strokes: @[stNa, stHeng, stPie, stNa], stroke_count: 4, primary_stroke: stNa, structure: stSingle, radical: "六"),
    Character(ch: "乙", strokes: @[stZhe], stroke_count: 1, primary_stroke: stZhe, structure: stSingle, radical: "乙"),
    Character(ch: "刀", strokes: @[stZhe, stPie], stroke_count: 2, primary_stroke: stZhe, structure: stSingle, radical: "刀"),
    Character(ch: "力", strokes: @[stZhe, stPie], stroke_count: 2, primary_stroke: stZhe, structure: stSingle, radical: "刀"),
    Character(ch: "了", strokes: @[stZhe, stShu], stroke_count: 2, primary_stroke: stZhe, structure: stSingle, radical: "了"),
    Character(ch: "子", strokes: @[stZhe, stZhe, stShu], stroke_count: 3, primary_stroke: stZhe, structure: stSingle, radical: "子"),
    Character(ch: "也", strokes: @[stZhe, stShu, stZhe], stroke_count: 3, primary_stroke: stZhe, structure: stSingle, radical: "也"),
    Character(ch: "炎", strokes: @[stNa, stHeng, stPie, stNa, stNa, stHeng, stPie, stNa], stroke_count: 8, primary_stroke: stNa, structure: stTopBottom, radical: "火"),
    Character(ch: "字", strokes: @[stNa, stZhe, stZhe, stZhe, stShu, stZhe, stShu], stroke_count: 7, primary_stroke: stNa, structure: stTopBottom, radical: "宀"),
  ]

# ═══════════════════════════════════════════════════════════════
## 统计: 每个容器里有多少
# ═══════════════════════════════════════════════════════════════

proc compute_stats*(chars: seq[Character]): Stats =
  result.total = chars.len
  
  for c in chars:
    # 按笔画类型统计
    result.by_stroke_type[ord(c.primary_stroke)] += 1
    
    # 按笔画数统计
    if not result.by_stroke_count.has_key(c.stroke_count):
      result.by_stroke_count[c.stroke_count] = 0
    result.by_stroke_count[c.stroke_count] += 1
    
    # 按结构统计
    result.by_structure[ord(c.structure)] += 1
    
    # 按偏旁统计
    if not result.by_radical.has_key(c.radical):
      result.by_radical[c.radical] = 0
    result.by_radical[c.radical] += 1

# ═══════════════════════════════════════════════════════════════
## 索引: 一看就知道怎么找
# ═══════════════════════════════════════════════════════════════

proc build_index*(chars: seq[Character]): Table[string, IndexPath] =
  # 按笔画类型 → 笔画数 → 结构 分组
  var groups: Table[string, seq[string]]
  
  for c in chars:
    let key = $ord(c.primary_stroke) & "." & $c.stroke_count & "." & $ord(c.structure)
    if not groups.has_key(key):
      groups[key] = @[]
    groups[key].add(c.ch)
  
  # 为每个字建立索引
  for c in chars:
    let key = $ord(c.primary_stroke) & "." & $c.stroke_count & "." & $ord(c.structure)
    let pos = groups[key].find(c.ch)
    
    var path: IndexPath
    case c.primary_stroke
    of stHeng: path.stroke_type = "横"
    of stShu:  path.stroke_type = "竖"
    of stPie:  path.stroke_type = "撇"
    of stNa:   path.stroke_type = "捺"
    of stZhe:  path.stroke_type = "折"
    
    path.stroke_count = c.stroke_count
    
    case c.structure
    of stSingle:    path.structure = "独体"
    of stLeftRight: path.structure = "左右"
    of stTopBottom: path.structure = "上下"
    of stSurround:  path.structure = "包围"
    
    path.position = pos
    path.radical = c.radical
    
    result[c.ch] = path

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(sr: StrokeRadix): string =
  var lines: seq[string] = @[]
  lines.add("═══ 笔画进制 · 统计 · 分类 · 索引 ═══")
  lines.add("  总字数: " & $sr.stats.total)
  lines.add("")
  
  lines.add("── 按笔画类型统计 ──")
  lines.add("  横: " & $sr.stats.by_stroke_type[0] & "字")
  lines.add("  竖: " & $sr.stats.by_stroke_type[1] & "字")
  lines.add("  撇: " & $sr.stats.by_stroke_type[2] & "字")
  lines.add("  捺: " & $sr.stats.by_stroke_type[3] & "字")
  lines.add("  折: " & $sr.stats.by_stroke_type[4] & "字")
  lines.add("")
  
  lines.add("── 按笔画数统计 ──")
  var stroke_counts: seq[int] = @[]
  for k in sr.stats.by_stroke_count.keys:
    stroke_counts.add(k)
  stroke_counts.sort()
  for sc in stroke_counts:
    lines.add("  " & $sc & "画: " & $sr.stats.by_stroke_count[sc] & "字")
  lines.add("")
  
  lines.add("── 按结构统计 ──")
  lines.add("  独体: " & $sr.stats.by_structure[0] & "字")
  lines.add("  左右: " & $sr.stats.by_structure[1] & "字")
  lines.add("  上下: " & $sr.stats.by_structure[2] & "字")
  lines.add("  包围: " & $sr.stats.by_structure[3] & "字")
  lines.add("")
  
  lines.add("── 按偏旁统计 ──")
  for radical, count in sr.stats.by_radical:
    lines.add("  " & radical & ": " & $count & "字")
  lines.add("")
  
  lines.add("── 索引表: 一看就知道怎么找 ──")
  lines.add("  字 │ 索引路径")
  lines.add("  ───┼──────────────────")
  for ch, path in sr.index:
    lines.add("  " & ch & " │ " & path.stroke_type & "." & $path.stroke_count & "." & path.structure & "." & $path.position & "(" & path.radical & ")")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  let chars = get_characters()
  let stats = compute_stats(chars)
  let index = build_index(chars)
  
  var sr = StrokeRadix(characters: chars, stats: stats, index: index, name: "笔画进制")
  
  echo sr.report()