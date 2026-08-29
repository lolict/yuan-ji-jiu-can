## ═══════════════════════════════════════════════════════════════
## 全局记忆系统 · 永不遗忘 · 跨对话持久 · 子代理共享
## ═══════════════════════════════════════════════════════════════
## 核心: 所有重要决策/规则/事件写入此文件
## 每个子代理启动时 · 先加载此文件 · 获取历史记忆
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, times

# ═══════════════════════════════════════════════════════════════
# 记忆条目
# ═══════════════════════════════════════════════════════════════
type
  MemoryType* = enum
    mtRule,           # 规则(必须遵守)
    mtDecision,       # 决策(已做出的选择)
    mtError,          # 错误(被批评的)
    mtCorrection,     # 纠正(主人纠正的)
    mtSymbol,         # 符号(已定义的)
    mtBoundary,       # 边界(因果边界)
    mtContract,       # 契约(公约数关系)

  MemoryEntry* = object
    id*: string               # 唯一ID
    mtype*: MemoryType        # 记忆类型
    content*: string          # 内容
    source*: string           # 来源(主人/元老/子代理)
    timestamp*: string        # 时间戳
    importance*: int          # 重要度(1-10 · 10=最高)
    permanent*: bool          # 是否永久(永久=不可覆盖)

# ═══════════════════════════════════════════════════════════════
# 全局记忆
# ═══════════════════════════════════════════════════════════════
type
  GlobalMemory* = object
    entries*: Table[string, MemoryEntry]
    rules*: seq[string]           # 规则列表(快速查找)
    errors*: seq[string]          # 错误列表(避免重犯)
    symbols*: Table[string, string]  # 符号表
    version*: int                 # 记忆版本

const MEMORY_FILE* = "cognition_os/global_memory.json"

# ═══════════════════════════════════════════════════════════════
# 初始化
# ═══════════════════════════════════════════════════════════════
func new_global_memory*(): GlobalMemory =
  result.entries = initTable[string, MemoryEntry]()
  result.rules = @[]
  result.errors = @[]
  result.symbols = initTable[string, string]()
  result.version = 0

# ═══════════════════════════════════════════════════════════════
# 添加记忆
# ═══════════════════════════════════════════════════════════════
proc add*(mem: var GlobalMemory; mtype: MemoryType; content: string;
          source = "主人"; importance = 5; permanent = false): string =
  let id = "mem_" & $mem.version
  mem.entries[id] = MemoryEntry(
    id: id,
    mtype: mtype,
    content: content,
    source: source,
    timestamp: $now(),
    importance: importance,
    permanent: permanent,
  )
  # 分类存储
  case mtype:
  of mtRule: mem.rules.add(content)
  of mtError: mem.errors.add(content)
  of mtSymbol: mem.symbols[content] = source
  else: discard
  mem.version += 1
  return id

# ═══════════════════════════════════════════════════════════════
# 查询记忆
# ═══════════════════════════════════════════════════════════════
func get_rules*(mem: GlobalMemory): seq[string] =
  return mem.rules

func get_errors*(mem: GlobalMemory): seq[string] =
  return mem.errors

func has_rule*(mem: GlobalMemory; rule: string): bool =
  for r in mem.rules:
    if r.contains(rule): return true
  return false

func has_error*(mem: GlobalMemory; err: string): bool =
  for e in mem.errors:
    if e.contains(err): return true
  return false

# ═══════════════════════════════════════════════════════════════
# 保存到文件(持久化)
# ═══════════════════════════════════════════════════════════════
proc save*(mem: GlobalMemory) =
  var j = newJObject()
  j["version"] = newJInt(mem.version)
  
  var entries_j = newJArray()
  for k, v in mem.entries:
    var entry_j = newJObject()
    entry_j["id"] = newJString(v.id)
    entry_j["type"] = newJString($v.mtype)
    entry_j["content"] = newJString(v.content)
    entry_j["source"] = newJString(v.source)
    entry_j["timestamp"] = newJString(v.timestamp)
    entry_j["importance"] = newJInt(v.importance)
    entry_j["permanent"] = newJBool(v.permanent)
    entries_j.add(entry_j)
  j["entries"] = entries_j
  
  var rules_j = newJArray()
  for r in mem.rules: rules_j.add(newJString(r))
  j["rules"] = rules_j
  
  var errors_j = newJArray()
  for e in mem.errors: errors_j.add(newJString(e))
  j["errors"] = errors_j
  
  writeFile(MEMORY_FILE, j.pretty)

# ═══════════════════════════════════════════════════════════════
# 从文件加载
# ═══════════════════════════════════════════════════════════════
proc load*(mem: var GlobalMemory) =
  if not fileExists(MEMORY_FILE): return
  let content = readFile(MEMORY_FILE)
  let j = parseJson(content)
  mem.version = j["version"].getInt()
  
  for entry_j in j["entries"]:
    let id = entry_j["id"].getStr()
    mem.entries[id] = MemoryEntry(
      id: id,
      mtype: parseEnum[MemoryType](entry_j["type"].getStr()),
      content: entry_j["content"].getStr(),
      source: entry_j["source"].getStr(),
      timestamp: entry_j["timestamp"].getStr(),
      importance: entry_j["importance"].getInt(),
      permanent: entry_j["permanent"].getBool(),
    )
  
  for r in j["rules"]: mem.rules.add(r.getStr())
  for e in j["errors"]: mem.errors.add(e.getStr())

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var mem = new_global_memory()
  
  # 加载已有记忆(如果有)
  mem.load()
  
  # 添加核心规则(永久 · 不可覆盖)
  discard mem.add(mtRule, "符号只能从symbol_index.nim取 · 禁止自创", "主人", 10, true)
  discard mem.add(mtRule, "基元冻结: 声母24+韵母33+声调4", "主人", 10, true)
  discard mem.add(mtRule, "白名单外字符一律死刑", "主人", 10, true)
  discard mem.add(mtRule, "元老规则 > 子代理认知", "主人", 9, true)
  
  # 添加错误记录(避免重犯)
  discard mem.add(mtError, "使用ꬰ等AI发明的伪符号", "主人批评", 8, false)
  discard mem.add(mtError, "用系统编码(U+0080+)覆盖用户符号", "主人批评", 8, false)
  discard mem.add(mtError, "忘记历史符号 · 重复定义", "主人批评", 7, false)
  
  # 添加符号记忆
  discard mem.add(mtSymbol, "a", "主人", 10, true)
  discard mem.add(mtSymbol, "ai", "主人", 10, true)
  discard mem.add(mtSymbol, "ꬰ=禁止(AI发明)", "主人", 10, true)
  
  # 保存
  mem.save()
  
  echo "═══ 全局记忆系统 ═══"
  echo "版本: ", mem.version
  echo "条目数: ", mem.entries.len
  echo "规则数: ", mem.rules.len
  echo "错误记录: ", mem.errors.len
  echo ""
  
  echo "── 核心规则 ──"
  for i, r in mem.rules:
    echo "  #", i, " ", r
  
  echo ""
  echo "── 错误记录(避免重犯) ──"
  for i, e in mem.errors:
    echo "  #", i, " ", e
  
  echo ""
  echo "── 查询测试 ──"
  echo "  有'基元冻结'规则: ", mem.has_rule("基元冻结")
  echo "  有'ꬰ'错误记录: ", mem.has_error("ꬰ")
  echo "  有'xxx'错误记录: ", mem.has_error("xxx")
