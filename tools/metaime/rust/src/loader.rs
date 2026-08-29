//! Loader · FQM 容器定义文件加载器
//! 加载 .fqm 容器定义文件 · 发音 · 共振 · 记忆的容器

use crate::constants::*;

/// 容器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    ShengMu,  // 声母
    YunMu,    // 韵母
    HanZi,    // 汉字
    Custom,   // 自定义
}

/// 容器定义
#[derive(Debug, Clone)]
pub struct Container {
    pub name: &'static str,
    pub container_type: ContainerType,
    pub capacity: usize,
    pub symbols: [&'static str; MAX_SYMBOLS_PER_CONTAINER],
    pub symbol_count: usize,
    pub positions: [usize; MAX_SYMBOLS_PER_CONTAINER],
    /// 近义词矩阵: [符号索引][变体索引] = 变体字符串
    pub synonyms: [[&'static str; MAX_VARIANTS]; MAX_SYMBOLS_PER_CONTAINER],
    pub synonym_counts: [usize; MAX_SYMBOLS_PER_CONTAINER],
}

impl Container {
    /// 创建空容器
    pub const fn empty() -> Self {
        Container {
            name: "",
            container_type: ContainerType::Custom,
            capacity: 64,
            symbols: [""; MAX_SYMBOLS_PER_CONTAINER],
            symbol_count: 0,
            positions: [0; MAX_SYMBOLS_PER_CONTAINER],
            synonyms: [[""; MAX_VARIANTS]; MAX_SYMBOLS_PER_CONTAINER],
            synonym_counts: [0; MAX_SYMBOLS_PER_CONTAINER],
        }
    }

    /// 编译时创建声母容器 (24位)
    pub const fn shengmu() -> Self {
        let mut symbols = [""; MAX_SYMBOLS_PER_CONTAINER];
        let mut positions = [0usize; MAX_SYMBOLS_PER_CONTAINER];
        let mut i = 0;
        while i < 24 {
            symbols[i] = SHENG_MU[i];
            positions[i] = i;
            i += 1;
        }
        Container {
            name: "声母容器",
            container_type: ContainerType::ShengMu,
            capacity: 64,
            symbols,
            symbol_count: 24,
            positions,
            synonyms: [[""; MAX_VARIANTS]; MAX_SYMBOLS_PER_CONTAINER],
            synonym_counts: [0; MAX_SYMBOLS_PER_CONTAINER],
        }
    }

    /// 编译时创建韵母容器 (24位)
    pub const fn yunmu() -> Self {
        let mut symbols = [""; MAX_SYMBOLS_PER_CONTAINER];
        let mut positions = [0usize; MAX_SYMBOLS_PER_CONTAINER];
        let mut i = 0;
        while i < 24 {
            symbols[i] = YUN_MU[i];
            positions[i] = i;
            i += 1;
        }
        Container {
            name: "韵母容器",
            container_type: ContainerType::YunMu,
            capacity: 64,
            symbols,
            symbol_count: 24,
            positions,
            synonyms: [[""; MAX_VARIANTS]; MAX_SYMBOLS_PER_CONTAINER],
            synonym_counts: [0; MAX_SYMBOLS_PER_CONTAINER],
        }
    }

    /// 查找符号的索引位置
    pub fn find_symbol_index(&self, symbol: &str) -> Option<usize> {
        let mut i = 0;
        while i < self.symbol_count {
            if self.symbols[i] == symbol {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 查找符号的本体论路径
    pub fn find_path(&self, symbol: &str) -> Option<(&str, usize)> {
        let idx = self.find_symbol_index(symbol)?;
        Some((self.symbols[idx], self.positions[idx]))
    }

    /// 获取指定位置的近义词列表
    pub fn get_synonyms(&self, index: usize) -> &[&'static str] {
        if index >= self.symbol_count {
            return &[];
        }
        &self.synonyms[index][..self.synonym_counts[index]]
    }
}

/// FQM 文件加载器
pub struct FQMLoader {
    pub containers: [Container; MAX_CONTAINERS],
    pub container_count: usize,
    /// 链接符号: [索引][0]=域, [索引][1]=符号
    pub links: [(&'static str, &'static str); MAX_LINKS],
    pub link_count: usize,
    /// 近义词池
    pub synonym_pool: [&'static str; MAX_SYNONYMS],
    pub synonym_count: usize,
}

impl FQMLoader {
    /// 创建空的加载器
    pub const fn empty() -> Self {
        FQMLoader {
            containers: [const { Container::empty() }; MAX_CONTAINERS],
            container_count: 0,
            links: [("", ""); MAX_LINKS],
            link_count: 0,
            synonym_pool: [""; MAX_SYNONYMS],
            synonym_count: 0,
        }
    }

    /// 创建带默认声母/韵母容器的加载器
    pub const fn with_defaults() -> Self {
        let mut containers = [const { Container::empty() }; MAX_CONTAINERS];
        containers[0] = Container::shengmu();
        containers[1] = Container::yunmu();
        FQMLoader {
            containers,
            container_count: 2,
            links: [("", ""); MAX_LINKS],
            link_count: 0,
            synonym_pool: [""; MAX_SYNONYMS],
            synonym_count: 0,
        }
    }

    /// 获取指定名称的容器
    pub fn get_container(&self, name: &str) -> Option<&Container> {
        let mut i = 0;
        while i < self.container_count {
            if self.containers[i].name == name {
                return Some(&self.containers[i]);
            }
            i += 1;
        }
        None
    }

    /// 获取指定类型的容器
    pub fn get_container_by_type(&self, ctype: ContainerType) -> Option<&Container> {
        let mut i = 0;
        while i < self.container_count {
            if self.containers[i].container_type == ctype {
                return Some(&self.containers[i]);
            }
            i += 1;
        }
        None
    }

    /// 查找符号所在的容器
    pub fn find_symbol_container(&self, symbol: &str) -> Option<&Container> {
        let mut i = 0;
        while i < self.container_count {
            if self.containers[i].find_symbol_index(symbol).is_some() {
                return Some(&self.containers[i]);
            }
            i += 1;
        }
        None
    }

    /// 获取声母的位置坐标 (编译时已知)
    pub const fn shengmu_position(symbol: &str) -> Option<usize> {
        let mut i = 0;
        while i < 24 {
            if const_str_eq(SHENG_MU[i], symbol) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 获取韵母的位置坐标 (编译时已知)
    pub const fn yunmu_position(symbol: &str) -> Option<usize> {
        let mut i = 0;
        while i < 24 {
            if const_str_eq(YUN_MU[i], symbol) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    /// 获取声调的位置坐标
    pub const fn tone_position(tone: &str) -> Option<usize> {
        let mut i = 0;
        while i < 4 {
            if const_str_eq(TONES[i], tone) {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

/// 编译时字符串相等比较
pub const fn const_str_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    if a_bytes.len() != b_bytes.len() {
        return false;
    }
    let mut i = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// 静态 FQM 容器定义 (编译时嵌入)
pub static DEFAULT_SHENG_MU: Container = Container::shengmu();
pub static DEFAULT_YUN_MU: Container = Container::yunmu();
