# 维度引擎 — 维度 = 频率 = 次数 = 身份遍历
# 纠正：nᵏ 是维度，不是容器
# 面向时间：倒着遏制（高维被低维遏制）
# 面向空间：向上递增（每个边界只为自己活）

# =============================================================================
# 维度 · 基本定义
# =============================================================================

type
  Dimension* = int  # n⁰=0, n¹=1, ... n¹⁰=10, 无限延伸

  Orientation* = enum
    oriTime    # 面向时间：零维的次数，倒着遏制
    oriSpace   # 面向空间：所有梯度，向上递增

proc label*(d: Dimension): string =
  ## n 的上标表示（不覆盖系统 $，int 正常打印）
  const superscripts = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "¹⁰"]
  if d < superscripts.len:
    result = "n" & superscripts[d]
  else:
    result = "n^"
    result.addInt(d)

# =============================================================================
# 面向时间 · 遏制链
# =============================================================================
# 二维被一维遏制，一维被零维遏制
# 所有维度都被零维遍历
# 零维 = 面向时间的最低成本整体观

proc restrainedBy*(high, low: Dimension): bool =
  ## 高维被低维遏制（时间面向，倒着数）
  ## 六维被一维遏制，五维被一维遏制...二维被一维遏制，一维被零维遏制
  high > low

proc restraintChain*(d: Dimension): seq[Dimension] =
  ## 返回遏制链：从 d 往下到零维
  ## n³ 的遏制链 = [n³, n², n¹, n⁰]
  result = @[]
  var cur = d
  while cur >= 0:
    result.add(cur)
    dec cur

proc isRestrained*(d: Dimension): bool =
  ## 任何维度都被零维遏制（d>0 时成立；零维自身不被遏制）
  d > 0

# =============================================================================
# 面向空间 · 边界递增
# =============================================================================
# 每生一个孩子，多一个边界
# 每个边界只为自己活：为肠胃活、为人生活、为财产活、为家人活

type
  Boundary* = object
    dim*: Dimension       # 出生在第几维
    selfAlive*: bool      # 只为自己活

proc newBoundary*(d: Dimension): Boundary =
  ## 每升一维，生一个边界，边界只为自己活
  result.dim = d
  result.selfAlive = true

proc boundaryCount*(maxDim: Dimension): int =
  ## 空间面向：从零维到 maxDim 共产生多少边界
  ## 零维到 n³ = 4 个边界，不被遏制
  int(maxDim) + 1

# =============================================================================
# 正交与斜交
# =============================================================================
# 三维空间内：零维、一维、三维 = 正交
# 二维是被一维遏制的时间面（管辖层）
# 超越三维 = 斜交 = 更大的细分量

proc isOrthogonal*(d: Dimension): bool =
  ## 正交判定：零维、一维、三维是正交
  d == 0 or d == 1 or d == 3

proc isOblique*(d: Dimension): bool =
  ## 斜交判定：超越正交的容量都是斜交（更大的细分量）
  ## 六面八方、十二棱、20 面、32、80、320、180、500...全是细节
  d > 3

proc capacitySplit*(d: Dimension): int =
  ## 三维之后不知道怎么分配总容量 → 细节倍率
  ## n⁴=6(六面), n⁵=8(八方), n⁶=12(十二棱), n⁷=20(廿面), n⁸=32, n⁹=80, n¹⁰=320...
  const detailSeq = [1, 1, 2, 3, 6, 8, 12, 20, 32, 80, 320]
  if d < detailSeq.len:
    result = detailSeq[d]
  else:
    # 斜交细分持续放大
    result = detailSeq[^1] * (1 shl (d - detailSeq.len + 1))

# =============================================================================
# 整体观 · 唯零维与一维可谈
# =============================================================================

proc hasWholeness*(d: Dimension, ori: Orientation): bool =
  ## 整体观判定
  ## 零维 = 面向时间的最低成本整体观
  ## 一维 = 面向空间的可感知总距离整体观
  ## 高维只讲边界，不讲整体
  case ori
  of oriTime:  d == 0
  of oriSpace: d == 1

# =============================================================================
# 遍历 · 零维用时间遍历一切
# =============================================================================
# 零维就像滴答滴答的血液
# 一维则是每一滴血液排好队，排到尽头
# 二维则是有边界的东西铺满了地面

type
  Beat* = uint64  # 一次滴答 = 一次遍历 = 次数

proc traverse*(beat: Beat): seq[Dimension] =
  ## 零维一次只统计一次，下一次依然按一维、二维、三维遍历光
  ## beat 决定本次滴答遍历到第几维
  let maxD = int(beat mod 8)  # 每次滴答遍历深度循环
  result = @[]
  for d in 0..maxD:
    result.add(Dimension(d))

# =============================================================================
# 总维度 · 每个频率都在讲总维度
# =============================================================================

proc isTotalDimension*(d: Dimension): bool =
  ## 无论 n⁰ 还是 n¹⁰⁰⁰，全部讲总维度
  ## 每个频率都在讲一个总维度——不是局部容器
  true

# =============================================================================
# 测试
# =============================================================================

when isMainModule:
  echo "=== 维度引擎测试 ==="
  
  echo "\n--- 遏制链 (时间面向，倒着数) ---"
  let chain = restraintChain(3)
  for d in chain:
    echo "  ", label(d), if isRestrained(d): " (被遏制)" else: " (不被遏制)"
  
  echo "\n--- 判定 ---"
  echo "  n² 被 n¹ 遏制：", restrainedBy(2, 1)
  echo "  n¹ 被 n⁰ 遏制：", restrainedBy(1, 0)
  
  echo "\n--- 边界递增 (空间面向) ---"
  for d in 0..5:
    let b = newBoundary(Dimension(d))
    echo "  ", label(Dimension(d)), " → 生边界", boundaryCount(Dimension(d)), "个，只为自己活：", b.selfAlive
  
  echo "\n--- 正交/斜交 ---"
  for d in 0..10:
    let dd = Dimension(d)
    let tag = if dd.isOrthogonal(): "正交" elif dd.isOblique(): "斜交细节" else: "管辖层"
    echo "  ", label(dd), ": ", tag, " (细分倍率 ", dd.capacitySplit(), ")"
  
  echo "\n--- 整体观 ---"
  echo "  零维+时间 = 整体观：", hasWholeness(0, oriTime)
  echo "  一维+空间 = 整体观：", hasWholeness(1, oriSpace)
  echo "  五维+空间 = 整体观：", hasWholeness(5, oriSpace), " (高维只讲边界)"
  
  echo "\n--- 零维滴答遍历 ---"
  for beat in 0u64..3u64:
    echo "  滴答 ", beat, " → 遍历到 n^", traverse(beat)[^1]
  
  echo "\n--- 总维度 ---"
  for d in [0, 7, 1000]:
    echo "  ", label(Dimension(d)), " 讲总维度：", isTotalDimension(Dimension(d))
  
  echo "\n=== 测试完成 ==="
