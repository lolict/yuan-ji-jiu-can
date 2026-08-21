## 双键盘 · 双容器架构
##
## 设计:
##   标准容器 (满全法声母48 + 韵母66 + 时钟60)
##   扩展容器 (少数民族/方言 · 拼音化表示)
##
## 双键盘:
##   底部键盘 → 人类用 · 标准汉语拼音
##   头部键盘 → AI/机器用 · 扩展声母
##
## 两容器通过"关系边"协作 · 类似范畴辐射


# ═══════════════════════════════════════════════════════════════
# 标准容器 · 满全法编码
# ═══════════════════════════════════════════════════════════════

const
  FQ_SM48* = [
    "b","p","m","f","d","t","n","l",
    "g","k","h","w","j","q","x","y",
    "z","c","s","r","ȥ","ç","ş","ŋ",
  ]

  FQ_YM66* = [
    "a","ȍ","ӗ","i","u","ü",
    "ꞟ","ɘ","ɒ","౪","ɛ","ѧ",
    "ɤ","ʉ","ꞛ","ã","ẽ","ĩ",
    "ʯ","ą","ę","į","๓","ⱄ",
    "⊰","ʌ","ɔ","и","ǡ","ѽ",
    "ÿ","ȳ",
  ]

# ═══════════════════════════════════════════════════════════════
# 扩展容器 · 少数民族/方言 · 拼音化表示
# 不用少数民族符号 · 用汉语拼音化的双字母/三字母表示
# ═══════════════════════════════════════════════════════════════

type
  ExtInitial* = object    # 扩展声母
    id*: string           # 拼音化表示 (如 "kh" "gh" "mh")
    src*: string          # 来源 (如 "维吾尔" "藏语" "彝语")
    ipa*: string          # 国际音标 (如 "kʰ" "qʰ")
    desc*: string         # 描述

  ExtFinal* = object      # 扩展韵尾（粤语等）
    id*: string           # 拼音化表示 (如 "p" "t" "k" "m" "n" "ng")
    src*: string          # 来源 (如 "粤语" "客家" "闽南")
    kind*: string         # 类型 (入声/鼻韵尾)

# 扩展声母表 · 拼音化表示少数民族辅音
const EXT_INITIALS* = [
  # 注意: ng ȥ ç ş 已在声母表里(ŋ ȥ ç ş) · 不在扩展层
  # 扩展层只收声母表里没有的
  
  # 突厥语系（维吾尔/哈萨克）
  ExtInitial(id: "kh", src: "突厥", ipa: "kʰ/qʰ", desc: "送气清软腭/小舌"),
  ExtInitial(id: "gh", src: "突厥", ipa: "ʁ/ɣ", desc: "浊小舌擦音"),
  
  # 藏缅语系
  # ts 已合并 → 标准声母的 c (同音)
  ExtInitial(id: "dz", src: "藏彝", ipa: "dz", desc: "浊齿龈塞擦"),
  ExtInitial(id: "nr", src: "彝语", ipa: "ɳʈ/ɳɖ", desc: "卷舌鼻+塞"),
  
  # 苗瑶语系
  ExtInitial(id: "mh", src: "苗瑶", ipa: "m̥", desc: "清鼻音"),
  ExtInitial(id: "nh", src: "苗瑶", ipa: "n̥", desc: "清鼻音"),
  ExtInitial(id: "hm", src: "苗瑶", ipa: "hm", desc: "清鼻+双唇"),
  ExtInitial(id: "hn", src: "苗瑶", ipa: "hn", desc: "清鼻+齿龈"),
  
  # 壮侗语系
  ExtInitial(id: "qh", src: "壮侗", ipa: "qʰ", desc: "送气小舌"),
  
  # 齿间音（通用）
  ExtInitial(id: "th", src: "通用", ipa: "θ/ð", desc: "齿间音"),
  ExtInitial(id: "dh", src: "通用", ipa: "ð/θ", desc: "浊齿间"),
  
  # 粤语特殊声母
  # 注意: gw kw 就是 g+k+w · 不是独立声母 · 已删
]

# 扩展韵尾 · 粤语入声+鼻韵尾
const EXT_FINALS* = [
  ExtFinal(id: "p", src: "粤语", kind: "入声"),
  ExtFinal(id: "t", src: "粤语", kind: "入声"),
  ExtFinal(id: "k", src: "粤语", kind: "入声"),
  ExtFinal(id: "m", src: "粤语", kind: "鼻韵尾"),
  ExtFinal(id: "n", src: "粤语", kind: "鼻韵尾"),
  ExtFinal(id: "ng", src: "粤语", kind: "鼻韵尾"),
  # 客家/闽南
  ExtFinal(id: "b", src: "客家", kind: "入声"),
  ExtFinal(id: "g", src: "客家", kind: "入声"),
]

# ═══════════════════════════════════════════════════════════════
# 双键盘架构
# ═══════════════════════════════════════════════════════════════

type
  KeyboardSide* = enum
    kbBottom    # 底部 · 人类 · 标准拼音
    kbTop       # 头部 · AI · 扩展声母

  KeyPress* = object
    side*: KeyboardSide
    symbol*: string
    timestamp*: int

  DualKeyboard* = object
    bottom_keys*: seq[string]     # 人类键盘 · 标准声母+韵母
    top_keys*: seq[string]        # AI键盘 · 扩展声母
    history*: seq[KeyPress]       # 按键历史

proc new_dual_keyboard*(): DualKeyboard =
  result.bottom_keys = @FQ_SM48 & @FQ_YM66
  for ei in EXT_INITIALS:
    result.top_keys.add(ei.id)

proc press*(kb: var DualKeyboard, side: KeyboardSide, sym: string, ts: int) =
  kb.history.add(KeyPress(side: side, symbol: sym, timestamp: ts))

# ═══════════════════════════════════════════════════════════════
# 双容器协作 · 关系边
# ═══════════════════════════════════════════════════════════════

type
  ContainerKind* = enum
    ckStandard    # 标准容器
    ckExtended    # 扩展容器

  Relation* = object
    from_kind*: ContainerKind
    from_id*: string
    to_kind*: ContainerKind
    to_id*: string
    kind*: string        # 关系类型
    strength*: float

  DualSociety* = object
    standard*: seq[string]     # 标准容器ID列表
    extended*: seq[string]     # 扩展容器ID列表
    relations*: seq[Relation]  # 跨容器关系边

proc build_society*(): DualSociety =
  # 标准容器
  for s in FQ_SM48:
    result.standard.add("std:" & s)
  for y in FQ_YM66:
    result.standard.add("std:" & y)
  
  # 扩展容器
  for ei in EXT_INITIALS:
    result.extended.add("ext:" & ei.id)
  for ef in EXT_FINALS:
    result.extended.add("ext:" & ef.id)
  
  # 跨容器关系边 · 标准↔扩展
  # 如: 标准 "k" ↔ 扩展 "kh" (近似音)
  result.relations.add(Relation(
    from_kind: ckStandard, from_id: "k",
    to_kind: ckExtended, to_id: "kh",
    kind: "近似", strength: 0.8
  ))
  result.relations.add(Relation(
    from_kind: ckStandard, from_id: "g",
    to_kind: ckExtended, to_id: "gh",
    kind: "近似", strength: 0.7
  ))
  result.relations.add(Relation(
    from_kind: ckStandard, from_id: "n",
    to_kind: ckExtended, to_id: "nh",
    kind: "清化", strength: 0.6
  ))
  result.relations.add(Relation(
    from_kind: ckStandard, from_id: "m",
    to_kind: ckExtended, to_id: "mh",
    kind: "清化", strength: 0.6
  ))

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════

when is_main_module:
  echo "═══ 双键盘 · 双容器 ═══"
  echo ""
  
  let society = build_society()
  echo "标准容器: ", society.standard.len, " 个"
  echo "扩展容器: ", society.extended.len, " 个"
  echo "跨容器关系: ", society.relations.len, " 条"
  echo ""
  
  echo "── 扩展声母 (拼音化) ──"
  for ei in EXT_INITIALS:
    echo "  ", ei.id, "\t", ei.src, "\t", ei.ipa, "\t", ei.desc
  echo ""
  
  echo "── 扩展韵尾 (粤语等) ──"
  for ef in EXT_FINALS:
    echo "  ", ef.id, "\t", ef.src, "\t", ef.kind
  echo ""
  
  echo "── 跨容器关系 ──"
  for r in society.relations:
    echo "  ", r.from_id, " → ", r.to_id, " | ", r.kind, " | ", r.strength
  echo ""
  
  echo "── 双键盘 ──"
  var kb = new_dual_keyboard()
  echo "底部键盘(人类): ", kb.bottom_keys.len, " 键"
  echo "头部键盘(AI): ", kb.top_keys.len, " 键"
  echo ""
  echo "模拟对弈:"
  kb.press(kbBottom, "b", 0)
  echo "  人按: b"
  kb.press(kbTop, "kh", 1)
  echo "  AI按: kh"
  kb.press(kbBottom, "a", 2)
  echo "  人按: a"
  kb.press(kbTop, "p", 3)
  echo "  AI按: p (入声韵尾)"
