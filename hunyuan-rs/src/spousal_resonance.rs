// ══════════════════════════════════════════════════════════════════════
// spousal_resonance.rs — 夫妻共鸣判定 · NJSG裁决引擎 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 输入: 两个汉字 (LivingGlyph)
// 过程: 计算声韵/极性/家族/距离 → 综合分
// 输出: 共鸣等级 + 裁决建议 + 见证签名
//
// 裁决逻辑:
//   - 同家族 + 同极性 = 僵局 (无发展)
//   - 同家族 + 反极性 = 夫妻 (动静相宜)
//   - 异家族 + 同极性 = 竞争 (互相消耗)
//   - 异家族 + 反极性 = 互补 (最佳发展)
//
// 约束:
//   - 仅加减 + 比较 (无乘除)
//   - 裁决基于确定性规则 (无随机)
// ══════════════════════════════════════════════════════════════════════

use crate::glyph_living::{LivingGlyph, GlyphFamily};
use crate::container_dynamics::{JudgmentVerdict, ContainerRelation};

/// 共鸣等级阈值 (静态常量)
const TH_HARMONIOUS: i64 = 15;
const TH_COMPATIBLE: i64 = 5;
const TH_STAGNANT: i64 = -5;

// ────────────────────────────────────────────────────────────
// 共鸣等级
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResonanceLevel {
    Harmonious,    // 和鸣 — 最佳共鸣 (天长地久)
    Compatible,    // 兼容 — 可用, 需磨合
    Stagnant,      // 僵持 — 同极性同家族, 无发展
    Conflicting,   // 冲突 — 互相消耗
    Severed,       // 断裂 — 无法共鸣
}

impl ResonanceLevel {
    pub fn description(self) -> &'static str {
        match self {
            Self::Harmonious => "和鸣 — 互补共生, 动静相宜, 可天长地久",
            Self::Compatible => "兼容 — 声韵有差异, 但极性互补, 需磨合",
            Self::Stagnant => "僵持 — 同极性同家族, 无张力也无发展",
            Self::Conflicting => "冲突 — 同极性异家族, 互相消耗",
            Self::Severed => "断裂 — 极性相反又无关系, 难以共鸣",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 共鸣报告
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ResonanceReport {
    pub glyph_a: char,
    pub glyph_b: char,
    pub level: ResonanceLevel,
    pub polarity_harmony: i64,      // 极性和谐度 (+10 ~ -10)
    pub family_tension: i64,        // 家族张力 (+10 ~ -10)
    pub distance_score: f64,        // 距离得分 (0..1)
    pub final_score: i64,           // 综合得分
    pub judgment: JudgmentVerdict,  // 容器动力学裁决
    pub can_witness: bool,          // 是否能被三元见证
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

// ────────────────────────────────────────────────────────────
// 夫妻共鸣引擎
// ────────────────────────────────────────────────────────────

pub struct SpousalResonanceEngine;

impl SpousalResonanceEngine {
    pub fn new() -> Self {
        Self
    }

    // ────────────────────────────────────────────────────────
    // 核心裁决: 两个汉字的共鸣
    // ────────────────────────────────────────────────────────

    pub fn judge(&self, a: &LivingGlyph, b: &LivingGlyph) -> ResonanceReport {
        // 1. 极性和谐度
        let polarity_harmony = Self::calc_polarity_harmony(a, b);

        // 2. 家族张力
        let family_tension = Self::calc_family_tension(a, b);

        // 3. 距离得分 (归一化到 0..1, 1=最近)
        let distance_score = Self::normalize_distance(a.distance_to(b).total);

        // 4. 综合分
        let final_score = polarity_harmony + family_tension;

        // 5. 定级
        let level = Self::score_to_level(final_score);

        // 6. 容器动力学裁决
        let is_alive = a.witness_count > 0 || b.witness_count > 0;
        let is_strong = final_score > 0;
        let has_loop = matches!(level, ResonanceLevel::Harmonious | ResonanceLevel::Compatible);
        let judgment = ContainerRelation::judge_raw(is_alive, is_strong, has_loop);

        // 7. 能否被三元见证: 至少一方已有见证, 且非断裂
        let can_witness = (a.witness_count > 0 || b.witness_count > 0)
            && !matches!(level, ResonanceLevel::Severed);

        ResonanceReport {
            glyph_a: a.base,
            glyph_b: b.base,
            level,
            polarity_harmony,
            family_tension,
            distance_score,
            final_score,
            judgment,
            can_witness,
        }
    }

    // ────────────────────────────────────────────────────────
    // 极性和谐度计算
    // ────────────────────────────────────────────────────────

    fn calc_polarity_harmony(a: &LivingGlyph, b: &LivingGlyph) -> i64 {
        let pa = a.family.polarity_bias();
        let pb = b.family.polarity_bias();

        if pa == pb {
            // 同极性: +5 (有共鸣但无张力)
            5
        } else {
            // 反极性: +10 (动静相宜, 最佳)
            10
        }
    }

    // ────────────────────────────────────────────────────────
    // 家族张力计算
    // ────────────────────────────────────────────────────────

    fn calc_family_tension(a: &LivingGlyph, b: &LivingGlyph) -> i64 {
        match (a.family, b.family) {
            // 同家族: 张力0 (无差异)
            (GlyphFamily::Container, GlyphFamily::Container) => 0,
            (GlyphFamily::Operation, GlyphFamily::Operation) => 0,
            (GlyphFamily::Couple, GlyphFamily::Couple) => 0,
            (GlyphFamily::Homophone, GlyphFamily::Homophone) => 0,
            (GlyphFamily::Phallus, GlyphFamily::Phallus) => 0,
            (GlyphFamily::YinYang, GlyphFamily::YinYang) => 0,

            // 夫妻+任何: 基础+5 (夫妻为贵)
            (GlyphFamily::Couple, _) => 5,
            (_, GlyphFamily::Couple) => 5,

            // 阴阳+任何: +3 (阴阳调和)
            (GlyphFamily::YinYang, _) => 3,
            (_, GlyphFamily::YinYang) => 3,

            // 谐音+其他: +2 (有语言桥梁)
            (GlyphFamily::Homophone, _) => 2,
            (_, GlyphFamily::Homophone) => 2,

            // 容器+操作: +1 (被动+主动, 小互补)
            (GlyphFamily::Operation, GlyphFamily::Container) => 1,
            (GlyphFamily::Container, GlyphFamily::Operation) => 1,

            _ => 0,
        }
    }

    // ────────────────────────────────────────────────────────
    // 距离归一化 (远大分低, 近高分)
    // ────────────────────────────────────────────────────────

    fn normalize_distance(d: f64) -> f64 {
        if d <= 0.0 {
            1.0
        } else if d >= 200.0 {
            0.0
        } else {
            1.0 - (d / 200.0)
        }
    }

    // ────────────────────────────────────────────────────────
    // 综合分 → 共鸣等级
    // ────────────────────────────────────────────────────────

    fn score_to_level(score: i64) -> ResonanceLevel {
        if score >= TH_HARMONIOUS {
            ResonanceLevel::Harmonious
        } else if score >= TH_COMPATIBLE {
            ResonanceLevel::Compatible
        } else if score >= TH_STAGNANT {
            ResonanceLevel::Stagnant
        } else if score > -10 {
            ResonanceLevel::Conflicting
        } else {
            ResonanceLevel::Severed
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glyph_living::{Shengmu, Yunmu, Tone};

    fn make(glyph: char, family: GlyphFamily) -> LivingGlyph {
        LivingGlyph::new(glyph, Shengmu::N, Yunmu::An, Tone::T1, family)
    }

    #[test]
    fn test_resonance_couple_pair() {
        // 囡(Couple) + 囝(Couple): 同极性+5, 同家族+0 → +5 Compatible
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
        // 阴(YinYang) + 囝(Couple): 反极性+10, Couple+5 → +15 Harmonious
        let a = make('阴', GlyphFamily::YinYang);
        let b = make('囝', GlyphFamily::Couple);
        let engine = SpousalResonanceEngine::new();
        let report = engine.judge(&a, &b);

        assert_eq!(report.polarity_harmony, 10);
        assert_eq!(report.family_tension, 5);
        assert_eq!(report.level, ResonanceLevel::Harmonious);
        assert!(report.can_witness);
    }

    #[test]
    fn test_resonance_same_family_same_polarity() {
        // 袋(Container) + 罐(Container): 同极性+5, 同家族+0 → +5 Compatible
        let a = make('袋', GlyphFamily::Container);
        let b = make('罐', GlyphFamily::Container);
        let engine = SpousalResonanceEngine::new();
        let report = engine.judge(&a, &b);

        assert_eq!(report.polarity_harmony, 5);
        assert_eq!(report.family_tension, 0);
        assert_eq!(report.final_score, 5);
    }

    #[test]
    fn test_resonance_container_operation() {
        // 照(Operation) + 袋(Container): 极性差异 + 容器操作互补
        let a = LivingGlyph::new('照', Shengmu::Zh, Yunmu::Ao, Tone::T4, GlyphFamily::Operation);
        let b = LivingGlyph::new('袋', Shengmu::D, Yunmu::Ai, Tone::T4, GlyphFamily::Container);
        let report = SpousalResonanceEngine::new().judge(&a, &b);

        // Operation极性=阳, Container极性=阴 → 反极性+10
        assert_eq!(report.polarity_harmony, 10);
        // Container+Operation → +1
        assert_eq!(report.family_tension, 1);
        assert_eq!(report.final_score, 11);
    }

    #[test]
    fn test_judgment_mapping() {
        // 高分解码 → Maintain
        let a = LivingGlyph::new('阴', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::YinYang);
        let b = LivingGlyph::new('囝', Shengmu::J, Yunmu::Iou, Tone::T1, GlyphFamily::Couple);
        let report = SpousalResonanceEngine::new().judge(&a, &b);

        assert_eq!(report.judgment, JudgmentVerdict::Maintain);
    }

    #[test]
    fn test_resonance_level_description() {
        assert!(matches!(
            ResonanceLevel::Harmonious.description(),
            "和鸣 — 互补共生, 动静相宜, 可天长地久"
        ));
    }
}
