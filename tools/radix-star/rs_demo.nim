# ============================================================
# 部首星图 · 完整演示
# 从真实 JSON 字典数据加载 → 星图检索 → 共振分析
# ============================================================

import os
import strutils
import options
import radixstar/loader
import radixstar/starmap

echo "═══════════════════════════════════════════"
echo "  部首星图 · 真实字典数据运行样例"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 加载真实数据 ──
echo "【阶段 1】加载真实字典数据"
echo ""

let filepath = if paramCount() > 0: paramStr(1) else: ""

var entries: seq[CharEntry]
var stats: LoadStats

if filepath.len > 0:
  (entries, stats) = load_dictionary(filepath)
else:
  (entries, stats) = load_default()

echo "  文件条目: " & $stats.total_entries
echo "  成功加载: " & $stats.loaded
echo "  跳过异常: " & $stats.skipped
echo "  部首覆盖: " & $stats.radical_coverage & " 种"
echo ""

if entries.len == 0:
  echo "没有数据可处理，退出。"
  quit(1)

# ── 阶段 2: 构建星图 ──
echo "【阶段 2】构建星图"
echo ""

let sm = build_star_map(entries)
echo star_map_stats(sm)
echo ""

# ── 阶段 3: 星图定位 ──
echo "【阶段 3】星图定位演示"
echo ""

let test_chars = @["想", "湖", "好", "松", "他", "问", "桃", "唱"]

for ch in test_chars:
  let pos = sm.star_position(ch)
  if pos.isSome():
    echo "  " & position_desc(pos.get())
  else:
    echo "  『" & ch & "』不在星图中"
echo ""

# ── 阶段 4: 按部首检索 ──
echo "【阶段 4】按部首检索"
echo ""

let test_radicals = @[9, 28, 35, 71]
for idx in test_radicals:
  let chars = sm.lookup_by_radical(idx)
  var line = "  " & radical_name(idx) & "(#" & $idx & "): "
  for i, e in chars:
    if i > 0: line &= "、"
    line &= e.character
  echo line & " （" & $chars.len & " 字）"
echo ""

# ── 阶段 5: 精确搜索 ──
echo "【阶段 5】精确搜索（部首 + 余笔画）"
echo ""

# 搜索木部余 3 画的字
let found = sm.search(71, some(3))
var line = "  木部 余3画: "
for i, e in found:
  if i > 0: line &= "、"
  line &= e.character & "(" & e.pinyin & ")"
echo line
echo ""

# ── 阶段 6: 部首共振 ──
echo "【阶段 6】部首共振分析"
echo ""

echo resonance_report(sm)
echo ""

# ── 阶段 7: 详细条目展示 ──
echo "【阶段 7】汉字详情（随机抽取 5 个）"
echo ""

let sample_count = min(5, entries.len)
for i in 0..<sample_count:
  let e = entries[i * (entries.len div sample_count)]
  let pos = sm.star_position(e.character)
  if pos.isSome():
    let p = pos.get()
    echo "  " & e.character & "  " & e.pinyin & "  " & e.meaning
    echo "    → " & position_desc(p)
echo ""

echo "═══════════════════════════════════════════"
echo "  星图构建完成"
echo "  " & $entries.len & " 颗行星归入 " & $sm.radical_count & " 个恒星系"
echo "═══════════════════════════════════════════"
