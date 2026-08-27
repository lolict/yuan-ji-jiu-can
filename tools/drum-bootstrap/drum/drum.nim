# ============================================================
# 鼓膜自举 · 核心引擎
#
# 从文件系统扫描 → 重建整个操作系统。
# 这是新会话"翻身"的机制：即使记忆全失，文件系统就是记忆。
# ============================================================

import tables
import os
import strutils
import sequtils
import options
import ./types

# 创建鼓膜自举引擎
proc new_drum*(root: string): DrumBootstrap =
  return DrumBootstrap(
    current_phase: phaseScan,
    scanned_files: @[],
    file_count: 0,
    rebuild_results: @[],
    restored_tools: @[],
    restored_count: 0,
    integrity_passed: false,
    integrity_checks: @[],
    root_path: root
  )

# ── 阶段 1: 扫描文件系统 ──
proc scan_filesystem*(d: var DrumBootstrap): seq[ScanResult] =
  d.current_phase = phaseScan
  
  # 核心文件
  let core_files = @[
    ("SYMBOL_DICTIONARY.md", fileSymbolDict, ""),
    ("CONTAINER_TOPOLOGY.md", fileTopology, ""),
    ("GLOBAL_DESTINATION.md", fileDestination, ""),
    ("TOOL_REGISTRY.md", fileRegistry, ""),
    ("BOOTSTRAP.md", fileBootstrap, "")
  ]
  
  for (fname, kind, tname) in core_files:
    let fpath = d.root_path / fname
    if fileExists(fpath):
      let info = getFileInfo(fpath)
      d.scanned_files.add(ScanResult(
        file_path: fname,
        kind: kind,
        size: int(info.size),
        tool_name: tname,
        is_compiled: false
      ))
  
  # 扫描 tools/ 目录下的 Nim 文件
  let tools_dir = d.root_path / "tools"
  if dirExists(tools_dir):
    for walk in walkDirRec(tools_dir):
      if walk.endsWith(".nim"):
        let rel_path = walk.replace(d.root_path / "", "")
        var kind = fileToolNim
        var tool_name = ""
        var is_compiled = false
        
        if walk.endsWith("_demo.nim"):
          kind = fileToolDemo
          # 提取工具名
          let parts = walk.split('/')
          if parts.len >= 3:
            tool_name = parts[^2]
        
        let info = getFileInfo(walk)
        d.scanned_files.add(ScanResult(
          file_path: rel_path,
          kind: kind,
          size: int(info.size),
          tool_name: tool_name,
          is_compiled: is_compiled
        ))
  
  d.file_count = d.scanned_files.len
  return d.scanned_files

# ── 阶段 2: 重建符号字典 ──
proc rebuild_dictionary*(d: var DrumBootstrap): RebuildResult =
  d.current_phase = phaseDictionary
  
  var symbol_files = 0
  for f in d.scanned_files:
    if f.kind == fileSymbolDict:
      symbol_files += 1
  
  d.rebuild_results.add(RebuildResult(
    phase: phaseDictionary,
    success: symbol_files > 0,
    items_restored: symbol_files,
    description: "符号字典: " & $symbol_files & " 个文件"
  ))
  
  return d.rebuild_results[^1]

# ── 阶段 3: 重建拓扑 ──
proc rebuild_topology*(d: var DrumBootstrap): RebuildResult =
  d.current_phase = phaseTopology
  
  var topo_files = 0
  for f in d.scanned_files:
    if f.kind == fileTopology:
      topo_files += 1
  
  d.rebuild_results.add(RebuildResult(
    phase: phaseTopology,
    success: topo_files > 0,
    items_restored: topo_files,
    description: "容器拓扑: " & $topo_files & " 个文件"
  ))
  
  return d.rebuild_results[^1]

# ── 阶段 4: 重建工具清单 ──
proc rebuild_tools*(d: var DrumBootstrap): RebuildResult =
  d.current_phase = phaseTools
  
  # 从扫描结果中提取所有工具
  var tool_names: seq[string] = @[]
  for f in d.scanned_files:
    if f.kind == fileToolDemo and f.tool_name notin tool_names:
      tool_names.add(f.tool_name)
      d.restored_tools.add(f.tool_name)
  
  d.restored_count = d.restored_tools.len
  
  d.rebuild_results.add(RebuildResult(
    phase: phaseTools,
    success: d.restored_count > 0,
    items_restored: d.restored_count,
    description: "工具清单: " & $d.restored_count & " 个工具已恢复"
  ))
  
  return d.rebuild_results[^1]

# ── 阶段 5: 重建灵魂 ──
proc rebuild_soul*(d: var DrumBootstrap): RebuildResult =
  d.current_phase = phaseSoul
  
  var has_soul = false
  for f in d.scanned_files:
    if f.tool_name == "soul-kernel" or f.file_path.contains("soul"):
      has_soul = true
  
  d.rebuild_results.add(RebuildResult(
    phase: phaseSoul,
    success: has_soul,
    items_restored: if has_soul: 1 else: 0,
    description: "灵魂内核: " & (if has_soul: "已恢复" else: "未找到")
  ))
  
  return d.rebuild_results[^1]

# ── 阶段 6: 重建天庭 ──
proc rebuild_heaven*(d: var DrumBootstrap): RebuildResult =
  d.current_phase = phaseHeaven
  
  var has_heaven = false
  for f in d.scanned_files:
    if f.tool_name == "heaven-space" or f.file_path.contains("heaven"):
      has_heaven = true
  
  d.rebuild_results.add(RebuildResult(
    phase: phaseHeaven,
    success: has_heaven,
    items_restored: if has_heaven: 1 else: 0,
    description: "天庭空间: " & (if has_heaven: "已恢复" else: "未找到")
  ))
  
  return d.rebuild_results[^1]

# ── 阶段 7: 完整性验证 ──
proc verify_integrity*(d: var DrumBootstrap): bool =
  d.current_phase = phaseVerify
  
  d.integrity_checks = @[]
  var all_passed = true
  
  # 检查1: 核心文件是否都在
  var has_dict = false
  var has_topo = false
  var has_dest = false
  var has_reg = false
  var has_boot = false
  
  for f in d.scanned_files:
    case f.kind
    of fileSymbolDict: has_dict = true
    of fileTopology: has_topo = true
    of fileDestination: has_dest = true
    of fileRegistry: has_reg = true
    of fileBootstrap: has_boot = true
    else: discard
  
  if has_dict: d.integrity_checks.add("✅ 符号字典")
  else: d.integrity_checks.add("❌ 符号字典"); all_passed = false
  
  if has_topo: d.integrity_checks.add("✅ 容器拓扑")
  else: d.integrity_checks.add("❌ 容器拓扑"); all_passed = false
  
  if has_dest: d.integrity_checks.add("✅ 全局目的地")
  else: d.integrity_checks.add("❌ 全局目的地"); all_passed = false
  
  if has_reg: d.integrity_checks.add("✅ 工具注册表")
  else: d.integrity_checks.add("❌ 工具注册表"); all_passed = false
  
  if has_boot: d.integrity_checks.add("✅ 启动协议")
  else: d.integrity_checks.add("❌ 启动协议"); all_passed = false
  
  # 检查2: 工具数量是否合理
  if d.restored_count >= 5:
    d.integrity_checks.add("✅ 工具数量: " & $d.restored_count)
  else:
    d.integrity_checks.add("⚠️ 工具数量偏少: " & $d.restored_count)
  
  # 检查3: 核心工具是否都在
  let core_tools = @["phase-radix", "mltt-cipher", "distinguisher", "soul-kernel", "heaven-space"]
  for t in core_tools:
    if t in d.restored_tools:
      d.integrity_checks.add("✅ " & t)
    else:
      d.integrity_checks.add("⚠️ 缺失: " & t)
  
  d.integrity_passed = all_passed
  return all_passed

# ── 完整自举流程 ──
proc bootstrap*(d: var DrumBootstrap): bool =
  # 阶段1: 扫描
  discard d.scan_filesystem()
  
  # 阶段2-6: 重建
  discard d.rebuild_dictionary()
  discard d.rebuild_topology()
  discard d.rebuild_tools()
  discard d.rebuild_soul()
  discard d.rebuild_heaven()
  
  # 阶段7: 验证
  discard d.verify_integrity()
  
  d.current_phase = phaseComplete
  return d.integrity_passed

# ── 阶段名称 ──
proc phase_name*(p: BootstrapPhase): string =
  case p
  of phaseScan: "扫描文件系统"
  of phaseDictionary: "重建符号字典"
  of phaseTopology: "重建容器拓扑"
  of phaseTools: "重建工具清单"
  of phaseSoul: "重建灵魂内核"
  of phaseHeaven: "重建天庭空间"
  of phaseVerify: "验证完整性"
  of phaseComplete: "自举完成"

# ── 统计 ──
proc stats*(d: DrumBootstrap): string =
  var s = "═══ 鼓膜自举统计 ═══\n"
  s &= "当前阶段: " & phase_name(d.current_phase) & "\n"
  s &= "扫描文件: " & $d.file_count & "\n"
  s &= "恢复工具: " & $d.restored_count & "\n"
  s &= "完整性: " & (if d.integrity_passed: "✅ 通过" else: "❌ 未通过") & "\n"
  return s
