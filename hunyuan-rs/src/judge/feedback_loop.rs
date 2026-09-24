// ══════════════════════════════════════════════════════════════════
// feedback_loop.rs — 反馈闭环 · 系统自适应 · 感知自修改
// ══════════════════════════════════════════════════════════════════
//
//   「感知 → 判断 → 评价 → 逻辑组织 → 反馈 → 感知 (闭环)」
//
//   闭环机制:
//   1. 判断引擎产生评价, 输出到规则库
//   2. 逻辑组织将规则按公约数排序, 形成层级
//   3. 反馈器读取层级结构, 生成三类回输:
//      a. 感知增强: 高频正确的源 → 提升灵敏度
//      b. 感知抑制: 低频/错误的源 → 降低灵敏度
//      c. 规则升级: 低层级规则表现良好 → 升级为高层级管辖者
//   4. 回输作用到判断引擎的感知层, 改变下次感知的"注意力分配"
//
//   核心哲学 (源自满全法):
//   - 系统不是被动接收, 而是主动塑造自己的感知偏好
//   - 固化规则成为"先验", 未固化规则仍是"经验"
//   - 反馈不是纠正, 是"迂回"让系统绕过自身盲区
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::hash::Hash;

use crate::judgment_power::{Evaluation, EvaluationLevel, FeedbackSignal, Feedback};
use crate::logical_org::{LogicalOrg, RuleNode, RelationResolver};

// ────────────────────────────────────────────────────────────
// 感知调节器 — 注意力分配的自适应
// ────────────────────────────────────────────────────────────

/// 感知通道的灵敏度状态
#[derive(Clone, Debug)]
pub struct PerceptionChannel {
    pub source_id: String,
    pub base_sensitivity: f64,    // 原始灵敏度 (出厂值)
    pub current_sensitivity: f64, // 当前灵敏度 (经反馈调节)
    pub enhance_count: u64,       // 被增强次数
    pub suppress_count: u64,      // 被抑制次数
    pub accuracy: f64,            // 历史准确率 (正确预警 / 总预警)
}

impl PerceptionChannel {
    pub fn new(source_id: &str, base: f64) -> Self {
        Self {
            source_id: source_id.into(),
            base_sensitivity: base,
            current_sensitivity: base,
            enhance_count: 0,
            suppress_count: 0,
            accuracy: 0.5,
        }
    }

    /// 增强灵敏度 (该源最近提供了有价值的信息)
    pub fn enhance(&mut self, factor: f64) {
        self.current_sensitivity *= (1.0 + factor).min(10.0);
        self.current_sensitivity = self.current_sensitivity.min(100.0); // 上限
        self.enhance_count += 1;
    }

    /// 抑制灵敏度 (该源最近产生了大量噪声)
    pub fn suppress(&mut self, factor: f64) {
        self.current_sensitivity *= (1.0 - factor).max(0.01);
        self.current_sensitivity = self.current_sensitivity.max(0.01); // 下限
        self.suppress_count += 1;
    }

    /// 重置感知通道 (系统重启/模式切换时)
    pub fn reset(&mut self) {
        self.current_sensitivity = self.base_sensitivity;
        self.enhance_count = 0;
        self.suppress_count = 0;
    }

    /// 漂移回中 (长时间未使用 → 缓慢回归基准)
    pub fn decay(&mut self) {
        let drift = (self.base_sensitivity - self.current_sensitivity) * 0.1;
        self.current_sensitivity += drift;
    }
}

// ────────────────────────────────────────────────────────────
// 反馈器 — 闭环的核心组件
// ────────────────────────────────────────────────────────────

/// 反馈决策
#[derive(Clone, Debug)]
pub enum FeedbackDecision {
    /// 增强某个感知通道
    EnhanceChannel(String, f64), // (source_id, factor)
    /// 抑制某个感知通道
    SuppressChannel(String, f64), // (source_id, factor)
    /// 升级规则层级 (将规则提升到更高管辖位置)
    UpgradeRule(u64, u64), // (rule_id, new_parent_id)
    /// 降级规则层级
    DemoteRule(u64, u64),
    /// 分裂规则 (一条规则太宽泛 → 拆为两条)
    SplitRule(u64, u64, u64), // (old_id, new_id_a, new_id_b)
    /// 维持现状
    Maintain,
}

impl std::fmt::Display for FeedbackDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedbackDecision::EnhanceChannel(s, v) => write!(f, "增强感知[{}]×{:.2}", s, v),
            FeedbackDecision::SuppressChannel(s, v) => write!(f, "抑制感知[{}]×{:.2}", s, v),
            FeedbackDecision::UpgradeRule(id, p) => write!(f, "升级规则{}→父{}", id, p),
            FeedbackDecision::DemoteRule(id, p) => write!(f, "降级规则{}→父{}", id, p),
            FeedbackDecision::SplitRule(old, a, b) => write!(f, "分裂规则{}→{},{}", old, a, b),
            FeedbackDecision::Maintain => write!(f, "维持"),
        }
    }
}

/// 反馈闭环控制器
///
/// 接收判断引擎的输出 + 逻辑组织的当前状态,
/// 做出反馈决策, 作用回判断引擎的感知层
pub struct FeedbackLoop {
    /// 感知通道注册表
    pub channels: HashMap<String, PerceptionChannel>,
    /// 历史决策记录
    pub decisions: Vec<FeedbackDecision>,
    /// 增强阈值: 规则激活次数超过此值 → 增强对应感知
    pub enhance_threshold: u64,
    /// 抑制阈值: 通道准确率低于此值 → 抑制
    pub suppress_accuracy_threshold: f64,
    /// 已做出的总反馈次数
    pub total_feedback_count: u64,
}

impl FeedbackLoop {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            decisions: Vec::new(),
            enhance_threshold: 5,
            suppress_accuracy_threshold: 0.3,
            total_feedback_count: 0,
        }
    }

    /// 注册感知通道
    pub fn register_channel(&mut self, source_id: &str, base_sensitivity: f64) {
        self.channels.insert(
            source_id.into(),
            PerceptionChannel::new(source_id, base_sensitivity),
        );
    }

    /// 处理评价集合, 做出反馈决策
    pub fn process_evaluations<T: Clone + Eq + Hash>(
        &mut self,
        evaluations: &[Evaluation<T>],
        org: &LogicalOrg,
    ) -> Vec<FeedbackDecision> {
        let mut new_decisions = Vec::new();

        for eval in evaluations {
            // 策略1: 评价权重高的因果对 → 增强对应源
            if eval.weight > 0.7 {
                let source_key = format!("{}_effect", eval.causal_pair.effect);
                if self.channels.contains_key(&source_key) {
                    let decision = FeedbackDecision::EnhanceChannel(source_key.clone(), 0.2);
                    if let Some(ch) = self.channels.get_mut(&source_key) {
                        ch.enhance(0.2);
                    }
                    new_decisions.push(decision);
                }
            }

            // 策略2: 评价等级为 Noise 且连续多次 → 抑制
            if eval.level == EvaluationLevel::Noise && eval.weight < 0.1 {
                let source_key = format!("{}_cause", eval.causal_pair.cause);
                if let Some(ch) = self.channels.get_mut(&source_key) {
                    ch.accuracy = ch.accuracy * 0.9; // 噪声降低准确率
                    if ch.accuracy < self.suppress_accuracy_threshold {
                        ch.suppress(0.3);
                        new_decisions.push(FeedbackDecision::SuppressChannel(source_key, 0.3));
                    }
                }
            }
        }

        // 策略3: 检查逻辑组织中的规则层级, 决定是否需要升级/降级
        let _resolver = RelationResolver::new(org);
        let ranked = org.ranked_rules();

        for (i, node) in ranked.iter().enumerate() {
            // 排名靠前 + 激活次数高 + 未固化 → 考虑升级
            if i < 3 && node.activation_count > 10 && !node.solidified {
                if let Some(top) = org.top_authority() {
                    if top.rule_id != node.rule_id {
                        new_decisions.push(FeedbackDecision::UpgradeRule(
                            node.rule_id,
                            top.rule_id,
                        ));
                    }
                }
            }
        }

        // 维持决策 (如果什么都没有)
        if new_decisions.is_empty() {
            new_decisions.push(FeedbackDecision::Maintain);
        }

        // 记录并返回
        self.decisions.extend(new_decisions.clone());
        self.total_feedback_count += new_decisions.len() as u64;
        new_decisions
    }

    /// 将判断引擎的反馈信号作用到感知通道
    pub fn apply_signals(&mut self, signals: &[FeedbackSignal]) {
        for signal in signals {
            match signal {
                crate::judgment_power::FeedbackSignal::EnhancePerception(source_id) => {
                    if let Some(ch) = self.channels.get_mut(source_id) {
                        ch.enhance(0.3);
                    }
                }
                crate::judgment_power::FeedbackSignal::SuppressPerception(source_id) => {
                    if let Some(ch) = self.channels.get_mut(source_id) {
                        ch.suppress(0.3);
                    }
                }
                crate::judgment_power::FeedbackSignal::ExpandObservation(source_id) => {
                    // 新增观察通道
                    self.register_channel(source_id, 1.0);
                }
                _ => {} // ModifyRule 和 Maintain 不直接影响感知
            }
        }
    }

    /// 全通道衰减 (每个周期结束时调用)
    pub fn decay_all(&mut self) {
        for ch in self.channels.values_mut() {
            ch.decay();
        }
    }

    /// 获取最灵敏的感知通道 (当前注意力焦点)
    pub fn top_channels(&self, n: usize) -> Vec<&PerceptionChannel> {
        let mut channels: Vec<&PerceptionChannel> = self.channels.values().collect();
        channels.sort_by(|a, b| {
            b.current_sensitivity
                .partial_cmp(&a.current_sensitivity)
                .unwrap()
        });
        channels.into_iter().take(n).collect()
    }

    /// 生成 Feedback 对象 (用于判断引擎)
    pub fn to_feedback(decisions: &[FeedbackDecision]) -> Feedback {
        let signals: Vec<FeedbackSignal> = decisions
            .iter()
            .filter_map(|d| match d {
                FeedbackDecision::EnhanceChannel(s, _) => {
                    Some(FeedbackSignal::EnhancePerception(s.clone()))
                }
                FeedbackDecision::SuppressChannel(s, _) => {
                    Some(FeedbackSignal::SuppressPerception(s.clone()))
                }
                _ => None,
            })
            .collect();

        Feedback::with_signals(
            signals,
            &format!("反馈闭环: {} 项决策", decisions.len()),
        )
    }
}

impl Default for FeedbackLoop {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 闭环系统 — 判断引擎 + 逻辑组织 + 反馈器 三组件集成
// ────────────────────────────────────────────────────────────

/// 闭环系统 — 将三个组件连接成一个自适应循环
///
/// 使用方式:
///   1. 系统接收输入 → engine.perceive()
///   2. 执行判断 → engine.run_cycle()
/// 3. 组织结果 → org.insert(rule_node)
/// 4. 反馈处理 → loop.process_evaluations()
/// 5. 感知调整 → loop.decay_all()
/// 6. 重复 (系统自适应)
pub struct ClosedLoopSystem<T: Clone + Eq + Hash> {
    pub(crate) org: LogicalOrg,
    pub(crate) loop_ctrl: FeedbackLoop,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Eq + Hash> ClosedLoopSystem<T> {
    pub fn new() -> Self {
        Self {
            org: LogicalOrg::new(),
            loop_ctrl: FeedbackLoop::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// 注册感知通道
    pub fn register_perception(&mut self, source_id: &str, base: f64) {
        self.loop_ctrl.register_channel(source_id, base);
    }

    /// 处理一批评价, 返回反馈决策
    pub fn process(
        &mut self,
        evaluations: &[Evaluation<T>],
    ) -> Vec<FeedbackDecision> {
        self.loop_ctrl.process_evaluations(evaluations, &self.org)
    }

    /// 向逻辑组织添加规则节点
    pub fn add_rule(&mut self, node: RuleNode) -> u64 {
        self.org.insert(node)
    }

    /// 固化规则
    pub fn solidify_rule(&mut self, id: u64) -> bool {
        self.org.solidify(id)
    }

    /// 衰减所有通道
    pub fn decay(&mut self) {
        self.loop_ctrl.decay_all();
    }

    /// 获取系统状态快照
    pub fn status(&self) -> LoopStatus {
        LoopStatus {
            rule_count: self.org.nodes.len(),
            frozen_count: self.org.frozen.len(),
            channel_count: self.loop_ctrl.channels.len(),
            total_decisions: self.loop_ctrl.total_feedback_count,
            tree_depth: self.org.depth(),
            root_count: self.org.root_count(),
        }
    }
}

impl<T: Clone + Eq + Hash> Default for ClosedLoopSystem<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 系统状态快照
#[derive(Clone, Debug)]
pub struct LoopStatus {
    pub rule_count: usize,
    pub frozen_count: usize,
    pub channel_count: usize,
    pub total_decisions: u64,
    pub tree_depth: usize,
    pub root_count: usize,
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::judgment_power::{CausalPair, RippleDiff, DiffKind};

    #[test]
    fn perception_channel_enhance() {
        let mut ch = PerceptionChannel::new("sensor_a", 1.0);
        ch.enhance(0.5);
        assert!((ch.current_sensitivity - 1.5).abs() < 1e-6);
        assert_eq!(ch.enhance_count, 1);
    }

    #[test]
    fn perception_channel_suppress() {
        let mut ch = PerceptionChannel::new("sensor_b", 1.0);
        ch.suppress(0.5);
        assert!((ch.current_sensitivity - 0.5).abs() < 1e-6);
        assert_eq!(ch.suppress_count, 1);
    }

    #[test]
    fn perception_channel_decay() {
        let mut ch = PerceptionChannel::new("sensor_c", 1.0);
        ch.enhance(1.0); // → 2.0
        assert!(ch.current_sensitivity > 1.0);
        // 衰减 10 次, 应该向 1.0 回归
        for _ in 0..100 {
            ch.decay();
        }
        assert!((ch.current_sensitivity - 1.0).abs() < 0.01);
    }

    #[test]
    fn feedback_loop_process_evaluations() {
        let mut loop_ctrl = FeedbackLoop::new();
        // 通道名格式: "{effect}_effect" (由代码自动生成)
        loop_ctrl.register_channel("result_effect", 1.0);
        loop_ctrl.register_channel("sensor_x_cause", 1.0);

        // 高权重评价 (Alert, magnitude=0.9) → weight=0.9 > 0.7 → 增强
        let pair: CausalPair<&str> = CausalPair::new(
            RippleDiff {
                from_ts: 0, to_ts: 1,
                source_id: "sensor_x".into(),
                amp_delta: 0.9, phase_delta: 0.0,
                kind: DiffKind::Amplitude,
            },
            "sensor_x", "result"
        );
        let eval = Evaluation::new(pair, EvaluationLevel::Alert, "high weight");

        let decisions = loop_ctrl.process_evaluations(&[eval], &LogicalOrg::new());

        // 至少有一个增强决策
        assert!(decisions.iter().any(|d| matches!(d,
            FeedbackDecision::EnhanceChannel(_, _)
        )));
    }

    #[test]
    fn feedback_loop_suppress_noisy_channel() {
        let mut loop_ctrl = FeedbackLoop::new();
        loop_ctrl.register_channel("noisy_source_cause", 1.0);

        // 连续多次噪声评价
        let mut evals = Vec::new();
        for _ in 0..20 {
            let pair: CausalPair<&str> = CausalPair::new(
                RippleDiff {
                    from_ts: 0, to_ts: 1,
                    source_id: "noisy_source".into(),
                    amp_delta: 0.01, phase_delta: 0.0,
                    kind: DiffKind::Amplitude,
                },
                "noisy_source", "effect"
            );
            evals.push(Evaluation::new(pair, EvaluationLevel::Noise, "noise"));
        }

        loop_ctrl.process_evaluations(&evals, &LogicalOrg::new());

        // 噪声源的准确率应该下降
        let ch = loop_ctrl.channels.get("noisy_source_cause").unwrap();
        assert!(ch.accuracy < 0.5);
    }

    #[test]
    fn apply_signals_enhance() {
        let mut loop_ctrl = FeedbackLoop::new();
        loop_ctrl.register_channel("ch_a", 1.0);

        let signals = vec![
            FeedbackSignal::EnhancePerception("ch_a".into()),
        ];
        loop_ctrl.apply_signals(&signals);

        assert!(loop_ctrl.channels["ch_a"].current_sensitivity > 1.0);
    }

    #[test]
    fn apply_signals_expand() {
        let mut loop_ctrl = FeedbackLoop::new();
        assert_eq!(loop_ctrl.channels.len(), 0);

        let signals = vec![
            FeedbackSignal::ExpandObservation("new_sensor".into()),
        ];
        loop_ctrl.apply_signals(&signals);

        assert_eq!(loop_ctrl.channels.len(), 1);
        assert!(loop_ctrl.channels.contains_key("new_sensor"));
    }

    #[test]
    fn top_channels_ranking() {
        let mut loop_ctrl = FeedbackLoop::new();
        loop_ctrl.register_channel("low", 1.0);
        loop_ctrl.register_channel("high", 1.0);
        loop_ctrl.register_channel("mid", 1.0);

        loop_ctrl.channels.get_mut("high").unwrap().enhance(2.0);
        loop_ctrl.channels.get_mut("mid").unwrap().enhance(0.5);

        let top = loop_ctrl.top_channels(2);
        assert_eq!(top[0].source_id, "high");
        assert_eq!(top[1].source_id, "mid");
    }

    #[test]
    fn closed_loop_system_status() {
        let mut system = ClosedLoopSystem::<String>::new();
        system.register_perception("sensor_1", 1.0);
        system.register_perception("sensor_2", 1.0);

        system.add_rule(RuleNode::new(1, 12, 24));
        system.add_rule(RuleNode::new(2, 6, 12));
        system.solidify_rule(1);

        let status = system.status();
        assert_eq!(status.rule_count, 2);
        assert_eq!(status.frozen_count, 1);
        assert_eq!(status.channel_count, 2);
    }

    #[test]
    fn channel_sensitivity_bounds() {
        let mut ch = PerceptionChannel::new("bounded", 1.0);

        // 增强到上限
        for _ in 0..100 {
            ch.enhance(1.0);
        }
        assert!(ch.current_sensitivity <= 100.0);

        // 抑制到下限
        for _ in 0..100 {
            ch.suppress(0.9);
        }
        assert!(ch.current_sensitivity >= 0.01);
    }
}
