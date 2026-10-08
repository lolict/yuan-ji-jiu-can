// v0.88 符号代数引擎 — KMER 映射表 · 容器四态 · 规则引导器 · 自生长代码生成
// 核心: 编译器不是外部工具, 是从文件系统长出来的
// KMER = 声母-韵母-声调-进制 的四元映射, 符号即代码
// 容器四态 = 注册/工作/时间/思维 四个绝对必要清单
// 规则引导器 = 每一步都有引导器引导完成布局
// 自生长 = 编译器用自己的规则生成自己的扩展

use crate::radix_addressing::Code64;
use std::collections::HashMap;

// ══════════════════════════════════════════════════════════════
// KMER 映射表 — 符号即代码
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Kmer {
    pub initial: u8,
    pub final_v: u8,
    pub tone: u8,
    pub radix: u8,
}

impl Kmer {
    pub fn new(initial: u8, final_v: u8, tone: u8, radix: u8) -> Self {
        Self { initial, final_v, tone, radix }
    }
    pub fn to_address(&self) -> u32 {
        let r = self.radix as u32;
        let i = self.initial as u32;
        let f = self.final_v as u32;
        let t = self.tone as u32;
        r | (i << 4) | (f << 8) | (t << 12)
    }
    pub fn from_address(addr: u32) -> Self {
        Self {
            radix: (addr & 0xF) as u8,
            initial: ((addr >> 4) & 0xFF) as u8,
            final_v: ((addr >> 8) & 0x7F) as u8,
            tone: ((addr >> 12) & 0xFF) as u8,
        }
    }
}

pub struct KmerTable {
    mappings: HashMap<u32, Code64>,
    labels: HashMap<u32, String>,
}

impl KmerTable {
    pub fn new() -> Self {
        Self { mappings: HashMap::new(), labels: HashMap::new() }
    }
    pub fn register(&mut self, kmer: Kmer, code: Code64) {
        self.mappings.insert(kmer.to_address(), code);
    }
    pub fn register_with_label(&mut self, kmer: Kmer, code: Code64, label: &str) {
        let addr = kmer.to_address();
        self.mappings.insert(addr, code);
        self.labels.insert(addr, label.to_string());
    }
    pub fn lookup(&self, kmer: &Kmer) -> Option<Code64> {
        self.mappings.get(&kmer.to_address()).copied()
    }
    pub fn lookup_by_address(&self, addr: u32) -> Option<Code64> {
        self.mappings.get(&addr).copied()
    }
    pub fn get_label(&self, kmer: &Kmer) -> Option<&str> {
        self.labels.get(&kmer.to_address()).map(|s| s.as_str())
    }
    pub fn count(&self) -> usize {
        self.mappings.len()
    }
    pub fn addresses(&self) -> Vec<u32> {
        self.mappings.keys().cloned().collect()
    }
}

// ══════════════════════════════════════════════════════════════
// 容器四态
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ContainerQuadState {
    pub registry: Vec<u32>,
    pub work: Vec<u32>,
    pub time_version: Vec<u32>,
    pub thought: Vec<u32>,
}

impl ContainerQuadState {
    pub fn new() -> Self {
        Self { registry: Vec::new(), work: Vec::new(), time_version: Vec::new(), thought: Vec::new() }
    }
    pub fn register(&mut self, addr: u32) {
        if !self.registry.contains(&addr) { self.registry.push(addr); }
    }
    pub fn assign_work(&mut self, addr: u32) {
        if !self.work.contains(&addr) { self.work.push(addr); }
    }
    pub fn advance_version(&mut self, addr: u32) {
        if !self.time_version.contains(&addr) { self.time_version.push(addr); }
    }
    pub fn link_thought(&mut self, from: u32, to: u32) {
        if !self.thought.contains(&from) { self.thought.push(from); }
        if !self.thought.contains(&to) { self.thought.push(to); }
    }
    pub fn registry_count(&self) -> usize { self.registry.len() }
    pub fn work_count(&self) -> usize { self.work.len() }
    pub fn is_registered(&self, addr: u32) -> bool { self.registry.contains(&addr) }
    pub fn is_busy(&self, addr: u32) -> bool { self.work.contains(&addr) }
}

// ══════════════════════════════════════════════════════════════
// 规则引导器
// ══════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub preconditions: Vec<u32>,
    pub produces: Vec<u32>,
    pub priority: u32,
}

impl Rule {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), preconditions: Vec::new(), produces: Vec::new(), priority: 0 }
    }
    pub fn with_precondition(mut self, addr: u32) -> Self { self.preconditions.push(addr); self }
    pub fn with_output(mut self, addr: u32) -> Self { self.produces.push(addr); self }
    pub fn with_priority(mut self, priority: u32) -> Self { self.priority = priority; self }
    pub fn executable(&self, quad: &ContainerQuadState) -> bool {
        self.preconditions.iter().all(|addr| quad.is_registered(*addr))
    }
}

pub struct RuleGuide {
    rules: Vec<Rule>,
    execution_log: Vec<(String, Vec<u32>)>,
}

impl RuleGuide {
    pub fn new() -> Self {
        Self { rules: Vec::new(), execution_log: Vec::new() }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
        self.rules.sort_by_key(|r| r.priority);
    }
    pub fn executable_rules(&self, quad: &ContainerQuadState) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.executable(quad)).collect()
    }
    pub fn execute(&mut self, rule_name: &str, quad: &mut ContainerQuadState) -> Option<Vec<u32>> {
        let rule_idx = self.rules.iter().position(|r| r.name == rule_name)?;
        let rule = &self.rules[rule_idx];
        if !rule.executable(quad) { return None; }
        let produced = rule.produces.clone();
        for addr in &produced {
            quad.register(*addr);
            quad.assign_work(*addr);
        }
        self.execution_log.push((rule.name.to_string(), produced.clone()));
        Some(produced)
    }
    pub fn execute_all(&mut self, quad: &mut ContainerQuadState) -> Vec<(String, Vec<u32>)> {
        let mut results = Vec::new();
        let names: Vec<String> = self.rules.iter().map(|r| r.name.clone()).collect();
        for name in names {
            if let Some(outputs) = self.execute(&name, quad) {
                results.push((name, outputs));
            }
        }
        results
    }
    pub fn rule_count(&self) -> usize { self.rules.len() }
    pub fn log(&self) -> &[(String, Vec<u32>)] { &self.execution_log }
}

// ══════════════════════════════════════════════════════════════
// 自生长代码生成器
// ══════════════════════════════════════════════════════════════

pub struct GrowthResult {
    pub new_entries: Vec<(u32, Code64)>,
    pub new_rule_names: Vec<String>,
    pub growth_points: u64,
}

pub struct SelfGrowthGenerator {
    kmer_table: KmerTable,
    quad: ContainerQuadState,
    guide: RuleGuide,
    generations: u64,
}

impl SelfGrowthGenerator {
    pub fn new() -> Self {
        Self {
            kmer_table: KmerTable::new(),
            quad: ContainerQuadState::new(),
            guide: RuleGuide::new(),
            generations: 0,
        }
    }
    pub fn seed_initial(&mut self, entries: Vec<(Kmer, Code64, &str)>) {
        for (kmer, code, label) in entries {
            self.kmer_table.register_with_label(kmer, code, label);
            self.quad.register(kmer.to_address());
        }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.guide.add_rule(rule);
    }
    pub fn step(&mut self) -> GrowthResult {
        self.generations += 1;
        let results = self.guide.execute_all(&mut self.quad);
        let mut new_entries = Vec::new();
        let mut new_rule_names = Vec::new();
        let mut growth_points: u64 = 0;
        for (name, outputs) in &results {
            new_rule_names.push(name.clone());
            for addr in outputs {
                let kmer = Kmer::from_address(*addr);
                if let Some(code) = self.kmer_table.lookup(&kmer) {
                    new_entries.push((*addr, code));
                }
                growth_points = growth_points.saturating_add(1);
            }
        }
        for addr in &self.quad.registry.clone() {
            self.quad.advance_version(*addr);
        }
        GrowthResult { new_entries, new_rule_names, growth_points }
    }
    pub fn kmer_table(&self) -> &KmerTable { &self.kmer_table }
    pub fn quad(&self) -> &ContainerQuadState { &self.quad }
    pub fn guide(&self) -> &RuleGuide { &self.guide }
    pub fn generations(&self) -> u64 { self.generations }
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::radix_addressing::Code64;

    fn mk_code(val: u32) -> Code64 {
        Code64::new(1, 1, 1, val)
    }

    #[test]
    fn kmer_to_address_roundtrip() {
        let kmer = Kmer::new(5, 10, 3, 4);
        let addr = kmer.to_address();
        let decoded = Kmer::from_address(addr);
        assert_eq!(decoded.initial, 5);
        assert_eq!(decoded.final_v, 10);
        assert_eq!(decoded.tone, 3);
        assert_eq!(decoded.radix, 4);
    }

    #[test]
    fn kmer_address_radix_first() {
        let kmer = Kmer::new(0, 0, 0, 7);
        assert_eq!(kmer.to_address() & 0xF, 7);
    }

    #[test]
    fn kmer_table_register_and_lookup() {
        let mut table = KmerTable::new();
        let kmer = Kmer::new(1, 2, 3, 4);
        table.register(kmer, mk_code(42));
        let result = table.lookup(&kmer);
        assert!(result.is_some());
        assert_eq!(result.unwrap().value(), 42);
    }

    #[test]
    fn kmer_table_with_label() {
        let mut table = KmerTable::new();
        let kmer = Kmer::new(1, 2, 3, 4);
        table.register_with_label(kmer, mk_code(1), "INIT");
        assert_eq!(table.get_label(&kmer), Some("INIT"));
    }

    #[test]
    fn quad_state_register() {
        let mut quad = ContainerQuadState::new();
        quad.register(100);
        quad.register(200);
        assert_eq!(quad.registry_count(), 2);
        assert!(quad.is_registered(100));
        assert!(!quad.is_registered(999));
    }

    #[test]
    fn quad_state_no_duplicate() {
        let mut quad = ContainerQuadState::new();
        quad.register(100);
        quad.register(100);
        assert_eq!(quad.registry_count(), 1);
    }

    #[test]
    fn quad_state_assign_work() {
        let mut quad = ContainerQuadState::new();
        quad.assign_work(50);
        assert!(quad.is_busy(50));
    }

    #[test]
    fn quad_state_thought_link() {
        let mut quad = ContainerQuadState::new();
        quad.link_thought(10, 20);
        assert!(quad.thought.contains(&10));
        assert!(quad.thought.contains(&20));
    }

    #[test]
    fn rule_executable_check() {
        let mut quad = ContainerQuadState::new();
        quad.register(1);
        let rule = Rule::new("test").with_precondition(1);
        assert!(rule.executable(&quad));
        let rule2 = Rule::new("test2").with_precondition(2);
        assert!(!rule2.executable(&quad));
    }

    #[test]
    fn rule_guide_execute() {
        let mut guide = RuleGuide::new();
        let mut quad = ContainerQuadState::new();
        quad.register(1);
        guide.add_rule(Rule::new("grow").with_precondition(1).with_output(100));
        let result = guide.execute("grow", &mut quad);
        assert!(result.is_some());
        assert!(quad.is_registered(100));
    }

    #[test]
    fn rule_guide_execute_all() {
        let mut guide = RuleGuide::new();
        let mut quad = ContainerQuadState::new();
        quad.register(1);
        quad.register(2);
        guide.add_rule(Rule::new("a").with_precondition(1).with_output(100));
        guide.add_rule(Rule::new("b").with_precondition(2).with_output(200));
        let results = guide.execute_all(&mut quad);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn growth_generator_seed() {
        let mut gen = SelfGrowthGenerator::new();
        gen.seed_initial(vec![
            (Kmer::new(1, 1, 1, 1), mk_code(10), "种子1"),
            (Kmer::new(2, 2, 2, 2), mk_code(20), "种子2"),
        ]);
        assert_eq!(gen.kmer_table().count(), 2);
        assert_eq!(gen.quad().registry_count(), 2);
    }

    #[test]
    fn growth_generator_step() {
        let mut gen = SelfGrowthGenerator::new();
        gen.seed_initial(vec![(Kmer::new(1, 1, 1, 1), mk_code(10), "种子")]);
        let seed_addr = Kmer::new(1, 1, 1, 1).to_address();
        gen.add_rule(Rule::new("grow").with_precondition(seed_addr).with_output(999).with_priority(1));
        let result = gen.step();
        assert_eq!(result.growth_points, 1);
    }

    #[test]
    fn growth_generations_count() {
        let mut gen = SelfGrowthGenerator::new();
        gen.seed_initial(vec![(Kmer::new(1, 1, 1, 1), mk_code(10), "x")]);
        gen.step();
        gen.step();
        assert_eq!(gen.generations(), 2);
    }

    #[test]
    fn rule_not_executed_without_precondition() {
        let mut guide = RuleGuide::new();
        let mut quad = ContainerQuadState::new();
        guide.add_rule(Rule::new("needs_1").with_precondition(1));
        let result = guide.execute("needs_1", &mut quad);
        assert!(result.is_none());
    }

    #[test]
    fn guide_log_records() {
        let mut guide = RuleGuide::new();
        let mut quad = ContainerQuadState::new();
        quad.register(1);
        guide.add_rule(Rule::new("r1").with_precondition(1).with_output(10));
        guide.execute_all(&mut quad);
        assert_eq!(guide.log().len(), 1);
    }

    #[test]
    fn quad_advance_version() {
        let mut quad = ContainerQuadState::new();
        quad.register(1);
        quad.advance_version(1);
        assert_eq!(quad.time_version.len(), 1);
        quad.advance_version(1);
        assert_eq!(quad.time_version.len(), 1);
    }
}
