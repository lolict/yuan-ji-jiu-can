## 满全法生命社会模拟 - Nim 实现
##
## 不是人工智能 · 是规则涌现出的社会分工智慧
##
## 定者=墙=裁判 · 动者=流=搬运 · 容器是管辖 · knd是边界

import std/[tables, times, strutils]

const
  TIER_ANCHOR = 1
  TIER_FUSION = 2
  TIER_TIME   = 3
  TIER_SPACE  = 4
  TIER_MATTER = 5

  PHASE_BIRTH  = 1
  PHASE_WORK   = 2
  PHASE_REWARD = 3
  PHASE_DEATH  = 4
  PHASE_WAIT   = 5

type
  RoleKind = enum
    roleDing
    roleDong

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

  Container = object
    id: int
    tier: int
    left_wall: char
    right_wall: char
    content: string
    owner: string
    sealed: bool

  Society = object
    roles: Table[string, LifeRole]
    containers: seq[Container]
    tick_counter: int

# 定者工作: 观察
proc ding_work(role: LifeRole, containers: seq[Container]): string =
  var watched = 0
  var sealed_count = 0
  var content_sum = 0
  for cid in role.territory.container_ids:
    if cid < containers.len:
      inc watched
      if containers[cid].sealed:
        inc sealed_count
      content_sum += containers[cid].content.len
  return "定者观:" & $watched & " 封:" & $sealed_count & " 内容:" & $content_sum

# 动者工作: 从同层取内容 · 送到高层
proc dong_work(role: LifeRole, containers: var seq[Container]): string =
  let my_tier = role.territory.tier
  
  var src_id = -1
  var src_tier = 0
  
  # 优先同层
  for cid in role.territory.container_ids:
    if cid < containers.len and containers[cid].content.len > 0:
      if containers[cid].tier == my_tier:
        src_id = cid
        src_tier = my_tier
        break
  
  # 其次低层
  if src_id < 0:
    for cid in role.territory.container_ids:
      if cid < containers.len and containers[cid].content.len > 0:
        if containers[cid].tier > src_tier:
          src_id = cid
          src_tier = containers[cid].tier
  
  if src_id < 0:
    return "动者无物可搬"
  
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
          if containers[cid].content.len < containers[src_id].content.len:
            dst_id = cid
            break
  
  if dst_id < 0:
    return "动者无路可走"
  
  # 搬运
  let payload = containers[src_id].content[0]
  containers[dst_id].content.add(payload)
  containers[src_id].content.delete(0, 0)
  return "动者搬运:" & $payload & " (tier" & $src_tier & "→tier" & $containers[dst_id].tier & ")"

# 生命周期
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

# 轮流执政
proc coordinate(society: var Society): string =
  let active_count = society.roles.len
  if active_count == 0:
    return ""
  
  let tick = society.tick_counter
  let role_idx = tick mod active_count
  
  var keys: seq[string] = @[]
  for k in society.roles.keys:
    keys.add(k)
  
  if role_idx < keys.len:
    let key = keys[role_idx]
    var role = society.roles[key]
    let log = lifecycle_tick(role, society.containers)
    society.roles[key] = role
    if log.len > 0:
      return log
  return ""

# 分配管辖
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

# 创建社会
proc create_society(): Society =
  result.tick_counter = 0
  
  # 5层容器 · 每层2个
  for tier in 1..5:
    for sub in 0..1:
      var init_content = ""
      if tier == 1 and sub == 0:
        init_content = "一二三"
      var c = Container(
        id: result.containers.len,
        tier: tier,
        left_wall: 'k',
        right_wall: 'd',
        content: init_content,
      )
      result.containers.add(c)
  
  # 定者
  for tier in 1..5:
    var role = LifeRole(
      id: "ding_" & $tier,
      name: case tier:
        of 1: "锚定者"
        of 2: "融合定者"
        of 3: "时间定者"
        of 4: "空间定者"
        of 5: "物质定者"
        else: "定者" & $tier,
      kind: roleDing,
      territory: Territory(tier: tier, knd_wall: char(ord('a') + tier)),
      phase: PHASE_WAIT,
      energy: 100,
    )
    result.roles[role.id] = role
  
  # 动者
  for tier in 1..5:
    var role = LifeRole(
      id: "dong_" & $tier,
      name: case tier:
        of 1: "锚动者"
        of 2: "融动者"
        of 3: "时动者"
        of 4: "空动者"
        of 5: "质动者"
        else: "动者" & $tier,
      kind: roleDong,
      territory: Territory(tier: tier, knd_wall: char(ord('A') + tier)),
      phase: PHASE_WAIT,
      energy: 100,
    )
    result.roles[role.id] = role
  
  # 分配管辖
  for key in result.roles.keys:
    var role = result.roles[key]
    assign_territory(role, result.containers)
    result.roles[key] = role

# ═══════════════════════════════════════════════════════════════
proc main() =
  var society = create_society()
  
  echo "══════════════════════════════════════════════════"
  echo " 满全法生命社会模拟 (Nim 实现)"
  echo " 角色: " & $society.roles.len & " | 容器: " & $society.containers.len
  echo "══════════════════════════════════════════════════"
  
  for tick in 0..79:
    society.tick_counter = tick
    let log = coordinate(society)
    if log.len > 0 and (tick mod 8 == 0 or (log.find("搬运") >= 0 and log.find("失败") < 0)):
      echo "[tick " & $tick & "] " & log
  
  echo "══════════════════════════════════════════════════"
  echo " 最终状态"
  echo "══════════════════════════════════════════════════"
  for key, role in society.roles:
    echo " " & role.name & 
         " | 工作:" & $role.work_count & 
         " | 奖励:" & $role.reward_score

when isMainModule:
  main()
