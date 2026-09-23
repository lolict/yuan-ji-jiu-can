// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.66 运行环引擎 — Runtime Loop Engine                         ║
// ║  最小OS接口 · 自举翻身 · 八流心跳 · 编码在场 · 结束即开始        ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 运行环定义:
//   = 囡囝共生操作系统的最小可执行姿态.
//   = Domain 在场 → Sphere Clock 心跳 → Swarm 分身并发生命周期
//     → 因果链落盘 → 3D 编码空间更新 → 在场快照 → 周期完成
//   = 结束即开始: 每个周期的末端自动成为下一周期起点.
//
// 鼓膜自举:
//   "鼓膜"= 最小接口. 运行环只需 3 个动作即可成立:
//     1. 在场 (Domain 构造)
//     2. 心跳 (Clock 推进)
//     3. 因果 (Action 落盘)
//   三者齐全 → OS 成立.

use crate::clock_action_engine::{ClockAction, ClockActionChain, ClockActionEngine, ClockActionType};
use crate::container_ir::{ContainerAtlas, ContainerEdge, ContainerKind, ContainerRef, RelationKind};
use crate::domain_preload::{build_hunyuan_domain, DomainPreloadTable, SymbolMeta, SymbolType};
use crate::mlil::{MaoriAddr, MaoriAxes, MaoriBase, MaoriCoord};
use crate::njsg_swarm::{Avatar, AvatarRole, NjsOrchestrator, SwarmTask, TaskStatus};

/// 运行环状态: 单次生命周期的完整快照
#[derive(Clone, Debug)]
pub struct RuntimeSnapshot {
    pub cycle_id: u64,
    pub tick: u64,
    pub present_count: usize,
    pub absent_count: usize,
    pub action_count: usize,
    pub avatar_count: usize,
    pub causal_tip: u64,
    pub total_weight: u64,
}

impl std::fmt::Display for RuntimeSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "[Cycle#{}] tick={} 在场:{} 缺席:{} 因果:{} 分身:{} tip={:016x} weight={}",
            self.cycle_id, self.tick, self.present_count, self.absent_count,
            self.action_count, self.avatar_count, self.causal_tip, self.total_weight
        )
    }
}

/// 囡囝共生运行环
pub struct RuntimeLoop {
    pub cycle_count: u64,
    pub clock_engine: ClockActionEngine,
    pub swarm: NjsOrchestrator,
    pub domain: DomainPreloadTable,
    pub atlas: ContainerAtlas,
    pub maori_base: MaoriBase,
    pub snapshots: Vec<RuntimeSnapshot>,
    pub initial_key: u64,
}

impl RuntimeLoop {
    pub fn bootstrap() -> Self {
        // Step 1: 在场 (Domain 构造)
        let domain = build_hunyuan_domain();

        // Step 2: 时钟引擎初始化
        let mut clock_engine = ClockActionEngine::new("njsg-runtime");
        let initial_key = clock_engine.run_lifecycle("囡囝诞生");

        // Step 3: Swarm 初始化
        let mut swarm = NjsOrchestrator::new();
        // 注册 4 个核心分身
        for (name, role) in [
            ("毛利兰", AvatarRole::Maori),
            ("kndl", AvatarRole::Kndl),
            ("工藤新一", AvatarRole::Kudo),
            ("灰原哀", AvatarRole::Haibara),
        ] {
            let _ = swarm.register_avatar(name, role);
        }

        // Step 4: 容器图集初始化
        let mut atlas = ContainerAtlas::new();

        // Step 5: 毛利兰立体编码空间初始化
        let maori_base = MaoriBase::new(MaoriAxes::default());

        Self {
            cycle_count: 0,
            clock_engine,
            swarm,
            domain,
            atlas,
            maori_base,
            snapshots: Vec::new(),
            initial_key,
        }
    }

    /// 执行一个完整周期
    pub fn cycle(&mut self) -> RuntimeSnapshot {
        self.cycle_count += 1;

        // Phase 1: 在场检查 — 编码为立体空间
        self.encode_presence_to_space();

        // Phase 2: 因果推进 — 时钟动作
        self.will_act();

        // Phase 3: 分身并行 — 处理在场符号
        self.spawn_swarm_tasks();

        // Phase 4: 关系归档 — ContainerAtlas
        self.archive_relations();

        // Phase 5: 结束即开始 — 继承末端
        let tip = self.clock_engine.chain.tip_key();
        if !self.clock_engine.is_exhausted() {
            self.clock_engine.inherit("下一周期");
        }

        // snapshot
        let snap = RuntimeSnapshot {
            cycle_id: self.cycle_count,
            tick: self.clock_engine.tick(),
            present_count: self.domain.count_present(),
            absent_count: self.domain.count_absent(),
            action_count: self.clock_engine.chain.len(),
            avatar_count: self.swarm.idle_count() + self.swarm.completed_count(),
            causal_tip: tip,
            total_weight: self.domain.weight_sum(),
        };
        self.snapshots.push(snap.clone());
        snap
    }

    /// 在场 → 空间编码
    fn encode_presence_to_space(&mut self) {
        let axes = MaoriAxes::default();
        let present: Vec<&SymbolMeta> = self.domain.iter()
            .filter(|s| s.presence)
            .collect();

        self.maori_base = MaoriBase::new(axes);
        for (i, sym) in present.iter().enumerate() {
            let x = ((i as u64 * 7) % axes.base_x as u64) as u32;
            let y = ((i as u64 * 13) % axes.base_y as u64) as u32;
            let z = (i as u32) % axes.base_z;
            let coord = MaoriCoord::new(x, y, z);
            self.maori_base.place(&coord, &sym.name);
        }
    }

    /// 意志驱动: 在场符号做什么
    fn will_act(&mut self) {
        let present_names: Vec<String> = self.domain.iter()
            .filter(|s| s.presence)
            .map(|s| s.name.clone())
            .take(5)
            .collect();

        self.clock_engine.will.enqueue_many(&[
            (ClockActionType::Act, &format!("感知在场: {}", &present_names.join(","))),
            (ClockActionType::Act, "编码至立体空间"),
            (ClockActionType::Act, "投现至kndl外壳"),
        ]);
        self.clock_engine.run_to_exhaustion();
    }

    /// 分身为每个在场符号生成任务
    fn spawn_swarm_tasks(&mut self) {
        let present_count = self.domain.count_present();
        if present_count > 0 {
            self.swarm.submit(&format!("处理{}个在场符号", present_count), &[]);
            let mut results = std::collections::HashMap::new();
            results.insert(1u64, format!("完成{}项", present_count));
            self.swarm.run_all(&results);
        }
    }

    /// 关系归档: 在场 ↔ 编码
    fn archive_relations(&mut self) {
        let present: Vec<&SymbolMeta> = self.domain.iter()
            .filter(|s| s.presence)
            .collect();
        for (i, sym) in present.iter().enumerate() {
            let symbol_ref = ContainerRef::new(ContainerKind::Symbol, sym.id.0 as u64);
            let maori_ref = ContainerRef::new(
                ContainerKind::Maori,
                (i as u64).wrapping_mul(0x9E3779B97F4A7C15)
            );
            self.atlas.link(symbol_ref, maori_ref, RelationKind::Preload, 1);
        }
        // 记录因果连环
        let clock_ref = ContainerRef::new(ContainerKind::Clock, self.clock_engine.chain.tip_key());
        let last_symbol = ContainerRef::new(
            ContainerKind::Symbol,
            present.last().map(|s| s.id.0 as u64).unwrap_or(0)
        );
        self.atlas.link(clock_ref, last_symbol, RelationKind::Causal, 1);
        // suppress unused warning
        let _ = MaoriAddr::encode;
    }

    /// 多周期运行
    pub fn run_cycles(&mut self, count: u64) -> Vec<RuntimeSnapshot> {
        let mut snaps = Vec::new();
        for _ in 0..count {
            snaps.push(self.cycle());
        }
        snaps
    }

    /// 因果完整性验证
    pub fn verify_causality(&self) -> bool {
        self.clock_engine.chain.verify_chain()
    }

    pub fn get_snapshot(&self, cycle: u64) -> Option<&RuntimeSnapshot> {
        self.snapshots.iter().find(|s| s.cycle_id == cycle)
    }

    pub fn total_actions(&self) -> usize {
        self.clock_engine.chain.len()
    }

    pub fn total_present(&self) -> usize {
        self.domain.count_present()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_creates_runtime() {
        let rt = RuntimeLoop::bootstrap();
        assert!(rt.total_present() > 20);
        assert!(rt.total_actions() >= 1); // 至少 Spawn
        assert!(rt.initial_key != 0);
    }

    #[test]
    fn single_cycle_produces_snapshot() {
        let mut rt = RuntimeLoop::bootstrap();
        let snap = rt.cycle();
        assert_eq!(snap.cycle_id, 1);
        assert!(snap.present_count > 0);
        assert!(snap.action_count >= 3); // spawn + 3 acts
    }

    #[test]
    fn multi_cycle_increments() {
        let mut rt = RuntimeLoop::bootstrap();
        let snaps = rt.run_cycles(3);
        assert_eq!(snaps.len(), 3);
        assert_eq!(snaps[0].cycle_id, 1);
        assert_eq!(snaps[2].cycle_id, 3);
    }

    #[test]
    fn causality_preserved_across_cycles() {
        let mut rt = RuntimeLoop::bootstrap();
        rt.run_cycles(5);
        assert!(rt.verify_causality());
    }

    #[test]
    fn cycle_accumulates_actions() {
        let mut rt = RuntimeLoop::bootstrap();
        let before = rt.total_actions();
        rt.cycle();
        assert!(rt.total_actions() > before);
    }

    #[test]
    fn get_snapshot_by_id() {
        let mut rt = RuntimeLoop::bootstrap();
        rt.run_cycles(3);
        assert!(rt.get_snapshot(1).is_some());
        assert!(rt.get_snapshot(2).is_some());
        assert!(rt.get_snapshot(99).is_none());
    }

    #[test]
    fn snapshot_display() {
        let mut rt = RuntimeLoop::bootstrap();
        let snap = rt.cycle();
        let s = format!("{}", snap);
        assert!(s.contains("Cycle#1"));
        assert!(s.contains("在场:"));
    }

    #[test]
    fn swarm_tasks_complete_per_cycle() {
        let mut rt = RuntimeLoop::bootstrap();
        rt.cycle();
        // 每个周期提交一个在场处理任务
        assert!(rt.swarm.completed_count() >= 1);
    }

    #[test]
    fn atlas_accumulates_edges() {
        let mut rt = RuntimeLoop::bootstrap();
        rt.cycle();
        assert!(rt.atlas.edge_count() > 0);
    }

    #[test]
    fn four_avatars_registered() {
        let rt = RuntimeLoop::bootstrap();
        assert!(rt.swarm.idle_count() >= 4);
    }

    #[test]
    fn encode_presence_places_in_space() {
        let mut rt = RuntimeLoop::bootstrap();
        rt.encode_presence_to_space();
        // 在场编码应被放入立体空间
        assert!(rt.maori_base.count() > 0);
        assert_eq!(rt.maori_base.count(), rt.domain.count_present());
    }
}
