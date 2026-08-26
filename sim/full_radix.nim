## ═══════════════════════════════════════════════════════════════
## full_radix.nim — 全维度分类 · 笔画/结构/偏旁/思想范畴
## ═══════════════════════════════════════════════════════════════
##
## 分类(大) → 聚类(小) → 索引(找)
##       笔画类型 → 笔画数 → 结构 → 偏旁 → 思想范畴

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables]

type
  StrokeType* = enum
    stHeng = 0, stShu = 1, stPie = 2, stNa = 3, stZhe = 4

  Structure* = enum
    stSingle = 0, stLeftRight = 1, stTopBottom = 2, stSurround = 3

  Category* = enum    # 思想范畴
    catWater = 0,     # 水
    catWood = 1,      # 木
    catFire = 2,      # 火
    catMetal = 3,     # 金
    catEarth = 4,     # 土
    catHuman = 5,     # 人
    catNature = 6,    # 自然
    catAbstract = 7,  # 抽象

  Character* = object
    ch*: string
    stroke_count*: int
    primary_stroke*: StrokeType
    structure*: Structure
    radical*: string
    category*: Category
    meaning*: string

  Radix* = object
    characters*: seq[Character]
    name*: string

# ═══════════════════════════════════════════════════════════════
## 数据
# ═══════════════════════════════════════════════════════════════

proc get_chars*(): seq[Character] =
  result = @[
    # 水范畴
    Character(ch: "江", stroke_count: 7, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "大河"),
    Character(ch: "河", stroke_count: 8, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "河流"),
    Character(ch: "湖", stroke_count: 12, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "湖泊"),
    Character(ch: "海", stroke_count: 11, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "大海"),
    Character(ch: "池", stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "池塘"),
    Character(ch: "汗", stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "汗水"),
    Character(ch: "汤", stroke_count: 6, primary_stroke: stNa, structure: stLeftRight, radical: "氵", category: catWater, meaning: "热水"),
    Character(ch: "水", stroke_count: 4, primary_stroke: stNa, structure: stSingle, radical: "水", category: catWater, meaning: "水"),
    
    # 木范畴
    Character(ch: "木", stroke_count: 4, primary_stroke: stHeng, structure: stSingle, radical: "木", category: catWood, meaning: "木"),
    Character(ch: "本", stroke_count: 5, primary_stroke: stHeng, structure: stSingle, radical: "木", category: catWood, meaning: "根本"),
    Character(ch: "朱", stroke_count: 5, primary_stroke: stPie, structure: stSingle, radical: "木", category: catWood, meaning: "红色"),
    Character(ch: "禾", stroke_count: 5, primary_stroke: stPie, structure: stSingle, radical: "禾", category: catWood, meaning: "禾苗"),
    
    # 火范畴
    Character(ch: "火", stroke_count: 4, primary_stroke: stNa, structure: stSingle, radical: "火", category: catFire, meaning: "火"),
    Character(ch: "炎", stroke_count: 8, primary_stroke: stNa, structure: stTopBottom, radical: "火", category: catFire, meaning: "炎热"),
    
    # 人范畴
    Character(ch: "人", stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "人", category: catHuman, meaning: "人类"),
    Character(ch: "入", stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "入", category: catHuman, meaning: "进入"),
    Character(ch: "八", stroke_count: 2, primary_stroke: stPie, structure: stSingle, radical: "八", category: catHuman, meaning: "数字八"),
    Character(ch: "千", stroke_count: 3, primary_stroke: stPie, structure: stSingle, radical: "千", category: catHuman, meaning: "一千"),
    
    # 自然范畴
    Character(ch: "日", stroke_count: 4, primary_stroke: stShu, structure: stSingle, radical: "日", category: catNature, meaning: "太阳"),
    Character(ch: "月", stroke_count: 4, primary_stroke: stPie, structure: stSingle, radical: "月", category: catNature, meaning: "月亮"),
    Character(ch: "目", stroke_count: 5, primary_stroke: stShu, structure: stSingle, radical: "目", category: catNature, meaning: "眼睛"),
    Character(ch: "田", stroke_count: 5, primary_stroke: stShu, structure: stSingle, radical: "田", category: catNature, meaning: "田地"),
    Character(ch: "口", stroke_count: 3, primary_stroke: stShu, structure: stSingle, radical: "口", category: catNature, meaning: "嘴巴"),
  ]

# ═══════════════════════════════════════════════════════════════
## 分类 → 聚类 → 索引
# ═══════════════════════════════════════════════════════════════

proc classify*(r: Radix): string =
  var lines: seq[string] = @[]
  lines.add("═══ 全维度分类 · 思想范畴 → 偏旁 → 笔画数 → 结构 ═══")
  lines.add("  总字数: " & $r.characters.len)
  lines.add("")
  
  for cat in Category:
    let cat_chars = r.characters.filter(proc(c: Character): bool = c.category == cat)
    if cat_chars.len == 0: continue
    
    let cat_name = case cat
      of catWater: "水"
      of catWood: "木"
      of catFire: "火"
      of catMetal: "金"
      of catEarth: "土"
      of catHuman: "人"
      of catNature: "自然"
      of catAbstract: "抽象"
    
    lines.add("── " & cat_name & "范畴 (" & $cat_chars.len & "字) ──")
    
    # 按偏旁聚类
    var by_radical: Table[string, seq[string]]
    for c in cat_chars:
      if not by_radical.has_key(c.radical):
        by_radical[c.radical] = @[]
      by_radical[c.radical].add(c.ch & "(" & $c.stroke_count & "画)")
    
    for radical, chars in by_radical:
      lines.add("  " & radical & ": " & chars.join(", "))
    lines.add("")
  
  result = lines.join("\n")

when isMainModule:
  let r = Radix(characters: get_chars(), name: "全维度进制")
  echo r.classify()