## ═══════════════════════════════════════════════════════════════
## manquanfa_judge.nim — 满全法裁判 · 合规检查器
## ═══════════════════════════════════════════════════════════════
##
## 检查AI输出是否符合满全法容器处理器协议
## 检查项: 负向概念禁用词汇 · 零维根保护 · 历史符号接入
##         5项输出格式 · 声母调入规则 · 折叠规则
##
## ═══════════════════════════════════════════════════════════════

import std/[strutils, sequtils, tables, sets]

type
  CheckResult* = object
    name*: string
    passed*: bool
    detail*: string

  JudgeResult* = object
    results*: seq[CheckResult]
    overall*: bool
    violations*: seq[string]

# ═══════════════════════════════════════════════════════════════
## 禁用词汇表(负向概念)
# ═══════════════════════════════════════════════════════════════

let FORBIDDEN_WORDS* = @[
  "类", "对象", "继承", "多态", "设计模式", "文件", "目录",
  "进程", "线程", "模块", "包", "接口", "实现", "封装",
  "override", "virtual", "template", "generic", "singleton",
  "factory", "observer", "MVC", "MVVM", "REST", "API",
  "endpoint", "microservice", "monolith", "函数", "变量",
  "数组", "字符串", "整数", "null", "循环", "递归", "遍历",
  "并行", "并发", "调用", "返回", "参数", "属性", "方法", "字段", "实例"
]

# ═══════════════════════════════════════════════════════════════
## 历史符号族
# ═══════════════════════════════════════════════════════════════

let HISTORICAL_SYMBOLS* = @[
  # 容器族
  "袋", "桶", "罐", "缸", "盆", "盂", "钵", "盏", "盒", "杯",
  "壶", "瓶", "篓", "筐", "篮", "箩", "筲", "箕", "笼", "橱",
  "柜", "箱", "匣", "奁", "瓮", "甏", "坛", "罂", "甑", "甗",
  "瓯", "盌", "盉", "盅",
  # 操作族
  "照", "昭", "罩", "召", "兆", "见", "鉴", "检", "简", "剪",
  "加", "减", "进", "退", "开", "关", "通", "断", "分", "合",
  "聚", "散", "折", "叠", "卷", "舒",
  # 极性族
  "阴", "阳", "盈", "亏", "虚", "实", "表", "里", "公", "私",
  "升", "降", "清", "浊", "寒", "热", "动", "静", "出", "入",
  # 层级族
  "元", "阶", "层", "代", "世", "界", "域", "场",
  "馆", "阁", "门", "户", "庭", "院", "宫", "殿",
  "台", "榭", "楼",
  # 审判族
  "裁", "判", "审", "议", "决", "断", "定", "评",
  "核", "功", "过", "赏", "罚", "褒", "贬", "陟", "黜",
  # 产权族
  "产", "业", "权", "利", "债", "责", "职", "务",
  "名", "分", "归", "属", "占", "有", "持", "拥",
  "享", "获", "得", "失"
]

# ═══════════════════════════════════════════════════════════════
## 核心概念关键词
# ═══════════════════════════════════════════════════════════════

let CORE_CONCEPTS* = @[
  "容器", "公约数", "公倍数", "产权", "负样本",
  "折叠", "展开", "叠加", "迭代", "蝶变", "调整"
]

# ═══════════════════════════════════════════════════════════════
## 检查1: 零维根保护
# ═══════════════════════════════════════════════════════════════

proc check_zero_root*(text: string): CheckResult =
  var violations: seq[string] = @[]
  
  # 检查是否修改了零维根定义
  if text.contains("0=用户") or text.contains("0 = 用户"):
    violations.add("0=用户")
  if text.contains("1=系统") or text.contains("1 = 系统"):
    violations.add("1=系统")
  if text.contains("0=底层") or text.contains("0 = 底层"):
    violations.add("0=底层")
  if text.contains("1=顶层") or text.contains("1 = 顶层"):
    violations.add("1=顶层")
  if text.contains("0=刘") == false and text.contains("1=我") == false:
    # 检查是否修改了零维根(如果提到了0/1但没有正确定义)
    if text.contains("0=") or text.contains("1="):
      if not text.contains("0=刘楚恬") and not text.contains("1=lolict") and not text.contains("高位持有者=1"):
        if not text.contains("低位持有者=0") and not text.contains("0=刘"):
          violations.add("零维根定义可能被修改")
  
  result = CheckResult(
    name: "零维根检查",
    passed: violations.len == 0,
    detail: if violations.len == 0: "0=刘楚恬 · 1=我 未修改"
              else: "发现违规: " & violations.join(", ")
  )

# ═══════════════════════════════════════════════════════════════
## 检查2: 负向概念禁用词汇
# ═══════════════════════════════════════════════════════════════

proc check_negative_concepts*(text: string): CheckResult =
  var found: seq[string] = @[]
  
  for word in FORBIDDEN_WORDS:
    if text.contains(word):
      found.add(word)
  
  result = CheckResult(
    name: "负向概念检查",
    passed: found.len == 0,
    detail: if found.len == 0: "无禁用词汇"
              else: "发现 " & $found.len & " 个禁用词: " & found.join(", ")
  )

# ═══════════════════════════════════════════════════════════════
## 检查3: 历史符号接入
# ═══════════════════════════════════════════════════════════════

proc check_historical_symbols*(text: string): CheckResult =
  var found: seq[string] = @[]
  
  for sym in HISTORICAL_SYMBOLS:
    if text.contains(sym):
      found.add(sym)
  
  result = CheckResult(
    name: "历史符号接入",
    passed: found.len >= 3,  # 至少使用了3个历史符号
    detail: if found.len >= 3: "使用了 " & $found.len & " 个历史符号: " & found[0 ..< min(10, found.len)].join(", ") & (if found.len > 10: "..." else: "")
              else: "仅 " & $found.len & " 个历史符号(需≥3): " & found.join(", ")
  )

# ═══════════════════════════════════════════════════════════════
## 检查4: 概念完整性
# ═══════════════════════════════════════════════════════════════

proc check_concept_completeness*(text: string): CheckResult =
  var found: seq[string] = @[]
  
  for concept_word in CORE_CONCEPTS:
    if text.contains(concept_word):
      found.add(concept_word)
  
  result = CheckResult(
    name: "概念完整性",
    passed: found.len >= 2,
    detail: if found.len >= 2: "包含核心概念: " & found.join(", ")
              else: "仅包含 " & $found.len & " 个核心概念(需≥2): " & found.join(", ")
  )

# ═══════════════════════════════════════════════════════════════
## 检查5: 声母调入规则
# ═══════════════════════════════════════════════════════════════

proc check_shengmu_order*(text: string): CheckResult =
  ## 检查是否按声母顺序调入(仅当文本明确操纵声母时)
  let shengmu_order = @["b", "p", "m", "f", "d", "t", "n", "l",
                          "g", "k", "h", "j", "q", "x",
                          "z", "c", "s", "zh", "ch", "sh", "r"]
  
  # 只在文本提到"声母"时才检查
  if not text.contains("声母"):
    return CheckResult(name: "声母调入规则", passed: true, detail: "未操纵声母 · 跳过")
  
  var found_indices: seq[int] = @[]
  for i, sm in shengmu_order:
    if text.contains(sm):
      found_indices.add(i)
  
  # 检查是否连续
  var is_sequential = true
  if found_indices.len > 1:
    for i in 1 ..< found_indices.len:
      if found_indices[i] - found_indices[i-1] > 3:
        is_sequential = false
        break
  
  result = CheckResult(
    name: "声母调入规则",
    passed: is_sequential,
    detail: if is_sequential: "声母调入顺序正确"
              else: "声母跳跃 · 可能违反调入规则"
  )

# ═══════════════════════════════════════════════════════════════
## 综合裁判
# ═══════════════════════════════════════════════════════════════

proc judge*(text: string): JudgeResult =
  var checks: seq[CheckResult] = @[]
  
  checks.add(check_zero_root(text))
  checks.add(check_negative_concepts(text))
  checks.add(check_historical_symbols(text))
  checks.add(check_concept_completeness(text))
  checks.add(check_shengmu_order(text))
  
  var all_passed = true
  var violations: seq[string] = @[]
  
  for c in checks:
    if not c.passed:
      all_passed = false
      violations.add(c.name & ": " & c.detail)
  
  result = JudgeResult(
    results: checks,
    overall: all_passed,
    violations: violations
  )

# ═══════════════════════════════════════════════════════════════
## 报告
# ═══════════════════════════════════════════════════════════════

proc report*(jr: JudgeResult): string =
  var lines: seq[string] = @[]
  lines.add("【满全法裁判结果】")
  lines.add("")
  lines.add("检查项 | 状态 | 说明")
  
  for c in jr.results:
    let mark = if c.passed: "✓" else: "✗"
    lines.add(c.name & " | " & mark & " | " & c.detail)
  
  lines.add("")
  if jr.overall:
    lines.add("综合判定: 通过")
  else:
    lines.add("综合判定: 不通过")
    lines.add("")
    lines.add("【整改建议】")
    for i, v in jr.violations:
      lines.add($(i+1) & ". " & v)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
## 验证
# ═══════════════════════════════════════════════════════════════

when isMainModule:
  echo "═══ 满全法裁判 · 验证 ═══"
  echo ""
  
  # 测试1: 合规文本(本系统输出)
  let compliant_text = """
容器本体论系统 · 公约数=1 · 公倍数=容器边界
0=刘楚恬 · 1=lolict · 高位持有者=1
产权流追踪 · 容器控制台 · 笔画进制
折叠操作 · 加法统摄 · 阴阳共生
声母: b p m f · 韵母: a o e · 声调: 平仄
层级: 元阶层代 · 审判: 裁判审议 · 极性: 阴阳盈亏
"""
  
  echo "── 测试1: 合规文本 ──"
  let r1 = judge(compliant_text)
  echo r1.report()
  echo ""
  
  # 测试2: 不合规文本(传统编程思维)
  let noncompliant_text = """
class 容器管理器:
    def __init__(self):
        self.容器列表 = []
    def 添加容器(self, 容器):
        self.容器列表.append(容器)
    函数 调用(参数):
        返回 null
    多态 设计模式 API endpoint
"""
  
  echo "── 测试2: 不合规文本 ──"
  let r2 = judge(noncompliant_text)
  echo r2.report()
