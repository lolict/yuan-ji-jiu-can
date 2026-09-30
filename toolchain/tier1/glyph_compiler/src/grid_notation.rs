// 田字格坐标偏置记号 · 执行层
// 把记号 +1, -1, ±1, ±2, 1±.±2, ±1.-3 解析为地址的一部分

/// 偏置类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bias {
    Fine(i32),     // 格内微调 ±1
    Grid(i32),     // 格间（韵母维度 ±66）
    Row(i32),      // 跨行（声母维度 ±3960）
    Fam(i32),      // 跨族（组级别）
}

/// 从记号解出所有偏置步骤
/// 语法：
///   "+1"      → Fine(+1)    正向量级 1
///   "-1"      → Fine(-1)    负向量级 1
///   "±1"      → Fine(+1)    双向（取正向为默认）
///   "±2"      → Grid(+66)   双向格间
///   "±2.-3"   → [Grid(+66), Row(-3960)]  复合
///   "-1.2+"   → [Fine(-1), Grid(+132)]
pub fn decode(s: &str) -> Vec<Bias> {
    let mut out = Vec::new();
    for tok in s.split('.') {
        let tok = tok.trim();
        if tok.is_empty() { continue; }

        let is_bi = tok.contains('±');

        // 方向：检查是否有 - 号（单向负或双向尾部负）
        let has_neg = tok.contains('-');
        let has_pos = tok.contains('+');
        let sign: i32 = if has_neg && !has_pos { -1 } else { 1 };

        // 提取量级数字：过滤掉 ±+- 符号，只取数字
        let num_str: String = tok.chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let mag: i32 = num_str.parse().unwrap_or(1);

        // 量级含义：1=微调1级, 2=格间1级(66), 3=跨行1级(3960), 4=跨族1级
        // 注意：量级是层级编号，不是乘数；偏移量 = sign * 层级基值
        out.push(match mag {
            1 => Bias::Fine(sign),
            2 => Bias::Grid(sign * 66),
            3 => Bias::Row(sign * 3960),
            4 => Bias::Fam(sign),
            _ => Bias::Fine(sign),
        });
    }
    out
}

/// 把偏置应用到地址公式
pub fn apply(base: u32, biases: &[Bias]) -> i64 {
    let mut addr = base as i64;
    for b in biases {
        match b {
            Bias::Fine(d) => addr += *d as i64,
            Bias::Grid(d) => addr += *d as i64,
            Bias::Row(d) => addr += *d as i64,
            Bias::Fam(d) => addr += *d as i64,
        }
    }
    addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let b = decode("+1");
        assert_eq!(b, vec![Bias::Fine(1)]);

        let b = decode("-1");
        assert_eq!(b, vec![Bias::Fine(-1)]);

        let b = decode("±1");
        assert_eq!(b, vec![Bias::Fine(1)]);

        let b = decode("±2");
        assert_eq!(b, vec![Bias::Grid(66)]);

        let b = decode("±3");
        assert_eq!(b, vec![Bias::Row(3960)]);
    }

    #[test]
    fn test_compound() {
        // 1±.±2 = 微调±1(取正=+1) + 格间1级(+66)
        let b = decode("1±.±2");
        assert_eq!(b, vec![Bias::Fine(1), Bias::Grid(66)]);

        // ±1.-2 = 微调+1 + 格间1级负向(-66)
        let b = decode("±1.-2");
        assert_eq!(b, vec![Bias::Fine(1), Bias::Grid(-66)]);

        // -1.2+ = 微调-1 + 格间1级正向(+66)
        let b = decode("-1.2+");
        assert_eq!(b, vec![Bias::Fine(-1), Bias::Grid(66)]);
    }

    #[test]
    fn test_application() {
        let base = 0u32;

        // +1 → 微调+1
        let b = decode("+1");
        assert_eq!(apply(base, &b), 1);

        // ±2 → 格间1级 = 66
        let b = decode("±2");
        assert_eq!(apply(base, &b), 66);

        // 1±.±2 = +1 + 66 = 67
        let b = decode("1±.±2");
        assert_eq!(apply(base, &b), 67);

        // ±1.-3 = +1 - 3960 = -3959
        let b = decode("±1.-3");
        assert_eq!(apply(base, &b), -3959);

        // -1.2+ = -1 + 66 = 65
        let b = decode("-1.2+");
        assert_eq!(apply(base, &b), 65);
    }
}
