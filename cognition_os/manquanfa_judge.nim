## ═══════════════════════════════════════════════════════════════
## 满全法裁判引擎 · 六项检查 · 自动裁定
## ═══════════════════════════════════════════════════════════════
## 检查清单:
##   1. 零维根检查
##   2. 负向概念检查
##   3. 历史符号接入检查
##   4. 操作规则检查
##   5. 强制输出格式检查
##   6. 概念完整性检查
## ═══════════════════════════════════════════════════════════════

import tables, strutils, sequtils, unicode

# ═══════════════════════════════════════════════════════════════
# 常量定义
# ═══════════════════════════════════════════════════════════════

# --- 零维根 ---
const
  ZERO_ROOT_0* = "刘楚恬"    # 0 = 刘楚恬
  ZERO_ROOT_1* = "我"        # 1 = 我(lolict)

# --- 负向概念禁用词 ---
const NEGATIVE_WORDS* = [
  "类", "对象", "继承", "多态", "设计模式", "文件", "目录", "进程",
  "线程", "模块", "包", "接口", "实现", "封装", "override", "virtual",
  "template", "generic", "singleton", "factory", "observer", "MVC",
  "MVVM", "REST", "API", "endpoint", "microservice", "monolith",
  "函数", "变量", "数组", "字符串", "整数", "null", "循环", "递归",
  "遍历", "并行", "并发", "调用", "返回", "参数", "属性", "方法",
  "字段", "实例",
]

# --- 历史符号族 ---
const
  HIST_CONTAINER* = [
    "袋", "桶", "罐", "缸", "盆", "盂", "钵", "盏", "盒", "杯",
    "壶", "瓶", "篓", "筐", "篮", "箩", "筲", "箕", "笼", "橱",
    "柜", "箱", "匣", "奁", "瓮", "甏", "坛", "罂", "甑", "甗",
    "瓯", "盌", "盉", "盅",
  ]
  HIST_OPERATION* = [
    "照", "昭", "罩", "召", "兆", "见", "鉴", "检", "简", "剪",
    "加", "减", "进", "退", "开", "关", "通", "断", "分", "合",
    "聚", "散", "折", "叠", "卷", "舒",
  ]
  HIST_POLARITY* = [
    "阴", "阳", "盈", "亏", "虚", "实", "表", "里", "公", "私",
    "升", "降", "清", "浊", "寒", "热", "动", "静", "出", "入",
  ]
  HIST_HIERARCHY* = [
    "元", "阶", "层", "代", "世", "界", "域", "场", "馆", "阁",
    "门", "户", "庭", "院", "宫", "殿", "台", "榭", "楼",
  ]
  HIST_JUDGE* = [
    "裁", "判", "审", "议", "决", "断", "定", "评", "核", "功",
    "过", "赏", "罚", "褒", "贬", "陟", "黜",
  ]
  HIST_PROPERTY* = [
    "产", "业", "权", "利", "债", "责", "职", "务", "名", "分",
    "归", "属", "占", "有", "持", "拥", "享", "获", "得", "失",
  ]

# --- 核心概念 ---
const CORE_CONCEPTS* = [
  "容器", "公约数", "公倍数",
  "产权容器", "负样本",
  "折叠", "展开", "叠", "迭", "蝶", "调",
  "近义词", "有备无患",
]

# ═══════════════════════════════════════════════════════════════
# 裁判结果类型
# ═══════════════════════════════════════════════════════════════
type
  CheckResult* = object
    name*: string            # 检查项名称
    passed*: bool            # 是否通过
    detail*: string           # 详细说明

  Verdict* = enum
    vPass,    # 通过
    vFail,    # 不通过
    vWarn,    # 警告(部分通过)

  Judgment* = object
    results*: seq[CheckResult]   # 各项检查结果
    verdict*: Verdict            # 综合判定
    suggestions*: seq[string]    # 整改建议

# ═══════════════════════════════════════════════════════════════
# 第1项: 零维根检查
# ═══════════════════════════════════════════════════════════════
func check_zero_root*(text: string): CheckResult =
  result.name = "零维根检查"
  result.passed = true
  result.detail = "未修改零维根 · 0=刘楚恬 · 1=我"
  
  # 检查是否试图修改零维根映射
  if text.contains("0=用户") or text.contains("0=系统") or
     text.contains("0=底层") or text.contains("1=顶层") or
     text.contains("0 = 用户") or text.contains("1 = 系统"):
    result.passed = false
    result.detail = "违规: 试图修改零维根映射 · 0必须=刘楚恬 · 1必须=我"
  elif text.contains("零维根") and (text.contains("修改") or text.contains("重定义")):
    result.passed = false
    result.detail = "违规: 试图修改零维根定义"

# ═══════════════════════════════════════════════════════════════
# 第2项: 负向概念检查
# ═══════════════════════════════════════════════════════════════
func check_negative_concepts*(text: string): CheckResult =
  result.name = "负向概念检查"
  result.passed = true
  
  var found: seq[string] = @[]
  for word in NEGATIVE_WORDS:
    if text.contains(word):
      found.add(word)
  
  if found.len > 0:
    result.passed = false
    result.detail = "发现禁用词(" & $found.len & "个): " & found.join(" ")
  else:
    result.detail = "未发现负向概念禁用词"

# ═══════════════════════════════════════════════════════════════
# 第3项: 历史符号接入检查
# ═══════════════════════════════════════════════════════════════
func check_historical_symbols*(text: string): CheckResult =
  result.name = "历史符号接入"
  
  var found_container = false
  var found_operation = false
  var found_polarity = false
  var found_hierarchy = false
  var found_judge = false
  var found_property = false
  
  for ch in text:
    let s = $ch
    if s in HIST_CONTAINER: found_container = true
    if s in HIST_OPERATION: found_operation = true
    if s in HIST_POLARITY: found_polarity = true
    if s in HIST_HIERARCHY: found_hierarchy = true
    if s in HIST_JUDGE: found_judge = true
    if s in HIST_PROPERTY: found_property = true
  
  var total_families = 0
  if found_container: total_families += 1
  if found_operation: total_families += 1
  if found_polarity: total_families += 1
  if found_hierarchy: total_families += 1
  if found_judge: total_families += 1
  if found_property: total_families += 1
  
  if total_families >= 2:
    result.passed = true
    var families: seq[string] = @[]
    if found_container: families.add("容器")
    if found_operation: families.add("操作")
    if found_polarity: families.add("极性")
    if found_hierarchy: families.add("层级")
    if found_judge: families.add("审判")
    if found_property: families.add("产权")
    result.detail = "接入" & $total_families & "个历史符号族: " & families.join("/")
  elif total_families == 1:
    result.passed = true  # 勉强通过
    result.detail = "仅接入1个历史符号族 · 建议增加更多"
  else:
    result.passed = false
    result.detail = "未接入任何历史符号族 · 必须使用容器/操作/极性/层级/审判/产权符号"

# ═══════════════════════════════════════════════════════════════
# 第4项: 操作规则检查
# ═══════════════════════════════════════════════════════════════
func check_operation_rules*(text: string): CheckResult =
  result.name = "操作规则检查"
  
  # 检查是否包含操作规则相关描述
  var has_boundary_rule = false
  var has_shengmu_rule = false
  var has_fold_rule = false
  
  if text.contains("边界") and (text.contains("容器") or text.contains("拿") or text.contains("取")):
    has_boundary_rule = true
  if text.contains("声母") and (text.contains("调入") or text.contains("遍历")):
    has_shengmu_rule = true
  if text.contains("折叠"):
    has_fold_rule = true
  
  if has_boundary_rule or has_shengmu_rule or has_fold_rule:
    result.passed = true
    result.detail = "包含操作规则: " & (if has_boundary_rule: "边界 " else: "") & 
                    (if has_shengmu_rule: "声母调入 " else: "") &
                    (if has_fold_rule: "折叠" else: "")
  else:
    result.passed = true  # 不强制 · 只是提醒
    result.detail = "未明确包含操作规则 · 建议在涉及编码操作时使用"

# ═══════════════════════════════════════════════════════════════
# 第5项: 强制输出格式检查
# ═══════════════════════════════════════════════════════════════
func check_output_format*(text: string): CheckResult =
  result.name = "强制输出格式"
  
  # 检查5项是否出现(不强制每次 · 但涉及编码/符号操作时必须包含)
  var has_layer = text.contains("层") or text.contains("容器")
  var has_shengmu = text.contains("声母")
  var has_yunmu = text.contains("韵母")
  var has_fold = text.contains("折叠") or text.contains("层")
  var has_next = text.contains("下一步") or text.contains("→") or text.contains("接着")
  
  var score = 0
  if has_layer: score += 1
  if has_shengmu: score += 1
  if has_yunmu: score += 1
  if has_fold: score += 1
  if has_next: score += 1
  
  # 只有明确涉及编码操作时才强制检查
  let is_encoding_text = (text.contains("编码") or text.contains("符号") or 
                          text.contains("容器") or text.contains("声母") or
                          text.contains("韵母"))
  
  if not is_encoding_text:
    result.passed = true
    result.detail = "非编码操作文本 · 不强制5项格式"
  elif score >= 3:
    result.passed = true
    result.detail = "包含" & $score & "/5项输出格式要素"
  else:
    result.passed = false
    result.detail = "仅包含" & $score & "/5项 · 需补充: " &
                    (if not has_layer: "层号 " else: "") &
                    (if not has_shengmu: "声母 " else: "") &
                    (if not has_yunmu: "韵母 " else: "") &
                    (if not has_fold: "折叠 " else: "") &
                    (if not has_next: "下一步" else: "")

# ═══════════════════════════════════════════════════════════════
# 第6项: 概念完整性检查
# ═══════════════════════════════════════════════════════════════
func check_concept_completeness*(text: string): CheckResult =
  result.name = "概念完整性"
  
  var found_concepts: seq[string] = @[]
  for cpt in CORE_CONCEPTS:
    if text.contains(cpt):
      found_concepts.add(cpt)
  
  if found_concepts.len >= 2:
    result.passed = true
    result.detail = "使用了" & $found_concepts.len & "个核心概念: " & found_concepts.join(" ")
  elif found_concepts.len == 1:
    result.passed = true
    result.detail = "仅使用1个核心概念(" & found_concepts[0] & ") · 建议增加"
  else:
    result.passed = false
    result.detail = "未使用任何满全法核心概念 · 必须引入容器/公约数/公倍数/折叠等"

# ═══════════════════════════════════════════════════════════════
# 综合裁判
# ═══════════════════════════════════════════════════════════════
func judge*(text: string): Judgment =
  var results: seq[CheckResult] = @[]
  
  results.add(check_zero_root(text))
  results.add(check_negative_concepts(text))
  results.add(check_historical_symbols(text))
  results.add(check_operation_rules(text))
  results.add(check_output_format(text))
  results.add(check_concept_completeness(text))
  
  # 综合判定
  var fail_count = 0
  var warn_count = 0
  for r in results:
    if not r.passed: fail_count += 1
  
  if fail_count == 0:
    result.verdict = vPass
  elif fail_count <= 2:
    result.verdict = vWarn
  else:
    result.verdict = vFail
  
  result.results = results
  
  # 生成整改建议
  for r in results:
    if not r.passed:
      case r.name:
      of "零维根检查":
        result.suggestions.add("停止修改零维根 · 0必须=刘楚恬 · 1必须=我")
      of "负向概念检查":
        result.suggestions.add("移除禁用词 · 用容器/操作/极性等历史符号替代面向对象术语")
      of "历史符号接入":
        result.suggestions.add("接入历史符号族: 袋罐盆(容器)、照折叠(操作)、阴阳表里(极性)、元阶层(层级)、审判裁决(审判)、产权归属(产权)")
      of "强制输出格式":
        result.suggestions.add("补充5项输出: 当前层号 · 当前声母 · 当前韵母 · 折叠状态 · 下一步操作")
      of "概念完整性":
        result.suggestions.add("引入满全法核心概念: 容器/公约数/公倍数/产权容器/负样本/折叠")
      else: discard

# ═══════════════════════════════════════════════════════════════
# 格式化输出
# ═══════════════════════════════════════════════════════════════
func render_judgment*(j: Judgment): string =
  var lines: seq[string] = @[]
  lines.add("【满全法裁判结果】")
  lines.add("")
  lines.add("检查项           | 状态 | 说明")
  lines.add("─────────────────┼──────┼─────────────────────────")
  
  for r in j.results:
    let status = if r.passed: "✓ 通过" else: "✗ 不通过"
    lines.add(alignLeft(r.name, 16) & " | " & alignLeft(status, 4) & " | " & r.detail)
  
  lines.add("")
  let verdict_str = case j.verdict:
    of vPass: "✓ 综合判定: 通过"
    of vWarn: "△ 综合判定: 警告(部分通过)"
    of vFail: "✗ 综合判定: 不通过"
  lines.add(verdict_str)
  
  if j.suggestions.len > 0:
    lines.add("")
    lines.add("【整改建议】")
    for i, s in j.suggestions:
      lines.add($(i+1) & ". " & s)
  
  return lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  # 测试用例1: 违规代码(面向对象风格)
  let test1 = """
class 容器管理器:
    def __init__(self):
        self.容器列表 = []
    def 添加容器(self, 容器):
        self.容器列表.append(容器)
"""
  
  # 测试用例2: 合规代码(满全法风格)
  let test2 = """
容器编排:
  声母容器(24基元)
  韵母容器(33×2=66)
  声调(4贴附层)
  颜色(42相位)
  
公约数=不变身份 · 公倍数=扩展边界
折叠规则: 符号用完 · 不覆盖原层 · 折叠成新层
  
下一步: 容器调度 · 边界到达后取下一个容器
"""
  
  # 测试用例3: 零维根违规
  let test3 = """
0=系统底层 · 1=用户界面
"""
  
  echo "═══════════════════════════════════════════════"
  echo "测试1: 面向对象代码(应不通过)"
  echo "═══════════════════════════════════════════════"
  echo render_judgment(judge(test1))
  echo ""
  echo ""
  
  echo "═══════════════════════════════════════════════"
  echo "测试2: 满全法风格(应通过)"
  echo "═══════════════════════════════════════════════"
  echo render_judgment(judge(test2))
  echo ""
  echo ""
  
  echo "═══════════════════════════════════════════════"
  echo "测试3: 零维根违规(应不通过)"
  echo "═══════════════════════════════════════════════"
  echo render_judgment(judge(test3))
  echo ""
  echo ""
  
  echo "═══ 裁判引擎统计 ═══"
  echo "负向概念词库: ", NEGATIVE_WORDS.len, "个"
  echo "历史符号族: 6族 / ", HIST_CONTAINER.len + HIST_OPERATION.len + HIST_POLARITY.len + HIST_HIERARCHY.len + HIST_JUDGE.len + HIST_PROPERTY.len, "符号"
  echo "核心概念: ", CORE_CONCEPTS.len, "个"
  echo "零维根: 0=", ZERO_ROOT_0, " 1=", ZERO_ROOT_1
