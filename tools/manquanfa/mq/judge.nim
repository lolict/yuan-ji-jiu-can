# ============================================================
# 满全法 · 三元见证裁判引擎
# 裁定每个容器的准入/拒绝
# 依据: 见证态 + 归属一致 + 极性平衡
# ============================================================

import strutils

# MlllContainer 类型定义 (64 位集装箱)
type
  MlllContainer* = uint64

  # 位掩码
const
  MC_PRESENCE_M*  = 0b1'u64
  MC_WITNESS_M*   = 0b1'u64
  MC_CHAR_CODE_M* = 0xFF'u64
  MC_ANCHOR_M*    = 0b1'u64
  MC_RELATION_M*  = 0xFF'u64
  MC_TIMING_M*    = 0xFFFF'u64
  MC_PAYLOAD_M*   = 0x3FFFFF'u64

  MC_PRESENCE*  = 0
  MC_WITNESS*   = 1
  MC_CHAR_CODE* = 2
  MC_ANCHOR*    = 10
  MC_RELATION*  = 11
  MC_TIMING*    = 19
  MC_PAYLOAD*   = 35

  ANCHOR_SELF*  = 1'u64
  ANCHOR_OTHER* = 0'u64

# 取字段
func presence*(c: MlllContainer): uint64 = (c shr MC_PRESENCE) and MC_PRESENCE_M
func witness*(c: MlllContainer): uint64 = (c shr MC_WITNESS) and MC_WITNESS_M
func char_code*(c: MlllContainer): uint64 = (c shr MC_CHAR_CODE) and MC_CHAR_CODE_M
func anchor*(c: MlllContainer): uint64 = (c shr MC_ANCHOR) and MC_ANCHOR_M
func relation*(c: MlllContainer): uint64 = (c shr MC_RELATION) and MC_RELATION_M
func timing*(c: MlllContainer): uint64 = (c shr MC_TIMING) and MC_TIMING_M
func payload*(c: MlllContainer): uint64 = (c shr MC_PAYLOAD) and MC_PAYLOAD_M

# 构建集装箱
func make_container*(presence, witness, char_code, anchor, relation, timing, payload: uint64): MlllContainer =
  result = (presence and MC_PRESENCE_M) shl MC_PRESENCE
  result = result or ((witness and MC_WITNESS_M) shl MC_WITNESS)
  result = result or ((char_code and MC_CHAR_CODE_M) shl MC_CHAR_CODE)
  result = result or ((anchor and MC_ANCHOR_M) shl MC_ANCHOR)
  result = result or ((relation and MC_RELATION_M) shl MC_RELATION)
  result = result or ((timing and MC_TIMING_M) shl MC_TIMING)
  result = result or ((payload and MC_PAYLOAD_M) shl MC_PAYLOAD)

# 锚点名称
func anchor_name*(a: uint64): string =
  if a == ANCHOR_SELF: "我(1)"
  elif a == ANCHOR_OTHER: "刘楚恬(0)"
  else: "?"

# 解码报告
func decode*(c: MlllContainer): string =
  var s = "集装箱 0x" & to_hex(c, 16) & "\n"
  s &= "  [presence] " & $c.presence & " → " & (if c.presence > 0: "有米" else: "空") & "\n"
  s &= "  [witness]  " & $c.witness & " → " & (if c.witness > 0: "已见证" else: "未见证") & "\n"
  s &= "  [char]     " & $c.char_code & " (槽位" & $c.char_code & "/254)\n"
  s &= "  [anchor]   " & $c.anchor & " → " & anchor_name(c.anchor) & "\n"
  s &= "  [relation] " & $c.relation & " → 指向#" & $c.relation & "\n"
  s &= "  [timing]   " & $c.timing & " tick\n"
  s &= "  [payload]  " & $c.payload & "\n"
  result = s

# 裁判结果
type
  Verdict* = enum
    vrAccept,    # 准入
    vrReject,    # 拒绝
    vrPending,   # 待裁
    vrAppeal     # 申诉

  Judgment* = object
    container_id*: int
    verdict*: Verdict
    reason*: string
    merit*: float   # 功德值
    demerit*: float # 业报值

# 见证规则常量
const
  MERIT_ACCEPT* = 10.0    # 准入功德
  DEMERIT_REJECT* = 15.0  # 拒绝业报

# 核心裁判函数
func judge*(c: MlllContainer, expected_anchor: uint64): Judgment =
  result.container_id = int(c.payload)  # 简化: 用 payload 作 ID
  result.merit = 0.0
  result.demerit = 0.0

  # 规则 1: 空容器直接拒
  if c.presence == 0:
    result.verdict = vrReject
    result.reason = "空袋无米"
    result.demerit = DEMERIT_REJECT
    return

  # 规则 2: 未见证的容器待裁
  if c.witness == 0:
    result.verdict = vrPending
    result.reason = "未见证，待定"
    return

  # 规则 3: 归属锚点不符 → 拒绝
  if c.anchor != expected_anchor:
    result.verdict = vrReject
    result.reason = "归属不符: 期望锚点" & $expected_anchor & " 实际" & $c.anchor
    result.demerit = DEMERIT_REJECT
    return

  # 规则 4: 字符槽位 0 → 无效
  if c.char_code == 0:
    result.verdict = vrReject
    result.reason = "槽位 0 为保留位"
    result.demerit = DEMERIT_REJECT
    return

  # 规则 5: 全部通过 → 准入
  result.verdict = vrAccept
  result.reason = "三元见证通过"
  result.merit = MERIT_ACCEPT

# 裁判结果描述
func verdict_desc*(v: Verdict): string =
  case v
  of vrAccept: "准入"
  of vrReject: "拒绝"
  of vrPending: "待裁"
  of vrAppeal: "申诉"

# 判决书
func judgment_report*(j: Judgment): string =
  var s = "━━ 三元见证裁判 ━━\n"
  s &= "  容器 ID: " & $j.container_id & "\n"
  s &= "  裁定:   " & verdict_desc(j.verdict) & "\n"
  s &= "  理由:   " & $j.reason & "\n"
  if j.merit > 0:
    s &= "  功德:   +" & $j.merit & "\n"
  if j.demerit > 0:
    s &= "  业报:   -" & $j.demerit & "\n"
  result = s
