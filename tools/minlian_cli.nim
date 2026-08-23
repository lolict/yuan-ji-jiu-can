# ============================================================
# 民联浏览器 · CLI 演示
# 统计文档聚合 → 态势化聚类 → 连接包裹树
# ============================================================

import minlian/browser/types
import minlian/browser/distance
import minlian/browser/cluster
import minlian/browser/wraptree
import std/random
import std/strutils

# 生成测试文档
proc make_test_docs*(n: int): seq[Document] =
  result = new_seq[Document](n)
  randomize(42)

  # 创建几个"主题簇"，每个簇内的文档内容相似
  let themes = [
    ("操作系统", 1),
    ("容器编码", 1),
    ("蚕蛹理论", 1),
    ("比特优化", 1),
    ("音乐合成", 1)
  ]

  for i in 0..<n:
    let theme = i mod themes.len
    let base_hash = uint64(theme + 1) * 0x1111111111111111'u64

    # 同主题内的文档: 基础哈希 + 小噪声 (少量位不同)
    let noise = uint64(rand(15))  # 只变低 4 位

    result[i] = Document(
      id: i,
      title: themes[theme][0] & "#" & $i,
      content_hash: base_hash xor noise,
      polarity: themes[theme][1],
      weight: 0.5 + rand(0.5),
      cluster_id: -1,
      wrapped: false
    )

proc demo() =
  echo "═══ 民联浏览器 ═══\n"

  # 1. 创建测试文档集
  let doc_count = 20
  var docs = make_test_docs(doc_count)
  echo "文档数: " & $docs.len

  # 显示前 5 个文档
  echo "\n─── 文档样本 ───"
  for i in 0..<min(5, docs.len):
    let d = docs[i]
    var pol_desc = "阳"
    if d.polarity < 0: pol_desc = "阴"
    elif d.polarity == 0: pol_desc = "中"
    echo "  #" & $d.id & " [" & d.title & "] 极性:" & pol_desc & " 权重:" & formatFloat(d.weight, ffDecimal, 2)
  echo ""

  # 2. 聚类
  echo "─── 态势化聚类 ───"
  let k = auto_k(docs.len)
  echo "自动选 K = " & $k
  var clusters = clusterize(docs, k)
  echo cluster_report(clusters)

  # 3. 构建包裹树
  echo "\n─── 构建连接包裹树 ───"
  var tree = build_wrap_tree(clusters, docs)
  echo render_tree(tree, clusters)
  echo wrap_stats(tree)

  # 4. 距离示例
  echo "\n─── 距离度量 ───"
  if docs.len >= 2:
    let d0 = docs[0].content_hash
    let d1 = docs[1].content_hash
    let sim = similarity(d0, d1)
    echo "  文档#0 ↔ 文档#1 相似度: " & formatFloat(sim, ffDecimal, 3)
    echo "  汉明距离: " & $hamming_distance(d0, d1) & " bits"

    # 找两个最相似的文档
    var best_i = 0
    var best_j = 1
    var best_sim = -1.0
    for i in 0..<docs.len:
      for j in (i+1)..<docs.len:
        let s = similarity(docs[i].content_hash, docs[j].content_hash)
        if s > best_sim:
          best_sim = s
          best_i = i
          best_j = j
    echo "  最相似对: #" & $best_i & " ↔ #" & $best_j & " = " & formatFloat(best_sim, ffDecimal, 3)

  # 5. 内存统计
  echo "\n─── 内存占用 ───"
  let doc_size = sizeof(Document)
  let total_mem = docs.len * doc_size
  echo "  每个 Document: " & $doc_size & " bytes"
  echo "  " & $docs.len & " 个文档: " & $total_mem & " bytes"
  echo "  聚类结果: " & $clusters.len & " 个簇"
  echo "  树节点:   " & $tree.len & " 个"

when is_main_module:
  demo()
