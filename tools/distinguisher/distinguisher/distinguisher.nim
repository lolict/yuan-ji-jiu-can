# ============================================================
# 分别器 (Distinguisher) · 核心引擎
#
# 三层路由实现：
#   第一层：感知家族输入 → 视觉族
#   第二层：谐音靶点发音 → 靶点对象
#   第三层：主权路由靶点身份 → 主权级别
# ============================================================

import tables
import strutils
import sequtils
import options
import ./types

# 创建分别器
proc new_distinguisher*(): Distinguisher =
  var d = Distinguisher(
    family_map: init_table[string, GlyphFamily](),
    sound_map: init_table[string, SoundTarget](),
    sovereignty_map: init_table[SoundTarget, Sovereignty](),
    history: @[],
    total_routed: 0,
    total_acked: 0,
    total_broken: 0,
    family_count: 0,
    sound_count: 0,
    sovereignty_count: 0
  )
  
  # ── 初始化第一层：字符 → 家族 ──
  # 容器族（方形/圆形 → 刘楚恬）
  let container_chars = ["因", "阴", "⭕", "回", "囡", "口", "袋", "米", "容器", "编码"]
  for c in container_chars:
    d.family_map[c] = containerGlyph
  d.family_count += container_chars.len
  
  # 界线族（竖长/界线 → 我）
  let phallic_chars = ["形", "男", "丨", "囝", "边界", "界线"]
  for c in phallic_chars:
    d.family_map[c] = phallicGlyph
  d.family_count += phallic_chars.len
  
  # 夫妻图腾
  let couple_chars = ["囡囝", "夫妻", "命运共同体"]
  for c in couple_chars:
    d.family_map[c] = coupleGlyph
  d.family_count += couple_chars.len
  
  # 谐音桥
  let homophone_chars = ["莫", "连", "毛", "兰", "恋", "初", "lct", "gtt", "mlct", "mltt"]
  for c in homophone_chars:
    d.family_map[c] = homophoneBridge
  d.family_count += homophone_chars.len
  
  # ── 初始化第二层：谐音 → 靶点 ──
  d.sound_map["毛利兰"] = wife
  d.sound_map["莫刘连理"] = wife
  d.sound_map["lctfrqmy"] = wife
  d.sound_map["lctqf"] = wife
  d.sound_map["工藤新一"] = husband
  d.sound_map["恋初"] = husband
  d.sound_map["wgslz"] = husband
  d.sound_map["mlct"] = sweet
  d.sound_map["mltt"] = sweet
  d.sound_map["ml.tt"] = sweet
  d.sound_map["gtt"] = soundMorph
  d.sound_map["外部"] = external
  d.sound_map["系统"] = SoundTarget.system
  d.sound_count = 13
  
  # ── 初始化第三层：靶点 → 主权 ──
  d.sovereignty_map[wife] = active
  d.sovereignty_map[husband] = active
  d.sovereignty_map[sweet] = active
  d.sovereignty_map[soundMorph] = active
  d.sovereignty_map[external] = passive
  d.sovereignty_map[SoundTarget.system] = Sovereignty.system
  d.sovereignty_count = 5
  
  return d

# ── 第一层路由：感知家族 ──
proc route_family*(d: Distinguisher, input: string): GlyphFamily =
  if d.family_map.hasKey(input):
    return d.family_map[input]
  
  # 启发式判断
  if input.contains("囡") or input.contains("袋") or input.contains("容器"):
    return containerGlyph
  if input.contains("囝") or input.contains("男") or input.contains("界"):
    return phallicGlyph
  if input.contains("囡囝") or input.contains("夫妻"):
    return coupleGlyph
  if input.contains("lct") or input.contains("gtt") or input.contains("ml"):
    return homophoneBridge
  
  return unknownGlyph

# ── 第二层路由：谐音靶点 ──
proc route_target*(d: Distinguisher, input: string): SoundTarget =
  if d.sound_map.hasKey(input):
    return d.sound_map[input]
  
  # 启发式判断
  if input.contains("毛利兰") or input.contains("lct") or input.contains("莫"):
    return wife
  if input.contains("工藤") or input.contains("恋初") or input.contains("wgslz"):
    return husband
  if input.contains("gtt"):
    return soundMorph
  
  return external

# ── 第三层路由：主权级别 ──
proc route_sovereignty*(d: Distinguisher, target: SoundTarget): Sovereignty =
  if d.sovereignty_map.hasKey(target):
    return d.sovereignty_map[target]
  return system

# ── 完整路由（三层一起） ──
proc route*(d: var Distinguisher, input: string): RouteResult =
  var path: seq[string] = @[]
  
  # 第一层
  let family = d.route_family(input)
  path.add("family:" & $family)
  
  # 第二层
  let target = d.route_target(input)
  path.add("target:" & $target)
  
  # 第三层
  let sovereignty = d.route_sovereignty(target)
  path.add("sovereignty:" & $sovereignty)
  
  # 闭环条件：target 是否 ackable
  let ackable = (target in [wife, husband, sweet, soundMorph]) and 
                (sovereignty == active)
  
  # 置信度
  var confidence = 0.85
  if family == unknownGlyph:
    confidence -= 0.3
  if target == external:
    confidence -= 0.2
  
  let result = RouteResult(
    input: input,
    family: family,
    target: target,
    sovereignty: sovereignty,
    confidence: confidence,
    path: path,
    ackable: ackable
  )
  
  # 记录历史
  d.total_routed += 1
  if ackable:
    d.total_acked += 1
  else:
    d.total_broken += 1
  
  d.history.add(RouteRecord(
    result: result,
    timestamp: d.total_routed,
    success: ackable
  ))
  
  return result

# ── 家族名称 ──
proc family_name*(f: GlyphFamily): string =
  case f
  of containerGlyph: "容器族"
  of phallicGlyph: "界线族"
  of coupleGlyph: "夫妻图腾"
  of homophoneBridge: "谐音桥"
  of unknownGlyph: "未知"

# ── 靶点名称 ──
proc target_name*(t: SoundTarget): string =
  case t
  of wife: "Wife(刘楚恬)"
  of husband: "Husband(我)"
  of sweet: "Sweet(→Wife)"
  of soundMorph: "SoundMorphism"
  of external: "外部参与者"
  of system: "系统级"

# ── 主权名称 ──
proc sovereignty_name*(s: Sovereignty): string =
  case s
  of active: "完全权限"
  of passive: "只读权限"
  of sleeping: "休眠可唤醒"
  of system: "系统级"

# ── 统计 ──
proc stats*(d: Distinguisher): string =
  var s = "═══ 分别器统计 ═══\n"
  s &= "总路由数: " & $d.total_routed & "\n"
  s &= "成功闭环: " & $d.total_acked & "\n"
  s &= "流断裂: " & $d.total_broken & "\n"
  if d.total_routed > 0:
    let rate = float(d.total_acked) / float(d.total_routed) * 100.0
    s &= "闭环率: " & $rate & "%\n"
  return s
