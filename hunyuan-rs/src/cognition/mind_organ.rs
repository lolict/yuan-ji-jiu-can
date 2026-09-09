// @name: mind_organ
// @type: core-mind-organ
// @owner: 五彩石
// @status: stable
// @priority: P0
// @description: 心智器官


// ══════════════════════════════════════════════════════════════════
// mind_organ.rs — 感知器官 · 聚类思维自迭代引擎
// ══════════════════════════════════════════════════════════════════
//
//   「声音和形状距离最近，0和1距离最近，阴和阳距离最近」
//
//   感知不是规则=距离：
//     1. 输入一批「存在」（64位单元）
//     2. 计算两两之间的距离
//     3. 最小距离的一对视为「最应匹配」
//     4. 它们用逗号（弱配对）连起来
//     5. 已经配对的集合=「同类范畴」
//     6. 同类内部用顿号；异类之间用分号
//     7. 最外层用句号停止
//     8. 整个结构可以再嵌入更深——嵌套分隔符
//
//   思维类型（自反）：
//     - perceive     感知：感受到存在
//     - compare      比较：计算距离
//     - cluster      聚类：绑定最近的对
//     - format       格式化：按分隔符层级输出
//     - reflect      反思：审视当前用什么思维
//     - switch       切换：切换思维模式
//
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, EncodedCell};

// ────────────────────────────────────────────────────────────
// 距离向量 · 两个存在之间的「远近距离」
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DistanceVector {
    pub source_id: u64,
    pub target_id: u64,
    pub d0_dist: f64,
    pub d1_dist: f64,
    pub d2_dist: f64,
    pub polarity_dist: f64,
    pub sovereignty_dist: f64,
    pub witness_dist: f64,
    pub total: f64,
}

impl DistanceVector {
    /// 加权总和——极性距离权重最高（阴阳翻转=大变）
    pub fn weighted_total(&self) -> f64 {
        self.d0_dist
        + self.d1_dist
        + self.d2_dist
        + self.polarity_dist * 10.0
        + self.sovereignty_dist * 5.0
        + self.witness_dist * 0.01
    }
}

// ────────────────────────────────────────────────────────────
// 距离计算器 · 核心感知算法
// ────────────────────────────────────────────────────────────

pub struct DistanceCalculator;

impl DistanceCalculator {
    /// 计算两个64位单元之间的距离向量
    pub fn between(a: EncodedCell, b: EncodedCell, id_a: u64, id_b: u64) -> DistanceVector {
        let d0_a = mirror::get_d0(a) as f64;
        let d0_b = mirror::get_d0(b) as f64;
        let d1_a = mirror::get_d1(a) as f64;
        let d1_b = mirror::get_d1(b) as f64;
        let d2_a = mirror::get_d2(a) as f64;
        let d2_b = mirror::get_d2(b) as f64;
        let p_a  = mirror::get_polarity(a) as f64;
        let p_b  = mirror::get_polarity(b) as f64;
        let s_a  = mirror::get_sovereignty(a) as f64;
        let s_b  = mirror::get_sovereignty(b) as f64;
        let w_a  = mirror::get_witness(a) as f64;
        let w_b  = mirror::get_witness(b) as f64;

        let dv = DistanceVector {
            source_id: id_a,
            target_id: id_b,
            d0_dist: (d0_a - d0_b).abs(),
            d1_dist: (d1_a - d1_b).abs(),
            d2_dist: (d2_a - d2_b).abs(),
            polarity_dist: (p_a - p_b).abs(),
            sovereignty_dist: (s_a - s_b).abs(),
            witness_dist: (w_a - w_b).abs(),
            total: 0.0,
        };
        DistanceVector { total: dv.weighted_total(), ..dv }
    }

    /// 计算一对存在之间的总距离
    pub fn pair_distance(a: EncodedCell, b: EncodedCell) -> f64 {
        Self::between(a, b, 0, 1).total
    }
}

// ────────────────────────────────────────────────────────────
// 簇（聚类结果） · 同类集合
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EntityCluster {
    pub id: u64,
    pub member_ids: Vec<u64>,
    pub centroid: Option<EncodedCell>,
    pub category_name: String,
}

impl EntityCluster {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            member_ids: Vec::new(),
            centroid: None,
            category_name: format!("簇-{}", id),
        }
    }

    pub fn add_member(&mut self, id: u64) {
        if !self.member_ids.contains(&id) {
            self.member_ids.push(id);
        }
    }

    pub fn size(&self) -> usize { self.member_ids.len() }

    /// 簇内平均紧密度（越小=越紧密）
    pub fn cohesion(&self, cells: &[(u64, EncodedCell)]) -> f64 {
        if self.member_ids.len() < 2 { return 0.0; }
        let cluster_cells: Vec<EncodedCell> = self.member_ids.iter()
            .filter_map(|id| cells.iter().find(|(cid, _)| cid == id).map(|(_, c)| *c))
            .collect();

        let mut sum_dist = 0.0;
        let mut count = 0;
        for i in 0..cluster_cells.len() {
            for j in (i+1)..cluster_cells.len() {
                sum_dist += DistanceCalculator::pair_distance(cluster_cells[i], cluster_cells[j]);
                count += 1;
            }
        }
        if count == 0 { return 0.0; }
        sum_dist / count as f64
    }

    /// 按顿号格式输出簇内成员
    pub fn format_members(&self) -> String {
        self.member_ids.iter()
            .map(|id| format!("{:X}", id))
            .collect::<Vec<_>>()
            .join("、")
    }
}

// ────────────────────────────────────────────────────────────
// 聚类器 · 迭代式最近邻绑定
// ────────────────────────────────────────────────────────────

pub struct ClusterEngine {
    pub threshold: f64,          // 距离低于此值=可配对
    pub clusters: Vec<EntityCluster>,
    pub next_cluster_id: u64,
}

impl ClusterEngine {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            clusters: Vec::new(),
            next_cluster_id: 0,
        }
    }

    /// 对输入的一批存在进行迭代聚类
    pub fn clusterize(&mut self, cells: &[(u64, EncodedCell)]) -> Vec<EntityCluster> {
        let mut unclustered: Vec<u64> = cells.iter().map(|(id, _)| *id).collect();
        let mut clusters: Vec<EntityCluster> = Vec::new();

        while !unclustered.is_empty() {
            if unclustered.len() == 1 {
                let id = unclustered[0];
                let mut single = EntityCluster::new(self.next_cluster_id);
                self.next_cluster_id += 1;
                single.add_member(id);
                clusters.push(single);
                break;
            }

            if let Some((min_i, min_j, min_dist)) = self.find_min_distance(cells, &unclustered) {
                if min_dist <= self.threshold {
                    let mut joined_existing = false;
                    for c in &mut clusters {
                        if c.member_ids.contains(&unclustered[min_i]) {
                            c.add_member(unclustered[min_j]);
                            joined_existing = true;
                            break;
                        }
                        if c.member_ids.contains(&unclustered[min_j]) {
                            c.add_member(unclustered[min_i]);
                            joined_existing = true;
                            break;
                        }
                    }
                    if !joined_existing {
                        let mut new_c = EntityCluster::new(self.next_cluster_id);
                        self.next_cluster_id += 1;
                        new_c.add_member(unclustered[min_i]);
                        new_c.add_member(unclustered[min_j]);
                        clusters.push(new_c);
                    }
                    let to_remove_j = unclustered[min_j];
                    let to_remove_i = unclustered[min_i];
                    unclustered.retain(|id| *id != to_remove_j && *id != to_remove_i);
                } else {
                    let id = unclustered[min_i];
                    let mut single = EntityCluster::new(self.next_cluster_id);
                    self.next_cluster_id += 1;
                    single.add_member(id);
                    clusters.push(single);
                    unclustered.retain(|x| *x != id);
                }
            } else {
                break;
            }
        }

        self.clusters = clusters.clone();
        clusters
    }

    fn find_min_distance(
        &self,
        cells: &[(u64, EncodedCell)],
        unclustered: &[u64],
    ) -> Option<(usize, usize, f64)> {
        let mut best: Option<(usize, usize, f64)> = None;
        for i in 0..unclustered.len() {
            for j in (i+1)..unclustered.len() {
                let cell_i = cells.iter().find(|(id, _)| *id == unclustered[i]).map(|(_, c)| *c);
                let cell_j = cells.iter().find(|(id, _)| *id == unclustered[j]).map(|(_, c)| *c);
                if let (Some(ci), Some(cj)) = (cell_i, cell_j) {
                    let d = DistanceCalculator::pair_distance(ci, cj);
                    if best.is_none() || d < best.unwrap().2 {
                        best = Some((i, j, d));
                    }
                }
            }
        }
        best
    }

    /// 按分号分隔的格式输出所有簇
    pub fn format_clusters(&self) -> String {
        self.clusters.iter()
            .map(|c| c.format_members())
            .collect::<Vec<_>>()
            .join("；")
    }
}

// ────────────────────────────────────────────────────────────
// 分隔符语法 · 层级存在结构
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Punctuation {
    Comma,
    EnumComma,
    Semicolon,
    Period,
}

impl Punctuation {
    pub fn as_char(&self) -> char {
        match self {
            Punctuation::Comma => ',',
            Punctuation::EnumComma => '、',
            Punctuation::Semicolon => ';',
            Punctuation::Period => '.',
        }
    }
}

// ────────────────────────────────────────────────────────────
// 思维类型 · 感知器官的当前状态
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum ThinkingMode {
    Perceive,
    Compare,
    Cluster,
    Format,
    Reflect,
    Switch,
    Learn,
    MetaPerceive,
}

impl ThinkingMode {
    pub fn name_cn(&self) -> &'static str {
        match self {
            ThinkingMode::Perceive => "感知",
            ThinkingMode::Compare => "比较",
            ThinkingMode::Cluster => "聚类",
            ThinkingMode::Format => "格式化",
            ThinkingMode::Reflect => "反思",
            ThinkingMode::Switch => "切换",
            ThinkingMode::Learn => "学习",
            ThinkingMode::MetaPerceive => "二阶感知",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 二阶感知 · 知道自己在用什么思维
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ThinkingState {
    pub current_mode: ThinkingMode,
    pub mode_history: Vec<ThinkingMode>,
    pub confidence: f64,
    pub perception_count: u64,
    pub reflection_count: u64,
}

impl ThinkingState {
    pub fn new() -> Self {
        Self {
            current_mode: ThinkingMode::Perceive,
            mode_history: Vec::new(),
            confidence: 1.0,
            perception_count: 0,
            reflection_count: 0,
        }
    }

    pub fn switch_to(&mut self, mode: ThinkingMode) {
        self.mode_history.push(self.current_mode.clone());
        self.current_mode = mode;
        self.confidence = 1.0;
    }

    pub fn reflect(&mut self) -> String {
        self.reflection_count += 1;
        let mut report = format!("当前思维：{}", self.current_mode.name_cn());
        if !self.mode_history.is_empty() {
            let prev = &self.mode_history[self.mode_history.len() - 1];
            report.push_str(&format!(" | 从 {} 切换而来", prev.name_cn()));
        }
        report.push_str(&format!(" | 感知={} 反思={} 置信度={:.2}",
            self.perception_count, self.reflection_count, self.confidence));
        report
    }
}

// ────────────────────────────────────────────────────────────
// 感知器官 · 主控体
// ────────────────────────────────────────────────────────────

pub struct MindOrgan {
    state: ThinkingState,
    cluster_engine: ClusterEngine,
    cells: Vec<(u64, EncodedCell)>,
    ticks: u64,
}

impl MindOrgan {
    pub fn new(threshold: f64) -> Self {
        Self {
            state: ThinkingState::new(),
            cluster_engine: ClusterEngine::new(threshold),
            cells: Vec::new(),
            ticks: 0,
        }
    }

    /// 输入一批存在
    pub fn ingest(&mut self, new_cells: &[(u64, EncodedCell)]) {
        self.cells.extend_from_slice(new_cells);
        self.state.current_mode = ThinkingMode::Perceive;
        self.state.perception_count += new_cells.len() as u64;
        self.ticks += 1;
    }

    /// 执行一次完整的感知→聚类→格式化 循环
    pub fn perceive_cluster_format(&mut self) -> PerceptionReport {
        self.state.switch_to(ThinkingMode::Compare);
        self.state.switch_to(ThinkingMode::Cluster);

        let clusters = self.cluster_engine.clusterize(&self.cells);

        self.state.switch_to(ThinkingMode::Format);
        let formatted = self.cluster_engine.format_clusters();

        self.state.switch_to(ThinkingMode::Reflect);
        let reflection = self.state.reflect();

        PerceptionReport {
            clusters,
            formatted,
            reflection,
            tick: self.ticks,
        }
    }

    pub fn learn_and_report(&mut self, new_cells: &[(u64, EncodedCell)]) -> PerceptionReport {
        self.ingest(new_cells);
        self.perceive_cluster_format()
    }

    pub fn current_thinking(&self) -> &ThinkingMode { &self.state.current_mode }

    pub fn reflect_str(&self) -> String {
        let mut s = String::from("=== 感知器官自我反思报告 ===\n");
        s.push_str(&format!("当前思维：{}\n", self.state.current_mode.name_cn()));
        s.push_str(&format!("已感知次数：{}\n", self.state.perception_count));
        s.push_str(&format!("已反思次数：{}\n", self.state.reflection_count));
        s.push_str(&format!("已存在数量：{}\n", self.cells.len()));
        s.push_str(&format!("范畴(簇)数量：{}\n", self.cluster_engine.clusters.len()));
        s.push_str(&format!("聚类阈值：{}\n", self.cluster_engine.threshold));
        s
    }
}

#[derive(Clone, Debug)]
pub struct PerceptionReport {
    pub clusters: Vec<EntityCluster>,
    pub formatted: String,
    pub reflection: String,
    pub tick: u64,
}

impl PerceptionReport {
    pub fn explain(&self) -> String {
        let mut s = format!("--- 第 {} 次感知 ---\n", self.tick);
        s.push_str(&format!("共有 {} 个范畴(簇)\n", self.clusters.len()));
        s.push_str(&format!("格式化结果：{}\n", self.formatted));
        for c in &self.clusters {
            s.push_str(&format!("  簇-{}：[{}] (共{}个成员)\n",
                c.id, c.format_members(), c.size()));
        }
        s.push_str(&format!("反思：{}\n", self.reflection));
        s
    }
}

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::{encode_cell, Sovereignty};

    fn yin_like(id: u64) -> (u64, EncodedCell) {
        (id, encode_cell(100, 50, 30, 1, Sovereignty::Active.encode(), 60, 1000))
    }

    fn yang_like(id: u64) -> (u64, EncodedCell) {
        (id, encode_cell(95, 48, 28, 2, Sovereignty::Passive.encode(), 55, 900))
    }

    fn zero_like(id: u64) -> (u64, EncodedCell) {
        (id, encode_cell(10, 10, 10, 1, Sovereignty::Active.encode(), 50, 500))
    }

    fn one_like(id: u64) -> (u64, EncodedCell) {
        (id, encode_cell(12, 12, 12, 2, Sovereignty::Passive.encode(), 49, 510))
    }

    #[test]
    fn distance_yin_yang_small() {
        let yin = yin_like(0).1;
        let yang = yang_like(1).1;
        let d = DistanceCalculator::pair_distance(yin, yang);
        assert!(d < 100.0, "阴阳距离应该较小：{}", d);
    }

    #[test]
    fn distance_zero_one_are_closest() {
        let zero = zero_like(0).1;
        let one = one_like(1).1;
        let d = DistanceCalculator::pair_distance(zero, one);
        assert!(d < 30.0, "0-1距离应该最小：{}", d);
    }

    #[test]
    fn distance_yin_zero_smaller_than_yin_one() {
        let yin = yin_like(0).1;
        let zero = zero_like(1).1;
        let one = one_like(2).1;
        let d_y0 = DistanceCalculator::pair_distance(yin, zero);
        let d_y1 = DistanceCalculator::pair_distance(yin, one);
        assert!(d_y0 < d_y1, "阴-零({}) < 阴-一({})", d_y0, d_y1);
    }

    #[test]
    fn clusterize_two_pairs() {
        let cells = vec![
            yin_like(0),
            yang_like(1),
            zero_like(2),
            one_like(3),
        ];
        let mut engine = ClusterEngine::new(1000.0);
        let clusters = engine.clusterize(&cells);
        assert!(clusters.len() >= 1);
    }

    #[test]
    fn cluster_format_with_enumeration_comma() {
        let cells = vec![yin_like(0), yang_like(1)];
        let mut engine = ClusterEngine::new(1000.0);
        engine.clusterize(&cells);
        let formatted = engine.format_clusters();
        assert!(formatted.contains('、'), "簇内用顿号：{}", formatted);
    }

    #[test]
    fn mind_organ_learn_and_report() {
        let mut mind = MindOrgan::new(1000.0);
        let report = mind.learn_and_report(&[
            yin_like(0),
            yang_like(1),
            zero_like(2),
            one_like(3),
        ]);
        assert!(report.clusters.len() >= 1);
        assert!(!report.formatted.is_empty());
    }

    #[test]
    fn reflection_self_awareness() {
        let mut mind = MindOrgan::new(1000.0);
        mind.learn_and_report(&[yin_like(0)]);
        let refl = mind.reflect_str();
        assert!(refl.contains("感知器官"));
    }

    #[test]
    fn thinking_mode_switch() {
        let mut state = ThinkingState::new();
        assert_eq!(state.current_mode.name_cn(), "感知");
        state.switch_to(ThinkingMode::Cluster);
        assert_eq!(state.current_mode.name_cn(), "聚类");
        state.switch_to(ThinkingMode::Reflect);
        assert_eq!(state.current_mode.name_cn(), "反思");
    }

    #[test]
    fn cluster_small_threshold_no_merge() {
        let cells = vec![yin_like(0), yang_like(1)];
        let mut engine = ClusterEngine::new(0.0001);
        engine.clusterize(&cells);
        let singles = engine.clusters.iter().filter(|c| c.size() == 1).count();
        assert!(singles >= 1);
    }

    #[test]
    fn cluster_multi_semicolon() {
        let cells = vec![yin_like(0), yang_like(1)];
        let mut engine = ClusterEngine::new(1000.0);
        engine.clusterize(&cells);
        let formatted = engine.format_clusters();
        println!("格式化：{}", formatted);
    }
}
