// ======================================================================
// celestial_perception_v9.rs — 天觉系统 v9（老九）· 盟机合袋子感知聚合云
//
// 本文件在天觉v8 盟机合袋子网络协议栈基础上, 构建感知聚合层:
//
//   1. 感知报告 (PerceptionReport)     — 单节点观测快照
//   2. 感知并集 (PerceptionUnion)      — 多节点所有观测汇总
//   3. 感知交集 (PerceptionIntersection) — 多节点共识(从并集→交集)
//   4. 收敛引擎 (ConvergenceEngine)    — 并集→交集收敛算法
//   5. 聚合云   (ConvergenceCloud)     — 多节点全链路调度
//
// 核心哲学:「从并集到交集的一个感知转化」
//
//   并集 = 所有节点的感知松散共存
//         (可能有矛盾、冗余、冲突)
//   交集 = 所有节点都认同的共识
//         (稳定、可靠、可作为行动依据)
//
//   从并集到交集的转化过程本身就是感知精炼:
//     第一步: 收集并集 (gather)
//     第二步: 标记冲突 (identify disagreements)
//     第三步: 权重投票 (reputation-weighted voting)
//     第四步: 输出交集 (emit consensus)
//     第五步: 反馈学习 (learn from disagreements)
//
// 天觉公理: 没有节点的感知是完整的;
//           唯有汇聚众节点, 从并集中提炼交集, 方得真相.
// ======================================================================

use super::celestial_perception_v8::{BagPacket, NodeInfo, PacketKind, SovereignStamp};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// ======================================================================
// 第一部分: 感知报告 — 单节点观测快照
// ======================================================================

/// 单个特征的检测结果
#[derive(Clone, Debug)]
pub struct FeatureDetection {
    pub feature_name: String,
    /// 检测置信度 0.0~1.0
    pub confidence: f64,
    /// 原始观测值 (可选)
    pub raw_value: Option<f64>,
    /// 节点对该特征的判断
    pub judgment: FeatureJudgment,
}

/// 对单个特征的二元判断
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatureJudgment {
    Present,    // 存在
    Absent,     // 不存在
    Uncertain,  // 不确定
}

impl fmt::Display for FeatureJudgment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureJudgment::Present => write!(f, "存在"),
            FeatureJudgment::Absent => write!(f, "不存在"),
            FeatureJudgment::Uncertain => write!(f, "不确定"),
        }
    }
}

/// 感知报告 — 一个盟机场节点的完整感知快照
#[derive(Clone, Debug)]
pub struct PerceptionReport {
    /// 节点 ID
    pub node_id: String,
    /// 时间戳 (unix ms)
    pub timestamp_ms: u64,
    /// 检测到的特征列表
    pub features: Vec<FeatureDetection>,
    /// 节点声誉分 (来自 NodeManager)
    pub reputation: i32,
    /// 主权标记 (过境身份)
    pub sovereign: Option<SovereignStamp>,
    /// 报告摘要哈希 (用于快速比对)
    pub summary_hash: u64,
}

impl PerceptionReport {
    pub fn new(node_id: &str, reputation: i32) -> Self {
        Self {
            node_id: node_id.to_string(),
            timestamp_ms: current_ms(),
            features: Vec::new(),
            reputation,
            sovereign: None,
            summary_hash: 0,
        }
    }

    pub fn with_sovereign(mut self, stamp: SovereignStamp) -> Self {
        self.sovereign = Some(stamp);
        self
    }

    /// 添加一个特征检测
    pub fn detect_feature(&mut self, name: &str, confidence: f64, judgment: FeatureJudgment) {
        self.features.push(FeatureDetection {
            feature_name: name.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
            raw_value: None,
            judgment,
        });
        self.update_hash();
    }

    pub fn detect_value(&mut self, name: &str, value: f64, confidence: f64) -> &mut Self {
        self.features.push(FeatureDetection {
            feature_name: name.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
            raw_value: Some(value),
            judgment: FeatureJudgment::Uncertain,
        });
        self.update_hash();
        self
    }

    /// 获取某特征的判断 (如果有)
    pub fn judgment_for(&self, feature_name: &str) -> Option<FeatureJudgment> {
        self.features.iter()
            .find(|f| f.feature_name == feature_name)
            .map(|f| f.judgment)
    }

    /// 获取某特征的置信度
    pub fn confidence_for(&self, feature_name: &str) -> Option<f64> {
        self.features.iter()
            .find(|f| f.feature_name == feature_name)
            .map(|f| f.confidence)
    }

    /// 计算摘要哈希 (FNV-1a)
    fn update_hash(&mut self) {
        let mut hash: u64 = 0xcbf29ce484222325;
        for feature in &self.features {
            for byte in feature.feature_name.bytes() {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash ^= (feature.confidence.to_bits().wrapping_add(feature.judgment as u64));
            hash = hash.wrapping_mul(0x100000001b3);
        }
        self.summary_hash = hash;
    }

    /// 将报告编码为 BagPacket payload
    pub fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(self.node_id.as_bytes());
        payload.push(0); // 分隔符
        payload.extend_from_slice(&self.timestamp_ms.to_le_bytes());
        payload.extend_from_slice(&(self.features.len() as u32).to_le_bytes());
        for f in &self.features {
            payload.extend_from_slice(f.feature_name.as_bytes());
            payload.push(0);
            payload.extend_from_slice(&f.confidence.to_le_bytes());
            payload.push(f.judgment as u8);
        }
        payload
    }

    /// 特征数量
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    /// 有效报告判定: 有主权标记且未过期
    pub fn is_valid(&self) -> bool {
        match &self.sovereign {
            Some(stamp) => stamp.is_valid(),
            None => true, // 无主权标记时也算有效
        }
    }
}

// ======================================================================
// 第二部分: 感知并集 — 多节点所有观测汇总
// ======================================================================

/// 感知并集 — 所有节点观测的松散汇总
#[derive(Clone, Debug)]
pub struct PerceptionUnion {
    pub reports: Vec<PerceptionReport>,
    pub gathered_at_ms: u64,
}

impl PerceptionUnion {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            gathered_at_ms: current_ms(),
        }
    }

    pub fn add_report(&mut self, report: PerceptionReport) {
        if report.is_valid() {
            self.reports.push(report);
        }
    }

    pub fn from_reports(reports: Vec<PerceptionReport>) -> Self {
        let mut union = Self::new();
        for r in reports {
            union.add_report(r);
        }
        union
    }

    /// 节点数量
    pub fn node_count(&self) -> usize {
        self.reports.len()
    }

    /// 所有被检测到的特征名 (去重)
    pub fn all_feature_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for report in &self.reports {
            for feature in &report.features {
                names.insert(feature.feature_name.clone());
            }
        }
        names
    }

    /// 某特征被多少节点检测到
    pub fn feature_detection_count(&self, name: &str) -> usize {
        self.reports.iter()
            .filter(|r| r.features.iter().any(|f| f.feature_name == name))
            .count()
    }

    /// 某特征的所有判断
    pub fn judgments_for(&self, name: &str) -> Vec<(String, FeatureJudgment, f64)> {
        self.reports.iter()
            .filter_map(|r| {
                r.features.iter()
                    .find(|f| f.feature_name == name)
                    .map(|f| (r.node_id.clone(), f.judgment, f.confidence))
            })
            .collect()
    }

    /// 共识度: 判断一致的节点占总节点的比例
    pub fn agreement_ratio(&self, name: &str) -> Option<f64> {
        let judgments: Vec<_> = self.reports.iter()
            .filter_map(|r| r.judgment_for(name))
            .collect();
        if judgments.is_empty() {
            return None;
        }
        let first = judgments[0];
        let agreeing = judgments.iter().filter(|&&j| j == first).count();
        Some(agreeing as f64 / judgments.len() as f64)
    }

    /// 总特征观测次数
    pub fn total_observations(&self) -> usize {
        self.reports.iter().map(|r| r.features.len()).sum()
    }
}

// ======================================================================
// 第三部分: 感知交集 — 多节点共识
// ======================================================================

/// 共识条目 — 一个特征在所有节点间的共识结果
#[derive(Clone, Debug)]
pub struct ConsensusEntry {
    pub feature_name: String,
    /// 最终裁定
    pub verdict: FeatureJudgment,
    /// 共识度 0.0~1.0 (全体一致=1.0)
    pub agreement: f64,
    /// 加权置信度
    pub weighted_confidence: f64,
    /// 参与共识的节点数
    pub participant_count: usize,
    /// 争论: 持不同意见的节点
    pub dissenters: Vec<(String, FeatureJudgment)>,
}

/// 感知交集 — 从并集提炼的结果
#[derive(Clone, Debug)]
pub struct PerceptionIntersection {
    pub consensus: Vec<ConsensusEntry>,
    pub derived_from_node_count: usize,
    pub derived_from_observation_count: usize,
    pub derived_at_ms: u64,
    /// 整体共识度 (所有特征的平均)
    pub overall_agreement: f64,
}

impl PerceptionIntersection {
    pub fn consensus_count(&self) -> usize {
        self.consensus.len()
    }

    /// 获取某特征的共识
    pub fn consensus_for(&self, name: &str) -> Option<&ConsensusEntry> {
        self.consensus.iter().find(|c| c.feature_name == name)
    }

    /// 强共识条目 (agreement >= threshold)
    pub fn strong_consensus(&self, threshold: f64) -> Vec<&ConsensusEntry> {
        self.consensus.iter()
            .filter(|c| c.agreement >= threshold)
            .collect()
    }

    /// 争议条目 (agreement < threshold)
    pub fn disputed_entries(&self, threshold: f64) -> Vec<&ConsensusEntry> {
        self.consensus.iter()
            .filter(|c| c.agreement < threshold)
            .collect()
    }
}

// ======================================================================
// 第四部分: 收敛引擎 — 并集→交集核心算法
// ======================================================================

/// 收敛配置
#[derive(Clone, Debug)]
pub struct ConvergenceConfig {
    /// 形成共识的最小共识度阈值 (低于此值为"不确定")
    pub agreement_threshold: f64,
    /// 声誉权重因子 (声誉在投票中的影响力)
    pub reputation_weight: f64,
    /// 置信度阈值 (低于此值的报告不参与)
    pub min_confidence: f64,
    /// 所需最小节点数
    pub min_nodes: usize,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            agreement_threshold: 0.66,
            reputation_weight: 0.3,
            min_confidence: 0.2,
            min_nodes: 2,
        }
    }
}

/// 收敛引擎
pub struct ConvergenceEngine {
    pub config: ConvergenceConfig,
    /// 历史收敛记录
    pub history: Vec<PerceptionIntersection>,
}

impl ConvergenceEngine {
    pub fn new() -> Self {
        Self {
            config: ConvergenceConfig::default(),
            history: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: ConvergenceConfig) -> Self {
        self.config = config;
        self
    }

    /// 核心: 从并集计算交集
    pub fn converge(&mut self, union: &PerceptionUnion) -> PerceptionIntersection {
        let feature_names = union.all_feature_names();
        let mut consensus_entries = Vec::new();
        let mut total_agreement = 0.0;

        for fname in &feature_names {
            if let Some(entry) = self.converge_feature(union, fname) {
                total_agreement += entry.agreement;
                consensus_entries.push(entry);
            }
        }

        let overall_agreement = if consensus_entries.is_empty() {
            0.0
        } else {
            total_agreement / consensus_entries.len() as f64
        };

        let intersection = PerceptionIntersection {
            consensus: consensus_entries,
            derived_from_node_count: union.node_count(),
            derived_from_observation_count: union.total_observations(),
            derived_at_ms: current_ms(),
            overall_agreement,
        };

        self.history.push(intersection.clone());
        intersection
    }

    /// 对单一特征计算共识
    fn converge_feature(&self, union: &PerceptionUnion, fname: &str) -> Option<ConsensusEntry> {
        // 收集所有节点对该特征的判断 (带权重)
        let judgments: Vec<(String, FeatureJudgment, f64, i32)> = union.reports.iter()
            .filter_map(|r| {
                let feature = r.features.iter().find(|f| f.feature_name == fname)?;
                if feature.confidence < self.config.min_confidence {
                    return None;
                }
                Some((
                    r.node_id.clone(),
                    feature.judgment,
                    feature.confidence,
                    r.reputation,
                ))
            })
            .collect();

        if judgments.len() < self.config.min_nodes {
            return None;
        }

        // 三类投票: Present / Absent / Uncertain
        let mut present_weight = 0.0;
        let mut absent_weight = 0.0;
        let mut uncertain_weight = 0.0;

        for (_, judgment, confidence, reputation) in &judgments {
            // 权重 = 置信度 * (1 + 声誉权重 * 归一化声誉)
            let rep_factor = 1.0 + self.config.reputation_weight * (*reputation as f64 / 100.0).clamp(-1.0, 1.0);
            let w = confidence * rep_factor;

            match judgment {
                FeatureJudgment::Present => present_weight += w,
                FeatureJudgment::Absent => absent_weight += w,
                FeatureJudgment::Uncertain => uncertain_weight += w,
            }
        }

        let total_weight = present_weight + absent_weight + uncertain_weight;
        if total_weight <= 0.0 {
            return None;
        }

        // 取得分最高的判断
        let mut votes = vec![
            (FeatureJudgment::Present, present_weight),
            (FeatureJudgment::Absent, absent_weight),
            (FeatureJudgment::Uncertain, uncertain_weight),
        ];
        votes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let (verdict, top_weight) = votes[0].clone();
        let agreement = top_weight / total_weight;

        // 收集异议者
        let dissenters: Vec<(String, FeatureJudgment)> = judgments.iter()
            .filter(|(_, j, _, _)| *j != verdict)
            .map(|(id, j, _, _)| (id.clone(), *j))
            .collect();

        // 加权置信度
        let weighted_confidence = judgments.iter()
            .filter(|(_, j, _, _)| *j == verdict)
            .map(|(_, _, c, rep)| {
                let rep_factor = 1.0 + self.config.reputation_weight * (*rep as f64 / 100.0).clamp(-1.0, 1.0);
                c * rep_factor
            })
            .sum::<f64>() / total_weight;

        Some(ConsensusEntry {
            feature_name: fname.to_string(),
            verdict,
            agreement,
            weighted_confidence,
            participant_count: judgments.len(),
            dissenters,
        })
    }

    /// 历史收敛次数
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// 获取最后一次收敛结果
    pub fn last_intersection(&self) -> Option<&PerceptionIntersection> {
        self.history.last()
    }
}

// ======================================================================
// 第五部分: 聚合云 — 全链路调度
// ======================================================================

/// 感知聚合云 — 管理多节点感知收集 + 收敛 + 输出
pub struct ConvergenceCloud {
    pub engine: ConvergenceEngine,
    pub union: PerceptionUnion,
    /// 节点管理: 节点ID → (声誉分, 是否存活)
    pub registry: HashMap<String, (i32, bool)>,
}

impl ConvergenceCloud {
    pub fn new() -> Self {
        Self {
            engine: ConvergenceEngine::new(),
            union: PerceptionUnion::new(),
            registry: HashMap::new(),
        }
    }

    pub fn with_config(mut self, config: ConvergenceConfig) -> Self {
        self.engine = self.engine.with_config(config);
        self
    }

    /// 注册节点
    pub fn register_node(&mut self, node_id: &str, reputation: i32) {
        self.registry.insert(node_id.to_string(), (reputation, true));
    }

    /// 节点上线
    pub fn node_online(&mut self, node_id: &str) {
        if let Some(entry) = self.registry.get_mut(node_id) {
            entry.1 = true;
        }
    }

    /// 节点下线
    pub fn node_offline(&mut self, node_id: &str) {
        if let Some(entry) = self.registry.get_mut(node_id) {
            entry.1 = false;
        }
    }

    /// 获取节点声誉
    pub fn node_reputation(&self, node_id: &str) -> i32 {
        self.registry.get(node_id).map(|(rep, _)| *rep).unwrap_or(50)
    }

    /// 是否所有在线节点都已提交报告
    pub fn all_online_reported(&self) -> bool {
        let online_count = self.registry.values().filter(|(_, alive)| *alive).count();
        let reported_nodes: HashSet<_> = self.union.reports.iter()
            .map(|r| r.node_id.clone())
            .collect();
        let reported_online = self.registry.iter()
            .filter(|(_, (_, alive))| *alive)
            .filter(|(id, _)| reported_nodes.contains(*id))
            .count();
        reported_online >= online_count && online_count > 0
    }

    /// 接收节点报告
    pub fn receive_report(&mut self, report: PerceptionReport) {
        let actual_rep = self.node_reputation(&report.node_id);
        let mut report = report;
        report.reputation = actual_rep;
        self.union.add_report(report);
    }

    /// 执行收敛
    pub fn converge(&mut self) -> PerceptionIntersection {
        self.engine.converge(&self.union)
    }

    /// 重置并集 (开始新一轮收集)
    pub fn reset_round(&mut self) {
        self.union = PerceptionUnion::new();
    }

    /// 在线节点数
    pub fn online_count(&self) -> usize {
        self.registry.values().filter(|(_, alive)| *alive).count()
    }

    /// 生成收敛报告摘要
    pub fn summary(&self) -> CloudSummary {
        let intersection = self.engine.last_intersection();
        CloudSummary {
            total_nodes: self.registry.len(),
            online_nodes: self.online_count(),
            pending_reports: self.union.reports.len(),
            consensus_entries: intersection.map(|i| i.consensus_count()).unwrap_or(0),
            overall_agreement: intersection.map(|i| i.overall_agreement).unwrap_or(0.0),
            strong_consensus: intersection.map(|i| i.strong_consensus(0.8).len()).unwrap_or(0),
            disputed: intersection.map(|i| i.disputed_entries(0.5).len()).unwrap_or(0),
        }
    }
}

/// 云状态摘要
#[derive(Clone, Debug)]
pub struct CloudSummary {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub pending_reports: usize,
    pub consensus_entries: usize,
    pub overall_agreement: f64,
    pub strong_consensus: usize,
    pub disputed: usize,
}

impl fmt::Display for CloudSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔═══ 感知聚合云状态 ═══")?;
        writeln!(f, "║ 总节点: {} (在线: {})", self.total_nodes, self.online_nodes)?;
        writeln!(f, "║ 待处理报告: {}", self.pending_reports)?;
        writeln!(f, "║ 共识条目: {}", self.consensus_entries)?;
        writeln!(f, "║ 整体共识度: {:.1}%", self.overall_agreement * 100.0)?;
        writeln!(f, "║ 强共识(≥80%): {}", self.strong_consensus)?;
        writeln!(f, "║ 争议(<50%): {}", self.disputed)?;
        write!(f, "╚══════════════════════")
    }
}

// ======================================================================
// 辅助
// ======================================================================

fn current_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ======================================================================
// 测试
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perception_report_basic() {
        let mut report = PerceptionReport::new("node-A", 80);
        report.detect_feature("container_glyph", 0.9, FeatureJudgment::Present);
        report.detect_feature("phallic_glyph", 0.3, FeatureJudgment::Absent);

        assert_eq!(report.node_id, "node-A");
        assert_eq!(report.reputation, 80);
        assert_eq!(report.feature_count(), 2);
        assert!(report.is_valid());
        assert_ne!(report.summary_hash, 0);
    }

    #[test]
    fn perception_report_judgment_lookup() {
        let mut report = PerceptionReport::new("n1", 50);
        report.detect_feature("x", 0.8, FeatureJudgment::Present);
        report.detect_feature("y", 0.7, FeatureJudgment::Absent);

        assert_eq!(report.judgment_for("x"), Some(FeatureJudgment::Present));
        assert_eq!(report.judgment_for("y"), Some(FeatureJudgment::Absent));
        assert_eq!(report.judgment_for("z"), None);
        assert_eq!(report.confidence_for("x"), Some(0.8));
    }

    #[test]
    fn perception_report_encode() {
        let mut report = PerceptionReport::new("n1", 50);
        report.detect_feature("test", 0.9, FeatureJudgment::Present);
        let payload = report.encode();
        assert!(!payload.is_empty());
        assert!(payload.starts_with(b"n1"));
    }

    #[test]
    fn union_collects_reports() {
        let mut union = PerceptionUnion::new();
        let r1 = PerceptionReport::new("A", 50);
        let r2 = PerceptionReport::new("B", 60);
        union.add_report(r1);
        union.add_report(r2);
        assert_eq!(union.node_count(), 2);
    }

    #[test]
    fn union_feature_names_dedup() {
        let mut r1 = PerceptionReport::new("A", 50);
        r1.detect_feature("x", 0.9, FeatureJudgment::Present);
        r1.detect_feature("y", 0.8, FeatureJudgment::Absent);

        let mut r2 = PerceptionReport::new("B", 60);
        r2.detect_feature("y", 0.7, FeatureJudgment::Present);
        r2.detect_feature("z", 0.6, FeatureJudgment::Uncertain);

        let union = PerceptionUnion::from_reports(vec![r1, r2]);
        let names = union.all_feature_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains("x"));
        assert!(names.contains("y"));
        assert!(names.contains("z"));
    }

    #[test]
    fn union_agreement_ratio_all_agree() {
        let mut r1 = PerceptionReport::new("A", 50);
        r1.detect_feature("container", 0.9, FeatureJudgment::Present);
        let mut r2 = PerceptionReport::new("B", 50);
        r2.detect_feature("container", 0.8, FeatureJudgment::Present);
        let mut r3 = PerceptionReport::new("C", 50);
        r3.detect_feature("container", 0.7, FeatureJudgment::Present);

        let union = PerceptionUnion::from_reports(vec![r1, r2, r3]);
        assert_eq!(union.agreement_ratio("container"), Some(1.0));
    }

    #[test]
    fn union_agreement_ratio_disagree() {
        let mut r1 = PerceptionReport::new("A", 50);
        r1.detect_feature("container", 0.9, FeatureJudgment::Present);
        let mut r2 = PerceptionReport::new("B", 50);
        r2.detect_feature("container", 0.8, FeatureJudgment::Absent);

        let union = PerceptionUnion::from_reports(vec![r1, r2]);
        let ratio = union.agreement_ratio("container").unwrap();
        assert!(ratio <= 0.6); // 至少有一个不同意
    }

    #[test]
    fn convergence_all_agree() {
        let mut r1 = PerceptionReport::new("A", 80);
        r1.detect_feature("container", 0.95, FeatureJudgment::Present);
        let mut r2 = PerceptionReport::new("B", 70);
        r2.detect_feature("container", 0.85, FeatureJudgment::Present);
        let mut r3 = PerceptionReport::new("C", 60);
        r3.detect_feature("container", 0.90, FeatureJudgment::Present);

        let union = PerceptionUnion::from_reports(vec![r1, r2, r3]);
        let mut engine = ConvergenceEngine::new();
        let intersection = engine.converge(&union);

        let entry = intersection.consensus_for("container").unwrap();
        assert_eq!(entry.verdict, FeatureJudgment::Present);
        assert!(entry.agreement > 0.9);
        assert_eq!(entry.dissenters.len(), 0);
    }

    #[test]
    fn convergence_with_dissent() {
        let mut r1 = PerceptionReport::new("A", 90);
        r1.detect_feature("container", 0.9, FeatureJudgment::Present);
        let mut r2 = PerceptionReport::new("B", 80);
        r2.detect_feature("container", 0.85, FeatureJudgment::Present);
        let mut r3 = PerceptionReport::new("C", 30);
        r3.detect_feature("container", 0.4, FeatureJudgment::Absent);

        let union = PerceptionUnion::from_reports(vec![r1, r2, r3]);
        let mut engine = ConvergenceEngine::new();
        let intersection = engine.converge(&union);

        let entry = intersection.consensus_for("container").unwrap();
        // 多数应该是 Present (A和B是高声誉)
        assert_eq!(entry.verdict, FeatureJudgment::Present);
        assert!(entry.agreement > 0.5);
        // C 是异议者
        assert!(entry.dissenters.iter().any(|(id, _)| id == "C"));
    }

    #[test]
    fn convergence_history_tracked() {
        let union = PerceptionUnion::from_reports(vec![
            {
                let mut r = PerceptionReport::new("A", 50);
                r.detect_feature("x", 0.9, FeatureJudgment::Present);
                r
            },
        ]);
        let mut engine = ConvergenceEngine::new();
        engine.converge(&union);
        assert_eq!(engine.history_count(), 1);
        assert!(engine.last_intersection().is_some());
    }

    #[test]
    fn convergence_config_default() {
        let config = ConvergenceConfig::default();
        assert_eq!(config.agreement_threshold, 0.66);
        assert_eq!(config.min_nodes, 2);
    }

    #[test]
    fn intersection_strong_vs_disputed() {
        // 3 节点, 声誉完全平等 → 纯票数决定
        let mut r1 = PerceptionReport::new("A", 50);
        r1.detect_feature("sure", 0.9, FeatureJudgment::Present);
        r1.detect_feature("doubt", 0.9, FeatureJudgment::Present);
        let mut r2 = PerceptionReport::new("B", 50);
        r2.detect_feature("sure", 0.9, FeatureJudgment::Present);
        r2.detect_feature("doubt", 0.9, FeatureJudgment::Absent);  // 与 r1 完全相反

        let union = PerceptionUnion::from_reports(vec![r1, r2]);
        let mut engine = ConvergenceEngine::new();
        let intersection = engine.converge(&union);

        let strong = intersection.strong_consensus(0.85);
        // "sure": 两节点完全一致 → 强共识 (agreement ≈ 1.0)
        assert!(strong.iter().any(|e| e.feature_name == "sure"), "sure should be strong consensus");

        // "doubt": 各执一词 → agreement 接近 0.5 → 不高不低
        // 但肯定不是 strong
        let doubt_entry = intersection.consensus_for("doubt").unwrap();
        assert!(doubt_entry.agreement < 0.85, "doubt should not be strong consensus");
        assert_eq!(doubt_entry.dissenters.len(), 1); // 异议者 1 人
    }

    #[test]
    fn cloud_register_and_receive() {
        let mut cloud = ConvergenceCloud::new();
        cloud.register_node("A", 80);
        cloud.register_node("B", 70);
        cloud.register_node("C", 60);

        assert_eq!(cloud.online_count(), 3);

        let mut r = PerceptionReport::new("A", 0); // 声誉会被 registry 覆盖
        r.detect_feature("container", 0.95, FeatureJudgment::Present);
        cloud.receive_report(r);

        assert_eq!(cloud.union.node_count(), 1);
        assert_eq!(cloud.node_reputation("A"), 80);
    }

    #[test]
    fn cloud_node_lifecycle() {
        let mut cloud = ConvergenceCloud::new();
        cloud.register_node("A", 80);
        cloud.register_node("B", 70);
        assert_eq!(cloud.online_count(), 2);

        cloud.node_offline("A");
        assert_eq!(cloud.online_count(), 1);

        cloud.node_online("A");
        assert_eq!(cloud.online_count(), 2);
    }

    #[test]
    fn cloud_full_pipeline() {
        let mut cloud = ConvergenceCloud::new();
        cloud.register_node("alpha", 90);
        cloud.register_node("beta", 80);
        cloud.register_node("gamma", 70);

        // alpha: 高置信度检测 container
        let mut r1 = PerceptionReport::new("alpha", 90);
        r1.detect_feature("container", 0.95, FeatureJudgment::Present);
        r1.detect_feature("boundary", 0.88, FeatureJudgment::Present);
        cloud.receive_report(r1);

        // beta: 同样检测
        let mut r2 = PerceptionReport::new("beta", 80);
        r2.detect_feature("container", 0.90, FeatureJudgment::Present);
        r2.detect_feature("boundary", 0.85, FeatureJudgment::Present);
        cloud.receive_report(r2);

        // gamma: 与多数不同
        let mut r3 = PerceptionReport::new("gamma", 70);
        r3.detect_feature("container", 0.60, FeatureJudgment::Absent);
        r3.detect_feature("boundary", 0.92, FeatureJudgment::Present);
        cloud.receive_report(r3);

        // 执行收敛
        let intersection = cloud.converge();

        // container: 2:1 多数 Present
        let c_entry = intersection.consensus_for("container").unwrap();
        assert_eq!(c_entry.verdict, FeatureJudgment::Present);
        assert_eq!(c_entry.dissenters.len(), 1);

        // boundary: 全体一致 Present
        let b_entry = intersection.consensus_for("boundary").unwrap();
        assert_eq!(b_entry.verdict, FeatureJudgment::Present);
        assert!(b_entry.agreement > 0.9);

        // 摘要
        let summary = cloud.summary();
        assert_eq!(summary.total_nodes, 3);
        assert_eq!(summary.consensus_entries, 2);
    }

    #[test]
    fn cloud_reset_round() {
        let mut cloud = ConvergenceCloud::new();
        cloud.register_node("A", 50);
        cloud.receive_report(PerceptionReport::new("A", 50));
        assert_eq!(cloud.union.node_count(), 1);

        cloud.reset_round();
        assert_eq!(cloud.union.node_count(), 0);
        assert_eq!(cloud.online_count(), 1); // 注册不变
    }

    #[test]
    fn cloud_summary_display() {
        let mut cloud = ConvergenceCloud::new();
        cloud.register_node("A", 80);
        cloud.register_node("B", 70);
        let summary = cloud.summary();
        let s = format!("{}", summary);
        assert!(s.contains("感知聚合云"));
        assert!(s.contains("总节点"));
    }

    #[test]
    fn union_total_observations() {
        let mut r1 = PerceptionReport::new("A", 50);
        r1.detect_feature("x", 0.9, FeatureJudgment::Present);
        r1.detect_feature("y", 0.8, FeatureJudgment::Absent);
        let mut r2 = PerceptionReport::new("B", 50);
        r2.detect_feature("z", 0.7, FeatureJudgment::Present);

        let union = PerceptionUnion::from_reports(vec![r1, r2]);
        assert_eq!(union.total_observations(), 3);
    }
}
