# ============================================================
# 负样本容器 · 完整演示
# 演示：拒绝 → 记录 → 申诉 → 条件变化 → 重新激活
# ============================================================

import strutils
import mq/judge
import mq/negative

# 创建测试容器
# 容器 1: 正常的（应通过）
let c1 = make_container(
  presence = 1, witness = 1, char_code = 42,
  anchor = 1, relation = 0, timing = 100, payload = 1
)

# 容器 2: 锚点不符（应被拒绝）
let c2 = make_container(
  presence = 1, witness = 1, char_code = 88,
  anchor = 0, relation = 0, timing = 200, payload = 2
)

# 容器 3: 空袋（应被拒绝，不可回写）
let c3 = make_container(
  presence = 0, witness = 0, char_code = 0,
  anchor = 0, relation = 0, timing = 0, payload = 3
)

# 容器 4: 槽位 0（应被拒绝）
let c4 = make_container(
  presence = 1, witness = 1, char_code = 0,
  anchor = 1, relation = 0, timing = 300, payload = 4
)

echo "═══════════════════════════════════════"
echo "  负样本容器 · 回写协议演示"
echo "═══════════════════════════════════════"
echo ""

# 创建负样本存储
var store = new_negative_store(capacity = 10)

# ---- 阶段 1: 裁判 ----
echo "【阶段 1】三容器同时提交裁判（期望锚点=1）"
echo ""

let j1 = judge(c1, 1)  # 应通过
let j2 = judge(c2, 1)  # 锚点不符，拒绝
let j3 = judge(c3, 1)  # 空袋，拒绝
let j4 = judge(c4, 1)  # 槽位0，拒绝

echo "容器 #1: " & verdict_desc(j1.verdict) & " → " & j1.reason
echo "容器 #2: " & verdict_desc(j2.verdict) & " → " & j2.reason
echo "容器 #3: " & verdict_desc(j3.verdict) & " → " & j3.reason
echo "容器 #4: " & verdict_desc(j4.verdict) & " → " & j4.reason
echo ""

# ---- 阶段 2: 记录负样本 ----
echo "【阶段 2】被拒绝的容器进入负样本存储"
echo ""

if j2.verdict == vrReject:
  let id2 = store.record_rejection(c2, j2)
  echo "容器 #2 记录为负样本 #" & $id2

if j3.verdict == vrReject:
  let id3 = store.record_rejection(c3, j3)
  echo "容器 #3 记录为负样本 #" & $id3

if j4.verdict == vrReject:
  let id4 = store.record_rejection(c4, j3)
  echo "容器 #4 记录为负样本 #" & $id4

echo ""
echo negative_report(store)
echo ""

# ---- 阶段 3: 申诉 ----
echo "【阶段 3】对可回写的负样本尝试申诉"
echo ""

# 容器 #2 锚点不符 → 现在条件变了，重新用 anchor=0 审查
echo "容器 #2: 之前 anchor=0 不符期望 anchor=1"
echo "          现在条件变化，用 anchor=0 重新裁判..."
let (ok2, newj2) = store.appeal(0, 0)  # index 0 = 容器#2
echo "          申诉结果: " & (if ok2: "✅ 激活！" else: "❌ 未通过") & " → " & newj2.reason
echo ""

# 容器 #3 空袋 → 不可回写
echo "容器 #3: 空袋无米，尝试申诉..."
let (ok3, newj3) = store.appeal(1, 0)  # index 1 = 容器#3（现在是第二个元素因为容器#2可能被移除）
echo "          申诉结果: " & (if ok3: "✅ 激活！" else: "❌ 未通过") & " → " & newj3.reason
echo ""

# ---- 阶段 4: 条件变化批量激活 ----
echo "【阶段 4】条件变化，批量尝试激活剩余负样本"
echo "          假设 anchor=1 重新开放..."
let activated = store.reactivate_on_condition(1)
echo "          成功激活: " & $activated & " 个"
echo ""

# ---- 最终状态 ----
echo "【最终状态】"
echo negative_report(store)
echo ""

echo "═══════════════════════════════════════"
echo "  演示完成"
echo "  负样本不是终点，是等待转机的记忆"
echo "═══════════════════════════════════════"
