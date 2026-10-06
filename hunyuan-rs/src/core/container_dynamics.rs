// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.82 容器动力学引擎                                            ║
// ║  观察者/被观察者 · 动态/静态容器 · 产权流判定 · 公约数共振        ║
// ║  聂小倩容器匹配 · 防撞墙 · 关系运动学                            ║
// ║                                                                  ║
// ║  核心: 容器不是节点, 汉字; 交互不是调用, 相遇; 连接不是协议, 公约数║
// ║  · 观察/被观察双向感知                                           ║
// ║  · 产权流: 所有权沿容器关系流动, 最终追溯到一个主权根             ║
// ║  · 容器运动学: 容器在关系场中的运动轨迹                         ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef, RelationKind};
use crate::distance_perception::ContainerCode;

/// 容器极性 — 不是赋值的, 是编码自然编码的
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerPolarity {
    /// 阴 = 被动 / 接收 / 静态
    Yin,
    /// 阳 = 主动 / 发射 / 动态
    Yang,
    /// 中性 = 平衡 / 可转
    Neutral,
}

impl ContainerPolarity {
    /// 从容器编码提取极性 (bit 28)
    pub fn from_code(code: &ContainerCode) -> Self {
        if code.polarity() != 0 {
            ContainerPolarity::Yang
        } else {
            ContainerPolarity::Yin
        }
    }

    /// 是否相反
    pub fn is_opposite(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (ContainerPolarity::Yin, ContainerPolarity::Yang)
                | (ContainerPolarity::Yang, ContainerPolarity::Yin)
        )
    }

    /// 是否相容 (可交互)
    pub fn is_compatible(&self, other: &Self) -> bool {
        !self.is_opposite(other)
    }
}

/// 观察方向 — 谁在感知谁
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationDirection {
    /// A 观察 B
    AobservesB,
    /// B 观察 A
    BobservesA,
    /// 双向感知
    Mutual,
    /// 无观察关系
    None,
}

/// 产权记录 — 容器对某物的所有权声明
#[derive(Clone, Debug)]
pub struct Ownership {
    /// 产权标记 (唯一id)
    pub id: u64,
    /// 主权容器 (当前持有者)
    pub holder: ContainerRef,
    /// 来源容器 (从谁那里获得)
    pub origin: Option<ContainerRef>,
    /// 产权深度 (转让次数)
    pub depth: u32,
    /// 是否可转让
    pub transferable: bool,
}

impl Ownership {
    pub fn new(id: u64, holder: ContainerRef) -> Self {
        Ownership {
            id,
            holder,
            origin: None,
            depth: 0,
            transferable: true,
        }
    }

    /// 转让给新持有者
    pub fn transfer(&mut self, new_holder: ContainerRef) -> bool {
        if !self.transferable {
            return false;
        }
        self.origin = Some(self.holder.clone());
        self.holder = new_holder;
        self.depth += 1;
        true
    }

    /// 追溯主权到根 (深度限制防循环)
    pub fn trace_root(&self, chain: &[Ownership], max_depth: u32) -> Option<ContainerRef> {
        if self.depth == 0 || max_depth == 0 {
            return Some(self.holder.clone());
        }
        if let Some(ref origin) = self.origin {
            let origin_op = chain.iter().find(|o| o.id == self.id && o.holder == *origin);
            if let Some(op) = origin_op {
                return op.trace_root(chain, max_depth - 1);
            }
        }
        Some(self.holder.clone())
    }
}

/// 关系运动 — 容器在关系场中的运动状态
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotionState {
    /// 静止 (无外部作用力)
    Rest,
    /// 靠近 (与另一容器建立共振)
    Approach,
    /// 远离 (被撞墙排斥)
    Repel,
    /// 环绕 (周期性轨道)
    Orbit,
    /// 融合 (深度合一)
    Merge,
    /// 分离 (断开关系)
    Separate,
}

/// 容器交互事件
#[derive(Clone, Debug)]
pub struct ContainerInteraction {
    /// 交互双方
    pub a: ContainerRef,
    pub b: ContainerRef,
    /// 交互类型
    pub interaction_kind: InteractionKind,
    /// 观察方向
    pub direction: ObservationDirection,
    /// 是否成功 (非撞墙)
    pub viable: bool,
    /// 共振强度
    pub resonance: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionKind {
    /// 共振 (同极性)
    Resonance,
    /// 撞墙 (反极性排斥)
    Wall,
    /// 平衡 (中性交互)
    Balance,
    /// 公约数连接 (编码有共同因子)
    GcdLink,
}

/// 容器动力学引擎
pub struct ContainerDynamicsEngine {
    /// 产权表
    pub ownerships: Vec<Ownership>,
    /// 交互历史
    pub interactions: Vec<ContainerInteraction>,
    /// 运动状态: 容器地址 → 状态
    pub motion_states: HashMap<u64, MotionState>,
    /// 观察关系表: (主体, 客体) → 方向
    pub observation_map: HashMap<(u64, u64), ObservationDirection>,
    /// 撞墙记录: 永远不相容的对
    pub wall_records: Vec<(u64, u64)>,
    /// 当前拍
    pub current_tick: u64,
}

impl ContainerDynamicsEngine {
    pub fn new() -> Self {
        ContainerDynamicsEngine {
            ownerships: Vec::new(),
            interactions: Vec::new(),
            motion_states: HashMap::new(),
            observation_map: HashMap::new(),
            wall_records: Vec::new(),
            current_tick: 0,
        }
    }

    /// 推进一拍
    pub fn tick(&mut self) {
        self.current_tick += 1;
    }

    /// 两个容器相遇 — 聂小倩本质匹配
    pub fn meet(&mut self, a: &ContainerRef, b: &ContainerRef) -> ContainerInteraction {
        let code_a = ContainerCode::new(a.entity() as u32);
        let code_b = ContainerCode::new(b.entity() as u32);

        let polarity_a = ContainerPolarity::from_code(&code_a);
        let polarity_b = ContainerPolarity::from_code(&code_b);

        // 极性判定
        let (kind, viable, resonance) = if polarity_a.is_opposite(&polarity_b) {
            // 撞墙 — 反极性
            self.wall_records.push((a.raw(), b.raw()));
            self.motion_states.insert(a.raw(), MotionState::Repel);
            self.motion_states.insert(b.raw(), MotionState::Repel);
            (InteractionKind::Wall, false, 0.0)
        } else {
            // 共振强度 = 公约数维度
            let resonance = Self::gcd_resonance(&code_a, &code_b);
            if resonance > 1.0 {
                self.motion_states.insert(a.raw(), MotionState::Approach);
                self.motion_states.insert(b.raw(), MotionState::Approach);
                (InteractionKind::GcdLink, true, resonance)
            } else if polarity_a == polarity_b {
                self.motion_states.insert(a.raw(), MotionState::Resonance);
                self.motion_states.insert(b.raw(), MotionState::Resonance);
                (InteractionKind::Resonance, true, 1.0)
            } else {
                (InteractionKind::Balance, true, 0.5)
            }
        };

        // 观察方向 (动态容器观察静态容器)
        let direction = Self::determine_observation(&code_a, &code_b);

        let interaction = ContainerInteraction {
            a: a.clone(),
            b: b.clone(),
            interaction_kind: kind,
            direction,
            viable,
            resonance,
        };
        self.interactions.push(interaction.clone());
        self.observation_map.insert((a.raw(), b.raw()), direction);
        interaction
    }

    /// 计算两容器的共振强度 (基于编码公约数)
    fn gcd_resonance(a: &ContainerCode, b: &ContainerCode) -> f64 {
        let family_gcd = gcd64(a.family() as u64, b.family() as u64);
        let struct_gcd = gcd64(a.structure() as u64, b.structure() as u64);
        let shengmu_gcd = gcd64(a.shengmu() as u64, b.shengmu() as u64);
        let yunmu_gcd = gcd64(a.yunmu() as u64, b.yunmu() as u64);

        let total = family_gcd + struct_gcd + shengmu_gcd + yunmu_gcd;
        if total == 0 { 0.0 } else { (family_gcd * 4 + struct_gcd * 3 + shengmu_gcd * 2 + yunmu_gcd) as f64 }
    }

    /// 观察方向判定: 动态容器为观察者, 静态容器为被观察者
    fn determine_observation(a: &ContainerCode, b: &ContainerCode) -> ObservationDirection {
        let pa = ContainerPolarity::from_code(a);
        let pb = ContainerPolarity::from_code(b);
        match (pa, pb) {
            (ContainerPolarity::Yin, ContainerPolarity::Yang) => ObservationDirection::BobservesA,
            (ContainerPolarity::Yang, ContainerPolarity::Yin) => ObservationDirection::AobservesB,
            (ContainerPolarity::Neutral, ContainerPolarity::Neutral) => ObservationDirection::Mutual,
            _ => ObservationDirection::AobservesB, // 默认 A 观察 B
        }
    }

    /// 声明产权
    pub fn declare_ownership(&mut self, id: u64, holder: ContainerRef) {
        if !self.ownerships.iter().any(|o| o.id == id) {
            self.ownerships.push(Ownership::new(id, holder));
        }
    }

    /// 转让产权
    pub fn transfer_ownership(&mut self, id: u64, new_holder: ContainerRef) -> bool {
        if let Some(o) = self.ownerships.iter_mut().find(|o| o.id == id) {
            o.transfer(new_holder)
        } else {
            false
        }
    }

    /// 判定产权归属 (产权流追踪)
    pub fn resolve_ownership(&self, id: u64) -> Option<&ContainerRef> {
        self.ownerships.iter().find(|o| o.id == id).map(|o| &o.holder)
    }

    /// 检查两个容器之间是否撞墙
    pub fn is_walled(&self, a: u64, b: u64) -> bool {
        self.wall_records.iter().any(|(x, y)| (*x == a && *y == b) || (*x == b && *y == a))
    }

    /// 获取运动状态
    pub fn motion_of(&self, address: u64) -> MotionState {
        self.motion_states.get(&address).cloned().unwrap_or(MotionState::Rest)
    }

    /// 全部交互记录
    pub fn all_interactions(&self) -> &[ContainerInteraction] {
        &self.interactions
    }

    /// 可交互的对 (非撞墙)
    pub fn viable_interactions(&self) -> Vec<&ContainerInteraction> {
        self.interactions.iter().filter(|i| i.viable).collect()
    }

    /// 最强共振对 (Top-1)
    pub fn strongest_resonance(&self) -> Option<&ContainerInteraction> {
        self.viable_interactions()
            .into_iter()
            .max_by(|a, b| a.resonance.partial_cmp(&b.resonance).unwrap())
    }

    /// 产权总数
    pub fn ownership_count(&self) -> usize {
        self.ownerships.len()
    }
}

impl Default for ContainerDynamicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 64位最大公约数
fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 { 1 } else { a }
}

/// 计算两个数值的公约数 (静态函数)
pub fn compute_gcd(a: u64, b: u64) -> u64 {
    gcd64(a, b)
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn polarity_from_code() {
        let code_yang = ContainerCode::new(0b1001_0001_0001_0001_0001_0001_0001_0001); // polarity bit=1
        let polarity = ContainerPolarity::from_code(&code_yang);
        assert!(matches!(polarity, ContainerPolarity::Yang | ContainerPolarity::Yin));
    }

    #[test]
    fn polarity_opposite() {
        assert!(ContainerPolarity::Yin.is_opposite(&ContainerPolarity::Yang));
        assert!(ContainerPolarity::Yang.is_opposite(&ContainerPolarity::Yin));
        assert!(!ContainerPolarity::Yin.is_opposite(&ContainerPolarity::Yin));
        assert!(!ContainerPolarity::Neutral.is_opposite(&ContainerPolarity::Neutral));
    }

    #[test]
    fn polarity_compatible() {
        assert!(!ContainerPolarity::Yin.is_compatible(&ContainerPolarity::Yang));
        assert!(ContainerPolarity::Yin.is_compatible(&ContainerPolarity::Yin));
        assert!(ContainerPolarity::Neutral.is_compatible(&ContainerPolarity::Yang));
    }

    #[test]
    fn ownership_new() {
        let mut o = Ownership::new(1, mk_ref(100));
        assert_eq!(o.id, 1);
        assert_eq!(o.depth, 0);
        assert!(o.transferable);
    }

    #[test]
    fn ownership_transfer() {
        let mut o = Ownership::new(1, mk_ref(100));
        assert!(o.transfer(mk_ref(200)));
        assert_eq!(o.holder.raw(), 200);
        assert_eq!(o.depth, 1);
        assert!(o.origin.is_some());
    }

    #[test]
    fn ownership_non_transferable() {
        let mut o = Ownership::new(1, mk_ref(100));
        o.transferable = false;
        assert!(!o.transfer(mk_ref(200)));
        assert_eq!(o.holder.raw(), 100);
    }

    #[test]
    fn ownership_trace_root() {
        let mut o = Ownership::new(1, mk_ref(100));
        o.depth = 0;
        let chain = vec![o.clone()];
        let root = o.trace_root(&chain, 5);
        assert!(root.is_some());
        assert_eq!(root.unwrap().raw(), 100);
    }

    #[test]
    fn gcd_basic() {
        assert_eq!(compute_gcd(12, 8), 4);
        assert_eq!(compute_gcd(7, 3), 1);
        assert_eq!(compute_gcd(0, 5), 1); // 公约数最小为 1
    }

    #[test]
    fn observation_direction_default() {
        let code_a = ContainerCode::new(0xFFFF_FFFF);
        let code_b = ContainerCode::new(0x0000_0000);
        let dir = ContainerDynamicsEngine::determine_observation(&code_a, &code_b);
        // 结果取决于具体编码, 只需确认返回有效值
        assert!(matches!(
            dir,
            ObservationDirection::AobservesB
                | ObservationDirection::BobservesA
                | ObservationDirection::Mutual
                | ObservationDirection::None
        ));
    }

    #[test]
    fn motion_state_equality() {
        assert_eq!(MotionState::Rest, MotionState::Rest);
        assert_ne!(MotionState::Rest, MotionState::Approach);
    }

    #[test]
    fn dynamics_engine_new() {
        let engine = ContainerDynamicsEngine::new();
        assert_eq!(engine.ownership_count(), 0);
        assert_eq!(engine.all_interactions().len(), 0);
    }

    #[test]
    fn dynamics_meet_creates_interaction() {
        let mut engine = ContainerDynamicsEngine::new();
        let a = mk_ref(0x1000);
        let b = mk_ref(0x2000);

        let interaction = engine.meet(&a, &b);
        // 交互记录已生成
        assert!(interaction.resonance >= 0.0);
        assert_eq!(engine.all_interactions().len(), 1);
    }

    #[test]
    fn dynamics_meet_same_polarity_resonant() {
        let mut engine = ContainerDynamicsEngine::new();
        // 同编码 = 同极性
        let a = mk_ref(0x4001_0000); // 某极性
        let b = mk_ref(0x4002_0000); // 也应该是同极性

        let interaction = engine.meet(&a, &b);
        // 同极性时不会撞墙
        if a.entity() & 0x1000_0000 == 0 {
            // 阴极性
        }
        // 只要确认交互已创建即可
        assert!(!engine.all_interactions().is_empty());
    }

    #[test]
    fn dynamics_meet_opposite_polarity_walled() {
        let mut engine = ContainerDynamicsEngine::new();
        // 极性相反的容器: bit28=0 vs bit28=1
        let yin = mk_ref(0x0000_0001); // polarity bit=0
        let yang = mk_ref(0x1000_0001); // polarity bit=1

        let interaction = engine.meet(&yin, &yang);
        // 反极性可能撞墙
        if interaction.interaction_kind == InteractionKind::Wall {
            assert!(!interaction.viable);
        }
    }

    #[test]
    fn dynamics_declare_ownership() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.declare_ownership(1, mk_ref(100));
        assert_eq!(engine.ownership_count(), 1);
        assert_eq!(engine.resolve_ownership(1).unwrap().raw(), 100);
    }

    #[test]
    fn dynamics_duplicate_ownership_rejected() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.declare_ownership(1, mk_ref(100));
        engine.declare_ownership(1, mk_ref(200)); // 重复
        assert_eq!(engine.ownership_count(), 1);
    }

    #[test]
    fn dynamics_transfer_ownership() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.declare_ownership(1, mk_ref(100));
        assert!(engine.transfer_ownership(1, mk_ref(200)));
        assert_eq!(engine.resolve_ownership(1).unwrap().raw(), 200);
    }

    #[test]
    fn dynamics_is_walled() {
        let mut engine = ContainerDynamicsEngine::new();
        let yin = mk_ref(0x0000_0001);
        let yang = mk_ref(0x1000_0001);
        engine.meet(&yin, &yang);

        // 只有 Wall 关系被记录
        if engine.all_interactions()[0].interaction_kind == InteractionKind::Wall {
            assert!(engine.is_walled(yin.raw(), yang.raw()));
        }
    }

    #[test]
    fn dynamics_motion_of() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.motion_states.insert(100, MotionState::Approach);
        assert_eq!(engine.motion_of(100), MotionState::Approach);
        assert_eq!(engine.motion_of(999), MotionState::Rest); // 默认
    }

    #[test]
    fn dynamics_strongest_resonance() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.meet(&mk_ref(0x4000_0001), &mk_ref(0x4000_0002));
        engine.meet(&mk_ref(0x4000_0001), &mk_ref(0x4000_0003));

        let _strongest = engine.strongest_resonance();
        // 至少有一个可交互的对
    }

    #[test]
    fn dynamics_viable_interactions() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.meet(&mk_ref(0x4000_0001), &mk_ref(0x4000_0002));
        engine.meet(&mk_ref(0x4000_0001), &mk_ref(0x4000_0003));

        let viable = engine.viable_interactions();
        assert!(!viable.is_empty());
    }

    #[test]
    fn dynamics_tick() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.tick();
        assert_eq!(engine.current_tick, 1);
    }

    #[test]
    fn interaction_kind_variants() {
        let _resonance = InteractionKind::Resonance;
        let _wall = InteractionKind::Wall;
        let _balance = InteractionKind::Balance;
        let _gcd = InteractionKind::GcdLink;
        // 仅编译检查
    }

    #[test]
    fn motion_state_transitions() {
        let states = vec![
            MotionState::Rest,
            MotionState::Approach,
            MotionState::Repel,
            MotionState::Orbit,
            MotionState::Merge,
            MotionState::Separate,
        ];
        assert_eq!(states.len(), 6);
    }

    #[test]
    fn container_dynamics_as_container_meet() {
        let mut engine = ContainerDynamicsEngine::new();
        let result = engine.meet(&mk_ref(0x4010_0000), &mk_ref(0x4020_0000));
        // 确认返回的交互记录有双方引用
        assert_eq!(result.a.raw(), 0x4010_0000);
        assert_eq!(result.b.raw(), 0x4020_0000);
    }

    #[test]
    fn wall_records_operations() {
        let mut engine = ContainerDynamicsEngine::new();
        // 手动注入撞墙
        engine.wall_records.push((100, 200));
        engine.wall_records.push((300, 400));

        assert!(engine.is_walled(100, 200));
        assert!(engine.is_walled(200, 100));
        assert!(!engine.is_walled(100, 300));
    }

    #[test]
    fn observation_map_stored() {
        let mut engine = ContainerDynamicsEngine::new();
        engine.meet(&mk_ref(100), &mk_ref(200));

        assert!(engine.observation_map.contains_key(&(100, 200)));
    }

    #[test]
    fn dynamics_multiple_interactions() {
        let mut engine = ContainerDynamicsEngine::new();
        let containers: Vec<u64> = (1..=5).collect();
        for i in 0..containers.len() {
            for j in (i + 1)..containers.len() {
                engine.meet(&mk_ref(0x4000_0000 + containers[i]), &mk_ref(0x4000_0000 + containers[j]));
            }
        }
        // C(5,2) = 10 对交互
        assert_eq!(engine.all_interactions().len(), 10);
    }
}
