//! Linker · 本体论链接器
//! 按域.身份本体论索引

use crate::constants::*;

/// 本体论路径 · 域.身份
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OntologyPath {
    pub domain: &'static str,    // 域 (容器名)
    pub identity: &'static str,  // 身份 (符号名)
    pub position: usize,         // 位置坐标
}

impl OntologyPath {
    /// 创建新的本体论路径
    pub const fn new(domain: &'static str, identity: &'static str, position: usize) -> Self {
        OntologyPath {
            domain,
            identity,
            position,
        }
    }

    /// 创建空路径
    pub const fn empty() -> Self {
        OntologyPath {
            domain: "",
            identity: "",
            position: 0,
        }
    }

    /// 判断是否为空路径
    pub const fn is_empty(&self) -> bool {
        self.domain.is_empty()
    }
}

/// 链接记录
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkRecord {
    pub source: OntologyPath,
    pub target: OntologyPath,
    pub link_type: LinkType,
}

/// 链接类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    Synonym,     // 近义词
    Homophone,   // 同音字
    Antonym,     // 反义
    Derived,     // 派生
    Container,   // 容器归属
}

/// FQM 链接器
pub struct FQMLinker {
    /// 符号到路径的映射
    pub symbol_to_path: [(&'static str, OntologyPath); MAX_LINKS],
    pub mapping_count: usize,
    /// 已解析的链接
    pub resolved: [LinkRecord; MAX_LINKS],
    pub resolved_count: usize,
    /// 未解析的链接
    pub unresolved: [OntologyPath; MAX_LINKS],
    pub unresolved_count: usize,
}

impl FQMLinker {
    /// 创建空链接器
    pub const fn empty() -> Self {
        FQMLinker {
            symbol_to_path: [("", OntologyPath::empty()); MAX_LINKS],
            mapping_count: 0,
            resolved: [LinkRecord {
                source: OntologyPath::empty(),
                target: OntologyPath::empty(),
                link_type: LinkType::Synonym,
            }; MAX_LINKS],
            resolved_count: 0,
            unresolved: [OntologyPath::empty(); MAX_LINKS],
            unresolved_count: 0,
        }
    }

    /// 从加载器构建链接索引
    pub const fn from_loader() -> Self {
        let mut linker = FQMLinker::empty();

        // 声母映射
        let mut i = 0;
        while i < 24 {
            linker.symbol_to_path[i] = (
                SHENG_MU[i],
                OntologyPath::new("声母容器", SHENG_MU[i], i),
            );
            i += 1;
        }

        // 韵母映射
        let mut j = 0;
        while j < 24 {
            linker.symbol_to_path[24 + j] = (
                YUN_MU[j],
                OntologyPath::new("韵母容器", YUN_MU[j], j),
            );
            j += 1;
        }

        linker.mapping_count = 48;
        linker
    }

    /// 查找符号的本体论路径
    pub fn lookup(&self, symbol: &str) -> Option<OntologyPath> {
        let mut i = 0;
        while i < self.mapping_count {
            if self.symbol_to_path[i].0 == symbol {
                return Some(self.symbol_to_path[i].1);
            }
            i += 1;
        }
        None
    }

    /// 按域查找符号
    pub fn lookup_by_domain(&self, domain: &str, symbol: &str) -> Option<OntologyPath> {
        let mut i = 0;
        while i < self.mapping_count {
            let (sym, path) = &self.symbol_to_path[i];
            if *sym == symbol && path.domain == domain {
                return Some(*path);
            }
            i += 1;
        }
        None
    }

    /// 解析所有链接
    pub fn resolve(&mut self) -> Result<(), &'static str> {
        self.resolved_count = 0;
        self.unresolved_count = 0;

        let mut i = 0;
        while i < self.mapping_count {
            let (symbol, path) = self.symbol_to_path[i];
            if !symbol.is_empty() && !path.is_empty() {
                // 添加到已解析
                if self.resolved_count < MAX_LINKS {
                    self.resolved[self.resolved_count] = LinkRecord {
                        source: path,
                        target: path,
                        link_type: LinkType::Container,
                    };
                    self.resolved_count += 1;
                }
            } else {
                // 添加到未解析
                if self.unresolved_count < MAX_LINKS {
                    self.unresolved[self.unresolved_count] = path;
                    self.unresolved_count += 1;
                }
            }
            i += 1;
        }

        Ok(())
    }

    /// 获取指定域的所有路径
    pub fn get_domain_paths(&self, domain: &str) -> [OntologyPath; MAX_LINKS] {
        let mut result = [OntologyPath::empty(); MAX_LINKS];
        let mut count = 0;
        let mut i = 0;
        while i < self.mapping_count {
            if self.symbol_to_path[i].1.domain == domain {
                if count < MAX_LINKS {
                    result[count] = self.symbol_to_path[i].1;
                    count += 1;
                }
            }
            i += 1;
        }
        result
    }

    /// 获取已解析链接数量
    pub fn resolved_count(&self) -> usize {
        self.resolved_count
    }

    /// 获取未解析链接数量
    pub fn unresolved_count(&self) -> usize {
        self.unresolved_count
    }
}

/// 静态默认链接器
pub static DEFAULT_LINKER: FQMLinker = FQMLinker::from_loader();
