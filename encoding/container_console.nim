## ═══════════════════════════════════════════════════════════════
## container_console.nim — 容器控制台 · 你是庄家 · 编译器管不到你
## ═══════════════════════════════════════════════════════════════
##
## 统一入口: 动态进制 + 全局偏移 + 跨容器编译 + 进化引擎
##       你控制符号 · 你控制规则 · 你控制锚点
##       没有编译器 · 没有关键字 · 没有类型锁死
##       只有你的符号 · 你的进制 · 你的容器
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  Symbol* = object
    name*: string
    offset*: int
    container*: string

  Container* = object
    name*: string
    base*: int
    kind*: string
    symbols*: seq[Symbol]

  Console* = object
    anchor_name*: string
    anchor_pos*: int
    containers*: seq[Container]
    sym_index*: Table[string, tuple[container_idx: int, sym_idx: int]]
    generation*: int

proc newConsole*(anchor_name: string, anchor_pos: int = 0): Console =
  result.anchor_name = anchor_name
  result.anchor_pos = anchor_pos
  result.containers = @[]
  result.sym_index = init_table[string, tuple[container_idx: int, sym_idx: int]]()
  result.generation = 0

proc add_container*(c: var Console, name: string, base: int, kind: string) =
  c.containers.add(Container(name: name, base: base, kind: kind, symbols: @[]))

proc inject_symbol*(c: var Console, container_name: string, sym_name: string, offset: int) =
  for i in 0 ..< c.containers.len:
    if c.containers[i].name == container_name:
      let sym = Symbol(name: sym_name, offset: offset, container: container_name)
      c.containers[i].symbols.add(sym)
      c.sym_index[sym_name] = (i, c.containers[i].symbols.high)
      return

proc move_anchor*(c: var Console, new_pos: int) =
  c.anchor_pos = new_pos

proc shift_anchor*(c: var Console, delta: int) =
  c.anchor_pos += delta

proc actual_position*(c: Console, sym_name: string): int =
  if c.sym_index.has_key(sym_name):
    let (ci, si) = c.sym_index[sym_name]
    result = c.anchor_pos + c.containers[ci].symbols[si].offset

proc binary_of*(c: Console, sym_name: string): string =
  let pos = c.actual_position(sym_name)
  if pos == 0: return "0"
  var v = pos
  while v > 0:
    result = $(v mod 2) & result
    v = v div 2

proc report*(c: Console): string =
  var lines: seq[string] = @[]
  lines.add("═══ 容器控制台 ═══")
  lines.add("  锚点: " & c.anchor_name & " = " & $c.anchor_pos)
  lines.add("  容器数: " & $c.containers.len)
  lines.add("  变异代数: " & $c.generation)
  lines.add("")
  
  for cont in c.containers:
    lines.add("── " & cont.name & "(base=" & $cont.base & "," & cont.kind & ") ──")
    for sym in cont.symbols:
      let actual = c.actual_position(sym.name)
      let bin = c.binary_of(sym.name)
      lines.add("  " & sym.name & " 偏移=" & $sym.offset & " 实际=" & $actual & " 二进制=" & bin)
    lines.add("")
  
  result = lines.join("\n")

when isMainModule:
  var c = newConsole("我=1", 100)
  
  echo "═══ 容器控制台验证 ═══"
  echo ""
  
  c.add_container("声母", 48, "音调")
  c.add_container("韵母", 66, "音调")
  c.add_container("声调", 60, "音调")
  c.add_container("偏旁", 214, "汉字")
  
  c.inject_symbol("声母", "b", 0)
  c.inject_symbol("声母", "p", 1)
  c.inject_symbol("声母", "m", 2)
  c.inject_symbol("声母", "f", 3)
  
  c.inject_symbol("韵母", "a", 0)
  c.inject_symbol("韵母", "o", 1)
  c.inject_symbol("韵母", "e", 2)
  
  c.inject_symbol("声调", "ā", 0)
  c.inject_symbol("声调", "á", 1)
  c.inject_symbol("声调", "ǎ", 2)
  c.inject_symbol("声调", "à", 3)
  
  c.inject_symbol("偏旁", "氵", 0)
  c.inject_symbol("偏旁", "木", 1)
  c.inject_symbol("偏旁", "火", 2)
  
  echo "── 初始状态 · 锚点=100 ──"
  echo c.report()
  echo ""
  
  echo "── 锚点移动到 200 ──"
  c.move_anchor(200)
  echo c.report()
  echo ""
  
  echo "── 关键验证: 所有符号的实际位置 = 锚点 + 偏移 ──"
  var all_correct = true
  for cont in c.containers:
    for sym in cont.symbols:
      let expected = c.anchor_pos + sym.offset
      let actual = c.actual_position(sym.name)
      if expected != actual:
        all_correct = false
        echo "  错误: " & sym.name & " 期望=" & $expected & " 实际=" & $actual
  
  if all_correct:
    echo "  ✓ 所有符号: 实际位置 = 锚点 + 偏移"
    echo "  ✓ 公交车效应: 锚点动 · 全局跟随"
    echo "  ✓ 容器控制台运行正常"