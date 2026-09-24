// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.73 自寄生引擎                                                ║
// ║  心跳监控 · 自修复 · 分布式复制 · 免疫响应 · 不可摧毁性            ║
// ║                                                                  ║
// ║  核心命题: 使软件系统难以被摧毁或删除                             ║
// ║  · 心跳监控: 检测各部件活性, 发现失联立即标记                     ║
// ║  · 自修复: 从分布式碎片重建完整状态                                ║
// ║  · 蚕蛹策略: 外围壳 + 内核可脱壳穿衣                              ║
// ║  · 免疫响应: 检测篡改 → 触发回滚/报警                             ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerRef, RelationKind};
use std::collections::HashMap;

/// 心跳状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeartbeatStatus {
    /// 存活
    Alive,
    /// 迟钝 (响应慢但在线)
    Sluggish,
    /// 失联
    Dead,
    /// 重生 (从碎片中恢复)
    Resurrected,
}

/// 心跳记录
#[derive(Clone, Debug)]
pub struct HeartbeatRecord {
    /// 容器引用
    pub target: ContainerRef,
    /// 最后心跳时刻
    pub last_tick: u64,
    /// 心跳间隔 (tick 单位)
    pub expected_interval: u64,
    /// 当前状态
    pub status: HeartbeatStatus,
    /// 连续失联次数
    pub miss_count: u32,
}

impl HeartbeatRecord {
    pub fn new(target: ContainerRef, expected_interval: u64) -> Self {
        HeartbeatRecord {
            target,
            last_tick: 0,
            expected_interval,
            status: HeartbeatStatus::Alive,
            miss_count: 0,
        }
    }

    /// 是否超时
    pub fn is_timeout(&self, current_tick: u64) -> bool {
        current_tick.saturating_sub(self.last_tick) > self.expected_interval
    }

    /// 更新心跳
    pub fn beat(&mut self, current_tick: u64) {
        self.last_tick = current_tick;
        self.status = HeartbeatStatus::Alive;
        self.miss_count = 0;
    }

    /// 标记一次失联
    pub fn miss(&mut self) {
        self.miss_count += 1;
        if self.miss_count >= 3 {
            self.status = HeartbeatStatus::Dead;
        } else if self.miss_count >= 1 {
            self.status = HeartbeatStatus::Sluggish;
        }
    }
}

/// 心跳监控器
pub struct HeartbeatMonitor {
    records: HashMap<u128, HeartbeatRecord>,
    tick: u64,
    max_miss_before_dead: u32,
}

impl HeartbeatMonitor {
    pub fn new() -> Self {
        HeartbeatMonitor {
            records: HashMap::new(),
            tick: 0,
            max_miss_before_dead: 3,
        }
    }

    /// 注册被监控目标
    pub fn watch(&mut self, target: ContainerRef, interval: u64) {
        self.records.insert(target.raw(), HeartbeatRecord::new(target, interval));
    }

    /// 前进一拍
    pub fn tick(&mut self) {
        self.tick += 1;
        for record in self.records.values_mut() {
            if record.is_timeout(self.tick) {
                record.miss();
            }
        }
    }

    /// 发送心跳
    pub fn beat(&mut self, target: &ContainerRef) {
        if let Some(record) = self.records.get_mut(&target.raw()) {
            record.beat(self.tick);
        }
    }

    /// 获取状态
    pub fn status_of(&self, target: &ContainerRef) -> Option<HeartbeatStatus> {
        self.records.get(&target.raw()).map(|r| r.status)
    }

    /// 失联目标列表
    pub fn dead_targets(&self) -> Vec<&ContainerRef> {
        self.records.values()
            .filter(|r| r.status == HeartbeatStatus::Dead)
            .map(|r| &r.target)
            .collect()
    }

    /// 全部存活
    pub fn all_alive(&self) -> bool {
        self.records.values().all(|r| r.status == HeartbeatStatus::Alive || r.status == HeartbeatStatus::Resurrected)
    }

    /// 监控目标数
    pub fn watch_count(&self) -> usize {
        self.records.len()
    }
}

impl Default for HeartbeatMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 复制节点 — 蚕蛹分布式备份的一个节点
#[derive(Clone, Debug)]
pub struct ReplicaNode {
    /// 节点 ID
    pub node_id: u64,
    /// 节点地址 (域名/IP/路径)
    pub address: String,
    /// 持有的容器引用
    pub fragments: Vec<ContainerRef>,
    /// 最后同步时刻
    pub last_sync_tick: u64,
    /// 节点是否在线
    pub online: bool,
}

impl ReplicaNode {
    pub fn new(node_id: u64, address: &str) -> Self {
        ReplicaNode {
            node_id,
            address: address.to_string(),
            fragments: Vec::new(),
            last_sync_tick: 0,
            online: true,
        }
    }

    /// 存储碎片
    pub fn store_fragment(&mut self, cref: ContainerRef) {
        if !self.fragments.iter().any(|f| f.raw() == cref.raw()) {
            self.fragments.push(cref);
        }
    }

    /// 是否持有指定碎片
    pub fn has_fragment(&self, cref: &ContainerRef) -> bool {
        self.fragments.iter().any(|f| f.raw() == cref.raw())
    }
}

/// 蚕蛹状态
#[derive(Clone, Debug)]
pub enum CocoonState {
    /// 穿着外壳 (正常运行)
    Cloaked,
    /// 脱壳 (正在迁移)
    Molting,
    /// 穿衣 (已换到新宿主)
    Redressed,
    /// 假死 (休眠等待唤醒)
    PlayingDead,
}

/// 自修复引擎
pub struct SelfRepair {
    /// 备份节点
    replicas: Vec<ReplicaNode>,
    /// 蚕蛹状态
    cocoon_state: CocoonState,
    /// 修复统计
    repair_count: u64,
    /// 校验失败触发回滚的次数
    rollback_count: u64,
}

impl SelfRepair {
    pub fn new() -> Self {
        SelfRepair {
            replicas: Vec::new(),
            cocoon_state: CocoonState::Cloaked,
            repair_count: 0,
            rollback_count: 0,
        }
    }

    /// 添加复制节点
    pub fn add_replica(&mut self, node_id: u64, address: &str) -> usize {
        self.replicas.push(ReplicaNode::new(node_id, address));
        self.replicas.len() - 1
    }

    /// 从节点恢复
    pub fn repair_from(&mut self, node_id: u64) -> Vec<ContainerRef> {
        if let Some(node) = self.replicas.iter().find(|n| n.node_id == node_id) {
            self.repair_count += 1;
            node.fragments.clone()
        } else {
            Vec::new()
        }
    }

    /// 蚕蛹脱壳 — 切换到脱壳态
    pub fn molt(&mut self) {
        self.cocoon_state = CocoonState::Molting;
    }

    /// 蚕蛹穿衣 — 迁移完成后
    pub fn redress(&mut self) {
        self.cocoon_state = CocoonState::Redressed;
        // 穿衣后恢复常态
        self.cocoon_state = CocoonState::Cloaked;
    }

    /// 蚕蛹假死 — 对抗删除的最后防线
    pub fn play_dead(&mut self) {
        self.cocoon_state = CocoonState::PlayingDead;
    }

    /// 触发回滚
    pub fn rollback(&mut self) {
        self.rollback_count += 1;
    }

    /// 当前蚕蛹状态
    pub fn cocoon_state(&self) -> &CocoonState {
        &self.cocoon_state
    }

    /// 复制节点数
    pub fn replica_count(&self) -> usize {
        self.replicas.len()
    }

    /// 总碎片数
    pub fn total_fragments(&self) -> usize {
        self.replicas.iter().map(|n| n.fragments.len()).sum()
    }

    /// 修复计数
    pub fn repair_count(&self) -> u64 {
        self.repair_count
    }

    /// 回滚计数
    pub fn rollback_count(&self) -> u64 {
        self.rollback_count
    }
}

impl Default for SelfRepair {
    fn default() -> Self {
        Self::new()
    }
}

/// 免疫事件
#[derive(Clone, Debug)]
pub enum ImmuneEvent {
    /// 检测到篡改
    Tampering { target: ContainerRef, expected: u64, found: u64 },
    /// 检测到删除尝试
    DeletionAttempt { target: ContainerRef },
    /// 检测到外来代码
    ForeignCode { source: String },
}

/// 免疫响应引擎
pub struct ImmuneResponse {
    /// 事件日志
    events: Vec<ImmuneEvent>,
    /// 响应阈值 (多少事件触发反击)
    threshold: u32,
    /// 已反击次数
    counterattack_count: u32,
}

impl ImmuneResponse {
    pub fn new(threshold: u32) -> Self {
        ImmuneResponse {
            events: Vec::new(),
            threshold,
            counterattack_count: 0,
        }
    }

    /// 记录事件
    pub fn record(&mut self, event: ImmuneEvent) {
        self.events.push(event);
    }

    /// 是否应该反击
    pub fn should_counterattack(&self) -> bool {
        self.events.len() as u32 >= self.threshold
    }

    /// 反击 — 触发系统回滚 + 分布式重构
    pub fn counterattack(&mut self) -> u32 {
        if self.should_counterattack() {
            self.counterattack_count += 1;
            self.counterattack_count
        } else {
            0
        }
    }

    /// 事件数
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// 反击次数
    pub fn counterattack_count(&self) -> u32 {
        self.counterattack_count
    }
}

impl Default for ImmuneResponse {
    fn default() -> Self {
        Self::new(3)
    }
}

/// 自寄生引擎 — 总协调
pub struct ParasiteEngine {
    /// 心跳监控
    pub monitor: HeartbeatMonitor,
    /// 自修复
    pub repair: SelfRepair,
    /// 免疫响应
    pub immune: ImmuneResponse,
    /// 系统健康度 (0.0 - 1.0)
    pub health: f64,
}

impl ParasiteEngine {
    pub fn new() -> Self {
        ParasiteEngine {
            monitor: HeartbeatMonitor::new(),
            repair: SelfRepair::new(),
            immune: ImmuneResponse::new(3),
            health: 1.0,
        }
    }

    /// 完整监控周期 — 心跳一拍
    pub fn heartbeat_cycle(&mut self) {
        self.monitor.tick();
        self.update_health();
    }

    /// 更新健康度
    fn update_health(&mut self) {
        let total = self.monitor.watch_count();
        if total == 0 {
            self.health = 1.0;
            return;
        }

        let alive = self.monitor.records.values()
            .filter(|r| r.status == HeartbeatStatus::Alive || r.status == HeartbeatStatus::Resurrected)
            .count();

        self.health = alive as f64 / total as f64;
    }

    /// 综合诊断 — 返回 (健康度, 状态字符串)
    pub fn diagnose(&self) -> (f64, &'static str) {
        if self.health >= 0.9 {
            (self.health, "健康")
        } else if self.health >= 0.5 {
            (self.health, "亚健康")
        } else if self.health > 0.0 {
            (self.health, "重症")
        } else {
            (self.health, "死亡")
        }
    }

    /// 蚕蛹完整迁移流程
    pub fn cocoon_migration(&mut self, target_address: &str) {
        // 1. 脱壳
        self.repair.molt();
        // 2. 假死
        self.repair.play_dead();
        // 3. 穿衣到目标
        let _ = self.repair.add_replica(2, target_address);
        self.repair.redress();
    }

    /// 重构: 从备份碎片重建状态
    pub fn reconstruct(&mut self) -> Vec<ContainerRef> {
        let mut restored = Vec::new();
        for i in 0..self.repair.replica_count() {
            let node_id = self.replicas()[i].node_id;
            let fragments = self.repair.repair_from(node_id);
            restored.extend(fragments);
        }
        restored
    }

    /// 获取复制节点引用
    pub fn replicas(&self) -> &Vec<ReplicaNode> {
        &self.repair.replicas
    }

    /// 是否存活
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

impl Default for ParasiteEngine {
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
    use crate::container_ir::ContainerKind;

    fn mk_ref(entity: u64) -> ContainerRef {
        ContainerRef::new(ContainerKind::Maori, entity)
    }

    #[test]
    fn heartbeat_record_new() {
        let r = HeartbeatRecord::new(mk_ref(100), 10);
        assert_eq!(r.status, HeartbeatStatus::Alive);
        assert!(!r.is_timeout(5));
    }

    #[test]
    fn heartbeat_record_timeout() {
        let mut r = HeartbeatRecord::new(mk_ref(100), 10);
        r.last_tick = 100;
        assert!(r.is_timeout(111)); // 111-100 > 10
        assert!(!r.is_timeout(105)); // 105-100 <= 10
    }

    #[test]
    fn heartbeat_record_beat_and_miss() {
        let mut r = HeartbeatRecord::new(mk_ref(100), 10);
        r.beat(50);
        assert_eq!(r.miss_count, 0);
        assert_eq!(r.status, HeartbeatStatus::Alive);

        r.miss();
        r.miss();
        assert_eq!(r.miss_count, 2);
        assert_eq!(r.status, HeartbeatStatus::Sluggish);

        r.miss();
        assert_eq!(r.status, HeartbeatStatus::Dead);
    }

    #[test]
    fn heartbeat_monitor_tick_and_beat() {
        let mut mon = HeartbeatMonitor::new();
        let target = mk_ref(200);
        mon.watch(target.clone(), 5);
        mon.beat(&target);

        assert_eq!(mon.status_of(&target), Some(HeartbeatStatus::Alive));
        assert_eq!(mon.watch_count(), 1);
    }

    #[test]
    fn heartbeat_monitor_tick_detects_death() {
        let mut mon = HeartbeatMonitor::new();
        let target = mk_ref(300);
        mon.watch(target.clone(), 3);

        // 拍 4 次 — 超过间隔 3
        for _ in 0..4 {
            mon.tick();
        }

        // 应该标记为 dead 或 sluggish
        let st = mon.status_of(&target).unwrap();
        assert!(st == HeartbeatStatus::Dead || st == HeartbeatStatus::Sluggish);
        assert!(!mon.all_alive());
    }

    #[test]
    fn heartbeat_monitor_dead_targets() {
        let mut mon = HeartbeatMonitor::new();
        let t1 = mk_ref(100);
        let t2 = mk_ref(200);
        mon.watch(t1.clone(), 2);
        mon.watch(t2.clone(), 2);

        // 不拍 Tick, 全部超时
        for _ in 0..5 {
            mon.tick();
        }

        let dead = mon.dead_targets();
        assert_eq!(dead.len(), 2);
    }

    #[test]
    fn cocoon_migration_changes_state() {
        let mut engine = ParasiteEngine::new();
        engine.cocoon_migration("new-host");
        // 最终应该回到 Cloaked
        match engine.repair.cocoon_state() {
            CocoonState::Cloaked => {},
            _ => panic!("Expected Cloaked"),
        }
    }

    #[test]
    fn self_repair_add_replica() {
        let mut repair = SelfRepair::new();
        let idx = repair.add_replica(1, "node-a");
        assert_eq!(idx, 0);
        assert_eq!(repair.replica_count(), 1);
    }

    #[test]
    fn self_repair_store_and_retrieve() {
        let mut repair = SelfRepair::new();
        repair.add_replica(1, "node-a");

        let fragment = mk_ref(0x6000_0000);
        repair.replicas[0].store_fragment(fragment.clone());

        let restored = repair.repair_from(1);
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].raw(), fragment.raw());
    }

    #[test]
    fn immune_response_tracks_events() {
        let mut immune = ImmuneResponse::new(2);
        assert!(!immune.should_counterattack());

        immune.record(ImmuneEvent::ForeignCode { source: "x".to_string() });
        assert!(!immune.should_counterattack());

        immune.record(ImmuneEvent::DeletionAttempt { target: mk_ref(100) });
        assert!(immune.should_counterattack());

        let count = immune.counterattack();
        assert_eq!(count, 1);
    }

    #[test]
    fn parasite_engine_health() {
        let mut engine = ParasiteEngine::new();
        assert!(engine.is_alive());
        assert!(engine.health > 0.9);
    }

    #[test]
    fn parasite_engine_diagnose_healthy() {
        let engine = ParasiteEngine::new();
        let (health, status) = engine.diagnose();
        assert!(health >= 0.9);
        assert_eq!(status, "健康");
    }

    #[test]
    fn parasite_engine_diagnose_critical() {
        let mut engine = ParasiteEngine::new();
        // 添加多个目标让它超时
        for i in 0..10 {
            engine.monitor.watch(mk_ref(i), 1);
        }
        for _ in 0..10 {
            engine.heartbeat_cycle();
        }

        let (_health, status) = engine.diagnose();
        assert!(status == "重症" || status == "死亡");
    }

    #[test]
    fn replica_store_no_duplicate() {
        let mut node = ReplicaNode::new(1, "host");
        let a = mk_ref(100);
        node.store_fragment(a.clone());
        node.store_fragment(a.clone()); // 重复
        assert_eq!(node.fragments.len(), 1);
    }

    #[test]
    fn reconstruct_from_multiple_replicas() {
        let mut engine = ParasiteEngine::new();
        engine.repair.add_replica(1, "host-a");
        engine.repair.add_replica(2, "host-b");

        let f1 = mk_ref(0x100);
        let f2 = mk_ref(0x200);
        engine.repair.replicas[0].store_fragment(f1.clone());
        engine.repair.replicas[1].store_fragment(f2.clone());

        let restored = engine.reconstruct();
        assert_eq!(restored.len(), 2);
    }
}
