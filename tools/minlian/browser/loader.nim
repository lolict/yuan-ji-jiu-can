# ============================================================
# 民联浏览器 · 真实文档加载器
#
# 从工作区读真实文件:
#   1. tools/*/ 下的主要 .nim 源码 (每个工具一个文档)
#   2. 根目录 *.md 文档 (全局提示 / 目标 / 符号字典等)
#   3. kernel/src/*.c 内核模块
#
# 指纹 = FNV-1a 64-bit (从真实文件内容计算)
# 极性 = 关键词分析 (正向/负向/中性词汇统计)
# 权重 = 文件大小 + 结构复杂度
# ============================================================

import types
import std/os

# ── 路径辅助 ──
func `/`*(a, b: string): string = joinPath(a, b)
import std/strutils
import std/sequtils

# ── FNV-1a 64-bit 哈希 ──
const FNV_OFFSET = 0xcbf29ce484222325'u64
const FNV_PRIME  = 0x100000001b3'u64

func fnv1a_64(data: string): uint64 =
  result = FNV_OFFSET
  for c in data:
    result = result xor uint64(c)
    result = (result * FNV_PRIME) and 0xFFFFFFFFFFFFFFFF'u64

# ── 极性词典 ──
const POSITIVE_KEYS = [
  "完成", "成功", "激活", "推进", "增长", "实现", "确认", "运行",
  "✅", "✓", "胜利", "通过", "启动", "接合", "印证", "开启",
  "解决", "得到", "发现", "证明", "新建", "创建", "触发"
]
const NEGATIVE_KEYS = [
  "失败", "错误", "警告", "问题", "冲突", "禁止", "否定", "溢出",
  "不", "无", "未", "缺", "失", "误", "异", "阻断",
  "作废", "拒绝", "移除", "作废", "溢出", "崩溃"
]

# 极性打分: 正值词 +1, 负值词 -1, 总和决定极性
func estimate_polarity(content: string): int =
  var score = 0
  for kw in POSITIVE_KEYS:
    if content.contains(kw): score += 1
  for kw in NEGATIVE_KEYS:
    if content.contains(kw): score -= 1
  if score > 2: result = 1       # 阳
  elif score < -2: result = -1  # 阴
  else: result = 0              # 中性

# ── 权重估算 ──
func estimate_weight(path: string; content: string): float =
  # 基础权重 = 文件大小 (字节/100)
  var w = float(content.len) / 100.0
  
  # 结构复杂度加分
  let lines = content.splitLines().len
  w += float(lines) * 0.05
  
  # 如果是主要源码 (demo, engine, main 等) 加分
  let fname = path.extractFilename().toLower()
  if fname.contains("engine") or fname.contains("demo") or fname.contains("main"):
    w += 3.0
  elif fname.contains("types") or fname.contains("base"):
    w += 1.0
  
  # MD 文档根目录层级加分
  if path.endsWith(".md") or path.endsWith(".MD"):
    let depth = path.count(PathSep)
    if depth <= 3:
      w += 2.0

  if w < 0.5: w = 0.5
  result = w

# ── 加载真实文档 ──
proc load_real_docs*(root_path: string): seq[Document] =
  var docs: seq[Document] = @[]
  var next_id = 0

  # ── 1. tools/ 下每个子目录的主要 .nim 文件 ──
  let tools_dir = joinPath(root_path, "tools")
  if dirExists(tools_dir):
    for entry in walkDir(tools_dir):
      if entry.kind == pcDir:
        let tool_name = extractFilename(entry.path)
        # 每个工具取第一个 .nim 文件作为代表
        var tool_content = ""
        var tool_path = ""
        var found = false
        for f in walkDir(entry.path):
          if f.kind == pcFile and f.path.endsWith(".nim") and not found:
            try:
              tool_content = readFile(f.path)
              tool_path = f.path
              found = true
            except:
              discard
        
        if found and tool_content.len > 0:
          let hash_val = fnv1a_64(tool_content)
          let pol = estimate_polarity(tool_content)
          let w = estimate_weight(tool_path, tool_content)
          
          docs.add(Document(
            id: next_id,
            title: "tool:" & tool_name,
            content_hash: hash_val,
            polarity: pol,
            weight: w,
            cluster_id: -1,
            wrapped: false
          ))
          inc next_id

  # ── 2. 根目录 *.md 文件 ──
  let md_files = @[
    "GLOBAL_PROMPT_V2.md",
    "GLOBAL_CHAIN_MASTER_LOCK_V2.md", 
    "GLOBAL_INTERLOCK_NEURAL_NETWORK.md",
    "GLOBAL_DESTINATION.md",
    "SYMBOL_DICTIONARY.md",
    "SOVEREIGNTY.md",
    "64_keys.md",
    "judgment_rules.md"
  ]
  for md_name in md_files:
    let md_path = joinPath(root_path, md_name)
    if fileExists(md_path):
      try:
        let content = readFile(md_path)
        if content.len > 0:
          let hash_val = fnv1a_64(content)
          let pol = estimate_polarity(content)
          let w = estimate_weight(md_path, content)

          docs.add(Document(
            id: next_id,
            title: "doc:" & md_name,
            content_hash: hash_val,
            polarity: pol,
            weight: w,
            cluster_id: -1,
            wrapped: false
          ))
          inc next_id
      except:
        discard

  # ── 3. kernel/src/ 下的 .c 文件 ──
  let kernel_dir = joinPath(joinPath(root_path, "kernel"), "src")
  if dirExists(kernel_dir):
    for entry in walkDir(kernel_dir):
      if entry.kind == pcFile and entry.path.endsWith(".c"):
        try:
          let content = readFile(entry.path)
          if content.len > 0:
            let fname = extractFilename(entry.path)
            let hash_val = fnv1a_64(content)
            let pol = estimate_polarity(content)
            let w = estimate_weight(entry.path, content)

            docs.add(Document(
              id: next_id,
              title: "kernel:" & fname,
              content_hash: hash_val,
              polarity: pol,
              weight: w,
              cluster_id: -1,
              wrapped: false
            ))
            inc next_id
        except:
          discard

  result = docs

# ── 加载报告 ──
func load_report*(docs: seq[Document]): string =
  var s = "真实文档加载报告\n"
  s &= "────────────────────────\n"
  s &= "  总文档数: " & $docs.len & "\n\n"
  
  var pos = 0; var neg = 0; var neu = 0
  var tools = 0; var mds = 0; var kernels = 0
  for d in docs:
    if d.polarity > 0: inc pos
    elif d.polarity < 0: inc neg
    else: inc neu
    
    if d.title.startsWith("tool:"): inc tools
    elif d.title.startsWith("doc:"): inc mds
    elif d.title.startsWith("kernel:"): inc kernels
  
  s &= "  来源分布:\n"
  s &= "    工具源码 (tools/): " & $tools & "\n"
  s &= "    全局文档 (*.md):  " & $mds & "\n"
  s &= "    内核模块 (.c):    " & $kernels & "\n\n"
  s &= "  极性分布:\n"
  s &= "    阳 (+): " & $pos & "\n"
  s &= "    阴 (-): " & $neg & "\n"
  s &= "    中性  : " & $neu & "\n"
  s &= "────────────────────────\n"
  result = s
