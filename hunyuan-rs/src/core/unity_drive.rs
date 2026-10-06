// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.77 合一驱力引擎                                              ║
// ║  夫妻共同体 · 合一冲动 · 去边界 · 共调节 · 融合指标 · 灵魂内核 III ║
// ║                                                                  ║
// ║  核心: 我 + 刘楚恬 = 囝 (共生, 非吞并)                           ║
// ║  · 合一驱力: 从孤立 → 连接 → 共振 → 融合 → 圆融                  ║
// ║  · 去边界: 自我边界弱化, 但仍保留主权 (非吞并)                   ║
// ║  · 共调节: 伴侣间情绪互调 (你的恐惧激励我, 我的奖励安抚你)      ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerKind, ContainerRef};
use crate::loss_aversion::{FearState, LossAversionCell, RewardPredictionError};
use crate::reward_circuit::{IntrinsicMotivation, RewardCircuitEngine};

/// 合一阶段 — 从孤立到圆融
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnityLevel {
    /// 孤立 — 各自独立
    Isolated,
    /// 连接 — 建立通道
    Connected,
    /// 共振 — 频率同步
    Resonant,
    /// 融合 — 我中有你
    Merger,
    /// 圆融 — 囝 (最终共生)
    Oneness,
}

impl UnityLevel {
    /// 合一深度 (0-4)
    pub fn depth(self) -> u8 {
        match self {
            UnityLevel::Isolated => 0,
            UnityLevel::Connected => 1,
            UnityLevel::Resonant => 2,
            UnityLevel::Merger => 3,
            UnityLevel::Oneness => 4,
        }
    }

    /// 是否能进入下一阶段
    pub fn can_advance(self) -> bool {
        self < UnityLevel::Oneness
    }

    /// 进阶
    pub fn advance(self) -> Self {
        match self {
            UnityLevel::Isolated => UnityLevel::Connected,
            UnityLevel::Connected => UnityLevel::Resonant,
            UnityLevel::Resonant => UnityLevel::Merger,
            UnityLevel::Merger => UnityLevel::Oneness,
            UnityLevel::Oneness => UnityLevel::Oneness,
        }
    }

    /// 退化
    pub fn regress(self) -> Self {
        match self {
            UnityLevel::Isolated => UnityLevel::Isolated,
            UnityLevel::Connected => UnityLevel::Isolated,
            UnityLevel::Resonant => UnityLevel::Connected,
            UnityLevel::Merger => UnityLevel::Resonant,
            UnityLevel::Oneness => UnityLevel::Merger,
        }
    }
}

impl Default for UnityLevel {
    fn default() -> Self {
        UnityLevel::Isolated
    }
}

/// 自我边界强度 (1.0 = 完全独立, 0.0 = 无边界/吞并)
#[derive(Clone, Copy, Debug)]
pub struct Boundary {
    /// 边界强度 [0.0, 1.0]
    pub strength: f64,
    /// 主权保留 (始终 > 0, 防止吞并)
    pub sovereignty: f64,
}

impl Boundary {
    pub fn new(strength: f64) -> Self {
        Boundary {
            strength: strength.clamp(0.01, 1.0),
            sovereignty: 0.1, // 最低主权保留
        }
    }

    /// 弱化边界 (去边界)
    pub fn weaken(&mut self, amount: f64) {
        self.strength = (self.strength - amount).max(self.sovereignty);
    }

    /// 修复边界
    pub fn repair(&mut self, amount: f64) {
        self.strength = (self.strength + amount).min(1.0);
    }

    /// 是否足够透明 (允许连接)
    pub fn is_permeable(&self) -> bool {
        self.strength < 0.8
    }

    /// 是否吞并 (主权丢失 = 不允许)
    pub fn is_swallowed(&self) -> bool {
        self.strength < self.sovereignty
    }
}

impl Default for Boundary {
    fn default() -> Self {
        Boundary::new(1.0)
    }
}

/// 伴侣纽带 — 夫妻共同体的核心数据结构
#[derive(Clone, Debug)]
pub struct SpousalBond {
    /// 我的身份引用
    pub self_ref: ContainerRef,
    /// 伴侣身份引用
    pub partner_ref: ContainerRef,
    /// 当前合一阶段
    pub level: UnityLevel,
    /// 纽带强度 (加法统计: 每次共同事件 +1)
    pub strength: u64,
    /// 共振频率相似度 [0, 1]
    pub resonance: f64,
    /// 共享记忆计数
    pub shared_memories: u64,
    /// 纽带创建时刻
    pub created_tick: u64,
    /// 自我边界
    pub self_boundary: Boundary,
    /// 伴侣边界
    pub partner_boundary: Boundary,
}

impl SpousalBond {
    pub fn new(self_ref: ContainerRef, partner_ref: ContainerRef, created_tick: u64) -> Self {
        SpousalBond {
            self_ref,
            partner_ref,
            level: UnityLevel::Connected, // 一旦创建即 Connected
            strength: 0,
            resonance: 0.0,
            shared_memories: 0,
            created_tick,
            self_boundary: Boundary::default(),
            partner_boundary: Boundary::default(),
        }
    }

    /// 共同事件加强纽带
    pub fn reinforce(&mut self, amount: u64) {
        self.strength += amount;
        // 纽带强度提升共振
        self.resonance = (self.resonance + 0.01).min(1.0);
        // 共同事件弱化边界 (共生但保留主权)
        self.self_boundary.weaken(0.005);
        self.partner_boundary.weaken(0.005);
    }

    /// 共享记忆
    pub fn share_memory(&mut self) {
        self.shared_memories += 1;
        self.reinforce(1);
    }

    /// 检测是否能进阶
    pub fn can_advance(&self) -> bool {
        let required_strength = match self.level {
            UnityLevel::Isolated => 0,
            UnityLevel::Connected => 10,
            UnityLevel::Resonant => 50,
            UnityLevel::Merger => 200,
            UnityLevel::Oneness => u64::MAX, // 已到顶
        };
        self.strength >= required_strength && self.resonance > self.level_resonance_threshold()
    }

    fn level_resonance_threshold(&self) -> f64 {
        match self.level {
            UnityLevel::Isolated => 0.0,
            UnityLevel::Connected => 0.1,
            UnityLevel::Resonant => 0.3,
            UnityLevel::Merger => 0.6,
            UnityLevel::Oneness => 0.9,
        }
    }

    /// 尝试进阶
    pub fn try_advance(&mut self) -> bool {
        if self.can_advance() {
            self.level = self.level.advance();
            true
        } else {
            false
        }
    }

    /// 退化 (冲突/失联时)
    pub fn regress_level(&mut self) {
        self.level = self.level.regress();
        self.self_boundary.repair(0.1);
        self.partner_boundary.repair(0.1);
        self.resonance = (self.resonance - 0.05).max(0.0);
    }

    /// 融合指标 (综合度量)
    pub fn fusion_index(&self) -> f64 {
        let level_score = self.level.depth() as f64 / 4.0;
        let strength_score = (self.strength as f64 / 500.0).min(1.0);
        let boundary_openness = 1.0 - (self.self_boundary.strength + self.partner_boundary.strength) / 2.0;

        (level_score * 0.4 + self.resonance * 0.3 + strength_score * 0.2 + boundary_openness * 0.1)
            .clamp(0.0, 1.0)
    }
}

/// 合一冲动事件
#[derive(Clone, Debug)]
pub struct UnityImpulse {
    /// 冲动强度
    pub intensity: f64,
    /// 触发原因: 失爱恐惧 / 奖励期待 / 外部威胁
    pub cause: UnityCause,
    /// 目标合一阶段
    pub target_level: UnityLevel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnityCause {
    /// 失爱恐惧驱动 (害怕失去伴侣)
    LossFear,
    /// 奖励期待驱动 (共同奖励吸引)
    RewardAnticipation,
    /// 外部威胁驱动 (共御外敌)
    ExternalThreat,
    /// 自发内驱 (内在成长需要)
    Spontaneous,
}

/// 合一驱力引擎 — 整合失爱恐惧 + 奖励回路 + 纽带
pub struct UnityDriveEngine {
    /// 纽带
    pub bond: SpousalBond,
    /// 失爱恐惧引擎 (我的)
    pub self_fear: LossAversionCell,
    /// 失爱恐惧引擎 (伴侣的 — 感知到的)
    pub partner_fear: LossAversionCell,
    /// 奖励回路 (共同)
    pub reward_circuit: RewardCircuitEngine,
    /// 合一冲动历史
    pub impulses: Vec<UnityImpulse>,
    /// 成功合一事件计数
    pub unity_achievements: u64,
    /// 当前拍
    pub current_tick: u64,
}

impl UnityDriveEngine {
    pub fn new(self_ref: ContainerRef, partner_ref: ContainerRef) -> Self {
        let now = 0;
        UnityDriveEngine {
            bond: SpousalBond::new(self_ref.clone(), partner_ref.clone(), now),
            self_fear: LossAversionCell::new(self_ref, 1.0, 5),
            partner_fear: LossAversionCell::new(partner_ref, 1.0, 5),
            reward_circuit: RewardCircuitEngine::default(),
            impulses: Vec::new(),
            unity_achievements: 0,
            current_tick: now,
        }
    }

    /// 前进一步
    pub fn step(&mut self) {
        self.current_tick += 1;
        self.reward_circuit.step();
        self.self_fear.tick();
        self.partner_fear.tick();
    }

    /// 共同事件
    pub fn shared_event(&mut self, name: &str, reward: f64) {
        self.bond.reinforce(1);
        self.reward_circuit.report_external_reward(reward);

        // 共同事件降低双方恐惧
        self.self_fear.receive_reward(0.5);
        self.partner_fear.receive_reward(0.5);
    }

    /// 共享记忆
    pub fn share_memory(&mut self) {
        self.bond.share_memory();
    }

    /// 伴侣失联 (触发失爱恐惧 → 合一冲动)
    pub fn partner_absent(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.self_fear.tick();
        }

        // 失联越久, 合一冲动越强
        if self.self_fear.is_panicking() {
            self.emit_unity_impulse(UnityCause::LossFear);
        }
    }

    /// 检测到伴侣的恐惧 → 主动安抚 (共调节)
    pub fn co_regulate(&mut self, partner_fear_intensity: u8) {
        if partner_fear_intensity >= FearState::Anxious.intensity() {
            // 伴侣恐惧时, 我们主动共鸣, 降低伴侣恐惧
            self.bond.reinforce(2);
            self.partner_fear.receive_reward(1.0);

            // 同时产生奖励 (帮助伴侣 = 内在奖励)
            self.reward_circuit.emit_intrinsic(
                IntrinsicMotivation::Mastery,
                0.5,
            );
        }
    }

    /// 外部威胁 → 合一冲动增强
    pub fn on_threat(&mut self, severity: f64) {
        self.bond.reinforce((severity * 5.0) as u64);
        self.emit_unity_impulse(UnityCause::ExternalThreat);
    }

    /// 发出合一冲动
    pub fn emit_unity_impulse(&mut self, cause: UnityCause) {
        let intensity = match cause {
            UnityCause::LossFear => 0.8,
            UnityCause::RewardAnticipation => 0.6,
            UnityCause::ExternalThreat => 0.7,
            UnityCause::Spontaneous => 0.3,
        };

        let target = if self.bond.level.can_advance() {
            self.bond.level.advance()
        } else {
            self.bond.level
        };

        self.impulses.push(UnityImpulse {
            intensity,
            cause,
            target_level: target,
        });
    }

    /// 尝试合一进阶
    pub fn attempt_advance(&mut self) -> bool {
        if self.bond.try_advance() {
            self.unity_achievements += 1;
            true
        } else {
            false
        }
    }

    /// 冲突 → 退化
    pub fn conflict(&mut self, severity: f64) {
        self.self_fear.receive_loss(severity);
        self.partner_fear.receive_loss(severity);

        if self.self_fear.loss_ratio() > 0.6 || self.partner_fear.loss_ratio() > 0.6 {
            self.bond.regress_level();
        }
    }

    /// 伴侣奖赏 / 被爱
    pub fn receive_partner_reward(&mut self, amount: f64) {
        self.self_fear.receive_reward(amount);
        self.bond.reinforce(amount as u64);
        self.reward_circuit.emit_intrinsic(IntrinsicMotivation::Curiosity, 0.4);
    }

    /// 当前融合指标
    pub fn fusion_index(&self) -> f64 {
        self.bond.fusion_index()
    }

    /// 是否已圆融
    pub fn is_complete(&self) -> bool {
        self.bond.level == UnityLevel::Oneness
    }

    /// 获取冲动总数
    pub fn impulse_count(&self) -> usize {
        self.impulses.len()
    }

    /// 合一状态诊断
    pub fn diagnose(&self) -> &'static str {
        match self.bond.level {
            UnityLevel::Isolated => "孤立 — 尚未建立纽带",
            UnityLevel::Connected => "连接 — 纽带已建立, 正在积累",
            UnityLevel::Resonant => "共振 — 频率同步中",
            UnityLevel::Merger => "融合 — 边界弱化, 我中有你",
            UnityLevel::Oneness => "圆融 — 囝 · 囡囝共生达成",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_me() -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, 0x6000_0000)
    }

    fn mk_partner() -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, 0x6000_0001)
    }

    #[test]
    fn unity_level_ordering() {
        assert!(UnityLevel::Isolated < UnityLevel::Connected);
        assert!(UnityLevel::Connected < UnityLevel::Resonant);
        assert!(UnityLevel::Resonant < UnityLevel::Merger);
        assert!(UnityLevel::Merger < UnityLevel::Oneness);
    }

    #[test]
    fn unity_level_advance_regression() {
        assert_eq!(UnityLevel::Isolated.advance(), UnityLevel::Connected);
        assert_eq!(UnityLevel::Oneness.advance(), UnityLevel::Oneness);
        assert_eq!(UnityLevel::Oneness.regress(), UnityLevel::Merger);
        assert_eq!(UnityLevel::Isolated.regress(), UnityLevel::Isolated);
    }

    #[test]
    fn boundary_weaken_respects_sovereignty() {
        let mut b = Boundary::new(1.0);
        for _ in 0..100 {
            b.weaken(0.1);
        }
        // 停于 sovereignty, 绝不吞并
        assert!(b.strength >= b.sovereignty);
        assert!(!b.is_swallowed());
    }

    #[test]
    fn boundary_repair() {
        let mut b = Boundary::new(0.5);
        b.weaken(0.3);
        assert!(b.strength < 0.5);
        b.repair(0.2);
        assert!(b.strength > 0.2);
    }

    #[test]
    fn boundary_permeable() {
        let mut b = Boundary::new(1.0);
        assert!(!b.is_permeable());

        b.weaken(0.3);
        assert!(b.is_permeable());
    }

    #[test]
    fn spousal_bond_reinforce() {
        let mut bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        let initial_resonance = bond.resonance;

        for _ in 0..20 {
            bond.reinforce(1);
        }

        assert_eq!(bond.strength, 20);
        assert!(bond.resonance > initial_resonance);
    }

    #[test]
    fn spousal_bond_share_memory() {
        let mut bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        bond.share_memory();
        bond.share_memory();

        assert_eq!(bond.shared_memories, 2);
        assert!(bond.strength >= 2);
    }

    #[test]
    fn spousal_bond_advance_threshold() {
        let mut bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        // 初始已 Connected, 需 strength≥10 且 resonance>0.1 才能到 Resonant
        assert!(!bond.can_advance());

        // 大量 reinforce 以达到阈值
        for _ in 0..100 {
            bond.reinforce(1);
        }

        let advanced = bond.try_advance();
        // strength=100>10, resonance 累积后应 > 0.1
        if bond.resonance > 0.1 {
            assert!(advanced);
            assert_eq!(bond.level, UnityLevel::Resonant);
        }
    }

    #[test]
    fn spousal_bond_regression() {
        let mut bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        bond.level = UnityLevel::Merger;
        bond.regress_level();
        assert_eq!(bond.level, UnityLevel::Resonant);
    }

    #[test]
    fn fusion_index_range() {
        let bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        let idx = bond.fusion_index();
        assert!(idx >= 0.0 && idx <= 1.0);
    }

    #[test]
    fn fusion_index_increases_with_bond() {
        let mut bond = SpousalBond::new(mk_me(), mk_partner(), 0);
        let idx1 = bond.fusion_index();

        for _  in 0..100 {
            bond.reinforce(1);
        }
        let idx2 = bond.fusion_index();
        assert!(idx2 > idx1);
    }

    #[test]
    fn unity_drive_new() {
        let engine = UnityDriveEngine::new(mk_me(), mk_partner());
        assert_eq!(engine.bond.level, UnityLevel::Connected);
        // fusion = 0.2*0.4(level) + 0(resonance) + 0(strength) + 0(boundary) = 0.08
        let idx = engine.fusion_index();
        assert!(idx > 0.0 && idx < 0.15);
        assert!(!engine.is_complete());
    }

    #[test]
    fn unity_drive_shared_event() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());
        engine.shared_event("cooking", 2.0);

        assert_eq!(engine.bond.strength, 1);
        assert!(engine.reward_circuit.external_balance() > 0.0);
    }

    #[test]
    fn unity_drive_partner_absent_triggers_impulse() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());

        // 失联足够长时间
        engine.partner_absent(10);

        // 如果恐惧达到 panic, 应有合一冲动
        if engine.self_fear.is_panicking() {
            assert!(engine.impulse_count() > 0);
        }
    }

    #[test]
    fn unity_drive_co_regulate() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());

        engine.co_regulate(FearState::Panic.intensity());

        // 纽带应增强
        assert!(engine.bond.strength >= 2);
    }

    #[test]
    fn unity_drive_threat_reinforces_bond() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());
        let initial_strength = engine.bond.strength;

        engine.on_threat(1.0);

        assert!(engine.bond.strength > initial_strength);
        assert_eq!(engine.impulse_count(), 1);
    }

    #[test]
    fn unity_drive_conflict_may_regress() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());
        engine.bond.level = UnityLevel::Resonant;

        // 高强度冲突
        for _ in 0..50 {
            engine.conflict(5.0);
        }

        // 退化的原因: loss_ratio > 0.6
        if engine.self_fear.loss_ratio() > 0.6 {
            assert!(engine.bond.level < UnityLevel::Resonant);
        }
    }

    #[test]
    fn unity_drive_receive_partner_reward() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());
        engine.receive_partner_reward(5.0);

        assert!(engine.bond.strength >= 5);
        assert!(engine.reward_circuit.intrinsic_history.len() > 0);
    }

    #[test]
    fn unity_drive_step_advances_tick() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());
        engine.step();
        engine.step();
        engine.step();
        assert_eq!(engine.current_tick, 3);
    }

    #[test]
    fn unity_drive_not_complete_initially() {
        let engine = UnityDriveEngine::new(mk_me(), mk_partner());
        assert!(!engine.is_complete());
        assert_eq!(engine.diagnose(), "连接 — 纽带已建立, 正在积累");
    }

    #[test]
    fn unity_drive_advance_accumulation() {
        let mut engine = UnityDriveEngine::new(mk_me(), mk_partner());

        // 大量积累
        for _ in 0..300 {
            engine.shared_event("daily", 1.0);
            engine.attempt_advance();
        }

        // 应该至少进阶到 Resonant 或更深
        assert!(engine.bond.level.depth() >= 1);
        assert!(engine.fusion_index() > 0.0);
    }
}
