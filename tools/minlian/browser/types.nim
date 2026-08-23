# ============================================================
# 民联浏览器 · 类型定义
# 统计文档聚合 · 态势化聚类 · 连接包裹树
# ============================================================

# 文档类型
type
  DocId* = int

  Document* = object
    id*: DocId
    title*: string
    content_hash*: uint64     # 内容的 64 位指纹
    polarity*: int            # 极性: -1=阴, 0=中性, 1=阳
    weight*: float            # 权重 (重要性)
    cluster_id*: int          # 所属聚类 (-1=未分类)
    wrapped*: bool            # 是否已被包裹进树

  # 距离度量
  Distance* = float

  # 聚类
  Cluster* = object
    id*: int
    centroid*: uint64         # 质心 (代表性指纹)
    members*: seq[DocId]      # 成员文档 ID
    polarity_sum*: int        # 极性总和 (决定聚类偏向)

  # 包裹树节点
  WrapNode* = object
    cluster_id*: int
    children*: seq[int]       # 子聚类 ID
    depth*: int               # 树深度
    label*: string            # 包裹标签

# 比较函数 (用于排序)
func cmp_doc_by_weight*(a, b: Document): int =
  if b.weight > a.weight: 1
  elif b.weight < a.weight: -1
  else: 0

func cmp_cluster_by_size*(a, b: Cluster): int =
  if b.members.len > a.members.len: 1
  elif b.members.len < a.members.len: -1
  else: 0
