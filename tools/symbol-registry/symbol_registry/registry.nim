# ============================================================
# 统一符号注册中心
# 
# 把散落在所有工具里的符号，汇总到一个可搜索的统一表。
# 每个符号带有"来源工具"标记，方便追溯。
# ============================================================

import tables
import sequtils
import strutils
import options
import algorithm

# 符号来源
type
  SymbolSource* = enum
    ssMlll254,      # mlll-254 集装箱
    ssColorBase,    # 颜色进制
    ssBitContainer, # 分比容器
    ssManquanfa,    # 满全法
    ssCanjiang,     # 蚕蛹寄生壳
    ssMinlian,      # 民联浏览器
    ssNjsgSwarm,    # njsg-swarm
    ssRadixStar,    # 部首星图
    ssPropertyJudge,# 产权判定
    ssSymbolEngine, # 符号引擎
    ssUserDefined,  # 用户自定义
    ssOther         # 其他

# 统一符号条目
type
  UnifiedEntry* = object
    code*: string              # 编码
    meaning*: string           # 含义
    category*: string          # 类别
    source*: SymbolSource      # 来源工具
    source_name*: string       # 来源工具名称
    synonyms*: seq[string]     # 近义词
    description*: string       # 描述
    version*: int              # 版本

# 注册中心
type
  SymbolRegistry* = object
    entries*: Table[string, UnifiedEntry]
    by_source*: Table[SymbolSource, seq[string]]
    by_category*: Table[string, seq[string]]
    by_meaning*: Table[string, seq[string]]
    total*: int
    source_count*: int
    category_count*: int
    meaning_count*: int

# 来源名称
func source_name*(s: SymbolSource): string =
  case s
  of ssMlll254: "mlll-254"
  of ssColorBase: "color-base"
  of ssBitContainer: "bit-container"
  of ssManquanfa: "manquanfa"
  of ssCanjiang: "canjiang"
  of ssMinlian: "minlian"
  of ssNjsgSwarm: "njsg-swarm"
  of ssRadixStar: "radix-star"
  of ssPropertyJudge: "property-judge"
  of ssSymbolEngine: "symbol-engine"
  of ssUserDefined: "user-defined"
  of ssOther: "other"

# 创建注册中心
func new_registry*(): SymbolRegistry =
  SymbolRegistry(
    entries: init_table[string, UnifiedEntry](),
    by_source: init_table[SymbolSource, seq[string]](),
    by_category: init_table[string, seq[string]](),
    by_meaning: init_table[string, seq[string]](),
    total: 0,
    source_count: 0,
    category_count: 0,
    meaning_count: 0
  )

# 注册符号
func register*(reg: var SymbolRegistry, entry: UnifiedEntry) =
  let code = entry.code
  reg.entries[code] = entry
  
  # 按来源索引
  if not reg.by_source.hasKey(entry.source):
    reg.by_source[entry.source] = @[]
    inc reg.source_count
  if not reg.by_source[entry.source].contains(code):
    reg.by_source[entry.source].add(code)
  
  # 按类别索引
  if not reg.by_category.hasKey(entry.category):
    reg.by_category[entry.category] = @[]
    inc reg.category_count
  if not reg.by_category[entry.category].contains(code):
    reg.by_category[entry.category].add(code)
  
  # 按含义索引
  if not reg.by_meaning.hasKey(entry.meaning):
    reg.by_meaning[entry.meaning] = @[]
    inc reg.meaning_count
  if not reg.by_meaning[entry.meaning].contains(code):
    reg.by_meaning[entry.meaning].add(code)
  
  # 近义词也注册
  for syn in entry.synonyms:
    if not reg.by_meaning[entry.meaning].contains(syn):
      reg.by_meaning[entry.meaning].add(syn)
  
  inc reg.total

# 解析符号
func resolve*(reg: SymbolRegistry, code: string): Option[UnifiedEntry] =
  if reg.entries.hasKey(code):
    return some(reg.entries[code])
  
  # 检查近义词
  for meaning, codes in reg.by_meaning:
    if codes.contains(code):
      for c in codes:
        if reg.entries.hasKey(c):
          return some(reg.entries[c])
  
  return none(UnifiedEntry)

# 按来源查找
func lookup_source*(reg: SymbolRegistry, source: SymbolSource): seq[UnifiedEntry] =
  var results: seq[UnifiedEntry] = @[]
  if reg.by_source.hasKey(source):
    for code in reg.by_source[source]:
      if reg.entries.hasKey(code):
        results.add(reg.entries[code])
  return results

# 按类别查找
func lookup_category*(reg: SymbolRegistry, category: string): seq[UnifiedEntry] =
  var results: seq[UnifiedEntry] = @[]
  if reg.by_category.hasKey(category):
    for code in reg.by_category[category]:
      if reg.entries.hasKey(code):
        results.add(reg.entries[code])
  return results

# 按含义查找
func lookup_meaning*(reg: SymbolRegistry, meaning: string): seq[string] =
  if reg.by_meaning.hasKey(meaning):
    return reg.by_meaning[meaning]
  return @[]

# 搜索
func search*(reg: SymbolRegistry, query: string): seq[UnifiedEntry] =
  var results: seq[UnifiedEntry] = @[]
  let q = query.toLower()
  
  for code, entry in reg.entries:
    if code.toLower().contains(q) or 
       entry.meaning.toLower().contains(q) or
       entry.category.toLower().contains(q) or
       entry.description.toLower().contains(q) or
       entry.source_name.toLower().contains(q):
      results.add(entry)
  
  return results

# 统计报告
func registry_report*(reg: SymbolRegistry): string =
  var s = "═══ 统一符号注册中心报告 ═══\n"
  s &= "符号总数: " & $reg.total & "\n"
  s &= "来源工具: " & $reg.source_count & " 个\n"
  s &= "类别总数: " & $reg.category_count & " 种\n"
  s &= "含义总数: " & $reg.meaning_count & " 种\n"
  s &= "════════════════════════════\n"
  
  # 按来源分布
  s &= "按来源分布:\n"
  for source in SymbolSource:
    if reg.by_source.hasKey(source):
      let count = reg.by_source[source].len
      s &= "  " & source_name(source) & ": " & $count & " 符号\n"
  
  result = s

# 条目详情
func entry_detail*(e: UnifiedEntry): string =
  var s = "『" & e.code & "』→ " & e.meaning & "\n"
  s &= "  类别: " & e.category & "\n"
  s &= "  来源: " & e.source_name & "\n"
  if e.description.len > 0:
    s &= "  描述: " & e.description & "\n"
  if e.synonyms.len > 0:
    s &= "  近义词: " & e.synonyms.join(", ") & "\n"
  result = s
