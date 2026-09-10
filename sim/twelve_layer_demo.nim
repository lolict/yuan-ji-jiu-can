## 满全法 · 十二层全谱演示
## 前10层 + 三元见证3 + 分形递归1
## 码元: 212 + 3 + 1 = 216
## 完整满全法 OS 编码体系

# ═══════════════════════════════════════════════════════════════
# 层一~八: 基础七层 + 意识流8
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

type ColorEntry = object
  sym: string
  name: string

const COLOR42 = [
  ColorEntry(sym: "ạ", name: "深红"), ColorEntry(sym: "a̋", name: "大红"), ColorEntry(sym: "å", name: "浅红"),
  ColorEntry(sym: "ã", name: "粉红"), ColorEntry(sym: "â", name: "玫红"), ColorEntry(sym: "ă", name: "殷红"),
  ColorEntry(sym: "ẹ", name: "深橙"), ColorEntry(sym: "e̋", name: "橙色"), ColorEntry(sym: "e̊", name: "浅橙"),
  ColorEntry(sym: "ẽ", name: "杏色"), ColorEntry(sym: "ê", name: "琥珀"), ColorEntry(sym: "ĕ", name: "橘红"),
  ColorEntry(sym: "ị", name: "深黄"), ColorEntry(sym: "i̋", name: "明黄"), ColorEntry(sym: "i̊", name: "浅黄"),
  ColorEntry(sym: "ĩ", name: "鹅黄"), ColorEntry(sym: "î", name: "金黄"), ColorEntry(sym: "ĭ", name: "枯黄"),
  ColorEntry(sym: "ọ", name: "深绿"), ColorEntry(sym: "ő", name: "翠绿"), ColorEntry(sym: "o̊", name: "浅绿"),
  ColorEntry(sym: "õ", name: "草绿"), ColorEntry(sym: "ô", name: "墨绿"), ColorEntry(sym: "ŏ", name: "黄绿"),
  ColorEntry(sym: "ụ", name: "深青"), ColorEntry(sym: "ű", name: "青色"), ColorEntry(sym: "ů", name: "浅青"),
  ColorEntry(sym: "ũ", name: "天青"), ColorEntry(sym: "û", name: "藏青"), ColorEntry(sym: "ŭ", name: "青绿"),
  ColorEntry(sym: "ȃ", name: "深蓝"), ColorEntry(sym: "ȇ", name: "蓝色"), ColorEntry(sym: "ȋ", name: "浅蓝"),
  ColorEntry(sym: "ȏ", name: "蔚蓝"), ColorEntry(sym: "ȗ", name: "宝蓝"), ColorEntry(sym: "ȁ", name: "灰蓝"),
  ColorEntry(sym: "ȅ", name: "深紫"), ColorEntry(sym: "ȉ", name: "紫色"), ColorEntry(sym: "ȍ", name: "浅紫"),
  ColorEntry(sym: "ȕ", name: "薰衣草"), ColorEntry(sym: "ä", name: "紫罗兰"), ColorEntry(sym: "ö", name: "暗紫"),
]
proc color_pos*(s: string): int =
  for i, c in COLOR42:
    if c.sym == s: return i
  return -1

type SpaceEntry = object
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

type MirrorEntry = object
  dim: string
  label: string

const MIRROR10 = [
  MirrorEntry(dim: "声", label: "声音蓝本"),
  MirrorEntry(dim: "相", label: "形象相位"),
  MirrorEntry(dim: "廓", label: "轮廓"),
  MirrorEntry(dim: "尺", label: "尺寸"),
  MirrorEntry(dim: "粗", label: "粗细"),
  MirrorEntry(dim: "含", label: "包含逻辑"),
  MirrorEntry(dim: "色", label: "色差"),
  MirrorEntry(dim: "高", label: "高低"),
  MirrorEntry(dim: "宽", label: "宽度"),
  MirrorEntry(dim: "斜", label: "斜度"),
]
proc mirror_pos*(d: string): int =
  for i, m in MIRROR10:
    if m.dim == d: return i
  return 0

type MusicEntry = object
  layer: string
  label: string

const MUSIC3 = [
  MusicEntry(layer: "音符", label: "Atomic"),
  MusicEntry(layer: "动机", label: "Phrase"),
  MusicEntry(layer: "乐章", label: "Section"),
]
proc music_pos*(l: string): int =
  for i, m in MUSIC3:
    if m.layer == l: return i
  return 0

type ConsciousEntry = object
  flow: string
  glyph: string
  desc: string

const CONSCIOUS8 = [
  ConsciousEntry(flow: "预加载流", glyph: "x", desc: "预先填装"),
  ConsciousEntry(flow: "范畴流", glyph: "h", desc: "归类同异"),
  ConsciousEntry(flow: "聚类流", glyph: "j", desc: "颗粒聚拢"),
  ConsciousEntry(flow: "分类流", glyph: "f", desc: "层级分判"),
  ConsciousEntry(flow: "属性流", glyph: "s", desc: "特征切面"),
  ConsciousEntry(flow: "解释流", glyph: "y", desc: "意义展开"),
  ConsciousEntry(flow: "关系流", glyph: "g", desc: "节点衔接"),
  ConsciousEntry(flow: "概念流", glyph: "n", desc: "名相浮现"),
]
proc conscious_pos*(g: string): int =
  for i, c in CONSCIOUS8:
    if c.glyph == g: return i
  return 0

type FlowEntry = object
  attr: string
  glyph: string

const FLOW12 = [
  FlowEntry(attr: "速度", glyph: "速"),
  FlowEntry(attr: "向度", glyph: "向"),
  FlowEntry(attr: "密度", glyph: "密"),
  FlowEntry(attr: "温度", glyph: "温"),
  FlowEntry(attr: "压强", glyph: "压"),
  FlowEntry(attr: "清浊", glyph: "清"),
  FlowEntry(attr: "锐钝", glyph: "锐"),
  FlowEntry(attr: "曲直", glyph: "曲"),
  FlowEntry(attr: "深浅", glyph: "深"),
  FlowEntry(attr: "宽窄", glyph: "宽"),
  FlowEntry(attr: "盈虚", glyph: "盈"),
  FlowEntry(attr: "生灭", glyph: "生"),
]
proc flow_pos*(g: string): int =
  for i, f in FLOW12:
    if f.glyph == g: return i
  return 0

type IdentityEntry = object
  anchor: string
  glyph: string

const IDENTITY2 = [
  IdentityEntry(anchor: "楚恬", glyph: "0"),
  IdentityEntry(anchor: "我", glyph: "1"),
]
proc identity_pos*(g: string): int =
  for i, id in IDENTITY2:
    if id.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十一: 三元见证3 (WITNESS3)
# 见证 = 观察者 + 被观察者 + 裁定 (三元裁判)
# ═══════════════════════════════════════════════════════════════

type
  WitnessEntry = object
    role: string
    glyph: string

const WITNESS3 = [
  WitnessEntry(role: "观察者", glyph: "观"),
  WitnessEntry(role: "被观者", glyph: "被"),
  WitnessEntry(role: "裁定符", glyph: "裁"),
]
proc witness_pos*(g: string): int =
  for i, w in WITNESS3:
    if w.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十二: 分形1 (FRACTAL1)
# 自指递归结构 · 只有一个符号指回自身
# ═══════════════════════════════════════════════════════════════

const FRACTAL1 = ["∞"]
proc fractal_pos*(g: string): int =
  if g == "∞": return 0
  return 0

# ═══════════════════════════════════════════════════════════════
# 十二维容器
# ═══════════════════════════════════════════════════════════════

type
  Container12* = object
    id*: int
    sm*, ym*, tn*, col*, sp*, mir*, mus*, cog*, flw*, idt*, wit*, frac*: string

# ═══════════════════════════════════════════════════════════════
# 十二维积木选址
# ═══════════════════════════════════════════════════════════════

proc fq_address12*(sm, ym, tn, col, sp, mir, mus, cog, flw, idt, wit, frac: string, n: int): int =
  let a = sm_pos(sm)
  let b = ym_pos(ym)
  let c = tn_pos(tn)
  let d = color_pos(col)
  let e = space_pos(sp)
  let f = mirror_pos(mir)
  let g = music_pos(mus)
  let h = conscious_pos(cog)
  let fl = flow_pos(flw)
  let id = identity_pos(idt)
  let wi = witness_pos(wit)
  let fr = if frac == "∞": 0 else: 0
  if n == 0: return -1
  result = (a * b * 5 + c + b + d + e + f + g + h + fl + id + wi + fr) mod n

# ═══════════════════════════════════════════════════════════════
# 创建社会
# ═══════════════════════════════════════════════════════════════

proc create_society12*(count: int): seq[Container12] =
  var i = 0
  var idx: array[12, int]
  let space_names = ["东","南","西","北","天","地","头","胸","腰","脚","里","外","中","侧","手","正","背","肩"]
  let mir_dims = ["声","相","廓","尺","粗","含","色","高","宽","斜"]
  let mus_layers = ["音符","动机","乐章"]
  let cog_glyphs = ["x","h","j","f","s","y","g","n"]
  let flw_attrs = ["速","向","密","温","压","清","锐","曲","深","宽","盈","生"]
  let idt_glyphs = ["0","1"]
  let wit_glyphs = ["观","被","裁"]

  while i < count:
    result.add(Container12(
      id: i,
      sm: SM50[idx[0] mod SM50.len],
      ym: YM66[idx[1] mod YM66.len],
      tn: CLOCK60[idx[2] mod CLOCK60.len],
      col: COLOR42[idx[3] mod COLOR42.len].sym,
      sp: space_names[idx[4] mod space_names.len],
      mir: mir_dims[idx[5] mod mir_dims.len],
      mus: mus_layers[idx[6] mod mus_layers.len],
      cog: cog_glyphs[idx[7] mod cog_glyphs.len],
      flw: flw_attrs[idx[8] mod flw_attrs.len],
      idt: idt_glyphs[idx[9] mod idt_glyphs.len],
      wit: wit_glyphs[idx[10] mod wit_glyphs.len],
      frac: "∞",
    ))
    for j in 0..11: idx[j] += 1
    i += 1

when is_main_module:
  echo "═══ 满全法 · 十二层全谱 ═══"
  let layers = [
    ("FQ-SM50  ", SM50.len), ("FQ-YM66  ", YM66.len), ("CLOCK60  ", CLOCK60.len),
    ("COLOR42  ", COLOR42.len), ("SPACE18  ", SPACE18.len), ("MIRROR10 ", MIRROR10.len),
    ("MUSIC3   ", MUSIC3.len), ("CONSCIO8 ", CONSCIOUS8.len), ("FLOW12   ", FLOW12.len),
    ("IDENTITY2", IDENTITY2.len), ("WITNESS3 ", WITNESS3.len), ("FRACTAL1 ", 1),
  ]
  for l in layers:
    echo "  ", l[0], " : ", l[1]
  echo "  ──────────────────────────────"
  let total = SM50.len + YM66.len + CLOCK60.len + COLOR42.len + SPACE18.len + MIRROR10.len + MUSIC3.len + CONSCIOUS8.len + FLOW12.len + IDENTITY2.len + WITNESS3.len + 1
  echo "  总码元               : ", total
  echo ""

  const N = 64
  var s = create_society12(N)
  echo "预装 ", N, " 容器(前3):"
  for i in 0..<3:
    let c = s[i]
    echo "  #", c.id, " ", c.sm, c.ym, c.tn, " +", c.col, " @", c.sp, " ⟨", c.mir, c.mus, c.cog, "·", c.flw, c.idt, c.wit, c.frac, "⟩"

  echo ""
  echo "── 十二维积木选址 ──"
  let q = ("ȥ", "ɘ", "Ǔ", "ö", "中", "斜", "乐章", "n", "生", "1", "裁")
  let vs = [sm_pos(q[0]), ym_pos(q[1]), tn_pos(q[2]), color_pos(q[3]),
            space_pos(q[4]), mirror_pos(q[5]), music_pos(q[6]), conscious_pos(q[7]),
            flow_pos(q[8]), identity_pos(q[9]), witness_pos(q[10])]
  let num = vs[0]*vs[1]*5 + vs[2] + vs[1] + vs[3] + vs[4] + vs[5] + vs[6] + vs[7] + vs[8] + vs[9] + vs[10]
  let target = num mod N
  echo "num = ", num, " mod ", N, " = ", target
  if target >= 0:
    let h = s[target]
    echo "命中: #", h.id, " ⟨", h.wit, h.idt, "⟩"
