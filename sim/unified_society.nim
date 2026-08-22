## ═══════════════════════════════════════════════════════════════
## 七层统一社会模拟 · 满全法编码系统
## ═══════════════════════════════════════════════════════════════
## 三层基础:
##   声母层 FQ-SM48 (48进制) — 位权 · 定家族 · 定极性
##   韵母层 FQ-YM66 (66进制) — 本体 · 体验 · 容器
##   时钟层 Clock-60 (60进制) — 时序 · 演化 · 滴答驱动
##
## 四层扩展:
##   颜色层 Color-42 (42进制) — 情感 · 视觉 · 体验质感
##   空间层 Space-18 (18进制) — 方位 · 坐标 · 站位
##   镜像层 Mirror-10 (10进制) — 对偶 · 反射 · 对称
##   音乐层 Music-3 (3进制) — 层级 · 原子/短语/段落
##
## 统一选址公式:
##   址 = (声母位 × 韵母位 × 声调率 + 时钟滴答 + 韵母位 + 颜色位 + 空间位 + 镜像位 + 音乐层位) mod 容器数
##
## 核心创新: 七维驱动社会关系流动
## ═══════════════════════════════════════════════════════════════

import std/[sets, algorithm, strutils]

# ═══════════════════════════════════════════════════════════════
# 编码系统独立身份声明
# ═══════════════════════════════════════════════════════════════
const
  FQ_SM48_NAME* = "满全法声母-48"
  FQ_YM66_NAME* = "满全法韵母-66"
  CLOCK_60_NAME* = "声调韵母时钟-60"
  COLOR_42_NAME* = "满全法颜色-42"
  SPACE_18_NAME* = "满全法空间-18"
  MIRROR_10_NAME* = "满全法镜像-10"
  MUSIC_3_NAME*  = "满全法音乐-3"

  FQ_SM48_BASE* = 48
  FQ_YM66_BASE* = 66
  CLOCK_60_BASE* = 60
  COLOR_42_BASE* = 42
  SPACE_18_BASE* = 18
  MIRROR_10_BASE* = 10
  MUSIC_3_BASE* = 3
  TONE_RATE* = 5

# ═══════════════════════════════════════════════════════════════
# FQ-SM48 · 满全法声母48进制
# ═══════════════════════════════════════════════════════════════
const SHENGMU48 = [
  "b", "p", "m", "f",
  "d", "t", "n", "l",
  "g", "k", "h", "w",
  "j", "q", "x", "y",
  "z", "c", "s", "r",
  "ȥ", "ç", "ş", "ŋ",
]
const SHENGMU48_UPPER = [
  "B", "P", "M", "F",
  "D", "T", "N", "L",
  "G", "K", "H", "W",
  "J", "Q", "X", "Y",
  "Z", "C", "S", "R",
  "Ȥ", "Ç", "Ş", "Ŋ",
]

proc sm_pos(sm: string): int =
  for i, s in SHENGMU48:
    if s == sm: return i
  for i, s in SHENGMU48_UPPER:
    if s == sm: return i + 24
  return 0

# ═══════════════════════════════════════════════════════════════
# FQ-YM66 · 满全法韵母66进制
# ═══════════════════════════════════════════════════════════════
const SYMBOLS66 = [
  "a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U",
  "ü", "Ü", "ꞟ", "Ꞟ", "ɘ", "Ǝ", "ɒ", "Ɒ", "౪", "ȣ",
  "ɛ", "Ɛ", "ѧ", "Ѧ", "ɤ", "ɣ", "ʉ", "Ʉ", "ꞛ", "Ꞛ",
  "ã", "Ã", "ẽ", "Ẽ", "ĩ", "Ĩ", "ʯ", "ʮ", "ą", "Ą",
  "ę", "Ę", "į", "Į", "๓", "ɷ", "ⱄ", "Ⱋ", "⊰", "ɞ",
  "ʌ", "Ʌ", "ɔ", "Ɔ", "и", "И", "ǡ", "Ǡ", "ѽ", "Ѽ",
  "ѻ", "Ѻ", "ÿ", "Ÿ", "ȳ", "Ȳ",
]

proc ym_pos(ym: string): int =
  for i, s in SYMBOLS66:
    if s == ym: return i
  return 0

proc tn_pos(c: char): int =
  if c == '1': 1
  elif c == '2': 2
  elif c == '3': 3
  elif c == '4': 4
  else: 1

# ═══════════════════════════════════════════════════════════════
# Clock-60 · 声调韵母60进制
# ═══════════════════════════════════════════════════════════════
const CLOCK_60 = [
  "a", "e", "i", "o", "u", "ü",
  "A", "E", "I", "O", "U", "Ü",
  "ā", "ē", "ī", "ō", "ū", "ǖ",
  "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ",
  "á", "é", "í", "ó", "ú", "ǘ",
  "Á", "É", "Í", "Ó", "Ú", "Ǘ",
  "ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ",
  "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ",
  "à", "è", "ì", "ò", "ù", "ǜ",
  "À", "È", "Ì", "Ò", "Ù", "Ǜ",
]

proc clock_value(c: string): int =
  for i, s in CLOCK_60:
    if s == c: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# Color-42 · 满全法颜色42进制
# 7主色 × 6色阶 = 42
# ═══════════════════════════════════════════════════════════════
const COLORS42* = [
  # 红色系 (#0-#5)
  "ạ", "a̋", "å", "ã", "â", "ă",
  # 橙色系 (#6-#11)
  "ẹ", "e̋", "e̊", "ẽ", "ê", "ĕ",
  # 黄色系 (#12-#17)
  "ị", "i̋", "i̊", "ĩ", "î", "ĭ",
  # 绿色系 (#18-#23)
  "ọ", "ő", "o̊", "õ", "ô", "ŏ",
  # 青色系 (#24-#29)
  "ụ", "ű", "ů", "ũ", "û", "ŭ",
  # 蓝色系 (#30-#35)
  "ȃ", "ȇ", "ȋ", "ȏ", "ȗ", "ȁ",
  # 紫色系 (#36-#41)
  "ȅ", "ȉ", "ȍ", "ȕ", "ä", "ö",
]

proc color_pos(c: string): int =
  for i, s in COLORS42:
    if s == c: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# Space-18 · 满全法空间18进制
# 6方位 + 4斜角 + 8天地角 = 18站位
# ═══════════════════════════════════════════════════════════════
const SPACE18* = [
  "E", "W", "S", "N", "H", "D",       # 6方位
  "ES", "WS", "EN", "WN",              # 4斜角
  "HN", "DN", "HS", "DS",              # 4天地角
  "HE", "DE", "HW", "DW",              # 4天地侧角
]

proc space_pos(s: string): int =
  for i, t in SPACE18:
    if t == s: return i
  return 0

# ═══════════════════════════════════════════════════════════════
# Mirror-10 · 满全法镜像10进制
# 10符号 · 对偶反射
# ═══════════════════════════════════════════════════════════════
const MIRROR10* = [
  "a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ", "i", "I", "u", "U",
]

proc mirror_pos(m: string): int =
  for i, s in MIRROR10:
    if s == m: return i
  return 0

proc mirror_reflect*(idx: int): int =
  ## 镜像反射: 位置 i ↔ 位置 (9-i)
  result = MIRROR10_BASE - 1 - idx

# ═══════════════════════════════════════════════════════════════
# Music-3 · 满全法音乐3进制
# 原子(0) → 短语(1) → 段落(2)
# ═══════════════════════════════════════════════════════════════
const MUSIC3* = ["原子", "短语", "段落"]

proc music_pos(m: string): int =
  case m
  of "原子": 0
  of "短语": 1
  of "段落": 2
  else: 0

# ═══════════════════════════════════════════════════════════════
# 类型定义
# ═══════════════════════════════════════════════════════════════
type
  ClockCoord* = object
    miao*: string
    fen*: string
    shi*: string

  FQCode* = object
    family*: char
    polarity*: char
    shengmu*: string
    yunmu*: string
    tone*: char

  Container* = object
    id*: int
    tier*: int
    wall*: array[2, char]
    polarity*: char
    code*: FQCode
    clock*: ClockCoord
    color*: string      # 颜色符号
    space*: string      # 空间站位
    mirror*: string     # 镜像符号
    music*: string      # 音乐层级
    descriptor*: string

  RelationKind* = enum
    relSameFamily       # 同一家族(声母同源)
    relSamePolarity     # 同极性(阴阳)
    relToneHarmony      # 声调和谐
    relClockSync        # 时钟同步(同时刻)
    relColorHarmony     # 颜色和谐(同色系)
    relSpaceAdjacent    # 空间相邻
    relMirrorPair       # 镜像对偶
    relMusicLayer       # 音乐同层
    relComplement       # 互补
    relCrossTier        # 跨层

  Relation* = object
    src*, dst*: int
    kind*: RelationKind
    strength*: float

  UnifiedState* = object
    tick*: int             # 全局时钟滴答
    phase*: string         # 当前时态(秒/分/时)

  Society* = object
    containers*: seq[Container]
    state*: UnifiedState

# ═══════════════════════════════════════════════════════════════
# 时钟操作
# ═══════════════════════════════════════════════════════════════
proc clock_ticks*(c: ClockCoord): int =
  result = clock_value(c.shi) * 3600 + clock_value(c.fen) * 60 + clock_value(c.miao)

proc clock_from_ticks*(ticks: int): ClockCoord =
  let miao = ticks mod 60
  let fen = (ticks div 60) mod 60
  let shi = (ticks div 3600) mod 24
  result.miao = CLOCK_60[miao]
  result.fen = CLOCK_60[fen]
  result.shi = CLOCK_60[shi]

proc clock_tick*(c: ClockCoord): ClockCoord =
  let total = clock_ticks(c) + 1
  result = clock_from_ticks(total)

proc `$`*(c: ClockCoord): string =
  result = c.shi & ":" & c.fen & ":" & c.miao

# ═══════════════════════════════════════════════════════════════
# 积木选址公式 (七层融合)
#
# 址 = (声母位 × 韵母位 × 声调率 + 时钟滴答 + 韵母位 + 颜色位 + 空间位 + 镜像位 + 音乐层位) mod 容器数
#
# 关键: 七维驱动让同一编码在不同条件下找到不同容器
# ═══════════════════════════════════════════════════════════════
proc block_addressing*(sm: string, ym: string, tone: char, color: string, space: string, mirror: string, music: string, society: Society): int =
  if society.containers.len == 0: return -1
  let sp = sm_pos(sm)
  let yp = ym_pos(ym)
  let cp = color_pos(color)
  let stp = space_pos(space)
  let mp = mirror_pos(mirror)
  let mup = music_pos(music)
  let clock_t = society.state.tick
  result = (sp * yp * TONE_RATE + clock_t + yp + cp + stp + mp + mup) mod society.containers.len

# ═══════════════════════════════════════════════════════════════
# 范畴辐射 (跨层 · 含新维度)
# ═══════════════════════════════════════════════════════════════
proc compute_radiation*(src: Container, dst: Container): float =
  var strength = 0.0
  if src.code.family == dst.code.family: strength += 0.3
  if src.polarity == dst.polarity: strength += 0.25
  else: strength += 0.15
  if src.code.tone == dst.code.tone: strength += 0.15
  let t1 = src.code.tone
  let t2 = dst.code.tone
  if (t1 == '1' and t2 == '4') or (t1 == '4' and t2 == '1') or
     (t1 == '2' and t2 == '3') or (t1 == '3' and t2 == '2'):
    strength += 0.05
  # 时钟同步加成
  if src.clock.miao == dst.clock.miao: strength += 0.1
  if src.clock.fen == dst.clock.fen: strength += 0.05
  # 颜色和谐 (同色系: 同除以6的商)
  let src_color_group = color_pos(src.color) div 6
  let dst_color_group = color_pos(dst.color) div 6
  if src_color_group == dst_color_group: strength += 0.08
  # 空间相邻 (位置差1以内)
  let src_sp = space_pos(src.space)
  let dst_sp = space_pos(dst.space)
  if abs(src_sp - dst_sp) <= 2: strength += 0.05
  # 镜像对偶
  let src_mi = mirror_pos(src.mirror)
  let dst_mi = mirror_pos(dst.mirror)
  if mirror_reflect(src_mi) == dst_mi: strength += 0.1
  # 音乐同层
  if src.music == dst.music: strength += 0.07
  # 层级衰减
  let td = abs(src.tier - dst.tier)
  if td == 1: strength *= 0.7
  elif td == 2: strength *= 0.4
  elif td >= 3: strength *= 0.2
  result = min(strength, 1.0)

proc category_radiation*(src_id: int, society: Society, max_depth: int): seq[Relation] =
  var visited: HashSet[int]
  var queue: seq[(int, int, float)] = @[(src_id, 0, 1.0)]
  visited.incl(src_id)
  while queue.len > 0:
    let (cur_id, dist, strength) = queue[0]
    queue.delete(0)
    if dist >= max_depth or strength < 0.1: continue
    for i in 0 ..< society.containers.len:
      if i == cur_id or i in visited: continue
      let s = compute_radiation(society.containers[cur_id], society.containers[i])
      if s < 0.05: continue
      let transmitted = strength * s
      var rkind = relSameFamily
      let c1 = society.containers[cur_id].code
      let c2 = society.containers[i].code
      # 判定关系类型 (按优先级)
      let src_mi = mirror_pos(society.containers[cur_id].mirror)
      let dst_mi = mirror_pos(society.containers[i].mirror)
      if mirror_reflect(src_mi) == dst_mi:
        rkind = relMirrorPair
      elif society.containers[cur_id].music == society.containers[i].music:
        rkind = relMusicLayer
      elif c1.family != c2.family and c1.polarity != c2.polarity:
        rkind = relComplement
      elif abs(society.containers[cur_id].tier - society.containers[i].tier) >= 2:
        rkind = relCrossTier
      elif c1.tone == c2.tone:
        rkind = relToneHarmony
      result.add(Relation(src: cur_id, dst: i, kind: rkind, strength: transmitted))
      if i notin visited:
        visited.incl(i)
        queue.add((i, dist + 1, transmitted))

# ═══════════════════════════════════════════════════════════════
# 创建社会 · 七层预装 · 可配置容器数
# ═══════════════════════════════════════════════════════════════
proc create_society*(num_containers: int = 15): Society =
  let sms = ["b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "w",
             "j", "q", "x", "y", "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ"]
  let yms = ["a", "ȍ", "ӗ", "i", "u", "ü", "ꞟ", "ɘ", "ɒ", "౪",
             "ɛ", "ѧ", "ɤ", "ʉ", "ꞛ", "ã", "ẽ", "ĩ", "ʯ", "ą"]
  let families = ['C', 'B', 'P']
  let polarities = ['1', '0']
  let tones: array[4, char] = ['1', '2', '3', '4']
  let clock_starts = ["a", "e", "i", "o", "u", "ü", "ā", "ē"]
  var sm_i = 0
  var ym_i = 0
  var tn_i = 0
  var fam_i = 0
  var pl_i = 0
  var cl_i = 0
  var co_i = 0
  var sp_i = 0
  var mi_i = 0
  var mu_i = 0
  let tiers = max(1, num_containers div 5)
  for i in 0..<num_containers:
    let tier = (i mod tiers) + 1
    let code = FQCode(
      family: families[fam_i mod 3],
      polarity: polarities[pl_i mod 2],
      shengmu: sms[sm_i mod sms.len],
      yunmu: yms[ym_i mod yms.len],
      tone: tones[tn_i mod 4],
    )
    let cc = ClockCoord(
      miao: clock_starts[cl_i mod clock_starts.len],
      fen: clock_starts[(cl_i + 1) mod clock_starts.len],
      shi: clock_starts[(cl_i + 2) mod clock_starts.len],
    )
    var wall_char = "knd"[(tier-1) mod 3]
    var desc = ""
    desc.add(code.family)
    desc.add(code.polarity)
    desc.add(code.shengmu)
    desc.add(code.yunmu)
    desc.add(code.tone)
    result.containers.add(Container(
      id: result.containers.len,
      tier: tier,
      wall: [wall_char, wall_char],
      polarity: code.polarity,
      code: code,
      clock: cc,
      color: COLORS42[co_i mod COLORS42.len],
      space: SPACE18[sp_i mod SPACE18.len],
      mirror: MIRROR10[mi_i mod MIRROR10.len],
      music: MUSIC3[mu_i mod MUSIC3.len],
      descriptor: desc,
    ))
    sm_i += 1
    ym_i += 1
    tn_i += 1
    fam_i += 1
    pl_i += 1
    cl_i += 1
    co_i += 1
    sp_i += 1
    mi_i += 1
    mu_i += 1
  result.state.tick = 0
  result.state.phase = "秒"

# ═══════════════════════════════════════════════════════════════
# 时钟推进 · 驱动社会演化
# ═══════════════════════════════════════════════════════════════
proc advance_clock*(society: var Society, steps: int) =
  society.state.tick += steps
  let phase_val = society.state.tick mod 3600
  if phase_val < 60: society.state.phase = "秒"
  elif phase_val < 3600: society.state.phase = "分"
  else: society.state.phase = "时"

# ═══════════════════════════════════════════════════════════════
# 输入积木 (在指定时刻 · 七维)
# ═══════════════════════════════════════════════════════════════
proc input_society*(society: Society, sm: string, ym: string, tone: char, color: string, space: string, mirror: string, music: string): string =
  let target = block_addressing(sm, ym, tone, color, space, mirror, music, society)
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add(" 七层统一 · " & FQ_SM48_NAME & " + " & FQ_YM66_NAME & " + " & CLOCK_60_NAME)
  lines.add("           + " & COLOR_42_NAME & " + " & SPACE_18_NAME & " + " & MIRROR_10_NAME & " + " & MUSIC_3_NAME)
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add("输入积木: 声母='" & sm & "' 韵母='" & ym & "' 声调='" & $tone & "'")
  lines.add("         颜色='" & color & "' 空间='" & space & "' 镜像='" & mirror & "' 音乐='" & music & "'")
  lines.add("时刻: 滴答=" & $society.state.tick & " · 相位=" & society.state.phase)
  let sp = sm_pos(sm)
  let yp = ym_pos(ym)
  let cp = color_pos(color)
  let stp = space_pos(space)
  let mp = mirror_pos(mirror)
  let mup = music_pos(music)
  lines.add("编码位置: 声母位=" & $sp & " · 韵母位=" & $yp & " · 颜色位=" & $cp & " · 空间位=" & $stp & " · 镜像位=" & $mp & " · 音乐位=" & $mup)
  lines.add("选址公式: (" & $sp & "×" & $yp & "×" & $TONE_RATE & " + " & $society.state.tick & " + " & $yp & " + " & $cp & " + " & $stp & " + " & $mp & " + " & $mup & ") mod " & $society.containers.len)
  lines.add("───────────────────────────────────────────────────────────")
  if target < 0:
    lines.add("✗ 未找到 · 此组合未预装")
  else:
    let c = society.containers[target]
    let rays = category_radiation(target, society, 3)
    lines.add("✓ 选址成功 → #" & $target & " [L" & $c.tier & "] " & c.descriptor)
    lines.add("  极性: " & (if c.polarity == '1': "阳(我)" else: "阴(刘楚恬)"))
    lines.add("  墙标: " & $c.wall[0] & " | " & $c.wall[1])
    lines.add("  时钟: " & $c.clock)
    lines.add("  颜色: " & c.color & " | 空间: " & c.space & " | 镜像: " & c.mirror & " | 音乐: " & c.music)
    lines.add("───────────────────────────────────────────────────────────")
    lines.add("辐射范围: " & $rays.len & " 个关联")
    var sorted = rays
    proc cmp_radiation(x, y: Relation): int = cmp(y.strength, x.strength)
    sorted.sort(cmp_radiation)
    for ray in sorted:
      let d = society.containers[ray.dst]
      var kind_str = ""
      case ray.kind
      of relSameFamily: kind_str = "同家族"
      of relSamePolarity: kind_str = "同极性"
      of relToneHarmony: kind_str = "调谐"
      of relClockSync: kind_str = "时钟同步"
      of relColorHarmony: kind_str = "同色系"
      of relSpaceAdjacent: kind_str = "空间相邻"
      of relMirrorPair: kind_str = "镜像对偶"
      of relMusicLayer: kind_str = "音乐同层"
      of relComplement: kind_str = "互补"
      of relCrossTier: kind_str = "跨层"
      lines.add("  → #" & $ray.dst & " [L" & $d.tier & "] " & $d.descriptor & " | " & kind_str & " | " & $ray.strength)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
var society = create_society()
echo "═══ 七层统一 · 满全法编码系统 ═══"
echo "系统身份: FQ-SM48(48) · FQ-YM66(66) · Clock-60(60)"
echo "         Color-42(42) · Space-18(18) · Mirror-10(10) · Music-3(3)"
echo "容器数: " & $society.containers.len
echo "初始滴答: 0"
echo ""
echo "── 预装容器 (七层编码) ──"
for c in society.containers:
  echo "  #" & $c.id & " [L" & $c.tier & "] " & c.descriptor &
    " 墙:" & $c.wall[0] & " 时钟:" & $c.clock &
    " 色:" & c.color & " 空:" & c.space & " 镜:" & c.mirror & " 乐:" & c.music

echo ""
echo "═══ 时刻 0 · 积木选址 ═══"
for i in [0, 5, 10]:
  if i < society.containers.len:
    let c = society.containers[i].code
    echo ""
    echo society.input_society(c.shengmu, c.yunmu, c.tone, society.containers[i].color, society.containers[i].space, society.containers[i].mirror, society.containers[i].music)

echo ""
echo "═══ 推进时钟 · 滴答+30 · 再次选址 ═══"
advance_clock(society, 30)
echo "当前滴答: " & $society.state.tick & " · 相位: " & society.state.phase
echo ""
for i in [0, 5, 10]:
  if i < society.containers.len:
    let c = society.containers[i].code
    echo ""
    echo society.input_society(c.shengmu, c.yunmu, c.tone, society.containers[i].color, society.containers[i].space, society.containers[i].mirror, society.containers[i].music)

echo ""
echo "═══ 推进时钟 · 滴答+3600(进位到小时) · 再次选址 ═══"
advance_clock(society, 3600)
echo "当前滴答: " & $society.state.tick & " · 相位: " & society.state.phase
echo ""
for i in [0, 5, 10]:
  if i < society.containers.len:
    let c = society.containers[i].code
    echo ""
    echo society.input_society(c.shengmu, c.yunmu, c.tone, society.containers[i].color, society.containers[i].space, society.containers[i].mirror, society.containers[i].music)

echo ""
echo "═══ 七层选址差异验证 ═══"
echo "同一编码(b+a+1)在不同条件下的选址:"
advance_clock(society, 0)
var addr0 = block_addressing("b", "a", '1', "ạ", "E", "a", "原子", society)
echo "  时刻0:    ", addr0, " → 容器(色:ạ 空:E 镜:a 乐:原子)"
advance_clock(society, 100)
var addr100 = block_addressing("b", "a", '1', "ẹ", "W", "A", "短语", society)
echo "  时刻100:  ", addr100, " → 容器(色:ẹ 空:W 镜:A 乐:短语)"
advance_clock(society, 200)
var addr200 = block_addressing("b", "a", '1', "ị", "S", "ȍ", "段落", society)
echo "  时刻200:  ", addr200, " → 容器(色:ị 空:S 镜:ȍ 乐:段落)"
advance_clock(society, 300)
var addr300 = block_addressing("b", "a", '1', "ọ", "N", "Ȍ", "原子", society)
echo "  时刻300:  ", addr300, " → 容器(色:ọ 空:N 镜:Ȍ 乐:原子)"

# ═══════════════════════════════════════════════════════════════
# 分布均匀性测试 · 多容器数 + 非15倍数时钟偏移
# ═══════════════════════════════════════════════════════════════
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  分布均匀性测试 · 多容器规模"
echo "═══════════════════════════════════════════════════════════════"

proc distribution_test*(num_containers: int, num_samples: int) =
  var soc = create_society(num_containers)
  var counts = newSeq[int](num_containers)
  let sms = ["b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "w"]
  let yms = ["a", "ȍ", "ӗ", "i", "u", "ü", "ꞟ", "ɘ"]
  let colors = ["ạ", "ẹ", "ị", "ọ", "ụ", "ȃ", "ȅ"]
  let spaces = ["E", "W", "S", "N", "H", "D"]
  let mirrors = ["a", "A", "ȍ", "Ȍ", "ӗ", "Ӗ"]
  let musics = ["原子", "短语", "段落"]
  for i in 0..<num_samples:
    let sm = sms[i mod sms.len]
    let ym = yms[(i * 3) mod yms.len]
    let tn = char(ord('1') + (i mod 4))
    let co = colors[(i * 2) mod colors.len]
    let sp = spaces[(i * 3) mod spaces.len]
    let mi = mirrors[(i * 5) mod mirrors.len]
    let mu = musics[i mod musics.len]
    # 非15倍数时钟偏移: 使用质数偏移 7, 13, 17, 23
    let offsets = [7, 13, 17, 23]
    soc.state.tick = offsets[i mod offsets.len] + (i div 4) * 11
    let addr_val = block_addressing(sm, ym, tn, co, sp, mi, mu, soc)
    if addr_val >= 0 and addr_val < num_containers:
      counts[addr_val] += 1
  let expected = num_samples / num_containers
  var min_hit = num_samples + 1
  var max_hit = 0
  for c in counts:
    if c < min_hit: min_hit = c
    if c > max_hit: max_hit = c
  let variance = float(max_hit - min_hit) / expected * 100.0
  echo "  容器数=" & $num_containers & " · 采样=" & $num_samples &
    " · 期望=" & $expected & " · 最小=" & $min_hit & " · 最大=" & $max_hit &
    " · 偏差=" & $variance & "%"

distribution_test(15, 300)
distribution_test(17, 340)
distribution_test(23, 460)
distribution_test(31, 620)
distribution_test(37, 740)
distribution_test(53, 1060)

echo ""
echo "═══ 非15倍数时钟偏移选址测试 ═══"
var soc23 = create_society(23)
echo "容器数=23 (质数 · 非15倍数):"
let test_offsets = [1, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
for off in test_offsets:
  soc23.state.tick = off
  let addr_val = block_addressing("b", "a", '1', "ạ", "E", "a", "原子", soc23)
  echo "  tick=" & $off & " → 容器#" & $addr_val

echo ""
echo "═══ 大规模容器测试 (101容器) ═══"
var soc101 = create_society(101)
echo "容器数=101 (质数):"
soc101.state.tick = 0
var hits101 = newSeq[int](101)
for t in 0..<1010:
  soc101.state.tick = t * 7 + 13
  let addr_val = block_addressing("b", "a", '1', "ạ", "E", "a", "原子", soc101)
  if addr_val >= 0 and addr_val < 101:
    hits101[addr_val] += 1
var empty101 = 0
for h in hits101:
  if h == 0: empty101 += 1
echo "  1010次采样 · 未命中容器数=" & $empty101 & " · 命中率=" & $(100.0 - float(empty101) / 101.0 * 100.0) & "%"
