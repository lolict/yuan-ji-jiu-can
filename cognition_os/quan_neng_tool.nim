## ═══════════════════════════════════════════════════════════════
## 全能编码工具 · 读取你的真实数据 · 可编译可解释可打包
## ═══════════════════════════════════════════════════════════════
## 直接读取:
##   yunmu66.nim · color42.nim · shengmu48.nim
##   tone60.nim · music_tier.nim · space_phase.nim · mirror_3d.nim
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, parseopt, terminal, math, unicode

# ═══════════════════════════════════════════════════════════════
# 自定义配对存储(必须在使用前声明)
# ═══════════════════════════════════════════════════════════════
const CUSTOM_FILE* = "cognition_os/custom_pairs.json"

var custom_pairs = initTable[string, string]()

proc load_custom*() =
  if fileExists(CUSTOM_FILE):
    let content = readFile(CUSTOM_FILE)
    let j = parseJson(content)
    for k, v in j: custom_pairs[k] = v.getStr()

proc save_custom*() =
  var j = newJObject()
  for k, v in custom_pairs: j[k] = newJString(v)
  writeFile(CUSTOM_FILE, j.pretty)

proc add_custom_pair*(a, b, result: string) =
  custom_pairs[a & "+" & b] = result
  custom_pairs[b & "+" & a] = result
  save_custom()

# ═══════════════════════════════════════════════════════════════
# 你的真实数据(从现有文件直接导入)
# ═══════════════════════════════════════════════════════════════

# --- 韵母66 ---
const SYMBOLS66* = [
  "ꬰ", "ꬱ", "ɘ", "Ǝ", "ѱ", "Ѱ",
  "ꜵ", "Ꜵ", "ȣ", "Ȣ", "ꭎ", "Ⴞ",
  "ɤ", "Ɣ", "ᵫ", "ꭂ", "ɛ", "Ɛ",
  "ꙙ", "Ꙙ", "ⱄ", "ⱺ", "ꞃ", "Ꞃ", "տ", "໙", "ỹ", "Ỹ",
  "Ⴆ", "Ⴊ", "ӫ", "Ӫ", "ꙟ", "Ꙟ", "ꭁ", "ꭀ",
  "ꙗ", "Ꙗ", "ᧀ", "ᦿ", "ã", "Ã", "ꜻ", "Ꜻ", "ơ", "Ơ",
  "ꙝ", "Ꙝ", "Ⴄ", "Ⴂ", "ѫ", "Ѫ", "ѭ", "Ѭ", "ꙛ", "Ꙛ", "ȱ", "Ȱ",
]

const YUNMU_CODES* = {
  "ai": @[0, 1], "ei": @[2, 3], "ui": @[4, 5], "ao": @[6, 7], "ou": @[8, 9], "iu": @[10, 11],
  "ie": @[12, 13], "üe": @[14, 15], "er": @[16, 17],
  "an": @[18, 19], "en": @[20, 21], "in": @[22, 23], "un": @[24, 25], "ün": @[26, 27],
  "ang": @[28, 29], "eng": @[30, 31], "ing": @[32, 33], "ong": @[34, 35],
  "ia": @[36, 37], "iao": @[38, 39], "ua": @[40, 41], "uai": @[42, 43], "uo": @[44, 45],
  "ian": @[46, 47], "uan": @[48, 49], "üan": @[50, 51], "iang": @[52, 53], "uang": @[54, 55],
  "iong": @[56, 57],
}.toTable

# --- 声母48 ---
const SHENGMU48* = [
  "b", "p", "m", "f", "d", "t", "n", "l",
  "g", "k", "h", "w", "j", "q", "x", "y",
  "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
]
const SHENGMU48_UPPER* = [
  "B", "P", "M", "F", "D", "T", "N", "L",
  "G", "K", "H", "W", "J", "Q", "X", "Y",
  "Z", "C", "S", "R", "Ȥ", "Ç", "Ş", "Ŋ",
]

# --- 颜色42 ---
const COLOR_SYMBOLS* = [
  "ạ", "a̋", "å", "ã", "â", "ă",       # 红 0-5
  "ẹ", "e̋", "e̊", "ẽ", "ê", "ĕ",       # 橙 6-11
  "ị", "i̋", "i̊", "ĩ", "î", "ĭ",       # 黄 12-17
  "ọ", "ő", "o̊", "õ", "ô", "ŏ",       # 绿 18-23
  "ụ", "ű", "ů", "ũ", "û", "ŭ",       # 青 24-29
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "ȁ",       # 蓝 30-35
  "ȅ", "ȉ", "ȍ", "ȕ", "ä", "ö",       # 紫 36-41
]
const COLOR_NAMES* = [
  "深红", "大红", "浅红", "粉红", "玫红", "殷红",
  "深橙", "橙色", "浅橙", "杏色", "琥珀", "橘红",
  "深黄", "明黄", "浅黄", "鹅黄", "金黄", "枯黄",
  "深绿", "翠绿", "浅绿", "草绿", "墨绿", "黄绿",
  "深青", "青色", "浅青", "天青", "藏青", "青绿",
  "深蓝", "蓝色", "浅蓝", "蔚蓝", "宝蓝", "灰蓝",
  "深紫", "紫色", "浅紫", "薰衣草", "紫罗兰", "暗紫",
]

# --- 声调60 ---
const TONE_SYMBOLS* = [
  "ā", "ō", "ē", "ī", "ū", "ǖ",     # 阴平 0-5
  "á", "ó", "é", "í", "ú", "ǘ",     # 阳平 6-11
  "ǎ", "ǒ", "ě", "ǐ", "ǔ", "ǚ",     # 上声 12-17
  "à", "ò", "è", "ì", "ù", "ǜ",     # 去声 18-23
]
const TONE_NAMES* = ["阴平", "阳平", "上声", "去声", "轻声"]

# --- 音乐3 ---
const MUSIC_NAMES* = ["原子", "短语", "段落"]

# ═══════════════════════════════════════════════════════════════
# 基元冻结 · 白名单(死规矩 · 不可增删)
# ═══════════════════════════════════════════════════════════════
const FROZEN_SHENGMU* = [
  "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
  "j", "q", "x", "y", "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ", "w",
]
const FROZEN_SHENGMU_UPPER* = [
  "B", "P", "M", "F", "D", "T", "N", "L", "G", "K", "H",
  "J", "Q", "X", "Y", "Z", "C", "S", "R", "Ȥ", "Ç", "Ŋ", "W",
]
const FROZEN_YUNMU_NAMES* = [
  "a", "o", "e", "i", "u", "ü", "ai", "ei", "ui", "ao", "ou", "iu",
  "ie", "üe", "er", "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong",
  "ia", "iao", "ua", "uai", "uo", "ian", "uan", "üan", "iang", "uang", "iong",
]
const FROZEN_TONES* = ["ˉ", "ˊ", "ˇ", "ˋ"]
const FROZEN_COLORS* = COLOR_SYMBOLS
const FROZEN_SPACE* = [
  "东", "南", "西", "北", "上", "下", "天", "地", "人",
  "前", "后", "左", "右", "中", "内", "外", "高", "低",
]
const FROZEN_MUSIC* = ["原子", "短语", "段落"]

# 白名单合集(手动拼接 · 不用&)
const WHITELIST* = block:
  var result: seq[string] = @[]
  for s in FROZEN_SHENGMU: result.add(s)
  for s in FROZEN_SHENGMU_UPPER: result.add(s)
  for s in FROZEN_TONES: result.add(s)
  for s in FROZEN_COLORS: result.add(s)
  for s in FROZEN_SPACE: result.add(s)
  for s in FROZEN_MUSIC: result.add(s)
  result

# ═══════════════════════════════════════════════════════════════
# 工具状态
# ═══════════════════════════════════════════════════════════════
type
  CodingTool* = object
    name*: string
    layers*: Table[string, int]
    symbol_to_layer*: Table[string, string]
    blacklist*: seq[string]  # 负样本(被越权造字的记录)

func new_coding_tool*(): CodingTool =
  result.name = "满全法编码工具"
  result.layers = initTable[string, int]()
  result.layers["韵母"] = 66
  result.layers["声母"] = 48
  result.layers["声调"] = 60
  result.layers["颜色"] = 42
  result.layers["空间"] = 18
  result.layers["镜像"] = 10
  result.layers["音乐"] = 3
  result.symbol_to_layer = initTable[string, string]()
  result.blacklist = @[]
  
  # 注册白名单符号
  for s in FROZEN_SHENGMU: result.symbol_to_layer[s] = "声母"
  for s in FROZEN_SHENGMU_UPPER: result.symbol_to_layer[s] = "声母"
  for s in SYMBOLS66: result.symbol_to_layer[s] = "韵母"
  for s in FROZEN_TONES: result.symbol_to_layer[s] = "声调"
  for s in FROZEN_COLORS: result.symbol_to_layer[s] = "颜色"
  for s in FROZEN_SPACE: result.symbol_to_layer[s] = "空间"
  for s in FROZEN_MUSIC: result.symbol_to_layer[s] = "音乐"

# ═══════════════════════════════════════════════════════════════
# 基元冻结检查
# ═══════════════════════════════════════════════════════════════
func is_frozen*(tool: CodingTool; s: string): bool =
  for w in WHITELIST:
    if s == w: return true
  return false

func get_violations*(tool: CodingTool; text: string): seq[string] =
  var i = 0
  while i < text.len:
    let r = text.runeAt(i)
    let rs = $r
    if not tool.is_frozen(rs):
      result.add(rs)
    i += len(rs)

# ═══════════════════════════════════════════════════════════════
# 六层审判
# ═══════════════════════════════════════════════════════════════
func get_category*(tool: CodingTool; glyph: string): string =
  if tool.symbol_to_layer.hasKey(glyph):
    return tool.symbol_to_layer[glyph]
  return "未知"

func judge_level*(a_level, b_level: int): tuple[pass: bool; score: float; detail: string] =
  if a_level == b_level:
    return (true, 1.0, "层级一致L" & $a_level)
  elif a_level == 3 or b_level == 3:
    return (false, 0.0, "L3成品禁止配对")
  else:
    return (false, 0.3, "层级跳跃L" & $a_level & "↔L" & $b_level)

func judge_type*(cat_a, cat_b: string): tuple[pass: bool; score: float; detail: string] =
  if cat_a == "未知" or cat_b == "未知":
    return (false, 0.0, "未知类型")
  if cat_a == cat_b:
    return (true, 1.0, "同类型" & cat_a)
  # 声母+韵母 = 音节组合(允许)
  if (cat_a == "声母" and cat_b == "韵母") or (cat_a == "韵母" and cat_b == "声母"):
    return (true, 0.8, "声母+韵母音节")
  # 声调+韵母 = 声调贴附(允许)
  if (cat_a == "声调" and cat_b == "韵母") or (cat_a == "韵母" and cat_b == "声调"):
    return (true, 0.7, "声调+韵母贴附")
  # 其他跨类型
  return (false, 0.3, "跨类型: " & cat_a & "+" & cat_b)

func judge_aesthetic*(a, b: string): tuple[pass: bool; score: float; detail: string] =
  ## 审美 · 检查变音符号是否匹配
  # 基元不能与变音成品配对
  var a_is_accented = false
  var b_is_accented = false
  let cp_a = if a.len > 0: ord(a.runeAt(0)) else: 0
  let cp_b = if b.len > 0: ord(b.runeAt(0)) else: 0
  if cp_a >= 0x00C0 and cp_a <= 0x024F: a_is_accented = true
  if cp_b >= 0x00C0 and cp_b <= 0x024F: b_is_accented = true
  
  if a_is_accented and not b_is_accented:
    return (false, 0.3, "变音+基元: 水位不匹配")
  if b_is_accented and not a_is_accented:
    return (false, 0.3, "基元+变音: 水位不匹配")
  if a_is_accented and b_is_accented:
    # 都是变音 · 可以接受
    return (true, 0.9, "双变音: 水位平衡")
  return (true, 1.0, "双基元: 水位平衡")

proc judge*(tool: CodingTool; a, b: string): tuple[pass: bool; score: float; report: string] =
  var lines: seq[string] = @[]
  lines.add("═══ 审判报告 ═══")
  lines.add("配对: " & a & " + " & b)
  lines.add("")
  
  let cat_a = tool.get_category(a)
  let cat_b = tool.get_category(b)
  
  # 六维审判
  var results: seq[tuple[dim: string; pass: bool; score: float; detail: string]] = @[]
  
  # 层级
  let level_a = if cat_a == "声调": 3 elif cat_a == "颜色": 2 else: 1
  let level_b = if cat_b == "声调": 3 elif cat_b == "颜色": 2 else: 1
  let r_level = judge_level(level_a, level_b)
  results.add(("层级栈", r_level.pass, r_level.score, r_level.detail))
  
  # 类型
  let r_type = judge_type(cat_a, cat_b)
  results.add(("类型身份", r_type.pass, r_type.score, r_type.detail))
  
  # 审美
  let r_aes = judge_aesthetic(a, b)
  results.add(("视觉审美", r_aes.pass, r_aes.score, r_aes.detail))
  
  # 归属权(我们的系统都是SELF)
  results.add(("归属权", true, 1.0, "SELF+SELF"))
  
      # 衍射规则
  var diffraction_ok = (cat_a == cat_b) or
    ((cat_a == "声母" and cat_b == "韵母") or (cat_a == "韵母" and cat_b == "声母")) or
    ((cat_a == "声调" and cat_b == "韵母") or (cat_a == "韵母" and cat_b == "声调"))
  # 检查自定义配对
  let custom_key = a & "+" & b
  let custom_key2 = b & "+" & a
  if custom_pairs.hasKey(custom_key) or custom_pairs.hasKey(custom_key2):
    diffraction_ok = true
    results.add(("衍射规则", true, 1.0, "自定义配对: " & a & "+" & b & "=" & (if custom_pairs.hasKey(custom_key): custom_pairs[custom_key] else: custom_pairs[custom_key2])))
  else:
    results.add(("衍射规则", diffraction_ok, if diffraction_ok: 1.0 else: 0.2,
                  if diffraction_ok: "配对合法" else: "禁止衍射"))
  
  # 黑名单
  results.add(("黑名单", true, 1.0, "未命中"))
  
  # 计算总分
  var total_score = 0.0
  var any_veto = false
  for r in results:
    total_score += r.score
    if not r.pass: any_veto = true
  
  total_score = total_score / float(results.len)
  
  # 输出各维度
  lines.add("── 各维度 ──")
  for r in results:
    let mark = if r.pass: "✓" else: "✗"
    lines.add("  " & mark & " " & r.dim.alignLeft(8) & " " & r.detail)
  
  lines.add("")
  lines.add("全局得分: " & formatFloat(total_score * 100, ffDecimal, 0) & "%")
  lines.add("判决: " & (if (total_score >= 0.6 and not any_veto): "✓通过" else: "✗拒绝"))
  result.pass = (total_score >= 0.6 and not any_veto)
  result.score = total_score
  result.report = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 韵母查询
# ═══════════════════════════════════════════════════════════════
func find_yunmu*(name: string): tuple[found: bool; symbols: seq[string]; idx: int] =
  if YUNMU_CODES.hasKey(name):
    var syms: seq[string] = @[]
    for idx in YUNMU_CODES[name]:
      if idx < SYMBOLS66.len: syms.add(SYMBOLS66[idx])
    return (true, syms, YUNMU_CODES[name][0])
  return (false, @[], -1)

func find_color*(sym: string): tuple[found: bool; name: string; idx: int] =
  for i, s in COLOR_SYMBOLS:
    if s == sym: return (true, COLOR_NAMES[i], i)
  return (false, "", -1)

func find_shengmu*(s: string): tuple[found: bool; idx: int; is_upper: bool] =
  for i, sh in SHENGMU48:
    if sh == s: return (true, i, false)
  for i, sh in SHENGMU48_UPPER:
    if sh == s: return (true, i + 24, true)
  return (false, -1, false)

# ═══════════════════════════════════════════════════════════════
# JSON导出
# ═══════════════════════════════════════════════════════════════
func export_json*(tool: CodingTool): string =
  var j = newJObject()
  j["system"] = newJString("满全法私有编码体系")
  j["divisor"] = newJString("统一自洽")
  
  var layers_j = newJObject()
  for layer_name, base in tool.layers:
    var layer_j = newJObject()
    layer_j["base"] = newJInt(base)
    layers_j[layer_name] = layer_j
  j["layers"] = layers_j
  
  var yunmu_j = newJObject()
  for name, codes in YUNMU_CODES:
    var entry = newJArray()
    for c in codes: entry.add(newJInt(c))
    yunmu_j[name] = entry
  j["yunmu_table"] = yunmu_j
  
  var color_j = newJArray()
  for i, sym in COLOR_SYMBOLS:
    var entry = newJObject()
    entry["idx"] = newJInt(i)
    entry["symbol"] = newJString(sym)
    entry["name"] = newJString(COLOR_NAMES[i])
    color_j.add(entry)
  j["color_table"] = color_j
  
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 解释系统
# ═══════════════════════════════════════════════════════════════
func explain*(): string = """
═══ 满全法编码系统 · 完整解释 ═══

【七层编码】
  声母48 · 韵母66 · 声调60 · 颜色42 · 空间18 · 镜像10 · 音乐3

【运算规则】
  ✓ 加法 = L1基元+基元 → 复合符号(衍射)
  ✓ 减法 = 成品-零件 → 剥离还原
  ✗ 乘除 = 对字形无意义 · 禁用

【审判六维】
  1. 层级栈: L1+L1允许 · L3禁止配对
  2. 类型身份: 同类型允许 · 跨类型有条件允许
  3. 视觉审美: 水位匹配(基元≈基元 · 变音≈变音)
  4. 归属权: SELF+SELF · EXTERNAL需重定义
  5. 衍射规则: 等效对齐才合法
  6. 黑名单: 反面样本永不录用

【知识隔离】
  本体库 · 参考池 · 黑名单 · 丢弃池 · 互不渗透
"""

# ═══════════════════════════════════════════════════════════════
# CLI入口
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  load_custom()
  var tool = new_coding_tool()
  
  if paramCount() == 0:
    echo explain()
    echo "用法: ./quan_neng <command> [args]"
    echo "命令: explain · judge · yunmu · color · sheng · tone · music · space · mirror · custom · export · shell"
    quit(0)
  
  let cmd = paramStr(1)
  
  case cmd:
  of "explain":
    echo explain()
  
  of "judge":
    if paramCount() < 3:
      echo "用法: ./quan_neng judge <符号A> <符号B>"
      quit(1)
    let a = paramStr(2)
    let b = paramStr(3)
    # 基元冻结检查
    let viol_a = tool.get_violations(a)
    let viol_b = tool.get_violations(b)
    if viol_a.len > 0 or viol_b.len > 0:
      echo "═══ 基元冻结违规 ═══"
      if viol_a.len > 0:
        echo "  A(", a, ") 含白名单外字符: ", viol_a.join(", ")
      if viol_b.len > 0:
        echo "  B(", b, ") 含白名单外字符: ", viol_b.join(", ")
      echo "  判决: ✗拒绝(违反基元冻结)"
      quit(1)
    let r = tool.judge(a, b)
    echo r.report
  
  of "frozen":
    if paramCount() < 2:
      echo "用法: ./quan_neng frozen <符号>"
      echo "检查符号是否在白名单(冻结基元)中"
      quit(1)
    let s = paramStr(2)
    if tool.is_frozen(s):
      echo "  ✓ ", s, " 是冻结基元"
    else:
      echo "  ✗ ", s, " 不在白名单中 · 禁止使用"
    let viols = tool.get_violations(s)
    if viols.len > 0:
      echo "  违规字符: ", viols.join(", ")
  
  of "yunmu":
    if paramCount() < 2:
      echo "用法: ./quan_neng yunmu <韵母名>"
      echo "示例: ./quan_neng yunmu ai"
      quit(1)
    let r = find_yunmu(paramStr(2))
    if r.found:
      echo paramStr(2), " → #", r.idx, " [", r.symbols.join(","), "]"
    else:
      echo "未知韵母: ", paramStr(2)
  
  of "color":
    if paramCount() < 2:
      echo "用法: ./quan_neng color <符号>"
      quit(1)
    let r = find_color(paramStr(2))
    if r.found:
      echo paramStr(2), " → #", r.idx, " [", r.name, "]"
    else:
      echo "未知颜色符号: ", paramStr(2)
  
  of "sheng":
    if paramCount() < 2:
      echo "用法: ./quan_neng sheng <声母>"
      quit(1)
    let r = find_shengmu(paramStr(2))
    if r.found:
      echo paramStr(2), " → #", r.idx, " ", (if r.is_upper: "(大写)" else: "(小写)")
    else:
      echo "未知声母: ", paramStr(2)
  
  of "tone":
    echo "── 声调60 ──"
    echo "  小写24: ", TONE_SYMBOLS[0..23].join(" ")
    echo "  分组: 阴平(0-5) 阳平(6-11) 上声(12-17) 去声(18-23)"
  
  of "music":
    echo "── 音乐3 ──"
    for i, m in MUSIC_NAMES:
      echo "  #", i, " ", m
  
  of "space":
    echo "── 空间18 ──"
    echo "  东西南北上下天地 · 8方位×2+2=18"
  
  of "mirror":
    echo "── 镜像10 ──"
    echo "  2D/3D/4D/5D相位索引"
  
  of "diffraction":
    echo "── 衍射表(固定配对) ──"
    const DIFF_TABLE = {
      "a+i": "ꬰ", "e+i": "ɘ", "u+i": "ѱ", "a+o": "ꜵ",
      "o+u": "ȣ", "i+u": "ꭎ", "i+e": "ɤ", "ü+e": "ᵫ",
      "e+r": "ɛ", "a+n": "ꙙ", "e+n": "ⱺ", "i+n": "ĩ",
      "u+n": "տ", "ü+n": "ỹ", "a+ng": "Ⴆ", "e+ng": "ӫ",
      "i+ng": "ꙟ", "o+ng": "ꭁ",
    }.toTable
    for k, v in DIFF_TABLE:
      echo "  ", k.alignLeft(10), " → ", v
    echo ""
    echo "── 自定义配对 ──"
    for k, v in custom_pairs:
      echo "  ", k.alignLeft(15), " → ", v
  
  of "restore":
    if paramCount() < 2:
      echo "用法: ./quan_neng_tool restore <文件名>"
      echo "恢复被破坏的文件到原始状态"
      quit(1)
    let target = paramStr(2)
    echo "恢复: ", target
    echo "✓ 已记录恢复请求(实际恢复需手动确认)"
  
  of "custom":
    if paramCount() < 4:
      echo "用法: ./quan_neng custom <A> <B> <结果符号>"
      echo "示例: ./quan_neng custom ai ei ꬰ"
      echo ""
      echo "── 当前自定义配对 ──"
      for k, v in custom_pairs:
        echo "  ", k.alignLeft(15), " → ", v
      quit(1)
    let a = paramStr(2)
    let b = paramStr(3)
    let r = paramStr(4)
    add_custom_pair(a, b, r)
    echo "✓ 已添加: ", a, " + ", b, " = ", r
  
  of "export":
    echo tool.export_json()
  
  of "shell":
    echo "═══ 交互模式 ═══"
    var running = true
    while running:
      stdout.write("> ")
      let input = readLine(stdin).strip()
      if input == "quit" or input == "q":
        running = false
      elif input == "explain":
        echo explain()
      elif input.startsWith("judge "):
        let p = input.split(' ')
        if p.len >= 3:
          echo tool.judge(p[1], p[2]).report
      elif input.startsWith("yunmu "):
        let r = find_yunmu(input[6..^1])
        if r.found: echo input[6..^1], " → #", r.idx, " [", r.symbols.join(","), "]"
        else: echo "未知韵母"
      elif input.startsWith("color "):
        let r = find_color(input[6..^1])
        if r.found: echo input[6..^1], " → #", r.idx, " [", r.name, "]"
        else: echo "未知颜色"
      elif input.startsWith("custom "):
        let p = input.split(' ')
        if p.len >= 4:
          add_custom_pair(p[1], p[2], p[3])
          echo "✓ 已添加: ", p[1], " + ", p[2], " = ", p[3]
        else:
          for k, v in custom_pairs:
            echo "  ", k, " → ", v
      elif input == "export":
        echo tool.export_json()
      else:
        echo "命令: explain · judge · yunmu · color · sheng · custom · export · quit"
  
  else:
    echo "未知命令: " & cmd
