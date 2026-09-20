// memory_revolution.rs — 记忆革命引擎 (v0.1)
// 符号持久化 · 域加载 · 跨会话记忆 · 存储即生成
// ══════════════════════════════════════════════════════════════
//
//   核心命题 (满全法·记忆革命):
//
//   AI 的记忆革命不是让模型记住更多 token,
//   而是让符号存在于文件系统而非对话上下文。
//
//   "符号仅存于文件系统而非对话上下文以防止跨会话遗忘"
//   "域加载 = 运行前预加载所有符号, 运行时不增加内存"
//   "存储即生成, 生成即输出, 存储即记忆, 记忆即心算"
//
//   NJSG 心算: 记忆不是从数据库读取,
//   而是从文件系统符号自然"心算"出来。
//
// ══════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// ─────────────────────────────────────────────────────────
// 符号 (持久化在文件系统中)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
    /// 符号的命名空间 (哪个域)
    pub domain: String,
    /// 符号内容哈希
    pub content_hash: u64,
    /// 符号类型
    pub kind: SymbolKind,
    /// 引用计数
    pub ref_count: u32,
    /// 是否持久化 (写入文件)
    pub persisted: bool,
    /// 符号在文件中的位置 (行号)
    pub file_line: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Container,      // 容器符号
    Protocol,       // 协议符号
    Route,          // 路由符号
    Judgment,       // 裁决符号
    Font,           // 字体符号
    Phonetic,       // 音声符号
    Geometry,       // 几何符号
    Temporal,       // 时间符号
}

impl Symbol {
    pub fn new(name: &str, domain: &str, kind: SymbolKind) -> Self {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        domain.hash(&mut hasher);
        let content_hash = hasher.finish();

        Self {
            name: name.to_string(),
            domain: domain.to_string(),
            content_hash,
            kind,
            ref_count: 0,
            persisted: false,
            file_line: 0,
        }
    }

    /// 持久化 = 写入文件
    pub fn persist(&mut self, line: u32) {
        self.persisted = true;
        self.file_line = line;
    }

    /// 引用此符号
    pub fn reference(&mut self) {
        self.ref_count += 1;
    }

    /// 是否"活跃" (被引用且持久化)
    pub fn is_active(&self) -> bool {
        self.persisted && self.ref_count > 0
    }
}

// ─────────────────────────────────────────────────────────
// 域 (符号的命名空间)
// ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Domain {
    pub name: String,
    pub symbols: HashMap<String, Symbol>,
    /// 依赖的域
    pub dependencies: HashSet<String>,
    /// 是否已加载
    pub loaded: bool,
}

impl Domain {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            symbols: HashMap::new(),
            dependencies: HashSet::new(),
            loaded: false,
        }
    }

    /// 注册符号
    pub fn define(&mut self, name: &str, kind: SymbolKind) -> &mut Symbol {
        let sym = Symbol::new(name, &self.name, kind);
        self.symbols.insert(name.to_string(), sym);
        self.symbols.get_mut(name).unwrap()
    }

    /// 查找符号 (心算: 从内存直接取出)
    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// 域中活跃符号数
    pub fn active_count(&self) -> usize {
        self.symbols.values().filter(|s| s.is_active()).count()
    }

    /// 域的总权重 (符号数 × 平均引用)
    pub fn weight(&self) -> f64 {
        let total_ref: u32 = self.symbols.values().map(|s| s.ref_count).sum();
        (self.symbols.len() as f64) * (1.0 + total_ref as f64 / 100.0)
    }
}

// ─────────────────────────────────────────────────────────
// 记忆引擎
// ─────────────────────────────────────────────────────────

pub struct MemoryEngine {
    /// 所有已注册的域
    pub domains: HashMap<String, Domain>,
    /// 全局符号索引 (跨域查找)
    pub global_index: HashMap<String, String>,
    /// 持久化计数
    pub persist_count: u64,
    /// 心算缓存 (最近使用的符号)
    pub lru_cache: Vec<String>,
    /// LRU 最大容量
    lru_capacity: usize,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
            global_index: HashMap::new(),
            persist_count: 0,
            lru_cache: Vec::new(),
            lru_capacity: 64,
        }
    }

    /// 创建域
    pub fn create_domain(&mut self, name: &str) -> &mut Domain {
        self.domains.insert(name.to_string(), Domain::new(name));
        self.domains.get_mut(name).unwrap()
    }

    /// 获取域 (如果不存在则创建)
    pub fn get_or_create_domain(&mut self, name: &str) -> &mut Domain {
        if !self.domains.contains_key(name) {
            self.create_domain(name);
        }
        self.domains.get_mut(name).unwrap()
    }

    /// 域加载: 运行前预加载所有符号
    pub fn load_domain(&mut self, name: &str) -> bool {
        if let Some(domain) = self.domains.get_mut(name) {
            domain.loaded = true;
            // 预加载: 将所有符号加入全局索引
            for (sym_name, sym) in &domain.symbols {
                let global_key = format!("{}::{}", name, sym_name);
                self.global_index.insert(global_key, name.to_string());
            }
            true
        } else {
            false
        }
    }

    /// 解析符号 (心算: 从内存直接取出)
    pub fn resolve(&mut self, domain: &str, name: &str) -> Option<&Symbol> {
        let cache_key = format!("{}::{}", domain, name);
        // 更新 LRU
        if !self.lru_cache.contains(&cache_key) {
            if self.lru_cache.len() >= self.lru_capacity {
                self.lru_cache.remove(0);
            }
            self.lru_cache.push(cache_key);
        }
        // 查找
        let d = self.domains.get(domain)?;
        d.resolve(name)
    }

    /// 跨域解析 (通过全局索引)
    pub fn resolve_global(&self, full_path: &str) -> Option<&Symbol> {
        let domain_name = self.global_index.get(full_path)?;
        let domain = self.domains.get(domain_name)?;
        // 从 full_path 提取符号名
        let sym_name = full_path.split("::").nth(1)?;
        domain.resolve(sym_name)
    }

    /// 持久化符号 = 写入文件
    pub fn persist_symbol(&mut self, domain: &str, name: &str, line: u32) -> bool {
        let d = match self.domains.get_mut(domain) {
            Some(d) => d,
            None => return false,
        };
        let sym = match d.symbols.get_mut(name) {
            Some(s) => s,
            None => return false,
        };
        sym.persist(line);
        self.persist_count += 1;
        true
    }

    /// 预热所有域 (跨会话启动时调用)
    pub fn warm_all(&mut self) -> u32 {
        let mut count = 0;
        for name in self.domains.keys().cloned().collect::<Vec<_>>() {
            if self.load_domain(&name) {
                count += 1;
            }
        }
        count
    }

    /// 统计
    pub fn stats(&self) -> MemoryStats {
        let total_symbols: usize = self.domains.values().map(|d| d.symbols.len()).sum();
        let total_active: usize = self.domains.values().map(|d| d.active_count()).sum();
        let loaded_domains = self.domains.values().filter(|d| d.loaded).count();

        MemoryStats {
            domain_count: self.domains.len(),
            total_symbols,
            active_symbols: total_active,
            loaded_domains,
            persist_count: self.persist_count,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemoryStats {
    pub domain_count: usize,
    pub total_symbols: usize,
    pub active_symbols: usize,
    pub loaded_domains: usize,
    pub persist_count: u64,
}

// ═══════════════════════════════════════════════════════════
// 测试
// ═══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn symbol_new() {
        let s = Symbol::new("root", "core", SymbolKind::Container);
        assert_eq!(s.name, "root");
        assert_eq!(s.kind, SymbolKind::Container);
        assert!(!s.persisted);
    }

    #[test] fn symbol_persist() {
        let mut s = Symbol::new("x", "core", SymbolKind::Route);
        s.persist(42);
        assert!(s.persisted);
        assert_eq!(s.file_line, 42);
    }

    #[test] fn symbol_reference_tracking() {
        let mut s = Symbol::new("x", "d", SymbolKind::Container);
        s.reference();
        s.reference();
        assert_eq!(s.ref_count, 2);
        assert!(!s.is_active()); // not persisted yet
        s.persist(1);
        assert!(s.is_active());
    }

    #[test] fn domain_define_and_resolve() {
        let mut d = Domain::new("core");
        d.define("root", SymbolKind::Container);
        assert!(d.resolve("root").is_some());
        assert!(d.resolve("nonexistent").is_none());
    }

    #[test] fn domain_weighted_by_references() {
        let mut d = Domain::new("core");
        let s = d.define("root", SymbolKind::Container);
        s.reference();
        assert!(d.weight() > 0.0);
    }

    #[test] fn memory_create_domain() {
        let mut m = MemoryEngine::new();
        m.create_domain("core");
        assert!(m.domains.contains_key("core"));
    }

    #[test] fn memory_load_domain() {
        let mut m = MemoryEngine::new();
        m.create_domain("core");
        assert!(m.load_domain("core"));
        assert!(m.domains["core"].loaded);
    }

    #[test] fn memory_resolve_after_load() {
        let mut m = MemoryEngine::new();
        let d = m.get_or_create_domain("core");
        d.define("root", SymbolKind::Container);
        m.load_domain("core");
        assert!(m.resolve("core", "root").is_some());
    }

    #[test] fn memory_load_nonexistent_fails() {
        let mut m = MemoryEngine::new();
        assert!(!m.load_domain("nonexistent"));
    }

    #[test] fn memory_persist_increments_count() {
        let mut m = MemoryEngine::new();
        let d = m.get_or_create_domain("core");
        d.define("root", SymbolKind::Container);
        assert!(m.persist_symbol("core", "root", 10));
        assert_eq!(m.persist_count, 1);
    }

    #[test] fn memory_persist_nonexistent_fails() {
        let mut m = MemoryEngine::new();
        assert!(!m.persist_symbol("core", "root", 10));
    }

    #[test] fn memory_warm_all() {
        let mut m = MemoryEngine::new();
        m.create_domain("core");
        m.create_domain("net");
        m.create_domain("ui");
        assert_eq!(m.warm_all(), 3);
    }

    #[test] fn memory_stats() {
        let mut m = MemoryEngine::new();
        {
            let d = m.get_or_create_domain("core");
            d.define("a", SymbolKind::Container);
            d.define("b", SymbolKind::Route);
        }
        m.load_domain("core");
        let s = m.stats();
        assert_eq!(s.domain_count, 1);
        assert_eq!(s.total_symbols, 2);
        assert_eq!(s.loaded_domains, 1);
    }

    #[test] fn memory_lru_cache() {
        let mut m = MemoryEngine::new();
        let d = m.get_or_create_domain("core");
        d.define("root", SymbolKind::Container);
        m.load_domain("core");
        let _ = m.resolve("core", "root");
        assert!(!m.lru_cache.is_empty());
    }

    #[test] fn global_index_populated_on_load() {
        let mut m = MemoryEngine::new();
        let d = m.get_or_create_domain("net");
        d.define("http", SymbolKind::Protocol);
        m.load_domain("net");
        assert!(m.global_index.contains_key("net::http"));
    }
}
