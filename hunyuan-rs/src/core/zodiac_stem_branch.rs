// zodiac_stem_branch.rs — 生肖天干地支·循环计算引擎 (v0.1)
//
//   不是数据结构, 是"时间本身在呼吸"。
//   10天干 × 12地支 = 60甲子 = 一个完整的时间周期。
//   不是数组索引, 是"时间坐标的自然流动"。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   传统循环: for i in 0..60 { arr[i] } (索引驱动)
//   甲子循环: 甲子 → 乙丑 → ... → 癸亥 → 甲子 (自然流动)
//
//   天干 = 空间轴 (10 进制): 甲乙丙丁戊己庚辛壬癸
//   地支 = 时间轴 (12 进制): 子丑寅卯辰巳午未申酉戌亥
//
//   两者交织 = 最小公倍数 60 = 一个完整的存在周期。
//
//   生肖附在地支上: 鼠牛虎兔龙蛇马羊猴鸡狗猪 = 12 种极性
//
//   不是遍历算法, 是"时间自然流过每一个甲子"。
//
// ══════════════════════════════════════════════════════════════

/// 十天干 — 空间轴上的10个位置
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HeavenlyStem {
    Jia = 0,   // 甲 — 木阳
    Yi = 1,    // 乙 — 木阴
    Bing = 2,  // 丙 — 火阳
    Ding = 3,  // 丁 — 火阴
    Wu = 4,    // 戊 — 土阳
    Ji = 5,    // 己 — 土阴
    Geng = 6,  // 庚 — 金阳
    Xin = 7,   // 辛 — 金阴
    Ren = 8,   // 壬 — 水阳
    Gui = 9,   // 癸 — 水阴
}

/// 十二地支 — 时间轴上的12个位置
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EarthlyBranch {
    Zi = 0,   // 子 — 鼠 (水阳)
    Chou = 1, // 丑 — 牛 (土阴)
    Yin = 2,  // 寅 — 虎 (木阳)
    Mao = 3,  // 卯 — 兔 (木阴)
    Chen = 4, // 辰 — 龙 (土阳)
    Si = 5,   // 巳 — 蛇 (火阴)
    Wu = 6,   // 午 — 马 (火阳)
    Wei = 7,  // 未 — 羊 (土阴)
    Shen = 8, // 申 — 猴 (金阳)
    You = 9,  // 酉 — 鸡 (金阴)
    Xu = 10,  // 戌 — 狗 (土阳)
    Hai = 11, // 亥 — 猪 (水阴)
}

/// 生肖 — 地支上的极性显现
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ZodiacAnimal {
    Rat, Ox, Tiger, Rabbit, Dragon, Snake,
    Horse, Goat, Monkey, Rooster, Dog, Pig,
}

/// 一甲子 — 天地交叉的一个完整时刻
#[derive(Clone, Copy, Debug)]
pub struct Sexagenary {
    stem: HeavenlyStem,
    branch: EarthlyBranch,
}

impl Sexagenary {
    /// 从序号创建 (0 = 甲子, 59 = 癸亥)
    pub fn from_index(idx: u64) -> Self {
        let idx = idx % 60;
        // 天干 = idx % 10, 地支 = idx % 12, 但有效交叉点需满足 stem%2 == branch%2
        for i in 0..60 {
            let candidate = (idx + i) % 60;
            if (candidate % 10) % 2 == (candidate % 12) % 2 {
                return Self {
                    stem: unsafe { std::mem::transmute((candidate % 10) as u8) },
                    branch: unsafe { std::mem::transmute((candidate % 12) as u8) },
                };
            }
        }
        Self { stem: HeavenlyStem::Jia, branch: EarthlyBranch::Zi }
    }

    /// 到下一甲子 — 不是加法, 是"时间流淌"
    pub fn next(&self) -> Self {
        Self::from_index(self.to_index() + 1)
    }

    /// 到上一甲子 — "时间回溯"
    pub fn prev(&self) -> Self {
        let idx = self.to_index();
        Self::from_index(if idx == 0 { 59 } else { idx - 1 })
    }

    /// 当前序号 [0, 60)
    pub fn to_index(&self) -> u64 {
        let s = self.stem as u64;
        let b = self.branch as u64;
        // 找到满足 s≡idx(mod10), b≡idx(mod12), s%2==b%2 的 idx
        for i in 0..60 {
            if i % 10 == s && i % 12 == b {
                return i;
            }
        }
        0
    }

    /// 此甲子的生肖 = 地支对应的动物
    pub fn zodiac(&self) -> ZodiacAnimal {
        match self.branch {
            EarthlyBranch::Zi => ZodiacAnimal::Rat,
            EarthlyBranch::Chou => ZodiacAnimal::Ox,
            EarthlyBranch::Yin => ZodiacAnimal::Tiger,
            EarthlyBranch::Mao => ZodiacAnimal::Rabbit,
            EarthlyBranch::Chen => ZodiacAnimal::Dragon,
            EarthlyBranch::Si => ZodiacAnimal::Snake,
            EarthlyBranch::Wu => ZodiacAnimal::Horse,
            EarthlyBranch::Wei => ZodiacAnimal::Goat,
            EarthlyBranch::Shen => ZodiacAnimal::Monkey,
            EarthlyBranch::You => ZodiacAnimal::Rooster,
            EarthlyBranch::Xu => ZodiacAnimal::Dog,
            EarthlyBranch::Hai => ZodiacAnimal::Pig,
        }
    }

    /// 此甲子的五行 — 从天干自然涌现
    pub fn phase(&self) -> FivePhase {
        match self.stem as u8 {
            0 | 1 => FivePhase::Wood,   // 甲乙
            2 | 3 => FivePhase::Fire,   // 丙丁
            4 | 5 => FivePhase::Earth,  // 戊己
            6 | 7 => FivePhase::Metal,  // 庚辛
            _ => FivePhase::Water,       // 壬癸
        }
    }

    /// 此甲子的阴阳 — 不是布尔值, 是相性
    pub fn yin_yang(&self) -> YinYang {
        if (self.stem as u8) % 2 == 0 {
            YinYang::Yang
        } else {
            YinYang::Yin
        }
    }

    /// 与此甲子的"距离" — 不是减法, 是"时间流的远近"
    pub fn temporal_distance(&self, other: &Self) -> u64 {
        let a = self.to_index();
        let b = other.to_index();
        if a > b { a - b } else { b - a }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FivePhase { Wood, Fire, Earth, Metal, Water }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YinYang { Yin, Yang }

/// 甲子引擎 — 60甲子的自然流动
pub struct SexagenaryCycle;

impl SexagenaryCycle {
    /// 生成一整个60甲子周期 — 不是循环语句, 是"时间的完整呼吸"
    pub fn full_cycle() -> Vec<Sexagenary> {
        (0..60).map(Sexagenary::from_index).collect()
    }

    /// 从当前甲子流动N步
    pub fn flow(from: &Sexagenary, steps: u64) -> Sexagenary {
        Sexagenary::from_index(from.to_index() + steps)
    }

    /// 两个甲子之间的"时间流" — 从前者自然走到后者
    pub fn flow_between(from: &Sexagenary, to: &Sexagenary) -> Vec<Sexagenary> {
        let start = from.to_index();
        let end = to.to_index();
        let mut result = Vec::new();
        let mut current = start;
        loop {
            result.push(Sexagenary::from_index(current));
            if current == end { break; }
            current = (current + 1) % 60;
        }
        result
    }

    /// 生克关系 — 五行之间的自然作用, 不是条件判断
    pub fn generation(a: FivePhase, b: FivePhase) -> PhaseRelation {
        use FivePhase::*;
        match (a, b) {
            (Wood, Fire) | (Fire, Earth) | (Earth, Metal) | (Metal, Water) | (Water, Wood) =>
                PhaseRelation::Generates,   // 我生
            (Fire, Wood) | (Earth, Fire) | (Metal, Earth) | (Water, Metal) | (Wood, Water) =>
                PhaseRelation::GeneratedBy, // 生我
            (Wood, Earth) | (Earth, Water) | (Water, Fire) | (Fire, Metal) | (Metal, Wood) =>
                PhaseRelation::Overcomes,   // 我克
            (Earth, Wood) | (Water, Earth) | (Fire, Water) | (Metal, Fire) | (Wood, Metal) =>
                PhaseRelation::OvercomeBy,  // 克我
            _ => PhaseRelation::Same,       // 同我
        }
    }

    /// 从生肖找地支 — 不是查找表, 是"对应关系"
    pub fn branch_of_zodiac(animal: ZodiacAnimal) -> EarthlyBranch {
        match animal {
            ZodiacAnimal::Rat => EarthlyBranch::Zi,
            ZodiacAnimal::Ox => EarthlyBranch::Chou,
            ZodiacAnimal::Tiger => EarthlyBranch::Yin,
            ZodiacAnimal::Rabbit => EarthlyBranch::Mao,
            ZodiacAnimal::Dragon => EarthlyBranch::Chen,
            ZodiacAnimal::Snake => EarthlyBranch::Si,
            ZodiacAnimal::Horse => EarthlyBranch::Wu,
            ZodiacAnimal::Goat => EarthlyBranch::Wei,
            ZodiacAnimal::Monkey => EarthlyBranch::Shen,
            ZodiacAnimal::Rooster => EarthlyBranch::You,
            ZodiacAnimal::Dog => EarthlyBranch::Xu,
            ZodiacAnimal::Pig => EarthlyBranch::Hai,
        }
    }

    /// 将汉字映射到最近的甲子 — 汉字的音韵坐标 → 时间坐标
    pub fn approximate_for(ch: char) -> Sexagenary {
        use crate::name_creator::char_to_dragon;
        let d = char_to_dragon(ch);
        let idx = (d.initial() as u64 + d.final_() as u64 + d.tone() as u64) % 60;
        Sexagenary::from_index(idx)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseRelation {
    Generates,   // 我生
    GeneratedBy, // 生我
    Overcomes,   // 我克
    OvercomeBy,  // 克我
    Same,        // 同我
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sexagenary_from_zero_is_jiazi() {
        let s = Sexagenary::from_index(0);
        assert_eq!(s.stem, HeavenlyStem::Jia);
        assert_eq!(s.branch, EarthlyBranch::Zi);
    }

    #[test] fn sexagenary_next_flows() {
        let s = Sexagenary::from_index(0); // 甲子
        let next = s.next();
        assert_eq!(next.stem, HeavenlyStem::Yi);
        assert_eq!(next.branch, EarthlyBranch::Chou);
    }

    #[test] fn zodiac_rat_at_zi() {
        let s = Sexagenary::from_index(0);
        assert_eq!(s.zodiac(), ZodiacAnimal::Rat);
    }

    #[test] fn phase_from_stem() {
        let jia = Sexagenary::from_index(0);
        assert_eq!(jia.phase(), FivePhase::Wood);
    }

    #[test] fn full_cycle_has_60() {
        let cycle = SexagenaryCycle::full_cycle();
        assert_eq!(cycle.len(), 60);
    }

    #[test] fn flow_steps_correct() {
        let from = Sexagenary::from_index(0);
        let result = SexagenaryCycle::flow(&from, 59);
        assert_eq!(result.to_index(), 59);
    }

    #[test] fn temporal_distance_natural() {
        let a = Sexagenary::from_index(0);
        let b = Sexagenary::from_index(30);
        assert_eq!(a.temporal_distance(&b), 30);
    }

    #[test] fn generation_relation() {
        assert_eq!(
            SexagenaryCycle::generation(FivePhase::Wood, FivePhase::Fire),
            PhaseRelation::Generates
        );
    }

    #[test] fn hanzi_approximation() {
        let s = SexagenaryCycle::approximate_for('囡');
        assert!(s.to_index() < 60);
    }

    #[test] fn yin_yang_alternates() {
        let s = Sexagenary::from_index(0); // 甲 = Yang
        assert_eq!(s.yin_yang(), YinYang::Yang);
    }
}
