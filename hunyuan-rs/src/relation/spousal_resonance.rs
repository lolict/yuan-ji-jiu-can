// ══════════════════════════════════════════════════════════════════════
// spousal_resonance.rs — 夫妻共鸣判定 · NJSG裁决引擎 (v0.1)
// ══════════════════════════════════════════════════════════════════════

use crate::glyph_living::{LivingGlyph, GlyphFamily};
use crate::container_dynamics::{JudgmentVerdict, ContainerRelation};

const TH_HARMONIOUS: i64 = 15;
const TH_COMPATIBLE: i64 = 5;
const TH_STAGNANT: i64 = -5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResonanceLevel {
    Harmonious, Compatible, Stagnant, Conflicting, Severed,
}

impl ResonanceLevel {
    pub fn description(self) -> &'static str {
        match self {
            Self::Harmonious => "和鸣 — 互补共生, 动静相宜",
            Self::Compatible => "兼容 — 可用, 需磨合",
            Self::Stagnant => "僵持 — 同极性同家族, 无发展",
            Self::Conflicting => "冲突 — 互相消耗",
            Self::Severed => "断裂 — 无法共鸣",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResonanceReport {
    pub glyph_a: char,
    pub glyph_b: char,
    pub level: ResonanceLevel,
    pub polarity_harmony: i64,
    pub family_tension: i64,
    pub distance_score: f64,
    pub final_score: i64,
    pub judgment: JudgmentVerdict,
    pub can_witness: bool,
}

impl ResonanceReport {
    pub fn summary(&self) -> String {
        format!(
            "【{} + {}】→ 共鸣等级: {:?} | 裁决: {:?} | 见证: {}",
            self.glyph_a, self.glyph_b,
            self.level, self.judgment,
            if self.can_witness { "✓ 通过" } else { "✗ 未通过" }
        )
    }
}

pub struct SpousalResonanceEngine;

impl SpousalResonanceEngine {
    pub fn new() -> Self { Self }

    pub fn judge(&self, a: &LivingGlyph, b: &LivingGlyph) -> ResonanceReport {
        let polarity_harmony = Self::calc_polarity_harmony(a, b);
        let family_tension = Self::calc_family_tension(a, b);
        let distance_score = Self::normalize_distance(a.distance_to(b).total);
        let final_score = polarity_harmony + family_tension;
        let level = Self::score_to_level(final_score);
        // 活容器判定: 显式见证 > 0, 或夫妻图腾互证 (YinYang + Couple = 囡囝)
        use GlyphFamily::*;
        let couple_yinyang = matches!((a.family, b.family),
            (YinYang, Couple) | (Couple, YinYang));
        let is_alive = a.witness_count > 0 || b.witness_count > 0 || couple_yinyang;
        let is_strong = final_score > 0;
        let has_loop = matches!(level, ResonanceLevel::Harmonious | ResonanceLevel::Compatible);
        let judgment = ContainerRelation::judge_raw(is_alive, is_strong, has_loop);
        let can_witness = (a.witness_count > 0 || b.witness_count > 0)
            && !matches!(level, ResonanceLevel::Severed);

        ResonanceReport {
            glyph_a: a.base, glyph_b: b.base, level,
            polarity_harmony, family_tension, distance_score,
            final_score, judgment, can_witness,
        }
    }

    /// 字符级极性: 阴=NEGATIVE(1), 阳=POSITIVE(0), 其他按家族
    /// 家族极性无法区分阴/阳(同属 YinYang 家族),需落到字符层
    fn glyph_polarity(g: &LivingGlyph) -> u64 {
        use crate::mirror;
        match g.base {
            '阴' => mirror::POLARITY_NEGATIVE,
            '阳' => mirror::POLARITY_POSITIVE,
            _ => g.family.polarity_bias(),
        }
    }

    fn calc_polarity_harmony(a: &LivingGlyph, b: &LivingGlyph) -> i64 {
        if Self::glyph_polarity(a) == Self::glyph_polarity(b) { 5 } else { 10 }
    }

    fn calc_family_tension(a: &LivingGlyph, b: &LivingGlyph) -> i64 {
        use GlyphFamily::*;
        match (a.family, b.family) {
            (Container, Container) | (Operation, Operation) | (Couple, Couple)
            | (Homophone, Homophone) | (Phallus, Phallus) | (YinYang, YinYang) => 0,
            (Couple, _) | (_, Couple) => 5,
            (YinYang, _) | (_, YinYang) => 3,
            (Homophone, _) | (_, Homophone) => 2,
            (Operation, Container) | (Container, Operation) => 1,
            _ => 0,
        }
    }

    fn normalize_distance(d: f64) -> f64 {
        if d <= 0.0 { 1.0 } else if d >= 200.0 { 0.0 } else { 1.0 - (d / 200.0) }
    }

    fn score_to_level(score: i64) -> ResonanceLevel {
        if score >= TH_HARMONIOUS { ResonanceLevel::Harmonious }
        else if score >= TH_COMPATIBLE { ResonanceLevel::Compatible }
        else if score >= TH_STAGNANT { ResonanceLevel::Stagnant }
        else if score > -10 { ResonanceLevel::Conflicting }
        else { ResonanceLevel::Severed }
    }
}

impl Default for SpousalResonanceEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glyph_living::{Shengmu, Yunmu, Tone};

    fn make(glyph: char, family: GlyphFamily) -> LivingGlyph {
        LivingGlyph::new(glyph, Shengmu::N, Yunmu::An, Tone::T1, family)
    }

    #[test]
    fn test_resonance_couple_pair() {
        let a = make('囡', GlyphFamily::Couple);
        let b = make('囝', GlyphFamily::Couple);
        let engine = SpousalResonanceEngine::new();
        let report = engine.judge(&a, &b);
        assert_eq!(report.polarity_harmony, 5);
        assert_eq!(report.family_tension, 0);
        assert_eq!(report.final_score, 5);
    }

    #[test]
    fn test_resonance_yinyang_couple() {
        let a = make('阴', GlyphFamily::YinYang);
        let b = make('囝', GlyphFamily::Couple);
        let report = SpousalResonanceEngine::new().judge(&a, &b);
        assert_eq!(report.polarity_harmony, 10);
        assert_eq!(report.family_tension, 5);
        assert_eq!(report.level, ResonanceLevel::Harmonious);
    }

    #[test]
    fn test_resonance_container_pair() {
        let a = make('袋', GlyphFamily::Container);
        let b = make('罐', GlyphFamily::Container);
        let report = SpousalResonanceEngine::new().judge(&a, &b);
        assert_eq!(report.final_score, 5);
    }

    #[test]
    fn test_judgment_mapping() {
        let a = LivingGlyph::new('阴', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::YinYang);
        let b = LivingGlyph::new('囝', Shengmu::J, Yunmu::Iou, Tone::T1, GlyphFamily::Couple);
        let report = SpousalResonanceEngine::new().judge(&a, &b);
        assert_eq!(report.judgment, JudgmentVerdict::Maintain);
    }

    #[test]
    fn test_resonance_level_description() {
        assert!(ResonanceLevel::Harmonious.description().len() > 0);
    }
}
