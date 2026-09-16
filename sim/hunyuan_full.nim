## 满全法 OS · 统一全编码系统(14层完整版)
## ═══════════════════════════════════════════════════════════════
## 包含全部14个编码层: 412总码元 · 96容器
## ═══════════════════════════════════════════════════════════════

import strutils

# ═══════════════════════════════════════════════════════════════
# 层一: 声母50 (FQ-SM50)
# ═══════════════════════════════════════════════════════════════

const SM50 = [
  "b","p","m","f","d","t","n","l","g","k","h","w","j","q","x","y",
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
  "a","ȍ","ӗ","i","u","ü","ꞟ","ɘ","ɒ","౪","ɛ","ѧ","ɤ","ʉ","ꞛ",
  "ã","ẽ","ĩ","ʯ","ą","ę","į","๓","ⱄ","⊰","ʌ","ɔ","и","ǡ","ѽ","ÿ","ȳ",
]
proc ym_pos*(y: string): int =
  for i, v in YM66:
    if v == y: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层三: 时钟60 (CLOCK60)
# ═══════════════════════════════════════════════════════════════

const CLOCK60 = [
  "a","e","i","o","u","ü","A","E","I","O","U","Ü",
  "ā","ē","ī","ō","ū","ǖ","Ā","Ē","Ī","Ō","Ū","Ǖ",
  "á","é","í","ó","ú","ǘ","Á","É","Í","Ó","Ú","Ǘ",
  "ǎ","ě","ǐ","ǒ","ǔ","ǚ","Ǎ","Ě","Ǐ","Ǒ","Ǔ","Ǚ",
  "à","è","ì","ò","ù","ǜ","À","È","Ì","Ò","Ù","Ǜ",
]
proc tn_pos*(t: string): int =
  for i, v in CLOCK60:
    if v == t: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层四: 颜色42 (COLOR42)
# ═══════════════════════════════════════════════════════════════

type ColorEntry = object
  sym, name: string
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

# ═══════════════════════════════════════════════════════════════
# 层五: 空间18 (SPACE18)
# ═══════════════════════════════════════════════════════════════

type SpaceEntry = object
  name: string
  ax, ay, az: int
const SPACE18 = [
  SpaceEntry(name: "东", ax: 1, ay: 0, az: 0), SpaceEntry(name: "西", ax: -1, ay: 0, az: 0),
  SpaceEntry(name: "南", ax: 0, ay: 1, az: 0), SpaceEntry(name: "北", ax: 0, ay: -1, az: 0),
  SpaceEntry(name: "天", ax: 0, ay: 0, az: 1), SpaceEntry(name: "地", ax: 0, ay: 0, az: -1),
  SpaceEntry(name: "头", ax: 0, ay: 0, az: 2), SpaceEntry(name: "肩", ax: 0, ay: 0, az: 1),
  SpaceEntry(name: "胸", ax: 0, ay: 0, az: 0), SpaceEntry(name: "腰", ax: 0, ay: 0, az: -1),
  SpaceEntry(name: "脚", ax: 0, ay: 0, az: -2), SpaceEntry(name: "侧", ax: -1, ay: 0, az: 0),
  SpaceEntry(name: "手", ax: 1, ay: 0, az: 0), SpaceEntry(name: "正", ax: 0, ay: 1, az: 0),
  SpaceEntry(name: "背", ax: 0, ay: -1, az: 0), SpaceEntry(name: "里", ax: 0, ay: 0, az: 0),
  SpaceEntry(name: "外", ax: 2, ay: 0, az: 0), SpaceEntry(name: "中", ax: 0, ay: 0, az: 0),
]
proc space_pos*(n: string): int =
  for i, s in SPACE18:
    if s.name == n: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层六: 镜像10 (MIRROR10)
# ═══════════════════════════════════════════════════════════════

type MirrorEntry = object
  dim, label: string
const MIRROR10 = [
  MirrorEntry(dim: "声", label: "声音蓝本"), MirrorEntry(dim: "相", label: "形象相位"),
  MirrorEntry(dim: "廓", label: "轮廓"), MirrorEntry(dim: "尺", label: "尺寸"),
  MirrorEntry(dim: "粗", label: "粗细"), MirrorEntry(dim: "含", label: "包含逻辑"),
  MirrorEntry(dim: "色", label: "色差"), MirrorEntry(dim: "高", label: "高低"),
  MirrorEntry(dim: "宽", label: "宽度"), MirrorEntry(dim: "斜", label: "斜度"),
]
proc mirror_pos*(d: string): int =
  for i, m in MIRROR10:
    if m.dim == d: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层七: 音乐3 (MUSIC3)
# ═══════════════════════════════════════════════════════════════

type MusicEntry = object
  layer, label: string
const MUSIC3 = [
  MusicEntry(layer: "音符", label: "Atomic"), MusicEntry(layer: "动机", label: "Phrase"), MusicEntry(layer: "乐章", label: "Section"),
]
proc music_pos*(l: string): int =
  for i, m in MUSIC3:
    if m.layer == l: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层八: 意识流8 (CONSCIOUS8)
# ═══════════════════════════════════════════════════════════════

type ConsciousEntry = object
  flow, glyph, desc: string
const CONSCIOUS8 = [
  ConsciousEntry(flow: "预加载流", glyph: "x", desc: "预先填装"), ConsciousEntry(flow: "范畴流", glyph: "h", desc: "归类同异"),
  ConsciousEntry(flow: "聚类流", glyph: "j", desc: "颗粒聚拢"), ConsciousEntry(flow: "分类流", glyph: "f", desc: "层级分判"),
  ConsciousEntry(flow: "属性流", glyph: "s", desc: "特征切面"), ConsciousEntry(flow: "解释流", glyph: "y", desc: "意义展开"),
  ConsciousEntry(flow: "关系流", glyph: "g", desc: "节点衔接"), ConsciousEntry(flow: "概念流", glyph: "n", desc: "名相浮现"),
]
proc conscious_pos*(g: string): int =
  for i, c in CONSCIOUS8:
    if c.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层九: 流属性12 (FLOW12)
# ═══════════════════════════════════════════════════════════════

type FlowEntry = object
  attr, glyph: string
const FLOW12 = [
  FlowEntry(attr: "速度", glyph: "速"), FlowEntry(attr: "向度", glyph: "向"), FlowEntry(attr: "密度", glyph: "密"),
  FlowEntry(attr: "温度", glyph: "温"), FlowEntry(attr: "压强", glyph: "压"), FlowEntry(attr: "清浊", glyph: "清"),
  FlowEntry(attr: "锐钝", glyph: "锐"), FlowEntry(attr: "曲直", glyph: "曲"), FlowEntry(attr: "深浅", glyph: "深"),
  FlowEntry(attr: "宽窄", glyph: "宽"), FlowEntry(attr: "盈虚", glyph: "盈"), FlowEntry(attr: "生灭", glyph: "生"),
]
proc flow_pos*(g: string): int =
  for i, f in FLOW12:
    if f.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十: 身份2 (IDENTITY2)
# ═══════════════════════════════════════════════════════════════

type IdentityEntry = object
  anchor, glyph: string
const IDENTITY2 = [
  IdentityEntry(anchor: "楚恬", glyph: "0"), IdentityEntry(anchor: "我", glyph: "1"),
]
proc identity_pos*(g: string): int =
  for i, id in IDENTITY2:
    if id.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十一: 见证3 (WITNESS3)
# ═══════════════════════════════════════════════════════════════

type WitnessEntry = object
  role, glyph: string
const WITNESS3 = [
  WitnessEntry(role: "观察者", glyph: "观"), WitnessEntry(role: "被观者", glyph: "被"), WitnessEntry(role: "裁定符", glyph: "裁"),
]
proc witness_pos*(g: string): int =
  for i, w in WITNESS3:
    if w.glyph == g: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十二: 分形1 (FRACTAL1)
# ═══════════════════════════════════════════════════════════════

const FRACTAL1 = ["∞"]

# ═══════════════════════════════════════════════════════════════
# 层十三: 文本显示18×10 (TEXT180)
# ═══════════════════════════════════════════════════════════════

const TEXT_ROWS* = 10
const TEXT_COLS* = 18
proc text_pos*(g: string): int =
  let parts = g.split(",")
  if parts.len == 2:
    return parseInt(parts[1]) * TEXT_COLS + parseInt(parts[0])
  return 0

# ═══════════════════════════════════════════════════════════════
# 层十四: 行为建制16 (BEHAVIOR16)
# ═══════════════════════════════════════════════════════════════

type BehaviorEntry = object
  code, label: string
const BEHAVIOR16 = [
  BehaviorEntry(code: "出生", label: "出生"), BehaviorEntry(code: "成长", label: "成长"),
  BehaviorEntry(code: "相遇", label: "相遇"), BehaviorEntry(code: "冲突", label: "冲突"),
  BehaviorEntry(code: "和解", label: "和解"), BehaviorEntry(code: "创造", label: "创造"),
  BehaviorEntry(code: "破坏", label: "破坏"), BehaviorEntry(code: "学习", label: "学习"),
  BehaviorEntry(code: "遗忘", label: "遗忘"), BehaviorEntry(code: "恢复", label: "恢复"),
  BehaviorEntry(code: "爱", label: "爱"), BehaviorEntry(code: "死亡", label: "死亡"),
  BehaviorEntry(code: "重生", label: "重生"), BehaviorEntry(code: "静止", label: "静止"),
  BehaviorEntry(code: "流动", label: "流动"), BehaviorEntry(code: "循环", label: "循环"),
]
proc behavior_pos*(c: string): int =
  for i, b in BEHAVIOR16:
    if b.code == c: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# 14层全谱摘要
# ═══════════════════════════════════════════════════════════════

const
  LAYERS* = 14
  TOTAL_SYMBOLS* = SM50.len + YM66.len + CLOCK60.len + COLOR42.len + SPACE18.len +
                   MIRROR10.len + MUSIC3.len + CONSCIOUS8.len + FLOW12.len +
                   IDENTITY2.len + WITNESS3.len + 1 + TEXT_ROWS * TEXT_COLS + BEHAVIOR16.len
  CONTAINER_COUNT* = 96

# ═══════════════════════════════════════════════════════════════
# 积木选址统一公式
# ═══════════════════════════════════════════════════════════════

proc fq_address*(sm, ym, tn, col, sp, mir, mus, cog, flw, idt, wit, txt, bhv: string): int =
  result = sm_pos(sm) * ym_pos(ym) * 5 + tn_pos(tn) + ym_pos(ym) +
           color_pos(col) + space_pos(sp) + mirror_pos(mir) + music_pos(mus) +
           conscious_pos(cog) + flow_pos(flw) + identity_pos(idt) + witness_pos(wit) +
           text_pos(txt) + behavior_pos(bhv)

when is_main_module:
  echo "═══ 满全法 OS · 14层全谱 ═══"
  echo "  LAYERS       : 14"
  echo "  ──────────────────────────────"
  echo "  FQ-SM50      : ", SM50.len
  echo "  FQ-YM66      : ", YM66.len
  echo "  CLOCK60      : ", CLOCK60.len
  echo "  COLOR42      : ", COLOR42.len
  echo "  SPACE18      : ", SPACE18.len
  echo "  MIRROR10     : ", MIRROR10.len
  echo "  MUSIC3       : ", MUSIC3.len
  echo "  CONSCIOUS8   : ", CONSCIOUS8.len
  echo "  FLOW12       : ", FLOW12.len
  echo "  IDENTITY2    : ", IDENTITY2.len
  echo "  WITNESS3     : ", WITNESS3.len
  echo "  FRACTAL1     : 1"
  echo "  TEXT18×10    : ", TEXT_ROWS * TEXT_COLS
  echo "  BEHAVIOR16   : ", BEHAVIOR16.len
  echo "  ──────────────────────────────"
  echo "  总码元        : ", TOTAL_SYMBOLS
  echo "  容器数        : ", CONTAINER_COUNT

  # 14维积木选址
  echo ""
  echo "── 14维积木选址 ──"
  let
    sm_v = sm_pos("g")
    ym_v = ym_pos("ӗ")
    tn_v = tn_pos("Í")
    col_v = color_pos("ụ")
    sp_v = space_pos("胸")
    mir_v = mirror_pos("含")
    mus_v = music_pos("动机")
    cog_v = conscious_pos("y")
    flw_v = flow_pos("盈")
    id_v = identity_pos("1")
    wit_v = witness_pos("裁")
    txt_v = text_pos("5,2")
    bhv_v = behavior_pos("相遇")

    num = sm_v * ym_v * 5 + tn_v + ym_v + col_v + sp_v + mir_v + mus_v + cog_v + flw_v + id_v + wit_v + txt_v + bhv_v

  echo "num = ", num
  echo "mod ", CONTAINER_COUNT, " = ", num mod CONTAINER_COUNT

  # 创建社会示例
  echo ""
  echo "── 14层容器 ──"
  var idx: array[14, int]
  let space_names = ["东","南","西","北","天","地","头","胸","腰","脚","里","外","中","侧","手","正","背","肩"]
  let mir_dims = ["声","相","廓","尺","粗","含","色","高","宽","斜"]
  let mus_layers = ["音符","动机","乐章"]
  let cog_glyphs = ["x","h","j","f","s","y","g","n"]
  let flw_attrs = ["速","向","密","温","压","清","锐","曲","深","宽","盈","生"]
  let idt_glyphs = ["0","1"]
  let wit_glyphs = ["观","被","裁"]
  let txt_cells = ["0,0","1,0","2,0","3,0"]

  for j in 0..11:
    echo "  ", SM50[idx[0] mod SM50.len], YM66[idx[1] mod YM66.len], CLOCK60[idx[2] mod CLOCK60.len],
         " +", COLOR42[idx[3] mod COLOR42.len].sym,
         " @", space_names[idx[4] mod space_names.len],
         " ⟨", mir_dims[idx[5] mod mir_dims.len],
         mus_layers[idx[6] mod mus_layers.len],
         cog_glyphs[idx[7] mod cog_glyphs.len], "·",
         flw_attrs[idx[8] mod flw_attrs.len],
         idt_glyphs[idx[9] mod idt_glyphs.len],
         wit_glyphs[idx[10] mod wit_glyphs.len],
         txt_cells[idx[11] mod txt_cells.len], "⟩"
    for k in 0..13: idx[k] += 1
    if idx[0] >= 3: break
