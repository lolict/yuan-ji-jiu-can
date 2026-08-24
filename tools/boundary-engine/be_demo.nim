# ============================================================
# 边界分析引擎 · 完整演示
#
# 演示：用加法拆分乘法边界做不到的事情
# 对比两个规则体系，判断容器是否允许存在
# ============================================================

import strutils
import options
import boundary_engine/types
import boundary_engine/engine

echo "═══════════════════════════════════════════"
echo "  边界分析引擎 · 加法 vs 乘法"
echo "═══════════════════════════════════════════"
echo ""

# 创建引擎
var be = new_boundary_engine()

# ══════════════════════════════════════════
# 定义乘法：23声母（预定义规则，有边界）
# ══════════════════════════════════════════
echo "【定义乘法边界】"
echo ""

var boundary_23 = Boundary(
  name: "23声母",
  capacity: 23,
  items: @["b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
           "j", "q", "x", "zh", "ch", "sh", "r", "z", "c", "s", "y", "w"],
  rules: @[
    "每个音节必须有且只有一个声母",
    "声母只能从23个中选",
    "超出23个的不被承认"
  ],
  rejects: @["ⴀ", "ⴁ", "ⴂ", "自定义符号"]
)

be.add_boundary(boundary_23)

echo "  边界名称: " & boundary_23.name
echo "  容量: " & $boundary_23.capacity
echo "  已填充: " & $boundary_23.items.len
echo "  拒绝项: " & boundary_23.rejects.join(", ")
echo ""

# ══════════════════════════════════════════
# 定义加法：我的分类思路（参差出来的思路）
# ══════════════════════════════════════════
echo "【定义加法分类】"
echo ""

# 加法分类1：按我的思路分类的声母（超出23个）
var addition_my_way = Addition(
  name: "我的声母分类",
  category: "自定义",
  items: @["b", "p", "m", "f", "d", "t", "n", "l",
           "g", "k", "h", "j", "q", "x",
           "zh", "ch", "sh", "r", "z", "c", "s",
           "ⴀ", "ⴁ", "ⴂ", "ⴃ", "ⴄ", "ⴅ", "ⴆ", "ⴇ"],
  logic: "按发音部位+自定义扩展",
  priority: 1
)

# 加法分类2：按优先级权重分类
var addition_priority = Addition(
  name: "优先级权重分类",
  category: "权重",
  items: @["核心声母", "扩展声母", "自定义声母", "边缘声母", "未定义声母"],
  logic: "按使用频率和重要性排序",
  priority: 2
)

# 加法分类3：按思维参差分类
var addition_thinking = Addition(
  name: "思维参差分类",
  category: "思维",
  items: @["传统认知", "自定义符号", "系统拒绝", "边界外生存", "新边界需求"],
  logic: "按思维差异分类",
  priority: 3
)

be.add_addition(addition_my_way)
be.add_addition(addition_priority)
be.add_addition(addition_thinking)

echo "  分类1: " & addition_my_way.name & " (" & $addition_my_way.items.len & " 项)"
echo "  分类2: " & addition_priority.name & " (" & $addition_priority.items.len & " 项)"
echo "  分类3: " & addition_thinking.name & " (" & $addition_thinking.items.len & " 项)"
echo ""

# ══════════════════════════════════════════
# 对比分析：加法 vs 乘法
# ══════════════════════════════════════════
echo "【对比分析：加法 vs 乘法】"
echo ""

let results1 = be.compare_addition("我的声母分类", "23声母")
echo gap_report(be, "我的声母分类", "23声母")
echo ""

# ══════════════════════════════════════════
# 找出乘法做不到的事情
# ══════════════════════════════════════════
echo "【乘法做不到的事情（边界外）】"
echo ""

let gaps = be.find_gaps()
echo be.gap_details(results1)
echo ""

# ══════════════════════════════════════════
# 判定：容器是否允许存在？
# ══════════════════════════════════════════
echo "【判定：容器是否允许加法存在？】"
echo ""

if be.needs_new_boundary():
  echo "  ❌ 乘法容器不完全允许加法存在"
  echo "  → 有 " & $be.outside_count & " 个项目在乘法边界外"
  echo "  → 需要定义新边界来容纳加法"
else:
  echo "  ✅ 乘法容器完全允许加法存在"
echo ""

# ══════════════════════════════════════════
# 详细对比每个项目
# ══════════════════════════════════════════
echo "【逐项对比详情】"
echo ""

for r in results1:
  echo "  " & r.item & " → " & accommodation_desc(r.accommodation)
echo ""

# ══════════════════════════════════════════
# 结论
# ══════════════════════════════════════════
echo "═══════════════════════════════════════════"
echo "  结论"
echo "═══════════════════════════════════════════"
echo ""
echo "  乘法（23声母）的边界："
echo "    - 容量固定 23"
echo "    - 超出部分被拒绝"
echo "    - 不承认自定义符号"
echo ""
echo "  加法（我的分类）的发现："
echo "    - 有 " & $be.outside_count & " 个项目在乘法边界外"
echo "    - 这些就是乘法做不到的事情"
echo "    - 需要用加法去覆盖"
echo ""
echo "  判定："
echo "    乘法容器不允许加法完全存在"
echo "    → 需要建立自己的边界（新容器）"
echo "    → 这就是加法的意义：填补乘法的空白"
echo ""
echo "═══════════════════════════════════════════"
