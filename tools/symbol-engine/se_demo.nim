# ============================================================
# 符号引擎 · 完整演示
# 
# 核心证明：符号表是数据，不编译也能用。
# 系统的解释器不定义你的符号，你的加载器定义。
# ============================================================

import strutils
import os
import options
import symbol_engine/types
import symbol_engine/engine
import symbol_engine/loader

echo "═══════════════════════════════════════════"
echo "  符号引擎 · 动态加载演示"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 从 JSON 加载符号表 ──
echo "【阶段 1】运行时加载符号表（不编译）"
echo ""

let json_path = if paramCount() > 0: paramStr(1) else: ""

var st: SymbolTable
if json_path.len > 0:
  st = load_from_json(json_path)
  echo "  从 " & json_path & " 加载"
else:
  st = load_default()

echo "  符号总数: " & $st.total_symbols
echo "  含义总数: " & $st.meaning_count
echo "  类别总数: " & $st.category_count
echo ""

# ── 阶段 2: 解析符号（64进制编码 → 含义）──
echo "【阶段 2】符号解析（64进制编码 → 含义）"
echo ""

let test_codes = @["A1", "B1", "C1", "D3", "E1", "F1", "G2", "H2", "I1", "J1", "K1", "L1"]

for code in test_codes:
  let result = st.resolve(code)
  echo "  " & resolve_desc(result)
echo ""

# ── 阶段 3: 近义词验证 ──
echo "【阶段 3】近义词系统（多个符号 = 同一含义）"
echo ""

echo "  解析 A1:"
let r_a1 = st.resolve("A1")
if r_a1.found:
  echo "    " & r_a1.entry.get().meaning
  echo "    近义词: " & r_a1.alternatives.join(", ")

echo "  解析 SELF（A1的近义词）:"
let r_self = st.resolve("SELF")
if r_self.found:
  echo "    " & r_self.entry.get().meaning
  echo "    近义词: " & r_self.alternatives.join(", ")

echo "  解析 ANCHOR_YANG（也是A1的近义词）:"
let r_ay = st.resolve("ANCHOR_YANG")
if r_ay.found:
  echo "    " & r_ay.entry.get().meaning

echo ""
echo "  → 三个不同符号，解析到同一个含义：我"
echo "  → 这就是近义词：避免冲突的命名方法"
echo ""

# ── 阶段 4: 反向查找 ──
echo "【阶段 4】反向查找（含义 → 所有编码）"
echo ""

let codes_for_bag = st.lookup_meaning("袋子")
echo "  含义'袋子'对应的所有编码: " & codes_for_bag.join(", ")

let codes_for_mirror = st.lookup_meaning("照镜子")
echo "  含义'照镜子'对应的所有编码: " & codes_for_mirror.join(", ")
echo ""

# ── 阶段 5: 按类别检索 ──
echo "【阶段 5】按类别检索"
echo ""

let categories = @["根基锚点", "七阶编码", "运算", "核心法则"]
for cat in categories:
  let entries = st.lookup_category(cat)
  var line = "  " & cat & ": "
  for i, e in entries:
    if i > 0: line &= ", "
    line &= e.code & "=" & e.meaning
  echo line
echo ""

# ── 阶段 6: 动态添加新符号（不重新编译引擎）──
echo "【阶段 6】运行时动态添加新符号（不重新编译引擎）"
echo ""

let new_entry = make_entry("M1", "自定义新符号", "用户扩展")
  .with_synonyms(@["MY_SYMBOL", "NEW_ONE"])
  .with_desc("这是运行时添加的符号，引擎没有重新编译")

st.add(new_entry)

echo "  添加前符号总数: " & $(st.total_symbols - 1)
echo "  添加后符号总数: " & $st.total_symbols

let r_new = st.resolve("M1")
echo "  解析 M1: " & resolve_desc(r_new)

let r_syn = st.resolve("MY_SYMBOL")
echo "  解析 MY_SYMBOL（近义词）: " & resolve_desc(r_syn)
echo ""

# ── 阶段 7: 模糊搜索 ──
echo "【阶段 7】模糊搜索（按关键词）"
echo ""

let results = st.search("锚点")
echo "  搜索'锚点':"
for e in results:
  echo "    " & e.code & " → " & e.meaning & " (" & e.category & ")"
echo ""

# ── 阶段 8: 序列化保存 ──
echo "【阶段 8】保存符号表到 JSON（数据独立于引擎）"
echo ""

# 保存到文件
st.save_to_json("test_symbols.json")
echo "  已保存到 test_symbols.json"
echo "  文件大小: " & $getFileSize("test_symbols.json") & " bytes"
echo ""

# ── 统计报告 ──
echo table_stats(st)

echo "═══════════════════════════════════════════"
echo "  演示完成"
echo "  符号表 = 数据，不煮熟也能用"
echo "  近义词 = 避免冲突的命名方法"
echo "═══════════════════════════════════════════"
