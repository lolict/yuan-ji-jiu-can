## ═══════════════════════════════════════════════════════════════
## stroke_radix.nim — 笔画进制 · 同笔画的字归为一组
## ═══════════════════════════════════════════════════════════════
##
## 核心: 笔画数相同的字归为一个进制容器
##       4画的字一组 · 5画的字一组 · 6画的字一组...
##       组内字与字之间的距离 = 结构差 + 发音差（笔画相同 · 笔画差=0）
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  Character* = object
    ch*: string
    radical*: string
    strokes*: int
    structure*: string
    pronunciation*: string
    meaning*: string

  StrokeGroup* = object
    stroke_count*: int
    characters*: seq[Character]
    center*: Character            # 该笔画组的中心（部首字）

  StrokeRadix* = object
    groups*: seq[StrokeGroup]
    stroke_index*: Table[int, int]  # 笔画数 → group index

# ═══════════════════════════════════════════════════════════════
## 数据: 按笔画数严格分组
# ═══════════════════════════════════════════════════════════════

proc get_characters_by_strokes*(): seq[Character] =
  result = @[
    # 4画组
    Character(ch: "水", radical: "水", strokes: 4, structure: "独体", pronunciation: "shui3", meaning: "水"),
    Character(ch: "火", radical: "火", strokes: 4, structure: "独体", pronunciation: "huo3", meaning: "火"),
    Character(ch: "木", radical: "木", strokes: 4, structure: "独体", pronunciation: "mu4", meaning: "木"),
    Character(ch: "日", radical: "日", strokes: 4, structure: "独体", pronunciation: "ri4", meaning: "日"),
    Character(ch: "月", radical: "月", strokes: 4, structure: "独体", pronunciation: "yue4", meaning: "月"),
    Character(ch: "心", radical: "心", strokes: 4, structure: "独体", pronunciation: "xin1", meaning: "心"),
    Character(ch: "言", radical: "言", strokes: 4, structure: "独体", pronunciation: "yan2", meaning: "言"),
    
    # 5画组
    Character(ch: "汁", radical: "水", strokes: 5, structure: "左右", pronunciation: "zhi1", meaning: "汁液"),
    Character(ch: "白", radical: "白", strokes: 5, structure: "独体", pronunciation: "bai2", meaning: "白色"),
    Character(ch: "目", radical: "目", strokes: 5, structure: "独体", pronunciation: "mu4", meaning: "眼睛"),
    Character(ch: "石", radical: "石", strokes: 5, structure: "独体", pronunciation: "shi2", meaning: "石头"),
    Character(ch: "示", radical: "示", strokes: 5, structure: "独体", pronunciation: "shi4", meaning: "展示"),
    
    # 6画组
    Character(ch: "江", radical: "水", strokes: 6, structure: "左右", pronunciation: "jiang1", meaning: "大河"),
    Character(ch: "池", radical: "水", strokes: 6, structure: "左右", pronunciation: "chi2", meaning: "池塘"),
    Character(ch: "汗", radical: "水", strokes: 6, structure: "左右", pronunciation: "han4", meaning: "汗水"),
    Character(ch: "汤", radical: "水", strokes: 6, structure: "左右", pronunciation: "tang1", meaning: "热水"),
    Character(ch: "灯", radical: "火", strokes: 6, structure: "左右", pronunciation: "deng1", meaning: "灯"),
    Character(ch: "灰", radical: "火", strokes: 6, structure: "半包围", pronunciation: "hui1", meaning: "灰烬"),
    Character(ch: "百", radical: "白", strokes: 6, structure: "上下", pronunciation: "bai3", meaning: "一百"),
    Character(ch: "竹", radical: "竹", strokes: 6, structure: "独体", pronunciation: "zhu2", meaning: "竹子"),
    
    # 7画组
    Character(ch: "言", radical: "言", strokes: 7, structure: "独体", pronunciation: "yan2", meaning: "说话"),
    Character(ch: "词", radical: "言", strokes: 7, structure: "左右", pronunciation: "ci2", meaning: "词语"),
    Character(ch: "河", radical: "水", strokes: 7, structure: "左右", pronunciation: "he2", meaning: "河流"),
    Character(ch: "泪", radical: "水", strokes: 7, structure: "左右", pronunciation: "lei4", meaning: "眼泪"),
    
    # 8画组
    Character(ch: "炎", radical: "火", strokes: 8, structure: "上下", pronunciation: "yan2", meaning: "炎热"),
    Character(ch: "海", radical: "水", strokes: 8, structure: "左右", pronunciation: "hai3", meaning: "大海"),
    Character(ch: "诗", radical: "言", strokes: 8, structure: "左右", pronunciation: "shi1", meaning: "诗歌"),
  ]

# ═══════════════════════════════════════════════════════════════
## 笔画进制: 按笔画数分组
# ═══════════════════════════════════════════════════════════════

proc build_stroke_radix*(characters: seq[Character]): StrokeRadix =
  # 按笔画数分组
  var groups_table: Table[int, seq[Character]]
  
  for c in characters:
    if not groups_table.has_key(c.strokes):
      groups_table[c.strokes] = @[]
    groups_table[c.strokes].add(c)
  
  # 构建StrokeGroup
  for stroke_count, chars in groups_table:
    var group: StrokeGroup
    group.stroke_count = stroke_count
    group.characters = chars
    
    # 找中心（部首字优先 · 否则取第一个）
    group.center = chars[0]
    for c in chars:
      if c.ch == c.radical:  # 本身是部首
        group.center = c
        break
    
    result.groups.add(group)
    result.stroke_index[stroke_count] = result.groups.high
  
  # 按笔画数排序
  proc cmp(a, b: StrokeGroup): int = cmp(a.stroke_count, b.stroke_count)
  result.groups.sort(cmp)

# ═══════════════════════════════════════════════════════════════
## 计算组内距离（笔画相同 · 只算结构+发音）
# ═══════════════════════════════════════════════════════════════

proc structure_to_y*(s: string): float =
  case s
  of "独体": result = 0.0
  of "左右": result = 1.0
  of "上下": result = 2.0
  of "半包围": result = 3.0
  of "全包围": result = 4.0
  else: result = -1.0

proc pronunciation_to_z*(p: string): float =
  if p.len >= 2:
    let tone = parseInt($p[^1])
    result = float(tone)
  else:
    result = 0.0

proc distance_in_group*(a, b: Character): float =
  ## 笔画相同 · 距离 = sqrt(结构差^2 + 发音差^2)
  let dy = structure_to_y(a.structure) - structure_to_y(b.structure)
  let dz = pronunciation_to_z(a.pronunciation) - pronunciation_to_z(b.pronunciation)
  result = sqrt(dy*dy + dz*dz)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(sr: StrokeRadix): string =
  var lines: seq[string] = @[]
  lines.add("═══ 笔画进制 · 同笔画归组 ═══")
  lines.add("  组数: " & $sr.groups.len)
  lines.add("")
  
  for group in sr.groups:
    lines.add("── " & $group.stroke_count & "画组 (" & $group.characters.len & "字) ──")
    lines.add("  中心: " & group.center.ch & "(" & group.center.radical & ")")
    
    for c in group.characters:
      if c.ch != group.center.ch:
        let dist = distance_in_group(c, group.center)
        lines.add("  " & c.ch & " | " & c.radical & " | " & c.structure & " | " & c.pronunciation & " | " & c.meaning & " | 距中心=" & formatFloat(dist, ffDecimal, 2))
    lines.add("")
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  let chars = get_characters_by_strokes()
  let sr = build_stroke_radix(chars)
  echo sr.report()