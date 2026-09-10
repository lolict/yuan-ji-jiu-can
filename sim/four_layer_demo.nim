## 满全法 · 四层整合演示
## 声母50 + 韵母66 + 时钟60 + 颜色42
## 每层独立命名 · 码元自有 · 无外借

# ═══════════════════════════════════════════════════════════════
# 层一: 声母50 (FQ-SM50)
# ═══════════════════════════════════════════════════════════════

const SM50 = [
  "b","p","m","f","d","t","n","l",
  "g","k","h","w","j","q","x","y",
  "z","c","s","r","ȥ","ç","ş","ŋ","v",
]
proc sm_pos*(s: string): int =
  for i, v in SM50:
    if v == s: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层二: 韵母66 (FQ-YM66)
# ═══════════════════════════════════════════════════════════════

const YM66 = [
  "a","ȍ","ӗ","i","u","ü",
  "ꞟ","ɘ","ɒ","౪","ɛ","ѧ",
  "ɤ","ʉ","ꞛ","ã","ẽ","ĩ",
  "ʯ","ą","ę","į","๓","ⱄ",
  "⊰","ʌ","ɔ","и","ǡ","ѽ",
  "ÿ","ȳ",
]
proc ym_pos*(y: string): int =
  for i, v in YM66:
    if v == y: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层三: 时钟60 (CLOCK60)
# ═══════════════════════════════════════════════════════════════

const CLOCK60 = [
  "a","e","i","o","u","ü",
  "A","E","I","O","U","Ü",
  "ā","ē","ī","ō","ū","ǖ",
  "Ā","Ē","Ī","Ō","Ū","Ǖ",
  "á","é","í","ó","ú","ǘ",
  "Á","É","Í","Ó","Ú","Ǘ",
  "ǎ","ě","ǐ","ǒ","ǔ","ǚ",
  "Ǎ","Ě","Ǐ","Ǒ","Ǔ","Ǚ",
  "à","è","ì","ò","ù","ǜ",
  "À","È","Ì","Ò","Ù","Ǜ",
]
proc tn_pos*(t: string): int =
  for i, v in CLOCK60:
    if v == t: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层四: 颜色42 (COLOR42)
# 7主色 × 6色阶 = 42
# ═══════════════════════════════════════════════════════════════

const COLOR42 = [
  # 红 (0-5)
  (sym:"ạ", name:"深红", emo:"激情"),
  (sym:"a̋", name:"大红", emo:"兴奋"),
  (sym:"å", name:"浅红", emo:"温柔"),
  (sym:"ã", name:"粉红", emo:"羞怯"),
  (sym:"â", name:"玫红", emo:"诱惑"),
  (sym:"ă", name:"殷红", emo:"危险"),
  # 橙 (6-11)
  (sym:"ẹ", name:"深橙", emo:"温暖"),
  (sym:"e̋", name:"橙色", emo:"活力"),
  (sym:"e̊", name:"浅橙", emo:"明亮"),
  (sym:"ẽ", name:"杏色", emo:"柔和"),
  (sym:"ê", name:"琥珀", emo:"神秘"),
  (sym:"ĕ", name:"橘红", emo:"急切"),
  # 黄 (12-17)
  (sym:"ị", name:"深黄", emo:"明朗"),
  (sym:"i̋", name:"明黄", emo:"快乐"),
  (sym:"i̊", name:"浅黄", emo:"纯净"),
  (sym:"ĩ", name:"鹅黄", emo:"天真"),
  (sym:"î", name:"金黄", emo:"辉煌"),
  (sym:"ĭ", name:"枯黄", emo:"衰败"),
  # 绿 (18-23)
  (sym:"ọ", name:"深绿", emo:"宁静"),
  (sym:"ő", name:"翠绿", emo:"生机"),
  (sym:"o̊", name:"浅绿", emo:"清新"),
  (sym:"õ", name:"草绿", emo:"舒适"),
  (sym:"ô", name:"墨绿", emo:"深沉"),
  (sym:"ŏ", name:"黄绿", emo:"不安"),
  # 青 (24-29)
  (sym:"ụ", name:"深青", emo:"冷静"),
  (sym:"ű", name:"青色", emo:"理性"),
  (sym:"ů", name:"浅青", emo:"透明"),
  (sym:"ũ", name:"天青", emo:"忧郁"),
  (sym:"û", name:"藏青", emo:"严肃"),
  (sym:"ŭ", name:"青绿", emo:"矛盾"),
  # 蓝 (30-35)
  (sym:"ȃ", name:"深蓝", emo:"深邃"),
  (sym:"ȇ", name:"蓝色", emo:"平和"),
  (sym:"ȋ", name:"浅蓝", emo:"自由"),
  (sym:"ȏ", name:"蔚蓝", emo:"梦想"),
  (sym:"ȗ", name:"宝蓝", emo:"高贵"),
  (sym:"ȁ", name:"灰蓝", emo:"忧郁"),
  # 紫 (36-41)
  (sym:"ȅ", name:"深紫", emo:"神秘"),
  (sym:"ȉ", name:"紫色", emo:"优雅"),
  (sym:"ȍ", name:"浅紫", emo:"浪漫"),
  (sym:"ȕ", name:"薰衣草", emo:"治愈"),
  (sym:"ä", name:"紫罗兰", emo:"灵性"),
  (sym:"ö", name:"暗紫", emo:"压抑"),
]
proc color_pos*(s: string): int =
  for i, c in COLOR42:
    if c.sym == s: return i
  return -1

# ═══════════════════════════════════════════════════════════════
# 容器结构 · 四层坐标
# ═══════════════════════════════════════════════════════════════

type
  Container4* = object
    id*: int
    sm*: string       # 声母
    ym*: string       # 韵母
    tn*: string       # 时钟(声调)
    col*: string      # 颜色
    tier*: int        # 层级(由tick派生)

# ═══════════════════════════════════════════════════════════════
# 四维积木选址
# 址 = (sp × yp × 5 + tp + yp + cp) mod N
# ═══════════════════════════════════════════════════════════════

proc fq_address4*(sm, ym, tn, col: string, n: int): int =
  let sp = sm_pos(sm)
  let yp = ym_pos(ym)
  let tp = tn_pos(tn)
  let cp = color_pos(col)
  if n == 0: return -1
  result = (sp * yp * 5 + tp + yp + cp) mod n

# ═══════════════════════════════════════════════════════════════
# 创建社会 · 预装四层编码容器
# ═══════════════════════════════════════════════════════════════

proc create_society4*(count: int): seq[Container4] =
  var sm_i, ym_i, tn_i, col_i = 0
  for i in 0..<count:
    result.add(Container4(
      id: i,
      sm: SM50[sm_i mod SM50.len],
      ym: YM66[ym_i mod YM66.len],
      tn: CLOCK60[tn_i mod CLOCK60.len],
      col: COLOR42[col_i mod COLOR42.len].sym,
      tier: 1 + (i mod 3),
    ))
    sm_i += 1
    ym_i += 1
    tn_i += 1
    col_i += 1

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════

when is_main_module:
  echo "═══ 满全法 · 四层整合模拟 ═══"
  echo "  声母层 FQ-SM50 : ", SM50.len, " 码元"
  echo "  韵母层 FQ-YM66 : ", YM66.len, " 码元"
  echo "  时钟层 CLOCK60  : ", CLOCK60.len, " 码元"
  echo "  颜色层 COLOR42 : ", COLOR42.len, " 码元(7色×6阶)"
  echo "  总码元池       : ", SM50.len + YM66.len + CLOCK60.len + COLOR42.len
  echo ""

  const N = 23
  var s = create_society4(N)
  echo "预装 ", N, " 容器:"
  for c in s.items:
    let cp = color_pos(c.col)
    let cp_name = if cp >= 0: COLOR42[cp].name else: "?"
    let cp_emo = if cp >= 0: COLOR42[cp].emo else: "?"
    echo "  #", c.id, " [L", c.tier, "] ",
        c.sm, c.ym, c.tn, " + ", c.col, "(", cp_name, "/", cp_emo, ")"

  echo ""
  echo "── 四维积木选址 ──"
  let sm_q = "t"
  let ym_q = "i"
  let tn_q = "á"
  let col_q = "ọ"
  let target = fq_address4(sm_q, ym_q, tn_q, col_q, N)
  let sp = sm_pos(sm_q)
  let yp = ym_pos(ym_q)
  let tp = tn_pos(tn_q)
  let cp = color_pos(col_q)
  echo "输入: 声母='", sm_q, "'", " 韵母='", ym_q, "'",
      " 时钟='", tn_q, "'", " 颜色='", col_q, "'"
  echo "位置: sp=", sp, " yp=", yp, " tp=", tp, " cp=", cp
  let numerator = sp * yp * 5 + tp + yp + cp
  echo "公式: (", sp, "×", yp, "×5 + ", tp, " + ", yp, " + ", cp, ") mod ", N
  echo "    = ", numerator, " mod ", N
  echo "    = ", target

  if target >= 0:
    let c = s[target]
    echo "命中: #", c.id, " [L", c.tier, "] ", c.sm, c.ym, c.tn, " + ", c.col

  echo ""
  echo "── 7主色 × 6色阶 完整 ──"
  for i, c in COLOR42:
    echo "  #", i, " ", c.sym, "\t", c.name, "\t", c.emo
