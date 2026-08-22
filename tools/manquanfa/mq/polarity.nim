# ============================================================
# 满全法 · YIN-YANG 极性平衡
# 两个锚点的极性分布
# ============================================================

# 极性状态
type
  Polarity* = enum
    plYin,      # 阴 (刘楚恬侧)
    plYang,     # 阳 (我侧)
    plBalanced  # 平衡

func polarity_name*(p: Polarity): string =
  case p
  of plYin: "阴(刘楚恬)"
  of plYang: "阳(lolict)"
  of plBalanced: "阴阳平衡"

# 判断一个容器的极性 (基于槽位)
func container_polarity*(slot: uint64): Polarity =
  if slot >= 1 and slot <= 127: plYin
  elif slot >= 128 and slot <= 254: plYang
  else: plBalanced  # slot 0 或 255

# 极性互补检查
func is_complementary*(a, b: uint64): bool =
  # a 和 b 是否为跨极性对
  let pa = container_polarity(a)
  let pb = container_polarity(b)
  (pa == plYin and pb == plYang) or (pa == plYang and pb == plYin)

# 极性报告
func polarity_report*(yin_count, yang_count: int): string =
  var s = "极性分布\n"
  s &= "────────────────────────\n"
  s &= "  阴 (刘楚恬侧): " & $yin_count & " 字符\n"
  s &= "  阳 (我侧):     " & $yang_count & " 字符\n"
  let diff = abs(yin_count - yang_count)
  if diff == 0:
    s &= "  状态: 完美平衡 ✓\n"
  elif diff <= 5:
    s &= "  状态: 近平衡 (差" & $diff & ")\n"
  else:
    s &= "  状态: 失衡 (差" & $diff & ") 需调整\n"
  s &= "────────────────────────\n"
  result = s
