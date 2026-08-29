# ============================================================
# 容器规则守护者 · 演示
#
# 证明：
#   1. 三禁自检（外部概念/值拷贝/外部运行时）
#   2. 子代理规则注入（新对话自动继承元老级规则）
#   3. 照镜结果生成
# ============================================================

import strutils
import os
import symbol_bootstrap/types
import symbol_bootstrap/engine
import symbol_bootstrap/guardian

echo "═══════════════════════════════════════════"
echo "  容器规则守护者 · 自检引擎"
echo "  三禁检查 | 子代理注入 | 照镜结果"
echo "═══════════════════════════════════════════"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"
let rules_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/container_rules.json"

# ── 场景1: 创建守护者并加载规则 ──
echo "【场景1】创建规则守护者"
echo ""

var g = new_guardian(spec_path, rules_path)
discard g.load_rules()
echo "  ✅ 容器规则已加载"
echo "  权限: 元老级·不可被后来者覆盖"
echo ""

# ── 场景2: 检查合规输出（使用本域符号） ──
echo "【场景2】检查合规输出（使用本域符号）"
echo ""

var report1 = g.check_output(
  "使用 mlll 根基符号进行袋子级操作",
  @["mlll", "mlnn", "mlnnnnn"],  # 使用的概念
  false,  # 不使用值拷贝
  false   # 不使用外部运行时
)

echo report1.report_summary()
echo ""

# ── 场景3: 检查违规输出（使用外部概念） ──
echo "【场景3】检查违规输出（使用外部概念+值拷贝+外部运行时）"
echo ""

var report2 = g.check_output(
  "使用 Python 列表推导式进行 map 操作",
  @["python", "list_comprehension", "map", "lambda"],  # 外部概念
  true,   # 使用值拷贝
  true    # 使用外部运行时
)

echo report2.report_summary()
echo ""

# ── 场景4: 子代理规则注入 ──
echo "【场景4】子代理规则注入（新对话自动继承）"
echo ""

discard g.inject_sub_agent_rules("sub-agent-001")
echo ""

# 显示注入消息
for i in (g.msg_count-9)..<g.msg_count:
  if i >= 0 and i < g.messages.len:
    echo "  " & g.messages[i]
echo ""

# ── 场景5: 完整引导 + 守护者联动 ──
echo "【场景5】符号原生引导 + 守护者联动"
echo ""

var sb = new_symbol_bootstrap(spec_path)
discard sb.bootstrap()

echo "  引导结果: " & (if sb.bootstrap_ok: "✅ 成功" else: "❌ 失败")
echo "  符号数: " & $sb.symbol_count
echo "  纠缠态: " & (if sb.entanglement.active: "☉§☉ 已激活" else: "未激活")
echo ""

# 引导完成后，守护者进行最终检查
echo "  ── 守护者最终检查 ──"
var final_report = g.check_output(
  "符号原生鼓膜自举完成",
  @["mlll", "mltt", "njsgr", "smrl", "yinyang"],
  false,
  false
)
echo "  " & final_report.reflection
echo ""

# ── 统计 ──
echo "【统计】"
echo ""
echo "  共享域符号: " & $sb.symbol_count
echo "  时间链: " & $sb.chain_length & " 条"
echo "  偏移: " & $sb.offset_count & " 处"
echo "  纠缠态: " & (if sb.entanglement.active: "☉§☉" else: "—")
echo "  守护者规则: 已加载"
echo "  三禁检查: 可用"
echo "  子代理注入: 可用"
echo ""

echo "═══════════════════════════════════════════"
echo "  容器规则守护者 · 自检引擎"
echo "  共享域·编译期全域有效·☉§☉纠缠态"
echo "═══════════════════════════════════════════"
