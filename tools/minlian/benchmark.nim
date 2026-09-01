# ============================================================
# 民联浏览器 · 大数据量聚类包裹树性能基准
# TASKS.md P3 — 任意规模 JSON 数据源 + 性能验证
# ============================================================

import browser/types
import browser/distance
import browser/cluster
import browser/wraptree
import std/json
import std/times
import std/random
import std/math
import std/strutils

# ---------- 指纹 + 极性（内置轻量版） ----------

const
  POL_WORDS_POS: array[6, string] = ["照", "昭", "召", "升", "盈", "清"]
  POL_WORDS_NEG: array[6, string] = ["亏", "虚", "里", "降", "浊", "寒"]

const FNV_OFFSET = cast[uint64](1469598103934665603)
const FNV_PRIME  = cast[uint64](1099511628211)

proc fnv1a_hash*(s: string): uint64 =
  var h = FNV_OFFSET
  for ch in s:
    h = h xor cast[uint64](ord(ch))
    h = (h * FNV_PRIME) and 0xFFFFFFFFFFFFFFFF'u64
  result = h

proc polarity_detected*(s: string): int =
  var pos = 0
  var neg = 0
  for w in POL_WORDS_POS:
    if s.contains(w): pos += 1
  for w in POL_WORDS_NEG:
    if s.contains(w): neg += 1
  if pos > neg: result = 1
  elif neg > pos: result = -1
  else: result = 0

# ---------- 合成 JSON 数据源（规模可控） ----------

proc make_payload(seed: int): string =
  var rd = initRand(seed)
  let pool_word_count = POL_WORDS_POS.len
  var body = "doc-" & $seed & " "
  for i in 0 ..< 12:
    let pick_is_pos = (rd.rand(0 .. 1)) == 0
    let w = if pick_is_pos:
      POL_WORDS_POS[rd.rand(0 ..< pool_word_count)]
    else:
      POL_WORDS_NEG[rd.rand(0 ..< pool_word_count)]
    body.add w
    if i mod 4 == 3: body.add "，"
  result = body & "。"

proc load_docs_from_json(n: int): seq[Document] =
  result = new_seq[Document](n)
  for i in 0 ..< n:
    let p = make_payload(i + 1)
    result[i] = Document(
      id: i,
      title: "src-" & $i,
      content_hash: fnv1a_hash(p),
      polarity: polarity_detected(p),
      weight: float(p.len),
      cluster_id: -1,
      wrapped: false
    )

# ---------- 计时器 ----------

type
  BenchResult* = object
    n_docs*: int
    k_used*: int
    t_cluster_ms*: float
    t_wrap_ms*: float
    total_ms*: float
    clusters_formed*: int
    tree_max_depth*: int
    tree_leaf_count*: int
    throughput_docs_per_sec*: float

proc run_one(n: int): BenchResult =
  let docs = load_docs_from_json(n)
  let k0 = auto_k(n)
  let t0 = get_time()
  var dc = docs
  let cs = clusterize(dc, k0)
  let t1 = get_time()
  let nodes = build_wrap_tree(cs, dc)
  let t2 = get_time()
  let tc = (t1 - t0).in_milliseconds.float
  let tw = (t2 - t1).in_milliseconds.float
  let tot = tc + tw
  result = BenchResult(
    n_docs: n, k_used: k0,
    t_cluster_ms: tc, t_wrap_ms: tw, total_ms: tot,
    clusters_formed: cs.len, tree_max_depth: 0, tree_leaf_count: 0,
    throughput_docs_per_sec: if tot > 0: n.float / (tot / 1000.0) else: 0.0
  )
  for node in nodes:
    if node.depth > result.tree_max_depth:
      result.tree_max_depth = node.depth
    if node.children.len == 0:
      result.tree_leaf_count += 1

# ---------- 入口 ----------

when is_main_module:
  echo "════════════════════════════════════════════════════════"
  echo "  民联浏览器 · 聚类包裹树性能基准 (P3 · TASKS.md)"
  echo "════════════════════════════════════════════════════════"

  # P5: 100K+ 规模验证 — 观察 O(n^1.5) 增长曲线
  let scales = [100, 500, 1000, 5000, 10000, 50000, 100000]
  var jarr = newJArray()
  for n in scales:
    let r = run_one(n)
    echo "n=", align($r.n_docs, 6), "  k=", align($r.k_used, 2),
        "  clusters=", align($r.clusters_formed, 3),
        "  t_cluster=", formatFloat(r.t_cluster_ms, ffDecimal, 2).align(9), "ms",
        "  t_wrap=", formatFloat(r.t_wrap_ms, ffDecimal, 2).align(8), "ms",
        "  total=", formatFloat(r.total_ms, ffDecimal, 2).align(10), "ms",
        "  throughput=", formatFloat(r.throughput_docs_per_sec, ffDecimal, 0).align(10), "doc/s"
    let o = newJObject()
    o["n_docs"] = newJInt(r.n_docs)
    o["k"] = newJInt(r.k_used)
    o["clusters"] = newJInt(r.clusters_formed)
    o["cluster_ms"] = newJFloat(r.t_cluster_ms)
    o["wrap_ms"] = newJFloat(r.t_wrap_ms)
    o["total_ms"] = newJFloat(r.total_ms)
    o["throughput"] = newJFloat(r.throughput_docs_per_sec)
    jarr.add(o)

  echo "\n════════════════════════════════════════════════════════"
  echo " JSON 基准报告（可接入前端可视化）"
  echo "════════════════════════════════════════════════════════"
  echo pretty(jarr, 2)
