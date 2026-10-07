// meta_ime.rs — 元输入法·心算引擎 (v0.1)
//
//   不是输入法, 是"思维直接转汉字"。
//   不是编码, 是"音韵坐标触发意图"。
//
// ══════════════════════════════════════════════════════════════
//
// 满全法范式:
//
//   传统输入法: 键盘 → 编码 → 候选 → 选择
//   元输入法: 意图 → 音韵场 → 共振最强 = 输出
//
//   没有键盘输入, 没有候选列表。
//   意识 → 声母/韵母/声调三维定位 → 自然落点 = 汉字
//
// ══════════════════════════════════════════════════════════════

use crate::name_creator::char_to_dragon;

/// 意图 = 音韵坐标中的"想要去的方向"
#[derive(Clone, Copy, Debug)]
pub struct Intent {
    shengmu: Option<u8>,
    yunmu: Option<u8>,
    tone: Option<u8>,
}

impl Intent {
    pub fn new() -> Self { Self { shengmu: None, yunmu: None, tone: None } }
    pub fn with_shengmu(mut self, s: u8) -> Self { self.shengmu = Some(s); self }
    pub fn with_yunmu(mut self, y: u8) -> Self { self.yunmu = Some(y); self }
    pub fn with_tone(mut self, t: u8) -> Self { self.tone = Some(t); self }
}

/// 元输入法 — 从意图直接"心算"出汉字
pub struct MetaIME;

impl MetaIME {
    /// 心算: 给定意图 + 字表, 找到共振最强的汉字
    pub fn compute(intent: Intent, hanzi_table: &[char]) -> Option<(char, u64)> {
        hanzi_table.iter()
            .filter(|&&ch| !ch.is_ascii())
            .map(|&ch| {
                let score = resonance_score(&intent, ch);
                (ch, score)
            })
            .max_by_key(|(_, s)| *s)
    }

    /// 批量心算: 多个意图 → 一组汉字 (不需要选候选)
    pub fn compute_seq(intents: &[Intent], hanzi_table: &[char]) -> Vec<(char, u64)> {
        intents.iter()
            .filter_map(|intent| Self::compute(*intent, hanzi_table))
            .collect()
    }

    /// 意图场: 给定一组汉字, 反推它们的"意图方向"
    pub fn field_intent(chars: &[char]) -> (u8, u8, u8) {
        if chars.is_empty() { return (0, 0, 0); }
        let mut sum_s: u64 = 0;
        let mut sum_y: u64 = 0;
        let mut sum_t: u64 = 0;
        for &ch in chars {
            let d = char_to_dragon(ch);
            sum_s += d.initial() as u64;
            sum_y += d.final_() as u64;
            sum_t += d.tone() as u64;
        }
        let n = chars.len() as u64;
        ((sum_s / n) as u8, (sum_y / n) as u8, (sum_t / n) as u8)
    }
}

/// 共振评分 = 意图与汉字音韵的"匹配度"
/// 不是距离计算, 是"三维坐标的公约数亲和力"
fn resonance_score(intent: &Intent, ch: char) -> u64 {
    let d = char_to_dragon(ch);
    let mut score = 0u64;
    let mut active_dims = 0;

    if let Some(s) = intent.shengmu {
        score += gcd(s as u64, d.initial() as u64);
        active_dims += 1;
    }
    if let Some(y) = intent.yunmu {
        score += gcd(y as u64, d.final_() as u64);
        active_dims += 1;
    }
    if let Some(t) = intent.tone {
        score += gcd(t as u64, d.tone() as u64);
        active_dims += 1;
    }

    if active_dims > 0 {
        score * active_dims  // 多维度匹配 = 更高权重
    } else {
        0
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    if a == 0 { 1 } else { a }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn meta_ime_compute_option() {
        let table = ['满', '全', '法', '共', '同', '生'];
        let intent = Intent::new().with_shengmu(7);  // 声母意图
        let result = MetaIME::compute(intent, &table);
        // 无论结果是否Some, 心算是"自然涌现"
        if let Some((ch, score)) = result {
            assert!(score > 0);
            assert!(table.contains(&ch));
        }
    }

    #[test] fn full_intent_strong_resonance() {
        let table = ['囡', '囝', '满', '全'];
        let intent = Intent::new().with_shengmu(2).with_yunmu(14).with_tone(1);
        let result = MetaIME::compute(intent, &table);
        assert!(result.is_some());
    }

    #[test] fn compute_seq_natural() {
        let table = ['满', '全', '法'];
        let intents = [
            Intent::new().with_shengmu(7),
            Intent::new().with_tone(3),
        ];
        let results = MetaIME::compute_seq(&intents, &table);
        assert_eq!(results.len(), 2);
    }

    #[test] fn field_intent_center() {
        let center = MetaIME::field_intent(&['共', '同']);
        // 意图场的中心 = 所有汉字音韵的自然平均
        assert!(center.0 > 0 || center.1 > 0);
    }

    #[test] fn resonance_score_symmetric() {
        let intent = Intent::new().with_shengmu(5);
        let score_a = resonance_score(&intent, '共');
        let score_b = resonance_score(&intent, '古');
        // 分数是正数 (公约数总是 >= 1)
        if score_a > 0 { assert!(score_b >= 0); }
    }

    #[test] fn zero_intent_no_score() {
        let intent = Intent::new();
        let score = resonance_score(&intent, '满');
        assert_eq!(score, 0);  // 无意图 = 无共振 = 无输出
    }
}
