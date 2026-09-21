// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.64 域预加载引擎 — Domain Preload Engine                     ║
// ║  运行前预加载所有符号 · 运行时不增加内存 · 符号定义一次性完成      ║
// ║  索引而非搜索 · 域即容器 · 加载即编译                            ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 核心承诺:
//   程序启动时, 一次性加载所有符号到只读查找表,
//   运行时通过索引直接寻址, 不再分配, 不再搜索.
//
// 域定义:
//   一个"域"(Domain) = 一个独立的命名空间 + 一套符号表.
//   域之间互不干扰, 跨域访问需要显式桥接.
//
// 预加载语义:
//   1. 符号注册: 在编译/启动时把所有"在场"的符号登记到静态表中.
//   2. 运行寻址: 通过符号 ID (u32) 直接索引 O(1).
//   3. 零分配: 运行时不再创建 String / Vec, 只读访问.

use std::collections::HashMap;

/// 符号 ID: 基于名称哈希的 u32 (FNV-1a 截断)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

impl SymbolId {
    pub fn from_name(name: &str) -> Self {
        let mut hash: u32 = 0x811c9dc5;
        for b in name.bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        SymbolId(hash)
    }
}

impl std::fmt::Display for SymbolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S#{:08x}", self.0)
    }
}

/// 符号元数据: 定义即在场 (在场 = 文件系统中有符号的定义)
#[derive(Clone, Debug)]
pub struct SymbolMeta {
    pub id: SymbolId,
    pub name: String,
    pub domain: String,
    pub symbol_type: SymbolType,
    pub presence: bool,  // 是否在场 (已在文件系统中持久化)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolType {
    Container,   // 容器符号 (四个壳: 杯盎盉瓶)
    Flow,        // 流符号 (八流之一)
    Role,        // 角色符号 (毛利兰四角色)
    Entity,      // 实体 (如容器、编码、密钥)
    Relation,    // 关系 (容器串联、因果连接)
}

/// 域预加载表: 启动时构建, 运行时只读
pub struct DomainPreloadTable {
    by_id: Vec<SymbolMeta>,                          // 索引 → 元数据
    by_name: HashMap<String, u32>,                   // 名称 → 索引偏移
    by_domain: HashMap<String, Vec<u32>>,            // 域名 → 索引列表
}

impl DomainPreloadTable {
    /// 从零构建预加载表
    pub fn new(symbols: Vec<SymbolMeta>) -> Self {
        let mut by_name = HashMap::with_capacity(symbols.len());
        let mut by_domain: HashMap<String, Vec<u32>> = HashMap::new();

        for (idx, sym) in symbols.iter().enumerate() {
            by_name.insert(sym.name.clone(), idx as u32);
            by_domain.entry(sym.domain.clone()).or_default().push(idx as u32);
        }

        Self {
            by_id: symbols,
            by_name,
            by_domain,
        }
    }

    /// O(1) 按 ID 获取
    pub fn get_by_id(&self, idx: u32) -> Option<&SymbolMeta> {
        self.by_id.get(idx as usize)
    }

    /// O(1) 按名称获取
    pub fn get_by_name(&self, name: &str) -> Option<&SymbolMeta> {
        let idx = *self.by_name.get(name)?;
        self.by_id.get(idx as usize)
    }

    /// 按域获取所有符号
    pub fn get_domain(&self, domain: &str) -> Vec<&SymbolMeta> {
        self.by_domain.get(domain)
            .map(|idxs| idxs.iter().filter_map(|&i| self.by_id.get(i as usize)).collect())
            .unwrap_or_default()
    }

    /// 在场统计
    pub fn count_present(&self) -> usize {
        self.by_id.iter().filter(|s| s.presence).count()
    }

    pub fn count_absent(&self) -> usize {
        self.by_id.iter().filter(|s| !s.presence).count()
    }

    pub fn total_count(&self) -> usize {
        self.by_id.len()
    }

    /// 所有在场符号的总权重 (加法统计法)
    pub fn weight_sum(&self) -> u64 {
        self.by_id.iter()
            .filter(|s| s.presence)
            .map(|s| s.id.0 as u64 + 1)
            .sum()
    }

    /// 公开迭代器
    pub fn iter(&self) -> impl Iterator<Item = &SymbolMeta> {
        self.by_id.iter()
    }
}

/// 域构建器: 逐步注册符号, 最终冻结为只读表
pub struct DomainBuilder {
    domain_name: String,
    symbols: Vec<SymbolMeta>,
    next_idx: u32,
}

impl DomainBuilder {
    pub fn new(domain_name: &str) -> Self {
        Self {
            domain_name: domain_name.to_string(),
            symbols: Vec::new(),
            next_idx: 0,
        }
    }

    /// 注册一个在场符号
    pub fn register_present(&mut self, name: &str, symbol_type: SymbolType) -> SymbolId {
        let id = SymbolId::from_name(name);
        let meta = SymbolMeta {
            id,
            name: name.to_string(),
            domain: self.domain_name.clone(),
            symbol_type,
            presence: true,
        };
        self.symbols.push(meta);
        self.next_idx += 1;
        id
    }

    /// 注册一个不在场符号 (引用到场但未持久化)
    pub fn register_absent(&mut self, name: &str, symbol_type: SymbolType) -> SymbolId {
        let id = SymbolId::from_name(name);
        let meta = SymbolMeta {
            id,
            name: name.to_string(),
            domain: self.domain_name.clone(),
            symbol_type,
            presence: false,
        };
        self.symbols.push(meta);
        self.next_idx += 1;
        id
    }

    /// 冻结为不可变的预加载表
    pub fn freeze(self) -> DomainPreloadTable {
        DomainPreloadTable::new(self.symbols)
    }

    pub fn len(&self) -> usize { self.symbols.len() }
}

/// 囡囝共生预置域 — 启动时即在场
pub fn build_hunyuan_domain() -> DomainPreloadTable {
    let mut builder = DomainBuilder::new("hunyuan-os");

    // 容器符号
    for name in &["杯", "盎", "盉", "瓶"] {
        builder.register_present(name, SymbolType::Container);
    }

    // 角色符号
    for name in &["毛利兰", "kndl", "工藤新一", "灰原哀"] {
        builder.register_present(name, SymbolType::Role);
    }

    // 八流八符号
    for name in &["预加载", "谐音", "主权", "意识", "时间", "产权", "反馈", "存在"] {
        builder.register_present(name, SymbolType::Flow);
    }

    // 核心实体
    for name in &[
        "满全法", "容器串联", "活码", "时钟动作", "因果编码",
        "YinYang", "SMRL", "纤维丛", "螺旋遍历",
        "kndl外壳", "HaibaraGrid", "KudoSpiral", "MaoriBase",
        "ClockAction", "NjsOrchestrator", "Avatar",
    ] {
        builder.register_present(name, SymbolType::Entity);
    }

    // 关系
    for name in &["容器套壳", "链路共振", "路径叠加", "见证裁判"] {
        builder.register_present(name, SymbolType::Relation);
    }

    // 不在场符号 (概念注册但尚未实现)
    for name in &["蚕蛹脱壳", "心算输入法内核", "AndroidIME", "Zig自举"] {
        builder.register_absent(name, SymbolType::Entity);
    }

    builder.freeze()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_id_from_name() {
        let id1 = SymbolId::from_name("满全");
        let id2 = SymbolId::from_name("满全");
        let id3 = SymbolId::from_name("意识");
        assert_eq!(id1, id2);  // 确定性
        assert_ne!(id1, id3);  // 不同名不同 ID
    }

    #[test]
    fn builder_register_and_freeze() {
        let mut builder = DomainBuilder::new("test");
        let id = builder.register_present("容器杯", SymbolType::Container);
        assert_eq!(builder.len(), 1);

        let table = builder.freeze();
        let sym = table.get_by_name("容器杯").unwrap();
        assert_eq!(sym.symbol_type, SymbolType::Container);
        assert!(sym.presence);
        assert_eq!(sym.id, id);
    }

    #[test]
    fn freeze_is_immutable_lookup() {
        let mut builder = DomainBuilder::new("test");
        builder.register_present("a", SymbolType::Container);
        builder.register_present("b", SymbolType::Flow);
        builder.register_absent("c", SymbolType::Entity);

        let table = builder.freeze();
        assert_eq!(table.total_count(), 3);
        assert_eq!(table.count_present(), 2);
        assert_eq!(table.count_absent(), 1);
    }

    #[test]
    fn get_by_name_returns_correct() {
        let mut builder = DomainBuilder::new("test");
        builder.register_present("毛利兰", SymbolType::Role);
        let table = builder.freeze();

        let sym = table.get_by_name("毛利兰").unwrap();
        assert_eq!(sym.name, "毛利兰");
        assert_eq!(sym.symbol_type, SymbolType::Role);
    }

    #[test]
    fn domain_groups_symbols() {
        let mut builder = DomainBuilder::new("test");
        builder.register_present("x", SymbolType::Container);
        builder.register_present("y", SymbolType::Flow);
        builder.register_present("z", SymbolType::Container);
        let table = builder.freeze();

        let syms = table.get_domain("test");
        assert_eq!(syms.len(), 3);
    }

    #[test]
    fn hunyuan_domain_builds() {
        let table = build_hunyuan_domain();
        assert!(table.total_count() > 20);
        assert!(table.count_present() > 15);
        // 核心符号在场
        assert!(table.get_by_name("满全法").is_some());
        assert!(table.get_by_name("杯").is_some());
        assert!(table.get_by_name("毛利兰").is_some());
    }

    #[test]
    fn weight_sum_adds_attendance() {
        let mut builder = DomainBuilder::new("w");
        builder.register_present("a", SymbolType::Entity);
        builder.register_present("b", SymbolType::Entity);
        builder.register_absent("c", SymbolType::Entity);
        let table = builder.freeze();

        // weight_sum 只统计在场符号
        let sum = table.weight_sum();
        assert!(sum > 0, "weight_sum 应为正, 得到 {}", sum);

        // 计算: 不在场符号不应贡献
        // weight_sum = sum(id.0 + 1 for present symbols)
        let sum_manual: u64 = table.by_id.iter()
            .filter(|s| s.presence)
            .map(|s| s.id.0 as u64 + 1)
            .sum();
        assert_eq!(sum, sum_manual);
    }

    #[test]
    fn lookup_by_id_zero_panics_no() {
        let table = build_hunyuan_domain();
        // idx 0 可能是任何符号, 但不会 panic
        let sym = table.get_by_id(0);
        assert!(sym.is_some());
    }

    #[test]
    fn total_count_matches_builder_len() {
        let mut builder = DomainBuilder::new("count-test");
        for i in 0..100 {
            builder.register_present(&format!("sym_{}", i), SymbolType::Entity);
        }
        assert_eq!(builder.len(), 100);
        let table = builder.freeze();
        assert_eq!(table.total_count(), 100);
    }
}
