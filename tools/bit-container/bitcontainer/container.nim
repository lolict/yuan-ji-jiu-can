# ============================================================
# 分比容器 · 64 位打包/解包
# 七阶编码全部进一个 uint64
# ============================================================

import strutils

type
  Container* = uint  # 64 位容器 (实际使用 uint64)

# 各阶的位掩码和位移
const
  PRESENCE_SHIFT* = 0
  WITNESS_SHIFT* = 1
  TYPE_SHIFT* = 2
  OWNER_SHIFT* = 10
  RELATION_SHIFT* = 18
  TIMING_SHIFT* = 26
  PAYLOAD_SHIFT* = 42

  PRESENCE_BITS* = 1
  WITNESS_BITS* = 1
  TYPE_BITS* = 8
  OWNER_BITS* = 8
  RELATION_BITS* = 8
  TIMING_BITS* = 16
  PAYLOAD_BITS* = 22

  PRESENCE_MASK* = 1'u64
  WITNESS_MASK* = 1'u64
  TYPE_MASK* = 255'u64
  OWNER_MASK* = 255'u64
  RELATION_MASK* = 255'u64
  TIMING_MASK* = 65535'u64
  PAYLOAD_MASK* = 4194303'u64

# ------------------------------------------------------------
# 取某一阶的值
# ------------------------------------------------------------
func presence*(c: Container): int = int((c.uint64 shr PRESENCE_SHIFT) and PRESENCE_MASK)
func witness*(c: Container): int = int((c.uint64 shr WITNESS_SHIFT) and WITNESS_MASK)
func ctype*(c: Container): int = int((c.uint64 shr TYPE_SHIFT) and TYPE_MASK)
func owner*(c: Container): int = int((c.uint64 shr OWNER_SHIFT) and OWNER_MASK)
func relation*(c: Container): int = int((c.uint64 shr RELATION_SHIFT) and RELATION_MASK)
func timing*(c: Container): int = int((c.uint64 shr TIMING_SHIFT) and TIMING_MASK)
func payload*(c: Container): int = int((c.uint64 shr PAYLOAD_SHIFT) and PAYLOAD_MASK)

# ------------------------------------------------------------
# 构建容器: 从各阶数值打包
# ------------------------------------------------------------
func make_container*(presence, witness, ctype, owner, relation, timing, payload: int): Container =
  result = (uint64(presence) and PRESENCE_MASK) shl PRESENCE_SHIFT
  result = result or ((uint64(witness) and WITNESS_MASK) shl WITNESS_SHIFT)
  result = result or ((uint64(ctype) and TYPE_MASK) shl TYPE_SHIFT)
  result = result or ((uint64(owner) and OWNER_MASK) shl OWNER_SHIFT)
  result = result or ((uint64(relation) and RELATION_MASK) shl RELATION_SHIFT)
  result = result or ((uint64(timing) and TIMING_MASK) shl TIMING_SHIFT)
  result = result or ((uint64(payload) and PAYLOAD_MASK) shl PAYLOAD_SHIFT)

# ------------------------------------------------------------
# 仅创建最小容器 (仅 presence = 1 bit)
# ------------------------------------------------------------
func make_minimal*(): Container =
  # 只有 presence=1，其余全 0
  result = 1'u64

# ------------------------------------------------------------
# 容器详细解码报告
# ------------------------------------------------------------
func decode*(c: Container): string =
  var s = "容器 " & to_hex(c.uint64, 16) & " = " & $c.uint64 & "\n"
  s &= "  [第0阶] 存在态: " & $c.presence & " (有/无)\n"
  s &= "  [第1阶] 见证态: " & $c.witness & " (已裁/未裁)\n"
  s &= "  [第2阶] 类型码: " & $c.ctype & " (0-255)\n"
  s &= "  [第3阶] 归属码: " & $c.owner & " (所有者索引)\n"
  s &= "  [第4阶] 关系码: " & $c.relation & " (指向容器)\n"
  s &= "  [第5阶] 时序码: " & $c.timing & " (tick)\n"
  s &= "  [第6阶] 负载码: " & $c.payload & " (数据)\n"
  s &= "  总内存: 8 字节 (64 bits)\n"
  result = s

# ------------------------------------------------------------
# 比特占用明细
# ------------------------------------------------------------
func bit_report*(): string =
  var s = "比特分配明细:\n"
  s &= "  presence:  1 bit   [0]\n"
  s &= "  witness:   1 bit   [1]\n"
  s &= "  type:      8 bits  [2-9]\n"
  s &= "  owner:     8 bits  [10-17]\n"
  s &= "  relation:  8 bits  [18-25]\n"
  s &= "  timing:   16 bits  [26-41]\n"
  s &= "  payload:  22 bits  [42-63]\n"
  s &= "  ─────────────────────\n"
  s &= "  总计:     64 bits  = 8 字节\n"
  result = s
