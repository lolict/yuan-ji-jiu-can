## 满全法 OS · 统一社会模拟器
## ═══════════════════════════════════════════════════════════════
## 14层协同 · 容器有编码 · 积木选址 · 范畴辐射 · 社会演化
## 运行: nim c -r --gcc.exe:/usr/bin/cc --gcc.linkerexe:/usr/bin/cc sim/hunyuan_society.nim

import strutils, sets, tables, math

# ═══════════════════════════════════════════════════════════════
# 编码表 · 同 hunyuan_full.nim 保持一致
# ═══════════════════════════════════════════════════════════════

const
  SM50 = [
    "b","p","m","f","d","t","n","l","g","k","h","w","j","q","x","y",
    "z","c","s","r","ȥ","ç","ş","ŋ","v",
  ]
  YM66 = [
    "a","ȍ","ӗ","i","u","ü","ꞟ","ɘ","ɒ","౪","ɛ","ѧ","ɤ","ʉ","ꞛ",
    "ã","ẽ","ĩ","ʯ","ą","ę","į","๓","ⱄ","⊰","ʌ","ɔ","и","ǡ","ѽ","ÿ","ȳ",
  ]
  CLOCK60 = [
    "a","e","i","o","u","ü","A","E","I","O","U","Ü",
    "ā","ē","ī","ō","ū","ǖ","Ā","Ē","Ī","Ō","Ū","Ǖ",
    "á","é","í","ó","ú","ǘ","Á","É","Í","Ó","Ú","Ǘ",
    "ǎ","ě","ǐ","ǒ","ǔ","ǚ","Ǎ","Ě","Ǐ","Ǒ","Ǔ","Ǚ",
    "à","è","ì","ò","ù","ǜ","À","È","Ì","Ò","Ù","Ǜ",
  ]
  N_TOTAL_SYMBOLS* = 25+32+60+42+18+10+3+8+12+2+3+1+180+16
  N_CONTAINER* = 96

type
  ColorEntry = object
    sym, name: string
  SpaceEntry = object
    name: string
    ax, ay, az: int
  MirrorEntry = object
    dim, label: string
  MusicEntry = object
    layer, label: string
  ConsciousEntry = object
    flow, glyph, desc: string
  FlowEntry = object
    attr, glyph: string
  IdentityEntry = object
    anchor, glyph: string
  WitnessEntry = object
    role, glyph: string
  BehaviorEntry = object
    code, label: string

const
  COLOR42 = [
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
  SPACE18 = [
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
  MIRROR10 = [
    MirrorEntry(dim: "声", label: "声音蓝本"), MirrorEntry(dim: "相", label: "形象相位"),
    MirrorEntry(dim: "廓", label: "轮廓"), MirrorEntry(dim: "尺", label: "尺寸"),
    MirrorEntry(dim: "粗", label: "粗细"), MirrorEntry(dim: "含", label: "包含逻辑"),
    MirrorEntry(dim: "色", label: "色差"), MirrorEntry(dim: "高", label: "高低"),
    MirrorEntry(dim: "宽", label: "宽度"), MirrorEntry(dim: "斜", label: "斜度"),
  ]
  MUSIC3 = [
    MusicEntry(layer: "音符", label: "Atomic"), MusicEntry(layer: "动机", label: "Phrase"), MusicEntry(layer: "乐章", label: "Section"),
  ]
  CONSCIOUS8 = [
    ConsciousEntry(flow: "预加载流", glyph: "x", desc: "预先填装"), ConsciousEntry(flow: "范畴流", glyph: "h", desc: "归类同异"),
    ConsciousEntry(flow: "聚类流", glyph: "j", desc: "颗粒聚拢"), ConsciousEntry(flow: "分类流", glyph: "f", desc: "层级分判"),
    ConsciousEntry(flow: "属性流", glyph: "s", desc: "特征切面"), ConsciousEntry(flow: "解释流", glyph: "y", desc: "意义展开"),
    ConsciousEntry(flow: "关系流", glyph: "g", desc: "节点衔接"), ConsciousEntry(flow: "概念流", glyph: "n", desc: "名相浮现"),
  ]
  FLOW12 = [
    FlowEntry(attr: "速度", glyph: "速"), FlowEntry(attr: "向度", glyph: "向"), FlowEntry(attr: "密度", glyph: "密"),
    FlowEntry(attr: "温度", glyph: "温"), FlowEntry(attr: "压强", glyph: "压"), FlowEntry(attr: "清浊", glyph: "清"),
    FlowEntry(attr: "锐钝", glyph: "锐"), FlowEntry(attr: "曲直", glyph: "曲"), FlowEntry(attr: "深浅", glyph: "深"),
    FlowEntry(attr: "宽窄", glyph: "宽"), FlowEntry(attr: "盈虚", glyph: "盈"), FlowEntry(attr: "生灭", glyph: "生"),
  ]
  IDENTITY2 = [
    IdentityEntry(anchor: "楚恬", glyph: "0"), IdentityEntry(anchor: "我", glyph: "1"),
  ]
  WITNESS3 = [
    WitnessEntry(role: "观察者", glyph: "观"), WitnessEntry(role: "被观者", glyph: "被"), WitnessEntry(role: "裁定符", glyph: "裁"),
  ]
  BEHAVIOR16 = [
    BehaviorEntry(code: "出生", label: "出生"), BehaviorEntry(code: "成长", label: "成长"),
    BehaviorEntry(code: "相遇", label: "相遇"), BehaviorEntry(code: "冲突", label: "冲突"),
    BehaviorEntry(code: "和解", label: "和解"), BehaviorEntry(code: "创造", label: "创造"),
    BehaviorEntry(code: "破坏", label: "破坏"), BehaviorEntry(code: "学习", label: "学习"),
    BehaviorEntry(code: "遗忘", label: "遗忘"), BehaviorEntry(code: "恢复", label: "恢复"),
    BehaviorEntry(code: "爱", label: "爱"), BehaviorEntry(code: "死亡", label: "死亡"),
    BehaviorEntry(code: "重生", label: "重生"), BehaviorEntry(code: "静止", label: "静止"),
    BehaviorEntry(code: "流动", label: "流动"), BehaviorEntry(code: "循环", label: "循环"),
  ]

# ═══════════════════════════════════════════════════════════════
# 定位函数
# ═══════════════════════════════════════════════════════════════

proc sm_pos*(s: string): int =
  for i, v in SM50:
    if v == s: return i
  return 0
proc ym_pos*(y: string): int =
  for i, v in YM66:
    if v == y: return i
  return 0
proc tn_pos*(t: string): int =
  for i, v in CLOCK60:
    if v == t: return i
  return 0
proc color_pos*(s: string): int =
  for i, c in COLOR42:
    if c.sym == s: return i
  return -1
proc space_pos*(n: string): int =
  for i, s in SPACE18:
    if s.name == n: return i
  return 0
proc mirror_pos*(d: string): int =
  for i, m in MIRROR10:
    if m.dim == d: return i
  return 0
proc music_pos*(l: string): int =
  for i, m in MUSIC3:
    if m.layer == l: return i
  return 0
proc conscious_pos*(g: string): int =
  for i, c in CONSCIOUS8:
    if c.glyph == g: return i
  return 0
proc flow_pos*(g: string): int =
  for i, f in FLOW12:
    if f.glyph == g: return i
  return 0
proc identity_pos*(g: string): int =
  for i, id in IDENTITY2:
    if id.glyph == g: return i
  return 0
proc witness_pos*(g: string): int =
  for i, w in WITNESS3:
    if w.glyph == g: return i
  return 0
proc behavior_pos*(c: string): int =
  for i, b in BEHAVIOR16:
    if b.code == c: return i
  return 0
proc text_pos*(g: string): int =
  let parts = g.split(",")
  if parts.len == 2:
    return parseInt(parts[1]) * 18 + parseInt(parts[0])
  return 0

# ═══════════════════════════════════════════════════════════════
# 容器 · 社会实体
# ═══════════════════════════════════════════════════════════════

type
  Entity* = object
    id: int
    name: string          # 例: "baa+ạ@东⟨声⟩"
    sm, ym, tn, col, sp, mir, mus, cog, flw, idt, wit, txt, bhv: string
    coord: array[13, int] # 各层坐标
    energy: float         # 生命力
    links: seq[int]       # 关联容器的 id

# ═══════════════════════════════════════════════════════════════
# 社会
# ═══════════════════════════════════════════════════════════════

type
  Society* = object
    entities: seq[Entity]
    time: int             # tick
    events: seq[string]   # 日志

# ═══════════════════════════════════════════════════════════════
# 编码 → 容器名称
# ═══════════════════════════════════════════════════════════════

func encode_name(sm, ym, tn, col, sp: string): string =
  result = sm & ym & tn & " +" & col & " @" & sp

# ═══════════════════════════════════════════════════════════════
# 创建初始社会 · 用14层全编码填充
# ═══════════════════════════════════════════════════════════════

proc create_society*(n: int): Society =
  result = Society(entities: @[], time: 0, events: @[])
  var idx: array[14, int]
  let space_names = ["东","南","西","北","天","地","头","胸","腰","脚","里","外","中","侧","手","正","背","肩"]
  let mir_dims = ["声","相","廓","尺","粗","含","色","高","宽","斜"]
  let mus_layers = ["音符","动机","乐章"]
  let cog_glyphs = ["x","h","j","f","s","y","g","n"]
  let flw_attrs = ["速","向","密","温","压","清","锐","曲","深","宽","盈","生"]
  let idt_glyphs = ["0","1"]
  let wit_glyphs = ["观","被","裁"]
  let bhv_codes = ["出生","成长","相遇","冲突","和解","创造","破坏","学习","爱","死亡","重生","静止","流动","循环","遗忘","恢复"]

  for i in 0..<n:
    let sm_s = SM50[idx[0] mod SM50.len]
    let ym_s = YM66[idx[1] mod YM66.len]
    let tn_s = CLOCK60[idx[2] mod CLOCK60.len]
    let col_s = COLOR42[idx[3] mod COLOR42.len].sym
    let sp_s = space_names[idx[4] mod space_names.len]
    let mir_s = mir_dims[idx[5] mod mir_dims.len]
    let mus_s = mus_layers[idx[6] mod mus_layers.len]
    let cog_s = cog_glyphs[idx[7] mod cog_glyphs.len]
    let flw_s = flw_attrs[idx[8] mod flw_attrs.len]
    let idt_s = idt_glyphs[idx[9] mod idt_glyphs.len]
    let wit_s = wit_glyphs[idx[10] mod wit_glyphs.len]
    let txt_s = $(idx[11] mod 18) & "," & $(idx[11] div 18)
    let bhv_s = bhv_codes[idx[12] mod bhv_codes.len]

    var e = Entity(
      id: i,
      name: encode_name(sm_s, ym_s, tn_s, col_s, sp_s),
      sm: sm_s, ym: ym_s, tn: tn_s, col: col_s, sp: sp_s,
      mir: mir_s, mus: mus_s, cog: cog_s, flw: flw_s,
      idt: idt_s, wit: wit_s, txt: txt_s, bhv: bhv_s,
      energy: 100.0,
      links: @[],
    )
    e.coord = [
      sm_pos(sm_s), ym_pos(ym_s), tn_pos(tn_s), color_pos(col_s),
      space_pos(sp_s), mirror_pos(mir_s), music_pos(mus_s), conscious_pos(cog_s),
      flow_pos(flw_s), identity_pos(idt_s), witness_pos(wit_s),
      text_pos(txt_s), behavior_pos(bhv_s),
    ]

    result.entities.add(e)
    for j in 0..13: idx[j] += 1

  result.events.add("创世: " & $n & " 个容器从14层编码海洋中涌现")

# ═══════════════════════════════════════════════════════════════
# 关系判定 · 五种关联
# ═══════════════════════════════════════════════════════════════

const
  REL_FAMILY = 0   # 同家族 (a族/y族/o族...)
  REL_POLARITY = 1 # 同极性 (同列/同层)
  REL_COMPLEMENT = 2 # 互补 (东西/南北 相吸)
  REL_CROSS = 3    # 跨层 (声母 ↔ 颜色 共振)
  RESONANCE = 4    # 调谐 (相同数字根)

func rel_name(r: int): string =
  case r
  of REL_FAMILY: "同家族"
  of REL_POLARITY: "同极性"
  of REL_COMPLEMENT: "互补"
  of REL_CROSS: "跨层共振"
  of RESONANCE: "调谐"
  else: "未知"

# ═══════════════════════════════════════════════════════════════
# 关联判定
# ═══════════════════════════════════════════════════════════════

func family(s: string): char =
  if s.len > 0: return s[0]
  return '\x00'

func is_related(a, b: Entity): tuple[rel: int, weight: float] =
  # 同家族
  if family(a.sm) == family(b.sm) and a.id != b.id:
    return (REL_FAMILY, 30.0)
  # 同极性
  if a.coord[0] mod 5 == b.coord[0] mod 5 and a.id != b.id:
    return (REL_POLARITY, 20.0)
  # 互补 (东西/南北)
  if (a.sp == "东" and b.sp == "西") or (a.sp == "西" and b.sp == "东"):
    return (REL_COMPLEMENT, 25.0)
  if (a.sp == "南" and b.sp == "北") or (a.sp == "北" and b.sp == "南"):
    return (REL_COMPLEMENT, 25.0)
  # 跨层共振
  if a.sm == b.mir:
    return (REL_CROSS, 15.0)
  return (-1, 0.0)

# ═══════════════════════════════════════════════════════════════
# BFS 范畴辐射 · 从 seed 向外扩散
# ═══════════════════════════════════════════════════════════════

proc category_radiation*(s: Society, seed_idx: int, depth: int = 3): seq[tuple[id: int, tier: int, rel: string, w: float]] =
  var visited = initHashSet[int]()
  var queue: seq[tuple[id, tier: int]]
  queue.add((seed_idx, 0))
  visited.incl(seed_idx)

  while queue.len > 0:
    let (cur_id, tier) = queue.pop()
    if tier >= depth: continue
    let cur = s.entities[cur_id]

    for e in s.entities:
      if e.id in visited: continue
      let (rel, w) = is_related(cur, e)
      if rel >= 0:
        let decayed = w * pow(0.7, float(tier))
        result.add((e.id, tier + 1, rel_name(rel), decayed))
        visited.incl(e.id)
        queue.add((e.id, tier + 1))

# ═══════════════════════════════════════════════════════════════
# 积木选址 (动态查询)
# ═══════════════════════════════════════════════════════════════

proc fq_locate*(s: Society, sm, ym, tn, col, sp: string): int =
  let a = sm_pos(sm)
  let b = ym_pos(ym)
  let c = tn_pos(tn)
  let d = color_pos(col)
  let e = space_pos(sp)
  result = (a * b * 5 + c + b + d + e) mod s.entities.len

# ═══════════════════════════════════════════════════════════════
# 演化 tick
# ═══════════════════════════════════════════════════════════════

proc tick*(s: var Society) =
  s.time += 1

  # 能量衰减
  for e in s.entities.mitems:
    e.energy *= 0.99

  # 随机关联生成
  if s.entities.len >= 2:
    let a_idx = s.time mod s.entities.len
    let b_idx = (s.time * 7 + 3) mod s.entities.len
    if a_idx != b_idx:
      let (rel, w) = is_related(s.entities[a_idx], s.entities[b_idx])
      if rel >= 0:
        s.entities[a_idx].links.add(b_idx)
        s.entities[b_idx].links.add(a_idx)
        s.entities[a_idx].energy += w * 0.1
        s.entities[b_idx].energy += w * 0.1
        s.events.add("t" & $s.time & ": #" & $a_idx & " ↔ #" & $b_idx & " [" & rel_name(rel) & "]")

  # 高能量容器产生新连接
  for i in 0..<s.entities.len:
    if s.entities[i].energy > 100:
      let ei = s.entities[i]
      let target = fq_locate(s, ei.sm, ei.ym, ei.tn, ei.col, ei.sp)
      if target != i:
        s.entities[i].links.add(target)
        s.entities[i].energy -= 10

# ═══════════════════════════════════════════════════════════════
# 可视化
# ═══════════════════════════════════════════════════════════════

proc status*(s: Society): string =
  result = "═══ t" & $s.time & " · " & $s.entities.len & " 实体 ═══"
  for e in s.entities:
    result.add("\n  #" & $e.id & " " & $e.name & " @e=" & formatFloat(e.energy, precision=1) & " ↺" & $e.links.len)

func `$`(s: Society): string = s.status()


# ═══════════════════════════════════════════════════════════════
# 主程序 · 演示10个tick的社会演化
# ═══════════════════════════════════════════════════════════════

when is_main_module:
  echo "═══ 满全法 · 社会生命模拟 ═══"
  echo "编码体系: ", N_TOTAL_SYMBOLS, " 码元 · ", N_CONTAINER, " 容器位"
  echo ""

  var s = create_society(N_CONTAINER)

  echo "初始状态:"
  echo s
  echo ""

  echo "── t0 范畴辐射(从#0出发, 深度3) ──"
  let rad = category_radiation(s, 0, 3)
  var rad_group = initTable[string, seq[int]]()
  for r in rad:
    rad_group.mget_or_put(r.rel, @[]).add(r.id)
  for rel_name, ids in rad_group:
    echo "  ", rel_name, ": → ", ids[0..<min(ids.len, 5)].join(" → ")
  echo "  关联数: ", rad.len
  echo ""

  echo "── 演化 10 tick ──"
  for _ in 1..10:
    tick(s)

  echo s
  echo ""
  echo "── 社会事件 ──"
  for ev in s.events:
    echo "  ", ev
  echo ""

  # 积木选址演示
  let target = fq_locate(s, "g", "ӗ", "Í", "ụ", "胸")
  echo "── 14编码积木选址 ──"
  echo "  gӗÍ +ụ @胸 → #", target
  echo "  社会演化稳定 · 编码秩序涌现"
