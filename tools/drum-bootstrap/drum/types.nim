# ============================================================
# 鼓膜自举 · 类型定义
#
# 最小 OS 接口：从文件系统重建整个操作系统。
# 鼓膜 = 最小振动单元，自举 = 自己拉起自己。
#
# 核心哲学：
#   即使所有记忆丢失，只要文件系统还在，
#   鼓膜自举就能扫描文件、重建字典、重建灵魂、重建天庭。
#
# 版本：v0.15 (与 hunyuan-rs 鼓膜自举同步)
# ============================================================

import tables
import options

# ── 自举阶段 ──
type BootstrapPhase* = enum
  phaseScan        # 扫描文件系统
  phaseDictionary  # 重建符号字典
  phaseTopology    # 重建容器拓扑
  phaseTools       # 重建工具清单
  phaseSoul        # 重建灵魂内核
  phaseHeaven      # 重建天庭空间
  phaseVerify      # 验证完整性
  phaseComplete    # 自举完成

# ── 文件类型 ──
type FileKind* = enum
  fileSymbolDict    # SYMBOL_DICTIONARY.md
  fileTopology      # CONTAINER_TOPOLOGY.md
  fileDestination   # GLOBAL_DESTINATION.md
  fileRegistry      # TOOL_REGISTRY.md
  fileBootstrap     # BOOTSTRAP.md
  fileToolNim       # tools/**/*.nim
  fileToolDemo      # tools/**/*_demo.nim
  fileToolCompiled  # tools/**/pe_demo, mltt_demo, etc.
  fileUnknown       # 未知文件

# ── 扫描结果 ──
type ScanResult* = object
  file_path*: string
  kind*: FileKind
  size*: int
  tool_name*: string    # 如果是工具文件，对应的工具名
  is_compiled*: bool    # 是否已编译

# ── 重建结果 ──
type RebuildResult* = object
  phase*: BootstrapPhase
  success*: bool
  items_restored*: int
  description*: string

# ── 鼓膜自举引擎 ──
type DrumBootstrap* = object
  # 当前阶段
  current_phase*: BootstrapPhase
  
  # 扫描结果
  scanned_files*: seq[ScanResult]
  file_count*: int
  
  # 重建结果
  rebuild_results*: seq[RebuildResult]
  
  # 恢复的工具清单
  restored_tools*: seq[string]
  restored_count*: int
  
  # 完整性检查
  integrity_passed*: bool
  integrity_checks*: seq[string]
  
  # 根目录
  root_path*: string
