// ══════════════════════════════════════════════════════════════════
// high_algebra.rs — 高维代数 · (n-1)×0.1 偏移率引擎
// ══════════════════════════════════════════════════════════════════
//
// 核心公式:
//   偏移率 δ = (n-1) × 0.1
//   乘数   M = 1 + δ
//
// 功能:
//   - 偏移量层级化体系 (初代数 → n次代数)
//   - 跨层级静态回归 (平行世界 → 初代数)
//   - 俘获规则 (非源头强制降维)
//   - 阶和叠加 (多世界共存 · 锐度感知 · 齐头并进偏移)
//   - 2²⁵⁶ 高位对齐
//   - 自我相似验证 (254+2 = 256)

/// 每层偏移量
pub const DELTA_PER_LEVEL: f64 = 0.1;

/// 根基: 254 集装箱 + 2 锚点
pub const BASE_FOUNDATION: i64 = 254;
pub const ANCHOR_COUNT: i64 = 2;
pub const BASE_RADIX: i64 = 256;  // 254 + 2

/// 2²⁵⁶ 指数
pub const HIGH_BIT_EXPONENT: u32 = 256;

/// 最大层级数 (九元体系)
pub const MAX_LEVELS: usize = 9;

// ── 层级定义 ──

/// 代数层级 (平行世界)
#[derive(Debug, Clone, PartialEq)]
pub struct AlgebraLevel {
    /// 代数层级 (1 = 初代数, 2 = 二次代数, ...)
    pub generation: i32,
    /// 偏移量 δ = (n-1) × 0.1
    pub offset_rate: f64,
    /// 乘数 M = 1 + δ
    pub multiplier: f64,
    /// 该层符号数量 (进制基数)
    pub base_radix: i64,
    /// 是否为源头 (偏移量最小 = 初代数)
    pub is_source: bool,
}

impl AlgebraLevel {
    /// 创建代数层级
    pub fn new(generation: i32, symbols: i64) -> Self {
        let delta = (generation - 1) as f64 * DELTA_PER_LEVEL;
        AlgebraLevel {
            generation,
            offset_rate: delta,
            multiplier: 1.0 + delta,
            base_radix: symbols,
            is_source: generation == 1,
        }
    }

    /// 创建标准九元层级 (radix=256)
    pub fn standard(generation: i32) -> Self {
        Self::new(generation, BASE_RADIX)
    }

    /// 层级描述
    pub fn describe(&self) -> String {
        if self.is_source {
            format!("第{}代(初代数/源头) δ={:.2} M={:.2}",
                    self.generation, self.offset_rate, self.multiplier)
        } else {
            format!("第{}代(平行世界) δ={:.2} M={:.2}",
                    self.generation, self.offset_rate, self.multiplier)
        }
    }
}

// ── 基本运算 ──

/// 计算偏移率 δ = (n-1) × 0.1
pub fn offset_rate(n: i32) -> f64 {
    (n - 1) as f64 * DELTA_PER_LEVEL
}

/// 计算乘数 M = 1 + δ
pub fn multiplier(n: i32) -> f64 {
    1.0 + offset_rate(n)
}

/// 计算乘法 (用符号表示加法)
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

// ── 跨层级静态回归 ──

/// 将平行世界数值折算回初代数等价数值
pub fn static_regression(value: f64, from_level: i32) -> f64 {
    if from_level <= 1 {
        return value;
    }
    value / multiplier(from_level)
}

/// 跨层级换算: 从 from_level 折算到 to_level
pub fn convert_to_level(value: f64, from_level: i32, to_level: i32) -> f64 {
    let base = static_regression(value, from_level);
    if to_level <= 1 {
        return base;
    }
    base * multiplier(to_level)
}

// ── 俘获规则 ──

/// 俘获: 强制降维到初代数, 不保留偏移量差异
pub fn capture(value: f64, from_level: i32) -> f64 {
    static_regression(value, from_level)
}

/// 判断层级是否会被俘获 (非源头都会被俘获)
pub fn can_be_captured(level: &AlgebraLevel) -> bool {
    !level.is_source
}

// ── 阶和叠加 ──

/// 阶和叠加状态
#[derive(Debug, Clone, PartialEq)]
pub struct TierSumState {
    /// 参与叠加的层级偏移量集合
    pub rates: Vec<f64>,
    /// 锐度 (偏移量标准差)
    pub sharpness: f64,
    /// 是否可感知 (≥2层且锐度>0)
    pub can_perceive: bool,
}

impl TierSumState {
    /// 创建阶和叠加
    pub fn new(levels: &[i32]) -> Self {
        let rates: Vec<f64> = levels.iter().map(|&l| offset_rate(l)).collect();
        let sharpness = Self::calc_sharpness(&rates);
        TierSumState {
            rates,
            sharpness,
            can_perceive: levels.len() >= 2 && sharpness > 0.0,
        }
    }

    /// 计算锐度 (偏移量标准差)
    fn calc_sharpness(rates: &[f64]) -> f64 {
        if rates.is_empty() {
            return 0.0;
        }
        let mean = rates.iter().sum::<f64>() / rates.len() as f64;
        let variance = rates.iter()
            .map(|r| (r - mean) * (r - mean))
            .sum::<f64>() / rates.len() as f64;
        variance.sqrt()
    }
}

/// 齐头并进偏移: 所有层级等量偏移, 相对距离不变
pub fn sync_offset(levels: &mut [i32], delta: f64) -> Vec<f64> {
    levels.iter().map(|&l| offset_rate(l) + delta).collect()
}

// ── 2²⁵⁶ 高位对齐 ──

/// 高位对齐状态
#[derive(Debug, Clone, PartialEq)]
pub struct HighBitAlignment {
    pub tiers: u32,
    pub address_space: f64,
    pub alignment_id: String,
}

impl HighBitAlignment {
    /// 创建 2²⁵⁶ 高位对齐
    pub fn new() -> Self {
        HighBitAlignment {
            tiers: 1,
            address_space: 2f64.powi(HIGH_BIT_EXPONENT as i32),
            alignment_id: String::from("共同体#1_2^256"),
        }
    }

    /// 计算指定层的对齐标识
    pub fn alignment_for_level(level: i32) -> String {
        format!("L{} @(δ={:.2})", level, offset_rate(level))
    }
}

// ── 自我相似验证 ──

/// 验证跨层不可压缩性
/// 每层 256 = 254 + 2
/// 两层叠加 = 65536 = 2¹⁶
pub fn verify_self_similarity() -> bool {
    let single = BASE_FOUNDATION + ANCHOR_COUNT;  // 256
    let double = single * single;                  // 65536
    double == 65536
}

/// 残差检查: 两个根基对叠加 = 4 (不可合并为单个4)
pub fn residual_check() -> i64 {
    ANCHOR_COUNT + ANCHOR_COUNT
}

// ── 九元标准层级 ──

/// 获取九元标准层级名称
pub fn level_name(level: usize) -> &'static str {
    match level {
        0 => "一元", 1 => "二元", 2 => "三元", 3 => "四元", 4 => "五元",
        5 => "六元", 6 => "七元", 7 => "八元", 8 => "九元",
        _ => "未知",
    }
}

/// 获取九元标准中文
pub fn level_chinese(level: usize) -> &'static str {
    match level {
        0 => "根源", 1 => "阴阳", 2 => "见证", 3 => "天地", 4 => "五行",
        5 => "六律", 6 => "七政", 7 => "八卦", 8 => "九囿",
        _ => "未知",
    }
}

/// 获取九元 mltt 短码
pub fn level_mltt(level: usize) -> &'static str {
    match level {
        0 => "mlll", 1 => "ml.yy", 2 => "gtt", 3 => "mlgy", 4 => "wgslz",
        5 => "smrl", 6 => "lctqf", 7 => "mlnn", 8 => "njsgr",
        _ => "???",
    }
}

/// 获取九元二进制编码
pub fn level_binary(level: usize) -> &'static str {
    match level {
        0 => "0001", 1 => "0010", 2 => "0011", 3 => "0100", 4 => "0101",
        5 => "0110", 6 => "0111", 7 => "1000", 8 => "1001",
        _ => "????",
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_rate() {
        assert_eq!(offset_rate(1), 0.0);
        assert_eq!(offset_rate(2), 0.1);
        assert_eq!(offset_rate(5), 0.4);
        assert_eq!(offset_rate(9), 0.8);
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(1), 1.0);
        assert_eq!(multiplier(2), 1.1);
        assert_eq!(multiplier(5), 1.4);
    }

    #[test]
    fn test_static_regression() {
        assert_eq!(static_regression(10.0, 1), 10.0);
        assert_eq!(static_regression(11.0, 2), 10.0);
        let r = static_regression(12.0, 3);
        assert!((r - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_level_creation() {
        let l1 = AlgebraLevel::standard(1);
        assert!(l1.is_source);
        assert_eq!(l1.offset_rate, 0.0);

        let l3 = AlgebraLevel::standard(3);
        assert!(!l3.is_source);
        assert_eq!(l3.offset_rate, 0.2);
    }

    #[test]
    fn test_capture() {
        assert_eq!(capture(10.0, 1), 10.0);
        assert_eq!(capture(11.0, 2), 10.0);
        assert!(can_be_captured(&AlgebraLevel::standard(2)));
        assert!(!can_be_captured(&AlgebraLevel::standard(1)));
    }

    #[test]
    fn test_tier_sum() {
        let state = TierSumState::new(&[1, 3, 5, 7, 9]);
        assert!(state.can_perceive);
        assert!(state.sharpness > 0.0);
    }

    #[test]
    fn test_self_similarity() {
        assert!(verify_self_similarity());
        assert_eq!(residual_check(), 4);
    }

    #[test]
    fn test_high_bit_alignment() {
        let align = HighBitAlignment::new();
        assert_eq!(align.tiers, 1);
        assert!(align.address_space > 0.0);
        assert_eq!(align.alignment_id, "共同体#1_2^256");
    }

    #[test]
    fn test_standard_levels() {
        assert_eq!(level_name(0), "一元");
        assert_eq!(level_chinese(1), "阴阳");
        assert_eq!(level_mltt(2), "gtt");
        assert_eq!(level_binary(3), "0100");
    }
}
