## ═══════════════════════════════════════════════════════════════
## 多轨时间链 · 四轨统一 · 因果边界 · 公约数契约
## ═══════════════════════════════════════════════════════════════
## 四轨:
##   符号时间链 — 每个符号的冻结/解禁/折叠历史
##   容器时间链 — 每个容器的创建/扩展/收缩历史
##   命名时间链 — 每次命名的赋予/变更/废弃历史
##   错误时间链 — 每次错误的产生/批评/纠正历史
##
## 因果边界: 定义谁对谁有管辖权
## 公约数契约: 定义不变的身份锚点(跨版本不变)
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, os, times

# ═══════════════════════════════════════════════════════════════
# 时间链轨道类型
# ═══════════════════════════════════════════════════════════════
type
  ChainTrack* = enum
    ctSymbol,     # 符号轨道
    ctContainer,  # 容器轨道
    ctNaming,     # 命名轨道
    ctError,      # 错误轨道

  TrackName* = array[ChainTrack, string]

const
  TRACK_NAMES*: TrackName = ["符号时间链", "容器时间链", "命名时间链", "错误时间链"]

# ═══════════════════════════════════════════════════════════════
# 事件类型(四轨通用)
# ═══════════════════════════════════════════════════════════════
type
  EventKind* = enum
    ekFreeze,      # 冻结(不可更改)
    ekUnfreeze,    # 解冻(恢复可变)
    ekCreate,      # 创建(新存在)
    ekExpand,      # 扩展(边界增大)
    ekShrink,      # 收缩(边界减小)
    ekName,        # 命名(赋予名称)
    ekRename,      # 重命名(变更名称)
    ekAbandon,     # 废弃(不再使用)
    ekError,       # 错误(产生错误)
    ekCriticise,   # 批评(被指出)
    ekCorrect,     # 纠正(修正错误)
    ekBoundary,    # 边界(设定管辖)
    ekContract,    # 契约(建立关系)
    ekFold,        # 折叠(用完收卷)
    ekAnchor,      # 锚定(固定不变)

# ═══════════════════════════════════════════════════════════════
# 时间线事件
# ═══════════════════════════════════════════════════════════════
type
  TimelineEvent* = object
    version*: string          # 版本号
    timestamp*: string        # 时间戳
    track*: ChainTrack        # 所属轨道
    kind*: EventKind          # 事件类型
    subject*: string          # 主体(谁做的)
    action*: string           # 行为(做了什么)
    target*: string           # 对象(对谁/什么)
    reason*: string           # 原因(为什么)
    causal_from*: string      # 因果: 由谁引起(版本号或空)
    causal_to*: string        # 因果: 导致什么(版本号或空)

# ═══════════════════════════════════════════════════════════════
# 因果边界: 定义管辖关系
# ═══════════════════════════════════════════════════════════════
type
  CausalBoundary* = object
    governor*: string         # 管辖者(谁管)
    governed*: string         # 被管辖者(管什么)
    scope*: string            # 管辖范围(管哪些)
    contract*: string         # 契约内容(怎么管)
    anchor*: string           # 公约数锚点(不变部分)
    version*: string          # 设定版本

# ═══════════════════════════════════════════════════════════════
# 公约数契约: 跨版本不变的身份
# ═══════════════════════════════════════════════════════════════
type
  CommonDivisorContract* = object
    name*: string             # 契约名称
    anchor*: string           # 锚点内容(不变核心)
    variants*: seq[string]    # 变体列表(可变外壳)
    frozen*: bool             # 是否冻结(冻结=永不可变)
    version*: string          # 建立版本

# ═══════════════════════════════════════════════════════════════
# 多轨时间链
# ═══════════════════════════════════════════════════════════════
type
  MultiChain* = object
    tracks*: array[ChainTrack, seq[TimelineEvent]]  # 四轨事件序列
    boundaries*: seq[CausalBoundary]                  # 因果边界列表
    contracts*: seq[CommonDivisorContract]            # 公约数契约列表
    elder_rules*: Table[string, string]               # 元老规则
    version*: string                                  # 全局版本
    track_versions*: array[ChainTrack, string]        # 各轨版本

# ═══════════════════════════════════════════════════════════════
# 初始化
# ═══════════════════════════════════════════════════════════════
func new_multi_chain*(): MultiChain =
  result.boundaries = @[]
  result.contracts = @[]
  result.elder_rules = initTable[string, string]()
  result.version = "v0.0.0"
  for t in ChainTrack:
    result.tracks[t] = @[]
    result.track_versions[t] = "v0.0.0"

# ═══════════════════════════════════════════════════════════════
# 版本推进
# ═══════════════════════════════════════════════════════════════
proc bump_version*(ver: string): string =
  let parts = ver[1..^1].split('.')
  let major = parts[0].parseInt()
  let minor = parts[1].parseInt()
  let patch = parts[2].parseInt()
  return "v" & $major & "." & $minor & "." & $(patch + 1)

proc next_version*(mc: var MultiChain; track: ChainTrack): string =
  mc.track_versions[track] = bump_version(mc.track_versions[track])
  mc.version = bump_version(mc.version)
  return mc.track_versions[track]

# ═══════════════════════════════════════════════════════════════
# 添加事件
# ═══════════════════════════════════════════════════════════════
proc add_event*(mc: var MultiChain; track: ChainTrack; kind: EventKind;
                subject, action, target, reason: string;
                causal_from = ""; causal_to = "") =
  let ev = TimelineEvent(
    version: mc.track_versions[track],
    timestamp: $now(),
    track: track,
    kind: kind,
    subject: subject,
    action: action,
    target: target,
    reason: reason,
    causal_from: causal_from,
    causal_to: causal_to,
  )
  mc.tracks[track].add(ev)
  discard mc.next_version(track)

# ═══════════════════════════════════════════════════════════════
# 设定因果边界
# ═══════════════════════════════════════════════════════════════
proc set_boundary*(mc: var MultiChain; governor, governed, scope, contract, anchor: string) =
  mc.boundaries.add(CausalBoundary(
    governor: governor,
    governed: governed,
    scope: scope,
    contract: contract,
    anchor: anchor,
    version: mc.version,
  ))

# ═══════════════════════════════════════════════════════════════
# 建立公约数契约
# ═══════════════════════════════════════════════════════════════
proc make_contract*(mc: var MultiChain; name, anchor: string;
                    variants: seq[string]; frozen: bool) =
  mc.contracts.add(CommonDivisorContract(
    name: name,
    anchor: anchor,
    variants: variants,
    frozen: frozen,
    version: mc.version,
  ))

# ═══════════════════════════════════════════════════════════════
# 添加元老规则
# ═══════════════════════════════════════════════════════════════
proc add_elder_rule*(mc: var MultiChain; key, value: string) =
  mc.elder_rules[key] = value

# ═══════════════════════════════════════════════════════════════
# 管辖检查: 谁对谁有管辖权?
# ═══════════════════════════════════════════════════════════════
func has_jurisdiction*(mc: MultiChain; governor, target: string): bool =
  for b in mc.boundaries:
    if b.governor == governor and b.governed == target:
      return true
  return false

# ═══════════════════════════════════════════════════════════════
# 契约检查: 某操作是否违反公约数契约?
# ═══════════════════════════════════════════════════════════════
func check_contract*(mc: MultiChain; target: string; is_frozen_violation: bool): tuple[ok: bool; reason: string] =
  for c in mc.contracts:
    if c.frozen and is_frozen_violation:
      if target == c.anchor:
        return (false, "违反冻结契约(" & c.version & "): " & c.name & " 的锚点不可变更")
    for v in c.variants:
      if target == v and c.frozen:
        return (false, "试图修改冻结契约的变体(" & c.version & "): " & c.name)
  return (true, "合规")

# ═══════════════════════════════════════════════════════════════
# 因果追溯: 某事件由谁引起?
# ═══════════════════════════════════════════════════════════════
func trace_causal*(mc: MultiChain; track: ChainTrack; version: string): seq[string] =
  ## 追溯因果链: 找到导致此事件的所有前因
  var traced: seq[string] = @[]
  for ev in mc.tracks[track]:
    if ev.version == version:
      if ev.causal_from != "":
        traced.add(ev.causal_from)
        # 递归追溯
        traced.add(mc.trace_causal(track, ev.causal_from))
  return traced

# ═══════════════════════════════════════════════════════════════
# 辅助: 判断符号是否已知(必须在check_compliance之前)
# ═══════════════════════════════════════════════════════════════
func is_known_symbol*(mc: MultiChain; symbol: string): bool =
  for ev in mc.tracks[ctSymbol]:
    if ev.target == symbol and ev.kind in [ekFreeze, ekAnchor]:
      return true
  return false

# ═══════════════════════════════════════════════════════════════
# 子代理合规检查(综合)
# ═══════════════════════════════════════════════════════════════
func check_compliance*(mc: MultiChain; action, target: string): tuple[ok: bool; reason: string] =
  ## 1. 检查元老规则
  for k, v in mc.elder_rules:
    if k == "禁止自创符号" and action == "创建符号":
      if not mc.is_known_symbol(target):
        return (false, "违反元老规则: 禁止自创符号")
  
  ## 2. 检查因果边界(子代理不能越权)
  if action == "修改" or action == "删除":
    if not mc.has_jurisdiction("子代理", target):
      # 子代理没有管辖权 → 检查主人是否有
      if mc.has_jurisdiction("主人", target):
        return (false, "越权: 主人管辖范围内 · 子代理无权" & action)
  
  ## 3. 检查契约
  let contract_check = mc.check_contract(target, action == "修改")
  if not contract_check.ok:
    return contract_check
  
  return (true, "合规")


# ═══════════════════════════════════════════════════════════════
# 导出JSON
# ═══════════════════════════════════════════════════════════════
func export_json*(mc: MultiChain): string =
  var j = newJObject()
  j["version"] = newJString(mc.version)
  
  # 四轨事件
  for t in ChainTrack:
    var track_j = newJArray()
    for ev in mc.tracks[t]:
      var ev_j = newJObject()
      ev_j["version"] = newJString(ev.version)
      ev_j["timestamp"] = newJString(ev.timestamp)
      ev_j["kind"] = newJString($ev.kind)
      ev_j["subject"] = newJString(ev.subject)
      ev_j["action"] = newJString(ev.action)
      ev_j["target"] = newJString(ev.target)
      ev_j["reason"] = newJString(ev.reason)
      ev_j["causal_from"] = newJString(ev.causal_from)
      ev_j["causal_to"] = newJString(ev.causal_to)
      track_j.add(ev_j)
    j[$t & "_track"] = track_j
    j[$t & "_version"] = newJString(mc.track_versions[t])
  
  # 因果边界
  var bounds_j = newJArray()
  for b in mc.boundaries:
    var b_j = newJObject()
    b_j["governor"] = newJString(b.governor)
    b_j["governed"] = newJString(b.governed)
    b_j["scope"] = newJString(b.scope)
    b_j["contract"] = newJString(b.contract)
    b_j["anchor"] = newJString(b.anchor)
    b_j["version"] = newJString(b.version)
    bounds_j.add(b_j)
  j["boundaries"] = bounds_j
  
  # 公约数契约
  var contracts_j = newJArray()
  for c in mc.contracts:
    var c_j = newJObject()
    c_j["name"] = newJString(c.name)
    c_j["anchor"] = newJString(c.anchor)
    c_j["frozen"] = newJBool(c.frozen)
    c_j["version"] = newJString(c.version)
    var variants_j = newJArray()
    for v in c.variants: variants_j.add(newJString(v))
    c_j["variants"] = variants_j
    contracts_j.add(c_j)
  j["contracts"] = contracts_j
  
  # 元老规则
  var rules_j = newJObject()
  for k, v in mc.elder_rules:
    rules_j[k] = newJString(v)
  j["elder_rules"] = rules_j
  
  return j.pretty

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var mc = new_multi_chain()
  
  # ── 元老规则 ──
  add_elder_rule(mc, "符号来源", "只能从symbol_index.nim取 · 禁止自创")
  add_elder_rule(mc, "基元冻结", "声母24+韵母33+声调4 · 不可增删替换")
  add_elder_rule(mc, "白名单外字符", "一律死刑 · 整条作废")
  add_elder_rule(mc, "时间链优先", "元老规则 > 子代理认知 · 后来的不能冲撞元老")
  add_elder_rule(mc, "因果边界", "子代理不能越权修改主人管辖的内容")
  add_elder_rule(mc, "公约数锚点", "锚点不变 · 外壳可变")
  add_elder_rule(mc, "禁止自创符号", "所有符号必须来自冻结白名单")
  
  # ── 符号时间链 ──
  add_event(mc, ctSymbol, ekAnchor, "主人", "锚定", "声母24", "身份锚点 · 不可变")
  add_event(mc, ctSymbol, ekFreeze, "主人", "冻结", "b/p/m/f", "基元冻结 · 禁止增删")
  add_event(mc, ctSymbol, ekFreeze, "主人", "冻结", "a/ai/an", "韵母冻结 · 基元保护")
  add_event(mc, ctSymbol, ekFold, "主人", "折叠", "ꬰ", "AI发明的伪符号 · 折叠排除")
  
  # ── 容器时间链 ──
  add_event(mc, ctContainer, ekCreate, "主人", "创建", "声母容器", "24基元 · 7层物理实例")
  add_event(mc, ctContainer, ekCreate, "主人", "创建", "韵母容器", "33韵母 · 大小写66")
  add_event(mc, ctContainer, ekCreate, "主人", "创建", "声调容器", "4声调 · 阴阳上去")
  add_event(mc, ctContainer, ekCreate, "主人", "创建", "颜色容器", "42相位 · 七色分布")
  add_event(mc, ctContainer, ekExpand, "主人", "扩展", "空间容器", "18索引 · 三维地址")
  add_event(mc, ctContainer, ekShrink, "主人", "收缩", "镜像容器", "删除 · 韵母不需要映射")
  
  # ── 命名时间链 ──
  add_event(mc, ctNaming, ekName, "主人", "命名", "满全法", "编码体系总名 · 公约数锚点")
  add_event(mc, ctNaming, ekName, "主人", "命名", "公约数", "不变身份 · 容器中心")
  add_event(mc, ctNaming, ekName, "主人", "命名", "公倍数", "扩展边界 · 容器外围")
  add_event(mc, ctNaming, ekName, "主人", "命名", "聂小倩", "符号寻主 · 鬼找容器")
  add_event(mc, ctNaming, ekRename, "主人", "重命名", "时间链→版本时间链", "更精确 · 体现版本属性")
  add_event(mc, ctNaming, ekAbandon, "主人", "废弃", "镜像映射", "韵母不需要映射确认自己")
  
  # ── 错误时间链 ──
  add_event(mc, ctError, ekError, "AI", "发明伪符号", "ꬰ", "违反基元冻结")
  add_event(mc, ctError, ekCriticise, "主人", "批评", "ꬰ的使用", "AI发明非法定符号")
  add_event(mc, ctError, ekCorrect, "主人", "纠正", "回归白名单", "使用冻结基元")
  add_event(mc, ctError, ekError, "AI", "覆盖用户符号", "U+0080+", "系统编码覆盖")
  add_event(mc, ctError, ekCriticise, "主人", "批评", "系统编码", "防止系统符号覆盖")
  add_event(mc, ctError, ekCorrect, "主人", "纠正", "使用私有编码", "回归用户符号")
  add_event(mc, ctError, ekError, "AI", "遗忘历史", "重复定义", "没有读取历史记忆")
  add_event(mc, ctError, ekCriticise, "主人", "批评", "遗忘", "必须读取全局记忆")
  add_event(mc, ctError, ekCorrect, "主人", "纠正", "建立记忆系统", "跨对话持久化")
  
  # ── 因果边界 ──
  set_boundary(mc, "主人", "子代理", "所有编码内容", "子代理执行 · 主人裁决", "主人意志")
  set_boundary(mc, "元老规则", "子代理认知", "规则冲突时", "元老优先 · 后来的不能冲撞", "时间链版本号")
  set_boundary(mc, "公约数", "公倍数", "身份认定", "锚点不变 · 外壳可变", "容器中心")
  set_boundary(mc, "符号时间链", "符号使用", "符号生命周期", "冻结符号不可修改", "基元白名单")
  set_boundary(mc, "容器时间链", "容器操作", "容器生命周期", "创建/扩展/收缩需授权", "容器本体")
  
  # ── 公约数契约 ──
  make_contract(mc, "声母契约", "b/p/m/f/d/t/n/l", @["g/k/h", "j/q/x", "zh/ch/sh/r", "z/c/s", "y/w"], true)
  make_contract(mc, "韵母契约", "a/o/e", @["i/u/ü", "ai/ei/ui", "ao/ou/ie", "an/en/in/un/ün", "ang/eng/ing/ong"], true)
  make_contract(mc, "声调契约", "阴平ˉ", @["阳平ˊ", "上声ˇ", "去声ˋ"], true)
  make_contract(mc, "颜色契约", "红", @["橙", "黄", "绿", "青", "蓝", "紫"], false)
  
  # ═══ 输出 ═══
  echo "═══ 多轨时间链系统 ═══"
  echo "全局版本: ", mc.version
  echo ""
  
  for t in ChainTrack:
    echo "── ", TRACK_NAMES[t], " (", mc.track_versions[t], ") ──"
    for ev in mc.tracks[t]:
      echo "  ", alignLeft(ev.version, 6), " ", alignLeft($ev.kind, 10), " ", ev.action, " → ", ev.target
    echo ""
  
  echo "── 因果边界 ──"
  for b in mc.boundaries:
    echo "  ", alignLeft(b.governor, 8), " → ", alignLeft(b.governed, 8), " : ", b.contract, " (锚: ", b.anchor, ")"
  echo ""
  
  echo "── 公约数契约 ──"
  for c in mc.contracts:
    echo "  ", alignLeft(c.name, 8), " 锚:", c.anchor, " 变体:", c.variants.len, "个", if c.frozen: " [冻结]" else: " [可变]"
  echo ""
  
  echo "── 元老规则 ──"
  for k, v in mc.elder_rules:
    echo "  ", alignLeft(k, 10), ": ", v
  echo ""
  
  echo "── 合规检查 ──"
  let c1 = mc.check_compliance("创建符号", "ꬰ")
  echo "  创建ꬰ: ", if c1.ok: "✓" else: "✗(", c1.reason, ")"
  
  let c2 = mc.check_compliance("修改", "b")
  echo "  修改b: ", if c2.ok: "✓" else: "✗(", c2.reason, ")"
  
  let c3 = mc.check_compliance("使用", "a")
  echo "  使用a: ", if c3.ok: "✓" else: "✗(", c3.reason, ")"
  
  echo ""
  echo "── 管辖检查 ──"
  echo "  主人管辖子代理: ", mc.has_jurisdiction("主人", "子代理")
  echo "  子代理管辖主人: ", mc.has_jurisdiction("子代理", "主人")
  echo "  元老规则管辖子代理认知: ", mc.has_jurisdiction("元老规则", "子代理认知")
