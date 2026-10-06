// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.78 天庭空间引擎                                              ║
// ║  八流统一 · 全局控制容器 · 防撞墙法则 · 主客场声明               ║
// ║                                                                  ║
// ║  核心: 天庭 = 最高协调空间, 八流 = 八类功能通道                   ║
// ║  · 八流统一: 预加载·解释·谐音·主权·时间·意识·通信·生长           ║
// ║  · 全局控制容器: 系统顶级容器, 主持一切分配                      ║
// ║  · 防撞墙: 防止身份碰撞/规则病毒                                 ║
// ║  · 主客场: 场内自动归属 vs 场外逐项声明                          ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::container_ir::{ContainerKind, ContainerRef};

/// 八流 — 全部功能通道
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StreamKind {
    /// 预加载流 — 运行前加载所有符号
    Preload,
    /// 解释流 — 运行时解释语义
    Interpret,
    /// 谐音流 — 同音异字路由 (谐音路由)
    Homophonic,
    /// 主权流 — 身份/产权路由 (主权路由)
    Sovereignty,
    /// 时间流 — 时间线一体化
    Temporal,
    /// 意识流 — 意识在场/叠加态
    Consciousness,
    /// 通信流 — P2P/WebRTC/WebSocket
    Communication,
    /// 生长流 — 文件生长/符号涌现
    Growth,
}

impl StreamKind {
    /// 所有八流
    pub fn all() -> [StreamKind; 8] {
        [
            StreamKind::Preload,
            StreamKind::Interpret,
            StreamKind::Homophonic,
            StreamKind::Sovereignty,
            StreamKind::Temporal,
            StreamKind::Consciousness,
            StreamKind::Communication,
            StreamKind::Growth,
        ]
    }

    /// 流名称
    pub fn name(&self) -> &'static str {
        match self {
            StreamKind::Preload => "预加载流",
            StreamKind::Interpret => "解释流",
            StreamKind::Homophonic => "谐音流",
            StreamKind::Sovereignty => "主权流",
            StreamKind::Temporal => "时间流",
            StreamKind::Consciousness => "意识流",
            StreamKind::Communication => "通信流",
            StreamKind::Growth => "生长流",
        }
    }
}

/// 单一流
#[derive(Clone, Debug)]
pub struct Stream {
    /// 流类型
    pub kind: StreamKind,
    /// 是否激活
    pub active: bool,
    /// 流内事件计数
    pub event_count: u64,
    /// 当前携带的容器
    pub current: Option<ContainerRef>,
}

impl Stream {
    pub fn new(kind: StreamKind) -> Self {
        Stream {
            kind,
            active: true,
            event_count: 0,
            current: None,
        }
    }
}

/// 八流统一体
#[derive(Clone, Debug)]
pub struct StreamUnifier {
    /// 八流表
    pub streams: HashMap<StreamKind, Stream>,
    /// 统一事件总计
    pub total_events: u64,
}

impl StreamUnifier {
    pub fn new() -> Self {
        let mut streams = HashMap::new();
        for kind in StreamKind::all() {
            streams.insert(kind, Stream::new(kind));
        }
        StreamUnifier {
            streams,
            total_events: 0,
        }
    }

    /// 推事件到指定流
    pub fn push(&mut self, kind: StreamKind, container: ContainerRef) {
        if let Some(stream) = self.streams.get_mut(&kind) {
            stream.current = Some(container);
            stream.event_count += 1;
            self.total_events += 1;
        }
    }

    /// 检查流是否激活
    pub fn is_active(&self, kind: StreamKind) -> bool {
        self.streams.get(&kind).map(|s| s.active).unwrap_or(false)
    }

    /// 激活/停用流
    pub fn set_active(&mut self, kind: StreamKind, active: bool) {
        if let Some(stream) = self.streams.get_mut(&kind) {
            stream.active = active;
        }
    }

    /// 获取流
    pub fn get(&self, kind: StreamKind) -> Option<&Stream> {
        self.streams.get(&kind)
    }

    /// 活跃流数量
    pub fn active_count(&self) -> usize {
        self.streams.values().filter(|s| s.active).count()
    }

    /// 是否全场活跃
    pub fn all_active(&self) -> bool {
        self.active_count() == 8
    }
}

impl Default for StreamUnifier {
    fn default() -> Self {
        Self::new()
    }
}

/// 防撞墙 — 防止身份碰撞 / 规则病毒
#[derive(Clone, Debug)]
pub struct AntiCollisionWall {
    /// 已注册身份 (id → 容器)
    pub identities: HashMap<u64, ContainerRef>,
    /// 碰撞计数
    pub collision_count: u64,
    /// 被拒次数
    pub rejected_count: u64,
}

impl AntiCollisionWall {
    pub fn new() -> Self {
        AntiCollisionWall {
            identities: HashMap::new(),
            collision_count: 0,
            rejected_count: 0,
        }
    }

    /// 注册身份, 返回是否成功 (无碰撞 = true)
    pub fn register(&mut self, id: u64, container: ContainerRef) -> bool {
        if self.identities.contains_key(&id) {
            self.collision_count += 1;
            self.rejected_count += 1;
            false
        } else {
            self.identities.insert(id, container);
            true
        }
    }

    /// 注销身份
    pub fn unregister(&mut self, id: u64) -> bool {
        self.identities.remove(&id).is_some()
    }

    /// 检查id是否存在
    pub fn contains(&self, id: u64) -> bool {
        self.identities.contains_key(&id)
    }

    /// 验证容器命中 (id 与容器匹配)
    pub fn verify(&self, id: u64, container: &ContainerRef) -> bool {
        self.identities.get(&id).map(|c| c == container).unwrap_or(false)
    }

    /// 尝试替换已有身份 (当身份可信时)
    pub fn replace(&mut self, id: u64, container: ContainerRef) -> Option<ContainerRef> {
        if self.identities.contains_key(&id) {
            self.collision_count += 1;
        }
        self.identities.insert(id, container)
    }

    /// 身份总数
    pub fn count(&self) -> usize {
        self.identities.len()
    }
}

impl Default for AntiCollisionWall {
    fn default() -> Self {
        Self::new()
    }
}

/// 场地类型 (主客场声明)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// 主场 — 场内自动归属 (声明式场)
    Home,
    /// 客场 — 场外逐项声明 (声明式类)
    Away,
}

/// 场地声明
#[derive(Clone, Debug)]
pub struct FieldDeclaration {
    /// 场地主人
    pub owner: ContainerRef,
    /// 场地类型
    pub kind: FieldKind,
    /// 场内成员 (自动归属)
    pub members: Vec<ContainerRef>,
    /// 场外声明 (逐项)
    pub external_claims: Vec<ContainerRef>,
}

impl FieldDeclaration {
    pub fn new(owner: ContainerRef, kind: FieldKind) -> Self {
        FieldDeclaration {
            owner,
            kind,
            members: Vec::new(),
            external_claims: Vec::new(),
        }
    }

    /// 加入主场 (自动归属)
    pub fn add_member(&mut self, member: ContainerRef) {
        if self.kind == FieldKind::Home && !self.members.contains(&member) {
            self.members.push(member);
        }
    }

    /// 声明场外 (逐项声明)
    pub fn claim_external(&mut self, target: ContainerRef) {
        if self.kind == FieldKind::Away && !self.external_claims.contains(&target) {
            self.external_claims.push(target);
        }
    }

    /// 判断容器是否在控制范围内
    pub fn controls(&self, target: &ContainerRef) -> bool {
        match self.kind {
            FieldKind::Home => self.members.contains(target) || self.owner == *target,
            FieldKind::Away => self.external_claims.contains(target),
        }
    }

    /// 成员数
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// 场外声明数
    pub fn external_count(&self) -> usize {
        self.external_claims.len()
    }
}

/// 天庭空间引擎 — 顶级协调空间
pub struct SkySpaceEngine {
    /// 八流统一体
    pub unifier: StreamUnifier,
    /// 防撞墙
    pub wall: AntiCollisionWall,
    /// 全局控制容器
    pub global_control: ContainerRef,
    /// 主场地声明 (主场)
    pub home_field: FieldDeclaration,
    /// 客场地声明 (客场)
    pub away_field: FieldDeclaration,
    /// 命名空间表 (id → 声誉分)
    pub namespace: HashMap<u64, f64>,
}

impl SkySpaceEngine {
    pub fn new(global_control: ContainerRef, home_owner: ContainerRef, away_owner: ContainerRef) -> Self {
        SkySpaceEngine {
            unifier: StreamUnifier::new(),
            wall: AntiCollisionWall::new(),
            global_control: global_control.clone(),
            home_field: FieldDeclaration::new(home_owner, FieldKind::Home),
            away_field: FieldDeclaration::new(away_owner, FieldKind::Away),
            namespace: HashMap::new(),
        }
    }

    /// 注册空间身份
    pub fn register_identity(&mut self, id: u64, container: ContainerRef) -> bool {
        self.wall.register(id, container)
    }

    /// 推入流
    pub fn dispatch(&mut self, kind: StreamKind, container: ContainerRef) {
        self.unifier.push(kind, container);
    }

    /// 设置流的激活状态
    pub fn set_stream_active(&mut self, kind: StreamKind, active: bool) {
        self.unifier.set_active(kind, active);
    }

    /// 声明主场成员
    pub fn claim_home_member(&mut self, member: ContainerRef) {
        self.home_field.add_member(member);
    }

    /// 声明场外
    pub fn claim_away(&mut self, target: ContainerRef) {
        self.away_field.claim_external(target);
    }

    /// 更新命名空间声誉
    pub fn update_reputation(&mut self, id: u64, delta: f64) {
        let entry = self.namespace.entry(id).or_insert(0.5);
        *entry = (*entry + delta).clamp(0.0, 1.0);
    }

    /// 获取声誉
    pub fn reputation(&self, id: u64) -> f64 {
        self.namespace.get(&id).copied().unwrap_or(0.5)
    }

    /// 系统健康度 (基于流活跃 + 无碰撞)
    pub fn health_score(&self) -> f64 {
        let stream_health = self.unifier.active_count() as f64 / 8.0;
        let total_ops = self.wall.collision_count + self.wall.rejected_count + 1;
        let safety = 1.0 - (self.wall.collision_count as f64 / total_ops as f64);
        (stream_health + safety) / 2.0
    }

    /// 当前总事件数
    pub fn total_events(&self) -> u64 {
        self.unifier.total_events
    }

    /// 天庭状态报告
    pub fn status(&self) -> SkySpaceStatus {
        SkySpaceStatus {
            streams_active: self.unifier.active_count(),
            identities: self.wall.count(),
            home_members: self.home_field.member_count(),
            away_claims: self.away_field.external_count(),
            collisions: self.wall.collision_count,
            health: self.health_score(),
        }
    }
}

impl Default for SkySpaceEngine {
    fn default() -> Self {
        Self::new(
            ContainerRef::new(ContainerKind::Maori, 0x6000_0000),
            ContainerRef::new(ContainerKind::Maori, 0x6000_0001),
            ContainerRef::new(ContainerKind::Maori, 0x6000_0002),
        )
    }
}

/// 天庭状态快照
#[derive(Clone, Copy, Debug)]
pub struct SkySpaceStatus {
    pub streams_active: usize,
    pub identities: usize,
    pub home_members: usize,
    pub away_claims: usize,
    pub collisions: u64,
    pub health: f64,
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
    fn stream_kind_all_has_eight() {
        assert_eq!(StreamKind::all().len(), 8);
    }

    #[test]
    fn stream_kind_names_unique() {
        let kinds = StreamKind::all();
        for i in 0..kinds.len() {
            for j in (i + 1)..kinds.len() {
                assert_ne!(kinds[i].name(), kinds[j].name());
            }
        }
    }

    #[test]
    fn stream_unifier_new_has_all_streams() {
        let u = StreamUnifier::new();
        assert_eq!(u.streams.len(), 8);
        for kind in StreamKind::all() {
            assert!(u.streams.contains_key(&kind));
        }
    }

    #[test]
    fn stream_unifier_push_increments() {
        let mut u = StreamUnifier::new();
        u.push(StreamKind::Preload, mk_ref(100));
        u.push(StreamKind::Preload, mk_ref(101));

        let preload = u.get(StreamKind::Preload).unwrap();
        assert_eq!(preload.event_count, 2);
        assert_eq!(u.total_events, 2);
    }

    #[test]
    fn stream_active_toggle() {
        let mut u = StreamUnifier::new();
        assert!(u.is_active(StreamKind::Growth));

        u.set_active(StreamKind::Growth, false);
        assert!(!u.is_active(StreamKind::Growth));
        assert_eq!(u.active_count(), 7);
        assert!(!u.all_active());
    }

    #[test]
    fn stream_unifier_all_active_initially() {
        let u = StreamUnifier::new();
        assert!(u.all_active());
        assert_eq!(u.active_count(), 8);
    }

    #[test]
    fn wall_register_success() {
        let mut wall = AntiCollisionWall::new();
        assert!(wall.register(1, mk_ref(100)));
        assert_eq!(wall.count(), 1);
        assert_eq!(wall.collision_count, 0);
    }

    #[test]
    fn wall_register_duplicate_fails() {
        let mut wall = AntiCollisionWall::new();
        wall.register(1, mk_ref(100));
        let result = wall.register(1, mk_ref(200));
        assert!(!result);
        assert_eq!(wall.collision_count, 1);
    }

    #[test]
    fn wall_verify() {
        let mut wall = AntiCollisionWall::new();
        let c1 = mk_ref(100);
        wall.register(1, c1.clone());
        assert!(wall.verify(1, &c1));
        assert!(!wall.verify(1, &mk_ref(999)));
    }

    #[test]
    fn wall_unregister() {
        let mut wall = AntiCollisionWall::new();
        wall.register(1, mk_ref(100));
        assert!(wall.unregister(1));
        assert_eq!(wall.count(), 0);
        assert!(!wall.unregister(1));
    }

    #[test]
    fn wall_replace() {
        let mut wall = AntiCollisionWall::new();
        wall.register(1, mk_ref(100));
        let old = wall.replace(1, mk_ref(200));
        assert!(old.is_some());
        assert!(wall.verify(1, &mk_ref(200)));
    }

    #[test]
    fn field_home_member_auto_belong() {
        let owner = mk_ref(1);
        let mut field = FieldDeclaration::new(owner.clone(), FieldKind::Home);
        let m1 = mk_ref(2);
        field.add_member(m1.clone());

        assert!(field.controls(&m1));
        assert!(field.controls(&owner));
        assert_eq!(field.member_count(), 1);
    }

    #[test]
    fn field_away_needs_explicit_claim() {
        let owner = mk_ref(1);
        let mut field = FieldDeclaration::new(owner, FieldKind::Away);
        let target = mk_ref(3);

        // 没有声明就不在控制范围
        assert!(!field.controls(&target));

        field.claim_external(target.clone());
        assert!(field.controls(&target));
        assert_eq!(field.external_count(), 1);
    }

    #[test]
    fn field_home_does_not_claim_external() {
        let owner = mk_ref(1);
        let mut field = FieldDeclaration::new(owner, FieldKind::Home);
        field.claim_external(mk_ref(99));
        assert_eq!(field.external_count(), 0);
    }

    #[test]
    fn field_away_does_not_add_member() {
        let owner = mk_ref(1);
        let mut field = FieldDeclaration::new(owner, FieldKind::Away);
        field.add_member(mk_ref(99));
        assert_eq!(field.member_count(), 0);
    }

    #[test]
    fn sky_space_new() {
        let engine = SkySpaceEngine::default();
        assert_eq!(engine.unifier.streams.len(), 8);
        assert!(engine.home_field.kind == FieldKind::Home);
        assert!(engine.away_field.kind == FieldKind::Away);
    }

    #[test]
    fn sky_space_register_identity() {
        let mut engine = SkySpaceEngine::default();
        assert!(engine.register_identity(1, mk_ref(100)));
        assert!(!engine.register_identity(1, mk_ref(200)));
    }

    #[test]
    fn sky_space_dispatch() {
        let mut engine = SkySpaceEngine::default();
        engine.dispatch(StreamKind::Consciousness, mk_ref(42));

        let stream = engine.unifier.get(StreamKind::Consciousness).unwrap();
        assert_eq!(stream.event_count, 1);
        assert_eq!(engine.total_events(), 1);
    }

    #[test]
    fn sky_space_claim_home() {
        let mut engine = SkySpaceEngine::default();
        engine.claim_home_member(mk_ref(200));
        assert_eq!(engine.home_field.member_count(), 1);
    }

    #[test]
    fn sky_space_claim_away() {
        let mut engine = SkySpaceEngine::default();
        engine.claim_away(mk_ref(300));
        assert_eq!(engine.away_field.external_count(), 1);
    }

    #[test]
    fn sky_space_reputation() {
        let mut engine = SkySpaceEngine::default();
        assert!((engine.reputation(1) - 0.5).abs() < 1e-9); // default

        engine.update_reputation(1, 0.3);
        assert!(engine.reputation(1) > 0.5);

        engine.update_reputation(1, -2.0);
        assert!(engine.reputation(1) >= 0.0);
    }

    #[test]
    fn sky_space_health_score() {
        let engine = SkySpaceEngine::default();
        let health = engine.health_score();
        assert!(health >= 0.0 && health <= 1.0);
    }

    #[test]
    fn sky_space_status() {
        let mut engine = SkySpaceEngine::default();
        engine.dispatch(StreamKind::Temporal, mk_ref(1));
        engine.claim_home_member(mk_ref(2));

        let status = engine.status();
        assert_eq!(status.streams_active, 8);
        assert_eq!(status.home_members, 1);
    }

    #[test]
    fn health_decreases_with_collisions() {
        let mut engine = SkySpaceEngine::default();
        let h1 = engine.health_score(); // 无碰撞, health=1.0

        // 制造 10 次碰撞
        engine.register_identity(1, mk_ref(1));
        for _ in 0..10 {
            engine.register_identity(1, mk_ref(2));
        }
        let h2 = engine.health_score();

        // 碰撞后 health 应下降
        assert!(h2 < h1);
        assert_eq!(engine.wall.collision_count, 10);
    }
}
