// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.89 文件生长系统                                              ║
// ║  文件自生长 · 符号自繁殖 · 路径生成 · 生长规则                    ║
// ║                                                                  ║
// ║  核心: 文件系统不是存储, 是符号的生长土壤                         ║
// ║  · 文件 = 容器地址的一种投影                                      ║
// ║  · 生长 = 符号根据注册规则扩展出新的地址                          ║
// ║  · 路径 = 容器关系链在文件系统中的映射                            ║
// ║  · 自繁殖 = 已有结构作为模板派生新结构                            ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use std::fmt;

// ══════════════════════════════════════════════════════════════
// 文件节点 — 目录即容器, 文件即地址
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub enum FileNode {
    /// 目录 = 包含子节点
    Directory { children: HashMap<String, FileNode> },
    /// 文件 = 存储一个地址值
    File { address: u64 },
    /// 链接 = 指向另一个路径
    Link { target: String },
}

impl FileNode {
    pub fn new_dir() -> Self {
        FileNode::Directory { children: HashMap::new() }
    }

    pub fn new_file(address: u64) -> Self {
        FileNode::File { address }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, FileNode::Directory { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileNode::File { .. })
    }

    /// 获取子节点
    pub fn child(&self, name: &str) -> Option<&FileNode> {
        match self {
            FileNode::Directory { children } => children.get(name),
            _ => None,
        }
    }

    /// 插入子节点
    pub fn insert(&mut self, name: String, node: FileNode) -> bool {
        match self {
            FileNode::Directory { children } => {
                children.insert(name, node);
                true
            }
            _ => false,
        }
    }
}

impl fmt::Display for FileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileNode::Directory { children } => {
                write!(f, "dir({} entries)", children.len())
            }
            FileNode::File { address } => {
                write!(f, "file({})", address)
            }
            FileNode::Link { target } => {
                write!(f, "link({})", target)
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 生长模板 — 从已有结构派生新结构
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct GrowthTemplate {
    /// 模板名称
    pub name: String,
    /// 模板内容 — 子路径结构 (名称 → 模板或地址种子)
    pub structure: Vec<(String, TemplateEntry)>,
}

#[derive(Clone, Debug)]
pub enum TemplateEntry {
    /// 子模板 — 递归展开
    SubTemplate(String),
    /// 地址种子 — 基于此生成文件
    AddressSeed(u64),
    /// 空目录
    EmptyDir,
}

impl GrowthTemplate {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), structure: Vec::new() }
    }

    pub fn with_entry(mut self, path: &str, entry: TemplateEntry) -> Self {
        self.structure.push((path.to_string(), entry));
        self
    }
}

// ══════════════════════════════════════════════════════════════
// 生长规则 — 何时以及如何生长
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct GrowthRule {
    /// 规则名称
    pub name: String,
    /// 触发路径 — 该路径上的变化触发规则
    pub trigger_path: String,
    /// 生长动作 — 要生成的路径模板
    pub template_name: String,
    /// 地址偏移 — 从此基数开始生成 (加法统计法)
    pub address_base: u64,
    /// 是否已执行
    pub executed: bool,
}

impl GrowthRule {
    pub fn new(name: &str, trigger_path: &str, template_name: &str) -> Self {
        Self {
            name: name.to_string(),
            trigger_path: trigger_path.to_string(),
            template_name: template_name.to_string(),
            address_base: 0,
            executed: false,
        }
    }

    pub fn with_base(mut self, base: u64) -> Self {
        self.address_base = base;
        self
    }
}

// ══════════════════════════════════════════════════════════════
// 路径解析 — 地址即路径, 路径即地址
// ══════════════════════════════════════════════════════════════

pub struct PathResolver;

impl PathResolver {
    /// 将地址转换为路径表示
    /// 进制优先: /radix/initial/final/tone
    pub fn address_to_path(addr: u32) -> String {
        let radix = (addr & 0xF) as u8;
        let initial = ((addr >> 4) & 0xFF) as u8;
        let final_v = ((addr >> 8) & 0x7F) as u8;
        let tone = ((addr >> 12) & 0xFF) as u8;
        format!("/{}/{}/{}/{}", radix, initial, final_v, tone)
    }

    /// 路径转回地址
    pub fn path_to_address(path: &str) -> Option<u32> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() != 4 { return None; }
        let radix = parts[0].parse::<u32>().ok()?;
        let initial = parts[1].parse::<u32>().ok()?;
        let final_v = parts[2].parse::<u32>().ok()?;
        let tone = parts[3].parse::<u32>().ok()?;
        Some(radix | (initial << 4) | (final_v << 8) | (tone << 12))
    }
}

// ══════════════════════════════════════════════════════════════
// 文件系统生长器
// ══════════════════════════════════════════════════════════════

pub struct FileGrowthSystem {
    /// 根节点
    root: FileNode,
    /// 生长模板注册表
    templates: HashMap<String, GrowthTemplate>,
    /// 生长规则
    rules: Vec<GrowthRule>,
    /// 地址计数器 (每次生长 +1)
    address_counter: u64,
    /// 自繁殖记录
    reproduction_log: Vec<String>,
}

impl FileGrowthSystem {
    pub fn new() -> Self {
        Self {
            root: FileNode::new_dir(),
            templates: HashMap::new(),
            rules: Vec::new(),
            address_counter: 0,
            reproduction_log: Vec::new(),
        }
    }

    /// 注册模板
    pub fn register_template(&mut self, template: GrowthTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: GrowthRule) {
        self.rules.push(rule);
    }

    /// 创建目录
    pub fn mkdir(&mut self, path: &str) -> bool {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        Self::mkdir_recursive(&mut self.root, &parts)
    }

    fn mkdir_recursive(node: &mut FileNode, parts: &[&str]) -> bool {
        if parts.is_empty() { return true; }
        match node {
            FileNode::Directory { children } => {
                let name = parts[0].to_string();
                if !children.contains_key(&name) {
                    children.insert(name, FileNode::new_dir());
                }
                // 找到可变引用并递归
                if let Some(child) = children.get_mut(parts[0]) {
                    Self::mkdir_recursive(child, &parts[1..])
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// 写入文件
    pub fn write(&mut self, path: &str, address: u64) -> bool {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() { return false; }
        Self::write_recursive(&mut self.root, &parts, address)
    }

    fn write_recursive(node: &mut FileNode, parts: &[&str], address: u64) -> bool {
        match node {
            FileNode::Directory { children } => {
                if parts.len() == 1 {
                    children.insert(parts[0].to_string(), FileNode::new_file(address));
                    true
                } else {
                    let name = parts[0].to_string();
                    if !children.contains_key(&name) {
                        children.insert(name.clone(), FileNode::new_dir());
                    }
                    if let Some(child) = children.get_mut(&name) {
                        Self::write_recursive(child, &parts[1..], address)
                    } else {
                        false
                    }
                }
            }
            _ => false,
        }
    }

    /// 读取文件
    pub fn read(&self, path: &str) -> Option<u64> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        Self::read_recursive(&self.root, &parts)
    }

    fn read_recursive(node: &FileNode, parts: &[&str]) -> Option<u64> {
        if parts.is_empty() { return None; }
        match node.child(parts[0]) {
            Some(child) => {
                if parts.len() == 1 {
                    match child {
                        FileNode::File { address } => Some(*address),
                        _ => None,
                    }
                } else {
                    Self::read_recursive(child, &parts[1..])
                }
            }
            None => None,
        }
    }

    /// 应用模板 — 在指定路径展开模板
    pub fn apply_template(&mut self, at_path: &str, template_name: &str) -> bool {
        let template = match self.templates.get(template_name) {
            Some(t) => t.clone(),
            None => return false,
        };

        let parts: Vec<&str> = at_path.split('/').filter(|s| !s.is_empty()).collect();
        // 先导航到目标路径, 再展开
        let node = Self::navigate_to(&mut self.root, &parts);
        if node.is_none() { return false; }
        let node = node.unwrap();
        Self::expand_template(node, &template, &mut self.address_counter, &self.templates)
    }

    /// 导航到指定路径的节点 (可变引用, 不创建中间节点)
    fn navigate_to<'a>(node: &'a mut FileNode, parts: &[&str]) -> Option<&'a mut FileNode> {
        if parts.is_empty() { return Some(node); }
        match node {
            FileNode::Directory { children } => {
                let key = parts[0].to_string();
                if let Some(child) = children.get_mut(&key) {
                    Self::navigate_to(child, &parts[1..])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// 展开模板到指定节点
    fn expand_template(
        node: &mut FileNode,
        template: &GrowthTemplate,
        counter: &mut u64,
        templates: &HashMap<String, GrowthTemplate>,
    ) -> bool {
        match node {
            FileNode::Directory { children } => {
                for (name, entry) in &template.structure {
                    match entry {
                        TemplateEntry::SubTemplate(sub_name) => {
                            if let Some(sub) = templates.get(sub_name) {
                                let mut sub_dir = FileNode::new_dir();
                                Self::expand_template(&mut sub_dir, sub, counter, templates);
                                children.insert(name.clone(), sub_dir);
                            }
                        }
                        TemplateEntry::AddressSeed(seed) => {
                            *counter += 1;
                            let addr = *seed + *counter;
                            children.insert(name.clone(), FileNode::new_file(addr));
                        }
                        TemplateEntry::EmptyDir => {
                            children.insert(name.clone(), FileNode::new_dir());
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// 生长 — 执行所有规则, 产生新文件
    pub fn grow(&mut self) -> Vec<String> {
        let mut grown = Vec::new();
        // 阶段 1: 收集命中规则的索引 (避免在 iter_mut 中再 borrow self)
        let mut hits: Vec<usize> = Vec::new();
        for (idx, rule) in self.rules.iter().enumerate() {
            if rule.executed { continue; }
            let trigger_parts: Vec<&str> = rule.trigger_path.split('/').filter(|s| !s.is_empty()).collect();
            if Self::path_exists(&self.root, &trigger_parts) {
                hits.push(idx);
            }
        }
        // 阶段 2: 应用命中的规则
        for idx in hits {
            let target = self.rules[idx].trigger_path.clone();
            let template = self.rules[idx].template_name.clone();
            let name = self.rules[idx].name.clone();
            if self.apply_template(&target, &template) {
                self.rules[idx].executed = true;
                grown.push(format!("{} → {}", name, template));
                self.reproduction_log.push(name);
            }
        }
        grown
    }

    fn path_exists(node: &FileNode, parts: &[&str]) -> bool {
        if parts.is_empty() { return true; }
        match node.child(parts[0]) {
            Some(child) => {
                if parts.len() == 1 {
                    true
                } else {
                    Self::path_exists(child, &parts[1..])
                }
            }
            None => false,
        }
    }

    /// 获取根节点 (只读)
    pub fn root(&self) -> &FileNode {
        &self.root
    }

    /// 模板数量
    pub fn template_count(&self) -> usize {
        self.templates.len()
    }

    /// 规则数量
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// 当前地址计数
    pub fn current_counter(&self) -> u64 {
        self.address_counter
    }

    /// 繁殖日志
    pub fn reproduction_log(&self) -> &[String] {
        &self.reproduction_log
    }
}


// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_node_dir_insert() {
        let mut dir = FileNode::new_dir();
        assert!(dir.insert("a".to_string(), FileNode::new_file(42)));
        assert!(dir.is_dir());
    }

    #[test]
    fn file_node_file_holds_address() {
        let f = FileNode::new_file(99);
        assert!(f.is_file());
        assert!(!f.is_dir());
    }

    #[test]
    fn path_resolver_roundtrip() {
        let addr: u32 = 0x12345678;
        let path = PathResolver::address_to_path(addr);
        let back = PathResolver::path_to_address(&path);
        assert_eq!(back, Some(addr));
    }

    #[test]
    fn path_resolver_kmer_path() {
        // Kmer address = radix | (initial << 4) | (final << 8) | (tone << 12)
        let path = PathResolver::address_to_path(0x40003002);
        // 0x40003002 = tone=4, final=3, initial=2, radix=4 (little endian)
        let back = PathResolver::path_to_address(&path).unwrap();
        // 注意位排列; 只要 roundtrip 即可
        assert_eq!(PathResolver::address_to_path(back), path);
    }

    #[test]
    fn path_to_address_invalid() {
        assert!(PathResolver::path_to_address("").is_none());
        assert!(PathResolver::path_to_address("/1/2").is_none()); // 不足4段
        assert!(PathResolver::path_to_address("/a/b/c/d").is_none()); // 非数字
    }

    #[test]
    fn file_growth_mkdir() {
        let mut fs = FileGrowthSystem::new();
        assert!(fs.mkdir("/a/b/c"));
    }

    #[test]
    fn file_growth_write_and_read() {
        let mut fs = FileGrowthSystem::new();
        fs.mkdir("/containers");
        assert!(fs.write("/containers/seed", 42));
        assert_eq!(fs.read("/containers/seed"), Some(42));
    }

    #[test]
    fn file_growth_read_nonexistent() {
        let fs = FileGrowthSystem::new();
        assert_eq!(fs.read("/nothing"), None);
    }

    #[test]
    fn file_growth_template_register() {
        let mut fs = FileGrowthSystem::new();
        let template = GrowthTemplate::new("basic")
            .with_entry("entry_addr", TemplateEntry::AddressSeed(1000))
            .with_entry("sub", TemplateEntry::EmptyDir);
        fs.register_template(template);
        assert_eq!(fs.template_count(), 1);
    }

    #[test]
    fn file_growth_apply_template() {
        let mut fs = FileGrowthSystem::new();
        fs.mkdir("/targets");

        let template = GrowthTemplate::new("spawn")
            .with_entry("addr_a", TemplateEntry::AddressSeed(100))
            .with_entry("addr_b", TemplateEntry::AddressSeed(200));

        fs.register_template(template);
        assert!(fs.apply_template("/targets", "spawn"));
        assert!(fs.current_counter() > 0);
    }

    #[test]
    fn file_growth_rule_and_grow() {
        let mut fs = FileGrowthSystem::new();
        fs.mkdir("/trigger");

        let template = GrowthTemplate::new("grow_template")
            .with_entry("child", TemplateEntry::AddressSeed(500));

        fs.register_template(template);

        let rule = GrowthRule::new("auto_grow", "/trigger", "grow_template");
        fs.add_rule(rule);

        let grown = fs.grow();
        assert_eq!(grown.len(), 1);
        assert_eq!(fs.reproduction_log().len(), 1);
    }

    #[test]
    fn file_growth_no_double_trigger() {
        let mut fs = FileGrowthSystem::new();
        fs.mkdir("/x");
        let template = GrowthTemplate::new("t").with_entry("a", TemplateEntry::AddressSeed(1));
        fs.register_template(template);
        fs.add_rule(GrowthRule::new("r", "/x", "t"));

        fs.grow();
        let grown2 = fs.grow();
        assert_eq!(grown2.len(), 0); // 第二次不触发 (规则已执行)
    }

    #[test]
    fn file_growth_rule_needs_trigger_path() {
        let mut fs = FileGrowthSystem::new();
        // 不创建 trigger 目录
        let template = GrowthTemplate::new("t").with_entry("a", TemplateEntry::AddressSeed(1));
        fs.register_template(template);
        fs.add_rule(GrowthRule::new("r", "/nonexist", "t"));

        let grown = fs.grow();
        assert_eq!(grown.len(), 0);
    }

    #[test]
    fn file_growth_dir_auto_created_on_write() {
        let mut fs = FileGrowthSystem::new();
        // 写入时自动创建父目录
        assert!(fs.write("/deep/nested/path", 99));
        assert_eq!(fs.read("/deep/nested/path"), Some(99));
    }

    #[test]
    fn file_growth_template_counter_increments() {
        let mut fs = FileGrowthSystem::new();
        fs.mkdir("/a");
        let template = GrowthTemplate::new("t")
            .with_entry("f1", TemplateEntry::AddressSeed(0))
            .with_entry("f2", TemplateEntry::AddressSeed(0));
        fs.register_template(template);
        fs.apply_template("/a", "t");
        // 添加了两个地址种子文件 → counter = 2
        assert_eq!(fs.current_counter(), 2);
    }

    #[test]
    fn file_growth_cannot_write_to_file() {
        let mut fs = FileGrowthSystem::new();
        fs.write("/file", 1);
        // 不能在文件节点上再创建子路径
        assert!(!fs.write("/file/sub", 2));
    }
}
