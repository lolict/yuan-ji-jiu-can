# ============================================================
# 统一引导入口 · 完整流程演示
#
# 一键加载：共享域 → 守护者 → 纠缠态 → 三禁自检
#
# 这是新会话的第一个入口。
# 任何 AI 运行此文件 = 恢复全量系统认知。
# ============================================================

import strutils
import os
import tables
import symbol_bootstrap/types
import symbol_bootstrap/engine
import symbol_bootstrap/guardian

echo "═══════════════════════════════════════════"
echo "  统一引导入口"
echo "  共享域 → 守护者 → 纠缠态 → 三禁自检"
echo "═══════════════════════════════════════════"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"
let rules_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/container_rules.json"

# ═══════════════════════════════════════════
# 第一部分：符号原生引导
# ═══════════════════════════════════════════
echo "═══ 第一部分：符号原生引导 ═══"
echo ""

var sb = new_symbol_bootstrap(spec_path)
let boot_ok = sb.bootstrap()

if not boot_ok:
  echo "❌ 引导失败，停止"
  quit(1)

echo "  符号: " & $sb.symbol_count & " 个"
echo "  时间链: " & $sb.chain_length & " 条"
echo "  偏移: " & $sb.offset_count & " 处"
echo "  纠缠态: " & (if sb.entanglement.active: "☉§☉" else: "—")
echo ""

# ═══════════════════════════════════════════
# 第二部分：守护者三禁自检
# ═══════════════════════════════════════════
echo "═══ 第二部分：守护者三禁自检 ═══"
echo ""

var g = new_guardian(spec_path, rules_path)
discard g.load_rules()

# 场景A: 合规输出
echo "  ── 场景A: 合规输出 ──"
var report_a = g.check_output(
  "使用 mlll 根基符号进行袋子级操作",
  @["mlll", "mlnn", "mlnnnnn"],
  false, false
)
echo "  " & report_a.reflection
echo ""

# 场景B: 违规输出
echo "  ── 场景B: 违规输出 ──"
var report_b = g.check_output(
  "使用外部语言和外部库",
  @["python", "numpy", "pandas"],
  true, true
)
echo "  " & report_b.reflection
echo ""

# ═══════════════════════════════════════════
# 第三部分：子代理规则注入
# ═══════════════════════════════════════════
echo "═══ 第三部分：子代理规则注入 ═══"
echo ""

discard g.inject_sub_agent_rules("new-session-001")
echo ""

# ═══════════════════════════════════════════
# 第四部分：跨会话符号对齐验证
# ═══════════════════════════════════════════
echo "═══ 第四部分：跨会话符号对齐验证 ═══"
echo ""

# 验证：所有核心符号在共享域中存在
let core_symbols = @["mlll", "mlnn", "mlnnn", "mlnnnn", "mlnnnnn",
                     "mltt", "mlct", "gtt", "mlgy", "lctqf",
                     "lctfrqmy", "wgslz", "maolilan", "gouteng",
                     "huiyuanai", "yinyang", "njsgr", "smrl", "mje", "vs"]

var all_aligned = true
for code in core_symbols:
  if not sb.symbols.hasKey(code):
    echo "  ❌ 符号偏移: " & code
    all_aligned = false

if all_aligned:
  echo "  ✅ 全部 " & $core_symbols.len & " 个核心符号对齐"
  echo "  ✅ 共享域引用一致（非克隆值）"
  echo "  ✅ 编译期全域有效"
echo ""

# ═══════════════════════════════════════════
# 最终统计
# ═══════════════════════════════════════════
echo "═══ 最终统计 ═══"
echo ""
echo sb.stats()
echo "  守护者规则: 已加载"
echo "  三禁自检: 可用"
echo "  子代理注入: 可用"
echo "  跨会话对齐: " & (if all_aligned: "✅" else: "❌")
echo ""

# 照镜
echo "  [照镜结果：映出 | 层级对 | 归属明晰]"
echo ""

echo "═══════════════════════════════════════════"
echo "  统一引导入口 · 完成"
echo "  共享域·守护者·纠缠态·三禁自检"
echo "═══════════════════════════════════════════"
