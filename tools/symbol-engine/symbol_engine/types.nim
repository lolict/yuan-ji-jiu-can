# ============================================================
# 符号引擎 · 类型定义
# 
# 核心思想：
#   符号表是数据，不是代码。
#   引擎编译一次，符号表随时可改。
#   多个符号可以是近义词（指向同一含义）。
#   系统解释器不定义你的符号，你的加载器定义。
# ============================================================

import tables
import sequtils
import strutils
import options

# 符号条目
type
  SymbolEntry* = object
    code*: string              # 64进制编码（如 "A1", "B3"）
    meaning*: string           # 含义
    category*: string          # 类别（编码/运算/层级/极性/容器）
    synonyms*: seq[string]     # 近义词编码列表
    description*: string       # 详细描述
    created_at*: string        # 创建时间
    version*: int              # 版本号

# 符号表
type
  SymbolTable* = object
    entries*: Table[string, SymbolEntry]      # code → entry
    meaning_index*: Table[string, seq[string]] # meaning → [codes]
    category_index*: Table[string, seq[string]] # category → [codes]
    total_symbols*: int
    meaning_count*: int
    category_count*: int
    version*: int
    last_modified*: string

# 解析结果
type
  ResolveResult* = object
    found*: bool
    entry*: Option[SymbolEntry]
    alternatives*: seq[string]  # 近义词建议
    path*: string               # 解析路径

# 构造函数
func make_entry*(code, meaning, category: string): SymbolEntry =
  SymbolEntry(
    code: code,
    meaning: meaning,
    category: category,
    synonyms: @[],
    description: "",
    created_at: "",
    version: 1
  )

func with_synonyms*(e: SymbolEntry, syns: seq[string]): SymbolEntry =
  result = e
  result.synonyms = syns

func with_desc*(e: SymbolEntry, desc: string): SymbolEntry =
  result = e
  result.description = desc

# 空符号表
func new_symbol_table*(): SymbolTable =
  SymbolTable(
    entries: init_table[string, SymbolEntry](),
    meaning_index: init_table[string, seq[string]](),
    category_index: init_table[string, seq[string]](),
    total_symbols: 0,
    meaning_count: 0,
    category_count: 0,
    version: 1,
    last_modified: ""
  )
