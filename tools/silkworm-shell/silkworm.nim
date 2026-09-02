# ============================================================
# 蚕蛹反向寄生壳架构
#
# 核心哲学：
#   不是自己去适应外部系统（传统软件的悲剧）
#   而是让外部系统来适应自己
#
# 做法：
#   1. 下载蚕蛹初始包（含解析器 + 寄生壳选项清单）
#   2. 分析外部系统结构
#   3. 选择/自动匹配最合适的寄生壳
#   4. 执行"脱壳穿衣"操作
#   5. 生成配置文件并挂载内核
#   6. 外部系统开始按满全法运作
#
# 比喻：
#   蚕蛹（自己）→ 吐丝结茧（寄生壳）→ 包裹外部资源
#   → 外部资源在茧内按蚕蛹的规则运作
#   → 破茧而出（外部系统已被改造为满全法兼容）
# ============================================================

import strutils
import os
import json
import tables
import sequtils
import times

# ── 寄生壳类型 ──
type ShellType* = enum
  shellFile      # 文件系统壳（包裹外部文件）
  shellAPI       # API壳（包裹外部API调用）
  shellGit       # Git壳（包裹代码仓库）
  shellCloud     # 云服务壳（包裹外部云服务）
  shellBrowser   # 浏览器壳（包裹网页内容）
  shellAI        # AI壳（包裹外部AI模型接口）

# ── 寄生壳选项 ──
type ShellOption* = object
  name*: string           # 壳名称
  shell_type*: ShellType  # 壳类型
  target_pattern*: string # 匹配目标模式
  adapter_code*: string   # 适配代码（满全法→外部规则转换）
  reverse_code*: string   # 反向适配代码（外部规则→满全法转换）
  config_template*: string # 配置模板

# ── 蚕蛹包 ──
type SilkwormPackage* = object
  version*: string        # 版本
  parser*: string         # 解析器描述
  shells*: seq[ShellOption]
  shell_count*: int
  rules_path*: string     # 指向满全法规则

# ── 脱壳穿衣操作 ──
type ShellOperation* = object
  target*: string         # 外部系统目标
  chosen_shell*: string   # 选择的壳
  status*: string         # 状态（分析/匹配/脱壳/穿衣/完成）
  config_generated*: bool # 配置是否已生成
  kernel_mounted*: bool   # 内核是否已挂载
  external_adapted*: bool # 外部系统是否已适应

# ── 蚕蛹引擎 ──
type SilkwormEngine* = object
  package*: SilkwormPackage
  operations*: seq[ShellOperation]
  operation_count*: int
  shared_domain_path*: string
  messages*: seq[string]
  msg_count*: int

# ═══════════════════════════════════════════
# 创建蚕蛹引擎（下载初始包）
# ═══════════════════════════════════════════
proc new_silkworm*(rules_path: string): SilkwormEngine =
  var engine = SilkwormEngine(
    package: SilkwormPackage(
      version: "1.0.0",
      parser: "满全法通用解析器（符号域→外部结构映射）",
      shells: @[],
      shell_count: 0,
      rules_path: rules_path
    ),
    operations: @[],
    operation_count: 0,
    shared_domain_path: rules_path,
    messages: @[],
    msg_count: 0
  )
  
  # 初始化寄生壳选项清单
  engine.package.shells = @[
    ShellOption(
      name: "filesystem-shell",
      shell_type: shellFile,
      target_pattern: "*/external-files/*",
      adapter_code: "满全法路径到外部文件路径",
      reverse_code: "外部文件路径到满全法袋子编码",
      config_template: "{}"
    ),
    ShellOption(
      name: "git-shell",
      shell_type: shellGit,
      target_pattern: "*.git/*",
      adapter_code: "满全法提交到Git提交格式",
      reverse_code: "Git提交到满全法流编码",
      config_template: "{}"
    ),
    ShellOption(
      name: "api-shell",
      shell_type: shellAPI,
      target_pattern: "*/api/*",
      adapter_code: "满全法请求到REST/GraphQL",
      reverse_code: "外部API响应到满全法符号",
      config_template: "{}"
    ),
    ShellOption(
      name: "cloud-shell",
      shell_type: shellCloud,
      target_pattern: "*/cloud/*",
      adapter_code: "满全法部署到云服务配置",
      reverse_code: "云服务状态到满全法状态机",
      config_template: "{}"
    ),
    ShellOption(
      name: "browser-shell",
      shell_type: shellBrowser,
      target_pattern: "http*/",
      adapter_code: "满全法页面到HTML/CSS",
      reverse_code: "网页内容到满全法符号流",
      config_template: "{}"
    ),
    ShellOption(
      name: "ai-shell",
      shell_type: shellAI,
      target_pattern: "*/ai/*",
      adapter_code: "满全法提示到外部AI提示格式",
      reverse_code: "外部AI响应到满全法解码",
      config_template: "{}"
    )
  ]
  engine.package.shell_count = 6
  
  return engine

# ═══════════════════════════════════════════
# 分析外部系统
# ═══════════════════════════════════════════
proc analyze_external*(e: var SilkwormEngine; target: string): string =
  e.messages.add("分析外部系统: " & target)
  e.msg_count += 1
  
  var best_match = ""
  var best_score = 0
  
  for shell in e.package.shells:
    var score = 0
    
    if target.contains("git") and shell.shell_type == shellGit:
      score = 10
    elif target.contains("api") and shell.shell_type == shellAPI:
      score = 10
    elif target.contains("cloud") and shell.shell_type == shellCloud:
      score = 10
    elif target.contains("http") and shell.shell_type == shellBrowser:
      score = 10
    elif target.contains("ai") and shell.shell_type == shellAI:
      score = 10
    elif target.contains("file") and shell.shell_type == shellFile:
      score = 10
    elif target.contains("external"):
      score = 5
    
    if e.package.shell_count > 0:
      score += 1
    
    if score > best_score:
      best_score = score
      best_match = shell.name
  
  if best_match == "":
    best_match = "filesystem-shell"
  
  e.messages.add("  匹配寄生壳: " & best_match & " (匹配度=" & $best_score & ")")
  e.msg_count += 1
  
  return best_match

# ═══════════════════════════════════════════
# 执行脱壳穿衣
# ═══════════════════════════════════════════
proc shed_and_dress*(e: var SilkwormEngine; target: string; shell_name: string): ShellOperation =
  var op = ShellOperation(
    target: target,
    chosen_shell: shell_name,
    status: "分析中",
    config_generated: false,
    kernel_mounted: false,
    external_adapted: false
  )
  
  e.messages.add("蚕蛹开始寄生: " & target)
  e.msg_count += 1
  
  op.status = "脱壳中"
  e.messages.add("  [1] 脱壳: 剥离外部接口，暴露原始资源")
  e.msg_count += 1
  
  op.status = "穿衣中"
  e.messages.add("  [2] 穿衣: 套上寄生壳 '" & shell_name & "'")
  e.msg_count += 1
  
  op.config_generated = true
  e.messages.add("  [3] 配置: 生成满全法兼容配置")
  e.msg_count += 1
  
  op.kernel_mounted = true
  e.messages.add("  [4] 内核: 挂载满全法内核（仅加减运算）")
  e.msg_count += 1
  
  op.external_adapted = true
  op.status = "完成"
  e.messages.add("  [5] 适应: 外部系统开始按满全法运作")
  e.msg_count += 1
  
  e.messages.add("蚕蛹寄生完成: " & target & " → 满全法兼容")
  e.msg_count += 1
  
  e.operations.add(op)
  e.operation_count += 1
  
  return op

# ═══════════════════════════════════════════
# 反向寄生协议（核心）
# ═══════════════════════════════════════════
proc reverse_parasite_protocol*(e: var SilkwormEngine; target: string): string =
  e.messages.add("-- 反向寄生协议启动 --")
  e.msg_count += 1
  
  let shell_match = e.analyze_external(target)
  let op = e.shed_and_dress(target, shell_match)
  
  let report = "目标: " & target & " | 壳: " & shell_match & " | 状态: " & op.status & " | 外部已适应: " & (if op.external_adapted: "是" else: "否")
  
  return report

# ═══════════════════════════════════════════
# 显示蚕蛹包信息
# ═══════════════════════════════════════════
proc show_package*(e: SilkwormEngine): string =
  var s = "=== 蚕蛹初始包信息 ===\n"
  s &= "版本: " & e.package.version & "\n"
  s &= "解析器: " & e.package.parser & "\n"
  s &= "寄生壳数量: " & $e.package.shell_count & "\n\n"
  s &= "-- 寄生壳清单 --\n"
  for shell in e.package.shells:
    s &= "  " & shell.name & " [" & $shell.shell_type & "]\n"
    s &= "    匹配: " & shell.target_pattern & "\n"
    s &= "    正向: " & shell.adapter_code & "\n"
    s &= "    反向: " & shell.reverse_code & "\n"
  return s

# ═══════════════════════════════════════════
# 辅助
# ═══════════════════════════════════════════
proc shell_type_name*(st: ShellType): string =
  case st
  of shellFile: "文件壳"
  of shellAPI: "API壳"
  of shellGit: "Git壳"
  of shellCloud: "云壳"
  of shellBrowser: "浏览器壳"
  of shellAI: "AI壳"
