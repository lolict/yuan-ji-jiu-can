## ═══════════════════════════════════════════════════════════════
## 全局符号索引表 · 预加载 · 不从头造 · 不取系统符号
## ═══════════════════════════════════════════════════════════════
## 原则:
##   1. 所有符号从这里取 · 不从头创造
##   2. 系统符号(U编码)不在白名单内 · 禁止使用
##   3. 每次写代码前 · 预加载此表
##   4. 新增符号必须经过审判 · 通过后才能入库
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os

# ═══════════════════════════════════════════════════════════════
# 全局符号索引(所有容器的符号 · 唯一来源)
# ═══════════════════════════════════════════════════════════════
type
  SymbolInfo* = object
    glyph*: string          # 符号形态
    container*: string      # 所属容器
    index_0d*: int          # 0维索引(起点位置)
    index_1d*: int          # 1维索引(终点位置)
    frozen*: bool           # 是否冻结(冻结后不可替换)
    owner*: string          # 归属权: SELF/EXTERNAL

  SymbolIndex* = object
    symbols*: Table[string, SymbolInfo]
    by_container*: Table[string, seq[string]]  # 容器 → 符号列表
    count*: int

func new_symbol_index*(): SymbolIndex =
  result.symbols = initTable[string, SymbolInfo]()
  result.by_container = initTable[string, seq[string]]()
  result.count = 0

# ═══════════════════════════════════════════════════════════════
# 添加符号(必须在preload之前)
# ═══════════════════════════════════════════════════════════════
proc add_symbol*(idx: var SymbolIndex; glyph, container: string;
                 idx0d, idx1d: int; frozen: bool) =
  idx.symbols[glyph] = SymbolInfo(
    glyph: glyph,
    container: container,
    index_0d: idx0d,
    index_1d: idx1d,
    frozen: frozen,
    owner: "SELF",
  )
  if not idx.by_container.hasKey(container):
    idx.by_container[container] = @[]
  idx.by_container[container].add(glyph)
  idx.count += 1

# ═══════════════════════════════════════════════════════════════
# 预加载: 所有容器符号一次性入库
# ═══════════════════════════════════════════════════════════════
func preload*(idx: var SymbolIndex) =
  ## 预加载所有符号 · 不从头造 · 不取系统符号
  
  # 声母容器(24基元)
  let shengmu = [
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "w", "j", "q", "x", "y",
    "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
  ]
  for i, s in shengmu:
    idx.add_symbol(s, "声母", i, i, true)

  # 韵母容器(33基元名 · 66符号)
  let yunmu_names = [
    "a", "o", "e", "i", "u", "ü",
    "ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er",
    "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong",
    "ia", "iao", "ua", "uai", "uo", "ian", "uan", "üan", "iang", "uang", "iong",
  ]
  for i, y in yunmu_names:
    idx.add_symbol(y, "韵母", i, i, true)

  # 声调容器(4个 · 贴附层)
  let tones = ["ˉ", "ˊ", "ˇ", "ˋ"]
  for i, t in tones:
    idx.add_symbol(t, "声调", i, i, true)

  # 颜色容器(42相位)
  let colors = [
    "ạ", "a̋", "å", "ã", "â", "ă",
    "ẹ", "e̋", "e̊", "ẽ", "ê", "ĕ",
    "ị", "i̋", "i̊", "ĩ", "î", "ĭ",
    "ọ", "ő", "o̊", "õ", "ô", "ŏ",
    "ụ", "ű", "ů", "ũ", "û", "ŭ",
    "ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "ȁ",
    "ȅ", "ȉ", "ȍ", "ȕ", "ä", "ö",
  ]
  for i, c in colors:
    idx.add_symbol(c, "颜色", i, i, true)

  # 空间容器(18索引)
  let space = [
    "东", "南", "西", "北", "上", "下", "天", "地", "人",
    "前", "后", "左", "右", "中", "内", "外", "高", "低",
  ]
  for i, s in space:
    idx.add_symbol(s, "空间", i, i, true)

  # 音乐容器(3层级)
  let music = ["原子", "短语", "段落"]
  for i, m in music:
    idx.add_symbol(m, "音乐", i, i, true)

# ═══════════════════════════════════════════════════════════════
# 查询: 只从这里取符号
# ═══════════════════════════════════════════════════════════════
func get_symbol*(idx: SymbolIndex; glyph: string): SymbolInfo =
  if idx.symbols.hasKey(glyph):
    return idx.symbols[glyph]
  return SymbolInfo()

func has_symbol*(idx: SymbolIndex; glyph: string): bool =
  return idx.symbols.hasKey(glyph)

func get_by_container*(idx: SymbolIndex; container: string): seq[string] =
  if idx.by_container.hasKey(container):
    return idx.by_container[container]
  return @[]

# ═══════════════════════════════════════════════════════════════
# 审判: 新符号必须经过审判才能入库
# ═══════════════════════════════════════════════════════════════
func can_add*(idx: SymbolIndex; glyph: string): tuple[ok: bool; reason: string] =
  if idx.symbols.hasKey(glyph):
    return (false, "符号已存在")
  # 检查是否是系统符号(U编码范围)
  if glyph.len == 1:
    let cp = ord(glyph[0])
    if cp >= 0x0080:
      return (false, "系统符号禁止入库: U+" & toHex(cp, 4))
  return (true, "允许入库")

# ═══════════════════════════════════════════════════════════════
# 导出JSON
# ═══════════════════════════════════════════════════════════════
func export_json*(idx: SymbolIndex): string =
  var j = newJObject()
  j["total"] = newJInt(idx.count)
  var sym_j = newJObject()
  for k, v in idx.symbols:
    var entry = newJObject()
    entry["container"] = newJString(v.container)
    entry["0d"] = newJInt(v.index_0d)
    entry["1d"] = newJInt(v.index_1d)
    entry["frozen"] = newJBool(v.frozen)
    sym_j[k] = entry
  j["symbols"] = sym_j
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var idx = new_symbol_index()
  idx.preload()
  
  echo "═══ 全局符号索引表 ═══"
  echo "总符号数: ", idx.count
  echo ""
  
  echo "── 各容器统计 ──"
  for container, symbols in idx.by_container:
    echo "  ", container.alignLeft(8), ": ", symbols.len, "个符号"
  echo ""
  
  echo "── 查询示例 ──"
  let s1 = idx.get_symbol("a")
  echo "  查 a: 容器=", s1.container, " 0d=", s1.index_0d, " 1d=", s1.index_1d, " 冻结=", s1.frozen
  
  let s2 = idx.get_symbol("ꬰ")
  echo "  查 ꬰ: ", (if idx.has_symbol("ꬰ"): "存在" else: "不存在(AI发明的 · 不在索引中)")
  echo ""
  
  echo "── 审判新符号 ──"
  let c1 = idx.can_add("x")
  echo "  x: ", c1.reason
  
  let c2 = idx.can_add("ꬰ")
  echo "  ꬰ: ", c2.reason
