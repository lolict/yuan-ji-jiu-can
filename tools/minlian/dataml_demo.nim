# ============================================================
# 民联浏览器 · Mode C · 真实数据输入器演示
#
# 输出: JSON 格式聚类结果 (供前端消费)
# ============================================================

import std/os
import std/json
import std/strutils
import dataml/data_input

echo "═══════════════════════════════════════════════════════"
echo "  民联浏览器 · Mode C · 真实数据输入器"
echo "═══════════════════════════════════════════════════════"
echo ""

# 获取项目根目录
let root_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os"

let extensions = @[".md", ".nim", ".c", ".h", ".fqf"]

echo "  扫描目录: ", root_path
echo "  文件类型: ", extensions.join(", ")
echo ""

let docs = scan_project(root_path, extensions)

if docs.len == 0:
  echo "   Warning: No files found"
else:
  let result = build_result(docs)

  echo "  +----------------------------------------------+"
  echo "  | Mode C - Real Data Input Report              |"
  echo "  +----------------------------------------------+"
  echo "  | Total docs:    ", result.total_docs
  echo "  | Positive:      ", result.polar_pos
  echo "  | Negative:      ", result.polar_neg
  echo "  | Neutral:       ", result.polar_neu
  echo "  | Total weight:  ", formatFloat(result.total_weight, ffDecimal, 2)
  echo "  +----------------------------------------------+"
  echo ""

  # show first 10 docs
  echo "  First 10 documents:"
  var i = 0
  for d in docs:
    if i >= 10:
      break
    echo "    [", d.polarity, "] ", d.path, " (", d.size, " bytes)"
    i = i + 1
  echo ""

  # output JSON (first 3 docs only)
  echo "  -- JSON output (first 3 docs) --"
  var short_docs: seq[DocEntry] = @[]
  for j in 0 ..< min(3, docs.len):
    short_docs.add(docs[j])
  let short_result = build_result(short_docs)
  let json_out = short_result.to_json()
  echo "  ", pretty(json_out)
  echo ""

  echo "  [OK] Mode C demo completed"
  echo ""

echo "========================================================="
echo "  JSON ready for frontend consumption"
echo "  Files: ", docs.len
echo "========================================================="
