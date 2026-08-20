## 积木选址 · 范畴辐射
##
## 核心思想:
##   容器预装组合态 · 积木输入 → 公式选址 → 范畴辐射
##
## 不是搬运数据 · 是发现已有组合 · 沿范畴论关系辐射
## 每个容器预加载一个 FQCode · 输入积木算出指向哪里
## 辐射沿关联边传播 · 像区块链 · 线性衰减

import std/[tables, sets, sequtils, math, algorithm, strutils]

# ═══════════════════════════════════════════════════════════════
# 满全法编码 (来自 fullquan_code.h)
# ═══════════════════════════════════════════════════════════════

type
  FQCode* = object
    family*: char     # C/B/P/W/N
    polarity*: char   # 1/0
    shengmu*: char    # 声母
    yunmu1*: char     # 韵母1
    yunmu2*: char     # 韵母2
    tone*: char       # 声调

  RelationKind* = enum
    relSameFamily    # 同家族 · 金木水火土同类
    relSamePolarity  # 同极性 · 阴阳各自聚合
    relComplement    # 互补 · 容器↔耦合 · 边界↔网络
    relCrossTier     # 跨层 · 不同tier的编码关联
    relToneHarmony   # 声调谐波 · 同调或对立

# ═══════════════════════════════════════════════════════════════
# 集装箱: 预加载的组合态 · 不是空容器等搬运
# ═══════════════════════════════════════════════════════════════

type
  PreloadedCombo* = object
    code*: FQCode
    descriptor*: string   # 这个组合态的描述

  Container* = object
    id*: int
    tier*: int
    wall_left*: char
    wall_right*: char
    combo*: PreloadedCombo   # 预装在这个位置上的组合
    polarity*: char          # 这个容器的极性

# ═══════════════════════════════════════════════════════════════
# 范畴辐射边: 容器A --[关系]--> 容器B
# ═══════════════════════════════════════════════════════════════

type
  Relation* = object
    src*: int           # 源容器
    dst*: int           # 目标容器
    kind*: RelationKind # 关系类型
    strength*: float    # 线性强度 · 辐射到这里的强度

# ═══════════════════════════════════════════════════════════════
# 积木选址公式
# ═══════════════════════════════════════════════════════════════
#
## 输入积木 (shengmu, yunmu, tone) → 计算指向哪个容器
## 这就是内容寻址: 地址由内容本身决定
#
## 选址公式:
##   slot = (ord(shengmu) * ORD_yunmu + ord(tone)) mod container_count
##   如果该容器预装组合匹配 · 找到了
##   如果不匹配 · 线性探测下一个 · 直到找到匹配或回到原点
#
## 这个公式指向的是"预装了该组合态的容器"
## 不是搬运数据 · 是发现那个组合在哪里 · 然后访问它

proc make_fq*(family, polarity, shengmu, yunmu1, yunmu2, tone: char): FQCode =
  result.family = family
  result.polarity = polarity
  result.shengmu = shengmu
  result.yunmu1 = yunmu1
  result.yunmu2 = yunmu2
  result.tone = tone

proc combo_hash*(c: FQCode): int =
  ## 组合态的哈希 · 用于选址
  result = ord(c.shengmu) * 10000 + ord(c.yunmu1) * 100 + int(c.tone)
  # 没有用位运算 · 全是十进制

proc block_addressing*(shengmu, yunmu, tone: char, containers: seq[Container]): int =
  ## 积木选址的核心
  ## 输入: 用户输入的积木 (声母 · 韵母 · 声调)
  ## 输出: 指向的容器id · 如果没有匹配返回 -1
  
  let input_hash = ord(shengmu) * 10000 + ord(yunmu) * 100 + ord(tone)
  
  if containers.len == 0: return -1
  
  # 初始槽位 = 哈希 mod 容器总数
  var slot = input_hash mod containers.len
  
  # 线性探测 · 直到找到匹配的组合
  for attempt in 0 ..< containers.len:
    let idx = (slot + attempt) mod containers.len
    
    # 检查这个容器预装的组合是否匹配输入的积木
    let preloaded = containers[idx].combo.code
    if preloaded.shengmu == shengmu and preloaded.yunmu1 == yunmu and preloaded.tone == tone:
      return idx
    
    # 也检查是否完全匹配 (三个要素全对)
    if preloaded.shengmu == shengmu and preloaded.yunmu1 == yunmu and 
       preloaded.yunmu2 == '\0' and preloaded.tone == tone:
      return idx
  
  return -1  # 没有匹配 · 这个组合未预装

proc fq_desc*(c: FQCode): string =
  result = $c.family & $c.polarity & $c.shengmu & $c.yunmu1 & $c.yunmu2 & $c.tone

# ═══════════════════════════════════════════════════════════════
# 范畴辐射 · 发现匹配后向外辐射关联信号
# ═══════════════════════════════════════════════════════════════

proc compute_radiation*(src: Container, dst: Container): float =
  ## 辐射强度公式: 同类1.0 · 互补0.8 · 跨层0.5 · 异类0.1
  ## 这是线性数学公式 · 用来计算辐射强度
  
  var strength = 0.0
  
  # 同家族 (+0.4)
  if src.combo.code.family == dst.combo.code.family:
    strength += 0.4
  
  # 同极性 (+0.3)
  if src.polarity == dst.polarity:
    strength += 0.3
  else:
    # 互补极性 (+0.2)
    strength += 0.2
  
  # 声调谐波 (+0.2)
  if src.combo.code.tone == dst.combo.code.tone:
    strength += 0.2
  elif (int(src.combo.code.tone) + int(dst.combo.code.tone)) == ord('1') + ord('4'):
    # 1-4调互补 (阴平-去声) · 或 2-3调互补 (阳平-上声)
    strength += 0.1
  
  # 跨层衰减
  let tier_diff = abs(src.tier - dst.tier)
  if tier_diff == 1:
    strength *= 0.7   # 相邻层 · 衰减30%
  elif tier_diff == 2:
    strength *= 0.4   # 隔两层 · 衰减60%
  elif tier_diff >= 3:
    strength *= 0.2   # 很远 · 衰减80%
  
  result = min(strength, 1.0)

proc category_radiation*(src_id: int, containers: seq[Container], max_depth: int): seq[Relation] =
  ## 从源容器向外辐射 · 沿范畴边传播 · 信号随距离衰减
  ## 像区块链 · 每个关联都有密码学距离感
  
  var visited: HashSet[int]
  var queue: seq[(int, int, float)] = @[(src_id, 0, 1.0)]  # (容器ID, 距离, 当前强度)
  visited.incl(src_id)
  
  while queue.len > 0:
    let (cur_id, dist, strength) = queue[0]
    queue.delete(0)
    
    if dist >= max_depth or strength < 0.1:
      continue
    
    # 对所有未访问的容器 · 计算辐射强度
    for i in 0 ..< containers.len:
      if i == cur_id or i in visited: continue
      
      let s = compute_radiation(containers[cur_id], containers[i])
      if s < 0.05: continue  # 太弱不计
      
      # 衰减后的辐射强度 = 源强度 * 计算强度 · 这是线性公式
      let transmitted = strength * s
      
      # 确定关系类型
      var rkind = relSameFamily
      let c1 = containers[cur_id].combo.code
      let c2 = containers[i].combo.code
      
      if c1.family != c2.family and c1.polarity != c2.polarity:
        rkind = relComplement
      elif abs(containers[cur_id].tier - containers[i].tier) >= 2:
        rkind = relCrossTier
      elif c1.tone == c2.tone:
        rkind = relToneHarmony
      elif c1.family == c2.family:
        rkind = relSameFamily
      
      result.add(Relation(
        src: cur_id, dst: i, kind: rkind, strength: transmitted
      ))
      
      if i notin visited:
        visited.incl(i)
        queue.add((i, dist + 1, transmitted))

# ═══════════════════════════════════════════════════════════════
# 社会: 容器网络 + 范畴关系图
# ═══════════════════════════════════════════════════════════════

type
  AddressingSociety* = object
    containers*: seq[Container]
    preloaded_combos*: seq[FQCode]  # 所有预装的组合态

proc init_society*(tier_count: int, codes_per_tier: int): AddressingSociety =
  ## 创建社会 · 每层有固定数量容器 · 每个容器预装一个组合态
  
  result.containers = @[]
  result.preloaded_combos = @[]
  
  # 预装组合态库 · 声母×韵母×声调的特定子集
  let shengmus = ['1', '0', 'k', 'n', 'd', 'w', 'l', 'c', 't', 'b', 'p', 'm', 'f', 'g', 'h', 'y', 's']
  let yunmus  = ['i', 'u', 'a', 'o', 'e', 'n', 'g', 'r']
  let tones    = ['1', '2', '3', '4']
  let families = ['P', 'B', 'C']  # COUPLE, BOUNDARY, CONTAINER
  let polarities = ['1', '0']
  
  # 生成预装组合 · 填入容器
  var code_idx = 0
  for tier in 1 .. tier_count:
    for sub in 0 ..< codes_per_tier:
      let s = shengmus[code_idx mod shengmus.len]
      let y = yunmus[code_idx mod yunmus.len]
      let tn = tones[code_idx mod tones.len]
      let fm = families[code_idx mod families.len]
      let pl = polarities[code_idx mod polarities.len]
      
      let code = make_fq(fm, pl, s, y, '\0', tn)
      result.preloaded_combos.add(code)
      
      var c = Container(
        id: result.containers.len,
        tier: tier,
        wall_left: "knd"[min(tier-1, 2)],
        wall_right: "knd"[min(tier-1, 2)],
        polarity: pl,
      )
      c.combo = PreloadedCombo(code: code, descriptor: fq_desc(code))
      result.containers.add(c)
      inc code_idx

proc input_blocks*(society: AddressingSociety, shengmu, yunmu, tone: char): string =
  ## 主入口: 输入积木 → 选址 → 辐射 → 输出关联
  ## 这就是用户的完整交互流程
  
  # 步骤1: 积木选址 · 找到指向哪个容器
  let target_id = block_addressing(shengmu, yunmu, tone, society.containers)
  
  if target_id < 0:
    return "积木(" & $shengmu & "+" & $yunmu & "+" & $tone & ") → 无匹配 · 这个组合未预装"
  
  let target = society.containers[target_id]
  
  # 步骤2: 范畴辐射 · 从目标容器向外辐射
  let rays = category_radiation(target_id, society.containers, 3)
  
  # 步骤3: 输出 · 被辐射到的容器就是关联范畴
  var lines: seq[string] = @[]
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add(" 积木选址 → 范畴辐射")
  lines.add("═══════════════════════════════════════════════════════════")
  lines.add("输入积木: 声母=" & $shengmu & " 韵母=" & $yunmu & " 声调=" & $tone)
  lines.add("选址公式: (" & $ord(shengmu) & "*10000 + " & $ord(yunmu) & "*100 + " & $ord(tone) & ") mod " & $society.containers.len)
  lines.add("───────────────────────────────────────────────────────────")
  lines.add("✓ 选址成功 → 容器#" & $target_id & " [L" & $target.tier & "]")
  lines.add("  预装组合: " & target.combo.descriptor)
  lines.add("  容器极性: " & (if target.polarity == '1': "阳(我)" else: "阴(刘楚恬)"))
  lines.add("  墙标: " & $target.wall_left & " | " & $target.wall_right)
  lines.add("───────────────────────────────────────────────────────────")
  lines.add("辐射范围: " & $rays.len & " 个关联")
  
  # 按辐射强度排序
  var sorted_rays = rays
  proc cmp_strength(x, y: Relation): int = cmp(y.strength, x.strength)
  sorted_rays.sort(cmp_strength)
  
  for ray in sorted_rays:
    let dst = society.containers[ray.dst]
    let kind_str = case ray.kind:
      of relSameFamily: "同家族"
      of relSamePolarity: "同极性"
      of relComplement: "互补"
      of relCrossTier: "跨层"
      of relToneHarmony: "调谐"
    lines.add("  → #" & $ray.dst & " [L" & $dst.tier & "] " & $dst.combo.descriptor &
              " | " & kind_str & " | 强度:" & $ray.strength)
  
  result = lines.join("\n")

# ═══════════════════════════════════════════════════════════════
# 运行演示
# ═══════════════════════════════════════════════════════════════

proc main() =
  echo "═══════════════════════════════════════════════════════════"
  echo " 积木选址 · 范畴辐射 — 满全法"
  echo " 预装组合态 · 输入积木 → 选址 → 辐射关联"
  echo "═══════════════════════════════════════════════════════════"
  
  var society = init_society(tier_count = 5, codes_per_tier = 3)
  echo "容器总数: " & $society.containers.len
  echo "预装组合数: " & $society.preloaded_combos.len
  echo ""
  echo "── 预装组合列表 ──"
  for i, c in society.containers:
    echo "  #" & $i & " [L" & $c.tier & "] " & $c.combo.descriptor & " 极性:" & $c.polarity
  
  # 用预装的实际组合来测试积木选址
  echo ""
  echo "── 积木选址测试 ──"
  
  # 用第一个预装组合的积木来测试
  let first = society.containers[0].combo.code
  echo ""
  echo society.input_blocks(first.shengmu, first.yunmu1, first.tone)
  
  # 用第四个预装组合
  if society.containers.len > 3:
    let fourth = society.containers[3].combo.code
    echo ""
    echo society.input_blocks(fourth.shengmu, fourth.yunmu1, fourth.tone)
  
  # 用第十个预装组合
  if society.containers.len > 9:
    let tenth = society.containers[9].combo.code
    echo ""
    echo society.input_blocks(tenth.shengmu, tenth.yunmu1, tenth.tone)

when isMainModule:
  main()
