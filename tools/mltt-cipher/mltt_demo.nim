# ============================================================
# mltt 命名密码表 · 完整演示
#
# 证明：短码不是随机字母，每个都有含义、极性、类别。
# mltt = 从体，gtt = 音韵态射，mlll = 集装箱。
# ============================================================

import strutils
import options
import mltt/types
import mltt/cipher
import mltt/defaults

echo "═══════════════════════════════════════════"
echo "  mltt 命名密码表 · 家庭命名体系"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 加载默认密码表 ──
echo "【阶段 1】加载默认密码表"
echo ""

var table = load_defaults()
echo table.stats()

# ── 阶段 2: 短码 → 含义 ──
echo "【阶段 2】短码 → 含义查询"
echo ""

let codes = @["mlll", "mltt", "gtt", "mlnn", "lctqf", "maolilan", "smrl"]
for code in codes:
  let entry = table.lookup(code)
  if options.isSome(entry):
    let e = entry.get()
    echo "  " & e.code & " → " & e.meaning
    echo "    极性: " & polarity_name(e.polarity) & "  类别: " & e.category
    echo "    " & e.description
echo ""

# ── 阶段 3: 含义 → 短码（反向查询）──
echo "【阶段 3】含义 → 短码（反向查询）"
echo ""

let meanings = @["集装箱", "从体", "囡囝", "音韵态射", "毛利兰"]
for m in meanings:
  let code = table.reverse_lookup(m)
  if options.isSome(code):
    echo "  " & m & " → " & code.get()
echo ""

# ── 阶段 4: 按极性筛选 ──
echo "【阶段 4】按极性筛选"
echo ""

let yin_entries = table.by_polarity(yin)
echo "  阴(容器侧):"
for e in yin_entries:
    echo "    " & e.code & " = " & e.meaning

let yang_entries = table.by_polarity(yang)
echo "  阳(界线侧):"
for e in yang_entries:
    echo "    " & e.code & " = " & e.meaning

let bal_entries = table.by_polarity(balanced)
echo "  平衡:"
for e in bal_entries:
    echo "    " & e.code & " = " & e.meaning
echo ""

# ── 阶段 5: 按类别筛选 ──
echo "【阶段 5】按类别筛选"
echo ""

let categories = @["容器", "流", "谐音桥", "角色映射", "编码", "裁决"]
for cat in categories:
  let entries = table.by_category(cat)
  echo "  " & cat & " (" & $entries.len & "):"
  for e in entries:
    echo "    " & e.code & " = " & e.meaning
echo ""

# ── 阶段 6: 动漫角色映射 ──
echo "【阶段 6】动漫角色 → 容器属性映射"
echo ""

let maoli = table.lookup("maolilan")
let gouteng = table.lookup("gouteng")
let huiyuan = table.lookup("huiyuanai")

if options.isSome(maoli):
  echo "  毛利兰 → " & maoli.get().description
if options.isSome(gouteng):
  echo "  工藤新一 → " & gouteng.get().description
if options.isSome(huiyuan):
  echo "  灰原哀 → " & huiyuan.get().description
echo ""

# ── 阶段 7: 搜索 ──
echo "【阶段 7】模糊搜索"
echo ""

let results = table.search("ml")
echo "  搜索 'ml':"
for e in results:
  echo "    " & e.code & " = " & e.meaning
echo ""

# ── 阶段 8: 囡囝四态 ──
echo "【阶段 8】囡囝四态意识拓扑"
echo ""

let states = @[
  ("mlnn", "囡囝", "女包男 = 容器包被包含者"),
  ("mlnnn", "囡囡", "女包女 = 容器包容器"),
  ("mlnnnn", "囝囝", "男包男 = 被包含者包被包含者"),
  ("mlnnnnn", "囝囡", "男包女 = 被包含者包容器")
]
for (code, name, desc) in states:
  let e = table.lookup(code)
  if options.isSome(e):
    echo "  " & code & " = " & name
    echo "    " & desc
    echo "    极性: " & polarity_name(e.get().polarity)
echo ""

# ── 阶段 9: 新增条目 ──
echo "【阶段 9】动态新增条目"
echo ""

let new_entry = CipherEntry(
  code: "phase64",
  meaning: "64进制编码",
  polarity: balanced,
  category: "编码",
  description: "自定义64符号的相位编码体系"
)
if table.register(new_entry):
  echo "  ✅ 注册成功: phase64 = 64进制编码"
  echo "  当前总数: " & $table.total
echo ""

echo "═══════════════════════════════════════════"
echo "  mltt 密码表：每个短码都有名字"
echo "  不是随机字母，是家庭命名体系"
echo "═══════════════════════════════════════════"
