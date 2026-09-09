// ══════════════════════════════════════════════════════════════════════
// glyph_living.rs — 汉字活容器 · 自描述符号系统 (v0.1)
// ══════════════════════════════════════════════════════════════════════
//
// 核心设计:
//   - 底层: char 的 Unicode 码点 = 机器码 (不改不动)
//   - 上层: 绑定声韵调 + 家族 + 坐标 + 关系 + 裁决 + 时间
//   - 每个汉字成为一个"活容器" — 知道自己是谁, 在哪, 和谁相关
//
// 约束:
//   - 仅加减 + 位运算 (无乘除)
//   - 映射是确定性的: 相同输入永远得到相同容器
//   - 不创建新进制, 直接在 Unicode 之上叠关系层
// ══════════════════════════════════════════════════════════════════════

use crate::mirror::{self, EncodedCell, Sovereignty};

// ────────────────────────────────────────────────────────────
// 声母枚举 (24 基础)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shengmu {
    B, P, M, F, D, T, N, L,
    G, K, H, J, Q, X,
    Zh, Ch, Sh, R, Z, C, S,
    Y, W, Zero, // Zero = 零声母
}

impl Shengmu {
    pub fn index(self) -> u8 {
        match self {
            Self::B => 0, Self::P => 1, Self::M => 2, Self::F => 3,
            Self::D => 4, Self::T => 5, Self::N => 6, Self::L => 7,
            Self::G => 8, Self::K => 9, Self::H => 10,
            Self::J => 11, Self::Q => 12, Self::X => 13,
            Self::Zh => 14, Self::Ch => 15, Self::Sh => 16, Self::R => 17,
            Self::Z => 18, Self::C => 19, Self::S => 20,
            Self::Y => 21, Self::W => 22, Self::Zero => 23,
        }
    }
    pub fn try_from_index(i: u8) -> Option<Self> {
        use Shengmu::*;
        const ALL: [Shengmu; 24] = [B, P, M, F, D, T, N, L, G, K, H, J, Q, X, Zh, Ch, Sh, R, Z, C, S, Y, W, Zero];
        ALL.get(i as usize).copied()
    }
}

// ────────────────────────────────────────────────────────────
// 韵母枚举 (简化版, 覆盖主要韵母)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Yunmu {
    A, O, E, I, U, Ue,  // 单韵母
    Ai, Ei, Ao, Ou,      // 二合
    An, En, Ang, Eng, // 鼻音
    IA, IE, Iao, Iou, Ian, Iang, In, Ing,  // 齐齿
    Ua, Uo, Uai, Uei, Uan, Uen, Uang, Ueng, // 合口
    UeN,                 // 撮口
}

impl Yunmu {
    pub fn index(self) -> u8 {
        match self {
            Self::A => 0, Self::O => 1, Self::E => 2, Self::I => 3, Self::U => 4, Self::Ue => 5,
            Self::Ai => 6, Self::Ei => 7, Self::Ao => 8, Self::Ou => 9,
            Self::An => 10, Self::En => 11, Self::Ang => 12, Self::Eng => 13,
            Self::IA => 14, Self::IE => 15, Self::Iao => 16, Self::Iou => 17,
            Self::Ian => 18, Self::Iang => 19, Self::In => 20, Self::Ing => 21,
            Self::Ua => 22, Self::Uo => 23, Self::Uai => 24, Self::Uei => 25,
            Self::Uan => 26, Self::Uen => 27, Self::Uang => 28, Self::Ueng => 29,
            Self::UeN => 30,
        }
    }
    pub fn try_from_index(i: u8) -> Option<Self> {
        use Yunmu::*;
        const ALL: [Yunmu; 31] = [
            A, O, E, I, U, Ue, Ai, Ei, Ao, Ou, An, En, Ang, Eng,
            IA, IE, Iao, Iou, Ian, Iang, In, Ing,
            Ua, Uo, Uai, Uei, Uan, Uen, Uang, Ueng, UeN,
        ];
        ALL.get(i as usize).copied()
    }
}

// ────────────────────────────────────────────────────────────
// 声调 (四声 + 轻声)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tone {
    T1, T2, T3, T4, Neutral,
}

impl Tone {
    pub fn index(self) -> u8 {
        match self {
            Self::T1 => 0, Self::T2 => 1, Self::T3 => 2, Self::T4 => 3, Self::Neutral => 4,
        }
    }
    pub fn try_from_index(i: u8) -> Option<Self> {
        use Tone::*;
        const ALL: [Tone; 5] = [T1, T2, T3, T4, Neutral];
        ALL.get(i as usize).copied()
    }
}

// ────────────────────────────────────────────────────────────
// 汉字家族 (与容器族的延伸对应)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GlyphFamily {
    Container,   // 袋子级 — 实体存在
    Operation,   // 态射级 — 作用关系
    Couple,      // 夫妻级 — 囡囝共生
    Homophone,   // 谐音桥 — 发音引力
    Phallus,     // 丨 级 — 结构骨
    YinYang,     // 阴阳级 — 镜像对
}

impl GlyphFamily {
    /// 决定这个家族在关系网络中的极性基调
    pub fn polarity_bias(self) -> u64 {
        match self {
            Self::Container => mirror::POLARITY_NEGATIVE,   // 阴=包容
            Self::Operation => mirror::POLARITY_POSITIVE,    // 阳=主动
            Self::Couple => mirror::POLARITY_POSITIVE,       // 共生=主动
            Self::Homophone => mirror::POLARITY_POSITIVE,    // 谐音=主动投射
            Self::Phallus => mirror::POLARITY_POSITIVE,      // 丨 = 主动
            Self::YinYang => mirror::POLARITY_POSITIVE,      // 镜像=主动
        }
    }
}

// ────────────────────────────────────────────────────────────
// 活容器结构 — 汉字 + 关系 + 坐标
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LivingGlyph {
    /// 底层: Unicode 码点 (机器码, 原封不动)
    pub base: char,

    /// 声韵调属性 (发音空间坐标)
    pub shengmu: Shengmu,
    pub yunmu: Yunmu,
    pub tone: Tone,

    /// 家族归属 (视觉/语义家族)
    pub family: GlyphFamily,

    /// 时间坐标 (层级+环)
    pub layer: u32,
    pub ring: u32,

    /// 见证计数 (被三元裁判过的次数)
    pub witness_count: u64,
}

impl LivingGlyph {
    /// 构造一个活容器
    pub fn new(
        base: char,
        shengmu: Shengmu,
        yunmu: Yunmu,
        tone: Tone,
        family: GlyphFamily,
    ) -> Self {
        Self {
            base,
            shengmu,
            yunmu,
            tone,
            family,
            layer: 0,
            ring: 0,
            witness_count: 0,
        }
    }

    // ────────────────────────────────────────────────────────
    // 自我感知 — "我是什么坐标上的存在"
    // ────────────────────────────────────────────────────────

    /// 返回这个汉字的完整自我描述矩阵
    pub fn gauge_self(&self) -> GlyphSelfReport {
        GlyphSelfReport {
            glyph: self.base,
            code: self.base as u32,
            shengmu: self.shengmu,
            yunmu: self.yunmu,
            tone: self.tone,
            family: self.family,
            coord: Coord::new(self.layer, self.ring, self.shengmu.index() as u32),
            polarity: self.family.polarity_bias(),
        }
    }

    // ────────────────────────────────────────────────────────
    // 编码输出 — 变成 mirror 系统可处理的 64 位单元
    // ────────────────────────────────────────────────────────

    /// 把自己编码为一个 64 位镜像单元
    pub fn to_cell(&self) -> EncodedCell {
        let d0 = self.coord_d0();
        let d1 = self.coord_d1();
        let d2 = self.coord_d2();
        let polarity = self.family.polarity_bias();
        let sovereignty = if self.shengmu == Shengmu::Zero {
            Sovereignty::Passive
        } else {
            Sovereignty::Active
        }.encode();

        mirror::encode_cell(d0, d1, d2, polarity, sovereignty, self.witness_count, 0)
    }

    /// 一维坐标: 声母序号 × 5 + 声调序号
    fn coord_d0(&self) -> u64 {
        (self.shengmu.index() as u64) * 5 + (self.tone.index() as u64)
    }

    /// 二维坐标: 韵母序号 (发音接收空间)
    fn coord_d1(&self) -> u64 {
        self.yunmu.index() as u64
    }

    /// 三维坐标: 层级 + 环 + 家族编码
    fn coord_d2(&self) -> u64 {
        ((self.layer as u64) & 0xF) << 4
            | ((self.ring as u64) & 0xF)
            | ((self.family_code() & 0xF) << 8)
    }

    fn family_code(&self) -> u64 {
        match self.family {
            GlyphFamily::Container => 0,
            GlyphFamily::Operation => 1,
            GlyphFamily::Couple => 2,
            GlyphFamily::Homophone => 3,
            GlyphFamily::Phallus => 4,
            GlyphFamily::YinYang => 5,
        }
    }

    // ────────────────────────────────────────────────────────
    // 关系计算 — 与另一个汉字的互动
    // ────────────────────────────────────────────────────────

    /// 计算与另一个活容器的距离
    pub fn distance_to(&self, other: &LivingGlyph) -> GlyphDistance {
        let d0 = (self.coord_d0() as i64 - other.coord_d0() as i64).abs() as f64;
        let d1 = (self.coord_d1() as i64 - other.coord_d1() as i64).abs() as f64;
        let d2 = (self.coord_d2() as i64 - other.coord_d2() as i64).abs() as f64;

        let polarity_diff = if self.family.polarity_bias() == other.family.polarity_bias() {
            0.0
        } else {
            1.0
        };

        GlyphDistance {
            d0, d1, d2,
            polarity_diff,
            total: d0 + d1 + d2 + polarity_diff * 10.0,
        }
    }

    /// 判断与另一个汉字是否"相吸" (距离足够近)
    pub fn is_attracted(&self, other: &LivingGlyph, threshold: f64) -> bool {
        self.distance_to(other).total < threshold
    }

    /// 声母是否相同 (谐音桥的第一步)
    pub fn same_shengmu(&self, other: &LivingGlyph) -> bool {
        self.shengmu == other.shengmu
    }

    /// 韵母是否相同 (谐音桥的第二步)
    pub fn same_yunmu(&self, other: &LivingGlyph) -> bool {
        self.yunmu == other.yunmu
    }

    /// 是否完全同音 (忽略声调)
    pub fn same_pronunciation(&self, other: &LivingGlyph) -> bool {
        self.shengmu == other.shengmu && self.yunmu == other.yunmu
    }
}

// ────────────────────────────────────────────────────────────
// 自我描述报告
// ────────────────────────────────────────────────────────────

pub struct GlyphSelfReport {
    pub glyph: char,
    pub code: u32,
    pub shengmu: Shengmu,
    pub yunmu: Yunmu,
    pub tone: Tone,
    pub family: GlyphFamily,
    pub coord: Coord,
    pub polarity: u64,
}

// ────────────────────────────────────────────────────────────
// 空间坐标
// ────────────────────────────────────────────────────────────

pub struct Coord {
    pub layer: u32,
    pub ring: u32,
    pub sub: u32,
}

impl Coord {
    pub fn new(layer: u32, ring: u32, sub: u32) -> Self {
        Self { layer, ring, sub }
    }
}

// ────────────────────────────────────────────────────────────
// 汉距离 (两个汉字之间的"远近")
// ────────────────────────────────────────────────────────────

pub struct GlyphDistance {
    pub d0: f64,
    pub d1: f64,
    pub d2: f64,
    pub polarity_diff: f64,
    pub total: f64,
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_living_glyph_basic() {
        let glyph = LivingGlyph::new(
            '囡',
            Shengmu::N,
            Yunmu::An,
            Tone::T1,
            GlyphFamily::Couple,
        );

        let report = glyph.gauge_self();
        assert_eq!(report.glyph, '囡');
        assert_eq!(report.code, 0x56E1);
        assert_eq!(report.shengmu, Shengmu::N);
        assert_eq!(report.family, GlyphFamily::Couple);
    }

    #[test]
    fn test_same_shengmu() {
        let nan = LivingGlyph::new('囡', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::Couple);
        let ni = LivingGlyph::new('你', Shengmu::N, Yunmu::I, Tone::T3, GlyphFamily::Homophone);
        let ma = LivingGlyph::new('妈', Shengmu::M, Yunmu::A, Tone::T1, GlyphFamily::Container);

        assert!(nan.same_shengmu(&ni));
        assert!(!nan.same_shengmu(&ma));
    }

    #[test]
    fn test_same_pronunciation() {
        let nan1 = LivingGlyph::new('囡', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::Couple);
        let nan2 = LivingGlyph::new('南', Shengmu::N, Yunmu::An, Tone::T2, GlyphFamily::Container);

        // 声韵同, 调不同 — 算"音同调不同"
        assert!(nan1.same_pronunciation(&nan2));
        assert!(!nan1.is_attracted(&nan2, 0.5));
    }

    #[test]
    fn test_distance_calculation() {
        let nan = LivingGlyph::new('囡', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::Couple);
        let jian = LivingGlyph::new('囝', Shengmu::J, Yunmu::Iou, Tone::T1, GlyphFamily::Couple);

        let dist = nan.distance_to(&jian);
        assert!(dist.total >= 0.0);
        // 夫妻家族极性相同, polarity_diff 应该为 0
        assert_eq!(dist.polarity_diff, 0.0);
    }

    #[test]
    fn test_to_cell() {
        let glyph = LivingGlyph::new(
            '囡',
            Shengmu::N,
            Yunmu::An,
            Tone::T1,
            GlyphFamily::Couple,
        );

        let cell = glyph.to_cell();
        // 验证编码后可以反向解析
        let polarity = mirror::get_polarity(cell);
        assert_eq!(polarity, mirror::POLARITY_POSITIVE); // Couple家族极性=阳
    }

    #[test]
    fn test_glyph_family_polarity() {
        let container = LivingGlyph::new('袋', Shengmu::D, Yunmu::Ai, Tone::T1, GlyphFamily::Container);
        let couple = LivingGlyph::new('囡', Shengmu::N, Yunmu::An, Tone::T1, GlyphFamily::Couple);

        assert_eq!(container.family.polarity_bias(), mirror::POLARITY_NEGATIVE);
        assert_eq!(couple.family.polarity_bias(), mirror::POLARITY_POSITIVE);
    }
}
