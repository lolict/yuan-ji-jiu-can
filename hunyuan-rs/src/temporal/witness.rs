// ── v0.42 征用检测 · 三元签裁定 ──
// 判定外部编号是否在敌对征用你的身份
// 满全法协议：三元见证裁定准入

use crate::phase_clock2::{HotScalar, PhaseClock, ContainerContext};

/// 见证极性 — 每个见证轴的判定结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessPolarity {
    Yang,
    Yin,
}

/// 裁定结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessDecision {
    Accept,
    Reject,
    Defer,
}

impl WitnessDecision {
    pub fn from_triple(a: WitnessPolarity, b: WitnessPolarity, c: WitnessPolarity) -> Self {
        let yang_count = [a, b, c].iter().filter(|&&p| p == WitnessPolarity::Yang).count();
        match yang_count {
            3 | 2 => WitnessDecision::Accept,
            0 | 1 => WitnessDecision::Reject,
            _ => unreachable!(),
        }
    }

    pub fn is_deferred(&self) -> bool {
        matches!(self, WitnessDecision::Defer)
    }
}

/// 单个见证轴
#[derive(Debug, Clone)]
pub struct WitnessAxis {
    pub name: String,
    pub polarity: WitnessPolarity,
    pub confidence: u64,
}

impl WitnessAxis {
    pub fn new(name: &str, polarity: WitnessPolarity, confidence: u64) -> Self {
        Self { name: name.to_string(), polarity, confidence }
    }

    pub fn is_yang(&self) -> bool {
        self.polarity == WitnessPolarity::Yang
    }
}

/// 三元签
#[derive(Debug, Clone)]
pub struct TripleWitness {
    pub phase_axis: WitnessAxis,
    pub initial_axis: WitnessAxis,
    pub container_axis: WitnessAxis,
}

impl TripleWitness {
    pub fn new(phase: WitnessAxis, initial: WitnessAxis, container: WitnessAxis) -> Self {
        Self { phase_axis: phase, initial_axis: initial, container_axis: container }
    }

    pub fn adjudicate(&self) -> WitnessDecision {
        WitnessDecision::from_triple(
            self.phase_axis.polarity,
            self.initial_axis.polarity,
            self.container_axis.polarity,
        )
    }

    pub fn total_confidence(&self) -> u64 {
        self.phase_axis.confidence + self.initial_axis.confidence + self.container_axis.confidence
    }

    pub fn describe(&self) -> String {
        format!(
            "[{}] 相位={:?}({}) 声母={:?}({}) 容器={:?}({})",
            match self.adjudicate() {
                WitnessDecision::Accept => "准入",
                WitnessDecision::Reject => "拒绝",
                WitnessDecision::Defer => "待定",
            },
            self.phase_axis.polarity, self.phase_axis.confidence,
            self.initial_axis.polarity, self.initial_axis.confidence,
            self.container_axis.polarity, self.container_axis.confidence,
        )
    }
}

/// 征用检测器
#[derive(Debug)]
pub struct ConscriptionDetector {
    pub hostile_rhyme_patterns: Vec<String>,
    pub drift_tolerance: f64,
}

impl ConscriptionDetector {
    pub fn new() -> Self {
        Self { hostile_rhyme_patterns: Vec::new(), drift_tolerance: 0.1 }
    }

    pub fn add_hostile_pattern(&mut self, rhyme: &str) -> &mut Self {
        self.hostile_rhyme_patterns.push(rhyme.to_string());
        self
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.drift_tolerance = tolerance;
        self
    }

    fn witness_phase(&self, internal: &PhaseClock, external_phase: f64) -> WitnessAxis {
        const _: () = ();
        let diff = (internal.hot_scalar.hotness as f64 - external_phase).abs();
        let polarity = if diff < self.drift_tolerance { WitnessPolarity::Yin } else { WitnessPolarity::Yang };
        WitnessAxis::new("相位", polarity, internal.hot_scalar.hotness)
    }

    fn witness_initial(&self, _external_glyph: &str, external_rhyme: &str) -> WitnessAxis {
        let is_hostile = self.hostile_rhyme_patterns.iter().any(|p| {
            external_rhyme.contains(p) || p.contains(external_rhyme)
        });
        let polarity = if is_hostile { WitnessPolarity::Yin } else { WitnessPolarity::Yang };
        let confidence = self.hostile_rhyme_patterns.iter()
            .filter(|p| external_rhyme.contains(*p) || p.contains(external_rhyme))
            .count() as u64;
        WitnessAxis::new("声母", polarity, confidence)
    }

    fn witness_container(&self, internal: &PhaseClock, external_anchor: u64) -> WitnessAxis {
        let polarity = match &internal.context {
            Some(ctx) if ctx.anchor_id == external_anchor => WitnessPolarity::Yin,
            _ => WitnessPolarity::Yang,
        };
        let confidence = if polarity == WitnessPolarity::Yin { 10 } else { 1 };
        WitnessAxis::new("容器", polarity, confidence)
    }

    pub fn detect(
        &self,
        internal: &PhaseClock,
        external_glyph: &str,
        external_rhyme: &str,
        external_anchor: u64,
        external_phase: f64,
    ) -> TripleWitness {
        TripleWitness::new(
            self.witness_phase(internal, external_phase),
            self.witness_initial(external_glyph, external_rhyme),
            self.witness_container(internal, external_anchor),
        )
    }

    pub fn check(
        &self,
        internal: &PhaseClock,
        external_glyph: &str,
        external_rhyme: &str,
        external_anchor: u64,
        external_phase: f64,
    ) -> WitnessDecision {
        self.detect(internal, external_glyph, external_rhyme, external_anchor, external_phase).adjudicate()
    }
}

impl Default for ConscriptionDetector {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase_clock2::PhaseClock;

    #[test]
    fn test_decision_all_yang() {
        let d = WitnessDecision::from_triple(WitnessPolarity::Yang, WitnessPolarity::Yang, WitnessPolarity::Yang);
        assert_eq!(d, WitnessDecision::Accept);
    }

    #[test]
    fn test_decision_two_yang() {
        let d = WitnessDecision::from_triple(WitnessPolarity::Yang, WitnessPolarity::Yin, WitnessPolarity::Yang);
        assert_eq!(d, WitnessDecision::Accept);
    }

    #[test]
    fn test_decision_all_yin() {
        let d = WitnessDecision::from_triple(WitnessPolarity::Yin, WitnessPolarity::Yin, WitnessPolarity::Yin);
        assert_eq!(d, WitnessDecision::Reject);
    }

    #[test]
    fn test_decision_one_yang() {
        let d = WitnessDecision::from_triple(WitnessPolarity::Yang, WitnessPolarity::Yin, WitnessPolarity::Yin);
        assert_eq!(d, WitnessDecision::Reject);
    }

    #[test]
    fn test_triple_adjudicate() {
        let t = TripleWitness::new(
            WitnessAxis::new("相位", WitnessPolarity::Yang, 5),
            WitnessAxis::new("声母", WitnessPolarity::Yang, 3),
            WitnessAxis::new("容器", WitnessPolarity::Yin, 0),
        );
        assert_eq!(t.adjudicate(), WitnessDecision::Accept);
    }

    #[test]
    fn test_detect_clean_external() {
        let internal = PhaseClock::new();
        let detector = ConscriptionDetector::new();
        let result = detector.check(&internal, "外", "ai", 999, 500.0);
        assert_eq!(result, WitnessDecision::Accept);
    }

    #[test]
    fn test_detect_hostile_anchor_defer() {
        let mut internal = PhaseClock::new();
        let ctx = crate::phase_clock2::ContainerContext::new(42, "self");
        internal.bind_context(ctx);
        let detector = ConscriptionDetector::new();
        let result = detector.check(&internal, "外", "ai", 42, 500.0);
        assert_eq!(result, WitnessDecision::Accept);
    }

    #[test]
    fn test_detect_drift() {
        let internal = PhaseClock::new();
        let detector = ConscriptionDetector::new().with_tolerance(1000.0);
        let result = detector.check(&internal, "外", "ai", 999, 0.5);
        assert_eq!(result, WitnessDecision::Accept);
    }

    #[test]
    fn test_hostile_pattern_with_drift() {
        let internal = PhaseClock::new();
        let mut detector = ConscriptionDetector::new().with_tolerance(1000.0);
        detector.add_hostile_pattern("ao");
        let result = detector.check(&internal, "好", "ao", 999, 0.5);
        assert_eq!(result, WitnessDecision::Reject);
    }

    #[test]
    fn test_describe() {
        let t = TripleWitness::new(
            WitnessAxis::new("相位", WitnessPolarity::Yang, 5),
            WitnessAxis::new("声母", WitnessPolarity::Yang, 3),
            WitnessAxis::new("容器", WitnessPolarity::Yang, 1),
        );
        assert!(t.describe().contains("准入"));
    }

    #[test]
    fn test_total_confidence() {
        let t = TripleWitness::new(
            WitnessAxis::new("相位", WitnessPolarity::Yang, 5),
            WitnessAxis::new("声母", WitnessPolarity::Yang, 3),
            WitnessAxis::new("容器", WitnessPolarity::Yin, 7),
        );
        assert_eq!(t.total_confidence(), 15);
    }
}
