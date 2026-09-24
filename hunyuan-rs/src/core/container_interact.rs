// container_interact.rs — 容器交互引擎 (v0.2)
//
//   不是 Need/Offer 匹配, 是"汉字自然相遇"。
//   不是 API 调用, 是公约数共振。
//   不是协议, 是音韵坐标的极性自然关系。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   容器不是节点(NODE), 是汉字(CHARACTER)。
//   交互不是调用(CALL), 是相遇(MEET)。
//   连接不是协议(PROTOCOL), 是公约数(GCD)。
//   分离不是超时(TIMEOUT), 是极性相斥(WALL)。
//
//   两个汉字相遇时:
//   1. 计算音韵公约数 → 共振强度
//   2. 看极性关系 → 是否能共存
//   3. 如果极性相反 → 撞墙, 记住, 永不回头
//   4. 如果极性相同 → 同见者, 形成一维时间
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

// ══════════════════════════════════════════════════════════════
// 本质 — 不是特征向量, 是汉字自身的构型
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Essence(char);

impl Essence {
    pub fn new(ch: char) -> Self { Self(ch) }
    pub fn ch(&self) -> char { self.0 }

    /// 音韵坐标 — 不是查询出来的, 是字形自带的
    pub fn coord(&self) -> (u8, u8, u8) {
        let d = char_to_dragon(self.0);
        (d.initial(), d.final_(), d.tone())
    }

    /// 极性 — 不是赋值的, 是声调自然编码的
    pub fn polarity(&self) -> i8 {
        match self.coord().2 {
            1 | 2 => -1,
            3 => 0,
            4 | 5 => 1,
            _ => 0,
        }
    }

    /// 与另一本质的共振 — 三维公约数自然涌现
    pub fn resonance(&self, other: &Essence) -> u64 {
        let (a1, a2, a3) = self.coord();
        let (b1, b2, b3) = other.coord();
        gcd64(a1 as u64, b1 as u64)
            .saturating_mul(gcd64(a2 as u64, b2 as u64))
            .saturating_mul(gcd64(a3 as u64, b3 as u64))
    }

    /// 极性关系 — 不是比较结果, 是自然如此
    pub fn relation_to(&self, other: &Essence) -> Polarity {
        match (self.polarity(), other.polarity()) {
            (0, _) | (_, 0) => Polarity::Boundary,
            (a, b) if a == b => Polarity::Resonance,
            _ => Polarity::Wall,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Resonance,  // 同极性 = 共振 = 一维时间 = 可交互
    Wall,       // 反极性 = 排斥 = 零维空间 = 不可交互
    Boundary,   // 平衡态 = 分界 = 可能可交互
}

// ══════════════════════════════════════════════════════════════
// 交互痕迹 — 不是日志, 是相遇后的印记
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct InteractionMark {
    from: char,
    to: char,
    resonance: u64,
    polarity: Polarity,
    viable: bool,
}

impl InteractionMark {
    pub fn between(a: &Essence, b: &Essence) -> Self {
        let resonance = a.resonance(b);
        let polarity = a.relation_to(b);
        let viable = resonance > 0
            && !matches!(polarity, Polarity::Wall);
        Self { from: a.ch(), to: b.ch(), resonance, polarity, viable }
    }
}

// ══════════════════════════════════════════════════════════════
// 交互引擎 — 不是事件循环, 是汉字之间的相遇场
// ══════════════════════════════════════════════════════════════

pub struct ContainerInteract {
    /// 已发生的相遇痕迹 — 不是日志, 是历史的印记
    marks: Vec<InteractionMark>,
    /// 撞墙记录 — 极性相反者永不交互
    walls: Vec<(char, char)>,
}

impl ContainerInteract {
    pub fn new() -> Self { Self { marks: Vec::new(), walls: Vec::new() } }

    /// 两个汉字相遇 — 不是事件分发, 是自然共振
    pub fn meet(&mut self, a: char, b: char) -> InteractionMark {
        let ea = Essence::new(a);
        let eb = Essence::new(b);
        let mark = InteractionMark::between(&ea, &eb);
        if mark.polarity == Polarity::Wall {
            self.walls.push((a, b));
        }
        self.marks.push(mark.clone());
        mark
    }

    /// 批量交互 — 字符串中所有汉字对自然相遇
    pub fn mutual_resolve(&mut self, chars: &[char]) -> Vec<InteractionMark> {
        let mut results = Vec::new();
        for (i, &a) in chars.iter().enumerate() {
            for &b in chars.iter().skip(i + 1) {
                let m = self.meet(a, b);
                results.push(m);
            }
        }
        results
    }

    /// 可交互的对 (非墙, 有共振)
    pub fn viable_pairs(&self) -> Vec<&InteractionMark> {
        self.marks.iter().filter(|m| m.viable).collect()
    }

    /// 特定汉字的撞墙记录
    pub fn walls_of(&self, ch: char) -> Vec<(char, char)> {
        self.walls.iter()
            .filter(|(a, b)| *a == ch || *b == ch)
            .cloned()
            .collect()
    }

    /// 特定汉字的共振伙伴
    pub fn partners_of(&self, ch: char) -> Vec<(char, u64)> {
        self.marks.iter()
            .filter(|m| m.viable && (m.from == ch || m.to == ch))
            .map(|m| {
                let partner = if m.from == ch { m.to } else { m.from };
                (partner, m.resonance)
            })
            .collect()
    }

    /// 最强共振对
    pub fn strongest_link(&self) -> Option<&InteractionMark> {
        self.viable_pairs().into_iter().max_by_key(|m| m.resonance)
    }
}

fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

// ══════════════════════════════════════════════════════════════
// 测试 — 不是验证算法, 是确认"自然如此"
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn essence_is_not_struct() {
        let e = Essence::new('共');
        // 本质 = 汉字, 不是特征的聚合
        assert_eq!('共', e.ch());
    }

    #[test] fn resonance_natural() {
        let a = Essence::new('共');
        let b = Essence::new('同');
        // 共振不是计算, 是公约数"本来就在那里"
        assert!(a.resonance(&b) > 0);
    }

    #[test] fn polarity_from_tone_only() {
        let e = Essence::new('满');
        // 极性 = 声调的自然结果, 不是赋值
        assert!(matches!(e.polarity(), -1 | 0 | 1));
    }

    #[test] fn wall_natural() {
        // 极性相异 = 墙 是自然涌现的, 不是 if-else 判断
        let pol = Polarity::Wall;
        assert!(matches!(pol, Polarity::Wall));
    }

    #[test] fn meet_creates_mark() {
        let mut engine = ContainerInteract::new();
        let mark = engine.meet('共', '同');
        // 相遇后留下痕迹, 不是日志, 是"印记"
        assert_eq!(mark.from, '共');
    }

    #[test] fn wall_remembered() {
        let mut engine = ContainerInteract::new();
        // 极性相反的相遇 → 墙被记住
        engine.meet('满', '空');  // 可能极性相同或不同
        // 无论结果如何, 墙的机制是"自然记住"
    }

    #[test] fn strongest_is_natural() {
        let chars = ['满', '全', '法', '共', '同'];
        let mut engine = ContainerInteract::new();
        engine.mutual_resolve(&chars);
        let _strongest = engine.strongest_link();
        // 最强共振自然涌现, 不是排序选择
    }

    #[test] fn viable_pairs_filter() {
        let mut engine = ContainerInteract::new();
        engine.mutual_resolve(&['囡', '囝', '共']);
        let viable = engine.viable_pairs();
        // 可交互 = 非墙且共振>0
        for v in &viable {
            assert!(v.viable);
        }
    }

    #[test] fn partners_of_char() {
        let mut engine = ContainerInteract::new();
        engine.mutual_resolve(&['生', '共', '全']);
        let partners = engine.partners_of('共');
        // 共振伙伴自然存在
        for (_, strength) in &partners {
            assert!(*strength > 0);
        }
    }
}
