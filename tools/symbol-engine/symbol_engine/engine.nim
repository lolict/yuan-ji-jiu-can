# ============================================================
# 符号引擎 · 核心解析逻辑
# 
# 动态解析：不依赖编译时符号，运行时从符号表查找
# 近义词支持：多个编码指向同一含义，避免冲突
# ============================================================

import tables
import sequtils
import strutils
import options
import algorithm
import types

# 添加符号（支持近义词）
func add*(st: var SymbolTable, entry: SymbolEntry) =
  let code = entry.code
  
  # 主条目
  st.entries[code] = entry
  
  # 含义索引（支持近义词）
  if not st.meaning_index.hasKey(entry.meaning):
    st.meaning_index[entry.meaning] = @[]
    inc st.meaning_count
  if not st.meaning_index[entry.meaning].contains(code):
    st.meaning_index[entry.meaning].add(code)
  
  # 近义词也指向同一含义
  for syn in entry.synonyms:
    if not st.meaning_index[entry.meaning].contains(syn):
      st.meaning_index[entry.meaning].add(syn)
  
  # 类别索引
  if not st.category_index.hasKey(entry.category):
    st.category_index[entry.category] = @[]
    inc st.category_count
  if not st.category_index[entry.category].contains(code):
    st.category_index[entry.category].add(code)
  
  inc st.total_symbols

# 移除符号
func remove*(st: var SymbolTable, code: string) =
  if st.entries.hasKey(code):
    let entry = st.entries[code]
    
    # 移除含义索引
    if st.meaning_index.hasKey(entry.meaning):
      st.meaning_index[entry.meaning] = st.meaning_index[entry.meaning].filterIt(it != code)
      if st.meaning_index[entry.meaning].len == 0:
        st.meaning_index.del(entry.meaning)
    
    # 移除类别索引
    if st.category_index.hasKey(entry.category):
      st.category_index[entry.category] = st.category_index[entry.category].filterIt(it != code)
      if st.category_index[entry.category].len == 0:
        st.category_index.del(entry.category)
    
    # 移除主条目
    st.entries.del(code)
    dec st.total_symbols

# 核心解析函数：编码 → 含义
func resolve*(st: SymbolTable, code: string): ResolveResult =
  var path = "resolve(" & code & ")"
  
  # 直接查找
  if st.entries.hasKey(code):
    let entry = st.entries[code]
    var alts: seq[string] = @[]
    
    # 收集近义词（不包括自己）
    for syn in entry.synonyms:
      if syn != code:
        alts.add(syn)
    
    # 同含义的其他编码
    if st.meaning_index.hasKey(entry.meaning):
      for c in st.meaning_index[entry.meaning]:
        if c != code and not alts.contains(c):
          alts.add(c)
    
    return ResolveResult(
      found: true,
      entry: some(entry),
      alternatives: alts,
      path: path & " → found"
    )
  
  # 没找到，检查是否是某个近义词
  for meaning, codes in st.meaning_index:
    if codes.contains(code):
      # 找到映射但是主条目
      for c in codes:
        if st.entries.hasKey(c):
          let entry = st.entries[c]
          return ResolveResult(
            found: true,
            entry: some(entry),
            alternatives: codes.filterIt(it != code),
            path: path & " → synonym_of(" & c & ")"
          )
  
  return ResolveResult(
    found: false,
    entry: none(SymbolEntry),
    alternatives: @[],
    path: path & " → not_found"
  )

# 反向查找：含义 → 所有编码
func lookup_meaning*(st: SymbolTable, meaning: string): seq[string] =
  if st.meaning_index.hasKey(meaning):
    return st.meaning_index[meaning]
  return @[]

# 按类别查找
func lookup_category*(st: SymbolTable, category: string): seq[SymbolEntry] =
  var results: seq[SymbolEntry] = @[]
  if st.category_index.hasKey(category):
    for code in st.category_index[category]:
      if st.entries.hasKey(code):
        results.add(st.entries[code])
  return results

# 搜索（模糊匹配）
func search*(st: SymbolTable, query: string): seq[SymbolEntry] =
  var results: seq[SymbolEntry] = @[]
  let q = query.toLower()
  
  for code, entry in st.entries:
    if code.toLower().contains(q) or entry.meaning.toLower().contains(q) or entry.description.toLower().contains(q):
      results.add(entry)
  
  return results

# 添加近义词关系到现有符号
func link_synonyms*(st: var SymbolTable, code: string, syns: seq[string]): bool =
  if not st.entries.hasKey(code):
    return false
  
  var entry = st.entries[code]
  for s in syns:
    if not entry.synonyms.contains(s):
      entry.synonyms.add(s)
    # 也反向链接
    if not st.entries.hasKey(s):
      # 创建一个"影子"条目指向同一含义
      var shadow = entry
      shadow.code = s
      shadow.synonyms = @[code]
      st.entries[s] = shadow
    # 更新含义索引
    if not st.meaning_index.hasKey(entry.meaning):
      st.meaning_index[entry.meaning] = @[]
    if not st.meaning_index[entry.meaning].contains(s):
      st.meaning_index[entry.meaning].add(s)
  
  st.entries[code] = entry
  return true

# 统计表状态
func table_stats*(st: SymbolTable): string =
  var s = "═══ 符号表统计 ═══\n"
  s &= "符号总数: " & $st.total_symbols & "\n"
  s &= "含义总数: " & $st.meaning_count & "\n"
  s &= "类别总数: " & $st.category_count & "\n"
  
  # 近义词组统计
  var syn_groups = 0
  for meaning, codes in st.meaning_index:
    if codes.len > 1:
      inc syn_groups
  
  s &= "近义词组: " & $syn_groups & "\n"
  s &= "═══════════════════\n"
  result = s

# 条目详情
func entry_detail*(e: SymbolEntry): string =
  var s = "『" & e.code & "』→ " & e.meaning & "\n"
  s &= "  类别: " & e.category & "\n"
  s &= "  描述: " & e.description & "\n"
  if e.synonyms.len > 0:
    s &= "  近义词: " & e.synonyms.join(", ") & "\n"
  s &= "  v" & $e.version & "\n"
  result = s

# 解析结果描述
func resolve_desc*(r: ResolveResult): string =
  if r.found:
    let e = r.entry.get()
    var s = "✅ " & e.code & " → " & e.meaning
    if r.alternatives.len > 0:
      s &= "  [近义词: " & r.alternatives.join(", ") & "]"
    return s
  else:
    return "❌ 未找到"
