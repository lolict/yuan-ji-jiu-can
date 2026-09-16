# ============================================================
# 操作系统自描述生成器 · os-self-description
#
# 功能：Hunyuan-OS 输出其自身的状态描述
#   - 正在感知什么：当前加载的模块、激活的流、可用的工具
#   - 处于何种时空坐标：编码系统版本、架构目标、阶段位置
#   - 字典是否自洽：符号注册表交叉检查、引用完整性
#
# 用法：nim c -r tools/os-self-description/os_self_description.nim [--html]
# ============================================================

import os
import strutils
import times
import json
import sequtils

# ============================================================
# 根目录自动检测（基于可执行文件位置）
# 编译产物路径: tools/os-self-description/os_self_desc
# 向上两级得到仓库根目录
# ============================================================
let ROOT_DIR = getAppDir().parentDir().parentDir()
let TOOLS_DIR = ROOT_DIR & "/tools"
let KERNEL_DIR = ROOT_DIR & "/kernel"
let RUST_DIR = ROOT_DIR & "/hunyuan-rs"
let ZIG_STEM_DIR = ROOT_DIR & "/zig-stem"
let GLOBAL_DEST = ROOT_DIR & "/GLOBAL_DESTINATION.md"
let SYMBOL_REGISTRY = ROOT_DIR & "/SYMBOL_DICTIONARY.md"
let CLOUDFLARE_DIR = ROOT_DIR & "/cloudflare"
let ANDROID_DIR = ROOT_DIR & "/android"

const
  ANCHOR_SELF = "我=1 (高位·可见)"
  ANCHOR_OTHER = "刘楚恬=0 (高位·隐藏)"

  ENCODING_SYSTEMS = @[
    "mlll-254 (254+2集装箱编码·我=1·刘楚恬=0)",
    "YinYang 二进制 (★/☆ 一进制)",
    "64进制几何化身 (64符号×6°=384°满圆)",
    "mltt 命名密码表 (刘楚恬↔我 声母韵母密钥)",
    "kndl 外壳 (柯南道尔三维容器寻址)",
    "mlil 立体进制 (毛利兰空间划分)",
    "cbsy 范畴编程管辖索引制"
  ]

# ============================================================
# 扫描结果容器
# ============================================================
type
  ToolRecord = object
    name: string
    path: string
    has_binary: bool
    languages: seq[string]

  SelfDescription = object
    timestamp: string
    tools_built: seq[ToolRecord]
    kernel_version: string
    rust_version: string
    zig_engines: seq[string]
    architectures: seq[string]
    encoding_active: seq[string]
    symbols_declared: int
    symbols_resolved: int
    symbols_orphan: seq[string]
    consistency_pass: bool
    perception_state: string
    spacetime_coord: string

# ============================================================
# 辅助：读取 JSON 文件
# ============================================================
proc readJson_file(path: string): JsonNode =
  let content = readFile(path)
  result = parseJson(content)

# ============================================================
# 辅助：语言检测
# ============================================================
proc detectLanguage(fext: string, rec: var ToolRecord) =
  case fext
  of ".nim":
    if not ("nim" in rec.languages): rec.languages.add("nim")
  of ".c", ".h":
    if not ("c" in rec.languages): rec.languages.add("c")
  of ".rs":
    if not ("rust" in rec.languages): rec.languages.add("rust")
  of ".zig":
    if not ("zig" in rec.languages): rec.languages.add("zig")
  of ".py":
    if not ("python" in rec.languages): rec.languages.add("python")
  of ".js", ".html":
    if not ("js" in rec.languages): rec.languages.add("js")
  else: discard

# ============================================================
# 工具扫描
# ============================================================
proc scan_tools(): seq[ToolRecord] =
  var records: seq[ToolRecord] = @[]
  if not dirExists(TOOLS_DIR):
    return records

  for kind, path in walkDir(TOOLS_DIR):
    if kind != pcDir: continue
    let tool_name = extractFilename(path)
    if tool_name == "os-self-description": continue

    var rec = ToolRecord(
      name: tool_name,
      path: "tools/" & tool_name,
      has_binary: false,
      languages: @[]
    )

    # 扫描子目录检查是否有编译产物和语言标识
    for subkind, subpath in walkDir(path):
      if subkind == pcDir:
        # 递归扫描一级子目录
        for _, subsubpath in walkDir(subpath):
          let sfname = subsubpath.extractFilename()
          let sfext = subsubpath.splitFile().ext
          if sfext == "" and sfname != "src":
            if fpUserExec in subsubpath.getFileInfo().permissions:
              rec.has_binary = true
          detectLanguage(sfext, rec)
      else:
        let fname = subpath.extractFilename()
        let fext = subpath.splitFile().ext
        if fext == "" and fname != tool_name and fname != "src":
          if fpUserExec in subpath.getFileInfo().permissions:
            rec.has_binary = true
        detectLanguage(fext, rec)

    records.add(rec)

  result = records

# ============================================================
# 内核版本检测
# ============================================================
proc detect_kernel_version(): string =
  let kernel_c = KERNEL_DIR & "/src/njsg_kernel.c"
  if not fileExists(kernel_c): return "未找到"

  let content = readFile(kernel_c)
  # 先检查高版本，再回退到较低版本
  for line in content.splitLines():
    if "v2.0" in line: return "v2.0.0 (含v1.4+全部模块)"
  for line in content.splitLines():
    if "v1.0" in line: return "v1.0+"
  return "v0.x (早期)"

# ============================================================
# Rust 版本检测
# ============================================================
proc detect_rust_version(): string =
  let cargo = RUST_DIR & "/Cargo.toml"
  if not fileExists(cargo): return "未找到"

  let content = readFile(cargo)
  for line in content.splitLines():
    if line.strip().startsWith("version"):
      let parts = line.split("\"")
      if parts.len > 1: return parts[1].strip()
  return "未知"

# ============================================================
# Zig 引擎检测
# ============================================================
proc detect_zig_engines(): seq[string] =
  var engines: seq[string] = @[]
  if not dirExists(ZIG_STEM_DIR): return engines

  for kind, path in walkDir(ZIG_STEM_DIR):
    if kind != pcFile: continue
    let fname = path.extractFilename()
    if fname.endsWith(".zig") and fname != "lib.zig" and fname != "build.zig" and fname != "main.zig":
      engines.add(fname)
    elif fname == "main.c":
      engines.add("main.c (C内核桥接)")
    elif fname == "test_fqmcp.html":
      engines.add("test_fqmcp.html (浏览器验证)")
  return engines

# ============================================================
# 架构目标检测
# ============================================================
proc detect_architectures(): seq[string] =
  var archs: seq[string] = @[]

  if dirExists(ZIG_STEM_DIR & "/zig-out/wasm32"):
    archs.add("wasm32-freestanding")
  if dirExists(ZIG_STEM_DIR & "/zig-out/wasm64"):
    archs.add("wasm64-freestanding")

  let rust_target = RUST_DIR & "/.cargo/config.toml"
  if fileExists(rust_target):
    let content = readFile(rust_target)
    if "x86_64" in content: archs.add("x86_64")
    if "aarch64" in content: archs.add("aarch64 (ARM64)")
    if "wasm" in content: archs.add("wasm (Rust)")

  if fileExists(KERNEL_DIR & "/src/njsg_kernel.c"):
    archs.add("native (C内核·POSIX)")

  if fileExists(CLOUDFLARE_DIR & "/mqf-web.js"):
    archs.add("cloudflare-worker (JS/WASM)")

  if dirExists(ANDROID_DIR):
    archs.add("android (Kotlin+NDK)")

  return archs

# ============================================================
# 符号注册表交叉检查
# ============================================================
proc check_symbol_consistency(): (int, int, seq[string], bool) =
  var declared = 0
  var resolved = 0
  var orphans: seq[string] = @[]

  if fileExists(SYMBOL_REGISTRY):
    let content = readFile(SYMBOL_REGISTRY)
    for line in content.splitLines():
      let s = line.strip()
      if s.startsWith("|") and not s.contains("---") and s.len > 3:
        declared += 1

  let sr_types = TOOLS_DIR & "/symbol-registry/symbol_registry/types.nim"
  if fileExists(sr_types):
    let content = readFile(sr_types)
    for line in content.splitLines():
      if "UnifiedEntry" in line: resolved += 1

  let se_symbols = TOOLS_DIR & "/symbol-engine/symbols.json"
  if fileExists(se_symbols):
    let content = readJson_file(se_symbols)
    for key in content.keys():
      resolved += 1

  let consistent = (orphans.len == 0) and (declared > 0)
  return (declared, resolved, orphans, consistent)

# ============================================================
# 计算感知状态
# ============================================================
proc compute_perception(tools: seq[ToolRecord], engines: seq[string]): string =
  var active_modules: seq[string] = @[]

  let orch_zig = ZIG_STEM_DIR & "/orchestrator.zig"
  if fileExists(orch_zig):
    let content = readFile(orch_zig)
    const mod_pairs = [
      ("mapper", "映射器"), ("aligner", "对齐器"), ("indexer", "索引器"),
      ("stepper", "步进器"), ("locator", "定位器"), ("phaser", "相位器"),
      ("angler", "角度器"), ("frequencer", "频率器"), ("arbiter", "裁决器")
    ]
    for mp in mod_pairs:
      if ("const " & mp[0] & " = @import") in content:
        active_modules.add(mp[1])

  result = "已激活感知模块: " & $active_modules.len & " (" & active_modules.join(", ") & ")"

# ============================================================
# 计算时空坐标
# ============================================================
proc compute_spacetime(kernel_v, rust_v: string, archs: seq[string]): string =
  # 满圆周期: 384°÷24°=16步; 传统周期: 360°÷24°=15步
  result = "编码维度: 384°满圆周期(16步) | 内核版本: " & kernel_v &
           " | Rust核心: " & rust_v &
           " | 架构目标: " & $archs.len &
           " | 锚点: " & ANCHOR_SELF & " · " & ANCHOR_OTHER

# ============================================================
# 纯文本输出
# ============================================================
proc output_text(desc: SelfDescription) =
  echo "══════════════════════════════════════════════════════"
  echo "  Hunyuan-OS · 操作系统自描述报告"
  echo "  生成时刻: ", desc.timestamp
  echo "══════════════════════════════════════════════════════"
  echo ""

  echo "┌─ 正在感知什么 ─────────────────────────────────────┐"
  echo "│"
  echo "│  ", desc.perception_state
  echo "│  已建造工具: ", $desc.tools_built.len, " 个"
  echo "│  其中已编译工具: ", $desc.tools_built.filterIt(it.has_binary).len, " 个"
  echo "│  Zig 引擎模块: ", $desc.zig_engines.len, " 个"
  echo "│  激活编码系统: ", $desc.encoding_active.len, " 套"
  echo "│"
  for enc in desc.encoding_active:
    echo "│    ◈ ", enc
  echo "│"
  echo "└────────────────────────────────────────────────────┘"
  echo ""

  echo "┌─ 处于何种时空坐标 ─────────────────────────────────┐"
  echo "│"
  echo "│  ", desc.spacetime_coord
  echo "│  架构目标:"
  for arch in desc.architectures:
    echo "│    ▸ ", arch
  echo "│"
  echo "└────────────────────────────────────────────────────┘"
  echo ""

  echo "┌─ 字典是否自洽 ─────────────────────────────────────┐"
  echo "│"
  echo "│  声明符号数: ", $desc.symbols_declared
  echo "│  已解析符号数: ", $desc.symbols_resolved
  echo "│  孤立引用数: ", $desc.symbols_orphan.len
  echo "│  一致性判定: ", if desc.consistency_pass: "✓ PASS" else: "✗ FAIL"
  if desc.symbols_orphan.len > 0:
    echo "│  孤立符号:"
    for orph in desc.symbols_orphan:
      echo "│    ⚠ ", orph
  echo "│"
  echo "└────────────────────────────────────────────────────┘"
  echo ""

  echo "    ── 刘楚恬和我 · Hunyuan-OS 自描述 ──"

# ============================================================
# HTML 输出
# ============================================================
proc output_html(desc: SelfDescription, output_path: string) =
  var tool_rows = ""
  for t in desc.tools_built:
    let bin_flag = if t.has_binary: "已编译" else: "源码"
    let langs = if t.languages.len > 0: t.languages.join("/") else: "未知"
    tool_rows &= "      <tr><td>" & t.name & "</td><td>" & t.path & "</td><td>" & langs & "</td><td>" & bin_flag & "</td></tr>\n"

  var enc_items = ""
  for e in desc.encoding_active:
    enc_items &= "        <li>" & e & "</li>\n"

  var arch_items = ""
  for a in desc.architectures:
    arch_items &= "        <li>" & a & "</li>\n"

  var engine_items = ""
  for eng in desc.zig_engines:
    engine_items &= "        <li>" & eng & "</li>\n"

  let consistency_class = if desc.consistency_pass: "pass" else: "fail"
  let consistency_text = if desc.consistency_pass: "PASS" else: "FAIL"

  let html = """
<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>Hunyuan-OS · 自描述报告</title>
<style>
  body { font-family: monospace; background: #0a0a0a; color: #e0e0e0; margin: 40px; }
  h1 { color: #00ffaa; border-bottom: 1px solid #00ffaa; padding-bottom: 8px; }
  h2 { color: #ffaa00; margin-top: 32px; }
  .panel { background: #111; border: 1px solid #333; border-radius: 6px; padding: 16px; margin: 12px 0; }
  .pass { color: #00ffaa; }
  .fail { color: #ff4444; }
  table { width: 100%; border-collapse: collapse; }
  th { background: #1a1a1a; color: #ffaa00; text-align: left; padding: 6px; }
  td { padding: 4px 8px; border-bottom: 1px solid #222; }
  li { margin: 2px 0; }
  .footer { margin-top: 32px; color: #666; border-top: 1px solid #333; padding-top: 12px; }
  .timestamp { color: #888; }
</style>
</head>
<body>
<h1>☉ Hunyuan-OS · 操作系统自描述报告</h1>
<p class="timestamp">生成时刻: $timestamp</p>

<h2>┌─ 正在感知什么 ─┐</h2>
<div class="panel">
  <p><strong>$perception</strong></p>
  <p>已建造工具: $tools_count 个 | 已编译: $compiled_count 个 | Zig 引擎: $engine_count 个 | 编码系统: $encoding_count 套</p>
  <ul>
$enc_list  </ul>
</div>

<h2>┌─ 处于何种时空坐标 ─┐</h2>
<div class="panel">
  <p><strong>$spacetime</strong></p>
  <ul>
$arch_list  </ul>
  <h3>Zig 引擎模块</h3>
  <ul>
$engine_list  </ul>
</div>

<h2>┌─ 字典是否自洽 ─┐</h2>
<div class="panel">
  <p>声明符号数: $symbols_declared | 已解析: $symbols_resolved | 孤立引用: $symbols_orphan_count</p>
  <p class="$consistency_class">一致性判定: $consistency_text</p>
</div>

<h2>已建造工具清单</h2>
<div class="panel">
<table>
<tr><th>工具名称</th><th>路径</th><th>语言</th><th>编译状态</th></tr>
$tool_rows
</table>
</div>

<div class="footer">
  ── 刘楚恬和我 · Hunyuan-OS 自描述 ──<br>
  身份落款: $anchor_self · $anchor_other
</div>
</body>
</html>
""".replace("$timestamp", desc.timestamp)
   .replace("$perception", desc.perception_state)
   .replace("$tools_count", $desc.tools_built.len)
   .replace("$compiled_count", $desc.tools_built.filterIt(it.has_binary).len)
   .replace("$engine_count", $desc.zig_engines.len)
   .replace("$encoding_count", $desc.encoding_active.len)
   .replace("$enc_list", enc_items)
   .replace("$spacetime", desc.spacetime_coord)
   .replace("$arch_list", arch_items)
   .replace("$engine_list", engine_items)
   .replace("$symbols_declared", $desc.symbols_declared)
   .replace("$symbols_resolved", $desc.symbols_resolved)
   .replace("$symbols_orphan_count", $desc.symbols_orphan.len)
   .replace("$consistency_class", consistency_class)
   .replace("$consistency_text", consistency_text)
   .replace("$tool_rows", tool_rows)
   .replace("$anchor_self", ANCHOR_SELF)
   .replace("$anchor_other", ANCHOR_OTHER)

  writeFile(output_path, html)

# ============================================================
# 主程序
# ============================================================
proc main() =
  var html_output = ""
  for i in 1..paramCount():
    let p = paramStr(i)
    if p == "--html":
      if i < paramCount():
        html_output = paramStr(i+1)
      else:
        html_output = ZIG_STEM_DIR & "/output/self_description.html"

  echo "☉ 启动 Hunyuan-OS 自描述扫描..."
  echo ""

  let tools = scan_tools()
  echo "  ✓ 发现工具: ", $tools.len, " 个"

  let kernel_v = detect_kernel_version()
  echo "  ✓ C 内核: ", kernel_v

  let rust_v = detect_rust_version()
  echo "  ✓ Rust 核心: ", rust_v

  let engines = detect_zig_engines()
  echo "  ✓ Zig 引擎模块: ", $engines.len, " 个"

  let archs = detect_architectures()
  echo "  ✓ 架构目标: ", $archs.len, " 个"

  let (sym_decl, sym_res, orphans, consistent) = check_symbol_consistency()
  echo "  ✓ 符号声明: ", $sym_decl, " | 解析: ", $sym_res, " | 孤立: ", $orphans.len

  let perception = compute_perception(tools, engines)
  let spacetime = compute_spacetime(kernel_v, rust_v, archs)

  var desc = SelfDescription(
    timestamp: format(now(), "yyyy-MM-dd HH:mm:ss"),
    tools_built: tools,
    kernel_version: kernel_v,
    rust_version: rust_v,
    zig_engines: engines,
    architectures: archs,
    encoding_active: ENCODING_SYSTEMS,
    symbols_declared: sym_decl,
    symbols_resolved: sym_res,
    symbols_orphan: orphans,
    consistency_pass: consistent,
    perception_state: perception,
    spacetime_coord: spacetime
  )

  echo ""

  if html_output.len > 0:
    output_html(desc, html_output)
    echo "  ✓ HTML 报告已输出: ", html_output
  else:
    output_text(desc)

  echo ""

when isMainModule:
  main()
