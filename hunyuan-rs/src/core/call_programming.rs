// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.72 呼唤编程引擎                                              ║
// ║  呼唤表 · 公约数调用 · 动态/静态容器 · 观察者/被观察者            ║
// ║  迂回反馈 · 反射感知 · 判断之力 · 评价之力                        ║
// ║                                                                  ║
// ║  核心: 编程 = 建立关系模板 + 只写数字                             ║
// ║  · 呼唤者 = 动态容器, 主动发出呼唤信号                            ║
// ║  · 被呼唤者 = 静态容器, 被动接收呼唤                              ║
// ║  · 每次呼唤自动建立观察/被观察关系                                ║
// ║  · 角色是会话状态, 可互换                                         ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerKind, ContainerRef, RelationKind};
use std::collections::HashMap;

/// 容器角色 (会话状态, 非固有属性)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallRole {
    /// 动态容器 — 主动发出呼唤
    Dynamic,
    /// 静态容器 — 被动接收呼唤
    Static,
    /// 中性 — 未参与呼唤
    Neutral,
}

/// 观察关系
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationRole {
    /// 观察者 — 呼唤发起方
    Observer,
    /// 被观察者 — 呼唤接收方
    Observed,
    /// 未参与观察
    Unrelated,
}

/// 反馈类型 — 信号在迂回中的回响
#[derive(Clone, Debug)]
pub enum Feedback {
    /// 直接响应
    Direct { value: u64 },
    /// 迂回回响 — 信号被环境改变后返回
    Echo { original: u64, reflected: u64, round: u32 },
    /// 差异标记 — 涟漪形状/幅度/相位变化
    Difference { from: u64, to: u64, offset: f64 },
    /// 关系锚点 — 判断结果: "A型偏移由于B引起"
    RelationAnchor { cause: u64, effect: u64, confidence: f64 },
}

/// 呼唤返回 — 不是值返回, 而是烙印 (时间链上的状态事件)
#[derive(Clone, Debug)]
pub struct CallAck {
    /// 本次呼唤的响应值
    pub response: u64,
    /// 反馈链
    pub feedback_chain: Vec<Feedback>,
    /// 观察者状态变化
    pub observer_state: u64,
    /// 被观察者状态变化
    pub observed_state: u64,
    /// 是否成功
    pub success: bool,
}

impl CallAck {
    pub fn new(response: u64) -> Self {
        CallAck {
            response,
            feedback_chain: Vec::new(),
            observer_state: 0,
            observed_state: 0,
            success: true,
        }
    }

    pub fn failed() -> Self {
        CallAck {
            response: 0,
            feedback_chain: Vec::new(),
            observer_state: 0,
            observed_state: 0,
            success: false,
        }
    }

    /// 添加反馈
    pub fn push_feedback(&mut self, fb: Feedback) {
        self.feedback_chain.push(fb);
    }

    /// 反馈深度
    pub fn feedback_depth(&self) -> usize {
        self.feedback_chain.len()
    }
}

/// 呼唤表条目 — 公约数藏路径, 不藏代码
#[derive(Clone)]
pub struct CallEntry {
    /// 路径索引 (零维地址)
    pub index: u64,
    /// 目标容器引用
    pub target: ContainerRef,
    /// 条目描述
    pub desc: String,
    /// 调用次数统计
    pub call_count: u64,
}

/// 呼唤表 — 公约数的容器
pub struct CallTable {
    /// 索引 → 条目
    entries: HashMap<u64, CallEntry>,
    /// 总调用次数
    total_calls: u64,
}

impl CallTable {
    pub fn new() -> Self {
        CallTable {
            entries: HashMap::new(),
            total_calls: 0,
        }
    }

    /// 注册条目 (公约数)
    pub fn register(&mut self, index: u64, target: ContainerRef, desc: &str) {
        self.entries.insert(index, CallEntry {
            index,
            target,
            desc: desc.to_string(),
            call_count: 0,
        });
    }

    /// 调用 — 只写零维数字
    pub fn call(&mut self, index: u64, caller_state: u64) -> CallAck {
        match self.entries.get_mut(&index) {
            Some(entry) => {
                entry.call_count += 1;
                self.total_calls += 1;

                let response = entry.target.entity().wrapping_add(caller_state);
                CallAck::new(response)
            }
            None => CallAck::failed(),
        }
    }

    /// 获取条目
    pub fn get(&self, index: u64) -> Option<&CallEntry> {
        self.entries.get(&index)
    }

    /// 条目数
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 总调用次数
    pub fn total_calls(&self) -> u64 {
        self.total_calls
    }

    /// 获取最常用的条目
    pub fn most_called(&self) -> Option<&CallEntry> {
        self.entries.values().max_by_key(|e| e.call_count)
    }

    /// 迭代所有条目
    pub fn iter(&self) -> impl Iterator<Item = &CallEntry> {
        self.entries.values()
    }
}

impl Default for CallTable {
    fn default() -> Self {
        Self::new()
    }
}

/// 会话状态 — 每次呼唤的角色绑定
#[derive(Clone, Debug)]
pub struct CallSession {
    /// 会话 ID
    pub session_id: u64,
    /// 呼唤者 (动态容器)
    pub caller: ContainerRef,
    /// 被呼唤者 (静态容器)
    pub callee: ContainerRef,
    /// 观察/被观察角色
    pub observation: ObservationRole,
    /// 反馈深度 (迂回次数)
    pub echo_depth: u32,
    /// 状态标记 — 时间链上的烙印
    pub state_mark: u64,
}

impl CallSession {
    pub fn new(session_id: u64, caller: ContainerRef, callee: ContainerRef) -> Self {
        CallSession {
            session_id,
            caller,
            callee,
            observation: ObservationRole::Observer,
            echo_depth: 0,
            state_mark: 0,
        }
    }

    /// 切换角色 — 呼唤完成后动态↔静态互换
    pub fn swap_roles(&mut self) {
        std::mem::swap(&mut self.caller, &mut self.callee);
        self.observation = match self.observation {
            ObservationRole::Observer => ObservationRole::Observed,
            ObservationRole::Observed => ObservationRole::Observer,
            ObservationRole::Unrelated => ObservationRole::Unrelated,
        };
    }

    /// 增加反馈深度
    pub fn echo(&mut self) {
        self.echo_depth += 1;
        self.state_mark = self.state_mark.wrapping_add(1);
    }

    /// 获取当前动态容器
    pub fn dynamic_ref(&self) -> &ContainerRef {
        &self.caller
    }

    /// 获取当前静态容器
    pub fn static_ref(&self) -> &ContainerRef {
        &self.callee
    }
}

/// 评价引擎 — 因果对排序, 形成逻辑路径
pub struct Evaluator {
    /// 因果对: (cause, effect) → 权重
    causal_pairs: HashMap<(u64, u64), f64>,
    /// 评价阈值
    threshold: f64,
}

impl Evaluator {
    pub fn new(threshold: f64) -> Self {
        Evaluator {
            causal_pairs: HashMap::new(),
            threshold,
        }
    }

    /// 添加因果对
    pub fn add_causal_pair(&mut self, cause: u64, effect: u64, weight: f64) {
        let key = (cause, effect);
        let current = self.causal_pairs.get(&key).copied().unwrap_or(0.0);
        self.causal_pairs.insert(key, current + weight);
    }

    /// 判断因果强度是否达标
    pub fn is_significant(&self, cause: u64, effect: u64) -> bool {
        self.causal_pairs
            .get(&(cause, effect))
            .map(|&w| w >= self.threshold)
            .unwrap_or(false)
    }

    /// 获取最强因果链
    pub fn strongest_cause(&self, effect: u64) -> Option<(u64, f64)> {
        self.causal_pairs
            .iter()
            .filter(|item| (item.0).1 == effect)  // key.1 is the effect
            .max_by(|a, b| (a.1).partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|item| ((item.0).0, *item.1))   // key.0 is the cause
    }

    /// 评价总数
    pub fn pair_count(&self) -> usize {
        self.causal_pairs.len()
    }

    /// 自组织: 调整呼唤策略 (返回应该强化的索引列表)
    pub fn reinforce_strategy(&self) -> Vec<(u64, u64, f64)> {
        let mut pairs: Vec<(u64, u64, f64)> = self.causal_pairs
            .iter()
            .filter(|item| *item.1 >= self.threshold)
            .map(|item| ((item.0).0, (item.0).1, *item.1))
            .collect();
        pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        pairs
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new(1.0)
    }
}

/// 呼唤编程引擎 — 总协调
pub struct CallingEngine {
    /// 呼唤表
    pub table: CallTable,
    /// 当前会话
    sessions: Vec<CallSession>,
    /// 评价引擎
    pub evaluator: Evaluator,
    /// 全局状态
    pub global_state: u64,
}

impl CallingEngine {
    pub fn new() -> Self {
        CallingEngine {
            table: CallTable::new(),
            sessions: Vec::new(),
            evaluator: Evaluator::new(1.0),
            global_state: 0,
        }
    }

    /// 创建会话并呼唤
    pub fn invoke(&mut self, index: u64, caller: ContainerRef, callee: ContainerRef) -> Option<CallAck> {
        let ack = self.table.call(index, self.global_state);

        if ack.success {
            let session = CallSession::new(self.sessions.len() as u64, caller, callee);
            self.sessions.push(session);
            self.global_state = self.global_state.wrapping_add(ack.response);
        }

        Some(ack)
    }

    /// 建立迂回反馈 — 一次呼唤多次回响
    pub fn echo_round(&mut self, session_idx: u64, rounds: u32, amplitude: f64) -> Vec<Feedback> {
        let mut feedbacks = Vec::new();

        if let Some(session) = self.sessions.get_mut(session_idx as usize) {
            for r in 1..=rounds {
                session.echo();
                let reflected = (session.state_mark as f64 * amplitude) as u64;
                feedbacks.push(Feedback::Echo {
                    original: session.state_mark,
                    reflected,
                    round: r,
                });
            }

            // 反射感知: 差异标记
            if rounds >= 2 {
                let diff = amplitude * rounds as f64;
                feedbacks.push(Feedback::Difference {
                    from: session.state_mark - rounds as u64,
                    to: session.state_mark,
                    offset: diff,
                });
            }
        }

        feedbacks
    }

    /// 仲裁评价 — 因果对强化
    pub fn evaluate(&mut self, cause: u64, effect: u64, weight: f64) {
        self.evaluator.add_causal_pair(cause, effect, weight);
    }

    /// 获取会话数
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// 同步到图集
    pub fn sync_to_atlas(&self, atlas: &mut ContainerAtlas) {
        for entry in self.table.iter() {
            atlas.register(entry.target.clone());
        }
        for session in &self.sessions {
            atlas.register(session.caller.clone());
            atlas.register(session.callee.clone());
            atlas.link(
                session.caller.clone(),
                session.callee.clone(),
                RelationKind::Resonance,
                1,
            );
        }
    }

    /// 自组织调整 — 根据评价结果优化呼唤策略
    pub fn auto_adjust(&self) -> Vec<(u64, u64, f64)> {
        self.evaluator.reinforce_strategy()
    }
}

impl Default for CallingEngine {
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
    fn call_table_register_and_call() {
        let mut table = CallTable::new();
        let target = mk_ref(0x6000_0000);
        table.register(7, target, "满全法");

        let ack = table.call(7, 100);
        assert!(ack.success);
        assert!(ack.response > 0);
        assert_eq!(table.total_calls(), 1);
    }

    #[test]
    fn call_table_missing_index() {
        let mut table = CallTable::new();
        let ack = table.call(999, 0);
        assert!(!ack.success);
    }

    #[test]
    fn call_table_increments_count() {
        let mut table = CallTable::new();
        table.register(1, mk_ref(100), "t1");

        table.call(1, 0);
        table.call(1, 0);
        table.call(1, 0);

        assert_eq!(table.get(1).unwrap().call_count, 3);
        assert_eq!(table.total_calls(), 3);
    }

    #[test]
    fn call_table_most_called() {
        let mut table = CallTable::new();
        table.register(1, mk_ref(100), "t1");
        table.register(2, mk_ref(200), "t2");

        for _ in 0..5 { table.call(1, 0); }
        for _ in 0..10 { table.call(2, 0); }

        let most = table.most_called().unwrap();
        assert_eq!(most.index, 2);
    }

    #[test]
    fn call_session_new() {
        let caller = mk_ref(100);
        let callee = mk_ref(200);
        let session = CallSession::new(0, caller, callee);

        assert_eq!(session.observation, ObservationRole::Observer);
        assert_eq!(session.echo_depth, 0);
    }

    #[test]
    fn call_session_swap_roles() {
        let caller = mk_ref(100);
        let callee = mk_ref(200);
        let mut session = CallSession::new(0, caller.clone(), callee.clone());
        session.swap_roles();

        // 呼唤后角色互换: 原caller变static, 原callee变dynamic
        assert_eq!(session.caller.raw(), callee.raw());
        assert_eq!(session.callee.raw(), caller.raw());
        assert_eq!(session.observation, ObservationRole::Observed);
    }

    #[test]
    fn call_session_echo_increments() {
        let mut session = CallSession::new(0, mk_ref(1), mk_ref(2));
        session.echo();
        session.echo();
        session.echo();

        assert_eq!(session.echo_depth, 3);
        assert_eq!(session.state_mark, 3);
    }

    #[test]
    fn evaluator_causal_pair() {
        let mut ev = Evaluator::new(1.0);
        ev.add_causal_pair(100, 200, 0.5);
        ev.add_causal_pair(100, 200, 0.5); // 累加到 1.0

        assert!(ev.is_significant(100, 200));
        assert!(!ev.is_significant(100, 999));
    }

    #[test]
    fn evaluator_strongest_cause() {
        let mut ev = Evaluator::new(0.5);
        ev.add_causal_pair(100, 500, 0.3);
        ev.add_causal_pair(200, 500, 0.8);
        ev.add_causal_pair(300, 500, 0.6);

        let strongest = ev.strongest_cause(500).unwrap();
        assert_eq!(strongest.0, 200);
        assert!((strongest.1 - 0.8).abs() < 1e-9);
    }

    #[test]
    fn evaluator_reinforce_strategy() {
        let mut ev = Evaluator::new(1.0);
        ev.add_causal_pair(10, 20, 2.0);
        ev.add_causal_pair(30, 40, 0.5); // 低于阈值
        ev.add_causal_pair(50, 60, 3.0);

        let strategy = ev.reinforce_strategy();
        assert_eq!(strategy.len(), 2); // 只有 >= 1.0 的
        assert_eq!(strategy[0].0, 50); // 权重最高的在前
        assert_eq!(strategy[1].0, 10);
    }

    #[test]
    fn calling_engine_invoke_creates_session() {
        let mut engine = CallingEngine::new();
        engine.table.register(7, mk_ref(0x6000_0000), "test");

        let ack = engine.invoke(7, mk_ref(100), mk_ref(200));
        assert!(ack.unwrap().success);
        assert_eq!(engine.session_count(), 1);
    }

    #[test]
    fn calling_engine_echo_round() {
        let mut engine = CallingEngine::new();
        engine.table.register(1, mk_ref(1000), "t");
        engine.invoke(1, mk_ref(100), mk_ref(200));

        let feedbacks = engine.echo_round(0, 5, 1.5);
        // 5 次 Echo + 1 次 Difference = 6
        assert_eq!(feedbacks.len(), 6);
    }

    #[test]
    fn calling_engine_global_state_accumulates() {
        let mut engine = CallingEngine::new();
        engine.table.register(1, mk_ref(0x6000_1000), "t");
        engine.invoke(1, mk_ref(100), mk_ref(200));
        let after_first = engine.global_state;

        engine.invoke(1, mk_ref(100), mk_ref(200));
        assert!(engine.global_state != after_first);
    }

    #[test]
    fn calling_engine_auto_adjust() {
        let mut engine = CallingEngine::new();
        engine.evaluate(100, 200, 2.0);
        engine.evaluate(300, 400, 0.5);
        engine.evaluate(500, 600, 5.0);

        let strategy = engine.auto_adjust();
        assert_eq!(strategy.len(), 2); // >= 1.0 的只有 (100,200) 和 (500,600)
        assert!(strategy.iter().any(|&(c, _, _)| c == 100));
        assert!(strategy.iter().any(|&(c, _, _)| c == 500));
    }

    #[test]
    fn calling_engine_sync_to_atlas() {
        let mut engine = CallingEngine::new();
        engine.table.register(7, mk_ref(0x6000_0000), "满全法");
        engine.table.register(8, mk_ref(0x6000_0001), "囡囝共生");

        let mut atlas = ContainerAtlas::new();
        engine.sync_to_atlas(&mut atlas);
        assert_eq!(atlas.count(), 2);
    }

    #[test]
    fn call_ack_feedback_chain() {
        let mut ack = CallAck::new(42);
        ack.push_feedback(Feedback::Direct { value: 100 });
        ack.push_feedback(Feedback::Echo { original: 100, reflected: 90, round: 1 });
        assert_eq!(ack.feedback_depth(), 2);
    }

    #[test]
    fn call_ack_failed() {
        let ack = CallAck::failed();
        assert!(!ack.success);
        assert_eq!(ack.response, 0);
        assert_eq!(ack.feedback_depth(), 0);
    }
}
