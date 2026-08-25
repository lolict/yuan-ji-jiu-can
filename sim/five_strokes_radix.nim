## ═══════════════════════════════════════════════════════════════
## five_strokes_radix.nim — 五笔进制 · 从一笔一画开始
## ═══════════════════════════════════════════════════════════════
##
## 根: 横 · 竖 · 撇 · 捺 · 折 = 五个基础容器
##       所有汉字都由这五种笔画组合而成
##       笔画类型 = 进制容器
##       笔画数 = 容器内的位置
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables]

type
  StrokeType* = enum
    stHeng    = 0,  # 横
    stShu     = 1,  # 竖
    stPie     = 2,  # 撇
    stNa      = 3,  # 捺
    stZhe     = 4,  # 折

  Stroke* = object
    kind*: StrokeType
    name*: string
    char_repr*: string

  Character* = object
    ch*: string
    strokes*: seq[StrokeType]    # 笔画序列
    stroke_count*: int           # 总笔画数
    primary_stroke*: StrokeType  # 主笔画(第一笔)

  StrokeRadix* = object
    containers*: array[5, seq[Character]]  # 5个笔画容器
    stroke_names*: array[5, string]

# ═══════════════════════════════════════════════════════════════
## 五笔定义
# ═══════════════════════════════════════════════════════════════

const STROKE_NAMES* = ["横", "竖", "撇", "捺", "折"]
const STROKE_CHARS* = ["一", "丨", "丿", "丶", "乙"]

# ═══════════════════════════════════════════════════════════════
## 数据: 按第一笔(主笔画)分类
# ═══════════════════════════════════════════════════════════════

proc get_characters*(): seq[Character] =
  result = @[
    # 横起笔组
    Character(ch: "一", strokes: @[stHeng], stroke_count: 1, primary_stroke: stHeng),
    Character(ch: "二", strokes: @[stHeng, stHeng], stroke_count: 2, primary_stroke: stHeng),
    Character(ch: "三", strokes: @[stHeng, stHeng, stHeng], stroke_count: 3, primary_stroke: stHeng),
    Character(ch: "王", strokes: @[stHeng, stHeng, stShu, stHeng], stroke_count: 4, primary_stroke: stHeng),
    Character(ch: "木", strokes: @[stHeng, stShu, stPie, stNa], stroke_count: 4, primary_stroke: stHeng),
    Character(ch: "本", strokes: @[stHeng, stShu, stPie, stNa, stHeng], stroke_count: 5, primary_stroke: stHeng),
    Character(ch: "朱", strokes: @[stPie, stHeng, stShu, stPie, stNa], stroke_count: 5, primary_stroke: stPie),
    
    # 竖起笔组
    Character(ch: "丨", strokes: @[stShu], stroke_count: 1, primary_stroke: stShu),
    Character(ch: "上", strokes: @[stShu, stHeng, stHeng], stroke_count: 3, primary_stroke: stShu),
    Character(ch: "下", strokes: @[stHeng, stShu, stHeng], stroke_count: 3, primary_stroke: stHeng),
    Character(ch: "口", strokes: @[stShu, stZhe, stHeng], stroke_count: 3, primary_stroke: stShu),
    Character(ch: "日", strokes: @[stShu, stZhe, stHeng, stHeng], stroke_count: 4, primary_stroke: stShu),
    Character(ch: "目", strokes: @[stShu, stZhe, stHeng, stHeng, stHeng], stroke_count: 5, primary_stroke: stShu),
    Character(ch: "田", strokes: @[stShu, stZhe, stHeng, stShu, stHeng], stroke_count: 5, primary_stroke: stShu),
    
    # 撇起笔组
    Character(ch: "丿", strokes: @[stPie], stroke_count: 1, primary_stroke: stPie),
    Character(ch: "人", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie),
    Character(ch: "入", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie),
    Character(ch: "八", strokes: @[stPie, stNa], stroke_count: 2, primary_stroke: stPie),
    Character(ch: "大", strokes: @[stHeng, stPie, stNa], stroke_count: 3, primary_stroke: stHeng),
    Character(ch: "天", strokes: @[stHeng, stHeng, stPie, stNa], stroke_count: 4, primary_stroke: stHeng),
    Character(ch: "禾", strokes: @[stPie, stHeng, stShu, stPie, stNa], stroke_count: 5, primary_stroke: stPie),
    Character(ch: "千", strokes: @[stPie, stHeng, stShu], stroke_count: 3, primary_stroke: stPie),
    
    # 捺起笔组
    Character(ch: "丶", strokes: @[stNa], stroke_count: 1, primary_stroke: stNa),
    Character(ch: "义", strokes: @[stNa, stPie, stNa], stroke_count: 3, primary_stroke: stNa),
    Character(ch: "之", strokes: @[stNa, stZhe, stNa], stroke_count: 3, primary_stroke: stNa),
    Character(ch: "文", strokes: @[stNa, stHeng, stPie, stNa], stroke_count: 4, primary_stroke: stNa),
    Character(ch: "方", strokes: @[stNa, stHeng, stZhe, stPie], stroke_count: 4, primary_stroke: stNa),
    Character(ch: "六", strokes: @[stNa, stHeng, stPie, stNa], stroke_count: 4, primary_stroke: stNa),
    
    # 折起笔组
    Character(ch: "乙", strokes: @[stZhe], stroke_count: 1, primary_stroke: stZhe),
    Character(ch: "九", strokes: @[stPie, stZhe], stroke_count: 2, primary_stroke: stPie),
    Character(ch: "刀", strokes: @[stZhe, stPie], stroke_count: 2, primary_stroke: stZhe),
    Character(ch: "力", strokes: @[stZhe, stPie], stroke_count: 2, primary_stroke: stZhe),
    Character(ch: "了", strokes: @[stZhe, stShu], stroke_count: 2, primary_stroke: stZhe),
    Character(ch: "子", strokes: @[stZhe, stZhe, stShu], stroke_count: 3, primary_stroke: stZhe),
    Character(ch: "也", strokes: @[stZhe, stShu, stZhe], stroke_count: 3, primary_stroke: stZhe),
  ]

# ═══════════════════════════════════════════════════════════════
## 五笔进制: 按主笔画分组
# ═══════════════════════════════════════════════════════════════

proc build_stroke_radix*(characters: seq[Character]): StrokeRadix =
  for c in characters:
    let idx = ord(c.primary_stroke)
    result.containers[idx].add(c)
  
  for i in 0..4:
    result.stroke_names[i] = STROKE_NAMES[i]

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(sr: StrokeRadix): string =
  var lines: seq[string] = @[]
  lines.add("═══ 五笔进制 · 从一笔一画开始 ═══")
  lines.add("  基础容器: 横 · 竖 · 撇 · 捺 · 折")
  lines.add("")
  
  for i in 0..4:
    let container = sr.containers[i]
    lines.add("── " & STROKE_NAMES[i] & "起笔组 (" & $container.len & "字) ──")
    for c in container:
      lines.add("  " & c.ch & " (" & $c.stroke_count & "画)")
    lines.add("")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  let chars = get_characters()
  let sr = build_stroke_radix(chars)
  echo sr.report()