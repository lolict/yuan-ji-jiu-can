# ============================================================
# 分别器 (Distinguisher) · 完整演示
#
# 证明：三层路由能正确将输入分类到正确的靶点，
# 闭环条件（ackable）决定流是否断裂。
# ============================================================

import strutils
import options
import distinguisher/types
import distinguisher/distinguisher

echo "═══════════════════════════════════════════"
echo "  分别器 (Distinguisher) · 三层路由系统"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 创建分别器 ──
echo "【阶段 1】创建分别器，加载映射表"
echo ""

var df = new_distinguisher()
echo "  ✅ 分别器初始化完成"
echo "  家族映射: " & $df.family_count & " 条"
echo "  谐音映射: " & $df.sound_count & " 条"
echo "  主权映射: " & $df.sovereignty_count & " 条"
echo ""

# ── 阶段 2: 第一层路由测试 ──
echo "【阶段 2】第一层：感知家族输入 → 视觉族"
echo ""

let family_inputs = @["因", "囝", "囡囝", "lct", "未知字符"]
for inp in family_inputs:
  let family = df.route_family(inp)
  echo "  '" & inp & "' → " & family_name(family)
echo ""

# ── 阶段 3: 第二层路由测试 ──
echo "【阶段 3】第二层：谐音靶点发音 → 靶点对象"
echo ""

let target_inputs = @["毛利兰", "工藤新一", "mltt", "gtt", "外部输入"]
for inp in target_inputs:
  let target = df.route_target(inp)
  echo "  '" & inp & "' → " & target_name(target)
echo ""

# ── 阶段 4: 完整三层路由 ──
echo "【阶段 4】完整三层路由（核心演示）"
echo ""

let route_inputs = @[
  "毛利兰",
  "工藤新一",
  "lctqf",
  "mltt",
  "gtt",
  "囡囝",
  "袋",
  "外部人员",
  "未知符号"
]

for inp in route_inputs:
  let result = df.route(inp)
  echo "  输入: '" & result.input & "'"
  echo "    第一层: " & family_name(result.family)
  echo "    第二层: " & target_name(result.target)
  echo "    第三层: " & sovereignty_name(result.sovereignty)
  echo "    闭环: " & (if result.ackable: "✅ 可承认" else: "❌ 流断裂")
  echo "    置信度: " & $result.confidence
  echo ""

# ── 阶段 5: 闭环条件验证 ──
echo "【阶段 5】闭环条件验证"
echo ""

echo "  闭环条件: target ∈ {Wife, Husband, Sweet, SoundMorph} AND sovereignty = Active"
echo ""
echo "  可承认（流不断）:"
echo "    毛利兰 → Wife → Active → ✅ ack"
echo "    工藤新一 → Husband → Active → ✅ ack"
echo "    mltt → Sweet → Active → ✅ ack"
echo ""
echo "  流断裂（对方不承认）:"
echo "    外部人员 → External → Passive → ❌ 只读，无法ack"
echo "    未知符号 → Unknown → System → ❌ 系统级，无个人主权"
echo ""

# ── 阶段 6: 路由统计 ──
echo "【阶段 6】路由统计"
echo ""
echo df.stats()

# ── 阶段 7: 路由历史 ──
echo "【阶段 7】路由历史（最近5条）"
echo ""

let recent = min(5, df.history.len)
for i in (df.history.len - recent)..<df.history.len:
  let record = df.history[i]
  echo "  #" & $record.timestamp & " '" & record.result.input & "' → " & 
       (if record.success: "✅" else: "❌")
echo ""

# ── 阶段 8: 囡囝四态的路由 ──
echo "【阶段 8】囡囝四态意识拓扑的路由"
echo ""

let states = @[
  ("囡囝", "女包男 = 容器包被包含者"),
  ("囡囡", "女包女 = 容器包容器"),
  ("囝囝", "男包男 = 被包含者包被包含者"),
  ("囝囡", "男包女 = 被包含者包容器")
]

for (inp, desc) in states:
  let result = df.route(inp)
  echo "  '" & inp & "' (" & desc & ")"
  echo "    → " & family_name(result.family) & " → " & target_name(result.target)
  echo "    闭环: " & (if result.ackable: "✅" else: "❌")
echo ""

echo "═══════════════════════════════════════════"
echo "  分别器：没有它，流到错误靶点 = 断裂"
echo "  有了它，输入被正确路由到能ack的目标"
echo "═══════════════════════════════════════════"
