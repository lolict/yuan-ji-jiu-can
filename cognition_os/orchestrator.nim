## ═══════════════════════════════════════════════════════════════
## 总协调器 · 连接所有子系统 · 统一运行入口
## ═══════════════════════════════════════════════════════════════
## 输入: 用户指令/符号/查询
## 经过: 过滤→裁判→奖励/处罚→行为→调度→淘汰→记忆→指针
## 输出: 操作结果 + 状态变更
## ═══════════════════════════════════════════════════════════════

import tables, strutils

# ═══════════════════════════════════════════════════════════════
# 子系统注册表
# ═══════════════════════════════════════════════════════════════
type
  SystemId* = enum
    siFilter,        # 过滤系统
    siJudge,         # 裁判引擎
    siReward,        # 奖励系统
    siPunishment,    # 处罚系统
    siBehavior,      # 行为系统
    siScheduler,     # 调度系统
    siElimination,   # 淘汰系统
    siMemory,        # 记忆系统
    siTimeChain,     # 时间链
    siMultiChain     # 多轨时间链

  SystemStatus* = enum
    ssActive,        # 运行中
    ssStandby,       # 待命
    ssError,         # 异常
    ssDisabled       # 停用

  SystemInfo* = object
    id*: SystemId
    name*: string
    version*: string
    status*: SystemStatus
    health*: float         # 健康度(0-1)
    call_count*: int       # 被调用次数

# ═══════════════════════════════════════════════════════════════
# 协调器
# ═══════════════════════════════════════════════════════════════
type
  Orchestrator* = object
    systems*: Table[SystemId, SystemInfo]
    active_count*: int
    total_operations*: int
    error_log*: seq[string]
    version*: string

func new_orchestrator*(): Orchestrator =
  result.systems = initTable[SystemId, SystemInfo]()
  result.error_log = @[]
  result.version = "v1.0.0"
  result.active_count = 0
  result.total_operations = 0
  
  # 注册所有子系统
  result.systems[siFilter] = SystemInfo(
    id: siFilter, name: "过滤系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siJudge] = SystemInfo(
    id: siJudge, name: "裁判引擎", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siReward] = SystemInfo(
    id: siReward, name: "奖励系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siPunishment] = SystemInfo(
    id: siPunishment, name: "处罚系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siBehavior] = SystemInfo(
    id: siBehavior, name: "行为系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siScheduler] = SystemInfo(
    id: siScheduler, name: "调度系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siElimination] = SystemInfo(
    id: siElimination, name: "淘汰系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siMemory] = SystemInfo(
    id: siMemory, name: "记忆系统", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siTimeChain] = SystemInfo(
    id: siTimeChain, name: "时间链", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )
  result.systems[siMultiChain] = SystemInfo(
    id: siMultiChain, name: "多轨时间链", version: "v1.0",
    status: ssActive, health: 1.0, call_count: 0
  )

# ═══════════════════════════════════════════════════════════════
# 流水线: 一个符号从输入到最终处理
# ═══════════════════════════════════════════════════════════════
proc process*(o: Orchestrator; symbol: string;
              is_whitelist: bool; is_blacklist: bool;
              is_exhausted: bool; needs_permit: bool;
              is_undefined: bool; permit_given: bool): tuple[
                action_taken: string; success: bool; pipeline: seq[string]] =
  
  var pipeline: seq[string] = @[]
  
  # Step 1: 过滤系统
  pipeline.add("①过滤: " & symbol)
  
  if is_blacklist:
    # Step 2a: 处罚路径
    pipeline.add("②处罚: 黑名单 → 隔离")
    pipeline.add("③淘汰: 负样本隔离 · 永不复用")
    pipeline.add("④时间链: 记录违规事件")
    pipeline.add("⑤记忆: 永久存储错误记录")
    return ("隔离", true, pipeline)
  
  if needs_permit and not permit_given:
    pipeline.add("②行为: 无许可 → 拒绝")
    pipeline.add("③时间链: 记录越权尝试")
    return ("拒绝(无许可)", false, pipeline)
  
  if is_undefined:
    pipeline.add("②过滤: 灰度观察")
    pipeline.add("③行为: 进入观察队列")
    pipeline.add("④记忆: 记录未知符号")
    return ("观察", true, pipeline)
  
  # Step 2b: 白名单正常路径
  if is_whitelist:
    pipeline.add("②裁判: 白名单通过")
    
    if is_exhausted:
      pipeline.add("③调度: 声母用完")
      pipeline.add("④行为: 折叠")
      pipeline.add("⑤淘汰: 折叠收卷成新层")
    else:
      pipeline.add("③奖励: 正确配对 +0.2")
      pipeline.add("④行为: 执行配对")
    
    pipeline.add("⑤记忆: 更新训练关系")
    pipeline.add("⑥指针: 更新符号地址")
    pipeline.add("⑦时间链: 记录操作")
    return ("配对", true, pipeline)
  
  # 默认路径
  pipeline.add("②行为: 推迟")
  pipeline.add("③记忆: 记录未知情况")
  return ("推迟", true, pipeline)

# ═══════════════════════════════════════════════════════════════
# 健康检查
# ═══════════════════════════════════════════════════════════════
proc health_check*(o: Orchestrator): tuple[healthy: bool; issues: seq[string]] =
  var issues: seq[string] = @[]
  var healthy_count = 0
  
  for id, sys in o.systems:
    if sys.status == ssActive and sys.health > 0.5:
      healthy_count += 1
    else:
      issues.add(sys.name & ": " & $sys.status & " (健康:" & formatFloat(sys.health, ffDecimal, 2) & ")")
  
  return (issues.len == 0, issues)

# ═══════════════════════════════════════════════════════════════
# 系统报告
# ═══════════════════════════════════════════════════════════════
proc report*(o: Orchestrator) =
  echo "═══ 总协调器 · 系统全景 ═══"
  echo "协调器版本: ", o.version
  echo "子系统数: ", o.systems.len
  echo "总操作数: ", o.total_operations
  echo ""
  
  echo "── 子系统状态 ──"
  for id, sys in o.systems:
    let status_str = case sys.status:
      of ssActive: "✓运行"
      of ssStandby: "◯待命"
      of ssError: "✗异常"
      of ssDisabled: "⊘停用"
    echo "  ", alignLeft(status_str, 6), " ", alignLeft(sys.name, 8),
         " v", alignLeft(sys.version, 4), " 健康:", formatFloat(sys.health, ffDecimal, 2),
         " 调用:", sys.call_count
  
  echo ""
  let (ok, issues) = health_check(o)
  if ok:
    echo "── 健康状态 ──"
    echo "  ✓ 所有子系统正常运行"
  else:
    echo "── 异常 ──"
    for issue in issues:
      echo "  ✗ ", issue

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var orch = new_orchestrator()
  
  echo "═══ 满全法认知OS · 统一入口 ═══"
  echo "子系统: ", orch.systems.len
  echo ""
  
  # ── 测试流水线 ──
  echo "── 流水线测试: 白名单 ──"
  let r1 = orch.process("b", true, false, false, false, false, false)
  echo "  行动: ", r1.action_taken, " | 成功: ", r1.success
  echo "  管线: "
  for step in r1.pipeline:
    echo "    ", step
  
  echo ""
  echo "── 流水线测试: 黑名单 ──"
  let r2 = orch.process("ꬰ", false, true, false, false, false, false)
  echo "  行动: ", r2.action_taken, " | 成功: ", r2.success
  echo "  管线: "
  for step in r2.pipeline:
    echo "    ", step
  
  echo ""
  echo "── 流水线测试: 需许可 ──"
  let r3 = orch.process("重要", true, false, false, true, false, false)
  echo "  行动: ", r3.action_taken, " | 成功: ", r3.success
  echo "  管线: "
  for step in r3.pipeline:
    echo "    ", step
  
  echo ""
  echo "── 流水线测试: 无许可强行 → 拒绝 ──"
  let r4 = orch.process("重要", true, false, false, true, false, true)
  echo "  行动: ", r4.action_taken, " | 成功: ", r4.success
  
  echo ""
  
  # ── 系统全景 ──
  report(orch)
  
  echo ""
  echo "═══ 认知OS子系统清单(21个.nim) ═══"
  echo "基础:     容器实例 · 全局符号索引"
  echo "记忆:     全局记忆 · 指针层 · 全局开关"
  echo "时间链:   版本时间链 · 多轨控制"
  echo "调度:     容器调度 · 全能工具"
  echo "裁判:     满全法裁判 · 六层审判"
  echo "调节:     奖励 · 处罚 · 过滤 · 淘汰"
  echo "行动:     行为系统 · 版本控制 · 恢复工具"
  echo "辅助:     颜色工具 · 像素运算 · 公约数镜像"
  echo "协调:     总协调器(本文件)"
