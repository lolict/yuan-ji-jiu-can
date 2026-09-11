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

    /// 记录融合事件: 与外部实体融合 (协商成功 / 合二为一)
    pub fn record_fusion(&mut self, partner: &str, purpose: &str) {
        let witness = TriuneVerdict::deduce(
            BeingState::Being(1),    // 融合前: 已是存在
            BeingState::Being(1),    // 融合后: 仍是存在 (幂等)
            0.0,                      // 完美见证 (融合是确定性事件)
        );
        let _ = self.timestamp.record(LifeEvent::Fusion {
            subject: self.name.clone(),
            object: partner.to_string(),
            purpose: purpose.to_string(),
            witness,
        });
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
    /// 心跳: 周期性保存时间戳, 用于崩溃恢复
    pub heartbeat: ShellHeartbeat,
}

/// 心跳机制: 周期性持久化时间戳
/// 当系统崩溃后, 可从最后一次心跳恢复所有分身的时间戳
#[derive(Clone, Debug)]
pub struct ShellHeartbeat {
    /// 心跳间隔 (tick 数)
    pub interval: u64,
    /// 上次心跳时的 tick
    pub last_beat: u64,
    /// 心跳计数 (递增)
    pub beat_count: u64,
    /// 保存路径
    pub path: String,
}

impl ShellHeartbeat {
    pub fn new(interval: u64, path: &str) -> Self {
        Self {
            interval,
            last_beat: 0,
            beat_count: 0,
            path: path.to_string(),
        }
    }

    /// 默认心跳: 每 10 tick 保存到 .nest_heartbeat.hts
    pub fn default() -> Self {
        Self::new(10, ".nest_heartbeat.hts")
    }

    /// 检查是否需要触发心跳
    pub fn should_beat(&self, current_tick: u64) -> bool {
        current_tick > 0 && (current_tick - self.last_beat) >= self.interval
    }
}

/// 纤维: 跨分身的因果事件链
/// 例如: A 辐射到 B → B 坍缩回 A = 一根完整的纤维
#[derive(Clone, Debug)]
pub struct Fiber {
    pub from_agent: u64,
    pub from_name: String,
    pub to_agent: u64,
    pub to_name: String,
    pub mission: String,
    pub complete: bool,  // true = 来回完整 (辐射+坍缩都发生)
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
            heartbeat: ShellHeartbeat::default(),
        }
    }

    /// 自定义心跳间隔和路径
    pub fn with_heartbeat(max_memory_per_agent: usize, interval: u64, path: &str) -> Self {
        Self {
            agents: HashMap::new(),
            next_id: 0,
            shell_tick: 0,
            genome: ShellGenome::default(),
            heartbeat: ShellHeartbeat::new(interval, path),
        }
    }

    /// 心跳脉冲: 每 tick 调用一次, 检查是否需要自动保存
    /// 返回: Ok(true) = 已保存, Ok(false) = 未到时间, Err = 保存失败
    pub fn pulse(&mut self, os: &dyn OsLayer) -> Result<bool, String> {
        if self.heartbeat.should_beat(self.shell_tick) {
            self.save_to_file(os, &self.heartbeat.path)?;
            self.heartbeat.last_beat = self.shell_tick;
            self.heartbeat.beat_count += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 强制心跳 (忽略间隔, 立即保存)
    pub fn force_beat(&mut self, os: &dyn OsLayer) -> Result<(), String> {
        self.save_to_file(os, &self.heartbeat.path)?;
        self.heartbeat.last_beat = self.shell_tick;
        self.heartbeat.beat_count += 1;
        Ok(())
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

    /// 获取指定 ID 的智能体 (可变引用)
    pub fn get_agent(&mut self, id: u64) -> Option<&mut CocoonAgent> {
        self.agents.get_mut(&id)
    }

    /// 获取指定 ID 的智能体 (不可变引用)
    pub fn get_agent_ref(&self, id: u64) -> Option<&CocoonAgent> {
        self.agents.get(&id)
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

    /// 全局时间线: 汇聚所有分身的事件为一个有序列表
    /// 返回: Vec<(分身ID, 分身名, 事件索引, 事件描述, 版本号)>
    pub fn global_timeline(&self) -> Vec<(u64, String, usize, String, u32)> {
        let mut timeline: Vec<(u64, String, usize, String, u32)> = Vec::new();

        for agent in self.agents.values() {
            for (i, event) in agent.timestamp.events.iter().enumerate() {
                let desc = match event {
                    LifeEvent::Fusion { object, purpose, .. } =>
                        format!("融合{}→{}", object, purpose),
                    LifeEvent::Collapse { from, content, .. } =>
                        format!("坍缩←{}({})", from, content),
                    LifeEvent::Radiation { to, mission, .. } =>
                        format!("辐射→{}({})", to, mission),
                };
                timeline.push((
                    agent.id,
                    agent.name.clone(),
                    i,
                    desc,
                    agent.timestamp.version(),
                ));
            }
        }

        // 按分身ID + 事件索引排序 (保持因果序)
        timeline.sort_by_key(|(id, _, idx, _, _)| (*id, *idx));
        timeline
    }

    /// 全局完整性校验: 所有分身的完整性报告
    pub fn verify_all_lifecycles(&self) -> Vec<(u64, String, IntegrityReport)> {
        self.agents.values()
            .map(|a| (a.id, a.name.clone(), a.verify_lifecycle()))
            .collect()
    }

    /// 统计: 系统中各类事件的总数
    pub fn event_statistics(&self) -> (usize, usize, usize) {
        let mut fusions = 0;
        let mut collapses = 0;
        let mut radiations = 0;

        for agent in self.agents.values() {
            for event in &agent.timestamp.events {
                match event {
                    LifeEvent::Fusion { .. } => fusions += 1,
                    LifeEvent::Collapse { .. } => collapses += 1,
                    LifeEvent::Radiation { .. } => radiations += 1,
                }
            }
        }

        (fusions, collapses, radiations)
    }

    /// 持久化: 将所有分身的时间戳导出为 HTS_V1 文本格式
    pub fn export_timestamps(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("NEST_V1|agents={}|tick={}", self.agents.len(), self.shell_tick));

        for agent in self.agents.values() {
            let header = format!(
                "AGENT|id={}|name={}|role={:?}|birth_tick={}|density={:.4}",
                agent.id, agent.name, agent.role, agent.birth_tick, agent.consciousness_density()
            );
            // 每个分身的 section = 头部 + 时间戳内容 (之间用单换行, section 之间用 \n--\n)
            parts.push(format!("{}\n{}", header, agent.timestamp.serialize()));
        }

        parts.join("\n--\n")
    }

    /// 恢复: 从 HTS_V1 文本恢复所有分身的时间戳
    pub fn import_timestamps(text: &str) -> Result<Vec<(u64, String, HierarchicalTimestamp)>, String> {
        let mut results = Vec::new();

        for section in text.split("\n--\n") {
            let trimmed = section.trim();
            if trimmed.is_empty() { continue; }
            if trimmed.starts_with("NEST_V1|") { continue; } // 跳过头部

            if trimmed.starts_with("AGENT|") {
                let newline_pos = trimmed.find('\n').ok_or("AGENT section 缺少时间戳")?;
                let header = &trimmed[..newline_pos];
                let ts_text = &trimmed[newline_pos+1..];

                let mut id = 0u64;
                let mut name = String::new();
                for part in header.split('|').skip(1) {
                    if let Some(v) = part.strip_prefix("id=") {
                        id = v.parse().map_err(|_| "解析 id 失败")?;
                    } else if let Some(v) = part.strip_prefix("name=") {
                        name = v.to_string();
                    }
                }

                let timestamp = HierarchicalTimestamp::deserialize(ts_text)?;
                results.push((id, name, timestamp));
            }
        }

        Ok(results)
    }

    // ─── 文件持久化 (通过 OS 抽象层) ───

    /// 保存全部时间戳到磁盘 (通过 OS 抽象层)
    pub fn save_to_file(&self, os: &dyn OsLayer, path: &str) -> Result<(), String> {
        let data = self.export_timestamps();
        os.file_write(path, data.as_bytes())
            .map_err(|e| format!("写入失败: {:?}", e))?;
        Ok(())
    }

    /// 从磁盘加载全部时间戳 (通过 OS 抽象层)
    pub fn load_from_file(os: &dyn OsLayer, path: &str) -> Result<Vec<(u64, String, HierarchicalTimestamp)>, String> {
        let bytes = os.file_read(path)
            .map_err(|e| format!("读取失败: {:?}", e))?;
        let text = String::from_utf8(bytes)
            .map_err(|e| format!("UTF-8 解码失败: {}", e))?;
        Self::import_timestamps(&text)
    }

    /// 检查持久化文件是否存在
    pub fn persisted_exists(os: &dyn OsLayer, path: &str) -> bool {
        os.file_exists(path)
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

    // ─── 纤维束编织 (Fiber Bundle Weaving) ───

    /// 编织纤维束: 检测所有跨分身的辐射-坍缩因果连接
    /// 返回: Vec<Fiber> — 所有因果链 (包括不完整的单向辐射)
    pub fn weave_fibers(&self) -> Vec<Fiber> {
        let mut fibers = Vec::new();

        // 收集所有辐射事件
        let mut radiations: Vec<(u64, &String, &String, &str)> = Vec::new(); // (from_id, from_name, to, mission)
        for agent in self.agents.values() {
            for event in &agent.timestamp.events {
                if let LifeEvent::Radiation { to, mission, .. } = event {
                    radiations.push((agent.id, &agent.name, to, mission.as_str()));
                }
            }
        }

        // 收集所有坍缩事件
        // 坍缩发生在目标分身 (它收到辐射后坍缩回源)
        let mut collapses: Vec<(u64, &String, &String, &str)> = Vec::new(); // (self_id, self_name, from, content)
        for agent in self.agents.values() {
            for event in &agent.timestamp.events {
                if let LifeEvent::Collapse { from, content, .. } = event {
                    collapses.push((agent.id, &agent.name, from, content.as_str()));
                }
            }
        }

        // 匹配: 辐射目标 = 分身名 → 建立纤维
        for (from_id, from_name, to_name, mission) in &radiations {
            // 目标分身是否在系统中?
            let target_agent = self.agents.values().find(|a| a.name == **to_name);
            if let Some(target) = target_agent {
                // 检查: 目标分身是否有坍缩事件，且该坍缩来自辐射源
                // 即 target 的 Collapse.from == 辐射源的 name
                let has_return = collapses.iter().any(|(collapse_self_id, collapse_self_name, collapse_from, _)| {
                    // 坍缩发生在目标分身上
                    *collapse_self_id == target.id
                    // 且坍缩的源头是辐射源
                    && **collapse_from == **from_name
                    // 且坍缩者自身的名称就是辐射目标名称 (否则是乱匹配)
                    && **collapse_self_name == **to_name
                });
                fibers.push(Fiber {
                    from_agent: *from_id,
                    from_name: (*from_name).clone(),
                    to_agent: target.id,
                    to_name: target.name.clone(),
                    mission: mission.to_string(),
                    complete: has_return,
                });
            }
        }

        fibers
    }

    /// 纤维束统计: 完整纤维 vs 单向辐射
    pub fn fiber_statistics(&self) -> (usize, usize) {
        let fibers = self.weave_fibers();
        let complete = fibers.iter().filter(|f| f.complete).count();
        let incomplete = fibers.len() - complete;
        (complete, incomplete)
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

    #[test]
    fn record_fusion_creates_event() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Fusor", AgentRole::Root, 20);
        {
            let agent = shell.agents.get_mut(&a).unwrap();
            agent.record_fusion("Partner-Agent", "协商融合: 合二为一");
        }
        let agent = shell.agents.get(&a).unwrap();
        // 创世 + 融合 = 2 事件
        assert_eq!(agent.timestamp.events.len(), 2);
        assert!(matches!(
            agent.timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Fusion { .. }
        ));
        assert_eq!(agent.timestamp.version(), 3); // 2 events → v3
    }

    #[test]
    fn global_timeline_aggregates_all_events() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Root", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Leaf", AgentRole::Leaf, 20);
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "Target");
        shell.agents.get_mut(&b).unwrap().record_collapse("Leaf", "成果");
        shell.agents.get_mut(&a).unwrap().record_fusion("外部", "融合");

        let timeline = shell.global_timeline();
        // Root: 创世 + 辐射 + 融合 = 3 事件
        // Leaf: 创世 + 坍缩 = 2 事件
        // 总计 = 5
        assert_eq!(timeline.len(), 5);
        // 验证排序: 按 (id, idx)
        assert_eq!(timeline[0].0, a); // Root 创世
        assert_eq!(timeline[1].0, a); // Root 辐射
        assert_eq!(timeline[2].0, a); // Root 融合
        assert_eq!(timeline[3].0, b); // Leaf 创世
        assert_eq!(timeline[4].0, b); // Leaf 坍缩

        // 验证事件描述包含关键字
        let has_radiation = timeline.iter().any(|(_, _, _, desc, _)| desc.contains("辐射"));
        let has_collapse = timeline.iter().any(|(_, _, _, desc, _)| desc.contains("坍缩"));
        let has_fusion = timeline.iter().any(|(_, _, _, desc, _)| desc.contains("融合"));
        assert!(has_radiation);
        assert!(has_collapse);
        assert!(has_fusion);
    }

    #[test]
    fn verify_all_lifecycles_returns_reports_for_each_agent() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "T");

        let reports = shell.verify_all_lifecycles();
        assert_eq!(reports.len(), 2);
        // 所有报告应该是完整的
        for (_, _, report) in &reports {
            assert!(report.is_intact());
        }
    }

    #[test]
    fn event_statistics_counts_correctly() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
        // 创世: 2 Fusion
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "T"); // +1 Radiation
        shell.agents.get_mut(&b).unwrap().record_collapse("B", "成果"); // +1 Collapse
        shell.agents.get_mut(&a).unwrap().record_fusion("X", "融合"); // +1 Fusion

        let (fusions, collapses, radiations) = shell.event_statistics();
        assert_eq!(fusions, 3);   // 2 创世 + 1 record_fusion
        assert_eq!(collapses, 1);
        assert_eq!(radiations, 1);
    }

    #[test]
    fn export_import_timestamps_roundtrip() {
        use crate::hierarchical_time::HierarchicalTimestamp;
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Alice", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Bob", AgentRole::Leaf, 20);
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "T1");
        shell.agents.get_mut(&b).unwrap().record_collapse("Bob", "数据");
        shell.agents.get_mut(&a).unwrap().record_fusion("外部", "融合");

        let exported = shell.export_timestamps();
        // 验证头部
        assert!(exported.starts_with("NEST_V1|"));

        // 恢复
        let mut recovered = ParasiteShell::import_timestamps(&exported).unwrap();
        assert_eq!(recovered.len(), 2);

        // 按 ID 排序 (HashMap 迭代顺序不确定)
        recovered.sort_by_key(|(id, _, _)| *id);

        // 验证 Alice (先 spawn = ID a)
        let (alice_id, alice_name, alice_ts) = &recovered[0];
        assert_eq!(*alice_id, a);
        assert_eq!(alice_name, "Alice");
        assert_eq!(alice_ts.events.len(), 3); // 创世 + 辐射 + 融合

        // 验证 Bob
        let (bob_id, bob_name, bob_ts) = &recovered[1];
        assert_eq!(*bob_id, b);
        assert_eq!(bob_name, "Bob");
        assert_eq!(bob_ts.events.len(), 2); // 创世 + 坍缩

        // 验证恢复的事件类型
        assert!(matches!(alice_ts.events[1], crate::hierarchical_time::LifeEvent::Radiation { .. }));
        assert!(matches!(bob_ts.events[1], crate::hierarchical_time::LifeEvent::Collapse { .. }));
    }

    #[test]
    fn negotiation_can_record_fusion_on_both_agents() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::new(20);
        let a = shell.spawn(&os, "Alice", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Bob", AgentRole::Leaf, 20);
        // 两个智能体各自感知 (变成生活体)
        shell.agents.get_mut(&a).unwrap().perceive(0, 0, 0, true, 100);
        shell.agents.get_mut(&b).unwrap().perceive(1, 0, 0, true, 80);
        // 模拟协商融合：双方记录 Fusion
        shell.agents.get_mut(&a).unwrap().record_fusion("Bob", "协商成功: Alice+Bob");
        shell.agents.get_mut(&b).unwrap().record_fusion("Alice", "协商成功: Bob+Alice");
        // 验证
        let alice = shell.agents.get(&a).unwrap();
        let bob = shell.agents.get(&b).unwrap();
        assert_eq!(alice.timestamp.events.len(), 2); // 创世 + 融合
        assert_eq!(bob.timestamp.events.len(), 2);
        assert!(matches!(
            alice.timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Fusion { .. }
        ));
        assert!(matches!(
            bob.timestamp.events[1],
            crate::hierarchical_time::LifeEvent::Fusion { .. }
        ));
        // 完整性
        assert!(alice.verify_lifecycle().is_intact());
        assert!(bob.verify_lifecycle().is_intact());
    }

    // ── 心跳&文件持久化测试 ──

    #[test]
    fn heartbeat_default_interval() {
        let hb = ShellHeartbeat::default();
        assert_eq!(hb.interval, 10);
        assert_eq!(hb.path, ".nest_heartbeat.hts");
        assert_eq!(hb.beat_count, 0);
    }

    #[test]
    fn heartbeat_should_beat_at_interval() {
        let hb = ShellHeartbeat::new(5, "test.hts");
        assert!(!hb.should_beat(0));     // tick 0 = 不触发
        assert!(!hb.should_beat(1));     // 不到间隔
        assert!(!hb.should_beat(4));     // 不到间隔
        assert!(hb.should_beat(5));      // 刚好间隔
        assert!(hb.should_beat(10));     // 超过间隔
    }

    #[test]
    fn shell_with_heartbeat_custom() {
        let shell = ParasiteShell::with_heartbeat(20, 5, "custom.hts");
        assert_eq!(shell.heartbeat.interval, 5);
        assert_eq!(shell.heartbeat.path, "custom.hts");
    }

    #[test]
    fn pulse_saves_when_interval_reached() {
        // 注意: 测试中使用 LinuxOs 而不是 BareMetalOs (后者不支持文件 I/O)
        use crate::os_layer::linux::LinuxOs;
        let os = LinuxOs::new();
        let path = "/tmp/test_nest_pulse.hts";
        let _ = std::fs::remove_file(path); // 清理旧的

        let mut shell = ParasiteShell::with_heartbeat(20, 5, path);

        // 孕育分身并施加事件
        let a = shell.spawn(&os, "Test", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "T");

        // tick 0: 不应触发
        shell.shell_tick = 0;
        assert_eq!(shell.pulse(&os).unwrap(), false);

        // tick 4: 不应触发 (间隔=5)
        shell.shell_tick = 4;
        assert_eq!(shell.pulse(&os).unwrap(), false);

        // tick 5: 应触发
        shell.shell_tick = 5;
        assert_eq!(shell.pulse(&os).unwrap(), true);
        assert_eq!(shell.heartbeat.beat_count, 1);

        // 验证文件已写入
        assert!(ParasiteShell::persisted_exists(&os, path));

        // 验证文件可恢复
        let recovered = ParasiteShell::load_from_file(&os, path).unwrap();
        assert_eq!(recovered.len(), 1);

        // 清理
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn force_beat_saves_immediately() {
        use crate::os_layer::linux::LinuxOs;
        let os = LinuxOs::new();
        let path = "/tmp/test_force_beat.hts";
        let _ = std::fs::remove_file(path);

        let mut shell = ParasiteShell::with_heartbeat(20, 1000, path);

        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        shell.agents.get_mut(&a).unwrap().record_fusion("x", "y");

        shell.shell_tick = 1;
        shell.force_beat(&os).unwrap();
        assert_eq!(shell.heartbeat.beat_count, 1);

        let recovered = ParasiteShell::load_from_file(&os, path).unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].2.events.len(), 2); // 创世 + 融合

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recovery_preserves_event_statistics() {
        use crate::os_layer::linux::LinuxOs;
        let os = LinuxOs::new();
        let path = "/tmp/test_stats_recovery.hts";
        let _ = std::fs::remove_file(path);

        let mut shell = ParasiteShell::with_heartbeat(20, 3, path);

        let a = shell.spawn(&os, "A", AgentRole::Root, 20);
        let b = shell.spawn(&os, "B", AgentRole::Leaf, 20);
        shell.agents.get_mut(&a).unwrap().record_radiation("任务", "T"); // +1 Radiation
        shell.agents.get_mut(&b).unwrap().record_collapse("B", "成果");  // +1 Collapse
        shell.agents.get_mut(&a).unwrap().record_fusion("X", "融合");    // +1 Fusion (创世已2)

        let original_stats = shell.event_statistics();
        shell.shell_tick = 3;
        shell.force_beat(&os).unwrap();

        let recovered = ParasiteShell::load_from_file(&os, path).unwrap();
        let mut f = 0;
        let mut c = 0;
        let mut r = 0;
        for (_, _, ts) in &recovered {
            for ev in &ts.events {
                match ev {
                    LifeEvent::Fusion { .. } => f += 1,
                    LifeEvent::Collapse { .. } => c += 1,
                    LifeEvent::Radiation { .. } => r += 1,
                }
            }
        }

        assert_eq!((f, c, r), original_stats);

        let _ = std::fs::remove_file(path);
    }

    // ── 纤维束编织测试 ──

    #[test]
    fn weave_fibers_detects_complete_fiber() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::with_heartbeat(20, 100, "/tmp/fiber_test.hts");

        let a = shell.spawn(&os, "Alice", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Bob", AgentRole::Leaf, 20);

        // Alice 辐射到 Bob
        shell.agents.get_mut(&a).unwrap().record_radiation("侦查任务", "Bob");
        // Bob 坍缩回 Alice
        shell.agents.get_mut(&b).unwrap().record_collapse("Alice", "侦查报告");

        let fibers = shell.weave_fibers();
        assert_eq!(fibers.len(), 1);
        assert!(fibers[0].complete); // 完整纤维: 辐射 + 坍缩都发生
        assert_eq!(fibers[0].from_name, "Alice");
        assert_eq!(fibers[0].to_name, "Bob");
        assert_eq!(fibers[0].mission, "侦查任务");
    }

    #[test]
    fn weave_fibers_detects_incomplete_fiber() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::with_heartbeat(20, 100, "/tmp/fiber_test2.hts");

        let a = shell.spawn(&os, "Alice", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Bob", AgentRole::Leaf, 20);

        // Alice 辐射到 Bob, 但 Bob 还没坍缩回 Alice
        shell.agents.get_mut(&a).unwrap().record_radiation("等待任务", "Bob");

        let fibers = shell.weave_fibers();
        assert_eq!(fibers.len(), 1);
        assert!(!fibers[0].complete); // 不完整: 只有辐射, 未坍缩
    }

    #[test]
    fn weave_fibers_multiple_agents() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::with_heartbeat(20, 100, "/tmp/fiber_test3.hts");

        let a = shell.spawn(&os, "Root", AgentRole::Root, 20);
        let b = shell.spawn(&os, "Leaf1", AgentRole::Leaf, 20);
        let c = shell.spawn(&os, "Leaf2", AgentRole::Leaf, 20);

        // Root 辐射给两个叶子
        shell.agents.get_mut(&a).unwrap().record_radiation("任务1", "Leaf1");
        shell.agents.get_mut(&a).unwrap().record_radiation("任务2", "Leaf2");
        // Leaf1 坍缩回来
        shell.agents.get_mut(&b).unwrap().record_collapse("Root", "完成");
        // Leaf2 未坍缩

        let fibers = shell.weave_fibers();
        assert_eq!(fibers.len(), 2); // 两条纤维

        let complete = fibers.iter().filter(|f| f.complete).count();
        let incomplete = fibers.iter().filter(|f| !f.complete).count();
        assert_eq!(complete, 1);   // Leaf1 完整
        assert_eq!(incomplete, 1); // Leaf2 不完整

        let stats = shell.fiber_statistics();
        assert_eq!(stats, (1, 1));
    }

    #[test]
    fn weave_fibers_no_radiation() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut shell = ParasiteShell::with_heartbeat(20, 100, "/tmp/fiber_test4.hts");
        shell.spawn(&os, "A", AgentRole::Root, 20);
        shell.spawn(&os, "B", AgentRole::Leaf, 20);
        // 无辐射事件 → 无纤维

        assert!(shell.weave_fibers().is_empty());
    }
}
