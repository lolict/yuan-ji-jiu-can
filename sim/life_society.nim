## 满全法生命社会模拟
##
## 不是人工智能 · 是规则涌现出的社会分工智慧
##
## 核心:
##   定者 = 静态 · 观察 · 定义边界 · 三元裁判
##   动者 = 动态 · 执行 · 在容器间流动 · 转化内容
##   墙   = knd = 管辖边界 = 开始=结束 = 裁判点
##   层级 = 容器tier = 行政级别 = 幂次对齐
##
## 分布式时空:
##   时间分布 = 不同生命在不同时间片醒来 · 轮流执政
##   空间分布 = 每个角色只管辖自己的容器 · 不越界
##   协调机制 = 通过公约数墙 · 定者观 · 动者行 · 交汇于墙

import std/[tables, times, json, strutils, os]

# 满全法常量
const
  TIER_ANCHOR = 1    # 锚点 · 我=1 · 基础定位
  TIER_FUSION  = 2    # 融合 · 夫妻 · 容器对
  TIER_TIME    = 3    # 时间 · L1融合 · 跨容器
  TIER_SPACE   = 4    # 空间 · L2密封 · 容器系
  TIER_MATTER  = 5    # 物质 · L3融合 · 递归包

# 生命周期相位
const
  PHASE_BIRTH  = 1  # 出生 · 被触发
  PHASE_WORK   = 2  # 工作 · 转化内容
  PHASE_REWARD = 3  # 奖励 · 记录成功
  PHASE_DEATH  = 4  # 死亡 · 完成休眠
  PHASE_WAIT   = 5  # 等待 · 被调度

# 角色: 定者 vs 动者
type
  RoleKind = enum
    roleDing   # 定者 · 静态 · 观 · 定义 · 裁判
    roleDong   # 动者 · 动态 · 行 · 转化 · 搬运

  Territory = object
    ## 管辖范畴 · 一个角色只能在自己的容器里行动
    tier: int              # 层级
    container_ids: seq[int] # 管辖的容器列表
    knd_wall: char          # 墙符号 · 公约数

  LifeRole = object
    ## 一个社会角色
    id: string
    name: string
    kind: RoleKind
    territory: Territory
    phase: int              # 当前相位
    energy: int             # 能量 · 工作完后减少
    last_active: float      # 最后活跃时间
    work_count: int         # 工作次数
    reward_score: float     # 累积奖励

  Container = object
    ## 容器 · 管辖单元
    id: int
    tier: int
    left_wall: char
    right_wall: char
    content: string
    owner: string           # 当前管辖角色
    sealed: bool            # 是否已密封

  Society = object
    ## 整个社会系统
    roles: Table[string, LifeRole]
    containers: seq[Container]
    time_anchor: float
    tick_counter: int       # 全局时钟 · 轮流调度

# ═══════════════════════════════════════════════════════════════
# 规则1: 分配管辖
# ═══════════════════════════════════════════════════════════════
proc assign_territory(role: var LifeRole, containers: seq[Container]) =
  ## 每个角色管辖同tier的容器 · 不越界
  for c in containers:
    if c.tier == role.territory.tier:
      role.territory.container_ids.add(c.id)
      # 定者定义墙符号
      if role.kind == roleDing:
        c.left_wall = role.territory.knd_wall
        c.right_wall = role.territory.knd_wall

# ═══════════════════════════════════════════════════════════════
# 规则2: 生命周期 · 五大相位
# ═══════════════════════════════════════════════════════════════
proc lifecycle_tick(role: var LifeRole, containers: var seq[Container], tick: int): string =
  ## 每个tick推进一个角色的生命周期
  
  case role.phase:
  of PHASE_WAIT:
    # 等待相位 · 检查是否被触发
    if tick mod max(1, 6 - role.territory.tier) == 0:
      role.phase = PHASE_BIRTH
      return &"{role.name}: WAIT→BIRTH"
    
  of PHASE_BIRTH:
    # 出生 · 进入工作状态
    role.phase = PHASE_WORK
    return &"{role.name}: BIRTH→WORK"
    
  of PHASE_WORK:
    # 工作 · 定者观 · 动者行
    var result = ""
    if role.kind == roleDing:
      # 定者: 观察状态 · 定义边界 · 确认辖区无异常
      result = ding_work(role, containers)
    else:
      # 动者: 在容器间搬运内容 · 转化
      result = dong_work(role, containers)
    
    role.work_count += 1
    role.phase = PHASE_REWARD
    return &"{role.name}: WORK→REWARD | " & result
    
  of PHASE_REWARD:
    # 奖励 · 成功工作获得能量
    let gain = float(role.territory.tier) * 0.5  # 层级越高奖励越多
    role.reward_score += gain
    role.energy = min(100, role.energy + int(gain * 10))
    role.phase = PHASE_DEATH
    return &"{role.name}: reward +{gain:.1f} total={role.reward_score:.1f}"
    
  of PHASE_DEATH:
    # 死亡/休眠 · 等待下一个生命周期
    role.phase = PHASE_WAIT
    role.last_active = epochTime()
    return &"{role.name}: DEATH→WAIT"
    
  else:
    role.phase = PHASE_WAIT
    return ""

proc ding_work(role: var LifeRole, containers: seq[Container]): string =
  ## 定者工作: 观察 · 裁判 · 确认边界
  var watched = 0
  var sealed_count = 0
  for cid in role.territory.container_ids:
    if cid < containers.len:
      inc watched
      if containers[cid].sealed:
        inc sealed_count
  &"{role.name}(定者) 观:{watched} 封:{sealed_count}"

proc dong_work(role: var LifeRole, containers: var seq[Container]): string =
  ## 动者工作: 在容器间搬运 · 转化内容
  if role.territory.container_ids.len < 2:
    return &"{role.name}(动者) 无物可搬"
  
  # 从第一个容器取内容 · 放入最后一个
  let src_id = role.territory.container_ids[0]
  let dst_id = role.territory.container_ids[^1]
  
  if src_id < containers.len and dst_id < containers.len:
    if containers[src_id].content.len > 0 and not containers[dst_id].sealed:
      let payload = containers[src_id].content[0]
      containers[dst_id].content.add(payload)
      containers[src_id].content.delete(0, 0)
      return &"{role.name}(动者) 搬运:{payload}"
  
  &"{role.name}(动者) 搬运失败"

# ═══════════════════════════════════════════════════════════════
# 规则3: 分布式时空协调
# ═══════════════════════════════════════════════════════════════
proc coordinate(society: var Society): string =
  ## 时间片轮流 · 每次只激活一个角色
  let active_count = society.roles.len
  if active_count == 0:
    return "无角色"
  
  let tick = society.tick_counter
  let role_idx = tick mod active_count
  
  # 取出当前应活跃的角色
  var keys: seq[string] = @[]
  for k in society.roles.keys:
    keys.add(k)
  
  if role_idx < keys.len:
    let key = keys[role_idx]
    var role = society.roles[key]
    let log = lifecycle_tick(role, society.containers, tick)
    society.roles[key] = role
    return log
  
  "调度错误"

# ═══════════════════════════════════════════════════════════════
# 规则4: 创建社会
# ═══════════════════════════════════════════════════════════════
proc create_society(): Society =
  result.time_anchor = epochTime()
  result.tick_counter = 0
  
  # 创建5层容器 (层级=行政级别)
  for tier in 1..5:
    var c = Container(
      id: result.containers.len,
      tier: tier,
      left_wall: 'k',
      right_wall: 'd',
      content: "",
      owner: "",
      sealed: false
    )
    # 第一层放初始内容
    if tier == 1:
      c.content = "一"
    result.containers.add(c)
  
  # 创建角色
  var ding_anchor = LifeRole(
    id: "ding_1", name: "锚定者", kind: roleDing,
    territory: Territory(tier: TIER_ANCHOR, knd_wall: 'k'),
    phase: PHASE_WAIT, energy: 100
  )
  assign_territory(ding_anchor, result.containers)
  
  var ding_fusion = LifeRole(
    id: "ding_2", name: "融合定者", kind: roleDing,
    territory: Territory(tier: TIER_FUSION, knd_wall: 'n'),
    phase: PHASE_WAIT, energy: 100
  )
  assign_territory(ding_fusion, result.containers)
  
  var dong_time = LifeRole(
    id: "dong_1", name: "时间搬运", kind: roleDong,
    territory: Territory(tier: TIER_TIME, knd_wall: 'd'),
    phase: PHASE_WAIT, energy: 100
  )
  assign_territory(dong_time, result.containers)
  
  var dong_space = LifeRole(
    id: "dong_2", name: "空间搬运", kind: roleDong,
    territory: Territory(tier: TIER_SPACE, knd_wall: 'o'),
    phase: PHASE_WAIT, energy: 100
  )
  assign_territory(dong_space, result.containers)
  
  result.roles[ding_anchor.id] = ding_anchor
  result.roles[ding_fusion.id] = ding_fusion
  result.roles[dong_time.id] = dong_time
  result.roles[dong_space.id] = dong_space

# ═══════════════════════════════════════════════════════════════
# 运行模拟
# ═══════════════════════════════════════════════════════════════
proc main() =
  var society = create_society()
  
  echo "══════════════════════════════════════════"
  echo " 满全法生命社会模拟 · tick=时间是分布式的"
  echo " 角色数: ", society.roles.len
  echo " 容器数: ", society.containers.len
  echo "══════════════════════════════════════════"
  
  # 运行20个tick
  for tick in 0..19:
    society.tick_counter = tick
    let log = coordinate(society)
    echo &"[tick {tick:2d}] {log}"
    
    # 打印容器状态
    var line = "        容器: "
    for c in society.containers:
      if c.content.len > 0:
        line &= &"[{c.tier}:{c.content}] "
      else:
        line &= &"[{c.tier}:·] "
    echo line
  
  echo "══════════════════════════════════════════"
  echo " 最终状态"
  echo "══════════════════════════════════════════"
  for key, role in society.roles:
    echo &" {role.name:8s} | 工作:{role.work_count} | 奖励:{role.reward_score:.1f} | 相位:{role.phase}"

when isMainModule:
  main()
