// ╔══════════════════════════════════════════════════════════════════╗
// ║  v0.69 距离感知引擎                                              ║
// ║  聂小倩匹配 · 加权距离 · 聚类分析 · 本质匹配                      ║
// ║                                                                  ║
// ║  距离公式 (mind_organ):                                          ║
// ║  distance = |Δ家族|×1.0 + |Δ极性|×10.0 + |Δ结构|×5.0             ║
// ║           + |Δ声母|×0.5 + |Δ韵母|×0.5 + |Δ声调|×0.1               ║
// ║                                                                  ║
// ║  聂小倩: 对全部候选躯壳计算加权距离, 选最近的                      ║
// ╚══════════════════════════════════════════════════════════════════╝

use crate::container_ir::{ContainerAtlas, ContainerRef, RelationKind};

/// 容器码字段提取器 (32 位容器码)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContainerCode {
    pub code: u32,
}

impl ContainerCode {
    pub fn new(code: u32) -> Self {
        ContainerCode { code }
    }

    /// 从各字段组装容器码
    pub fn compose(family: u8, polarity: u8, structure: u8, shengmu: u8, yunmu: u8, tone: u8) -> Self {
        let code = ((family as u32 & 0x7) << 29)
            | ((polarity as u32 & 0x1) << 28)
            | ((structure as u32 & 0x7) << 25)
            | ((shengmu as u32 & 0xF) << 19)
            | ((yunmu as u32 & 0x3F) << 13)
            | ((tone as u32 & 0x7) << 10);
        ContainerCode { code }
    }

    /// 家族 [31:29]
    pub fn family(&self) -> u8 {
        ((self.code >> 29) & 0x7) as u8
    }

    /// 极性 [28]
    pub fn polarity(&self) -> u8 {
        ((self.code >> 28) & 0x1) as u8
    }

    /// 结构 [27:25]
    pub fn structure(&self) -> u8 {
        ((self.code >> 25) & 0x7) as u8
    }

    /// 声母 [22:19]
    pub fn shengmu(&self) -> u8 {
        ((self.code >> 19) & 0xF) as u8
    }

    /// 韵母 [18:13]
    pub fn yunmu(&self) -> u8 {
        ((self.code >> 13) & 0x3F) as u8
    }

    /// 声调 [12:10]
    pub fn tone(&self) -> u8 {
        ((self.code >> 10) & 0x7) as u8
    }

    pub fn raw(&self) -> u32 {
        self.code
    }
}

/// 距离权重常量 (mind_organ 公式)
pub const W_FAMILY: f64 = 1.0;
pub const W_POLARITY: f64 = 10.0;
pub const W_STRUCTURE: f64 = 5.0;
pub const W_SHENGMU: f64 = 0.5;
pub const W_YUNMU: f64 = 0.5;
pub const W_TONE: f64 = 0.1;

/// 距离感知引擎
pub struct DistancePerception;

impl DistancePerception {
    /// 计算两个容器码的加权距离
    pub fn distance(a: &ContainerCode, b: &ContainerCode) -> f64 {
        let df = (a.family() as i32 - b.family() as i32).abs() as f64 * W_FAMILY;
        let dp = (a.polarity() as i32 - b.polarity() as i32).abs() as f64 * W_POLARITY;
        let ds = (a.structure() as i32 - b.structure() as i32).abs() as f64 * W_STRUCTURE;
        let dsm = (a.shengmu() as i32 - b.shengmu() as i32).abs() as f64 * W_SHENGMU;
        let dym = (a.yunmu() as i32 - b.yunmu() as i32).abs() as f64 * W_YUNMU;
        let dt = (a.tone() as i32 - b.tone() as i32).abs() as f64 * W_TONE;

        df + dp + ds + dsm + dym + dt
    }

    /// 计算 ContainerRef 到 ContainerCode 的距离 (通过 entity 字段)
    pub fn distance_ref_to_code(cref: &ContainerRef, code: &ContainerCode) -> f64 {
        let entity_bytes = (cref.entity() as u32).to_le_bytes();
        // 用 entity 的低 32 位反推字段
        let entity = cref.entity() as u32;
        let code_a = ContainerCode::compose(
            ((entity >> 29) & 0x7) as u8,
            ((entity >> 28) & 0x1) as u8,
            ((entity >> 25) & 0x7) as u8,
            ((entity >> 19) & 0xF) as u8,
            ((entity >> 13) & 0x3F) as u8,
            ((entity >> 10) & 0x7) as u8,
        );
        Self::distance(&code_a, code)
    }
}

/// 候选躯壳 (聂小倩匹配结果)
#[derive(Clone, Debug)]
pub struct CandidateBody {
    pub code: ContainerCode,
    pub distance: f64,
    pub index: usize,
}

/// 聂小倩匹配 — 找最近的躯壳
pub struct NieXiaoqianMatcher;

impl NieXiaoqianMatcher {
    /// 从候选集中找距离 ghost 最近的躯壳
    /// 返回 None 当候选集为空
    pub fn find_body(ghost: &ContainerCode, candidates: &[ContainerCode]) -> Option<CandidateBody> {
        candidates.iter().enumerate().min_by(|(_, a), (_, b)| {
            let da = DistancePerception::distance(ghost, a);
            let db = DistancePerception::distance(ghost, b);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        }).map(|(idx, c)| CandidateBody {
            code: c.clone(),
            distance: DistancePerception::distance(ghost, c),
            index: idx,
        })
    }

    /// 找最近的 k 个躯壳 (k-NN)
    pub fn find_k_nearest(ghost: &ContainerCode, candidates: &[ContainerCode], k: usize) -> Vec<CandidateBody> {
        let mut scored: Vec<CandidateBody> = candidates.iter().enumerate().map(|(idx, c)| {
            CandidateBody {
                code: c.clone(),
                distance: DistancePerception::distance(ghost, c),
                index: idx,
            }
        }).collect();

        scored.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(k).collect()
    }

    /// 距离是否在阈值内 (本质匹配判定)
    pub fn is_match(ghost: &ContainerCode, body: &ContainerCode, threshold: f64) -> bool {
        DistancePerception::distance(ghost, body) <= threshold
    }
}

/// 聚类结果
#[derive(Clone, Debug)]
pub struct Cluster {
    pub members: Vec<usize>, // indices into original code array
    pub centroid: ContainerCode,
}

/// 聚类分析器 — 基于距离阈值的简单聚类
pub struct ClusterAnalyzer;

impl ClusterAnalyzer {
    /// 基于距离阈值聚类
    /// 算法: 遍历每个样本, 将其归入第一个满足 distance(centroid, sample) <= threshold 的簇
    ///       若无匹配则新建簇
    pub fn clusterize(codes: &[ContainerCode], threshold: f64) -> Vec<Cluster> {
        let mut clusters: Vec<Cluster> = Vec::new();

        for (i, code) in codes.iter().enumerate() {
            let mut matched = false;
            for cluster in &mut clusters {
                if DistancePerception::distance(&cluster.centroid, code) <= threshold {
                    cluster.members.push(i);
                    matched = true;
                    break;
                }
            }
            if !matched {
                clusters.push(Cluster {
                    members: vec![i],
                    centroid: code.clone(),
                });
            }
        }

        clusters
    }

    /// 获取簇数量
    pub fn cluster_count(codes: &[ContainerCode], threshold: f64) -> usize {
        Self::clusterize(codes, threshold).len()
    }
}

/// 感知器官 — 整合距离感知·匹配·聚类的统一接口
pub struct MindOrgan;

impl MindOrgan {
    /// 计算两个容器码的距离
    pub fn distance(a: &ContainerCode, b: &ContainerCode) -> f64 {
        DistancePerception::distance(a, b)
    }

    /// 聂小倩找躯壳
    pub fn find_body(ghost: &ContainerCode, candidates: &[ContainerCode]) -> Option<CandidateBody> {
        NieXiaoqianMatcher::find_body(ghost, candidates)
    }

    /// 聚类
    pub fn clusterize(codes: &[ContainerCode], threshold: f64) -> Vec<Cluster> {
        ClusterAnalyzer::clusterize(codes, threshold)
    }

    /// 分析容器图集中所有因果关系的距离
    pub fn analyze_atlas(atlas: &ContainerAtlas) -> Vec<(ContainerRef, ContainerRef, f64)> {
        let mut results = Vec::new();
        for edge in atlas.edges_of(&RelationKind::Causal) {
            let a = ContainerCode::new(edge.from.entity() as u32);
            let b = ContainerCode::new(edge.to.entity() as u32);
            results.push((edge.from.clone(), edge.to.clone(), DistancePerception::distance(&a, &b)));
        }
        results
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_code_fields() {
        let code = ContainerCode::compose(3, 1, 5, 10, 42, 2);
        assert_eq!(code.family(), 3);
        assert_eq!(code.polarity(), 1);
        assert_eq!(code.structure(), 5);
        assert_eq!(code.shengmu(), 10);
        assert_eq!(code.yunmu(), 42);
        assert_eq!(code.tone(), 2);
    }

    #[test]
    fn container_code_roundtrip() {
        let original = ContainerCode::compose(0, 0, 0, 0, 0, 0);
        assert_eq!(original.family(), 0);
        assert_eq!(original.polarity(), 0);

        let maxed = ContainerCode::compose(7, 1, 7, 15, 63, 7);
        assert_eq!(maxed.family(), 7);
        assert_eq!(maxed.polarity(), 1);
        assert_eq!(maxed.structure(), 7);
        assert_eq!(maxed.shengmu(), 15);
        assert_eq!(maxed.yunmu(), 63);
        assert_eq!(maxed.tone(), 7);
    }

    #[test]
    fn distance_identical_codes() {
        let a = ContainerCode::compose(3, 1, 5, 10, 42, 2);
        let b = ContainerCode::compose(3, 1, 5, 10, 42, 2);
        assert_eq!(DistancePerception::distance(&a, &b), 0.0);
    }

    #[test]
    fn distance_polarity_heavy() {
        // 极性差异 = 1 × 10.0 = 10.0
        let a = ContainerCode::compose(1, 0, 1, 1, 1, 1);
        let b = ContainerCode::compose(1, 1, 1, 1, 1, 1);
        let d = DistancePerception::distance(&a, &b);
        assert!((d - 10.0).abs() < 1e-9);
    }

    #[test]
    fn distance_structure_mid() {
        // 结构差异 = 2 × 5.0 = 10.0
        let a = ContainerCode::compose(1, 1, 1, 1, 1, 1);
        let b = ContainerCode::compose(1, 1, 3, 1, 1, 1);
        let d = DistancePerception::distance(&a, &b);
        assert!((d - 10.0).abs() < 1e-9);
    }

    #[test]
    fn distance_all_fields() {
        let a = ContainerCode::compose(1, 0, 2, 5, 20, 3);
        let b = ContainerCode::compose(3, 1, 5, 10, 42, 1);
        let d = DistancePerception::distance(&a, &b);
        // |Δ家族|=2 × 1 = 2
        // |Δ极性|=1 × 10 = 10
        // |Δ结构|=3 × 5 = 15
        // |Δ声母|=5 × 0.5 = 2.5
        // |Δ韵母|=22 × 0.5 = 11
        // |Δ声调|=2 × 0.1 = 0.2
        // total = 40.7
        assert!((d - 40.7).abs() < 1e-9);
    }

    #[test]
    fn find_body_closest() {
        let ghost = ContainerCode::compose(3, 1, 5, 10, 42, 2);
        let candidates = vec![
            ContainerCode::compose(3, 1, 5, 10, 42, 2), // identical, dist=0
            ContainerCode::compose(7, 0, 0, 0, 0, 0),   // far
            ContainerCode::compose(3, 1, 5, 10, 42, 3), // tone+1, dist=0.1
        ];
        let result = NieXiaoqianMatcher::find_body(&ghost, &candidates);
        assert!(result.is_some());
        let best = result.unwrap();
        assert_eq!(best.index, 0);
        assert_eq!(best.distance, 0.0);
    }

    #[test]
    fn find_body_empty_candidates() {
        let ghost = ContainerCode::compose(1, 1, 1, 1, 1, 1);
        let candidates: Vec<ContainerCode> = vec![];
        let result = NieXiaoqianMatcher::find_body(&ghost, &candidates);
        assert!(result.is_none());
    }

    #[test]
    fn find_k_nearest_returns_k() {
        let ghost = ContainerCode::compose(0, 0, 0, 0, 0, 0);
        let candidates = vec![
            ContainerCode::compose(0, 0, 0, 0, 0, 0),
            ContainerCode::compose(1, 0, 0, 0, 0, 0),
            ContainerCode::compose(2, 0, 0, 0, 0, 0),
            ContainerCode::compose(3, 0, 0, 0, 0, 0),
            ContainerCode::compose(4, 0, 0, 0, 0, 0),
        ];
        let result = NieXiaoqianMatcher::find_k_nearest(&ghost, &candidates, 3);
        assert_eq!(result.len(), 3);
        // 最近的是 index 0 (distance=0)
        assert_eq!(result[0].index, 0);
    }

    #[test]
    fn is_match_within_threshold() {
        let a = ContainerCode::compose(1, 1, 1, 1, 1, 1);
        let b = ContainerCode::compose(1, 1, 1, 1, 1, 1); // identical
        assert!(NieXiaoqianMatcher::is_match(&a, &b, 0.0));
    }

    #[test]
    fn is_match_outside_threshold() {
        let a = ContainerCode::compose(0, 0, 0, 0, 0, 0);
        let b = ContainerCode::compose(7, 1, 7, 15, 63, 7); // max fields
        // 总距离 > 40
        assert!(!NieXiaoqianMatcher::is_match(&a, &b, 1.0));
    }

    #[test]
    fn clusterize_basic() {
        let codes = vec![
            ContainerCode::compose(0, 0, 0, 0, 0, 0),
            ContainerCode::compose(0, 0, 0, 0, 0, 0),
            ContainerCode::compose(0, 0, 0, 0, 0, 0),
            ContainerCode::compose(7, 1, 7, 15, 63, 7),
        ];
        let clusters = ClusterAnalyzer::clusterize(&codes, 1.0);
        // 前三个应该聚为一簇, 第四个单独一簇
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn clusterize_all_same() {
        let codes = vec![
            ContainerCode::compose(3, 1, 5, 10, 42, 2),
            ContainerCode::compose(3, 1, 5, 10, 42, 2),
            ContainerCode::compose(3, 1, 5, 10, 42, 2),
        ];
        let clusters = ClusterAnalyzer::clusterize(&codes, 0.0);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].members.len(), 3);
    }

    #[test]
    fn cluster_count() {
        let codes = vec![
            ContainerCode::compose(0, 0, 0, 0, 0, 0),
            ContainerCode::compose(7, 1, 7, 15, 63, 7),
            ContainerCode::compose(0, 0, 0, 0, 1, 0), // close to first
        ];
        let count = ClusterAnalyzer::cluster_count(&codes, 1.0);
        assert_eq!(count, 2);
    }

    #[test]
    fn mind_organ_unified_api() {
        let a = ContainerCode::compose(3, 1, 5, 10, 42, 2);
        let b = ContainerCode::compose(3, 1, 5, 10, 42, 2);

        let d = MindOrgan::distance(&a, &b);
        assert_eq!(d, 0.0);

        let body = MindOrgan::find_body(&a, &[b]);
        assert!(body.is_some());

        let clusters = MindOrgan::clusterize(&[a, b], 0.0);
        assert_eq!(clusters.len(), 1);
    }
}
