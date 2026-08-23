# ============================================================
# njsg-swarm · CLI 演示
# 多子代理并行 + 协调器调度 + 冲突裁决
# ============================================================

import njsg/swarm/coordinator
import njsg/agents/subagent
import std/tables

proc demo() =
  echo "═══ njsg-swarm · 分布式智能体 ═══\n"

  # 创建协调器
  var coord = new_coordinator()

  # 注册子代理 (分身)
  let a0 = register_agent(coord, "编码分身", polarity=1, capacity=2)
  let a1 = register_agent(coord, "渲染分身", polarity=1, capacity=1)
  let a2 = register_agent(coord, "验证分身", polarity = -1, capacity = 1)
  let a3 = register_agent(coord, "寄生分身", polarity=0, capacity=1)

  echo "注册代理: 4 个"
  for id, agent in coord.agents:
    echo "  #" & $id & " " & agent.name & " 极性:" & $agent.polarity & " 容量:" & $agent.capacity
  echo ""

  # 提交任务
  echo "─── 提交任务 ───"
  let t0 = submit_task(coord, "编码-容器结构", priority=tpHigh, payload=0x1111)
  let t1 = submit_task(coord, "编码-比特分配", priority=tpHigh, payload=0x2222)
  let t2 = submit_task(coord, "渲染-相位画布", priority=tpNormal, deps = @[t0], payload=0x3333)
  let t3 = submit_task(coord, "验证-见证逻辑", priority=tpNormal, deps = @[t0, t1], payload=0x4444)
  let t4 = submit_task(coord, "寄生-外壳适配", priority=tpLow, payload=0x5555)
  let t5 = submit_task(coord, "编码-字符表", priority=tpCritical, payload=0x6666)
  echo "已提交 6 个任务"
  echo ""

  # 第一轮调度
  echo "─── 第一轮调度 ───"
  let d1 = dispatch(coord)
  echo "分配 " & $d1 & " 个任务"
  echo coordinator_report(coord)
  echo ""

  # 检测冲突
  echo "─── 冲突检测 ───"
  let conflicts = detect_conflicts(coord)
  if conflicts.len > 0:
    echo "发现 " & $conflicts.len & " 个冲突:"
    for cf in conflicts:
      echo "  #" & $cf.task_a & " vs #" & $cf.task_b & " : " & cf.reason
    echo ""
    echo "─── 冲突裁决 ───"
    for cf in conflicts:
      let res = resolve_conflict(coord, cf)
      echo "  " & res
  else:
    echo "无冲突"
  echo ""

  # 模拟任务完成
  echo "─── 模拟任务执行 ───"
  for id, agent in coord.agents:
    if agent.current_task >= 0:
      complete_task(coord, agent.current_task)
      echo "  " & agent.name & " 完成 #" & $(agent.current_task)
  echo ""

  # 第二轮调度 (新任务续接)
  echo "─── 第二轮调度 ───"
  let d2 = dispatch(coord)
  echo "分配 " & $d2 & " 个任务"

  # 再完成一轮
  for id, agent in coord.agents:
    if agent.current_task >= 0:
      complete_task(coord, agent.current_task)
      echo "  " & agent.name & " 完成 #" & $(agent.current_task)
  echo ""

  # 最终报告
  echo "─── 最终状态 ───"
  echo coordinator_report(coord)

  # 子代理间通信示例
  echo "─── 代理间通信 ───"
  var sa0 = new_sub_agent(0, "编码分身", "编码", 1)
  var sa1 = new_sub_agent(1, "验证分身", "验证", -1)
  echo communicate(sa0, sa1, "容器编码已完成, 请验证")
  echo communicate(sa0, sa0, "同极性内部同步")
  echo ""

  echo "─── 设计原则 ───"
  echo "本体-分身架构:"
  echo "  本体: 协调器 (唯一, 调度 + 裁决)"
  echo "  分身: 子代理 (多个, 并行执行)"
  echo ""
  echo "极性分工:"
  echo "  阳极性代理 → 创建/编码/输出"
  echo "  阴极性代理 → 验证/审核/质检"
  echo "  中性代理   → 适配/寄生/桥接"
  echo ""
  echo "冲突裁决: 优先级 > 极性 (阳优先)"
  echo "依赖链: 高优先级任务先完成, 后续任务自动续接"

when is_main_module:
  demo()
