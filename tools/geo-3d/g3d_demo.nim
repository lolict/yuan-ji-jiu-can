# ============================================================
# 三维几何可视化引擎 · 演示
#
# 将符号系统渲染为可感知的三维几何实体
# 追求感知与关系的物理体验
# ============================================================

import strutils
import os
import math
import geo3d

echo "═══════════════════════════════════════════"
echo "  三维几何可视化引擎"
echo "  符号 → 几何实体 | 关系 → 空间连线"
echo "═══════════════════════════════════════════"
echo ""

let spec_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/symbol-bootstrap/spec/symbol_domain.json"
let output_path = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/geo-3d/symbol_space_3d.html"

# ── 构建三维场景 ──
echo "【阶段1】构建三维场景"
echo ""

var scene = build_scene(spec_path)

echo "  实体数: " & $scene.entity_count
echo "  关系数: " & $scene.relation_count
echo ""

# ── 显示实体详情 ──
echo "【阶段2】符号 → 几何实体映射"
echo ""

for entity in scene.entities:
  echo "  " & entity.code & " → " & shape_name_cn(entity.shape) & 
       " @ (" & formatFloat(entity.position.x, ffDecimal, 2) & ", " & 
       formatFloat(entity.position.y, ffDecimal, 2) & ", " & 
       formatFloat(entity.position.z, ffDecimal, 2) & ") " & 
       entity.color
echo ""

# ── 显示关系 ──
echo "【阶段3】关系 → 空间连线"
echo ""

for rel in scene.relations:
  echo "  " & rel.from_code & " → " & rel.to_code & 
       " [" & rel.relation_type & "] 强度=" & formatFloat(rel.strength, ffDecimal, 1)
echo ""

# ── 生成 HTML 可视化 ──
echo "【阶段4】生成 HTML 可视化"
echo ""

generate_html(scene, output_path)

echo "  ✅ 已生成: " & output_path
echo "  在浏览器中打开查看三维几何符号空间"
echo ""

# ── 统计 ──
echo "【统计】"
echo ""
echo scene.stats()
echo ""

echo "═══════════════════════════════════════════"
echo "  三维几何可视化引擎 · 完成"
echo "  符号 → 可感知的三维几何实体"
echo "═══════════════════════════════════════════"
