// ══════════════════════════════════════════════════════════════════════
// eight_unify.rs — 八流统一引擎 · 操作系统的心脏 (v0.54)
// ══════════════════════════════════════════════════════════════════════
//
//   「八流不是八个管道 — 是八条解读同一编码的不同维度」
//
//   八流统一引擎是 hunyuan-os 的运行时心脏:
//
//     预加载流 (Preload)      — 运行前加载所有符号, 运行时不增加内存
//     谐音流   (Resonance)    — 韵母/声母的公约数共振, 不是搜索是吸引
//     主权流   (Sovereignty)  — 极性(夫/妻) × 主权(全/部分) 决定路由
//     意识流   (Consciousness) — 观察者/被观察者 双向觉知
//     时间流   (Temporal)     — 生命周期相位, 时间层级, 时序因果
//     产权流   (Property)     — Need/Offer 匹配, 在场登记
//     反馈流   (Feedback)     — 迂回反馈 · 反射感知 · 判断/评价
//     存在流   (Presence)     — 到场/退场, 局部投现自杀, 冰箱模型
//
//   引擎的职责:
//     1. 编排八流执行的时序 (因果路径排序 → 逻辑组织引擎 v0.46)
//     2. 在各流之间传递签名上下文 (容器交互 v0.47)
//     3. 在每条编码上同步三元形态 (容器=字体=时间 v0.48)
//     4. 维持在场状态并持久化到 .presence 文件
//
//   这不是"控制器" — 是"协议栈的编排器"
//   各流之间不直接调用, 通过引擎的上下文总线交换签名
//
//   设计约束:
//     - 八流地位相等, 没有"主流程"
//     - 引擎是单次"心跳"驱动: tick → advance → persist
//     - 不阻塞: 哪些流在本 tick 可推进, 由因果图决定
//

use crate::logic_organizer::{CausalGraph, CausalNode, CausalPath, InvocationStrategy, LogicOrganizer};
use crate::container_interact::{ContainerPort, InteractionEngine, RoutingDirection};
use crate::encoding::container_font_unify::Trinity;
use crate::consciousness_router::ConsciFlowType;
use crate::resonance_discovery::is_resonant;

// ────────────────────────────────────────────────────────────
// 八流上下文总线 — 单次心跳传递的签名状态
// ────────────────────────────────────────────────────────────

pub struct EightContext {
    /// 当前主签名 (编码)
    pub signature: u64,
    /// 三元解读
    pub trinity: Trinity,
    /// 所在八流类型
    pub flow_type: ConsciFlowType,
    /// 心跳计数
    pub tick: u64,
    /// 已通过的流 (因果链)
    pub visited_flows: Vec<FlowId>,
    /// 附加数据 (流间传递)
    pub payload: Vec<u8>,
    /// 是否已经过谐音校验
    pub resonance_verified: bool,
    /// 是否已经过主权校验
    pub sovereignty_verified: bool,
    /// 是否已经过意识觉知
    pub consciousness_aware: bool,
}

impl EightContext {
    pub fn new(signature: u64) -> Self {
        Self {
            signature,
            trinity: Trinity::decode(signature),
            flow_type: ConsciFlowType::Perception,
            tick: 0,
            visited_flows: Vec::new(),
            payload: Vec::new(),
            resonance_verified: false,
            sovereignty_verified: false,
            consciousness_aware: false,
        }
    }

    pub fn with_flow(signature: u64, flow_type: ConsciFlowType) -> Self {
        Self {
            signature,
            trinity: Trinity::decode(signature),
            flow_type,
            tick: 0,
            visited_flows: Vec::new(),
            payload: Vec::new(),
            resonance_verified: false,
            sovereignty_verified: false,
            consciousness_aware: false,
        }
    }

    /// 流转到下一个流
    pub fn transition_to(&mut self, next: FlowId) {
        self.visited_flows.push(next);
    }

    /// 是否访问过某流
    pub fn visited(&self, flow: FlowId) -> bool {
        self.visited_flows.contains(&flow)
    }

    /// 在场检查: 是否在活跃时间相
    pub fn is_alive(&self) -> bool {
        self.trinity.time.is_active()
    }

    /// 协调性检查: 容器=字体=时间 三者和睦
    pub fn is_coherent(&self) -> bool {
        self.trinity.is_coherent()
    }
}

// ────────────────────────────────────────────────────────────
// 流标识 — 八条流
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlowId {
    Preload,
    Resonance,
    Sovereignty,
    Consciousness,
    Temporal,
    Property,
    Feedback,
    Presence,
}

impl FlowId {
    pub fn all() -> [FlowId; 8] {
        [
            FlowId::Preload,
            FlowId::Resonance,
            FlowId::Sovereignty,
            FlowId::Consciousness,
            FlowId::Temporal,
            FlowId::Property,
            FlowId::Feedback,
            FlowId::Presence,
        ]
    }

    pub fn base_priority(&self) -> u8 {
        match self {
            FlowId::Preload => 0,       // 最先
            FlowId::Resonance => 1,
            FlowId::Sovereignty => 2,
            FlowId::Consciousness => 3,
            FlowId::Temporal => 4,
            FlowId::Property => 5,
            FlowId::Feedback => 6,
            FlowId::Presence => 7,      // 最后 (持久化)
        }
    }
}

impl std::fmt::Display for FlowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FlowId::Preload => "预加载",
            FlowId::Resonance => "谐音",
            FlowId::Sovereignty => "主权",
            FlowId::Consciousness => "意识",
            FlowId::Temporal => "时间",
            FlowId::Property => "产权",
            FlowId::Feedback => "反馈",
            FlowId::Presence => "存在",
        })
    }
}

// ────────────────────────────────────────────────────────────
// 心跳事件 — 引擎的运行记录
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TickEvent {
    pub tick: u64,
    pub flow: FlowId,
    pub signature: u64,
    pub action: TickAction,
}

#[derive(Clone, Debug)]
pub enum TickAction {
    FlowAdvanced,
    FlowBlocked(String),
    ResonanceMatched(u64, u64),
    SovereigntyOk,
    SovereigntyRejected(String),
    ConsciousnessAware,
    TimeAdvanced(u8),
    FeedbackReceived(String),
    Persisted,
    Suicide, // 局部投现自杀
}

impl TickAction {
    fn describe(&self) -> String {
        match self {
            Self::FlowAdvanced => "流推进".to_string(),
            Self::FlowBlocked(r) => format!("流阻塞({})", r),
            Self::ResonanceMatched(a, b) => format!("共振匹配({a},{b})"),
            Self::SovereigntyOk => "主权校验通过".to_string(),
            Self::SovereigntyRejected(r) => format!("主权拒绝({})", r),
            Self::ConsciousnessAware => "意识觉知".to_string(),
            Self::TimeAdvanced(p) => format!("时间推进(phase={p})"),
            Self::FeedbackReceived(f) => format!("反馈({f})"),
            Self::Persisted => "已持久化".to_string(),
            Self::Suicide => "局部投现自杀".to_string(),
        }
    }
}

// ────────────────────────────────────────────────────────────
// 推进结果 — 单条流在本 tick 的状态
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowStatus {
    Ready,      // 可以推进
    Blocked,    // 等待上游流完成
    Completed,  // 已完成, 不再前进
    NotReady,   // 条件不满足
}

// ────────────────────────────────────────────────────────────
// 八流统一引擎 — 核心结构
// ────────────────────────────────────────────────────────────

pub struct EightUnifyEngine {
    /// 主因果图 (编排八流依赖)
    pub organizer: LogicOrganizer,
    /// 交互引擎 (容器端口管理)
    pub interact: InteractionEngine,
    /// 当前运行的签名集合
    pub signatures: Vec<u64>,
    /// 心跳日志
    pub events: Vec<TickEvent>,
    /// 当前 tick
    pub current_tick: u64,
    /// 已处理签名计数
    pub processed: usize,
    /// 流完成状态
    pub flow_status: std::collections::HashMap<FlowId, FlowStatus>,
}

impl EightUnifyEngine {
    pub fn new(strategy: InvocationStrategy) -> Self {
        let mut flow_status = std::collections::HashMap::new();
        for flow in FlowId::all() {
            flow_status.insert(flow, FlowStatus::NotReady);
        }

        Self {
            organizer: LogicOrganizer::new(strategy),
            interact: InteractionEngine::new(strategy),
            signatures: Vec::new(),
            events: Vec::new(),
            current_tick: 0,
            processed: 0,
            flow_status,
        }
    }

    /// 输入一组签名
    pub fn input(&mut self, signatures: &[u64]) {
        self.signatures.extend_from_slice(signatures);
    }

    /// 注册容器端口
    pub fn register_port(&mut self, port: ContainerPort) {
        self.interact.register(port);
    }

    /// 初始化 — 构建八流因果图
    fn build_flow_graph(&mut self) {
        // 八流依赖关系:
        // Preload → Resonance → Sovereignty → Consciousness
        //                         ↓
        //                       Temporal → Property → Feedback → Presence
        self.organizer.add(0, 100, vec![1]);    // Preload → Resonance
        self.organizer.add(1, 200, vec![2]);    // Resonance → Sovereignty
        self.organizer.add(2, 300, vec![3, 4]); // Sovereignty → Consciousness + Temporal
        self.organizer.add(3, 400, vec![5]);    // Consciousness → Property
        self.organizer.add(4, 500, vec![5]);    // Temporal → Property
        self.organizer.add(5, 600, vec![6]);    // Property → Feedback
        self.organizer.add(6, 700, vec![7]);    // Feedback → Presence
        self.organizer.add(7, 800, vec![]);     // Presence (终点)
        self.organizer.build();
    }

    /// 推进一个心跳 tick
    pub fn tick(&mut self) -> Vec<TickEvent> {
        self.current_tick += 1;
        let mut tick_events = Vec::new();

        // 每个心跳推进预加载流 (装载所有流的状态)
        if self.current_tick == 1 {
            self.build_flow_graph();
        }

        // 按优先级推进各流
        for flow in FlowId::all() {
            let status = self.flow_status.get(&flow).copied().unwrap_or(FlowStatus::NotReady);
            match status {
                FlowStatus::Completed | FlowStatus::Blocked => continue,
                _ => {
                    let ev = self.advance_flow(flow);
                    tick_events.extend(ev);
                }
            }
        }

        self.events.extend(tick_events.clone());
        tick_events
    }

    /// 推进单条流
    fn advance_flow(&mut self, flow: FlowId) -> Vec<TickEvent> {
        let mut events = Vec::new();
        let tick = self.current_tick;

        match flow {
            FlowId::Preload => {
                // 预加载: 确保所有签名已三元解读
                // 预加载瞬间完成
                self.flow_status.insert(flow, FlowStatus::Completed);
                events.push(TickEvent {
                    tick, flow, signature: 0,
                    action: TickAction::FlowAdvanced,
                });
            }
            FlowId::Resonance => {
                // 谐音流: 检查签名间的共振
                for &sig in &self.signatures {
                    for port in &self.interact.ports {
                        if port.active && is_resonant(sig, port.signature, 1) {
                            events.push(TickEvent {
                                tick, flow, signature: sig,
                                action: TickAction::ResonanceMatched(sig, port.signature),
                            });
                            break; // 一个匹配即可
                        }
                    }
                }
                if events.is_empty() {
                    events.push(TickEvent {
                        tick, flow, signature: 0,
                        action: TickAction::FlowBlocked("无共振端口".into()),
                    });
                }
                self.flow_status.insert(flow, FlowStatus::Completed);
            }
            FlowId::Sovereignty => {
                // 主权流: 验证容器极性/主权
                let mut ok = true;
                for &sig in &self.signatures {
                    let t = Trinity::decode(sig);
                    if !t.sovereignty_temporal_match() {
                        ok = false;
                        events.push(TickEvent {
                            tick, flow, signature: sig,
                            action: TickAction::SovereigntyRejected("时空主权不匹配".into()),
                        });
                    }
                }
                if ok {
                    for &sig in &self.signatures {
                        events.push(TickEvent {
                            tick, flow, signature: sig,
                            action: TickAction::SovereigntyOk,
                        });
                    }
                    self.flow_status.insert(flow, FlowStatus::Completed);
                } else {
                    self.flow_status.insert(flow, FlowStatus::Blocked);
                }
            }
            FlowId::Consciousness => {
                // 意识流: 观察者觉知
                for &sig in &self.signatures {
                    let ctx = EightContext::new(sig);
                    if ctx.is_coherent() {
                        events.push(TickEvent {
                            tick, flow, signature: sig,
                            action: TickAction::ConsciousnessAware,
                        });
                    }
                }
                self.flow_status.insert(flow, FlowStatus::Completed);
            }
            FlowId::Temporal => {
                // 时间流: 推进相位
                for &sig in &self.signatures {
                    let t = Trinity::decode(sig);
                    events.push(TickEvent {
                        tick, flow, signature: sig,
                        action: TickAction::TimeAdvanced(t.time.phase),
                    });
                }
                self.flow_status.insert(flow, FlowStatus::Completed);
            }
            FlowId::Property => {
                // 产权流: 需要上下游都完成
                let res_done = self.flow_status.get(&FlowId::Resonance)
                    == Some(&FlowStatus::Completed);
                let sov_done = self.flow_status.get(&FlowId::Sovereignty)
                    == Some(&FlowStatus::Completed);
                let temp_done = self.flow_status.get(&FlowId::Temporal)
                    == Some(&FlowStatus::Completed);

                if res_done && sov_done && temp_done {
                    self.interact.match_needs_offers();
                    events.push(TickEvent {
                        tick, flow, signature: 0,
                        action: TickAction::FlowAdvanced,
                    });
                    self.flow_status.insert(flow, FlowStatus::Completed);
                } else {
                    self.flow_status.insert(flow, FlowStatus::Blocked);
                }
            }
            FlowId::Feedback => {
                // 反馈流: 判断/评价
                let prop_done = self.flow_status.get(&FlowId::Property)
                    == Some(&FlowStatus::Completed);
                if prop_done {
                    for &sig in &self.signatures {
                        events.push(TickEvent {
                            tick, flow, signature: sig,
                            action: TickAction::FeedbackReceived("反馈环处理".into()),
                        });
                    }
                    self.flow_status.insert(flow, FlowStatus::Completed);
                } else {
                    self.flow_status.insert(flow, FlowStatus::Blocked);
                }
            }
            FlowId::Presence => {
                // 存在流: 在场检查 + 局部投现自杀 + 持久化
                let fb_done = self.flow_status.get(&FlowId::Feedback)
                    == Some(&FlowStatus::Completed);
                if fb_done {
                    for &sig in &self.signatures {
                        let t = Trinity::decode(sig);
                        if t.is_present() {
                            events.push(TickEvent {
                                tick, flow, signature: sig,
                                action: TickAction::Persisted,
                            });
                            self.processed += 1;
                        } else {
                            events.push(TickEvent {
                                tick, flow, signature: sig,
                                action: TickAction::Suicide,
                            });
                        }
                    }
                    self.flow_status.insert(flow, FlowStatus::Completed);
                } else {
                    self.flow_status.insert(flow, FlowStatus::Blocked);
                }
            }
        }

        events
    }

    /// 计算引擎中 flow_status 的值 (辅助函数)
    fn _flow_status(&self) -> usize {
        self.flow_status.values().filter(|&&s| s == FlowStatus::Completed).count()
    }

    /// 在所有 tick 上运行直到全部完成
    pub fn run_to_completion(&mut self, max_ticks: u64) -> EngineRunResult {
        for _ in 0..max_ticks {
            self.tick();
            if self.is_complete() {
                break;
            }
        }
        self.run_result()
    }

    /// 是否所有流都完成了
    pub fn is_complete(&self) -> bool {
        FlowId::all().iter().all(|f| {
            self.flow_status.get(f) == Some(&FlowStatus::Completed)
        })
    }

    /// 运行结果
    pub fn run_result(&self) -> EngineRunResult {
        EngineRunResult {
            ticks_used: self.current_tick,
            signatures_processed: self.processed,
            total_events: self.events.len(),
            complete: self.is_complete(),
            event_log: self.events.clone(),
        }
    }

    /// 统计
    pub fn stats(&self) -> EightStats {
        let completed = self.flow_status.values()
            .filter(|&&s| s == FlowStatus::Completed)
            .count();
        EightStats {
            signature_count: self.signatures.len(),
            port_count: self.interact.port_count(),
            tick: self.current_tick,
            flow_completed: completed,
            total_events: self.events.len(),
            processed: self.processed,
        }
    }
}

impl Default for EightUnifyEngine {
    fn default() -> Self {
        Self::new(InvocationStrategy::FullyResonant)
    }
}

// ────────────────────────────────────────────────────────────
// 运行结果
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EngineRunResult {
    pub ticks_used: u64,
    pub signatures_processed: usize,
    pub total_events: usize,
    pub complete: bool,
    pub event_log: Vec<TickEvent>,
}

// ────────────────────────────────────────────────────────────
// 统计
// ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct EightStats {
    pub signature_count: usize,
    pub port_count: usize,
    pub tick: u64,
    pub flow_completed: usize,
    pub total_events: usize,
    pub processed: usize,
}

// ────────────────────────────────────────────────────────────
// 便捷函数
// ────────────────────────────────────────────────────────────

/// 快速运行签名集合往返一次心跳
pub fn run_signatures(signatures: &[u64], max_ticks: u64) -> EngineRunResult {
    let mut engine = EightUnifyEngine::default();
    engine.input(signatures);
    engine.run_to_completion(max_ticks)
}

/// 带容器端口注册后运行
pub fn run_with_ports(
    signatures: &[u64],
    ports: Vec<ContainerPort>,
    max_ticks: u64,
) -> EngineRunResult {
    let mut engine = EightUnifyEngine::default();
    engine.input(signatures);
    for port in ports {
        engine.register_port(port);
    }
    engine.run_to_completion(max_ticks)
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_id_all() {
        assert_eq!(FlowId::all().len(), 8);
    }

    #[test]
    fn test_flow_priority_order() {
        let flows = FlowId::all();
        assert_eq!(flows[0], FlowId::Preload);
        assert_eq!(flows[7], FlowId::Presence);
    }

    #[test]
    fn test_eight_context_new() {
        let ctx = EightContext::new(0x1234);
        assert_eq!(ctx.signature, 0x1234);
        assert_eq!(ctx.visited_flows.len(), 0);
        assert!(ctx.visited_flows.is_empty());
    }

    #[test]
    fn test_context_transition() {
        let mut ctx = EightContext::new(0x100);
        ctx.transition_to(FlowId::Resonance);
        ctx.transition_to(FlowId::Sovereignty);
        assert!(ctx.visited(FlowId::Resonance));
        assert!(ctx.visited(FlowId::Sovereignty));
        assert!(!ctx.visited(FlowId::Presence));
    }

    #[test]
    fn test_context_is_alive() {
        // phase=1 → Growth → alive
        let ctx = EightContext::new(0x08);
        assert!(ctx.is_alive());
    }

    #[test]
    fn test_engine_new() {
        let engine = EightUnifyEngine::new(InvocationStrategy::MaxResonance);
        assert_eq!(engine.signatures.len(), 0);
        assert_eq!(engine.current_tick, 0);
    }

    #[test]
    fn test_engine_input_and_tick() {
        let mut engine = EightUnifyEngine::new(InvocationStrategy::MaxResonance);
        engine.input(&[0x10, 0x20]);

        let events = engine.tick();
        assert!(!events.is_empty());
        assert_eq!(engine.current_tick, 1);
    }

    #[test]
    fn test_run_to_completion() {
        let mut engine = EightUnifyEngine::new(InvocationStrategy::FullyResonant);
        engine.input(&[0x10, 0x30, 0x40]);
        let result = engine.run_to_completion(20);
        assert!(result.complete);
        assert!(result.ticks_used > 0);
    }

    #[test]
    fn test_run_signatures_convenience() {
        let result = run_signatures(&[0x10, 0x20, 0x30], 20);
        assert!(result.complete);
        assert!(result.signatures_processed > 0);
    }

    #[test]
    fn test_engine_stats() {
        let mut engine = EightUnifyEngine::default();
        engine.input(&[0x10, 0x20]);
        engine.tick();
        let stats = engine.stats();
        assert_eq!(stats.signature_count, 2);
        assert!(stats.tick >= 1);
    }

    #[test]
    fn test_flow_status_tracking() {
        let mut engine = EightUnifyEngine::default();
        engine.input(&[0x10]);

        // 全开始时 NotReady
        let not_ready_count = engine.flow_status.values()
            .filter(|&&s| s == FlowStatus::NotReady)
            .count();
        assert_eq!(not_ready_count, 8);

        engine.tick();
        // 推进后 Preload 和 Resonance 应该完成了
        let completed = engine.flow_status.values()
            .filter(|&&s| s == FlowStatus::Completed)
            .count();
        assert!(completed >= 2);
    }

    #[test]
    fn test_tick_event_describe() {
        let ev = TickEvent {
            tick: 1,
            flow: FlowId::Resonance,
            signature: 0x1234,
            action: TickAction::ResonanceMatched(0x10, 0x20),
        };
        let desc = ev.action.describe();
        assert!(desc.contains("共振"));
    }

    #[test]
    fn test_run_with_ports() {
        use crate::container_dynamics::Call;
        let ports = vec![
            ContainerPort::new(6, "A"),
            ContainerPort::new(9, "B"),
        ];
        let result = run_with_ports(&[6, 9], ports, 20);
        assert!(result.complete);
    }

    #[test]
    fn test_context_flow_type() {
        let ctx = EightContext::with_flow(0x10, ConsciFlowType::Judgment);
        assert_eq!(ctx.flow_type, ConsciFlowType::Judgment);
    }

    #[test]
    fn test_display_flow_id() {
        assert_eq!(format!("{}", FlowId::Preload), "预加载");
        assert_eq!(format!("{}", FlowId::Presence), "存在");
    }

    #[test]
    fn test_base_priority() {
        assert_eq!(FlowId::Preload.base_priority(), 0);
        assert_eq!(FlowId::Presence.base_priority(), 7);
        assert!(FlowId::Resonance.base_priority() < FlowId::Consciousness.base_priority());
    }
}
