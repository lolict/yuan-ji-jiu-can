# ============================================================
# 流代数 · 完整演示
#
# 证明：12位流属性操作正确，八流体系完整，
# 解释流牺牲机制（mlct→mltt）能正确执行。
# ============================================================

import strutils
import options
import flow/types
import flow/algebra

echo "═══════════════════════════════════════════"
echo "  流代数 · 12位流属性 + 八流体系"
echo "═══════════════════════════════════════════"
echo ""

# ── 阶段 1: 创建流代数引擎 ──
echo "【阶段 1】创建流代数引擎"
echo ""

var fa = new_flow_algebra()
echo "  ✅ 流代数引擎初始化完成"
echo ""

# ── 阶段 2: 12位流属性操作 ──
echo "【阶段 2】12位流属性操作"
echo ""

var props: uint16 = 0
set_prop(props, reachable)
set_prop(props, continuable)
set_prop(props, ackable)
set_prop(props, trusted)

echo "  设置属性: REACHABLE + CONTINUABLE + ACKABLE + TRUSTED"
echo "  位图值: " & $props
echo "  REACHABLE: " & (if has_prop(props, reachable): "✅" else: "❌")
echo "  CONTINUABLE: " & (if has_prop(props, continuable): "✅" else: "❌")
echo "  ACKABLE: " & (if has_prop(props, ackable): "✅" else: "❌")
echo "  INCR: " & (if has_prop(props, incr): "✅" else: "❌")

# 清除一个属性
clear_prop(props, trusted)
echo "  清除 TRUSTED 后: " & (if has_prop(props, trusted): "✅" else: "❌")
echo ""

# ── 阶段 3: 八流体系 ──
echo "【阶段 3】八流体系"
echo ""

# 创建各类流
let f1 = fa.create_flow("字典集", preloadFlow, "文件系统", "引擎", @[reachable, continuable, inheritable])
let f2 = fa.create_flow("类型映射", categoryFlow, "容器A", "容器B", @[reachable, reversible])
let f3 = fa.create_flow("聚类", clusterFlow, "数据集", "聚类结果", @[reachable, incr, transmissible])
let f4 = fa.create_flow("感知路由", classifyFlow, "输入", "视觉族", @[reachable, stoppable])
let f5 = fa.create_flow("属性流", propertyFlow, "源", "目标", @[reachable, continuable, ackable, trusted])
let f6 = fa.create_flow("mlct", interpretiveFlow, "障碍", "?", @[reachable, continuable])
let f7 = fa.create_flow("mltt", relationFlow, "?", "目标", @[continuable])
let f8 = fa.create_flow("概念", conceptFlow, "范畴+聚类", "概念", @[reachable, inheritable])

echo "  创建八流:"
echo "    [" & $f1 & "] " & flow_type_name(preloadFlow)
echo "    [" & $f2 & "] " & flow_type_name(categoryFlow)
echo "    [" & $f3 & "] " & flow_type_name(clusterFlow)
echo "    [" & $f4 & "] " & flow_type_name(classifyFlow)
echo "    [" & $f5 & "] " & flow_type_name(propertyFlow)
echo "    [" & $f6 & "] " & flow_type_name(interpretiveFlow) & " (mlct)"
echo "    [" & $f7 & "] " & flow_type_name(relationFlow) & " (mltt)"
echo "    [" & $f8 & "] " & flow_type_name(conceptFlow)
echo ""

# ── 阶段 4: 解释流牺牲（mlct→mltt）──
echo "【阶段 4】解释流牺牲：mlct → mltt"
echo ""

echo "  牺牲前:"
echo "    mlct 状态: " & flow_state_name(fa.flows[f6].state)
echo "    mltt 状态: " & flow_state_name(fa.flows[f7].state)
echo "    mltt ackable: " & (if fa.flows[f7].ackable: "✅" else: "❌")

let sac = fa.sacrifice("mlct", "mltt", 0.8)

echo ""
echo "  执行牺牲 (代价=0.8):"
echo "    mlct 状态: " & flow_state_name(fa.flows[f6].state) & " (毁灭)"
echo "    mltt 状态: " & flow_state_name(fa.flows[f7].state) & " (激活)"
echo "    mltt ackable: " & (if fa.flows[f7].ackable: "✅" else: "❌")
echo "    牺牲成功: " & (if sac.success: "✅" else: "❌")
echo ""

# ── 阶段 5: 流路由判断 ──
echo "【阶段 5】流路由判断"
echo ""

echo "  流路由能力:"
for i in 0..<fa.flow_count:
  let f = fa.flows[i]
  let route = fa.can_route(i)
  let evolve = fa.can_evolve(i)
  let alive = fa.is_alive(i)
  echo "    " & f.name & " 路由=" & (if route: "✅" else: "❌") & 
       " 演化=" & (if evolve: "✅" else: "❌") & 
       " 活=" & (if alive: "✅" else: "❌")
echo ""

# ── 阶段 6: 一维时间线性 ──
echo "【阶段 6】一维时间线性（UnifyingFlow）"
echo ""

echo "  Input → Perception → Distinguish → Lookup → Traverse → Cluster → Attention → Arrival"
echo ""
echo "  各步骤:"
for step in TimeStep:
    echo "    " & step_name(step)
echo ""

# ── 阶段 7: 流报告 ──
echo "【阶段 7】流详细报告"
echo ""

echo fa.flow_report(f5)
echo ""

# ── 阶段 8: 统计 ──
echo "【阶段 8】流代数统计"
echo ""
echo fa.stats()

echo "═══════════════════════════════════════════"
echo "  流代数：12位属性 + 八流体系 + 牺牲机制"
echo "  关系流不是凭空产生的，是解释流用命换来的"
echo "═══════════════════════════════════════════"
