// 态势感知层 — 距离感知聚类引擎 (连接 state_enumerator ↔ mind_organ)
//
// 架构:
//   StateClassification → encode_to_cell → DistanceCalculator → ClusterEngine
//   → EntityCluster → WrapperTree → pipeline 输出
//
// 距离度量 (五维加权):
//   d0: 状态位汉明距离 (bit-level difference)
//   d1: 轨道大小差 (orbit size)
//   d2: 分类语义距离 (category code diff)
//   d3: 代表元偏移距离 (representative offset)
//   d4: 补码距离 (complement distance)
//
// 权重: 位差(1.0) + 轨道差(3.0) + 分类差(8.0) + 代表元差(0.5) + 补码差(2.0)
//     分类差权重最高——阴阳翻转=根本不同的容器类型

use crate::cross_isa::state_enumerator::*;
use crate::cognition::mind_organ::{EntityCluster, ClusterEngine};
use crate::mirror::EncodedCell;

// ── 状态编码器 ────────────────────────────────────────────────────

/// 将 StateClassification 编码为 64 位单元 (SMRL 格式)
///
/// 位布局 (64位):
/// [63:56] 分类码 (1=轴心,2=闭环,3=纠缠,4=扩展)
/// [55:48] 位宽
/// [47:32] 轨道大小
/// [31:16] 代表元
/// [15:0]  状态值
pub fn encode_classification(c: &StateClassification) -> u64 {
    let cat_code: u64 = match c.category {
        StateCategory::AxialContainer => 1,
        StateCategory::ClosedLoop => 2,
        StateCategory::MirrorEntangled => 3,
        StateCategory::ExtendedEntangled => 4,
    };
    (cat_code << 56)
        | ((c.bit_width as u64) << 48)
        | ((c.orbit.len() as u64) << 32)
        | ((c.orbit.representative as u64) << 16)
        | c.state
}

/// 编码后镜像兼容单元格 (EncodedCell = u64)
pub fn to_encoded_cell(value: u64) -> EncodedCell {
    value
}

// ── 状态距离计算器 ───────────────────────────────────────────────

pub struct StateDistance;

impl StateDistance {
    /// 计算两个状态分类之间的距离向量 (五维)
    pub fn between(a: &StateClassification, b: &StateClassification) -> StateDistanceVector {
        // d0: 位汉明距离
        let hamming = (a.state ^ b.state).count_ones() as f64;
        let max_bits = a.bit_width.max(b.bit_width) as f64;
        let d0 = if max_bits > 0.0 { hamming / max_bits } else { 0.0 };

        // d1: 轨道大小差
        let d1 = ((a.orbit.len() as i64) - (b.orbit.len() as i64)).abs() as f64;

        // d2: 分类语义距离 (0 if same, 1 if different)
        let cat_a = a.category.code() as f64;
        let cat_b = b.category.code() as f64;
        let d2 = if (cat_a - cat_b).abs() < 0.5 { 0.0 } else { (cat_a - cat_b).abs() / 4.0 };

        // d3: 代表元偏移
        let rep_a = a.orbit.representative as f64;
        let rep_b = b.orbit.representative as f64;
        let max_rep = (1u64 << a.bit_width.max(b.bit_width)) as f64;
        let d3 = if max_rep > 0.0 { (rep_a - rep_b).abs() / max_rep } else { 0.0 };

        // d4: 补码距离 (NOT(a.state) 与 b.state 的接近程度)
        // 同一状态下补码距离无意义，置零
        let d4 = if a.state == b.state {
            0.0
        } else {
            let complement_a = (!a.state) & ((1u64 << a.bit_width) - 1);
            let complement_dist = (complement_a ^ b.state).count_ones() as f64;
            if max_bits > 0.0 { complement_dist / max_bits } else { 0.0 }
        };

        // 加权总距离
        let total = d0 * 1.0 + d1 * 3.0 + d2 * 8.0 + d3 * 0.5 + d4 * 2.0;

        StateDistanceVector {
            state_a: a.state,
            state_b: b.state,
            d0_hamming: d0,
            d1_orbit: d1,
            d2_category: d2,
            d3_representative: d3,
            d4_complement: d4,
            total,
        }
    }

    /// 耦合 ClusterEngine 的距离
    pub fn pair_distance(a: &StateClassification, b: &StateClassification) -> f64 {
        Self::between(a, b).total
    }
}

// ── 距离向量 ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StateDistanceVector {
    pub state_a: u64,
    pub state_b: u64,
    pub d0_hamming: f64,
    pub d1_orbit: f64,
    pub d2_category: f64,
    pub d3_representative: f64,
    pub d4_complement: f64,
    pub total: f64,
}

// ── 距离矩阵 ──────────────────────────────────────────────────────

pub struct DistanceMatrix {
    pub labels: Vec<u64>,
    pub matrix: Vec<Vec<f64>>,
}

impl DistanceMatrix {
    pub fn new(classifications: &[StateClassification]) -> Self {
        let reps: Vec<&StateClassification> = classifications.iter()
            .filter(|c| c.orbit.representative == c.state)
            .collect();

        let labels: Vec<u64> = reps.iter().map(|c| c.state).collect();
        let n = reps.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = StateDistance::pair_distance(reps[i], reps[j]);
            }
        }

        Self { labels, matrix }
    }

    /// 找到最近邻对
    pub fn nearest_pair(&self) -> Option<(usize, usize, f64)> {
        let n = self.labels.len();
        if n < 2 { return None; }

        let mut min_dist = f64::INFINITY;
        let mut best = (0, 1);
        for i in 0..n {
            for j in (i + 1)..n {
                if self.matrix[i][j] < min_dist {
                    min_dist = self.matrix[i][j];
                    best = (i, j);
                }
            }
        }
        Some((best.0, best.1, min_dist))
    }

    /// 找到最远对
    pub fn farthest_pair(&self) -> Option<(usize, usize, f64)> {
        let n = self.labels.len();
        if n < 2 { return None; }

        let mut max_dist = 0.0;
        let mut best = (0, 1);
        for i in 0..n {
            for j in (i + 1)..n {
                if self.matrix[i][j] > max_dist {
                    max_dist = self.matrix[i][j];
                    best = (i, j);
                }
            }
        }
        Some((best.0, best.1, max_dist))
    }
}

// ── 态势聚类 ──────────────────────────────────────────────────────

pub struct SituationCluster {
    pub clusters: Vec<EntityCluster>,
    pub matrix: DistanceMatrix,
    pub classifications: Vec<StateClassification>,
}

impl SituationCluster {
    /// 对状态分类进行自动聚类
    ///
    /// 使用 ClusterEngine (mind_organ) 的阈值聚类算法。
    /// threshold 越小 → 聚类越多（更精细）
    pub fn clusterize(classifications: &[StateClassification], threshold: f64) -> Self {
        let reps: Vec<StateClassification> = classifications.iter()
            .filter(|c| c.orbit.representative == c.state)
            .cloned()
            .collect();

        // 编码为 (id, EncodedCell) 供 ClusterEngine 使用
        let cells: Vec<(u64, EncodedCell)> = reps.iter()
            .map(|c| (encode_classification(c), to_encoded_cell(encode_classification(c))))
            .collect();

        let mut engine = ClusterEngine::new(threshold);
        let clusters = engine.clusterize(&cells);

        let matrix = DistanceMatrix::new(classifications);

        Self { clusters, matrix, classifications: reps }
    }

    /// 格式化聚类输出
    pub fn format_report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("态势聚类报告 ({} 簇, {} 状态)\n", self.clusters.len(), self.classifications.len()));
        out.push_str(&"─".repeat(40));
        out.push('\n');

        for (i, cluster) in self.clusters.iter().enumerate() {
            out.push_str(&format!("  簇-{}: {} 成员\n", i, cluster.size()));
            for &member_id in cluster.member_ids.iter().take(4) {
                if let Some(c) = self.classifications.iter().find(|c| encode_classification(c) == member_id) {
                    out.push_str(&format!("    · {:0w$b} [{}]\n", c.state, c.category.name_cn(), w = c.bit_width as usize));
                }
            }
            if cluster.size() > 4 {
                out.push_str(&format!("    ... +{} 更多\n", cluster.size() - 4));
            }
        }

        if let Some((i, j, dist)) = self.matrix.nearest_pair() {
            out.push_str(&format!("\n  最近邻距离: {:.2} ({} ↔ {})\n",
                dist, self.matrix.labels[i], self.matrix.labels[j]));
        }
        if let Some((i, j, dist)) = self.matrix.farthest_pair() {
            out.push_str(&format!("  最远距离:   {:.2} ({} ↔ {})\n",
                dist, self.matrix.labels[i], self.matrix.labels[j]));
        }

        out
    }
}

// ── 连接包裹树 (簇间关系可视化) ──────────────────────────────────

pub struct WrapperTree {
    pub root_nodes: Vec<TreeNode>,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub label: String,
    pub distance_to_parent: f64,
    pub children: Vec<TreeNode>,
    pub cluster_size: usize,
}

impl WrapperTree {
    /// 从聚类结果构建包裹树
    pub fn from_clusters(situation: &SituationCluster) -> Self {
        let mut nodes = Vec::new();

        for (i, cluster) in situation.clusters.iter().enumerate() {
            let label = format!("簇{}[{}]成员", i, cluster.size());
            let category_name = if let Some(first_id) = cluster.member_ids.first() {
                situation.classifications.iter()
                    .find(|c| encode_classification(c) == *first_id)
                    .map(|c| c.category.name_cn().to_string())
                    .unwrap_or_else(|| "未知".to_string())
            } else {
                "空".to_string()
            };

            let node = TreeNode {
                label: format!("{}({})", label, category_name),
                distance_to_parent: 0.0,
                children: Vec::new(),
                cluster_size: cluster.size(),
            };
            nodes.push(node);
        }

        Self { root_nodes: nodes }
    }

    /// 渲染 ASCII 树
    pub fn render_ascii(&self) -> String {
        let mut out = String::new();
        out.push_str("连接包裹树\n");
        out.push_str(&"═".repeat(30));
        out.push('\n');

        for (i, node) in self.root_nodes.iter().enumerate() {
            let is_last = i == self.root_nodes.len() - 1;
            let branch = if is_last { "└──" } else { "├──" };
            let ext = if is_last { "   " } else { "│  " };
            out.push_str(&format!("{} {} [{}] d={:.1}\n", branch, node.label, node.cluster_size, node.distance_to_parent));
            for (j, child) in node.children.iter().enumerate() {
                let child_is_last = j == node.children.len() - 1;
                let cb = if child_is_last { "└──" } else { "├──" };
                out.push_str(&format!("{}{} {} [{}] d={:.1}\n", ext, cb, child.label, child.cluster_size, child.distance_to_parent));
            }
        }
        out.push('\n');
        out
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 综合 API
// ══════════════════════════════════════════════════════════════════════════

/// 运行态势感知：编码 → 距离矩阵 → 聚类 → 包裹树
pub fn analyze(classifications: &[StateClassification], threshold: f64) -> (SituationCluster, WrapperTree) {
    let situation = SituationCluster::clusterize(classifications, threshold);
    let tree = WrapperTree::from_clusters(&situation);
    (situation, tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_class(state: u64, cat: StateCategory) -> StateClassification {
        StateClassification {
            state,
            bit_width: 3,
            category: cat,
            orbit: crate::cross_isa::state_enumerator::Orbit {
                representative: state,
                elements: vec![state],
                stabilizer: vec![],
            },
            explanation: String::new(),
        }
    }

    #[test]
    fn encode_roundtrip() {
        let c = make_class(0b101, StateCategory::AxialContainer);
        let encoded = encode_classification(&c);
        assert_eq!(encoded & 0xFFFF, 0b101);
        assert_eq!((encoded >> 56) & 0xFF, 1);
    }

    #[test]
    fn hamming_distance() {
        let a = make_class(0b000, StateCategory::AxialContainer);
        let b = make_class(0b111, StateCategory::AxialContainer);
        let dv = StateDistance::between(&a, &b);
        assert!(dv.d0_hamming > 0.9);
        assert!(dv.total > 0.0);
    }

    #[test]
    fn same_state_zero_distance() {
        let a = make_class(0b010, StateCategory::MirrorEntangled);
        let dv = StateDistance::between(&a, &a);
        assert!(dv.total < 0.01);
    }

    #[test]
    fn clusterize_basic() {
        let enumerator = StateEnumerator::new(3);
        let classifications = enumerator.enumerate();
        let (situation, _) = analyze(&classifications, 5.0);
        assert!(!situation.clusters.is_empty());
        let rep_count = classifications.iter().filter(|c| c.orbit.representative == c.state).count();
        assert!(situation.clusters.len() <= rep_count);
    }

    #[test]
    fn distance_matrix_size() {
        let enumerator = StateEnumerator::new(3);
        let classifications = enumerator.enumerate();
        let matrix = DistanceMatrix::new(&classifications);
        let rep_count = classifications.iter().filter(|c| c.orbit.representative == c.state).count();
        assert_eq!(matrix.labels.len(), rep_count);
        assert_eq!(matrix.matrix.len(), rep_count);
        if rep_count >= 2 {
            assert_eq!(matrix.matrix[0].len(), rep_count);
        }
    }

    #[test]
    fn wrapper_tree_builds() {
        let enumerator = StateEnumerator::new(3);
        let classifications = enumerator.enumerate();
        let (_, tree) = analyze(&classifications, 5.0);
        assert!(!tree.root_nodes.is_empty());
    }
}
