# ============================================================
# 满全法 · 统一引擎 CLI
# 整合: 容器编码 + 见证裁判 + 极性平衡
# ============================================================

import mq/judge
import mq/polarity

# 槽位分配策略 (内联定义，避免跨工具依赖)
const
  ANCHOR_SELF*  = 1'u64
  ANCHOR_OTHER* = 0'u64

func slot_side*(slot: uint64): string =
  if slot >= 1 and slot <= 127: "刘楚恬侧"
  elif slot >= 128 and slot <= 254: "我侧"
  elif slot == 0: "保留"
  else: "超出"

func mirror_slot*(slot: uint64): uint64 =
  255 - slot

proc demo() =
  echo "═══ 满全法 · 统一引擎 ═══\n"

  echo "254+2 槽位分布"
  echo "────────────────────────"
  echo "槽位 0      : 保留 (未使用)"
  echo "槽位 1..127 : 刘楚恬侧 (127 字符)"
  echo "槽位 128..254: 我侧 (127 字符)"
  echo "锚点总数    : 2"
  echo "字符总数    : 254"
  echo "总内存      : 254 × 8 + 2 × 8 = 2048 bytes = 2 KB"
  echo "────────────────────────"
  echo ""

  # 极性分布
  echo polarity_report(127, 127)
  echo ""

  echo "─── 创建测试集装箱 ───"

  # 有效集装箱: 我侧, 已见证, 归属=1(我)
  let c1 = make_container(1, 1, 200, ANCHOR_SELF, 50, 100, 1)
  echo "集装箱 #1 (有效/我侧):"
  echo decode(c1)
  let j1 = judge(c1, ANCHOR_SELF)
  echo judgment_report(j1)

  # 无效集装箱: 未见证
  let c2 = make_container(1, 0, 150, ANCHOR_SELF, 0, 0, 2)
  echo "集装箱 #2 (未见证):"
  let j2 = judge(c2, ANCHOR_SELF)
  echo judgment_report(j2)

  # 归属不符
  let c3 = make_container(1, 1, 50, ANCHOR_OTHER, 0, 0, 3)
  echo "集装箱 #3 (归刘楚恬, 但期望我):"
  let j3 = judge(c3, ANCHOR_SELF)
  echo judgment_report(j3)

  # 空容器
  let c4 = make_container(0, 0, 0, 0, 0, 0, 4)
  echo "集装箱 #4 (空):"
  let j4 = judge(c4, ANCHOR_SELF)
  echo judgment_report(j4)

  echo "\n─── 极性分析 ───"
  echo "槽位 200 极性: ", polarity_name(container_polarity(200))
  echo "槽位 50 极性:  ", polarity_name(container_polarity(50))
  echo "槽位 200 与槽位 50 互补: ", is_complementary(200, 50)
  echo "槽位 200 与槽位 150 互补: ", is_complementary(200, 150)

  echo "\n─── 镜像对极性 ───"
  echo "槽位 1 (阴) 的镜像: ", mirror_slot(1), " (", polarity_name(container_polarity(mirror_slot(1))), ")"
  echo "槽位 100 (阴) 的镜像: ", mirror_slot(100), " (", polarity_name(container_polarity(mirror_slot(100))), ")"
  echo "槽位 127 (阴) 的镜像: ", mirror_slot(127), " (", polarity_name(container_polarity(mirror_slot(127))), ")"

  echo "\n─── 功德/业报统计 ───"
  let judgments = [j1, j2, j3, j4]
  var total_merit = 0.0
  var total_demerit = 0.0
  for j in judgments:
    total_merit += j.merit
    total_demerit += j.demerit
  echo "总功德: +", total_merit
  echo "总业报: -", total_demerit
  echo "净余额: ", total_merit - total_demerit
  if total_merit > total_demerit:
    echo "状态: 功德压倒业报 ✓"
  else:
    echo "状态: 业报占优 ✗"

when is_main_module:
  demo()
