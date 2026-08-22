# ============================================================
# 满全法 · 高维层级关系系统
# ============================================================
# 低维层级：数米粒，一粒一粒数。
# 高维层级：看袋子。
#
# 袋子包袋子：
#   小袋 → 中袋 → 大袋 → 麻袋 → 仓 → 库 → 天地
#
# 一袋子里装几袋小袋，
# 一中袋里装几袋小袋，
# 一大袋里装几中袋，
# 一麻袋里装几大袋，
# 一仓里装几麻袋。
#
# 高维认知：
#   不是数米粒有几个，
#   是数这仓里有多少袋，
#   这袋里套着几袋，
#   这一共有几层袋子。
#
# 关系：
#   父袋 — 我装谁
#   子袋 — 谁装我
#   兄袋 — 和谁同在一袋里
#   邻袋 — 挨着谁
#
# 不数米粒，
# 只数袋子。
# ============================================================

import tables
import math
import strutils
import ./rongqi

type
  # --- 层级单位（不叫袋子，叫"层"）---
  CengJiLevel* = enum
    clGrain,       # 米粒级别（这是低频认知，不使用）
    clXiaoDai,     # 小袋
    clZhongDai,    # 中袋
    clDaDai,       # 大袋
    clMaDai,       # 麻袋
    clCang,        # 仓
    clKu,          # 库
    clTianDi,      # 天地

  # --- 层级节点（一袋子在这层级体系中的位置）---
  CengJiNode* = object
    bag*: RongQi                 # 这袋子本身
    level*: CengJiLevel          # 哪一层
    parent_id*: string           # 被谁装着（父袋）
    child_ids*: seq[string]      # 装着哪些小袋（子袋）
    sibling_ids*: seq[string]    # 和谁同在一袋里（兄弟袋）
    depth*: int                  # 在整棵袋树里深几层
    path*: seq[string]           # 从根节点到这袋的路径

  # --- 袋子树（层级结构）---
  CengJiTree* = object
    name*: string
    nodes*: Table[string, CengJiNode]
    root_ids*: seq[string]       # 根节点（没有被任何袋子装的顶层袋）
    max_depth*: int

  # --- 层级关系查询结果 ---
  CengJiQuery* = object
    target_id*: string
    ancestors*: seq[string]      # 祖先链
    descendants*: seq[string]    # 后代
    neighbors*: seq[string]      # 邻居（同在一袋里的袋子）
    level_path*: seq[string]     # 从底到顶经过哪些层


# ============================================================
# 构造
# ============================================================
func level_name*(l: CengJiLevel): string =
  case l
  of clGrain:  return "米粒"
  of clXiaoDai: return "小袋"
  of clZhongDai: return "中袋"
  of clDaDai: return "大袋"
  of clMaDai: return "麻袋"
  of clCang:   return "仓"
  of clKu:     return "库"
  of clTianDi: return "天地"

func new_cengji_tree*(name: string): CengJiTree =
  result.name = name
  result.nodes = initTable[string, CengJiNode]()
  result.root_ids = @[]
  result.max_depth = 0


# ============================================================
# 放进层级
# ============================================================
func add_to_tree*(tree: var CengJiTree; bag: RongQi; level: CengJiLevel; parent_id: string = ""): string =
  var node = CengJiNode(
    bag: bag,
    level: level,
    parent_id: parent_id,
    child_ids: @[],
    sibling_ids: @[],
    depth: 0,
    path: @[]
  )
  tree.nodes[bag.id] = node

  # 更新父节点
  if parent_id.len > 0 and parent_id in tree.nodes:
    tree.nodes[parent_id].child_ids.add(bag.id)
    # 更新兄弟关系
    let siblings = tree.nodes[parent_id].child_ids
    for sid in siblings:
      if sid != bag.id:
        if sid notin tree.nodes[bag.id].sibling_ids:
          tree.nodes[bag.id].sibling_ids.add(sid)
        if bag.id notin tree.nodes[sid].sibling_ids:
          tree.nodes[sid].sibling_ids.add(bag.id)

  result = bag.id


# ============================================================
# 计算深度（从底到顶遍历）
# ============================================================
func compute_depths*(tree: var CengJiTree) =
  # 找所有根节点（没有父袋的）
  tree.root_ids = @[]
  for id, node in tree.nodes:
    if node.parent_id.len == 0:
      tree.root_ids.add(id)
      tree.nodes[id].depth = 0
      # 从根开始递归计算
      var stack = @[id]
      while stack.len > 0:
        let cur = stack.pop
        for child_id in tree.nodes[cur].child_ids:
          tree.nodes[child_id].depth = tree.nodes[cur].depth + 1
          # 计算路径
          tree.nodes[child_id].path = tree.nodes[cur].path & @[cur]
          stack.add(child_id)
          if tree.nodes[child_id].depth > tree.max_depth:
            tree.max_depth = tree.nodes[child_id].depth


# ============================================================
# 查询：这袋子从哪来，到哪去，挨着谁
# ============================================================
func query*(tree: CengJiTree; bag_id: string): CengJiQuery =
  var ancestors: seq[string] = @[]
  var descendants: seq[string] = @[]
  var neighbors: seq[string] = @[]
  var level_path: seq[string] = @[]

  if bag_id notin tree.nodes:
    return CengJiQuery(target_id: bag_id)

  # 祖先链（从底到顶）
  var cur = bag_id
  while tree.nodes[cur].parent_id.len > 0:
    ancestors.add(tree.nodes[cur].parent_id)
    cur = tree.nodes[cur].parent_id

  # 后代（所有子孙）
  var stack = @[bag_id]
  while stack.len > 0:
    let c = stack.pop
    for child_id in tree.nodes[c].child_ids:
      descendants.add(child_id)
      stack.add(child_id)

  # 邻居
  neighbors = tree.nodes[bag_id].sibling_ids

  # 经过的层
  cur = bag_id
  level_path.add(tree.nodes[cur].level.level_name)
  while tree.nodes[cur].parent_id.len > 0:
    cur = tree.nodes[cur].parent_id
    level_path.add(tree.nodes[cur].level.level_name)

  return CengJiQuery(
    target_id: bag_id,
    ancestors: ancestors,
    descendants: descendants,
    neighbors: neighbors,
    level_path: level_path
  )


# ============================================================
# 统计整棵树（不数米粒）
# ============================================================
func count_bags*(tree: CengJiTree; level: CengJiLevel): int =
  ## 数这一层有几袋（不数米粒）
  for id, node in tree.nodes:
    if node.level == level:
      result += 1

func total_bags*(tree: CengJiTree): int =
  tree.nodes.len

func value_at_level*(tree: CengJiTree; level: CengJiLevel): float =
  ## 这一层袋子总共值多少钱
  for id, node in tree.nodes:
    if node.level == level:
      result += node.bag.value_coin

func avg_value*(tree: CengJiTree): float =
  if tree.nodes.len == 0: return 0.0
  var total = 0.0
  for id, node in tree.nodes:
    total += node.bag.value_coin
  return total / tree.nodes.len.float


# ============================================================
# 层级报告
# ============================================================
func hierarchy_report*(tree: CengJiTree): string =
  var s = "层级报告: " & tree.name & "\n"
  s &= "总袋数: " & $tree.total_bags & "\n"
  s &= "最大深度: " & $tree.max_depth & "\n"
  s &= "各层袋数: "
  for level in [clXiaoDai, clZhongDai, clDaDai, clMaDai, clCang, clKu, clTianDi]:
    let n = tree.count_bags(level)
    if n > 0:
      s &= level.level_name & "=" & $n & " "
  s &= "\n"
  return s


# ============================================================
# 测试
# ============================================================
when is_main_module:
  echo "=== 高维层级测试 ==="

  var tree = new_cengji_tree("张家的袋阵")

  # --- 创建层级结构 ---
  # 天地（最顶层，包含所有库）
  var tianDi = new_rongqi("td1", "天地", "造化")
  tianDi.value_coin = 10000.0

  # 库（天地之下）
  var ku1 = new_rongqi("ku1", "张家东库", "张家")
  ku1.value_coin = 5000.0

  # 仓（库之下）
  var cang1 = new_rongqi("cg1", "东库甲仓", "张家")
  cang1.value_coin = 2000.0
  var cang2 = new_rongqi("cg2", "东库乙仓", "张家")
  cang2.value_coin = 1500.0

  # 麻袋（仓之下）
  var md1 = new_rongqi("md1", "甲仓一号麻袋", "张家")
  md1.value_coin = 500.0
  var md2 = new_rongqi("md2", "甲仓二号麻袋", "张家")
  md2.value_coin = 300.0

  # 大袋（麻袋之下）
  var dd1 = new_rongqi("dd1", "麻1内大袋甲", "张家")
  dd1.value_coin = 150.0
  var dd2 = new_rongqi("dd2", "麻1内大袋乙", "张家")
  dd2.value_coin = 100.0

  # 中袋（大袋之下）
  var zd1 = new_rongqi("zd1", "大袋内中袋A", "张家")
  zd1.value_coin = 50.0

  # 小袋（中袋之下）
  var xd1 = new_rongqi("xd1", "中袋内小袋1", "张家")
  xd1.value_coin = 20.0
  var xd2 = new_rongqi("xd2", "中袋内小袋2", "张家")
  xd2.value_coin = 15.0

  # --- 装配 ---
  discard tree.add_to_tree(tianDi, clTianDi)
  discard tree.add_to_tree(ku1, clKu, parent_id="td1")
  discard tree.add_to_tree(cang1, clCang, parent_id="ku1")
  discard tree.add_to_tree(cang2, clCang, parent_id="ku1")
  discard tree.add_to_tree(md1, clMaDai, parent_id="cg1")
  discard tree.add_to_tree(md2, clMaDai, parent_id="cg1")
  discard tree.add_to_tree(dd1, clDaDai, parent_id="md1")
  discard tree.add_to_tree(dd2, clDaDai, parent_id="md1")
  discard tree.add_to_tree(zd1, clZhongDai, parent_id="dd1")
  discard tree.add_to_tree(xd1, clXiaoDai, parent_id="zd1")
  discard tree.add_to_tree(xd2, clXiaoDai, parent_id="zd1")

  tree.compute_depths()

  echo hierarchy_report(tree)

  # 查一袋子的层级关系
  echo "【查 xd1 小袋的关系】"
  let q = tree.query("xd1")
  echo "  祖先链: ", q.ancestors.join(" → ")
  echo "  邻居: ", q.neighbors.join(", ")
  echo "  层级路径(底→顶): ", q.level_path.join(" → ")

  echo ""
  echo "【查 cg1 甲仓的关系】"
  let q2 = tree.query("cg1")
  echo "  后代数: ", q2.descendants.len
  echo "  层级路径: ", q2.level_path.join(" → ")
