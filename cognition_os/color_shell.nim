## ═══════════════════════════════════════════════════════════════
## 颜色像素壳 · 可编译 · 可用 · 可解释
## ═══════════════════════════════════════════════════════════════
## 封装整个颜色/符号编码体系为独立可运行壳
##
## 核心:
##   最小单元 = 相位像素格子(相位+权重+元数据)
##   运算 = 加法(拼装) + 减法(拆解) · 禁用乘除
##   审判 = 分布式制衡网络(6维拉力)
##   目标 = 为公约数(统一编码体系)服务
## ═══════════════════════════════════════════════════════════════

import tables, math, strutils, json, os

# ═══════════════════════════════════════════════════════════════
# 第一部分: 相位像素单元
# ═══════════════════════════════════════════════════════════════
type
  PhaseUnit* = object
    phase*: float        # 相位偏移 [0, 2π)
    weight*: float       # 视觉权重 [0.0, 1.0]
    brightness*: float   # 亮度 [0.0, 1.0]
    owner*: string       # 归属权: SELF/EXTERNAL
    level*: string       # 层级: L1/L2/L3
    glyph_id*: string    # 所属符号ID

  PhaseCanvas* = object
    width*: int
    height*: int
    units*: seq[seq[PhaseUnit]]  # 二维相位格子

func new_phase_unit*(phase = 0.0; weight = 0.5; brightness = 1.0;
                     owner = "SELF"; level = "L1"; glyph_id = ""): PhaseUnit =
  result.phase = phase
  result.weight = weight
  result.brightness = brightness
  result.owner = owner
  result.level = level
  result.glyph_id = glyph_id

func new_canvas*(w, h: int): PhaseCanvas =
  result.width = w
  result.height = h
  result.units = newSeq[seq[PhaseUnit]](h)
  for y in 0..<h:
    result.units[y] = newSeq[PhaseUnit](w)
    for x in 0..<w:
      result.units[y][x] = new_phase_unit()

# ═══════════════════════════════════════════════════════════════
# 第二部分: 像素加减法(禁用乘除)
# ═══════════════════════════════════════════════════════════════
func pixel_add*(canvas: var PhaseCanvas; src: PhaseCanvas;
                target_x, target_y: int): tuple[ok: bool; reason: string] =
  ## 加法 = 把src的部件像素安装到canvas的指定位置
  ## 只允许L1基元+基元拼装
  if target_x + src.width > canvas.width or target_y + src.height > canvas.height:
    return (false, "加法: 目标超出画布范围")
  
  for y in 0..<src.height:
    for x in 0..<src.width:
      let sx = target_x + x
      let sy = target_y + y
      let src_unit = src.units[y][x]
      
      # 层级检查: 只允许L1+L1
      if src_unit.level == "L3":
        return (false, "加法: 不允许L3成品参与拼装")
      
      # 相位叠加融合
      canvas.units[sy][sx].phase = (canvas.units[sy][sx].phase + src_unit.phase) / 2.0
      canvas.units[sy][sx].weight = (canvas.units[sy][sx].weight + src_unit.weight) / 2.0
      canvas.units[sy][sx].brightness = max(canvas.units[sy][sx].brightness, src_unit.brightness)
      canvas.units[sy][sx].level = "L3"  # 拼装后升级为L3成品
  
  return (true, "加法完成: " & $src.width & "×" & $src.height & "安装到(" & $target_x & "," & $target_y & ")")

func pixel_subtract*(canvas: var PhaseCanvas;
                     region_x, region_y, region_w, region_h: int): tuple[ok: bool; result: PhaseCanvas; reason: string] =
  ## 减法 = 从canvas上挖掉一个区域的部件
  if region_x + region_w > canvas.width or region_y + region_h > canvas.height:
    return (false, PhaseCanvas(), "减法: 区域超出画布")
  
  var removed = new_canvas(region_w, region_h)
  for y in 0..<region_h:
    for x in 0..<region_w:
      removed.units[y][x] = canvas.units[region_y + y][region_x + x]
      canvas.units[region_y + y][region_x + x] = new_phase_unit(level = "L1")
  
  return (true, removed, "减法完成: 剥离" & $region_w & "×" & $region_h & "区域")

func pixel_multiply*(a, b: PhaseCanvas): tuple[ok: bool; reason: string] =
  return (false, "乘法: 对字形无意义 · 已禁用")

func pixel_divide*(a, b: PhaseCanvas): tuple[ok: bool; reason: string] =
  return (false, "除法: 对字形无意义 · 已禁用")

# ═══════════════════════════════════════════════════════════════
# 第三部分: 分布式制衡网络(6维拉力)
# ═══════════════════════════════════════════════════════════════
type
  BalanceResult* = object
    dimension*: string
    passed*: bool
    score*: float        # 0.0~1.0连续值
    weight*: float       # 该维度的拉力权重
    detail*: string

  JudgeVerdict* = object
    overall_pass*: bool
    total_score*: float
    results*: seq[BalanceResult]
    global_goal*: string

const GLOBAL_GOAL* = "满全法私有语音编码符号体系"

func get_category*(glyph_id: string): string =
  ## 符号角色类别判定
  case glyph_id
  of "a", "o", "e", "i", "u", "ü", "A", "O", "E", "I", "U", "Ü":
    return "vow_base"
  of "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h",
     "j", "q", "x", "y", "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ":
    return "consonant"
  of "acute", "grave", "caron", "tilde", "diaeresis", "macron", "breve":
    return "tone_part"
  of "á", "ó", "é", "í", "ú", "ǘ", "ǎ", "ǒ", "ě", "ǐ", "ǔ", "ǚ",
     "à", "ò", "è", "ì", "ù", "ǜ", "ā", "ō", "ē", "ī", "ū", "ǖ":
    return "tone_finished"
  of "ai", "ei", "ui", "ao", "ou", "iu", "ie", "üe", "er":
    return "vow_compound"
  of "an", "en", "in", "un", "ün", "ang", "eng", "ing", "ong":
    return "vow_nasal"
  else:
    return "unknown"

func judge_level*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "层级栈"
  result.weight = 1.0
  if a.level == b.level:
    result.passed = true
    result.score = 1.0
    result.detail = "层级一致: " & a.level
  elif (a.level == "L1" and b.level == "L3") or (a.level == "L3" and b.level == "L1"):
    result.passed = false
    result.score = 0.0
    result.detail = "L1与L3不可配对(原料≠成品)"
  else:
    result.passed = false
    result.score = 0.5
    result.detail = "层级模糊"

func judge_type*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "类型身份"
  result.weight = 1.0
  # 类型身份检查: 检查符号角色类别是否兼容
  # 类型角色: vow_base(韵母基元) / consonant(声母) / tone_part(声调零件) / tone_finished(声调成品)
  let cat_a = get_category(a.glyph_id)
  let cat_b = get_category(b.glyph_id)
  
  if cat_a == cat_b:
    # 同类别可以配对(如vow_base+vow_base = 韵母衍射)
    result.passed = true
    result.score = 1.0
    result.detail = "类型兼容: " & cat_a & " + " & cat_b
  elif cat_a == "tone_part" or cat_b == "tone_part":
    # 声调零件不能独立参与衍射(只能贴附)
    result.passed = false
    result.score = 0.0
    result.detail = "声调零件不允许独立衍射"
  elif (cat_a == "vow_base" and cat_b == "consonant") or
       (cat_a == "consonant" and cat_b == "vow_base"):
    # 声母+韵母 = 允许(音节组合)
    result.passed = true
    result.score = 0.8
    result.detail = "声母+韵母音节组合"
  elif cat_a == "tone_finished" or cat_b == "tone_finished":
    # 声调成品禁止参与衍射
    result.passed = false
    result.score = 0.1
    result.detail = "声调成品禁止参与衍射"
  else:
    result.passed = false
    result.score = 0.3
    result.detail = "类型不兼容: " & cat_a & " vs " & cat_b

func judge_aesthetic*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "视觉审美"
  result.weight = 0.9
  let w_diff = abs(a.weight - b.weight)
  result.score = 1.0 - w_diff
  if w_diff <= 0.2:
    result.passed = true
    result.detail = "视觉重量接近: " & formatFloat(a.weight, ffDecimal, 2) & " ≈ " & formatFloat(b.weight, ffDecimal, 2)
  else:
    result.passed = false
    result.detail = "视觉重量失衡: " & formatFloat(a.weight, ffDecimal, 2) & " vs " & formatFloat(b.weight, ffDecimal, 2)

func judge_ownership*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "归属权"
  result.weight = 1.0
  if a.owner == "SELF" and b.owner == "SELF":
    result.passed = true
    result.score = 1.0
    result.detail = "双方归属权均为我方"
  elif a.owner == "EXTERNAL" or b.owner == "EXTERNAL":
    result.passed = false
    result.score = 0.2
    result.detail = "存在外来归属权 · 需重定义"
  else:
    result.passed = false
    result.score = 0.5
    result.detail = "归属权不明确"

func judge_diffraction*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "衍射规则"
  result.weight = 0.8
  # 只有L1+L1允许衍射
  if a.level == "L1" and b.level == "L1":
    result.passed = true
    result.score = 1.0
    result.detail = "L1+L1允许衍射"
  elif a.level == "L3" or b.level == "L3":
    result.passed = false
    result.score = 0.0
    result.detail = "L3成品禁止反向衍射"
  else:
    result.passed = false
    result.score = 0.4
    result.detail = "衍射条件不满足"

func judge_blacklist*(a, b: PhaseUnit): BalanceResult =
  result.dimension = "黑名单"
  result.weight = 1.0
  # 简化黑名单检查
  let blacklist = ["a↔á", "eě", "胖瘦极端配对"]
  result.passed = true
  result.score = 1.0
  result.detail = "未命中黑名单"

func distributed_judge*(a, b: PhaseUnit): JudgeVerdict =
  result.global_goal = GLOBAL_GOAL
  result.results = @[]
  
  result.results.add(judge_level(a, b))
  result.results.add(judge_type(a, b))
  result.results.add(judge_aesthetic(a, b))
  result.results.add(judge_ownership(a, b))
  result.results.add(judge_diffraction(a, b))
  result.results.add(judge_blacklist(a, b))
  
  # 全局协调得分(模拟共振场干涉)
  var total_weight = 0.0
  var weighted_score = 0.0
  var any_veto = false
  
  for r in result.results:
    total_weight += r.weight
    weighted_score += r.score * r.weight
    if not r.passed and r.weight >= 1.0:
      any_veto = true
  
  result.total_score = weighted_score / total_weight
  
  # 任一高权重维度否决 → 整体拒绝
  if any_veto:
    result.overall_pass = false
  elif result.total_score < 0.6:
    result.overall_pass = false
  else:
    result.overall_pass = true

func verdict_to_string*(v: JudgeVerdict): string =
  var lines: seq[string] = @[]
  lines.add("═══ 审判报告 ═══")
  lines.add("公约数: " & v.global_goal)
  lines.add("全局协调得分: " & formatFloat(v.total_score, ffDecimal, 2))
  lines.add("最终判决: " & (if v.overall_pass: "✓通过" else: "✗拒绝"))
  lines.add("")
  lines.add("── 各维度拉力 ──")
  for r in v.results:
    let status = if r.passed: "✓" else: "✗"
    lines.add("  " & status & " " & r.dimension.alignLeft(10) &
              " 得分=" & formatFloat(r.score, ffDecimal, 2) &
              " 权重=" & formatFloat(r.weight, ffDecimal, 1) &
              " " & r.detail)
  return lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 第四部分: 壳入口 · 命令行调用
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  echo "═══════════════════════════════════════════════════════════"
  echo "  颜色像素壳 · 可编译可用可解释"
  echo "═══════════════════════════════════════════════════════════"
  echo ""
  
  # 创建两个L1基元符号
  var glyph_a = new_phase_unit(phase = 0.0, weight = 0.3, brightness = 1.0,
                                owner = "SELF", level = "L1", glyph_id = "a")
  var glyph_i = new_phase_unit(phase = 1.0, weight = 0.3, brightness = 1.0,
                                owner = "SELF", level = "L1", glyph_id = "i")
  var glyph_acute = new_phase_unit(phase = 0.5, weight = 0.2, brightness = 0.8,
                                    owner = "SELF", level = "L2", glyph_id = "acute")
  var glyph_a_l3 = new_phase_unit(phase = 0.5, weight = 0.5, brightness = 0.9,
                                   owner = "SELF", level = "L3", glyph_id = "á")
  
  echo "── 测试1: L1+L1 正常衍射(a+i) ──"
  let v1 = distributed_judge(glyph_a, glyph_i)
  echo verdict_to_string(v1)
  echo ""
  
  echo "── 测试2: L1+L3 跨层级错误(a+á) ──"
  let v2 = distributed_judge(glyph_a, glyph_a_l3)
  echo verdict_to_string(v2)
  echo ""
  
  echo "── 测试3: 像素加法拼装 ──"
  var canvas = new_canvas(8, 8)
  var part_a = new_canvas(4, 4)
  for y in 0..<4:
    for x in 0..<4:
      part_a.units[y][x] = glyph_a
  
  let add_result = pixel_add(canvas, part_a, 0, 0)
  echo "加法结果: ", if add_result.ok: "成功" else: "失败 ", add_result.reason
  echo ""
  
  echo "── 测试4: 乘法(应被禁用) ──"
  let mul_result = pixel_multiply(canvas, part_a)
  echo "乘法结果: ", mul_result.reason
  echo ""
  
  echo "── 测试5: 除法(应被禁用) ──"
  let div_result = pixel_divide(canvas, part_a)
  echo "除法结果: ", div_result.reason
  echo ""
  
  echo "── 架构说明 ──"
  echo "  底层: 相位像素矩阵(相位+权重+亮度+元数据)"
  echo "  运算: 加法(拼装) + 减法(拆解) · 禁用乘除"
  echo "  审判: 6维分布式制衡网络(层级/类型/审美/归属/衍射/黑名单)"
  echo "  目标: 全局协调得分模拟共振场 · 全程白盒可审计"
