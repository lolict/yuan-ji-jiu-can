// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.79 流代数引擎                                                ║
// ║  分别器 · 预加载流 · 谐音路由 · 主权路由 · 流属性位               ║
// ║                                                                  ║
// ║  核心: 流 = 信号的纤维丛截面, 代数 = 流的变形/合并/分裂           ║
// ║  · 分别器: 区分流类型并分派到不同处理路径                         ║
// ║  · 谐音路由: 同音异字 → 统一处理                                 ║
// ║  · 主权路由: 根据产权标记决定去向                                 ║
// ║  · 流属性位: 流的状态标记 (在场/叠加/阴/阳)                       ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};
use crate::sky_space::StreamKind;

/// 流属性位 — 流的状态标记
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StreamFlags(u8);

impl StreamFlags {
    /// 在场 (符号已落地于文件系统)
    pub const PRESENT: u8 = 0b0001;
    /// 叠加态 (未观测前多种可能共存)
    pub const SUPERPOSITION: u8 = 0b0010;
    /// 阴 (被动/静态/接收)
    pub const YIN: u8 = 0b0100;
    /// 阳 (主动/动态/发射)
    pub const YANG: u8 = 0b1000;
    /// 全部标记
    pub const ALL: u8 = 0b1111;

    pub fn new() -> Self {
        StreamFlags(0)
    }

    pub fn with(flags: u8) -> Self {
        StreamFlags(flags & Self::ALL)
    }

    pub fn has(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    pub fn set(&mut self, flag: u8) {
        self.0 |= flag & Self::ALL;
    }

    pub fn clear(&mut self, flag: u8) {
        self.0 &= !(flag & Self::ALL);
    }

    pub fn is_present(&self) -> bool {
        self.has(Self::PRESENT)
    }

    pub fn is_superposition(&self) -> bool {
        self.has(Self::SUPERPOSITION)
    }

    pub fn is_yin(&self) -> bool {
        self.has(Self::YIN)
    }

    pub fn is_yang(&self) -> bool {
        self.has(Self::YANG)
    }

    /// 极性字符串表示
    pub fn polarity_label(&self) -> &'static str {
        match (self.is_yin(), self.is_yang()) {
            (true, false) => "阴",
            (false, true) => "阳",
            (true, true) => "叠加",
            (false, false) => "中性",
        }
    }

    pub fn raw(&self) -> u8 {
        self.0
    }
}

impl Default for StreamFlags {
    fn default() -> Self {
        StreamFlags::new()
    }
}

/// 流事件 — 流过分别器的信号
#[derive(Clone, Debug)]
pub struct FlowEvent {
    /// 来源容器
    pub source: ContainerRef,
    /// 目标容器
    pub target: ContainerRef,
    /// 携带的数据 (容器编码)
    pub payload: u64,
    /// 流类型
    pub stream_kind: StreamKind,
    /// 属性标记
    pub flags: StreamFlags,
    /// 时间拍
    pub tick: u64,
    /// 权重 (经历此事件的次数)
    pub weight: u64,
}

impl FlowEvent {
    pub fn new(source: ContainerRef, target: ContainerRef, payload: u64, stream_kind: StreamKind, tick: u64) -> Self {
        FlowEvent {
            source,
            target,
            payload,
            stream_kind,
            flags: StreamFlags::new(),
            tick,
            weight: 1,
        }
    }

    pub fn with_flags(mut self, flags: StreamFlags) -> Self {
        self.flags = flags;
        self
    }
}

/// 路由决定
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteDecision {
    /// 接受 (流进入此节点)
    Accept,
    /// 拒绝 (流无法通过)
    Reject,
    /// 转发到另一容器
    Forward(ContainerRef),
    /// 分裂 (流被复制到多条路径)
    Split(Vec<ContainerRef>),
    /// 合并 (等待配对流到达)
    Hold,
}

/// 分别器 — 区分流类型并分派
#[derive(Clone, Debug)]
pub struct Separator {
    /// 分别器名称
    pub name: String,
    /// 分派规则: 流类型 → 转向目标
    pub dispatches: HashMap<StreamKind, Vec<ContainerRef>>,
    /// 被拒流计数
    pub rejected: u64,
    /// 通过流计数
    pub passed: u64,
    /// 分裂计数
    pub splits: u64,
}

impl Separator {
    pub fn new(name: &str) -> Self {
        Separator {
            name: name.to_string(),
            dispatches: HashMap::new(),
            rejected: 0,
            passed: 0,
            splits: 0,
        }
    }

    /// 注册分派规则
    pub fn on(&mut self, kind: StreamKind, targets: Vec<ContainerRef>) {
        self.dispatches.insert(kind, targets);
    }

    /// 分流: 决定流的去向
    pub fn separate(&mut self, event: &FlowEvent) -> RouteDecision {
        match self.dispatches.get(&event.stream_kind) {
            Some(targets) => {
                if targets.is_empty() {
                    self.rejected += 1;
                    RouteDecision::Reject
                } else if targets.len() == 1 {
                    self.passed += 1;
                    RouteDecision::Forward(targets[0].clone())
                } else {
                    self.splits += 1;
                    RouteDecision::Split(targets.clone())
                }
            }
            None => {
                // 没有分派规则时拒绝
                self.rejected += 1;
                RouteDecision::Reject
            }
        }
    }

    /// 处理并计数
    pub fn process(&mut self, event: &FlowEvent) -> RouteDecision {
        self.separate(event)
    }

    /// 总处理数
    pub fn total_processed(&self) -> u64 {
        self.passed + self.rejected + self.splits
    }

    /// 通过率
    pub fn pass_rate(&self) -> f64 {
        let total = self.total_processed();
        if total == 0 {
            return 0.0;
        }
        self.passed as f64 / total as f64
    }
}

/// 谐音路由 — 同音异字 → 统一处理
#[derive(Clone, Debug)]
pub struct HomophonicRouter {
    /// 谐音映射: 同音编码 → 统一目标表 (key = hash of sound)
    pub sound_map: HashMap<u64, Vec<ContainerRef>>,
    /// 路由命中计数
    pub hits: u64,
    /// 路由未命中
    pub misses: u64,
}

impl HomophonicRouter {
    pub fn new() -> Self {
        HomophonicRouter {
            sound_map: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// 注册: 同音 → 目标
    pub fn register_sound(&mut self, sound_key: u64, target: ContainerRef) {
        self.sound_map
            .entry(sound_key)
            .or_default()
            .push(target);
    }

    /// 路由: 按声音找目标
    pub fn route(&mut self, sound_key: u64) -> Option<&Vec<ContainerRef>> {
        if let Some(targets) = self.sound_map.get(&sound_key) {
            self.hits += 1;
            Some(targets)
        } else {
            self.misses += 1;
            None
        }
    }

    /// 是否已注册
    pub fn knows(&self, sound_key: u64) -> bool {
        self.sound_map.contains_key(&sound_key)
    }

    /// 命中率
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// 已知声音数
    pub fn known_sounds(&self) -> usize {
        self.sound_map.len()
    }
}

impl Default for HomophonicRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// 主权路由 — 根据产权标记决定去向
#[derive(Clone, Debug)]
pub struct SovereignRouter {
    /// 产权标记 → 路由目标 (owner_id → target)
    pub ownership: HashMap<u64, ContainerRef>,
    /// 默认路由 (无标记时走的路径)
    pub default: Option<ContainerRef>,
    /// 命中/未命中
    pub hits: u64,
    pub misses: u64,
}

impl SovereignRouter {
    pub fn new() -> Self {
        SovereignRouter {
            ownership: HashMap::new(),
            default: None,
            hits: 0,
            misses: 0,
        }
    }

    /// 设置默认路由
    pub fn set_default(&mut self, target: ContainerRef) {
        self.default = Some(target);
    }

    /// 注册产权
    pub fn claim(&mut self, owner_id: u64, target: ContainerRef) {
        self.ownership.insert(owner_id, target);
    }

    /// 按产权路由
    pub fn route(&mut self, owner_id: u64) -> Option<&ContainerRef> {
        match self.ownership.get(&owner_id) {
            Some(target) => {
                self.hits += 1;
                Some(target)
            }
            None => {
                self.misses += 1;
                self.default.as_ref()
            }
        }
    }

    /// 验证产权
    pub fn verify(&self, owner_id: u64, target: &ContainerRef) -> bool {
        self.ownership.get(&owner_id).map(|t| t == target).unwrap_or(false)
    }

    /// 注销产权
    pub fn revoke(&mut self, owner_id: u64) -> bool {
        self.ownership.remove(&owner_id).is_some()
    }

    /// 命中率
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// 产权总数
    pub fn claim_count(&self) -> usize {
        self.ownership.len()
    }
}

impl Default for SovereignRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// 预加载流管理器 — 运行前预加载所有符号
#[derive(Clone, Debug)]
pub struct PreloadStream {
    /// 预加载表: 地址 → 容器 (内存不在运行时增加 = "加载即映射")
    pub preload_map: HashMap<u64, ContainerRef>,
    /// 已加载 (在位的)
    pub loaded: u64,
    /// 缺失 (需要后加载)
    pub missing: u64,
}

impl PreloadStream {
    pub fn new() -> Self {
        PreloadStream {
            preload_map: HashMap::new(),
            loaded: 0,
            missing: 0,
        }
    }

    /// 注册预加载地址
    pub fn preload(&mut self, address: u64, container: ContainerRef) {
        self.preload_map.insert(address, container);
        self.loaded += 1;
    }

    /// 解析地址
    pub fn resolve(&self, address: u64) -> Option<&ContainerRef> {
        let result = self.preload_map.get(&address);
        result
    }

    /// 是否已全部加载
    pub fn is_fully_loaded(&self, expected_count: usize) -> bool {
        self.loaded >= expected_count as u64
    }

    /// 预加载总数
    pub fn count(&self) -> usize {
        self.preload_map.len()
    }

    /// 批量预加载
    pub fn preload_batch(&mut self, items: &[(u64, ContainerRef)]) {
        for (addr, container) in items {
            self.preload(*addr, container.clone());
        }
    }

    /// 解析并标记 (未找到 = missing)
    pub fn resolve_or_mark(&mut self, address: u64) -> Option<&ContainerRef> {
        match self.preload_map.get(&address) {
            Some(container) => Some(container),
            None => {
                self.missing += 1;
                None
            }
        }
    }
}

impl Default for PreloadStream {
    fn default() -> Self {
        Self::new()
    }
}

/// 流代数引擎 — 整合分别器 · 谐音路由 · 主权路由 · 预加载
pub struct FlowAlgebraEngine {
    /// 分别器表
    pub separators: HashMap<String, Separator>,
    /// 谐音路由
    pub homophonic: HomophonicRouter,
    /// 主权路由
    pub sovereign: SovereignRouter,
    /// 预加载流
    pub preload: PreloadStream,
}

impl FlowAlgebraEngine {
    pub fn new() -> Self {
        FlowAlgebraEngine {
            separators: HashMap::new(),
            homophonic: HomophonicRouter::new(),
            sovereign: SovereignRouter::new(),
            preload: PreloadStream::new(),
        }
    }

    /// 注册分别器
    pub fn add_separator(&mut self, sep: Separator) {
        self.separators.insert(sep.name.clone(), sep);
    }

    /// 获取分别器
    pub fn get_separator(&mut self, name: &str) -> Option<&mut Separator> {
        self.separators.get_mut(name)
    }

    /// 分离流
    pub fn separate(&mut self, sep_name: &str, event: &FlowEvent) -> Option<RouteDecision> {
        self.separators.get_mut(sep_name).map(|s| s.process(event))
    }

    /// 注册谐音
    pub fn register_sound(&mut self, key: u64, target: ContainerRef) {
        self.homophonic.register_sound(key, target);
    }

    /// 谐音路由
    pub fn route_by_sound(&mut self, key: u64) -> Option<&Vec<ContainerRef>> {
        self.homophonic.route(key)
    }

    /// 注册产权
    pub fn claim_owner(&mut self, owner_id: u64, target: ContainerRef) {
        self.sovereign.claim(owner_id, target);
    }

    /// 主权路由
    pub fn route_by_owner(&mut self, owner_id: u64) -> Option<&ContainerRef> {
        self.sovereign.route(owner_id)
    }

    /// 预加载
    pub fn preload_symbol(&mut self, address: u64, container: ContainerRef) {
        self.preload.preload(address, container);
    }

    /// 解析符号
    pub fn resolve_symbol(&mut self, address: u64) -> Option<&ContainerRef> {
        self.preload.resolve_or_mark(address)
    }

    /// 流事件属性更新
    pub fn update_flags(event: &mut FlowEvent, flags: StreamFlags) {
        event.flags = flags;
    }

    /// 系统报告
    pub fn status(&self) -> FlowStatus {
        FlowStatus {
            separators: self.separators.len(),
            homophonic_known: self.homophonic.known_sounds(),
            sovereign_claims: self.sovereign.claim_count(),
            preload_loaded: self.preload.count(),
        }
    }
}

impl Default for FlowAlgebraEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FlowStatus {
    pub separators: usize,
    pub homophonic_known: usize,
    pub sovereign_claims: usize,
    pub preload_loaded: usize,
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
    fn stream_flags_default_empty() {
        let f = StreamFlags::new();
        assert!(!f.is_present());
        assert!(!f.is_superposition());
        assert!(!f.is_yin());
        assert!(!f.is_yang());
    }

    #[test]
    fn stream_flags_set_and_has() {
        let mut f = StreamFlags::new();
        f.set(StreamFlags::PRESENT);
        assert!(f.is_present());

        f.set(StreamFlags::YANG);
        assert!(f.is_yang());
        assert!(!f.is_yin());
    }

    #[test]
    fn stream_flags_clear() {
        let mut f = StreamFlags::with(StreamFlags::ALL);
        assert!(f.raw() == StreamFlags::ALL);

        f.clear(StreamFlags::YANG);
        assert!(!f.is_yang());
        assert!(f.is_yin());
    }

    #[test]
    fn stream_flags_polarity_label() {
        let yin = StreamFlags::with(StreamFlags::YIN);
        assert_eq!(yin.polarity_label(), "阴");

        let yang = StreamFlags::with(StreamFlags::YANG);
        assert_eq!(yang.polarity_label(), "阳");

        let both = StreamFlags::with(StreamFlags::YIN | StreamFlags::YANG);
        assert_eq!(both.polarity_label(), "叠加");

        let neutral = StreamFlags::new();
        assert_eq!(neutral.polarity_label(), "中性");
    }

    #[test]
    fn flow_event_basic() {
        let event = FlowEvent::new(
            mk_ref(1),
            mk_ref(2),
            42,
            StreamKind::Preload,
            0,
        );
        assert_eq!(event.payload, 42);
        assert_eq!(event.weight, 1);
        assert_eq!(event.tick, 0);
    }

    #[test]
    fn separator_single_target() {
        let mut sep = Separator::main_gate();
        sep.on(StreamKind::Preload, vec![mk_ref(100)]);

        let event = FlowEvent::new(mk_ref(1), mk_ref(2), 0, StreamKind::Preload, 0);
        let decision = sep.separate(&event);

        assert_eq!(decision, RouteDecision::Forward(mk_ref(100)));
        assert_eq!(sep.passed, 1);
    }

    #[test]
    fn separator_split() {
        let mut sep = Separator::new("splitter");
        sep.on(StreamKind::Growth, vec![mk_ref(100), mk_ref(200)]);

        let event = FlowEvent::new(mk_ref(1), mk_ref(2), 0, StreamKind::Growth, 0);
        let decision = sep.separate(&event);

        assert_eq!(decision, RouteDecision::Split(vec![mk_ref(100), mk_ref(200)]));
        assert_eq!(sep.splits, 1);
    }

    #[test]
    fn separator_reject_unknown() {
        let mut sep = Separator::new("strict");

        let event = FlowEvent::new(mk_ref(1), mk_ref(2), 0, StreamKind::Preload, 0);
        let decision = sep.separate(&event);

        assert_eq!(decision, RouteDecision::Reject);
        assert_eq!(sep.rejected, 1);
    }

    #[test]
    fn separator_pass_rate() {
        let mut sep = Separator::new("test");
        // 不注册, 全部拒绝

        for _ in 0..10 {
            let event = FlowEvent::new(mk_ref(1), mk_ref(2), 0, StreamKind::Homophonic, 0);
            sep.separate(&event);
        }

        assert_eq!(sep.pass_rate(), 0.0);
        assert_eq!(sep.total_processed(), 10);
    }

    #[test]
    fn homophonic_register_and_route() {
        let mut router = HomophonicRouter::new();
        router.register_sound(0xABCD, mk_ref(100));
        router.register_sound(0xABCD, mk_ref(200));

        let targets = router.route(0xABCD);
        assert!(targets.is_some());
        assert_eq!(targets.unwrap().len(), 2);
    }

    #[test]
    fn homophonic_miss() {
        let mut router = HomophonicRouter::new();
        let result = router.route(0xFFFF);
        assert!(result.is_none());
        assert_eq!(router.misses, 1);
    }

    #[test]
    fn homophonic_knows() {
        let mut router = HomophonicRouter::new();
        router.register_sound(0x1234, mk_ref(1));

        assert!(router.knows(0x1234));
        assert!(!router.knows(0x5678));
    }

    #[test]
    fn homophonic_hit_rate() {
        let mut router = HomophonicRouter::new();
        router.register_sound(0x100, mk_ref(1));
        router.route(0x100); // hit
        router.route(0x200); // miss
        router.route(0x100); // hit

        assert!((router.hit_rate() - 0.6667).abs() < 0.01);
    }

    #[test]
    fn sovereign_claim_and_route() {
        let mut router = SovereignRouter::new();
        router.claim(0x100, mk_ref(100));

        let target = router.route(0x100);
        assert!(target.is_some());
        assert_eq!(target.unwrap().raw(), 100);
    }

    #[test]
    fn sovereign_default() {
        let mut router = SovereignRouter::new();
        router.set_default(mk_ref(999));

        let target = router.route(0x100);
        assert!(target.is_some());
        assert_eq!(target.unwrap().raw(), 999);
    }

    #[test]
    fn sovereign_verify() {
        let mut router = SovereignRouter::new();
        router.claim(0x100, mk_ref(100));

        assert!(router.verify(0x100, &mk_ref(100)));
        assert!(!router.verify(0x100, &mk_ref(999)));
    }

    #[test]
    fn sovereign_revoke() {
        let mut router = SovereignRouter::new();
        router.claim(0x100, mk_ref(100));
        assert!(router.verify(0x100, &mk_ref(100)));

        router.revoke(0x100);
        assert!(!router.verify(0x100, &mk_ref(100)));
    }

    #[test]
    fn preload_register_and_resolve() {
        let mut s = PreloadStream::new();
        s.preload(0x6000_0000, mk_ref(42));

        let resolved = s.resolve(0x6000_0000);
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().raw(), 42);
    }

    #[test]
    fn preload_resolve_or_mark() {
        let mut s = PreloadStream::new();
        s.preload(0x6000_0000, mk_ref(42));

        let r1 = s.resolve_or_mark(0x6000_0000);
        assert!(r1.is_some());

        let r2 = s.resolve_or_mark(0xFFFF_FFFF);
        assert!(r2.is_none());
        assert_eq!(s.missing, 1);
    }

    #[test]
    fn preload_batch() {
        let mut s = PreloadStream::new();
        let batch: Vec<(u64, ContainerRef)> = (0..10)
            .map(|i| (0x6000_0000 + i, mk_ref(i as u64)))
            .collect();
        s.preload_batch(&batch);

        assert_eq!(s.count(), 10);
        assert!(s.is_fully_loaded(10));
    }

    #[test]
    fn preload_fully_loaded() {
        let mut s = PreloadStream::new();
        assert!(!s.is_fully_loaded(1));

        s.preload(0x0, mk_ref(1));
        assert!(s.is_fully_loaded(1));
    }

    #[test]
    fn flow_engine_new() {
        let engine = FlowAlgebraEngine::new();
        assert_eq!(engine.separators.len(), 0);
        assert_eq!(engine.preload.count(), 0);
    }

    #[test]
    fn flow_engine_add_separator() {
        let mut engine = FlowAlgebraEngine::new();
        engine.add_separator(Separator::new("main"));

        assert_eq!(engine.separators.len(), 1);
        assert!(engine.separators.contains_key("main"));
    }

    #[test]
    fn flow_engine_separate() {
        let mut engine = FlowAlgebraEngine::new();
        let mut sep = Separator::new("gate");
        sep.on(StreamKind::Communication, vec![mk_ref(300)]);
        engine.add_separator(sep);

        let event = FlowEvent::new(mk_ref(1), mk_ref(2), 0, StreamKind::Communication, 0);
        let decision = engine.separate("gate", &event);

        assert!(decision.is_some());
        assert_eq!(decision.unwrap(), RouteDecision::Forward(mk_ref(300)));
    }

    #[test]
    fn flow_engine_preload_and_resolve() {
        let mut engine = FlowAlgebraEngine::new();
        engine.preload_symbol(0x6000_0000, mk_ref(42));

        let r = engine.resolve_symbol(0x6000_0000);
        assert!(r.is_some());
    }

    #[test]
    fn flow_engine_status() {
        let mut engine = FlowAlgebraEngine::new();
        engine.register_sound(0x1, mk_ref(1));
        engine.claim_owner(0x1, mk_ref(1));
        engine.preload_symbol(0x1, mk_ref(1));

        let status = engine.status();
        assert_eq!(status.homophonic_known, 1);
        assert_eq!(status.sovereign_claims, 1);
        assert_eq!(status.preload_loaded, 1);
    }

    #[test]
    fn route_decision_split_has_multiple() {
        let decision = RouteDecision::Split(vec![mk_ref(1), mk_ref(2), mk_ref(3)]);
        if let RouteDecision::Split(targets) = decision {
            assert_eq!(targets.len(), 3);
        } else {
            panic!("expected Split");
        }
    }

    #[test]
    fn separator_multiple_dispatches() {
        let mut sep = Separator::new("multi");
        sep.on(StreamKind::Preload, vec![mk_ref(1)]);
        sep.on(StreamKind::Growth, vec![mk_ref(2), mk_ref(3)]);

        let ev1 = FlowEvent::new(mk_ref(10), mk_ref(20), 0, StreamKind::Preload, 0);
        let r1 = sep.separate(&ev1);
        assert_eq!(r1, RouteDecision::Forward(mk_ref(1)));

        let ev2 = FlowEvent::new(mk_ref(10), mk_ref(20), 0, StreamKind::Growth, 1);
        let r2 = sep.separate(&ev2);
        assert_eq!(r2, RouteDecision::Split(vec![mk_ref(2), mk_ref(3)]));
    }
}

impl Separator {
    /// 快速创建名为 "main" 的分别器
    pub fn main_gate() -> Self {
        Separator::new("main_gate")
    }
}

// ── helpers ──────────────────────────────────────────────────

impl FlowEvent {
    pub fn raw(&self) -> u64 {
        self.payload
    }
}
