# 满全法编码容器系统
# 基于64进制unicode韵母编码，严格使用确认配对
# 替代旧版ASCII方案

import yunmu64

type
  # 根容器节点
  KGRoot* = object
    anchor*: string
    children*: seq[KGNode]
    symbol*: string

  # 容器节点
  KGNode* = object
    idx*: int
    symbol*: string
    payload*: string
    boundary*: int
    children*: seq[KGNode]

# ============================================================
# 基本操作
# ============================================================
func new_root*(anchor: string): KGRoot =
  result.anchor = anchor
  result.children = @[]
  result.symbol = "#" & anchor

func new_node*(idx: int, symbol: string, payload: string): KGNode =
  result.idx = idx
  result.symbol = symbol
  result.payload = payload
  result.boundary = (idx + 1) * 10
  result.children = @[]

func add_child*(parent: var KGNode, child: KGNode) =
  parent.children.add(child)
  parent.boundary += child.boundary div 10

func find_child*(parent: KGNode, idx: int): int =
  for i, c in parent.children:
    if c.idx == idx: return i
  return -1

func remove_child*(parent: var KGNode, idx: int): bool =
  let pos = find_child(parent, idx)
  if pos >= 0:
    parent.boundary -= parent.children[pos].boundary div 10
    parent.children.delete(pos)
    return true
  return false

# ============================================================
# 关系判断
# ============================================================
func compare*(a: KGNode, b: KGNode): string =
  if a.boundary > b.boundary: return ">"
  elif a.boundary < b.boundary: return "<"
  else: return "="

func same_level*(a: KGNode, b: KGNode): bool =
  return a.boundary == b.boundary

func is_same_node*(a: KGNode, b: KGNode): bool =
  return a.idx == b.idx

# ============================================================
# 完整容器工厂（自动装载所有韵母）
# ============================================================
func build_full_container*(anchor: string): KGRoot =
  result = new_root(anchor)
  var r = new_rongqi64("full")

  var yunmu_list = @[
    "a", "o", "e", "i", "u",
    "ai", "ei", "ao", "ou", "er",
    "ia", "ie", "ua", "uo", "iu", "ue", "ui",
    "an", "en", "in", "ang", "eng", "ing", "ong",
    "iao", "uai", "ian", "uan", "uang", "üan", "iong",
    "vn", "iai", "eo", "oi", "ueng"
  ]

  for ym in yunmu_list:
    r.add_yunmu(ym)

  for node in r.nodes:
    let kg = new_node(
      primary_code(node),
      if node.symbols.len > 0: node.symbols[0] else: "?",
      node.yunmu
    )
    result.children.add(kg)

# ============================================================
# 调试输出
# ============================================================
proc print_node*(node: KGNode, level: int) =
  var indent = ""
  for i in 1..level: indent &= "  "
  echo indent, "├─ #", node.idx, " ", node.symbol,
       " @", node.payload, " [边界=", node.boundary, "]"
  for child in node.children:
    print_node(child, level + 1)

proc print_root*(root: KGRoot) =
  echo "=== 根容器 [", root.anchor, "] ==="
  echo "符号: ", root.symbol
  echo "子容器数: ", root.children.len
  for c in root.children:
    print_node(c, 1)

# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 满全法容器系统 ==="
  echo ""

  var root = build_full_container("00")
  print_root(root)

  echo ""
  echo "=== 测试关系 ==="
  let n_a = new_node(0, "a", "a")
  let n_o = new_node(2, "ȍ", "o")
  let n_er = new_node(18, "ɛ", "er")

  echo "a vs o: ", compare(n_a, n_o)
  echo "o vs er: ", compare(n_er, n_o)

  echo ""
  echo "=== 容器完成 ==="
