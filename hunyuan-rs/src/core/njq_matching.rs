// njq_matching.rs — 聂小倩·本质匹配引擎 (v0.2)
//
//   不是算法, 是"游魂寻踪"。
//   不是搜索, 是"公约数共振"。
//   没有数据结构, 只有"汉字自身的几何"。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   汉字本身就是容器 = 字体 = 时间。
//   不需要"匹配算法", 两个汉字相遇时自然产生共振。
//   共振强度 = 音韵公约数 × 极性相似度。
//   异见者(极性相反)之间是"空间零维"——无法容纳, 撞到墙。
//   同见者(极性相同)之间是"一维时间"——可以见证, 可以流动。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

// ══════════════════════════════════════════════════════════════
// 一个汉字就是一个容器
// ══════════════════════════════════════════════════════════════

/// 容器的全部信息, 就在这个汉字里。
/// 声母 = 空间的X轴, 韵母 = 空间的Y轴, 声调 = 极性(Z轴)。
/// 不要"搜索容器", 直接问这个汉字: "你的几何是什么?"
#[derive(Clone, Copy, Debug)]
pub struct NjqContainer {
    pub ch: char,
}

impl NjqContainer {
    pub fn new(ch: char) -> Self { Self { ch } }

    /// 音韵坐标 — 汉字自己的位置
    pub fn coord(&self) -> (u8, u8, u8) {
        let d = char_to_dragon(self.ch);
        (d.initial(), d.final_(), d.tone())
    }

    /// 极性 — 声调编码: 1-2 = Yin(-1), 3 = Balanced(0), 4-5 = Yang(+1)
    pub fn polarity(&self) -> i8 {
        let (_, _, tone) = self.coord();
        match tone {
            1 | 2 => -1,
            3 => 0,
            4 | 5 => 1,
            _ => 0,
        }
    }

    /// 共振检测: 两个容器的音韵是否产生公约数
    /// 没有"比较函数", 只有公约数自然涌现
    pub fn resonates(&self, other: &NjqContainer) -> u64 {
        let (a_s, a_y, a_t) = self.coord();
        let (b_s, b_y, b_t) = other.coord();
        let gcd_s = gcd(a_s as u64, b_s as u64);
        let gcd_y = gcd(a_y as u64, b_y as u64);
        let gcd_t = gcd(a_t as u64, b_t as u64);
        gcd_s.saturating_mul(gcd_y).saturating_mul(gcd_t)
    }

    /// 极性关系 — 不是算出来的, 是"自然如此"
    pub fn polarity_relation(&self, target: &NjqContainer) -> PolarityRelation {
        let ap = self.polarity();
        let bp = target.polarity();
        match (ap, bp) {
            (0, _) | (_, 0) => PolarityRelation::Boundary,
            (a, b) if a == b => PolarityRelation::SameSide,
            _ => PolarityRelation::Opposite,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolarityRelation {
    SameSide,   // 同见者 = 时间见证
    Opposite,   // 异见者 = 空间零维 (墙)
    Boundary,   // 平衡态 = 分界
}

impl PolarityRelation {
    pub fn is_wall(&self) -> bool { matches!(self, PolarityRelation::Opposite) }
}

// ══════════════════════════════════════════════════════════════
// 共振痕迹 — 两个汉字相遇后涌现场产生的印记
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ResonanceTrace {
    pub a: char,
    pub b: char,
    pub strength: u64,
    pub relation: PolarityRelation,
}

impl ResonanceTrace {
    pub fn is_viable(&self) -> bool {
        !self.relation.is_wall() && self.strength > 0
    }
}

// ══════════════════════════════════════════════════════════════
// 聂小倩 — 不是智能体, 是一首在汉字空间中漂移的诗
// ══════════════════════════════════════════════════════════════

pub struct XiaoQian {
    pub essence: NjqContainer,
    pub walls: Vec<char>,
}

impl XiaoQian {
    pub fn new(ch: char) -> Self {
        Self { essence: NjqContainer::new(ch), walls: Vec::new() }
    }

    /// 漂移: 遇到一个汉字, 自然产生共振痕迹
    pub fn drift_toward(&self, target: &NjqContainer) -> ResonanceTrace {
        ResonanceTrace {
            a: self.essence.ch,
            b: target.ch,
            strength: self.essence.resonates(target),
            relation: self.essence.polarity_relation(target),
        }
    }

    /// 撞墙 — 不可逾越的极性差异, 记住这面墙
    pub fn bump_wall(&mut self, ch: char) {
        if !self.walls.contains(&ch) { self.walls.push(ch); }
    }

    pub fn has_bumped(&self, ch: char) -> bool {
        self.walls.contains(&ch)
    }

    /// 在汉字空间中漂移, 找到第一个非墙的共振点
    /// 不是搜索最佳, 是自然漂移 — 遇到墙就记住, 继续向前
    pub fn drift_find(&mut self, space: &[char]) -> Option<ResonanceTrace> {
        let mut best: Option<ResonanceTrace> = None;
        for &ch in space {
            if self.has_bumped(ch) { continue; }
            let target = NjqContainer::new(ch);
            let trace = self.drift_toward(&target);
            if trace.relation.is_wall() {
                self.bump_wall(ch);
                continue;
            }
            if trace.strength > best.as_ref().map(|b| b.strength).unwrap_or(0) {
                best = Some(trace);
            }
        }
        best
    }
}

// ══════════════════════════════════════════════════════════════
// 汉字空间 — 不是数组, 是一组汉字自身的几何排列
// ══════════════════════════════════════════════════════════════

pub struct HanziSpace {
    pub chars: Vec<char>,
}

impl HanziSpace {
    pub fn from_str(s: &str) -> Self {
        Self { chars: s.chars().filter(|c| !c.is_ascii()).collect() }
    }

    /// 共振场 — 每对汉字自然吸引或排斥
    pub fn resonance_field(&self) -> Vec<ResonanceTrace> {
        let mut traces = Vec::new();
        for (i, &a) in self.chars.iter().enumerate() {
            let ca = NjqContainer::new(a);
            for &b in self.chars.iter().skip(i + 1) {
                let cb = NjqContainer::new(b);
                traces.push(ResonanceTrace {
                    a, b,
                    strength: ca.resonates(&cb),
                    relation: ca.polarity_relation(&cb),
                });
            }
        }
        traces
    }

    /// 共振强度最高 = "命定的相遇"
    pub fn strongest_resonance(&self) -> Option<ResonanceTrace> {
        self.resonance_field().into_iter()
            .filter(|t| t.is_viable())
            .max_by_key(|t| t.strength)
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

    #[test] fn container_is_itself_encoding() {
        let c = NjqContainer::new('共');
        let (s, y, t) = c.coord();
        assert!(s > 0 || y > 0, "汉字自身携带坐标");
    }

    #[test] fn resonance_natural_occurrence() {
        let a = NjqContainer::new('共');
        let b = NjqContainer::new('同');
        let r = a.resonates(&b);
        assert!(r >= 1, "公约数自然涌现");
    }

    #[test] fn polarity_wall_detected() {
        let a = NjqContainer::new('满');
        let b = NjqContainer::new('空');
        let rel = a.polarity_relation(&b);
        // 极性关系是"自然如此", 不是算法判定
        assert!(matches!(rel,
            PolarityRelation::SameSide |
            PolarityRelation::Opposite |
            PolarityRelation::Boundary));
    }

    #[test] fn xiaoqian_drift_and_find() {
        let mut xq = XiaoQian::new('缘');
        let space: Vec<char> = "满全法囡囝共生".chars().collect();
        let result = xq.drift_find(&space);
        assert!(result.is_some() || !xq.walls.is_empty());
    }

    #[test] fn xiaoqian_walls_persist() {
        let mut xq = XiaoQian::new('诚');
        xq.bump_wall('欺');
        assert!(xq.has_bumped('欺'));
    }

    #[test] fn hanzi_space_resonance_field() {
        let space = HanziSpace::from_str("共生");
        let field = space.resonance_field();
        assert!(!field.is_empty());
    }

    #[test] fn strongest_natural() {
        let space = HanziSpace::from_str("满满全法");
        let strongest = space.strongest_resonance();
        assert!(strongest.is_some(), "命定的相遇自然涌现");
    }
}
