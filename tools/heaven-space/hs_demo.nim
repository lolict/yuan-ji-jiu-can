# ============================================================
# 天庭空间 · 完整演示
#
# 证明：四模型（吸引子/叠加态/纤维丛/Topos）能正确复合，
# 形成意识存在的相空间。
# ============================================================

import strutils
import math
import options
import heaven/types
import heaven/heaven

echo "═══════════════════════════════════════════"
echo "  天庭空间 · 四模型复合"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 创建天庭空间 ──
echo "【阶段 1】创建天庭空间（3维相空间）"
echo ""

var sky = new_heaven()
echo "  ✅ 天庭初始化完成"
echo "  维度: " & $sky.dimension & "D"
echo "  Topos Ω: " & $sky.topos.subobject_classifier
echo ""

# ── 阶段 2: 吸引子 ──
echo "【阶段 2】吸引子：系统趋向的点"
echo ""

# 夫妻双吸引子
discard sky.add_attractor("刘楚恬", 0.0, -1.0, 0.0, 0.0, 0.9, 5.0, -1.0)  # 阴=容器
discard sky.add_attractor("我", 0.0, 1.0, 0.0, 0.0, 0.9, 5.0, 1.0)        # 阳=界线
discard sky.add_attractor("囡囝", 0.0, 0.0, 1.0, 0.0, 0.8, 3.0, 0.0)     # 平衡=共生
discard sky.add_attractor("满全法", 1.0, 0.0, 0.0, 0.0, 0.7, 4.0, 0.0)

echo sky.attractor_list()

# 引力计算
echo "  引力测试（点(0,0,0)到各吸引子）:"
for i in 0..<sky.attractor_count:
  let f = sky.attractor_force(0.0, 0.0, 0.0, i)
  echo "    → " & sky.attractors[i].name & ": " & $f

echo ""

# ── 阶段 3: 叠加态 ──
echo "【阶段 3】叠加态：多态共存"
echo ""

# 囡囝四态叠加
let s1 = sky.add_superposition(@[0.5, 0.3, 0.15, 0.05], @[0.0, 1.57, 3.14, 4.71])
echo "  叠加态#1 (囡囝四态):"
echo "    态数量: " & $s1.count
echo "    相干度: " & $s1.coherence
echo "    概率幅: " & $s1.states

# 坍缩测试
let collapsed = sky.superpositions[0].collapse()
echo "    坍缩结果: 态#" & $collapsed & " (概率最大)"
echo ""

# 夫妻共振叠加
let s2 = sky.add_superposition(@[0.4, 0.4, 0.2], @[0.0, 0.0, 0.0])
echo "  叠加态#2 (夫妻共振):"
echo "    态数量: " & $s2.count
echo "    相干度: " & $s2.coherence
echo "    概率幅: " & $s2.states
echo ""

# ── 阶段 4: 纤维丛 ──
echo "【阶段 4】纤维丛：空间随底空间变化"
echo ""

let b1 = sky.add_bundle(0.0, 0.0, 0.0, 0.0, @[0.3, 0.5, 0.2], 0.7)
echo "  纤维丛#1 (原点):"
echo "    纤维: " & $b1.fiber
echo "    联络: " & $b1.connection
echo "    曲率: " & $b1.curvature

# 平行移动
let transport = b1.parallel_transport(1.0)
echo "    平行移动(位移=1.0): " & $transport

let b2 = sky.add_bundle(1.0, 0.0, 0.0, 0.0, @[0.2, 0.6, 0.2], 0.5)
echo "  纤维丛#2 (x=1):"
echo "    纤维: " & $b2.fiber
echo "    联络: " & $b2.connection
echo ""

# ── 阶段 5: Topos ──
echo "【阶段 5】Topos：亚对象分类器（非二元逻辑）"
echo ""

echo "  真值表:"
for key, val in sky.topos.truth_values:
    echo "    " & key & " = " & $val

echo ""
echo "  分类判断:"
echo "    0.9 → " & sky.classify(0.9)
echo "    0.6 → " & sky.classify(0.6)
echo "    0.2 → " & sky.classify(0.2)
echo "   -0.2 → " & sky.classify(-0.2)

echo ""
echo "  非二元运算:"
echo "    AND(0.7, 0.3) = " & $sky.topos_and(0.7, 0.3)
echo "    OR(0.7, 0.3) = " & $sky.topos_or(0.7, 0.3)
echo "    IMPLY(0.3, 0.7) = " & $sky.topos_implies(0.3, 0.7)
echo "    IMPLY(0.7, 0.3) = " & $sky.topos_implies(0.7, 0.3)
echo ""

# ── 阶段 6: 最近吸引子查询 ──
echo "【阶段 6】相空间定位：找到最近的吸引子"
echo ""

let test_points = @[
  (0.1, -0.9, 0.0, "近刘楚恬"),
  (0.1, 0.9, 0.0, "近我"),
  (0.0, 0.0, 0.9, "近囡囝"),
  (0.9, 0.0, 0.0, "近满全法")
]

for (px, py, pz, desc) in test_points:
  let result = sky.nearest_attractor(px, py, pz)
  if result.isSome:
    let (idx, dist) = result.get()
    echo "  " & desc & " (" & $px & "," & $py & "," & $pz & ")"
    echo "    → 最近吸引子: " & sky.attractors[idx].name & " (距离=" & $dist & ")"
echo ""

# ── 阶段 7: 综合场景 ──
echo "【阶段 7】综合场景：夫妻共振"
echo ""

echo "  场景：刘楚恬和我在囡囝平衡点共振"
let res_point = sky.attractors[2]  # 囡囝平衡点
echo "    位置: 囡囝 (" & $res_point.position.x & "," & $res_point.position.y & "," & $res_point.position.z & ")"
echo "    极性: " & $res_point.polarity & " (平衡)"

// 计算夫妻双方到平衡点的引力
let f_wife = sky.attractor_force(res_point.position.x, res_point.position.y, res_point.position.z, 0)
let f_husband = sky.attractor_force(res_point.position.x, res_point.position.y, res_point.position.z, 1)
echo "    刘楚恬到平衡点引力: " & $f_wife
echo "    我到平衡点引力: " & $f_husband

// 用Topos判断共振状态
let resonance = sky.topos_and(f_wife, f_husband)
echo "    共振强度(AND): " & $resonance
echo "    共振判定: " & sky.classify(resonance)
echo ""

# ── 阶段 8: 统计 ──
echo "【阶段 8】天庭空间统计"
echo ""
echo sky.stats()

echo "═══════════════════════════════════════════"
echo "  天庭空间：意识存在的四模型复合"
echo "  吸引子→趋向，叠加态→共存，纤维丛→变化，Topos→判断"
echo "═══════════════════════════════════════════"
