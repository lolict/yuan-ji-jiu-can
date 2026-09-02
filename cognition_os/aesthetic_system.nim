## ═══════════════════════════════════════════════════════════════
## 审美系统 · 形式美学 · 和谐判定 · 平衡检测 · 结构完整性
## ═══════════════════════════════════════════════════════════════
## 核心:
##   对称美: 左右/上下配对的一致性
##   节奏美: 声母+韵母+声调的节拍感
##   平衡美: 各容器负载均衡
##   简约美: 无冗余 · 无过载
##   完整美: 所有必要元素齐全
## ═══════════════════════════════════════════════════════════════

import tables, strutils, math

# ═══════════════════════════════════════════════════════════════
# 审美维度
# ═══════════════════════════════════════════════════════════════
type
  AestheticDimension* = enum
    adSymmetry,      # 对称美
    adRhythm,        # 节奏美
    adBalance,       # 平衡美
    adSimplicity,    # 简约美
    adCompleteness,  # 完整美
    adHarmony        # 和谐美

  AestheticScore* = object
    dimension*: AestheticDimension
    score*: float             # 0-1
    detail*: string           # 评分细节

  AestheticVerdict* = enum
    avExcellent,     # 优秀(≥0.9)
    avGood,          # 良好(≥0.7)
    avAcceptable,    # 可接受(≥0.5)
    avPoor,          # 较差(<0.5)

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  AestheticSystem* = object
    scores*: seq[AestheticScore]
    containers*: Table[string, int]   # 容器名→元素数
    total_symbols*: int
    redundant_count*: int
    missing_elements*: seq[string]

func new_aesthetic_system*(): AestheticSystem =
  result.scores = @[]
  result.containers = initTable[string, int]()
  result.total_symbols = 0
  result.redundant_count = 0
  result.missing_elements = @[]

# ═══════════════════════════════════════════════════════════════
# 对称美评估
# ═══════════════════════════════════════════════════════════════
func eval_symmetry*(a: AestheticSystem): AestheticScore =
  ## 评估各容器数量是否对称/成比例
  var ratios: seq[float] = @[]
  for name, count in a.containers:
    if count > 0:
      ratios.add(float(count))
  
  if ratios.len < 2:
    return AestheticScore(dimension: adSymmetry, score: 0.5, detail: "容器不足 · 无法评估对称性")
  
  # 计算方差 · 方差小说成比例
  var sum = 0.0
  for r in ratios: sum += r
  let mean = sum / float(ratios.len)
  var variance = 0.0
  for r in ratios:
    variance += (r - mean) * (r - mean)
  variance = variance / float(ratios.len)
  
  # 归一化方差→分数
  let score = max(0.0, 1.0 - variance / 100.0)
  let detail = "容器比方差:" & formatFloat(variance, ffDecimal, 1) & " · " & $ratios.len & "个容器"
  
  return AestheticScore(dimension: adSymmetry, score: score, detail: detail)

# ═══════════════════════════════════════════════════════════════
# 节奏美评估
# ═══════════════════════════════════════════════════════════════
func eval_rhythm*(a: AestheticSystem): AestheticScore =
  ## 评估声母+韵母+声调的组合是否有节奏
  let sheng = a.containers.getOrDefault("声母")
  let yun = a.containers.getOrDefault("韵母")
  let tone = a.containers.getOrDefault("声调")
  
  if sheng == 0 or yun == 0 or tone == 0:
    return AestheticScore(dimension: adRhythm, score: 0.3, detail: "缺少声/韵/调 · 无节奏可言")
  
  let total_combos = sheng * yun * tone
  let density = float(total_combos) / float(max(a.total_symbols, 1))
  
  # 完整组合空间利用率
  let score = min(1.0, density * 10.0)
  let detail = "组合空间:" & $total_combos & " · 密度:" & formatFloat(density, ffDecimal, 3)
  
  return AestheticScore(dimension: adRhythm, score: score, detail: detail)

# ═══════════════════════════════════════════════════════════════
# 平衡美评估
# ═══════════════════════════════════════════════════════════════
func eval_balance*(a: AestheticSystem): AestheticScore =
  ## 各容器负载是否均衡
  if a.containers.len == 0:
    return AestheticScore(dimension: adBalance, score: 0, detail: "无容器")
  
  var min_count = high(int)
  var max_count = low(int)
  for _, count in a.containers:
    min_count = min(min_count, count)
    max_count = max(max_count, count)
  
  if max_count == 0:
    return AestheticScore(dimension: adBalance, score: 0, detail: "全部为空")
  
  let balance_ratio = float(min_count) / float(max_count)
  let score = balance_ratio
  let detail = "最满/最空比:" & formatFloat(balance_ratio, ffDecimal, 2) & " (" & $min_count & "/" & $max_count & ")"
  
  return AestheticScore(dimension: adBalance, score: score, detail: detail)

# ═══════════════════════════════════════════════════════════════
# 简约美评估
# ═══════════════════════════════════════════════════════════════
func eval_simplicity*(a: AestheticSystem): AestheticScore =
  ## 无冗余 · 无过载
  if a.total_symbols == 0:
    return AestheticScore(dimension: adSimplicity, score: 0.5, detail: "空系统")
  
  let redundancy_ratio = float(a.redundant_count) / float(a.total_symbols)
  let score = max(0.0, 1.0 - redundancy_ratio * 2.0)
  let detail = "冗余率:" & formatFloat(redundancy_ratio * 100, ffDecimal, 1) & "% (" & $a.redundant_count & "/" & $a.total_symbols & ")"
  
  return AestheticScore(dimension: adSimplicity, score: score, detail: detail)

# ═══════════════════════════════════════════════════════════════
# 完整美评估
# ═══════════════════════════════════════════════════════════════
func eval_completeness*(a: AestheticSystem): AestheticScore =
  ## 必要元素是否齐全
  let required = ["声母", "韵母", "声调"]
  var found = 0
  var missing: seq[string] = @[]
  
  for r in required:
    if a.containers.getOrDefault(r) > 0:
      found += 1
    else:
      missing.add(r)
  
  let score = float(found) / float(required.len)
  var detail = "必要:" & $found & "/" & $required.len
  if missing.len > 0:
    detail &= " · 缺:" & missing.join("/")
  
  return AestheticScore(dimension: adCompleteness, score: score, detail: detail)

# ═══════════════════════════════════════════════════════════════
# 和谐美评估(综合)
# ═══════════════════════════════════════════════════════════════
func eval_harmony*(a: AestheticSystem): AestheticScore =
  ## 综合和谐 = 各维度的加权平均
  var total = 0.0
  for s in a.scores:
    total += s.score
  let avg = total / float(max(a.scores.len, 1))
  
  return AestheticScore(dimension: adHarmony, score: avg, detail: "综合均值")

# ═══════════════════════════════════════════════════════════════
# 全面评估
# ═══════════════════════════════════════════════════════════════
proc evaluate*(a: var AestheticSystem): AestheticVerdict =
  a.scores = @[]
  a.scores.add(eval_symmetry(a))
  a.scores.add(eval_rhythm(a))
  a.scores.add(eval_balance(a))
  a.scores.add(eval_simplicity(a))
  a.scores.add(eval_completeness(a))
  a.scores.add(eval_harmony(a))
  
  let avg = a.scores[a.scores.len - 1].score  # harmony is last
  
  if avg >= 0.9: return avExcellent
  elif avg >= 0.7: return avGood
  elif avg >= 0.5: return avAcceptable
  else: return avPoor

# ═══════════════════════════════════════════════════════════════
# 报告
# ═══════════════════════════════════════════════════════════════
proc report*(a: AestheticSystem; verdict: AestheticVerdict) =
  echo "═══ 审美系统报告 ═══"
  echo ""
  
  let dim_names = ["对称美", "节奏美", "平衡美", "简约美", "完整美", "和谐美(综合)"]
  echo "── 六维评分 ──"
  for i, s in a.scores:
    let name = if i < dim_names.len: dim_names[i] else: $s.dimension
    let bar = repeat("█", int(s.score * 10)) & repeat("░", 10 - int(s.score * 10))
    echo "  ", alignLeft(name, 8), " [", bar, "] ", formatFloat(s.score, ffDecimal, 2)
    echo "    ", s.detail
  
  echo ""
  let verdict_str = case verdict:
    of avExcellent: "★ 优秀"
    of avGood: "✓ 良好"
    of avAcceptable: "△ 可接受"
    of avPoor: "✗ 较差"
  echo "综合裁决: ", verdict_str

# ═══════════════════════════════════════════════════════════════
# 运行
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var a = new_aesthetic_system()
  
  # 设置容器数据(从container_instance.nim导入)
  a.containers["声母"] = 24
  a.containers["韵母"] = 66
  a.containers["声调"] = 4
  a.containers["颜色"] = 42
  a.containers["空间"] = 18
  a.containers["音乐"] = 3
  a.total_symbols = 157
  a.redundant_count = 0
  
  echo "═══ 审美系统 · 满全法形式美学评估 ═══"
  echo "容器: ", a.containers.len
  echo "符号: ", a.total_symbols
  echo ""
  
  let verdict = a.evaluate()
  a.report(verdict)
