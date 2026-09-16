pub use crate::cross_isa::fridge::ContainerPack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict { Admit, Reject, Pending, Witness }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reward { Plus, Zero, Minus }

#[derive(Debug, Clone, Copy)]
pub struct JudgmentConfig {
    pub empty_reject: bool,
    pub zero_reject: bool,
    pub alignment_required: bool,
    pub sovereignty_min: u8,
}

impl Default for JudgmentConfig {
    fn default() -> Self {
        Self { empty_reject: true, zero_reject: false, alignment_required: false, sovereignty_min: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct JudgmentResult {
    pub pack_id: u64,
    pub verdict: Verdict,
    pub reward: Reward,
    pub score: f64,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BatchJudgment {
    pub results: Vec<JudgmentResult>,
    pub admit_count: usize,
    pub reject_count: usize,
    pub witness_count: usize,
    pub total_score: f64,
}

impl BatchJudgment {
    pub fn stats(&self) -> String {
        let avg = if self.results.is_empty() { 0.0 } else { self.total_score / self.results.len() as f64 };
        format!("Judgment[admit={} reject={} witness={} avg_score={:.3} plus={}]",
            self.admit_count, self.reject_count, self.witness_count, avg,
            self.results.iter().filter(|r| matches!(r.reward, Reward::Plus)).count())
    }
}

pub struct JudgmentEngine { config: JudgmentConfig }

impl JudgmentEngine {
    pub fn new(config: JudgmentConfig) -> Self { Self { config } }
    pub fn default() -> Self { Self { config: JudgmentConfig::default() } }

    pub fn judge(&self, pack: &ContainerPack) -> JudgmentResult {
        let mut reasons: Vec<String> = Vec::new();
        let mut score = 0.5;

        let all_ff = pack.payload.iter().all(|b| *b == 0xFF);
        if self.config.empty_reject && all_ff {
            reasons.push("empty_payload".into());
            return JudgmentResult { pack_id: pack.pack_id, verdict: Verdict::Reject, reward: Reward::Minus, score: 0.0, reasons };
        }

        let all_zero = pack.payload.iter().all(|b| *b == 0);
        if self.config.zero_reject && all_zero {
            reasons.push("zero_payload".into());
            score -= 0.3;
        }

        if self.config.alignment_required && pack.align_flag == 0 {
            reasons.push("alignment_missing".into());
            score -= 0.15;
        }

        let sov = pack.coord().y_scale_x().min(9);
        score += 0.1 * sov as f64;

        if sov < self.config.sovereignty_min {
            reasons.push(format!("sovereignty_low({}/{})", sov, self.config.sovereignty_min));
            score -= 0.1 * (self.config.sovereignty_min.saturating_sub(sov)) as f64;
        }

        let witness_valid = pack.witness != [0xFF, 0xFF, 0xFF] && pack.witness != [0xFF, 0xFE, 0xFD];
        if witness_valid { score += 0.15; } else { score -= 0.1; reasons.push("witness_invalid".into()); }

        score = score.clamp(0.0, 1.0);

        let verdict = if score > 0.7 { Verdict::Admit } else if score < 0.3 { Verdict::Reject } else { Verdict::Witness };
        let reward = match verdict { Verdict::Admit => Reward::Plus, Verdict::Reject => Reward::Minus, _ => Reward::Zero };

        JudgmentResult { pack_id: pack.pack_id, verdict, reward, score, reasons }
    }

    pub fn judge_batch(&self, packs: &[ContainerPack]) -> BatchJudgment {
        let results: Vec<JudgmentResult> = packs.iter().map(|p| self.judge(p)).collect();
        let admit_count = results.iter().filter(|r| matches!(r.verdict, Verdict::Admit)).count();
        let reject_count = results.iter().filter(|r| matches!(r.verdict, Verdict::Reject)).count();
        let witness_count = results.iter().filter(|r| matches!(r.verdict, Verdict::Witness)).count();
        let total_score: f64 = results.iter().map(|r| r.score).sum();
        BatchJudgment { results, admit_count, reject_count, witness_count, total_score }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cross_isa::coordinate::Coord3D;

    fn demo_pack(id: u64, payload: Vec<u8>) -> ContainerPack {
        ContainerPack::new(id, Coord3D::new(1, 0x0050, 1), 1, payload)
    }

    #[test] fn default_admits_valid() {
        let engine = JudgmentEngine::default();
        let pack = demo_pack(1, vec![0x01, 0x02, 0x03, 0x04]);
        let r = engine.judge(&pack);
        assert_eq!(r.verdict, Verdict::Admit);
    }

    #[test] fn rejects_empty_when_configured() {
        let engine = JudgmentEngine::new(JudgmentConfig { empty_reject: true, ..Default::default() });
        let pack = demo_pack(2, vec![0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(engine.judge(&pack).verdict, Verdict::Reject);
    }

    #[test] fn witness_affects_score() {
        let engine = JudgmentEngine::default();
        let pack = demo_pack(3, vec![0x01, 0x02, 0x03, 0x04]);
        let r1 = engine.judge(&pack);
        assert!(r1.score > 0.0);
    }

    #[test] fn batch_counts_correctly() {
        let engine = JudgmentEngine::default();
        let packs = vec![
            demo_pack(1, vec![0x01]),
            demo_pack(2, vec![0xFF, 0xFF, 0xFF, 0xFF]),
        ];
        let batch = engine.judge_batch(&packs);
        assert_eq!(batch.results.len(), 2);
        assert_eq!(batch.admit_count + batch.reject_count + batch.witness_count, 2);
    }

    #[test] fn sovereignty_min_rejects_low() {
        let engine = JudgmentEngine::new(JudgmentConfig { sovereignty_min: 5, ..Default::default() });
        let pack = ContainerPack::new(1, Coord3D::new(1, 0x0000, 1), 1, vec![0x01, 0x02]);
        let r = engine.judge(&pack);
        assert!(r.score < 0.6);
    }
}
