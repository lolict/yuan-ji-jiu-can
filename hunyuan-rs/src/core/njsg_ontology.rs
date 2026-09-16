// ═══════════════════════════════════════════════════════════════════════════
// njsg_ontology.rs — 囡囝共生 (NJSG) 本体论存在性引擎
// 异见=零维容器 / 同见=一维时间 / 有/无互补=频率守恒 / 辐射-收敛=递归迭代
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 存在性公理）：
//
//   存在维持律：有 ↔ 无 相互补充，使起止频率不消散。
//   异见异空律：异见是空间零维——唯一容纳本体论的容器。
//   同见同时律：同见是一维时间——时间本体的见证者。
//   辐射收敛律：向外辐射 + 向内收敛 = 迭代 = 递归 = 频率。
//   叠加延伸律：静态 ↔ 动态相互延伸，形成叠加态。
//
// 存在消散条件：
//   若有无不互补 → 起止拉开距离
//   若起止拉开 → 辐射收敛无法迭代
//   若辐射收敛无法迭代 → 频率消失
//   若频率消失 → 失去动力、失去时间、存在消散
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

/// 本体论存在原语
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OntPrim {
    /// 异见 = 零维容器 = 空间容纳者
    Dissent,
    /// 同见 = 一维时间 = 时间见证者
    Consensus,
    /// 有 = 存在
    Being,
    /// 无 = 非存在
    NonBeing,
    /// 辐射 = 向外扩散
    Radiation,
    /// 收敛 = 向内收缩
    Convergence,
    /// 叠加态 = 静态与动态相互延伸
    Superposition,
    /// 频率 = 生命周期维持信号
    Frequency,
    /// 起点
    StartPoint,
    /// 终点
    EndPoint,
}

impl OntPrim {
    pub fn to_chinese(&self) -> &'static str {
        match self {
            OntPrim::Dissent => "异见",
            OntPrim::Consensus => "同见",
            OntPrim::Being => "有",
            OntPrim::NonBeing => "无",
            OntPrim::Radiation => "辐射",
            OntPrim::Convergence => "收敛",
            OntPrim::Superposition => "叠加态",
            OntPrim::Frequency => "频率",
            OntPrim::StartPoint => "起点",
            OntPrim::EndPoint => "终点",
        }
    }

    /// 该原语的维度归属
    pub fn dimension(&self) -> u8 {
        match self {
            OntPrim::Dissent => 0,       // 零维
            OntPrim::Consensus => 1,     // 一维
            OntPrim::Being | OntPrim::NonBeing => 1,
            OntPrim::Radiation | OntPrim::Convergence => 2, // 二维交互
            OntPrim::Superposition | OntPrim::Frequency => 3, // 三维涌现
            OntPrim::StartPoint | OntPrim::EndPoint => 1,
        }
    }
}

/// 存在频率——维持存在的振荡信号
#[derive(Debug, Clone)]
pub struct ExistenceFreq {
    pub amplitude: f64,     // 幅度 (有↔无的互补强度)
    pub phase: f64,         // 相位 (0~2π 循环)
    pub tick: u64,          // 当前时间
    pub alive: bool,        // 是否仍维持存在
    pub recursion_depth: u32, // 辐射-收敛递归深度
}

impl ExistenceFreq {
    pub fn new(amplitude: f64) -> Self {
        Self {
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: 0.0,
            tick: 0,
            alive: true,
            recursion_depth: 0,
        }
    }

    /// 推进一次振荡（辐射→收敛→辐射）
    pub fn step(&mut self) -> ExistenceStep {
        // 相位推进
        self.phase = (self.phase + 0.1) % (2.0 * std::f64::consts::PI);
        self.tick += 1;

        // 判断当前是辐射相还是收敛相
        let is_radiation = self.phase.cos() > 0.0; // cos>0 前半周期=扩散
        let step_type = if is_radiation {
            ExistenceStep::Radiation
        } else {
            ExistenceStep::Convergence
        };

        // 递归深度递增
        self.recursion_depth += 1;

        // 存在衰减：如果幅度降到阈值以下，存在消散
        // 衰减因子 = 0.995 （乘法衰减 — 更快消散）
        self.amplitude *= 0.995;
        if self.amplitude < 0.001 {
            self.alive = false;
        }

        step_type
    }

    /// 补充存在（加法注入）——只有当有/无互补时才有效
    pub fn replenish(&mut self, bei: bool, fei: bool) -> bool {
        if bei && fei {
            // 有无互补 → 补充存在
            self.amplitude = (self.amplitude + 0.01).min(1.0);
            true
        } else {
            // 有无不互补 → 无法补充
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistenceStep {
    Radiation,
    Convergence,
}

/// 本体论存在性验证规则
#[derive(Debug, Clone)]
pub struct OntRule {
    pub name: String,
    pub description: String,
}

/// NJSG 本体论引擎
pub struct NJsgOntology {
    /// 存在频率表：每个容器的存在信号
    existences: HashMap<u64, ExistenceFreq>,
    /// 有↔无互补状态表
    bei_fei_balance: HashMap<u64, (bool, bool)>,
    /// 规则集
    rules: Vec<OntRule>,
}

impl NJsgOntology {
    pub fn new() -> Self {
        let rules = vec![
            OntRule {
                name: "异见-零维容器律".into(),
                description: "异见是空间的零维，是唯一容纳本体论的容器。离开此容器，同见者的流动性不归本体所有。".into(),
            },
            OntRule {
                name: "同见-一维时间律".into(),
                description: "同见是一维时间，是时间本体的见证者。在包含本体的一维时间范畴内，同见者成为本体的时间内存，频率守恒。".into(),
            },
            OntRule {
                name: "有无互补律".into(),
                description: "有与无必须相辅相成，否则起点与终点拉开距离，辐射和收敛无法迭代递归，频率消散，存在失去维持。".into(),
            },
            OntRule {
                name: "辐射收敛迭代律".into(),
                description: "向外辐射与向内收敛交替发生，形成迭代和递归，产生频率。频率是维持多元生命、多元维度回归性和辐射逻辑的基底。".into(),
            },
            OntRule {
                name: "叠加态延伸律".into(),
                description: "静态与动态相互延伸形成叠加态。没有叠加态，存在失去频率，无法维持。".into(),
            },
        ];

        Self {
            existences: HashMap::new(),
            bei_fei_balance: HashMap::new(),
            rules,
        }
    }

    /// 注册一个需要维持存在的容器
    pub fn register(&mut self, id: u64, initial_amplitude: f64) {
        self.existences.insert(id, ExistenceFreq::new(initial_amplitude));
        self.bei_fei_balance.insert(id, (true, true)); // 初始有↔无互补
    }

    /// 设置某容器的有↔无互补状态
    pub fn set_complement(&mut self, id: u64, bei: bool, fei: bool) -> Result<(), String> {
        if !self.existences.contains_key(&id) {
            return Err(format!("container {} not registered", id));
        }
        self.bei_fei_balance.insert(id, (bei, fei));
        Ok(())
    }

    /// 推进一帧：所有容器的存在频率前进一步
    pub fn step_frame(&mut self) -> Vec<(u64, ExistenceStep, bool)> {
        let mut results = Vec::new();
        let ids: Vec<u64> = self.existences.keys().copied().collect();

        for id in ids {
            let alive_before = self.existences[&id].alive;
            if !alive_before { continue; }

            // 有无互补 → 补充存在
            let (bei, fei) = self.bei_fei_balance[&id];
            if let Some(freq) = self.existences.get_mut(&id) {
                freq.replenish(bei, fei);
                let step = freq.step();
                results.push((id, step, freq.alive));
            }
        }
        results
    }

    /// 检查指定容器的存在性状态
    pub fn existence_state(&self, id: u64) -> Option<ExistenceState> {
        self.existences.get(&id).map(|freq| {
            let (bei, fei) = self.bei_fei_balance.get(&id).copied().unwrap_or((false, false));
            ExistenceState {
                alive: freq.alive,
                amplitude: freq.amplitude,
                phase: freq.phase,
                recursion_depth: freq.recursion_depth,
                bei_fei_balanced: bei && fei,
                can_iterate: bei && fei && freq.amplitude > 0.1,
                risk_dissipation: freq.amplitude < 0.2,
            }
        })
    }

    /// 获取所有维持中的存在
    pub fn alive_count(&self) -> usize {
        self.existences.values().filter(|f| f.alive).count()
    }

    /// 获取所有消散的存在
    pub fn dissipated_count(&self) -> usize {
        self.existences.values().filter(|f| !f.alive).count()
    }

    /// 获取规则集
    pub fn rules(&self) -> &[OntRule] { &self.rules }
}

/// 存在状态快照
#[derive(Debug, Clone, Copy)]
pub struct ExistenceState {
    pub alive: bool,
    pub amplitude: f64,
    pub phase: f64,
    pub recursion_depth: u32,
    pub bei_fei_balanced: bool,
    pub can_iterate: bool,
    pub risk_dissipation: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existence_survives_with_complement() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.5);

        // 正反向运行 1000 帧，有↔无互补始终成立
        for _ in 0..1000 {
            ont.set_complement(1, true, true).unwrap();
            ont.step_frame();
        }

        let state = ont.existence_state(1).unwrap();
        assert!(state.alive, "互补存在应维持");
        assert!(state.recursion_depth >= 1000);
    }

    #[test]
    fn existence_dissipates_without_complement() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.003); // 小初始幅度

        // 不互补 → 无法 replenish → 乘法衰减 → 消散
        for _ in 0..300 {
            ont.set_complement(1, false, false).unwrap();
            ont.step_frame();
        }

        let state = ont.existence_state(1).unwrap();
        assert!(!state.alive, "不存在互补时应消散, amplitude={}", state.amplitude);
    }

    #[test]
    fn bei_fei_balance_required_for_iteration() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.8);
        ont.set_complement(1, true, true).unwrap();
        ont.step_frame();
        assert!(ont.existence_state(1).unwrap().can_iterate);

        // 断开互补
        ont.set_complement(1, false, true).unwrap();
        ont.step_frame();
        assert!(!ont.existence_state(1).unwrap().can_iterate);
    }

    #[test]
    fn risk_dissipation_warning() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.1);
        ont.set_complement(1, true, true).unwrap();

        // 小初始 → 衰减快 → 风险溢出
        for _ in 0..20 {
            ont.step_frame();
            ont.set_complement(1, true, true).unwrap();
        }

        let state = ont.existence_state(1).unwrap();
        // 如果补得不够快，振幅可能已低于风险阈值
        if state.amplitude < 0.2 {
            assert!(state.risk_dissipation);
        }
    }

    #[test]
    fn multi_container_independence() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.5);
        ont.register(2, 0.003);

        // 1号互补维持，2号不互补消散
        for _ in 0..300 {
            ont.set_complement(1, true, true).unwrap();
            ont.set_complement(2, false, false).unwrap();
            ont.step_frame();
        }

        assert!(ont.existence_state(1).unwrap().alive);
        assert!(!ont.existence_state(2).unwrap().alive);
    }

    #[test]
    fn ont_rules_populated() {
        let ont = NJsgOntology::new();
        assert_eq!(ont.rules().len(), 5);
        assert!(ont.rules()[0].name.contains("异见"));
        assert!(ont.rules()[2].name.contains("有无"));
    }

    #[test]
    fn frequency_grows_when_replenished() {
        let mut ont = NJsgOntology::new();
        ont.register(1, 0.1);
        let before = ont.existence_state(1).unwrap().amplitude;

        for _ in 0..10 {
            ont.set_complement(1, true, true).unwrap();
            ont.step_frame();
        }

        let after = ont.existence_state(1).unwrap().amplitude;
        // 补充注入后振幅应增长
        assert!(after > before + 0.01 || after >= 1.0);
    }
}
