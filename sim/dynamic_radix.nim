## ═══════════════════════════════════════════════════════════════
## dynamic_radix.nim — 动态进制 · 符号流动 · 永不编译
## ═══════════════════════════════════════════════════════════════
##
## 核心: 符号不是静态映射到二进制值
##       符号是流动的节点 · 二进制值 = 当前所在位置
##       插入/删除/移动 → 二进制值自动跟随
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables]

type
  SymbolNode* = object
    name*: string
    position*: int

  DynamicRadix* = object
    symbols*: seq[SymbolNode]
    name_index*: Table[string, int]
    base*: int
    generation*: int

# ═══════════════════════════════════════════════════════════════
## 辅助: 整数 → 二进制字符串
# ═══════════════════════════════════════════════════════════════

proc int_to_bin_len(n: int): int =
  if n <= 1: return 1
  var v = n
  while v > 0:
    result += 1
    v = v div 2

proc int_to_bin_str(n: int, min_bits: int = 8): string =
  if n == 0:
    result = "0".repeat(min_bits)
    return
  var v = n
  while v > 0:
    result = $(v mod 2) & result
    v = v div 2
  while result.len < min_bits:
    result = "0" & result

# ═══════════════════════════════════════════════════════════════
## 初始化: 空容器 · 零符号 · 等待注入
# ═══════════════════════════════════════════════════════════════

proc newDynamicRadix*(): DynamicRadix =
  result.symbols = @[]
  result.name_index = init_table[string, int]()
  result.base = 0
  result.generation = 0

# ═══════════════════════════════════════════════════════════════
## 核心操作1: 注入符号（像输液一样注入新血）
# ═══════════════════════════════════════════════════════════════

proc inject*(dr: var DynamicRadix, name: string, at_position: int = -1) =
  if dr.name_index.has_key(name):
    return
  let insert_pos = if at_position < 0: dr.symbols.len else: min(at_position, dr.symbols.len)
  var node: SymbolNode
  node.name = name
  node.position = insert_pos
  dr.symbols.insert(node, insert_pos)
  for i in insert_pos ..< dr.symbols.len:
    dr.symbols[i].position = i
    dr.name_index[dr.symbols[i].name] = i
  dr.base = dr.symbols.len
  dr.generation += 1

# ═══════════════════════════════════════════════════════════════
## 核心操作2: 移除符号（像抽血一样移除旧血）
# ═══════════════════════════════════════════════════════════════

proc remove*(dr: var DynamicRadix, name: string) =
  if not dr.name_index.has_key(name):
    return
  let idx = dr.name_index[name]
  dr.symbols.delete(idx)
  dr.name_index.del(name)
  for i in idx ..< dr.symbols.len:
    dr.symbols[i].position = i
    dr.name_index[dr.symbols[i].name] = i
  dr.base = dr.symbols.len
  dr.generation += 1

# ═══════════════════════════════════════════════════════════════
## 核心操作3: 移动符号（像血液在血管中流动）
# ═══════════════════════════════════════════════════════════════

proc move*(dr: var DynamicRadix, name: string, to_position: int) =
  if not dr.name_index.has_key(name):
    return
  let old_idx = dr.name_index[name]
  let new_idx = max(0, min(to_position, dr.symbols.high))
  if old_idx == new_idx:
    return
  var node = dr.symbols[old_idx]
  dr.symbols.delete(old_idx)
  dr.symbols.insert(node, new_idx)
  let start_idx = min(old_idx, new_idx)
  for i in start_idx ..< dr.symbols.len:
    dr.symbols[i].position = i
    dr.name_index[dr.symbols[i].name] = i
  dr.generation += 1

# ═══════════════════════════════════════════════════════════════
## 核心操作4: 交换符号
# ═══════════════════════════════════════════════════════════════

proc swap*(dr: var DynamicRadix, name_a: string, name_b: string) =
  if not (dr.name_index.has_key(name_a) and dr.name_index.has_key(name_b)):
    return
  let idx_a = dr.name_index[name_a]
  let idx_b = dr.name_index[name_b]
  var tmp = dr.symbols[idx_a]
  dr.symbols[idx_a] = dr.symbols[idx_b]
  dr.symbols[idx_b] = tmp
  dr.symbols[idx_a].position = idx_a
  dr.symbols[idx_b].position = idx_b
  dr.name_index[name_a] = idx_a
  dr.name_index[name_b] = idx_b
  dr.generation += 1

# ═══════════════════════════════════════════════════════════════
## 查询: 符号 → 当前二进制值（动态计算 · 不存储）
# ═══════════════════════════════════════════════════════════════

proc binary_of*(dr: DynamicRadix, name: string): string =
  if not dr.name_index.has_key(name):
    return "UNDEFINED"
  let pos = dr.name_index[name]
  let bits = if dr.base <= 1: 1 else: int_to_bin_len(dr.base - 1)
  result = int_to_bin_str(pos, bits)

proc value_of*(dr: DynamicRadix, name: string): int =
  if dr.name_index.has_key(name):
    result = dr.name_index[name]

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(dr: DynamicRadix): string =
  var lines: seq[string] = @[]
  lines.add("═══ 动态进制 · 血液流动状态 ═══")
  lines.add("  进制基数: " & $dr.base)
  lines.add("  变异代数: " & $dr.generation)
  lines.add("  符号数: " & $dr.symbols.len)
  lines.add("")
  for i, sym in dr.symbols:
    lines.add("  [" & $i & "] " & sym.name &
      "  二进制=" & dr.binary_of(sym.name) &
      "  值=" & $dr.value_of(sym.name))
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var dr = newDynamicRadix()
  
  echo "═══ 动态进制验证 ═══"
  echo ""
  
  # 注入64个符号
  let symbols = [
    "者","这","做","中","正","制","轴","周",
    "奏","骤","主","住","注","驻","著","柱",
    "助","术","朱","珠","株","诸","猪","竹",
    "逐","烛","瞩","嘱","主","煮","渚","属",
    "暑","署","薯","曙","出","初","除","储",
    "楚","处","触","畜","厨","雏","矗","搐",
    "怵","绌","黜","础","刍","雏","橱","躇",
    "川","传","船","穿","喘","串","钏","篡",
    "窜","摧","催","崔","脆","翠","淬","粹"
  ]
  
  for s in symbols:
    dr.inject(s)
  
  echo dr.report()
  echo ""
  
  # 动态插入新符号
  echo "── 在位置5插入新符号'新血' ──"
  dr.inject("新血", 5)
  echo dr.report()
  echo ""
  
  # 移动符号
  echo "── 把'者'移动到位置10 ──"
  dr.move("者", 10)
  echo "者的新位置: " & $dr.value_of("者") & "  二进制: " & dr.binary_of("者")
  echo ""
  
  # 移除符号
  echo "── 移除'竹' ──"
  dr.remove("竹")
  echo "竹还存在吗: " & $dr.name_index.has_key("竹")
  echo "当前符号数: " & $dr.symbols.len
  echo ""
  
  echo "── 关键验证: 所有符号的二进制值 = 其位置 ──"
  var all_match = true
  for i, sym in dr.symbols:
    if dr.value_of(sym.name) != i:
      all_match = false
      echo "  不匹配: " & sym.name & " 位置=" & $i & " 值=" & $dr.value_of(sym.name)
  
  if all_match:
    echo "  ✓ 所有符号的二进制值 = 当前位置 · 血液流动正常"