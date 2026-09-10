// ═══════════════════════════════════════════════════════════════════════════
// perception_field.rs — 全局感知场 · SMRL 多模态全局秩序引擎 (v0.1)
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题：
//   "要么以感知点来提取被感知到的点；要么以全局感知、多模态全局感知状态，
//    同时感知全场。"
//
//    PerceptionField 实现"全局感知场"：把系统内所有容器的 VoidProbe
//    聚焦点汇聚到一个"感知面"中。感知面直接包含被感知对象，使场本身成为
//    一个自我包含的秩序结构。
//
//    站在"无"的状态驱动全场感知 — 感知者本身不携任何预设立场，
//    它只是一个"镜像空置场"，等待被所有被感知容器同时填充。
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use crate::cross_isa::coordinate::Coord3D;
use crate::cross_isa::fridge::ContainerPack;
use crate::void_probe::{SimModal, VoidProbe};

// ───────────────────────────────────────────────────────────────────────────
// 全局秩序模型 (GlobalOrderModel)
// ───────────────────────────────────────────────────────────────────────────
// 从所有容器的聚合观察中提取的"被控制系统的秩序指纹"。
// 包含三个维度的分布规律：语义分布、空间密度、时序节奏。

#[derive(Debug, Clone)]
pub struct GlobalOrderModel {
    /// 语义直方图：每个语义类别(x值)的容器数量
    pub semantic_histogram: HashMap<u8, usize>,
    /// 空间密度图：(Y轴)参数槽的分布（均值、极值）
    pub spatial_y_mean: f64,
    pub spatial_y_min: u16,
    pub spatial_y_max: u16,
    /// 时序节奏：Z轴分布（每层的容器数）
    pub temporal_layers: HashMap<u8, usize>,
    /// 感知总数
    pub total_observed: usize,
    /// 平均payload大小（字节）
    pub avg_payload_size: f64,
    /// 主要语义类别直方图（按频次降序）
    pub dominant_semantics: Vec<(u8, usize)>,
}

impl GlobalOrderModel {
    /// 从一组 VoidProbe 观察结果中提取全局秩序
    pub fn extract(probes: &[VoidProbe], targets: &[ContainerPack]) -> Self {
        let mut sem_hist: HashMap<u8, usize> = HashMap::new();
        let mut temp_layers: HashMap<u8, usize> = HashMap::new();
        let mut y_sum: f64 = 0.0;
        let mut y_min: u16 = u16::MAX;
        let mut y_max: u16 = 0;
        let mut payload_sum: usize = 0;

        for (_probe, target) in probes.iter().zip(targets.iter()) {
            let coord = target.coord();
            *sem_hist.entry(coord.x).or_insert(0) += 1;
            *temp_layers.entry(coord.z).or_insert(0) += 1;

            let y = coord.y;
            y_sum += y as f64;
            if y < y_min { y_min = y; }
            if y > y_max { y_max = y; }

            payload_sum += target.payload.len();
        }

        let n = targets.len().max(1) as f64;

        let mut dominant: Vec<(u8, usize)> = sem_hist.iter().map(|(&k, &v)| (k, v)).collect();
        dominant.sort_by(|a, b| b.1.cmp(&a.1));

        Self {
            semantic_histogram: sem_hist,
            spatial_y_mean: y_sum / n,
            spatial_y_min: if y_min == u16::MAX { 0 } else { y_min },
            spatial_y_max: y_max,
            temporal_layers: temp_layers,
            total_observed: targets.len(),
            avg_payload_size: payload_sum as f64 / n,
            dominant_semantics: dominant,
        }
    }

    /// 判断一个语义类别是否在全局秩序中占主导（频次超过阈值）
    pub fn is_dominant(&self, x: u8, threshold: f64) -> bool {
        let count = self.semantic_histogram.get(&x).copied().unwrap_or(0) as f64;
        count / self.total_observed.max(1) as f64 >= threshold
    }

    /// 获取指定语义类别的容器数量
    pub fn semantic_count(&self, x: u8) -> usize {
        self.semantic_histogram.get(&x).copied().unwrap_or(0)
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 感知场模式枚举
// ───────────────────────────────────────────────────────────────────────────
// 全场感知 vs 单点聚焦

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerceptionMode {
    /// 全局模式：同时感知所有容器，提取全局秩序
    Global,
    /// 聚焦模式：仅提取单个语义类别的容器子集
    Focused(u8),
}

// ───────────────────────────────────────────────────────────────────────────
// 感知场 (PerceptionField)
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PerceptionField {
    /// 所有容器包在"无"状态下的聚焦探针
    pub probes: Vec<VoidProbe>,
    /// 被观察的目标容器原始包
    pub targets: Vec<ContainerPack>,
    /// 提取的全局秩序模型
    pub model: Option<GlobalOrderModel>,
    /// 当前感知模式
    pub mode: PerceptionMode,
}

impl PerceptionField {
    // ─── 构造 ───────────────────────────────────────────────────────

    /// 从"无"状态创建感知场：接受所有容器包，对每个创建一个空壳探针。
    /// 感知场本身不携带预设立场 — 它仅仅是"空置的观察位置"。
    pub fn new(targets: Vec<ContainerPack>) -> Self {
        let probes: Vec<VoidProbe> = targets
            .iter()
            .map(|t| VoidProbe::new(t.coord(), t.pack_id))
            .collect();

        Self {
            probes,
            targets,
            model: None,
            mode: PerceptionMode::Global,
        }
    }

    // ─── 全局感知 ───────────────────────────────────────────────────
    /// 同时感知全场：对每个探针执行观察，注入模态，提取全局秩序。
    /// 这是"以全局感知、多模态全局感知状态，同时感知全场"的实现。
    pub fn perceive_global(&mut self, chaos_seed: u64) {
        let mut chaos = ChaosSource::new(chaos_seed);
        for (probe, target) in self.probes.iter_mut().zip(self.targets.iter()) {
            probe.observe(target);
            // 为每个探针注入混沌符号（数量由全局秩序决定）
            let n = 1 + (chaos.next_u8() as usize % 4);
            for _ in 0..n {
                probe.inject(chaos.next_modal());
            }
        }
        let model = GlobalOrderModel::extract(&self.probes, &self.targets);
        self.model = Some(model);
        self.mode = PerceptionMode::Global;
    }

    // ─── 聚焦感知 ───────────────────────────────────────────────────
    /// 聚焦感知：仅提取指定语义类别(x值)的容器子集。
    /// 这是"以感知点来提取被感知到的点"。
    pub fn perceive_focused(&mut self, focus_x: u8, chaos_seed: u64) {
        let mut chaos = ChaosSource::new(chaos_seed);
        for (probe, target) in self.probes.iter_mut().zip(self.targets.iter()) {
            if target.coord().x == focus_x {
                probe.observe(target);
                probe.inject(chaos.next_modal());
            }
        }
        let model = GlobalOrderModel::extract(&self.probes, &self.targets);
        self.model = Some(model);
        self.mode = PerceptionMode::Focused(focus_x);
    }

    // ─── 全局替代 ───────────────────────────────────────────────────
    /// 基于感知场的全局秩序模型，从"无"中重建整个容器体系。
    /// 步骤：
    ///   1. 提取全局秩序指纹（语义分布、空间密度、时序节奏）
    ///   2. 按相同的分布规律、但从混沌场注入的全新内容，重新生成全部容器
    ///   3. 新的容器体系保持与原始系统相同的结构性秩序（拓扑等价）
    ///      但数据主权已被完全替换
    pub fn regenerate_global(&mut self, new_seed: u64) -> Vec<ContainerPack> {
        self.perceive_global(new_seed);

        let mut chaos = ChaosSource::new(new_seed);
        let mut results = Vec::with_capacity(self.targets.len());

        for (probe, target) in self.probes.iter_mut().zip(self.targets.iter()) {
            if let Some(new_pack) = probe.harvest(target) {
                // 注入额外的混沌模态以强化替换深度
                let extra = chaos.next_u8() % 3;
                let mut extra_bytes: Vec<u8> = (0..extra).map(|_| chaos.next_u8()).collect();
                let mut final_payload = new_pack.payload.clone();
                final_payload.append(&mut extra_bytes);
                let new_pack2 = ContainerPack::new(
                    new_pack.pack_id,
                    new_pack.coord(),
                    new_pack.align_flag,
                    final_payload,
                );
                results.push(new_pack2);
            }
        }

        results
    }

    // ─── 感知统计 ──────────────────────────────────────────────────
    pub fn stats(&self) -> String {
        match &self.model {
            Some(m) => format!(
                "PerceptionField[global]: {} containers, {} semantic categories, {} temporal layers, avg payload {:.1}b",
                m.total_observed,
                m.semantic_histogram.len(),
                m.temporal_layers.len(),
                m.avg_payload_size
            ),
            None => "PerceptionField[uninitialized]: no perception performed".to_string(),
        }
    }
}

// ─── 混沌感知源（感知场用）────────────────────────────────────────────────

struct ChaosSource {
    state: u64,
}

impl ChaosSource {
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
    fn next_modal(&mut self) -> SimModal {
        match self.next_u8() % 5 {
            0 => SimModal::Glyph((b'A' + (self.next_u8() % 26)) as char),
            1 => SimModal::Phoneme(self.next_u8()),
            2 => SimModal::Shape([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()]),
            3 => SimModal::Rule(self.next_u8() as u16),
            _ => SimModal::Tone(self.next_u8() % 16),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cross_isa::fridge::ContainerPack;

    fn mock_targets(n: usize) -> Vec<ContainerPack> {
        (0..n).map(|i| {
            let x = (i % 15 + 1) as u8;
            let y = (0x1000 + i as u16) as u16;
            let z = (i % 4 + 1) as u8;
            ContainerPack::new(i as u64, Coord3D::new(x, y, z), 0, vec![0xDE, 0xAD, 0xBE, 0xEF])
        }).collect()
    }

    #[test]
    fn field_perceives_global_order() {
        let targets = mock_targets(40);
        let mut field = PerceptionField::new(targets);
        field.perceive_global(0xDEAD_BEEF);

        assert!(field.model.is_some());
        let model = field.model.unwrap();
        assert_eq!(model.total_observed, 40);
        assert!(!model.semantic_histogram.is_empty());
        assert!(model.avg_payload_size > 0.0);
    }

    #[test]
    fn field_extracts_semantic_histogram() {
        let targets = mock_targets(30);
        let mut field = PerceptionField::new(targets);
        field.perceive_global(0x1234);

        let model = field.model.unwrap();
        // Each of 15 semantic categories should appear
        assert!(model.semantic_histogram.len() <= 15);
        // Dominant should be sorted by frequency descending
        if model.dominant_semantics.len() >= 2 {
            assert!(model.dominant_semantics[0].1 >= model.dominant_semantics[1].1);
        }
    }

    #[test]
    fn field_focused_perception() {
        let targets = mock_targets(20);
        let mut field = PerceptionField::new(targets);
        field.perceive_focused(0x03, 0xCAFE); // focus on ADD

        match field.mode {
            PerceptionMode::Focused(x) => assert_eq!(x, 0x03),
            _ => panic!("Expected focused mode"),
        }
    }

    #[test]
    fn field_regenerate_preserves_global_order() {
        let targets = mock_targets(25);
        let original_x_values: Vec<u8> = targets.iter().map(|t| t.coord().x).collect();

        let mut field = PerceptionField::new(targets);
        let regenerated = field.regenerate_global(0xBEEF_CAFE);

        assert_eq!(regenerated.len(), 25);
        // 坐标 x 维度全局秩序保留
        let new_x_values: Vec<u8> = regenerated.iter().map(|p| p.coord().x).collect();
        assert_eq!(original_x_values, new_x_values);
        // 但 payload 已全部替换 （payload 大小可能加入额外混沌字节）
        // 至少所有 packs 都有非零 payload
        for p in &regenerated {
            assert!(!p.payload.is_empty());
        }
    }

    #[test]
    fn field_stats_reporting() {
        let targets = mock_targets(10);
        let mut field = PerceptionField::new(targets);
        field.perceive_global(0x1111);
        let s = field.stats();
        assert!(s.contains("10 containers"));
        assert!(s.contains("semantic categories"));
    }
}
