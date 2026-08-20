## 积木选址 · 范畴辐射 — 满全法版
##
## 使用自有进制: 声母48 · 韵母66 · 声调5
## 选址公式: (声母idx * 66 * 5 + 韵母idx * 5 + 声调) mod 容器数
## 没有 ASCII · 没有工业进制 · 全是满全法自己的编码表

import std/[sets, algorithm, strutils]

# ═══════════════════════════════════════════════════════════════
# 满全法自有编码表 (来自 encoding/shengmu.h · yunmu.h)
# ═══════════════════════════════════════════════════════════════

const
  # 进制宽度 (满全法自有 · 非工业进制)
  SHENGMU_RADIX = 48    # 声母 48 进制 (24小写 + 24大写)
  YUNMU_RADIX   = 66    # 韵母 66 进制 (33小写 + 33大写)
  TONE_RADIX    = 5     # 声调 5 进制 (1-4 + 轻声0)

  # 感知家族
  FAM_CONTAINER = 'C'
  FAM_BOUNDARY  = 'B'
  FAM_COUPLE    = 'P'
  FAM_WEB       = 'W'
  FAM_NEUTRAL   = 'N'

  # 极性
  POLAR_WO  = '1'   # 我
  POLAR_CHT = '0'   # 刘楚恬

# 声母表 (48进制 · index 0..23 小写 · 24..47 大写)
const SHENGMU_NAMES = [
  "b", "p", "m", "f", "d", "t", "n", "l",
  "g", "k", "h", "w", "j", "q", "x", "y",
  "z", "c", "s", "r", "zh", "ch", "sh", "ri",
]

# 韵母表 (66进制 · index 0..31 偶数小写 · 奇数大写)
const YUNMU_NAMES = [
  "a", "o", "e", "i", "u", "ai", "ei", "ao",
  "ou", "er", "ia", "ie", "ua", "uo", "an", "en",
  "in", "ün", "ang", "eng", "ing", "ong", "iao", "uai",
  "ian", "uan", "üan", "iang", "uang", "iong", "üe", "iu",
]

# ═══════════════════════════════════════════════════════════════
# 编码: 家族 + 极性 + 声母idx + 韵母idx + 声调
# ═══════════════════════════════════════════════════════════════

type
  FQCode* = object
    family*: char     # C/B/P/W/N
    polarity*: char   # 1/0
    shengmu_idx*: int # 0..47
    yunmu_idx*: int   # 0..65
    tone*: int        # 1..4 (0=轻声)

  RelationKind* = enum
    relSameFamily
    relSamePolarity
    relComplement
    relCrossTier
    relToneHarmony

  Container* = object
    id*: int
    tier*: int
    wall_left*: char
    wall_right*: char
    polarity*: char
    code*: FQCode
    descriptor*: string

  Relation* = object
    src*: int
    dst*: int
    kind*: RelationKind
    strength*: float

  AddressingSociety* = object
    containers*: seq[Container]

# ═══════════════════════════════════════════════════════════════
# 辅助
# ═══════════════════════════════════════════════════════════════

proc shengmu_to_idx*(name: string): int =
  for i, n in SHENGMU_NAMES:
    if n == name: return i
  return -1

proc yunmu_to_idx*(name: string): int =
  for i, n in YUNMU_NAMES:
    if n == name: return i * 2  # 偶数=小写
  return -1

proc fq_desc*(c: FQCode): string =
  var sm = "?"
  var ym = "?"
  if c.shengmu_idx >= 0 and c.shengmu_idx < SHENGMU_NAMES.len:
    sm = SHENGMU_NAMES[c.shengmu_idx]
  if c.yunmu_idx >= 0 and c.yunmu_idx < YUNMU_NAMES.len * 2:
    ym = YUNMU_NAMES[c.yunmu_idx div 2]
  result = $c.family & $c.polarity & sm & ym & $c.tone

# ═══════════════════════════════════════════════════════════════
# 积木选址 — 用满全法自己的进制
# ═══════════════════════════════════════════════════════════════
#
## 公式: address = (声母idx * YUNMU_RADIX * TONE_RADIX + 韵母idx * TONE_RADIX + 声调) mod 容器数
## 这是满全法自有进制的组合 · 没有工业时代数字参与

proc block_addressing*(sm_idx, ym_idx, tone: int, containers: seq[Container]): int =
  if containers.len == 0: return -1
  
  # 满全法自有公式: 声母48 * 韵母66 * 声调5 = 15840 种组合空间
  let slot = (sm_idx * YUNMU_RADIX * TONE_RADIX + ym_idx * TONE_RADIX + tone) mod containers.len
  
  # 线性探测 · 找到预装了该组合的容器
  for attempt in 0 ..< containers.len:
    let idx = (slot + attempt) mod containers.len
    let pre = containers[idx].code
    if pre.shengmu_idx == sm_idx and pre.yunmu_idx == ym_idx and pre.tone == tone:
      return idx
  
  return -1  # 未预装

# ═══════════════════════════════════════════════════════════════
# 范畴辐射 — 线性衰减
# ═══════════════════════════════════════════════════════════════

proc compute_radiation*(src: Container, dst: Container): float =
  var strength = 0.0
  
  if src.code.family == dst.code.family:
    strength += 0.4
  if src.polarity == dst.polarity:
    strength += 0.3
  else:
    strength += 0.2
  if src.code.tone == dst.code.tone:
    strength += 0.2
  elif (src.code.tone + dst.code.tone) == 5:  # 1+4=5 · 2+3=5 · 调互补
    strength += 0.1
  
  let tier_diff = abs(src.tier - dst.tier)
  if tier_diff == 1: strength *= 0.7
  elif tier_diff == 2: strength *= 0.4
  elif tier_diff >= 3: strength *= 0.2
  
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
# 创建社会: 预装容器 · 每个放一个满全法编码
# ═══════════════════════════════════════════════════════════════

proc create_society*(tier_count: int, per_tier: int): AddressingSociety =
  var families = [FAM_CONTAINER, FAM_BOUNDARY, FAM_COUPLE]
  var polarities = [POLAR_WO, POLAR_CHT]
  var sm_idx = 0
  var ym_idx = 0
  var tone_val = 1
  var fam_idx = 0
  var pl_idx = 0
  
  for tier in 1 .. tier_count:
    for sub in 0 ..< per_tier:
      let code = FQCode(
        family: families[fam_idx mod families.len],
        polarity: polarities[pl_idx mod polarities.len],
        shengmu_idx: sm_idx,
        yunmu_idx: ym_idx,
        tone: tone_val,
      )
      
      var desc = fq_desc(code)
      
      var c = Container(
        id: result.containers.len,
        tier: tier,
        wall_left: "knd"[(tier-1) mod 3],
        wall_right: "knd"[(tier-1) mod 3],
        polarity: code.polarity,
        code: code,
        descriptor: desc,
      )
      result.containers.add(c)
      
      # 轮转 · 用满全法自己的进制递进
      sm_idx = (sm_idx + 1) mod SHENGMU_NAMES.len
      ym_idx = (ym_idx + 2) mod (YUNMU_NAMES.len * 2)  # 偶数递增
      tone_val = tone_val + 1
      if tone_val > 4: tone_val = 1
      fam_idx += 1
      pl_idx += 1

# ═══════════════════════════════════════════════════════════════
# 输入积木 → 选址 → 辐射
# ═══════════════════════════════════════════════════════════════

proc input_blocks*(society: AddressingSociety, sm_name, ym_name: string, tone: int): string =
  let sm_idx = shengmu_to_idx(sm_name)
  let ym_idx = yunmu_to_idx(ym_name)
  
  if sm_idx < 0:
    return "声母\"" & sm_name & "\"不在满全法表里"
  if ym_idx < 0:
    return "韵母\"" & ym_name & "\"不在满全法表里"
  
  let target_id = block_addressing(sm_idx, ym_idx, tone, society.containers)
  
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add(" 积木选址 → 范畴辐射 (满全法自有进制)")
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add("输入积木: 声母=" & sm_name & "(idx" & $sm_idx & ") 韵母=" & ym_name & "(idx" & $ym_idx & ") 声调=" & $tone)
  lines.add("选址公式: (" & $sm_idx & " * " & $YUNMU_RADIX & " * " & $TONE_RADIX & " + " & $ym_idx & " * " & $TONE_RADIX & " + " & $tone & ") mod " & $society.containers.len)
  lines.add("        = " & $(sm_idx * YUNMU_RADIX * TONE_RADIX + ym_idx * TONE_RADIX + tone) & " mod " & $society.containers.len)
  lines.add("───────────────────────────────────────────────────────────")
  
  if target_id < 0:
    lines.add("✗ 未找到 · 这个组合未预装")
    result = lines.join("\n")
    return
  
  let target = society.containers[target_id]
  
  let rays = category_radiation(target_id, society.containers, 3)
  
  lines.add("✓ 选址成功 → 容器#" & $target_id & " [L" & $target.tier & "]")
  lines.add("  预装: " & target.descriptor)
  let polar_str = if target.polarity == '1': "阳(我)" else: "阴(刘楚恬)"
  lines.add("  极性: " & polar_str)
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add("辐射范围: " & $rays.len & " 个关联")
  
  var sorted_rays = rays
  proc cmp_strength(x, y: Relation): int = cmp(y.strength, x.strength)
  sorted_rays.sort(cmp_strength)
  
  for ray in sorted_rays:
    let dst = society.containers[ray.dst]
    let kind_str = case ray.kind:
      of relSameFamily: "同家族"
      of relSamePolarity: "同极性"
      of relComplement: "互补"
      of relCrossTier: "跨层"
      of relToneHarmony: "调谐"
    lines.add("  → #" & $ray.dst & " [L" & $dst.tier & "] " & $dst.descriptor & " | " & kind_str & " | " & $ray.strength)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════

proc main() =
  echo "═══════════════════════════════════════════════════════════"
  echo " 积木选址 · 范畴辐射"
  echo " 满全法进制: 声母48 · 韵母66 · 声调5"
  echo "═══════════════════════════════════════════════════════════"
  
  var society = create_society(tier_count = 5, per_tier = 3)
  echo "容器: " & $society.containers.len
  echo ""
  echo "── 预装组合 ──"
  for i, c in society.containers:
    echo "  #" & $i & " [L" & $c.tier & "] " & $c.descriptor
  
  echo ""
  echo "── 积木选址测试 ──"
  echo ""
  
  # 用预装的实际积木测试
  for i in [0, 3, 7, 12]:
    if i < society.containers.len:
      let c = society.containers[i].code
      var sm = SHENGMU_NAMES[c.shengmu_idx]
      var ym = YUNMU_NAMES[c.yunmu_idx div 2]
      echo society.input_blocks(sm, ym, c.tone)
      echo ""

when isMainModule:
  main()
