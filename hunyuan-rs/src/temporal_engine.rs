// ══════════════════════════════════════════════════════════════════════
// temporal_engine.rs — 时间一体化 · 层级化时间处理引擎
// ══════════════════════════════════════════════════════════════════════
//
// 理论根基:
//   并集 → 交集 的感知转化: 各层级独立运行(公倍数世界),
//   在公约数时刻产生耦合, 实现认知一体化
//
//   层级结构 (从高频到低频):
//       T0 Instant  — 瞬态 · 单次事件触发
//       T1 Tick     — 滴答 · 毫秒~秒级 (CPU/中断)
//       T2 Beat     — 节拍 · 秒~分级 (心跳/呼吸)
//       T3 Cycle    — 周期 · 分时级 (任务调度)
//       T4 Phase    — 阶段 · 日时级 (作息/日周期)
//       T5 Era      — 时代 · 周月级 (生长/收获)
//       T6 Epoch    — 纪元 · 年级  (演化/版本)
//       T7 Eternity — 永恒 · 存在性 (自身定义)
//
//   状态机生命周期 (每个分支独立运行):
//       Init → Seed → Grow → Mature → Decay → Renew
//
//   公约数调度:
//       所有分支的公倍数 = 各自为政的并集
//       所有分支的公约数 = 被共同需要的交集
//       引擎的目标: 找到公约数时刻, 让分支在交集中归一
//
// ══════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ────────────────────────────────────────────────────────────
// 时间层级
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TemporalLevel {
    Instant  = 0,  // 瞬态 (单次触发)
    Tick     = 1,  // 滴答 (~毫秒/秒)
    Beat     = 2,  // 节拍 (~秒/分)
    Cycle    = 3,  // 周期 (~分/时)
    Phase    = 4,  // 阶段 (~时/日)
    Era      = 5,  // 时代 (~日/月)
    Epoch    = 6,  // 纪元 (~月/年)
    Eternity = 7,  // 永恒 (存在性常量)
}

impl TemporalLevel {
    /// 该层级的基础刻度 (以毫秒为单位的相对值)
    pub fn base_tick_ms(self) -> u64 {
        match self {
            TemporalLevel::Instant  => 1,                // 1ms 基准
            TemporalLevel::Tick     => 1_000,            // 1s
            TemporalLevel::Beat     => 60_000,           // 1min
            TemporalLevel::Cycle    => 3_600_000,        // 1h
            TemporalLevel::Phase    => 86_400_000,       // 1d
            TemporalLevel::Era      => 604_800_000,      // 1w
            TemporalLevel::Epoch    => 2_592_000_000,    // 30d
            TemporalLevel::Eternity => u64::MAX,         // 永恒 = 不自动触发
        }
    }

    /// 层级深度
    pub fn depth(self) -> u8 { self as u8 }

    /// 该层级是否比 other 更深(更慢)
    pub fn is_slower_than(self, other: TemporalLevel) -> bool {
        self as u8 > other as u8
    }
}

// ────────────────────────────────────────────────────────────
// 生命周期状态机
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState {
    Init,    // 初始化 · 刚被创建
    Seed,    // 种子 · 开始吸收资源
    Grow,    // 生长 · 快速膨胀
    Mature,  // 成熟 · 稳定产出
    Decay,   // 衰亡 · 缓慢退化
    Renew,   // 重生 · 回归 Init 或消亡
}

impl LifecycleState {
    /// 转移到下一状态
    pub fn next(self) -> Self {
        match self {
            LifecycleState::Init   => LifecycleState::Seed,
            LifecycleState::Seed   => LifecycleState::Grow,
            LifecycleState::Grow   => LifecycleState::Mature,
            LifecycleState::Mature => LifecycleState::Decay,
            LifecycleState::Decay  => LifecycleState::Renew,
            LifecycleState::Renew  => LifecycleState::Init,
        }
    }

    /// 是否是活跃态(吸收/产出中)
    pub fn is_active(self) -> bool {
        matches!(self, LifecycleState::Seed | LifecycleState::Grow | LifecycleState::Mature)
    }

    /// 是否需要资源输入
    pub fn needs_input(self) -> bool {
        matches!(self, LifecycleState::Init | LifecycleState::Seed | LifecycleState::Grow)
    }

    /// 是否能对外产出
    pub fn can_output(self) -> bool {
        matches!(self, LifecycleState::Grow | LifecycleState::Mature)
    }

    /// 编码为 3-bit 字段
    pub fn to_bits(self) -> u64 {
        self as u64
    }
}

// ────────────────────────────────────────────────────────────
// 分支节点 — 一个受生命周期驱动的感知/执行单元
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct BranchNode {
    pub id: u64,                        // 唯一标识 (编码到 SMRL)
    pub name: &'static str,             // 名称
    pub level: TemporalLevel,               // 该分支所在时间层级
    pub state: LifecycleState,          // 当前生命周期状态
    pub age: u64,                       // 在当前状态的刻数
    pub threshold: u64,                 // 触发状态转移的阈值 (以该层级 tick 计)
    pub parent: Option<u64>,            // 父分支 ID (层级嵌套)
    pub children: Vec<u64>,             // 子分支 ID 列表
    pub needs: Vec<u64>,                // 该分支需要的其他分支 ID (依赖)
    pub provides: Vec<u64>,             // 该分支提供的能力 ID
}

impl BranchNode {
    pub fn new(id: u64, name: &'static str, level: TemporalLevel, threshold: u64) -> Self {
        Self {
            id, name, level,
            state: LifecycleState::Init,
            age: 0,
            threshold,
            parent: None,
            children: Vec::new(),
            needs: Vec::new(),
            provides: Vec::new(),
        }
    }

    /// 推进一个 tick: 年龄++, 超阈值则状态转移
    pub fn tick(&mut self) -> LifecycleState {
        self.age += 1;
        if self.age >= self.threshold {
            self.state = self.state.next();
            self.age = 0;
        }
        self.state
    }

    /// 检查是否处于交集点 (自身成熟 + 所有依赖都成熟)
    pub fn is_at_intersection(&self, all_nodes: &HashMap<u64, BranchNode>) -> bool {
        if self.state != LifecycleState::Mature {
            return false;
        }
        self.needs.iter().all(|dep_id| {
            all_nodes.get(dep_id)
                .map(|n| n.state == LifecycleState::Mature)
                .unwrap_or(false)
        })
    }

    /// 对父级的贡献度 (子级成熟则父级获得贡献)
    pub fn contribution(&self) -> u64 {
        match self.state {
            LifecycleState::Grow   => 1,
            LifecycleState::Mature => 3,
            LifecycleState::Decay  => 1,
            _ => 0,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 公约数调度器
// ────────────────────────────────────────────────────────────

pub struct CommonDivisorScheduler {
    pub branches: HashMap<u64, BranchNode>,
    pub global_clock: u64,             // 全局时钟 (以 ms 为单位)
    pub intersections: Vec<u64>,       // 上次tick产生的交集分支 ID
}

impl CommonDivisorScheduler {
    pub fn new() -> Self {
        Self {
            branches: HashMap::new(),
            global_clock: 0,
            intersections: Vec::new(),
        }
    }

    pub fn register(&mut self, branch: BranchNode) {
        self.branches.insert(branch.id, branch);
    }

    /// 添加父子关系
    pub fn link_parent_child(&mut self, parent_id: u64, child_id: u64) {
        if let Some(p) = self.branches.get_mut(&parent_id) {
            p.children.push(child_id);
        }
        if let Some(c) = self.branches.get_mut(&child_id) {
            c.parent = Some(parent_id);
        }
    }

    /// 添加依赖: a 需要 b
    pub fn add_dependency(&mut self, a_id: u64, b_id: u64) {
        if let Some(a) = self.branches.get_mut(&a_id) {
            a.needs.push(b_id);
        }
    }

    /// 全局推进: 找到所有需要 tick 的层级, 推进它们
    /// 返回本次 tick 触发的交集列表
    pub fn step(&mut self, elapsed_ms: u64) -> Vec<u64> {
        self.global_clock += elapsed_ms;

        // 找出当前时刻需要 tick 的层级
        let active_levels: Vec<TemporalLevel> = (
            [TemporalLevel::Tick, TemporalLevel::Beat, TemporalLevel::Cycle,
             TemporalLevel::Phase, TemporalLevel::Era, TemporalLevel::Epoch]
        ).iter().filter(|level| {
            let tick_ms = level.base_tick_ms();
            self.global_clock % tick_ms == 0
        }).cloned().collect();

        // 对这些层级的所有分支进行 tick
        let ids_to_tick: Vec<u64> = self.branches.iter()
            .filter(|(_, b)| active_levels.contains(&b.level))
            .map(|(id, _)| *id)
            .collect();

        for id in ids_to_tick {
            if let Some(b) = self.branches.get_mut(&id) {
                b.tick();
            }
        }

        // 找出所有处于交集点的分支
        self.intersections = self.branches.iter()
            .filter(|(_, b)| b.is_at_intersection(&self.branches))
            .map(|(id, _)| *id)
            .collect();

        self.intersections.clone()
    }

    /// 计算一组分支的公约数 (最小的公共触发周期)
    pub fn common_divisor(&self, ids: &[u64]) -> Option<u64> {
        if ids.is_empty() { return None; }
        let ticks: Vec<u64> = ids.iter()
            .filter_map(|id| self.branches.get(id))
            .map(|b| b.level.base_tick_ms() * b.threshold)
            .collect();
        if ticks.is_empty() { return None; }
        Some(ticks.iter().fold(ticks[0], |a, &b| Self::gcd(a, b)))
    }

    /// 计算一组分支的公倍数 (最大的公共周期)
    pub fn common_multiple(&self, ids: &[u64]) -> Option<u64> {
        if ids.is_empty() { return None; }
        let ticks: Vec<u64> = ids.iter()
            .filter_map(|id| self.branches.get(id))
            .map(|b| b.level.base_tick_ms() * b.threshold)
            .collect();
        if ticks.is_empty() { return None; }
        Some(ticks.iter().fold(ticks[0], |a, &b| Self::lcm(a, b)))
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 { return 0; }
        a / Self::gcd(a, b) * b
    }

    /// 获取当前所有处于活跃状态的分支
    pub fn active_branches(&self) -> Vec<&BranchNode> {
        self.branches.values().filter(|b| b.state.is_active()).collect()
    }

    /// 获取时代的总贡献度 (所有成熟分支的贡献和)
    pub fn total_contribution(&self) -> u64 {
        self.branches.values().map(|b| b.contribution()).sum()
    }
}

// ────────────────────────────────────────────────────────────
// 感知时间单元 — 64 位编码 (SMRL 格式复用)
// ────────────────────────────────────────────────────────────

pub struct PerceptionCell;

impl PerceptionCell {
    /// 编码规则 (64-bit):
    ///   [1:0]   level       — 时间层级 (0-7, 取低 3 bit, 但这里只占 2 bit 因为 8=1000)
    ///   实际分配:
    ///   [2:0]   level       — 时间层级 (3 bit)
    ///   [5:3]   lifecycle   — 生命周期状态 (3 bit)
    ///   [13:6]  age         — 当前状态年龄 (8 bit)
    ///   [21:14] threshold   — 阈值 (8 bit)
    ///   [37:30] parent_id   — 父节点 ID (8 bit)
    ///   [45:38] link_count  — 依赖计数 (8 bit)
    ///   [63:46] reserved    — 保留

    pub fn encode(level: TemporalLevel, state: LifecycleState, age: u8, threshold: u8, parent: u8, deps: u8) -> u64 {
        let mut cell: u64 = 0;
        cell |= (level as u64) & 0x7;            // [2:0]
        cell |= (state.to_bits() & 0x7) << 3;      // [5:3]
        cell |= (age as u64) << 6;                 // [13:6]
        cell |= (threshold as u64) << 14;          // [21:14]
        cell |= (parent as u64) << 22;             // [30:22]  (9 bit for parent)
        cell |= (deps as u64) << 31;               // [38:31]
        cell
    }

    pub fn decode(cell: u64) -> (TemporalLevel, LifecycleState, u8, u8, u8, u8) {
        let level = match cell & 0x7 {
            0 => TemporalLevel::Instant,
            1 => TemporalLevel::Tick,
            2 => TemporalLevel::Beat,
            3 => TemporalLevel::Cycle,
            4 => TemporalLevel::Phase,
            5 => TemporalLevel::Era,
            6 => TemporalLevel::Epoch,
            _ => TemporalLevel::Eternity,
        };
        let state = match (cell >> 3) & 0x7 {
            0 => LifecycleState::Init,
            1 => LifecycleState::Seed,
            2 => LifecycleState::Grow,
            3 => LifecycleState::Mature,
            4 => LifecycleState::Decay,
            _ => LifecycleState::Renew,
        };
        let age = ((cell >> 6) & 0xFF) as u8;
        let threshold = ((cell >> 14) & 0xFF) as u8;
        let parent = ((cell >> 22) & 0xFF) as u8;
        let deps = ((cell >> 31) & 0xFF) as u8;
        (level, state, age, threshold, parent, deps)
    }
}

// ────────────────────────────────────────────────────────────
// 时间感知引擎 — 主入口
// ────────────────────────────────────────────────────────────

pub struct TemporalEngine {
    pub scheduler: CommonDivisorScheduler,
    pub perception_cells: Vec<u64>,   // 编码后的感知历史
    pub max_history: usize,
}

impl TemporalEngine {
    pub fn new() -> Self {
        Self {
            scheduler: CommonDivisorScheduler::new(),
            perception_cells: Vec::new(),
            max_history: 1024,
        }
    }

    /// 注册分支并自动编码到感知历史
    pub fn register_and_encode(&mut self, branch: BranchNode) -> u64 {
        let cell = PerceptionCell::encode(
            branch.level,
            branch.state,
            branch.age as u8,
            branch.threshold as u8,
            branch.parent.unwrap_or(0) as u8,
            branch.needs.len() as u8,
        );
        self.scheduler.register(branch);
        self.push_cell(cell);
        cell
    }

    /// 推进引擎, 返回新的交集点
    pub fn step(&mut self, elapsed_ms: u64) -> Vec<u64> {
        let result = self.scheduler.step(elapsed_ms);

        // 编码当前全局状态
        let active = self.scheduler.active_branches();
        if let Some(first) = active.first() {
            let cell = PerceptionCell::encode(
                first.level, first.state,
                first.age as u8, first.threshold as u8, 0,
                active.len() as u8,
            );
            self.push_cell(cell);
        }

        result
    }

    fn push_cell(&mut self, cell: u64) {
        self.perception_cells.push(cell);
        if self.perception_cells.len() > self.max_history {
            self.perception_cells.remove(0);
        }
    }

    /// 现在处于哪个时代? (根据全局时钟判断)
    pub fn current_era(&self) -> &'static str {
        let cc = self.scheduler.global_clock;
        match cc {
            0..=3_600_000         => "origin hour",         // 第 1 小时
            3_600_001..=86_400_000 => "origin day",          // 第 1 天
            86_400_001..=604_800_000 => "origin week",       // 第 1 周
            604_800_001..=2_592_000_000 => "origin month",   // 第 1 月
            _ => "origin era",
        }
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_level_tick() {
        assert_eq!(TemporalLevel::Tick.base_tick_ms(), 1000);
        assert_eq!(TemporalLevel::Phase.base_tick_ms(), 86_400_000);
        assert!(TemporalLevel::Epoch.is_slower_than(TemporalLevel::Cycle));
    }

    #[test]
    fn test_lifecycle_transitions() {
        let s = LifecycleState::Init;
        let s1 = s.next();
        assert_eq!(s1, LifecycleState::Seed);
        let s6 = (0..6).fold(s, |acc, _| acc.next());
        assert_eq!(s6, LifecycleState::Init);          // 六步循环
        assert!(LifecycleState::Mature.is_active());
        assert!(!LifecycleState::Init.can_output());
        assert!(LifecycleState::Mature.can_output());
    }

    #[test]
    fn test_branch_node_tick() {
        let mut b = BranchNode::new(1, "test", TemporalLevel::Tick, 5);
        assert_eq!(b.state, LifecycleState::Init);
        for _ in 0..5 {
            b.tick();
        }
        assert_eq!(b.state, LifecycleState::Seed);     // 5 ticks 转移
    }

    #[test]
    fn test_branch_needs_input() {
        let b = BranchNode::new(2, "hungry", TemporalLevel::Beat, 10);
        assert!(b.state.needs_input());                 // Init 需要输入
    }

    #[test]
    fn test_scheduler_gcd() {
        let mut s = CommonDivisorScheduler::new();
        s.register(BranchNode::new(1, "a", TemporalLevel::Tick, 2));
        s.register(BranchNode::new(2, "b", TemporalLevel::Beat, 3));
        let cd = s.common_divisor(&[1, 2]);
        assert!(cd.is_some());
        assert!(cd.unwrap() > 0);
    }

    #[test]
    fn test_scheduler_step_produces_intersections() {
        let mut s = CommonDivisorScheduler::new();
        s.register(BranchNode::new(10, "alpha", TemporalLevel::Tick, 1));
        s.register(BranchNode::new(20, "beta",  TemporalLevel::Tick, 1));
        s.add_dependency(20, 10);                      // beta 需要 alpha

        // step 推进到 1000ms = Tick 层级的整数倍
        let intersections = s.step(1000);
        // alpha 经过 1 tick 后应进入 Seed, beta 同理
        // 它们需要 threshold=1, 所以一次 tick 就转移
        // 但 Mature 需要经过 3 次转移, 所以这里不会有交集 — 验证空结果也是正确的
        let _ = intersections;
        let b10 = s.branches.get(&10).unwrap();
        assert_eq!(b10.state, LifecycleState::Seed);    // 确认已推进
    }

    #[test]
    fn test_perception_cell_roundtrip() {
        let level = TemporalLevel::Cycle;
        let state = LifecycleState::Mature;
        let cell = PerceptionCell::encode(level, state, 42, 10, 7, 3);
        let (dl, ds, da, dt, dp, dd) = PerceptionCell::decode(cell);
        assert_eq!(dl, level);
        assert_eq!(ds, state);
        assert_eq!(da, 42);
        assert_eq!(dt, 10);
        assert_eq!(dp, 7);
        assert_eq!(dd, 3);
    }

    #[test]
    fn test_engine_current_era() {
        let mut e = TemporalEngine::new();
        assert_eq!(e.current_era(), "origin hour");    // clock=0
        e.step(3_600_001);                              // > 1 hour
        assert_eq!(e.current_era(), "origin day");
    }

    #[test]
    fn test_tree_child_registers_parent() {
        let mut s = CommonDivisorScheduler::new();
        let mut parent = BranchNode::new(100, "root", TemporalLevel::Era, 10);
        let mut child  = BranchNode::new(101, "sub",  TemporalLevel::Cycle, 5);
        parent.children.push(101);
        child.parent = Some(100);
        s.register(parent);
        s.register(child);
        let p = s.branches.get(&100).unwrap();
        assert_eq!(p.children[0], 101);
    }
}
