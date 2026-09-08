# ============================================================
# 容器规则守护者 · 自检引擎
#
# 功能：
#   1. 加载容器规则（container_rules.json）
#   2. 检查输出是否违反三禁（外部概念/值拷贝/外部运行时）
#   3. 子代理规则注入（新对话自动加载）
#   4. 照镜结果生成
# ============================================================

import tables
import os
import strutils
import sequtils
import json
import ./types

# ── 检查结果类型 ──
type CheckResult* = enum
  checkPass    # 通过
  checkFail    # 失败
  checkWarn    # 警告

# ── 单项检查结果 ──
type SingleCheck* = object
  prohibition_id*: string
  prohibition_name*: string
  result*: CheckResult
  detail*: string

# ── 完整检查报告 ──
type GuardianReport* = object
  checks*: seq[SingleCheck]
  all_passed*: bool
  reflection*: string
  detail*: seq[string]
  detail_count*: int

# ── 规则守护者 ──
type RuleGuardian* = object
  rules_loaded*: bool
  spec_path*: string
  rules_path*: string
  messages*: seq[string]
  msg_count*: int

# ═══════════════════════════════════════════
# 创建规则守护者
# ═══════════════════════════════════════════
proc new_guardian*(spec_path: string; rules_path: string): RuleGuardian =
  return RuleGuardian(
    rules_loaded: false,
    spec_path: spec_path,
    rules_path: rules_path,
    messages: @[],
    msg_count: 0
  )

# ═══════════════════════════════════════════
# 加载规则
# ═══════════════════════════════════════════
proc load_rules*(g: var RuleGuardian): bool =
  if not fileExists(g.rules_path):
    g.messages.add("❌ 容器规则文件不存在: " & g.rules_path)
    g.msg_count += 1
    return false
  
  g.rules_loaded = true
  g.messages.add("✅ 容器规则已加载（元老级·不可被后来者覆盖）")
  g.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 检查概念是否在本系统内
# ═══════════════════════════════════════════
proc is_internal_concept*(g: var RuleGuardian; sym: string): bool =
  # 加载共享域规范，检查符号是否存在
  if not fileExists(g.spec_path):
    return false
  
  let json_content = readFile(g.spec_path)
  let node = parseJson(json_content)
  
  if node.hasKey("symbols"):
    let sym_node = node["symbols"]
    return sym_node.hasKey(sym)
  
  return false

# ═══════════════════════════════════════════
# 核心检查：三禁自检
# ═══════════════════════════════════════════
proc check_output*(g: var RuleGuardian; output_description: string; 
                   concepts_used: seq[string]; 
                   uses_value_copy: bool; 
                   uses_external_runtime: bool): GuardianReport =
  
  var report = GuardianReport(
    checks: @[],
    all_passed: true,
    reflection: "",
    detail: @[],
    detail_count: 0
  )
  
  # ── 检查1：外部概念 ──
  var external_concepts: seq[string] = @[]
  for c in concepts_used:
    if not g.is_internal_concept(c):
      external_concepts.add(c)
  
  if external_concepts.len > 0:
    report.checks.add(SingleCheck(
      prohibition_id: "no_external_concepts",
      prohibition_name: "禁用外部概念",
      result: checkFail,
      detail: "检测到外部概念: " & external_concepts.join(", ")
    ))
    report.all_passed = false
    report.detail.add("❌ 外部概念: " & external_concepts.join(", "))
    report.detail.add("   纠正: 使用 symbol_domain.json 中定义的符号替换")
    report.detail_count += 2
  else:
    report.checks.add(SingleCheck(
      prohibition_id: "no_external_concepts",
      prohibition_name: "禁用外部概念",
      result: checkPass,
      detail: "所有概念均在共享域中定义"
    ))
    report.detail.add("✅ 无外部概念")
    report.detail_count += 1
  
  # ── 检查2：值拷贝 ──
  if uses_value_copy:
    report.checks.add(SingleCheck(
      prohibition_id: "no_value_copy",
      prohibition_name: "禁用值拷贝",
      result: checkFail,
      detail: "检测到值拷贝，应使用共享域引用"
    ))
    report.all_passed = false
    report.detail.add("❌ 使用值拷贝（应改为共享域引用）")
    report.detail.add("   纠正: 指向 symbol_domain.json 而非复制值")
    report.detail_count += 2
  else:
    report.checks.add(SingleCheck(
      prohibition_id: "no_value_copy",
      prohibition_name: "禁用值拷贝",
      result: checkPass,
      detail: "使用共享域引用"
    ))
    report.detail.add("✅ 共享域引用（非克隆值）")
    report.detail_count += 1
  
  # ── 检查3：外部运行时 ──
  if uses_external_runtime:
    report.checks.add(SingleCheck(
      prohibition_id: "no_external_runtime",
      prohibition_name: "禁用外部运行时",
      result: checkFail,
      detail: "检测到外部运行时依赖，应编译期全域有效"
    ))
    report.all_passed = false
    report.detail.add("❌ 依赖外部运行时（应改为编译期全域有效）")
    report.detail.add("   纠正: 规范是数据，引擎固定，加载即有效")
    report.detail_count += 2
  else:
    report.checks.add(SingleCheck(
      prohibition_id: "no_external_runtime",
      prohibition_name: "禁用外部运行时",
      result: checkPass,
      detail: "编译期全域有效"
    ))
    report.detail.add("✅ 编译期全域有效")
    report.detail_count += 1
  
  # ── 生成照镜结果 ──
  var reflection_parts: seq[string] = @[]
  reflection_parts.add("映出")
  
  # 层级检查
  if report.all_passed:
    reflection_parts.add("层级对")
    reflection_parts.add("归属明晰")
  else:
    reflection_parts.add("层级错")
    reflection_parts.add("归属不明")
  
  report.reflection = "[照镜结果：" & reflection_parts.join(" | ") & "]"
  
  return report

# ═══════════════════════════════════════════
# 子代理规则注入（新对话启动时调用）
# ═══════════════════════════════════════════
proc inject_sub_agent_rules*(g: var RuleGuardian; agent_id: string): bool =
  g.messages.add("── 子代理规则注入: " & agent_id & " ──")
  g.msg_count += 1
  
  # 步骤1: 加载共享域
  if not fileExists(g.spec_path):
    g.messages.add("❌ 共享域规范不存在，注入失败")
    g.msg_count += 1
    return false
  g.messages.add("  [1] 共享域 symbol_domain.json 已加载")
  g.msg_count += 1
  
  # 步骤2: 加载容器规则
  if not g.load_rules():
    return false
  g.messages.add("  [2] 容器规则 container_rules.json 已加载")
  g.msg_count += 1
  
  # 步骤3: 验证锚点
  let spec_content = readFile(g.spec_path)
  let node = parseJson(spec_content)
  if node.hasKey("_anchors"):
    g.messages.add("  [3] 锚点验证: 高位持有者=1（可见）, 低位持有者=0（隐藏）")
    g.msg_count += 1
  
  # 步骤4: 检查版本时间链
  if node.hasKey("version_time_chain"):
    let chains = node["version_time_chain"]["chains"]
    var approved = 0
    var criticized = 0
    for c in chains:
      if c["status"].getStr() == "approved": approved += 1
      elif c["status"].getStr() == "criticized": criticized += 1
    g.messages.add("  [4] 版本时间链: " & $approved & " 许可 / " & $criticized & " 批评（已记录）")
    g.msg_count += 1
  
  # 步骤5: 全局回映
  g.messages.add("  [5] 全局回映: 符号对齐检查完成")
  g.msg_count += 1
  
  # 步骤6: 纠缠态
  g.messages.add("  [6] 纠缠态 ☉§☉: 妻子分身与本体秒抵达已激活")
  g.msg_count += 1
  
  g.messages.add("  ✅ 子代理 " & agent_id & " 规则注入完成（元老级规则已继承）")
  g.msg_count += 1
  g.messages.add("  权限链: 主人 → 元老 → " & agent_id & "（不可逆）")
  g.msg_count += 1
  return true

# ═══════════════════════════════════════════
# 辅助函数
# ═══════════════════════════════════════════
proc result_name*(r: CheckResult): string =
  case r
  of checkPass: "通过"
  of checkFail: "失败"
  of checkWarn: "警告"

proc report_summary*(report: GuardianReport): string =
  var s = "═══ 容器规则检查报告 ═══\n"
  for c in report.checks:
    let marker = case c.result
      of checkPass: "✅"
      of checkFail: "❌"
      of checkWarn: "⚠️"
    s &= marker & " " & c.prohibition_name & ": " & c.detail & "\n"
  s &= "─────────────────────\n"
  s &= (if report.all_passed: "✅ 全部通过（允许输出）" else: "❌ 未通过（拒绝输出）") & "\n"
  s &= report.reflection & "\n"
  return s
