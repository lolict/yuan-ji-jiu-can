# ============================================================
# 民联浏览器 · 双模式演示
#
# Mode A: 硬编码符号表 (原始)
# Mode B: 真实文件加载器 (增强) — 真实内容哈希 + 极性分析 + 结构权重
# ============================================================

import strutils
import math
import std/os
import browser/types
import browser/distance
import browser/cluster
import browser/wraptree
import browser/loader

echo "═══════════════════════════════════════════════════════════"
echo "  民联浏览器 · 双 Mode 聚合演示"
echo "  Mode A 硬编码符号表 · Mode B 真实文件加载器"
echo "═══════════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════════
# Mode A: 硬编码符号表 (原始逻辑)
# ═══════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════"
echo "【Mode A】硬编码符号表 (原始逻辑)"
echo "═══════════════════════════════════════════════════════════"
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

var docs_a: seq[Document] = @[]

for i, sym in REAL_SYMBOLS:
  var hash_val = 0'u64
  for c in sym[0]:
    hash_val = (hash_val * 31 + uint64(c)) and 0xFFFFFFFFFFFFFFFF'u64
  
  let pol = sym[2]
  let weight = float(sym[0].len) * 0.5 + abs(float(pol)) * 2.0
  
  docs_a.add(Document(
    id: i,
    title: sym[0],
    content_hash: hash_val,
    polarity: pol,
    weight: weight,
    cluster_id: -1,
    wrapped: false
  ))

echo "  已加载符号文档数: ", docs_a.len
echo ""

var pos_a = 0; var neg_a = 0; var neu_a = 0
for d in docs_a:
  if d.polarity > 0: inc pos_a
  elif d.polarity < 0: inc neg_a
  else: inc neu_a
echo "  极性分布: 阳(+)=" & $pos_a & " 阴(-)=" & $neg_a & " 中性=" & $neu_a
echo ""

let k_a = auto_k(docs_a.len)
echo "  自动选取 K = sqrt(" & $docs_a.len & ") = " & $k_a
echo ""

var clusters_a = clusterize(docs_a, k_a)
echo cluster_report(clusters_a)
echo ""

# ═══════════════════════════════════════════════════════════
# Mode B: 真实文件加载器 (增强逻辑)
# ═══════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════"
echo "【Mode B】真实文件加载器 (增强逻辑)"
echo "═══════════════════════════════════════════════════════════"
echo ""

let root_path = parentDir(parentDir(getCurrentDir()))
var docs_b = load_real_docs(root_path)
echo load_report(docs_b)
echo ""

if docs_b.len > 0:
  let k_b = auto_k(docs_b.len)
  echo "  自动选取 K = sqrt(" & $docs_b.len & ") = " & $k_b
  echo ""
  
  var clusters_b = clusterize(docs_b, k_b)
  echo cluster_report(clusters_b)
  echo ""
  
  # ── Mode B 的包裹树 ──
  let wrap_b = build_wrap_tree(clusters_b, docs_b)
  echo render_tree(wrap_b, clusters_b)
  echo ""
  echo wrap_stats(wrap_b)
  echo ""
  
  # ── Mode B 的统计聚合 ──
  var total_weight_b = 0.0
  for d in docs_b: total_weight_b += d.weight
  
  echo "  总文档权重: " & formatFloat(total_weight_b, ffDecimal, 2)
  echo "  平均权重:   " & formatFloat(total_weight_b / float(docs_b.len), ffDecimal, 2)
  echo ""
  
  for c in clusters_b:
    var cw = 0.0
    for mid in c.members:
      if mid < docs_b.len:
        cw += docs_b[mid].weight
    if total_weight_b > 0:
      echo "  聚类#" & $c.id & " 权重: " & formatFloat(cw, ffDecimal, 2) &
           " (占比 " & formatFloat(cw / total_weight_b * 100.0, ffDecimal, 1) & "%)"
  echo ""

# ═══════════════════════════════════════════════════════════
# 完成
# ═══════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════"
echo "  民联浏览器 · 双 Mode 演示完成"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "  Mode A: 硬编码符号表 (" & $docs_a.len & " 个文档)"
echo "  Mode B: 真实文件加载 (" & $docs_b.len & " 个文档)"
echo ""
echo "  [总开关已接入] 民联浏览器双Mode就绪。"
echo ""
