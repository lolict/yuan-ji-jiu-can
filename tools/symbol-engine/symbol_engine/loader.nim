# ============================================================
# 符号引擎 · JSON 加载器
# 
# 符号表是纯数据文件（JSON），引擎编译一次，数据随时可改。
# 不重新编译核心，只需修改 JSON 即可扩展符号系统。
# ============================================================

import json
import streams
import strutils
import os
import tables
import types
import engine

# 从 JSON 文件加载符号表
proc load_from_json*(filepath: string): SymbolTable =
  var st = new_symbol_table()
  
  var f = newFileStream(filepath, fmRead)
  if f.isNil:
    echo "错误: 无法打开符号表文件 " & filepath
    return st
  
  let parsed = parseJson(f.readAll())
  f.close()
  
  if parsed.kind != JArray:
    if parsed.hasKey("symbols"):
      let meta = parsed
      if meta.hasKey("version") and meta["version"].kind == JInt:
        st.version = meta["version"].getInt()
      let symbols = parsed["symbols"]
      if symbols.kind == JArray:
        for item in symbols:
          let code = item["code"].getStr()
          let meaning = item["meaning"].getStr()
          let category = if item.hasKey("category"): item["category"].getStr() else: "未分类"
          let desc = if item.hasKey("description"): item["description"].getStr() else: ""
          let created = if item.hasKey("created"): item["created"].getStr() else: ""
          let ver = if item.hasKey("version"): item["version"].getInt() else: 1
          var syns: seq[string] = @[]
          if item.hasKey("synonyms"):
            for s in item["synonyms"]:
              syns.add(s.getStr())
          
          var entry = SymbolEntry(
            code: code,
            meaning: meaning,
            category: category,
            synonyms: syns,
            description: desc,
            created_at: created,
            version: ver
          )
          st.add(entry)
    return st
  
  # 直接是数组格式
  for item in parsed:
    if item.kind != JArray or item.len < 3:
      continue
    let code = item[0].getStr()
    let meaning = item[1].getStr()
    let category = if item.len > 2: item[2].getStr() else: "未分类"
    var syns: seq[string] = @[]
    if item.len > 3 and item[3].kind == JArray:
      for s in item[3]:
        syns.add(s.getStr())
    
    var entry = make_entry(code, meaning, category).with_synonyms(syns)
    st.add(entry)
  
  return st

# 保存符号表到 JSON
proc save_to_json*(st: SymbolTable, filepath: string) =
  var arr = newJArray()
  
  for code, entry in st.entries:
    var obj = newJObject()
    obj["code"] = newJString(entry.code)
    obj["meaning"] = newJString(entry.meaning)
    obj["category"] = newJString(entry.category)
    obj["description"] = newJString(entry.description)
    obj["created"] = newJString(entry.created_at)
    obj["version"] = newJInt(entry.version)
    var syn_arr = newJArray()
    for s in entry.synonyms:
      syn_arr.add(newJString(s))
    obj["synonyms"] = syn_arr
    arr.add(obj)
  
  let text = pretty(arr)
  writeFile(filepath, text)

# 加载默认位置的符号表
proc load_default*(): SymbolTable =
  let default_paths = @[
    "symbols.json",
    "symbol-engine/symbols.json",
    "tools/symbol-engine/symbols.json"
  ]
  
  for p in default_paths:
    if fileExists(p):
      return load_from_json(p)
  
  return new_symbol_table()
