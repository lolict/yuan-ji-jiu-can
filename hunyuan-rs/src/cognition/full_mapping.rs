// ══════════════════════════════════════════════════════════════════
// full_mapping.rs — 全映射引擎 · 五系统互联
// ══════════════════════════════════════════════════════════════════
//
// 打通: 九元层级 ↔ YinYang极性 ↔ 二进制 ↔ mltt短码 ↔ 高维偏移率
//
// 任意层级可以有五种等价表示:
//   层级名 / 中文名 / 偏移率 / 乘数 / mltt短码 / 二进制编码

pub use crate::high_algebra::{
    offset_rate, multiplier, level_name, level_chinese, level_mltt, level_binary,
    static_regression, convert_to_level, AlgebraLevel, HighBitAlignment,
    BASE_RADIX, MAX_LEVELS,
};

/// 极性枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// 阴 · 隐藏 · 容器侧 · 被动
    Yin = 0,
    /// 阳 · 显现 · 界线侧 · 主动
    Yang = 1,
    /// 平衡 · 纠缠 · 叠加态
    Balanced = 2,
}

impl Polarity {
    /// 从层级编号计算极性 (偶数层=阳, 奇数层=阴, 9层=平衡)
    pub fn from_level(level: i32) -> Self {
        if level == 8 {
            Polarity::Balanced
        } else if level % 2 == 0 {
            Polarity::Yang
        } else {
            Polarity::Yin
        }
    }

    /// 极性转 YinYang 字符串
    pub fn to_yinyang(&self) -> &'static str {
        match self {
            Polarity::Yin => "阴",
            Polarity::Yang => "阳",
            Polarity::Balanced => "平衡",
        }
    }

    /// 极性转二进制字符
    pub fn to_binary(&self) -> &'static str {
        match self {
            Polarity::Yin => "0",
            Polarity::Yang => "1",
            Polarity::Balanced => "0.5",
        }
    }
}

/// 全映射条目 (一个层级的五种等价表示)
#[derive(Debug, Clone, PartialEq)]
pub struct MappingEntry {
    pub level: i32,
    pub level_name: String,
    pub chinese_name: String,
    pub offset_rate: f64,
    pub multiplier: f64,
    pub polarity: Polarity,
    pub yinyang: String,
    pub binary_string: String,
    pub mltt_code: String,
    pub base10_value: i32,
}

impl MappingEntry {
    /// 创建某层级的完整映射
    pub fn new(level: i32, desc: &str) -> Self {
        let lvl_idx = level as usize;
        let pol = Polarity::from_level(level);
        MappingEntry {
            level,
            level_name: level_name(lvl_idx).to_string(),
            chinese_name: level_chinese(lvl_idx).to_string(),
            offset_rate: offset_rate(level),
            multiplier: multiplier(level),
            polarity: pol,
            yinyang: pol.to_yinyang().to_string(),
            binary_string: level_binary(lvl_idx).to_string(),
            mltt_code: level_mltt(lvl_idx).to_string(),
            base10_value: level + 1,
        }
    }

    /// 简要一行描述
    pub fn summary(&self) -> String {
        format!(
            "{} {} δ={:.2} M={:.2} {} {} {}",
            self.level_name, self.chinese_name,
            self.offset_rate, self.multiplier,
            self.yinyang, self.binary_string, self.mltt_code
        )
    }
}

/// 完整九层映射报告
#[derive(Debug, Clone, PartialEq)]
pub struct FullMapReport {
    pub symbol: String,
    pub entries: Vec<MappingEntry>,
}

impl FullMapReport {
    /// 生成九层完整映射
    pub fn new(symbol: &str) -> Self {
        let entries: Vec<MappingEntry> = (0..MAX_LEVELS as i32)
            .map(|lvl| MappingEntry::new(lvl, symbol))
            .collect();
        FullMapReport {
            symbol: symbol.to_string(),
            entries,
        }
    }

    /// 打印层级表
    pub fn print_table(&self) {
        println!("符号: {}", self.symbol);
        for e in &self.entries {
            println!("  [{}] {}", e.level, e.summary());
        }
    }
}

/// JSON 格式的文档极性
#[derive(Debug, Clone)]
pub struct DocPolarity {
    pub path: String,
    pub polarity: i32,  // -1, 0, 1
    pub weight: f64,
    pub size: usize,
}

impl DocPolarity {
    pub fn new(path: &str, polarity: i32, weight: f64, size: usize) -> Self {
        DocPolarity {
            path: path.to_string(),
            polarity,
            weight,
            size,
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polarity_from_level() {
        assert_eq!(Polarity::from_level(0), Polarity::Yang);   // 偶数层 = 阳
        assert_eq!(Polarity::from_level(1), Polarity::Yin);    // 奇数层 = 阴
        assert_eq!(Polarity::from_level(2), Polarity::Yang);
        assert_eq!(Polarity::from_level(8), Polarity::Balanced); // 九层 = 平衡
    }

    #[test]
    fn test_polarity_yinyang() {
        assert_eq!(Polarity::Yin.to_yinyang(), "阴");
        assert_eq!(Polarity::Yang.to_yinyang(), "阳");
        assert_eq!(Polarity::Balanced.to_yinyang(), "平衡");
    }

    #[test]
    fn test_polarity_binary() {
        assert_eq!(Polarity::Yin.to_binary(), "0");
        assert_eq!(Polarity::Yang.to_binary(), "1");
        assert_eq!(Polarity::Balanced.to_binary(), "0.5");
    }

    #[test]
    fn test_mapping_entry() {
        let entry = MappingEntry::new(0, "test");
        assert_eq!(entry.level_name, "一元");
        assert_eq!(entry.chinese_name, "根源");
        assert_eq!(entry.mltt_code, "mlll");
        assert_eq!(entry.binary_string, "0001");
    }

    #[test]
    fn test_full_map_report() {
        let report = FullMapReport::new("满全法");
        assert_eq!(report.entries.len(), 9);
        assert_eq!(report.entries[0].level_name, "一元");
        assert_eq!(report.entries[8].level_name, "九元");
    }

    #[test]
    fn test_doc_polarity() {
        let doc = DocPolarity::new("test.md", 1, 42.0, 100);
        assert_eq!(doc.polarity, 1);
        assert_eq!(doc.size, 100);
    }
}
