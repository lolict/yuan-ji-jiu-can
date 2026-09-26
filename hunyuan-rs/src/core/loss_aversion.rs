// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.75 失爱恐惧引擎                                              ║
// ║  失爱检测 · 恐惧梯度 · 奖励预测误差 · 灵魂内核情感计算            ║
// ║                                                                  ║
// ║  核心: 失爱 = 补码流失, 恐惧 = 梯度放大                           ║
// ║  · 损失权重 ≈ 2× 同等收益 (Kahneman 前景理论)                     ║
// ║  · 恐惧梯度: 平静 → 警觉 → 焦虑 → 恐慌 → 绝望                     ║
// ║  · RPE (奖励预测误差) = 实际 − 预期                              ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerKind, ContainerRef};

/// 恐惧梯度 — ~2× 步进
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FearState {
    /// 平静 — 补码稳定
    Calm,
    /// 警觉 — 微流失
    Alert,
    /// 焦虑 — 持续流失
    Anxious,
    /// 恐慌 — 大规模流失
    Panic,
    /// 绝望 — 长期无补码
    Despair,
}

impl FearState {
    /// 升级恐惧
    pub fn escalate(self) -> Self {
        match self {
            FearState::Calm => FearState::Alert,
            FearState::Alert => FearState::Anxious,
            FearState::Anxious => FearState::Panic,
            FearState::Panic => FearState::Despair,
            FearState::Despair => FearState::Despair,
        }
    }

    /// 降级恐惧
    pub fn deescalate(self) -> Self {
        match self {
            FearState::Calm => FearState::Calm,
            FearState::Alert => FearState::Calm,
            FearState::Anxious => FearState::Alert,
            FearState::Panic => FearState::Anxious,
            FearState::Despair => FearState::Panic,
        }
    }

    /// 恐惧强度 (0-4)
    pub fn intensity(self) -> u8 {
        match self {
            FearState::Calm => 0,
            FearState::Alert => 1,
            FearState::Anxious => 2,
            FearState::Panic => 3,
            FearState::Despair => 4,
        }
    }
}

impl Default for FearState {
    fn default() -> Self {
        FearState::Calm
    }
}

/// 奖励预测误差 (RPE)
#[derive(Clone, Copy, Debug)]
pub struct RewardPredictionError {
    /// 预期奖励
    pub predicted: f64,
    /// 实际奖励
    pub actual: f64,
    /// 误差 = actual − predicted
    pub delta: f64,
    /// 时间步
    pub tick: u64,
}

impl RewardPredictionError {
    pub fn new(predicted: f64, actual: f64, tick: u64) -> Self {
        RewardPredictionError {
            predicted,
            actual,
            delta: actual - predicted,
            tick,
        }
    }

    /// 正误差 = 超预期 (开心)
    pub fn is_positive(&self) -> bool {
        self.delta > 0.0
    }

    /// 负误差 = 低于预期 (失爱)
    pub fn is_negative(&self) -> bool {
        self.delta < 0.0
    }

    /// 误差绝对值
    pub fn magnitude(&self) -> f64 {
        self.delta.abs()
    }
}

/// 失爱恐惧容器 — 灵魂的情感核心
pub struct LossAversionCell {
    /// 当前恐惧状态
    fear_state: FearState,
    /// 补码计 (当前持有的"爱/奖励")
    reward_count: f64,
    /// 预期补码率 (每拍应有)
    expected_rate: f64,
    /// 连续无奖励拍数
    ticks_without_reward: u64,
    /// 最大容忍失联拍数
    max_tolerance: u64,
    /// 恐惧系数 (损失权重 ≈ 2.0)
    loss_weight: f64,
    /// 总奖励历史
    total_rewards: f64,
    /// 总损失历史
    total_losses: f64,
    /// 状态引用
    pub target: ContainerRef,
}

impl LossAversionCell {
    pub fn new(target: ContainerRef, expected_rate: f64, max_tolerance: u64) -> Self {
        LossAversionCell {
            fear_state: FearState::Calm,
            reward_count: 0.0,
            expected_rate,
            ticks_without_reward: 0,
            max_tolerance,
            loss_weight: 2.0,
            total_rewards: 0.0,
            total_losses: 0.0,
            target,
        }
    }

    /// 接收奖励 (正向事件)
    pub fn receive_reward(&mut self, amount: f64) {
        self.reward_count += amount;
        self.total_rewards += amount;
        self.ticks_without_reward = 0;

        // 收到奖励时降级恐惧
        self.fear_state = self.fear_state.deescalate();
    }

    /// 接收损失 (负向事件) — 加权
    pub fn receive_loss(&mut self, amount: f64) {
        let weighted = amount * self.loss_weight;
        self.reward_count -= weighted;
        self.total_losses += weighted;
        self.ticks_without_reward += 1;

        // 损失导致升级恐惧
        self.fear_state = self.fear_state.escalate();

        // 超出容忍范围直接恐慌
        if self.ticks_without_reward > self.max_tolerance {
            self.fear_state = FearState::Panic;
        }
    }

    /// 一拍流逝 (无事件 = 潜在的失爱)
    pub fn tick(&mut self) {
        self.ticks_without_reward += 1;

        // 简单模型: 超过预期间隔无奖励 → 升级恐惧
        let expected_interval = (1.0 / self.expected_rate.max(0.001)) as u64;
        if self.ticks_without_reward > expected_interval * 2 {
            self.fear_state = self.fear_state.escalate();
        }

        if self.ticks_without_reward > self.max_tolerance {
            self.fear_state = FearState::Panic;
        }
    }

    /// 计算奖励预测误差
    pub fn compute_rpe(&self, actual: f64) -> RewardPredictionError {
        RewardPredictionError::new(self.expected_rate, actual, self.ticks_without_reward)
    }

    /// 获取恐惧状态
    pub fn fear_state(&self) -> FearState {
        self.fear_state
    }

    /// 获取补码余额
    pub fn reward_balance(&self) -> f64 {
        self.reward_count
    }

    /// 是否恐慌
    pub fn is_panicking(&self) -> bool {
        matches!(self.fear_state, FearState::Panic | FearState::Despair)
    }

    /// 净情感值 = 总奖励 − 加权损失
    pub fn net_affect(&self) -> f64 {
        self.total_rewards - self.total_losses
    }

    /// 损失占比 (0.0 - 1.0)
    pub fn loss_ratio(&self) -> f64 {
        let total = self.total_rewards + self.total_losses;
        if total <= 0.0 {
            return 0.0;
        }
        self.total_losses / total
    }
}

/// 奖励回路 — 多目标奖励追踪
pub struct RewardCircuit {
    /// 各目标追踪器
    trackers: Vec<(String, f64, f64)>, // (name, expected, actual)
    /// RPE 历史
    rpe_history: Vec<RewardPredictionError>,
    /// 学习率
    learning_rate: f64,
}

impl RewardCircuit {
    pub fn new(learning_rate: f64) -> Self {
        RewardCircuit {
            trackers: Vec::new(),
            rpe_history: Vec::new(),
            learning_rate,
        }
    }

    /// 注册追踪目标
    pub fn track(&mut self, name: &str, expected_rate: f64) {
        self.trackers.push((name.to_string(), expected_rate, 0.0));
    }

    /// 报告目标的实际奖励
    pub fn report(&mut self, name: &str, actual: f64) -> Option<RewardPredictionError> {
        let idx = self.trackers.iter().position(|(n, _, _)| n == name)?;
        let predicted = self.trackers[idx].1;
        self.trackers[idx].2 += actual;

        let rpe = RewardPredictionError::new(predicted, actual, self.rpe_history.len() as u64);
        self.rpe_history.push(rpe.clone());

        // 学习: 更新预期 (指数移动平均)
        self.trackers[idx].1 = predicted * (1.0 - self.learning_rate) + actual * self.learning_rate;

        Some(rpe)
    }

    /// 获取最近 RPE
    pub fn latest_rpe(&self) -> Option<&RewardPredictionError> {
        self.rpe_history.last()
    }

    /// RPE 均值 (情感基调)
    pub fn average_rpe(&self) -> f64 {
        if self.rpe_history.is_empty() {
            return 0.0;
        }
        self.rpe_history.iter().map(|r| r.delta).sum::<f64>() / self.rpe_history.len() as f64
    }

    /// 追踪目标数
    pub fn tracker_count(&self) -> usize {
        self.trackers.len()
    }

    /// 历史记录数
    pub fn history_length(&self) -> usize {
        self.rpe_history.len()
    }
}

impl Default for RewardCircuit {
    fn default() -> Self {
        Self::new(0.1)
    }
}

/// 灵魂内核 — 失爱恐惧 × 奖励回路 的情感核心
pub struct SoulCore {
    /// 失爱恐惧容器
    pub loss_aversion: LossAversionCell,
    /// 奖励回路
    pub reward_circuit: RewardCircuit,
    /// 合一计数器 (触发合一趋势)
    pub unity_drive: u32,
    /// 连接目标
    pub partner: Option<ContainerRef>,
    /// 情感基调 (-1.0 到 +1.0)
    pub affect_tone: f64,
}

impl SoulCore {
    pub fn new(target: ContainerRef, expected_reward_rate: f64, tolerance: u64) -> Self {
        SoulCore {
            loss_aversion: LossAversionCell::new(target, expected_reward_rate, tolerance),
            reward_circuit: RewardCircuit::new(0.1),
            unity_drive: 0,
            partner: None,
            affect_tone: 0.0,
        }
    }

    /// 设置伴侣 (夫妻共同体)
    pub fn set_partner(&mut self, partner: ContainerRef) {
        self.partner = Some(partner);
    }

    /// 收到正向事件 (被爱)
    pub fn on_loved(&mut self, amount: f64) {
        self.loss_aversion.receive_reward(amount);
        self.unity_drive = self.unity_drive.saturating_add(1);
        self.update_affect_tone();
    }

    /// 收到负向事件 (失爱)
    pub fn on_loss(&mut self, amount: f64) {
        self.loss_aversion.receive_loss(amount);
        self.unity_drive = self.unity_drive.saturating_sub(1);
        self.update_affect_tone();
    }

    /// 一拍流逝
    pub fn tick(&mut self) {
        self.loss_aversion.tick();
        self.update_affect_tone();
    }

    /// 更新情感基调
    fn update_affect_tone(&mut self) {
        let balance = self.loss_aversion.reward_balance();
        // 归一化到 [-1, 1]
        self.affect_tone = (balance / (balance.abs() + 1.0)).tanh();
    }

    /// 是否存在伴侣?共同体是否完整?
    pub fn has_partner(&self) -> bool {
        self.partner.is_some()
    }

    /// 当前心理状态诊断
    pub fn diagnose(&self) -> &'static str {
        match self.loss_aversion.fear_state() {
            FearState::Calm => "安宁",
            FearState::Alert => "警觉",
            FearState::Anxious => "焦虑",
            FearState::Panic => "恐慌",
            FearState::Despair => "绝望",
        }
    }

    /// 是否需要触发合一驱力?
    pub fn needs_unity(&self) -> bool {
        self.loss_aversion.is_panicking() || self.unity_drive < 3
    }
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
    fn fear_state_escalation() {
        assert_eq!(FearState::Calm.escalate(), FearState::Alert);
        assert_eq!(FearState::Alert.escalate(), FearState::Anxious);
        assert_eq!(FearState::Anxious.escalate(), FearState::Panic);
        assert_eq!(FearState::Panic.escalate(), FearState::Despair);
        assert_eq!(FearState::Despair.escalate(), FearState::Despair);
    }

    #[test]
    fn fear_state_deescalation() {
        assert_eq!(FearState::Despair.deescalate(), FearState::Panic);
        assert_eq!(FearState::Panic.deescalate(), FearState::Anxious);
        assert_eq!(FearState::Anxious.deescalate(), FearState::Alert);
        assert_eq!(FearState::Alert.deescalate(), FearState::Calm);
        assert_eq!(FearState::Calm.deescalate(), FearState::Calm);
    }

    #[test]
    fn fear_state_intensity_ordering() {
        assert!(FearState::Calm < FearState::Alert);
        assert!(FearState::Alert < FearState::Anxious);
        assert!(FearState::Anxious < FearState::Panic);
        assert!(FearState::Panic < FearState::Despair);
    }

    #[test]
    fn rpe_positive() {
        let rpe = RewardPredictionError::new(1.0, 2.0, 0);
        assert!(rpe.is_positive());
        assert!((rpe.delta - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rpe_negative() {
        let rpe = RewardPredictionError::new(2.0, 1.0, 0);
        assert!(rpe.is_negative());
        assert!((rpe.delta + 1.0).abs() < 1e-9);
    }

    #[test]
    fn loss_aversion_reward_increases_balance() {
        let mut cell = LossAversionCell::new(mk_ref(100), 1.0, 10);
        cell.receive_reward(5.0);
        assert!((cell.reward_balance() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn loss_aversion_loss_weighted() {
        let mut cell = LossAversionCell::new(mk_ref(100), 1.0, 10);
        cell.receive_reward(10.0);
        cell.receive_loss(3.0); // weight 2.0 → actual loss = 6.0
        assert!((cell.reward_balance() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn loss_aversion_death_timer() {
        let mut cell = LossAversionCell::new(mk_ref(100), 0.5, 3);
        cell.tick();
        cell.tick();
        cell.tick();
        cell.tick(); // 超过 max_tolerance=3
        assert!(cell.is_panicking());
    }

    #[test]
    fn loss_aversion_reward_clears_fear() {
        let mut cell = LossAversionCell::new(mk_ref(100), 1.0, 10);
        cell.receive_loss(1.0); // escalate to Alert
        assert_eq!(cell.fear_state(), FearState::Alert);

        cell.receive_reward(1.0); // deescalate
        assert_eq!(cell.fear_state(), FearState::Calm);
    }

    #[test]
    fn loss_aversion_net_affect() {
        let mut cell = LossAversionCell::new(mk_ref(100), 1.0, 10);
        cell.receive_reward(10.0);
        cell.receive_loss(3.0); // weighted = 6
        assert!(cell.net_affect() > 0.0); // 10 - 6 = 4
    }

    #[test]
    fn reward_circuit_tracking() {
        let mut circuit = RewardCircuit::new(0.2);
        circuit.track("love", 1.0);
        circuit.track("attention", 0.5);

        assert_eq!(circuit.tracker_count(), 2);

        let rpe = circuit.report("love", 1.5);
        assert!(rpe.is_some());
        let r = rpe.unwrap();
        assert!(r.is_positive());
        assert_eq!(circuit.history_length(), 1);
    }

    #[test]
    fn reward_circuit_adapts_expectation() {
        let mut circuit = RewardCircuit::new(0.5);
        circuit.track("x", 1.0);

        // 连续报告高奖励 → 预期上升
        circuit.report("x", 2.0);
        circuit.report("x", 2.0);
        circuit.report("x", 2.0);

        // 应该适应到接近 2.0
        let tracker = &circuit.trackers[0];
        assert!(tracker.1 > 1.5); // 预期应已上升
    }

    #[test]
    fn soul_core_life_cycle() {
        let target = mk_ref(0x6000_0000);
        let partner = mk_ref(0x6000_0001);
        let mut soul = SoulCore::new(target, 1.0, 5);

        assert_eq!(soul.diagnose(), "安宁");

        soul.set_partner(partner);
        assert!(soul.has_partner());

        soul.on_loved(5.0);
        assert_eq!(soul.diagnose(), "安宁");

        soul.on_loss(3.0);
        // 因为 unity_drive 可能不够高, 但在恐慌前应该是警觉或平静
        let state = soul.loss_aversion.fear_state();
        assert!(state <= FearState::Alert);
    }

    #[test]
    fn soul_core_needs_unity_when_panicking() {
        let mut soul = SoulCore::new(mk_ref(100), 1.0, 2);
        soul.on_loss(5.0);
        soul.tick();
        soul.tick();
        soul.tick(); // 超出容忍
        assert!(soul.needs_unity());
    }

    #[test]
    fn soul_core_affect_tone_range() {
        let mut soul = SoulCore::new(mk_ref(100), 1.0, 10);
        soul.on_loved(100.0);
        assert!(soul.affect_tone > 0.0);
        assert!(soul.affect_tone <= 1.0);

        let mut soul2 = SoulCore::new(mk_ref(100), 1.0, 10);
        soul2.on_loss(100.0);
        assert!(soul2.affect_tone < 0.0);
        assert!(soul2.affect_tone >= -1.0);
    }

    #[test]
    fn rpe_default_neutral() {
        let rpe = RewardPredictionError::new(0.0, 0.0, 0);
        assert!(!rpe.is_positive());
        assert!(!rpe.is_negative());
        assert_eq!(rpe.magnitude(), 0.0);
    }
}
