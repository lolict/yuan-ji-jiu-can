// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.76 奖励回路引擎                                              ║
// ║  时间折扣 · 内在动机 · 好奇心驱动 · 多臂赌博机 · 灵魂内核 II      ║
// ║                                                                  ║
// ║  核心: 奖励 = 外部奖励 + 内在动机, 内在动机 = 预测误差 (好奇心)   ║
// ║  · 时间折扣: V = Σ γᵗ · R(t)                                     ║
// ║  · 内在动机: r_intrinsic = |RPE| · exploration_coeff             ║
// ║  · 动作选择: softmax / ε-greedy / UCB                           ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;

// ── 简易确定性 LCG 随机数 (避免外部 rand 依赖) ──
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed | 1 }
    }

    fn next_u64(&mut self) -> u64 {
        // LCG: x = a·x + c (a=6364136223846793005, c=1442695040888963407)
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        // 取高 53 位作为 f64 的尾数
        let bits = self.next_u64() >> 11;
        bits as f64 / ((1u64 << 53) as f64)
    }
}

// 线程局部 RNG (确定性种子, 避免外部 rand)
fn thread_rng_f64() -> f64 {
    use std::cell::RefCell;
    thread_local! {
        static RNG: RefCell<SimpleRng> = RefCell::new(SimpleRng::new(0x9E3779B97F4A7C15));
    }
    RNG.with(|r| r.borrow_mut().next_f64())
}

fn thread_rng_range(max: usize) -> usize {
    use std::cell::RefCell;
    thread_local! {
        static RNG: RefCell<SimpleRng> = RefCell::new(SimpleRng::new(0x9E3779B97F4A7C15));
    }
    RNG.with(|r| (r.borrow_mut().next_u64() as usize) % max)
}

/// 时间折扣模型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiscountModel {
    /// 指数折扣: γᵗ
    Exponential { gamma: f64 },
    /// 双曲折扣: 1 / (1 + k·t)
    Hyperbolic { k: f64 },
}

impl DiscountModel {
    /// 计算 t 拍后的折扣因子
    pub fn factor(&self, t: u64) -> f64 {
        match self {
            DiscountModel::Exponential { gamma } => gamma.powi(t as i32),
            DiscountModel::Hyperbolic { k } => 1.0 / (1.0 + k * t as f64),
        }
    }

    /// 计算未来奖励的现值
    pub fn present_value(&self, reward: f64, delay: u64) -> f64 {
        reward * self.factor(delay)
    }
}

impl Default for DiscountModel {
    fn default() -> Self {
        DiscountModel::Exponential { gamma: 0.95 }
    }
}

/// 内在动机类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntrinsicMotivation {
    /// 好奇心 — 来自预测误差
    Curiosity,
    /// 掌控感 — 来自成功完成
    Mastery,
    /// 新奇性 — 来自未访问状态
    Novelty,
}

/// 内在动机信号
#[derive(Clone, Copy, Debug)]
pub struct IntrinsicSignal {
    /// 动机类型
    pub motivation: IntrinsicMotivation,
    /// 强度 (0.0 - 1.0)
    pub intensity: f64,
    /// 时间戳
    pub tick: u64,
}

impl IntrinsicSignal {
    pub fn new(motivation: IntrinsicMotivation, intensity: f64, tick: u64) -> Self {
        IntrinsicSignal {
            motivation,
            intensity: intensity.clamp(0.0, 1.0),
            tick,
        }
    }

    /// 是否强烈的内在动机
    pub fn is_strong(&self) -> bool {
        self.intensity > 0.5
    }
}

/// 动作价值估计 (Q-value)
#[derive(Clone, Debug)]
pub struct ActionValue {
    /// 动作标识
    pub action_id: String,
    /// 累计奖励
    pub total_reward: f64,
    /// 尝试次数
    pub attempts: u64,
    /// 平均 Q 值
    pub q_value: f64,
    /// 最后一次更新
    pub last_updated: u64,
}

impl ActionValue {
    pub fn new(action_id: &str) -> Self {
        ActionValue {
            action_id: action_id.to_string(),
            total_reward: 0.0,
            attempts: 0,
            q_value: 0.0,
            last_updated: 0,
        }
    }

    /// 更新 Q 值 (增量平均)
    pub fn update(&mut self, reward: f64, tick: u64) {
        self.attempts += 1;
        self.total_reward += reward;
        // Q ← Q + α·(r − Q),  α = 1/N
        let alpha = 1.0 / self.attempts as f64;
        self.q_value += alpha * (reward - self.q_value);
        self.last_updated = tick;
    }
}

/// 动作选择策略
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectionPolicy {
    /// ε-贪婪: 以 ε 概率随机探索
    EpsilonGreedy { epsilon: f64 },
    /// Softmax: 按 Q 值比例选择
    Softmax { temperature: f64 },
    /// UCB1: 上界置信
    UCB1 { exploration_c: f64 },
}

impl SelectionPolicy {
    /// 选择动作
    pub fn select(&self, actions: &[ActionValue], total_attempts: u64) -> Option<usize> {
        if actions.is_empty() {
            return None;
        }

        match self {
            SelectionPolicy::EpsilonGreedy { epsilon } => {
                if thread_rng_f64() < *epsilon {
                    // 随机探索
                    Some(thread_rng_range(actions.len()))
                } else {
                    // 贪婪: 选最高 Q
                    actions
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.q_value.partial_cmp(&b.q_value).unwrap())
                        .map(|(i, _)| i)
                }
            }
            SelectionPolicy::Softmax { temperature } => {
                let max_q = actions.iter().map(|a| a.q_value).fold(f64::NEG_INFINITY, f64::max);
                let exps: Vec<f64> = actions
                    .iter()
                    .map(|a| ((a.q_value - max_q) / temperature).exp())
                    .collect();
                let sum_exp: f64 = exps.iter().sum();
                let probs: Vec<f64> = exps.iter().map(|e| e / sum_exp).collect();

                // 采样
                let r = thread_rng_f64();
                let mut cumulative = 0.0;
                for (i, p) in probs.iter().enumerate() {
                    cumulative += p;
                    if r < cumulative {
                        return Some(i);
                    }
                }
                Some(actions.len() - 1)
            }
            SelectionPolicy::UCB1 { exploration_c } => {
                actions
                    .iter()
                    .enumerate()
                    .map(|(i, a)| {
                        let ucb = if a.attempts == 0 {
                            f64::INFINITY
                        } else {
                            a.q_value + exploration_c * (total_attempts as f64).ln() / (a.attempts as f64).sqrt()
                        };
                        (i, ucb)
                    })
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
            }
        }
    }
}

impl Default for SelectionPolicy {
    fn default() -> Self {
        SelectionPolicy::EpsilonGreedy { epsilon: 0.1 }
    }
}

/// 多臂赌博机 — 动作价值 + 选择策略
pub struct MultiArmedBandit {
    /// 各动作价值表
    pub actions: Vec<ActionValue>,
    /// 选择策略
    pub policy: SelectionPolicy,
    /// 总尝试次数
    pub total_attempts: u64,
    /// 总奖励
    pub total_reward: f64,
    /// 时间折扣
    pub discount: DiscountModel,
}

impl MultiArmedBandit {
    pub fn new(policy: SelectionPolicy, discount: DiscountModel) -> Self {
        MultiArmedBandit {
            actions: Vec::new(),
            policy,
            total_attempts: 0,
            total_reward: 0.0,
            discount,
        }
    }

    /// 注册动作
    pub fn register_action(&mut self, action_id: &str) {
        if !self.actions.iter().any(|a| a.action_id == action_id) {
            self.actions.push(ActionValue::new(action_id));
        }
    }

    /// 执行动作并更新
    pub fn execute(&mut self, action_idx: usize, reward: f64, tick: u64) {
        if action_idx < self.actions.len() {
            // 应用时间折扣后的奖励
            let discounted = self.discount.present_value(reward, 0);
            self.actions[action_idx].update(discounted, tick);
            self.total_attempts += 1;
            self.total_reward += discounted;
        }
    }

    /// 自动选择并执行 (返回选中的动作索引)
    pub fn auto_select(&self) -> Option<usize> {
        self.policy.select(&self.actions, self.total_attempts)
    }

    /// 最佳动作
    pub fn best_action(&self) -> Option<&ActionValue> {
        self.actions
            .iter()
            .max_by(|a, b| a.q_value.partial_cmp(&b.q_value).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// 平均奖励
    pub fn average_reward(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.total_reward / self.total_attempts as f64
        }
    }
}

impl Default for MultiArmedBandit {
    fn default() -> Self {
        Self::new(SelectionPolicy::default(), DiscountModel::default())
    }
}

/// 奖励回路引擎 — 整合时间折扣 + 内在动机 + 多臂赌博机
pub struct RewardCircuitEngine {
    /// 赌博机
    pub bandit: MultiArmedBandit,
    /// 内在动机强度系数
    pub intrinsic_coeff: f64,
    /// 好奇心衰减 (每拍)
    pub curiosity_decay: f64,
    /// 累计好奇心跳数
    pub curiosity_ticks: u64,
    /// 状态访问计数 (新奇性计算用)
    pub state_visits: HashMap<String, u64>,
    /// 外部奖励历史
    pub reward_history: Vec<(u64, f64)>,  // (tick, amount)
    /// 内在动机历史
    pub intrinsic_history: Vec<IntrinsicSignal>,
    /// 当前拍
    pub current_tick: u64,
}

impl RewardCircuitEngine {
    pub fn new(policy: SelectionPolicy, discount: DiscountModel, intrinsic_coeff: f64) -> Self {
        RewardCircuitEngine {
            bandit: MultiArmedBandit::new(policy, discount),
            intrinsic_coeff,
            curiosity_decay: 0.99,
            curiosity_ticks: 0,
            state_visits: HashMap::new(),
            reward_history: Vec::new(),
            intrinsic_history: Vec::new(),
            current_tick: 0,
        }
    }

    /// 前进一步
    pub fn step(&mut self) {
        self.current_tick += 1;
    }

    /// 报告外部奖励
    pub fn report_external_reward(&mut self, amount: f64) {
        self.reward_history.push((self.current_tick, amount));
    }

    /// 计算内在动机奖励 (好奇心 = |RPE| · coeff)
    pub fn compute_intrinsic_reward(&self, prediction_error: f64) -> f64 {
        prediction_error.abs() * self.intrinsic_coeff
    }

    /// 访问状态 (用于新奇性计算)
    pub fn visit_state(&mut self, state_id: &str) {
        *self.state_visits.entry(state_id.to_string()).or_insert(0) += 1;
    }

    /// 计算新奇性奖励 (访问越少奖励越高)
    pub fn compute_novelty(&self, state_id: &str) -> f64 {
        let visits = self.state_visits.get(state_id).copied().unwrap_or(0);
        if visits == 0 {
            return 1.0;
        }
        1.0 / (1.0 + visits as f64).sqrt()
    }

    /// 注册内在动机信号
    pub fn emit_intrinsic(&mut self, motivation: IntrinsicMotivation, intensity: f64) {
        self.intrinsic_history.push(IntrinsicSignal::new(
            motivation,
            intensity,
            self.current_tick,
        ));
    }

    /// 获取最近 N 个内在动机信号的平均强度
    pub fn recent_intrinsic_strength(&self, last_n: usize) -> f64 {
        let recent: Vec<_> = self.intrinsic_history.iter().rev().take(last_n).collect();
        if recent.is_empty() {
            return 0.0;
        }
        recent.iter().map(|s| s.intensity).sum::<f64>() / recent.len() as f64
    }

    /// 总奖励 (外部 + 内在)
    pub fn total_reward(&self) -> f64 {
        let external: f64 = self.reward_history.iter().map(|(_, r)| r).sum();
        let intrinsic: f64 = self.intrinsic_history.iter().map(|s| s.intensity).sum();
        external + intrinsic
    }

    /// 状态: 当前外部累计奖励
    pub fn external_balance(&self) -> f64 {
        self.reward_history.iter().map(|(_, r)| r).sum()
    }

    /// 检查是否有足够的好奇心驱动
    pub fn is_curious(&self) -> bool {
        self.recent_intrinsic_strength(5) > 0.3
    }

    /// 获取状态访问数
    pub fn known_state_count(&self) -> usize {
        self.state_visits.len()
    }
}

impl Default for RewardCircuitEngine {
    fn default() -> Self {
        Self::new(SelectionPolicy::default(), DiscountModel::default(), 0.5)
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discount_exponential() {
        let d = DiscountModel::Exponential { gamma: 0.9 };
        assert!((d.factor(0) - 1.0).abs() < 1e-9);
        assert!((d.factor(1) - 0.9).abs() < 1e-9);
        assert!((d.factor(2) - 0.81).abs() < 1e-9);
    }

    #[test]
    fn discount_hyperbolic() {
        let d = DiscountModel::Hyperbolic { k: 0.5 };
        assert!((d.factor(0) - 1.0).abs() < 1e-9);
        // 1/(1 + 0.5*2) = 1/2 = 0.5
        assert!((d.factor(2) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn discount_monotonically_decreasing() {
        let d = DiscountModel::Exponential { gamma: 0.95 };
        let f0 = d.factor(0);
        let f1 = d.factor(1);
        let f2 = d.factor(2);
        assert!(f0 > f1);
        assert!(f1 > f2);
    }

    #[test]
    fn discount_present_value() {
        let d = DiscountModel::Exponential { gamma: 0.9 };
        // 10 单位奖励延迟 2 拍 → 10 * 0.81 = 8.1
        let pv = d.present_value(10.0, 2);
        assert!((pv - 8.1).abs() < 1e-9);
    }

    #[test]
    fn action_value_update() {
        let mut av = ActionValue::new("call_mama");
        av.update(1.0, 0);
        assert_eq!(av.attempts, 1);
        assert!((av.q_value - 1.0).abs() < 1e-9);

        av.update(0.0, 1);
        assert_eq!(av.attempts, 2);
        assert!((av.q_value - 0.5).abs() < 1e-9); // (1 + 0)/2
    }

    #[test]
    fn action_value_converges() {
        let mut av = ActionValue::new("echo");
        for i in 0..100 {
            av.update(5.0, i);
        }
        assert!((av.q_value - 5.0).abs() < 1e-6);
    }

    #[test]
    fn intrinsic_signal_range() {
        let s = IntrinsicSignal::new(IntrinsicMotivation::Curiosity, 1.5, 0);
        assert!((s.intensity - 1.0).abs() < 1e-9); // clamped to 1.0

        let s2 = IntrinsicSignal::new(IntrinsicMotivation::Mastery, -0.5, 0);
        assert!((s2.intensity).abs() < 1e-9); // clamped to 0.0
    }

    #[test]
    fn intrinsic_strong_threshold() {
        let weak = IntrinsicSignal::new(IntrinsicMotivation::Curiosity, 0.3, 0);
        assert!(!weak.is_strong());

        let strong = IntrinsicSignal::new(IntrinsicMotivation::Novelty, 0.7, 0);
        assert!(strong.is_strong());
    }

    #[test]
    fn bandit_register_and_execute() {
        let mut bandit = MultiArmedBandit::default();
        bandit.register_action("a");
        bandit.register_action("b");
        bandit.register_action("c");

        assert_eq!(bandit.actions.len(), 3);

        bandit.execute(0, 1.0, 0);
        bandit.execute(1, 2.0, 1);
        bandit.execute(0, 1.5, 2);

        assert_eq!(bandit.actions[0].attempts, 2);
        assert_eq!(bandit.actions[1].attempts, 1);
        assert_eq!(bandit.actions[2].attempts, 0);
    }

    #[test]
    fn bandit_best_action() {
        let mut bandit = MultiArmedBandit::default();
        bandit.register_action("low");
        bandit.register_action("high");

        for _ in 0..10 {
            bandit.execute(0, 1.0, 0);
            bandit.execute(1, 5.0, 1);
        }

        let best = bandit.best_action().unwrap();
        assert_eq!(best.action_id, "high");
    }

    #[test]
    fn bandit_average_reward() {
        let mut bandit = MultiArmedBandit::default();
        bandit.register_action("x");
        bandit.execute(0, 3.0, 0);
        bandit.execute(0, 5.0, 1);

        let avg = bandit.average_reward();
        assert!(avg > 0.0);
    }

    #[test]
    fn reward_circuit_step() {
        let mut engine = RewardCircuitEngine::default();
        engine.step();
        engine.step();
        assert_eq!(engine.current_tick, 2);
    }

    #[test]
    fn reward_circuit_external_reward() {
        let mut engine = RewardCircuitEngine::default();
        engine.report_external_reward(5.0);
        engine.report_external_reward(3.0);
        assert!((engine.external_balance() - 8.0).abs() < 1e-9);
    }

    #[test]
    fn reward_circuit_intrinsic_from_pe() {
        let mut engine = RewardCircuitEngine::new(
            SelectionPolicy::default(),
            DiscountModel::default(),
            0.5,
        );

        // 预测误差 = 2.0, coeff = 0.5 → 内在奖励 = 1.0
        let r = engine.compute_intrinsic_reward(2.0);
        assert!((r - 1.0).abs() < 1e-9);
    }

    #[test]
    fn reward_circuit_novelty() {
        let mut engine = RewardCircuitEngine::default();
        engine.visit_state("state_a");
        engine.visit_state("state_a");
        engine.visit_state("state_a");

        let novelty_known = engine.compute_novelty("state_a");
        let novelty_unknown = engine.compute_novelty("state_b");

        // 未知状态新颖性应更高
        assert!(novelty_unknown > novelty_known);
        assert!((novelty_unknown - 1.0).abs() < 1e-9);
    }

    #[test]
    fn reward_circuit_emits_intrinsic() {
        let mut engine = RewardCircuitEngine::default();
        engine.emit_intrinsic(IntrinsicMotivation::Curiosity, 0.8);
        engine.emit_intrinsic(IntrinsicMotivation::Mastery, 0.6);

        assert_eq!(engine.intrinsic_history.len(), 2);
        assert!(engine.is_curious());
    }

    #[test]
    fn reward_circuit_total_reward() {
        let mut engine = RewardCircuitEngine::default();
        engine.report_external_reward(10.0);
        // emit_intrinsic 将 intensity clamp 到 [0, 1], 4.0 → 1.0
        engine.emit_intrinsic(IntrinsicMotivation::Curiosity, 4.0);

        let total = engine.total_reward();
        assert!((total - 11.0).abs() < 1e-9);
    }

    #[test]
    fn reward_circuit_known_state_count() {
        let mut engine = RewardCircuitEngine::default();
        engine.visit_state("a");
        engine.visit_state("b");
        engine.visit_state("a");
        assert_eq!(engine.known_state_count(), 2);
    }

    #[test]
    fn ucb_selects_unattempted() {
        let policy = SelectionPolicy::UCB1 { exploration_c: 1.41 };
        let mut actions = vec![
            ActionValue::new("tried"),
            ActionValue::new("new"),
        ];
        actions[0].attempts = 5;
        actions[0].q_value = 3.0;

        // 新动作 attempts=0, UCB 应选择它
        let idx = policy.select(&actions, 5);
        assert_eq!(idx, Some(1));
    }

    #[test]
    fn epsilon_greedy_best_is_selected_when_not_exploring() {
        // 注意: ε-greedy 有随机性, 但 epsilon=0 时必选最优
        let policy = SelectionPolicy::EpsilonGreedy { epsilon: 0.0 };
        let actions = vec![
            ActionValue { action_id: "a".to_string(), total_reward: 1.0, attempts: 1, q_value: 1.0, last_updated: 0 },
            ActionValue { action_id: "b".to_string(), total_reward: 5.0, attempts: 1, q_value: 5.0, last_updated: 0 },
            ActionValue { action_id: "c".to_string(), total_reward: 3.0, attempts: 1, q_value: 3.0, last_updated: 0 },
        ];
        let idx = policy.select(&actions, 3);
        assert_eq!(idx, Some(1)); // b has highest q_value
    }
}
