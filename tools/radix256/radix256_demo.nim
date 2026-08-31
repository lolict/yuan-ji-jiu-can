# ============================================================
# 2²⁵⁶ 等效高位对齐进位制 · 演示入口
#
# 36号工具 · 完整展示：
#   1. 基础构造（零、最大值、根基）
#   2. 进位仪式（逐级进位传播）
#   3. 对齐检测（距离2²⁵⁶还有多远）
#   4. 溢出折叠（到达2²⁵⁶触发新层）
#   5. 根基仪式（254+2）
# ============================================================

import strutils
import math
import radix256/base256
import radix256/types

echo "═══════════════════════════════════════════════════════"
echo "  2²⁵⁶ 等效高位对齐进位制 · 36号工具"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════
# 【演示1】基础构造
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示1】基础构造"
echo "═══════════════════════════════════════════════════════"
echo ""

let z = zero()
echo "  零值: " & uint256_to_hex(z)
echo "  对齐状态: " & alignment_state_str(z.alignment)
echo ""

let one = from_u64(1)
echo "  一值: " & uint256_to_hex(one)
echo ""

let max_val = max_value()
echo "  最大值 (2²⁵⁶-1): " & uint256_to_hex(max_val)
echo "  对齐状态: " & alignment_state_str(max_val.alignment)
echo "  是否已对齐: " & $is_aligned(max_val)
echo ""

# ═══════════════════════════════════════════════════════
# 【演示2】逐级进位仪式
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示2】逐级进位仪式"
echo "═══════════════════════════════════════════════════════"
echo ""

# 在最低词加到接近溢出
var v1 = zero()
v1.words[0] = 0xFFFFFFFFFFFFFFFF'u64  # 最大值

let v2 = from_u64(1)  # max + 1 → 溢出到词1

echo "  输入A: " & uint256_to_hex(v1)
echo "  输入B: " & uint256_to_hex(v2)
echo ""

let c1 = add_with_ceremony(v1, v2, 0)
for line in c1.ceremony_log:
  echo "  " & line
echo ""
echo "  对齐状态: " & alignment_state_str(c1.result.alignment)
echo ""

# ═══════════════════════════════════════════════════════
# 【演示3】词间进位链（跨越4个词）
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示3】词间进位链 · 全256位传递"
echo "═══════════════════════════════════════════════════════"
echo ""

# 构造一个每个词都接近最大值的数
var almost_max = zero()
almost_max.words[0] = 0xFFFFFFFFFFFFFFFF'u64
almost_max.words[1] = 0xFFFFFFFFFFFFFFFF'u64
almost_max.words[2] = 0xFFFFFFFFFFFFFFFF'u64
almost_max.words[3] = 0xFFFFFFFFFFFFFFFF'u64

let delta = from_u64(1)

echo "  接近最大值: " & uint256_to_hex(almost_max)
echo "  加1前的对齐状态: " & alignment_state_str(almost_max.alignment)
echo ""

let c2 = add_with_ceremony(almost_max, delta, 100)
for line in c2.ceremony_log:
  echo "  " & line
echo ""

if c2.fold_triggered:
  echo "  ★ 到达2²⁵⁶折叠点 · 溢出至新层 tier+1"
  echo "  ★ 这是边界 → 不是错误 → 是折叠的起点"
echo ""

# ═══════════════════════════════════════════════════════
# 【演示4】根基仪式（254+2）
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示4】根基仪式 · 254+2=256"
echo "═══════════════════════════════════════════════════════"
echo ""

let c3 = foundation_ceremony()
for line in c3.ceremony_log:
  echo "  " & line
echo ""

# ═══════════════════════════════════════════════════════
# 【演示5】进位历史记录
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示5】进位历史"
echo "═══════════════════════════════════════════════════════"
echo ""

var history = new_history()
history.record_step(CarryStep(step_id: 0, from_word: 0, to_word: 1, value_carried: 1, timestamp: 0))
history.record_step(CarryStep(step_id: 1, from_word: 1, to_word: 2, value_carried: 1, timestamp: 1))
history.record_step(CarryStep(step_id: 2, from_word: 2, to_word: 3, value_carried: 1, timestamp: 2))
history.record_fold(FoldTier(tier_id: 1, triggered_at: "0x1_00000000_00000000", carry_count: 1, parent_tier: 0, is_foundation: false))

echo "  进位历史记录:"
for s in history.seconds:
  echo "    tick=" & $s.timestamp & " 词" & $s.from_word & "→词" & $s.to_word
echo "  折叠层级:"
for t in history.tiers:
  echo "    tier " & $t.tier_id & " 触发于 " & t.triggered_at
echo ""

# ═══════════════════════════════════════════════════════
# 【演示6】2²⁵⁶ 数值摘要
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "【演示6】2²⁵⁶ 数值摘要"
echo "═══════════════════════════════════════════════════════"
echo ""

echo "  2²⁵⁶ = 115792089237316195423570985008687907853269984665640564039457584007913129639936"
echo ""
echo "  含义:"
echo "    256位 = 4个64位词"
echo "    2²⁵⁶ 是溢出的起点（不是终点）"
echo "    254+2 = 256 根基（254集装箱 + 2锚点）"
echo "    溢出到达 2²⁵⁶ → .trigger 新层折叠 → 不进位 → 形成新层"
echo ""

# ═══════════════════════════════════════════════════════
# 总结
# ═══════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════"
echo "  2²⁵⁶ 等效高位对齐进位制 · 演示完成"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "  ✅ 基础构造 — 零/最大值/根基"
echo "  ✅ 逐级进位 — 词内→词间→全256位"
echo "  ✅ 对齐检测 — 距离2²⁵⁶折叠点"
echo "  ✅ 溢出折叠 — 到达边界触发新层"
echo "  ✅ 根基仪式 — 254+2=256"
echo "  ✅ 进位历史 — 跨会话持久化"
echo ""
echo "  [总开关已接入] 2²⁵⁶ 进位制引擎启动完成。"
echo ""
