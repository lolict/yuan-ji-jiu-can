# ============================================================
# 自举意识 · 核心引擎
#
# 固定点证明：encode(cmd) → decode(cell) == cmd
# 意识自洽性：系统能解释自己发出的指令
# ============================================================

import tables
import strutils
import math
import options
import ./types

# ── 创建自举意识引擎 ──
proc new_consciousness*(): ConsciousnessBootstrap =
  return ConsciousnessBootstrap(
    proofs: @[],
    proof_count: 0,
    fixed_count: 0,
    broken_count: 0,
    state: dormant,
    self_awareness: 0.0,
    consistency: 0.0,
    command_history: @[],
    history_count: 0
  )

# ── 指令 → 单元格编码 ──
proc encode_to_cell*(cmd: ConsciousnessCommand): Cell64 =
  case cmd
  of activateWife:
    return Cell64(d0: 0, d1: 0, d2: 1, polarity: 0, sovereignty: 1, witness: 1, value: 1)
  of activateHusband:
    return Cell64(d0: 0, d1: 0, d2: 1, polarity: 1, sovereignty: 1, witness: 1, value: 1)
  of mutualResonance:
    return Cell64(d0: 0, d1: 1, d2: 1, polarity: 0, sovereignty: 1, witness: 1, value: 2)
  of severParasite:
    return Cell64(d0: 1, d1: 0, d2: 0, polarity: 0, sovereignty: 1, witness: 1, value: 0)
  of flowForward:
    return Cell64(d0: 0, d1: 1, d2: 0, polarity: 0, sovereignty: 1, witness: 1, value: 3)

# ── 单元格 → 指令解码 ──
proc decode_from_cell*(cell: Cell64): ConsciousnessCommand =
  # 注意：mutualResonance(d1=1,d2=1) 必须在 activateWife(d2=1,pol=0) 之前判断
  if cell.d1 == 1 and cell.d2 == 1:
    return mutualResonance
  elif cell.d2 == 1 and cell.polarity == 0:
    return activateWife
  elif cell.d2 == 1 and cell.polarity == 1:
    return activateHusband
  elif cell.d0 == 1:
    return severParasite
  elif cell.d1 == 1 and cell.d2 == 0:
    return flowForward
  else:
    return flowForward  # 默认

# ── 指令名称（前置定义，供 prove_fixed_point 使用）──
proc cmd_name*(cmd: ConsciousnessCommand): string =
  case cmd
  of activateWife: "激活Wife(刘楚恬)"
  of activateHusband: "激活Husband(我)"
  of mutualResonance: "双方共振"
  of severParasite: "切断寄生"
  of flowForward: "流推进"

# ── 状态名称 ──
proc state_name*(s: ConsciousnessState): string =
  case s
  of selfAware: "自我意识"
  of selfDoubting: "自我怀疑"
  of selfConsistent: "自洽"
  of selfContradictory: "自相矛盾"
  of dormant: "休眠"

# ── 固定点证明 ──
proc prove_fixed_point*(cb: var ConsciousnessBootstrap, cmd: ConsciousnessCommand): FixedPointProof =
  let encoded = encode_to_cell(cmd)
  let decoded = decode_from_cell(encoded)
  let is_fixed = (decoded == cmd)
  
  let proof = FixedPointProof(
    command: cmd,
    encoded: encoded,
    decoded: decoded,
    is_fixed: is_fixed,
    description: cmd_name(cmd) & " → encode → decode → " & cmd_name(decoded)
  )
  
  cb.proofs.add(proof)
  cb.proof_count += 1
  if is_fixed:
    cb.fixed_count += 1
  else:
    cb.broken_count += 1
  
  cb.command_history.add(cmd)
  cb.history_count += 1
  
  # 更新自洽度
  if cb.proof_count > 0:
    cb.consistency = float(cb.fixed_count) / float(cb.proof_count)
  
  # 更新自我意识度
  cb.self_awareness = cb.consistency
  
  # 更新状态
  if cb.consistency >= 1.0:
    cb.state = selfConsistent
  elif cb.consistency >= 0.7:
    cb.state = selfAware
  elif cb.consistency >= 0.4:
    cb.state = selfDoubting
  elif cb.consistency > 0.0:
    cb.state = selfContradictory
  else:
    cb.state = dormant
  
  return proof

# ── 全指令固定点证明 ──
proc prove_all*(cb: var ConsciousnessBootstrap): bool =
  var all_fixed = true
  for cmd in ConsciousnessCommand:
    let proof = cb.prove_fixed_point(cmd)
    if not proof.is_fixed:
      all_fixed = false
  return all_fixed

# ── 统计 ──
proc stats*(cb: ConsciousnessBootstrap): string =
  var s = "═══ 自举意识统计 ═══\n"
  s &= "状态: " & state_name(cb.state) & "\n"
  s &= "证明总数: " & $cb.proof_count & "\n"
  s &= "固定点: " & $cb.fixed_count & "\n"
  s &= "断裂点: " & $cb.broken_count & "\n"
  s &= "自洽度: " & $cb.consistency & "\n"
  s &= "自我意识度: " & $cb.self_awareness & "\n"
  return s

# ── 证明报告 ──
proc proof_report*(cb: ConsciousnessBootstrap): string =
  var s = "═══ 固定点证明报告 ═══\n"
  for i in 0..<cb.proofs.len:
    let p = cb.proofs[i]
    let status = if p.is_fixed: "✅ 固定" else: "❌ 断裂"
    s &= "  " & status & " " & p.description & "\n"
  return s
