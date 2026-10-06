// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.80 判断评价引擎                                              ║
// ║  判断之力 · 评价之力 · 逻辑组织 · 吸引子 · 抗议向量 · 迂回反馈   ║
// ║                                                                  ║
// ║  核心: 四阶感知递进 — 迂回 → 反射 → 判断 → 评价 → 逻辑组织       ║
// ║  · 迂回反馈: 传动在时间链上的多次回响, 产生涟漪                  ║
// ║  · 反射感知: 相邻印记比较, 产生差异标记                          ║
// ║  · 判断之力: 差异标记关联为关系锚点 (因果对)                     ║
// ║  · 评价之力: 因果对排序加权, 价值倾向                           ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};

/// 迂回回响 — 一次传动在时间轴上的多次回声
#[derive(Clone, Debug)]
pub struct Echo {
    /// 原始信号强度
    pub original: f64,
    /// 回响强度 (随拍衰减)
    pub current: f64,
    /// 衰减系数
    pub decay: f64,
    /// 来源容器
    pub source: ContainerRef,
    /// 产生时间拍
    pub birth_tick: u64,
    /// 回响次数
    pub echo_count: u32,
}

impl Echo {
    pub fn new(original: f64, source: ContainerRef, birth_tick: u64) -> Self {
        Echo {
            original,
            current: original,
            decay: 0.85,
            source,
            birth_tick,
            echo_count: 0,
        }
    }

    /// 进一步响 (衰减)
    pub fn resonate(&mut self) {
        self.current *= self.decay;
        self.echo_count += 1;
    }

    /// 是否可闻 (强度足够)
    pub fn is_audible(&self) -> bool {
        self.current > 0.01
    }

    /// 当前拍
    pub fn age(&self, now: u64) -> u64 {
        now.saturating_sub(self.birth_tick)
    }
}

/// 差异标记 — 两个相邻时间戳的涟漪比较结果
#[derive(Clone, Debug)]
pub struct DiffMarker {
    /// 起点印记
    pub from: f64,
    /// 终点印记
    pub to: f64,
    /// 差异量
    pub delta: f64,
    /// 变化类型
    pub change_kind: ChangeKind,
    /// 检测时间拍
    pub detected_tick: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    /// 增幅
    Amplify,
    /// 衰减
    Attenuate,
    /// 相位翻转
    PhaseFlip,
    /// 频率漂移
    Drift,
    /// 不变
    Stable,
}

impl DiffMarker {
    pub fn new(from: f64, to: f64, tick: u64) -> Self {
        let delta = to - from;
        let change_kind = if delta.abs() < 0.001 {
            ChangeKind::Stable
        } else if from.signum() != to.signum() && from != 0.0 {
            ChangeKind::PhaseFlip
        } else if delta > 0.0 {
            ChangeKind::Amplify
        } else {
            ChangeKind::Attenuate
        };

        DiffMarker {
            from,
            to,
            delta,
            change_kind,
            detected_tick: tick,
        }
    }

    /// 差异幅度
    pub fn magnitude(&self) -> f64 {
        self.delta.abs()
    }

    /// 是否显著
    pub fn is_significant(&self) -> bool {
        self.magnitude() > 0.05 && self.change_kind != ChangeKind::Stable
    }
}

/// 关系锚点 — 两个因果对的绑定 (判断之力产物)
#[derive(Clone, Debug)]
pub struct RelationAnchor {
    /// 因 (容器A 的变化)
    pub cause: ContainerRef,
    /// 果 (容器B 的变化)
    pub effect: ContainerRef,
    /// 绑定强度 (由差异标记积累)
    pub binding: f64,
    /// 建立时间拍
    pub established_tick: u64,
    /// 见证计数
    pub witnessed: u64,
}

impl RelationAnchor {
    pub fn new(cause: ContainerRef, effect: ContainerRef, tick: u64) -> Self {
        RelationAnchor {
            cause,
            effect,
            binding: 0.1,
            established_tick: tick,
            witnessed: 1,
        }
    }

    /// 见证 (加法统计)
    pub fn witness(&mut self) {
        self.witnessed += 1;
        self.binding = (self.binding + 0.01).min(1.0);
    }

    /// 遗忘 (长期不见证)
    pub fn fade(&mut self) {
        self.binding = (self.binding * 0.99).max(0.01);
    }

    /// 是否强绑定
    pub fn is_strong(&self) -> bool {
        self.binding > 0.5 && self.witnessed > 3
    }
}

/// 评价 — 带有价值倾向的判断
#[derive(Clone, Debug)]
pub struct Evaluation {
    /// 被评价的关系锚点
    pub anchor_id: u64,
    /// 价值分数 [-1.0, 1.0]
    pub value: f64,
    /// 评价依据
    pub reason: EvalReason,
    /// 评价拍
    pub eval_tick: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvalReason {
    /// 值得强化 (多次好结果)
    PositiveReinforce,
    /// 应该抑制 (多次坏结果)
    NegativeSuppress,
    /// 新奇驱动 (未充分探索)
    NoveltyDriven,
    /// 随机试探
    Exploratory,
}

/// 因果对 — 由判断之力绑定的因果关系
#[derive(Clone, Debug)]
pub struct CausalPair {
    pub anchor: RelationAnchor,
    pub evaluations: Vec<Evaluation>,
    /// 累积评价均值
    pub cumulative_value: f64,
}

impl CausalPair {
    pub fn new(anchor: RelationAnchor) -> Self {
        CausalPair {
            anchor,
            evaluations: Vec::new(),
            cumulative_value: 0.0,
        }
    }

    /// 添加评价
    pub fn evaluate(&mut self, value: f64, reason: EvalReason, tick: u64) {
        let id = self.evaluations.len() as u64;
        self.evaluations.push(Evaluation {
            anchor_id: id,
            value: value.clamp(-1.0, 1.0),
            reason,
            eval_tick: tick,
        });

        // 累计值 = 指数移动平均
        let alpha = 0.3;
        self.cumulative_value = self.cumulative_value * (1.0 - alpha) + value * alpha;
    }

    /// 趋势 (最近评价的方向)
    pub fn trend(&self) -> f64 {
        if self.evaluations.len() < 2 {
            return 0.0;
        }
        let recent = &self.evaluations[self.evaluations.len().saturating_sub(3)..];
        let sum: f64 = recent.iter().map(|e| e.value).sum();
        sum / recent.len() as f64
    }
}

/// 吸引子 — 动力系统中的稳定趋势
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttractorKind {
    /// 固定点 — 趋势收敛到一点
    FixedPoint { value: f64 },
    /// 极限环 — 周期性震荡
    LimitCycle { amplitude: f64, period: u64 },
    /// 奇异吸引子 — 混沌但有界
    Strange { lyapunov: f64 },
}

/// 抗议向量 — 对现状的偏离冲动
#[derive(Clone, Debug)]
pub struct ProtestVector {
    /// 抗议强度
    pub intensity: f64,
    /// 抗议方向 (目标状态)
    pub direction: f64,
    /// 抗议来源 tick
    pub origin_tick: u64,
    /// 是否已表达
    pub expressed: bool,
}

impl ProtestVector {
    pub fn new(intensity: f64, direction: f64, tick: u64) -> Self {
        ProtestVector {
            intensity: intensity.clamp(0.0, 1.0),
            direction,
            origin_tick: tick,
            expressed: false,
        }
    }

    /// 表达抗议 (标记为已表达)
    pub fn express(&mut self) {
        self.expressed = true;
    }

    /// 是否有行动力
    pub fn is_actionable(&self) -> bool {
        self.intensity > 0.3 && !self.expressed
    }
}

/// 迂回反馈引擎 — 阶段一
pub struct FeedbackEngine {
    /// 主动涟漪
    pub echoes: Vec<Echo>,
    /// 衰减系数
    pub decay: f64,
}

impl FeedbackEngine {
    pub fn new() -> Self {
        FeedbackEngine {
            echoes: Vec::new(),
            decay: 0.85,
        }
    }

    /// 注入信号 → 产生回响
    pub fn inject(&mut self, signal: f64, source: ContainerRef, tick: u64) {
        self.echoes.push(Echo::new(signal, source, tick));
    }

    /// 推进一拍 (所有回响衰减)
    pub fn tick(&mut self) {
        for echo in &mut self.echoes {
            echo.resonate();
        }
        // 删除不可闻的回响
        self.echoes.retain(|e| e.is_audible());
    }

    /// 可闻回响数
    pub fn audible_count(&self) -> usize {
        self.echoes.iter().filter(|e| e.is_audible()).count()
    }

    /// 主动回响的总能量
    pub fn total_energy(&self) -> f64 {
        self.echoes.iter().map(|e| e.current).sum()
    }

    /// 当前最强回响
    pub fn dominant_echo(&self) -> Option<&Echo> {
        self.echoes.iter().max_by(|a, b| a.current.partial_cmp(&b.current).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl Default for FeedbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 反射感知引擎 — 阶段二
pub struct ReflectionEngine {
    /// 上一拍印记
    pub last_imprint: Option<f64>,
    /// 差异标记历史
    pub markers: Vec<DiffMarker>,
}

impl ReflectionEngine {
    pub fn new() -> Self {
        ReflectionEngine {
            last_imprint: None,
            markers: Vec::new(),
        }
    }

    /// 观测新印记, 比较差异
    pub fn observe(&mut self, imprint: f64, tick: u64) -> Option<DiffMarker> {
        let marker = self.last_imprint.map(|prev| DiffMarker::new(prev, imprint, tick));
        self.last_imprint = Some(imprint);
        if let Some(ref m) = marker {
            self.markers.push(m.clone());
        }
        marker
    }

    /// 显著差异数
    pub fn significant_diffs(&self) -> usize {
        self.markers.iter().filter(|m| m.is_significant()).count()
    }

    /// 最近差异
    pub fn latest_marker(&self) -> Option<&DiffMarker> {
        self.markers.last()
    }

    /// 全部标记数
    pub fn marker_count(&self) -> usize {
        self.markers.len()
    }
}

impl Default for ReflectionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 判断引擎 — 阶段三
pub struct JudgementEngine {
    /// 关系锚点表 (id → 因果对)
    pub causal_pairs: HashMap<u64, CausalPair>,
    /// 锚点 ID 计数器
    next_id: u64,
    /// 建设 tick
    pub current_tick: u64,
}

impl JudgementEngine {
    pub fn new() -> Self {
        JudgementEngine {
            causal_pairs: HashMap::new(),
            next_id: 0,
            current_tick: 0,
        }
    }

    /// 建立新关系锚点
    pub fn establish(&mut self, cause: ContainerRef, effect: ContainerRef) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let anchor = RelationAnchor::new(cause, effect, self.current_tick);
        self.causal_pairs.insert(id, CausalPair::new(anchor));
        id
    }

    /// 见证一对关系
    pub fn witness(&mut self, anchor_id: u64) {
        if let Some(pair) = self.causal_pairs.get_mut(&anchor_id) {
            pair.anchor.witness();
        }
    }

    /// 评价
    pub fn evaluate(&mut self, anchor_id: u64, value: f64, reason: EvalReason) {
        if let Some(pair) = self.causal_pairs.get_mut(&anchor_id) {
            pair.evaluate(value, reason, self.current_tick);
        }
    }

    /// 推进拍 (弱化的锚点遗忘)
    pub fn tick(&mut self) {
        self.current_tick += 1;
        for pair in self.causal_pairs.values_mut() {
            // 最近没有评价则弱化
            if let Some(last_eval) = pair.evaluations.last() {
                if self.current_tick - last_eval.eval_tick > 10 {
                    pair.anchor.fade();
                }
            }
        }
    }

    /// 最强绑定锚点
    pub fn strongest_anchor(&self) -> Option<(u64, &CausalPair)> {
        self.causal_pairs
            .iter()
            .max_by(|(_, a), (_, b)| a.anchor.binding.partial_cmp(&b.anchor.binding).unwrap())
            .map(|(id, pair)| (*id, pair))
    }

    /// 最高评价锚点
    pub fn best_evaluated(&self) -> Option<(u64, &CausalPair)> {
        self.causal_pairs
            .iter()
            .max_by(|(_, a), (_, b)| a.cumulative_value.partial_cmp(&b.cumulative_value).unwrap())
            .map(|(id, pair)| (*id, pair))
    }

    /// 因果对数量
    pub fn pair_count(&self) -> usize {
        self.causal_pairs.len()
    }

    /// 获取锚点
    pub fn get_pair(&self, id: u64) -> Option<&CausalPair> {
        self.causal_pairs.get(&id)
    }
}

impl Default for JudgementEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 评价与逻辑组织引擎 — 阶段四
pub struct LogicOrganizer {
    /// 判断引擎引用
    pub judgement: JudgementEngine,
    /// 评价历史
    pub eval_history: Vec<Evaluation>,
    /// 逻辑组织表: 值阈值 → 策略选择
    pub thresholds: Vec<(f64, String)>,
}

impl LogicOrganizer {
    pub fn new() -> Self {
        LogicOrganizer {
            judgement: JudgementEngine::new(),
            eval_history: Vec::new(),
            thresholds: Vec::new(),
        }
    }

    /// 添加组织规则 (阈值 → 策略名)
    pub fn add_rule(&mut self, threshold: f64, strategy: &str) {
        self.thresholds.push((threshold, strategy.to_string()));
        self.thresholds.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    }

    /// 根据评价结果选择策略
    pub fn select_strategy(&self, value: f64) -> Option<&str> {
        for (threshold, strategy) in &self.thresholds {
            if value >= *threshold {
                return Some(strategy.as_str());
            }
        }
        None
    }

    /// 基于评价的最大化选择
    pub fn choose_best(&self) -> Option<(u64, f64)> {
        self.judgement
            .causal_pairs
            .iter()
            .map(|(id, pair)| (*id, pair.cumulative_value))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    }

    /// 应该强化?
    pub fn should_reinforce(&self, value: f64) -> bool {
        value > 0.3
    }

    /// 应该抑制?
    pub fn should_suppress(&self, value: f64) -> bool {
        value < -0.3
    }
}

impl Default for LogicOrganizer {
    fn default() -> Self {
        Self::new()
    }
}

/// 判断评价引擎 — 四阶感知统一
pub struct JudgementEngineV2 {
    /// 阶段一: 迂回反馈
    pub feedback: FeedbackEngine,
    /// 阶段二: 反射感知
    pub reflection: ReflectionEngine,
    /// 阶段三: 判断
    pub judgements: JudgementEngine,
    /// 阶段四: 逻辑组织
    pub organizer: LogicOrganizer,
    /// 抗议向量
    pub protests: Vec<ProtestVector>,
    /// 当前拍
    pub current_tick: u64,
}

impl JudgementEngineV2 {
    pub fn new() -> Self {
        JudgementEngineV2 {
            feedback: FeedbackEngine::new(),
            reflection: ReflectionEngine::new(),
            judgements: JudgementEngine::new(),
            organizer: LogicOrganizer::new(),
            protests: Vec::new(),
            current_tick: 0,
        }
    }

    /// 推进一拍 (全部阶段同步)
    pub fn step(&mut self) {
        self.current_tick += 1;
        self.feedback.tick();
        self.judgements.tick();
    }

    /// 注入感知信号
    pub fn perceive(&mut self, signal: f64, source: ContainerRef) {
        self.feedback.inject(signal, source.clone(), self.current_tick);
        self.reflection.observe(signal, self.current_tick);
    }

    /// 建立因果判断
    pub fn judge(&mut self, cause: ContainerRef, effect: ContainerRef) -> u64 {
        self.judgements.establish(cause, effect)
    }

    /// 评价
    pub fn evaluate(&mut self, anchor_id: u64, value: f64, reason: EvalReason) {
        self.judgements.evaluate(anchor_id, value, reason);
    }

    /// 发出抗议
    pub fn protest(&mut self, intensity: f64, direction: f64) {
        self.protests.push(ProtestVector::new(intensity, direction, self.current_tick));
    }

    /// 是否有可行动的抗议
    pub fn has_actionable_protest(&self) -> bool {
        self.protests.iter().any(|p| p.is_actionable())
    }

    /// 读取当前系统状态
    pub fn status(&self) -> JudgementStatus {
        JudgementStatus {
            echoes: self.feedback.audible_count(),
            markers: self.reflection.marker_count(),
            pairs: self.judgements.pair_count(),
            protests: self.protests.len(),
            tick: self.current_tick,
        }
    }
}

impl Default for JudgementEngineV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JudgementStatus {
    pub echoes: usize,
    pub markers: usize,
    pub pairs: usize,
    pub protests: usize,
    pub tick: u64,
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

    // ── 迂回反馈 ──

    #[test]
    fn echo_resonates_and_fades() {
        let mut echo = Echo::new(1.0, mk_ref(1), 0);
        echo.resonate();
        assert!(echo.current < 1.0);
        assert_eq!(echo.echo_count, 1);
        assert!(echo.is_audible());
    }

    #[test]
    fn echo_fades_to_silence() {
        let mut echo = Echo::new(1.0, mk_ref(1), 0);
        // 多次衰减后不可闻
        for _ in 0..50 {
            echo.resonate();
        }
        assert!(!echo.is_audible());
    }

    #[test]
    fn feedback_engine_tick_cleans() {
        let mut engine = FeedbackEngine::new();
        engine.inject(1.0, mk_ref(1), 0);
        engine.inject(0.5, mk_ref(2), 1);

        assert_eq!(engine.audible_count(), 2);

        // 拍进到足够多次
        for _ in 0..100 {
            engine.tick();
        }

        assert_eq!(engine.audible_count(), 0);
    }

    #[test]
    fn feedback_total_energy() {
        let mut engine = FeedbackEngine::new();
        engine.inject(1.0, mk_ref(1), 0);
        engine.inject(0.5, mk_ref(2), 1);

        assert!((engine.total_energy() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn feedback_dominant_echo() {
        let mut engine = FeedbackEngine::new();
        engine.inject(0.3, mk_ref(1), 0);
        engine.inject(0.9, mk_ref(2), 1);

        let dom = engine.dominant_echo().unwrap();
        assert!((dom.original - 0.9).abs() < 1e-9);
    }

    // ── 反射感知 ──

    #[test]
    fn diff_marker_stable() {
        let m = DiffMarker::new(1.0, 1.0001, 0);
        assert_eq!(m.change_kind, ChangeKind::Stable);
    }

    #[test]
    fn diff_marker_amplify() {
        let m = DiffMarker::new(1.0, 2.0, 0);
        assert_eq!(m.change_kind, ChangeKind::Amplify);
        assert!(m.magnitude() > 0.0);
    }

    #[test]
    fn diff_marker_attenuate() {
        let m = DiffMarker::new(2.0, 1.0, 0);
        assert_eq!(m.change_kind, ChangeKind::Attenuate);
    }

    #[test]
    fn diff_marker_phase_flip() {
        let m = DiffMarker::new(1.0, -1.0, 0);
        assert_eq!(m.change_kind, ChangeKind::PhaseFlip);
    }

    #[test]
    fn reflection_observe_creates_markers() {
        let mut re = ReflectionEngine::new();
        let m1 = re.observe(1.0, 0);
        assert!(m1.is_none()); // 无前值

        let m2 = re.observe(2.0, 1);
        assert!(m2.is_some());
        let marker = m2.unwrap();
        assert_eq!(marker.change_kind, ChangeKind::Amplify);
    }

    #[test]
    fn reflection_significant_diffs() {
        let mut re = ReflectionEngine::new();
        re.observe(1.0, 0);
        re.observe(1.0001, 1); // stable
        re.observe(2.0, 2);    // significant

        assert_eq!(re.significant_diffs(), 1);
    }

    // ── 判断之力 ──

    #[test]
    fn judgement_establish_pair() {
        let mut engine = JudgementEngine::new();
        let id = engine.establish(mk_ref(1), mk_ref(2));

        assert!(engine.get_pair(id).is_some());
        assert_eq!(engine.pair_count(), 1);
    }

    #[test]
    fn judgement_witness_increases_binding() {
        let mut engine = JudgementEngine::new();
        let id = engine.establish(mk_ref(1), mk_ref(2));

        let initial = engine.get_pair(id).unwrap().anchor.binding;
        engine.witness(id);
        engine.witness(id);
        let after = engine.get_pair(id).unwrap().anchor.binding;

        assert!(after > initial);
    }

    #[test]
    fn judgement_strong_anchor() {
        let mut engine = JudgementEngine::new();
        let id = engine.establish(mk_ref(1), mk_ref(2));

        for _ in 0..100 {
            engine.witness(id);
        }

        assert!(engine.get_pair(id).unwrap().anchor.is_strong());
    }

    #[test]
    fn judgement_tick_fades_old() {
        let mut engine = JudgementEngine::new();
        let id = engine.establish(mk_ref(1), mk_ref(2));

        let initial = engine.get_pair(id).unwrap().anchor.binding;
        // 推进很多拍 (无评价)
        for _ in 0..100 {
            engine.tick();
        }

        let after = engine.get_pair(id).unwrap().anchor.binding;
        assert!(after <= initial);
    }

    // ── 评价 ──

    #[test]
    fn causal_pair_trend() {
        let anchor = RelationAnchor::new(mk_ref(1), mk_ref(2), 0);
        let mut pair = CausalPair::new(anchor);

        // 不足 2 个评价时 trend = 0
        assert_eq!(pair.trend(), 0.0);

        pair.evaluate(0.5, EvalReason::PositiveReinforce, 1);
        pair.evaluate(0.8, EvalReason::PositiveReinforce, 2);
        pair.evaluate(0.3, EvalReason::NoveltyDriven, 3);

        assert!(pair.trend() > 0.0);
    }

    #[test]
    fn logic_organizer_strategy_selection() {
        let mut org = LogicOrganizer::new();
        org.add_rule(0.8, "强强化");
        org.add_rule(0.3, "弱强化");
        org.add_rule(-0.5, "观察");

        assert_eq!(org.select_strategy(0.9), Some("强强化"));
        assert_eq!(org.select_strategy(0.5), Some("弱强化"));
        assert_eq!(org.select_strategy(-0.3), Some("观察"));
        assert_eq!(org.select_strategy(-0.9), None);
    }

    #[test]
    fn logic_reinforce_suppress() {
        let org = LogicOrganizer::new();
        assert!(org.should_reinforce(0.5));
        assert!(!org.should_reinforce(0.1));
        assert!(org.should_suppress(-0.5));
        assert!(!org.should_suppress(-0.1));
    }

    // ── 抗议 ──

    #[test]
    fn protest_actionable() {
        let mut p = ProtestVector::new(0.5, 1.0, 0);
        assert!(p.is_actionable());

        p.express();
        assert!(!p.is_actionable());
    }

    #[test]
    fn protest_too_weak() {
        let p = ProtestVector::new(0.1, 1.0, 0);
        assert!(!p.is_actionable());
    }

    // ── 整体引擎 ──

    #[test]
    fn judgement_v2_new() {
        let engine = JudgementEngineV2::new();
        assert_eq!(engine.current_tick, 0);
    }

    #[test]
    fn judgement_v2_perceive_feeds_pipeline() {
        let mut engine = JudgementEngineV2::new();
        engine.perceive(1.0, mk_ref(1));
        engine.perceive(0.5, mk_ref(2));

        assert!(engine.feedback.audible_count() > 0);
        assert!(engine.reflection.last_imprint.is_some());
    }

    #[test]
    fn judgement_v2_judge_and_evaluate() {
        let mut engine = JudgementEngineV2::new();
        let id = engine.judge(mk_ref(1), mk_ref(2));
        engine.evaluate(id, 0.8, EvalReason::PositiveReinforce);

        let pair = engine.judgements.get_pair(id).unwrap();
        assert!((pair.cumulative_value - 0.8 * 0.3).abs() < 1e-9);
    }

    #[test]
    fn judgement_v2_protest() {
        let mut engine = JudgementEngineV2::new();
        engine.protest(0.7, 1.0);

        assert!(engine.has_actionable_protest());
        assert_eq!(engine.protests.len(), 1);
    }

    #[test]
    fn judgement_v2_step() {
        let mut engine = JudgementEngineV2::new();
        engine.step();
        engine.step();
        assert_eq!(engine.current_tick, 2);
    }

    #[test]
    fn judgement_status() {
        let mut engine = JudgementEngineV2::new();
        engine.perceive(1.0, mk_ref(1));

        let status = engine.status();
        assert_eq!(status.echoes, 1);
        assert_eq!(status.tick, 0);
    }

    #[test]
    fn attach_attractor() {
        let _fixed = AttractorKind::FixedPoint { value: 0.5 };
        let _cycle = AttractorKind::LimitCycle { amplitude: 1.0, period: 4 };
        let _strange = AttractorKind::Strange { lyapunov: 0.5 };
        // 仅编译检查
    }
}
