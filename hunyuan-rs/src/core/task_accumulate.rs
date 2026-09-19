// ═══════════════════════════════════════════════════════════════════════════
// task_accumulate.rs — 分布式等效任务积累引擎
// 聚类统计 · 非重叠积累 · 先验/后验判定 · 产权配置 · 递归消耗 · 未来连接
// ═══════════════════════════════════════════════════════════════════════════
//
// 核心命题（满全法·囡囝共生 NJSG · 认知编译管线）：
//
//   积累公式：Accum_n = Σ Δ_i · 1[task_i ∉ History]
//   非重叠：等效任务不重复累加，只记录版本号（同见者/备用版本）
//   先验 vs 后验：等效=先验(不累加)，非等效=后验(累加频率)
//   异见者：梯度差异 → 统计分类 → 过渡中间态
//   产权配置：主人/客人 · 正/负 · 静态/动态 · 归属/损耗
//   感知返回：声音→声母韵母声调(一维时间) 像素→笔画(零维点)→句子→状态向量
//   递归消耗：消耗后才能连接未来节点，未消耗则失去连接
//
// ═══════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────────────
// 任务类型系统
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskState {
    /// 先验任务：与历史等效，不重复累加
    Prior,
    /// 后验任务：新产生的子任务叠加态，需累加频率
    Posterior,
    /// 备用版本号：同见者的临时修改版本
    Alternate,
    /// 异见者：有梯度差异，进入统计分类
    Dissenter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modality {
    /// 声音状态 → 一维时间轨道
    Phonetic,
    /// 像素状态 → 零维点
    Pixel,
    /// 抽象概念 → 元空间
    Abstract,
}

/// 产权极性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Positive,
    Negative,
}

/// 产权归属
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ownership {
    /// 主人配置：绝对支配权
    Master,
    /// 客人配置：受限访问
    Guest,
    /// 公共配置：共享
    Public,
}

/// 时间性配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalConfig {
    Static,
    Dynamic,
}

// ───────────────────────────────────────────────────────────────────────────
// 感知寻址
// ───────────────────────────────────────────────────────────────────────────

/// 声母 (21个标准)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShengMu {
    B, P, M, F, D, T, N, L,
    G, K, H, J, Q, X,
    Zh, Ch, Sh, R, Z, C, S,
    Y, W,
}

/// 韵母 (39个标准，这里简化为代表性分类)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YunMu {
    A, O, E, I, U, V,
    Ai, Ei, Ui, Ao, Ou, Iu,
    Ie, Ve, Er,
    An, En, In, Un, Vn,
    Ang, Eng, Ing, Ong,
    Ia, Iao, Iou, Ian, Iang, Ionɡ,
    Ua, Uo, Uai, Uei, Uan, Uen, Uang, Ueng,
    Van,
}

/// 声调 (阴平/阳平/上声/去声/轻声)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShengDiao {
    YinPing,  // 阴平 55
    YangPing, // 阳平 35
    ShangSheng, // 上声 214
    QuSheng,  // 去声 51
    QingSheng, // 轻声
}

/// 笔画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stroke {
    Heng,    // 横
    Shu,     // 竖
    Pie,     // 撇
    Na,      // 捺
    Dian,    // 点
    Ti,      // 提
    Zhe,     // 折
    Gou,     // 钩
}

impl Stroke {
    pub fn to_zero_dim_point(&self) -> (f64, f64, f64) {
        match self {
            Stroke::Heng => (1.0, 0.0, 0.0),
            Stroke::Shu => (0.0, 1.0, 0.0),
            Stroke::Pie => (-0.707, 0.707, 0.0),
            Stroke::Na => (0.707, 0.707, 0.0),
            Stroke::Dian => (0.0, 0.0, 1.0),
            Stroke::Ti => (0.707, 0.0, 0.707),
            Stroke::Zhe => (0.0, 0.0, 0.0),
            Stroke::Gou => (0.5, -0.5, 0.0),
        }
    }
}

/// 拼音寻址：声母+韵母+声调 → 一维时间地址
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinyinAddress {
    pub shengmu: u8,      // 声母编码 0-20
    pub yunmu: u8,        // 韵母编码 0-38
    pub shengdiao: u8,    // 声调编码 0-4
}

impl PinyinAddress {
    pub fn new(shengmu: u8, yunmu: u8, shengdiao: u8) -> Self {
        Self { shengmu: shengmu.min(20), yunmu: yunmu.min(38), shengdiao: shengdiao.min(4) }
    }

    /// 一维时间地址计算
    pub fn one_dim_position(&self) -> f64 {
        self.shengmu as f64 * 200.0 + self.yunmu as f64 * 5.0 + self.shengdiao as f64
    }

    pub fn to_string(&self) -> String {
        format!("s{}y{}t{}", self.shengmu, self.yunmu, self.shengdiao)
    }
}

/// 笔画模板空间：零维点串联的一维空间
#[derive(Debug, Clone)]
pub struct StrokeTemplateSpace {
    pub points: Vec<(f64, f64, f64)>,
    /// 频率叠加态
    pub frequency_sum: f64,
}

impl StrokeTemplateSpace {
    pub fn new() -> Self { Self { points: Vec::new(), frequency_sum: 0.0 } }

    /// 零维点串联
    pub fn chain(&mut self, point: (f64, f64, f64)) -> usize {
        self.points.push(point);
        self.points.len() - 1
    }

    /// 返回一维状态向量（各点的加权和）
    pub fn to_state_vector(&self) -> f64 {
        if self.points.is_empty() { return 0.0; }
        let sum_x: f64 = self.points.iter().map(|p| p.0).sum();
        let sum_y: f64 = self.points.iter().map(|p| p.1).sum();
        let sum_z: f64 = self.points.iter().map(|p| p.2).sum();
        (sum_x + sum_y + sum_z).abs() / self.points.len() as f64 + self.frequency_sum
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 任务单元
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TaskUnit {
    pub task_id: String,
    pub phase: f64,
    pub frequency: f64,
    pub time_version: u64,
    pub state: TaskState,
    pub modality: Modality,
    /// 感知地址（拼音或笔画序列）
    pub address: String,
    /// 产权配置
    pub ownership: Ownership,
    pub polarity: Polarity,
    pub temporal: TemporalConfig,
    /// 访问计数器
    pub access_count: u32,
    /// 梯度差异（异见者使用）
    pub gradient_diff: f64,
    /// 递归消耗值
    pub consumption: f64,
    /// 未来连接状态
    pub future_connected: bool,
}

impl TaskUnit {
    pub fn new(id: &str, phase: f64, frequency: f64, time_version: u64) -> Self {
        Self {
            task_id: id.into(),
            phase,
            frequency,
            time_version,
            state: TaskState::Posterior,
            modality: Modality::Abstract,
            address: String::new(),
            ownership: Ownership::Master,
            polarity: Polarity::Positive,
            temporal: TemporalConfig::Dynamic,
            access_count: 0,
            gradient_diff: 0.0,
            consumption: 0.0,
            future_connected: false,
        }
    }

    pub fn with_state(mut self, state: TaskState) -> Self { self.state = state; self }
    pub fn with_modality(mut self, modality: Modality) -> Self { self.modality = modality; self }
    pub fn with_address(mut self, addr: &str) -> Self { self.address = addr.into(); self }
    pub fn with_polarity(mut self, polarity: Polarity) -> Self { self.polarity = polarity; self }
    pub fn with_ownership(mut self, ownership: Ownership) -> Self { self.ownership = ownership; self }

    /// 与另一任务的等效判定
    pub fn is_equivalent(&self, other: &TaskUnit, eps_phase: f64, eps_freq: f64) -> bool {
        let phase_diff = (self.phase - other.phase).abs();
        let freq_diff = (self.frequency - other.frequency).abs();
        phase_diff < eps_phase && freq_diff < eps_freq
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 聚类统计
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TaskCluster {
    pub cluster_id: usize,
    pub centroid_phase: f64,
    pub tasks: Vec<usize>,      // 索引到 task_list
    pub total_frequency: f64,
    pub task_count: usize,
    pub time_min: u64,
    pub time_max: u64,
}

pub struct TaskClusteringEngine;

impl TaskClusteringEngine {
    /// 按相位进行简单K-Means聚类
    pub fn cluster(tasks: &[TaskUnit], k: usize) -> Vec<TaskCluster> {
        if tasks.is_empty() || k == 0 { return Vec::new(); }
        let k = k.min(tasks.len());

        // 初始化中心点（等间距取相位）
        let phases: Vec<f64> = tasks.iter().map(|t| t.phase).collect();
        let mut centroids: Vec<f64> = if phases.len() >= k {
            let step = phases.len() / k;
            (0..k).map(|i| phases[i * step]).collect()
        } else {
            phases.clone()
        };

        let mut assignments = vec![0usize; tasks.len()];

        // 迭代收敛
        for _ in 0..50 {
            // 分配到最近中心
            for (i, phase) in phases.iter().enumerate() {
                let mut best_cluster = 0;
                let mut best_dist = f64::INFINITY;
                for (ci, c) in centroids.iter().enumerate() {
                    let d = (*phase - *c).abs();
                    if d < best_dist { best_dist = d; best_cluster = ci; }
                }
                assignments[i] = best_cluster;
            }

            // 重新计算中心
            let mut sums = vec![0.0f64; k];
            let mut counts = vec![0u32; k];
            for (i, &a) in assignments.iter().enumerate() {
                sums[a] += phases[i];
                counts[a] += 1;
            }
            for ci in 0..k {
                if counts[ci] > 0 {
                    centroids[ci] = sums[ci] / counts[ci] as f64;
                }
            }
        }

        // 构建结果
        (0..k).map(|ci| {
            let cluster_tasks: Vec<usize> = assignments.iter()
                .enumerate()
                .filter(|(_, &a)| a == ci)
                .map(|(i, _)| i)
                .collect();
            let total_freq: f64 = cluster_tasks.iter().map(|&i| tasks[i].frequency).sum();
            let time_min = cluster_tasks.iter().map(|&i| tasks[i].time_version).min().unwrap_or(0);
            let time_max = cluster_tasks.iter().map(|&i| tasks[i].time_version).max().unwrap_or(0);

            let count = cluster_tasks.len();
            TaskCluster {
                cluster_id: ci,
                centroid_phase: centroids[ci],
                tasks: cluster_tasks,
                total_frequency: total_freq,
                task_count: count,
                time_min,
                time_max,
            }
        }).filter(|c| !c.tasks.is_empty()).collect()
    }

    /// 统计每个聚类的频率分布
    pub fn frequency_distribution(clusters: &[TaskCluster]) -> Vec<(usize, f64, f64)> {
        clusters.iter().map(|c| {
            let avg_freq = if c.task_count > 0 { c.total_frequency / c.task_count as f64 } else { 0.0 };
            (c.cluster_id, c.total_frequency, avg_freq)
        }).collect()
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 产权与打分配置
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PropertyConfig {
    pub ownership: Ownership,
    pub polarity: Polarity,
    pub temporal: TemporalConfig,
    pub has_support: bool,      // 可依托/失去依托
    pub within_bounds: bool,    // 有限范畴内/外
    pub score: f64,             // 打分器输出 [-1, 1]
}

impl PropertyConfig {
    pub fn new(ownership: Ownership, polarity: Polarity) -> Self {
        Self {
            ownership,
            polarity,
            temporal: TemporalConfig::Dynamic,
            has_support: true,
            within_bounds: true,
            score: 0.0,
        }
    }

    /// 心理打分器：返回 [-1, 1]
    pub fn compute_score(&mut self) -> f64 {
        let ownership_weight = match self.ownership {
            Ownership::Master => 1.0,
            Ownership::Guest => 0.5,
            Ownership::Public => 0.7,
        };
        let polarity_weight = match self.polarity {
            Polarity::Positive => 1.0,
            Polarity::Negative => -1.5,
        };
        let support_bonus = if self.has_support { 0.1 } else { -0.5 };
        let bounds_bonus = if self.within_bounds { 0.05 } else { -0.8 };

        let raw_score: f64 = ownership_weight * polarity_weight + support_bonus + bounds_bonus;
        self.score = raw_score.clamp(-1.0_f64, 1.0_f64);
        self.score
    }

    /// 是否可积累（正分且在界限内）
    pub fn is_accumulable(&self) -> bool {
        self.score > -0.3 && self.within_bounds
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 积累引擎核心
// ───────────────────────────────────────────────────────────────────────────

/// 积累引擎：非重叠记忆 + 等效匹配 + 递归消耗 + 未来连接
pub struct AccumulationEngine {
    /// 历史任务库（仅追加，不覆盖）
    pub history: Vec<TaskUnit>,
    /// 当前积累量
    pub accumulation: f64,
    /// 聚类结果缓存
    pub clusters: Vec<TaskCluster>,
    /// 等效判定参数
    pub eps_phase: f64,
    pub eps_freq: f64,
    /// 递归消耗阈值
    pub consumption_threshold: f64,
}

impl AccumulationEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            accumulation: 0.0,
            clusters: Vec::new(),
            eps_phase: 5.0,
            eps_freq: 0.05,
            consumption_threshold: 0.1,
        }
    }

    pub fn with_tolerance(mut self, eps_phase: f64, eps_freq: f64) -> Self {
        self.eps_phase = eps_phase;
        self.eps_freq = eps_freq;
        self
    }

    // ─── 核心摄入流程 ───────────────────────────────────────────────────

    /// 摄入一个任务：判定先验/后验 → 累加或跳过 → 写历史 → 未来连接
    pub fn ingest(&mut self, mut task: TaskUnit) -> IngestResult {
        // Step 1: 等效匹配
        let match_result = self.match_equivalent(&task);

        match match_result {
            MatchResult::Prior(matched_idx) => {
                // 先验任务：不累加频率，但更新访问计数
                self.history[matched_idx].access_count += 1;
                task.state = TaskState::Prior;
                IngestResult::Prior(matched_idx, self.history[matched_idx].access_count)
            }
            MatchResult::Alternate(matched_idx) => {
                // 备用版本号：增量版本号
                self.history[matched_idx].access_count += 1;
                task.state = TaskState::Alternate;
                task.time_version = self.history[matched_idx].time_version + 1;
                IngestResult::Alternate(matched_idx)
            }
            MatchResult::Posterior => {
                // 后验任务：累加频率 → 写入历史 → 递归消耗 → 未来连接
                task.state = TaskState::Posterior;

                // 非重叠指示函数 = 1 (新任务)
                let non_overlap_indicator = 1.0;

                // 累加
                self.accumulation += task.frequency * non_overlap_indicator;

                // 写入历史
                let idx = self.history.len();
                self.history.push(task.clone());

                // 递归消耗检查
                self.recursive_consume(idx);

                // 未来连接尝试
                let future_ok = self.connect_future(idx);

                IngestResult::Posterior(idx, task.frequency, future_ok)
            }
            MatchResult::Dissenter(gradient) => {
                // 异见者：梯度差异 → 统计分类
                task.state = TaskState::Dissenter;
                task.gradient_diff = gradient;
                let idx = self.history.len();
                self.history.push(task.clone());
                IngestResult::Dissenter(idx, gradient)
            }
        }
    }

    /// 等效匹配
    fn match_equivalent(&self, task: &TaskUnit) -> MatchResult {
        if self.history.is_empty() {
            return MatchResult::Posterior;
        }

        for (idx, hist) in self.history.iter().enumerate() {
            if hist.is_equivalent(task, self.eps_phase, self.eps_freq) {
                if hist.state == TaskState::Alternate {
                    return MatchResult::Alternate(idx);
                }
                return MatchResult::Prior(idx);
            }
        }

        // 检查是否有近似等效（梯度差异）
        let mut min_gradient = f64::INFINITY;
        for hist in &self.history {
            let g = (hist.phase - task.phase).abs() + (hist.frequency - task.frequency).abs();
            if g < min_gradient { min_gradient = g; }
        }

        // 如果最小梯度在容忍度的数倍内，视为异见者而非全新后验
        if min_gradient < (self.eps_phase + self.eps_freq) * 5.0 {
            return MatchResult::Dissenter(min_gradient);
        }

        MatchResult::Posterior
    }

    /// 递归消耗：产权空间内部的消耗循环
    fn recursive_consume(&mut self, idx: usize) {
        let freq = self.history[idx].frequency;
        // 消耗量 = 频率 × 当前积累量的反比（积累越多，边际消耗越少）
        let consumption = freq / (1.0 + self.accumulation.abs());
        if let Some(task) = self.history.get_mut(idx) {
            task.consumption = consumption;
        }
    }

    /// 连接未来时间节点：消耗后才能连接
    fn connect_future(&mut self, idx: usize) -> bool {
        let consumption = self.history[idx].consumption;
        let connected = consumption >= self.consumption_threshold;
        if let Some(task) = self.history.get_mut(idx) {
            task.future_connected = connected;
        }
        connected
    }

    // ─── 感知路径 ─────────────────────────────────────────────────────

    /// 声音状态 → 声母韵母声调寻址
    pub fn phonetic_address(shengmu: u8, yunmu: u8, shengdiao: u8) -> PinyinAddress {
        PinyinAddress::new(shengmu, yunmu, shengdiao)
    }

    /// 像素状态 → 笔画零维点
    pub fn pixel_stroke(stroke: Stroke) -> (f64, f64, f64) {
        stroke.to_zero_dim_point()
    }

    /// 零维点串联成一维句子模板空间
    pub fn chain_zero_dim(strokes: &[Stroke]) -> StrokeTemplateSpace {
        let mut space = StrokeTemplateSpace::new();
        for stroke in strokes {
            space.chain(stroke.to_zero_dim_point());
        }
        space
    }

    /// 一维句子模板空间 → 一维状态向量
    pub fn to_state_vector(space: &mut StrokeTemplateSpace, frequency: f64) -> f64 {
        space.frequency_sum += frequency;
        space.to_state_vector()
    }

    // ─── 聚类与统计 ────────────────────────────────────────────────────

    pub fn cluster_tasks(&mut self, k: usize) -> &[TaskCluster] {
        self.clusters = TaskClusteringEngine::cluster(&self.history, k);
        &self.clusters
    }

    pub fn cluster_stats(&self) -> Vec<(usize, f64, f64)> {
        TaskClusteringEngine::frequency_distribution(&self.clusters)
    }

    // ─── 查询接口 ─────────────────────────────────────────────────────

    pub fn get_accumulation(&self) -> f64 { self.accumulation }
    pub fn history_size(&self) -> usize { self.history.len() }

    pub fn prior_count(&self) -> usize {
        self.history.iter().filter(|t| t.access_count >= 1).count()
    }

    pub fn posterior_count(&self) -> usize {
        self.history.iter().filter(|t| matches!(t.state, TaskState::Posterior)).count()
    }

    pub fn dissenter_count(&self) -> usize {
        self.history.iter().filter(|t| matches!(t.state, TaskState::Dissenter)).count()
    }

    pub fn connected_future_count(&self) -> usize {
        self.history.iter().filter(|t| t.future_connected).count()
    }

    pub fn dissipation_count(&self) -> usize {
        self.history.iter().filter(|t| !t.alive()).count()
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 结果类型
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum MatchResult {
    Prior(usize),
    Alternate(usize),
    Posterior,
    Dissenter(f64),
}

#[derive(Debug, Clone)]
pub enum IngestResult {
    Prior(usize, u32),
    Alternate(usize),
    Posterior(usize, f64, bool),
    Dissenter(usize, f64),
}

// ─── 为 TaskUnit 添加 alive 方法 ───
impl TaskUnit {
    pub fn alive(&self) -> bool {
        match self.state {
            TaskState::Posterior | TaskState::Prior => {
                self.frequency > 0.0 &&
                (self.polarity == Polarity::Positive || self.gradient_diff < 10.0)
            }
            TaskState::Alternate => true,
            TaskState::Dissenter => self.gradient_diff < 10.0,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────
// 测试
// ───────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prior_not_accumulated() {
        let mut engine = AccumulationEngine::new();
        let t1 = TaskUnit::new("a", 10.0, 1.0, 1000);
        let t2 = TaskUnit::new("b", 10.5, 1.02, 1001); // 近似等效 → Prior

        let _ = engine.ingest(t1);
        let accum_before = engine.get_accumulation();
        let _ = engine.ingest(t2);
        let accum_after = engine.get_accumulation();

        assert_eq!(accum_after, accum_before);
        assert_eq!(engine.prior_count(), 1);
    }

    #[test]
    fn posterior_accumulated() {
        let mut engine = AccumulationEngine::new();
        let t1 = TaskUnit::new("a", 10.0, 1.0, 1000);
        let t2 = TaskUnit::new("b", 200.0, 2.0, 1001); // 远离 → Posterior

        let _ = engine.ingest(t1);
        let _ = engine.ingest(t2);

        assert!(engine.get_accumulation() > 1.0);
        assert_eq!(engine.posterior_count(), 2);
    }

    #[test]
    fn clustering_groups_by_phase() {
        let mut engine = AccumulationEngine::new();

        // 3个不同相位的聚类
        for i in 0..9 {
            let phase = if i < 3 { 10.0 + i as f64 } else if i < 6 { 100.0 + i as f64 } else { 200.0 + i as f64 };
            engine.ingest(TaskUnit::new(&format!("t{}", i), phase, 1.0, 1000 + i as u64));
        }

        let clusters = engine.cluster_tasks(3);
        assert_eq!(clusters.len(), 3);
    }

    #[test]
    fn cluster_frequency_stats() {
        let mut engine = AccumulationEngine::new();
        // 使用不同相位和频率，避免被等效去重
        for i in 0..6 {
            let phase = if i < 3 { 10.0 + i as f64 * 10.0 } else { 100.0 + i as f64 * 10.0 };
            engine.ingest(TaskUnit::new(&format!("t{}", i), phase, 2.0, 1000 + i as u64));
        }

        engine.cluster_tasks(2);
        let stats = engine.cluster_stats();
        assert_eq!(stats.len(), 2);
        // 每个聚类3个任务, 每个频率2.0, 总频率6.0
        let total_freq_sum: f64 = stats.iter().map(|s| s.1).sum();
        assert!((total_freq_sum - 12.0).abs() < 0.01, "所有聚类总频率应为12.0, 实际{}", total_freq_sum);
    }

    #[test]
    fn pinyin_address_one_dim() {
        let addr = PinyinAddress::new(5, 10, 2);
        let pos = addr.one_dim_position();
        assert!((pos - (5.0 * 200.0 + 10.0 * 5.0 + 2.0)).abs() < 0.01);
    }

    #[test]
    fn stroke_zero_dim_point() {
        let p = Stroke::Heng.to_zero_dim_point();
        assert_eq!(p, (1.0, 0.0, 0.0));
    }

    #[test]
    fn chain_to_state_vector() {
        let strokes = vec![Stroke::Heng, Stroke::Shu, Stroke::Pie, Stroke::Na];
        let mut space = AccumulationEngine::chain_zero_dim(&strokes);
        let vector = AccumulationEngine::to_state_vector(&mut space, 1.0);
        assert!(vector > 0.0);
    }

    #[test]
    fn property_score_computation() {
        let mut config = PropertyConfig::new(Ownership::Master, Polarity::Positive);
        config.has_support = true;
        config.within_bounds = true;
        let score = config.compute_score();

        assert!(score > 0.0);
        assert!(score <= 1.0);
        assert!(config.is_accumulable());
    }

    #[test]
    fn negative_owner_not_accumulable() {
        let mut config = PropertyConfig::new(Ownership::Guest, Polarity::Negative);
        config.within_bounds = true;
        let score = config.compute_score();
        assert!(score < 0.0);
        assert!(!config.is_accumulable());
    }

    #[test]
    fn future_connection_gated_by_consumption() {
        let mut engine = AccumulationEngine::new().with_tolerance(5.0, 0.05);
        engine.consumption_threshold = 0.01; // 低阈值使连接更容易

        // 高频率任务 → 高消耗 → 未来连接
        let t = TaskUnit::new("f", 50.0, 5.0, 1000);
        let result = engine.ingest(t);

        match result {
            IngestResult::Posterior(_, _, connected) => {
                assert!(connected, "高频率任务应该能连接未来");
            }
            _ => {}
        }
    }

    #[test]
    fn dissenter_detected() {
        let mut engine = AccumulationEngine::new().with_tolerance(1.0, 0.01);

        // 先注入一个建立基线
        let t1 = TaskUnit::new("base", 100.0, 1.0, 1000);
        let _ = engine.ingest(t1);

        // 注入一个有梯度差异但仍在容忍范围内的任务 (梯度 < 5×容忍度)
        // 相位差=2 (< 5×1.0=5), 频率差=1.0 (< 5×0.01=0.05? No, 1.0 > 0.05)
        // 但梯度总和=3.0 < 5.05 → Dissenter
        let t2 = TaskUnit::new("grad", 102.0, 2.0, 1001);
        let result = engine.ingest(t2);

        // 应该被识别为 Dissenter 而非 Posterior
        assert!(
            matches!(result, IngestResult::Dissenter(_, gradient) if gradient > 0.0),
            "梯度差异应被检测, 实际结果: {:?}", result
        );
    }

    #[test]
    fn non_overlap_guaranteed() {
        let mut engine = AccumulationEngine::new();

        // 注入同一任务两次
        let t1 = TaskUnit::new("dup", 50.0, 2.0, 1000);
        let t2 = TaskUnit::new("dup2", 50.5, 2.01, 1001); // 在容忍度内

        let result1 = engine.ingest(t1);
        let result2 = engine.ingest(t2);

        match (&result1, &result2) {
            (IngestResult::Posterior(_, freq1, _), IngestResult::Prior(_, _)) => {
                // 第一次累加，第二次识别为先验不重复累加
                assert!(*freq1 > 0.0);
                assert_eq!(engine.history.len(), 1);
            }
            _ => panic!("预期先验/后验模式"),
        }
    }
}
