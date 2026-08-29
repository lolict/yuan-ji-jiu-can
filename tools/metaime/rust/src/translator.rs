//! Translator · 外部符号→本体论路径翻译器
//! 将外部符号（如 Python 关键字、C 类型）翻译为本体论路径

use crate::constants::*;
use crate::linker::OntologyPath;

/// FQM 翻译器
pub struct FQMTranslator;

impl FQMTranslator {
    /// 创建翻译器
    pub const fn new() -> Self {
        FQMTranslator
    }

    /// 翻译外部符号为本体论路径
    pub fn translate(&self, symbol: &str) -> Option<OntologyPath> {
        let mut i = 0;
        while i < FOREIGN_MAP.len() {
            if FOREIGN_MAP[i].0 == symbol {
                return Some(OntologyPath::new(
                    "外部符号",
                    FOREIGN_MAP[i].0,
                    i,
                ));
            }
            i += 1;
        }
        None
    }

    /// 翻译并返回完整路径字符串
    pub fn translate_path(&self, symbol: &str) -> Option<&'static str> {
        let mut i = 0;
        while i < FOREIGN_MAP.len() {
            if FOREIGN_MAP[i].0 == symbol {
                return Some(FOREIGN_MAP[i].1);
            }
            i += 1;
        }
        None
    }

    /// 反向翻译：从本体论路径查找外部符号
    pub fn reverse_translate(&self, path: &str) -> Option<&'static str> {
        let mut i = 0;
        while i < FOREIGN_MAP.len() {
            if FOREIGN_MAP[i].1 == path {
                return Some(FOREIGN_MAP[i].0);
            }
            i += 1;
        }
        None
    }

    /// 判断符号是否在映射表中
    pub fn contains(&self, symbol: &str) -> bool {
        let mut i = 0;
        while i < FOREIGN_MAP.len() {
            if FOREIGN_MAP[i].0 == symbol {
                return true;
            }
            i += 1;
        }
        false
    }

    /// 获取映射表大小
    pub fn mapping_count(&self) -> usize {
        FOREIGN_MAP.len()
    }

    /// 批量翻译
    pub fn translate_batch(&self, symbols: &[&str]) -> [Option<OntologyPath>; MAX_SYMBOLS_PER_CONTAINER] {
        let mut result = [None; MAX_SYMBOLS_PER_CONTAINER];
        let mut i = 0;
        while i < symbols.len() && i < MAX_SYMBOLS_PER_CONTAINER {
            result[i] = self.translate(symbols[i]);
            i += 1;
        }
        result
    }
}

/// 静态翻译器实例
pub static DEFAULT_TRANSLATOR: FQMTranslator = FQMTranslator::new();
