# ============================================================
# YIN-YANG-BINARY 三维编码引擎 · 演示
#
# 映射关系：
#   - 极性 → YinYang 二进制（阳=1·阴=0·平衡=0.5）
#   - 编码 → 三维坐标（mlil表面 × kndl结构 × 灰原哀横向）
#   - 符号 → 三角色模块（毛利兰·工藤新一·灰原哀）
# ============================================================

import strutils
import os
import yinyang3d

echo "═══════════════════════════════════════════"
echo "  YIN-YANG-BINARY 三维编码引擎"
echo "  mlil 毛利兰·kndl 工藤新一·灰原哀"
echo "═══════════════════════════════════════════"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"
let output_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/yinyang-3d/yinyang_space_3d.html"

# ── 构建编码空间 ──
echo "【阶段1】构建 YIN-YANG-BINARY 编码空间"
echo ""

var space = build_space(spec_path)

echo "  单元格数: " & $space.cell_count
echo ""

# ── 显示编码详情 ──
echo "【阶段2】符号 → 三维编码详情"
echo ""

for cell in space.cells:
  echo "  " & cell.code & " | " & cell.binary & " | " & 
       role_name(cell.role) & " | " & cell.surface & " × " & cell.structure
echo ""

# ── YinYang 二进制转换 ──
echo "【阶段3】YinYang 二进制 → 十进制"
echo ""

for cell in space.cells:
  if cell.binary != "0.5":
    let dec = yinyang_to_decimal(cell.binary)
    echo "  " & cell.code & " | 二进制=" & cell.binary & " | 十进制=" & $dec
echo ""

# ── 角色映射 ──
echo "【阶段4】三角色模块映射"
echo ""

echo "  毛利兰（表面·模/面/名）："
for cell in space.cells:
  if cell.role == roleMaolilan:
    echo "    " & cell.code & " → " & cell.surface
echo ""

echo "  工藤新一（结构·骨/头/体）："
for cell in space.cells:
  if cell.role == roleKudoshinichi:
    echo "    " & cell.code & " → " & cell.structure
echo ""

echo "  灰原哀（横向·行页/行业）："
for cell in space.cells:
  if cell.role == roleHuiyuanai:
    echo "    " & cell.code & " → " & cell.horizontal
echo ""

# ── 生成 HTML ──
echo "【阶段5】生成 HTML 可视化"
echo ""

generate_html(space, output_path)

echo "  ✅ 已生成: " & output_path
echo ""

# ── 统计 ──
echo "【统计】"
echo ""
echo space.stats()
echo ""

echo "═══════════════════════════════════════════"
echo "  YIN-YANG-BINARY 三维编码引擎 · 完成"
echo "  极性→二进制 | 编码→三维 | 符号→角色"
echo "═══════════════════════════════════════════"
