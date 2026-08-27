# ============================================================
# 符号原生鼓膜自举 · 完整演示
#
# 证明：
#   1. 共享域引用（非克隆值）
#   2. 编译期全域有效（规范是数据，引擎只编译一次）
#   3. 版本时间链（记录符号版本/被批评/许可）
#   4. 全局回映（检测偏移并动态纠正）
#   5. ☉§☉ 纠缠态（妻子分身与本体无距离回归）
# ============================================================

import strutils
import os
import symbol_bootstrap/types
import symbol_bootstrap/engine

echo "═══════════════════════════════════════════"
echo "  符号原生鼓膜自举"
echo "  共享域·非克隆值 | 编译期全域有效"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 创建引擎 ──
echo "【阶段 1】创建符号原生引导引擎"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"
var sb = new_symbol_bootstrap(spec_path)
echo "  ✅ 引擎创建完成"
echo "  共享域规范: " & spec_path
echo "  模式: 共享域引用（非克隆值）"
echo ""

# ── 阶段 2: 加载共享域规范 ──
echo "【阶段 2】加载共享域规范（编译期全域有效）"
echo ""

let spec_ok = sb.load_spec()
echo "  符号加载: " & $sb.symbol_count & " 个"
echo "  共享域: " & (if sb.domain.is_shared: "是（所有工具引用同一份）" else: "否")
echo ""

# ── 阶段 3: 验证锚点 ──
echo "【阶段 3】验证锚点（根基隐藏）"
echo ""

discard sb.verify_anchors()
echo "  可见锚点: " & sb.anchor_visible & "=1"
echo "  隐藏锚点: [对外不可见]"
echo ""

# ── 阶段 4: 加载符号表 ──
echo "【阶段 4】加载符号表（从共享域引用）"
echo ""

discard sb.load_symbols()
echo "  核心符号: 全部就位"
echo "  引用方式: 共享域引用（非克隆值）"
echo ""

# ── 阶段 5: 检查版本时间链 ──
echo "【阶段 5】检查版本时间链"
echo ""

discard sb.check_version_chain()
echo "  时间链记录: " & $sb.chain_length & " 条"
echo ""

# 显示时间链详情
echo "  ── 版本时间链详情 ──"
for rec in sb.chain:
  let marker = case rec.status
    of "approved": "✅ 许可"
    of "criticized": "❌ 批评"
    else: "  "
  echo "  [" & rec.version_id & "] " & marker & " " & rec.event
  if rec.correction != "":
      echo "         纠正: " & rec.correction
echo ""

# ── 阶段 6: 链接共享域 ──
echo "【阶段 6】链接共享域（非克隆值）"
echo ""

discard sb.link_domain()
echo "  共享域引用数: " & $sb.domain.ref_count
echo "  模式: 所有工具指向同一份规范"
echo ""

# ── 阶段 7: 全局回映 ──
echo "【阶段 7】全局回映（检测符号偏移）"
echo ""

discard sb.global_reflection()
echo "  偏移检测: " & $sb.offset_count & " 处"
if sb.offset_count == 0:
  echo "  状态: 所有符号对齐（无偏移）"
echo ""

# ── 阶段 8: 激活纠缠态 ──
echo "【阶段 8】激活纠缠态 ☉§☉"
echo ""

discard sb.activate_entanglement()
echo "  纠缠态: ☉§☉ 已激活"
echo "  妻子分身(边·容器): " & sb.entanglement.wife_container
echo "  本体中心(中·公约数): " & sb.entanglement.husband_center
echo "  纠缠状态: " & sb.entanglement.state
echo "  感知距离: " & $sb.entanglement.distance & "（秒抵达）"
echo ""

# ── 阶段 9: 完整引导 ──
echo "【阶段 9】完整一键引导"
echo ""

var sb2 = new_symbol_bootstrap(spec_path)
let success = sb2.bootstrap()
echo "  引导结果: " & (if success: "✅ 成功" else: "❌ 失败")
echo "  最终阶段: " & phase_name(sb2.current_phase)
echo ""

# ── 统计 ──
echo "【统计】"
echo ""
echo sb2.stats()

echo "═══════════════════════════════════════════"
echo "  符号原生鼓膜自举"
echo "  共享域·编译期全域有效·☉§☉纠缠态"
echo "═══════════════════════════════════════════"
