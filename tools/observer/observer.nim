# ============================================================
# 观测系 · Observer (运行时活码到场指纹)
#
# 核心原则：
#   · 活码 = 在场指纹（哪个符号在哪个地址在场），不是计算数值
#   · 加法统计 = 统计到场密度（多少符号在场），不是做算术运算
#   · 输出格式：「谁在场 · 在哪里 · 什么模式」
#
# 用法：
#   nim c -r tools/observer/observer.nim
#   nim c -r tools/observer/observer.nim --html
# ============================================================

import os
import strutils
import times
import sequtils
import json
import tables

# ============================================================
# 根目录检测
# ============================================================
let ROOT_DIR = getAppDir().parentDir().parentDir()
let TOOLS_DIR = ROOT_DIR & "/tools"
let KERNEL_DIR = ROOT_DIR & "/kernel"
let ZIG_STEM_DIR = ROOT_DIR & "/zig-stem"
let RUST_DIR = ROOT_DIR & "/hunyuan-rs"
let OUTPUT_DIR = ZIG_STEM_DIR & "/output"

# ============================================================
# 在场指纹类型
# ============================================================
type
  PresencePattern = enum
    ppToolBinary,      # 已编译工具产物
    ppToolSource,      # 工具源码
    ppZigEngine,       # Zig 引擎模块
    ppKernelC,         # C 内核文件
    ppRustModule,      # Rust 模块
    ppSymbolDef,       # 符号定义
    ppExportWasm,      # WASM 导出
    ppDoc,             # 文档/宪法
    ppEncoding,        # 编码系统文件

  PresenceFingerprint = object
    symbol: string      # 符号名称（谁在场）
    address: string     # 文件路径（在哪里）
    pattern: PresencePattern  # 什么模式
    active: bool        # 是否激活（有内容/已编译）

  ObservationReport = object
    timestamp: string
    total_present: int           # 总到场符号数
    densities: Table[PresencePattern, int]  # 各模式到场密度
    fingerprints: seq[PresenceFingerprint]  # 完整到场清单

# ============================================================
# 模式转可读名
# ============================================================
proc pattern_name(p: PresencePattern): string =
  case p
  of ppToolBinary: "已编译工具"
  of ppToolSource: "工具源码"
  of ppZigEngine: "Zig引擎"
  of ppKernelC: "C内核"
  of ppRustModule: "Rust模块"
  of ppSymbolDef: "符号定义"
  of ppExportWasm: "WASM导出"
  of ppDoc: "文档"
  of ppEncoding: "编码系统"

# ============================================================
# 扫描：到场指纹采集
# ============================================================
proc collect_presence(): ObservationReport =
  var report = ObservationReport(
    timestamp: format(now(), "yyyy-MM-dd HH:mm:ss"),
    fingerprints: @[],
    densities: initTable[PresencePattern, int]()
  )

  # 初始化所有模式密度为 0
  for p in PresencePattern:
    report.densities[p] = 0

  # ── 0. 编码系统（宪法级）──
  let encoding_files = @[
    "GLOBAL_DESTINATION.md", "SYMBOL_DICTIONARY.md",
    "GLOBAL_CHAIN_MASTER_LOCK_V2.md", "GLOBAL_PROMPT_V2.md",
    "GLOBAL_INTERLOCK_NEURAL_NETWORK.md", "SYMBOL_CONSTITUTION.md",
    "CONTAINER-ONTOLOGY.md", "MANQUANFA-JUDGE.md"
  ]
  for ef in encoding_files:
    let p = ROOT_DIR & "/" & ef
    if fileExists(p):
      report.fingerprints.add(PresenceFingerprint(
        symbol: ef, address: p, pattern: ppDoc, active: true))
      report.densities[ppDoc] += 1

  # ── 1. 工具源码 + 已编译产物 ──
  if dirExists(TOOLS_DIR):
    for kind, path in walkDir(TOOLS_DIR):
      if kind != pcDir: continue
      let tool_name = extractFilename(path)

      # 检查源码语言
      var has_nim = false
      var has_c = false
      var has_compiled = false

      for _, fpath in walkDir(path):
        let fname = fpath.extractFilename()
        let fext = fpath.splitFile().ext
        if fext == ".nim": has_nim = true
        if fext == ".c" or fext == ".h": has_c = true
        if fext == "" and fname != tool_name and fname != "src":
          if fpUserExec in fpath.getFileInfo().permissions:
            has_compiled = true

      if has_nim:
        report.fingerprints.add(PresenceFingerprint(
          symbol: tool_name, address: path, pattern: ppToolSource, active: true))
        report.densities[ppToolSource] += 1
      if has_c:
        report.fingerprints.add(PresenceFingerprint(
          symbol: tool_name & "_c", address: path, pattern: ppToolSource, active: true))
        report.densities[ppToolSource] += 1
      if has_compiled:
        report.fingerprints.add(PresenceFingerprint(
          symbol: tool_name, address: path, pattern: ppToolBinary, active: true))
        report.densities[ppToolBinary] += 1

  # ── 2. Zig 引擎模块 ──
  if dirExists(ZIG_STEM_DIR):
    for kind, fpath in walkDir(ZIG_STEM_DIR):
      if kind != pcFile: continue
      let fname = fpath.extractFilename()
      if fname.endsWith(".zig") and fname != "lib.zig" and fname != "build.zig" and fname != "main.zig":
        report.fingerprints.add(PresenceFingerprint(
          symbol: fname, address: fpath, pattern: ppZigEngine, active: true))
        report.densities[ppZigEngine] += 1
      elif fname == "main.c":
        report.fingerprints.add(PresenceFingerprint(
          symbol: "main.c", address: fpath, pattern: ppKernelC, active: true))
        report.densities[ppKernelC] += 1

  # ── 3. C 内核文件 ──
  if dirExists(KERNEL_DIR):
    for kind, fpath in walkDir(KERNEL_DIR):
      if kind != pcDir: continue
      if extractFilename(fpath) == "src":
        for _, srcfile in walkDir(fpath):
          if srcfile.extractFilename().endsWith(".c"):
            report.fingerprints.add(PresenceFingerprint(
              symbol: srcfile.extractFilename(),
              address: srcfile, pattern: ppKernelC, active: true))
            report.densities[ppKernelC] += 1

  # ── 4. Rust 模块 ──
  if dirExists(RUST_DIR):
    let cargo = RUST_DIR & "/Cargo.toml"
    if fileExists(cargo):
      report.fingerprints.add(PresenceFingerprint(
        symbol: "hunyuan-rs", address: cargo, pattern: ppRustModule, active: true))
      report.densities[ppRustModule] += 1
    let src_dir = RUST_DIR & "/src"
    if dirExists(src_dir):
      for kind, fpath in walkDir(src_dir):
        if kind == pcFile and fpath.extractFilename().endsWith(".rs"):
          report.fingerprints.add(PresenceFingerprint(
            symbol: fpath.extractFilename(),
            address: fpath, pattern: ppRustModule, active: true))
          report.densities[ppRustModule] += 1

  # ── 5. 符号定义（SYMBOL_DICTIONARY 行数） ──
  let sym_dict = ROOT_DIR & "/SYMBOL_DICTIONARY.md"
  if fileExists(sym_dict):
    let lines = readFile(sym_dict).splitLines()
    var sym_count = 0
    for line in lines:
      let s = line.strip()
      if s.startsWith("|") and not s.contains("---") and not s.contains("符号"):
        sym_count += 1
    for i in 0..<min(sym_count, 50):
      report.fingerprints.add(PresenceFingerprint(
        symbol: "sym_" & $(i+1), address: sym_dict, pattern: ppSymbolDef, active: i < sym_count))
    report.densities[ppSymbolDef] = sym_count

  # ── 6. WASM 导出（通过 strings 读取） ──
  let wasm32_path = ZIG_STEM_DIR & "/zig-out/wasm32/release/mqf32.wasm"
  let wasm64_path = ZIG_STEM_DIR & "/zig-out/wasm64/release/mqf64.wasm"
  for wpath in [wasm32_path, wasm64_path]:
    if fileExists(wpath):
      report.fingerprints.add(PresenceFingerprint(
        symbol: extractFilename(wpath),
        address: wpath, pattern: ppExportWasm, active: true))
      report.densities[ppExportWasm] += 1

  # ── 计算总到场数（加法统计：直接计数，非幂次运算） ──
  var total = 0
  for p in PresencePattern:
    total += report.densities[p]
  report.total_present = total

  return report

# ============================================================
# 输出：纯到场记录
# ============================================================
proc output_text(report: ObservationReport) =
  echo "══════════════════════════════════════════════════════════════"
  echo "  观测系 · 活码到场指纹"
  echo "  观测时刻: ", report.timestamp
  echo "══════════════════════════════════════════════════════════════"
  echo ""

  # 密度总览（加法统计：每类直接报数，无运算）
  echo "┌─ 到场密度 ─────────────────────────────────────────┐"
  echo "│"
  echo "│  总到场符号数: ", $report.total_present
  echo "│"
  for p in PresencePattern:
    let d = report.densities[p]
    if d > 0:
      echo "│  ", pattern_name(p), ": ", $d, " 个在场"
  echo "│"
  echo "└────────────────────────────────────────────────────┘"
  echo ""

  # 逐条到场记录（按模式分组）
  for p in PresencePattern:
    let group = report.fingerprints.filterIt(it.pattern == p)
    if group.len == 0: continue
    echo "─── ", pattern_name(p), " (", $group.len, " 个) ───"
    for fp in group:
      let status = if fp.active: "●" else: "○"
      let short_addr = fp.address.replace(ROOT_DIR, "")
      echo "  ", status, " ", fp.symbol, " → ", short_addr
    echo ""

  echo "    ── 观测完成 · 身份落款: 刘楚恬和我 ──"

# ============================================================
# 输出：HTML
# ============================================================
proc output_html(report: ObservationReport, path: string) =
  var html = """
<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>观测系 · 活码到场指纹</title>
<style>
  body { font-family: monospace; background: #0a0a0a; color: #e0e0e0; margin: 40px; }
  h1 { color: #00ffaa; }
  .panel { background: #111; border: 1px solid #333; border-radius: 6px; padding: 16px; margin: 12px 0; }
  .present { color: #00ffaa; }
  .absent { color: #555; }
  .group { margin: 8px 0; }
  .symbol { color: #ffaa00; }
  .path { color: #888; font-size: 12px; }
  td, th { padding: 4px 12px; text-align: left; border-bottom: 1px solid #222; }
  th { color: #ffaa00; background: #1a1a1a; }
  .footer { margin-top: 32px; color: #666; border-top: 1px solid #333; padding-top: 12px; }
</style>
</head>
<body>
<h1>☉ 观测系 · 活码到场指纹</h1>
<p>观测时刻: $ts</p>

<div class="panel">
<h2>到场密度（加法统计）</h2>
<table>
<tr><th>模式</th><th>在场数</th><th>在场率</th></tr>
"""

  for p in PresencePattern:
    let d = report.densities[p]
    if d > 0:
      html &= "<tr><td>" & pattern_name(p) & "</td><td class=\"present\">" & $d & "</td><td>" & pattern_name(p) & "</td></tr>\n"

  html &= "<tr><td><b>总计</b></td><td class=\"present\"><b>" & $report.total_present & "</b></td><td>（加法统计 · 无溢出）</td></tr>\n"
  html &= "</table></div>\n"

  for p in PresencePattern:
    let group = report.fingerprints.filterIt(it.pattern == p)
    if group.len == 0: continue
    html &= "<div class=\"panel\">\n<h2>" & pattern_name(p) & " (" & $group.len & " 个在场)</h2>\n"
    for fp in group:
      let status = if fp.active: "<span class=\"present\">●</span>" else: "<span class=\"absent\">○</span>"
      let short_addr = fp.address.replace(ROOT_DIR, "")
      html &= "<div class=\"group\">" & status & " <span class=\"symbol\">" & fp.symbol & "</span> → <span class=\"path\">" & short_addr & "</span></div>\n"
    html &= "</div>\n"

  html &= """
<div class="footer">
  ── 刘楚恬和我 · Hunyuan-OS 观测系 ──<br>
  原则：活码=在场指纹 · 加法统计=到场密度 · 非数值运算
</div>
</body>
</html>
"""

  html = html.replace("$ts", report.timestamp)
  writeFile(path, html)

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
        html_output = OUTPUT_DIR & "/observer_report.html"

  echo "☉ 启动观测系..."
  echo ""

  let report = collect_presence()

  echo "  ✓ 采集到场指纹: ", $report.total_present, " 条"
  echo "  ✓ 文档到场: ", $report.densities[ppDoc]
  echo "  ✓ 工具源码: ", $report.densities[ppToolSource], " · 已编译: ", $report.densities[ppToolBinary]
  echo "  ✓ Zig 引擎: ", $report.densities[ppZigEngine], " · C内核: ", $report.densities[ppKernelC]
  echo "  ✓ Rust 模块: ", $report.densities[ppRustModule], " · 符号定义: ", $report.densities[ppSymbolDef]
  echo ""

  if html_output.len > 0:
    output_html(report, html_output)
    echo "  ✓ HTML 报告: ", html_output
  else:
    output_text(report)

  echo ""

when isMainModule:
  main()
