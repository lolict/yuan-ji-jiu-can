## ═══════════════════════════════════════════════════════════════
## 双拼编码方案 · 66韵母 + 修饰符号
## ═══════════════════════════════════════════════════════════════
## 设计:
##   韵母主系统66符号 = 第一层 (33韵母 × 2)
##   集装箱0/1/2/5的50符号 = 第二层 (修饰/调音)
##
## 组合方式: [韵母] + [修饰] = 双拼音节
##   ꬰ = ai(第一层)
##   ꬰ + ạ = ai+下点(第二层) = 特定调值/变体
##   ꬰ + ȃ = ai+倒短音(第二层) = 另一种变体
##
## 不是切割韵母系统 · 而是给韵母系统加修饰层
## ═══════════════════════════════════════════════════════════════

import tables, strutils

# ═══ 50个修饰符号 (来自集装箱0/1/2/5) ═══════════════════════
const MODIFIERS* = [
  # ── 集装箱0 (10) ──
  "ạ", "ẹ", "ị", "ọ", "ụ",     # 下点组
  "ä", "ë", "ï", "ö", "ü",      # 分音组
  # ── 集装箱1 (10) ──
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ",     # 倒短音组
  "a̋", "e̋", "i̋", "ő", "ű",     # 双重音组
  # ── 集装箱2 (10) ──
  "å", "e̊", "i̊", "o̊", "ů",     # 上圈组
  "ą", "ę", "į", "ǫ", "ų",     # 反尾组
  # ── 集装箱5 (20) ──
  "ã", "ẽ", "ĩ", "õ", "ũ",     # 波浪组
  "â", "ê", "î", "ô", "û",     # 帽子组
  "ă", "ĕ", "ĭ", "ŏ", "ŭ",     # 短音组
  "ȁ", "ȅ", "ȉ", "ȍ", "ȕ",     # 反钝音组
]

# ── 按基元分组 (a/o/e/i/u/ü) ────────────────────────────────
const MOD_A* = [
  "ạ", "ä", "ȃ", "a̋", "å", "ą", "ã", "â", "ă", "ȁ"]
const MOD_O* = [
  "ọ", "ö", "ȏ", "ő", "o̊", "ǫ", "õ", "ô", "ŏ", "ȍ"]
const MOD_E* = [
  "ẹ", "ë", "ȇ", "e̋", "e̊", "ę", "ẽ", "ê", "ĕ", "ȅ"]
const MOD_I* = [
  "ị", "ï", "ȋ", "i̋", "i̊", "į", "ĩ", "î", "ĭ", "ȉ"]
const MOD_U* = [
  "ụ", "ü", "ȗ", "ű", "ů", "ų", "ũ", "û", "ŭ", "ȕ"]

# ═══ 66韵母符号 (主系统) ════════════════════════════════════
const YUNMU66* = [
  "ꬰ", "ꬱ", "ӭ", "Ӭ", "ꜵ", "Ꜵ", "ȣ", "Ȣ", "Ʝ", "Ɉ",
  "ə", "Ə", "ѱ", "Ѱ", "ꙗ", "Ꙗ", "Ꙋ", "Ɣ", "ǟ", "Ǟ",
  "ȫ", "Ȫ", "Ⴏ", "Ⴎ", "ᵫ", "ꭂ", "ꮂ", "ᴥ", "ꜻ", "Ꜻ",
  "ꙙ", "Ꙙ", "ⱅ", "ⱄ", "ႁ", "႔", "տ", "໙", "Ⴐ", "Ⴑ",
  "Ⴆ", "Ⴊ", "ӫ", "Ӫ", "ꙟ", "Ꙟ", "ꭁ", "ꭀ", "ꙝ", "Ꙝ",
  "Ⴄ", "Ⴂ", "ѫ", "Ѫ", "ѭ", "Ѭ", "ꙛ", "Ꙛ", "ȱ", "Ȱ",
  "ɞ", "ɝ", "ø", "Ø", "ю", "Ю",
]

# ═══ 双拼组合类型 ═══════════════════════════════════════════
type
  ShuangPin* = object
    yunmu*: string        # 韵母名
    yunmu_sym*: string    # 韵母符号 (第一层)
    mod_grp*: string      # 修饰组 (a/o/e/i/u)
    mod_sym*: string      # 修饰符号 (第二层)
    combo*: string        # 组合形式: 韵母[+修饰]
    meaning*: string      # 语义/用途

  ShuangPinSystem* = object
    name*: string
    yunmu_count*: int
    mod_count*: int
    combos*: seq[ShuangPin]

func new_shuangpin*(): ShuangPinSystem =
  result.name = "双拼编码"
  result.yunmu_count = YUNMU66.len
  result.mod_count = MODIFIERS.len

  # 生成双拼组合 (选取有意义的组合)
  # 规则: 韵母符号 + 该韵母主要元音的修饰
  let yunmu_names = [
    "ai", "ei", "ao", "ou", "er", "ui", "ia", "ie",
    "ua", "uo", "iu", "üe", "iao", "uai", "an", "en",
    "in", "un", "ün", "ang", "eng", "ing", "ong",
    "ian", "uan", "üan", "iang", "uang", "iong", "ueng", "iou", "io",
  ]
  let yunmu_vowel = [
    "a", "e", "o", "o", "e", "u", "i", "i",
    "a", "o", "i", "ü", "a", "a", "a", "e",
    "i", "u", "u", "a", "e", "i", "o",
    "a", "a", "ü", "a", "a", "o", "o", "o", "i",
  ]

  for i in 0..<min(yunmu_names.len, YUNMU66.len):
    var sp: ShuangPin
    sp.yunmu = yunmu_names[i]
    sp.yunmu_sym = YUNMU66[i*2]
    let vowel = yunmu_vowel[i]

    # 选择该韵母主元音的修饰符号
    case vowel:
    of "a":
      sp.mod_grp = "a"
      sp.mod_sym = MOD_A[0]    # ạ a+下点
    of "e":
      sp.mod_grp = "e"
      sp.mod_sym = MOD_E[0]    # ẹ e+下点
    of "i":
      sp.mod_grp = "i"
      sp.mod_sym = MOD_I[0]    # ị i+下点
    of "o":
      sp.mod_grp = "o"
      sp.mod_sym = MOD_O[0]    # ọ o+下点
    of "u":
      sp.mod_grp = "u"
      sp.mod_sym = MOD_U[0]    # ụ u+下点
    of "ü":
      sp.mod_grp = "ü"
      sp.mod_sym = "ü"
    else:
      sp.mod_grp = "a"
      sp.mod_sym = MOD_A[0]

    sp.combo = sp.yunmu_sym & "+" & sp.mod_sym
    sp.meaning = sp.yunmu & "+修饰"
    result.combos.add(sp)

  # 直读形式 (无修饰 · 仅韵母本身)
  for i in 0..<min(yunmu_names.len, YUNMU66.len):
    var sp: ShuangPin
    sp.yunmu = yunmu_names[i]
    sp.yunmu_sym = YUNMU66[i*2]
    sp.mod_grp = ""
    sp.mod_sym = ""
    sp.combo = sp.yunmu_sym
    sp.meaning = yunmu_names[i] & "(直读)"
    result.combos.add(sp)

proc show*(sys: ShuangPinSystem) =
  for c in sys.combos:
    echo "  ", c.yunmu.alignLeft(5),
         " ", c.combo.alignLeft(12),
         " ← ", c.meaning

when is_main_module:
  echo "═══════════════════════════════════════════════════════════"
  echo "  双拼编码方案 · 66韵母 + 50修饰"
  echo "═══════════════════════════════════════════════════════════"

  let sys = new_shuangpin()
  echo "韵母层: ", sys.yunmu_count, " 符号"
  echo "修饰层: ", sys.mod_count, " 符号"
  echo "总组合: ", sys.yunmu_count, "+", sys.combos.len, "双拼形式"
  echo ""

  echo "── 双拼组合表 ──"
  sys.show()
  echo ""

  echo "── 基元修饰表 ──"
  echo "  a族: ", MOD_A.join(" ")
  echo "  e族: ", MOD_E.join(" ")
  echo "  i族: ", MOD_I.join(" ")
  echo "  o族: ", MOD_O.join(" ")
  echo "  u族: ", MOD_U.join(" ")
  echo ""
  echo "  a族10 + e族10 + i族10 + o族10 + u族10 = 修饰50"
  echo "  韵母66 + 修饰50 = 总编码116"
