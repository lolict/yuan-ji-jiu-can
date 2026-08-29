//! Divisor · 公约数公倍数关系裁定解释器
//! 关系裁定 · 整除 · 公倍数

use crate::constants::*;

/// 整除关系结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Divisibility {
    Divides,       // a 整除 b
    Multiple,      // a 是 b 的倍数
    Coprime,       // 互质
    Identical,     // 相等
}

/// 公约数公倍数解释器
pub struct DivisorInterpreter;

impl DivisorInterpreter {
    /// 创建解释器
    pub const fn new() -> Self {
        DivisorInterpreter
    }

    /// 计算最大公约数 (编译时 const fn)
    pub const fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    /// 计算最小公倍数
    pub const fn lcm(a: usize, b: usize) -> usize {
        if a == 0 || b == 0 {
            0
        } else {
            a / Self::gcd(a, b) * b
        }
    }

    /// 判断整除关系
    pub const fn relation(a: usize, b: usize) -> Divisibility {
        if a == b {
            Divisibility::Identical
        } else if a == 0 || b == 0 {
            Divisibility::Coprime
        } else if b % a == 0 {
            Divisibility::Divides
        } else if a % b == 0 {
            Divisibility::Multiple
        } else if Self::gcd(a, b) == 1 {
            Divisibility::Coprime
        } else {
            // 有公因数但非整除
            Divisibility::Coprime
        }
    }

    /// 判断是否为公倍数关系
    pub const fn same_divisor(a: usize, b: usize) -> bool {
        Self::gcd(a, b) > 1
    }

    /// 获取所有公因数表（编译时常量）
    pub const fn common_divisors(a: usize, b: usize) -> [usize; 12] {
        let mut result = [0usize; 12];
        let g = Self::gcd(a, b);
        if g == 0 {
            return result;
        }

        let mut count = 0;
        let mut i = 1;
        while i <= g && count < 12 {
            if g % i == 0 {
                result[count] = i;
                count += 1;
            }
            i += 1;
        }
        result
    }

    /// 在声母序列中查找具有公因数关系的符号对
    pub fn find_divisible_pairs() -> [((&'static str, usize), (&'static str, usize)); 24] {
        let mut result = [(("", 0), ("", 0)); 24];
        let mut count = 0;

        let mut i = 0;
        while i < 24 && count < 24 {
            let mut j = i + 1;
            while j < 24 && count < 24 {
                if Self::same_divisor(i, j) {
                    result[count] = ((SHENG_MU[i], i), (SHENG_MU[j], j));
                    count += 1;
                }
                j += 1;
            }
            i += 1;
        }

        result
    }

    /// 在韵母序列中查找具有公因数关系的符号对
    pub fn find_divisible_pairs_yun() -> [((&'static str, usize), (&'static str, usize)); 24] {
        let mut result = [(("", 0), ("", 0)); 24];
        let mut count = 0;

        let mut i = 0;
        while i < 24 && count < 24 {
            let mut j = i + 1;
            while j < 24 && count < 24 {
                if Self::same_divisor(i, j) {
                    result[count] = ((YUN_MU[i], i), (YUN_MU[j], j));
                    count += 1;
                }
                j += 1;
            }
            i += 1;
        }

        result
    }

    /// 公约数关系表: 预计算所有声母对的 gcd
    pub fn gcd_matrix() -> [[usize; 24]; 24] {
        let mut result = [[0usize; 24]; 24];
        let mut i = 0;
        while i < 24 {
            let mut j = 0;
            while j < 24 {
                result[i][j] = Self::gcd(i, j);
                j += 1;
            }
            i += 1;
        }
        result
    }

    /// 判断两个位置是否共享公约数
    pub const fn share_divisor(pos1: usize, pos2: usize) -> bool {
        Self::gcd(pos1, pos2) > 1
    }

    /// 获取最大公约数链
    pub const fn divisor_chain(n: usize) -> [usize; 12] {
        let mut result = [0usize; 12];
        let mut count = 0;
        let mut i = 1;
        while i <= n && count < 12 {
            if n % i == 0 {
                result[count] = i;
                count += 1;
            }
            i += 1;
        }
        result
    }
}

/// 静态解释器实例
pub static DEFAULT_DIVISOR: DivisorInterpreter = DivisorInterpreter::new();
