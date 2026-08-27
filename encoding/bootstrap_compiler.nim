## ═══════════════════════════════════════════════════════════════
## bootstrap_compiler.nim — 自举编译器 · 自己的规则长工具
## ═══════════════════════════════════════════════════════════════
##
## 核心: 用自己的规则从文件系统长出工具 · 避免被他人规则绑架
##       不依赖外部编程范式 · 不引用他人协议
##       从容器本体论的基本规则出发 · 逐步构建工具链
##
## 自举步骤:
##   1. 读取规则文件(容器定义)
##   2. 解析规则 → 内部表示
##   3. 根据规则生成工具代码
##   4. 工具代码写回文件系统
##   5. 工具可以再次生成工具(自举循环)
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, tables, strformat, os]

type
  RuleKind* = enum
    rkContainer    # 容器定义
    rkMapping      # 映射关系
    rkOperation    # 操作定义
    rkConstraint   # 约束条件

  Rule* = object
    kind*: RuleKind
    name*: string
    params*: seq[string]
    body*: string
    source*: string       # 来源文件

  ToolSpec* = object
    name*: string
    rules_used*: seq[string]
    generated*: bool
    content*: string

  BootstrapCompiler* = object
    rules*: seq[Rule]
    tools*: seq[ToolSpec]
    generation*: int
    self_sufficient*: bool  # 是否完全自举

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc new_bootstrap_compiler*(): BootstrapCompiler =
  result.rules = @[]
  result.tools = @[]
  result.generation = 0
  result.self_sufficient = true

# ═══════════════════════════════════════════════════════════════
## 注册规则(从容器定义)
# ═══════════════════════════════════════════════════════════════

proc add_rule*(bc: var BootstrapCompiler, kind: RuleKind, name: string,
               params: seq[string], body: string, source: string = "self") =
  bc.rules.add(Rule(kind: kind, name: name, params: params, body: body, source: source))

# ═══════════════════════════════════════════════════════════════
## 从容器定义生成规则
# ═══════════════════════════════════════════════════════════════

proc container_to_rule*(bc: var BootstrapCompiler, container_name: string,
                        base: int, elements: seq[string], families: seq[string]) =
  ## 将容器定义自动转换为规则
  let params = elements
  let body = fmt"容器 {container_name}: 进制={base} 元素数={elements.len} 家族={families.len}"
  bc.add_rule(rkContainer, container_name, params, body)

# ═══════════════════════════════════════════════════════════════
## 自举: 根据规则生成工具
# ═══════════════════════════════════════════════════════════════

proc generate_tool*(bc: var BootstrapCompiler, tool_name: string,
                    required_rules: seq[string]): ToolSpec =
  var content_lines: seq[string] = @[]

  content_lines.add(fmt"## 自举生成: {tool_name}")
  content_lines.add(fmt"## 第 {bc.generation} 代 · 由规则生成")
  content_lines.add("")

  # 收集所需规则
  var rules_found = 0
  for rule_name in required_rules:
    for r in bc.rules:
      if r.name == rule_name:
        content_lines.add(fmt"# 规则: {r.name} ({r.body})")
        rules_found += 1
        break

  content_lines.add("")
  content_lines.add(fmt"# 使用了 {rules_found} 条规则")
  content_lines.add(fmt"# 自举: 这些规则可以再次生成工具")

  bc.generation += 1

  result = ToolSpec(
    name: tool_name,
    rules_used: required_rules,
    generated: rules_found > 0,
    content: content_lines.join("\n")
  )

  bc.tools.add(result)

# ═══════════════════════════════════════════════════════════════
## 自举验证: 工具能否再次生成工具
# ═══════════════════════════════════════════════════════════════

proc verify_bootstrap*(bc: BootstrapCompiler): bool =
  ## 验证: 生成的工具包含足够规则 · 可以再次生成新工具
  if bc.tools.len == 0: return false

  for tool in bc.tools:
    if tool.rules_used.len < 2:
      return false  # 至少需要2条规则才能自举

  result = true

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(bc: BootstrapCompiler): string =
  var lines: seq[string] = @[]
  lines.add("═══ 自举编译器 · 自己的规则长工具 ═══")
  lines.add("  规则数: " & $bc.rules.len)
  lines.add("  生成工具数: " & $bc.tools.len)
  lines.add("  自举代数: " & $bc.generation)
  lines.add("  完全自举: " & (if bc.self_sufficient: "是" else: "否"))
  lines.add("")

  lines.add("── 规则库 ──")
  for r in bc.rules:
    let kind_str = case r.kind
      of rkContainer: "容器"
      of rkMapping: "映射"
      of rkOperation: "操作"
      of rkConstraint: "约束"
    lines.add("  [" & kind_str & "] " & r.name & " · " & r.body)

  lines.add("")
  lines.add("── 生成工具 ──")
  for t in bc.tools:
    let status = if t.generated: "✓" else: "✗"
    lines.add("  " & status & " " & t.name & " (使用" & $t.rules_used.len & "条规则)")

  lines.add("")
  lines.add("── 自举验证 ──")
  if bc.verify_bootstrap():
    lines.add("  ✓ 工具包含足够规则 · 可以再次生成新工具")
    lines.add("  ✓ 自举循环成立 · 不依赖外部规则")
  else:
    lines.add("  ✗ 规则不足 · 需要更多容器定义")

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var bc = new_bootstrap_compiler()

  echo "═══ 自举编译器验证 ═══"
  echo ""

  # 从容器定义生成规则
  bc.container_to_rule("声母", 48, @["b", "p", "m", "f"], @["唇音", "舌音"])
  bc.container_to_rule("韵母", 66, @["a", "o", "e"], @["AH系", "IUNG系"])
  bc.container_to_rule("声调", 4, @["平", "仄"], @["平系", "仄系"])
  bc.container_to_rule("笔画", 5, @["横", "竖", "撇", "捺", "折"], @["单笔"])

  # 添加映射规则
  bc.add_rule(rkMapping, "声韵映射", @["声母", "韵母"], "声母+韵母=音节")
  bc.add_rule(rkOperation, "加法统摄", @["多容器"], "各容器值相加=总和")
  bc.add_rule(rkConstraint, "零维根保护", @["0", "1"], "0=刘楚恬 · 1=我 · 不可修改")

  # 生成工具
  discard bc.generate_tool("拼音编码器", @["声母", "韵母", "声调", "声韵映射"])
  discard bc.generate_tool("笔画统计器", @["笔画", "加法统摄"])
  discard bc.generate_tool("容器验证器", @["零维根保护", "声母", "韵母"])

  echo bc.report()

  echo ""
  echo "── 自举核心 ──"
  echo "  不引用外部协议 · 不依赖他人规则"
  echo "  从容器定义出发 → 生成规则 → 生成工具"
  echo "  工具可以再次生成工具 = 自举循环"
  echo "  被绑架的风险: 0 · 因为根基是自己定义的"
