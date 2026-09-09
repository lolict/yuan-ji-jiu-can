// gtt_learn.rs - 反馈学习引擎 · 感知器官可调节自身阈值和权重
//
// 四个层连成管道：
//   输入存在 → 感知(悖散检测) → 聚类(距离最近绑定) → 格式化(标点层级输出) → 反向(结果评估)
//
// 学习的核心：不是改规则，是调节「距离权重」和「聚类阈值」
//   - 对的存在被聚在一起 → 强化当前权重
//   - 错的存在被聚在一起 → 调整权重 / 收紧阈值
//
// 反馈管道：
//   action   = 选择用什么「思维模式」处理这批输入
//   outcome  = 聚类结果的「紧密度」和「一致性」
//   reward   = 对自己思维选择和权重参数的评估
//   adjust   = 修改权重，让下次同样输入得到更好的聚类
//
// 即：MetaJudge 是裁判自己，gtt_learn 是裁判的裁判

use crate::mirror::{self, EncodedCell};
use crate::mind_organ::{DistanceCalculator, DistanceVector, ClusterEngine, EntityCluster, MindOrgan, PerceptionReport, ThinkingMode};
use crate::gtt_perception::{PerceptionServer, PerceptionEvent, PerceptionSeverity};
use crate::gtt_category::{CategoryRegistry, CategoryCorrector};

// ────────────────────────────────────────────────────────────
// 可调节参数 · 学习的目标
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LearnableWeights {
    pub w_d0: f64,          // d0 维度的权重
    pub w_d1: f64,          // d1 维度的权重
    pub w_d2: f64,          // d2 维度的权重
    pub w_polarity: f64,    // 极性距离权重（最重要）
    pub w_sovereignty: f64, // 主权距离权重
    pub w_witness: f64,     // 见证距离权重
    pub cluster_threshold: f64, // 聚类阈值
}

impl LearnableWeights {
    pub fn new() -> Self {
        Self {
            w_d0: 1.0,
            w_d1: 1.0,
            w_d2: 1.0,
            w_polarity: 10.0,   // 极性翻转=大变故权重高
            w_sovereignty: 5.0,
            w_witness: 0.01,
            cluster_threshold: 100.0,
        }
    }

    /// 应用当前权重计算加权距离
    pub fn weighted_distance(&self, a: EncodedCell, b: EncodedCell) -> f64 {
        let d0 = (mirror::get_d0(a) as f64 - mirror::get_d0(b) as f64).abs();
        let d1 = (mirror::get_d1(a) as f64 - mirror::get_d1(b) as f64).abs();
        let d2 = (mirror::get_d2(a) as f64 - mirror::get_d2(b) as f64).abs();
        let po = (mirror::get_polarity(a) as f64 - mirror::get_polarity(b) as f64).abs();
        let so = (mirror::get_sovereignty(a) as f64 - mirror::get_sovereignty(b) as f64).abs();
        let wi = (mirror::get_witness(a) as f64 - mirror::get_witness(b) as f64).abs();

        d0 * self.w_d0
        + d1 * self.w_d1
        + d2 * self.w_d2
        + po * self.w_polarity
        + so * self.w_sovereignty
        + wi * self.w_witness
    }
}

// ────────────────────────────────────────────────────────────
// 反馈信号 · 对聚类结果的评估
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Feedback {
    pub action: String,      // 做了什么（思维选择）
    pub cluster_cohesion: f64, // 簇内紧密度（越小越好）
    pub cross_cluster_dist: f64, // 簇间距离（越大越好）
    pub perception_score: f64,   // 感知检测的异常数（越少越好）
}

impl Feedback {
    /// 综合奖励 = 簇内紧密 / 簇间分离 × 感知正常率
    pub fn reward(&self) -> f64 {
        let cohesion_part = if self.cluster_cohesion > 0.0 {
            1.0 / (1.0 + self.cluster_cohesion)
        } else {
            1.0
        };
        let separation_part = if self.cross_cluster_dist > 0.0 {
            (self.cross_cluster_dist / (1.0 + self.cross_cluster_dist)).min(1.0)
        } else {
            0.5
        };
        cohesion_part * separation_part * self.perception_score
    }
}

// ────────────────────────────────────────────────────────────
// 学习引擎 · 根据反馈调整权重和阈值
// ────────────────────────────────────────────────────────────

pub struct LearningEngine {
    pub weights: LearnableWeights,
    pub learning_rate: f64,
    pub history: Vec<(Feedback, LearnableWeights)>, // 历史调整记录
    pub mode: LearningMode,
}

#[derive(Clone, Debug)]
pub enum LearningMode {
    Unsupervised,   // 无监督：自己找最好的聚类
    Reinforcement,  // 强化：接受外部reward信号
    Curriculum,     // 课程：由简到繁逐步学
}

impl LearningEngine {
    pub fn new() -> Self {
        Self {
            weights: LearnableWeights::new(),
            learning_rate: 0.01,
            history: Vec::new(),
            mode: LearningMode::Unsupervised,
        }
    }

    /// 执行一次完整的「感知→聚类→评估→调整」循环
    pub fn learn_cycle(&mut self, cells: &[(u64, EncodedCell)]) -> LearningReport {
        // 1. 感知检测
        let mut perception = PerceptionServer::new();
        let mut anomalies = Vec::new();
        for (id, cell) in cells {
            let ch = char::from_u32(mirror::get_d0(*id) as u32).unwrap_or('?');
            anomalies.extend(perception.perceive(ch));
        }

        // 2. 聚类
        let mut engine = ClusterEngine::new(self.weights.cluster_threshold);
        let clusters = engine.clusterize(cells);

        // 3. 计算反馈
        let avg_cohesion = if !clusters.is_empty() {
            clusters.iter().map(|c| c.cohesion(cells)).sum::<f64>() / clusters.len() as f64
        } else {
            0.0
        };
        let cross_dist = Self::cross_cluster_distance(cells, &clusters);
        let perception_normal = 1.0 / (1.0 + anomalies.len() as f64);

        let feedback = Feedback {
            action: format!("{:?}", self.mode),
            cluster_cohesion: avg_cohesion,
            cross_cluster_dist: cross_dist,
            perception_score: perception_normal,
        };

        // 4. 根据反馈调整权重
        self.adjust_weights(&feedback);

        // 5. 保存历史
        self.history.push((feedback.clone(), self.weights.clone()));

        LearningReport {
            clusters_count: clusters.len(),
            anomalies_count: anomalies.len(),
            weights: self.weights.clone(),
            feedback,
        }
    }

    fn adjust_weights(&mut self, feedback: &Feedback) {
        let reward = feedback.reward();
        let lr = self.learning_rate;

        // 奖励低 → 调整权重（尝试改变聚类结果）
        if reward < 0.5 {
            // 随机扰动极性权重（最重要的参数）
            self.weights.w_polarity += (reward - 0.5) * lr * 10.0;
            self.weights.w_polarity = self.weights.w_polarity.max(1.0).min(50.0);

            // 调整阈值
            if feedback.cluster_cohesion > feedback.cross_cluster_dist {
                // 簇内太松 → 收紧阈值
                self.weights.cluster_threshold *= 0.95;
            } else if feedback.cross_cluster_dist < 1.0 {
                // 簇间分不开 → 放宽阈值
                self.weights.cluster_threshold *= 1.05;
            }
        }
    }

    fn cross_cluster_distance(cells: &[(u64, EncodedCell)], clusters: &[EntityCluster]) -> f64 {
        if clusters.len() < 2 { return 0.0; }
        let mut min_dist = f64::MAX;
        for i in 0..clusters.len() {
            for j in (i+1)..clusters.len() {
                // 取两个簇最近成员之间的距离作为簇间距离
                for &id_i in &clusters[i].member_ids {
                    for &id_j in &clusters[j].member_ids {
                        if let (Some(ci), Some(cj)) = (
                            cells.iter().find(|(id, _)| *id == id_i).map(|(_, c)| *c),
                            cells.iter().find(|(id, _)| *id == id_j).map(|(_, c)| *c),
                        ) {
                            let d = DistanceCalculator::pair_distance(ci, cj);
                            if d < min_dist { min_dist = d; }
                        }
                    }
                }
            }
        }
        if min_dist == f64::MAX { 0.0 } else { min_dist }
    }

    /// 获取历史最佳权重（历史上reward最高的那次权重）
    pub fn best_weights(&self) -> Option<&LearnableWeights> {
        self.history.iter()
            .max_by(|(a, _), (b, _)| a.reward().partial_cmp(&b.reward()).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(_, w)| w)
    }
}

#[derive(Clone, Debug)]
pub struct LearningReport {
    pub clusters_count: usize,
    pub anomalies_count: usize,
    pub weights: LearnableWeights,
    pub feedback: Feedback,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::{encode_cell, Sovereignty};

    fn mk_cells() -> Vec<(u64, EncodedCell)> {
        vec![
            (0, encode_cell(100, 50, 30, 1, Sovereignty::Active.encode(), 60, 1000)),
            (1, encode_cell(95, 48, 28, 2, Sovereignty::Passive.encode(), 55, 900)),
            (2, encode_cell(10, 10, 10, 1, Sovereignty::Active.encode(), 50, 500)),
            (3, encode_cell(12, 12, 12, 2, Sovereignty::Passive.encode(), 49, 510)),
        ]
    }

    #[test]
    fn learn_cycle_basic() {
        let mut engine = LearningEngine::new();
        let report = engine.learn_cycle(&mk_cells());
        assert!(report.clusters_count >= 1);
        // 权重应在合理范围
        assert!(report.weights.w_polarity > 0.0);
        assert!(report.weights.cluster_threshold > 0.0);
    }

    #[test]
    fn learn_weights_in_range() {
        let weights = LearnableWeights::new();
        assert!(weights.w_polarity >= 1.0 && weights.w_polarity <= 50.0);
        assert!(weights.cluster_threshold > 0.0);
    }

    #[test]
    fn feedback_reward_range() {
        let fb = Feedback {
            action: "test".to_string(),
            cluster_cohesion: 10.0,
            cross_cluster_dist: 100.0,
            perception_score: 0.8,
        };
        let r = fb.reward();
        assert!(r >= 0.0 && r <= 1.0);
    }

    #[test]
    fn learn_multiple_cycles() {
        let mut engine = LearningEngine::new();
        let cells = mk_cells();
        let r1 = engine.learn_cycle(&cells);
        let r2 = engine.learn_cycle(&cells);
        // 至少历史应有两条记录
        assert!(engine.history.len() >= 2);
    }

    #[test]
    fn best_weights_after_learning() {
        let mut engine = LearningEngine::new();
        let cells = mk_cells();
        for _ in 0..5 {
            engine.learn_cycle(&cells);
        }
        let best = engine.best_weights();
        assert!(best.is_some());
    }
}
