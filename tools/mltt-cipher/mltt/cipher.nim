# ============================================================
# mltt 命名密码表 · 核心引擎
#
# 家庭命名密码表 + 动漫角色映射 + 极性标注
# 每个短码都有独立含义，不是随机字母。
# ============================================================

import tables
import sequtils
import strutils
import options
import ./types

# 创建空密码表
proc new_cipher_table*(): CipherTable =
  result = CipherTable(
    entries: init_table[string, CipherEntry](),
    meaning_index: init_table[string, string](),
    category_index: init_table[string, seq[string]](),
    polarity_index: init_table[Polarity, seq[string]](),
    total: 0
  )

# 注册一条密码
proc register*(table: var CipherTable, entry: CipherEntry): bool =
  if table.entries.hasKey(entry.code):
    return false  # 已存在，不覆盖
  
  table.entries[entry.code] = entry
  table.meaning_index[entry.meaning] = entry.code
  
  # 更新类别索引
  if not table.category_index.hasKey(entry.category):
    table.category_index[entry.category] = @[]
  table.category_index[entry.category].add(entry.code)
  
  # 更新极性索引
  if not table.polarity_index.hasKey(entry.polarity):
    table.polarity_index[entry.polarity] = @[]
  table.polarity_index[entry.polarity].add(entry.code)
  
  table.total += 1
  return true

# 短码 → 条目
proc lookup*(table: CipherTable, code: string): Option[CipherEntry] =
  if table.entries.hasKey(code):
    return some(table.entries[code])
  return none(CipherEntry)

# 含义 → 短码
proc reverse_lookup*(table: CipherTable, meaning: string): Option[string] =
  if table.meaning_index.hasKey(meaning):
    return some(table.meaning_index[meaning])
  return none(string)

# 按类别筛选
proc by_category*(table: CipherTable, category: string): seq[CipherEntry] =
  result = @[]
  if table.category_index.hasKey(category):
    for code in table.category_index[category]:
      if table.entries.hasKey(code):
        result.add(table.entries[code])

# 按极性筛选
proc by_polarity*(table: CipherTable, polarity: Polarity): seq[CipherEntry] =
  result = @[]
  if table.polarity_index.hasKey(polarity):
    for code in table.polarity_index[polarity]:
      if table.entries.hasKey(code):
        result.add(table.entries[code])

# 极性 → 中文
proc polarity_name*(p: Polarity): string =
  case p
  of yin: "阴(容器)"
  of yang: "阳(界线)"
  of balanced: "平衡"

# 搜索（模糊匹配短码或含义）
proc search*(table: CipherTable, query: string): seq[CipherEntry] =
  result = @[]
  let q = query.toLowerAscii
  for code, entry in table.entries:
    if code.toLowerAscii.contains(q) or entry.meaning.toLowerAscii.contains(q):
      result.add(entry)

# 统计表状态
proc stats*(table: CipherTable): string =
  var s = "═══ mltt 密码表统计 ═══\n"
  s &= "条目总数: " & $table.total & "\n"
  s &= "类别数: " & $table.category_index.len & "\n"
  s &= "阴(容器): " & $(if table.polarity_index.hasKey(yin): table.polarity_index[yin].len else: 0) & "\n"
  s &= "阳(界线): " & $(if table.polarity_index.hasKey(yang): table.polarity_index[yang].len else: 0) & "\n"
  s &= "平衡: " & $(if table.polarity_index.hasKey(balanced): table.polarity_index[balanced].len else: 0) & "\n"
  return s
