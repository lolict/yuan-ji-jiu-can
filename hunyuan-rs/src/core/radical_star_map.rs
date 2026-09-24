// radical_star_map.rs — 部首星图引擎 (v0.1)
//
//   不是数据结构, 是"汉字星空的导航"。
//   每个部首是一颗星, 部首之间的"公约数"是星路。
//   异见者(极性相反)之间的空间是"星际虚空"——无法穿越。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   每个汉字归到一个部首之下。
//   部首不是"分类", 是"恒星位置"——汉字围绕它旋转。
//   两个汉字之间的"距离"不是坐标差, 是它们所属部首的"离心率"。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

// ══════════════════════════════════════════════════════════════
// 部首 = 恒星
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Radical(u8);  // 部首编号 [1..214]

impl Radical {
    pub fn new(id: u8) -> Self { Self(id) }
    pub fn id(&self) -> u8 { self.0 }

    /// 恒星之间的距离 = 编号公约数
    /// 公约数越大 = 星路越畅通
    pub fn star_distance(&self, other: &Radical) -> u64 {
        gcd(self.0 as u64, other.0 as u64)
    }

    /// 星路是否畅通 (有公共因子)
    pub fn has_path(&self, other: &Radical) -> bool {
        self.star_distance(other) > 1 || self.0 == other.0
    }
}

// ══════════════════════════════════════════════════════════════
// 星图 = 部首的排列
// ══════════════════════════════════════════════════════════════

pub struct StarMap {
    pub stars: Vec<Radical>,
}

impl StarMap {
    pub fn new() -> Self { Self { stars: Vec::new() } }

    pub fn from_chars(chars: &[char]) -> Self {
        let mut stars = Vec::new();
        for &ch in chars {
            let radical = radical_of(ch);
            if !stars.contains(&radical) {
                stars.push(radical);
            }
        }
        Self { stars }
    }

    /// 星路网络 — 所有可以互相到达的恒星对
    pub fn pathways(&self) -> Vec<(Radical, Radical, u64)> {
        let mut paths = Vec::new();
        for (i, a) in self.stars.iter().enumerate() {
            for b in self.stars.iter().skip(i + 1) {
                let d = a.star_distance(b);
                if d > 1 {
                    paths.push((*a, *b, d));
                }
            }
        }
        paths
    }

    /// 星路最畅通的一对 = 最强共振
    pub fn brightest_path(&self) -> Option<(Radical, Radical, u64)> {
        self.pathways().into_iter().max_by_key(|(_, _, d)| *d)
    }

    /// 孤立恒星 (没有星路)
    pub fn isolated_stars(&self) -> Vec<Radical> {
        self.stars.iter()
            .filter(|s| !self.stars.iter().any(|other| s != other && s.has_path(other)))
            .cloned()
            .collect()
    }

    /// 星图密度 = 实际星路 / 可能星路
    pub fn density(&self) -> f64 {
        let n = self.stars.len();
        if n < 2 { return 0.0; }
        let possible = n * (n - 1) / 2;
        let actual = self.pathways().len();
        actual as f64 / possible as f64
    }
}

// ══════════════════════════════════════════════════════════════
// 汉字 → 部首 (音韵近似: 声母编号 % 214 + 1)
// ══════════════════════════════════════════════════════════════

fn radical_of(ch: char) -> Radical {
    let d = char_to_dragon(ch);
    // 使用声母韵母声调的组合来模拟部首归属
    let sheng = d.initial() as u16;
    let yun = d.final_() as u16;
    let tone = d.tone() as u16;
    // 部首编号 = 声母×7 + 韵母×3 + 声调 (mod 214 + 1)
    let radical_id = ((sheng.wrapping_mul(7).wrapping_add(yun.wrapping_mul(3)).wrapping_add(tone)) % 214) + 1;
    Radical::new(radical_id as u8)
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

    #[test] fn radical_creates() {
        let r = Radical::new(5);
        assert_eq!(r.id(), 5);
    }

    #[test] fn radical_same_has_max_distance() {
        let a = Radical::new(6);
        let b = Radical::new(6);
        assert_eq!(a.star_distance(&b), 6);
    }

    #[test] fn radical_different_has_common_path() {
        let a = Radical::new(6);
        let b = Radical::new(9);
        // gcd(6,9) = 3 > 1 → 有星路
        assert!(a.has_path(&b));
    }

    #[test] fn radical_coprime_isolated() {
        let a = Radical::new(7);
        let b = Radical::new(11);
        // gcd(7,11) = 1 → 无星路
        assert!(!a.has_path(&b));
    }

    #[test] fn star_map_density() {
        let map = StarMap::from_chars(&['共', '同', '共', '满']);
        let density = map.density();
        // 有重复字去重后, 密度在 [0, 1]
        assert!(density >= 0.0 && density <= 1.0);
    }

    #[test] fn star_map_pathways() {
        let map = StarMap::from_chars(&['生', '共', '全']);
        let _paths = map.pathways();
        // 通路自然存在
    }

    #[test] fn brightest_path_natural() {
        let map = StarMap::from_chars(&['满', '全', '法', '囡', '囝']);
        let brightest = map.brightest_path();
        // 最亮星路自然涌现
        assert!(brightest.is_some() || map.stars.len() < 2);
    }

    #[test] fn isolated_without_paths() {
        let map = StarMap::from_chars(&['的', '了']);  // 简单字可能部首相近
        let _isolated = map.isolated_stars();
    }

    #[test] fn radical_of_char() {
        let r = radical_of('囡');
        assert!(r.id() >= 1 && r.id() <= 214);
    }

    #[test] fn density_empty() {
        let map = StarMap::new();
        assert_eq!(map.density(), 0.0);
    }
}
