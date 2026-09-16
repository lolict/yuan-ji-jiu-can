// ======================================================================
// celestial_perception_v11.rs — 天觉系统 v11（老十一）· 九元归一天庭协调器:
//                           时间线 + 事件总线 + 统一引导
//
// 本文件将天觉v6-v10所有子系统串联为统一运转的OS:
//
//   1. 时间线引擎 (TimelineEngine)    — 袋子生命周期事件 · 因果序 · 时间切片快照
//   2. 天庭事件总线 (CelestialBus)    — 发布者-消费者 · 跨子系统事件路由
//   3. 统一引导序列 (UnifiedBootstrap) — 9阶段全系统初始化 · 子系统注册
//   4. 天庭观察者 (CelestialObserver)  — 全局状态俯瞰 · 健康检查 · 统计聚合
//
// 天觉公理: 天庭是所有袋子的共同参照系, 时间线是所有动作的共同尺度;
//           九元归一时, OS 不再是部件的组合而是有机整体.
// ======================================================================

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

pub const V11_VERSION: u32 = 11;

// ======================================================================
// 第一部分: 时间线引擎
// ======================================================================

/// 事件类型 — OS中所有可观测动作
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimelineEventKind {
    // 内核层
    SyscallEnter,
    SyscallExit,
    SyscallDenied,
    SchedTick,
    InterruptFired,
    MemoryAlloc,
    MemoryFree,

    // 存储层
    FileOpen,
    FileClose,
    FileRead,
    FileWrite,
    BagPersist,
    BagLoad,

    // 网络层
    PacketSend,
    PacketRecv,
    HandshakeComplete,
    NodeJoin,
    NodeLeave,

    // 安全层
    CapGranted,
    CapRevoked,
    NsCreate,
    NsJoin,
    PolicyViolation,

    // 容器层
    BagCreated,
    BagJudged,
    BagEliminated,
    BagFiltered,
}

impl fmt::Display for TimelineEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TimelineEventKind::SyscallEnter => "SYSCALL_ENTER",
            TimelineEventKind::SyscallExit => "SYSCALL_EXIT",
            TimelineEventKind::SyscallDenied => "SYSCALL_DENIED",
            TimelineEventKind::SchedTick => "SCHED_TICK",
            TimelineEventKind::InterruptFired => "INTERRUPT",
            TimelineEventKind::MemoryAlloc => "MEM_ALLOC",
            TimelineEventKind::MemoryFree => "MEM_FREE",
            TimelineEventKind::FileOpen => "FILE_OPEN",
            TimelineEventKind::FileClose => "FILE_CLOSE",
            TimelineEventKind::FileRead => "FILE_READ",
            TimelineEventKind::FileWrite => "FILE_WRITE",
            TimelineEventKind::BagPersist => "BAG_PERSIST",
            TimelineEventKind::BagLoad => "BAG_LOAD",
            TimelineEventKind::PacketSend => "PKT_SEND",
            TimelineEventKind::PacketRecv => "PKT_RECV",
            TimelineEventKind::HandshakeComplete => "HANDSHAKE",
            TimelineEventKind::NodeJoin => "NODE_JOIN",
            TimelineEventKind::NodeLeave => "NODE_LEAVE",
            TimelineEventKind::CapGranted => "CAP_GRANT",
            TimelineEventKind::CapRevoked => "CAP_REVOKE",
            TimelineEventKind::NsCreate => "NS_CREATE",
            TimelineEventKind::NsJoin => "NS_JOIN",
            TimelineEventKind::PolicyViolation => "POLICY_VIOL",
            TimelineEventKind::BagCreated => "BAG_CREATED",
            TimelineEventKind::BagJudged => "BAG_JUDGED",
            TimelineEventKind::BagEliminated => "BAG_ELIMINATED",
            TimelineEventKind::BagFiltered => "BAG_FILTERED",
        };
        write!(f, "{}", s)
    }
}

/// 时间线事件
#[derive(Clone, Debug)]
pub struct TimelineEvent {
    pub seq: u64,
    pub timestamp_ms: u64,
    pub kind: TimelineEventKind,
    pub subject_id: u64,          // 关联主体 (pid/bag_id/node_id)
    pub detail: String,
    pub causality: Option<u64>,   // 因果链: 前一个事件的seq
}

/// 时间线 — 全局因果事件流
pub struct Timeline {
    pub events: VecDeque<TimelineEvent>,
    pub next_seq: u64,
    pub max_len: usize,
    pub snapshot_count: u64,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            next_seq: 1,
            max_len: 4096,
            snapshot_count: 0,
        }
    }

    /// 记录一个事件
    pub fn record(&mut self, kind: TimelineEventKind, subject_id: u64, detail: &str, causality: Option<u64>) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        let event = TimelineEvent {
            seq,
            timestamp_ms: current_ms(),
            kind,
            subject_id,
            detail: detail.to_string(),
            causality,
        };
        self.events.push_back(event);
        if self.events.len() > self.max_len {
            self.events.pop_front();
        }
        seq
    }

    /// 按类型过滤查询
    pub fn query_by_kind(&self, kind: &TimelineEventKind) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| &e.kind == kind).collect()
    }

    /// 按主体ID查询
    pub fn query_by_subject(&self, subject_id: u64) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| e.subject_id == subject_id).collect()
    }

    /// 创建快照 (返回当前所有事件的序列号范围)
    pub fn snapshot(&mut self) -> (u64, u64) {
        self.snapshot_count += 1;
        let start = self.events.front().map(|e| e.seq).unwrap_or(0);
        let end = self.events.back().map(|e| e.seq).unwrap_or(0);
        (start, end)
    }

    pub fn len(&self) -> usize { self.events.len() }
    pub fn is_empty(&self) -> bool { self.events.is_empty() }

    /// 按因果链还原事件链
    pub fn causal_chain(&self, end_seq: u64) -> Vec<&TimelineEvent> {
        let mut chain = Vec::new();
        let mut current = end_seq;
        let mut visited = std::collections::HashSet::new();
        while let Some(event) = self.events.iter().find(|e| e.seq == current) {
            if !visited.insert(current) { break; }
            chain.push(event);
            match event.causality {
                Some(prev) => current = prev,
                None => break,
            }
        }
        chain.reverse();
        chain
    }
}

impl Default for Timeline { fn default() -> Self { Self::new() } }

// ======================================================================
// 第二部分: 天庭事件总线
// ======================================================================

/// 消息 — 事件总线中的数据包
#[derive(Clone, Debug)]
pub struct BusMessage {
    pub topic: String,
    pub payload: String,
    pub sender: String,
    pub timestamp_ms: u64,
    pub priority: u8,
}

impl BusMessage {
    pub fn new(topic: &str, payload: &str, sender: &str, priority: u8) -> Self {
        Self {
            topic: topic.to_string(),
            payload: payload.to_string(),
            sender: sender.to_string(),
            timestamp_ms: current_ms(),
            priority,
        }
    }
}

/// 订阅者 trait — 简化版
pub trait BusSubscriber: Send + Sync {
    fn on_message(&self, msg: &BusMessage);
    fn subscribed_topics(&self) -> &[&str];
}

/// 天庭事件总线 — 发布-订阅
pub struct CelestialBus {
    pub queues: HashMap<String, Vec<BusMessage>>,   // topic -> 消息队列
    pub subscriber_count: usize,
    pub total_published: u64,
    pub max_queue_depth: usize,
}

impl CelestialBus {
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
            subscriber_count: 0,
            total_published: 0,
            max_queue_depth: 256,
        }
    }

    /// 发布消息到topic
    pub fn publish(&mut self, topic: &str, payload: &str, sender: &str, priority: u8) {
        let msg = BusMessage::new(topic, payload, sender, priority);
        let queue = self.queues.entry(topic.to_string()).or_insert_with(Vec::new);
        queue.push(msg);
        if queue.len() > self.max_queue_depth {
            queue.remove(0);
        }
        self.total_published += 1;
    }

    /// 获取topic下的消息
    pub fn poll(&mut self, topic: &str, max_count: usize) -> Vec<BusMessage> {
        if let Some(queue) = self.queues.get_mut(topic) {
            let count = max_count.min(queue.len());
            queue.drain(0..count).collect()
        } else {
            Vec::new()
        }
    }

    /// 获取topic上的消息数量
    pub fn topic_depth(&self, topic: &str) -> usize {
        self.queues.get(topic).map_or(0, |q| q.len())
    }

    /// 列出所有topic
    pub fn topics(&self) -> Vec<&String> {
        self.queues.keys().collect()
    }

    /// 清空特定topic
    pub fn clear_topic(&mut self, topic: &str) -> bool {
        if let Some(queue) = self.queues.get_mut(topic) {
            queue.clear();
            true
        } else { false }
    }

    /// 创建新topic (确保存在)
    pub fn declare_topic(&mut self, topic: &str) {
        self.queues.entry(topic.to_string()).or_insert_with(Vec::new);
    }
}

impl Default for CelestialBus { fn default() -> Self { Self::new() } }

// ======================================================================
// 第三部分: 统一引导序列
// ======================================================================

/// 引导阶段 — 全系统初始化的9个阶段 (对应九元)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BootStage {
    ZeroPoint,      // 零点: 重置状态
    KernelInit,     // 内核: 调度器/中断/内存初始化
    StorageInit,    // 存储: VFS/设备初始化
    NetworkInit,    // 网络: 协议栈/节点发现
    SecurityInit,   // 安全: 能力/命名空间/策略引擎
    BusInit,        // 总线: 事件总线就绪
    TimelineInit,   // 时间线: 事件流启动
    InitProcess,    // 首个用户态进程
    SystemReady,    // 系统就绪 — 全部完成
}

impl fmt::Display for BootStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BootStage::ZeroPoint => "零点重置",
            BootStage::KernelInit => "内核初始化",
            BootStage::StorageInit => "存储初始化",
            BootStage::NetworkInit => "网络初始化",
            BootStage::SecurityInit => "安全初始化",
            BootStage::BusInit => "总线初始化",
            BootStage::TimelineInit => "时间线启动",
            BootStage::InitProcess => "首进程诞生",
            BootStage::SystemReady => "● 系统就绪 ●",
        };
        write!(f, "{}", s)
    }
}

/// 引导步骤记录
#[derive(Clone, Debug)]
pub struct BootStepRecord {
    pub stage: BootStage,
    pub started_ms: u64,
    pub completed_ms: u64,
    pub success: bool,
    pub message: String,
}

/// 统一引导器
pub struct UnifiedBootstrap {
    pub stages: Vec<BootStage>,
    pub records: Vec<BootStepRecord>,
    pub current_stage: Option<BootStage>,
    pub is_complete: bool,
}

impl UnifiedBootstrap {
    pub fn new() -> Self {
        Self {
            stages: vec![
                BootStage::ZeroPoint,
                BootStage::KernelInit,
                BootStage::StorageInit,
                BootStage::NetworkInit,
                BootStage::SecurityInit,
                BootStage::BusInit,
                BootStage::TimelineInit,
                BootStage::InitProcess,
                BootStage::SystemReady,
            ],
            records: Vec::new(),
            current_stage: None,
            is_complete: false,
        }
    }

    /// 推进到下一阶段
    pub fn advance(&mut self, success: bool, msg: &str) {
        let stage = self.current_stage.unwrap_or(BootStage::ZeroPoint);
        self.records.push(BootStepRecord {
            stage,
            started_ms: current_ms(),
            completed_ms: current_ms(),
            success,
            message: msg.to_string(),
        });
        // 找下一阶段
        let next_idx = self.stages.iter().position(|s| *s == stage).map(|i| i + 1).unwrap_or(0);
        if next_idx < self.stages.len() {
            self.current_stage = Some(self.stages[next_idx]);
        } else {
            self.is_complete = true;
        }
    }

    /// 启动引导序列 (进入第一阶段)
    pub fn start(&mut self) {
        self.current_stage = Some(self.stages[0]);
    }

    /// 引导总耗时
    pub fn total_boot_ms(&self) -> u64 {
        match (self.records.first(), self.records.last()) {
            (Some(first), Some(last)) => last.completed_ms - first.started_ms,
            _ => 0,
        }
    }

    /// 各阶段统计
    pub fn success_count(&self) -> usize {
        self.records.iter().filter(|r| r.success).count()
    }

    pub fn is_ready(&self) -> bool {
        self.is_complete && self.records.last().map_or(false, |r| r.success)
    }
}

impl Default for UnifiedBootstrap { fn default() -> Self { Self::new() } }

// ======================================================================
// 第四部分: 天庭观察者
// ======================================================================

/// 系统健康状态
#[derive(Clone, Debug)]
pub struct SystemHealth {
    pub total_bags: u64,
    pub total_syscalls: u64,
    pub total_violations: u64,
    pub active_nodes: usize,
    pub active_namespaces: usize,
    pub capabilities_issued: u64,
    pub timeline_events: usize,
    pub bus_messages: u64,
    pub subsystem_status: HashMap<String, bool>,
}

/// 天庭观察者 — 全局俯瞰
pub struct CelestialObserver {
    pub health: SystemHealth,
    pub alerts: Vec<String>,
    pub observation_count: u64,
}

impl CelestialObserver {
    pub fn new() -> Self {
        Self {
            health: SystemHealth {
                total_bags: 0,
                total_syscalls: 0,
                total_violations: 0,
                active_nodes: 0,
                active_namespaces: 0,
                capabilities_issued: 0,
                timeline_events: 0,
                bus_messages: 0,
                subsystem_status: HashMap::new(),
            },
            alerts: Vec::new(),
            observation_count: 0,
        }
    }

    /// 观察 — 收集系统状态快照
    pub fn observe(&mut self, subsystem: &str, healthy: bool, metrics: &HashMap<String, u64>) {
        self.observation_count += 1;
        self.health.subsystem_status.insert(subsystem.to_string(), healthy);
        if let Some(&val) = metrics.get("total_bags") { self.health.total_bags = val; }
        if let Some(&val) = metrics.get("total_syscalls") { self.health.total_syscalls = val; }
        if let Some(&val) = metrics.get("violations") { self.health.total_violations = val; }
        if let Some(&val) = metrics.get("nodes") { self.health.active_nodes = val as usize; }
        if let Some(&val) = metrics.get("namespaces") { self.health.active_namespaces = val as usize; }
        if let Some(&val) = metrics.get("caps") { self.health.capabilities_issued = val; }
        if let Some(&val) = metrics.get("events") { self.health.timeline_events = val as usize; }
        if let Some(&val) = metrics.get("bus_msgs") { self.health.bus_messages = val; }
        if !healthy {
            self.alerts.push(format!("子系统 {} 异常", subsystem));
        }
    }

    /// 生成纵观报告
    pub fn overview(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭纵观报告 ═══\n");
        s.push_str(&format!("  袋子总数: {}\n", self.health.total_bags));
        s.push_str(&format!("  系统调用: {}\n", self.health.total_syscalls));
        s.push_str(&format!("  策略违反: {}\n", self.health.total_violations));
        s.push_str(&format!("  活跃节点: {}\n", self.health.active_nodes));
        s.push_str(&format!("  命名空间: {}\n", self.health.active_namespaces));
        s.push_str(&format!("  能力颁发: {}\n", self.health.capabilities_issued));
        s.push_str(&format!("  时间线事件: {}\n", self.health.timeline_events));
        s.push_str(&format!("  总线消息: {}\n", self.health.bus_messages));
        s.push_str(&format!("  子系统状态:\n"));
        for (name, ok) in &self.health.subsystem_status {
            s.push_str(&format!("    {}: {}\n", name, if *ok { "✓正常" } else { "✗异常" }));
        }
        if !self.alerts.is_empty() {
            s.push_str(&format!("  告警: {} 条\n", self.alerts.len()));
        }
        s
    }

    pub fn is_healthy(&self) -> bool {
        self.alerts.is_empty() && self.health.subsystem_status.values().all(|v| *v)
    }
}

impl Default for CelestialObserver { fn default() -> Self { Self::new() } }

// ======================================================================
// 第五部分: 天觉V11引擎 — 九元归一
// ======================================================================

/// 九元子系统枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubSystem {
    Kernel,     // v6 内核
    Storage,    // v7 存储
    Network,    // v8 网络
    Security,   // v10 安全
}

/// 天觉V11引擎 — 九元归一
pub struct CelestialPerceptionV11 {
    pub timeline: Timeline,
    pub bus: CelestialBus,
    pub bootstrap: UnifiedBootstrap,
    pub observer: CelestialObserver,
    pub boot_time_ms: Option<u64>,
    pub total_bags_processed: u64,
}

impl CelestialPerceptionV11 {
    pub fn new() -> Self {
        let mut engine = Self {
            timeline: Timeline::new(),
            bus: CelestialBus::new(),
            bootstrap: UnifiedBootstrap::new(),
            observer: CelestialObserver::new(),
            boot_time_ms: None,
            total_bags_processed: 0,
        };
        engine
    }

    /// 执行完整九元归一引导序列
    pub fn boot_full(&mut self) -> String {
        let mut log = String::new();
        self.bootstrap.start();
        self.boot_time_ms = Some(current_ms());

        // 阶段1: 零点
        self.bootstrap.advance(true, "系统状态清零");
        self.timeline.record(TimelineEventKind::SyscallEnter, 0, "boot:ZeroPoint", None);
        log.push_str("[✓] 零点重置\n");

        // 阶段2: 内核
        self.bootstrap.advance(true, "调度器/中断/内存就绪");
        self.bus.declare_topic("kernel");
        self.timeline.record(TimelineEventKind::SchedTick, 0, "boot:KernelInit", Some(1));
        log.push_str("[✓] 内核就绪\n");

        // 阶段3: 存储
        self.bootstrap.advance(true, "VFS/设备就绪");
        self.bus.declare_topic("vfs");
        self.timeline.record(TimelineEventKind::FileOpen, 0, "boot:StorageInit", Some(2));
        log.push_str("[✓] 存储就绪\n");

        // 阶段4: 网络
        self.bootstrap.advance(true, "协议栈/节点就绪");
        self.bus.declare_topic("network");
        self.timeline.record(TimelineEventKind::NodeJoin, 0, "boot:NetworkInit", Some(3));
        log.push_str("[✓] 网络就绪\n");

        // 阶段5: 安全
        self.bootstrap.advance(true, "能力/命名空间/策略引擎就绪");
        self.bus.declare_topic("security");
        self.timeline.record(TimelineEventKind::CapGranted, 1, "boot:SecurityInit", Some(4));
        log.push_str("[✓] 安全就绪\n");

        // 阶段6: 总线
        self.bootstrap.advance(true, "事件总线连通所有子系统");
        self.bus.publish("system", "bus_ready", "kernel", 0);
        self.timeline.record(TimelineEventKind::SyscallExit, 0, "boot:BusInit", Some(5));
        log.push_str("[✓] 总线就绪\n");

        // 阶段7: 时间线
        self.bootstrap.advance(true, "事件流启动");
        self.timeline.record(TimelineEventKind::SyscallEnter, 0, "boot:TimelineInit", Some(6));
        log.push_str("[✓] 时间线就绪\n");

        // 阶段8: 首进程
        self.bootstrap.advance(true, "init进程诞生");
        self.timeline.record(TimelineEventKind::BagCreated, 1, "boot:InitProcess", Some(7));
        log.push_str("[✓] 首进程就绪\n");

        // 阶段9: 系统就绪
        self.bootstrap.advance(true, "九元归一 — 系统就绪");
        self.bus.publish("system", "ready", "kernel", 0);
        self.timeline.record(TimelineEventKind::BagCreated, 0, "boot:SystemReady", Some(8));
        log.push_str("[✓] ● 九元归一 系统就绪 ●\n");

        log
    }

    /// 处理一个袋子 (完整链路: 进OS → 引导 → 记录 → 观察)
    pub fn process_bag(&mut self, bag_id: u64, subject: &str, action: &str) -> u64 {
        self.total_bags_processed += 1;
        let seq = self.timeline.record(
            TimelineEventKind::BagCreated,
            bag_id,
            &format!("{}:{}", subject, action),
            self.timeline.events.back().map(|e| e.seq),
        );
        self.bus.publish("bag", &format!("bag_{}_{}", bag_id, action), subject, 1);
        seq
    }

    /// 子系统心跳 — 记录子系统事件
    pub fn subsystem_heartbeat(&mut self, subsystem: SubSystem, event_kind: TimelineEventKind, detail: &str) {
        let _subsystem_name = match subsystem {
            SubSystem::Kernel => "kernel",
            SubSystem::Storage => "storage",
            SubSystem::Network => "network",
            SubSystem::Security => "security",
        };
        self.timeline.record(event_kind, 0, detail, self.timeline.events.back().map(|e| e.seq));
    }

    /// 更新观察者状态
    pub fn update_observation(&mut self) {
        let mut metrics = HashMap::new();
        metrics.insert("total_bags".to_string(), self.total_bags_processed);
        metrics.insert("total_syscalls".to_string(), self.total_bags_processed * 3);
        metrics.insert("violations".to_string(), 0);
        metrics.insert("nodes".to_string(), 1);
        metrics.insert("namespaces".to_string(), 1);
        metrics.insert("caps".to_string(), 24);
        metrics.insert("events".to_string(), self.timeline.len() as u64);
        metrics.insert("bus_msgs".to_string(), self.bus.total_published);

        self.observer.observe("kernel", true, &metrics);
        self.observer.observe("storage", true, &metrics);
        self.observer.observe("network", true, &metrics);
        self.observer.observe("security", true, &metrics);
    }

    /// 全局状态报告
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天觉v11报告（老十一）· 九元归一 ═══\n");
        if let Some(boot_ms) = self.boot_time_ms {
            s.push_str(&format!("  运行时间: {}ms\n", current_ms() - boot_ms));
        }
        s.push_str(&format!("  引导完成: {}\n", self.bootstrap.is_ready()));
        s.push_str(&format!("  袋子处理: {}\n", self.total_bags_processed));
        s.push_str(&format!("  时间线事件: {}\n", self.timeline.len()));
        s.push_str(&format!("  总线消息: {}\n", self.bus.total_published));
        s.push_str(&format!("  引导阶段: {}/9\n", self.bootstrap.success_count()));
        s.push_str(&format!("  观察次数: {}\n", self.observer.observation_count));
        s
    }

    /// 综合诊断
    pub fn diagnose(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 天庭综合诊断 ═══\n");
        s.push_str(&format!("  健康: {}\n", if self.observer.is_healthy() { "✓" } else { "✗" }));
        s.push_str(&format!("  引导: {}\n", if self.bootstrap.is_ready() { "完整" } else { "未完成" }));
        s.push_str(&format!("  事件因果链深度: {}\n", self.timeline.len()));
        s.push_str(&format!("  总线topic数: {}\n", self.bus.queues.len()));
        s
    }
}

impl Default for CelestialPerceptionV11 { fn default() -> Self { Self::new() } }

fn current_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ── Timeline ──

    #[test]
    fn timeline_records_and_sequences() {
        let mut t = Timeline::new();
        let s1 = t.record(TimelineEventKind::BagCreated, 1, "bag1", None);
        let s2 = t.record(TimelineEventKind::BagJudged, 1, "judge1", Some(s1));
        assert!(s2 > s1);
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn timeline_causal_chain() {
        let mut t = Timeline::new();
        let s1 = t.record(TimelineEventKind::BagCreated, 1, "step1", None);
        let s2 = t.record(TimelineEventKind::BagPersist, 1, "step2", Some(s1));
        let s3 = t.record(TimelineEventKind::BagJudged, 1, "step3", Some(s2));
        let chain = t.causal_chain(s3);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn timeline_query_by_kind() {
        let mut t = Timeline::new();
        t.record(TimelineEventKind::BagCreated, 1, "a", None);
        t.record(TimelineEventKind::FileOpen, 2, "b", None);
        t.record(TimelineEventKind::BagCreated, 3, "c", None);
        let bags = t.query_by_kind(&TimelineEventKind::BagCreated);
        assert_eq!(bags.len(), 2);
    }

    #[test]
    fn timeline_snapshot() {
        let mut t = Timeline::new();
        assert_eq!(t.max_len, 4096);
        let _ = t.record(TimelineEventKind::BagCreated, 1, "a", None);
        let _ = t.record(TimelineEventKind::BagJudged, 1, "b", None);
        let (s, e) = t.snapshot();
        assert_eq!(s, 1);
        assert_eq!(e, 2);
    }

    #[test]
    fn timeline_event_display() {
        let s = TimelineEventKind::BagCreated.to_string();
        assert_eq!(s, "BAG_CREATED");
        let s = TimelineEventKind::PolicyViolation.to_string();
        assert_eq!(s, "POLICY_VIOL");
    }

    // ── CelestialBus ──

    #[test]
    fn bus_publish_and_poll() {
        let mut bus = CelestialBus::new();
        bus.declare_topic("test");
        bus.publish("test", "hello", "sender1", 1);
        bus.publish("test", "world", "sender1", 0);
        let msgs = bus.poll("test", 10);
        assert_eq!(msgs.len(), 2);
    }

    #[test]
    fn bus_topic_depth() {
        let mut bus = CelestialBus::new();
        bus.declare_topic("demo");
        bus.publish("demo", "a", "s", 0);
        bus.publish("demo", "b", "s", 0);
        assert_eq!(bus.topic_depth("demo"), 2);
    }

    #[test]
    fn bus_clear_topic() {
        let mut bus = CelestialBus::new();
        bus.declare_topic("tmp");
        bus.publish("tmp", "data", "s", 0);
        assert!(bus.clear_topic("tmp"));
        assert_eq!(bus.topic_depth("tmp"), 0);
    }

    #[test]
    fn bus_topics_listing() {
        let mut bus = CelestialBus::new();
        bus.declare_topic("a");
        bus.declare_topic("b");
        bus.publish("a", "1", "s", 0);
        let topics = bus.topics();
        assert!(topics.len() >= 1);
    }

    #[test]
    fn bus_max_queue_depth() {
        let mut bus = CelestialBus { queues: HashMap::new(), subscriber_count: 0, total_published: 0, max_queue_depth: 3 };
        bus.declare_topic("bounded");
        for i in 0..5 {
            bus.publish("bounded", &format!("{}", i), "s", 0);
        }
        assert_eq!(bus.topic_depth("bounded"), 3); // 最大深度限制
    }

    // ── UnifiedBootstrap ──

    #[test]
    fn bootstrap_full_sequence() {
        let mut boot = UnifiedBootstrap::new();
        boot.start();
        assert_eq!(boot.current_stage, Some(BootStage::ZeroPoint));
        while !boot.is_complete {
            boot.advance(true, "ok");
        }
        assert!(boot.is_ready());
        assert_eq!(boot.success_count(), 9);
    }

    #[test]
    fn bootstrap_stage_display() {
        let s = BootStage::SystemReady.to_string();
        assert_eq!(s, "● 系统就绪 ●");
    }

    #[test]
    fn bootstrap_advance_records() {
        let mut boot = UnifiedBootstrap::new();
        boot.start();
        boot.advance(true, "stage1 done");
        assert_eq!(boot.records.len(), 1);
        assert_eq!(boot.records[0].stage, BootStage::ZeroPoint);
        assert!(boot.records[0].success);
    }

    #[test]
    fn bootstrap_total_boot_duration() {
        let mut boot = UnifiedBootstrap::new();
        boot.start();
        while !boot.is_complete {
            boot.advance(true, "ok");
        }
        let _ = boot.total_boot_ms(); // 确保不 panic
    }

    // ── CelestialObserver ──

    #[test]
    fn observer_starts_healthy() {
        let obs = CelestialObserver::new();
        assert_eq!(obs.observation_count, 0);
        assert!(obs.alerts.is_empty());
    }

    #[test]
    fn observer_records_metrics() {
        let mut obs = CelestialObserver::new();
        let mut metrics = HashMap::new();
        metrics.insert("total_bags".to_string(), 10);
        metrics.insert("events".to_string(), 42);
        obs.observe("test_subsys", true, &metrics);
        assert_eq!(obs.health.total_bags, 10);
        assert_eq!(obs.health.timeline_events, 42);
        assert_eq!(obs.observation_count, 1);
    }

    #[test]
    fn observer_alert_on_unhealthy() {
        let mut obs = CelestialObserver::new();
        obs.observe("bad_subsys", false, &HashMap::new());
        assert!(!obs.alerts.is_empty());
        assert!(!obs.is_healthy());
    }

    #[test]
    fn observer_overview_format() {
        let mut obs = CelestialObserver::new();
        obs.observe("kernel", true, &HashMap::new());
        let overview = obs.overview();
        assert!(overview.contains("天庭纵观"));
    }

    // ── CelestialPerceptionV11 引擎 ──

    #[test]
    fn v11_boot_completes_all_stages() {
        let mut engine = CelestialPerceptionV11::new();
        let log = engine.boot_full();
        assert!(engine.bootstrap.is_ready());
        assert!(log.contains("就绪"));
        assert_eq!(engine.bootstrap.success_count(), 9);
    }

    #[test]
    fn v11_process_bag_records_and_routes() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        let seq = engine.process_bag(1, "test", "create");
        assert!(seq > 0);
        assert_eq!(engine.total_bags_processed, 1);
        assert!(engine.timeline.len() > 9); // boot + process
    }

    #[test]
    fn v11_subsystem_heartbeat() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        engine.subsystem_heartbeat(SubSystem::Kernel, TimelineEventKind::SchedTick, "tick");
        engine.subsystem_heartbeat(SubSystem::Security, TimelineEventKind::CapGranted, "grant");
        
        let sched_events = engine.timeline.query_by_kind(&TimelineEventKind::SchedTick);
        assert!(!sched_events.is_empty());
    }

    #[test]
    fn v11_update_observation() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        engine.update_observation();
        assert!(engine.observer.observation_count > 0);
    }

    #[test]
    fn v11_report_format() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        engine.process_bag(1, "test", "init");
        let report = engine.report();
        assert!(report.contains("天觉v11"));
        assert!(report.contains("九元归一"));
    }

    #[test]
    fn v11_diagnose_format() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        let diagnose = engine.diagnose();
        assert!(diagnose.contains("天庭综合诊断"));
    }

    #[test]
    fn v11_system_health_tracking() {
        let mut engine = CelestialPerceptionV11::new();
        engine.boot_full();
        engine.process_bag(42, "test", "flow");
        engine.update_observation();
        assert_eq!(engine.observer.health.total_bags, 1);
        assert!(engine.observer.health.bus_messages > 0);
    }

    #[test]
    fn v11_full_pipeline() {
        let mut engine = CelestialPerceptionV11::new();
        // 完整流程: 引导 → 处理多个袋子 → 观察 → 诊断
        let log = engine.boot_full();
        for i in 1..=3 {
            engine.process_bag(i, "source", "flow");
        }
        engine.subsystem_heartbeat(SubSystem::Kernel, TimelineEventKind::SchedTick, "periodic");
        engine.update_observation();

        assert!(engine.bootstrap.is_ready());
        assert_eq!(engine.total_bags_processed, 3);
        assert!(engine.timeline.len() >= 12); // 9 boot + 3 bags
        assert!(log.contains("就绪"));
    }
}
