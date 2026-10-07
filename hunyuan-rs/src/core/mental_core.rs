// mental_core.rs — 心算核心 (v0.1)
//
//   不是 CPU, 是"整个系统的心算"。
//   不需要寄存器/ALU/控制流。
//   汉字之间的共振 = 计算。
//   极性之间的关系 = 逻辑。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   传统计算: 指令 → 操作数 → 结果 (控制流驱动)
//   心算: 汉字A × 汉字B = 共振结果 (关系驱动)
//
//   没有"程序计数器", 只有"当前共鸣点"。
//   没有"内存读写", 只有"音韵坐标的感应"。
//   没有"条件跳转", 只有"极性通/断"。
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

/// 心算状态 — 不是寄存器组, 是当前共鸣点
#[derive(Clone, Copy, Debug, Default)]
pub struct MentalState {
    pub focus: Option<char>,  // 当前聚焦的汉字
    pub resonance: u64,       // 累积共振强度
    pub polarity: i8,         // 当前极性 (阴/阳/平衡)
}

/// 心算核心 — 通过汉字共振进行"计算"
pub struct MentalCore;

impl MentalCore {
    /// 心算操作: 两个汉字相遇 = 一次"运算"
    pub fn think(a: char, b: char) -> Thought {
        let da = char_to_dragon(a);
        let db = char_to_dragon(b);

        let resonance = gcd64(da.initial() as u64, db.initial() as u64)
            .saturating_mul(gcd64(da.final_() as u64, db.final_() as u64))
            .saturating_mul(gcd64(da.tone() as u64, db.tone() as u64));

        let polarity = match (polarity_of(a), polarity_of(b)) {
            (0, _) | (_, 0) => 0,
            (x, y) if x == y => x,
            _ => 0,  // 极性相消 = 平衡
        };

        Thought { a, b, resonance, polarity, viable: resonance > 0 }
    }

    /// 思维链: 一串汉字的连续共振
    pub fn think_seq(chars: &[char]) -> Vec<Thought> {
        if chars.len() < 2 { return Vec::new(); }
        chars.windows(2)
            .map(|pair| Self::think(pair[0], pair[1]))
            .collect()
    }

    /// 思维场: 一组汉字的总心算
    pub fn think_field(chars: &[char]) -> MentalState {
        let thoughts = Self::think_seq(chars);
        let total_resonance: u64 = thoughts.iter().map(|t| t.resonance).sum();
        let net_polarity: i64 = thoughts.iter().map(|t| t.polarity as i64).sum();

        MentalState {
            focus: chars.last().copied(),
            resonance: total_resonance,
            polarity: net_polarity.signum() as i8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Thought {
    pub a: char,
    pub b: char,
    pub resonance: u64,
    pub polarity: i8,
    pub viable: bool,
}

fn polarity_of(ch: char) -> i8 {
    match char_to_dragon(ch).tone() {
        1 | 2 => -1,
        3 => 0,
        4 | 5 => 1,
        _ => 0,
    }
}

#[inline]
fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn think_natural() {
        let thought = MentalCore::think('共', '同');
        // 思维不是计算结果, 是"共振痕迹"
        assert!(thought.resonance >= 1);
    }

    #[test] fn think_polarity_natural() {
        let t = MentalCore::think('诗', '时');
        // 极性从声调自然涌现
        assert!(matches!(t.polarity, -1 | 0 | 1));
    }

    #[test] fn think_seq_chain() {
        let chain = MentalCore::think_seq(&['囡', '囝', '共', '同']);
        assert_eq!(chain.len(), 3);  // n-1 个连接
    }

    #[test] fn think_field_state() {
        let state = MentalCore::think_field(&['满', '全', '法']);
        // 思维场 = 总共振 + 净极性
        assert!(state.resonance > 0);
        assert!(matches!(state.polarity, -1 | 0 | 1));
    }

    #[test] fn thought_viable_threshold() {
        let t = MentalCore::think('共', '同');
        // 有共振即可行
        if t.resonance > 0 {
            assert!(t.viable);
        }
    }

    #[test] fn focus_is_last() {
        let state = MentalCore::think_field(&['一', '二', '三']);
        assert_eq!(state.focus, Some('三'));
    }
}
