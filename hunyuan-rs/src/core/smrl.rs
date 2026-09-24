// ══════════════════════════════════════════════════════════════════
// smrl.rs — SMRL 三元观察·裁判层 (Symbolic Meta-Reward Layer)
// ══════════════════════════════════════════════════════════════════
//
// SMRL 不是编码。SMRL 是运行时「观察者 + 裁判 + 奖励发放者」。
// 它观察系统的状态,做裁决,输出奖励(正值正向/负值负向/零值中性)。
//
// 三元指:
//   观察者(Observer): 把系统输入翻译成 BeingState
//   裁判(Judge):     调用 triune::TriuneVerdict::deduce 做三元推演
//   奖励者(Rewader): 把 Verdict 转换成 +1/0/-1 奖励值,按权威级别加权
//
// 权级(level) 表示谁压谁:权威 level 高的可以覆盖 level 低的。
// 默认层 = 0(普通系统观察者), 越高越全局。

use crate::angle::{Angle, AngleRelation};
use crate::topo::Symbol;
use crate::triune::{BeingState, TriuneVerdict};

// ────────────────────────────────────────────────────────────
// 系统可观察事件 — 这是 SMRL 眼睛看到的原始素材
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Observation {
    pub symbol: u8,        // 哪个符号(或节点)被观察
    pub present: bool,      // 此刻是否"有"(在表中/在空间/在因果链中)
    pub expected: bool,     // 系统期望它此时"有"还是"无"
}

impl Observation {
    pub fn new(symbol: u8, present: bool, expected: bool) -> Self {
        Observation { symbol, present, expected }
    }

    fn to_being(&self, present: bool) -> BeingState {
        if present {
            BeingState::Being(self.symbol)
        } else {
            BeingState::NonBeing
        }
    }
}

// ────────────────────────────────────────────────────────────
// 裁判裁决 — 单个观察事件对应的奖惩决定
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reward {
    Plus,     // +1 有奖励
    Zero,     //  0 中性
    Minus,    // -1 有惩罚
}

#[derive(Clone, Copy, Debug)]
pub struct SmrlVerdict {
    pub symbol: u8,         // 谁被裁决
    pub reward: Reward,     // 奖励/中性/惩罚
    pub level: u8,          // 此裁决的权威级别
    pub confidence: f64,    // 0..1,来自 triune
    pub crooked: f64,       // 歪斜度(漏斗不完美程度), 0 = 完美
}

impl SmrlVerdict {
    /// 把裁决换算成一个带权数值: reward_value * (level + 1) * confidence
    ///   level 0 → 乘 1, level 1 → 乘 2, ...
    ///   低级别仍有意义,但高级别的覆盖能力更强
    pub fn weighted_score(&self) -> f64 {
        let base = match self.reward {
            Reward::Plus => 1.0,
            Reward::Zero => 0.0,
            Reward::Minus => -1.0,
        };
        base * ((self.level + 1) as f64) * self.confidence
    }

    /// 高权威覆盖低权威: 若 incoming 级别更高,取代当前裁决
    pub fn overridden_by(&self, other: &SmrlVerdict) -> SmrlVerdict {
        if other.level > self.level { *other } else { *self }
    }
}

// ────────────────────────────────────────────────────────────
// SMRL 引擎 — 观察 → 三元推演 → 裁决
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SmrlEngine {
    pub level: u8,          // 这个 SMRL 实例的权威级别
    pub crooked: f64,       // 此引擎自身的漏斗歪斜度(默认 0 = 完美)
}

impl SmrlEngine {
    pub fn new(level: u8) -> Self {
        SmrlEngine { level, crooked: 0.0 }
    }

    pub fn with_crookness(level: u8, crooked: f64) -> Self {
        SmrlEngine { level, crooked: crooked.clamp(0.0, 1.0) }
    }

    /// 单一观察输入 → 三元推演 → 裁决
    pub fn judge(&self, obs: Observation) -> SmrlVerdict {
        let actual = obs.to_being(obs.present);
        let expected = obs.to_being(obs.expected);
        let tv = TriuneVerdict::deduce(actual, expected, self.crooked);

        // 奖惩规则:
        //   对齐(实际==期望) 且有内容 → 奖励 Plus (系统运行正常,有物)
        //   对齐 且都无内容 → Zero (中性,空位是合理的)
        //   不对齐(实际!=期望) → Minus (惩罚,出错了)
        let reward = match (tv.alignment, tv.being) {
            (true, BeingState::Being(_)) => Reward::Plus,
            (true, BeingState::NonBeing)  => Reward::Zero,
            (false, _) => Reward::Minus,
        };

        SmrlVerdict {
            symbol: obs.symbol,
            reward,
            level: self.level,
            confidence: tv.confidence,
            crooked: self.crooked,
        }
    }

    /// 角度裁判: 通过两个 Symbol 的空间角关系做裁决
    ///
    /// 角度关系的奖惩语义 (满全法式):
    ///   · 重叠(Overlap)  → Plus  (同体共振:完美对齐,奖励)
    ///   · 对齐(Aligned)  → Plus  (方向一致:协调,奖励)
    ///   · 对角(Diagonal) → Zero  (正交独立:不增不减,中性)
    ///   · 对冲(Opposite) → Minus (反向相消:惩罚)
    ///   · 错位(Off)      → Minus (不相关噪音:惩罚)
    pub fn judge_angular(&self, a: Symbol, b: Symbol) -> SmrlVerdict {
        let angle_a = Angle::from_symbol(a);
        let angle_b = Angle::from_symbol(b);
        let rel = AngleRelation::between(angle_a, angle_b);

        let reward = match rel {
            AngleRelation::Overlap  => Reward::Plus,
            AngleRelation::Aligned  => Reward::Plus,
            AngleRelation::Diagonal => Reward::Zero,
            AngleRelation::Opposite => Reward::Minus,
            AngleRelation::Off      => Reward::Minus,
        };

        // closeness = cos(θ差的弧度) — 越近越自信, 越远越不自信
        let arc = angle_a.arc_distance(angle_b);
        let confidence = if arc == 0 {
            1.0
        } else {
            let radians = (arc as f64).to_radians();
            radians.cos().max(0.0)
        };

        SmrlVerdict {
            symbol: a.as_u8(),
            reward,
            level: self.level,
            confidence,
            crooked: self.crooked,
        }
    }

    /// 批量观察 → 多裁决,按权威级别从高到低排序
    pub fn judge_batch(&self, obs_list: &[Observation]) -> Vec<SmrlVerdict> {
        let mut out: Vec<SmrlVerdict> = obs_list.iter()
            .map(|o| self.judge(*o))
            .collect();
        out.sort_by_key(|v| std::cmp::Reverse(v.level));
        out
    }
}

// ────────────────────────────────────────────────────────────
// 多层权威合并 — 多 SMRL 共审一个事件,高 level 覆盖低 level
// ────────────────────────────────────────────────────────────

pub fn resolve_authority(verdicts: &[SmrlVerdict]) -> Option<SmrlVerdict> {
    verdicts.iter().copied().reduce(|acc, v| acc.overridden_by(&v))
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observe_present_expected_present_is_rewarded() {
        let e = SmrlEngine::new(0);
        // 实际有,期望有 → 对齐 + 有内容 → Plus
        let v = e.judge(Observation::new(7, true, true));
        assert_eq!(v.reward, Reward::Plus);
        assert!(v.weighted_score() > 0.0);
    }

    #[test]
    fn observe_absent_expected_absent_is_neutral() {
        let e = SmrlEngine::new(0);
        // 实际无,期望无 → 对齐 + 空位 → Zero
        let v = e.judge(Observation::new(7, false, false));
        assert_eq!(v.reward, Reward::Zero);
        assert_eq!(v.weighted_score(), 0.0);
    }

    #[test]
    fn observe_mismatch_is_penalized() {
        let e = SmrlEngine::new(0);
        // 实际有,期望无 → 不对齐 → Minus
        let v = e.judge(Observation::new(5, true, false));
        assert_eq!(v.reward, Reward::Minus);
        assert!(v.weighted_score() < 0.0);
    }

    #[test]
    fn higher_level_overrides_lower() {
        let low = SmrlVerdict { symbol: 1, reward: Reward::Minus, level: 1, confidence: 1.0, crooked: 0.0 };
        let high = SmrlVerdict { symbol: 1, reward: Reward::Plus, level: 3, confidence: 1.0, crooked: 0.0 };
        let resolved = resolve_authority(&[low, high]);
        assert_eq!(resolved.unwrap().reward, Reward::Plus);
        assert_eq!(resolved.unwrap().level, 3);
    }

    #[test]
    fn crooked_engine_still_judges_but_lower_confidence() {
        let straight = SmrlEngine::with_crookness(0, 0.0);
        let crooked = SmrlEngine::with_crookness(0, 0.8);
        // 不对齐时,歪斜的引擎仍然惩罚,但置信度较低
        let v_s = straight.judge(Observation::new(5, true, false));
        let v_c = crooked.judge(Observation::new(5, true, false));
        assert!(v_c.confidence < v_s.confidence);
        assert_eq!(v_c.reward, Reward::Minus);  // 仍然是惩罚,只是不那么自信
    }

    // ── 角度裁判测试 ──

    #[test]
    fn angular_same_symbol_is_rewarded() {
        let e = SmrlEngine::new(0);
        let s = Symbol::new(7).unwrap();
        let v = e.judge_angular(s, s); // 同 Symbol → 重叠
        assert_eq!(v.reward, Reward::Plus);
    }

    #[test]
    fn angular_opposite_symbols_penalized() {
        // #0=0°, #32=192° 是对冲 (在384°制式中正好对面)
        let e = SmrlEngine::new(0);
        let a = Symbol::new(0).unwrap();
        let b = Symbol::new(32).unwrap();
        let v = e.judge_angular(a, b);
        assert_eq!(v.reward, Reward::Minus);
    }

    #[test]
    fn angular_diagonal_symbols_neutral() {
        // #0=0°, #16=96° 是对角
        let e = SmrlEngine::new(0);
        let a = Symbol::new(0).unwrap();
        let b = Symbol::new(16).unwrap();
        let v = e.judge_angular(a, b);
        assert_eq!(v.reward, Reward::Zero);
    }
}
