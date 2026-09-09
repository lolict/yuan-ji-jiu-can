// ══════════════════════════════════════════════════════════════════
// angle.rs — 角度子系统 (384° 满圆制式)
// ══════════════════════════════════════════════════════════════════
//
// 满全法不以 360° 为一圈,而是 384°。
// 选择 384 的原因:
//   384 = 64 × 6  → 每个 #0..#63 符号精确分到 6° 的整数倍 (格点)
//   384 = 8 × 48  → 与八流 (EightFlow) 同构
//   384 = 2^7 × 3 → 二进制友好 × 三元融合
//
// 本系统负责:
//   1. Symbol (#0..#63) ↔ 角度 (0..383) 的双向映射
//   2. 角距离计算 (环绕最短弧)
//   3. 角差方向 (顺时针逆时针)
//   4. 角的"象限归属" — 4 主宫 (Inner Space 视角)
//   5. 角对 $(θ_a, θ_b)$ 的"对齐/错位/重叠"判定 → 桥接 SMRL

use crate::topo::Symbol;

// ────────────────────────────────────────────────────────────
// 常量
// ────────────────────────────────────────────────────────────

pub const FULL_CIRCLE: i16 = 384;       // 满圆角值
pub const PER_SYMBOL:  i16 = 6;         // 384 / 64 = 6°

// 四主宫 (以 96° 为一象限)
pub const QUADRANT_NORTH: i16 = 0;     //     0..=95   (上 / 首脑 / 意)
pub const QUADRANT_EAST:  i16 = 96;    //    96..=191  (右 / 手 / 造)
pub const QUADRANT_SOUTH: i16 = 192;   //   192..=287  (下 / 足 / 行)
pub const QUADRANT_WEST:  i16 = 288;   //   288..=383  (左 / 基 / 根)

// ────────────────────────────────────────────────────────────
// 角度类型 — 满圆角格点 (0..383)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Angle(pub i16);   // 0..383 有效

impl Angle {
    /// 构造: 超过 383 自动回绕
    pub fn new(raw: i16) -> Self {
        Angle(((raw % FULL_CIRCLE) + FULL_CIRCLE) % FULL_CIRCLE)
    }

    /// Symbol → 角度 (每个符号占 6°, symbol #0 = 0°, #63 = 378°)
    pub fn from_symbol(s: Symbol) -> Self {
        let idx = s.as_u8() as i16;
        Angle::new(idx * PER_SYMBOL)
    }

    /// 角度 → 所属 Symbol (取最接近的 6° 格点)
    pub fn to_symbol(self) -> Option<Symbol> {
        let idx = (self.0 / PER_SYMBOL) as u8;
        Symbol::new(idx)
    }

    /// 加角 (回绕)
    pub fn add(self, delta: i16) -> Self {
        Angle::new(self.0 + delta)
    }

    /// 两个角之间的最短有向距离: 正=顺时针, 负=逆时针
    pub fn signed_distance_to(self, other: Angle) -> i16 {
        let mut d = other.0 - self.0;
        if d > FULL_CIRCLE / 2 { d -= FULL_CIRCLE; }
        if d < -FULL_CIRCLE / 2 { d += FULL_CIRCLE; }
        d
    }

    /// 绝对角距离 (无方向)
    pub fn arc_distance(self, other: Angle) -> i16 {
        let d = self.signed_distance_to(other).abs();
        d.min(FULL_CIRCLE - d)
    }

    /// 是否对齐 (角距离 ≤ 2°, 视为测不准容忍)
    pub fn aligned_with(self, other: Angle) -> bool {
        self.arc_distance(other) <= 2
    }

    /// 是否重叠 (角距离 = 0)
    pub fn overlaps(self, other: Angle) -> bool {
        self.0 == other.0
    }

    /// 是否基本对齐 (角距离在 ±PER_SYMBOL 内, 即同一个 Symbol 格点范围)
    pub fn roughly_aligned(self, other: Angle) -> bool {
        self.arc_distance(other) <= PER_SYMBOL
    }

    /// 所属四主宫
    pub fn quadrant(self) -> Quadrant {
        match self.0 {
            0..=95   => Quadrant::North,
            96..=191 => Quadrant::East,
            192..=287 => Quadrant::South,
            _        => Quadrant::West,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 四主宫
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quadrant {
    North,  // 上 / 首脑 / 意
    East,   // 右 / 手 / 造
    South,  // 下 / 足 / 行
    West,   // 左 / 基 / 根
}

// ────────────────────────────────────────────────────────────
// 角对判定 — 桥接:角度差异 → 三元推演 → SMRL
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AngleRelation {
    Overlap,        // 完全重叠 (0°)
    Aligned,        // 对齐 (角距离 ≤ TOL)
    Diagonal,       // 对角 (96° ± TOL)
    Opposite,       // 对冲 (192° ± TOL)
    Off,            // 以上都不是
}

impl AngleRelation {
    /// 从两个 Angle 判定角关系
    /// 在 384° 制式中:
    ///   · 96° 为对角  (Quadrant 切换点)
    ///   · 192° 为对冲 (半圆 = 384/2)
    pub fn between(a: Angle, b: Angle) -> Self {
        let d = a.arc_distance(b);
        const TOL: i16 = 6; // ±1 个 symbol 宽度的容差
        match d {
            0 => AngleRelation::Overlap,
            1..=TOL => AngleRelation::Aligned,
            90..=102 => AngleRelation::Diagonal,      // 96° ± 6
            186..=198 => AngleRelation::Opposite,     // 192° ± 6
            _ => AngleRelation::Off,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_0_is_0deg() {
        let a = Angle::from_symbol(Symbol::new(0).unwrap());
        assert_eq!(a.0, 0);
    }

    #[test]
    fn symbol_63_is_378deg() {
        let a = Angle::from_symbol(Symbol::new(63).unwrap());
        assert_eq!(a.0, 378);   // 63 × 6 = 378
    }

    #[test]
    fn angle_roundtrip_symbol() {
        for i in 0u8..64 {
            let s = Symbol::new(i).unwrap();
            let a = Angle::from_symbol(s);
            let back = a.to_symbol().unwrap();
            assert_eq!(back.as_u8(), i, "roundtrip 失败 @ {}", i);
        }
    }

    #[test]
    fn wrapped_angle() {
        let a = Angle::new(390);   // 390 - 384 = 6
        assert_eq!(a.0, 6);
        let b = Angle::new(-6);    // -6 + 384 = 378
        assert_eq!(b.0, 378);
    }

    #[test]
    fn arc_distance_symmetric() {
        let a = Angle::new(0);
        let b = Angle::new(10);
        assert_eq!(a.arc_distance(b), 10);
        assert_eq!(b.arc_distance(a), 10);
    }

    #[test]
    fn wraparound_shortest_arc() {
        // 374° 与 10° 之间的最短弧是 20° (回绕)，不是 364°
        let a = Angle::new(374);
        let b = Angle::new(10);
        assert_eq!(a.arc_distance(b), 20);
    }

    #[test]
    fn signed_distance_direction() {
        let a = Angle::new(0);
        let b = Angle::new(374);   // 374 = -10 (逆时针 10°)
        assert_eq!(a.signed_distance_to(b), -10);   // 负 = 逆时针更近
    }

    #[test]
    fn alignment_tolerance() {
        let a = Angle::new(0);
        assert!(a.aligned_with(Angle::new(0)));
        assert!(a.aligned_with(Angle::new(1)));
        assert!(a.aligned_with(Angle::new(2)));
        assert!(!a.aligned_with(Angle::new(3)));
    }

    #[test]
    fn quadrant_north() {
        let a = Angle::new(50);
        assert_eq!(a.quadrant(), Quadrant::North);
    }

    #[test]
    fn quadrant_east() {
        let a = Angle::new(100);
        assert_eq!(a.quadrant(), Quadrant::East);
    }

    #[test]
    fn opposite_detected() {
        let a = Angle::new(0);
        let b = Angle::new(192);   // 半圆 = 180° → 满圆的相对
        // 在 384° 制式中,对面的角是 192° (384/2) 而非 180°
        assert_eq!(a.arc_distance(b), 192);
        // 注意:180°(i16)在384制式里不对齐任何符号,180/6=30,所以 #30 = 180°
        let c = Angle::new(180);
        assert_eq!(c.to_symbol().unwrap().as_u8(), 30);
    }

    #[test]
    fn relation_overlap() {
        let a = Angle::new(0);
        let b = Angle::new(0);
        assert_eq!(AngleRelation::between(a, b), AngleRelation::Overlap);
    }
}
