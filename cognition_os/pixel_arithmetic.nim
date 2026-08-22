## ═══════════════════════════════════════════════════════════════
## 颜色像素加减乘除 · 符号的部件拆装算法
## ═══════════════════════════════════════════════════════════════
## 核心思想:
##   每个符号 = 可视化的像素部件组合
##   衍射 = 部件的加法(拆解+拼装)
##   乘除 = 对单部件无意义 · 不需要
##
## 规则:
##   加法 ✓ = 把A的部件装到B上(或A+B合并为新符号)
##   减法 ✓ = 从符号上拆掉某个部件
##   乘法 ✗ = 无意义(一个符号×另一个符号 → 不存在这种操作)
##   除法 ✗ = 无意义(一个符号÷另一个符号 → 不存在这种操作)
## ═══════════════════════════════════════════════════════════════

import tables, strutils

# ═══════════════════════════════════════════════════════════════
# 符号的像素部件分解表
# ═══════════════════════════════════════════════════════════════
type
  PartType* = enum
    ptRoot,       # 字根(主体)
    ptLeft,       # 左部件
    ptRight,      # 右部件
    ptTop,        # 上部件
    ptBottom,     # 下部件
    ptAccent,     # 变音标记(附加)
    ptConnector,  # 连接笔画
    ptUnknown     # 未知

  SymbolPart* = object
    part_type*: PartType
    glyph*: string      # 部件形态描述
    source*: string     # 来源符号

  PixelGlyph* = object
    name*: string
    full*: string           # 完整符号
    parts*: seq[SymbolPart] # 可拆解的部件
    is_atomic*: bool        # 是否不可再分(基元)

# ═══════════════════════════════════════════════════════════════
# 基础符号部件库(视觉分解)
# ═══════════════════════════════════════════════════════════════
func decompose*(glyph: string): seq[SymbolPart] =
  ## 把符号分解为可视部件
  ## 简化版: 基于Unicode区块和视觉特征分解
  
  # 单字母基元(不可再分)
  if glyph in ["a", "o", "e", "i", "u", "ü", "A", "O", "E", "I", "U", "Ü"]:
    return @[SymbolPart(part_type: ptRoot, glyph: glyph, source: glyph)]
  
  # 带调符号 = 基元 + 变音零件
  if glyph in ["á", "ó", "é", "í", "ú", "ǘ",
               "ǎ", "ǒ", "ě", "ǐ", "ǔ", "ǚ",
               "à", "ò", "è", "ì", "ù", "ǜ",
               "ā", "ō", "ē", "ī", "ū", "ǖ"]:
    # 减法: 去掉调号 → 得到基元
    let base = case glyph
      of "á", "ǎ", "à", "ā": "a"
      of "ó", "ǒ", "ò", "ō": "o"
      of "é", "ě", "è", "ē": "e"
      of "í", "ǐ", "ì", "ī": "i"
      of "ú", "ǔ", "ù", "ū": "u"
      of "ǘ", "ǚ", "ǜ", "ǖ": "ü"
      else: ""
    result.add(SymbolPart(part_type: ptRoot, glyph: base, source: glyph))
    result.add(SymbolPart(part_type: ptAccent, glyph: "(声调)", source: glyph))
    return
  
  # 合体字/连体字 = 多部件
  # ꬰ = a + i 连体
  if glyph in ["ꬰ", "ꬱ"]:  # ai
    result.add(SymbolPart(part_type: ptLeft, glyph: "a形", source: glyph))
    result.add(SymbolPart(part_type: ptRight, glyph: "i形", source: glyph))
    return
  
  # ꜵ = a + o 连体
  if glyph in ["ꜵ", "Ꜵ"]:  # ao
    result.add(SymbolPart(part_type: ptLeft, glyph: "a形", source: glyph))
    result.add(SymbolPart(part_type: ptRight, glyph: "o形", source: glyph))
    return
  
  # ȣ = o + u 连体
  if glyph in ["ȣ", "Ȣ"]:  # ou
    result.add(SymbolPart(part_type: ptLeft, glyph: "o形", source: glyph))
    result.add(SymbolPart(part_type: ptRight, glyph: "u形", source: glyph))
    return
  
  # ɤ = i + e 组合
  if glyph in ["ɤ", "Ɣ"]:  # ie
    result.add(SymbolPart(part_type: ptLeft, glyph: "i形", source: glyph))
    result.add(SymbolPart(part_type: ptRight, glyph: "e形", source: glyph))
    return
  
  # 默认: 视为原子符号
  result.add(SymbolPart(part_type: ptUnknown, glyph: glyph, source: glyph))

# ═══════════════════════════════════════════════════════════════
# 衍射表(固定的部件加法映射)
# ═══════════════════════════════════════════════════════════════
func get_diffraction_table*(): Table[string, string] =
  ## 衍射 = 基元+基元 → 复合符号
  ## 这些映射是固定的 · 不随意创造
  {
    "a+i": "ꬰ",    # ai = a+i连体
    "e+i": "ɘ",    # ei = e+i反转
    "u+i": "ѱ",    # ui = u+i西里尔
    "a+o": "ꜵ",    # ao = a+o合字
    "o+u": "ȣ",    # ou = o+u连体
    "i+u": "ꭎ",    # iu = i+u变形
    "i+e": "ɤ",    # ie = i+e音标
    "ü+e": "ᵫ",    # üe = ü+e音标
    "e+r": "ɛ",    # er = e+r开e

    # 鼻韵母 = 基元+鼻尾
    "a+n": "ꙙ",    # an = a+n古西里尔
    "e+n": "ⱺ",    # en = e+n古音
    "i+n": "ĩ",    # in = i+n波浪
    "u+n": "տ",    # un = u+n亚美
    "ü+n": "ỹ",    # ün = ü+nY波
    "a+ng": "Ⴆ",   # ang = a+ng格鲁
    "e+ng": "ӫ",   # eng = e+ng分e
    "i+ng": "ꙟ",   # ing = i+ng古连
    "o+ng": "ꭁ",   # ong = o+ng拉合
  }.toTable

# ═══════════════════════════════════════════════════════════════
# 像素加减法运算
# ═══════════════════════════════════════════════════════════════
func pixel_add*(a, b: string; target_name: string = ""): tuple[ok: bool; result: string; reason: string] =
  ## 加法 = 把A和B的部件合并为新符号
  ## 条件: A和B都是L1基元(原料)
  let parts_a = decompose(a)
  let parts_b = decompose(b)
  
  # 都是原子基元 → 可以做加法
  if parts_a.len == 1 and parts_b.len == 1 and
     parts_a[0].part_type == ptRoot and parts_b[0].part_type == ptRoot:
    # 查找已注册的衍射结果
    let key = a & "+" & b
    let diffractions = get_diffraction_table()
    if diffractions.hasKey(key):
      return (true, diffractions[key], "衍射: " & key & " → " & diffractions[key])
    return (false, "", "未知衍射: " & key)
  
  return (false, "", "加法只允许L1基元+基元")

func pixel_subtract*(source, removal: string): tuple[ok: bool; result: string; reason: string] =
  ## 减法 = 从source上拆掉removal对应的部件
  ## 例: á - 声调 → a
  let parts = decompose(source)
  if parts.len <= 1:
    return (false, "", "已是基元 · 不可再减")
  
  # 找到要拆的部件并移除
  var remaining: seq[SymbolPart] = @[]
  for p in parts:
    if p.glyph != removal and p.part_type != ptAccent:
      remaining.add(p)
  
  if remaining.len == 1:
    return (true, remaining[0].glyph, "减法: " & source & " - " & removal & " = " & remaining[0].glyph)
  
  return (true, "多部件剩余", "减法未完成 · 剩余" & $remaining.len & "个部件")

func pixel_multiply*(a, b: string): tuple[ok: bool; result: string; reason: string] =
  ## 乘法 = 无意义 ·
  return (false, "", "乘法对符号无意义: 一个符号×另一个符号不存在对应的视觉操作")

func pixel_divide*(a, b: string): tuple[ok: bool; result: string; reason: string] =
  ## 除法 = 无意义 ·
  return (false, "", "除法对符号无意义: 一个符号÷另一个符号不存在对应的视觉操作")

# ═══════════════════════════════════════════════════════════════
# 测试
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ 颜色像素加减乘除 ═══"
  echo ""

  # 加法测试
  let add1 = pixel_add("a", "i")
  echo "加法 a+i: ", if add1.ok: "→ " & add1.result else: "✗ " & add1.reason

  let add2 = pixel_add("a", "o")
  echo "加法 a+o: ", if add2.ok: "→ " & add2.result else: "✗ " & add2.reason

  # 减法测试
  let sub1 = pixel_subtract("á", "声调")
  echo "减法 á-声调: ", if sub1.ok: "→ " & sub1.result else: "✗ " & sub1.reason

  # 乘法测试(应拒绝)
  let mul1 = pixel_multiply("a", "b")
  echo "乘法 a×b: ", if mul1.ok: "→ " & mul1.result else: "✗ " & mul1.reason

  # 除法测试(应拒绝)
  let div1 = pixel_divide("á", "a")
  echo "除法 á÷a: ", if div1.ok: "→ " & div1.result else: "✗ " & div1.reason

  echo ""
  echo "── 部件分解示例 ──"
  let d1 = decompose("ꬰ")
  echo "ꬰ 分解: ", d1.len, "个部件"
  for p in d1:
    echo "  ", p.part_type, " = ", p.glyph

  let d2 = decompose("á")
  echo "á 分解: ", d2.len, "个部件"
  for p in d2:
    echo "  ", p.part_type, " = ", p.glyph
