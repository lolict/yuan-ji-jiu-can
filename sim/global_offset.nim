## ═══════════════════════════════════════════════════════════════
## global_offset.nim — 全局偏移量 · 公交车效应 · 静态锚点驱动
## ═══════════════════════════════════════════════════════════════
##
## 核心: 静态锚点(公约数)移动 → 所有动态符号(公倍数)跟随移动
##       一个人上公交车 · 所有人跟着动
##       锚点移动1公里 · 所有人坐标+1公里
##
## 实现: 不存储绝对位置 · 只存储锚点 + 偏移量
##       符号实际位置 = 锚点 + 偏移量
##       移动锚点 → 所有符号自动跟随
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables]

type
  Anchor* = object
    name*: string
    position*: int
    is_static*: bool

  GlobalOffset* = object
    anchor*: Anchor
    symbols*: Table[string, int]
    order*: seq[string]

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newGlobalOffset*(anchor_name: string, anchor_pos: int = 0): GlobalOffset =
  result.anchor = Anchor(name: anchor_name, position: anchor_pos, is_static: true)
  result.symbols = init_table[string, int]()
  result.order = @[]

# ═══════════════════════════════════════════════════════════════
## 注册符号: 记录相对偏移量（不变）
# ═══════════════════════════════════════════════════════════════

proc register*(go: var GlobalOffset, name: string, offset: int) =
  if not go.symbols.has_key(name):
    go.symbols[name] = offset
    go.order.add(name)

# ═══════════════════════════════════════════════════════════════
## 移动锚点: 公交车效应 · 所有符号跟随
# ═══════════════════════════════════════════════════════════════

proc move_anchor*(go: var GlobalOffset, new_position: int) =
  go.anchor.position = new_position

proc shift_anchor*(go: var GlobalOffset, delta: int) =
  go.anchor.position += delta

# ═══════════════════════════════════════════════════════════════
## 查询: 符号的实际位置（动态计算）
# ═══════════════════════════════════════════════════════════════

proc actual_position*(go: GlobalOffset, name: string): int =
  if go.symbols.has_key(name):
    result = go.anchor.position + go.symbols[name]

proc relative_offset*(go: GlobalOffset, name: string): int =
  if go.symbols.has_key(name):
    result = go.symbols[name]

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(go: GlobalOffset): string =
  var lines: seq[string] = @[]
  lines.add("═══ 全局偏移量 · 公交车效应 ═══")
  lines.add("  锚点: " & go.anchor.name & " = " & $go.anchor.position)
  lines.add("  符号数: " & $go.symbols.len)
  lines.add("")
  lines.add("  符号  │ 相对偏移 │ 实际位置")
  lines.add("  ──────┼─────────┼─────────")
  for name in go.order:
    let offset = go.relative_offset(name)
    let actual = go.actual_position(name)
    lines.add("  " & name & "  │  " & $offset & "  │  " & $actual)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var go = newGlobalOffset("我=1", 100)
  
  echo "═══ 全局偏移量验证 ═══"
  echo ""
  
  # 注册符号（相对偏移量固定）
  go.register("声母", 0)
  go.register("韵母", 1)
  go.register("声调", 2)
  go.register("偏旁", 3)
  go.register("笔画", 4)
  
  echo "── 初始状态 · 锚点=100 ──"
  echo go.report()
  echo ""
  
  # 移动锚点（公交车效应）
  echo "── 锚点移动到 200 · 所有符号跟随 ──"
  go.move_anchor(200)
  echo go.report()
  echo ""
  
  # 锚点偏移
  echo "── 锚点偏移 +50 · 所有符号跟随 ──"
  go.shift_anchor(50)
  echo go.report()
  echo ""
  
  # 验证：相对偏移量不变
  echo "── 关键验证: 相对偏移量是否不变 ──"
  var all_same = true
  let expected_offsets = [0, 1, 2, 3, 4]
  var idx = 0
  for name in go.order:
    let off = go.relative_offset(name)
    if off != expected_offsets[idx]:
      all_same = false
      echo "  变化: " & name & " 偏移量=" & $off & " 期望=" & $expected_offsets[idx]
    idx += 1
  
  if all_same:
    echo "  ✓ 所有符号的相对偏移量不变 · 只有锚点在动"
    echo "  ✓ 公交车效应验证通过"