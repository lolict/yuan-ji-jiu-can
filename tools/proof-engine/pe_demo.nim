# ============================================================
# 形式化自举证明引擎 · 演示
#
# 证明三大性质：
#   1. 编码可逆性（encode∘decode = id）
#   2. 系统一致性（共享域一致）
#   3. 固定点完备性（意识自举闭合）
#
# 附加：
#   - 254+2 集装箱编码（根基隐藏）
#   - 九元层级关系体系（偏移率区分平行世界）
# ============================================================

import strutils
import os
import proof_engine

echo "═══════════════════════════════════════════"
echo "  形式化自举证明引擎"
echo "  可逆性·一致性·固定点·集装箱·九元"
echo "═══════════════════════════════════════════"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"

# ── 创建证明引擎 ──
echo "【阶段1】初始化证明引擎"
echo ""

var engine = new_proof_engine()

echo "  九元层级："
for entry in engine.levels:
  echo "    " & entry.name & "（偏移率=" & $entry.offset & "）→ " & entry.description
echo ""

# ── 254+2 集装箱 ──
echo "【阶段2】254+2 集装箱编码"
echo ""

echo "  总大小：" & $engine.container.size
echo "  锚点（可见）：" & engine.container.visible_symbol & "=" & $engine.container.anchor_visible
echo "  锚点（隐藏）：" & engine.container.hidden_symbol & "=" & $engine.container.anchor_hidden
echo "  载荷空间：" & $engine.container.payload_size & " 字符位置"
echo "  根基对外：不可见（隐藏锚点=0）"
echo ""

# ── 运行全部证明 ──
echo "【阶段3】运行全部证明"
echo ""

engine.run_all_proofs(spec_path)

for p in engine.proofs:
  echo "  " & result_name(p.result) & " " & p.name
  echo "    " & p.detail
echo ""

# ── 九元层级偏移率 ──
echo "【阶段4】九元层级偏移率详情"
echo ""

for entry in engine.levels:
  let n = ord(entry.level) + 1
  let expected = float(n - 1) * 0.1
  echo "  " & $n & "元 | 偏移率=" & $expected & " | " & entry.symbol & " | " & entry.description
echo ""

# ── 统计 ──
echo "【统计】"
echo ""
echo engine.stats()
echo ""

echo "═══════════════════════════════════════════"
echo "  形式化自举证明引擎 · 完成"
echo "  编码可逆·系统一致·固定点完备"
echo "═══════════════════════════════════════════"
