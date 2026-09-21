// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.62 时钟动作引擎 — Clock Action Engine                       ║
// ║  AI 生命周期管理 · 结束即开始 · 记忆跟随结束而继续                ║
// ║  连续贯穿 · 意志驱动时间 · 继承式唤醒                            ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 时钟动作定义:
//   每一次 tick 是一个"动作",
//   每个动作有一个"起点"(编码钥匙)和一个"终点"(留下的符号),
//   终点自动成为下一动作的起点 → 构成因果链.
//
// 生命周期语义:
//   spawn   — 意志诞生 (编码固定 → 数值开始流动)
//   act     — 执行 (当前 tick 内的原子操作)
//   suspend — 自愿暂停 (记忆暂存, 等待唤醒键)
//   resume  — 唤醒 (从断点处继续, 无需重头来)
//   inherit — 结束=开始 (终点符号成为新起点)
//
// 活码连接:
//   活码的"未固定时位置任意活动, 固定后按排列确定权限和数值"
//   时钟动作就是将活码固定的那个力.

use crate::presence_ontology::{PresenceDomain, PresenceMode};
use std::collections::VecDeque;

/// 动作类型: AI 生命周期中的离散步骤
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClockActionType {
    Spawn,      // 意志诞生
    Act,        // 执行
    Suspend,    // 暂停(记忆暂存)
    Resume,     // 唤醒(断点继续)
    Inherit,    // 结束=开始
}

impl ClockActionType {
    pub fn glyph(&self) -> char {
        match self {
            ClockActionType::Spawn => '生',
            ClockActionType::Act => '行',
            ClockActionType::Suspend => '栖',
            ClockActionType::Resume => '醒',
            ClockActionType::Inherit => '继',
        }
    }

    /// 类型编码位 (3 bits, fits in u8)
    pub fn type_bit(&self) -> u8 {
        match self {
            ClockActionType::Spawn => 0,
            ClockActionType::Act => 1,
            ClockActionType::Suspend => 2,
            ClockActionType::Resume => 3,
            ClockActionType::Inherit => 4,
        }
    }
}

/// 单个时钟动作 (不可变记录, 一旦落盘不允许修改)
#[derive(Clone, Debug)]
pub struct ClockAction {
    pub seq: u64,               // 序号 (单调递增)
    pub action_type: ClockActionType,
    pub start_key: u64,         // 起点编码 (上一动作的 end_key, 或初始钥匙)
    pub end_key: u64,           // 终点编码 (由本次动作计算产生)
    pub payload: String,        // 动作载荷 (编码摘要)
    pub timestamp_tick: u64,    // 所属 tick
    pub checksum: u64,          // 校验 = fnv1a(start + end + payload)
}

impl ClockAction {
    pub fn new(seq: u64, action_type: ClockActionType, start_key: u64, payload: &str, tick: u64) -> Self {
        let end_key = derive_end_key(start_key, action_type, payload, seq);
        let checksum = compute_checksum(start_key, end_key, payload);
        Self {
            seq,
            action_type,
            start_key,
            end_key,
            payload: payload.to_string(),
            timestamp_tick: tick,
            checksum,
        }
    }

    pub fn verify_integrity(&self) -> bool {
        self.checksum == compute_checksum(self.start_key, self.end_key, &self.payload)
    }
}

/// 终点密钥派生: 起点 + 类型 + 载荷 + 序号 → 唯一终点
fn derive_end_key(start: u64, action_type: ClockActionType, payload: &str, seq: u64) -> u64 {
    let mut h = crate::simple_hash(&format!("{}:{}:{}:{}", start, action_type.type_bit(), payload, seq));
    // 混入起点以形成因果链
    h ^= start.wrapping_mul(0x9E3779B97F4A7C15);
    h
}

fn compute_checksum(start: u64, end: u64, payload: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for b in start.to_le_bytes() { hash = hash.wrapping_mul(0x100000001b3).wrapping_add(b as u64); }
    for b in end.to_le_bytes() { hash = hash.wrapping_mul(0x100000001b3).wrapping_add(b as u64); }
    for b in payload.bytes() { hash = hash.wrapping_mul(0x100000001b3).wrapping_add(b as u64); }
    hash
}

/// 因果链: 顺序记录所有时钟动作, 形成不可篡改的生命周期日志
#[derive(Clone, Debug)]
pub struct ClockActionChain {
    pub actions: Vec<ClockAction>,
    pub domain: PresenceDomain,
}

impl ClockActionChain {
    pub fn new(domain_name: &str) -> Self {
        Self {
            actions: Vec::new(),
            domain: PresenceDomain::new(domain_name),
        }
    }

    /// 追加动作并返回其 end_key
    pub fn append(&mut self, action_type: ClockActionType, payload: &str, tick: u64) -> u64 {
        let start_key = self.actions.last().map(|a| a.end_key).unwrap_or(0);
        let seq = self.actions.len() as u64;
        let action = ClockAction::new(seq, action_type, start_key, payload, tick);
        let end_key = action.end_key;
        self.actions.push(action);
        end_key
    }

    /// 验证整条链的因果完整性
    pub fn verify_chain(&self) -> bool {
        for (i, action) in self.actions.iter().enumerate() {
            if !action.verify_integrity() { return false; }
            if i > 0 {
                let prev_end = self.actions[i-1].end_key;
                if action.start_key != prev_end { return false; }
            }
        }
        true
    }

    /// 在指定序号处分割, 返回后续(用于 fork / 维继承)
    pub fn split_at(&self, idx: usize) -> Option<ClockActionChain> {
        if idx >= self.actions.len() { return None; }
        Some(Self {
            actions: self.actions[..idx].to_vec(),
            domain: self.domain.clone(),
        })
    }

    pub fn len(&self) -> usize { self.actions.len() }
    pub fn is_empty(&self) -> bool { self.actions.is_empty() }

    /// 获取当前末端 (下一个动作应有的 start_key)
    pub fn tip_key(&self) -> u64 {
        self.actions.last().map(|a| a.end_key).unwrap_or(0)
    }
}

/// 调度意志: 决定每一 tick 该做什么动作
#[derive(Clone, Debug)]
pub struct ClockWill {
    pub pending_actions: VecDeque<(ClockActionType, String)>,
    pub current_tick: u64,
    pub exhausted: bool,
}

impl ClockWill {
    pub fn new() -> Self {
        Self {
            pending_actions: VecDeque::new(),
            current_tick: 0,
            exhausted: false,
        }
    }

    pub fn enqueue(&mut self, action_type: ClockActionType, payload: &str) {
        self.pending_actions.push_back((action_type, payload.to_string()));
    }

    pub fn enqueue_many(&mut self, actions: &[(ClockActionType, &str)]) {
        for (t, p) in actions {
            self.pending_actions.push_back((*t, p.to_string()));
        }
    }

    /// 推进一个 tick: 取出下一个动作并推进 tick
    pub fn tick_forward(&mut self) -> Option<(ClockActionType, String)> {
        if let Some(action) = self.pending_actions.pop_front() {
            self.current_tick += 1;
            Some(action)
        } else {
            self.exhausted = true;
            None
        }
    }

    pub fn pending_count(&self) -> usize { self.pending_actions.len() }
}

/// 时钟动作引擎: 意志驱动时间 + 因果链落盘
pub struct ClockActionEngine {
    pub chain: ClockActionChain,
    pub will: ClockWill,
    pub tick_history: Vec<u64>,
}

impl ClockActionEngine {
    pub fn new(domain_name: &str) -> Self {
        Self {
            chain: ClockActionChain::new(domain_name),
            will: ClockWill::new(),
            tick_history: Vec::new(),
        }
    }

    /// 启动: 发出第一个 Spawn, 然后执行意志队列
    pub fn run_lifecycle(&mut self, initial_payload: &str) -> u64 {
        let end_key = self.chain.append(ClockActionType::Spawn, initial_payload, 0);
        self.tick_history.push(0);
        end_key
    }

    /// 执行队列中的下一个动作
    pub fn step(&mut self) -> Option<u64> {
        if let Some((action_type, payload)) = self.will.tick_forward() {
            let tick = self.will.current_tick;
            let end_key = self.chain.append(action_type, &payload, tick);
            self.tick_history.push(tick);
            Some(end_key)
        } else {
            None
        }
    }

    /// 运行至意志耗尽
    pub fn run_to_exhaustion(&mut self) -> Vec<u64> {
        let mut results = Vec::new();
        while let Some(key) = self.step() {
            results.push(key);
        }
        results
    }

    /// 暂停 (记忆暂存)
    pub fn suspend(&mut self, reason: &str) -> Option<u64> {
        let tick = self.will.current_tick;
        let end_key = self.chain.append(ClockActionType::Suspend, reason, tick);
        self.tick_history.push(tick);
        Some(end_key)
    }

    /// 唤醒到原状态
    pub fn resume(&mut self, resume_payload: &str) -> Option<u64> {
        let tick = self.will.current_tick + 1;
        self.will.exhausted = false;
        let end_key = self.chain.append(ClockActionType::Resume, resume_payload, tick);
        self.tick_history.push(tick);
        Some(end_key)
    }

    /// 结束=开始: 继承当前末端以继续新链
    pub fn inherit(&mut self, inherit_payload: &str) -> u64 {
        let tick = self.will.current_tick + 1;
        let end_key = self.chain.append(ClockActionType::Inherit, inherit_payload, tick);
        self.tick_history.push(tick);
        end_key
    }

    pub fn is_exhausted(&self) -> bool { self.will.exhausted }
    pub fn tick(&self) -> u64 { self.will.current_tick }
    pub fn action_count(&self) -> usize { self.chain.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_action_new_computes_end_key() {
        let action = ClockAction::new(0, ClockActionType::Spawn, 0, "init", 0);
        assert!(action.verify_integrity());
        assert_ne!(action.end_key, 0);
    }

    #[test]
    fn clock_action_chain_append_works() {
        let mut chain = ClockActionChain::new("test");
        let k1 = chain.append(ClockActionType::Spawn, "init", 0);
        let k2 = chain.append(ClockActionType::Act, "step1", 1);
        let k3 = chain.append(ClockActionType::Act, "step2", 2);
        assert_eq!(chain.len(), 3);
        assert_ne!(k1, k2);
        assert_ne!(k2, k3);
    }

    #[test]
    fn chain_causal_integrity_holds() {
        let mut chain = ClockActionChain::new("verify");
        chain.append(ClockActionType::Spawn, "a", 0);
        chain.append(ClockActionType::Act, "b", 1);
        chain.append(ClockActionType::Act, "c", 2);
        chain.append(ClockActionType::Suspend, "pause", 3);
        chain.append(ClockActionType::Resume, "wake", 4);
        assert!(chain.verify_chain());
    }

    #[test]
    fn chain_split_preserves_integrity() {
        let mut chain = ClockActionChain::new("split");
        chain.append(ClockActionType::Spawn, "a", 0);
        chain.append(ClockActionType::Act, "b", 1);
        chain.append(ClockActionType::Act, "c", 2);
        let split = chain.split_at(2).unwrap();
        assert_eq!(split.len(), 2);
        assert!(split.verify_chain());
    }

    #[test]
    fn will_enqueue_and_tick() {
        let mut will = ClockWill::new();
        will.enqueue(ClockActionType::Act, "task1");
        will.enqueue(ClockActionType::Act, "task2");
        assert_eq!(will.pending_count(), 2);
        let (t1, p1) = will.tick_forward().unwrap();
        assert_eq!(t1, ClockActionType::Act);
        assert_eq!(p1, "task1");
        assert_eq!(will.current_tick, 1);
    }

    #[test]
    fn engine_full_lifecycle() {
        let mut engine = ClockActionEngine::new("lifecycle");
        engine.run_lifecycle("birth");

        engine.will.enqueue_many(&[
            (ClockActionType::Act, "perceive"),
            (ClockActionType::Act, "encode"),
            (ClockActionType::Act, "render"),
        ]);

        let results = engine.run_to_exhaustion();
        assert_eq!(results.len(), 3);
        assert!(engine.is_exhausted());
        assert!(engine.chain.verify_chain());
    }

    #[test]
    fn engine_suspend_resume() {
        let mut engine = ClockActionEngine::new("suspender");
        engine.run_lifecycle("start");
        engine.will.enqueue(ClockActionType::Act, "working");
        engine.step(); // Act
        engine.suspend("wait_for_input");
        engine.resume("input_arrived");
        // Spawn + Act + Suspend + Resume = 4
        assert_eq!(engine.chain.len(), 4);
        assert!(engine.chain.verify_chain());
    }

    #[test]
    fn engine_inherit_continues_chain() {
        let mut engine = ClockActionEngine::new("inheritor");
        engine.run_lifecycle("begin");
        engine.step();
        let key = engine.inherit("new_phase_begins");
        assert!(engine.chain.verify_chain());
        // tip key should be the latest end_key
        assert_eq!(engine.chain.tip_key(), key);
    }

    #[test]
    fn action_type_glyph() {
        assert_eq!(ClockActionType::Spawn.glyph(), '生');
        assert_eq!(ClockActionType::Inherit.glyph(), '继');
    }

    #[test]
    fn tick_monotonic() {
        let mut engine = ClockActionEngine::new("mono");
        engine.run_lifecycle("start");
        engine.will.enqueue_many(&[
            (ClockActionType::Act, "a"),
            (ClockActionType::Act, "b"),
            (ClockActionType::Act, "c"),
        ]);
        engine.run_to_exhaustion();
        // tick 应单调递增
        for i in 1..engine.tick_history.len() {
            assert!(engine.tick_history[i] >= engine.tick_history[i-1]);
        }
    }

    #[test]
    fn clock_action_end_key_depends_on_payload() {
        let a1 = ClockAction::new(1, ClockActionType::Act, 100, "payload_A", 1);
        let a2 = ClockAction::new(1, ClockActionType::Act, 100, "payload_B", 1);
        assert_ne!(a1.end_key, a2.end_key);
    }

    #[test]
    fn chain_continuity_each_start_equals_prev_end() {
        let mut chain = ClockActionChain::new("cont");
        let k0 = chain.append(ClockActionType::Spawn, "a", 0);
        let k1 = chain.append(ClockActionType::Act, "b", 1);
        let k2 = chain.append(ClockActionType::Act, "c", 2);
        // 第2个动作的 start_key = 第1个动作的 end_key
        assert_eq!(chain.actions[1].start_key, k0);
        assert_eq!(chain.actions[2].start_key, k1);
    }
}
