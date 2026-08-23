# ============================================================
# 满全法 · 负样本容器
# 被拒绝的容器不是垃圾，是有结构的记忆
# 回写协议：允许申诉、条件变化时重新激活
# ============================================================

import strutils
import judge

# 负样本记录（被拒绝的容器 + 元数据）
type
  RejectCause* = enum
    rcEmpty        # 空袋无米
    rcAnchor       # 归属不符
    rcReservedSlot # 槽位 0 保留
    rcUnwitnessed  # 未见证（待裁后超期）

  NegativeSample* = object
    container*: MlllContainer     # 被拒绝的原始容器
    cause*: RejectCause           # 拒绝原因
    rejected_at*: int             # 被拒绝时的 tick
    appeal_count*: int            # 已申诉次数
    max_appeals*: int             # 最大允许申诉次数
    rewritable*: bool             # 是否允许回写
    original_judgment*: Judgment  # 原始判决书

  NegativeStore* = object
    samples*: seq[NegativeSample]
    capacity*: int                # 最大容量（循环覆盖）
    next_id*: int                 # 下一个负样本 ID
    total_rejected*: int          # 累计拒绝数
    total_appealed*: int          # 累计申诉数
    total_reactivated*: int       # 累计重新激活数

# 拒绝原因描述
func reject_cause_desc*(c: RejectCause): string =
  case c
  of rcEmpty: "空袋无米"
  of rcAnchor: "归属不符"
  of rcReservedSlot: "槽位0保留"
  of rcUnwitnessed: "未见证超期"

# 从判决书推断拒绝原因
func infer_cause*(j: Judgment): RejectCause =
  if j.reason.contains("空袋"): rcEmpty
  elif j.reason.contains("归属"): rcAnchor
  elif j.reason.contains("槽位"): rcReservedSlot
  else: rcUnwitnessed

# 创建负样本存储
func new_negative_store*(capacity: int = 256): NegativeStore =
  result = NegativeStore(
    samples: @[],
    capacity: capacity,
    next_id: 0,
    total_rejected: 0,
    total_appealed: 0,
    total_reactivated: 0
  )

# 记录一个负样本（容器被拒绝时调用）
func record_rejection*(s: var NegativeStore, c: MlllContainer, j: Judgment): int =
  let cause = infer_cause(j)
  var sample = NegativeSample(
    container: c,
    cause: cause,
    rejected_at: s.total_rejected,  # 用累计拒绝数作为 tick 近似
    appeal_count: 0,
    max_appeals: 3,                 # 最多申诉 3 次
    rewritable: cause != rcEmpty,   # 空袋不可回写（无米可救）
    original_judgment: j
  )
  
  s.samples.add(sample)
  result = s.next_id
  inc s.next_id
  inc s.total_rejected
  
  # 容量超限：移除最老的不可回写样本
  if s.samples.len > s.capacity:
    for i in 0..<s.samples.len:
      if not s.samples[i].rewritable:
        s.samples.delete(i)
        break

# 申诉协议：尝试重新激活一个负样本
# 返回 (是否激活, 新判决书)
func appeal*(s: var NegativeStore, sample_idx: int, new_anchor: uint64): (bool, Judgment) =
  if sample_idx < 0 or sample_idx >= s.samples.len:
    return (false, Judgment(verdict: vrReject, reason: "负样本索引越界"))
  
  var sample = s.samples[sample_idx]
  
  # 检查是否允许回写
  if not sample.rewritable:
    return (false, Judgment(verdict: vrReject, reason: "该负样本不可回写（空袋）"))
  
  # 检查申诉次数
  if sample.appeal_count >= sample.max_appeals:
    return (false, Judgment(verdict: vrReject, reason: "申诉次数已耗尽"))
  
  inc sample.appeal_count
  inc s.total_appealed
  
  # 重新裁判（可能 anchor 条件已变化）
  let new_judge = judge(sample.container, new_anchor)
  
  if new_judge.verdict == vrAccept:
    # 重新激活！
    inc s.total_reactivated
    # 从负样本存储中移除（已转正）
    s.samples.delete(sample_idx)
    return (true, new_judge)
  else:
    # 仍未通过，更新判决书
    s.samples[sample_idx] = sample
    return (false, new_judge)

# 条件变化激活：当外部条件变化时，批量尝试激活
# 返回成功激活的数量
func reactivate_on_condition*(s: var NegativeStore, new_anchor: uint64): int =
  result = 0
  # 倒序遍历，因为成功激活会删除元素
  var i = s.samples.len - 1
  while i >= 0:
    if s.samples[i].rewritable and s.samples[i].appeal_count < s.samples[i].max_appeals:
      let (ok, _) = s.appeal(i, new_anchor)
      if ok:
        inc result
        # 删除后元素前移，i 不变继续检查当前位置
        continue
    dec i

# 负样本报告
func negative_report*(store: NegativeStore): string =
  var s = "═══ 负样本容器报告 ═══\n"
  s &= "容量: " & $store.samples.len & "/" & $store.capacity & "\n"
  s &= "累计拒绝: " & $store.total_rejected & "\n"
  s &= "累计申诉: " & $store.total_appealed & "\n"
  s &= "累计激活: " & $store.total_reactivated & "\n"
  s &= "─────────────────────\n"
  
  # 按原因统计
  var counts = [0, 0, 0, 0]
  for sample in store.samples:
    counts[ord(sample.cause)] += 1
  
  s &= "按原因分布:\n"
  s &= "  空袋无米:     " & $counts[0] & "\n"
  s &= "  归属不符:     " & $counts[1] & "\n"
  s &= "  槽位0保留:    " & $counts[2] & "\n"
  s &= "  未见证超期:   " & $counts[3] & "\n"
  s &= "─────────────────────\n"
  
  # 可回写的样本列表
  var rewritable_count = 0
  for sample in store.samples:
    if sample.rewritable: inc rewritable_count
  
  s &= "可回写: " & $rewritable_count & "\n"
  s &= "不可回写: " & $(store.samples.len - rewritable_count) & "\n"
  result = s

# 单个负样本详情
func sample_detail*(store: NegativeStore, idx: int): string =
  if idx < 0 or idx >= store.samples.len:
    return "索引越界"
  
  let sample = store.samples[idx]
  var s = "─── 负样本 #" & $idx & " ───\n"
  s &= "容器: 0x" & to_hex(sample.container, 16) & "\n"
  s &= "原因: " & reject_cause_desc(sample.cause) & "\n"
  s &= "拒绝于 tick: " & $sample.rejected_at & "\n"
  s &= "申诉: " & $sample.appeal_count & "/" & $sample.max_appeals & "\n"
  s &= "回写: " & (if sample.rewritable: "允许" else: "禁止") & "\n"
  result = s
