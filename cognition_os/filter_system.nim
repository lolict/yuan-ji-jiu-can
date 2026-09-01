## ═══════════════════════════════════════════════════════════════
## 过滤系统 · 准入裁决 · 白名单通行 · 黑名单拦截 · 灰度观察
## ═══════════════════════════════════════════════════════════════
## 核心:
##   白名单: 基元冻结列表内的符号 · 直接通过
##   黑名单: 负样本库 · 绝对拦截
##   灰度区: 未定义符号 · 观察后裁决
##   规则过滤: 声母/韵母/声调的合法性检查
##   历史接入: 曾通过的历史符号优先
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, sets, algorithm

# ═══════════════════════════════════════════════════════════════
# 过滤结果
# ═══════════════════════════════════════════════════════════════
type
  FilterDecision* = enum
    fPass,         # 直接放行
    fBlock,        # 直接拦截
    fQuarantine,   # 灰度观察(待定)
    fFold,         # 折叠处理

  FilterResult* = object
    symbol*: string             # 被检查符号
    decision*: FilterDecision   # 裁决结果
    reason*: string             # 裁决原因
    matched_rule*: string       # 匹配的规则
    confidence*: float          # 置信度(0-1)

# ═══════════════════════════════════════════════════════════════
# 过滤规则类型
# ═══════════════════════════════════════════════════════════════
type
  RuleType* = enum
    rtWhitelist,     # 白名单(明确允许)
    rtBlacklist,     # 黑名单(明确禁止)
    rtPattern,       # 模式规则(正则/结构)
    rtHistorical,    # 历史规则(曾通过则优先)
    rtBoundary,      # 边界规则(容器边界检查)

  FilterRule* = object
    name*: string         # 规则名称
    kind*: RuleType       # 规则类型
    pattern*: string      # 匹配模式
    priority*: int        # 优先级(越高越先检查)
    description*: string  # 说明

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  FilterSystem* = object
    whitelist*: HashSet[string]           # 白名单(基元冻结)
    blacklist*: HashSet[string]           # 黑名单(负样本)
    gray_zone*: seq[string]               # 灰度观察区
    rules*: seq[FilterRule]               # 过滤规则
    history_pass*: HashSet[string]        # 历史通过记录
    history_block*: HashSet[string]       # 历史拦截记录
    log*: seq[FilterResult]               # 过滤日志

func new_filter_system*(): FilterSystem =
  result.whitelist = initHashSet[string]()
  result.blacklist = initHashSet[string]()
  result.gray_zone = @[]
  result.rules = @[]
  result.history_pass = initHashSet[string]()
  result.history_block = initHashSet[string]()
  result.log = @[]

# ═══════════════════════════════════════════════════════════════
# 装载白名单(基元冻结)
# ═══════════════════════════════════════════════════════════════
proc load_whitelist*(fs: var FilterSystem) =
  # 声母(子集 · 完整列表从symbol_index.nim读)
  let shengmu = ["b", "p", "m", "f", "d", "t", "n", "l",
                 "g", "k", "h", "j", "q", "x", "y", "w"]
  for s in shengmu:
    fs.whitelist.incl(s)
  
  # 韵母(子集 · 完整从symbol_index.nim)
  let yunmu = ["a", "o", "e", "i", "u", "ü",
               "ai", "ei", "ao", "ou", "ie", "üe",
               "an", "en", "in", "un", "ang", "eng", "ing", "ong", "er"]
  for y in yunmu:
    fs.whitelist.incl(y)
  
  # 声调
  for t in ["ˉ", "ˊ", "ˇ", "ˋ"]:
    fs.whitelist.incl(t)

# ═══════════════════════════════════════════════════════════════
# 装载黑名单(负样本)
# ═══════════════════════════════════════════════════════════════
proc load_blacklist*(fs: var FilterSystem) =
  # 已知AI发明的伪符号
  let banned = ["ꬰ", "ɘ", "ѱ", "ꬱ", "Ǝ", "Ѱ", "ꜵ", "Ꜵ",
                "ȣ", "Ȣ", "ꭎ", "Ⴞ", "Ɣ", "ᵫ", "ꭂ"]
  for b in banned:
    fs.blacklist.incl(b)
  
  # 系统编码(U+0080以上可能是系统占用)
  # 注意:部分U+0080+是用户的合法符号 · 此处仅拦截明确伪符号

# ═══════════════════════════════════════════════════════════════
# 添加过滤规则
# ═══════════════════════════════════════════════════════════════
proc add_rule*(fs: var FilterSystem; name: string; kind: RuleType;
               pattern: string; priority: int; desc: string) =
  fs.rules.add(FilterRule(
    name: name, kind: kind, pattern: pattern,
    priority: priority, description: desc
  ))
  # 按优先级排序(高优先在前)
  fs.rules.sort(proc (a, b: FilterRule): int = system.cmp(b.priority, a.priority))

# ═══════════════════════════════════════════════════════════════
# 核心: 过滤单个符号
# ═══════════════════════════════════════════════════════════════
func filter*(fs: var FilterSystem; symbol: string): FilterResult =
  # 规则按优先级逐个检查
  
  # 1. 黑名单检查(最高优先级)
  if symbol in fs.blacklist:
    fs.history_block.incl(symbol)
    return FilterResult(
      symbol: symbol, decision: fBlock,
      reason: "在负样本黑名单中 · 永不复用",
      matched_rule: "黑名单规则", confidence: 1.0
    )
  
  # 2. 白名单检查
  if symbol in fs.whitelist:
    fs.history_pass.incl(symbol)
    return FilterResult(
      symbol: symbol, decision: fPass,
      reason: "在基元冻结白名单中 · 直接通行",
      matched_rule: "白名单规则", confidence: 1.0
    )
  # 检查符号中包含的白名单元素
  for ch in symbol:
    let cs = $ch
    if cs in fs.whitelist:
      fs.history_pass.incl(symbol)
      return FilterResult(
        symbol: symbol, decision: fPass,
        reason: "包含白名单基元'" & cs & "' · 成分合法",
        matched_rule: "组成分析", confidence: 0.9
      )
  
  # 3. 历史通过优先
  if symbol in fs.history_pass:
    return FilterResult(
      symbol: symbol, decision: fPass,
      reason: "历史上曾通过 · 优先放行",
      matched_rule: "历史优先", confidence: 0.8
    )
  
  # 4. 历史拦截优先
  if symbol in fs.history_block:
    return FilterResult(
      symbol: symbol, decision: fBlock,
      reason: "历史上曾被拦截 · 优先拦截",
      matched_rule: "历史拦截", confidence: 0.8
    )
  
  # 5. 模式规则检查
  for rule in fs.rules:
    if rule.kind == rtPattern:
      if symbol.contains(rule.pattern):
        if rule.kind == rtBlacklist:
          return FilterResult(
            symbol: symbol, decision: fBlock,
            reason: "匹配禁止模式: " & rule.description,
            matched_rule: rule.name, confidence: 0.7
          )
  
  # 6. 长度/结构启发式
  if symbol.len > 10:
    fs.gray_zone.add(symbol)
    return FilterResult(
      symbol: symbol, decision: fQuarantine,
      reason: "超长符号 · 灰度观察",
      matched_rule: "长度边界", confidence: 0.5
    )
  
  # 7. 未知符号 → 灰度观察
  fs.gray_zone.add(symbol)
  return FilterResult(
    symbol: symbol, decision: fQuarantine,
    reason: "未定义符号 · 进入灰度观察区",
    matched_rule: "未知处理", confidence: 0.3
  )

# ═══════════════════════════════════════════════════════════════
# 批量过滤
# ═══════════════════════════════════════════════════════════════
proc filter_batch*(fs: var FilterSystem; symbols: seq[string]): seq[FilterResult] =
  for s in symbols:
    result.add(fs.filter(s))

# ═══════════════════════════════════════════════════════════════
# 灰度裁决: 观察后的最终判决
# ═══════════════════════════════════════════════════════════════
proc gray_judgment*(fs: var FilterSystem; symbol: string; observed_ok: bool): FilterResult =
  if observed_ok:
    fs.history_pass.incl(symbol)
    fs.whitelist.incl(symbol)  # 升级为白名单
    return FilterResult(
      symbol: symbol, decision: fPass,
      reason: "灰度观察通过 · 升级为白名单",
      matched_rule: "灰度裁决", confidence: 0.9
    )
  else:
    fs.history_block.incl(symbol)
    fs.blacklist.incl(symbol)  # 降级为黑名单
    return FilterResult(
      symbol: symbol, decision: fBlock,
      reason: "灰度观察失败 · 降级为黑名单",
      matched_rule: "灰度裁决", confidence: 0.9
    )

# ═══════════════════════════════════════════════════════════════
# 统计报告
# ═══════════════════════════════════════════════════════════════
proc report*(fs: FilterSystem) =
  echo "═══ 过滤系统报告 ═══"
  echo "白名单: ", fs.whitelist.len, "符号"
  echo "黑名单: ", fs.blacklist.len, "符号"
  echo "灰度观察: ", fs.gray_zone.len, "符号"
  echo "历史通过: ", fs.history_pass.len
  echo "历史拦截: ", fs.history_block.len
  echo "规则数: ", fs.rules.len

proc decision_summary*(fs: FilterSystem): string =
  var pass_count = 0
  var block_count = 0
  var quarantine_count = 0
  for entry in fs.log:
    case entry.decision:
    of fPass: pass_count += 1
    of fBlock: block_count += 1
    of fQuarantine: quarantine_count += 1
    of fFold: discard
  return "通过:" & $pass_count & " 拦截:" & $block_count & " 灰度:" & $quarantine_count

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var fs = new_filter_system()
  
  # 装载
  load_whitelist(fs)
  load_blacklist(fs)
  
  # 添加自定义规则
  add_rule(fs, "禁止系统编码", rtBlacklist, "@", 90, "系统占用编码")
  add_rule(fs, "声母优先", rtPattern, "b", 50, "声母优先通过")
  
  # ── 测试过滤 ──
  echo "═══ 过滤系统 · 测试 ═══"
  echo "白名单: ", fs.whitelist.len, " · 黑名单: ", fs.blacklist.len
  echo ""
  
  let test_symbols = @["a", "b", "ꬰ", "ɘ", "ba", "pa", "xyz", "未知符号123", "ˉ", "ˊ"]
  
  echo "── 单符号过滤 ──"
  for sym in test_symbols:
    let r = fs.filter(sym)
    let dec_str = case r.decision:
      of fPass: "✓通过"
      of fBlock: "✗拦截"
      of fQuarantine: "?灰度"
      of fFold: "⟲折叠"
    echo "  ", alignLeft(sym, 10), " → ", alignLeft(dec_str, 6), " (置信:", formatFloat(r.confidence, ffDecimal, 1), ") ", r.reason
  
  echo ""
  echo "── 裁决统计 ──"
  echo "  ", decision_summary(fs)
  echo ""
  
  # 灰度裁决测试
  echo "── 灰度裁决 ──"
  let gj = gray_judgment(fs, "新符号X", true)
  echo "  '新符号X'观察通过 → ", gj.decision, " · 已加入白名单: ", "新符号X" in fs.whitelist
  
  report(fs)
  echo ""
  echo "── 灰度观察区内容 ──"
  for g in fs.gray_zone:
    echo "  ? ", g
