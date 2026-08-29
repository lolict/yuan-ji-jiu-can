//! Interpreter · 容器操作解释器
//! 装入/取出/比较

use crate::constants::*;
use crate::linker::{OntologyPath, FQMLinker};

/// 操作结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpResult {
    Success,
    DomainNotFound,
    SymbolNotFound,
    CapacityFull,
    NotBound,
}

/// FQM 解释器
pub struct FQMInterpreter {
    pub loader: FQMLinker,
    /// 内存中的符号: 每个域最多64个符号
    pub memory: [(&'static str, &'static str); MAX_SYMBOLS_PER_CONTAINER],
    pub memory_count: usize,
}

impl FQMInterpreter {
    /// 创建空解释器
    pub const fn empty() -> Self {
        FQMInterpreter {
            loader: FQMLinker::from_loader(),
            memory: [("", ""); MAX_SYMBOLS_PER_CONTAINER],
            memory_count: 0,
        }
    }

    /// 创建带默认声母/韵母的解释器
    pub const fn with_defaults() -> Self {
        FQMInterpreter {
            loader: FQMLinker::from_loader(),
            memory: [("", ""); MAX_SYMBOLS_PER_CONTAINER],
            memory_count: 0,
        }
    }

    /// 装入操作: 将符号装入指定域
    pub fn load(&mut self, domain: &'static str, symbol: &'static str) -> Result<(), OpResult> {
        // 查找符号是否存在
        let path = self.loader.lookup(symbol).ok_or(OpResult::SymbolNotFound)?;

        // 验证域匹配
        if path.domain != domain && !domain.is_empty() {
            return Err(OpResult::DomainNotFound);
        }

        // 装入内存
        if self.memory_count >= MAX_SYMBOLS_PER_CONTAINER {
            return Err(OpResult::CapacityFull);
        }

        self.memory[self.memory_count] = (domain, symbol);
        self.memory_count += 1;
        Ok(())
    }

    /// 取出操作: 从指定域取出符号
    pub fn extract(&self, domain: &str, identity: &str) -> Result<&'static str, OpResult> {
        // 在内存中查找
        let mut i = 0;
        while i < self.memory_count {
            let (d, s) = self.memory[i];
            if (domain.is_empty() || d == domain) && s == identity {
                return Ok(s);
            }
            i += 1;
        }

        // 在链接器中查找
        if let Some(path) = self.loader.lookup_by_domain(domain, identity) {
            return Ok(path.identity);
        }

        Err(OpResult::SymbolNotFound)
    }

    /// 比较操作: 比较两个符号是否在同一个容器
    pub fn compare(&self, sym1: &str, sym2: &str) -> bool {
        let path1 = self.loader.lookup(sym1);
        let path2 = self.loader.lookup(sym2);

        match (path1, path2) {
            (Some(p1), Some(p2)) => p1.domain == p2.domain,
            _ => false,
        }
    }

    /// 比较两个符号的位置是否相邻
    pub fn compare_adjacent(&self, sym1: &str, sym2: &str) -> bool {
        let path1 = self.loader.lookup(sym1);
        let path2 = self.loader.lookup(sym2);

        match (path1, path2) {
            (Some(p1), Some(p2)) => {
                if p1.domain == p2.domain {
                    let diff = if p1.position > p2.position {
                        p1.position - p2.position
                    } else {
                        p2.position - p1.position
                    };
                    diff == 1
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// 绑定操作: 将符号绑定到指定路径
    pub fn bind(&mut self, _domain: &str, symbol: &'static str, path: OntologyPath) -> Result<(), OpResult> {
        if self.loader.mapping_count >= MAX_LINKS {
            return Err(OpResult::CapacityFull);
        }
        self.loader.symbol_to_path[self.loader.mapping_count] = (symbol, path);
        self.loader.mapping_count += 1;
        Ok(())
    }

    /// 解除绑定
    pub fn unbind(&mut self, symbol: &str) -> Result<(), OpResult> {
        let mut found = false;
        let mut i = 0;
        while i < self.loader.mapping_count {
            if self.loader.symbol_to_path[i].0 == symbol {
                found = true;
                // 移动后续元素
                let mut j = i;
                while j < self.loader.mapping_count - 1 {
                    self.loader.symbol_to_path[j] = self.loader.symbol_to_path[j + 1];
                    j += 1;
                }
                self.loader.mapping_count -= 1;
                break;
            }
            i += 1;
        }
        if found {
            Ok(())
        } else {
            Err(OpResult::NotBound)
        }
    }

    /// 查找符号的链接
    pub fn link(&self, symbol: &str) -> Option<OntologyPath> {
        self.loader.lookup(symbol)
    }

    /// 按域批量取出
    pub fn extract_domain(&self, domain: &str) -> [(&'static str, &'static str); MAX_SYMBOLS_PER_CONTAINER] {
        let mut result = [("", ""); MAX_SYMBOLS_PER_CONTAINER];
        let mut count = 0;
        let mut i = 0;
        while i < self.memory_count && count < MAX_SYMBOLS_PER_CONTAINER {
            if self.memory[i].0 == domain {
                result[count] = self.memory[i];
                count += 1;
            }
            i += 1;
        }
        result
    }

    /// 清除内存
    pub fn clear_memory(&mut self) {
        self.memory_count = 0;
    }

    /// 获取内存使用量
    pub fn memory_usage(&self) -> usize {
        self.memory_count
    }
}

/// 静态默认解释器
pub static DEFAULT_INTERPRETER: FQMInterpreter = FQMInterpreter::with_defaults();
