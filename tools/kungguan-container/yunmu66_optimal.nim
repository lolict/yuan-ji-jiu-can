## ═══════════════════════════════════════════════════════════════
## 最优66进制韵母编码容器 · 33韵母 × 2(大小写)
## ═══════════════════════════════════════════════════════════════
## 设计原则:
##   ① aoeiuv 已交给60声系统 · 此处33韵母全为复韵母+鼻韵母
##   ② 每韵母严格配对(小写+大写) · 66符号全部唯一
##   ③ 符号选取标准: 不混淆 · 好记 · 好看 · 有逻辑关联
##   ④ 大小写之间有视觉/语义关联(同族·镜像·同构)
##   ⑤ 凡是"两个符号长得太像"的配对一律剔除
## ═══════════════════════════════════════════════════════════════

import tables, strutils

# ═══════════════════════════════════════════════════════════════
# 63对最优符号 (33韵母 × 2 - 3个三重韵母额外变体)
# ═══════════════════════════════════════════════════════════════

const SYMBOLS66* = [
  # ── 复韵母 (开口呼 · 8对) ──────────────────────────────
  # ai: 开口呼 · 选用 ꬰ/ꬱ (米拉维连字 · 不与其它ai符号混淆)
  "ꬰ",    # 0  ai小写   米拉维I+A连字
  "ꬱ",    # 1  ai大写   米拉维I+A连字大写
  # ei: 开口呼 · 选用 ӭ/Ӭ (西里尔E+分音 · 视觉清晰)
  "ӭ",    # 2  ei小写   西里尔E分音小写
  "Ӭ",    # 3  ei大写   西里尔E分音大写
  # ao: 开口呼 · 选用 ꜵ/Ꜵ (A+O融合 · 直观)
  "ꜵ",    # 4  ao小写   A+O融合小写
  "Ꜵ",    # 5  ao大写   A+O融合大写
  # ou: 开口呼 · 选用 ȣ/Ȣ (UK连体 · 与uo区分)
  "ȣ",    # 6  ou小写   UK连体小写
  "Ȣ",    # 7  ou大写   UK连体大写
  # er: 卷舌呼 · 选用 Ʝ (特殊 · 全球唯一)
  "Ʝ",    # 8  er小写   拉丁钩J变体(卷舌专属)
  "Ɉ",    # 9  er大写   拉丁J+杠(卷舌大写)
  # ei(二选): 备选 ə/Ə (中央元音 · 极清晰)
  "ə",    # 10 ei-变体  音標中央元音小写
  "Ə",    # 11 ei-变体  音標中央元音大写
  # ui: 开口呼 · 选用 ѱ/Ѱ (Cyrillic-like · 不混淆)
  "ѱ",    # 12 ui小写   类似Cyrillic P变体
  "Ѱ",    # 13 ui大写   类似Cyrillic P变体大写

  # ── 介音韵母 (齐齿呼+合口呼 · 10对) ────────────────────
  # ia: 齐齿呼 · ꙗ/Ꙗ (古西里尔IA · 天然IA配对)
  "ꙗ",    # 14 ia小写   古西里尔IA小写
  "Ꙗ",    # 15 ia大写   古西里尔IA大写
  # ie: 齐齿呼 · Ꙋ/Ɣ (古Cyrillic + 音标 · 清晰对比)
  "Ꙋ",    # 16 ie小写   古西里尔UK变体小写
  "Ɣ",    # 17 ie大写   音標gamma大写
  # ua: 合口呼 · ǟ/Ǟ (A+分音波浪 · 似水波动)
  "ǟ",    # 18 ua小写   A+分音+波浪小写
  "Ǟ",    # 19 ua大写   A+分音+波浪大写
  # uo: 合口呼 · ȫ/Ȫ (O+分音 · 圆唇清晰)
  "ȫ",    # 20 uo小写   O+分音小写
  "Ȫ",    # 21 uo大写   O+分音大写
  # iu: 齐齿呼 · Ⴏ/Ⴎ (I+U合体 · 不混淆于ou)
  "Ⴏ",    # 22 iu小写   古格鲁吉亚IU小写
  "Ⴎ",    # 23 iu大写   古格鲁吉亚IU大写
  # üe: 撮口呼 · ᵫ/ꭂ (U+分音变体 · 撮口特征)
  "ᵫ",    # 24 üe小写   U+分音变体小写
  "ꭂ",    # 25 üe大写   U+分音专有变体大写
  # iao: 齐齿呼 · 小写ꮂ/大写ᴥ (iao专属 · 不混淆)
  "ꮂ",    # 26 iao小写   IAO专用连字小写
  "ᴥ",    # 27 iao大写   IAO专用连字大写
  # uai: 合口呼 · ꜻ/Ꜻ (UA+I融合 · 直观)
  "ꜻ",    # 28 uai小写   UA+I融合小写
  "Ꜻ",    # 29 uai大写   UA+I融合大写

  # ── 前鼻韵母 (5对) ──────────────────────────────────
  # an: ꙙ/Ꙙ (A+短音 · 与en/ang明确区分)
  "ꙙ",    # 30 an小写   A+短音小写
  "Ꙙ",    # 31 an大写   A+短音大写
  # en: ⱅ/ⱄ (古西里尔EN · 专属)
  "ⱅ",    # 32 en小写   古西里尔EN变体小写
  "ⱄ",    # 33 en大写   古西里尔EN变体大写
  # in: ႁ/႔ (I+鼻化 · 清晰)
  "ႁ",    # 34 in小写   古格鲁吉亚IN小写
  "႔",    # 35 in大写   古格鲁吉亚IN大写
  # un: տ/໙ (Un专属符号 · 视觉独特)
  "տ",    # 36 un小写   UN专属小写
  "໙",    # 37 un大写   老挝数字9(作UN大写)
  # ün: Ⴐ/Ⴐ (Ün符号 · 大小写同形)
  "Ⴐ",    # 38 ün小写   ÜN专属小写
  "Ⴑ",    # 39 ün大写   ÜN专属大写

  # ── 后鼻韵母 (4对) ──────────────────────────────────
  # ang: Ⴆ/Ⴊ (A+后鼻 · 与an区分)
  "Ⴆ",    # 40 ang小写   A+后鼻小写
  "Ⴊ",    # 41 ang大写   A+后鼻大写
  # eng: ӫ/Ӫ (Cyrillic E+分音+短音 · 专属)
  "ӫ",    # 42 eng小写   Cyrillic E变体小写
  "Ӫ",    # 43 eng大写   Cyrillic E变体大写
  # ing: ꙟ/Ꙟ (I+后鼻 · 镜像配对)
  "ꙟ",    # 44 ing小写   I+后鼻小写
  "Ꙟ",    # 45 ing大写   I+后鼻大写
  # ong: ꭁ/ꭀ (O+后鼻 · 圆唇专属)
  "ꭁ",    # 46 ong小写   O+后鼻小写
  "ꭀ",    # 47 ong大写   O+后鼻大写

  # ── 三合韵母 (5对) ──────────────────────────────────
  # ian: ꙝ/Ꙝ (IA+鼻化 · 天然配对)
  "ꙝ",    # 48 ian小写   IA+鼻化小写
  "Ꙝ",    # 49 ian大写   IA+鼻化大写
  # uan: Ⴄ/Ⴂ (UA+鼻化 · 合口特征)
  "Ⴄ",    # 50 uan小写   UA+鼻化小写
  "Ⴂ",    # 51 uan大写   UA+鼻化大写
  # üan: ѫ/Ѫ (ÜA+鼻化 · 撮口特征)
  "ѫ",    # 52 üan小写   ÜA+鼻化小写
  "Ѫ",    # 53 üan大写   ÜA+鼻化大写
  # iang: ѭ/Ѭ (IA+后鼻 · 镜像清晰)
  "ѭ",    # 54 iang小写   IA+后鼻小写
  "Ѭ",    # 55 iang大写   IA+后鼻大写
  # uang: ꙛ/Ꙛ (UA+后鼻 · 圆唇+鼻化)
  "ꙛ",    # 56 uang小写   UA+后鼻小写
  "Ꙛ",    # 57 uang大写   UA+后鼻大写

  # ── 四合韵母 (3对) ──────────────────────────────────
  # iong: ȱ/Ȱ (IO+后鼻 · 撮口+鼻化)
  "ȱ",    # 58 iong小写   IO+后鼻小写
  "Ȱ",    # 59 iong大写   IO+后鼻大写
  # ueng: ɞ/ɝ (UE+后鼻 · 与ong区分)
  "ɞ",    # 60 ueng小写   UE+后鼻小写
  "ɝ",    # 61 ueng大写   UE+后鼻大写
  # iou(压缩io): ø/Ø (圆唇介音 · 北欧字母清晰)
  "ø",    # 62 iou小写   O+分音小写(北欧)
  "Ø",    # 63 iou大写   O+分音大写(北欧)

  # ── 额外韵母 (2对 · 补全33韵母) ──────────────────────
  # io: 罕见韵母(哟/唷) · 选用 ю/Ю (Cyrillic ju · 专属)
  "ю",    # 64 io小写   古Cyrillic ju小写(作io)
  "Ю",    # 65 io大写   古Cyrillic ju大写(作io)
]

# ═══════════════════════════════════════════════════════════════
# 韵母到编码索引的映射
# ═══════════════════════════════════════════════════════════════
const YUNMU_CODES* = {
  # 复韵母(开口呼)
  "ai":       @[0, 1],
  "ei":       @[2, 3, 10, 11],    # ə/Ə为ei变体
  "ao":       @[4, 5],
  "ou":       @[6, 7],
  "er":       @[8, 9],
  "ui":       @[12, 13],
  # 介音韵母
  "ia":       @[14, 15],
  "ie":       @[16, 17],
  "ua":       @[18, 19],
  "uo":       @[20, 21],
  "iu":       @[22, 23],
  "üe":       @[24, 25],
  "iao":      @[26, 27],
  "uai":      @[28, 29],
  # 前鼻韵母
  "an":       @[30, 31],
  "en":       @[32, 33],
  "in":       @[34, 35],
  "un":       @[36, 37],
  "ün":       @[38, 39],
  # 后鼻韵母
  "ang":      @[40, 41],
  "eng":      @[42, 43],
  "ing":      @[44, 45],
  "ong":      @[46, 47],
  # 三合韵母
  "ian":      @[48, 49],
  "uan":      @[50, 51],
  "üan":      @[52, 53],
  "iang":     @[54, 55],
  "uang":     @[56, 57],
  # 四合韵母
  "iong":     @[58, 59],
  "ueng":     @[60, 61],
  "iou":      @[62, 63],
  # 额外韵母(补全33)
  "io":       @[64, 65],
  "ve":       @[24, 25],   # ve=üe (输入法写法 · 共享符号)
}.toTable

# ═══════════════════════════════════════════════════════════════
# 容器节点定义
# ═══════════════════════════════════════════════════════════════
type
  YunmuNode* = object
    yunmu*: string
    codes*: seq[int]
    symbols*: seq[string]
    group*: string    # 四呼分组

  RongQi66* = object
    name*: string
    symbol_to_idx*: Table[string, int]
    idx_to_yunmu*: Table[int, string]
    nodes*: seq[YunmuNode]

func yunmu_group*(ym: string): string =
  case ym:
  of "ai", "ei", "ao", "ou", "er", "ui": return "开口呼(复)"
  of "ia", "ie", "iao", "iu", "iou": return "齐齿呼"
  of "ua", "uo", "uai", "uan", "ueng": return "合口呼"
  of "üe", "ve", "üan", "ün": return "撮口呼"
  of "an", "en", "in", "un": return "前鼻韵"
  of "ang", "eng", "ing", "ong": return "后鼻韵"
  of "ian", "iang": return "齐齿+鼻"
  of "uang", "iong": return "合/撮+鼻"
  of "io": return "齐齿呼(罕)"
  else: return "其他"

func new_rongqi66*(name: string): RongQi66 =
  result.name = name
  result.symbol_to_idx = initTable[string, int]()
  result.idx_to_yunmu = initTable[int, string]()
  for i, sym in SYMBOLS66:
    result.symbol_to_idx[sym] = i

func symbol_value*(r: RongQi66, sym: string): int =
  if r.symbol_to_idx.hasKey(sym): return r.symbol_to_idx[sym]
  return -1

proc add_yunmu*(r: var RongQi66, name: string) =
  var node: YunmuNode
  node.yunmu = name
  node.group = yunmu_group(name)
  if YUNMU_CODES.hasKey(name):
    node.codes = YUNMU_CODES[name]
    for idx in node.codes:
      if idx < SYMBOLS66.len:
        node.symbols.add(SYMBOLS66[idx])
        r.idx_to_yunmu[idx] = name
  r.nodes.add(node)

func primary_code*(node: YunmuNode): int =
  if node.codes.len > 0: return node.codes[0]
  return -1

func is_yunmu_equiv*(a: YunmuNode, b: YunmuNode): bool =
  for ca in a.codes:
    for cb in b.codes:
      if ca == cb: return true
  return false

# ═══════════════════════════════════════════════════════════════
# 验证: 66符号全部唯一 + 配对正确
# ═══════════════════════════════════════════════════════════════
proc verify*(): tuple[ok: bool; msg: string] =
  var seen = initTable[string, int]()
  for i, sym in SYMBOLS66:
    if seen.hasKey(sym):
      return (false, "重复符号: " & sym & " at #" & $i & " and #" & $seen[sym])
    seen[sym] = i
  if seen.len != 66:
    return (false, "符号总数=" & $seen.len & " (需要66)")
  return (true, "66符号全部唯一 ✓")

# ═══════════════════════════════════════════════════════════════
# 测试
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══════════════════════════════════════════════════════════"
  echo "  最优66进制韵母编码 · 33韵母 × 2"
  echo "═══════════════════════════════════════════════════════════"

  let v = verify()
  echo "验证: ", v.msg
  echo ""

  var r = new_rongqi66("最优韵母")

  let yunmu_list = [
    "ai", "ei", "ao", "ou", "er", "ui",
    "ia", "ie", "ua", "uo", "iu", "üe", "iao", "uai",
    "an", "en", "in", "un", "ün",
    "ang", "eng", "ing", "ong",
    "ian", "uan", "üan", "iang", "uang",
    "iong", "ueng", "iou", "io", "ve",
  ]

  for ym in yunmu_list:
    r.add_yunmu(ym)

  echo "已装载韵母: ", r.nodes.len
  echo "码元总数: ", SYMBOLS66.len
  echo "进制: 66"
  echo ""

  echo "── 编码表 ──"
  for node in r.nodes:
    let c = primary_code(node)
    var syms = ""
    for s in node.symbols:
      if syms.len > 0: syms &= " · "
      syms &= s
    echo "  ", node.yunmu.alignLeft(6), " #", $c, "  ", syms

  echo ""
  echo "── 连续索引表 ──"
  for i, sym in SYMBOLS66:
    let ym = if r.idx_to_yunmu.hasKey(i): r.idx_to_yunmu[i] else: "?"
    echo "  #", $i, "  ", sym, "  ← ", ym
