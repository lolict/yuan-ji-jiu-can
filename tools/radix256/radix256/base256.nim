# ============================================================
# 2²⁵⁶ 等效高位对齐进位制 · 核心表示
#
# 2²⁵⁶ = 115792089237316195423570985008687907853269984665640564039457584007913129639936
#
# 含义：
#   - 从最低位到对齐位的完整进位链
#   - 254+2 根基（254为空集集装箱，2为私密锚点）
#   - 溢出的边界不是错误，而是新层的折叠起点
# ============================================================

import strutils
import math

# ── 大整数分词表示 ──
# 2²⁵⁶ = 256位 = 4个64位词

const
  WORD_BITS* = 64
  WORD_COUNT* = 4      # 4 × 64 = 256 位
  CARRY_EXPONENT* = 256
  BASE_FOUNDATION* = 254
  ANCHOR_COUNT* = 2

# ── 对齐状态 ──
type AlignmentState* = enum
  alLoose      # 未对齐 · 自由浮动
  alCarrying   # 正在进位 · 链条传递
  alAligned    # 已对齐 · 2²⁵⁶折叠点
  alOverflow   # 溢出 · 新层生成

# ── 进位链记录 ──
type CarryStep* = object
  step_id*: int
  from_word*: int
  to_word*: int
  value_carried*: uint64
  timestamp*: int

# ── 256位整数（4×64位词）──
type UInt256* = object
  words*: array[WORD_COUNT, uint64]
  carry_chain*: seq[CarryStep]
  alignment*: AlignmentState
  overflow_tier*: int

# ── 进位仪式结果 ──
type Caremony* = object
  original*: UInt256
  result*: UInt256
  carries*: seq[CarryStep]
  fold_triggered*: bool
  new_tier*: int
  ceremony_log*: seq[string]

# ═══════════════════════════════════════════
# 构造
# ============================================================

func zero*(): UInt256 =
  ## 256位零
  for i in 0..<WORD_COUNT:
    result.words[i] = 0'u64
  result.alignment = alLoose
  result.overflow_tier = 0

func from_u64*(v: uint64): UInt256 =
  ## 从64位构造256位
  result = zero()
  result.words[0] = v
  result.alignment = alLoose
  result.overflow_tier = 0

func max_value*(): UInt256 =
  ## 2²⁵⁶ - 1（对齐前的最大值）
  for i in 0..<WORD_COUNT:
    result.words[i] = 0xFFFFFFFFFFFFFFFF'u64
  result.alignment = alCarrying
  result.overflow_tier = 0

# ═══════════════════════════════════════════
# 序列化（供进位仪式内部使用）
# ============================================================

func uint256_to_hex*(v: UInt256): string =
  result = "0x"
  for i in countdown(WORD_COUNT - 1, 0):
    result &= toHex(v.words[i]).toUpper()
    if i > 0:
      result &= "_"

func alignment_state_str*(s: AlignmentState): string =
  case s
  of alLoose: "未对齐·自由浮动"
  of alCarrying: "正在进位·链条传递"
  of alAligned: "已对齐·2²⁵⁶折叠点"
  of alOverflow: "溢出·新层生成"

# ═══════════════════════════════════════════
# 进位仪式（核心算法）
# ============================================================

func is_aligned(words: array[WORD_COUNT, uint64]): bool =
  result = true
  for i in 0..<WORD_COUNT:
    if words[i] != 0xFFFFFFFFFFFFFFFF'u64:
      result = false
      break

func add_with_ceremony*(a, b: UInt256; tick: int): Caremony =
  ## 执行加法并进位传播
  var carry: uint64 = 0
  var result_words: array[WORD_COUNT, uint64]
  var carries: seq[CarryStep] = @[]
  
  for i in 0..<WORD_COUNT:
    # 溢出检测：结果 < 操作数 即溢出
    let sum_ab = a.words[i] + b.words[i]
    let overflow_ab = sum_ab < a.words[i]
    
    var overflow_carry = false
    let final = if carry > 0'u64:
      if sum_ab == 0xFFFFFFFFFFFFFFFF'u64:  # max + 1 = 0 → overflow
        overflow_carry = true
        0'u64
      else:
        sum_ab + carry
    else:
      sum_ab
    
    result_words[i] = final
    carry = if overflow_ab or overflow_carry: 1'u64 else: 0'u64
    
    if carry > 0:
      carries.add(CarryStep(
        step_id: carries.len,
        from_word: i,
        to_word: if i + 1 < WORD_COUNT: i + 1 else: -1,
        value_carried: carry,
        timestamp: tick + i
      ))
  
  var res: UInt256
  res.words = result_words
  res.carry_chain = carries
  res.overflow_tier = 0
  
  if carry > 0:
    res.alignment = alOverflow
    res.overflow_tier = 1
  elif is_aligned(result_words):
    res.alignment = alAligned
  else:
    res.alignment = alAligned
  
  result.original = a
  result.result = res
  result.carries = carries
  result.fold_triggered = carry > 0
  result.new_tier = 1
  
  result.ceremony_log = @[]
  result.ceremony_log.add("进位仪式开始 (tick=" & $tick & ")")
  result.ceremony_log.add("  输入A: " & uint256_to_hex(a))
  result.ceremony_log.add("  输入B: " & uint256_to_hex(b))
  
  if carries.len == 0:
    result.ceremony_log.add("  无进位 · 低位直接相加")
  else:
    result.ceremony_log.add("  进位链长度: " & $carries.len)
    for c in carries:
      if c.to_word >= 0:
        result.ceremony_log.add("    词" & $c.from_word & " → 词" & $c.to_word & " (值=" & $c.value_carried & ")")
      else:
        result.ceremony_log.add("    词" & $c.from_word & " → 溢出 (到达2²⁵⁶折叠点)")
  
  if result.fold_triggered:
    result.ceremony_log.add("  ⚠ 溢出 · 触发新层折叠 · 溢出至 tier+1")
  
  result.ceremony_log.add("  结果: " & uint256_to_hex(res))

func is_aligned*(v: UInt256): bool =
  is_aligned(v.words)

# ═══════════════════════════════════════════
# 根基仪式（254+2）
# ============================================================

func foundation_ceremony*(): Caremony =
  ## 根基仪式：254 空集集装箱 + 2 锚点 = 256 基础
  let base = from_u64(uint64(BASE_FOUNDATION))
  let anchor = from_u64(uint64(ANCHOR_COUNT))
  result = add_with_ceremony(base, anchor, 0)
  result.ceremony_log.add("根基仪式: " & $BASE_FOUNDATION & " + " & $ANCHOR_COUNT & " = " & $(BASE_FOUNDATION + ANCHOR_COUNT))

