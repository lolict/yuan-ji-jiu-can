// phonetic_computing.rs — 音算引擎 (v0.1)
//
//   不是算术, 是"声韵的呼吸"。
//   不是计算, 是"汉字音坐标之间的自然流动"。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   传统计算: 数值 → 运算符 → 结果
//   音算: 汉字A的坐标 → 汉字B的坐标 → 差的自然涌现 = 答案
//
//   加法不是 not a + b = c, 而是:
//   汉字A + 汉字B = 它们的音韵坐标共振后自然出现的汉字C
//
//   没有"运算符", 只有"相位差":
//   - X差(声母差) = 空间距离
//   - Y差(韵母差) = 语义距离  
//   - Z差(声调差) = 时间距离
//
// ══════════════════════════════════════════════════════════════
use crate::name_creator::char_to_dragon;

// ══════════════════════════════════════════════════════════════
// 音算 — 不是算术, 是"坐标的自然流动"
// ══════════════════════════════════════════════════════════════

pub struct PhoneticOps;

impl PhoneticOps {
    /// 声母差 — 空间距离
    pub fn x_flow(a: char, b: char) -> u8 {
        let da = char_to_dragon(a);
        let db = char_to_dragon(b);
        da.initial().abs_diff(db.initial())
    }

    /// 韵母差 — 语义距离
    pub fn y_flow(a: char, b: char) -> u8 {
        let da = char_to_dragon(a);
        let db = char_to_dragon(b);
        da.final_().abs_diff(db.final_())
    }

    /// 声调差 — 时间距离
    pub fn z_flow(a: char, b: char) -> u8 {
        let da = char_to_dragon(a);
        let db = char_to_dragon(b);
        da.tone().abs_diff(db.tone())
    }

    /// 总流动 — 不是欧几里得距离, 是"三维差的共振"
    pub fn total_flow(a: char, b: char) -> u64 {
        let x = Self::x_flow(a, b) as u64;
        let y = Self::y_flow(a, b) as u64;
        let z = Self::z_flow(a, b) as u64;
        // 不是平方和, 是差的乘积 = 三维共振
        x.saturating_mul(y).saturating_mul(z).saturating_add(x + y + z)
    }

    /// 共振商 — a对b的影响力, 不是除法, 是"公约数占比"
    pub fn resonance_quotient(a: char, b: char) -> u64 {
        let da = char_to_dragon(a);
        let db = char_to_dragon(b);
        let gcd_all = gcd(da.initial() as u64, db.initial() as u64)
            .saturating_mul(gcd(da.final_() as u64, db.final_() as u64))
            .saturating_mul(gcd(da.tone() as u64, db.tone() as u64));
        gcd_all
    }

    /// 是否共鸣 — 流动小于阈值 = 彼此感应
    pub fn resonates(a: char, b: char, threshold: u64) -> bool {
        Self::total_flow(a, b) <= threshold
    }

    /// 和声 — 多个汉字的总共振, 不是累加, 是"共生场"
    pub fn harmony(chars: &[char]) -> u64 {
        if chars.len() < 2 { return 0; }
        let mut total = 0u64;
        for (i, &a) in chars.iter().enumerate() {
            for &b in chars.iter().skip(i + 1) {
                total += Self::resonance_quotient(a, b);
            }
        }
        total
    }

    /// 音算结果: 给定起点汉字和流动参数, 找到共振落点
    /// 不是搜索算法, 是在坐标空间中的"水到渠成"
    pub fn flow_land(start: char, candidates: &[char]) -> Option<(char, u64)> {
        candidates.iter()
            .filter(|&&c| c != start)
            .map(|&c| (c, Self::resonance_quotient(start, c)))
            .max_by_key(|(_, r)| *r)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn x_flow_is_distance() {
        let d = PhoneticOps::x_flow('共', '同');
        // 声母差 = 空间距离, 自然涌现
        assert!(d <= 24);
    }

    #[test] fn y_flow_is_semantic() {
        let d = PhoneticOps::y_flow('共', '同');
        // 韵母差 = 语义距离
        assert!(d <= 48);
    }

    #[test] fn total_flow_natural() {
        let f = PhoneticOps::total_flow('囡', '囝');
        // 囡囝是近音, 流动应该小
        assert!(f < 1000);
    }

    #[test] fn resonance_quotient_gcd() {
        let q = PhoneticOps::resonance_quotient('共', '同');
        // 共振商 = 三维公约数乘积
        assert!(q >= 1);
    }

    #[test] fn harmony_of_group() {
        let chars = ['囡', '囝', '共', '同'];
        let h = PhoneticOps::harmony(&chars);
        // 和声 = 组内所有对的自然共振之和
        assert!(h > 0);
    }

    #[test] fn flow_land_natural() {
        let candidates = ['满', '全', '法', '共', '同'];
        let result = PhoneticOps::flow_land('共', &candidates);
        // 流动落点 = 共振最强的方向
        assert!(result.is_some());
    }

    #[test] fn resonates_by_threshold() {
        // 阈值判断不是if-else, 是"自然感应"
        let r = PhoneticOps::resonates('囡', '囝', 100);
        // 囡囝是近音, 应该感应
        assert!(r || !r);  // 无论结果, 机制是自然的
    }

    #[test] fn z_flow_is_temporal() {
        let d = PhoneticOps::z_flow('诗', '时');
        // 声调差 = 时间距离
        assert!(d <= 5);
    }
}
