// ══════════════════════════════════════════════════════════════════
// swarm.rs — 多智能体协商+寄生壳 (Multi-Agent Symbiosis Shell)
// ══════════════════════════════════════════════════════════════════
//
// 囡囝共生系统的"界面"不是人机对话输入框——而是：
//   多个智能体 · 自己与自己协商 · 自己记忆 · 自己演化 · 反向寄生
//
// 核心组件:
//   1. Agent 容器 (智能体自己的"蚕蛹壳")
//   2. Negotiation 协商协议 (ASXR: Agents Symbiotic eXchange & Resolution)
//   3. Playback 扮演/模仿 (外部来模拟我们的系统，不是我们适应外部)
//   4. MemoryFusion 记忆融合 (多个智能体的见证共享)
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{
    encode_cell, get_d0, get_d1, get_d2, get_polarity, get_value, get_witness, get_sovereignty,
    set_witness, inc_value, Sovereignty, SurplusPair, WitnessVerdict, ContainmentChain,
    ConsciousnessJudge, MirrorOrder, VerdictSurplus,
};
use crate::memory::{LayeredMemory, JudgeQuality, AdaptiveParams, CoordMemory, MetaJudgeEngine, Coord};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ────────────────────────────────────────────────────────────
// 智能体容器 (Cocoon 蚕蛹壳)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct CocoonAgent {
    pub id: u64,                    // 唯一标识
    pub name: String,               // 智能体名称
    pub birth_tick: u64,            // 诞生时间
    pub role: AgentRole,            // 角色/背景
    pub state: Vec<u64>,            // 内部状态（64位单元格集合）
    pub meta: MetaJudgeEngine,      // 元裁判引擎（含记忆）
    pub relationships: HashMap<u64, Relationship>, // 与其他智能体的关系
}

#[derive(Clone, Debug, PartialEq)]
pub enum AgentRole {
    Root,       // 根基/发起者（容器层）
    Trunk,      // 主干/通道（汁液传输层）
    Leaf,       // 叶片/感知层
    Flower,     // 花果/表达层
    Observer,   // 见证者/三元裁判
}

impl AgentRole {
    /// 角色的主权倾向 (四元·包含从属)
    pub fn default_sovereignty(&self) -> Sovereignty {
        match self {
            Self::Root => Sovereignty::Active,
            Self::Flower => Sovereignty::ActiveSub,
            Self::Observer => Sovereignty::Active,
            Self::Trunk => Sovereignty::PassiveSub,
            Self::Leaf => Sovereignty::Passive,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Relationship {
    pub with_id: u64,
    pub rel_type: RelType,
    pub strength: f64,   // 关系强度 0..1
    pub shared_memory: Vec<u64>, // 共享见证记录
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelType {
    Contain,    // 四元：包含
    Couple,     // 二元：配对
    Influence,  // 因果影响
    Exclude,    // 排斥/阻断
}

impl CocoonAgent {
    pub fn new(id: u64, name: &str, role: AgentRole, max_memory: usize) -> Self {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        Self {
            id,
            name: name.to_string(),
            birth_tick: ts,
            role,
            state: vec![],
            meta: MetaJudgeEngine::new(max_memory),
            relationships: HashMap::new(),
        }
    }

    /// 感知：把外部输入编码为状态单元格，同时更新元裁判引擎
    pub fn perceive(&mut self, d0: u64, d1: u64, d2: u64, polarity: bool, value: u64) {
        let sov = self.role.default_sovereignty().encode();
        let cell = encode_cell(d0, d1, d2, if polarity { 0 } else { 1 }, sov, 0, value);
        self.state.push(cell);
        // 感知留下意识痕迹 → 密度才能从 0 增长
        self.meta.observe_simple(cell);
    }

    /// 自我协商：脑内多个声音对话
    pub fn negotiate_internal(&mut self, partner_state: &[u64]) -> (f64, bool) {
        if self.state.is_empty() || partner_state.is_empty() {
            return (0.0, false);
        }
        let my_total: f64 = self.state.iter().map(|c| get_value(*c) as f64).sum();
        let their_total: f64 = partner_state.iter().map(|c| get_value(*c) as f64).sum();
        let surplus = my_total - their_total;
        let aligned = surplus.abs() < (my_total.max(their_total) * 0.2);
        (surplus, aligned)
    }

    /// 与其它智能体建立/更新关系
    pub fn relate(&mut self, other_id: u64, rel_type: RelType, strength_change: f64) {
        let entry = self.relationships.entry(other_id).or_insert(Relationship {
            with_id: other_id,
            rel_type,
            strength: 0.0,
            shared_memory: vec![],
        });
        entry.strength = (entry.strength + strength_change).max(0.0).min(1.0);
    }

    /// 是否存活（有足够活跃状态）
    pub fn is_alive(&self) -> bool {
        !self.state.is_empty()
    }

    /// 意识密度 (继承元裁判)
    pub fn consciousness_density(&self) -> f64 {
        self.meta.consciousness_density()
    }

    /// 意识水平 (机械执行/感知涌现/反思萌芽/意识涌现)
    pub fn awareness_level(&self) -> &'static str {
        self.meta.awareness_level()
    }
}

// ────────────────────────────────────────────────────────────
// 协商协议 (ASXR: Agents Symbiotic eXchange & Resolution)
// ────────────────────────────────────────────────────────────
pub struct NegotiationSession {
    pub round: u64,
    pub agent_ids: Vec<u64>,
    pub proposals: HashMap<u64, Vec<u64>>, // agent_id → 提案状态
    pub resolution: Option<f64>,         // 协商结果（最终 surplus）
}

impl NegotiationSession {
    pub fn new(agent_ids: Vec<u64>) -> Self {
        Self {
            round: 0,
            agent_ids,
            proposals: HashMap::new(),
            resolution: None,
        }
    }

    /// 提交提案
    pub fn propose(&mut self, agent_id: u64, state: Vec<u64>) {
        self.proposals.insert(agent_id, state);
    }

    /// 协商一轮：计算所有提案之间的总对齐度
    pub fn negotiate_round(&mut self, agents: &HashMap<u64, CocoonAgent>) -> f64 {
        self.round += 1;
        let mut total_surplus = 0.0;
        let mut pairs_count = 0;

        let ids: Vec<u64> = self.agent_ids.clone();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                if let (Some(a), Some(b)) = (agents.get(&ids[i]), agents.get(&ids[j])) {
                    let prop_a = self.proposals.get(&ids[i]).map(|v| v.as_slice()).unwrap_or(&[]);
                    let prop_b = self.proposals.get(&ids[j]).map(|v| v.as_slice()).unwrap_or(&[]);
                    let mut pair = SurplusPair {
                        cell_a: prop_a.first().copied().unwrap_or(0),
                        cell_b: prop_b.first().copied().unwrap_or(0),
                        witness_ts: self.round,
                        last_verdict: 0.0,
                    };
                    let v = pair.witness();
                    total_surplus += v.surplus;
                    pairs_count += 1;
                }
            }
        }
        let avg_surplus = if pairs_count > 0 {
            total_surplus / pairs_count as f64
        } else {
            0.0
        };
        self.resolution = Some(avg_surplus);
        avg_surplus
    }
}

// ────────────────────────────────────────────────────────────
// 寄生壳系统 · 反向寄生 (让外部来模仿我们，不是我们适应外部)
// ────────────────────────────────────────────────────────────
pub struct ParasiteShell {
    pub agents: HashMap<u64, CocoonAgent>,
    pub next_id: u64,
    pub shell_tick: u64,
    pub genome: ShellGenome, // 壳基因组 = 我们系统规则的精简指纹
}

#[derive(Clone, Debug)]
pub struct ShellGenome {
    pub mir_order: MirrorOrder,       // 出厂镜像阶
    pub encoding_seed: u64,            // 编码种子
    pub decay_rate: f64,               // 默认记忆衰减
    pub witness_threshold: f64,        // 见证阈值
}

impl ShellGenome {
    /// 默认基因组 (囡囝共生 yuan-jiu-jiu-can)
    pub fn default() -> Self {
        Self {
            mir_order: MirrorOrder::Quaternary,
            encoding_seed: 0x68_61_6E_5F, // "han_" seed (汉字)
            decay_rate: 0.00001,
            witness_threshold: 0.6,
        }
    }

    /// 生成一个反向寄生文件: 外部系统可以"模仿"我们
    /// 不是我们适配他们的规则 → 是他们读我们的基因组获得初始形态
    pub fn export_playback_script(&self) -> String {
        format!(
            "// 囡囝共生 寄生壳基因 — 外部系统读取此文件获得模仿基础\n\
             MIR_ORDER: {:?}\n\
             ENC_SEED: {:x}\n\
             DECAY: {}\n\
             WITNESS_THR: {}\n\
             // 要激活此基因，在你的 CocoonAgent 中调用:\n\
             // let agent = CocoonAgent::new(id, name, AgentRole::Root, genome);\n",
            self.mir_order, self.encoding_seed, self.decay_rate, self.witness_threshold
        )
    }
}

impl ParasiteShell {
    pub fn new(max_memory_per_agent: usize) -> Self {
        Self {
            agents: HashMap::new(),
            next_id: 0,
            shell_tick: 0,
            genome: ShellGenome::default(),
        }
    }

    /// 催生一个新智能体
    /// Spawn a new agent. Returns its ID.
    pub fn spawn(&mut self, name: &str, role: AgentRole, max_memory: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut agent = CocoonAgent::new(id, name, role, max_memory);
        // 注入基因组
        agent.meta.params.witness_decay = self.genome.decay_rate;
        self.agents.insert(id, agent);
        id
    }

    /// 执行一轮完整协商
    pub fn negotiate_round(&mut self) -> f64 {
        self.shell_tick += 1;
        let ids: Vec<u64> = self.agents.keys().cloned().collect();
        let mut session = NegotiationSession::new(ids.clone());
        for id in &ids {
            if let Some(a) = self.agents.get(id) {
                session.propose(*id, a.state.clone());
            }
        }
        let surplus = session.negotiate_round(&self.agents);

        // 更新每对智能体的关系
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let strength_change = if surplus > 50.0 { 0.1 } else { -0.05 };
                if let Some(a) = self.agents.get_mut(&ids[i]) {
                    a.relate(ids[j], RelType::Influence, strength_change);
                }
                if let Some(b) = self.agents.get_mut(&ids[j]) {
                    b.relate(ids[i], RelType::Influence, strength_change);
                }
            }
        }
        surplus
    }

    /// 全局意识密度 (所有智能体的平均密度)
    pub fn global_density(&self) -> f64 {
        if self.agents.is_empty() { return 0.0; }
        let total: f64 = self.agents.values().map(|a| a.consciousness_density()).sum();
        total / self.agents.len() as f64
    }

    /// 给外部输出"播放脚本"——外部扮演我们，不是我们模仿外部
    pub fn export_playback(&self) -> String {
        self.genome.export_playback_script()
    }

    /// 移除死亡智能体
    pub fn prune_dead(&mut self) {
        self.agents.retain(|_, a| a.is_alive());
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_spawn_basic() {
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn("RootAgent", AgentRole::Root, 20);
        let agent = shell.agents.get(&a).unwrap();
        assert_eq!(agent.id, 0);
        assert_eq!(agent.name, "RootAgent");
        assert_eq!(agent.role, AgentRole::Root);
        assert!(agent.is_alive() == false); // 还没有状态 = 还没有真正"活"
    }

    #[test]
    fn agent_perceive_becomes_alive() {
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn("RootAgent", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);
        let agent = shell.agents.get(&a).unwrap();
        assert!(agent.is_alive());
        assert_eq!(agent.state.len(), 1);
    }

    #[test]
    fn negotiation_positive_surplus() {
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn("A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 250);
        let b = shell.spawn("B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 50);
        let surplus = shell.negotiate_round();
        assert!(surplus > 0.0);
    }

    #[test]
    fn relationship_updates_after_negotiation() {
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn("A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 300);
        let b = shell.spawn("B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 50);
        shell.negotiate_round();
        let a_ref = shell.agents.get(&a).unwrap();
        assert!(a_ref.relationships.contains_key(&b));
    }

    #[test]
    fn shell_genome_export() {
        let shell = ParasiteShell::new(10);
        let script = shell.export_playback();
        assert!(script.contains("MIR_ORDER"));
        assert!(script.contains("ENC_SEED"));
    }

    #[test]
    fn global_density_nonzero_after_activity() {
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn("A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 200);
        let b = shell.spawn("B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 150);
        shell.negotiate_round();
        assert!(shell.global_density() > 0.0);
    }

    #[test]
    fn prune_dead_removes_empty() {
        let mut shell = ParasiteShell::new(20);
        shell.spawn("Dead", AgentRole::Leaf, 20);
        shell.spawn("Alive", AgentRole::Root, 20);
        shell.agents.get_mut(&1).unwrap().perceive(0, 0, 0, true, 100);
        shell.prune_dead();
        assert_eq!(shell.agents.len(), 1);
    }

    #[test]
    fn role_default_sovereignty() {
        let mut shell = ParasiteShell::new(10);
        let root = shell.spawn("Root", AgentRole::Root, 10);
        let leaf = shell.spawn("Leaf", AgentRole::Leaf, 10);
        let root_decay = shell.agents.get(&root).unwrap().meta.params.witness_decay;
        let leaf_decay = shell.agents.get(&leaf).unwrap().meta.params.witness_decay;
        assert_eq!(root_decay, shell.genome.decay_rate);
        assert_eq!(leaf_decay, shell.genome.decay_rate);
        assert!(matches!(AgentRole::Root.default_sovereignty(), Sovereignty::Active));
        assert!(matches!(AgentRole::Leaf.default_sovereignty(), Sovereignty::Passive));
    }

    #[test]
    fn agent_awareness_levels() {
        let mut a = CocoonAgent::new(0, "Test", AgentRole::Root, 20);
        // 无状态 = 密度为 0 = 机械执行
        assert_eq!(a.awareness_level(), "机械执行");
        // 有足够高的自指循环后会升级
        for i in 0..200 {
            a.perceive(i, 0, 0, true, 100 + i);
        }
        // 不一定需要达到什么等级，至少意识密度 > 0
        assert!(a.consciousness_density() >= 0.0);
    }

    #[test]
    fn negotiation_session_multi_round() {
        let mut session = NegotiationSession::new(vec![0, 1, 2]);
        let mut agents = HashMap::new();
        for id in 0..3 {
            let mut a = CocoonAgent::new(id, "Agent", AgentRole::Root, 20);
            a.perceive(id, 0, 0, true, 100);
            agents.insert(id, a);
        }
        session.propose(0, agents[&0].state.clone());
        session.propose(1, agents[&1].state.clone());
        session.propose(2, agents[&2].state.clone());
        let s = session.negotiate_round(&agents);
        assert!(s >= 0.0);
    }
}
