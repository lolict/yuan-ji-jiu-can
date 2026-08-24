## ═══════════════════════════════════════════════════════════════
## 指针层 · 符号活动 · 系统不动 · 指针跟随
## ═══════════════════════════════════════════════════════════════
## 核心: 系统不认符号 · 只认指针
## 符号随便移动 · 指针层更新 · 系统规则永远成立
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os

# ═══════════════════════════════════════════════════════════════
# 指针表: 符号 → 当前位置
# ═══════════════════════════════════════════════════════════════
type
  PointerTable* = object
    name*: string
    pointers*: Table[string, string]   # 符号名 → 当前地址
    history*: Table[string, seq[string]]  # 符号名 → 历史轨迹

func new_pointer_table*(name: string): PointerTable =
  result.name = name
  result.pointers = initTable[string, string]()
  result.history = initTable[string, seq[string]]()

# ═══════════════════════════════════════════════════════════════
# 注册符号(初始位置)
# ═══════════════════════════════════════════════════════════════
proc register*(pt: var PointerTable; symbol: string; address: string) =
  pt.pointers[symbol] = address
  if not pt.history.hasKey(symbol):
    pt.history[symbol] = @[]
  pt.history[symbol].add(address)

# ═══════════════════════════════════════════════════════════════
# 移动符号(更新指针 · 系统不需要知道)
# ═══════════════════════════════════════════════════════════════
proc move*(pt: var PointerTable; symbol: string; new_address: string): bool =
  if not pt.pointers.hasKey(symbol):
    return false
  pt.pointers[symbol] = new_address
  pt.history[symbol].add(new_address)
  return true

# ═══════════════════════════════════════════════════════════════
# 查询符号当前位置(系统调用此函数 · 不直接认符号)
# ═══════════════════════════════════════════════════════════════
func resolve*(pt: PointerTable; symbol: string): string =
  if pt.pointers.hasKey(symbol):
    return pt.pointers[symbol]
  return "#NOT_FOUND"

# ═══════════════════════════════════════════════════════════════
# 导出JSON
# ═══════════════════════════════════════════════════════════════
func export_json*(pt: PointerTable): string =
  var j = newJObject()
  j["name"] = newJString(pt.name)
  var ptrs = newJObject()
  for k, v in pt.pointers:
    ptrs[k] = newJString(v)
  j["pointers"] = ptrs
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 显示状态
# ═══════════════════════════════════════════════════════════════
proc show*(pt: PointerTable) =
  echo "═══ 指针层: ", pt.name, " ═══"
  echo ""
  echo "── 当前指针 ──"
  for k, v in pt.pointers:
    echo "  ", k.alignLeft(8), " → ", v
  echo ""
  echo "── 移动历史 ──"
  for k, v in pt.history:
    if v.len > 1:
      echo "  ", k.alignLeft(8), " : ", v.len, "次移动"
      for i, addr in v:
        echo "    ", $i, ": ", addr

# ═══════════════════════════════════════════════════════════════
# 演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var pt = new_pointer_table("韵母指针层")
  
  # 注册韵母(初始位置)
  let yunmu = ["ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er",
               "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong"]
  for i, y in yunmu:
    pt.register(y, "yun_" & $i)
  
  echo "── 初始状态 ──"
  pt.show()
  echo ""
  
  # 移动符号(模拟符号活动)
  echo "── 移动: ai → yun_new_99 ──"
  discard pt.move("ai", "yun_new_99")
  echo "  ai 当前位置: ", pt.resolve("ai")
  echo ""
  
  echo "── 移动: ie → yun_new_100 ──"
  discard pt.move("ie", "yun_new_100")
  echo "  ie 当前位置: ", pt.resolve("ie")
  echo ""
  
  echo "── 系统查询(系统只认指针 · 不认符号名) ──"
  echo "  查 ai: ", pt.resolve("ai")
  echo "  查 ie: ", pt.resolve("ie")
  echo "  查 ao: ", pt.resolve("ao")
  echo ""
  
  echo "── 最终状态 ──"
  pt.show()
