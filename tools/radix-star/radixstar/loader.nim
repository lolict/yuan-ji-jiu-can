# ============================================================
# 部首星图 · JSON 字典加载器
# 从真实字典数据文件加载汉字到星图系统
# ============================================================

import json
import streams
import strutils
import os

# 汉字条目
type
  CharEntry* = object
    character*: string
    radical_index*: int
    residual_strokes*: int
    total_strokes*: int
    pinyin*: string
    meaning*: string

# 加载统计
type
  LoadStats* = object
    total_entries*: int
    loaded*: int
    skipped*: int
    radical_coverage*: int  # 多少种不同部首

# 从 JSON 文件加载字典
proc load_dictionary*(filepath: string): (seq[CharEntry], LoadStats) =
  var entries: seq[CharEntry] = @[]
  var stats = LoadStats()
  var radical_set: seq[int] = @[]
  
  var f = newFileStream(filepath, fmRead)
  if f.isNil:
    echo "错误: 无法打开文件 " & filepath
    return (entries, stats)
  
  var parsed = parseJson(f.readAll())
  f.close()
  
  if parsed.kind != JArray:
    if parsed.hasKey("entries"):
      parsed = parsed["entries"]
    else:
      echo "错误: JSON 格式不正确"
      return (entries, stats)
  
  stats.total_entries = parsed.len
  
  for item in parsed:
    if item.kind != JArray or item.len < 4:
      inc stats.skipped
      continue
    
    let ch = item[0].getStr()
    let radical_idx = item[1].getInt()
    let residual = item[2].getInt()
    let total = item[3].getInt()
    let py = if item.len > 4: item[4].getStr() else: ""
    let mn = if item.len > 5: item[5].getStr() else: ""
    
    entries.add(CharEntry(
      character: ch,
      radical_index: radical_idx,
      residual_strokes: residual,
      total_strokes: total,
      pinyin: py,
      meaning: mn
    ))
    
    var found = false
    for r in radical_set:
      if r == radical_idx:
        found = true
        break
    if not found:
      radical_set.add(radical_idx)
    
    inc stats.loaded
  
  stats.radical_coverage = radical_set.len
  return (entries, stats)

# 快速加载（带默认路径）
proc load_default*(): (seq[CharEntry], LoadStats) =
  let default_paths = @[
    "dictionary.json",
    "radix-star/dictionary.json",
    "tools/radix-star/dictionary.json"
  ]
  
  for p in default_paths:
    if fileExists(p):
      return load_dictionary(p)
  
  echo "错误: 找不到 dictionary.json"
  return (@[], LoadStats())
