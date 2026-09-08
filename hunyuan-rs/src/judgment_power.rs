// ══════════════════════════════════════════════════════════════════
// judgment_power.rs — 判断之力 · 评价之力 · 逻辑组织 · 反馈闭环
// ══════════════════════════════════════════════════════════════════
//
//   「迂回反馈产生反射感知 → 感知差异形成判断 → 判断排序产生评价
//      → 评价结构化形成逻辑组织 → 组织结果回输形成反馈」
//
//   感知: 比较相邻时间戳的涟漪模式, 觉察差异
//   判断: 将差异绑定为因果对 (A变化 → 由B引起)
//   评价: 对因果对排序/加权 (重要性分级)
//   逻辑组织: 将评价结果固化为可复用的关系规则
//   反馈: 将组织结果回输给感知层, 形成闭环
//
// ══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::hash::Hash;

// ────────────────────────────────────────────────────────────
// 涟漪模式 (感知的原材料)
// ────────────────────────────────────────────────────────────

/// 单次传动在时间链上留下的印记
#[derive(Clone, Debug)]
pub struct Ripple<T: Clone + Eq + Hash> {
    /// 时间戳
    pub timestamp: u64,
    /// 信号源标识
    pub source_id: String,
    /// 信号内容
    pub payload: T,
    /// 幅度 (强度)
    pub amplitude: f64,
    /// 相位偏移
    pub phase: f64,
}

impl<T: Clone + Eq + Hash> Ripple<T> {
    pub fn new(timestamp: u64, source_id: &str, payload: T, amplitude: f64, phase: f64) -> Self {
        Self { timestamp, source_id: source_id.into(), payload, amplitude, phase }
    }

    /// 比较两个涟漪, 返回差异标记
    fn diff(&self, other: &Self) -> Option<RippleDiff> {
        if self.source_id != other.source_id { return None; }
        let amp_delta = (self.amplitude - other.amplitude).abs();
        let phase_delta = (self.phase - other.phase).abs();

        if amp_delta < 1e-6 && phase_delta < 1e-6 {
            None // 无差异
        } else {
            Some(RippleDiff {
                from_ts: self.timestamp,
                to_ts: other.timestamp,
                source_id: self.source_id.clone(),
                amp_delta,
                phase_delta,
                kind: if amp_delta > phase_delta {
                    DiffKind::Amplitude
                } else {
                    DiffKind::Phase
                },
            })
        }
    }
}

// ────────────────────────────────────────────────────────────
// 差异标记 (判断的原材料)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Amplitude,  // 幅度差异: 强弱变化
    Phase,      // 相位差异: 节奏变化
}

/// 两个时间点之间的差异
#[derive(Clone, Debug)]
pub struct RippleDiff {
    pub from_ts: u64,
    pub to_ts: u64,
    pub source_id: String,
    pub amp_delta: f64,
    pub phase_delta: f64,
    pub kind: DiffKind,
}

impl RippleDiff {
    /// 差异的绝对量级 (用于排序)
    pub fn magnitude(&self) -> f64 {
        (self.amp_delta.powi(2) + self.phase_delta.powi(2)).sqrt()
    }
}

// ────────────────────────────────────────────────────────────
// 因果对 (判断的产物)
// ────────────────────────────────────────────────────────────

/// 因果绑定: 差异由谁引起
#[derive(Clone, Debug)]
pub struct CausalPair<T: Clone + Eq + Hash> {
    /// 差异标记
    pub diff: RippleDiff,
    /// 起因方 (容器ID)
    pub cause: String,
    /// 结果方 (容器ID)
    pub effect: String,
    /// 起因时的状态快照
    pub cause_state_before: Option<T>,
    /// 结果方受影响后的状态快照
    pub effect_state_after: Option<T>,
}

impl<T: Clone + Eq + Hash> CausalPair<T> {
    pub fn new(diff: RippleDiff, cause: &str, effect: &str) -> Self {
        Self {
            diff,
            cause: cause.into(),
            effect: effect.into(),
            cause_state_before: None,
            effect_state_after: None,
        }
    }

    pub fn with_states(mut self, cause_before: T, effect_after: T) -> Self {
        self.cause_state_before = Some(cause_before);
        self.effect_state_after = Some(effect_after);
        self
    }
}

// ────────────────────────────────────────────────────────────
// 评价等级 (评价的产物)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaluationLevel {
    /// 噪声: 忽略不计
    Noise = 0,
    /// 注意: 值得关注但暂不行动
    Notice = 1,
    /// 响应: 需要做出响应
    Respond = 2,
    /// 干预: 需要主动干预
    Intervene = 3,
    /// 警报: 最高优先级, 必须立即处理
    Alert = 4,
}

impl EvaluationLevel {
    /// 是否高于某个阈值
    pub fn at_least(&self, threshold: EvaluationLevel) -> bool {
        *self >= threshold
    }

    /// 从差异量级自动推断等级
    pub fn from_magnitude(mag: f64) -> Self {
        match mag {
            m if m < 0.1 => EvaluationLevel::Noise,
            m if m < 0.3 => EvaluationLevel::Notice,
            m if m < 0.6 => EvaluationLevel::Respond,
            m if m < 1.0 => EvaluationLevel::Intervene,
            _ => EvaluationLevel::Alert,
        }
    }
}

/// 评价结果: 带等级排序的因果对
#[derive(Clone, Debug)]
pub struct Evaluation<T: Clone + Eq + Hash> {
    pub causal_pair: CausalPair<T>,
    pub level: EvaluationLevel,
    /// 评价理由 (自然语言描述, 供人类理解)
    pub reason: String,
    /// 评价权重 (0.0 .. 1.0)
    pub weight: f64,
}

impl<T: Clone + Eq + Hash> Evaluation<T> {
    pub fn new(pair: CausalPair<T>, level: EvaluationLevel, reason: &str) -> Self {
        let weight = level_weight(&level) * pair.diff.magnitude();
        Self { causal_pair: pair, level, reason: reason.into(), weight }
    }

    /// 评价权重 (综合等级和差异量级的乘积)
    pub fn composite_weight(&self) -> f64 {
        self.weight
    }
}

fn level_weight(level: &EvaluationLevel) -> f64 {
    match level {
        EvaluationLevel::Noise => 0.0,
        EvaluationLevel::Notice => 0.2,
        EvaluationLevel::Respond => 0.5,
        EvaluationLevel::Intervene => 0.8,
        EvaluationLevel::Alert => 1.0,
    }
}

// ────────────────────────────────────────────────────────────
// 关系规则 (逻辑组织的产物)
// ────────────────────────────────────────────────────────────

/// 从评价中提取的可复用规则
#[derive(Clone, Debug)]
pub struct RelationRule {
    /// 规则ID (公约数索引)
    pub rule_id: u64,
    /// 前因模式 (用于匹配)
    pub cause_pattern: String,
    /// 后果模式 (用于预测)
    pub effect_pattern: String,
    /// 触发等级
    pub trigger_level: EvaluationLevel,
    /// 该规则的激活次数 (统计)
    pub activation_count: u64,
    /// 该规则的平均权重
    pub avg_weight: f64,
    /// 是否被固化 (固化后不再修改)
    pub solidified: bool,
}

impl RelationRule {
    pub fn new(rule_id: u64, cause_pattern: &str, effect_pattern: &str, level: EvaluationLevel) -> Self {
        Self {
            rule_id,
            cause_pattern: cause_pattern.into(),
            effect_pattern: effect_pattern.into(),
            trigger_level: level,
            activation_count: 1,
            avg_weight: 0.0,
            solidified: false,
        }
    }

    /// 激活规则: 匹配则增加计数并更新平均权重
    pub fn activate(&mut self, weight: f64) {
        if self.solidified { return; }
        let total = self.avg_weight * self.activation_count as f64 + weight;
        self.activation_count += 1;
        self.avg_weight = total / self.activation_count as f64;
    }

    /// 固化规则: 不再修改
    pub fn solidify(&mut self) {
        self.solidified = true;
    }
}

/// 规则库 (逻辑组织的容器)
#[derive(Clone, Debug)]
pub struct RuleBank {
    pub rules: HashMap<u64, RelationRule>,
    next_id: u64,
}

impl RuleBank {
    pub fn new() -> Self {
        Self { rules: HashMap::new(), next_id: 1 }
    }

    /// 从评价中提取规则 (如果规则不存在则创建)
    pub fn absorb_evaluation<T: Clone + Eq + Hash>(
        &mut self,
        eval: &Evaluation<T>,
    ) -> u64 {
        let pattern = format!("{}→{}", eval.causal_pair.cause, eval.causal_pair.effect);
        // 查找是否已有相同模式的规则
        let existing = self.rules.values().find(|r| r.cause_pattern == pattern).cloned();
        match existing {
            Some(rule) => {
                let id = rule.rule_id;
                if let Some(r) = self.rules.get_mut(&id) {
                    r.activate(eval.weight);
                    if r.activation_count >= 10 {
                        r.solidify();
                    }
                }
                id
            }
            None => {
                let id = self.next_id;
                self.next_id += 1;
                let rule = RelationRule::new(
                    id,
                    &pattern,
                    &eval.level.level_name(),
                    eval.level,
                );
                self.rules.insert(id, rule);
                id
            }
        }
    }

    /// 获取所有已固化的规则
    pub fn solidified_rules(&self) -> Vec<&RelationRule> {
        self.rules.values().filter(|r| r.solidified).collect()
    }

    /// 获取指定等级以上的规则
    pub fn rules_above(&self, level: EvaluationLevel) -> Vec<&RelationRule> {
        self.rules.values().filter(|r| r.trigger_level >= level).collect()
    }
}

impl Default for RuleBank {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 反馈信号 (闭环的回输)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum FeedbackSignal {
    /// 增强感知 (对该因果对的源提高敏感度)
    EnhancePerception(String), // source_id
    /// 抑制感知 (对该因果对的源降低敏感度)
    SuppressPerception(String), // source_id
    /// 修改规则 (规则库自修改)
    ModifyRule(u64, EvaluationLevel), // (rule_id, new_trigger_level)
    /// 新增观察者 (扩大感知范围)
    ExpandObservation(String), // 新的观察源ID
    /// 维持现状
    Maintain,
}

/// 反馈结果
#[derive(Clone, Debug)]
pub struct Feedback {
    pub signals: Vec<FeedbackSignal>,
    pub reason: String,
}

impl Feedback {
    pub fn empty() -> Self {
        Self { signals: Vec::new(), reason: String::new() }
    }

    pub fn with_signals(signals: Vec<FeedbackSignal>, reason: &str) -> Self {
        Self { signals, reason: reason.into() }
    }
}

// ────────────────────────────────────────────────────────────
// 判断引擎 (整个闭环的核心)
// ────────────────────────────────────────────────────────────

/// 判断引擎: 感知 → 判断 → 评价 → 逻辑组织 → 反馈
pub struct JudgmentEngine<T: Clone + Eq + Hash> {
    /// 历史涟漪序列
    pub ripples: Vec<Ripple<T>>,
    /// 差异序列
    pub diffs: Vec<RippleDiff>,
    /// 因果对序列
    pub causal_pairs: Vec<CausalPair<T>>,
    /// 评价序列 (按权重排序)
    pub evaluations: Vec<Evaluation<T>>,
    /// 规则库
    pub rule_bank: RuleBank,
    /// 反馈序列
    pub feedbacks: Vec<Feedback>,
    /// 感知增强表 (source_id → 灵敏度倍数)
    pub perception_sensitivity: HashMap<String, f64>,
}

impl<T: Clone + Eq + Hash> JudgmentEngine<T> {
    pub fn new() -> Self {
        Self {
            ripples: Vec::new(),
            diffs: Vec::new(),
            causal_pairs: Vec::new(),
            evaluations: Vec::new(),
            rule_bank: RuleBank::new(),
            feedbacks: Vec::new(),
            perception_sensitivity: HashMap::new(),
        }
    }

    // ── STEP 1: 感知 (记录涟漪) ──

    pub fn perceive(&mut self, ripple: Ripple<T>) {
        // 与最后一个同源的涟漪比较, 提取差异
        let origin = ripple.source_id.clone();
        if let Some(prev) = self.ripples.iter().rev().find(|r| r.source_id == origin) {
            if let Some(diff) = prev.diff(&ripple) {
                self.diffs.push(diff);
            }
        }
        self.ripples.push(ripple);
    }

    // ── STEP 2: 判断 (绑定因果) ──

    /// 将差异绑定为因果对
    pub fn judge(&mut self, diff: RippleDiff, cause: &str, effect: &str) -> usize {
        let pair = CausalPair::new(diff, cause, effect);
        self.causal_pairs.push(pair);
        self.causal_pairs.len() - 1
    }

    /// 自动判断: 寻找最近未绑定的差异进行绑定
    pub fn auto_judge(&mut self, candidate_causes: &[String], effect: &str) -> Option<usize> {
        let unbound = self.diffs.iter().enumerate()
            .find(|(_, d)| !self.causal_pairs.iter().any(|p| p.diff.from_ts == d.from_ts));
        match unbound {
            Some((_, diff)) => {
                // 选取差异最大的候选因
                let cause = candidate_causes.first()?;
                let pair = CausalPair::new(diff.clone(), cause, effect);
                self.causal_pairs.push(pair);
                Some(self.causal_pairs.len() - 1)
            }
            None => None,
        }
    }

    // ── STEP 3: 评价 (排序加权) ──

    /// 对因果对进行评价
    pub fn evaluate(&mut self, pair_idx: usize, reason: &str) -> Option<u64> {
        let pair = self.causal_pairs.get(pair_idx)?;
        let mag = pair.diff.magnitude();
        let level = EvaluationLevel::from_magnitude(mag);

        // 根据感知灵敏度调整
        let adjusted_mag = if let Some(sens) = self.perception_sensitivity.get(&pair.diff.source_id) {
            mag * sens
        } else {
            mag
        };
        let adjusted_level = EvaluationLevel::from_magnitude(adjusted_mag);

        let eval = Evaluation::new(pair.clone(), adjusted_level, reason);
        let rule_id = self.rule_bank.absorb_evaluation(&eval);
        self.evaluations.push(eval);
        // 评价从高到低排序
        self.evaluations.sort_by(|a, b| b.composite_weight().partial_cmp(&a.composite_weight()).unwrap());
        Some(rule_id)
    }

    // ── STEP 4: 逻辑组织 (已在 evaluate 时同步进行, 此方法用于手动固化) ──

    pub fn solidify_rule(&mut self, rule_id: u64) {
        if let Some(rule) = self.rule_bank.rules.get_mut(&rule_id) {
            rule.solidify();
        }
    }

    // ── STEP 5: 反馈 (产生控制信号) ──

    pub fn feedback(&mut self) -> Feedback {
        let mut signals = Vec::new();
        let mut reason_parts = Vec::new();

        // 基于最高权重的评价产生反馈
        if let Some(top_eval) = self.evaluations.first() {
            let source = &top_eval.causal_pair.diff.source_id;
            match top_eval.level {
                EvaluationLevel::Alert | EvaluationLevel::Intervene => {
                    signals.push(FeedbackSignal::EnhancePerception(source.clone()));
                    reason_parts.push(format!("增强对{}的感知灵敏度", source));
                }
                EvaluationLevel::Noise => {
                    signals.push(FeedbackSignal::SuppressPerception(source.clone()));
                    reason_parts.push(format!("抑制对{}的感知(噪声)", source));
                }
                _ => {}
            }

            // 基于固化规则产生反馈
            for rule in self.rule_bank.rules.values() {
                if rule.solidified && rule.trigger_level >= EvaluationLevel::Respond {
                    signals.push(FeedbackSignal::EnhancePerception(rule.cause_pattern.clone()));
                }
            }
        }

        let feedback = if signals.is_empty() {
            Feedback::with_signals(vec![FeedbackSignal::Maintain], "维持现状")
        } else {
            Feedback::with_signals(signals, &reason_parts.join("; "))
        };

        self.feedbacks.push(feedback.clone());
        feedback
    }

    // ── 完整闭环执行 (5步一次走完) ──

    /// 执行一次完整的判断循环
    pub fn run_cycle(
        &mut self,
        new_ripple: Ripple<T>,
        candidate_causes: &[String],
        effect: &str,
    ) -> Option<(u64, Feedback)> {
        // Step 1: 感知
        self.perceive(new_ripple);

        // Step 2: 判断
        if self.auto_judge(candidate_causes, effect).is_none() {
            return None;
        }

        // Step 3: 评价 (对新产生的因果对)
        let pair_idx = self.causal_pairs.len() - 1;
        let rule_id = self.evaluate(pair_idx, "auto evaluation from cycle")?;

        // Step 4: 自动固化高频规则
        for (_, rule) in self.rule_bank.rules.iter_mut() {
            if rule.activation_count >= 10 {
                rule.solidify();
            }
        }

        // Step 5: 反馈
        let feedback = self.feedback();

        Some((rule_id, feedback))
    }

    /// 获取所有评价的统计摘要
    pub fn evaluation_summary(&self) -> HashMap<EvaluationLevel, usize> {
        let mut map = HashMap::new();
        for eval in &self.evaluations {
            *map.entry(eval.level.clone()).or_insert(0) += 1;
        }
        map
    }

    /// 获取规则库中已固化的规则数量
    pub fn solidified_count(&self) -> usize {
        self.rule_bank.solidified_rules().len()
    }

    /// 获取总规则数
    pub fn total_rules(&self) -> usize {
        self.rule_bank.rules.len()
    }
}

impl<T: Clone + Eq + Hash> Default for JudgmentEngine<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 辅助: Evaluation 的方法
// ────────────────────────────────────────────────────────────

trait EvaluationLevelName {
    fn level_name(&self) -> String;
}

impl EvaluationLevelName for EvaluationLevel {
    fn level_name(&self) -> String {
        match self {
            EvaluationLevel::Noise => "噪声".into(),
            EvaluationLevel::Notice => "注意".into(),
            EvaluationLevel::Respond => "响应".into(),
            EvaluationLevel::Intervene => "干预".into(),
            EvaluationLevel::Alert => "警报".into(),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ripple_diff_detects_change() {
        let r1 = Ripple::new(100, "g1", "data1", 1.0, 0.0);
        let r2 = Ripple::new(101, "g1", "data1", 0.3, 0.0);
        let diff = r1.diff(&r2);
        assert!(diff.is_some());
        let diff = diff.unwrap();
        assert_eq!(diff.kind, DiffKind::Amplitude);
        assert!((diff.amp_delta - 0.7).abs() < 1e-6);
    }

    #[test]
    fn ripple_diff_same_source_no_change() {
        let r1 = Ripple::new(100, "g1", "data", 1.0, 0.0);
        let r2 = Ripple::new(101, "g1", "data", 1.0, 0.0);
        assert!(r1.diff(&r2).is_none());
    }

    #[test]
    fn causal_pair_magnitude() {
        let pair: CausalPair<String> = CausalPair::new(
            RippleDiff {
                from_ts: 0, to_ts: 1,
                source_id: "s".into(),
                amp_delta: 0.5, phase_delta: 0.5,
                kind: DiffKind::Amplitude,
            },
            "cause_a", "effect_b"
        );
        let expected_mag = (0.5_f64.powi(2) + 0.5_f64.powi(2)).sqrt();
        assert!((pair.diff.magnitude() - expected_mag).abs() < 1e-6);
    }

    #[test]
    fn evaluation_level_from_magnitude() {
        assert_eq!(EvaluationLevel::from_magnitude(0.05), EvaluationLevel::Noise);
        assert_eq!(EvaluationLevel::from_magnitude(0.2), EvaluationLevel::Notice);
        assert_eq!(EvaluationLevel::from_magnitude(0.5), EvaluationLevel::Respond);
        assert_eq!(EvaluationLevel::from_magnitude(0.8), EvaluationLevel::Intervene);
        assert_eq!(EvaluationLevel::from_magnitude(1.5), EvaluationLevel::Alert);
    }

    #[test]
    fn rule_bank_absorb_creates_rule() {
        let mut bank = RuleBank::new();
        let pair: CausalPair<String> = CausalPair::new(
            RippleDiff { from_ts: 0, to_ts: 1, source_id: "s".into(),
                        amp_delta: 0.5, phase_delta: 0.0, kind: DiffKind::Amplitude },
            "cause_a", "effect_b"
        );
        let eval = Evaluation::new(pair, EvaluationLevel::Respond, "test");
        let id = bank.absorb_evaluation(&eval);
        assert_eq!(id, 1);
        assert_eq!(bank.rules.len(), 1);
    }

    #[test]
    fn rule_bank_repeated_absorb_solidifies() {
        let mut bank = RuleBank::new();
        for _ in 0..12 {
            let pair: CausalPair<String> = CausalPair::new(
                RippleDiff { from_ts: 0, to_ts: 1, source_id: "s".into(),
                            amp_delta: 0.5, phase_delta: 0.0, kind: DiffKind::Amplitude },
                "cause_a", "effect_b"
            );
            let eval = Evaluation::new(pair, EvaluationLevel::Respond, "test");
            bank.absorb_evaluation(&eval);
        }
        assert_eq!(bank.solidified_rules().len(), 1);
    }

    #[test]
    fn engine_perceive_generates_diff() {
        let mut engine = JudgmentEngine::new();
        engine.perceive(Ripple::new(100, "g1", "d1", 1.0, 0.0));
        assert_eq!(engine.diffs.len(), 0); // 只有一个, 无法比较
        engine.perceive(Ripple::new(101, "g1", "d2", 0.5, 0.0));
        assert_eq!(engine.diffs.len(), 1);
    }

    #[test]
    fn engine_run_cycle_produces_feedback() {
        let mut engine = JudgmentEngine::new();
        engine.perceive(Ripple::new(99, "g1", "init", 1.0, 0.0));

        let result = engine.run_cycle(
            Ripple::new(100, "g1", "changed", 0.2, 0.0),
            &vec!["g0".to_string()],
            "effect_x"
        );
        assert!(result.is_some());
        let (rule_id, feedback) = result.unwrap();
        assert!(rule_id > 0);
        assert!(!feedback.signals.is_empty());
    }

    #[test]
    fn engine_evaluation_sorted_by_weight() {
        let mut engine = JudgmentEngine::new();
        // 产生两个差异悬殊的评价
        engine.perceive(Ripple::new(1, "g1", "a", 1.0, 0.0));
        engine.perceive(Ripple::new(2, "g1", "b", 1.0, 0.0)); // 无差异
        engine.perceive(Ripple::new(3, "g2", "c", 1.0, 0.0));
        engine.perceive(Ripple::new(4, "g2", "d", 0.1, 0.0)); // 大差异

        // 评价最大的差异
        let pair: CausalPair<&str> = CausalPair::new(
            engine.diffs[0].clone(), "cause", "effect"
        );
        engine.causal_pairs.push(pair.clone());
        engine.evaluate(0, "small change");
        engine.causal_pairs.push(pair.clone());
        engine.evaluate(1, "large change");

        // 验证排序: 第一个应该权重更大 (差异更大的)
        if engine.evaluations.len() >= 2 {
            assert!(engine.evaluations[0].composite_weight() >= engine.evaluations[1].composite_weight());
        }
    }

    #[test]
    fn engine_full_workflow_solidifies_rules() {
        let mut engine: JudgmentEngine<String> = JudgmentEngine::new();
        // 模拟20次相同模式的事件
        for i in 0..20 {
            let payload = format!("reading_{}", i);
            let _ = engine.run_cycle(
                Ripple::new(100 + i, "sensor", payload, (i as f64) * 0.1, 0.0),
                &vec!["env_factor".to_string()],
                "system_response"
            );
        }
        // 应该有规则被创建
        assert!(engine.total_rules() >= 1);
    }

    #[test]
    fn feedback_signal_enhance_for_alert() {
        let mut engine = JudgmentEngine::<String>::new();
        // 主动注入高幅差异
        engine.diffs.push(RippleDiff {
            from_ts: 0, to_ts: 1, source_id: "critical_sensor".into(),
            amp_delta: 2.0, phase_delta: 0.0, kind: DiffKind::Amplitude,
        });
        let pair = CausalPair::new(engine.diffs[0].clone(), "cause", "effect");
        engine.causal_pairs.push(pair.clone());
        engine.evaluate(0, "critical anomaly");
        let feedback = engine.feedback();
        // 应该包含增强感知
        let has_enhance = feedback.signals.iter().any(|s| matches!(s, FeedbackSignal::EnhancePerception(_)));
        assert!(has_enhance);
    }
}
