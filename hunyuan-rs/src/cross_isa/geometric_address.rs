// 几何交叉寻址 — 五进制容器体积计算器
// 
// 五种进制 = 五种不同长度的容器。交叉 = 体积融合。
// 地址 = 交叉后立方体中的统计位置（非坐标）。
//
// 体积容量:
//   声母 48 × 韵母 60 × 声调 66 × 十进制 42 × 百扩展 100
//   = 798,336,000 ≈ 8 亿地址
//
// 多音节推导:
//   3/4 音节不新增地址维度。
//   通过 SmrlCell 极性位 bit38/39/40 做推导规则 (derivation rule)。
//   任意时刻只有 2 音节占用寻址通道。
//
// 替换模式:
//   第 3/4 音节 = 极性态叠加在基础 2 音节地址上。
//   bit38=1 第3音节夫标记 / bit39=1 第3音节妻标记 / bit40=1 争议态

use crate::smrl_cell::{SmrlCell, CellOwner};
use crate::cross_isa::coordinate::Coord3D;

// ══════════════════════════════════════════════════════════════════════
// 五进制定义：每种进制的符号数量 = 该维度的几何长度
// ══════════════════════════════════════════════════════════════════════

/// 声母进制 — 24 小写 + 24 大写 = 48 个符号
pub const RADIX_INITIAL: u64 = 48;

/// 韵母进制 — 自定义单字符韵母（非拼音组合体），约 30~36，取 60 含变体
pub const RADIX_FINAL: u64 = 60;

/// 声调进制 — 非拼音四声，30~60 个可统计符号，含大写
pub const RADIX_TONE: u64 = 66;

/// 十进制变体 — 阿拉伯数字 0-9 的视觉变体数（相位/字体）
pub const RADIX_DECIMAL_VARIANT: u64 = 42;

/// 百扩展 — 100 进制延伸层（更大容量的命名空间）
pub const RADIX_HUNDRED_EXTENSION: u64 = 100;

// ══════════════════════════════════════════════════════════════════════
// 几何体积 — 交叉后的总容量
// ══════════════════════════════════════════════════════════════════════

/// 五维交叉总体积 = 8 亿地址
pub const TOTAL_ADDRESS_SPACE: u64 = RADIX_INITIAL
    .wrapping_mul(RADIX_FINAL)
    .wrapping_mul(RADIX_TONE)
    .wrapping_mul(RADIX_DECIMAL_VARIANT)
    .wrapping_mul(RADIX_HUNDRED_EXTENSION);

/// 不含百扩展的体积（前四维）= 800 万地址
pub const BASE_ADDRESS_SPACE: u64 = RADIX_INITIAL
    .wrapping_mul(RADIX_FINAL)
    .wrapping_mul(RADIX_TONE)
    .wrapping_mul(RADIX_DECIMAL_VARIANT);

// ══════════════════════════════════════════════════════════════════════
// 二维面容量（快速查表用）
// ══════════════════════════════════════════════════════════════════════

/// 声母 × 韵母 的平面 = 同音面子空间
pub const PLANE_INITIAL_FINAL: u64 = RADIX_INITIAL * RADIX_FINAL;

/// 声调 × 十进制变体 的平面 = 色调相位面
pub const PLANE_TONE_DECIMAL: u64 = RADIX_TONE * RADIX_DECIMAL_VARIANT;

// ══════════════════════════════════════════════════════════════════════
// 几何形容器 — 每种进制的"形状身份"
// ══════════════════════════════════════════════════════════════════════

pub enum GeometricRole {
    Width,   // 宽 — 最短维度（十进制 10 的本体）
    Length,  // 长 — 中等维度（声母 24~48）
    Height,  // 高 — 最长维度（声调 30~60）
}

pub struct GeometricDimension {
    pub name: &'static str,
    pub radix: u64,
    pub role: GeometricRole,
    pub shape: &'static str, // 几何描述
}

pub const DIMENSIONS: [GeometricDimension; 5] = [
    GeometricDimension {
        name: "声母",
        radix: RADIX_INITIAL,
        role: GeometricRole::Length,
        shape: "长向 · 线性容器",
    },
    GeometricDimension {
        name: "韵母",
        radix: RADIX_FINAL,
        role: GeometricRole::Height,
        shape: "高向 · 面容器",
    },
    GeometricDimension {
        name: "声调",
        radix: RADIX_TONE,
        role: GeometricRole::Height,
        shape: "高向 · 体容器",
    },
    GeometricDimension {
        name: "十进制变体",
        radix: RADIX_DECIMAL_VARIANT,
        role: GeometricRole::Width,
        shape: "宽向 · 线容器",
    },
    GeometricDimension {
        name: "百扩展",
        radix: RADIX_HUNDRED_EXTENSION,
        role: GeometricRole::Width,
        shape: "宽向 · 线容器（延伸）",
    },
];

// ══════════════════════════════════════════════════════════════════════
// 坐标 — 在交叉体积中的统计位置
// ══════════════════════════════════════════════════════════════════════

/// 五维坐标 = 在交叉体积中的唯一地址索引
pub struct GeoCoordinate {
    pub initial: u8,          // 声母索引 [0, 48)
    pub final_tone: u8,       // 韵母索引 [0, 60)
    pub tone: u8,             // 声调索引 [0, 66)
    pub decimal_variant: u8,  // 十进制变体索引 [0, 42)
    pub hundred_ext: u16,     // 百扩展索引 [0, 100)
}

impl GeoCoordinate {
    /// 计算在五维体积中的一维展开地址
    /// 公式: (((((hundred * DECIMAL) + decimal) * TONE) + tone) * FINAL) + final) * INITIAL + initial
    pub fn to_address(&self) -> u64 {
        let mut addr: u64 = self.hundred_ext as u64;
        addr = addr * RADIX_DECIMAL_VARIANT + self.decimal_variant as u64;
        addr = addr * RADIX_TONE + self.tone as u64;
        addr = addr * RADIX_FINAL + self.final_tone as u64;
        addr = addr * RADIX_INITIAL + self.initial as u64;
        addr
    }

    /// 从一维地址反算五维坐标
    pub fn from_address(mut addr: u64) -> Self {
        let initial = (addr % RADIX_INITIAL) as u8;
        addr /= RADIX_INITIAL;
        let final_tone = (addr % RADIX_FINAL) as u8;
        addr /= RADIX_FINAL;
        let tone = (addr % RADIX_TONE) as u8;
        addr /= RADIX_TONE;
        let decimal_variant = (addr % RADIX_DECIMAL_VARIANT) as u8;
        addr /= RADIX_DECIMAL_VARIANT;
        let hundred_ext = (addr % RADIX_HUNDRED_EXTENSION) as u16;
        Self {
            initial,
            final_tone,
            tone,
            decimal_variant,
            hundred_ext,
        }
    }

    /// 验证坐标是否越界
    pub fn is_valid(&self) -> bool {
        (self.initial as u64) < RADIX_INITIAL
            && (self.final_tone as u64) < RADIX_FINAL
            && (self.tone as u64) < RADIX_TONE
            && (self.decimal_variant as u64) < RADIX_DECIMAL_VARIANT
            && (self.hundred_ext as u64) < RADIX_HUNDRED_EXTENSION
    }

    /// 映射到三维冰箱坐标
    /// X = 声母 % 256, Y = (韵母 << 8 | 声调), Z = (十进制变体 << 8 | 百扩展低8位)
    pub fn to_coord3d(&self) -> Coord3D {
        Coord3D::new(
            self.initial,
            (self.final_tone as u16) << 8 | (self.tone & 0x0F) as u16,
            (self.decimal_variant << 2) | (self.hundred_ext & 0x03) as u8,
        )
    }
}

// ══════════════════════════════════════════════════════════════════════
// 多音节推导 — 通过 SmrlCell 极性位扩展
// ══════════════════════════════════════════════════════════════════════

/// 推导规则 = 额外音节如何通过极性位叠加在基础 2 音节地址上
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivationRule {
    /// 第 3 音节 — 夫位标记 (bit38=1)
    ThirdSyllableHusband,
    /// 第 3 音节 — 妻位标记 (bit39=1)
    ThirdSyllableWife,
    /// 第 4 音节 — 争议态 (bit40=1)
    FourthDisputed,
    /// 推导完成（所有极性位已用完）
    Exhausted,
}

/// 派生地址 = 基础地址 + 极性态
pub struct DerivedAddress {
    /// 基础 2 音节地址（统计位置）
    pub base: u64,
    /// 极性态 = 哪些额外音节参与了推导
    pub cell_marker: SmrlCell,
}

impl DerivedAddress {
    /// 从基础坐标 + 音节序列构建派生地址
    /// syllable_chain: [idx0=声母, idx1=韵母, idx2=第3可选, idx3=第4可选]
    pub fn derive(idx_chain: &[u8]) -> Option<Self> {
        if idx_chain.len() < 2 || idx_chain.len() > 4 {
            return None;
        }

        // 基础 2 声母位置
        let base_coord = GeoCoordinate {
            initial: idx_chain[0],
            final_tone: idx_chain[1],
            tone: 0,
            decimal_variant: 0,
            hundred_ext: 0,
        };
        let base = base_coord.to_address();

        // 极性位标记额外音节
        let mut marker = SmrlCell::from_axes(0, 0, 0, 0, 1);
        if idx_chain.len() >= 3 {
            // 第 3 音节：夫位先占 (YANG=1)
            marker = SmrlCell::from_axes(0, POLARITY_HUSBAND, 0, 0, 1);
        }
        if idx_chain.len() >= 4 {
            // 第 4 音节：争议态 (BALANCED=2)
            marker = SmrlCell::from_axes(0, POLARITY_DISPUTED, 0, 0, 1);
        }

        Some(Self { base, cell_marker: marker })
    }
}

// 极性位轴
pub enum PolarityBit {
    Husband,  // 夫标记
    Wife,     // 妻标记
    Disputed, // 待裁定
}

pub use PolarityBit as Axis;

// SmrlCell polarity 值（基于 smrl_cell.rs YIN=0/YANG=1/BALANCED=2 编码）
const POLARITY_HUSBAND: u8 = 1;   // YANG = 夫位激活
const POLARITY_WIFE: u8 = 0;      // YIN = 妻位激活
const POLARITY_DISPUTED: u8 = 2;  // BALANCED = 争议态

// ══════════════════════════════════════════════════════════════════════
// 统计接口 — 确定性和可统计性保障
// ══════════════════════════════════════════════════════════════════════

pub struct GeometryStats;

impl GeometryStats {
    /// 总体积
    pub fn total_volume() -> u64 {
        TOTAL_ADDRESS_SPACE
    }

    /// 当前已用地址数（通过已插入容器的表格大小）
    pub fn occupied_count(table_count: usize) -> usize {
        table_count
    }

    /// 利用率
    pub fn utilization(occupied: usize) -> f64 {
        if TOTAL_ADDRESS_SPACE == 0 {
            return 1.0;
        }
        occupied as f64 / TOTAL_ADDRESS_SPACE as f64
    }

    /// 基础面利用率（不含百扩展）
    pub fn base_utilization(occupied: usize) -> f64 {
        if BASE_ADDRESS_SPACE == 0 {
            return 1.0;
        }
        occupied as f64 / BASE_ADDRESS_SPACE as f64
    }
}

// ══════════════════════════════════════════════════════════════════════
// 映射兼容性 — 与现有 SmrlCell / CellOwner 对接
// ══════════════════════════════════════════════════════════════════════

/// 将几何坐标 + 主权映射到 SmrlCell
/// - coord ← coord3d.x (声母维度)
/// - polarity ← 声调 % 3 (YIN=0/YANG=1/BALANCED=2 三态循环)
/// - sovereignty ← owner.encode()
/// - value ← coord3d.y (韵母←低位 + 声调→高位)
/// - generation ← coord3d.z
pub fn coord_to_cell_with_geo(coord: &GeoCoordinate, owner: CellOwner) -> SmrlCell {
    let c = coord.to_coord3d();
    let polarity = (coord.tone % 3) as u8;
    SmrlCell::from_axes(c.x as u16, polarity, owner.encode(), c.y, c.z)
}

// ══════════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_volume_correct() {
        assert_eq!(TOTAL_ADDRESS_SPACE, 798_336_000);
    }

    #[test]
    fn base_volume_correct() {
        assert_eq!(BASE_ADDRESS_SPACE, 7_983_360);
    }

    #[test]
    fn coord_roundtrip() {
        let coord = GeoCoordinate {
            initial: 10,
            final_tone: 20,
            tone: 30,
            decimal_variant: 5,
            hundred_ext: 50,
        };
        let addr = coord.to_address();
        let decoded = GeoCoordinate::from_address(addr);
        assert_eq!(decoded.initial, 10);
        assert_eq!(decoded.final_tone, 20);
        assert_eq!(decoded.tone, 30);
        assert_eq!(decoded.decimal_variant, 5);
        assert_eq!(decoded.hundred_ext, 50);
    }

    #[test]
    fn coord_validity() {
        let valid = GeoCoordinate {
            initial: 47,  // < 48 有效
            final_tone: 59, // < 60 有效
            tone: 65,     // < 66 有效
            decimal_variant: 41, // < 42 有效
            hundred_ext: 99, // < 100 有效
        };
        assert!(valid.is_valid());

        let invalid = GeoCoordinate {
            initial: 48,  // == 48 越界
            final_tone: 0,
            tone: 0,
            decimal_variant: 0,
            hundred_ext: 0,
        };
        assert!(!invalid.is_valid());
    }

    #[test]
    fn derived_address_basic() {
        let derived = DerivedAddress::derive(&[5, 10]).unwrap();
        assert_eq!(derived.cell_marker.polarity(), 0); // 无额外音节
    }

    #[test]
    fn derived_address_third_syllable() {
        let derived = DerivedAddress::derive(&[5, 10, 15]).unwrap();
        // 第 3 音节设置了极性位
        assert_ne!(derived.cell_marker.polarity(), 0);
    }

    #[test]
    fn derived_address_fourth_syllable() {
        let derived = DerivedAddress::derive(&[5, 10, 15, 20]).unwrap();
        // 第 4 音节 = 争议态
        assert_ne!(derived.cell_marker.polarity(), 0);
    }

    #[test]
    fn stats_total() {
        assert_eq!(GeometryStats::total_volume(), 798_336_000);
    }
}
