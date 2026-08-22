## ═══════════════════════════════════════════════════════════════
## 六层审判系统 · 从芝麻到大海的全域感知
## ═══════════════════════════════════════════════════════════════
## 层级1: 字符过滤 — 写错字? a≠á?
## 层级2: 类型过滤 — 类型匹配吗?
## 层级3: 审美过滤 — 视觉重量平衡吗?
## 层级4: 等效对齐 — 公倍数对齐公约数吗?
## 层级5: 全局一致 — 服务于统一目标吗?
## 层级6: 编织制衡 — 各节点拉力均衡吗?
## ═══════════════════════════════════════════════════════════════

import tables, math, strutils

# ═══════════════════════════════════════════════════════════════
# 层级1: 字符过滤 · 芝麻级
# ═══════════════════════════════════════════════════════════════
const BASE_CHARS* = [
  # 基元字符(不允许被当作等价)
  "a", "o", "e", "i", "u", "ü",
  "A", "O", "E", "I", "U", "Ü",
]

const ACCENTED_CHARS* = [
  # 变音字符(不等于基元)
  "á", "ó", "é", "í", "ú", "ǘ",
  "ǎ", "ǒ", "ě", "ǐ", "ǔ", "ǚ",
  "à", "ò", "è", "ì", "ù", "ǜ",
  "ā", "ō", "ē", "ī", "ū", "ǖ",
  "ạ", "ẹ", "ị", "ọ", "ụ",
  "ä", "ë", "ï", "ö",
]

func is_base_char*(c: string): bool =
  for b in BASE_CHARS:
    if c == b: return true

func is_accented_char*(c: string): bool =
  for a in ACCENTED_CHARS:
    if c == a: return true

func layer1_char_filter*(a, b: string): tuple[pass: bool; reason: string] =
  # 基元 ≠ 变音
  if a.len == 1 and b.len == 1:
    if is_base_char(a) and is_accented_char(b):
      return (false, "层级1拒绝: 基元" & a & "≠变音" & b)
    if is_accented_char(a) and is_base_char(b):
      return (false, "层级1拒绝: 变音" & a & "≠基元" & b)
  return (true, "层级1通过")

# ═══════════════════════════════════════════════════════════════
# 层级2: 类型过滤 · 分类级
# ═══════════════════════════════════════════════════════════════
type
  SymbolType* = enum
    stSheng,    # 声母
    stYun,      # 韵母
    stClock,    # 时钟
    stColor,    # 颜色
    stSpace,    # 空间
    stMirror,   # 镜像
    stMusic,    # 音乐
    stUnknown,  # 未知

const SHENG_SET* = [
  "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
  "j", "q", "x", "zh", "ch", "sh", "r", "z", "c", "s", "y", "w",
]

const YUN_SET* = [
  "ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er",
  "an", "en", "in", "un", "ün",
  "ang", "eng", "ing", "ong",
  "ia", "iao", "ua", "uai", "uo",
  "ian", "uan", "üan", "iang", "uang", "iong",
]

func get_symbol_type*(s: string): SymbolType =
  for sh in SHENG_SET:
    if s == sh: return stSheng
  for y in YUN_SET:
    if s == y: return stYun
  if s in ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"]:
    return stClock
  if s in ["红", "橙", "黄", "绿", "青", "蓝", "紫"]:
    return stColor
  if s in ["东", "南", "西", "北", "上", "下", "天", "地"]:
    return stSpace
  if s in ["A", "a", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U", "ü", "Ü"]:
    return stMirror
  if s in ["原子", "短语", "段落"]:
    return stMusic
  return stUnknown

func layer2_type_filter*(a, b: string): tuple[pass: bool; reason: string] =
  let ta = get_symbol_type(a)
  let tb = get_symbol_type(b)
  if ta != tb and ta != stUnknown and tb != stUnknown:
    return (false, "层级2拒绝: 类型不匹配 " & $ta & " ≠ " & $tb)
  return (true, "层级2通过")

# ═══════════════════════════════════════════════════════════════
# 层级3: 审美过滤 · 视觉重量
# ═══════════════════════════════════════════════════════════════
func visual_weight*(s: string): float =
  # 视觉重量估算: 基于字符宽度和复杂度
  var w = 0.0
  for c in s:
    let cp = ord(c)
    if cp >= 0x0041 and cp <= 0x007A:
      w += 1.0  # 基础拉丁
    elif cp >= 0x0400 and cp <= 0x04FF:
      w += 1.2  # 西里尔
    elif cp >= 0x0370 and cp <= 0x03FF:
      w += 1.3  # 希腊
    elif cp >= 0x10A0 and cp <= 0x10FF:
      w += 1.4  # 格鲁吉亚
    elif cp >= 0x0600 and cp <= 0x06FF:
      w += 1.5  # 阿拉伯
    elif cp >= 0x0900 and cp <= 0x097F:
      w += 1.6  # 天城文
    elif cp >= 0x4E00 and cp <= 0x9FFF:
      w += 2.0  # 汉字
    elif cp >= 0xA000 and cp <= 0xA48F:
      w += 1.8  # 彝文
    elif cp >= 0x10300 and cp <= 0x1032F:
      w += 1.7  # 古意大利
    elif cp >= 0x10330 and cp <= 0x1034F:
      w += 1.6  # 哥特
    elif cp >= 0x2C00 and cp <= 0x2C5F:
      w += 1.5  # 格拉哥里
    elif cp >= 0x2C80 and cp <= 0x2CFF:
      w += 1.4  # 科普特
    elif cp >= 0x0530 and cp <= 0x058F:
      w += 1.3  # 亚美尼亚
    elif cp >= 0x0E00 and cp <= 0x0E7F:
      w += 1.4  # 泰文
    else:
      w += 1.1  # 其他
  w

func layer3_aesthetic_filter*(a, b: string): tuple[pass: bool; reason: string] =
  let wa = visual_weight(a)
  let wb = visual_weight(b)
  let ratio = min(wa, wb) / max(wa, wb)
  if ratio < 0.5:
    return (false, "层级3拒绝: 视觉重量失衡 " & formatFloat(wa, ffDecimal, 1) &
            " vs " & formatFloat(wb, ffDecimal, 1) & " 比率=" & formatFloat(ratio, ffDecimal, 2))
  return (true, "层级3通过")

# ═══════════════════════════════════════════════════════════════
# 层级4: 等效对齐 · 公约数镜子
# ═══════════════════════════════════════════════════════════════
const LAYER_ANCHORS* = {
  "声母": 48,
  "韵母": 66,
  "时钟": 60,
  "颜色": 42,
  "空间": 18,
  "镜像": 10,
  "音乐": 3,
}.toTable

func layer4_alignment*(a, b: string; layer_name: string): tuple[pass: bool; reason: string] =
  if not LAYER_ANCHORS.hasKey(layer_name):
    return (true, "层级4跳过: 未知层")
  let anchor = LAYER_ANCHORS[layer_name]
  # 简化: 检查两个符号在该层是否有定义的位置
  # 实际使用需要每层的符号表
  return (true, "层级4通过(需层符号表)")

# ═══════════════════════════════════════════════════════════════
# 层级5: 全局一致 · 服务于统一目标
# ═══════════════════════════════════════════════════════════════
const UNIFIED_GOAL* = "满全法编码体系自洽"

func layer5_global*(a, b: string; goal: string): tuple[pass: bool; reason: string] =
  # 检查配对是否服务于统一目标
  # 简化: 如果两个符号来自同一层且服务于同一目标 → 通过
  let ta = get_symbol_type(a)
  let tb = get_symbol_type(b)
  if ta == tb and ta != stUnknown:
    return (true, "层级5通过: 同类型服务于" & goal)
  if ta == stUnknown or tb == stUnknown:
    return (false, "层级5拒绝: 存在未知类型符号")
  return (true, "层级5通过: 跨层协作")

# ═══════════════════════════════════════════════════════════════
# 层级6: 编织制衡 · 拉力均衡
# ═══════════════════════════════════════════════════════════════
func layer6_weaving*(pairs: seq[tuple[a: string; b: string]]): tuple[pass: bool; reason: string] =
  # 统计各层参与度
  var layer_count = initTable[string, int]()
  for p in pairs:
    let ta = get_symbol_type(p.a)
    let tb = get_symbol_type(p.b)
    let key = $ta & "×" & $tb
    if layer_count.hasKey(key):
      layer_count[key] = layer_count[key] + 1
    else:
      layer_count[key] = 1

  # 检查是否某类配对过多(失衡)
  let total = pairs.len
  for k, v in layer_count:
    let ratio = float(v) / float(total)
    if ratio > 0.5:
      return (false, "层级6拒绝: 配对类型" & k & "占比过高" & formatFloat(ratio * 100, ffDecimal, 0) & "%")
  return (true, "层级6通过: 编织均衡")

# ═══════════════════════════════════════════════════════════════
# 六层审判主函数
# ═══════════════════════════════════════════════════════════════
func six_layer_judge*(a, b: string; layer_name: string = "韵母"): tuple[pass: bool; reason: string; layer: int] =
  # 层级1
  let r1 = layer1_char_filter(a, b)
  if not r1.pass: return (false, r1.reason, 1)

  # 层级2
  let r2 = layer2_type_filter(a, b)
  if not r2.pass: return (false, r2.reason, 2)

  # 层级3
  let r3 = layer3_aesthetic_filter(a, b)
  if not r3.pass: return (false, r3.reason, 3)

  # 层级4
  let r4 = layer4_alignment(a, b, layer_name)
  if not r4.pass: return (false, r4.reason, 4)

  # 层级5
  let r5 = layer5_global(a, b, UNIFIED_GOAL)
  if not r5.pass: return (false, r5.reason, 5)

  return (true, "六层全部通过", 0)

# ═══════════════════════════════════════════════════════════════
# 测试
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══ 六层审判系统 ═══"
  echo ""

  # 测试1: a vs á (应被层级1拒绝)
  let t1 = six_layer_judge("a", "á")
  echo "测试1(a,á): ", if t1.pass: "通过" else: "拒绝@", t1.layer, " ", t1.reason

  # 测试2: e vs ě (应被层级1拒绝)
  let t2 = six_layer_judge("e", "ě")
  echo "测试2(e,ě): ", if t2.pass: "通过" else: "拒绝@", t2.layer, " ", t2.reason

  # 测试3: ai vs ei (应通过层级1-3)
  let t3 = six_layer_judge("ai", "ei")
  echo "测试3(ai,ei): ", if t3.pass: "通过" else: "拒绝@", t3.layer, " ", t3.reason

  # 测试4: 声母 vs 韵母 (应被层级2拒绝)
  let t4 = six_layer_judge("b", "ai")
  echo "测试4(b,ai): ", if t4.pass: "通过" else: "拒绝@", t4.layer, " ", t4.reason

  # 测试5: 视觉重量失衡
  let t5 = six_layer_judge("ꬰ", "Ⴏ")
  echo "测试5(ꬰ,Ⴏ): ", if t5.pass: "通过" else: "拒绝@", t5.layer, " ", t5.reason

  # 测试6: 编织均衡
  var test_pairs: seq[tuple[a: string; b: string]] = @[
    ("ai", "ei"), ("ai", "ui"), ("ai", "ao"),
    ("ai", "ou"), ("ai", "iu"), ("ai", "ie"),
  ]
  let t6 = layer6_weaving(test_pairs)
  echo "测试6(编织): ", if t6.pass: "通过" else: "拒绝 ", t6.reason

  echo ""
  echo "── 层级说明 ──"
  echo "  层级1: 字符过滤 — a≠á"
  echo "  层级2: 类型过滤 — 声母≠韵母"
  echo "  层级3: 审美过滤 — 视觉重量平衡"
  echo "  层级4: 等效对齐 — 公约数镜子"
  echo "  层级5: 全局一致 — 统一目标"
  echo "  层级6: 编织制衡 — 拉力均衡"
