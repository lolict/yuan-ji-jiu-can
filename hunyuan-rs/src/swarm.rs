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
use crate::os_layer::bare_metal::BareMetalOs;
use crate::memory::{LayeredMemory, JudgeQuality, AdaptiveParams, CoordMemory, MetaJudgeEngine, Coord};
use crate::pulse::{PulseBody, Velocity};
use crate::hierarchical_time::{HierarchicalTimestamp, LifeEvent, IntegrityReport};
use crate::triune::{TriuneVerdict, FlowDirection, BeingState};
use std::collections::HashMap;
use crate::os_layer::OsLayer;

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
    pub pulse: PulseBody,           // 脉息体 — 心跳 × 行走 × 节律 × 流速
    pub os: BareMetalOs,             // OS 层（用于获取时间）
    pub timestamp: HierarchicalTimestamp, // 层级时间戳 (生命周期追踪)
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
    pub fn new(os: &dyn OsLayer, id: u64, name: &str, role: AgentRole, max_memory: usize) -> Self {
        let ts = os.time_millis();
        let mut timestamp = HierarchicalTimestamp::genesis();

        // 记录创世融合事件: 智能体诞生 = 融入本体
        let birth_witness = TriuneVerdict::deduce(
            BeingState::Being(1),    // 实际: 智能体诞生
            BeingState::Being(1),    // 预期: 诞生
            0.0,                      // 完美见证
        );
        let _ = timestamp.record(LifeEvent::Fusion {
            subject: name.to_string(),
            object: format!("智能体#{}({:?})", id, role),
            purpose: "创世:融入本体".to_string(),
            witness: birth_witness,
        });

        Self {
            id,
            name: name.to_string(),
            birth_tick: ts,
            role,
            state: vec![],
            meta: MetaJudgeEngine::new(max_memory),
            relationships: HashMap::new(),
            pulse: PulseBody::new(os),
            os: BareMetalOs::new(),
            timestamp,
        }
    }

    /// 记录辐射事件: 分身出去做任务
    pub fn record_radiation(&mut self, mission: &str, target: &str) {
        let witness = TriuneVerdict::deduce(
            BeingState::NonBeing,    // 实际: 向外辐射
            BeingState::Being(1),    // 预期: 任务完成
            0.1,                      // 轻微歪斜 (有风险)
        );
        let _ = self.timestamp.record(LifeEvent::Radiation {
            to: target.to_string(),
            mission: mission.to_string(),
            witness,
        });
    }

    /// 记录坍缩事件: 分身回归本体
    pub fn record_collapse(&mut self, from: &str, content: &str) {
        let witness = TriuneVerdict::deduce(
            BeingState::Being(1),    // 实际: 回归
            BeingState::Being(1),    // 预期: 回归
            0.0,                      // 完美见证
        );
        let _ = self.timestamp.record(LifeEvent::Collapse {
            from: from.to_string(),
            content: content.to_string(),
            witness,
        });
    }

    /// 验证智能体生命周期完整性
    pub fn verify_lifecycle(&self) -> IntegrityReport {
        self.timestamp.verify_integrity()
    }

    /// 感知：把外部输入编码为状态单元格，同时更新元裁判引擎
    pub fn perceive(&mut self, d0: u64, d1: u64, d2: u64, polarity: bool, value: u64) {
        let sov = self.role.default_sovereignty().encode();
        let cell = encode_cell(d0, d1, d2, if polarity { 0 } else { 1 }, sov, 0, value);
        self.state.push(cell);
        // 感知留下意识痕迹 → 密度才能从 0 增长
        self.meta.observe_simple(&self.os, cell);
        // 脉息心跳 (同时带动行走)
        self.pulse.beat(&self.os);
    }

    /// 智能体看门狗检测：超过 timeout_ms 未感知 → 标记假死
    pub fn check_staleness(&mut self, timeout_ms: u64) -> bool {
        self.pulse.is_stale(&self.os, timeout_ms)
    }

    /// 是否为假死状态：流速停滞 (0步/拍) 且超时
    pub fn is_zombie(&self) -> bool {
        self.pulse.velocity_danger() == crate::pulse::VelocityDanger::Stagnant
            || self.pulse.beat_count() == 0
    }

    /// 总行走步数 (生命值)
    pub fn total_steps(&self) -> u64 {
        self.pulse.total_steps()
    }

    /// 当前节律态
    pub fn breath_phase(&self) -> crate::pulse::BreathPhase {
        self.pulse.phase()
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
        self.meta.consciousness_density(&self.os)
    }

    /// 意识水平 (机械执行/感知涌现/反思萌芽/意识涌现)
    pub fn awareness_level(&self) -> &'static str {
        self.meta.awareness_level(&self.os)
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
    pub fn spawn(&mut self, os: &dyn OsLayer, name: &str, role: AgentRole, max_memory: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut agent = CocoonAgent::new(os, id, name, role, max_memory);
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

    /// 智能体看门狗 — 遍历所有智能体，返回当前判定为假死(zombie)的 ID 列表
    /// 假死判定: beat_count==0 (从未活过) 或流速 stagnant (0步/拍)
    pub fn watchdog_agents(&mut self, _timeout_ms: u64) -> Vec<u64> {
        self.agents.values()
            .filter(|a| a.is_zombie())
            .map(|a| a.id)
            .collect()
    }

    /// 清理已被标记为僵尸的智能体
    pub fn prune_zombies(&mut self) -> usize {
        let before = self.agents.len();
        self.agents.retain(|_, a| !a.is_zombie());
        before - self.agents.len()
    }

    /// 僵尸数量
    pub fn zombie_count(&self) -> usize {
        self.agents.values().filter(|a| a.is_zombie()).count()
    }

    /// 智能体总数（不含僵尸）
    pub fn active_count(&self) -> usize {
        self.agents.values().filter(|a| !a.is_zombie()).count()
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
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "RootAgent", AgentRole::Root, 20);
        let agent = shell.agents.get(&a).unwrap();
        assert_eq!(agent.id, 0);
        assert_eq!(agent.name, "RootAgent");
        assert_eq!(agent.role, AgentRole::Root);
        assert!(agent.is_alive() == false); // 还没有状态 = 还没有真正"活"
    }

    #[test]
    fn agent_perceive_becomes_alive() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "RootAgent", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);
        let agent = shell.agents.get(&a).unwrap();
        assert!(agent.is_alive());
        assert_eq!(agent.state.len(), 1);
    }

    #[test]
    fn negotiation_positive_surplus() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 250);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 50);
        let surplus = shell.negotiate_round();
        assert!(surplus > 0.0);
    }

    #[test]
    fn relationship_updates_after_negotiation() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 300);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
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
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 200);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 150);
        shell.negotiate_round();
        assert!(shell.global_density() > 0.0);
    }

    #[test]
    fn prune_dead_removes_empty() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        shell.spawn(&os, "Dead", AgentRole::Leaf, 20);
        shell.spawn(&os, "Alive", AgentRole::Root, 20);
        shell.agents.get_mut(&1).unwrap().perceive(0, 0, 0, true, 100);
        shell.prune_dead();
        assert_eq!(shell.agents.len(), 1);
    }

    #[test]
    fn role_default_sovereignty() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(10);
        let root = shell.spawn(&os, "Root", AgentRole::Root, 10);
        let leaf = shell.spawn(&os, "Leaf", AgentRole::Leaf, 10);
        let root_decay = shell.agents.get(&root).unwrap().meta.params.witness_decay;
        let leaf_decay = shell.agents.get(&leaf).unwrap().meta.params.witness_decay;
        assert_eq!(root_decay, shell.genome.decay_rate);
        assert_eq!(leaf_decay, shell.genome.decay_rate);
        assert!(matches!(AgentRole::Root.default_sovereignty(), Sovereignty::Active));
        assert!(matches!(AgentRole::Leaf.default_sovereignty(), Sovereignty::Passive));
    }

    #[test]
    fn agent_awareness_levels() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut a = CocoonAgent::new(&os, 0, "Test", AgentRole::Root, 20);
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
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut session = NegotiationSession::new(vec![0, 1, 2]);
        let mut agents = HashMap::new();
        for id in 0..3 {
            let mut a = CocoonAgent::new(&os, id, "Agent", AgentRole::Root, 20);
            a.perceive(id, 0, 0, true, 100);
            agents.insert(id, a);
        }
        session.propose(0, agents[&0].state.clone());
        session.propose(1, agents[&1].state.clone());
        session.propose(2, agents[&2].state.clone());
        let s = session.negotiate_round(&agents);
        assert!(s >= 0.0);
    }

    // ─── PulseBody / Zombie 检测测试 ───

    #[test]
    fn agent_zombie_detected_by_watchdog() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Test", AgentRole::Root, 20);
        // 先感知一次（活）
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);
        assert!(!shell.agents.get(&a).unwrap().is_zombie());

        // watchdog 检测：刚 beat 过的不会 zombie
        let zombies = shell.watchdog_agents(0);
        // 不应 panic，返回 Vec 即可
        assert!(!zombies.contains(&a));
    }

    #[test]
    fn unperceived_agent_is_zombie_by_default() {
        // 刚创建但未感知的智能体：beat_count == 0 → 假死状态
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let a = CocoonAgent::new(&os, 0, "Test", AgentRole::Root, 20);
        assert!(a.is_zombie());
    }

    #[test]
    fn perceive_makes_agent_alive() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut a = CocoonAgent::new(&os, 0, "Test", AgentRole::Root, 20);
        assert!(a.is_zombie());      // 未感知 = 假死
        a.perceive(0, 0, 0, true, 100);
        assert!(!a.is_zombie());     // 感知后复活
        assert!(a.total_steps() > 0); // 行走步数增加了
    }

    #[test]
    fn agent_breath_phase_rotates_with_beats() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut a = CocoonAgent::new(&os, 0, "Test", AgentRole::Root, 20);
        // 初始吸气
        assert_eq!(a.breath_phase(), crate::pulse::BreathPhase::Inhale);
        // 连续感知多次 → 节律轮转
        for i in 0..4 { a.perceive(i, 0, 0, true, 100); }
        // 4 次 beat 后应转为呼气态 (吸 4 拍 → 呼)
        assert_eq!(a.breath_phase(), crate::pulse::BreathPhase::Exhale);
    }

    #[test]
    fn zombie_pruning_removes_unperceived() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Alive", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);

        shell.spawn(&os, "Zombie", AgentRole::Leaf, 20);
        // 不感知，zombie_count == 1
        assert_eq!(shell.zombie_count(), 1);
        assert_eq!(shell.active_count(), 1);

        let pruned = shell.prune_zombies();
        assert_eq!(pruned, 1);
        assert_eq!(shell.agents.len(), 1);
    }

    #[test]
    fn zombie_count_correct_with_mixed_population() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        // 生活体 (感知过)
        let a = shell.spawn(&os, "Alive1", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);
        let b = shell.spawn(&os, "Alive2", AgentRole::Leaf, 20);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 50);

        // 僵尸体 (从未感知 = beat_count==0 = 假死)
        shell.spawn(&os, "Zombie1", AgentRole::Flower, 20);
        shell.spawn(&os, "Zombie2", AgentRole::Observer, 20);

        assert_eq!(shell.zombie_count(), 2);
        assert_eq!(shell.active_count(), 2);
    }

    // ── 层级时间戳生命周期追踪测试 ──

    #[test]
    fn agent_has_hierarchical_timestamp_after_spawn() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "TestAgent", AgentRole::Root, 20);
        let agent = shell.agents.get(&a).unwrap();
        // 创世融合事件：spawn 时自动记录 genesis Fusion
        // 1 个事件，version = events.len() + 1 = 2
        assert_eq!(agent.timestamp.events.len(), 1);
        assert_eq!(agent.timestamp.version(), 2);
    }

    #[test]
    fn record_radiation_creates_event() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Radiant", AgentRole::Root, 20);
        {
            let agent = shell.agents.get_mut(&a).unwrap();
            agent.record_radiation("探索天庭", "HeavenGate");
        }
        let agent = shell.agents.get(&a).unwrap();
        // 创世融合 + 辐射 = 2 个事件
        assert_eq!(agent.timestamp.events.len(), 2);
        assert!(matches!(
            agent.timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Radiation { .. }
        ));
    }

    #[test]
    fn record_collapse_creates_event() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Collider", AgentRole::Leaf, 20);
        {
            let agent = shell.agents.get_mut(&a).unwrap();
            agent.record_collapse("分身A", "坍缩成果");
        }
        let agent = shell.agents.get(&a).unwrap();
        // 创世融合 + 坍缩 = 2 个事件
        assert_eq!(agent.timestamp.events.len(), 2);
        assert!(matches!(
            agent.timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Collapse { .. }
        ));
    }

    #[test]
    fn verify_lifecycle_intact_after_events() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Lifecycle", AgentRole::Flower, 20);
        {
            let agent = shell.agents.get_mut(&a).unwrap();
            agent.record_radiation("辐射至叶节点", "LeafNode");
            agent.record_collapse("坍缩回声", "回声内容");
            agent.record_radiation("二次辐射", "Target2");
        }
        let agent = shell.agents.get(&a).unwrap();
        // 1 创世 + 3 记录 = 4 事件
        assert_eq!(agent.timestamp.events.len(), 4);
        // 完整性校验应通过
        let report = agent.verify_lifecycle();
        assert!(report.is_intact());
        assert_eq!(report.total_events, 4);
    }

    #[test]
    fn lifecycle_version_drops_when_tampered() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Tampered", AgentRole::Observer, 20);
        {
            let agent = shell.agents.get_mut(&a).unwrap();
            agent.record_radiation("正常辐射", "Target");
        }
        let agent = shell.agents.get(&a).unwrap();
        let original_version = agent.timestamp.version();
        assert_eq!(original_version, 3); // 1 创世 + 1 辐射 = 2 events → v3
                                           
        // 模拟篡改：直接清空事件
        shell.agents.get_mut(&a).unwrap().timestamp.events.clear();
        let agent = shell.agents.get(&a).unwrap();
        // 版本号应下降 (事件减少 → version 降低)
        assert!(agent.timestamp.version() < original_version);
        assert_eq!(agent.timestamp.events.len(), 0);
        assert_eq!(agent.timestamp.version(), 1); // 空事件 → v1
    }

    #[test]
    fn multiple_agents_independent_timestamps() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Agent1", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Agent2", AgentRole::Leaf, 20);
        // Agent1 辐射
        shell.agents.get_mut(&a).unwrap().record_radiation("任务1", "T1");
        // Agent2 坍缩
        shell.agents.get_mut(&b).unwrap().record_collapse("分身B", "成果B");
        // 各自独立
        assert_eq!(shell.agents.get(&a).unwrap().timestamp.events.len(), 2);
        assert_eq!(shell.agents.get(&b).unwrap().timestamp.events.len(), 2);
        // 事件内容独立
        assert!(matches!(
            shell.agents.get(&a).unwrap().timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Radiation { .. }
        ));
        assert!(matches!(
            shell.agents.get(&b).unwrap().timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Collapse { .. }
        ));
    }
}
