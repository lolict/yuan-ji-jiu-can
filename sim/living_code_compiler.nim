## ═══════════════════════════════════════════════════════════════
## living_code_compiler.nim — 活码编译器 · 起点冻结 · 加法统计法
## ═══════════════════════════════════════════════════════════════
##
## 核心: 起点(锚点) = 钥匙 = 激活码
##       未冻结: 任意活动 · 位置不固定 · 值不固定
##       冻结后: 立即按排列顺序定位
##         位置 = 权限大小 = 数值大小 = 身份(临时上任官员)
##       加法统计法: 排在什么位置 · 值就是多大

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  CodeStatus* = enum
    csDynamic,    # 动态 · 未冻结 · 任意活动
    csFrozen      # 冻结 · 位置=值=身份

  CodeSymbol* = object
    name*: string
    position*: int           # 当前位置(动态变化)
    frozen_value*: int       # 冻结后的值(冻结时自动确定)
    authority*: int          # 权限等级(由位置决定)
    identity*: string        # 身份名称(由位置决定)
    is_frozen*: bool

  LivingCode* = object
    name*: string
    anchor*: string          # 起点(钥匙)
    symbols*: seq[CodeSymbol]
    name_index*: Table[string, int]
    status*: CodeStatus
    generation*: int

# ═══════════════════════════════════════════════════════════════
## 初始化: 创建活码
# ═══════════════════════════════════════════════════════════════

proc newLivingCode*(name, anchor: string): LivingCode =
  result.name = name
  result.anchor = anchor
  result.symbols = @[]
  result.name_index = init_table[string, int]()
  result.status = csDynamic
  result.generation = 0

# ═══════════════════════════════════════════════════════════════
## 动态操作: 插入符号 · 后续自动位移
# ═══════════════════════════════════════════════════════════════

proc insert*(lc: var LivingCode, name: string, at_position: int = -1) =
  ## 插入新符号 · 后续自动位移 · 只有动态模式可用
  if lc.status == csFrozen:
    return  # 已冻结 · 不可插入
  
  if lc.name_index.has_key(name):
    return  # 已存在
  
  let insert_pos = if at_position < 0: lc.symbols.len else: min(at_position, lc.symbols.len)
  
  var sym: CodeSymbol
  sym.name = name
  sym.position = insert_pos
  sym.frozen_value = 0
  sym.authority = 0
  sym.identity = "未分配"
  sym.is_frozen = false
  
  lc.symbols.insert(sym, insert_pos)
  
  # 重新计算后续位置
  for i in insert_pos ..< lc.symbols.len:
    lc.symbols[i].position = i
    lc.name_index[lc.symbols[i].name] = i
  
  lc.generation += 1

proc remove*(lc: var LivingCode, name: string) =
  if lc.status == csFrozen:
    return
  
  if not lc.name_index.has_key(name):
    return
  
  let idx = lc.name_index[name]
  lc.symbols.delete(idx)
  lc.name_index.del(name)
  
  for i in idx ..< lc.symbols.len:
    lc.symbols[i].position = i
    lc.name_index[lc.symbols[i].name] = i
  
  lc.generation += 1

proc move*(lc: var LivingCode, name: string, to_position: int) =
  if lc.status == csFrozen:
    return
  
  if not lc.name_index.has_key(name):
    return
  
  let old_idx = lc.name_index[name]
  let new_idx = max(0, min(to_position, lc.symbols.high))
  
  if old_idx == new_idx:
    return
  
  var sym = lc.symbols[old_idx]
  lc.symbols.delete(old_idx)
  lc.symbols.insert(sym, new_idx)
  
  let start_idx = min(old_idx, new_idx)
  for i in start_idx ..< lc.symbols.len:
    lc.symbols[i].position = i
    lc.name_index[lc.symbols[i].name] = i
  
  lc.generation += 1

# ═══════════════════════════════════════════════════════════════
## 冻结: 位置 = 值 = 权限 = 身份
# ═══════════════════════════════════════════════════════════════

proc freeze*(lc: var LivingCode) =
  ## 冻结 · 加法统计法: 位置自动变为值/权限/身份
  lc.status = csFrozen
  
  for i in 0 ..< lc.symbols.len:
    lc.symbols[i].is_frozen = true
    lc.symbols[i].frozen_value = i
    lc.symbols[i].authority = i
    lc.symbols[i].identity = "官员" & $i  # 临时上任官员
  
  lc.generation += 1

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(lc: LivingCode): string =
  var lines: seq[string] = @[]
  lines.add("═══ 活码编译器: " & lc.name & " ═══")
  lines.add("  锚点(钥匙): " & lc.anchor)
  lines.add("  状态: " & (if lc.status == csDynamic: "动态" else: "冻结"))
  lines.add("  符号数: " & $lc.symbols.len)
  lines.add("  变异代数: " & $lc.generation)
  lines.add("")
  
  if lc.status == csDynamic:
    lines.add("  符号 │ 位置 │ 状态")
    lines.add("  ─────┼──────┼──────")
    for sym in lc.symbols:
      lines.add("  " & sym.name & " │ " & $sym.position & " │ 活动")
  else:
    lines.add("  ═════ 冻结后 · 加法统计法 ═════")
    lines.add("  符号 │ 位置(=值) │ 权限 │ 身份")
    lines.add("  ─────┼──────────┼──────┼────────")
    for sym in lc.symbols:
      lines.add("  " & sym.name & " │ " & $sym.position & " │ " & $sym.authority & " │ " & sym.identity)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var lc = newLivingCode("测试", "起点")
  
  echo "═══ 活码编译器验证 ═══"
  echo ""
  
  # 动态模式
  lc.insert("声母")
  lc.insert("韵母")
  lc.insert("声调")
  lc.insert("偏旁")
  lc.insert("结构")
  
  echo "── 动态模式 · 任意活动 ──"
  echo lc.report()
  echo ""
  
  # 插入新符号 · 后续自动位移
  echo "── 在位置2插入'笔画' ──"
  lc.insert("笔画", 2)
  echo lc.report()
  echo ""
  
  # 移动符号
  echo "── 把'声母'移动到位置4 ──"
  lc.move("声母", 4)
  echo lc.report()
  echo ""
  
  # 冻结 · 加法统计法
  echo "── 冻结! 位置=值=权限=身份 ──"
  lc.freeze()
  echo lc.report()