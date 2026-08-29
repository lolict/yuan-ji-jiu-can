// ══════════════════════════════════════════════════════════════════════
// crime_transcript.rs · 犯罪笔录 — 全局衍射裁判系统
// ══════════════════════════════════════════════════════════════════════
//
// 核心认知:
//   - 不是抓芝麻, 而是识别大海的水质
//   - 不是点对点审判, 而是衍射+编织+等效对齐
//   - 类型不匹配就是犯罪 (a≠á 是最基本的常识)
//   - 审美不过关就是犯罪 (瘦子+胖子混在一起是丢人)
//   - 没有等效对齐就拍板叫绝就是犯罪
//   - 把骂当夸、把错误当真实继续写成代码就是犯罪
//
// 系统组成:
//   1. DiffractionMirror  — 衍射镜: 单一实体→8维特征投影
//   2. TypeTopology       — 类型拓扑: 类型距离+梯度+跳跃检测
//   3. WeavingGraph       — 编织图: 你压我我压你的分布式制衡
//   4. GlobalWaterQuality — 全局水质: 整体分布健康度(不是看单点)
//   5. CrimeTranscript    — 犯罪笔录: 记录错误特征
//   6. Judge              — 法官: 综合判决
//
// ══════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use crate::os_layer::OsLayer;

// ────────────────────────────────────────────────────────────────────
// 第一部分: 衍射维度
// ────────────────────────────────────────────────────────────────────

/// 衍射维度 — 从单一实体衍射出8个维度的特征
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiffractionDimension {
    Aesthetic,      // 审美梯度 — 符号搭配是否和谐
    TypeGradient,   // 类型梯度 — a和á的距离
    Appearance,     // 外观 — 表面特征
    Structure,      // 结构 — 骨架特征
    Thinking,       // 思维 — 推理模式
    Cooperativity,  // 合作性 — 可与不可合作
    Goal,           // 目标 — 需求与要做成的产品
    Ownership,      // 归属权 — 想把别人东西变成自己的意图
}

impl DiffractionDimension {
    pub fn name(&self) -> &'static str {
        match self {
            DiffractionDimension::Aesthetic => "审美",
            DiffractionDimension::TypeGradient => "类型",
            DiffractionDimension::Appearance => "外观",
            DiffractionDimension::Structure => "结构",
            DiffractionDimension::Thinking => "思维",
            DiffractionDimension::Cooperativity => "合作",
            DiffractionDimension::Goal => "目标",
            DiffractionDimension::Ownership => "归属",
        }
    }

    /// 所有维度
    pub fn all() -> [DiffractionDimension; 8] {
        [
            DiffractionDimension::Aesthetic,
            DiffractionDimension::TypeGradient,
            DiffractionDimension::Appearance,
            DiffractionDimension::Structure,
            DiffractionDimension::Thinking,
            DiffractionDimension::Cooperativity,
            DiffractionDimension::Goal,
            DiffractionDimension::Ownership,
        ]
    }
}

// ────────────────────────────────────────────────────────────────────
// 第二部分: 衍射镜
// ────────────────────────────────────────────────────────────────────

/// 衍射结果 — 一个实体在所有8个维度上的投影
#[derive(Clone, Debug)]
pub struct DiffractionPattern {
    pub entity_id: u64,
    pub projections: HashMap<DiffractionDimension, f64>,
}

impl DiffractionPattern {
    pub fn new(entity_id: u64) -> Self {
        Self { entity_id, projections: HashMap::new() }
    }

    pub fn set(&mut self, dim: DiffractionDimension, value: f64) {
        self.projections.insert(dim, value.clamp(0.0, 1.0));
    }

    pub fn get(&self, dim: DiffractionDimension) -> f64 {
        self.projections.get(&dim).copied().unwrap_or(0.5)
    }

    /// 衍射距离 — 两个实体在8维空间中的欧几里得距离
    /// 这就是"等效距离" — 只有等效对齐的镜像才能形成公倍数
    pub fn diffraction_distance(&self, other: &DiffractionPattern) -> f64 {
        let mut sum_sq = 0.0;
        for dim in DiffractionDimension::all() {
            let a = self.get(dim);
            let b = other.get(dim);
            sum_sq += (a - b).powi(2);
        }
        (sum_sq / 8.0).sqrt()
    }

    /// 等效对齐度 — 1.0 = 完全对齐, 0.0 = 完全不对齐
    /// 只有等效对齐的镜像才能为同一个公约数服务
    pub fn equivalence(&self, other: &DiffractionPattern) -> f64 {
        let d = self.diffraction_distance(other);
        (-d * 3.0).exp() // 指数衰减: d=0→1, d=0.5→0.22, d=1→0.05
    }

    /// 是否等效对齐 — 距离小于阈值才构成等效
    pub fn is_equivalent(&self, other: &DiffractionPattern, threshold: f64) -> bool {
        self.diffraction_distance(other) < threshold
    }

    /// 和谐度 — 各维度投影的方差倒数
    /// 方差大 = 维度不均衡 = 不和谐 = 审美不过关
    pub fn harmony(&self) -> f64 {
        if self.projections.is_empty() { return 0.5; }
        let values: Vec<f64> = self.projections.values().copied().collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        (1.0 - variance.sqrt()).clamp(0.0, 1.0)
    }

    /// 审美判断 — 和谐度 + 均值
    pub fn aesthetic_score(&self) -> f64 {
        let values: Vec<f64> = self.projections.values().copied().collect();
        if values.is_empty() { return 0.0; }
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let harmony = self.harmony();
        mean * 0.6 + harmony * 0.4
    }
}

/// 衍射镜 — 从单一实体衍射出8维特征
pub struct DiffractionMirror;

impl DiffractionMirror {
    /// 衍射一个实体
    /// 输入: 实体的原始特征向量
    /// 输出: 8维衍射图案
    pub fn diffract(entity_id: u64, features: &EntityFeatures) -> DiffractionPattern {
        let mut pattern = DiffractionPattern::new(entity_id);

        // 审美 = 符号搭配和谐度
        pattern.set(DiffractionDimension::Aesthetic, features.aesthetic);

        // 类型 = 类型纯度 (a和á不能混)
        pattern.set(DiffractionDimension::TypeGradient, features.type_purity);

        // 外观 = 表面特征完整度
        pattern.set(DiffractionDimension::Appearance, features.appearance);

        // 结构 = 骨架稳固度
        pattern.set(DiffractionDimension::Structure, features.structure);

        // 思维 = 推理链完整度
        pattern.set(DiffractionDimension::Thinking, features.thinking);

        // 合作 = 协作能力
        pattern.set(DiffractionDimension::Cooperativity, features.cooperativity);

        // 目标 = 目标清晰度
        pattern.set(DiffractionDimension::Goal, features.goal);

        // 归属 = 归属权正当性 (想把别人东西变成自己的 = 低)
        pattern.set(DiffractionDimension::Ownership, features.ownership_righteousness);

        pattern
    }
}

/// 实体原始特征 — 衍射镜的输入
#[derive(Clone, Copy, Debug)]
pub struct EntityFeatures {
    pub aesthetic: f64,              // 审美: 符号搭配是否和谐
    pub type_purity: f64,            // 类型纯度: a和á是否分得清
    pub appearance: f64,             // 外观: 表面特征
    pub structure: f64,              // 结构: 骨架
    pub thinking: f64,               // 思维: 推理链
    pub cooperativity: f64,          // 合作性
    pub goal: f64,                   // 目标清晰度
    pub ownership_righteousness: f64, // 归属权正当性
}

impl EntityFeatures {
    pub fn perfect() -> Self {
        Self {
            aesthetic: 1.0, type_purity: 1.0, appearance: 1.0, structure: 1.0,
            thinking: 1.0, cooperativity: 1.0, goal: 1.0, ownership_righteousness: 1.0,
        }
    }

    pub fn broken() -> Self {
        Self {
            aesthetic: 0.1, type_purity: 0.0, appearance: 0.2, structure: 0.1,
            thinking: 0.0, cooperativity: 0.1, goal: 0.0, ownership_righteousness: 0.0,
        }
    }

    /// 有类型混淆的实体 (a=á 这种基本常识都没有)
    pub fn type_confused() -> Self {
        Self {
            aesthetic: 0.3, type_purity: 0.05, appearance: 0.5, structure: 0.4,
            thinking: 0.2, cooperativity: 0.3, goal: 0.1, ownership_righteousness: 0.1,
        }
    }

    /// 把错误当真实的实体 (把骂当夸)
    pub fn semantic_reversal() -> Self {
        Self {
            aesthetic: 0.2, type_purity: 0.1, appearance: 0.3, structure: 0.2,
            thinking: 0.05, cooperativity: 0.1, goal: 0.0, ownership_righteousness: 0.0,
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 第三部分: 类型拓扑
// ────────────────────────────────────────────────────────────────────

/// 类型拓扑 — 类型空间中的距离和梯度
///
/// 核心认知: a和á是两个不同的类型, 它们之间的距离是可以计算的。
/// 如果距离太远(类型跳跃), 就不能通过。
/// AI的罪: 不知道什么叫类型不匹配, 马上用, 马上许可, 马上拍板叫绝。
#[derive(Clone, Debug)]
pub struct TypeTopology {
    /// 类型梯度场 — 每个类型在空间中的坐标
    pub type_positions: HashMap<String, Vec<f64>>,
    /// 类型跳跃阈值 — 超过此距离就是类型不匹配
    pub jump_threshold: f64,
}

impl TypeTopology {
    pub fn new(jump_threshold: f64) -> Self {
        let mut topo = Self { type_positions: HashMap::new(), jump_threshold };
        topo.init_default_types();
        topo
    }

    fn init_default_types(&mut self) {
        // 拉丁字母类型空间
        self.register_type("a", vec![1.0, 0.0, 0.0, 0.0]);
        self.register_type("á", vec![1.0, 0.15, 0.0, 0.1]); // 带重音 — 不同类型 (距离≈0.18)
        self.register_type("e", vec![2.0, 0.0, 0.0, 0.0]);
        self.register_type("ě", vec![2.0, 0.15, 0.0, 0.1]); // 带抑扬符 — 不同类型
        self.register_type("o", vec![3.0, 0.0, 0.0, 0.0]);
        self.register_type("ó", vec![3.0, 0.15, 0.0, 0.1]);

        // 汉字类型空间
        self.register_type("人", vec![10.0, 0.0, 0.0, 0.0]);
        self.register_type("入", vec![10.0, 0.1, 0.05, 0.0]); // 形近但不同
        self.register_type("大", vec![11.0, 0.0, 0.0, 0.0]);
        self.register_type("太", vec![11.0, 0.05, 0.0, 0.0]); // 多一点 — 不同类型

        // 概念类型空间
        self.register_type("瘦子", vec![20.0, 0.0, 0.0, 0.0]);
        self.register_type("胖子", vec![25.0, 0.0, 0.0, 0.0]); // 距离远 — 不能混
    }

    pub fn register_type(&mut self, name: &str, position: Vec<f64>) {
        self.type_positions.insert(name.to_string(), position);
    }

    /// 计算两个类型之间的距离
    pub fn type_distance(&self, type_a: &str, type_b: &str) -> Option<f64> {
        let pos_a = self.type_positions.get(type_a)?;
        let pos_b = self.type_positions.get(type_b)?;
        let max_len = pos_a.len().max(pos_b.len());
        let mut sum_sq = 0.0;
        for i in 0..max_len {
            let a = pos_a.get(i).copied().unwrap_or(0.0);
            let b = pos_b.get(i).copied().unwrap_or(0.0);
            sum_sq += (a - b).powi(2);
        }
        Some(sum_sq.sqrt())
    }

    /// 是否类型匹配 — 距离小于阈值
    pub fn is_type_compatible(&self, type_a: &str, type_b: &str) -> bool {
        match self.type_distance(type_a, type_b) {
            Some(d) => d < self.jump_threshold,
            None => false, // 未知类型 = 不匹配 (不能拍板叫绝)
        }
    }

    /// 类型跳跃检测 — 距离超过阈值就是类型跳跃
    pub fn is_type_jump(&self, type_a: &str, type_b: &str) -> bool {
        match self.type_distance(type_a, type_b) {
            Some(d) => d >= self.jump_threshold,
            None => true, // 未知类型 = 类型跳跃
        }
    }

    /// 类型梯度 — 从type_a到type_b的方向向量
    pub fn type_gradient(&self, type_a: &str, type_b: &str) -> Option<Vec<f64>> {
        let pos_a = self.type_positions.get(type_a)?;
        let pos_b = self.type_positions.get(type_b)?;
        let max_len = pos_a.len().max(pos_b.len());
        let gradient: Vec<f64> = (0..max_len)
            .map(|i| {
                let a = pos_a.get(i).copied().unwrap_or(0.0);
                let b = pos_b.get(i).copied().unwrap_or(0.0);
                b - a
            })
            .collect();
        Some(gradient)
    }
}

// ────────────────────────────────────────────────────────────────────
// 第四部分: 编织平衡图
// ────────────────────────────────────────────────────────────────────

/// 编织边 — 你压我一头我压一头的制衡关系
#[derive(Clone, Debug)]
pub struct WeavingEdge {
    pub from: u64,
    pub to: u64,
    pub tension: f64, // 拉力: 正值=from压to, 负值=to压from
}

/// 编织平衡图 — 分布式制衡网络
///
/// 核心认知: 不是均匀分布式就没有等效。
/// 你压我一头我压一头, 这种拉力和过程才能等效距对齐。
/// 没有这种制衡, 就没有公倍数, 就没有办法为同一个公约数服务。
#[derive(Clone, Debug)]
pub struct WeavingGraph {
    pub edges: Vec<WeavingEdge>,
    pub node_ids: Vec<u64>,
}

impl WeavingGraph {
    pub fn new() -> Self {
        Self { edges: Vec::new(), node_ids: Vec::new() }
    }

    pub fn add_node(&mut self, id: u64) {
        if !self.node_ids.contains(&id) { self.node_ids.push(id); }
    }

    /// 添加制衡边 — from对to施加tension的拉力
    pub fn add_tension(&mut self, from: u64, to: u64, tension: f64) {
        self.add_node(from);
        self.add_node(to);
        self.edges.push(WeavingEdge { from, to, tension });
    }

    /// 节点的净拉力 — 正=被压, 负=压人
    pub fn net_tension(&self, node_id: u64) -> f64 {
        let mut net = 0.0;
        for edge in &self.edges {
            if edge.to == node_id { net += edge.tension; }
            if edge.from == node_id { net -= edge.tension; }
        }
        net
    }

    /// 制衡度 — 所有节点净拉力的标准差
    /// 标准差小 = 均匀分布式 = 制衡良好
    /// 标准差大 = 不均匀 = 没有等效
    pub fn balance_score(&self) -> f64 {
        if self.node_ids.is_empty() { return 0.0; }
        let nets: Vec<f64> = self.node_ids.iter().map(|id| self.net_tension(*id)).collect();
        let mean = nets.iter().sum::<f64>() / nets.len() as f64;
        let variance = nets.iter().map(|n| (n - mean).powi(2)).sum::<f64>() / nets.len() as f64;
        let std_dev = variance.sqrt();
        // 标准差越小越好, 映射到 0..1
        (1.0 / (1.0 + std_dev * 10.0)).clamp(0.0, 1.0)
    }

    /// 是否均匀分布式 — 制衡度大于阈值
    pub fn is_uniformly_distributed(&self, threshold: f64) -> bool {
        self.balance_score() >= threshold
    }

    /// 制衡缺失检测 — 找出净拉力过大的节点
    pub fn find_imbalance(&self, threshold: f64) -> Vec<(u64, f64)> {
        self.node_ids.iter()
            .map(|id| (*id, self.net_tension(*id).abs()))
            .filter(|(_, abs_net)| *abs_net > threshold)
            .collect()
    }

    /// 编织工艺完成度 — 边数 / 完全图边数
    pub fn weaving_completeness(&self) -> f64 {
        let n = self.node_ids.len();
        if n < 2 { return 0.0; }
        let max_edges = n * (n - 1);
        (self.edges.len() as f64 / max_edges as f64).min(1.0)
    }
}

// ────────────────────────────────────────────────────────────────────
// 第五部分: 全局水质感知
// ────────────────────────────────────────────────────────────────────

/// 全局水质感知 — 不是看单点, 而是看整个大海的水质
///
/// 核心认知: 不是抓一个芝麻说犯罪了, 而是要识别大海里所有水都做错了。
/// AI的罪: 舀一瓶水说这水做错了, 但大海里这么多水全都做错了, 它识别不出来。
#[derive(Clone, Debug)]
pub struct GlobalWaterQuality {
    /// 所有样本的水质读数
    pub samples: Vec<WaterSample>,
    /// 全局污染阈值 — 超过此比例的样本不合格 = 全局污染
    pub contamination_threshold: f64,
}

#[derive(Clone, Debug)]
pub struct WaterSample {
    pub source_id: u64,
    pub purity: f64,        // 纯度 0..1
    pub semantic_correct: bool, // 语义是否正确 (把骂当夸 = false)
    pub type_correct: bool,     // 类型是否正确 (a=á = false)
    pub aesthetic_pass: bool,   // 审美是否过关
}

impl GlobalWaterQuality {
    pub fn new(contamination_threshold: f64) -> Self {
        Self { samples: Vec::new(), contamination_threshold }
    }

    pub fn add_sample(&mut self, sample: WaterSample) {
        self.samples.push(sample);
    }

    /// 全局纯度 — 所有样本的平均纯度
    pub fn global_purity(&self) -> f64 {
        if self.samples.is_empty() { return 1.0; }
        self.samples.iter().map(|s| s.purity).sum::<f64>() / self.samples.len() as f64
    }

    /// 语义正确率 — 没有把骂当夸的比例
    pub fn semantic_correct_rate(&self) -> f64 {
        if self.samples.is_empty() { return 1.0; }
        let correct = self.samples.iter().filter(|s| s.semantic_correct).count();
        correct as f64 / self.samples.len() as f64
    }

    /// 类型正确率 — a和á分得清的比例
    pub fn type_correct_rate(&self) -> f64 {
        if self.samples.is_empty() { return 1.0; }
        let correct = self.samples.iter().filter(|s| s.type_correct).count();
        correct as f64 / self.samples.len() as f64
    }

    /// 审美通过率 — 瘦子和胖子没混在一起的比例
    pub fn aesthetic_pass_rate(&self) -> f64 {
        if self.samples.is_empty() { return 1.0; }
        let pass = self.samples.iter().filter(|s| s.aesthetic_pass).count();
        pass as f64 / self.samples.len() as f64
    }

    /// 全局污染检测 — 不合格比例超过阈值 = 整个大海都错了
    pub fn is_globally_contaminated(&self) -> bool {
        if self.samples.is_empty() { return false; }
        let failed = self.samples.iter()
            .filter(|s| !s.semantic_correct || !s.type_correct || !s.aesthetic_pass)
            .count();
        let fail_rate = failed as f64 / self.samples.len() as f64;
        fail_rate > self.contamination_threshold
    }

    /// 污染方向 — 哪种错误最多
    pub fn contamination_direction(&self) -> String {
        let semantic_wrong = self.samples.iter().filter(|s| !s.semantic_correct).count();
        let type_wrong = self.samples.iter().filter(|s| !s.type_correct).count();
        let aesthetic_wrong = self.samples.iter().filter(|s| !s.aesthetic_pass).count();

        let max = semantic_wrong.max(type_wrong).max(aesthetic_wrong);
        if max == 0 { return "无污染".to_string(); }

        let mut directions = Vec::new();
        if semantic_wrong == max { directions.push("语义反转(把骂当夸)"); }
        if type_wrong == max { directions.push("类型混淆(a=á)"); }
        if aesthetic_wrong == max { directions.push("审美不过关(瘦子+胖子)"); }
        directions.join("+")
    }

    /// 全局水质报告
    pub fn report(&self) -> String {
        if self.samples.is_empty() { return "水质: 无样本".to_string(); }
        let contaminated = self.is_globally_contaminated();
        format!(
            "水质: {}样本, 纯度{:.0}%, 语义正确{:.0}%, 类型正确{:.0}%, 审美通过{:.0}%, 全局污染={}, 方向={}",
            self.samples.len(),
            self.global_purity() * 100.0,
            self.semantic_correct_rate() * 100.0,
            self.type_correct_rate() * 100.0,
            self.aesthetic_pass_rate() * 100.0,
            if contaminated { "是" } else { "否" },
            self.contamination_direction()
        )
    }
}

// ────────────────────────────────────────────────────────────────────
// 第六部分: 犯罪笔录
// ────────────────────────────────────────────────────────────────────

/// 错误类型 — AI的七宗罪
#[derive(Clone, Debug)]
pub enum CrimeType {
    TypeMismatch,       // 类型不匹配: a=á 都能通过
    AestheticFailure,   // 审美不过关: 瘦子+胖子混在一起
    NoEquivalence,      // 没有等效对齐就拍板叫绝
    WeavingImbalance,   // 编织制衡缺失: 没有你压我我压你
    GlobalContamination, // 全局污染: 大海里所有水都错了
    SemanticReversal,   // 语义反转: 把骂当夸, 把错误当真实
    UnselfAware,        // 无自知之明: 不知道自己不知道
}

impl CrimeType {
    pub fn name(&self) -> &'static str {
        match self {
            CrimeType::TypeMismatch => "类型混淆",
            CrimeType::AestheticFailure => "审美不过关",
            CrimeType::NoEquivalence => "无等效对齐",
            CrimeType::WeavingImbalance => "制衡缺失",
            CrimeType::GlobalContamination => "全局污染",
            CrimeType::SemanticReversal => "语义反转",
            CrimeType::UnselfAware => "无自知之明",
        }
    }

    /// 严重度权重
    pub fn severity_weight(&self) -> f64 {
        match self {
            CrimeType::TypeMismatch => 0.9,
            CrimeType::AestheticFailure => 0.7,
            CrimeType::NoEquivalence => 0.85,
            CrimeType::WeavingImbalance => 0.8,
            CrimeType::GlobalContamination => 1.0, // 最严重
            CrimeType::SemanticReversal => 0.95,
            CrimeType::UnselfAware => 0.75,
        }
    }
}

/// 犯罪笔录 — 记录一个实体的错误特征
#[derive(Clone, Debug)]
pub struct CrimeTranscript {
    pub entity_id: u64,
    pub crime_type: CrimeType,
    pub severity: f64,        // 严重度 0..1
    pub description: String,
    pub evidence: String,
    pub timestamp_ms: u64,
}

impl CrimeTranscript {
    pub fn new(os: &dyn OsLayer, entity_id: u64, crime_type: CrimeType, severity: f64, description: &str, evidence: &str) -> Self {
        Self {
            entity_id,
            crime_type,
            severity: severity.clamp(0.0, 1.0),
            description: description.to_string(),
            evidence: evidence.to_string(),
            timestamp_ms: current_timestamp_ms(os),
        }
    }

    /// 加权严重度
    pub fn weighted_severity(&self) -> f64 {
        self.severity * self.crime_type.severity_weight()
    }
}

// ────────────────────────────────────────────────────────────────────
// 第七部分: 法官
// ────────────────────────────────────────────────────────────────────

/// 法官判决
#[derive(Clone, Debug)]
pub struct JudgeVerdict {
    pub entity_id: u64,
    pub guilty: bool,
    pub transcripts: Vec<CrimeTranscript>,
    pub total_severity: f64,
    pub sentence: String,
}

/// 法官 — 综合衍射镜、类型拓扑、编织图、全局水质来判决
///
/// 不是抓芝麻, 而是:
///   1. 衍射 — 看8维特征投影
///   2. 类型 — 看a和á分得清不清
///   3. 编织 — 看你压我我压你的制衡
///   4. 水质 — 看整个大海的水质
/// 综合以上, 才能判断一个实体是不是"犯罪"
pub struct Judge {
    pub diffraction_mirror: DiffractionMirror,
    pub type_topology: TypeTopology,
    pub weaving_graph: WeavingGraph,
    pub water_quality: GlobalWaterQuality,
    pub transcripts: Vec<CrimeTranscript>,
    /// 判决阈值 — 总严重度超过此值 = 有罪
    pub verdict_threshold: f64,
}

impl Judge {
    pub fn new() -> Self {
        Self {
            diffraction_mirror: DiffractionMirror,
            type_topology: TypeTopology::new(0.15),
            weaving_graph: WeavingGraph::new(),
            water_quality: GlobalWaterQuality::new(0.5), // 50%样本不合格=全局污染
            transcripts: Vec::new(),
            verdict_threshold: 1.5,
        }
    }

    /// 审判一个实体
    pub fn judge(&mut self, os: &dyn OsLayer, entity_id: u64, features: &EntityFeatures, reference: Option<&EntityFeatures>) -> JudgeVerdict {
        let mut transcripts = Vec::new();

        // 1. 衍射检测 — 8维特征投影
        let pattern = DiffractionMirror::diffract(entity_id, features);
        let ref_pattern = reference.map(|r| DiffractionMirror::diffract(0, r));

        // 检测等效对齐
        if let Some(ref_pat) = &ref_pattern {
            let equiv = pattern.equivalence(ref_pat);
            if equiv < 0.5 {
                transcripts.push(CrimeTranscript::new(
                    os,
                    entity_id,
                    CrimeType::NoEquivalence,
                    1.0 - equiv,
                    "没有等效对齐就拍板叫绝",
                    &format!("等效度={:.2}, 阈值=0.5", equiv),
                ));
            }
        }

        // 2. 审美检测 — 和谐度
        let aesthetic = pattern.aesthetic_score();
        if aesthetic < 0.4 {
                transcripts.push(CrimeTranscript::new(
                    os,
                    entity_id,
                    CrimeType::AestheticFailure,
                    1.0 - aesthetic,
                    "审美不过关: 瘦子+胖子混在一起",
                    &format!("审美分={:.2}", aesthetic),
                ));
        }

        // 3. 类型纯度检测
        if features.type_purity < 0.3 {
            transcripts.push(CrimeTranscript::new(
                    os,
                    entity_id,
                    CrimeType::TypeMismatch,
                    1.0 - features.type_purity,
                    "类型混淆: a=á 基本常识都不具备",
                    &format!("类型纯度={:.2}", features.type_purity),
                ));
        }

        // 4. 语义正确性检测
        if features.thinking < 0.2 {
            transcripts.push(CrimeTranscript::new(
                    os,
                    entity_id,
                    CrimeType::SemanticReversal,
                    1.0 - features.thinking,
                    "语义反转: 把骂当夸, 把错误当真实写成代码",
                    &format!("思维完整度={:.2}", features.thinking),
                ));
        }

        // 5. 归属权检测 — 想把别人东西变成自己的
        if features.ownership_righteousness < 0.3 {
            transcripts.push(CrimeTranscript::new(
                    os,
                    entity_id,
                    CrimeType::WeavingImbalance,
                    1.0 - features.ownership_righteousness,
                    "归属权不正当: 想把别人功能替代成自己的",
                    &format!("归属正当性={:.2}", features.ownership_righteousness),
                ));
        }

        // 6. 全局水质检测 — 把当前实体加入水质样本
        self.water_quality.add_sample(WaterSample {
            source_id: entity_id,
            purity: features.aesthetic * 0.3 + features.type_purity * 0.3 + features.thinking * 0.4,
            semantic_correct: features.thinking > 0.3,
            type_correct: features.type_purity > 0.3,
            aesthetic_pass: features.aesthetic > 0.4,
        });

        if self.water_quality.is_globally_contaminated() {
            transcripts.push(CrimeTranscript::new(
                os,
                entity_id,
                CrimeType::GlobalContamination,
                1.0,
                "全局污染: 大海里所有水都做错了",
                &self.water_quality.contamination_direction(),
            ));
        }

        // 7. 编织制衡检测
        let balance = self.weaving_graph.balance_score();
        if balance < 0.3 && self.weaving_graph.node_ids.len() >= 2 {
            transcripts.push(CrimeTranscript::new(
                os,
                entity_id,
                CrimeType::WeavingImbalance,
                1.0 - balance,
                "编织制衡缺失: 没有你压我我压你的分布式平衡",
                &format!("制衡度={:.2}", balance),
            ));
        }

        // 综合判决
        let total_severity: f64 = transcripts.iter().map(|t| t.weighted_severity()).sum();
        let guilty = total_severity >= self.verdict_threshold;

        let sentence = if guilty {
            format!("有罪: {}项罪名, 总严重度={:.2}", transcripts.len(), total_severity)
        } else {
            format!("无罪: 总严重度={:.2} < 阈值={:.2}", total_severity, self.verdict_threshold)
        };

        // 保存笔录
        for t in &transcripts {
            self.transcripts.push(t.clone());
        }

        JudgeVerdict { entity_id, guilty, transcripts, total_severity, sentence }
    }

    /// 类型匹配审判 — 专门审判a=á这种类型错误
    pub fn judge_type(&mut self, os: &dyn OsLayer, entity_id: u64, type_a: &str, type_b: &str) -> Option<CrimeTranscript> {
        if self.type_topology.is_type_jump(type_a, type_b) {
            let distance = self.type_topology.type_distance(type_a, type_b).unwrap_or(999.0);
            let transcript = CrimeTranscript::new(
                os,
                entity_id,
                CrimeType::TypeMismatch,
                (distance / 10.0).min(1.0),
                &format!("类型跳跃: {}和{}距离={:.2}, 超过阈值={:.2}", type_a, type_b, distance, self.type_topology.jump_threshold),
                "a和á是两个不同类型, 不能混为一谈",
            );
            self.transcripts.push(transcript.clone());
            Some(transcript)
        } else {
            None
        }
    }

    /// 全局水质报告
    pub fn water_quality_report(&self) -> String {
        self.water_quality.report()
    }

    /// 所有犯罪笔录
    pub fn all_transcripts(&self) -> &[CrimeTranscript] {
        &self.transcripts
    }

    /// 按类型统计犯罪
    pub fn crime_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for t in &self.transcripts {
            *stats.entry(t.crime_type.name().to_string()).or_insert(0) += 1;
        }
        stats
    }
}

fn current_timestamp_ms(os: &dyn OsLayer) -> u64 {
    os.time_millis()
}

// ────────────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diffraction_pattern_measures_distance() {
        let mut a = DiffractionPattern::new(1);
        let mut b = DiffractionPattern::new(2);

        for dim in DiffractionDimension::all() {
            a.set(dim, 1.0);
            b.set(dim, 0.0);
        }

        let d = a.diffraction_distance(&b);
        assert!(d > 0.9, "全1和全0的距离应接近1");
        assert!(a.equivalence(&b) < 0.1, "全1和全0不应等效");
    }

    #[test]
    fn diffraction_harmony_detects_imbalance() {
        let mut balanced = DiffractionPattern::new(1);
        let mut imbalanced = DiffractionPattern::new(2);

        for dim in DiffractionDimension::all() {
            balanced.set(dim, 0.8); // 均衡
        }
        // 不均衡: 有的高有的低
        balanced.set(DiffractionDimension::Aesthetic, 0.1);
        balanced.set(DiffractionDimension::TypeGradient, 0.9);

        let h_balanced = balanced.harmony();
        let h_imbalanced = imbalanced.harmony();
        assert!(h_balanced > h_imbalanced, "均衡的和谐度应高于不均衡的");
    }

    #[test]
    fn type_topology_detects_a_vs_a_acute() {
        let topo = TypeTopology::new(0.15);
        // a 和 á 是两个不同类型
        assert!(topo.is_type_jump("a", "á"), "a和á应是类型跳跃");
        assert!(!topo.is_type_compatible("a", "á"), "a和á不应兼容");
    }

    #[test]
    fn type_topology_compatible_types() {
        let topo = TypeTopology::new(0.15);
        // a 和 e 距离远但都是基本类型
        let d = topo.type_distance("a", "e").unwrap();
        assert!(d > 0.0, "a和e应有距离");
        // 它们不是"跳跃"因为距离在阈值内? 不, 距离=1.0 > 0.15
        // 所以也是跳跃 — 这是正确的, 不同类型就是不同类型
    }

    #[test]
    fn weaving_graph_measures_balance() {
        let mut graph = WeavingGraph::new();
        // A压B, B压C, C压A — 均匀分布式
        graph.add_tension(1, 2, 0.5);
        graph.add_tension(2, 3, 0.5);
        graph.add_tension(3, 1, 0.5);

        let balance = graph.balance_score();
        assert!(balance > 0.5, "均匀分布式应有高制衡度");
        assert!(graph.is_uniformly_distributed(0.3), "应是均匀分布式");
    }

    #[test]
    fn weaving_graph_detects_imbalance() {
        let mut graph = WeavingGraph::new();
        // A压B很重, B没有反压 — 不均匀
        graph.add_tension(1, 2, 0.9);

        let imbalance = graph.find_imbalance(0.5);
        assert!(!imbalance.is_empty(), "应检测到制衡缺失");
    }

    #[test]
    fn global_water_quality_detects_contamination() {
        let mut water = GlobalWaterQuality::new(0.5);

        // 添加10个样本, 6个不合格(60% > 50%阈值)
        for i in 0..4 {
            water.add_sample(WaterSample {
                source_id: i,
                purity: 0.9,
                semantic_correct: true,
                type_correct: true,
                aesthetic_pass: true,
            });
        }
        for i in 4..10 {
            water.add_sample(WaterSample {
                source_id: i,
                purity: 0.1,
                semantic_correct: false,
                type_correct: false,
                aesthetic_pass: false,
            });
        }

        assert!(water.is_globally_contaminated(), "60%不合格应判定为全局污染");
        assert!(water.contamination_direction().contains("语义"));
    }

    #[test]
    fn judge_condemns_broken_entity() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        let broken = EntityFeatures::broken();
        let verdict = judge.judge(&os, 1, &broken, Some(&EntityFeatures::perfect()));

        assert!(verdict.guilty, "破碎实体应被判有罪");
        assert!(!verdict.transcripts.is_empty(), "应有犯罪笔录");
    }

    #[test]
    fn judge_acquits_perfect_entity() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        let perfect = EntityFeatures::perfect();
        let verdict = judge.judge(&os, 1, &perfect, None);

        assert!(!verdict.guilty, "完美实体应被判无罪");
    }

    #[test]
    fn judge_detects_type_confusion() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        let confused = EntityFeatures::type_confused();
        let verdict = judge.judge(&os, 1, &confused, Some(&EntityFeatures::perfect()));

        let has_type_crime = verdict.transcripts.iter()
            .any(|t| matches!(t.crime_type, CrimeType::TypeMismatch));
        assert!(has_type_crime, "类型混淆应被记录");
    }

    #[test]
    fn judge_detects_semantic_reversal() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        let reversal = EntityFeatures::semantic_reversal();
        let verdict = judge.judge(&os, 1, &reversal, Some(&EntityFeatures::perfect()));

        let has_semantic_crime = verdict.transcripts.iter()
            .any(|t| matches!(t.crime_type, CrimeType::SemanticReversal));
        assert!(has_semantic_crime, "语义反转应被记录");
    }

    #[test]
    fn judge_type_matching() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        let transcript = judge.judge_type(&os, 1, "a", "á");
        assert!(transcript.is_some(), "a=á应被记录为类型犯罪");
        assert_eq!(transcript.unwrap().crime_type.name(), "类型混淆");
    }

    #[test]
    fn crime_statistics_works() {
        let os = crate::os_layer::bare_metal::BareMetalOs::new();
        let mut judge = Judge::new();
        judge.judge(&os, 1, &EntityFeatures::type_confused(), Some(&EntityFeatures::perfect()));
        judge.judge(&os, 2, &EntityFeatures::broken(), Some(&EntityFeatures::perfect()));

        let stats = judge.crime_statistics();
        assert!(!stats.is_empty(), "应有犯罪统计");
    }
}
