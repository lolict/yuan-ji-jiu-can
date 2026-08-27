## ═══════════════════════════════════════════════════════════════
## chrysalis_shell.nim — 蚕蛹反向寄生壳 · 脱壳穿衣
## ═══════════════════════════════════════════════════════════════
##
## 核心: 下载蚕蛹初始包 · 内含解析器与寄生壳选项清单
##       用户选择或自动匹配寄生壳
##       执行"脱壳穿衣"操作
##       外部系统模仿自身 · 而非自身适应外部规则
##
## 步骤:
##   1. 初始化蚕蛹包(解析器 + 壳选项清单)
##   2. 扫描外部环境(识别可寄生的壳)
##   3. 匹配/选择寄生壳
##   4. 脱壳(去除外壳) → 穿衣(穿上新壳)
##   5. 生成配置文件 · 挂载内核
##   6. 外部系统开始模仿自身
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, tables, strformat]

type
  ShellKind* = enum
    skWeb        # Web 壳 (浏览器/HTTP)
    skMobile     # 移动壳 (Android/iOS)
    skDesktop    # 桌面壳 (Windows/Mac/Linux)
    skCloud      # 云壳 (服务器/容器)
    skIME        # 输入法壳
    skBrowser    # 浏览器扩展壳

  ShellOption* = object
    name*: string
    kind*: ShellKind
    compatible*: bool
    description*: string

  ChrysalisState* = enum
    csInitial     # 初始(蚕蛹)
    csHatching    # 脱壳中
    csDressing    # 穿衣中
    csMounted     # 已挂载(运行中)

  ChrysalisShell* = object
    state*: ChrysalisState
    parser_version*: string
    shell_options*: seq[ShellOption]
    selected_shell*: string
    config*: Table[string, string]
    kernel_mounted*: bool

# ═══════════════════════════════════════════════════════════════
## 初始化蚕蛹包
# ═══════════════════════════════════════════════════════════════

proc new_chrysalis*(): ChrysalisShell =
  result.state = csInitial
  result.parser_version = "1.0.0"
  result.shell_options = @[]
  result.selected_shell = ""
  result.config = init_table[string, string]()
  result.kernel_mounted = false

# ═══════════════════════════════════════════════════════════════
## 注册寄生壳选项
# ═══════════════════════════════════════════════════════════════

proc register_shell*(c: var ChrysalisShell, name: string, kind: ShellKind,
                    compatible: bool, description: string) =
  c.shell_options.add(ShellOption(
    name: name, kind: kind, compatible: compatible, description: description
  ))

# ═══════════════════════════════════════════════════════════════
## 扫描外部环境
# ═══════════════════════════════════════════════════════════════

proc scan_environment*(c: var ChrysalisShell): seq[string] =
  ## 扫描当前环境 · 返回可寄生的壳名称
  for opt in c.shell_options:
    if opt.compatible:
      result.add(opt.name)

# ═══════════════════════════════════════════════════════════════
## 选择寄生壳
# ═══════════════════════════════════════════════════════════════

proc select_shell*(c: var ChrysalisShell, shell_name: string): bool =
  for opt in c.shell_options:
    if opt.name == shell_name and opt.compatible:
      c.selected_shell = shell_name
      return true
  return false

# ═══════════════════════════════════════════════════════════════
## 脱壳穿衣
# ═══════════════════════════════════════════════════════════════

proc molt_and_dress*(c: var ChrysalisShell): bool =
  ## 执行脱壳穿衣操作
  if c.selected_shell == "":
    return false

  c.state = csHatching

  # 脱壳: 去除外壳(移除外部规则依赖)
  c.config["outer_rules"] = "removed"
  c.config["self_rules"] = "preserved"

  c.state = csDressing

  # 穿衣: 穿上新壳(外部系统模仿自身)
  c.config["shell"] = c.selected_shell
  c.config["mode"] = "reverse_parasite"
  c.config["behavior"] = "external_imitates_self"

  c.state = csMounted
  c.kernel_mounted = true
  return true

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(c: ChrysalisShell): string =
  var lines: seq[string] = @[]
  lines.add("═══ 蚕蛹反向寄生壳 · 脱壳穿衣 ═══")
  lines.add("  解析器版本: " & c.parser_version)
  lines.add("  状态: " & $c.state)
  lines.add("  已选壳: " & (if c.selected_shell == "": "未选择" else: c.selected_shell))
  lines.add("  内核挂载: " & (if c.kernel_mounted: "已挂载" else: "未挂载"))
  lines.add("")

  lines.add("── 寄生壳选项清单 ──")
  for opt in c.shell_options:
    let status = if opt.compatible: "✓ 兼容" else: "✗ 不兼容"
    lines.add("  " & opt.name & " [" & $opt.kind & "] " & status & " · " & opt.description)

  if c.config.len > 0:
    lines.add("")
    lines.add("── 配置 ──")
    for k, v in c.config:
      lines.add("  " & k & " = " & v)

  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var c = new_chrysalis()

  echo "═══ 蚕蛹反向寄生壳验证 ═══"
  echo ""

  # 注册寄生壳选项
  c.register_shell("Web容器", skWeb, true, "浏览器环境 · HTTP协议")
  c.register_shell("Android输入法", skIME, true, "Android IME · Kotlin+Rust")
  c.register_shell("Docker容器", skCloud, true, "容器化部署 · 微服务")
  c.register_shell("iOS原生", skMobile, false, "iOS限制 · 暂不兼容")
  c.register_shell("浏览器扩展", skBrowser, true, "Chrome/Firefox扩展")

  echo "── 扫描环境 ──"
  let available = c.scan_environment()
  echo "  可寄生壳: " & available.join(", ")
  echo ""

  echo "── 选择壳: Android输入法 ──"
  if c.select_shell("Android输入法"):
    echo "  ✓ 已选择"
  echo ""

  echo "── 执行脱壳穿衣 ──"
  if c.molt_and_dress():
    echo "  ✓ 脱壳穿衣完成"
    echo "  ✓ 外部系统开始模仿自身"
  echo ""

  echo c.report()

  echo ""
  echo "── 核心验证 ──"
  echo "  蚕蛹初始包: 解析器 + 壳选项清单"
  echo "  脱壳: 去除外壳 · 移除外部规则依赖"
  echo "  穿衣: 穿上新壳 · 外部系统模仿自身"
  echo "  结果: 不是自身适应外部 · 是外部适应自身"
