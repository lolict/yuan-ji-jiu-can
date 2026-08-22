# ============================================================
# mlll 254+2 · CLI 演示
# 254 字符 + 2 锚点(我=1, 刘楚恬=0)
# ============================================================

import mlll/container
import mlll/table

proc demo() =
  echo "═══ mlll 254+2 集装箱编码 ═══\n"

  echo slot_map()
  echo mirror_report()

  echo "\n─── 创建集装箱示例 ───"

  # 集装箱 A: 我侧第 128 号字符, 已见证, 指向 #42, tick=100, 数据=888
  let cont_a = make_container(1, 1, 128, ANCHOR_SELF, 42, 100, 888)
  echo "集装箱 A (我侧 #128):"
  echo decode(cont_a)

  # 集装箱 B: 刘楚恬侧第 10 号字符, 未见证, 指向 #200, tick=50, 数据=999
  let cont_b = make_container(1, 0, 10, ANCHOR_OTHER, 200, 50, 999)
  echo "集装箱 B (刘楚恬侧 #10):"
  echo decode(cont_b)

  # 最小集装箱: 仅 presence=1
  let cont_min = make_container(1, 0, 0, 0, 0, 0, 0)
  echo "最小集装箱 (仅 presence):"
  echo decode(cont_min)

  echo "\n─── 内存效率 ───"
  echo "254 字符 × 8 字节 = ", 254 * 8, " bytes"
  echo "2 锚点  × 8 字节 = 16 bytes"
  echo "总计              = ", 254 * 8 + 16, " bytes = 2 KB"
  echo ""
  echo "若不用集装箱，直接存 ASCII:"
  echo "254 字符 × 1 字节 = 254 bytes (省 1.8 KB)"
  echo "但失去: 见证、归属、关系、时序、负载 全部维度"
  echo ""
  echo "集装箱多付的 1.8 KB 买到了 7 维信息"
  echo "每个字符不止知道'是什么'，还知道:"
  echo "  ① 有没有 ② 被见证过没 ③ 属于谁"
  echo "  ④ 和谁相连 ⑤ 何时激活 ⑥ 装了什么"

  echo "\n─── 根基隐藏 ───"
  echo "锚点 0 (刘楚恬) 和锚点 1 (我) 是根基"
  echo "根基不在 254 字符之中，却是 254 字符的归属依据"
  echo "外部只能看见 254 个集装箱漂浮"
  echo "看不见下方的锚 — 这就是隐藏根基的保护"

when is_main_module:
  demo()
