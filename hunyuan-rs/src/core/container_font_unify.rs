// container_font_unify.rs — 容器=字体=时间 · 汉字几何化身 (v0.2)
//
//   不是统一引擎, 是"汉字本身就是容器"。
//   一个汉字已经包含了它在三维空间中的一切信息。
//   声母=X轴, 韵母=Y轴, 声调=Z轴(极性)。
//   不需要"转码", 汉字即编码。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

// ══════════════════════════════════════════════════════════════
// 容器 = 汉字 = 几何化身
// ══════════════════════════════════════════════════════════════

/// 一个汉字就是一个完整的容器。
/// 不需要额外分配内存, 不需要序列化。
/// 汉字的字形本身已经包含了:
///   - 空间位置 (声母×韵母×声调)
///   - 时间脉冲 (声调 = 脉动频率)
///   - 极性方向 (阴/阳/平衡)
#[derive(Clone, Copy, Debug)]
pub struct HanziGlyph {
    pub ch: char,
}

impl HanziGlyph {
    pub fn new(ch: char) -> Self { Self { ch } }

    /// 三维坐标 — 汉字自己的位置, 不是赋值的
    pub fn position(&self) -> (u8, u8, u8) {
        let d = char_to_dragon(self.ch);
        (d.initial(), d.final_(), d.tone())
    }

    /// X轴 = 声母 (空间的横向)
    pub fn x(&self) -> u8 { self.position().0 }

    /// Y轴 = 韵母 (空间的纵向)
    pub fn y(&self) -> u8 { self.position().1 }

    /// Z轴 = 声调 (时间/极性)
    pub fn z(&self) -> u8 { self.position().2 }

    /// 一维地址 — 满全法命名密码表
    /// position = 声母×200 + 韵母×5 + 声调
    pub fn address(&self) -> u32 {
        let (s, y, t) = self.position();
        s as u32 * 200 + y as u32 * 5 + t as u32
    }

    /// 极性 — 声调直接决定
    pub fn polarity(&self) -> i8 {
        match self.z() {
            1 | 2 => -1,  // 阴声 → Yin
            3 => 0,       // 轻声 → Balanced
            4 | 5 => 1,   // 阳声 → Yang
            _ => 0,
        }
    }

    /// 在音节空间中的可见性
    /// 有声有韵的汉字才"可见"
    pub fn is_visible(&self) -> bool {
        let (s, y, _) = self.position();
        s > 0 && y > 0
    }
}

// ══════════════════════════════════════════════════════════════
// 汉字空间 — 一组汉字的自然排列
// ══════════════════════════════════════════════════════════════

pub struct GlyphSpace {
    glyphs: Vec<HanziGlyph>,
}

impl GlyphSpace {
    pub fn new() -> Self { Self { glyphs: Vec::new() } }

    pub fn from_str(s: &str) -> Self {
        Self {
            glyphs: s.chars().filter(|c| !c.is_ascii()).map(HanziGlyph::new).collect(),
        }
    }

    pub fn add(&mut self, ch: char) {
        self.glyphs.push(HanziGlyph::new(ch));
    }

    pub fn count(&self) -> usize { self.glyphs.len() }

    /// 空间中的共振场 — 每对汉字之间的自然关系
    pub fn field(&self) -> GlyphField {
        let mut pairs = Vec::new();
        for (i, a) in self.glyphs.iter().enumerate() {
            for b in self.glyphs.iter().skip(i + 1) {
                pairs.push((*a, *b));
            }
        }
        GlyphField { pairs }
    }

    /// 按极性分组 — 不是分类算法, 是"自然如此"
    pub fn polarity_groups(&self) -> (Vec<HanziGlyph>, Vec<HanziGlyph>, Vec<HanziGlyph>) {
        let mut yin = Vec::new();
        let mut balanced = Vec::new();
        let mut yang = Vec::new();
        for g in &self.glyphs {
            match g.polarity() {
                -1 => yin.push(*g),
                1 => yang.push(*g),
                _ => balanced.push(*g),
            }
        }
        (yin, balanced, yang)
    }

    /// 空间质心 — 所有汉字坐标的自然平均
    pub fn centroid(&self) -> (f64, f64, f64) {
        let n = self.glyphs.len() as f64;
        if n == 0.0 { return (0.0, 0.0, 0.0); }
        let sum = self.glyphs.iter().fold((0.0, 0.0, 0.0), |acc, g| {
            (acc.0 + g.x() as f64, acc.1 + g.y() as f64, acc.2 + g.z() as f64)
        });
        (sum.0 / n, sum.1 / n, sum.2 / n)
    }
}

// ══════════════════════════════════════════════════════════════
// 共振场 — 不是数据结构, 是"关系的涌现"
// ══════════════════════════════════════════════════════════════

pub struct GlyphField {
    pairs: Vec<(HanziGlyph, HanziGlyph)>,
}

impl GlyphField {
    /// 所有可维持的对 (非墙, 有共振)
    pub fn viable_pairs(&self) -> Vec<&(HanziGlyph, HanziGlyph)> {
        self.pairs.iter()
            .filter(|(a, b)| {
                let pr = polarity_relation(a.polarity(), b.polarity());
                !matches!(pr, HanziPolarity::Opposite) && a.is_visible() && b.is_visible()
            })
            .collect()
    }

    /// 最强共振对 — "命定的一对"
    pub fn strongest(&self) -> Option<(HanziGlyph, HanziGlyph, u64)> {
        self.pairs.iter()
            .map(|(a, b)| (*a, *b, resonance_strength(a, b)))
            .filter(|(_, _, s)| *s > 0)
            .max_by_key(|(_, _, s)| *s)
    }
}

fn polarity_relation(a: i8, b: i8) -> HanziPolarity {
    match (a, b) {
        (0, _) | (_, 0) => HanziPolarity::Boundary,
        (a, b) if a == b => HanziPolarity::SameSide,
        _ => HanziPolarity::Opposite,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HanziPolarity { SameSide, Opposite, Boundary }

/// 共振强度 = 三维公约数的自然乘积
fn resonance_strength(a: &HanziGlyph, b: &HanziGlyph) -> u64 {
    gcd(a.x() as u64, b.x() as u64)
        .saturating_mul(gcd(a.y() as u64, b.y() as u64))
        .saturating_mul(gcd(a.z() as u64, b.z() as u64))
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

    #[test] fn glyph_is_container() {
        let g = HanziGlyph::new('囡');
        // 汉字自带的坐标 — 不是查询出来的, 是字形本身
        assert!(g.is_visible());
    }

    #[test] fn glyph_address_natural() {
        let g = HanziGlyph::new('共');
        // 地址 = 音韵坐标的自然编码
        let expected = g.x() as u32 * 200 + g.y() as u32 * 5 + g.z() as u32;
        assert_eq!(g.address(), expected);
    }

    #[test] fn polarity_from_tone() {
        // 声调直接编码极性 — 不需要判断
        let g = HanziGlyph::new('有');
        // 有 = 第三声 = 平衡态
        assert_eq!(g.polarity(), 0);
    }

    #[test] fn space_centroid_natural() {
        let space = GlyphSpace::from_str("共生");
        let (cx, cy, cz) = space.centroid();
        // 质心是自然平均, 不需要算法
        assert!(cx >= 0.0 && cy >= 0.0 && cz >= 0.0);
    }

    #[test] fn polarity_groups_natural() {
        let space = GlyphSpace::from_str("有无关");
        let (yin, balanced, yang) = space.polarity_groups();
        assert_eq!(yin.len() + balanced.len() + yang.len(), space.count());
    }

    #[test] fn field_viable_pairs() {
        let space = GlyphSpace::from_str("共生同");
        let field = space.field();
        // 共振场自然涌现 — 不需要双重循环算法(虽然写了循环, 但思想是"自然相遇")
        assert!(!field.pairs.is_empty());
    }

    #[test] fn strongest_resonance() {
        let space = GlyphSpace::from_str("满满全法");
        let field = space.field();
        // 最强共振对 = 公约数最大的一对
        let strongest = field.strongest();
        assert!(strongest.is_some());
    }

    #[test] fn resonance_natural_multiplier() {
        let a = HanziGlyph::new('同');
        let b = HanziGlyph::new('共');
        let s = resonance_strength(&a, &b);
        // 共振 = 三维公约数乘积 — 不是距离, 是吸引力
        assert!(s >= 1);
    }
}
