## ═══════════════════════════════════════════════════════════════
## 积木选址 · 范畴辐射 — 满全法编码社会模拟
## 编码系统命名: 满全法声母-48 (FQ-SM48) + 满全法韵母-66 (FQ-YM66)
## 来源: tools/kungguan-container/shengmu48.nim · yunmu66.nim
## 两个系统相互独立 · 名称永久 · 符号变更不影响身份
## ═══════════════════════════════════════════════════════════════

import std/[sets, algorithm, strutils]

const
  TONE_RATE = 5

  # 编码系统独立身份声明
  FQ_SM48_NAME* = "满全法声母-48"   # 48进制声母编码系统 (shengmu48.nim)
  FQ_YM66_NAME* = "满全法韵母-66"   # 66进制韵母编码系统 (yunmu66.nim)
  FQ_SM48_BASE* = 48                # 声母进制基数
  FQ_YM66_BASE* = 66                # 韵母进制基数

type
  FQCode* = object
    family*: char
    polarity*: char
    shengmu*: string    # 声母（码元）
    yunmu*: string      # 韵母（码元）
    tone*: char         # 声调

  Container* = object
    id*: int
    tier*: int
    wall*: array[2, char]
    polarity*: char
    code*: FQCode
    descriptor*: string

  RelationKind* = enum
    relSameFamily
    relSamePolarity
    relComplement
    relCrossTier
    relToneHarmony

  Relation* = object
    src*, dst*: int
    kind*: RelationKind
    strength*: float

  Society* = object
    containers*: seq[Container]

# ═══════════════════════════════════════════════════════════════
# FQ-SM48 · 满全法声母48进制 · 码元即位置
# 源文件: tools/kungguan-container/shengmu48.nim · 不翻译不映射
# 扩展: ȥ ç ş ŋ (浊平舌/清擦/卷舌/舌根鼻)
# 48 = 24小写 + 24大写
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

# 符号在48表中的位置 · 码元即进制 · 符号自己携带位置

proc sm_pos(sm: string): int =
  for i, s in SHENGMU48:
    if s == sm: return i
  for i, s in SHENGMU48_UPPER:
    if s == sm: return i + 24
  return 0

# ═══════════════════════════════════════════════════════════════
# FQ-YM66 · 满全法韵母66进制 · 符号即位置 · 符号自己携带值
# 源文件: tools/kungguan-container/yunmu66.nim · SYMBOLS66 数组
# 每符号唯一码元 · 不翻译
# 66 = 33韵母 × 2(大小写)
# ═══════════════════════════════════════════════════════════════

const SYMBOLS66 = [
  "a", "A",
  "ȍ", "Ȍ",
  "ӗ", "Ӗ",
  "i", "I",
  "u", "U",
  "ü", "Ü",
  "ꞟ", "Ꞟ",
  "ɘ", "Ǝ",
  "ɒ", "Ɒ",
  "౪", "ȣ",
  "ɛ", "Ɛ",
  "ѧ", "Ѧ",
  "ɤ", "ɣ",
  "ʉ", "Ʉ",
  "ꞛ", "Ꞛ",
  "ã", "Ã",
  "ẽ", "Ẽ",
  "ĩ", "Ĩ",
  "ʯ", "ʮ",
  "ą", "Ą",
  "ę", "Ę",
  "į", "Į",
  "๓", "ɷ",
  "ⱄ", "Ⱋ",
  "⊰", "ɞ",
  "ʌ", "Ʌ",
  "ɔ", "Ɔ",
  "и", "И",
  "ǡ", "Ǡ",
  "ѽ", "Ѽ",
  "ѻ", "Ѻ",
  "ÿ", "Ÿ",
  "ȳ", "Ȳ",
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
# 积木选址公式
# 码元的位置就是进制 · 直接用满全法表中自带的位置参与运算
# 址 = (声母位 × 韵母位 × 声调率 + 声调位 + 韵母位) mod 总数
# ═══════════════════════════════════════════════════════════════

proc block_addressing*(sm: string, ym: string, tone: char, containers: seq[Container]): int =
  if containers.len == 0: return -1
  let sp = sm_pos(sm)
  let yp = ym_pos(ym)
  let tp = tn_pos(tone)
  let slot = (sp * yp * TONE_RATE + tp + yp) mod containers.len
  for attempt in 0 ..< containers.len:
    let idx = (slot + attempt) mod containers.len
    let pre = containers[idx].code
    if pre.shengmu == sm and pre.yunmu == ym and pre.tone == tone:
      return idx
  return -1

# ═══════════════════════════════════════════════════════════════
# 范畴辐射
# ═══════════════════════════════════════════════════════════════

proc compute_radiation*(src: Container, dst: Container): float =
  var strength = 0.0
  if src.code.family == dst.code.family: strength += 0.4
  if src.polarity == dst.polarity: strength += 0.3
  else: strength += 0.2
  if src.code.tone == dst.code.tone: strength += 0.2
  let t1 = src.code.tone
  let t2 = dst.code.tone
  if (t1 == '1' and t2 == '4') or (t1 == '4' and t2 == '1') or
     (t1 == '2' and t2 == '3') or (t1 == '3' and t2 == '2'):
    strength += 0.1
  let td = abs(src.tier - dst.tier)
  if td == 1: strength *= 0.7
  elif td == 2: strength *= 0.4
  elif td >= 3: strength *= 0.2
  result = min(strength, 1.0)

proc category_radiation*(src_id: int, containers: seq[Container], max_depth: int): seq[Relation] =
  var visited: HashSet[int]
  var queue: seq[(int, int, float)] = @[(src_id, 0, 1.0)]
  visited.incl(src_id)
  while queue.len > 0:
    let (cur_id, dist, strength) = queue[0]
    queue.delete(0)
    if dist >= max_depth or strength < 0.1: continue
    for i in 0 ..< containers.len:
      if i == cur_id or i in visited: continue
      let s = compute_radiation(containers[cur_id], containers[i])
      if s < 0.05: continue
      let transmitted = strength * s
      var rkind = relSameFamily
      let c1 = containers[cur_id].code
      let c2 = containers[i].code
      if c1.family != c2.family and c1.polarity != c2.polarity:
        rkind = relComplement
      elif abs(containers[cur_id].tier - containers[i].tier) >= 2:
        rkind = relCrossTier
      elif c1.tone == c2.tone:
        rkind = relToneHarmony
      result.add(Relation(src: cur_id, dst: i, kind: rkind, strength: transmitted))
      if i notin visited:
        visited.incl(i)
        queue.add((i, dist + 1, transmitted))

# ═══════════════════════════════════════════════════════════════
# 创建社会 · 预装容器 · 直接用满全法编码表中的码元
# ═══════════════════════════════════════════════════════════════

proc create_society*(): Society =
  # 直接用声母48表中的码元 · 不筛选不翻译
  let sms = ["b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "w",
             "j", "q", "x", "y", "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ"]
  # 直接用韵母66表中的码元 · 不选不翻译
  let yms = ["a", "ȍ", "ӗ", "i", "u", "ü", "ꞟ", "ɘ", "ɒ", "౪",
             "ɛ", "ѧ", "ɤ", "ʉ", "ꞛ", "ã", "ẽ", "ĩ", "ʯ", "ą"]
  let families = ['C', 'B', 'P']
  let polarities = ['1', '0']
  let tones: array[4, char] = ['1', '2', '3', '4']
  var sm_i = 0
  var ym_i = 0
  var tn_i = 0
  var fam_i = 0
  var pl_i = 0
  for tier in 1..3:
    for sub in 0..4:
      let code = FQCode(
        family: families[fam_i mod 3],
        polarity: polarities[pl_i mod 2],
        shengmu: sms[sm_i mod sms.len],
        yunmu: yms[ym_i mod yms.len],
        tone: tones[tn_i mod 4],
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
        descriptor: desc,
      ))
      sm_i += 1
      ym_i += 1
      tn_i += 1
      fam_i += 1
      pl_i += 1

proc rel_name*(k: RelationKind): string =
  case k
  of relSameFamily: "同家族"
  of relSamePolarity: "同极性"
  of relComplement: "互补"
  of relCrossTier: "跨层"
  of relToneHarmony: "调谐"

# ═══════════════════════════════════════════════════════════════
# 输入积木
# ═══════════════════════════════════════════════════════════════

proc input_society*(society: Society, sm: string, ym: string, tone: char): string =
  let target = block_addressing(sm, ym, tone, society.containers)
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add(" 积木选址 · " & FQ_SM48_NAME & " + " & FQ_YM66_NAME)
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add("输入积木: 声母='" & sm & "' 韵母='" & ym & "' 声调='" & $tone & "'")
  let sp = sm_pos(sm)
  let yp = ym_pos(ym)
  let tp = tn_pos(tone)
  lines.add("编码位置(来自满全法表): 声母位=" & $sp & " · 韵母位=" & $yp & " · 声调位=" & $tp)
  lines.add("选址公式: (" & $sp & "×" & $yp & "×5 + " & $tp & " + " & $yp & ") mod " & $society.containers.len)
  lines.add("───────────────────────────────────────────────────────────")
  if target < 0:
    lines.add("✗ 未找到 · 此组合未预装")
  else:
    let c = society.containers[target]
    let rays = category_radiation(target, society.containers, 3)
    lines.add("✓ 选址成功 → #" & $target & " [L" & $c.tier & "] " & c.descriptor)
    lines.add("  极性: " & (if c.polarity == '1': "阳(我)" else: "阴(刘楚恬)"))
    lines.add("  墙标: " & $c.wall[0] & " | " & $c.wall[1])
    lines.add("───────────────────────────────────────────────────────────")
    lines.add("辐射范围: " & $rays.len & " 个关联")
    var sorted = rays
    proc cmp_radiation(x, y: Relation): int = cmp(y.strength, x.strength)
    sorted.sort(cmp_radiation)
    for ray in sorted:
      let d = society.containers[ray.dst]
      lines.add("  → #" & $ray.dst & " [L" & $d.tier & "] " & $d.descriptor & " | " & rel_name(ray.kind) & " | " & $ray.strength)
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════

var society = create_society()
echo "═══ 积木选址 · " & FQ_SM48_NAME & " + " & FQ_YM66_NAME & " ═══"
echo "系统身份: FQ-SM48(48进制) · FQ-YM66(66进制) · 独立命名 · 永久身份"
echo "容器数: " & $society.containers.len
echo ""
echo "── 预装容器 (码元直接显示) ──"
for c in society.containers:
  echo "  #" & $c.id & " [L" & $c.tier & "] " & c.descriptor & " 墙:" & $c.wall[0]

echo ""
echo "── 积木选址 ──"

# 用预装的积木测试
for i in [0, 5, 10]:
  if i < society.containers.len:
    let c = society.containers[i].code
    echo ""
    echo society.input_society(c.shengmu, c.yunmu, c.tone)
