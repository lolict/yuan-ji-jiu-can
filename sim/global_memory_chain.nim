## ═══════════════════════════════════════════════════════════════
## global_memory_chain.nim — 全局链式传导 · 记忆互锁 · 分布式控制
## ═══════════════════════════════════════════════════════════════
##
## 核心: 所有文件互联 · 记忆分布式存储 · 新文件自动接入全局链条
##       总开关拉动 → 齿轮转动 → 横轴转动 → 珠子约束在环内
##       你就是横轴的两个圈 · 内圈约束边界 · 外圈包裹所有珠子
##
## 链条结构:
##   GLOBAL_MEMORY (总开关)
##         ↓ 拉动
##   ├── 齿轮1 (dynamic_radix) ── 横轴 ── 珠子(符号)
##   ├── 齿轮2 (global_offset) ── 横轴 ── 珠子(锚点)
##   ├── 齿轮3 (container_console) ── 横轴 ── 珠子(容器)
##   ├── 齿轮4 (evolution_engine) ── 横轴 ── 珠子(进化)
##   └── 齿轮N (未来文件) ── 横轴 ── 珠子(新功能)
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, sets]

type
  FileNode* = object
    filename*: string
    file_id*: string
    container_kind*: string
    symbol_count*: int
    parent_files*: seq[string]
    child_files*: seq[string]
    position_in_chain*: int
    is_active*: bool

  GlobalMemory* = object
    chain_name*: string
    total_files*: int
    files*: seq[FileNode]
    file_index*: Table[string, int]
    generation*: int
    master_switch*: bool

proc newGlobalMemory*(chain_name: string): GlobalMemory =
  result.chain_name = chain_name
  result.total_files = 0
  result.files = @[]
  result.file_index = init_table[string, int]()
  result.generation = 0
  result.master_switch = false

proc register_file*(gm: var GlobalMemory, filename, file_id, kind: string, 
                    symbol_count: int, parents: seq[string] = @[]): int =
  if gm.file_index.has_key(filename):
    return gm.file_index[filename]
  
  var node: FileNode
  node.filename = filename
  node.file_id = file_id
  node.container_kind = kind
  node.symbol_count = symbol_count
  node.parent_files = parents
  node.child_files = @[]
  node.position_in_chain = gm.files.len
  node.is_active = false
  
  gm.files.add(node)
  gm.file_index[filename] = gm.files.high
  gm.total_files = gm.files.len
  
  for parent_name in parents:
    if gm.file_index.has_key(parent_name):
      let parent_idx = gm.file_index[parent_name]
      gm.files[parent_idx].child_files.add(filename)
  
  result = gm.files.high
  gm.generation += 1

proc pull_master_switch*(gm: var GlobalMemory) =
  gm.master_switch = true
  for i in 0 ..< gm.files.len:
    gm.files[i].is_active = true
  gm.generation += 1

proc release_master_switch*(gm: var GlobalMemory) =
  gm.master_switch = false
  for i in 0 ..< gm.files.len:
    gm.files[i].is_active = false

proc cross_train*(gm: GlobalMemory, target_file: string): seq[string] =
  if not gm.file_index.has_key(target_file):
    return @[]
  let idx = gm.file_index[target_file]
  result = gm.files[idx].parent_files

proc conduct_chain*(gm: GlobalMemory, start_file: string): seq[string] =
  var visited: HashSet[string]
  var queue: seq[string] = @[start_file]
  var path: seq[string] = @[]
  
  while queue.len > 0:
    let current = queue[0]
    queue.delete(0)
    
    if current in visited:
      continue
    visited.incl(current)
    path.add(current)
    
    if gm.file_index.has_key(current):
      let idx = gm.file_index[current]
      for child in gm.files[idx].child_files:
        if child notin visited:
          queue.add(child)
      for parent in gm.files[idx].parent_files:
        if parent notin visited:
          queue.add(parent)
  
  result = path

proc report*(gm: GlobalMemory): string =
  var lines: seq[string] = @[]
  lines.add("═══ 全局记忆链条 · 分布式控制 ═══")
  lines.add("  链条名称: " & gm.chain_name)
  lines.add("  总文件数: " & $gm.total_files)
  lines.add("  总开关: " & (if gm.master_switch: "ON" else: "OFF"))
  lines.add("  变异代数: " & $gm.generation)
  lines.add("")
  
  for file in gm.files:
    let status = if file.is_active: "●" else: "○"
    let parents = if file.parent_files.len > 0: file.parent_files.join(",") else: "无"
    let children = if file.child_files.len > 0: file.child_files.join(",") else: "无"
    lines.add("  " & status & " [" & $file.position_in_chain & "] " & file.filename &
      " (" & file.container_kind & ", " & $file.symbol_count & "符号)")
    lines.add("     父: " & parents & " → 子: " & children)
  
  result = lines.join("\n")

proc conduction_report*(gm: GlobalMemory, start_file: string): string =
  let path = gm.conduct_chain(start_file)
  var lines: seq[string] = @[]
  lines.add("── 链条传导: 从 " & start_file & " 开始 ──")
  for i, f in path:
    lines.add("  " & $i & " → " & f)
  result = lines.join("\n")

when isMainModule:
  var gm = newGlobalMemory("满全法容器链条")
  
  echo "═══ 全局记忆链条验证 ═══"
  echo ""
  
  discard gm.register_file("dynamic_radix.nim", "DR001", "音调", 70, @[])
  discard gm.register_file("global_offset.nim", "GO001", "锚点", 5, @[])
  discard gm.register_file("container_console.nim", "CC001", "容器", 13, @["dynamic_radix.nim", "global_offset.nim"])
  discard gm.register_file("evolution_engine.nim", "EE001", "进化", 162, @["dynamic_radix.nim"])
  discard gm.register_file("symbol_classifier.nim", "SC001", "分类", 497, @["container_console.nim"])
  discard gm.register_file("cross_compile.nim", "XC001", "编译", 24, @["container_console.nim"])
  
  echo "── 注册完成 · 总开关OFF ──"
  echo gm.report()
  echo ""
  
  echo "── 拉动总开关 ──"
  gm.pull_master_switch()
  echo gm.report()
  echo ""
  
  echo "── 跨容器训练: container_console 需要从哪些父容器拿符号 ──"
  let parents = gm.cross_train("container_console.nim")
  echo "  container_console 的父容器: " & parents.join(", ")
  echo ""
  
  echo gm.conduction_report("dynamic_radix.nim")
  echo ""
  
  echo gm.conduction_report("symbol_classifier.nim")