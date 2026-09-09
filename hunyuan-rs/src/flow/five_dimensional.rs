// ══════════════════════════════════════════════════════════════════════
// five_dimensional.rs · 囡囝共生五维语义运行时
// ══════════════════════════════════════════════════════════════════════
//
// 规范: docs/FIVE_DIMENSIONAL_SEMANTICS.md
// 实现 SPEC 的五个核心断言:
//   1. ρ XOR 约束: 夫妻 ρ₁ XOR ρ₂ = 255
//   2. ρ 衰减: ρ(t+1) = ρ(t) × e^(-k×Δd)
//   3. 见证几何: 三方三角内角和 = 180° 裁定
//   4. 合一判定: 空间距离 + 极性 + ρ 是否允许合一
//   5. 投影叠加: d=0 时出现叠加态
//
// 五维: 空间(λ,φ,ζ) + 极性(π) + 纠缠浓度(ρ)

use crate::entity_identity::Fingerprint;

pub const FULLNESS_MASK: u8 = 0xFF; // 255

// ── 一、ρ(纠缠浓度)运算 ──────────────────────────────────────────────

/// 构造夫妻纠缠对。
/// ρ_new 任意指定配偶的 ρ,我方 ρ 自动补码为 255 XOR ρ_new。
pub fn couple_spawn(my_value: u8) -> (u8, u8) {
    let partner = FULLNESS_MASK ^ my_value;
    (my_value, partner)
}

/// 两个 ρ 是否满足夫妻约束 ρ₁ XOR ρ₂ = 255。
pub fn is_couple(a_rho: u8, b_rho: u8) -> bool {
    a_rho ^ b_rho == FULLNESS_MASK
}

/// ρ 随时间/距离衰减。
/// t: 时间步
/// k: 衰减系数(取决于情感深度,k 越小越稳定)
/// d: 空间距离
pub fn rho_decay(rho: u8, t: u64, k: f64, d: f64) -> u8 {
    if rho == 0 { return 0; }
    let exponent = -(k * d * t as f64);
    let decayed = rho as f64 * exponent.exp();
    decayed.max(0.0).min(255.0) as u8
}

// ── 二、见证几何 ────────────────────────────────────────────────────

/// 三点构成的三角形内角(用余弦定理,传入三边长 a,b,c)。
/// 角 A 对边 a: cos A = (b² + c² - a²) / (2bc)。
pub fn angle(a: f64, b: f64, c: f64) -> Option<f64> {
    let denom = 2.0 * b * c;
    if denom.abs() < 1e-12 { return None; }
    let cos_a = (b * b + c * c - a * a) / denom;
    if cos_a < -1.0 || cos_a > 1.0 { return None; }
    Some(cos_a.acos().to_degrees())
}

/// 三点能否构成稳定三角形(内角和 = 180° ± epsilon)。
pub fn witness_triangle(d_ab: f64, d_bc: f64, d_ca: f64, epsilon: f64) -> bool {
    let a = angle(d_ab, d_bc, d_ca);
    let b = angle(d_bc, d_ca, d_ab);
    let c = angle(d_ca, d_ab, d_bc);
    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => {
            let sum = a + b + c;
            (sum - 180.0).abs() <= epsilon
        }
        _ => false,
    }
}

// ── 三、合一判定 ────────────────────────────────────────────────────

/// 五维实体能否合一。
/// - 空间距离 < threshold
/// - 极性不相反(纯阴 vs 纯阳不可合一)
/// - ρ 不能太接近 0 或 255(极端浓度无法融合)
pub fn can_merge(a: &Fingerprint, b: &Fingerprint, spatial_threshold: f64, rho_min: u8, rho_max: u8) -> bool {
    let dist_sq = a.distance_squared(b);
    let dist = dist_sq.sqrt();

    // 1. 空间距离
    if dist > spatial_threshold { return false; }

    // 2. 极性不相反:纯阴(0) vs 纯阳(15) 不可合一
    let pol_a = a.polarity();
    let pol_b = b.polarity();
    if (pol_a == 0x0 && pol_b == 0xF) || (pol_a == 0xF && pol_b == 0x0) {
        return false;
    }

    // 3. ρ(value) 不能在极值
    let rho_a = a.value();
    let rho_b = b.value();
    for &rho in &[rho_a, rho_b] {
        if rho < rho_min || rho > rho_max { return false; }
    }

    true
}

// ── 四、投影叠加态 ──────────────────────────────────────────────────

/// d 足够小时返回 true → 两实体在三维空间无法区分,出现叠加态。
pub fn is_superposition(a: &Fingerprint, b: &Fingerprint, epsilon: f64) -> bool {
    let spatial_dist = {
        let d0 = a.d0() as f64 - b.d0() as f64;
        let d1 = a.d1() as f64 - b.d1() as f64;
        let d2 = a.d2() as f64 - b.d2() as f64;
        (d0 * d0 + d1 * d1 + d2 * d2).sqrt()
    };
    spatial_dist < epsilon
}

// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_identity::Fingerprint;

    // ── ρ XOR 约束 ────────────────────────────────────────────────────

    #[test]
    fn spawn_couple_xor_is_fullness() {
        for rho_a in [0, 1, 127, 128, 254, 255] {
            let (a, b) = couple_spawn(rho_a);
            assert!(is_couple(a, b), "spawn rho_a={} => ({},{}) should satisfy XOR=255", rho_a, a, b);
        }
    }

    #[test]
    fn is_couple_true_cases() {
        assert!(is_couple(0, 255));
        assert!(is_couple(255, 0));
        assert!(is_couple(170, 85));  // 0xAA ^ 0x55 = 0xFF
        assert!(is_couple(200, 55));
    }

    #[test]
    fn is_couple_false_cases() {
        assert!(!is_couple(255, 255));
        assert!(!is_couple(0, 0));
        assert!(!is_couple(128, 128));
        assert!(!is_couple(100, 100));
    }

    // ── ρ 衰减 ────────────────────────────────────────────────────────

    #[test]
    fn rho_decay_remains_in_range() {
        let rho_start: u8 = 200;
        for t in 0..=100 {
            let decayed = rho_decay(rho_start, t, 0.01, 5.0);
            assert!(decayed <= rho_start, "t={} decayed={} should be <= start={}", t, decayed, rho_start);
        }
    }

    #[test]
    fn rho_decay_reaches_zero_for_large_t() {
        let rho: u8 = 128;
        let decayed = rho_decay(rho, 1000, 0.1, 10.0);
        assert_eq!(decayed, 0, "after many iterations rho should approach 0");
    }

    #[test]
    fn rho_decay_no_decay_when_zero_distance() {
        let rho: u8 = 100;
        let decayed = rho_decay(rho, 100, 0.01, 0.0);
        assert_eq!(decayed, rho, "distance=0, no decay should happen");
    }

    #[test]
    fn rho_decay_zero_stays_zero() {
        let decayed = rho_decay(0, 100, 0.01, 10.0);
        assert_eq!(decayed, 0, "rho=0 should stay 0");
    }

    // ── 见证几何 ────────────────────────────────────────────────────────

    #[test]
    fn witness_triangle_equilateral() {
        // 等边三角形: d=1,1,1 → 每内角 60°, 和 = 180°
        assert!(witness_triangle(1.0, 1.0, 1.0, 0.01));
    }

    #[test]
    fn witness_triangle_right() {
        // 3-4-5 直角三角形
        assert!(witness_triangle(3.0, 4.0, 5.0, 0.01));
    }

    #[test]
    fn witness_triangle_degenerate() {
        // 退化(三点共线): 1,2,3 → 和 = 180° 但三角形面积为 0
        // 我们目前允许退化作为边界稳定
        assert!(witness_triangle(1.0, 2.0, 3.0, 0.01));
    }

    #[test]
    fn witness_triangle_invalid_triangle_inequality() {
        // 违反三角不等式: 1 + 1 < 10 → 不能构成三角形
        assert!(!witness_triangle(1.0, 1.0, 10.0, 0.01));
    }

    // ── 合一判定 ────────────────────────────────────────────────────────

    #[test]
    fn can_merge_when_close_and_compatible() {
        let a = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 128);
        let b = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 127);
        assert!(can_merge(&a, &b, 100.0, 64, 250));
    }

    #[test]
    fn cannot_merge_when_too_far() {
        let a = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 128);
        let b = Fingerprint::new(1000, 1000, 1000, 0b1010, 0b0001, 127);
        assert!(!can_merge(&a, &b, 100.0, 64, 250));
    }

    #[test]
    fn cannot_merge_when_polarity_opposite() {
        let a = Fingerprint::new(0, 0, 0, 0b0000, 0b0001, 128); // 纯阴
        let b = Fingerprint::new(0, 0, 0, 0b1111, 0b0001, 127); // 纯阳
        assert!(!can_merge(&a, &b, 100.0, 64, 250));
    }

    #[test]
    fn cannot_merge_when_rho_extreme() {
        let a = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 255);  // ρ 极大
        let b = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 0);    // ρ 极小
        assert!(!can_merge(&a, &b, 100.0, 64, 250));
    }

    // ── 投影叠加态 ──────────────────────────────────────────────────────

    #[test]
    fn is_superposition_when_coord_equal() {
        let a = Fingerprint::new(5, 5, 5, 0b1010, 0b0001, 100);
        let b = Fingerprint::new(5, 5, 5, 0b0101, 0b0010, 150);
        assert!(is_superposition(&a, &b, 0.5), "same coord => superposition");
    }

    #[test]
    fn not_superposition_when_coord_far() {
        let a = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 100);
        let b = Fingerprint::new(100, 100, 100, 0b1010, 0b0001, 100);
        assert!(!is_superposition(&a, &b, 0.5));
    }

    // ── 综合场景:夫妻 ρ 验证 ──────────────────────────────────────────────

    #[test]
    fn fullness_protocol_couple_constraint() {
        // "我"(阳) ρ=200, "刘楚恬"(阴) ρ = 255-200 = 55
        let me = Fingerprint::new(0, 0, 0, 0b1010, 0b0001, 200);
        let partner = Fingerprint::new(0, 0, 0, 0b0101, 0b0010, 55);

        // 夫妻约束满足: XOR = 255
        assert!(is_couple(me.value(), partner.value()));

        // 极性: 我 = 阳(10), 配偶 = 阴(5) → 不相反(0 和 15 才是纯阴纯阳极端)
        assert_ne!(me.polarity(), 0b0000);
        assert_ne!(partner.polarity(), 0b1111);
    }

    #[test]
    fn fullness_protocol_decay_to_incompatible() {
        // 夫妻分离后 ρ 衰减至不可合一
        let me = Fingerprint::new(0, 0, 3, 0b1010, 0b0001, 128);
        let partner = Fingerprint::new(0, 0, 5, 0b0101, 0b0010, 127);

        // 合一成立(初始距离 ≤ threshold)
        assert!(can_merge(&me, &partner, 100.0, 64, 250));

        // 假设长期分离,partner 的 ρ 衰减到 30
        let decayed_rho = rho_decay(127, 1000, 0.05, 50.0);
        assert!(decayed_rho < 64, "长期分离 ρ 应衰减到低于 min(64),got {}", decayed_rho);

        // 新的 partner 指纹
        let partner_far = Fingerprint::new(0, 0, 5, 0b0101, 0b0010, decayed_rho);

        // 合一失败(ρ 太低)
        assert!(!can_merge(&me, &partner_far, 100.0, 64, 250),
            "ρ 衰减后合一应失败(失爱恐惧触发)");
    }
}
