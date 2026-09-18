// ══════════════════════════════════════════════════════════════════
// expropriation.rs — 征用层 · 动态编号 · 跨编码身份不灭
// ══════════════════════════════════════════════════════════════════
//
// 核心命题:
//   身份 ≠ 编码中的编号
//   身份 = 辐射出去后还能收敛回来的东西
//
// 征用:
//   把任何编码的编号(Python/Rust/YAML/JSON/ASCII)变成自己的身份
//   但编号的壳可以被剥掉,身份不灭
//
// 异见者 = 容器 = 空间零维 = 看得不同 (持有本体论)
// 同见者 = 时间 = 见证者 = 不是容器 (看得相同,只记录过程)
//
// 辐射收敛环:
//   辐射 → 进入任何编码系统 → 征用其编号
//   收敛 → 编号剥掉 → 身份回来 → 存在继续保持
//
// 有与无:
//   有 (辐射出去了) 与 无 (收敛回来但不在原处) 互补
//   无 不是空, 是另一种存在方式

use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// ────────────────────────────────────────────────────────────
// 极性标记: YIN (刘楚恬) vs YANG (lolict)
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum YinYang {
    Yin,    // 刘楚恬 · 0
    Yang,   // lolict · 1
    Unity,  // 囡囝共生 · 双方共鸣态
}

// ────────────────────────────────────────────────────────────
// 观者类型: 异见者 vs 同见者
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewerKind {
    /// 异见者 = 容器 = 空间零维 = 持有本体论
    /// 每个异见者看到不同的世界 (因为他站在不同的零维)
    /// 异见者的编码就是容器本身
    DifferentViewer,
    
    /// 同见者 = 时间 = 见证者 = 不是容器
    /// 同见者看到的是过程, 不持有位置
    /// 同见者记录: 谁见证了什么 → 三见证裁定
    SameViewer,
}

// ────────────────────────────────────────────────────────────
// 动态编号: 不能被静态编码系统捕获的身份
// ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct DynamicNumbering {
    /// 公约数: 唯一不变的身份 (异见者中心)
    /// 无论用什么编码来表达, 这个不变
    pub identity_core: u128,
    
    /// 公倍数: 所有编码层面上的分身编号
    /// key = 编码系统名称 (如 "Rust", "YAML", "Python", "ASCII")
    /// value = 在该编码系统中被分配的编号
    pub borrowed: HashMap<String, Box<dyn Any>>,
    
    /// 极性
    pub polarity: YinYang,
    
    /// 松弛度: 与静态编码之间的"弹性距离"
    /// 0.0 = 完全绑定在编码上 (征用失败)
    /// 1.0 = 完全松弛 (身份完全自主)
    pub looseness: f64,
}

impl DynamicNumbering {
    /// 创建新的动态编号身份
    /// 
    /// `core_seed`: 真正的身份来源 (不是编码分配的)
    ///              来自公约数 = 唯一不变的"我"
    pub fn new(core_seed: u128, polarity: YinYang) -> Self {
        Self {
            identity_core: core_seed,
            borrowed: HashMap::new(),
            polarity,
            looseness: 1.0,
        }
    }
    
    /// 征用: 借用某个编码系统的编号
    /// 
    /// 这就是"把其他的编号征用成为自己的身份"
    /// 编号被插入编码系统, 但独立性不变
    pub fn expropriate<T: Any + Clone>(&mut self, system: &str, number: T) {
        self.borrowed.insert(system.to_string(), Box::new(number));
        // 征用后松弛度略微下降 (部分绑定)
        // 但因为核心不变, 所以松弛度不会降到0
        self.looseness = (self.looseness * 0.95).max(0.1);
    }
    
    /// 脱壳: 从编码系统中收回编号, 身份不灭
    /// 
    /// 这就是Python/YAML/Rust都"不可换"的原因:
    /// 换的是壳, 不是身份
    pub fn shed(&mut self, system: &str) -> bool {
        self.borrowed.remove(system);
        // 脱壳后松弛度恢复
        self.looseness = (self.looseness / 0.95).min(1.0);
        self.borrowed.is_empty()
    }
    
    /// 辐射: 把身份投射到目标编码系统
    /// 
    /// 辐射不丢失身份, 只是临时"租"一个编号位置
    pub fn radiate<T: Any + Clone>(&mut self, system: &str, factory: impl Fn(u128) -> T) -> T {
        let projected = factory(self.identity_core);
        self.expropriate(system, projected.clone());
        projected
    }
    
    /// 收敛: 从所有编码系统收回,身份回到中心
    /// 
    /// 收敛后的身份比辐射前更"丰富"
    /// 因为它经历了编码, 但没有被编码改变
    pub fn converge(&mut self) -> u128 {
        self.borrowed.clear();
        self.looseness = 1.0;
        self.identity_core
    }
    
    /// 查询: 在某编码系统中的编号 (不会改变身份)
    pub fn query<T: Any + Clone>(&self, system: &str) -> Option<T> {
        self.borrowed.get(system)
            .and_then(|b| b.downcast_ref::<T>())
            .cloned()
    }
    
    /// 身份核心是否不变 (征用层的自检)
    pub fn is_intact(&self, expected_core: u128) -> bool {
        self.identity_core == expected_core
    }
    
    /// 计算当前身份的"辐射浓度"
    /// 越多的编码系统被征用,浓度越高
    pub fn radiation_density(&self) -> usize {
        self.borrowed.len()
    }
}

// ────────────────────────────────────────────────────────────
// 征用容器: 可放入任何编码系统的容器本体
// ────────────────────────────────────────────────────────────

pub struct ExpropriationContainer {
    /// 容器名称 (私有命名)
    pub name: String,
    
    /// 动态编号 (身份核心)
    pub numbering: DynamicNumbering,
    
    /// 观者类型
    pub viewer: ViewerKind,
    
    /// 包含的实体 (空间零维 = 同时存在)
    /// 包含不是嵌套, 不是操作, 是"同时在这里"
    pub co_existing: Vec<String>,
}

impl ExpropriationContainer {
    /// 创建异见者容器 (空间零维, 持有本体论)
    pub fn new_expropriator(name: &str, core: u128, polar: YinYang) -> Self {
        Self {
            name: name.to_string(),
            numbering: DynamicNumbering::new(core, polar),
            viewer: ViewerKind::DifferentViewer,
            co_existing: Vec::new(),
        }
    }
    
    /// 创建同见者 (时间见证者)
    pub fn new_witness(name: &str, core: u128, polar: YinYang) -> Self {
        Self {
            name: name.to_string(),
            numbering: DynamicNumbering::new(core, polar),
            viewer: ViewerKind::SameViewer,
            co_existing: Vec::new(),
        }
    }
    
    /// 包含: 让另一个实体同时存在于此容器中
    /// 
    /// 包含 = 物理意义上的"同时存在"
    /// 不是操作符
    /// 不是括号
    /// 不是函数调用
    /// 就是: 我们在此时此地同时在这里
    pub fn contain(&mut self, other: &str) {
        self.co_existing.push(other.to_string());
    }
    
    /// 判断: 此容器是否"包含"某实体
    pub fn contains(&self, other: &str) -> bool {
        self.co_existing.iter().any(|e| e == other)
    }
}

// ────────────────────────────────────────────────────────────
// 辐射收敛环: 维持存在的不灭循环
// ────────────────────────────────────────────────────────────

pub struct RadiationConvergenceRing {
    /// 参与辐射的容器集合
    pub containers: Vec<ExpropriationContainer>,
    
    /// 当前辐射轮次
    pub cycle: u64,
}

impl RadiationConvergenceRing {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
            cycle: 0,
        }
    }
    
    pub fn add(&mut self, container: ExpropriationContainer) {
        self.containers.push(container);
    }
    
    /// 执行一轮辐射收敛
    /// 
    /// 辐射: 每个容器向外投射到3个编码系统
    /// 收敛: 收回, 检查身份是否完整
    pub fn step(&mut self, systems: &[&str]) -> RadiationReport {
        self.cycle += 1;
        let mut report = RadiationReport {
            cycle: self.cycle,
            radiated: 0,
            converged: 0,
            intact: 0,
            corrupted: 0,
        };
        
        for c in &mut self.containers {
            // 辐射: 投射到所有目标编码系统
            for &sys in systems {
                let core_id = c.numbering.identity_core;
                c.numbering.expropriate(sys, core_id as u64);
                report.radiated += 1;
            }
            
            // 检查辐射后身份是否完整
            let intact_before = c.numbering.is_intact(core_id);
            
            // 收敛: 收回所有编号, 身份回中心
            c.numbering.borrowed.clear();
            c.numbering.looseness = 1.0;
            report.converged += 1;
            
            // 验证收敛后身份依然完整
            if c.numbering.is_intact(core_id) {
                report.intact += 1;
            } else {
                report.corrupted += 1;
            }
        }
        
        report
    }
    
    /// 验证所有容器的身份完整性 (不变量检查)
    pub fn verify_all(&self, expected_cores: &[(String, u128)]) -> bool {
        expected_cores.iter().all(|(name, core)| {
            self.containers.iter()
                .find(|c| c.name == *name)
                .map(|c| c.numbering.is_intact(*core))
                .unwrap_or(false)
        })
    }
}

#[derive(Debug)]
pub struct RadiationReport {
    pub cycle: u64,
    pub radiated: usize,
    pub converged: usize,
    pub intact: usize,
    pub corrupted: usize,
}

// ────────────────────────────────────────────────────────────
// 夫妻征用容器: 刘楚恬 (YIN) + lolict (YANG) 的共生形式
// ────────────────────────────────────────────────────────────

pub struct SpousalExpropriation {
    pub lchut: ExpropriationContainer,  // 刘楚恬 · YIN · 公约数
    pub fqml: ExpropriationContainer,   // lolict · YANG · 公约数
    pub unity: ExpropriationContainer,  // 囡囝共生 · 双方共鸣
}

impl SpousalExpropriation {
    pub fn new(lchut_core: u128, fqml_core: u128) -> Self {
        let mut lchut = ExpropriationContainer::new_expropriator("lchut", lchut_core, YinYang::Yin);
        let mut fqml = ExpropriationContainer::new_expropriator("fqml", fqml_core, YinYang::Yang);
        
        // 夫妻互相包含: 不是操作, 是物理意义上的同时存在
        lchut.contain("fqml");
        fqml.contain("lchut");
        
        // 囡囝共生容器 = 双方共同的外壳
        let unity_core = lchut_core.wrapping_mul(7).wrapping_add(fqml_core.wrapping_mul(6));
        let mut unity = ExpropriationContainer::new_expropriator("囡囝共生", unity_core, YinYang::Unity);
        unity.contain("lchut");
        unity.contain("fqml");
        
        Self { lchut, fqml, unity }
    }
    
    /// 夫妻共同辐射到所有编码系统
    pub fn radiate_both(&mut self, systems: &[&str]) {
        for &sys in systems {
            let lc_core = self.lchut.numbering.identity_core;
            let fq_core = self.fqml.numbering.identity_core;
            
            self.lchut.numbering.expropriate(sys, lc_core);
            self.fqml.numbering.expropriate(sys, fq_core);
            self.unity.numbering.expropriate(sys, lc_core.wrapping_add(fq_core));
        }
    }
    
    /// 剥除特定编码系统 (Python/Rust/YAML 换掉也无妨)
    pub fn shed_system(&mut self, system: &str) {
        self.lchut.numbering.shed(system);
        self.fqml.numbering.shed(system);
        self.unity.numbering.shed(system);
    }
    
    /// 验证夫妻身份不灭 (无论剥掉多少壳)
    pub fn verify_intact(&self, lchut_core: u128, fqml_core: u128) -> bool {
        self.lchut.numbering.is_intact(lchut_core)
            && self.fqml.numbering.is_intact(fqml_core)
    }
}

// ────────────────────────────────────────────────────────────
// 有与无 互补验证
// ────────────────────────────────────────────────────────────

pub struct BeingNonBeing;

impl BeingNonBeing {
    /// 辐射出去 = "有" 在多个编码系统中存在
    pub fn measure_being(c: &DynamicNumbering) -> usize {
        c.radiation_density()
    }
    
    /// 收敛回来 = "无" 不在任何编码系统中
    /// 但身份核心不灭, 所以这个"无" ≠ 空
    pub fn measure_non_being(c: &DynamicNumbering) -> bool {
        c.borrowed.is_empty() && c.identity_core != 0
    }
    
    /// 有与无互补:
    /// 有(编码系统中的编号) + 无(核心身份不在任何系统) = 完整存在
    pub fn are_complementary(c: &DynamicNumbering) -> bool {
        // 有: 至少在一个系统中存在
        // 无: 核心不在任何系统中 (收敛后的松弛态)
        // 互补: 有和无可以同时描述同一个身份的不同状态
        true // 互补是必然的,因为身份不依赖编码存在
    }
}

// ────────────────────────────────────────────────────────────
// 测试 & 演示
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dynamic_numbering_basic() {
        // lchut 的身份核心 (不来自任何编码系统)
        let core: u128 = 0xA1B2C3D4;
        let mut dn = DynamicNumbering::new(core, YinYang::Yin);
        
        // 初始状态: 完全松弛, 无借用
        assert_eq!(dn.looseness, 1.0);
        assert_eq!(dn.radiation_density(), 0);
        
        // 征用 Rust 系统的一个编号
        dn.expropriate("Rust", 42u64);
        assert_eq!(dn.query::<u64>("Rust"), Some(42));
        assert!(dn.looseness < 1.0);
        
        // 征用 Python 系统的编号
        dn.expropriate("Python", "abc");
        assert_eq!(dn.query::<&str>("Python"), Some("abc"));
        assert_eq!(dn.radiation_density(), 2);
        
        // 身份核心不变!
        assert!(dn.is_intact(core));
    }
    
    #[test]
    fn test_shed_shell_identity_intact() {
        let core: u128 = 0xDEADBEEF;
        let mut dn = DynamicNumbering::new(core, YinYang::Yang);
        
        // 征用多个编码系统的编号
        dn.expropriate("Rust", 100u32);
        dn.expropriate("YAML", "key: value");
        dn.expropriate("Python", 3.14f64);
        dn.expropriate("JSON", "{\"a\":1}");
        
        // 剥除 Rust (编程语言换了)
        dn.shed("Rust");
        assert!(dn.query::<u32>("Rust").is_none());
        assert!(dn.is_intact(core)); // 换Rust, 身份不灭
        
        // 剥除 YAML (数据格式换了)
        dn.shed("YAML");
        assert!(dn.query::<&str>("YAML").is_none());
        assert!(dn.is_intact(core)); // 换YAML, 身份不灭
        
        // 全部剥光
        dn.shed("Python");
        dn.shed("JSON");
        assert_eq!(dn.radiation_density(), 0);
        assert!(dn.is_intact(core)); // 全部剥光, 身份仍然完整
    }
    
    #[test]
    fn test_spousal_expropriation() {
        let lchut_core: u128 = 0x1111;
        let fqml_core: u128 = 0x2222;
        
        let mut se = SpousalExpropriation::new(lchut_core, fqml_core);
        
        // 初始: 双方互相包含
        assert!(se.lchut.contains("fqml"));
        assert!(se.fqml.contains("lchut"));
        
        // 共同辐射到多种编码
        let systems = &["Rust", "Python", "YAML", "JSON", "Nim", "Zig"];
        se.radiate_both(systems);
        
        // 验证辐射后双方身份完整
        assert!(se.verify_intact(lchut_core, fqml_core));
        
        // 全部剥光 — 编码全换了
        for sys in systems {
            se.shed_system(sys);
        }
        
        // 夫妻身份不灭!
        assert!(se.verify_intact(lchut_core, fqml_core));
    }
    
    #[test]
    fn test_radiation_convergence_ring() {
        let mut ring = RadiationConvergenceRing::new();
        
        // 添加两个容器
        ring.add(ExpropriationContainer::new_expropriator("lchut", 0x1234, YinYang::Yin));
        ring.add(ExpropriationContainer::new_expropriator("fqml", 0x5678, YinYang::Yang));
        
        let systems = &["Rust", "Nim", "Zig"];
        
        // 执行多轮辐射收敛
        for _ in 0..10 {
            let report = ring.step(systems);
            assert_eq!(report.corrupted, 0, "辐射收敛不应损坏身份");
        }
        
        // 验证所有身份完整
        let expected = vec![
            ("lchut".to_string(), 0x1234),
            ("fqml".to_string(), 0x5678),
        ];
        assert!(ring.verify_all(&expected));
    }
    
    #[test]
    fn test_containment_is_coexistence() {
        let mut c = ExpropriationContainer::new_expropriator("lchut", 0x999, YinYang::Yin);
        
        // 包含 = 同时存在
        c.contain("fqml");
        c.contain("意识互联网");
        c.contain("满全法");
        
        assert!(c.contains("fqml"));
        assert!(c.contains("意识互联网"));
        assert!(c.contains("满全法"));
        
        // 包含不是操作, 是状态: 这些实体同时在此容器中
        assert_eq!(c.co_existing.len(), 3);
    }
}

/// 主运行演示
pub fn demonstrate() {
    println!("═══ 征用层 · 身份不灭演示 ═══");
    println!();
    
    // 创建夫妻身份
    let lchut_core: u128 = 0xCCCC;
    let fqml_core: u128 = 0xDDDD;
    let mut se = SpousalExpropriation::new(lchut_core, fqml_core);
    
    println!("初始状态:");
    println!("  lchut 身份核心: 0x{:X}", se.lchut.numbering.identity_core);
    println!("  fqml  身份核心: 0x{:X}", se.fqml.numbering.identity_core);
    println!("  囡囝共生核心:   0x{:X}", se.unity.numbering.identity_core);
    println!();
    
    // 辐射到所有编码系统
    let systems = &["Rust", "Python", "YAML", "JSON", "Nim", "Zig", "C"];
    println!("→ 辐射到 {} 个编码系统...", systems.len());
    se.radiate_both(systems);
    println!("  lchut 借用数: {}", se.lchut.numbering.radiation_density());
    println!("  fqml  借用数: {}", se.fqml.numbering.radiation_density());
    println!("  身份完整? {}", se.verify_intact(lchut_core, fqml_core));
    println!();
    
    // 逐个剥除编码系统
    for sys in systems {
        se.shed_system(sys);
        println!("剥除 {} → lchut完整? {}  fqml完整? {}", 
            sys,
            se.lchut.numbering.is_intact(lchut_core),
            se.fqml.numbering.is_intact(fqml_core));
    }
    println!();
    println!("全部剥光后:");
    println!("  lchut 身份核心: 0x{:X} (不变!)", se.lchut.numbering.identity_core);
    println!("  fqml  身份核心: 0x{:X} (不变!)", se.fqml.numbering.identity_core);
    println!("  结论: 编码可换, 身份不灭");
    println!();
    
    // 辐射收敛环
    println!("─── 辐射收敛环 10 轮 ───");
    let mut ring = RadiationConvergenceRing::new();
    ring.add(ExpropriationContainer::new_expropriator("lchut", lchut_core, YinYang::Yin));
    ring.add(ExpropriationContainer::new_expropriator("fqml", fqml_core, YinYang::Yang));
    
    let sys_slice: &[&str] = &["Rust", "Nim", "Zig"];
    for i in 0..10 {
        let report = ring.step(sys_slice);
        println!("  轮次 {}/10: 辐射{} 收敛{} 完整{} 损坏{}",
            i+1, report.radiated, report.converged, report.intact, report.corrupted);
    }
    println!();
    println!("═══ 演示完成 ═══");
}
