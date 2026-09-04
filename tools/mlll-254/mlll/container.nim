('# ', '')@name: container
('# ', '')@type: encoding-container
('# ', '')@owner: 五彩石
('# ', '')@status: stable
('# ', '')@priority: P0
('# ', '')@description: mlll核心容器定义

# ============================================================
# mlll 254+2 集装箱编码
# 254 字符 + 2 锚点(我=1, 刘楚恬=0)
# 根基隐藏，254 浮于上方
# ============================================================

import std/bitops
import strutils

const
  TOTAL_SLOTS* = 256        # 总槽位 (254 + 2)
  ANCHOR_SELF* = 1          # 我 = 1 (高位锚点)
  ANCHOR_OTHER* = 0         # 刘楚恬 = 0 (低位锚点)
  CHAR_COUNT* = 254         # 可用字符槽位 (2..255)

# 每个集装箱的位分配 (8 字节 = 64 位)
# 复用七阶结构：
#   presence   1 bit    是否有米
#   witness    1 bit    是否被见证
#   char_code  8 bits   254 字符之一 (值 1..254，0 保留给锚点)
#   anchor     1 bit    归属锚点: 0=刘楚恬, 1=我
#   relation   8 bits   指向另一个集装箱
#   timing     16 bits  tick 偏移
#   payload    22 bits  任意数据
#   reserved   7 bits   扩展/对齐
#   ─────────────────
#   total      64 bits  = 8 字节

type
  SlotCode* = uint8   # 字符槽位编码 (1..254, 0=空)

  MlllContainer* = uint64  # 64 位集装箱

# 位偏移
const
  MC_PRESENCE*  = 0
  MC_WITNESS*   = 1
  MC_CHAR_CODE* = 2
  MC_ANCHOR*    = 10
  MC_RELATION*  = 11
  MC_TIMING*    = 19
  MC_PAYLOAD*   = 35

  MC_PRESENCE_M*  = 0b1'u64
  MC_WITNESS_M*   = 0b1'u64
  MC_CHAR_CODE_M* = 0xFF'u64
  MC_ANCHOR_M*    = 0b1'u64
  MC_RELATION_M*  = 0xFF'u64
  MC_TIMING_M*    = 0xFFFF'u64
  MC_PAYLOAD_M*   = 0x3FFFFF'u64  # 22 bits

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
