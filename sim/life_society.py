"""
满全法生命社会模拟

不是人工智能 · 是规则涌现出的社会分工智慧

核心:
  定者 = 静态 · 观察 · 定义边界 · 三元裁判
  动者 = 动态 · 执行 · 在容器间流动 · 转化内容
  墙   = knd = 管辖边界 = 开始=结束 = 裁判点
  层级 = 容器tier = 行政级别 = 幂次对齐

分布式时空:
  时间分布 = 不同生命在不同时间片醒来 · 轮流执政
  空间分布 = 每个角色只管辖自己的容器 · 不越界
  协调机制 = 通过公约数墙 · 定者观 · 动者行 · 交汇于墙
  
智慧涌现:
  不是单个角色有智慧 · 是分工协作涌现出系统智能
  定者观状态 · 动者行搬运 · knd墙交汇 · 层级间信息流动
  低层产生内容 · 高层聚合抽象 · 分布式时空产生超越个体的智慧
"""

import time
from enum import Enum, IntEnum
from dataclasses import dataclass, field
from typing import List, Dict

# 满全法常量
class Tier(IntEnum):
    ANCHOR = 1    # 锚点 · 我=1 · 基础定位
    FUSION = 2    # 融合 · 夫妻 · 容器对
    TIME   = 3    # 时间 · L1融合 · 跨容器
    SPACE  = 4    # 空间 · L2密封 · 容器系
    MATTER = 5    # 物质 · L3融合 · 递归包

class Phase(IntEnum):
    BIRTH  = 1  # 出生 · 被触发
    WORK   = 2  # 工作 · 转化内容
    REWARD = 3  # 奖励 · 记录成功
    DEATH  = 4  # 死亡 · 完成休眠
    WAIT   = 5  # 等待 · 被调度

class RoleKind(Enum):
    DING = "定者"   # 静态 · 观 · 定义 · 裁判
    DONG = "动者"   # 动态 · 行 · 转化 · 搬运

@dataclass
class Territory:
    """管辖范畴 · 一个角色只能在自己的容器里行动"""
    tier: int = Tier.ANCHOR
    container_ids: List[int] = field(default_factory=list)
    knd_wall: str = 'k'

@dataclass
class LifeRole:
    """一个社会角色"""
    id: str
    name: str
    kind: RoleKind
    territory: Territory
    phase: int = Phase.WAIT
    energy: int = 100
    last_active: float = 0.0
    work_count: int = 0
    reward_score: float = 0.0

@dataclass
class Container:
    """容器 · 管辖单元"""
    id: int
    tier: int
    left_wall: str = 'k'
    right_wall: str = 'd'
    content: str = ""
    owner: str = ""
    sealed: bool = False

@dataclass
class Society:
    """整个社会系统"""
    roles: Dict[str, LifeRole] = field(default_factory=dict)
    containers: List[Container] = field(default_factory=list)
    time_anchor: float = 0.0
    tick_counter: int = 0

# ═══════════════════════════════════════════════════════════════
# 规则1: 分配管辖 (定者管辖同层 · 动者管辖同层及以下)
# ═══════════════════════════════════════════════════════════════
def assign_territory(role: LifeRole, containers: List[Container]):
    """定者只管辖同层 · 动者可触及同层及以下 (搬运需要跨层)"""
    for c in containers:
        if role.kind == RoleKind.DING:
            # 定者: 只看同层 · 定义墙
            if c.tier == role.territory.tier:
                role.territory.container_ids.append(c.id)
                c.left_wall = role.territory.knd_wall
                c.right_wall = role.territory.knd_wall
        else:
            # 动者: 同层及以下 · 可以跨层搬运
            if c.tier <= role.territory.tier:
                role.territory.container_ids.append(c.id)

# ═══════════════════════════════════════════════════════════════
# 规则2: 生命周期 · 五大相位
# ═══════════════════════════════════════════════════════════════
def lifecycle_tick(role: LifeRole, containers: List[Container], tick: int) -> str:
    """每个tick推进一个角色的整个生命周期 (WAIT→BIRTH→WORK→REWARD→DEATH→WAIT)"""
    
    if role.phase == Phase.WAIT:
        # 被调度 · 立即出生
        role.phase = Phase.BIRTH
        return "%s: WAIT→BIRTH" % role.name
    
    elif role.phase == Phase.BIRTH:
        # 出生 · 立即投入工作
        role.phase = Phase.WORK
        return "%s: BIRTH→WORK" % role.name
    
    elif role.phase == Phase.WORK:
        # 工作 · 定者观 · 动者行
        if role.kind == RoleKind.DING:
            result = ding_work(role, containers)
        else:
            result = dong_work(role, containers)
        role.work_count += 1
        role.phase = Phase.REWARD
        return "%s: WORK→REWARD | %s" % (role.name, result)
    
    elif role.phase == Phase.REWARD:
        # 奖励 · 层级越高奖励越多 · 定者稳定 · 动者耗体力
        if role.kind == RoleKind.DING:
            gain = float(role.territory.tier) * 0.5
        else:
            gain = float(role.territory.tier) * 1.0
        role.reward_score += gain
        role.energy = min(100, role.energy + int(gain * 5))
        role.phase = Phase.DEATH
        return "%s: reward +%.1f total=%.1f" % (role.name, gain, role.reward_score)
    
    elif role.phase == Phase.DEATH:
        # 死亡 · 回到等待 · 下一轮再转生
        role.phase = Phase.WAIT
        role.last_active = time.time()
        return "%s: DEATH→WAIT (转生)" % role.name
    
    else:
        role.phase = Phase.WAIT
        return ""

def ding_work(role: LifeRole, containers: List[Container]) -> str:
    """定者工作: 观察 · 裁判 · 确认边界"""
    watched = 0
    sealed_count = 0
    content_sum = 0
    for cid in role.territory.container_ids:
        if cid < len(containers):
            watched += 1
            if containers[cid].sealed:
                sealed_count += 1
            content_sum += len(containers[cid].content)
    return "定者观:%d 封:%d 内容:%d" % (watched, sealed_count, content_sum)

def dong_work(role: LifeRole, containers: List[Container]) -> str:
    """动者工作: 从自己层级取内容 · 送到上一层 (逐层递进)"""
    my_tier = role.territory.tier
    
    # 找到自己有内容的容器 (优先同层 · 其次低层)
    src_id = -1
    src_tier = 0
    # 先扫同层
    for cid in role.territory.container_ids:
        if cid < len(containers) and containers[cid].content:
            if containers[cid].tier == my_tier:
                src_id = cid
                src_tier = my_tier
                break
    # 再扫低层
    if src_id < 0:
        for cid in role.territory.container_ids:
            if cid < len(containers) and containers[cid].content:
                if containers[cid].tier > src_tier:
                    src_id = cid
                    src_tier = containers[cid].tier
    
    if src_id < 0:
        return "动者无物可搬"
    
    # 优先送到比自己层级高的容器 (低层→高层 = 汇聚)
    dst_id = -1
    dst_tier = 999
    for cid in role.territory.container_ids:
        if cid < len(containers) and cid != src_id:
            ctier = containers[cid].tier
            # 优先高层 · 其次同层 · 高层容器越空越优先
            if ctier > src_tier and not containers[cid].sealed:
                if ctier < dst_tier or (ctier == dst_tier and 
                    len(containers[cid].content) < (len(containers[dst_id].content) if dst_id >= 0 else 999)):
                    dst_id = cid
                    dst_tier = ctier
    
    # 如果没有高层可送 · 同层搬运
    if dst_id < 0:
        for cid in role.territory.container_ids:
            if cid < len(containers) and cid != src_id:
                if containers[cid].tier == role.territory.tier and not containers[cid].sealed:
                    if len(containers[cid].content) < len(containers[src_id].content):
                        dst_id = cid
                        break
    
    # 兜底 · 自己层任意空容器
    if dst_id < 0:
        for cid in role.territory.container_ids:
            if cid < len(containers) and cid != src_id:
                if containers[cid].tier == role.territory.tier and not containers[cid].sealed:
                    dst_id = cid
                    break
    
    if dst_id < 0:
        return "动者无路可走"
    
    # 搬运
    payload = containers[src_id].content[0]
    containers[dst_id].content += payload
    containers[src_id].content = containers[src_id].content[1:]
    return "动者搬:%s (tier%d→tier%d)" % (payload, src_tier, containers[dst_id].tier)

# ═══════════════════════════════════════════════════════════════
# 规则3: 分布式时空协调 (轮流执政 · 每个tick一个角色)
# ═══════════════════════════════════════════════════════════════
def coordinate(society: Society) -> str:
    """轮流执政 · tick mod 角色数 = 当前当值者"""
    active_count = len(society.roles)
    if active_count == 0:
        return ""
    
    tick = society.tick_counter
    role_idx = tick % active_count
    
    keys = list(society.roles.keys())
    if role_idx < len(keys):
        key = keys[role_idx]
        role = society.roles[key]
        log = lifecycle_tick(role, society.containers, tick)
        society.roles[key] = role
        return log
    
    return ""

# ═══════════════════════════════════════════════════════════════
# 规则4: 创建社会 · 五层容器 · 定者为墙 · 动者为流
# ═══════════════════════════════════════════════════════════════
def create_society() -> Society:
    society = Society(time_anchor=time.time())
    
    # 创建5层容器 · 每层的容器数 = 2^(tier-1) (幂次对齐)
    for tier in range(1, 6):
        count = 1 << (tier - 1) if tier <= 3 else 2  # tier1=1, tier2=2, tier3=4, tier4=2, tier5=2
        if count < 2:
            count = 2  # 至少2个 · 动者才能搬运
        for sub in range(count):
            c = Container(
                id=len(society.containers),
                tier=tier,
                left_wall='k',
                right_wall='d',
                content="一二三" if tier == 1 and sub == 0 else ("" if tier == 1 else ""),
                # L1容器初始有3个字 · 其他层空 · 等待被搬运上来
            )
            society.containers.append(c)
    
    # 创建角色 · 每层一个定者 · 每层一个动者
    roles = [
        LifeRole(id="ding_1", name="锚定者", kind=RoleKind.DING,
                 territory=Territory(tier=Tier.ANCHOR, knd_wall='k')),
        LifeRole(id="ding_2", name="融合定者", kind=RoleKind.DING,
                 territory=Territory(tier=Tier.FUSION, knd_wall='n')),
        LifeRole(id="ding_3", name="时间定者", kind=RoleKind.DING,
                 territory=Territory(tier=Tier.TIME, knd_wall='d')),
        LifeRole(id="ding_4", name="空间定者", kind=RoleKind.DING,
                 territory=Territory(tier=Tier.SPACE, knd_wall='o')),
        LifeRole(id="ding_5", name="物质定者", kind=RoleKind.DING,
                 territory=Territory(tier=Tier.MATTER, knd_wall='y')),
        LifeRole(id="dong_1", name="锚动者", kind=RoleKind.DONG,
                 territory=Territory(tier=Tier.ANCHOR, knd_wall='a')),
        LifeRole(id="dong_2", name="融动者", kind=RoleKind.DONG,
                 territory=Territory(tier=Tier.FUSION, knd_wall='b')),
        LifeRole(id="dong_3", name="时动者", kind=RoleKind.DONG,
                 territory=Territory(tier=Tier.TIME, knd_wall='c')),
        LifeRole(id="dong_4", name="空动者", kind=RoleKind.DONG,
                 territory=Territory(tier=Tier.SPACE, knd_wall='e')),
        LifeRole(id="dong_5", name="质动者", kind=RoleKind.DONG,
                 territory=Territory(tier=Tier.MATTER, knd_wall='f')),
    ]
    
    for role in roles:
        assign_territory(role, society.containers)
        society.roles[role.id] = role
    
    return society

# ═══════════════════════════════════════════════════════════════
# 规则5: 涌现智慧 (系统日志 · 让运行可见)
# ═══════════════════════════════════════════════════════════════
def society_status(society: Society) -> str:
    """当前社会状态快照"""
    layers = {}
    for c in society.containers:
        if c.tier not in layers:
            layers[c.tier] = {"count": 0, "content": 0}
        layers[c.tier]["count"] += 1
        layers[c.tier]["content"] += len(c.content)
    
    total_work = sum(r.work_count for r in society.roles.values())
    total_reward = sum(r.reward_score for r in society.roles.values())
    
    status = []
    for tier in sorted(layers.keys()):
        status.append("L%d:%d容/%d字" % (tier, layers[tier]["count"], layers[tier]["content"]))
    
    return " | ".join(status) + " | 总工作:%d 总奖励:%.1f" % (total_work, total_reward)

# ═══════════════════════════════════════════════════════════════
# 运行模拟
# ═══════════════════════════════════════════════════════════════
def main():
    society = create_society()
    
    print("══════════════════════════════════════════════════════════════")
    print(" 满全法生命社会模拟")
    print(" · 分布时间 · 轮流执政")
    print(" · 定者=墙=裁判 · 动者=流=搬运")
    print(" · 容器是管辖 · knd是边界")
    print("══════════════════════════════════════════════════════════════")
    print(" 角色:", len(society.roles), "| 容器:", len(society.containers))
    print(" 初始:", society_status(society))
    print("══════════════════════════════════════════════════════════════")
    
    # 运行tick
    for tick in range(80):
        society.tick_counter = tick
        log = coordinate(society)
        if log:
            # 每5个tick打印一次容器状态
            if tick % 8 == 0 or ("搬" in log and "失败" not in log):
                print("[t%2d] %s" % (tick, log))
                print("     ", society_status(society))
    
    print("══════════════════════════════════════════════════════════════")
    print(" 最终状态")
    print("══════════════════════════════════════════════════════════════")
    for key, role in society.roles.items():
        print(" %-8s | 工作:%d | 奖励:%.1f | 能量:%d | %s" % (
            role.name, role.work_count, role.reward_score, role.energy, role.kind.value))
    print("──────────────────────────────────────────────────────────────")
    print(" 容器详情:")
    for c in society.containers:
        wall = "%s|%s" % (c.left_wall, c.right_wall)
        print("   #%d L%d [%s] content='%s' %s" % (c.id, c.tier, wall, c.content,
              "(密封)" if c.sealed else ""))

if __name__ == "__main__":
    main()
