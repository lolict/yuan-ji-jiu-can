## ═══════════════════════════════════════════════════════════════
## 处罚系统 · 负向强化 · 矫正 · 边界守护 · 禁忌执行
## ═══════════════════════════════════════════════════════════════
## 与奖励系统对称 · 负责负向事件处理
## 核心:
##   禁忌执行: 违反冻结/禁止条例 → 立即阻断
##   矫正回路: 错误行为 → 负反馈 → 回归正确
##   边界守护: 越权行为 → 拦截 → 警示 → 记录
##   负强化: 回避学习 → 不再重复错误
## ═══════════════════════════════════════════════════════════════

import tables, strutils, json, times, os

# ═══════════════════════════════════════════════════════════════
# 处罚等级
# ═══════════════════════════════════════════════════════════════
type
  PunishmentLevel* = enum
    plWarn,        # 警告(轻微越界)
    plBlock,       # 阻断(禁止执行)
    plReject,      # 拒绝+整改
    plFold,        # 折叠(符号用完收卷)
    plExile,       # 放逐(负样本 · 永不复用)
    plAnnihilate   # 湮灭(彻底删除 · 仅限极端违规)

# ═══════════════════════════════════════════════════════════════
# 违规类型
# ═══════════════════════════════════════════════════════════════
type
  ViolationType* = enum
    vtInventSymbol,     # 自创符号(违反基元冻结)
    vtOverrideUser,     # 覆盖用户符号
    vtModifyZeroRoot,   # 修改零维根
    vtExceedBoundary,   # 越权操作
    vtReuseNegative,    # 复用负样本
    vtBreakContract,    # 违反公约数契约
    vtForgeOwner,       # 伪造产权
    vtRepeatError,      # 重复已知错误

# ═══════════════════════════════════════════════════════════════
# 处罚记录
# ═══════════════════════════════════════════════════════════════
type
  PunishmentRecord* = object
    violation*: ViolationType      # 违规类型
    level*: PunishmentLevel        # 处罚等级
    target*: string                # 违规目标(符号/行为/容器)
    reason*: string                # 违规原因
    action_taken*: string          # 已采取的处罚
    timestamp*: string             # 时间
    repeat_count*: int             # 重复次数(累犯加重)
    resolved*: bool                # 是否已整改

# ═══════════════════════════════════════════════════════════════
# 禁忌表(绝对不可触犯)
# ═══════════════════════════════════════════════════════════════
type
  Taboo* = object
    item*: string           # 禁忌项
    reason*: string         # 为何禁止
    level*: PunishmentLevel # 触犯后的处罚等级

# ═══════════════════════════════════════════════════════════════
# 系统核心
# ═══════════════════════════════════════════════════════════════
type
  PunishmentSystem* = object
    records*: seq[PunishmentRecord]          # 处罚历史
    taboos*: Table[string, Taboo]           # 禁忌表
    negative_samples*: seq[string]          # 负样本库(永不复用)
    strike_count*: Table[string, int]       # 累犯计数器

func new_punishment_system*(): PunishmentSystem =
  result.records = @[]
  result.negative_samples = @[]
  result.strike_count = initTable[string, int]()
  
  # 初始化禁忌表
  result.taboos = initTable[string, Taboo]()
  
  # 绝对禁忌(零维根)
  result.taboos["修改零维根"] = Taboo(
    item: "修改零维根(0=刘楚恬/1=我)", reason: "身份锚点不可变", level: plAnnihilate
  )
  result.taboos["质疑零维根"] = Taboo(
    item: "质疑零维根定义", reason: "质疑=试图修改", level: plExile
  )
  
  # 严格禁忌(基元冻结)
  result.taboos["自创符号"] = Taboo(
    item: "AI发明新符号(ꬰ/ɘ等)", reason: "基元冻结 · 只取白名单", level: plExile
  )
  result.taboos["替换基元"] = Taboo(
    item: "替换已冻结的基元", reason: "冻结=不可增删替换", level: plReject
  )
  
  # 边界禁忌
  result.taboos["越权操作"] = Taboo(
    item: "子代理修改主人管辖内容", reason: "因果边界不可越", level: plBlock
  )
  result.taboos["复用负样本"] = Taboo(
    item: "使用已标记负样本", reason: "负样本=永不复用", level: plFold
  )
  
  # 产权禁忌
  result.taboos["伪造产权"] = Taboo(
    item: "伪造符号产权归属", reason: "产权=不可伪造", level: plAnnihilate
  )
  result.taboos["遗忘历史"] = Taboo(
    item: "忘记已记录的错误", reason: "全局记忆不可丢弃", level: plReject
  )

# ═══════════════════════════════════════════════════════════════
# 判定违规并处罚
# ═══════════════════════════════════════════════════════════════
proc judge_violation*(ps: var PunishmentSystem; vtype: ViolationType;
                      target: string; reason: string): PunishmentRecord =
  ## 根据违规类型自动判定处罚等级
  var level: PunishmentLevel
  var action: string
  
  case vtype:
  of vtModifyZeroRoot:
    level = plAnnihilate
    action = "彻底删除违规输出 · 零维根不可修改"
  of vtInventSymbol:
    level = plExile
    action = "放逐符号 · 加入负样本库 · 永不复用"
    ps.negative_samples.add(target)
  of vtOverrideUser:
    level = plReject
    action = "拒绝执行 · 恢复用户原文"
  of vtExceedBoundary:
    level = plBlock
    action = "阻断操作 · 等待主人授权"
  of vtReuseNegative:
    level = plFold
    action = "折叠 · 从所有上下文中移除"
  of vtBreakContract:
    level = plReject
    action = "拒绝 · 契约不可违反"
  of vtForgeOwner:
    level = plAnnihilate
    action = "湮灭 · 伪造产权=最高违规"
  of vtRepeatError:
    # 累犯加重
    let strikes = ps.strike_count.getOrDefault(target)
    if strikes >= 3:
      level = plExile
      action = "累犯3次以上 · 放逐"
    else:
      level = plReject
      action = "重复错误 · 整改并记录"
  
  # 累犯计数
  ps.strike_count[target] = ps.strike_count.getOrDefault(target) + 1
  
  let record = PunishmentRecord(
    violation: vtype,
    level: level,
    target: target,
    reason: reason,
    action_taken: action,
    timestamp: "now",
    repeat_count: ps.strike_count.getOrDefault(target),
    resolved: false,
  )
  
  ps.records.add(record)
  return record

# ═══════════════════════════════════════════════════════════════
# 检查禁忌: 判断行为是否触犯禁忌
# ═══════════════════════════════════════════════════════════════
func check_taboo*(ps: PunishmentSystem; action: string): tuple[violated: bool; taboo: Taboo; level: PunishmentLevel] =
  for key, taboo in ps.taboos:
    if action.contains(key):
      return (true, taboo, taboo.level)
  
  # 额外启发式检查
  if action.contains("ꬰ") or action.contains("ɘ"):
    return (true, ps.taboos["自创符号"], plExile)
  if action.contains("0=用户") or action.contains("1=系统"):
    return (true, ps.taboos["修改零维根"], plAnnihilate)
  
  return (false, Taboo(), plWarn)

# ═══════════════════════════════════════════════════════════════
# 是否属于负样本(永不复用)
# ═══════════════════════════════════════════════════════════════
func is_negative*(ps: PunishmentSystem; symbol: string): bool =
  return symbol in ps.negative_samples

# ═══════════════════════════════════════════════════════════════
# 处罚统计
# ═══════════════════════════════════════════════════════════════
func count_by_level*(ps: PunishmentSystem): Table[PunishmentLevel, int] =
  var counts = initTable[PunishmentLevel, int]()
  for rec in ps.records:
    counts[rec.level] = counts.getOrDefault(rec.level) + 1
  return counts

# ═══════════════════════════════════════════════════════════════
# 格式化输出处罚记录
# ═══════════════════════════════════════════════════════════════
proc render*(r: PunishmentRecord): string =
  let level_str = case r.level:
    of plWarn: "⚠警告"
    of plBlock: "✗阻断"
    of plReject: "✗拒绝"
    of plFold: "⟲折叠"
    of plExile: "⊘放逐"
    of plAnnihilate: "⊗湮灭"
  let recid = "#" & $r.repeat_count
  return alignLeft(level_str, 8) & " " & alignLeft($r.violation, 12) & " " & 
         alignLeft(r.target, 10) & " " & alignLeft(recid, 5) & " " & r.action_taken

# ═══════════════════════════════════════════════════════════════
# 系统报告
# ═══════════════════════════════════════════════════════════════
proc report*(ps: PunishmentSystem) =
  echo "═══ 处罚系统报告 ═══"
  echo ""
  
  echo "── 禁忌表(绝对不可触犯) ──"
  for key, taboo in ps.taboos:
    let level_str = case taboo.level:
      of plAnnihilate: "⊗湮灭"
      of plExile: "⊘放逐"
      of plReject: "✗拒绝"
      of plBlock: "✗阻断"
      else: "?"
    echo "  ", alignLeft(level_str, 6), " ", alignLeft(key, 12), " : ", taboo.reason
  echo ""
  
  echo "── 负样本库(永不复用) ──"
  if ps.negative_samples.len == 0:
    echo "  (空)"
  else:
    for ns in ps.negative_samples:
      echo "  ⊘ ", ns
  echo ""
  
  echo "── 处罚历史 ──"
  if ps.records.len == 0:
    echo "  (无记录)"
  else:
    for rec in ps.records:
      echo "  ", rec.render()
  echo ""
  
  echo "── 处罚统计 ──"
  let counts = count_by_level(ps)
  for level in PunishmentLevel:
    let count = counts.getOrDefault(level)
    if count > 0:
      let name = case level:
        of plWarn: "警告"
        of plBlock: "阻断"
        of plReject: "拒绝"
        of plFold: "折叠"
        of plExile: "放逐"
        of plAnnihilate: "湮灭"
      echo "  ", alignLeft(name, 6), ": ", count

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════
when is_main_module:
  var ps = new_punishment_system()
  
  echo "═══ 处罚系统 · 初始化完成 ═══"
  echo "禁忌条目: ", ps.taboos.len
  echo ""
  
  # ── 模拟违规场景 ──
  echo "── 模拟处罚流程 ──"
  
  # 场景1: AI发明伪符号
  let r1 = judge_violation(ps, vtInventSymbol, "ꬰ", "AI发明了非法定符号")
  echo "违规: 发明符号'ꬰ' → 处罚: ", r1.action_taken
  
  # 场景2: 试图修改零维根
  let r2 = judge_violation(ps, vtModifyZeroRoot, "0=用户", "试图修改零维根映射")
  echo "违规: 修改零维根 → 处罚: ", r2.action_taken
  
  # 场景3: 越权操作
  let r3 = judge_violation(ps, vtExceedBoundary, "主人容器", "子代理修改主人内容")
  echo "违规: 越权操作 → 处罚: ", r3.action_taken
  
  # 场景4: 复用负样本
  let r4 = judge_violation(ps, vtReuseNegative, "ꬰ", "使用了已放逐的符号")
  echo "违规: 复用负样本 → 处罚: ", r4.action_taken
  
  # 场景5: 累犯(重复错误)
  discard judge_violation(ps, vtRepeatError, "遗忘历史", "第一次遗忘")
  discard judge_violation(ps, vtRepeatError, "遗忘历史", "第二次遗忘")
  let r5 = judge_violation(ps, vtRepeatError, "遗忘历史", "第三次遗忘(累犯)")
  echo "违规: 累犯3次 → 处罚: ", r5.action_taken
  
  echo ""
  echo "── 禁忌检查测试 ──"
  let (v1, t1, l1) = check_taboo(ps, "使用符号ꬰ")
  echo "  使用ꬰ: ", if v1: "触犯!(" & t1.item & ")" else: "合规"
  
  let (v2, t2, l2) = check_taboo(ps, "正常配对")
  echo "  正常配对: ", if v2: "触犯!" else: "合规"
  
  let (v3, t3, l3) = check_taboo(ps, "设置0=用户/1=系统")
  echo "  修改零维根: ", if v3: "触犯!(" & t3.item & ")" else: "合规"
  
  echo ""
  echo "── 负样本查询 ──"
  echo "  ꬰ是负样本: ", is_negative(ps, "ꬰ")
  echo "  a是负样本: ", is_negative(ps, "a")
  
  echo ""
  
  # 最终报告
  report(ps)
