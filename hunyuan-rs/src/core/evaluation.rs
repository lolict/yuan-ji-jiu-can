// ═══════════════════════════════════════════════════════════════════════════
// evaluation.rs — 评价之力 · 容器反馈引擎 (v0.1)
// 三元: 反馈(Feedback) → 评分(Score) → 进化(Evolution)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 容器关系动力学 阶段四）：
//
//   评价是"带有价值倾向的判断"。
//   判断回答"这种变化与谁有关"（关系锚点），
//   评价回答"哪种关系值得强化"（因果对排序）。
//
//   评价结果积累到临界点，自组织形成逻辑路径：
//   哪种因果模式值得强化、哪种应该抑制。
//
//   系统根据评价结果调整自身呼唤策略 → 这就是进化。
//
// 四阶感知链：
//   迂回反馈 → 反射感知 → 判断之力 → 评价之力
//   (涟漪)     (差异标记)   (关系锚点)  (因果对排序)
//                                     ↑ 我们在这里
//
// ═══════════════════════════════════════════════════════════════════════════

use crate::container_judgment::JudgmentResult;
use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

// ───────────────────────────────────────────────────────────────────────────
// 反馈信号 — 容器交互中的涟漪记录
// ───────────────────────────────────────────────────────────────────────────

/// 一次容器交互的反馈记录
#[derive(Debug, Clone)]
pub struct FeedbackSignal {
    pub from_id: u64,
    pub to_id: u64,
    pub tick: u32,
    pub amplitude: f64,
    pub phase_diff: f64,
    pub latency: u32,
    pub acknowledged: bool,
}

impl FeedbackSignal {
    pub fn new(from_id: u64, to_id: u64, tick: u32, amplitude: f64, phase_diff: f64, latency: u32, acknowledged: bool) -> Self {
        Self { from_id, to_id, tick, amplitude, phase_diff, latency, acknowledged }
    }

    /// 信号质量: 幅度 × 相位吻合度 × 承认因子 × 延迟衰减
    fn quality(&self) -> f64 {
        let phase_match = (self.phase_diff.cos() + 1.0) / 2.0;
        let ack_factor = if self.acknowledged { 1.0 } else { 0.5 };
        let latency_decay = 1.0 / (1.0 + self.latency as f64 * 0.1);
        self.amplitude * phase_match * ack_factor * latency_decay
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 因果对 — 两个容器之间的关系锚点
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CausalPair {
    pub source_id: u64,
    pub target_id: u64,
    pub feedback_count: usize,
    pub total_quality: f64,
    pub avg_quality: f64,
    pub trend: f64,
}

impl CausalPair {
    pub fn new(source_id: u64, target_id: u64) -> Self {
        Self { source_id, target_id, feedback_count: 0, total_quality: 0.0, avg_quality: 0.0, trend: 0.0 }
    }

    fn add_feedback(&mut self, quality: f64) {
        self.feedback_count += 1;
        let old_avg = self.avg_quality;
        self.total_quality += quality;
        self.avg_quality = self.total_quality / self.feedback_count as f64;
        self.trend = self.avg_quality - old_avg;
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 进化策略 — 根据评分决定强化/抑制/观望
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolveStrategy {
    Reinforce,
    Suppress,
    Observe,
    Restructure,
}

impl EvolveStrategy {
    pub fn to_glyph(&self) -> char {
        match self {
            EvolveStrategy::Reinforce => '↗',
            EvolveStrategy::Suppress => '↘',
            EvolveStrategy::Observe => '○',
            EvolveStrategy::Restructure => '⟲',
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 评价结果 — 一个容器的进化决策
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub pack_id: u64,
    pub relationship_count: usize,
    pub total_score: f64,
    pub strategy: EvolveStrategy,
    pub top_pairs: Vec<(u64, f64)>,
    pub reasons: Vec<String>,
}

// ───────────────────────────────────────────────────────────────────────────
// 评价引擎 — 反馈 → 评分 → 进化
// ───────────────────────────────────────────────────────────────────────────

pub struct EvaluationEngine {
    pairs: Vec<CausalPair>,
    feedbacks: Vec<FeedbackSignal>,
    reinforce_threshold: f64,
    suppress_threshold: f64,
    min_samples: usize,
}

impl EvaluationEngine {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
            feedbacks: Vec::new(),
            reinforce_threshold: 0.6,
            suppress_threshold: 0.2,
            min_samples: 3,
        }
    }

    pub fn with_thresholds(mut self, reinforce: f64, suppress: f64, min_samples: usize) -> Self {
        self.reinforce_threshold = reinforce;
        self.suppress_threshold = suppress;
        self.min_samples = min_samples;
        self
    }

    pub fn record_feedback(&mut self, signal: FeedbackSignal) {
        self.feedbacks.push(signal.clone());
        self.update_pair(signal.from_id, signal.to_id, signal.quality());
    }

    pub fn record_batch(&mut self, signals: &[FeedbackSignal]) {
        for s in signals { self.record_feedback(s.clone()); }
    }

    fn update_pair(&mut self, source: u64, target: u64, quality: f64) {
        if let Some(pair) = self.pairs.iter_mut().find(|p| p.source_id == source && p.target_id == target) {
            pair.add_feedback(quality);
        } else {
            let mut new_pair = CausalPair::new(source, target);
            new_pair.add_feedback(quality);
            self.pairs.push(new_pair);
        }
    }

    /// 从容器包的坐标和审判结果推断反馈信号
    pub fn infer_from_packs_and_judgments(&mut self, packs: &[ContainerPack], judgments: &[JudgmentResult]) {
        let judgment_map: std::collections::HashMap<u64, &JudgmentResult> =
            judgments.iter().map(|j| (j.pack_id, j)).collect();

        for i in 0..packs.len() {
            for j in (i + 1)..packs.len() {
                let a = &packs[i];
                let b = &packs[j];
                let ca = a.coord();
                let cb = b.coord();
                if ca.z != cb.z { continue; }

                let dx = (ca.x as f64) - (cb.x as f64);
                let dy = (ca.y as f64) - (cb.y as f64);
                let dz = (ca.z as f64) - (cb.z as f64);
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                let phase_diff = (dist * 0.01) % (2.0 * std::f64::consts::PI);

                let score_a = judgment_map.get(&a.pack_id).map(|r| r.score).unwrap_or(0.5);
                let score_b = judgment_map.get(&b.pack_id).map(|r| r.score).unwrap_or(0.5);
                let amplitude = score_a.min(score_b);
                let latency = if a.func_code == b.func_code { 1 } else { 2 };
                let ack = judgment_map.get(&a.pack_id).map(|r| r.score > 0.5).unwrap_or(false)
                    && judgment_map.get(&b.pack_id).map(|r| r.score > 0.5).unwrap_or(false);
                let tick = ca.z as u32;

                self.record_feedback(FeedbackSignal::new(a.pack_id, b.pack_id, tick, amplitude, phase_diff, latency, ack));
                self.record_feedback(FeedbackSignal::new(b.pack_id, a.pack_id, tick, amplitude, phase_diff, latency, ack));
            }
        }
    }

    /// 计算因果对的评分 (加权排序)
    pub fn score_pairs(&self) -> Vec<(u64, u64, f64, EvolveStrategy)> {
        self.pairs.iter().map(|pair| {
            let weighted_score = pair.avg_quality * (1.0 + pair.feedback_count as f64 * 0.05);
            let strategy = if pair.feedback_count < self.min_samples {
                EvolveStrategy::Observe
            } else if weighted_score >= self.reinforce_threshold && pair.trend >= 0.0 {
                EvolveStrategy::Reinforce
            } else if weighted_score < self.suppress_threshold {
                EvolveStrategy::Suppress
            } else if pair.trend < -0.1 {
                EvolveStrategy::Restructure
            } else {
                EvolveStrategy::Observe
            };
            (pair.source_id, pair.target_id, weighted_score, strategy)
        }).collect()
    }

    /// 评价单个容器的进化方向
    pub fn evaluate(&self, pack_id: u64) -> Evaluation {
        let related: Vec<&CausalPair> = self.pairs.iter()
            .filter(|p| p.source_id == pack_id || p.target_id == pack_id)
            .collect();

        let total_score: f64 = related.iter().map(|p| p.avg_quality).sum();
        let rel_count = related.len();
        let total_feedbacks: usize = related.iter().map(|p| p.feedback_count).sum();

        let mut top: Vec<(u64, f64)> = related.iter().map(|p| {
            let partner = if p.source_id == pack_id { p.target_id } else { p.source_id };
            (partner, p.avg_quality)
        }).collect();
        top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        top.truncate(3);

        let avg = if rel_count == 0 { 0.0 } else { total_score / rel_count as f64 };
        let strategy = if total_feedbacks < self.min_samples {
            EvolveStrategy::Observe
        } else if avg >= self.reinforce_threshold {
            EvolveStrategy::Reinforce
        } else if avg < self.suppress_threshold {
            EvolveStrategy::Suppress
        } else {
            EvolveStrategy::Observe
        };

        let mut reasons = Vec::new();
        if rel_count == 0 { reasons.push("isolated_container".into()); }
        if avg >= self.reinforce_threshold { reasons.push("strong_positive_causal_pattern".into()); }
        if avg < self.suppress_threshold { reasons.push("weak_or_negative_feedback".into()); }

        Evaluation { pack_id, relationship_count: rel_count, total_score, strategy, top_pairs: top, reasons }
    }

    pub fn evaluate_all(&self, packs: &[ContainerPack]) -> Vec<Evaluation> {
        packs.iter().map(|p| self.evaluate(p.pack_id)).collect()
    }

    pub fn pair_count(&self) -> usize { self.pairs.len() }
    pub fn feedback_count(&self) -> usize { self.feedbacks.len() }

    pub fn strategy_distribution(&self) -> (usize, usize, usize, usize) {
        let scores = self.score_pairs();
        let mut r = 0; let mut s = 0; let mut o = 0; let mut re = 0;
        for (_, _, _, strat) in scores {
            match strat {
                EvolveStrategy::Reinforce => r += 1,
                EvolveStrategy::Suppress => s += 1,
                EvolveStrategy::Observe => o += 1,
                EvolveStrategy::Restructure => re += 1,
            }
        }
        (r, s, o, re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_quality_computes() {
        let s = FeedbackSignal::new(1, 2, 0, 0.8, 0.0, 1, true);
        let q = s.quality();
        assert!(q > 0.7 && q < 0.75, "q={}", q);
    }

    #[test]
    fn feedback_phase_inverts_quality() {
        let s_in = FeedbackSignal::new(1, 2, 0, 1.0, 0.0, 1, true);
        let s_out = FeedbackSignal::new(1, 2, 0, 1.0, std::f64::consts::PI, 1, true);
        assert!(s_in.quality() > s_out.quality());
    }

    #[test]
    fn evaluation_classifies_strategy() {
        let mut engine = EvaluationEngine::new().with_thresholds(0.6, 0.2, 2);
        for _ in 0..5 { engine.record_feedback(FeedbackSignal::new(1, 10, 0, 0.9, 0.0, 1, true)); }
        for _ in 0..5 { engine.record_feedback(FeedbackSignal::new(2, 20, 0, 0.1, std::f64::consts::PI, 5, false)); }

        assert_eq!(engine.evaluate(1).strategy, EvolveStrategy::Reinforce);
        assert_eq!(engine.evaluate(2).strategy, EvolveStrategy::Suppress);
    }

    #[test]
    fn evaluation_counts_relationships() {
        let mut engine = EvaluationEngine::new();
        engine.record_feedback(FeedbackSignal::new(1, 2, 0, 0.5, 0.0, 1, true));
        engine.record_feedback(FeedbackSignal::new(1, 3, 0, 0.6, 0.0, 1, true));
        assert_eq!(engine.pair_count(), 2);
        assert_eq!(engine.evaluate(1).relationship_count, 2);
    }

    #[test]
    fn strategy_distribution_works() {
        let mut engine = EvaluationEngine::new().with_thresholds(0.6, 0.2, 2);
        for _ in 0..5 { engine.record_feedback(FeedbackSignal::new(1, 10, 0, 0.9, 0.0, 1, true)); }
        for _ in 0..5 { engine.record_feedback(FeedbackSignal::new(2, 20, 0, 0.1, std::f64::consts::PI, 5, false)); }
        let (r, s, o, _) = engine.strategy_distribution();
        assert!(r >= 1);
        assert!(s >= 1);
    }

    #[test]
    fn infer_from_packs_generates_feedback() {
        use crate::cross_isa::coordinate::Coord3D;
        use crate::container_judgment::JudgmentEngine;

        let packs = vec![
            ContainerPack::new(1, Coord3D::new(1, 0x0050, 1), 1, vec![0x01, 0x02]),
            ContainerPack::new(2, Coord3D::new(2, 0x0060, 1), 1, vec![0x03, 0x04]),
        ];
        let batch = JudgmentEngine::default().judge_batch(&packs);
        let mut engine = EvaluationEngine::new();
        engine.infer_from_packs_and_judgments(&packs, &batch.results);
        assert!(engine.pair_count() >= 2, "pair_count={}", engine.pair_count());
    }
}
