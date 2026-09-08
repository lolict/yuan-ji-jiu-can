// 组合层 — 把词汇串联成句子
// 词汇 = 一条指令 = 一个可执行的积木
// 句子 = 一段可连续执行的程序 = 词汇的加法拼接

use crate::cross_isa::vocabulary::*;

/// 句子：词汇的有序序列
#[derive(Debug, Clone)]
pub struct Sentence {
    pub words: Vec<VocabWord>,
}

impl Sentence {
    pub fn new() -> Self { Self { words: Vec::new() } }

    /// 加法拼接：把一个词汇追加到句尾
    pub fn append(&mut self, word: VocabWord) {
        self.words.push(word);
    }

    /// 拼接另一个句子
    pub fn extend(&mut self, other: Sentence) {
        self.words.extend(other.words);
    }

    /// 计算句子在所有平台的总字节数
    pub fn total_bytes(&self) -> usize {
        self.words.iter().map(|w| w.native_encoding.len()).sum()
    }

    /// 获取指定平台的全部原生编码（已拼接）
    pub fn native_bytes(&self, platform: Platform) -> Vec<u8> {
        let mut out = Vec::new();
        for w in &self.words {
            if w.platform == platform {
                out.extend_from_slice(&w.native_encoding);
            }
        }
        out
    }

    /// 句子的长度（词汇数）
    pub fn len(&self) -> usize { self.words.len() }
    pub fn is_empty(&self) -> bool { self.words.is_empty() }
}

impl Default for Sentence {
    fn default() -> Self { Self::new() }
}

/// 便捷宏：构建一条句子
#[macro_export]
macro_rules! sentence {
    ($($sid:ident => [$($op:expr),*] => $plat:ident),* $(,)?) => {
        {
            let mut s = Sentence::new();
            $(
                s.append(VocabWord::new(
                    SemanticId::$sid,
                    vec![$($op),*],
                    Platform::$plat,
                ));
            )*
            s
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_sentence() {
        let mut s = Sentence::new();
        s.append(VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(1), Operand::Reg(0), Operand::Imm(10)],
            Platform::RiscV,
        ));
        s.append(VocabWord::new(
            SemanticId::Addi,
            vec![Operand::Reg(2), Operand::Reg(0), Operand::Imm(20)],
            Platform::RiscV,
        ));
        s.append(VocabWord::new(
            SemanticId::Add,
            vec![Operand::Reg(3), Operand::Reg(1), Operand::Reg(2)],
            Platform::RiscV,
        ));
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn macro_builds() {
        let _s = sentence!(
            Addi => [Operand::Reg(1), Operand::Reg(0), Operand::Imm(10)] => RiscV,
            Addi => [Operand::Reg(2), Operand::Reg(0), Operand::Imm(20)] => RiscV,
            Add => [Operand::Reg(3), Operand::Reg(1), Operand::Reg(2)] => RiscV,
        );
        assert_eq!(_s.len(), 3);
    }
}
