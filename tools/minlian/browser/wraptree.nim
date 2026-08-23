# ============================================================
# 民联浏览器 · 连接包裹树
# 将聚类结果组织成树状包裹结构
# ============================================================

import types
import distance
import strutils

# 从聚类构建包裹树
proc build_wrap_tree*(clusters: seq[Cluster], docs: seq[Document]): seq[WrapNode] =
  let n = clusters.len
  if n == 0: return

  result = new_seq[WrapNode](n)

  # 初始化每个节点
  for i, c in clusters:
    result[i] = WrapNode(
      cluster_id: c.id,
      children: @[],
      depth: 0,
      label: ""
    )

  # 确定父子关系: 质心距离近的聚类互为父子
  for i in 0..<n:
    if i == 0:
      result[i].depth = 0
      result[i].label = "根"
      continue

    # 找最近的聚类作为父节点
    var best_parent = 0
    var best_sim = -1.0
    for j in 0..<i:
      let sim = similarity(clusters[i].centroid, clusters[j].centroid)
      if sim > best_sim:
        best_sim = sim
        best_parent = j

    result[best_parent].children.add(i)
    result[i].depth = result[best_parent].depth + 1

  # 生成标签
  for i in 0..<n:
    if result[i].depth == 0:
      result[i].label = "根节点"
    else:
      let c = clusters[i]
      var pol_tag = ""
      if c.polarity_sum > 0: pol_tag = "[阳]"
      elif c.polarity_sum < 0: pol_tag = "[阴]"
      else: pol_tag = "[中]"
      result[i].label = "第" & $result[i].depth & "层-" & pol_tag & $c.members.len & "件"

# 渲染树 (ASCII)
func render_tree*(nodes: seq[WrapNode], clusters: seq[Cluster]): string =
  if nodes.len == 0: return "(空树)"

  var s = "连接包裹树\n"
  s &= "────────────────────────\n"

  # 深度优先渲染
  proc draw(idx: int, prefix: string, result: var string) =
    let node = nodes[idx]
    let c = clusters[idx]
    var line = prefix
    if node.depth > 0:
      line &= "├─ "
    else:
      line &= "● "
    line &= node.label & " (质心:" & to_hex(c.centroid, 8) & ")\n"
    result &= line

    for i, child in node.children:
      let is_last = (i == node.children.high)
      var new_prefix = prefix
      if node.depth > 0:
        if is_last: new_prefix &= "   "
        else: new_prefix &= "│  "
      else:
        if is_last: new_prefix &= "   "
        else: new_prefix &= "│  "
      draw(child, new_prefix, result)

  draw(0, "", s)
  s &= "────────────────────────\n"
  result = s

# 包裹统计
func wrap_stats*(nodes: seq[WrapNode]): string =
  var max_depth = 0
  var total_children = 0
  var leaf_count = 0
  for node in nodes:
    if node.depth > max_depth: max_depth = node.depth
    total_children += node.children.len
    if node.children.len == 0: leaf_count += 1

  var s = "包裹树统计\n"
  s &= "────────────────────────\n"
  s &= "  节点总数: " & $nodes.len & "\n"
  s &= "  树深度:   " & $max_depth & "\n"
  s &= "  叶子数:   " & $leaf_count & "\n"
  s &= "  总分叉:   " & $total_children & "\n"
  result = s
