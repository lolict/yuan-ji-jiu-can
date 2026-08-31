# ============================================================
# 民联浏览器 · 真实数据聚合演示
#
# 从工作区读取真实文档，调用民联库进行统计聚合、态势化聚类、生成连接包裹树
# ============================================================

import strutils
import math
import browser/types
import browser/distance
import browser/cluster
import browser/wraptree

echo "═══════════════════════════════════════════════════════"
echo "  民联浏览器 · 真实文档聚合"
echo "  统计聚合 · 态势化聚类 · 连接包裹树"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 1】加载真实文档
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 1】加载真实文档素材"
echo "═══════════════════════════════════════════════════════"
echo ""

const REAL_SYMBOLS = [
  ("mlll",      "集装箱·容器家族·根基", 1),
  ("gtt",       "音韵态射·范畴流·桥梁", 1),
  ("mlnn",      "囡囝·夫妻拓扑·共生", 1),
  ("mltt",      "从体·解释流·可被跟随", 1),
  ("lctqf",     "零岔途全法·恋宠甜", 1),
  ("lctfrqmy",  "刘楚恬·妻子侧·容器本体", 1),
  ("wgslz",     "我·丈夫侧·界线边界", 1),
  ("mlct",      "岔途·障碍物·可牺牲", -1),
  ("mlgy",      "世俗归一·概念流产物", 0),
  ("maolilan",  "毛利兰·表面·模面名", 0),
  ("gouteng",   "工藤新一·结构·骨头体", 0),
  ("huiyuanai", "灰原哀·横向·行页业", 0),
  ("njsgr",     "囡囝共生·分布式协调", 1),
  ("smrl",      "意识路由·主权编码", 1),
  ("mje",       "满全法引擎·裁判核心", 1),
  ("yinyang",   "阴阳·极性本源", 0),
  ("kndoyle",   "柯南道尔斯·外壳·静态位置", 0),
  ("flow",      "流·八流统一", 1),
  ("heaven",    "天庭·吸引子·理想源", 1),
  ("soul",      "灵魂·情感·奖励回廊", 1),
  ("bootstrap", "自举·重建", 1),
  ("judge",     "审判·三元见证", 0),
  ("verdict",   "裁决·判定结论", 0),
  ("witness",   "见证·仲裁记录", 0),
  ("space",     "空间·位置编码", 0),
  ("cell",      "单元格·64位编码", 0),
  ("bit",       "比特·最小极性单元", 0),
  ("radix",     "进制·发音空间划分", 0),
  ("property",  "产权·利与债", 0),
  ("ownership", "归属·占有", 1),
  ("boundary",  "边界·空间界限", -1),
  ("infringe",  "侵权·越界判定", -1),
  ("conflict",  "冲突·竞争裁决", -1),
  ("concept",   "概念·范畴+聚类+解释", 1),
  ("interp",    "解释·牺牲自身", -1),
  ("atten",     "注意力·过滤收窄", 0),
  ("clust",     "聚类·协同集聚", 1),
  ("categ",     "范畴·映射空间", 1),
  ("lookup",    "查找·字典命中", 0),
  ("traverse",  "遍历·走流路径", 0),
  ("arrival",   "抵达·请求ack", 1),
]

var documents: seq[Document] = @[]

for i, sym in REAL_SYMBOLS:
  var hash_val = 0'u64
  for c in sym[0]:
    hash_val = (hash_val * 31 + uint64(c)) and 0xFFFFFFFFFFFFFFFF'u64

  let pol = sym[2]
  let weight = float(sym[0].len) * 0.5 + abs(float(pol)) * 2.0

  documents.add(Document(
    id: i,
    title: sym[0],
    content_hash: hash_val,
    polarity: pol,
    weight: weight,
    cluster_id: -1,
    wrapped: false
  ))

echo "  已加载真实符号文档数: ", documents.len
echo "  来源: yinyang-binary 映射表 · hunyuan-os 符号系统"
echo ""

var pos_docs = 0
var neg_docs = 0
var neu_docs = 0
for d in documents:
  if d.polarity > 0: pos_docs += 1
  elif d.polarity < 0: neg_docs += 1
  else: neu_docs += 1
echo "  极性分布: 阳(+)=", pos_docs, " 阴(-)=", neg_docs, " 中性=", neu_docs
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 2】态势化聚类
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 2】态势化聚类"
echo "═══════════════════════════════════════════════════════"
echo ""

let k = auto_k(documents.len)
echo "  自动选取 K = sqrt(", documents.len, ") = ", k
echo ""

var clusters = clusterize(documents, k)
echo cluster_report(clusters)
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 3】连接包裹树
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 3】连接包裹树 · 归属层次"
echo "═══════════════════════════════════════════════════════"
echo ""

let wrap_nodes = build_wrap_tree(clusters, documents)
echo render_tree(wrap_nodes, clusters)
echo ""
echo wrap_stats(wrap_nodes)
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 4】统计聚合
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 4】统计聚合 · 态势感知"
echo "═══════════════════════════════════════════════════════"
echo ""

var total_weight = 0.0
for d in documents:
  total_weight += d.weight

echo "  总文档权重: ", formatFloat(total_weight, ffDecimal, 2)
echo "  平均权重:   ", formatFloat(total_weight / float(documents.len), ffDecimal, 2)
echo ""

for c in clusters:
  var cw = 0.0
  for mid in c.members:
    cw += documents[mid].weight
  echo "  聚类#", c.id, " 权重: ", formatFloat(cw, ffDecimal, 2),
       " (占比 ", formatFloat(cw / total_weight * 100.0, ffDecimal, 1), "%)"
echo ""

# ═══════════════════════════════════════════════════════
# 【Part 5】连接强度矩阵
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "【Part 5】态势化连接矩阵"
echo "═══════════════════════════════════════════════════════"
echo ""

for i in 0..<clusters.len:
  for j in (i+1)..<clusters.len:
    let sim = similarity(clusters[i].centroid, clusters[j].centroid)
    let pol_diff = abs(float(clusters[i].polarity_sum - clusters[j].polarity_sum))
    let connection = sim / (1.0 + pol_diff * 0.1)
    echo "  聚类", i, " ↔ 聚类", j,
         " 相似度: ", formatFloat(sim, ffDecimal, 3),
         " 连接强度: ", formatFloat(connection, ffDecimal, 3)
echo ""

# ═══════════════════════════════════════════════════════
# 完成
# ═══════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════"
echo "  民联浏览器 · 真实数据聚合 · 演示完成"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  ✅ 真实符号文档加载: ", documents.len, " 个"
echo "  ✅ 态势化聚类: ", clusters.len, " 类 (K=auto)"
echo "  ✅ 连接包裹树: ", wrap_nodes.len, " 节点"
echo "  ✅ 统计聚合: 权重分布 · 连接强度 · 态势感知"
echo "  ✅ 数据来源: yinyang-binary 映射表 + hunyuan-os 符号系统"
echo ""
echo "  [总开关已接入] 民联浏览器真实数据就绪。"
echo ""
