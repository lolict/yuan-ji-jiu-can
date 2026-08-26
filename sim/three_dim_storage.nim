## ═══════════════════════════════════════════════════════════════
## three_dim_storage.nim — 三维存储 · 笔画/结构/编码 = 索引维度
## ═══════════════════════════════════════════════════════════════
##
## 核心: 三个索引维度 · 存储即生成 · 生成即输出 · 输出即存储
##       X轴: 笔画数 · Y轴: 结构类型 · Z轴: 编码
##       三维坐标 = 唯一存储位置

## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, algorithm, tables, math]

type
  Coord3D* = object
    x*: int              # 笔画数
    y*: int              # 结构类型(0=独体,1=左右,2=上下,3=包围)
    z*: string           # 编码

  StorageEntry* = object
    symbol*: string
    coord*: Coord3D
    data*: string        # 存储内容
    timestamp*: string

  ThreeDimStorage* = object
    entries*: seq[StorageEntry]
    index_by_coord*: Table[string, int]     # "x,y,z" → index
    index_by_symbol*: Table[string, int]    # symbol → index
    capacity*: int
    generation*: int

# ═══════════════════════════════════════════════════════════════
## 初始化
# ═══════════════════════════════════════════════════════════════

proc newThreeDimStorage*(): ThreeDimStorage =
  result.entries = @[]
  result.index_by_coord = init_table[string, int]()
  result.index_by_symbol = init_table[string, int]()
  result.capacity = 1000
  result.generation = 0

# ═══════════════════════════════════════════════════════════════
## 存储: 三维坐标 = 唯一位置
# ═══════════════════════════════════════════════════════════════

proc store*(tds: var ThreeDimStorage, symbol: string, x, y: int, z, data: string): int =
  let coord_key = $x & "," & $y & "," & z
  
  # 已存在 · 更新
  if tds.index_by_symbol.has_key(symbol):
    let idx = tds.index_by_symbol[symbol]
    tds.entries[idx].data = data
    tds.entries[idx].timestamp = "2026-08-24"
    result = idx
    return
  
  # 新条目
  let entry = StorageEntry(
    symbol: symbol,
    coord: Coord3D(x: x, y: y, z: z),
    data: data,
    timestamp: "2026-08-24"
  )
  
  tds.entries.add(entry)
  let idx = tds.entries.high
  tds.index_by_coord[coord_key] = idx
  tds.index_by_symbol[symbol] = idx
  tds.generation += 1
  result = idx

# ═══════════════════════════════════════════════════════════════
## 查询
# ═══════════════════════════════════════════════════════════════

proc get_by_symbol*(tds: ThreeDimStorage, symbol: string): StorageEntry =
  if tds.index_by_symbol.has_key(symbol):
    result = tds.entries[tds.index_by_symbol[symbol]]

proc get_by_coord*(tds: ThreeDimStorage, x, y: int, z: string): StorageEntry =
  let coord_key = $x & "," & $y & "," & z
  if tds.index_by_coord.has_key(coord_key):
    result = tds.entries[tds.index_by_coord[coord_key]]

# ═══════════════════════════════════════════════════════════════
## 范围查询: 同笔画 · 同结构 · 同编码前缀
# ═══════════════════════════════════════════════════════════════

proc find_by_strokes*(tds: ThreeDimStorage, strokes: int): seq[StorageEntry] =
  for entry in tds.entries:
    if entry.coord.x == strokes:
      result.add(entry)

proc find_by_structure*(tds: ThreeDimStorage, structure: int): seq[StorageEntry] =
  for entry in tds.entries:
    if entry.coord.y == structure:
      result.add(entry)

proc find_by_code_prefix*(tds: ThreeDimStorage, prefix: string): seq[StorageEntry] =
  for entry in tds.entries:
    if entry.coord.z.startsWith(prefix):
      result.add(entry)

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(tds: ThreeDimStorage): string =
  var lines: seq[string] = @[]
  lines.add("═══ 三维存储 · 笔画/结构/编码 ═══")
  lines.add("  条目数: " & $tds.entries.len)
  lines.add("  容量: " & $tds.capacity)
  lines.add("  变异代数: " & $tds.generation)
  lines.add("")
  lines.add("  符号 │ 笔画(X) │ 结构(Y) │ 编码(Z) │ 内容")
  lines.add("  ─────┼─────────┼─────────┼─────────┼──────")
  
  for entry in tds.entries:
    lines.add("  " & entry.symbol & " │ " & $entry.coord.x & " │ " & $entry.coord.y & " │ " & entry.coord.z & " │ " & entry.data)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  var tds = newThreeDimStorage()
  
  echo "═══ 三维存储验证 ═══"
  echo ""
  
  # 存储: 三维坐标 = 唯一位置
  discard tds.store("江", 7, 1, "d71", "河流 · 氵+工")
  discard tds.store("河", 8, 1, "d81", "河流 · 氵+可")
  discard tds.store("湖", 12, 1, "d121", "湖泊 · 氵+胡")
  discard tds.store("海", 11, 1, "d111", "大海 · 氵+每")
  discard tds.store("炎", 8, 2, "d82", "炎热 · 火+火")
  discard tds.store("字", 7, 2, "d72", "文字 · 宀+子")
  discard tds.store("一", 1, 0, "a10", "数字一")
  discard tds.store("人", 2, 0, "c20", "人类")
  
  echo tds.report()
  echo ""
  
  echo "── 查询 ──"
  let jiang = tds.get_by_symbol("江")
  echo "  江: 笔画=" & $jiang.coord.x & " 结构=" & $jiang.coord.y & " 编码=" & jiang.coord.z
  
  let byCoord = tds.get_by_coord(8, 1, "d81")
  echo "  坐标(8,1,d81): " & byCoord.symbol
  
  echo ""
  echo "── 范围查询 ──"
  let stroke8 = tds.find_by_strokes(8)
  echo "  8画: " & stroke8.map(proc(e: StorageEntry): string = e.symbol).join(", ")
  
  let struct1 = tds.find_by_structure(1)
  echo "  左右结构: " & struct1.map(proc(e: StorageEntry): string = e.symbol).join(", ")
  
  let codeD = tds.find_by_code_prefix("d")
  echo "  d编码: " & codeD.map(proc(e: StorageEntry): string = e.symbol).join(", ")