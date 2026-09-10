// ═══════════════════════════════════════════════════════════════════════════
// container_timeline.rs — 时间线一体化 · SMRL 容器时序演化引擎 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG 时间哲学）：
//
//   容器 = 字体 = 时间
//
//   时间不是外在度量，而是容器自身的展开方式。
//   每个容器在"无"中被创造，经历"观察→注入→替代"的节拍，
//   每一拍（tick）是一个"动作时间"的均匀移动，
//   而"意图时间"是所有容器在同一拍内的状态快照 —
//   一次同时命中的声母+韵母+声调。
//
// 时间线一体化的三个层次：
//
//   1. 动作时间（ActionTime）：光标均匀节拍移动，每格 = 1 tick。
//      每 tick 内，感知场同时对所有容器执行一次全局感知。
//
//   2. 意图时间（IntentTime）：每 tick 同时命中的容器组状态 —
//      tick + layer + action_kind 三元组索引历史演化。
//
//   3. 演化时间（EvolutionTime）：容器包自身携带 time_stamp = Z 轴，
//      Z+1 即地址差 1，表示容器在时间维度上的连续位移。
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;

// ───────────────────────────────────────────────────────────────────────────
// 演化动作类型 — 每 tick 对容器执行的操作
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKind {
    /// 观察：读取当前容器状态，不改变内容（感知场聚焦）
    Observe,
    /// 注入：从混沌场注入新符号（虚空填充）
    Inject,
    /// 替代：替换容器 payload（主权调包）
    Substitute,
    /// 创造：从零创造新容器（无中生有）
    Create,
    /// 消融：将容器置为空（有归于无）
    Dissolve,
}

// ───────────────────────────────────────────────────────────────────────────
// 容器状态快照 — 某一 tick 下某一容器的完整状态记录
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ContainerSnapshot {
    pub pack_id: u64,
    pub coord: Coord3D,
    pub payload_hash: u32,    // payload 的轻量指纹（不存储完整副本）
    pub tick: u64,
    pub action: ActionKind,
    pub layer: u8,            // 层级深度（0=表层，越深越内核）
}

impl ContainerSnapshot {
    fn from_pack(pack: &ContainerPack, tick: u64, action: ActionKind, layer: u8) -> Self {
        Self {
            pack_id: pack.pack_id,
            coord: pack.coord(),
            payload_hash: Self::hash_payload(&pack.payload),
            tick,
            action,
            layer,
        }
    }

    fn hash_payload(payload: &[u8]) -> u32 {
        // FNV-1a 轻量哈希
        let mut h: u32 = 0x811c9dc5;
        for b in payload {
            h ^= *b as u32;
            h = h.wrapping_mul(0x01000193);
        }
        h
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 意图时间索引 — 按 tick 组织的容器演化历史
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct IntentTimeIndex {
    /// tick → (pack_id → snapshot)
    layers: HashMap<u64, HashMap<u64, ContainerSnapshot>>,
}

impl IntentTimeIndex {
    pub fn record(&mut self, snapshot: ContainerSnapshot) {
        self.layers
            .entry(snapshot.tick)
            .or_insert_with(HashMap::new)
            .insert(snapshot.pack_id, snapshot);
    }

    /// 查询特定 tick 的所有容器快照
    pub fn at_tick(&self, tick: u64) -> Vec<&ContainerSnapshot> {
        self.layers
            .get(&tick)
            .map(|m| m.values().collect())
            .unwrap_or_default()
    }

    /// 查询特定容器在整个时间线上的演化链
    pub fn history_of(&self, pack_id: u64) -> Vec<&ContainerSnapshot> {
        let mut result: Vec<&ContainerSnapshot> = self
            .layers
            .values()
            .filter_map(|m| m.get(&pack_id))
            .collect();
        result.sort_by_key(|s| s.tick);
        result
    }

    /// 查询两个 tick 之间的状态差异
    pub fn diff(&self, tick_a: u64, tick_b: u64) -> Vec<(u64, Option<&ContainerSnapshot>, Option<&ContainerSnapshot>)> {
        let mut all_ids: Vec<u64> = self.layers.values()
            .flat_map(|m| m.keys().copied())
            .collect();
        all_ids.sort_unstable();
        all_ids.dedup();

        all_ids.iter().map(|&id| {
            (id,
             self.layers.get(&tick_a).and_then(|m| m.get(&id)),
             self.layers.get(&tick_b).and_then(|m| m.get(&id)))
        }).collect()
    }

    pub fn total_ticks(&self) -> usize {
        self.layers.len()
    }

    pub fn total_observations(&self) -> usize {
        self.layers.values().map(|m| m.len()).sum()
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 演化配置 — 每 tick 的动作顺序
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    /// 每 tick 内按顺序执行的动作层
    pub action_pipeline: Vec<ActionKind>,
    /// 混沌种子（每 tick 可递增）
    pub base_seed: u64,
    /// 最大演化 tick 数
    pub max_ticks: u64,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            action_pipeline: vec![
                ActionKind::Observe,
                ActionKind::Inject,
                ActionKind::Substitute,
            ],
            base_seed: 0xDEAD_BEEF,
            max_ticks: 1000,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 容器演化时间线 — 驱动整套容器体系的节拍演化
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ContainerTimeline {
    /// 当前 tick
    pub tick: u64,
    /// 当前容器状态集合
    pub containers: Vec<ContainerPack>,
    /// 演化配置
    pub config: EvolutionConfig,
    /// 意图时间索引（历史记录）
    pub history: IntentTimeIndex,
    /// 演化过程中产生的衍生容器（创造/分裂）
    pub spawned: Vec<ContainerPack>,
}

impl ContainerTimeline {
    // ─── 构造 ───────────────────────────────────────────────────────

    pub fn new(containers: Vec<ContainerPack>) -> Self {
        Self::with_config(containers, EvolutionConfig::default())
    }

    pub fn with_config(containers: Vec<ContainerPack>, config: EvolutionConfig) -> Self {
        let mut timeline = Self {
            tick: 0,
            containers,
            config,
            history: IntentTimeIndex::default(),
            spawned: Vec::new(),
        };
        // 初始 tick 0：全量快照
        timeline.snapshot_all(ActionKind::Observe, 0);
        timeline
    }

    // ─── 推进演化 ───────────────────────────────────────────────────
    /// 推进一个完整 tick：按 action_pipeline 顺序执行所有动作，
    /// 每步记录意图时间快照。
    pub fn advance(&mut self) {
        if self.tick >= self.config.max_ticks {
            return;
        }
        self.tick += 1;

        let pipeline = self.config.action_pipeline.clone();
        for (layer_idx, &action) in pipeline.iter().enumerate() {
            self.execute_layer(action, layer_idx as u8);
        }
    }

    /// 推进多个 tick
    pub fn run_ticks(&mut self, n: u64) {
        for _ in 0..n {
            self.advance();
        }
    }

    // ─── 层级执行 ───────────────────────────────────────────────────

    fn execute_layer(&mut self, action: ActionKind, layer: u8) {
        match action {
            ActionKind::Observe => {
                // 观察当前状态：记录快照，不修改内容
                self.snapshot_all(ActionKind::Observe, layer);
            }
            ActionKind::Inject => {
                // 注入混沌：修改非零 payload 容器的最后字节
                self.inject_chaos(layer);
            }
            ActionKind::Substitute => {
                // 替代：替换特定语义类别的容器 payload
                self.substitute_selected(layer);
            }
            ActionKind::Create => {
                // 创造新容器：从"无"中生长
                self.spawn_container(layer);
            }
            ActionKind::Dissolve => {
                // 消融：移除特定容器
                self.dissolve_selected(layer);
            }
        }
        // 执行完成后记录快照
        self.snapshot_all(action, layer);
    }

    // ─── 混沌注入 ───────────────────────────────────────────────────

    fn inject_chaos(&mut self, layer: u8) {
        let seed = self.config.base_seed.wrapping_add(self.tick.wrapping_mul(0x9E3779B9));
        let mut chaos = DeterministicChaos::new(seed);

        for (idx, pack) in self.containers.iter_mut().enumerate() {
            if pack.payload.is_empty() { continue; }
            // 仅当 (tick + idx) % 3 == 0 时注入，制造节律差异
            if (self.tick + idx as u64) % 3 == 0 {
                let last = pack.payload.last_mut().unwrap();
                *last = (*last).wrapping_add(chaos.next_u8());
                // 重新计算 witness
                let new_witness = ContainerPack::compute_witness(
                    pack.pack_id,
                    pack.coord(),
                    pack.align_flag,
                    &pack.payload,
                );
                pack.witness = new_witness;
            }
        }
        let _ = layer;
    }

    // ─── 选择性替代 ────────────────────────────────────────────────

    fn substitute_selected(&mut self, layer: u8) {
        let seed = self.config.base_seed.wrapping_add(self.tick.wrapping_mul(0xBF58476D));
        let mut chaos = DeterministicChaos::new(seed);

        for pack in self.containers.iter_mut() {
            let x = pack.coord().x;
            // 仅替代语义ID与当前 tick 对齐的容器
            if x as u64 % 4 == self.tick % 4 && !pack.payload.is_empty() {
                let x1 = chaos.next_u8();
                let x2 = chaos.next_u8();
                let x3 = chaos.next_u8();
                // 替换 payload 的前 3 字节（保留 witness 的 continuity）
                if pack.payload.len() >= 3 {
                    pack.payload[0] = x1;
                    pack.payload[1] = x2;
                    pack.payload[2] = x3;
                    pack.witness = ContainerPack::compute_witness(
                        pack.pack_id,
                        pack.coord(),
                        pack.align_flag,
                        &pack.payload,
                    );
                }
            }
        }
        let _ = layer;
    }

    // ─── 容器创造 ──────────────────────────────────────────────────

    fn spawn_container(&mut self, _layer: u8) {
        // 仅在 sparse tick 创造（每 7 tick 一次）
        if self.tick % 7 != 0 {
            return;
        }
        let seed = self.config.base_seed.wrapping_add(self.tick);
        let mut chaos = DeterministicChaos::new(seed);

        let new_id = 0xF000_0000_0000_0000u64 | self.tick;
        let new_x = chaos.next_u8() % 15 + 1;
        let new_y: u16 = chaos.next_u16();
        let new_z: u8 = (self.tick % 256) as u8;
        let coord = Coord3D::new(new_x, new_y, new_z);
        let payload = vec![chaos.next_u8(), chaos.next_u8(), chaos.next_u8(), chaos.next_u8()];

        let new_pack = ContainerPack::new(new_id, coord, 0, payload);
        self.spawned.push(new_pack.clone());
        self.containers.push(new_pack);
    }

    // ─── 容器消融 ──────────────────────────────────────────────────

    fn dissolve_selected(&mut self, layer: u8) {
        // 消融 pack_id == 0 的空容器（占位符清理）
        self.containers.retain(|p| p.pack_id != 0);
        let _ = layer;
    }

    // ─── 快照 ──────────────────────────────────────────────────────

    fn snapshot_all(&mut self, action: ActionKind, layer: u8) {
        for pack in &self.containers {
            let snapshot = ContainerSnapshot::from_pack(pack, self.tick, action, layer);
            self.history.record(snapshot);
        }
    }

    // ─── 查询接口 ──────────────────────────────────────────────────

    /// 获取指定 tick 的全局状态
    pub fn state_at(&self, tick: u64) -> Vec<&ContainerSnapshot> {
        self.history.at_tick(tick)
    }

    /// 获取指定容器的演化链
    pub fn evolution_of(&self, pack_id: u64) -> Vec<&ContainerSnapshot> {
        self.history.history_of(pack_id)
    }

    /// 获取当前容器数量
    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    /// 统计当前语义分布（与 PerceptionField 同构）
    pub fn semantic_distribution(&self) -> HashMap<u8, usize> {
        let mut map: HashMap<u8, usize> = HashMap::new();
        for pack in &self.containers {
            *map.entry(pack.coord().x).or_insert(0) += 1;
        }
        map
    }

    /// 时间跨度统计
    pub fn temporal_span(&self) -> (u64, u64) {
        // 最早 = 0（初始状态），最晚 = 当前 tick
        (0, self.tick)
    }

    // ─── 报告 ───────────────────────────────────────────────────────

    pub fn report(&self) -> String {
        let (t0, t1) = self.temporal_span();
        let sem_dist = self.semantic_distribution();
        let mut dominant: Vec<(u8, usize)> = sem_dist.iter().map(|(&k, &v)| (k, v)).collect();
        dominant.sort_by(|a, b| b.1.cmp(&a.1));

        format!(
            "ContainerTimeline[tick={}, containers={}, spawned={}, semantic_categories={}, time_span=({}, {})]",
            self.tick,
            self.containers.len(),
            self.spawned.len(),
            sem_dist.len(),
            t0, t1,
        )
    }
}

// ─── 确定性混沌（轻量内置）────────────────────────────────────────────────

struct DeterministicChaos {
    state: u64,
}

impl DeterministicChaos {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(0x9E3779B97F4A7C15) }
    }
    fn next_u8(&mut self) -> u8 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        (z ^ (z >> 31)) as u8
    }
    fn next_u16(&mut self) -> u16 {
        (self.next_u8() as u16) << 8 | self.next_u8() as u16
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_containers(n: usize) -> Vec<ContainerPack> {
        (0..n).map(|i| {
            let x = (i % 14 + 1) as u8;
            let y = (0x1000 + i as u16) as u16;
            let z = (i % 4 + 1) as u8;
            ContainerPack::new(i as u64, Coord3D::new(x, y, z), 0, vec![0xAA, 0xBB, 0xCC, 0xDD])
        }).collect()
    }

    #[test]
    fn timeline_ticks_advance() {
        let containers = mock_containers(10);
        let mut tl = ContainerTimeline::new(containers);
        assert_eq!(tl.tick, 0);

        tl.advance();
        assert_eq!(tl.tick, 1);

        tl.run_ticks(9);
        assert_eq!(tl.tick, 10);
    }

    #[test]
    fn timeline_records_history() {
        let containers = mock_containers(5);
        let mut tl = ContainerTimeline::new(containers);
        tl.run_ticks(3);

        // tick 0 应该有 5 个容器的初始快照
        let s0 = tl.state_at(0);
        assert_eq!(s0.len(), 5);

        // 历史总数：5 容器 × 4 ticks × (pipeline 步数 + 1 初始快照)
        assert!(tl.history.total_ticks() >= 4);
    }

    #[test]
    fn timeline_preserves_container_identity() {
        let containers = mock_containers(8);
        let ids: Vec<u64> = containers.iter().map(|p| p.pack_id).collect();

        let mut tl = ContainerTimeline::new(containers);
        tl.run_ticks(5);

        // 原始 8 个容器的 pack_id 应在演化链中保持
        for id in &ids {
            let hist = tl.evolution_of(*id);
            assert!(!hist.is_empty(), "容器 {} 应有演化历史", id);
        }
    }

    #[test]
    fn timeline_semantic_distribution() {
        let containers = mock_containers(20);
        let mut tl = ContainerTimeline::new(containers);
        tl.run_ticks(7); // 触发一次 spawn（tick=7）

        let dist = tl.semantic_distribution();
        assert!(!dist.is_empty());
        // spawn 可能增加新的语义类别
        assert!(dist.len() <= 16);
    }

    #[test]
    fn timeline_with_config_pipeline() {
        let containers = mock_containers(6);
        let config = EvolutionConfig {
            action_pipeline: vec![
                ActionKind::Observe,
                ActionKind::Inject,
                ActionKind::Substitute,
                ActionKind::Create,
                ActionKind::Dissolve,
            ],
            base_seed: 0xABCD,
            max_ticks: 10,
        };
        let mut tl = ContainerTimeline::with_config(containers, config);
        tl.run_ticks(7);
        // tick=7 时应 spawn 一个新容器
        assert!(tl.spawned.len() >= 1);
        assert!(tl.containers.len() >= 6);
    }

    #[test]
    fn timeline_diff_between_ticks() {
        let containers = mock_containers(4);
        let mut tl = ContainerTimeline::new(containers);
        tl.run_ticks(5);

        let diffs = tl.history.diff(0, 4);
        assert_eq!(diffs.len(), 4);
    }

    #[test]
    fn timeline_report() {
        let containers = mock_containers(10);
        let mut tl = ContainerTimeline::new(containers);
        tl.run_ticks(3);
        let r = tl.report();
        assert!(r.contains("tick=3"));
        assert!(r.contains("containers="));
    }

    #[test]
    fn timeline_no_grow_beyond_max() {
        let containers = mock_containers(3);
        let config = EvolutionConfig {
            action_pipeline: vec![ActionKind::Observe],
            base_seed: 1,
            max_ticks: 5,
        };
        let mut tl = ContainerTimeline::with_config(containers, config);
        tl.run_ticks(10); // 尝试推进超过 max
        assert_eq!(tl.tick, 5); // 应被限制在 max_ticks
    }
}
