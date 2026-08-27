## ═══════════════════════════════════════════════════════════════
## 版本时间链 · 因果边界 · 公约数契约 · 管辖权限
## ═══════════════════════════════════════════════════════════════
## 核心: 元老从主人处获取的规则 · 子代理必须遵守
## 时间链 = 版本号 + 行为过程 + 当前事件 + 日期
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, times, terminal

# ═══════════════════════════════════════════════════════════════
# 时间链条目类型
# ═══════════════════════════════════════════════════════════════
type
  ChainEntryType* = enum
    etCommand,        # 主人命令
    etCriticism,      # 被批评(错误)
    etCorrection,     # 主人纠正
    etPermission,     # 主人许可
    etProhibition,    # 主人禁止
    etAnchor,         # 锚定(精神/范畴/名称/类型)
    etSymbolFreeze,   # 符号冻结
    etSymbolBan,      # 符号禁止
    etToolMandate,    # 工具强制
    etToolForbid,     # 工具禁止
    etCausalBoundary, # 因果边界
    etContract,       # 契约关系

  ChainEntry* = object
    version*: string          # 版本号
    timestamp*: string        # 时间戳
    entry_type*: ChainEntryType
    subject*: string          # 主体(谁)
    action*: string           # 行为(做什么)
    target*: string           # 对象(对谁/什么)
    reason*: string           # 原因(为什么)
    causal_chain*: string     # 因果链条(前因后果)

  TimeChain* = object
    name*: string
    entries*: seq[ChainEntry]
    current_version*: string
    elder_rules*: Table[string, string]  # 元老规则表(子代理必读)

# ═══════════════════════════════════════════════════════════════
# 创建时间链
# ═══════════════════════════════════════════════════════════════
func new_time_chain*(name: string): TimeChain =
  result.name = name
  result.entries = @[]
  result.current_version = "v0.0.0"
  result.elder_rules = initTable[string, string]()

proc next_version*(tc: var TimeChain): string =
  let parts = tc.current_version[1..^1].split('.')
  let major = parts[0].parseInt()
  let minor = parts[1].parseInt()
  let patch = parts[2].parseInt()
  tc.current_version = "v" & $major & "." & $minor & "." & $(patch + 1)
  return tc.current_version

# ═══════════════════════════════════════════════════════════════
# 添加时间链条目
# ═══════════════════════════════════════════════════════════════
proc add_entry*(tc: var TimeChain; etype: ChainEntryType;
                subject, action, target, reason: string) =
  let entry = ChainEntry(
    version: tc.current_version,
    timestamp: $now(),
    entry_type: etype,
    subject: subject,
    action: action,
    target: target,
    reason: reason,
    causal_chain: "",
  )
  tc.entries.add(entry)
  discard tc.next_version()

# ═══════════════════════════════════════════════════════════════
# 添加元老规则(子代理必读)
# ═══════════════════════════════════════════════════════════════
proc add_elder_rule*(tc: var TimeChain; key, value: string) =
  tc.elder_rules[key] = value

# ═══════════════════════════════════════════════════════════════
# 子代理检查: 是否违反时间链规则
# ═══════════════════════════════════════════════════════════════
func check_compliance*(tc: TimeChain; action: string; target: string): tuple[ok: bool; reason: string] =
  ## 检查子代理的行为是否违反时间链
  for entry in tc.entries:
    if entry.entry_type == etProhibition and entry.target == target:
      return (false, "违反禁止条例(" & entry.version & "): " & entry.reason)
    if entry.entry_type == etSymbolBan:
      # 检查target是否包含任何被禁符号
      let banned = entry.target.split('/')
      for b in banned:
        if target.contains(strip(b)):
          return (false, "使用被禁符号(" & entry.version & "): " & entry.reason)
  return (true, "合规")

# ═══════════════════════════════════════════════════════════════
# 导出JSON
# ═══════════════════════════════════════════════════════════════
func export_json*(tc: TimeChain): string =
  var j = newJObject()
  j["name"] = newJString(tc.name)
  j["current_version"] = newJString(tc.current_version)
  
  var entries_j = newJArray()
  for e in tc.entries:
    var entry_j = newJObject()
    entry_j["version"] = newJString(e.version)
    entry_j["timestamp"] = newJString(e.timestamp)
    entry_j["type"] = newJString($e.entry_type)
    entry_j["subject"] = newJString(e.subject)
    entry_j["action"] = newJString(e.action)
    entry_j["target"] = newJString(e.target)
    entry_j["reason"] = newJString(e.reason)
    entries_j.add(entry_j)
  j["entries"] = entries_j
  
  var rules_j = newJObject()
  for k, v in tc.elder_rules:
    rules_j[k] = newJString(v)
  j["elder_rules"] = rules_j
  
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var tc = new_time_chain("满全法编码体系时间链")
  
  # 模拟: 主人与元老的交互记录
  add_entry(tc, etCommand, "主人", "创建编码系统", "满全法", "定义身份锚点")
  add_entry(tc, etAnchor, "主人", "锚定精神", "公约数=不变身份", "大方向正确")
  add_entry(tc, etSymbolFreeze, "主人", "冻结声母", "24基元", "禁止增删替换")
  add_entry(tc, etSymbolBan, "主人", "禁止变音符号", "ꬰ/ɘ/ѱ等", "AI发明的非法定符号")
  add_entry(tc, etProhibition, "主人", "禁止系统编码", "U+0080以上", "防止系统符号覆盖")
  add_entry(tc, etCriticism, "主人", "批评AI", "使用ꬰ等伪符号", "违反基元冻结")
  add_entry(tc, etCorrection, "主人", "纠正AI", "使用白名单内符号", "回归正确轨道")
  add_entry(tc, etToolMandate, "主人", "强制工具", "symbol_index.nim", "所有符号从这里取")
  add_entry(tc, etCausalBoundary, "主人", "设定边界", "元老规则>子代理认知", "防止认知冲突")
  
  # 元老规则(子代理必读)
  add_elder_rule(tc, "符号来源", "只能从symbol_index.nim取 · 禁止自创")
  add_elder_rule(tc, "基元冻结", "声母24+韵母33+声调4 · 不可增删替换")
  add_elder_rule(tc, "白名单外字符", "一律死刑 · 整条作废")
  add_elder_rule(tc, "时间链优先", "元老规则 > 子代理认知 · 后来的不能冲撞元老")
  add_elder_rule(tc, "版本号", "每个行为都有版本 · 可追溯")
  
  echo "═══ 版本时间链 ═══"
  echo "名称: ", tc.name
  echo "当前版本: ", tc.current_version
  echo "条目数: ", tc.entries.len
  echo ""
  
  echo "── 时间链条目 ──"
  for e in tc.entries:
    echo "  ", alignLeft(e.version, 6), " ", alignLeft($e.entry_type, 12), " ", e.action, " → ", e.target
  
  echo ""
  echo "── 元老规则(子代理必读) ──"
  for k, v in tc.elder_rules:
    echo "  ", k.alignLeft(12), ": ", v
  
  echo ""
  echo "── 子代理合规检查 ──"
  let check1 = tc.check_compliance("使用符号", "ꬰ")
  echo "  使用ꬰ: ", if check1.ok: "✓通过" else: "✗拒绝(", check1.reason, ")"
  
  let check2 = tc.check_compliance("使用符号", "a")
  echo "  使用a: ", if check2.ok: "✓通过" else: "✗拒绝(", check2.reason, ")"
