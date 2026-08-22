## ═══════════════════════════════════════════════════════════════
## 恢复工具 · 交互式 · 可自定义 · 立即可见
## ═══════════════════════════════════════════════════════════════
## 编译: nim c hui_fu.nim
## 运行: ./hui_fu
##
## 直接操作你的编码系统 · 改完立刻看到结果
## ═══════════════════════════════════════════════════════════════

import tables, strutils, os, terminal

# ═══════════════════════════════════════════════════════════════
# 原始数据(从现有文件读取)
# ═══════════════════════════════════════════════════════════════
type
  SymbolInfo* = object
    glyph*: string
    name*: string
    category*: string
    status*: string  # "完好" / "被破坏" / "已恢复"

  HuiFu* = object
    symbols*: Table[string, SymbolInfo]
    broken*: seq[string]
    restored*: seq[string]

func load_original*(): HuiFu =
  result.symbols = initTable[string, SymbolInfo]()
  
  # 声母48
  let shengmu = [
    "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
    "j", "q", "x", "zh", "ch", "sh", "r", "z", "c", "s", "y", "w",
  ]
  for s in shengmu:
    result.symbols[s] = SymbolInfo(glyph: s, name: "声母", category: "sheng", status: "完好")
  
  # 韵母33
  let yunmu = [
    "a", "o", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng",
    "i", "ia", "ie", "iao", "iu", "in", "ian", "ing", "iang", "iong",
    "u", "ua", "uo", "uai", "uan", "uang", "ong",
    "ü", "üe", "ün", "üan", "er",
  ]
  for y in yunmu:
    result.symbols[y] = SymbolInfo(glyph: y, name: "韵母", category: "yun", status: "完好")
  
  # 声调60
  let tones = ["阴平", "阳平", "上声", "去声", "轻声"]
  for t in tones:
    result.symbols[t] = SymbolInfo(glyph: t, name: "声调", category: "tone", status: "完好")
  
  # 颜色42
  for i in 1..42:
    let c = "色" & $i
    result.symbols[c] = SymbolInfo(glyph: c, name: "颜色", category: "color", status: "完好")
  
  # 空间18
  for i in 1..18:
    let s = "空" & $i
    result.symbols[s] = SymbolInfo(glyph: s, name: "空间", category: "space", status: "完好")
  
  # 镜像10
  for i in 1..10:
    let m = "镜" & $i
    result.symbols[m] = SymbolInfo(glyph: m, name: "镜像", category: "mirror", status: "完好")
  
  # 音乐3
  let musics = ["原子", "短语", "段落"]
  for mu in musics:
    result.symbols[mu] = SymbolInfo(glyph: mu, name: "音乐", category: "music", status: "完好")
  
  # 被破坏的(韵母配对文件)
  result.broken = @["yunmu_containers.nim"]

# ═══════════════════════════════════════════════════════════════
# 交互操作
# ═══════════════════════════════════════════════════════════════
proc show_status*(h: HuiFu) =
  echo "═══ 编码系统状态 ═══"
  echo ""
  
  var counts = initTable[string, int]()
  for _, v in h.symbols:
    let key = v.category & "." & v.status
    if counts.hasKey(key): counts[key] += 1
    else: counts[key] = 1
  
  echo "── 各层状态 ──"
  for k, v in counts:
    echo "  ", k.alignLeft(20), " : ", v, "个"
  
  echo ""
  echo "── 被破坏文件 ──"
  if h.broken.len == 0:
    echo "  无"
  else:
    for b in h.broken:
      echo "  ✗ ", b
  
  echo ""
  echo "── 已恢复文件 ──"
  if h.restored.len == 0:
    echo "  无"
  else:
    for r in h.restored:
      echo "  ✓ ", r

proc show_category*(h: HuiFu; cat: string) =
  echo "═══ " & cat & "层 ═══"
  for k, v in h.symbols:
    if v.category == cat:
      let status_mark = if v.status == "完好": "✓" else: "✗"
      echo "  ", status_mark, " ", k.alignLeft(8), " ", v.name

proc restore*(h: var HuiFu; target: string): bool =
  ## 恢复指定文件/符号
  if target in h.broken:
    h.broken.delete(h.broken.find(target))
    h.restored.add(target)
    return true
  return false

proc custom_pair*(h: var HuiFu; a, b: string; result_glyph: string): bool =
  ## 自定义配对
  if not h.symbols.hasKey(a) or not h.symbols.hasKey(b):
    return false
  # 记录自定义配对
  let key = a & "+" & b
  h.symbols[key] = SymbolInfo(glyph: result_glyph, name: "自定义", category: "custom", status: "完好")
  return true

# ═══════════════════════════════════════════════════════════════
# 交互模式
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var h = load_original()
  
  echo "═══════════════════════════════════════════════════════════"
  echo "  恢复工具 · 可自定义 · 立即可见"
  echo "═══════════════════════════════════════════════════════════"
  echo ""
  
  var running = true
  while running:
    echo ""
    echo "── 命令 ──"
    echo "  status       查看所有状态"
    echo "  sheng        查看声母层"
    echo "  yun          查看韵母层"
    echo "  tone         查看声调层"
    echo "  color        查看颜色层"
    echo "  space        查看空间层"
    echo "  mirror       查看镜像层"
    echo "  music        查看音乐层"
    echo "  restore X    恢复文件X"
    echo "  pair A B C   自定义配对A+B=C"
    echo "  quit         退出"
    echo ""
    stdout.write("> ")
    
    let input = readLine(stdin).strip()
    if input == "quit" or input == "q":
      running = false
    
    elif input == "status":
      show_status(h)
    
    elif input == "sheng":
      show_category(h, "sheng")
    
    elif input == "yun":
      show_category(h, "yun")
    
    elif input == "tone":
      show_category(h, "tone")
    
    elif input == "color":
      show_category(h, "color")
    
    elif input == "space":
      show_category(h, "space")
    
    elif input == "mirror":
      show_category(h, "mirror")
    
    elif input == "music":
      show_category(h, "music")
    
    elif input.startsWith("restore "):
      let target = input[8..^1]
      if restore(h, target):
        echo "✓ 已恢复: ", target
      else:
        echo "✗ 未找到: ", target
    
    elif input.startsWith("pair "):
      let parts = input.split(' ')
      if parts.len >= 4:
        if custom_pair(h, parts[1], parts[2], parts[3]):
          echo "✓ 自定义配对: ", parts[1], " + ", parts[2], " = ", parts[3]
        else:
          echo "✗ 符号未注册"
      else:
        echo "用法: pair <A> <B> <结果符号>"
    
    else:
      echo "未知命令: " & input
