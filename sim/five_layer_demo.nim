## 满全法 · 五层整合演示
## 声母50 + 韵母66 + 时钟60 + 颜色42 + 空间18
## 每层独立命名 · 码元自有

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
# ═══════════════════════════════════════════════════════════════

type
  ColorEntry = object
    sym: string
    name: string

const COLOR42 = [
  ColorEntry(sym:"ạ", name:"深红"), ColorEntry(sym:"a̋", name:"大红"), ColorEntry(sym:"å", name:"浅红"),
  ColorEntry(sym:"ã", name:"粉红"), ColorEntry(sym:"â", name:"玫红"), ColorEntry(sym:"ă", name:"殷红"),
  ColorEntry(sym:"ẹ", name:"深橙"), ColorEntry(sym:"e̋", name:"橙色"), ColorEntry(sym:"e̊", name:"浅橙"),
  ColorEntry(sym:"ẽ", name:"杏色"), ColorEntry(sym:"ê", name:"琥珀"), ColorEntry(sym:"ĕ", name:"橘红"),
  ColorEntry(sym:"ị", name:"深黄"), ColorEntry(sym:"i̋", name:"明黄"), ColorEntry(sym:"i̊", name:"浅黄"),
  ColorEntry(sym:"ĩ", name:"鹅黄"), ColorEntry(sym:"î", name:"金黄"), ColorEntry(sym:"ĭ", name:"枯黄"),
  ColorEntry(sym:"ọ", name:"深绿"), ColorEntry(sym:"ő", name:"翠绿"), ColorEntry(sym:"o̊", name:"浅绿"),
  ColorEntry(sym:"õ", name:"草绿"), ColorEntry(sym:"ô", name:"墨绿"), ColorEntry(sym:"ŏ", name:"黄绿"),
  ColorEntry(sym:"ụ", name:"深青"), ColorEntry(sym:"ű", name:"青色"), ColorEntry(sym:"ů", name:"浅青"),
  ColorEntry(sym:"ũ", name:"天青"), ColorEntry(sym:"û", name:"藏青"), ColorEntry(sym:"ŭ", name:"青绿"),
  ColorEntry(sym:"ȃ", name:"深蓝"), ColorEntry(sym:"ȇ", name:"蓝色"), ColorEntry(sym:"ȋ", name:"浅蓝"),
  ColorEntry(sym:"ȏ", name:"蔚蓝"), ColorEntry(sym:"ȗ", name:"宝蓝"), ColorEntry(sym:"ȁ", name:"灰蓝"),
  ColorEntry(sym:"ȅ", name:"深紫"), ColorEntry(sym:"ȉ", name:"紫色"), ColorEntry(sym:"ȍ", name:"浅紫"),
  ColorEntry(sym:"ȕ", name:"薰衣草"), ColorEntry(sym:"ä", name:"紫罗兰"), ColorEntry(sym:"ö", name:"暗紫"),
]
proc color_pos*(s: string): int =
  for i, c in COLOR42:
    if c.sym == s: return i
  return -1

# ═══════════════════════════════════════════════════════════════
# 层五: 空间18 (SPACE18 · 6绝对×3人体×里外中)
# 编码绝对方向6 + 人体锚点9 + 分三类 + 斜角
# ═══════════════════════════════════════════════════════════════

type
  SpaceEntry = object
    name: string
    ax, ay, az: int

const SPACE18 = [
  SpaceEntry(name: "东", ax: 1, ay: 0, az: 0),
  SpaceEntry(name: "西", ax: -1, ay: 0, az: 0),
  SpaceEntry(name: "南", ax: 0, ay: 1, az: 0),
  SpaceEntry(name: "北", ax: 0, ay: -1, az: 0),
  SpaceEntry(name: "天", ax: 0, ay: 0, az: 1),
  SpaceEntry(name: "地", ax: 0, ay: 0, az: -1),
  SpaceEntry(name: "头", ax: 0, ay: 0, az: 2),
  SpaceEntry(name: "肩", ax: 0, ay: 0, az: 1),
  SpaceEntry(name: "胸", ax: 0, ay: 0, az: 0),
  SpaceEntry(name: "腰", ax: 0, ay: 0, az: -1),
  SpaceEntry(name: "脚", ax: 0, ay: 0, az: -2),
  SpaceEntry(name: "侧", ax: -1, ay: 0, az: 0),
  SpaceEntry(name: "手", ax: 1, ay: 0, az: 0),
  SpaceEntry(name: "正", ax: 0, ay: 1, az: 0),
  SpaceEntry(name: "背", ax: 0, ay: -1, az: 0),
  SpaceEntry(name: "里", ax: 0, ay: 0, az: 0),
  SpaceEntry(name: "外", ax: 2, ay: 0, az: 0),
  SpaceEntry(name: "中", ax: 0, ay: 0, az: 0),
]
proc space_pos*(n: string): int =
  for i, s in SPACE18:
    if s.name == n: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 五维容器
# ═══════════════════════════════════════════════════════════════

type
  Container5* = object
    id*: int
    sm*: string
    ym*: string
    tn*: string
    col*: string
    sp*: string       # 空间方位

# ═══════════════════════════════════════════════════════════════
# 五维积木选址
# 址 = (sp × yp × 5 + tp + yp + cp + ap) mod N
# ═══════════════════════════════════════════════════════════════

proc fq_address5*(sm, ym, tn, col, sp: string, n: int): int =
  let a = sm_pos(sm)
  let b = ym_pos(ym)
  let c = tn_pos(tn)
  let d = color_pos(col)
  let e = space_pos(sp)
  if n == 0: return -1
  result = (a * b * 5 + c + b + d + e) mod n

# ═══════════════════════════════════════════════════════════════
# 创建社会
# ═══════════════════════════════════════════════════════════════

proc create_society5*(count: int): seq[Container5] =
  var i = 0
  var sm_i, ym_i, tn_i, col_i, sp_i = 0
  let space_names = ["东","南","西","北","天","地","头","胸","腰","脚","里","外","中","侧","手","正","背","肩"]

  while i < count:
    result.add(Container5(
      id: i,
      sm: SM50[sm_i mod SM50.len],
      ym: YM66[ym_i mod YM66.len],
      tn: CLOCK60[tn_i mod CLOCK60.len],
      col: COLOR42[col_i mod COLOR42.len].sym,
      sp: space_names[sp_i mod space_names.len],
    ))
    i += 1
    sm_i += 1
    ym_i += 1
    tn_i += 1
    col_i += 1
    sp_i += 1

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════

when is_main_module:
  echo "═══ 满全法 · 五层整合 ═══"
  echo "  FQ-SM50  声母 : ", SM50.len
  echo "  FQ-YM66  韵母 : ", YM66.len
  echo "  CLOCK60  时钟 : ", CLOCK60.len
  echo "  COLOR42  颜色 : ", COLOR42.len
  echo "  SPACE18  空间 : ", SPACE18.len
  echo "  ──────────────────────"
  echo "  总码元         : ", SM50.len + YM66.len + CLOCK60.len + COLOR42.len + SPACE18.len
  echo ""

  const N = 31
  var s = create_society5(N)
  echo "预装 ", N, " 容器(前5):"
  for i in 0..<5:
    let c = s[i]
    echo "  #", c.id, " ", c.sm, c.ym, c.tn, " + ", c.col, " @", c.sp

  echo ""
  echo "── 五维积木选址 ──"
  let sm_q = "h"
  let ym_q = "i"
  let tn_q = "à"
  let col_q = "ọ"
  let sp_q  = "北"
  let target = fq_address5(sm_q, ym_q, tn_q, col_q, sp_q, N)
  let a = sm_pos(sm_q)
  let b = ym_pos(ym_q)
  let c = tn_pos(tn_q)
  let d = color_pos(col_q)
  let e = space_pos(sp_q)
  echo "编码: sm=", a, " ym=", b, " tn=", c, " col=", d, " sp=", e
  echo "公式: (", a, "×", b, "×5 + ", c, " + ", b, " + ", d, " + ", e, ") mod ", N
  let num = a * b * 5 + c + b + d + e
  echo "    = ", num, " mod ", N, " = ", target
  if target >= 0:
    let hit = s[target]
    echo "命中: #", hit.id, " [", hit.sm, hit.ym, hit.tn, " + ", hit.col, " @", hit.sp, "]"
