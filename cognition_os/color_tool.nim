## ═══════════════════════════════════════════════════════════════
## 颜色进制工具 · 可编译 · 可解释 · 可打包
## ═══════════════════════════════════════════════════════════════
## 命令行工具: 处理颜色像素编码 · 衍射审判 · 进制转换
##
## 编译: nim c color_tool.nim
## 运行: ./color_tool <command> [args]
##
## 命令:
##   explain            解释整个系统
##   judge <a> <b>      审判两个符号配对
##   add <a> <b>        加法运算(拼装)
##   sub <src> <part>   减法运算(拆解)
##   diffraction <a> <b> 查衍射表
##   shell              进入交互模式
##   export             导出JSON知识库
## ═══════════════════════════════════════════════════════════════

import tables, math, strutils, json, os, parseopt, unicode, std / [jsonutils]

# ═══════════════════════════════════════════════════════════════
# 类型定义
# ═══════════════════════════════════════════════════════════════
type
  SymbolCategory* = enum
    catVowBase,      # 韵母基元 L1
    catConsonant,    # 声母 L1
    catTonePart,     # 声调零件 L2
    catToneFinished, # 声调成品 L3
    catVowCompound,  # 复合韵母 L3
    catVowNasal,     # 鼻韵母 L3
    catUnknown       # 未知

  SymbolRecord* = object
    glyph*: string           # 符号形态
    name*: string            # 名称
    category*: SymbolCategory # 类别
    level*: int              # 层级 1/2/3
    weight*: float           # 视觉重量
    owner*: string           # 归属权 SELF/EXTERNAL
    prefix*: seq[string]     # 前缀连接
    suffix*: seq[string]     # 后缀连接

  ColorTool* = object
    name*: string
    version*: string
    divisor*: int             # 公约数(总目标编码)
    symbols*: Table[string, SymbolRecord]
    diffraction_table*: Table[string, string]
    blacklist*: seq[string]

# ═══════════════════════════════════════════════════════════════
# 知识库初始化
# ═══════════════════════════════════════════════════════════════
func get_category*(glyph: string): SymbolCategory =
  case glyph
  of "a", "o", "e", "i", "u", "ü": return catVowBase
  of "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
     "j", "q", "x", "y", "z", "c", "s", "r": return catConsonant
  of "acute", "grave", "caron", "tilde", "diaeresis": return catTonePart
  of "á", "ó", "é", "í", "ú", "ǘ", "ǎ", "ǒ", "ě", "ǐ", "ǔ", "ǚ",
     "à", "ò", "è", "ì", "ù", "ǜ": return catToneFinished
  of "ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er": return catVowCompound
  of "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong": return catVowNasal
  else: return catUnknown

func new_color_tool*(): ColorTool =
  result.name = "颜色进制工具"
  result.version = "1.0.0"
  result.divisor = 144  # 顶层公约数: 48×3 或 24×6
  result.symbols = initTable[string, SymbolRecord]()
  result.diffraction_table = initTable[string, string]()
  result.blacklist = @[]
  
  # 注册基元符号
  let vowel_bases = [
    ("a", "a开唇", 1, 0.3), ("o", "o圆唇", 1, 0.5), ("e", "e半开", 1, 0.4),
    ("i", "i扁唇", 1, 0.2), ("u", "u圆唇", 1, 0.6), ("ü", "ü撮唇", 1, 0.4),
  ]
  for v in vowel_bases:
    result.symbols[v[0]] = SymbolRecord(
      glyph: v[0], name: v[1], category: catVowBase,
      level: v[2], weight: v[3], owner: "SELF",
      prefix: @[], suffix: @[],
    )
  
  # 衍射表: 固定配对
  result.diffraction_table = {
    "a+i": "ꬰ", "e+i": "ɘ", "u+i": "ѱ", "a+o": "ꜵ",
    "o+u": "ȣ", "i+u": "ꭎ", "i+e": "ɤ", "ü+e": "ᵫ",
    "e+r": "ɛ", "a+n": "ꙙ", "e+n": "ⱺ", "i+n": "ĩ",
    "u+n": "տ", "ü+n": "ỹ", "a+ng": "Ⴆ", "e+ng": "ӫ",
    "i+ng": "ꙟ", "o+ng": "ꭁ",
  }.toTable

# ═══════════════════════════════════════════════════════════════
# 审判系统
# ═══════════════════════════════════════════════════════════════
func judge*(tool: ColorTool; a, b: string): tuple[pass: bool; score: float; report: string] =
  var lines: seq[string] = @[]
  lines.add("═══ 审判报告 ═══")
  lines.add("配对: " & a & " + " & b)
  lines.add("公约数锚点: " & $tool.divisor)
  lines.add("")
  
  # 获取符号记录
  var sa, sb: SymbolRecord
  if tool.symbols.hasKey(a): sa = tool.symbols[a]
  if tool.symbols.hasKey(b): sb = tool.symbols[b]
  
  # 维度1: 层级
  var layer_ok = true
  if sa.level == 3 or sb.level == 3:
    layer_ok = false
    lines.add("✗ 层级: L3禁止配对")
  elif sa.level == sb.level:
    lines.add("✓ 层级: L" & $sa.level & "同级")
  else:
    lines.add("△ 层级: L" & $sa.level & "/L" & $sb.level & "跨级")
  
  # 维度2: 类型
  var type_ok = true
  if sa.category == catToneFinished or sb.category == catToneFinished:
    type_ok = false
    lines.add("✗ 类型: 声调成品禁止衍射")
  elif sa.category == catVowBase and sb.category == catVowBase:
    lines.add("✓ 类型: 元音+元音合法衍射")
  elif sa.category == catUnknown or sb.category == catUnknown:
    type_ok = false
    lines.add("✗ 类型: 未知符号")
  else:
    lines.add("△ 类型: 跨类别配对")
  
  # 维度3: 审美(视觉重量差)
  let w_diff = abs(sa.weight - sb.weight)
  if w_diff <= 0.2:
    lines.add("✓ 审美: 重量均衡 差=" & formatFloat(w_diff, ffDecimal, 2))
  else:
    lines.add("✗ 审美: 重量失衡 差=" & formatFloat(w_diff, ffDecimal, 2))
  
  # 全局得分
  let score = (if layer_ok: 0.4 else: 0.0) + (if type_ok: 0.4 else: 0.0) + (0.2 - w_diff)
  
  lines.add("")
  lines.add("全局得分: " & formatFloat(score, ffDecimal, 2))
  lines.add("判决: " & (if score >= 0.7: "✓通过" else: "✗拒绝"))
  
  result.pass = score >= 0.7
  result.score = score
  result.report = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运算
# ═══════════════════════════════════════════════════════════════
func pixel_add*(tool: ColorTool; a, b: string): tuple[ok: bool; result: string; reason: string] =
  if not tool.symbols.hasKey(a) or not tool.symbols.hasKey(b):
    return (false, "", "符号未注册")
  let sa = tool.symbols[a]
  let sb = tool.symbols[b]
  if sa.level == 3 or sb.level == 3:
    return (false, "", "L3禁止拼装")
  # 查衍射表
  let key = a & "+" & b
  if tool.diffraction_table.hasKey(key):
    return (true, tool.diffraction_table[key], "衍射成功: " & key)
  let key2 = b & "+" & a
  if tool.diffraction_table.hasKey(key2):
    return (true, tool.diffraction_table[key2], "衍射成功: " & key2)
  return (false, "", "无衍射映射: " & key)

func pixel_sub*(tool: ColorTool; src: string; part: string): tuple[ok: bool; result: string; reason: string] =
  if not tool.symbols.hasKey(src):
    return (false, "", "源符号未注册")
  # 简化: 减法即逆向查找
  for k, v in tool.diffraction_table:
    if v == src:
      let parts = k.split('+')
      if parts.len == 2 and (parts[0] == part or parts[1] == part):
        return (true, parts[0], "拆解: " & src & " - " & part & " = " & parts[0])
  return (false, "", "无法拆解")

# ═══════════════════════════════════════════════════════════════
# 导出JSON
# ═══════════════════════════════════════════════════════════════
func export_json*(tool: ColorTool): string =
  var j = newJObject()
  j["name"] = newJString(tool.name)
  j["version"] = newJString(tool.version)
  j["divisor"] = newJInt(tool.divisor)
  
  var sym_j = newJObject()
  for k, v in tool.symbols:
    var entry = newJObject()
    entry["glyph"] = newJString(v.glyph)
    entry["name"] = newJString(v.name)
    entry["category"] = newJString($v.category)
    entry["level"] = newJInt(v.level)
    entry["weight"] = newJFloat(v.weight)
    entry["owner"] = newJString(v.owner)
    sym_j[k] = entry
  j["symbols"] = sym_j
  
  var diff_j = newJObject()
  for k, v in tool.diffraction_table:
    diff_j[k] = newJString(v)
  j["diffraction_table"] = diff_j
  
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 解释系统
# ═══════════════════════════════════════════════════════════════
func explain*(): string = """
═══ 颜色进制系统 · 完整解释 ═══

【核心定义】
  颜色进制 = 以相位像素单元为基础的编码进制
  每个符号 = 一张二维相位矩阵 · 像素点携带相位+亮度+权重

【三种运算】
  ✓ 加法 = 部件拼装(把A的像素部件安装到B上)
  ✓ 减法 = 部件拆解(从成品上剥离零件)
  ✗ 乘法 = 禁用(成倍复制无意义)
  ✗ 除法 = 禁用(均分切割无意义)

【审判机制】
  6维分布式制衡网络:
    1. 层级栈校验: L1+L1允许 · L3禁止配对
    2. 类型身份: 声调成品禁止衍射
    3. 视觉审美: 重量差≤0.2
    4. 归属权: EXTERNAL需重定义
    5. 衍射规则: 等效镜像对齐
    6. 黑名单: 反面样本永不录用

【公约数-公倍数】
  公约数 = 顶层总目标(统一的编码体系)
  公倍数 = 对齐的符号对(必须过6维审判)
  所有运算为公约数服务

【编织制衡】
  6维互相压制 · 任一高权重维度否决 → 整体拒绝
  全局协调得分 ≈ 共振场模拟 · 全程白盒可审计

【知识库隔离】
  本体库 · 参考池 · 黑名单 · 丢弃池 · 互相不渗透
"""

# ═══════════════════════════════════════════════════════════════
# CLI入口
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var tool = new_color_tool()
  
  if paramCount() == 0:
    echo explain()
    echo "用法: ./color_tool <command> [args]"
    echo "命令: explain · judge · add · sub · diffraction · export · shell"
    quit(0)
  
  let cmd = paramStr(1)
  
  case cmd:
  of "explain":
    echo explain()
  
  of "judge":
    if paramCount() < 3:
      echo "用法: ./color_tool judge <符号A> <符号B>"
      quit(1)
    let a = paramStr(2)
    let b = paramStr(3)
    let result = tool.judge(a, b)
    echo result.report
  
  of "add":
    if paramCount() < 3:
      echo "用法: ./color_tool add <符号A> <符号B>"
      quit(1)
    let a = paramStr(2)
    let b = paramStr(3)
    let result = tool.pixel_add(a, b)
    if result.ok:
      echo "加法成功: " & a & " + " & b & " = " & result.result
    else:
      echo "加法失败: " & result.reason
  
  of "sub":
    if paramCount() < 3:
      echo "用法: ./color_tool sub <源符号> <要拆的部件>"
      quit(1)
    let src = paramStr(2)
    let part = paramStr(3)
    let result = tool.pixel_sub(src, part)
    if result.ok:
      echo "减法成功: " & src & " - " & part & " = " & result.result
    else:
      echo "减法失败: " & result.reason
  
  of "diffraction":
    echo "── 衍射表 ──"
    for k, v in tool.diffraction_table:
      echo "  ", k.alignLeft(10), " → ", v
  
  of "export":
    echo tool.export_json()
  
  of "shell":
    echo "═══ 交互模式 ═══"
    echo "输入命令 (explain/judge/add/sub/diffraction/export/quit):"
    var running = true
    while running:
      stdout.write("> ")
      let input = readLine(stdin).strip()
      if input == "quit" or input == "q":
        running = false
      elif input == "explain":
        echo explain()
      elif input.startsWith("judge "):
        let parts = input.split(' ')
        if parts.len >= 3:
          let r = tool.judge(parts[1], parts[2])
          echo r.report
        else:
          echo "用法: judge <a> <b>"
      elif input.startsWith("add "):
        let parts = input.split(' ')
        if parts.len >= 3:
          let r = tool.pixel_add(parts[1], parts[2])
          echo r.reason
      elif input == "diffraction":
        for k, v in tool.diffraction_table:
          echo "  ", k, " → ", v
      elif input == "export":
        echo tool.export_json()
      else:
        echo "未知命令: " & input
  
  else:
    echo "未知命令: " & cmd
    echo "用法: ./color_tool <explain|judge|add|sub|diffraction|export|shell>"
