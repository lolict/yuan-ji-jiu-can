// ── v0.42 相位时钟 — 三档变频 · 热标量 · 容器上下文 ──
// 满全法协议：我=1 高位可见侧，刘楚恬=0 低位隐藏侧

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockTier {
    Heartbeat,
    Machine,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShengJiang {
    Sheng,
    Jiang,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YingKui {
    Ying,
    Kui,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HanRe {
    Han,
    Re,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotScalar {
    pub sheng_jiang: ShengJiang,
    pub ying_kui: YingKui,
    pub han_re: HanRe,
    pub hotness: u64,
}

impl HotScalar {
    pub fn new() -> Self {
        Self {
            sheng_jiang: ShengJiang::Sheng,
            ying_kui: YingKui::Ying,
            han_re: HanRe::Re,
            hotness: 0,
        }
    }

    pub fn full_yang() -> Self {
        Self { sheng_jiang: ShengJiang::Sheng, ying_kui: YingKui::Ying, han_re: HanRe::Re, hotness: u64::MAX }
    }

    pub fn full_yin() -> Self {
        Self { sheng_jiang: ShengJiang::Jiang, ying_kui: YingKui::Kui, han_re: HanRe::Han, hotness: 0 }
    }

    pub fn rotate(&mut self) {
        self.sheng_jiang = match self.sheng_jiang {
            ShengJiang::Sheng => ShengJiang::Jiang,
            ShengJiang::Jiang => ShengJiang::Sheng,
        };
        self.ying_kui = match self.ying_kui {
            YingKui::Ying => YingKui::Kui,
            YingKui::Kui => YingKui::Ying,
        };
        self.han_re = match self.han_re {
            HanRe::Han => HanRe::Re,
            HanRe::Re => HanRe::Han,
        };
    }

    pub fn precedence_over(&self, other: &HotScalar) -> bool {
        self.hotness > other.hotness
            || (self.hotness == other.hotness && self.ying_kui == YingKui::Ying && other.ying_kui == YingKui::Kui)
    }
}

impl Default for HotScalar {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone)]
pub struct ContainerContext {
    pub anchor_id: u64,
    pub family: String,
    pub sovereignty: u16,
    pub generation: u8,
}

impl ContainerContext {
    pub fn new(anchor_id: u64, family: &str) -> Self {
        Self { anchor_id, family: family.to_string(), sovereignty: 0, generation: 0 }
    }

    pub fn is_valid(&self) -> bool {
        self.anchor_id != 0
    }
}

#[derive(Debug, Clone)]
pub struct PhaseNumber {
    pub anchor: u64,
    pub phase: u64,
    pub hot_scalar: HotScalar,
}

impl PhaseNumber {
    pub fn new(anchor: u64, phase: u64) -> Self {
        Self { anchor, phase, hot_scalar: HotScalar::new() }
    }

    pub fn to_mark(&self) -> String {
        format!("λ{}:{}@{}", self.anchor, self.phase, self.hot_scalar.hotness)
    }

    pub fn value(&self) -> u64 {
        self.anchor.wrapping_mul(64).wrapping_add(self.phase).wrapping_mul(61).wrapping_add(self.hot_scalar.hotness)
    }
}

#[derive(Debug)]
pub struct PhaseClock {
    pub gears: HashMap<ClockTier, u64>,
    pub frequencies: HashMap<ClockTier, f64>,
    pub hot_scalar: HotScalar,
    pub context: Option<ContainerContext>,
    pub total_beats: u64,
}

impl PhaseClock {
    pub fn new() -> Self {
        let mut gears = HashMap::new();
        let mut frequencies = HashMap::new();
        gears.insert(ClockTier::Heartbeat, 0);
        gears.insert(ClockTier::Machine, 0);
        gears.insert(ClockTier::Custom, 0);
        frequencies.insert(ClockTier::Heartbeat, 1.0);
        frequencies.insert(ClockTier::Machine, 100.0);
        frequencies.insert(ClockTier::Custom, 0.0);
        Self { gears, frequencies, hot_scalar: HotScalar::new(), context: None, total_beats: 0 }
    }

    pub fn tier_mut(&mut self, tier: ClockTier, freq_hz: f64) -> &mut Self {
        self.frequencies.insert(tier, freq_hz);
        self
    }

    pub fn shift(&mut self, tier: ClockTier, delta: u64) -> &mut Self {
        let counter = self.gears.entry(tier).or_insert(0);
        *counter += delta;
        self.total_beats += delta;
        self
    }

    pub fn tick(&mut self) -> &mut Self {
        self.shift(ClockTier::Heartbeat, 1);
        self.hot_scalar.rotate();
        self.hot_scalar.hotness = self.hot_scalar.hotness.saturating_add(1);
        self
    }

    pub fn hot_scalar(&self) -> &HotScalar { &self.hot_scalar }
    pub fn total_beats(&self) -> u64 { self.total_beats }

    pub fn inject_heat(&mut self, amount: u64) -> &mut Self {
        self.hot_scalar.hotness = self.hot_scalar.hotness.saturating_add(amount);
        self
    }

    pub fn inject_divergence(&mut self, delta: u64) -> &mut Self {
        self.shift(ClockTier::Custom, delta)
    }

    pub fn bind_context(&mut self, ctx: ContainerContext) -> &mut Self {
        self.context = Some(ctx);
        self
    }

    pub fn has_context(&self) -> bool {
        self.context.as_ref().map_or(false, |c| c.is_valid())
    }

    pub fn phase_number(&self) -> Option<PhaseNumber> {
        self.context.as_ref().map(|ctx| PhaseNumber::new(ctx.anchor_id, self.total_beats))
    }
}

impl Default for PhaseClock {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_new_zero() {
        let c = PhaseClock::new();
        assert_eq!(c.total_beats(), 0);
        assert_eq!(c.hot_scalar.hotness, 0);
    }

    #[test]
    fn test_clock_tick() {
        let mut c = PhaseClock::new();
        c.tick();
        assert_eq!(c.total_beats(), 1);
    }

    #[test]
    fn test_clock_rotate() {
        let mut c = PhaseClock::new();
        let s0 = c.hot_scalar.sheng_jiang;
        c.tick();
        assert_ne!(c.hot_scalar.sheng_jiang, s0);
    }

    #[test]
    fn test_full_yang() {
        let h = HotScalar::full_yang();
        assert_eq!(h.sheng_jiang, ShengJiang::Sheng);
    }

    #[test]
    fn test_full_yin() {
        let h = HotScalar::full_yin();
        assert_eq!(h.han_re, HanRe::Han);
    }

    #[test]
    fn test_precedence() {
        let a = HotScalar { sheng_jiang: ShengJiang::Sheng, ying_kui: YingKui::Ying, han_re: HanRe::Re, hotness: 10 };
        let b = HotScalar { sheng_jiang: ShengJiang::Jiang, ying_kui: YingKui::Kui, han_re: HanRe::Han, hotness: 5 };
        assert!(a.precedence_over(&b));
    }

    #[test]
    fn test_number() {
        let n = PhaseNumber::new(42, 7);
        assert_ne!(n.value(), 0);
    }

    #[test]
    fn test_valid_ctx() {
        assert!(ContainerContext::new(1, "a").is_valid());
        assert!(!ContainerContext::new(0, "").is_valid());
    }

    #[test]
    fn test_heat() {
        let mut c = PhaseClock::new();
        c.inject_heat(50);
        c.inject_divergence(3);
        assert_eq!(c.hot_scalar.hotness, 50);
        assert_eq!(c.total_beats(), 3);
    }
}
