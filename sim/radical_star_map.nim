## ═══════════════════════════════════════════════════════════════
## radical_star_map.nim — 部首星图算法 · 水/火/言三部端到端
## ═══════════════════════════════════════════════════════════════
##
## 核心: 部首是星图的中心 · 含该部首的字是围绕的星
##       星与星之间的距离 = 笔画差 + 结构差 + 发音差
##       星图算法: 给定部首 · 生成所有相关字的星图 · 计算相互距离
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  Character* = object
    ch*: string              # 汉字
    radical*: string         # 部首
    strokes*: int            # 笔画数
    structure*: string       # 结构: 左右/上下/包围/独体
    pronunciation*: string   # 发音(声母+韵母+声调)
    meaning*: string         # 含义

  StarNode* = object
    character*: Character
    x*: float                # 星图X坐标(基于笔画)
    y*: float                # 星图Y坐标(基于结构)
    z*: float                # 星图Z坐标(基于发音)
    distance_to_center*: float

  StarMap* = object
    center_radical*: string
    center*: StarNode
    stars*: seq[StarNode]
    connections*: seq[tuple[a: int, b: int, distance: float]]

# ═══════════════════════════════════════════════════════════════
## 数据: 水/火/言三部的字
# ═══════════════════════════════════════════════════════════════

proc get_shui_characters*(): seq[Character] =
  result = @[
    Character(ch: "水", radical: "水", strokes: 4, structure: "独体", pronunciation: "shui3", meaning: "水"),
    Character(ch: "江", radical: "水", strokes: 6, structure: "左右", pronunciation: "jiang1", meaning: "大河"),
    Character(ch: "河", radical: "水", strokes: 8, structure: "左右", pronunciation: "he2", meaning: "河流"),
    Character(ch: "海", radical: "水", strokes: 10, structure: "左右", pronunciation: "hai3", meaning: "大海"),
    Character(ch: "湖", radical: "水", strokes: 12, structure: "左右", pronunciation: "hu2", meaning: "湖泊"),
    Character(ch: "洋", radical: "水", strokes: 9, structure: "左右", pronunciation: "yang2", meaning: "海洋"),
    Character(ch: "池", radical: "水", strokes: 6, structure: "左右", pronunciation: "chi2", meaning: "池塘"),
    Character(ch: "汤", radical: "水", strokes: 6, structure: "左右", pronunciation: "tang1", meaning: "热水"),
    Character(ch: "汁", radical: "水", strokes: 5, structure: "左右", pronunciation: "zhi1", meaning: "汁液"),
    Character(ch: "汗", radical: "水", strokes: 6, structure: "左右", pronunciation: "han4", meaning: "汗水"),
    Character(ch: "泪", radical: "水", strokes: 8, structure: "左右", pronunciation: "lei4", meaning: "眼泪"),
    Character(ch: "泉", radical: "水", strokes: 9, structure: "上下", pronunciation: "quan2", meaning: "泉水"),
  ]

proc get_huo_characters*(): seq[Character] =
  result = @[
    Character(ch: "火", radical: "火", strokes: 4, structure: "独体", pronunciation: "huo3", meaning: "火"),
    Character(ch: "炎", radical: "火", strokes: 8, structure: "上下", pronunciation: "yan2", meaning: "炎热"),
    Character(ch: "焰", radical: "火", strokes: 12, structure: "左右", pronunciation: "yan4", meaning: "火焰"),
    Character(ch: "烧", radical: "火", strokes: 10, structure: "左右", pronunciation: "shao1", meaning: "燃烧"),
    Character(ch: "灯", radical: "火", strokes: 6, structure: "左右", pronunciation: "deng1", meaning: "灯"),
    Character(ch: "灰", radical: "火", strokes: 6, structure: "半包围", pronunciation: "hui1", meaning: "灰烬"),
    Character(ch: "炭", radical: "火", strokes: 9, structure: "上下", pronunciation: "tan4", meaning: "木炭"),
    Character(ch: "烟", radical: "火", strokes: 10, structure: "左右", pronunciation: "yan1", meaning: "烟"),
    Character(ch: "煮", radical: "火", strokes: 12, structure: "上下", pronunciation: "zhu3", meaning: "煮"),
    Character(ch: "热", radical: "火", strokes: 10, structure: "上下", pronunciation: "re4", meaning: "热"),
    Character(ch: "煌", radical: "火", strokes: 13, structure: "左右", pronunciation: "huang2", meaning: "辉煌"),
  ]

proc get_yan_characters*(): seq[Character] =
  result = @[
    Character(ch: "言", radical: "言", strokes: 7, structure: "独体", pronunciation: "yan2", meaning: "说话"),
    Character(ch: "话", radical: "言", strokes: 8, structure: "左右", pronunciation: "hua4", meaning: "话语"),
    Character(ch: "语", radical: "言", strokes: 9, structure: "左右", pronunciation: "yu3", meaning: "语言"),
    Character(ch: "说", radical: "言", strokes: 9, structure: "左右", pronunciation: "shuo1", meaning: "说话"),
    Character(ch: "词", radical: "言", strokes: 7, structure: "左右", pronunciation: "ci2", meaning: "词语"),
    Character(ch: "诗", radical: "言", strokes: 8, structure: "左右", pronunciation: "shi1", meaning: "诗歌"),
    Character(ch: "读", radical: "言", strokes: 10, structure: "左右", pronunciation: "du2", meaning: "阅读"),
    Character(ch: "论", radical: "言", strokes: 6, structure: "左右", pronunciation: "lun4", meaning: "讨论"),
    Character(ch: "讲", radical: "言", strokes: 6, structure: "左右", pronunciation: "jiang3", meaning: "讲话"),
    Character(ch: "记", radical: "言", strokes: 5, structure: "左右", pronunciation: "ji4", meaning: "记忆"),
    Character(ch: "认", radical: "言", strokes: 4, structure: "左右", pronunciation: "ren4", meaning: "认识"),
    Character(ch: "让", radical: "言", strokes: 5, structure: "左右", pronunciation: "rang4", meaning: "让"),
  ]

# ═══════════════════════════════════════════════════════════════
## 星图算法: 计算坐标
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

proc build_star_map*(radical: string, characters: seq[Character]): StarMap =
  result.center_radical = radical
  
  # 找中心字(部首本身)
  for c in characters:
    if c.ch == radical:
      result.center = StarNode(
        character: c,
        x: float(c.strokes),
        y: structure_to_y(c.structure),
        z: pronunciation_to_z(c.pronunciation),
        distance_to_center: 0.0
      )
      break
  
  # 生成星
  for c in characters:
    if c.ch != radical:
      let node = StarNode(
        character: c,
        x: float(c.strokes),
        y: structure_to_y(c.structure),
        z: pronunciation_to_z(c.pronunciation),
        distance_to_center: 0.0
      )
      result.stars.add(node)
  
  # 计算距离
  for i in 0 ..< result.stars.len:
    let dx = result.stars[i].x - result.center.x
    let dy = result.stars[i].y - result.center.y
    let dz = result.stars[i].z - result.center.z
    result.stars[i].distance_to_center = sqrt(dx*dx + dy*dy + dz*dz)
  
  # 按距离排序
  proc cmp(a, b: StarNode): int = cmp(a.distance_to_center, b.distance_to_center)
  result.stars.sort(cmp)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(sm: StarMap): string =
  var lines: seq[string] = @[]
  lines.add("═══ 部首星图: " & sm.center_radical & "部 ═══")
  lines.add("  中心: " & sm.center.character.ch &
    " (笔画=" & $sm.center.character.strokes &
    ", 结构=" & sm.center.character.structure &
    ", 发音=" & sm.center.character.pronunciation & ")")
  lines.add("  星数: " & $sm.stars.len)
  lines.add("")
  lines.add("  星 │ 笔画 │ 结构   │ 发音   │ 含义   │ 距中心")
  lines.add("  ───┼──────┼────────┼────────┼────────┼───────")
  
  for star in sm.stars:
    lines.add("  " & star.character.ch & " │ " & $star.character.strokes &
      " │ " & star.character.structure &
      " │ " & star.character.pronunciation &
      " │ " & star.character.meaning &
      " │ " & formatFloat(star.distance_to_center, ffDecimal, 2))
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 部首星图算法 · 端到端验证 ═══"
  echo ""
  
  # 水部
  let shui_chars = get_shui_characters()
  let shui_map = build_star_map("水", shui_chars)
  echo shui_map.report()
  echo ""
  
  # 火部
  let huo_chars = get_huo_characters()
  let huo_map = build_star_map("火", huo_chars)
  echo huo_map.report()
  echo ""
  
  # 言部
  let yan_chars = get_yan_characters()
  let yan_map = build_star_map("言", yan_chars)
  echo yan_map.report()