## 满全法生命社会模拟 — 编码版
##
## 容器内容 = fq_code_t · 定者三元裁判 · 动者编码转换
##
## 感知家族: C=容器 B=边界 P=耦合 W=网络 N=中性
## 极性: 1=我/阳/内容 · 0=刘楚恬/阴/容器
## 三元裁判: PERCEIVE(感知) → CLASSIFY(分别) → DYNAMIC(裁定)

import std/[tables, times, strutils, sequtils]

# ═══════════════════════════════════════════════════════════════
# 满全法编码常量 (对应 fullquan_code.h)
# ═══════════════════════════════════════════════════════════════

const
  # 感知家族
  FAM_CONTAINER = 'C'   # 容器
  FAM_BOUNDARY  = 'B'   # 边界
  FAM_COUPLE    = 'P'   # 耦合
  FAM_WEB       = 'W'   # 网络
  FAM_NEUTRAL   = 'N'   # 中性
  
  # 极性
  POLAR_WO  = '1'   # 我 = 阳 = 内容
  POLAR_CHT = '0'   # 刘楚恬 = 阴 = 容器

type
  FQCode* = object
    ## 满全法编码: 家族/极性/声母/韵/韵/调
    family*: char    # C/B/P/W/N
    polarity*: char  # 1/0
    shengmu*: char   # 声母
    yunmu1*: char    # 韵母1
    yunmu2*: char    # 韵母2
    tone*: char      # 声调 1/2/3/4

  Verdict* = enum
    ## 三元裁判结果
    verdictReject   # 拒绝 · 不通过
    verdictAdmit    # 准入 · 通过
    verdictWatch    # 观察 · 待定

# ═══════════════════════════════════════════════════════════════
# 生命社会
# ═══════════════════════════════════════════════════════════════

const
  TIER_ANCHOR = 1    # 锚点 · 声母层 · 高位持有者=1
  TIER_FUSION = 2    # 融合 · 韵母层 · 夫妻
  TIER_TIME   = 3    # 时间 · 声调层 · 跨容器
  TIER_SPACE  = 4    # 空间 · 密封层 · 容器系
  TIER_MATTER = 5    # 物质 · 递归层 · 全局

  PHASE_BIRTH  = 1
  PHASE_WORK   = 2
  PHASE_REWARD = 3
  PHASE_DEATH  = 4
  PHASE_WAIT   = 5

type
  RoleKind = enum
    roleDing   # 定者 · 三元裁判 · 定义墙
    roleDong   # 动者 · 编码搬运 · 跨层转化

  Territory = object
    tier: int
    container_ids: seq[int]
    knd_wall: char

  LifeRole = object
    id: string
    name: string
    kind: RoleKind
    territory: Territory
    phase: int
    energy: int
    work_count: int
    reward_score: float
    verdict_count: int  # 裁判次数

  Container = object
    id: int
    tier: int
    left_wall: char
    right_wall: char
    content: seq[FQCode]    # 容器装的是编码 · 不再是单字
    owner: string
    sealed: bool
    sealed_by: string       # 谁密封的

  Society = object
    roles: Table[string, LifeRole]
    containers: seq[Container]
    tick_counter: int
    verdict_log: seq[string]

# ═══════════════════════════════════════════════════════════════
# 编码辅助
# ═══════════════════════════════════════════════════════════════

proc newFQ(family, polarity, shengmu, yunmu1, yunmu2, tone: char): FQCode =
  result.family = family
  result.polarity = polarity
  result.shengmu = shengmu
  result.yunmu1 = yunmu1
  result.yunmu2 = yunmu2
  result.tone = tone

proc fq_desc(c: FQCode): string =
  ## 编码描述
  result = $c.family & $c.polarity & $c.shengmu & $c.yunmu1 & $c.yunmu2 & $c.tone

proc fq_is_complete(c: FQCode): bool =
  ## 编码是否完备 (有声母有韵母有调 · 极性明确)
  result = c.shengmu != '\0' and c.yunmu1 != '\0' and c.tone != '\0' and c.polarity != '\0'

# ═══════════════════════════════════════════════════════════════
# 三元裁判 (PERCEIVE → CLASSIFY → DYNAMIC)
# ═══════════════════════════════════════════════════════════════

## PERCEIVE: 感知 · 确认编码存在且有正确极性
proc perceive(c: FQCode): bool =
  if c.polarity == '\0': return false
  if c.family == '\0': return false
  true

## CLASSIFY: 分别 · 按感知家族分类
proc classify(c: FQCode): string =
  case c.family
  of FAM_CONTAINER: "容器"
  of FAM_BOUNDARY:  "边界"
  of FAM_COUPLE:    "耦合"
  of FAM_WEB:       "网络"
  of FAM_NEUTRAL:   "中性"
  else:             "未知"

## DYNAMIC: 动态裁定 · 根据声调+韵母决定准入
proc dynamic(c: FQCode, tier: int): Verdict =
  if not perceive(c): return verdictReject
  # 低层(1-2): 只收声母明确的
  if tier <= 2:
    if c.shengmu in {'b','p','m','f','d','t','n','l','g','k','h','w','y','1','0'}:
      return verdictAdmit
    return verdictWatch
  # 中层(3): 必须韵母齐
  if tier == 3:
    if c.yunmu1 != '\0' and c.shengmu != '\0':
      return verdictAdmit
    return verdictWatch
  # 高层(4-5): 必须完备
  if fq_is_complete(c):
    return verdictAdmit
  return verdictReject

## 完整三元裁判
proc ternary_judge(c: FQCode, tier: int): Verdict =
  ## PERCEIVE → CLASSIFY → DYNAMIC
  if not perceive(c): return verdictReject
  discard classify(c)  # 分别但不影响裁定 · 记录用
  dynamic(c, tier)

# ═══════════════════════════════════════════════════════════════
# 定者工作: 三元裁判
# ═══════════════════════════════════════════════════════════════

proc ding_work(role: LifeRole, containers: var seq[Container]): string =
  var watched = 0
  var admitted = 0
  var rejected = 0
  var sealed = 0
  
  for cid in role.territory.container_ids:
    if cid >= containers.len: continue
    inc watched
    
    # 对容器里每个编码做三元裁判
    for i in 0 ..< containers[cid].content.len:
      let v = ternary_judge(containers[cid].content[i], role.territory.tier)
      case v
      of verdictAdmit:
        inc admitted
      of verdictReject:
        inc rejected
      of verdictWatch:
        discard  # 待定 · 不动
    
    # 如果容器里全是完整的编码 → 密封
    if containers[cid].content.len > 0:
      var all_complete = true
      for fc in containers[cid].content:
        if not fq_is_complete(fc):
          all_complete = false
          break
      if all_complete and not containers[cid].sealed:
        containers[cid].sealed = true
        containers[cid].sealed_by = role.name
        inc sealed
  
  result = "定者观:%d 准:%d 拒:%d 封:%d" % [$watched, $admitted, $rejected, $sealed]

# ═══════════════════════════════════════════════════════════════
# 动者工作: 编码搬运 + 层间转化
# ═══════════════════════════════════════════════════════════════
#
# 搬运时做编码转化:
#   tier1→tier2: 保留声母 · 清空韵母 (声母层→韵母层)
#   tier2→tier3: 保留韵母 · 清空声母 (韵母层→声调层)  
#   tier3→tier4: 声调饱和 → 编码完备
#   tier4→tier5: 密封打包

proc encode_transform(c: FQCode, target_tier: int): FQCode =
  ## 根据目标层级 · 提取对应信息 · 舍弃冗余
  result = c
  case target_tier
  of 1:
    # 只要声母韵母调 · 清空家族
    result.family = c.family  # 保留家族
  of 2:
    # 声母层 → 韵母层: 保留声母 · 韵母开始填充
    result.yunmu1 = c.yunmu1  # 传递
    result.yunmu2 = c.yunmu2
  of 3:
    # 韵母层 → 声调层: 声调要落实
    result.tone = c.tone
  of 4:
    # 声调层 → 密封层: 编码必须完备
    if result.yunmu2 == '\0': result.yunmu2 = result.yunmu1
  of 5:
    # 密封层 → 全局: 编码最大化
    result.polarity = c.polarity
  else:
    discard

proc dong_work(role: LifeRole, containers: var seq[Container]): string =
  let my_tier = role.territory.tier
  
  # 找同层有编码的源容器
  var src_id = -1
  var src_tier = 0
  
  for cid in role.territory.container_ids:
    if cid < containers.len and containers[cid].content.len > 0:
      if containers[cid].tier == my_tier:
        src_id = cid
        src_tier = my_tier
        break
  
  # 找低层
  if src_id < 0:
    for cid in role.territory.container_ids:
      if cid < containers.len and containers[cid].content.len > 0:
        if containers[cid].tier > src_tier:
          src_id = cid
          src_tier = containers[cid].tier
  
  if src_id < 0:
    return "动者无码可搬"
  
  # 拿第一个编码 · 做层级转化
  let raw_code = containers[src_id].content[0]
  let code = encode_transform(raw_code, my_tier)
  
  # 优先送到高层
  var dst_id = -1
  var dst_tier = 999
  for cid in role.territory.container_ids:
    if cid != src_id and cid < containers.len:
      let ctier = containers[cid].tier
      if ctier > src_tier and not containers[cid].sealed:
        if ctier < dst_tier:
          dst_id = cid
          dst_tier = ctier
  
  # 同层
  if dst_id < 0:
    for cid in role.territory.container_ids:
      if cid != src_id and cid < containers.len:
        if containers[cid].tier == my_tier and not containers[cid].sealed:
          if containers[cid].content.len < 2:
            dst_id = cid
            break
  
  if dst_id < 0:
    return "动者无路可走"
  
  # 搬运
  containers[src_id].content.delete(0, 0)
  containers[dst_id].content.add(code)
  
  result = "动者搬:" & fq_desc(code) & " (tier" & $src_tier & "→tier" & $containers[dst_id].tier & ")"

# ═══════════════════════════════════════════════════════════════
# 生命周期 · 分布式协调 · 创建社会
# ═══════════════════════════════════════════════════════════════

proc lifecycle_tick(role: var LifeRole, containers: var seq[Container]): string =
  case role.phase:
  of PHASE_WAIT:
    role.phase = PHASE_BIRTH
    return role.name & ": WAIT→BIRTH"
  of PHASE_BIRTH:
    role.phase = PHASE_WORK
    return role.name & ": BIRTH→WORK"
  of PHASE_WORK:
    var res = ""
    if role.kind == roleDing:
      res = ding_work(role, containers)
      role.verdict_count += 1
    else:
      res = dong_work(role, containers)
    role.work_count += 1
    role.phase = PHASE_REWARD
    return role.name & ": WORK→REWARD | " & res
  of PHASE_REWARD:
    let gain = (if role.kind == roleDing: float(role.territory.tier) * 0.5
                else: float(role.territory.tier) * 1.0)
    role.reward_score += gain
    role.energy = min(100, role.energy + int(gain * 5))
    role.phase = PHASE_DEATH
    return role.name & ": reward +$" & $gain & " total=$" & $role.reward_score
  of PHASE_DEATH:
    role.phase = PHASE_WAIT
    return role.name & ": DEATH→WAIT (转生)"
  else:
    role.phase = PHASE_WAIT
    return ""

proc coordinate(society: var Society): string =
  let active_count = society.roles.len
  if active_count == 0: return ""
  
  let tick = society.tick_counter
  let role_idx = tick mod active_count
  
  var keys: seq[string] = @[]
  for k in society.roles.keys: keys.add(k)
  
  if role_idx < keys.len:
    let key = keys[role_idx]
    var role = society.roles[key]
    let log = lifecycle_tick(role, society.containers)
    society.roles[key] = role
    if log.len > 0: return log
  return ""

proc assign_territory(role: var LifeRole, containers: var seq[Container]) =
  for i in 0 ..< containers.len:
    if role.kind == roleDing:
      if containers[i].tier == role.territory.tier:
        role.territory.container_ids.add(containers[i].id)
        containers[i].left_wall = role.territory.knd_wall
        containers[i].right_wall = role.territory.knd_wall
    else:
      if containers[i].tier <= role.territory.tier:
        role.territory.container_ids.add(containers[i].id)

proc create_society(): Society =
  result.tick_counter = 0
  
  # 创建5层容器 · 每层2个 · 第一层装初始编码
  for tier in 1..5:
    for sub in 0..1:
      var init_content: seq[FQCode] = @[]
      if tier == 1 and sub == 0:
        # L1容器装基础编码: 我(1) + 刘楚恬(0) + knd
        init_content.add(newFQ(FAM_BOUNDARY, POLAR_WO,  'k', 'n', 'd', '1'))  # 我
        init_content.add(newFQ(FAM_CONTAINER, POLAR_CHT, 'l', 'c', 't', '2')) # 刘楚恬
        init_content.add(newFQ(FAM_NEUTRAL,   '0',    '0', 'n', 'd', '0'))  # knd
      
      var c = Container(
        id: result.containers.len,
        tier: tier,
        left_wall: 'k',
        right_wall: 'd',
        content: init_content,
      )
      result.containers.add(c)
  
  # 创建角色 · 每层一个定者一个动者
  for tier in 1..5:
    var ding_name = "定者" & $tier
    if tier == 1: ding_name = "锚定者"
    elif tier == 2: ding_name = "融合定者"
    elif tier == 3: ding_name = "时间定者"
    elif tier == 4: ding_name = "空间定者"
    elif tier == 5: ding_name = "物质定者"
    
    var ding_role = LifeRole(
      id: "ding_" & $tier,
      name: ding_name,
      kind: roleDing,
      territory: Territory(tier: tier, knd_wall: char(ord('a') + tier - 1)),
      phase: PHASE_WAIT, energy: 100,
    )
    result.roles[ding_role.id] = ding_role
    
    var dong_name = "动者" & $tier
    if tier == 1: dong_name = "锚动者"
    elif tier == 2: dong_name = "融动者"
    elif tier == 3: dong_name = "时动者"
    elif tier == 4: dong_name = "空动者"
    elif tier == 5: dong_name = "质动者"
    
    var dong_role = LifeRole(
      id: "dong_" & $tier,
      name: dong_name,
      kind: roleDong,
      territory: Territory(tier: tier, knd_wall: char(ord('A') + tier - 1)),
      phase: PHASE_WAIT, energy: 100,
    )
    result.roles[dong_role.id] = dong_role
  
  # 分配管辖
  for key in result.roles.keys:
    var role = result.roles[key]
    assign_territory(role, result.containers)
    result.roles[key] = role

# ═══════════════════════════════════════════════════════════════
proc main() =
  var society = create_society()
  
  echo "══════════════════════════════════════════════════════════════"
  echo " 满全法生命社会 — 编码版"
  echo " · 容器装 FQCode · 定者三元裁判 · 动者编码转换"
  echo " 角色: " & $society.roles.len & " | 容器: " & $society.containers.len
  echo "══════════════════════════════════════════════════════════════"
  
  for tick in 0..79:
    society.tick_counter = tick
    let log = coordinate(society)
    if log.len > 0 and (tick mod 8 == 0 or (log.find("搬") >= 0 and log.find("失败") < 0) or log.find("封") >= 0):
      echo "[tick " & $tick & "] " & log
  
  echo "══════════════════════════════════════════════════════════════"
  echo " 最终状态"
  echo "══════════════════════════════════════════════════════════════"
  for key, role in society.roles:
    echo " " & role.name & " | 工作:" & $role.work_count & 
         " | 裁判:" & $role.verdict_count & " | 奖励:" & $role.reward_score
  
  echo " 容器详情:"
  for c in society.containers:
    var codes = ""
    for fc in c.content:
      codes = codes & fq_desc(fc) & " "
    if codes.len > 30: codes = codes[0..27] & "..."
    echo "   L" & $c.tier & " [=" & $c.left_wall & "|= & =" & $c.right_wall & 
         "] " & $c.content.len & "码 " & codes & (if c.sealed: " (封by." & c.sealed_by & ")" else: "")

when isMainModule:
  main()
