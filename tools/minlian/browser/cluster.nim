# ============================================================
# 民联浏览器 · 态势化聚类引擎
# 将相似文档聚合成簇
# ============================================================

import types
import distance
import std/algorithm
import std/math

# 简单 K-Means 聚类
proc clusterize*(docs: var seq[Document], k: int): seq[Cluster] =
  let n = docs.len
  if n == 0 or k <= 0: return

  # 初始化: 取前 k 个文档作为初始质心
  var centroids: seq[uint64] = @[]
  for i in 0..<k:
    if i < n:
      centroids.add(docs[i].content_hash)
    else:
      centroids.add(docs[n-1].content_hash)

  var changed = true
  var iterations = 0
  const MAX_ITER = 50

  while changed and iterations < MAX_ITER:
    changed = false
    inc iterations

    # 分配阶段: 每个文档归入最近的质心
    for d in docs.mitems:
      var best_c = 0
      var best_dist = 1e10
      for ci, c in centroids:
        let dist = polarity_distance(d.content_hash, c, d.polarity, 0)
        if dist < best_dist:
          best_dist = dist
          best_c = ci
      if d.cluster_id != best_c:
        d.cluster_id = best_c
        changed = true

    # 更新阶段: 重新计算质心
    for ci in 0..<k:
      var sum: uint64 = 0
      var cnt = 0
      var pol_sum = 0
      for d in docs:
        if d.cluster_id == ci:
          sum = sum xor d.content_hash  # XOR 混合指纹
          pol_sum += d.polarity
          cnt += 1
      if cnt > 0:
        centroids[ci] = sum

  # 构建聚类结果
  result = new_seq[Cluster](k)
  for ci in 0..<k:
    result[ci] = Cluster(
      id: ci,
      centroid: centroids[ci],
      members: @[],
      polarity_sum: 0
    )

  for d in docs:
    if d.cluster_id >= 0 and d.cluster_id < k:
      result[d.cluster_id].members.add(d.id)
      result[d.cluster_id].polarity_sum += d.polarity

  # 按大小降序排列
  result.sort(cmp_cluster_by_size)

# 自动选 K: 用平方根启发式
func auto_k*(n: int): int =
  if n <= 0: return 0
  if n <= 4: return 1
  result = int(sqrt(float(n)))
  if result < 1: result = 1
  if result > 10: result = 10

# 聚类报告
func cluster_report*(clusters: seq[Cluster]): string =
  var s = "态势化聚类结果\n"
  s &= "────────────────────────\n"
  for c in clusters:
    var pol_desc = "中性"
    if c.polarity_sum > 0: pol_desc = "偏阳(" & $c.polarity_sum & ")"
    elif c.polarity_sum < 0: pol_desc = "偏阴(" & $c.polarity_sum & ")"
    s &= "  聚类#" & $c.id & " 成员:" & $c.members.len & " " & pol_desc & "\n"
    if c.members.len > 0:
      s &= "    成员: "
      for i, m in c.members:
        if i > 0: s &= ", "
        s &= "#" & $m
      s &= "\n"
  s &= "────────────────────────\n"
  s &= "聚类总数: " & $clusters.len & "\n"
  result = s
