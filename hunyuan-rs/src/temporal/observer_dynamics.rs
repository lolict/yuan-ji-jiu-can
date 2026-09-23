// ── v0.42 容器关系动力学 — 吸引排斥 · 共生吞噬 · 反馈回路 ──
// 观察者影响 / 动态静态容器 / 迂回反馈 / 抗议向量
// 满全法协议：容器 = 第一存在单位 = 空间本身

use std::collections::HashMap;

/// 容器极性 — 动态 vs 静态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerPolarity {
    /// 动态容器 — 主动、观察者、施力
    Dynamic,
    /// 静态容器 — 被动、被观察者、受力
    Static,
    /// 叠加态容器 — 未观测前的共存
    Superposition,
}

/// 容器关系类型（四象关系）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRelation {
    /// 吸引 — 同源共鸣（同韵母 → 吸引子）
    Attract,
    /// 排斥 — 异源仇视（敌意命名 → 推斥）
    Repel,
    /// 共生 — 互补依赖（夫妻族 → 共生赔伴律）
    Symbiosis,
    /// 吞噬 — 强力征用（容器A 吞噬 容器B）
    Engulf,
}

impl ContainerRelation {
    /// 判断极性是否匹配 — 吸引需要同向，排斥需要反向
    pub fn matches_polarity(&self, a: ContainerPolarity, b: ContainerPolarity) -> bool {
        match self {
            ContainerRelation::Attract => a == b || a == ContainerPolarity::Superposition || b == ContainerPolarity::Superposition,
            ContainerRelation::Repel => a != b && a != ContainerPolarity::Superposition && b != ContainerPolarity::Superposition,
            ContainerRelation::Symbiosis => true,
            ContainerRelation::Engulf => a != b,
        }
    }
}

/// 容器状态
#[derive(Debug, Clone)]
pub struct ContainerState {
    pub id: u64,
    pub polarity: ContainerPolarity,
    /// 容器完整性 0..255（吞噬后递减）
    pub integrity: u8,
    /// 观察者标记
    pub observer: bool,
    /// 温度（活跃度）
    pub temperature: u64,
    /// 代数层
    pub generation: u8,
}

impl ContainerState {
    pub fn new(id: u64, polarity: ContainerPolarity) -> Self {
        Self {
            id,
            polarity,
            integrity: 255,
            observer: false,
            temperature: 0,
            generation: 0,
        }
    }

    pub fn as_observer(mut self) -> Self {
        self.observer = true;
        self
    }

    /// 容器是否活跃（可参与交互）
    pub fn is_active(&self) -> bool {
        self.integrity > 0 && self.temperature > 0
    }

    /// 观察行为 — 观察者施力坍缩叠加态
    pub fn observe_target(&self, target: &mut ContainerState) {
        if !self.observer { return; }
        if target.polarity == ContainerPolarity::Superposition {
            // 观察导致坍缩：随机选择（由代数层决定）
            target.polarity = if target.generation % 2 == 0 {
                ContainerPolarity::Dynamic
            } else {
                ContainerPolarity::Static
            };
        }
        // 观察行为本身传递温度
        target.temperature = target.temperature.saturating_add(self.temperature / 2);
    }
}

/// 反馈回路
#[derive(Debug, Clone)]
pub struct FeedbackLoop {
    pub participant_ids: Vec<u64>,
    /// 累积反馈能量
    pub energy: u64,
    /// 回路上限
    pub max_capacity: u64,
    /// 非线性因子：超过阈值后反馈放大或衰减
    pub amplification_factor: f64,
    /// 迂回反馈标志（延迟触发的间接反馈）
    pub is_detour: bool,
}

impl FeedbackLoop {
    pub fn new(max_capacity: u64) -> Self {
        Self {
            participant_ids: Vec::new(),
            energy: 0,
            max_capacity,
            amplification_factor: 1.0,
            is_detour: false,
        }
    }

    pub fn add_participant(&mut self, id: u64) -> &mut Self {
        if !self.participant_ids.contains(&id) {
            self.participant_ids.push(id);
        }
        self
    }

    /// 注入反馈能量 — 返回实际溢出的能量
    pub fn feed(&mut self, amount: u64) -> u64 {
        let overflow = if self.is_detour {
            // 迂回反馈：延迟，先存储后释放
            self.energy = self.energy.saturating_add(amount / 2);
            amount - amount / 2
        } else {
            self.energy = self.energy.saturating_add(amount);
            0
        };

        if self.energy > self.max_capacity {
            let excess = self.energy - self.max_capacity;
            self.energy = self.max_capacity;
            excess
        } else {
            overflow
        }
    }

    /// 释放存储的迂回能量
    pub fn release_detour(&mut self) -> u64 {
        if !self.is_detour { return 0; }
        let released = self.energy / 2;
        self.energy = self.energy.saturating_sub(released);
        released
    }

    /// 回路是否超载
    pub fn is_overloaded(&self) -> bool {
        self.energy >= self.max_capacity
    }

    /// 参与者数量
    pub fn participant_count(&self) -> usize {
        self.participant_ids.len()
    }
}

/// 抗议向量 — 容器对外力的抵抗方向
#[derive(Debug, Clone)]
pub struct ProtestVector {
    /// 施力者 id
    pub source_id: u64,
    /// 抗议强度
    pub intensity: u64,
    /// 抗议方向：与原力相反（向量反转）
    pub direction_reversed: bool,
}

impl ProtestVector {
    pub fn new(source_id: u64, intensity: u64) -> Self {
        Self {
            source_id,
            intensity,
            direction_reversed: true, // 默认方向反转
        }
    }

    /// 计算抗议是否足以抵消外力
    pub fn can_resist(&self, external_force: u64) -> bool {
        self.direction_reversed && self.intensity >= external_force
    }
}

/// 容器关系动力学引擎
#[derive(Debug)]
pub struct RelationDynamics {
    pub containers: HashMap<u64, ContainerState>,
    pub loops: HashMap<String, FeedbackLoop>,
    pub relations: Vec<(u64, u64, ContainerRelation, u64)>, // (a, b, relation, strength)
}

impl RelationDynamics {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            loops: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn register_container(&mut self, state: ContainerState) -> &mut Self {
        self.containers.insert(state.id, state);
        self
    }

    pub fn add_relation(&mut self, a: u64, b: u64, relation: ContainerRelation, strength: u64) -> &mut Self {
        self.relations.push((a, b, relation, strength));
        self
    }

    /// 交互运算：根据关系类型计算两个容器的相互作用结果
    pub fn interact(&self, a_id: u64, b_id: u64) -> Option<ContainerInteraction> {
        let a = self.containers.get(&a_id)?;
        let b = self.containers.get(&b_id)?;

        // 查找关系
        let found = self.relations.iter()
            .find(|&&(x, y, _, _)| (x == a_id && y == b_id) || (x == b_id && y == a_id))?;

        let relation = found.2.clone();
        let strength = found.3;

        Some(ContainerInteraction {
            relation,
            strength,
            attract_score: self.compute_attract(a, b),
            observer_effect: if a.observer { 1 } else { 0 } + if b.observer { 1 } else { 0 },
        })
    }

    /// 吸引子评分 — 同韵母/同极性容器评分高
    fn compute_attract(&self, a: &ContainerState, b: &ContainerState) -> u64 {
        let polarity_match = if a.polarity == b.polarity { 10 } else { 0 };
        let gen_proximity = if a.generation == b.generation { 5 } else { 0 };
        let temp_product = (a.temperature + 1).min(100) * (b.temperature + 1).min(100) / 100;
        polarity_match + gen_proximity + temp_product
    }

    /// 回路由路通过指定容器
    pub fn route_loop(&mut self, loop_key: &str, energy: u64) -> u64 {
        if let Some(loop_) = self.loops.get_mut(loop_key) {
            loop_.feed(energy)
        } else {
            energy
        }
    }

    /// 创建回路
    pub fn create_loop(&mut self, key: &str, max_capacity: u64) -> &mut Self {
        self.loops.insert(key.to_string(), FeedbackLoop::new(max_capacity));
        self
    }

    /// 获取容器
    pub fn get_container(&self, id: u64) -> Option<&ContainerState> {
        self.containers.get(&id)
    }

    /// 获取容器可变引用
    pub fn get_container_mut(&mut self, id: u64) -> Option<&mut ContainerState> {
        self.containers.get_mut(&id)
    }

    /// 容器计数
    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    /// 关系计数
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }
}

impl Default for RelationDynamics {
    fn default() -> Self {
        Self::new()
    }
}

/// 容器交互结果
#[derive(Debug, Clone)]
pub struct ContainerInteraction {
    pub relation: ContainerRelation,
    pub strength: u64,
    pub attract_score: u64,
    pub observer_effect: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dynamics() {
        let d = RelationDynamics::new();
        assert_eq!(d.container_count(), 0);
    }

    #[test]
    fn test_register_container() {
        let mut d = RelationDynamics::new();
        d.register_container(ContainerState::new(1, ContainerPolarity::Dynamic));
        assert_eq!(d.container_count(), 1);
    }

    #[test]
    fn test_container_observer() {
        let c = ContainerState::new(1, ContainerPolarity::Dynamic).as_observer();
        assert!(c.observer);
    }

    #[test]
    fn test_observer_collapses_superposition() {
        let observer = ContainerState::new(1, ContainerPolarity::Dynamic).as_observer();
        let mut target = ContainerState::new(2, ContainerPolarity::Superposition);
        target.temperature = 10;
        observer.observe_target(&mut target);
        assert_ne!(target.polarity, ContainerPolarity::Superposition);
    }

    #[test]
    fn test_non_observer_no_effect() {
        let non_obs = ContainerState::new(1, ContainerPolarity::Dynamic);
        let mut target = ContainerState::new(2, ContainerPolarity::Superposition);
        non_obs.observe_target(&mut target);
        assert_eq!(target.polarity, ContainerPolarity::Superposition);
    }

    #[test]
    fn test_interact_attract() {
        let mut d = RelationDynamics::new();
        let mut a = ContainerState::new(1, ContainerPolarity::Dynamic);
        a.temperature = 50;
        let mut b = ContainerState::new(2, ContainerPolarity::Dynamic);
        b.temperature = 50;
        d.register_container(a);
        d.register_container(b);
        d.add_relation(1, 2, ContainerRelation::Attract, 10);
        let result = d.interact(1, 2);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.relation, ContainerRelation::Attract);
        assert!(r.attract_score > 0);
    }

    #[test]
    fn test_interact_no_relation() {
        let mut d = RelationDynamics::new();
        d.register_container(ContainerState::new(1, ContainerPolarity::Dynamic));
        d.register_container(ContainerState::new(2, ContainerPolarity::Dynamic));
        let result = d.interact(1, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_feedback_loop_feed() {
        let mut fb = FeedbackLoop::new(100);
        let overflow = fb.feed(80);
        assert_eq!(overflow, 0);
        assert_eq!(fb.energy, 80);
    }

    #[test]
    fn test_feedback_loop_overflow() {
        let mut fb = FeedbackLoop::new(50);
        let overflow = fb.feed(80);
        assert_eq!(overflow, 30);
        assert_eq!(fb.energy, 50);
    }

    #[test]
    fn test_detour_feedback() {
        let mut fb = FeedbackLoop::new(100);
        fb.is_detour = true;
        let overflow = fb.feed(80);
        assert_eq!(fb.energy, 40); // 一半存储
        assert_eq!(overflow, 40); // 另一半溢出
        let released = fb.release_detour();
        assert_eq!(released, 20);
    }

    #[test]
    fn test_protest_vector_resist() {
        let pv = ProtestVector::new(5, 100);
        assert!(pv.can_resist(80));
        assert!(!pv.can_resist(150));
    }

    #[test]
    fn test_protest_no_reversal() {
        let mut pv = ProtestVector::new(5, 100);
        pv.direction_reversed = false;
        assert!(!pv.can_resist(50));
    }

    #[test]
    fn test_container_active() {
        let mut c = ContainerState::new(1, ContainerPolarity::Dynamic);
        assert!(!c.is_active()); // temperature = 0
        c.temperature = 10;
        assert!(c.is_active());
    }

    #[test]
    fn test_relation_polarity_match() {
        assert!(ContainerRelation::Attract.matches_polarity(ContainerPolarity::Dynamic, ContainerPolarity::Dynamic));
        assert!(!ContainerRelation::Repel.matches_polarity(ContainerPolarity::Dynamic, ContainerPolarity::Dynamic));
        assert!(ContainerRelation::Symbiosis.matches_polarity(ContainerPolarity::Dynamic, ContainerPolarity::Static));
    }

    #[test]
    fn test_route_loop() {
        let mut d = RelationDynamics::new();
        d.create_loop("test", 100);
        let overflow = d.route_loop("test", 60);
        assert_eq!(overflow, 0);
        assert_eq!(d.loops.get("test").unwrap().energy, 60);
    }

    #[test]
    fn test_container_integrity() {
        let c = ContainerState::new(1, ContainerPolarity::Dynamic);
        assert_eq!(c.integrity, 255);
    }

    #[test]
    fn test_loop_participants() {
        let mut fb = FeedbackLoop::new(100);
        fb.add_participant(1).add_participant(2).add_participant(1); // 重复不计
        assert_eq!(fb.participant_count(), 2);
    }
}
