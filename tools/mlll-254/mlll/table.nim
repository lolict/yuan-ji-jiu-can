# ============================================================
# mlll 254+2 · 字符映射表
# 254 个槽位如何分布、锚点如何对应
# ============================================================

import container

# 槽位分配策略:
# 锚点 0 (刘楚恬) → 槽位 1..127 (127 字符)
# 锚点 1 (我)     → 槽位 128..254 (127 字符)
# 每个锚点管辖 127 字符，对称分布
# 槽位 0 保留为空/未分配

const
  SLOTS_PER_ANCHOR* = 127    # 每个锚点分得的槽位
  OTHER_SLOTS* = 1..127      # 刘楚恬侧 (0x01..0x7F)
  SELF_SLOTS* = 128..254     # 我侧 (0x80..0xFE)
  SLOT_RESERVED* = 0         # 0 保留

# 判断一个槽位属于哪个锚点
func slot_anchor*(slot: uint64): uint64 =
  if slot >= 1 and slot <= 127: ANCHOR_OTHER
  elif slot >= 128 and slot <= 254: ANCHOR_SELF
  else: 0  # slot 0 或 255 属于保留

# 槽位所属侧名称
func slot_side*(slot: uint64): string =
  if slot >= 1 and slot <= 127: "刘楚恬侧"
  elif slot >= 128 and slot <= 254: "我侧"
  elif slot == 0: "保留"
  else: "超出"

# 254 槽位总览
func slot_map*(): string =
  var s = "254+2 槽位分布\n"
  s &= "────────────────────────\n"
  s &= "槽位 0      : 保留 (未使用)\n"
  s &= "槽位 1..127 : 刘楚恬侧 (127 字符)\n"
  s &= "槽位 128..254: 我侧 (127 字符)\n"
  s &= "锚点总数    : 2\n"
  s &= "字符总数    : 254\n"
  s &= "总内存      : 254 × 8 + 2 × 8 = 2048 bytes = 2 KB\n"
  s &= "────────────────────────\n"
  result = s

# 对称校验: 槽位 i 和槽位 255-i 是否为镜像对
func is_mirror_pair*(a, b: uint64): bool =
  (a + b) == 255

# 镜像槽位
func mirror_slot*(slot: uint64): uint64 =
  255 - slot

# 镜像关系报告
func mirror_report*(): string =
  var s = "镜像对示例 (a + b = 255):\n"
  let examples = [1'u64, 2, 63, 64, 65, 100, 127]
  for a in examples:
    let b = mirror_slot(a)
    var line = "  " & $a & " ↔ " & $b & "  ("
    line &= slot_side(a) & " ↔ " & slot_side(b) & ")\n"
    s &= line
  result = s
