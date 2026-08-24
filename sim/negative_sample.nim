## ═══════════════════════════════════════════════════════════════
## negative_sample.nim — 负样本容器 · 免疫记忆 · 回写协议
## ═══════════════════════════════════════════════════════════════
##
## 核心: 被否定的错误过程保存为负样本 · 作为免疫记忆
##       下次遇到相似错误 · 自动识别并拒绝
##       回写协议: 负样本可以"回写"到符号表 · 标记为"已感染"
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, sets, times]

type
  NegativeSample* = object
    id*: string
    symbol*: string
    error_type*: string
    description*: string
    timestamp*: string
    severity*: int
    is_active*: bool
    write_back_count*: int

  ImmuneMemory* = object
    samples*: seq[NegativeSample]
    index_by_symbol*: Table[string, seq[int]]
    index_by_type*: Table[string, seq[int]]
    total_blocked*: int
    generation*: int

proc newImmuneMemory*(): ImmuneMemory =
  result.samples = @[]
  result.index_by_symbol = init_table[string, seq[int]]()
  result.index_by_type = init_table[string, seq[int]]()
  result.total_blocked = 0
  result.generation = 0

proc record_negative*(im: var ImmuneMemory, symbol, error_type, description: string, severity: int = 5): int =
  var sample: NegativeSample
  sample.id = "NEG" & $(im.samples.len + 1)
  sample.symbol = symbol
  sample.error_type = error_type
  sample.description = description
  sample.timestamp = "2026-08-24"
  sample.severity = severity
  sample.is_active = true
  sample.write_back_count = 0
  
  im.samples.add(sample)
  let idx = im.samples.high
  
  if not im.index_by_symbol.has_key(symbol):
    im.index_by_symbol[symbol] = @[]
  im.index_by_symbol[symbol].add(idx)
  
  if not im.index_by_type.has_key(error_type):
    im.index_by_type[error_type] = @[]
  im.index_by_type[error_type].add(idx)
  
  im.generation += 1
  result = idx

proc check_immunity*(im: ImmuneMemory, symbol: string): tuple[is_blocked: bool, reasons: seq[string]] =
  if im.index_by_symbol.has_key(symbol):
    for idx in im.index_by_symbol[symbol]:
      if im.samples[idx].is_active:
        result.is_blocked = true
        result.reasons.add("[" & im.samples[idx].id & "] " & im.samples[idx].error_type & ": " & im.samples[idx].description)

proc write_back*(im: var ImmuneMemory, symbol: string): int =
  if im.index_by_symbol.has_key(symbol):
    for idx in im.index_by_symbol[symbol]:
      if im.samples[idx].is_active:
        im.samples[idx].write_back_count += 1
        im.total_blocked += 1
        result += 1
  im.generation += 1

proc clear_immunity*(im: var ImmuneMemory, symbol: string): int =
  if im.index_by_symbol.has_key(symbol):
    for idx in im.index_by_symbol[symbol]:
      if im.samples[idx].is_active:
        im.samples[idx].is_active = false
        result += 1
  im.generation += 1

proc report*(im: ImmuneMemory): string =
  var lines: seq[string] = @[]
  lines.add("═══ 负样本容器 · 免疫记忆 ═══")
  lines.add("  负样本总数: " & $im.samples.len)
  lines.add("  活跃样本: " & $im.samples.filter(proc(s: NegativeSample): bool = s.is_active).len)
  lines.add("  总拦截次数: " & $im.total_blocked)
  lines.add("  变异代数: " & $im.generation)
  lines.add("")
  
  for sample in im.samples:
    let status = if sample.is_active: "●" else: "○"
    lines.add("  " & status & " " & sample.id & " | " & sample.symbol &
      " | " & sample.error_type & " | 严重度=" & $sample.severity &
      " | 回写=" & $sample.write_back_count)
    lines.add("     " & sample.description)
  
  result = lines.join("\n")

when isMainModule:
  var im = newImmuneMemory()
  
  echo "═══ 负样本容器验证 ═══"
  echo ""
  
  # 记录负样本
  discard im.record_negative("NJSG", "编码冲突", "NJSG误解", 3)
  discard im.record_negative("HashMap", "fan-shi", "wrong words", 8)
  discard im.record_negative("class", "fan-shi", "wrong words", 9)
  discard im.record_negative("multiply", "op-error", "wrong op", 6)
  discard im.record_negative("for", "key-word", "wrong key", 7)
  
  echo "── 记录负样本后 ──"
  echo im.report()
  echo ""
  
  # 免疫检测
  echo "── 免疫检测 ──"
  let (blocked1, reasons1) = im.check_immunity("HashMap")
  echo "  HashMap: 拦截=" & $blocked1 & " 原因=" & $reasons1.len
  
  let (blocked2, reasons2) = im.check_immunity("者")
  echo "  者: 拦截=" & $blocked2
  
  let (blocked3, reasons3) = im.check_immunity("class")
  echo "  class: 拦截=" & $blocked3 & " 原因=" & $reasons3[0]
  echo ""
  
  # 回写协议
  echo "── 回写协议: HashMap被触发3次 ──"
  discard im.write_back("HashMap")
  discard im.write_back("HashMap")
  discard im.write_back("HashMap")
  echo "  HashMap回写次数: " & $im.samples[1].write_back_count
  echo "  总拦截次数: " & $im.total_blocked
  echo ""
  
  # 清除免疫
  echo "── 清除免疫: NJSG被平反 ──"
  discard im.clear_immunity("NJSG")
  let (blocked4, _) = im.check_immunity("NJSG")
  echo "  NJSG: 拦截=" & $blocked4 & "(应为false)"
  echo ""
  
  echo im.report()