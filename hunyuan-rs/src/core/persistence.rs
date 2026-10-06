// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.81 在场持久化引擎                                            ║
// ║  注册清单 · 清单索引 · 特征编程 · 工作空间 · 时间版本 · 思维入口  ║
// ║                                                                  ║
// ║  核心: 符号在场 = 落盘可寻址, 不依赖对话上下文                    ║
// ║  · 注册清单: 数值索引地址检索, 一次性构建                        ║
// ║  · 特征编程: 特征标签 = 门禁入口, 入口之下全排队                  ║
// ║  · 工作空间: 时间·工作·注册·思维 四清单                          ║
// ║  · 时间版本: 工作状态/版本号/时间标注/认知更新/错误总结           ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};

/// 文档状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocStatus {
    /// 草稿
    Draft,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已归档
    Archived,
    /// 弃用
    Deprecated,
}

/// 注册条目 — 一个被注册的符号/文档
#[derive(Clone, Debug)]
pub struct RegistryEntry {
    /// 数值索引地址 (进制优先, 地址在前)
    pub address: u64,
    /// 名称空间类型 (绑在地址后面)
    pub namespace: String,
    /// 容器引用 (在场位置)
    pub container: ContainerRef,
    /// 文档状态
    pub status: DocStatus,
    /// 版本号
    pub version: u32,
    /// 创建拍
    pub created_tick: u64,
    /// 最后更新拍
    pub updated_tick: u64,
    /// 特征标签 (用于特征编程入口)
    pub features: Vec<String>,
    /// 关联条目地址
    pub related: Vec<u64>,
    /// 条目权重 (被引用加法统计)
    pub weight: u64,
}

impl RegistryEntry {
    pub fn new(address: u64, namespace: &str, container: ContainerRef, tick: u64) -> Self {
        RegistryEntry {
            address,
            namespace: namespace.to_string(),
            container,
            status: DocStatus::Draft,
            version: 1,
            created_tick: tick,
            updated_tick: tick,
            features: Vec::new(),
            related: Vec::new(),
            weight: 0,
        }
    }

    /// 添加特征
    pub fn with_feature(mut self, feature: &str) -> Self {
        if !self.features.contains(&feature.to_string()) {
            self.features.push(feature.to_string());
        }
        self
    }

    /// 更新版本 (加法)
    pub fn bump_version(&mut self, tick: u64) {
        self.version += 1;
        self.updated_tick = tick;
    }

    /// 添加关联
    pub fn link_to(&mut self, address: u64) {
        if !self.related.contains(&address) {
            self.related.push(address);
        }
    }

    /// 被见证 (加法统计)
    pub fn witnessed(&mut self) {
        self.weight += 1;
    }

    /// 特征匹配
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
}

/// 注册清单 — 系统的在场符号注册表
#[derive(Clone, Debug, Default)]
pub struct Registry {
    /// 主索引: 地址 → 条目
    pub by_address: HashMap<u64, RegistryEntry>,
    /// 特征索引: 特征 → [地址] (特征编程入口)
    pub by_feature: HashMap<String, Vec<u64>>,
    /// 命名空间索引: 类型 → [地址]
    pub by_namespace: HashMap<String, Vec<u64>>,
    /// 总注册数
    pub total_registered: u64,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            by_address: HashMap::new(),
            by_feature: HashMap::new(),
            by_namespace: HashMap::new(),
            total_registered: 0,
        }
    }

    /// 注册条目
    pub fn register(&mut self, entry: RegistryEntry) -> bool {
        if self.by_address.contains_key(&entry.address) {
            return false; // 已在场
        }

        // 特征索引
        for feature in &entry.features {
            self.by_feature
                .entry(feature.clone())
                .or_default()
                .push(entry.address);
        }

        // 命名空间索引
        self.by_namespace
            .entry(entry.namespace.clone())
            .or_default()
            .push(entry.address);

        self.total_registered += 1;
        self.by_address.insert(entry.address, entry);
        true
    }

    /// 按地址查询
    pub fn find(&self, address: u64) -> Option<&RegistryEntry> {
        self.by_address.get(&address)
    }

    /// 按地址查询 (可变)
    pub fn find_mut(&mut self, address: u64) -> Option<&mut RegistryEntry> {
        self.by_address.get_mut(&address)
    }

    /// 特征编程入口: 特征 → 排队的地址
    pub fn query_feature(&self, feature: &str) -> Option<&Vec<u64>> {
        self.by_feature.get(feature)
    }

    /// 遍历特征地址 → 取出完整条目
    pub fn resolve_feature(&self, feature: &str) -> Vec<&RegistryEntry> {
        self.by_feature
            .get(feature)
            .map(|addrs| {
                addrs.iter()
                    .filter_map(|a| self.by_address.get(a))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 命名空间遍历
    pub fn query_namespace(&self, namespace: &str) -> Vec<&RegistryEntry> {
        self.by_namespace
            .get(namespace)
            .map(|addrs| {
                addrs.iter()
                    .filter_map(|a| self.by_address.get(a))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 按状态过滤
    pub fn filter_by_status(&self, status: DocStatus) -> Vec<&RegistryEntry> {
        self.by_address
            .values()
            .filter(|e| e.status == status)
            .collect()
    }

    /// 地址数量
    pub fn count(&self) -> usize {
        self.by_address.len()
    }

    /// 全部地址列表 (排序)
    pub fn all_addresses(&self) -> Vec<u64> {
        let mut addrs: Vec<u64> = self.by_address.keys().copied().collect();
        addrs.sort();
        addrs
    }

    /// 按地址前缀搜索
    pub fn search_prefix(&self, prefix: u64, radix: u64) -> Vec<&RegistryEntry> {
        self.by_address
            .iter()
            .filter(|(addr, _)| *addr / radix == prefix)
            .map(|(_, entry)| entry)
            .collect()
    }
}

/// 时间版本 — 文档的时间线
#[derive(Clone, Debug)]
pub struct TimeVersion {
    /// 版本号
    pub version: u32,
    /// 时间戳
    pub tick: u64,
    /// 变更摘要
    pub summary: String,
    /// 认知更新 (本版本新增理解)
    pub cognition_update: String,
    /// 错误总结 (本版本发现的错误)
    pub error_summary: String,
    /// 未来预期
    pub future_expectation: String,
    /// 约束方案
    pub constraint_plan: String,
}

impl TimeVersion {
    pub fn new(version: u32, tick: u64, summary: &str) -> Self {
        TimeVersion {
            version,
            tick,
            summary: summary.to_string(),
            cognition_update: String::new(),
            error_summary: String::new(),
            future_expectation: String::new(),
            constraint_plan: String::new(),
        }
    }

    pub fn with_cognition(mut self, update: &str) -> Self {
        self.cognition_update = update.to_string();
        self
    }

    pub fn with_error(mut self, err: &str) -> Self {
        self.error_summary = err.to_string();
        self
    }

    pub fn with_future(mut self, exp: &str) -> Self {
        self.future_expectation = exp.to_string();
        self
    }

    pub fn with_constraint(mut self, plan: &str) -> Self {
        self.constraint_plan = plan.to_string();
        self
    }
}

/// 工作空间 — 四类清单的组织
#[derive(Clone, Debug, Default)]
pub struct Workspace {
    /// 时间清单: 按时间排序的地址
    pub time_list: Vec<u64>,
    /// 工作清单: 当前工作的地址
    pub work_list: Vec<u64>,
    /// 注册清单: 已注册的地址
    pub registry_list: Vec<u64>,
    /// 思维清单: 思维入口的地址
    pub thought_list: Vec<u64>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace::default()
    }

    /// 添加到工作清单
    pub fn add_work(&mut self, address: u64) {
        if !self.work_list.contains(&address) {
            self.work_list.push(address);
        }
    }

    /// 添加到思维清单
    pub fn add_thought(&mut self, address: u64) {
        if !self.thought_list.contains(&address) {
            self.thought_list.push(address);
        }
    }

    /// 添加到时间清单 (按时间排序插入)
    pub fn add_time(&mut self, address: u64) {
        if !self.time_list.contains(&address) {
            self.time_list.push(address);
        }
    }

    /// 注册
    pub fn add_registry(&mut self, address: u64) {
        if !self.registry_list.contains(&address) {
            self.registry_list.push(address);
        }
    }

    /// 工作清单长度
    pub fn work_count(&self) -> usize {
        self.work_list.len()
    }

    /// 全部活跃 (工作 + 思维)
    pub fn active_items(&self) -> Vec<u64> {
        let mut items = Vec::new();
        items.extend(&self.work_list);
        items.extend(self.thought_list.iter().filter(|a| !self.work_list.contains(a)));
        items
    }
}

/// 思维入口
#[derive(Clone, Debug)]
pub struct ThoughtEntry {
    /// 入口地址
    pub address: u64,
    /// 思维标签
    pub label: String,
    /// 引导下一跳地址
    pub next_links: Vec<u64>,
    /// 前置地址 (依赖)
    pub prerequisites: Vec<u64>,
    /// 是否出口 (终结点)
    pub is_exit: bool,
}

impl ThoughtEntry {
    pub fn new(address: u64, label: &str) -> Self {
        ThoughtEntry {
            address,
            label: label.to_string(),
            next_links: Vec::new(),
            prerequisites: Vec::new(),
            is_exit: false,
        }
    }

    pub fn link_next(mut self, addr: u64) -> Self {
        if !self.next_links.contains(&addr) {
            self.next_links.push(addr);
        }
        self
    }

    pub fn require(mut self, addr: u64) -> Self {
        if !self.prerequisites.contains(&addr) {
            self.prerequisites.push(addr);
        }
        self
    }

    /// 是否可达 (无未满足前置)
    pub fn is_reachable(&self, available: &[u64]) -> bool {
        self.prerequisites.iter().all(|p| available.contains(p))
    }
}

/// 在场持久化引擎
pub struct PersistenceEngine {
    /// 注册清单
    pub registry: Registry,
    /// 工作空间
    pub workspace: Workspace,
    /// 思维入口表
    pub thoughts: HashMap<u64, ThoughtEntry>,
    /// 版本历史: 地址 → [版本]
    pub versions: HashMap<u64, Vec<TimeVersion>>,
    /// 当前拍
    pub current_tick: u64,
}

impl PersistenceEngine {
    pub fn new() -> Self {
        PersistenceEngine {
            registry: Registry::new(),
            workspace: Workspace::new(),
            thoughts: HashMap::new(),
            versions: HashMap::new(),
            current_tick: 0,
        }
    }

    /// 在场注册 (一次性落盘)
    pub fn persist(&mut self, entry: RegistryEntry) -> bool {
        if self.registry.register(entry.clone()) {
            self.workspace.add_registry(entry.address);
            self.workspace.add_time(entry.address);
            true
        } else {
            false
        }
    }

    /// 推进拍
    pub fn tick(&mut self) {
        self.current_tick += 1;
    }

    /// 版本记录
    pub fn record_version(&mut self, address: u64, version: TimeVersion) {
        self.versions
            .entry(address)
            .or_default()
            .push(version);
    }

    /// 获取最新版本的条目
    pub fn latest_version(&self, address: u64) -> Option<&TimeVersion> {
        self.versions
            .get(&address)?
            .last()
    }

    /// 添加思维入口
    pub fn add_thought(&mut self, thought: ThoughtEntry) {
        self.thoughts.insert(thought.address, thought);
    }

    /// 按思维入口遍历
    pub fn trace_thought(&self, start: u64) -> Vec<u64> {
        let mut visited = Vec::new();
        let mut queue = vec![start];

        while let Some(addr) = queue.pop() {
            if visited.contains(&addr) {
                continue;
            }
            visited.push(addr);

            if let Some(thought) = self.thoughts.get(&addr) {
                for &next in &thought.next_links {
                    if !visited.contains(&next) {
                        queue.push(next);
                    }
                }
            }
        }
        visited
    }

    /// 按特征编程取出所有地址的条目
    pub fn feature_select(&self, feature: &str) -> Vec<&RegistryEntry> {
        self.registry.resolve_feature(feature)
    }

    /// 特征统计: 有多少地址拥有此特征
    pub fn feature_count(&self, feature: &str) -> usize {
        self.registry.by_feature.get(feature).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for PersistenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn registry_entry_new() {
        let e = RegistryEntry::new(100, "测试", mk_ref(1), 0);
        assert_eq!(e.address, 100);
        assert_eq!(e.version, 1);
        assert_eq!(e.status, DocStatus::Draft);
        assert_eq!(e.weight, 0);
    }

    #[test]
    fn registry_entry_feature() {
        let e = RegistryEntry::new(100, "测试", mk_ref(1), 0)
            .with_feature("意识")
            .with_feature("路由");
        assert!(e.has_feature("意识"));
        assert!(e.has_feature("路由"));
        assert!(!e.has_feature("通信"));
    }

    #[test]
    fn registry_entry_bump_version() {
        let mut e = RegistryEntry::new(100, "测试", mk_ref(1), 0);
        e.bump_version(5);
        assert_eq!(e.version, 2);
        assert_eq!(e.updated_tick, 5);
    }

    #[test]
    fn registry_entry_witnessed() {
        let mut e = RegistryEntry::new(100, "测试", mk_ref(1), 0);
        e.witnessed();
        e.witnessed();
        assert_eq!(e.weight, 2);
    }

    #[test]
    fn registry_register() {
        let mut reg = Registry::new();
        let e = RegistryEntry::new(100, "工作", mk_ref(1), 0);
        assert!(reg.register(e));
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn registry_duplicate_rejected() {
        let mut reg = Registry::new();
        reg.register(RegistryEntry::new(100, "工作", mk_ref(1), 0));
        let result = reg.register(RegistryEntry::new(100, "工作", mk_ref(1), 0));
        assert!(!result);
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn registry_feature_index() {
        let mut reg = Registry::new();
        reg.register(
            RegistryEntry::new(100, "意识", mk_ref(1), 0)
                .with_feature("路由"),
        );
        reg.register(
            RegistryEntry::new(200, "通信", mk_ref(2), 0)
                .with_feature("路由"),
        );
        reg.register(
            RegistryEntry::new(300, "时间", mk_ref(3), 0)
                .with_feature("版本"),
        );

        let routes = reg.query_feature("路由").unwrap();
        assert_eq!(routes.len(), 2);
        assert!(routes.contains(&100));
        assert!(routes.contains(&200));
    }

    #[test]
    fn registry_resolve_feature() {
        let mut reg = Registry::new();
        reg.register(
            RegistryEntry::new(100, "意识", mk_ref(1), 0)
                .with_feature("路由"),
        );

        let entries = reg.resolve_feature("路由");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].address, 100);
    }

    #[test]
    fn registry_namespace_query() {
        let mut reg = Registry::new();
        reg.register(RegistryEntry::new(100, "工作", mk_ref(1), 0));
        reg.register(RegistryEntry::new(200, "工作", mk_ref(2), 0));
        reg.register(RegistryEntry::new(300, "思维", mk_ref(3), 0));

        let works = reg.query_namespace("工作");
        assert_eq!(works.len(), 2);
    }

    #[test]
    fn registry_filter_by_status() {
        let mut reg = Registry::new();
        reg.register(RegistryEntry::new(100, "意识", mk_ref(1), 0));
        let mut e = RegistryEntry::new(200, "通信", mk_ref(2), 0);
        e.status = DocStatus::Completed;
        reg.register(e);

        let drafts = reg.filter_by_status(DocStatus::Draft);
        assert_eq!(drafts.len(), 1);
        assert_eq!(drafts[0].address, 100);
    }

    #[test]
    fn time_version_full() {
        let v = TimeVersion::new(2, 100, "更新路由算法")
            .with_cognition("理解了谐音路由")
            .with_error("旧版本遗漏了Null处理")
            .with_future("计划加入主权路由")
            .with_constraint("需要兼容旧版格式");

        assert_eq!(v.version, 2);
        assert_eq!(v.cognition_update, "理解了谐音路由");
        assert_eq!(v.error_summary, "旧版本遗漏了Null处理");
    }

    #[test]
    fn workspace_add_work() {
        let mut ws = Workspace::new();
        ws.add_work(100);
        ws.add_work(101);
        assert_eq!(ws.work_count(), 2);
    }

    #[test]
    fn workspace_active_items() {
        let mut ws = Workspace::new();
        ws.add_work(100);
        ws.add_thought(200);
        ws.add_thought(100); // 已在 work 中

        let active = ws.active_items();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn workspace_dedup() {
        let mut ws = Workspace::new();
        ws.add_work(100);
        ws.add_work(100);
        assert_eq!(ws.work_count(), 1);
    }

    #[test]
    fn thought_entry_links() {
        let t = ThoughtEntry::new(100, "起点")
            .link_next(200)
            .link_next(300);
        assert_eq!(t.next_links.len(), 2);
    }

    #[test]
    fn thought_reachable() {
        let t = ThoughtEntry::new(200, "过程")
            .require(100);
        assert!(t.is_reachable(&[100]));
        assert!(!t.is_reachable(&[]));
    }

    #[test]
    fn thought_not_reachable_partial() {
        let t = ThoughtEntry::new(300, "高级")
            .require(100)
            .require(200);
        assert!(!t.is_reachable(&[100]));
        assert!(t.is_reachable(&[100, 200]));
    }

    #[test]
    fn persistence_engine_new() {
        let engine = PersistenceEngine::new();
        assert_eq!(engine.registry.count(), 0);
        assert_eq!(engine.current_tick, 0);
    }

    #[test]
    fn persistence_persist_and_find() {
        let mut engine = PersistenceEngine::new();
        let entry = RegistryEntry::new(0x6000_0000, "意识", mk_ref(1), 0)
            .with_feature("路由");

        assert!(engine.persist(entry));
        assert!(engine.registry.find(0x6000_0000).is_some());
    }

    #[test]
    fn persistence_tick() {
        let mut engine = PersistenceEngine::new();
        engine.tick();
        engine.tick();
        assert_eq!(engine.current_tick, 2);
    }

    #[test]
    fn persistence_record_version() {
        let mut engine = PersistenceEngine::new();
        let entry = RegistryEntry::new(100, "意识", mk_ref(1), 0);
        engine.persist(entry);

        engine.record_version(100, TimeVersion::new(1, 0, "初始"));
        engine.record_version(100, TimeVersion::new(2, 10, "更新"));

        let latest = engine.latest_version(100).unwrap();
        assert_eq!(latest.version, 2);
    }

    #[test]
    fn persistence_trace_thought() {
        let mut engine = PersistenceEngine::new();
        engine.add_thought(
            ThoughtEntry::new(1, "起点")
                .link_next(2)
                .link_next(3),
        );
        engine.add_thought(
            ThoughtEntry::new(2, "中继")
                .link_next(4),
        );
        engine.add_thought(ThoughtEntry::new(3, "分支"));
        engine.add_thought(ThoughtEntry::new(4, "终点"));

        let trace = engine.trace_thought(1);
        assert!(trace.contains(&1));
        assert!(trace.contains(&2));
        assert!(trace.contains(&3));
        assert!(trace.contains(&4));
    }

    #[test]
    fn persistence_feature_select() {
        let mut engine = PersistenceEngine::new();
        engine.persist(
            RegistryEntry::new(100, "意识", mk_ref(1), 0)
                .with_feature("路由"),
        );
        engine.persist(
            RegistryEntry::new(200, "通信", mk_ref(2), 0)
                .with_feature("路由"),
        );

        let routes = engine.feature_select("路由");
        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn persistence_feature_count() {
        let mut engine = PersistenceEngine::new();
        engine.persist(
            RegistryEntry::new(100, "意识", mk_ref(1), 0)
                .with_feature("路由"),
        );
        engine.persist(
            RegistryEntry::new(200, "通信", mk_ref(2), 0)
                .with_feature("路由"),
        );
        engine.persist(
            RegistryEntry::new(300, "时间", mk_ref(3), 0)
                .with_feature("版本"),
        );

        assert_eq!(engine.feature_count("路由"), 2);
        assert_eq!(engine.feature_count("版本"), 1);
        assert_eq!(engine.feature_count("不存在"), 0);
    }

    #[test]
    fn doc_status_equality() {
        assert_eq!(DocStatus::Draft, DocStatus::Draft);
        assert_ne!(DocStatus::Draft, DocStatus::Completed);
    }

    #[test]
    fn registry_count() {
        let mut reg = Registry::new();
        assert_eq!(reg.count(), 0);

        reg.register(RegistryEntry::new(1, "a", mk_ref(1), 0));
        reg.register(RegistryEntry::new(2, "b", mk_ref(2), 0));
        assert_eq!(reg.count(), 2);
    }

    #[test]
    fn thought_exit_flag() {
        let mut t = ThoughtEntry::new(100, "终点");
        assert!(!t.is_exit);
        t.is_exit = true;
        assert!(t.is_exit);
    }
}
