# ============================================================
# 部首星图 · 核心引擎
# 以部首为恒星、汉字为行星的层级检索系统
# ============================================================

import tables
import sequtils
import strutils
import options
import algorithm
import loader

# 星图位置
type
  StarPosition* = object
    character*: string
    radical_index*: int
    radical_name*: string
    residual_strokes*: int
    total_strokes*: int
    siblings_count*: int
    orbit*: int

# 星图引擎
type
  StarMap* = object
    entries*: seq[CharEntry]
    by_radical*: Table[int, seq[CharEntry]]
    by_strokes*: Table[int, seq[CharEntry]]
    radical_names*: Table[int, string]
    radical_count*: int
    stroke_count*: int

# 214 康熙部首名称表（常用部分）
func radical_name*(idx: int): string =
  let names = {
    1: "一(横)", 2: "丨(竖)", 3: "丶(点)", 9: "人",
    28: "口", 35: "女", 57: "心", 68: "日",
    71: "木", 81: "水", 82: "火", 145: "言"
  }.toTable
  if names.hasKey(idx):
    return names[idx]
  return "部首" & $idx

# 从加载的条目构建星图
func build_star_map*(entries: seq[CharEntry]): StarMap =
  result = StarMap(
    entries: entries,
    by_radical: init_table[int, seq[CharEntry]](),
    by_strokes: init_table[int, seq[CharEntry]](),
    radical_names: init_table[int, string](),
    radical_count: 0,
    stroke_count: 0
  )
  
  var radical_seen: seq[int] = @[]
  var stroke_seen: seq[int] = @[]
  
  for e in entries:
    # 按部首索引
    if not result.by_radical.hasKey(e.radical_index):
      result.by_radical[e.radical_index] = @[]
      radical_seen.add(e.radical_index)
    result.by_radical[e.radical_index].add(e)
    
    # 按笔画索引
    if not result.by_strokes.hasKey(e.total_strokes):
      result.by_strokes[e.total_strokes] = @[]
      stroke_seen.add(e.total_strokes)
    result.by_strokes[e.total_strokes].add(e)
    
    # 部首名称
    if not result.radical_names.hasKey(e.radical_index):
      result.radical_names[e.radical_index] = radical_name(e.radical_index)
  
  result.radical_count = radical_seen.len
  result.stroke_count = stroke_seen.len

# 查找汉字的星图位置
func star_position*(sm: StarMap, ch: string): Option[StarPosition] =
  for e in sm.entries:
    if e.character == ch:
      let siblings = if sm.by_radical.hasKey(e.radical_index):
        sm.by_radical[e.radical_index].len
      else: 0
      let orbit = (e.residual_strokes mod 8) + (siblings div 10)
      return some(StarPosition(
        character: ch,
        radical_index: e.radical_index,
        radical_name: if sm.radical_names.hasKey(e.radical_index): sm.radical_names[e.radical_index] else: "未知",
        residual_strokes: e.residual_strokes,
        total_strokes: e.total_strokes,
        siblings_count: siblings,
        orbit: orbit
      ))
  return none(StarPosition)

# 按部首查找所有汉字
func lookup_by_radical*(sm: StarMap, radical_idx: int): seq[CharEntry] =
  if sm.by_radical.hasKey(radical_idx):
    return sm.by_radical[radical_idx]
  return @[]

# 按笔画查找汉字
func lookup_by_strokes*(sm: StarMap, strokes: int): seq[CharEntry] =
  if sm.by_strokes.hasKey(strokes):
    return sm.by_strokes[strokes]
  return @[]

# 搜索：按部首 + 可选余笔画
func search*(sm: StarMap, radical_idx: int, residual: Option[int] = none(int)): seq[CharEntry] =
  let candidates = sm.lookup_by_radical(radical_idx)
  if residual.isNone:
    return candidates
  let r = residual.get()
  return candidates.filterIt(it.residual_strokes == r)

# 部首共振检测（公约数 > 1）
func radical_gcd*(a, b: int): int =
  var x = a
  var y = b
  while y != 0:
    let t = y
    y = x mod y
    x = t
  return x

func radicals_resonate*(a, b: int): bool =
  radical_gcd(a, b) > 1

# 星图统计
func star_map_stats*(sm: StarMap): string =
  var s = "═══ 星图统计 ═══\n"
  s &= "汉字总数: " & $sm.entries.len & "\n"
  s &= "部首覆盖: " & $sm.radical_count & " 种\n"
  s &= "笔画范围: " & $sm.stroke_count & " 种\n"
  
  # 找最大部首
  var max_radical = 0
  var max_count = 0
  for idx, chars in sm.by_radical:
    if chars.len > max_count:
      max_count = chars.len
      max_radical = idx
  
  s &= "最大部首: " & radical_name(max_radical) & " (" & $max_count & " 字)\n"
  result = s

# 星图位置描述
func position_desc*(sp: StarPosition): string =
  var s = "『" & sp.character & "』"
  s &= " 部首" & sp.radical_name & "(#" & $sp.radical_index & ")"
  s &= " 总" & $sp.total_strokes & "画"
  s &= " 余" & $sp.residual_strokes & "画"
  s &= " 第" & $sp.orbit & "轨道"
  s &= " 兄弟" & $sp.siblings_count & "个"
  result = s

# 共振报告
func resonance_report*(sm: StarMap): string =
  var s = "═══ 部首共振图 ═══\n"
  var indices: seq[int] = @[]
  for k in sm.by_radical.keys:
    indices.add(k)
  indices.sort()
  
  for i in 0..<indices.len:
    for j in (i+1)..<indices.len:
      let a = indices[i]
      let b = indices[j]
      if radicals_resonate(a, b):
        let g = radical_gcd(a, b)
        s &= "  " & radical_name(a) & "(#" & $a & ") ⟷ " & radical_name(b) & "(#" & $b & ")"
        s &= "  公约数=" & $g & "\n"
  
  result = s
