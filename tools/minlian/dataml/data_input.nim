# ============================================================
# 民联浏览器 · Mode C · 真实数据输入器
#
# 修复 path 问题，使用绝对路径直接从项目根目录扫描，
# 输出 JSON 格式的聚类结果供前端消费。
# ============================================================

import std/os
import std/json
import std/strutils
import std/sequtils
import std/math

# ── FNV-1a 哈希 ──

const FNV_OFFSET = 0xcbf29ce484222325'u64
const FNV_PRIME  = 0x100000001b3'u64

func fnv1a(data: string): string =
  var h = FNV_OFFSET
  for c in data:
    h = h xor uint64(c)
    h = (h * FNV_PRIME) and 0xFFFFFFFFFFFFFFFF'u64
  result = toHex(h)

# ── 极性分析 ──

const POSITIVE_KEYS = [
  "完成", "成功", "激活", "推进", "增长", "实现", "确认", "运行",
  "✅", "通过", "启动", "解决", "发现", "证明", "新建", "创建"
]

const NEGATIVE_KEYS = [
  "失败", "错误", "警告", "问题", "冲突", "禁止", "否定", "溢出",
  "不", "无", "未", "作废", "拒绝", "移除", "崩溃"
]

func estimate_polarity(content: string): int =
  var score = 0
  for kw in POSITIVE_KEYS:
    if content.contains(kw): inc score
  for kw in NEGATIVE_KEYS:
    if content.contains(kw): dec score
  if score > 2: result = 1
  elif score < -2: result = -1
  else: result = 0

# ── 权重估算 ──

func estimate_weight(content: string): float =
  result = float(content.len) / 100.0

# ── JSON 文档 ──

type
  DocEntry* = object
    path*: string
    hash*: string
    polarity*: int
    weight*: float
    size*: int

  ClusterResult* = object
    total_docs*: int
    polar_pos*: int
    polar_neg*: int
    polar_neu*: int
    total_weight*: float
    docs*: seq[DocEntry]
    generated_at*: string

# ── 扫描文件 ──

proc scan_project*(root: string; extensions: seq[string]): seq[DocEntry] =
  var count = 0
  for path in walkDirRec(root):
    let ext = path.splitFile.ext
    if ext in extensions:
      try:
        let content = readFile(path)
        if content.len > 0:
          let pol = estimate_polarity(content)
          let w = estimate_weight(content)
          let rel_path = path.replace(root & "/", "")
          result.add(DocEntry(
            path: rel_path,
            hash: fnv1a(content),
            polarity: pol,
            weight: w,
            size: content.len
          ))
          count = count + 1
          if count >= 200:
            break
      except:
        discard

# ── 输出 JSON ──

proc to_json*(cr: ClusterResult): JsonNode =
  result = newJObject()
  result["total_docs"] = newJInt(cr.total_docs)
  result["polar_pos"] = newJInt(cr.polar_pos)
  result["polar_neg"] = newJInt(cr.polar_neg)
  result["polar_neu"] = newJInt(cr.polar_neu)
  result["total_weight"] = newJFloat(cr.total_weight)

  var docs_array = newJArray()
  for d in cr.docs:
    var obj = newJObject()
    obj["path"] = newJString(d.path)
    obj["hash"] = newJString(d.hash)
    obj["polarity"] = newJInt(d.polarity)
    obj["weight"] = newJFloat(d.weight)
    obj["size"] = newJInt(d.size)
    docs_array.add(obj)
  result["docs"] = docs_array

# ── 构建结果 ──

proc build_result*(docs: seq[DocEntry]): ClusterResult =
  var pos_c, neg_c, neu_c = 0
  var tw = 0.0
  for d in docs:
    if d.polarity > 0: inc pos_c
    elif d.polarity < 0: inc neg_c
    else: inc neu_c
    tw += d.weight
  ClusterResult(
    total_docs: docs.len,
    polar_pos: pos_c,
    polar_neg: neg_c,
    polar_neu: neu_c,
    total_weight: tw,
    docs: docs,
    generated_at: "now"
  )
