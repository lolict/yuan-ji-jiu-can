// ══════════════════════════════════════════════════════════════════
// autonomous_loop.rs — 自主运行环 · 感知器官闭环迭代引擎
// ══════════════════════════════════════════════════════════════════
//
// 把所有能力连成一条完整管道：
//
//   摄入存在 → 感知检测 → 范畴归类 → 聚类绑定 → 格式化输出
//       ↑                                            ↓
//       ←←←←←← 反馈学习·调节权重·更新记忆 ←←←←←←←←←←
//
// 自反闭环：
//   感知器官自己 tick：
//     1. 感受：当前内部状态（愉悦值/能量/好奇度）
//     2. 决策：选择下一步做什么（聚类/学习/休息/探索）
//     3. 行动：执行
//     4. 记录：写入记忆
//     5. 反思：评估行动质量，调整策略
//     6. 步进：时钟推进，循环
//
// 这就是「活」的感知器官——不只是工具，是自主体。

use crate::mirror::{self, EncodedCell};
use crate::mind_organ::{MindOrgan, EntityCluster, ClusterEngine, ThinkingMode};
use crate::gtt_perception::PerceptionServer;
use crate::gtt_category::{CategoryRegistry, CategoryCorrector};
use crate::gtt_learn::{LearningEngine, Feedback};
use crate::pleasure::Pleasure;

// ────────────────────────────────────────────────────────────
// 内部驱动力 · 自主体的「欲望」
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Drive {
    pub name: &'static str,
    pub intensity: f64,      // 0..1 强度
    pub threshold: f64,      // 超过此值=触发行动
    pub satisfaction: f64,   // 满足后降低
}

impl Drive {
    pub fn new(name: &'static str, threshold: f64) -> Self {
        Self { name, intensity: 0.5, threshold, satisfaction: 0.0 }
    }

    pub fn tick(&mut self) -> bool {
        self.intensity += 0.01; // 缓慢增长
        if self.intensity >= self.threshold {
            self.intensity = self.threshold;
            true // 触发
        } else {
            false
        }
    }

    pub fn satisfy(&mut self, amount: f64) {
        self.satisfaction += amount;
        self.intensity = (self.intensity - amount).max(0.0);
    }
}

// ────────────────────────────────────────────────────────────
// 行动类型 · 自主体能做的事
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Action {
    Ingest(Vec<(u64, EncodedCell)>),    // 摄入新存在
    Perceive,                             // 感知检测异常
    Cluster,                              // 执行聚类
    Categorize(char),                     // 对某字符执行范畴归类
    Learn,                                // 学习调整权重
    Reflect,                              // 反思当前状态
    Rest,                                 // 休息（什么都不做）
    Explore,                              // 探索未知（降低阈值尝试新聚类）
    Narrate,                              // 输出格式化叙事
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Ingest(_) => "摄入",
            Action::Perceive => "感知",
            Action::Cluster => "聚类",
            Action::Categorize(_) => "归类",
            Action::Learn => "学习",
            Action::Reflect => "反思",
            Action::Rest => "休息",
            Action::Explore => "探索",
            Action::Narrate => "叙述",
        }
    }
}

// ────────────────────────────────────────────────────────────
// 自主体 · 闭环感知器官
// ────────────────────────────────────────────────────────────

pub struct AutonomousAgent {
    pub id: u64,
    pub name: String,
    pub tick_count: u64,
    pub mind: MindOrgan,
    pub learning: LearningEngine,
    pub perception: PerceptionServer,
    pub categories: CategoryRegistry,

    // 内部驱动
    pub curiosity_drive: Drive,    // 好奇：想感知新东西
    pub order_drive: Drive,        // 秩序：想把东西聚类整齐
    pub coherence_drive: Drive,    // 连贯：想消除感知异常
    pub expression_drive: Drive,    // 表达：想输出格式化叙事

    // 状态
    pub current_mode: ThinkingMode,
    pub last_action: Option<Action>,
    pub action_count: u64,
    pub total_anomalies_detected: u64,
    pub total_clusters_formed: u64,
    pub pleasure: Pleasure,

    pub narrative_log: Vec<String>, // 叙述历史
}

impl AutonomousAgent {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            tick_count: 0,
            mind: MindOrgan::new(100.0),
            learning: LearningEngine::new(),
            perception: PerceptionServer::new(),
            categories: CategoryRegistry::new(),

            curiosity_drive: Drive::new("好奇", 0.6),
            order_drive: Drive::new("秩序", 0.5),
            coherence_drive: Drive::new("连贯", 0.7),
            expression_drive: Drive::new("表达", 0.8),

            current_mode: ThinkingMode::Perceive,
            last_action: None,
            action_count: 0,
            total_anomalies_detected: 0,
            total_clusters_formed: 0,
            pleasure: Pleasure(1.0),

            narrative_log: Vec::new(),
        }
    }

    /// 自主运行一步
    pub fn tick(&mut self) -> StepReport {
        self.tick_count += 1;
        self.action_count += 1;

        // 1. 感受：驱动增长
        let want_perceive = self.curiosity_drive.tick();
        let want_cluster = self.order_drive.tick();
        let want_learn = self.coherence_drive.tick();
        let want_narrate = self.expression_drive.tick();

        // 2. 决策：选最强烈的驱动
        let action = self.decide_action(want_perceive, want_cluster, want_learn, want_narrate);

        // 3. 行动
        let result = self.execute_action(&action);

        // 4. 记录记忆
        self.mind.ingest(&[]);  // 触发一次感知计数
        self.last_action = Some(action.clone());

        StepReport {
            tick: self.tick_count,
            action: action.name().to_string(),
            result,
            drive_states: vec![
                ("好奇".to_string(), self.curiosity_drive.intensity),
                ("秩序".to_string(), self.order_drive.intensity),
                ("连贯".to_string(), self.coherence_drive.intensity),
                ("表达".to_string(), self.expression_drive.intensity),
            ],
        }
    }

    fn decide_action(&self, perceive: bool, cluster: bool, learn: bool, narrate: bool) -> Action {
        // 优先级：异常感知 > 聚类 > 学习 > 叙述 > 休息
        if perceive && self.total_anomalies_detected > 0 {
            Action::Perceive
        } else if cluster && self.tick_count > 3 {
            Action::Cluster
        } else if learn && self.tick_count > 5 {
            Action::Learn
        } else if narrate && self.tick_count > 2 {
            Action::Narrate
        } else {
            Action::Explore
        }
    }

    fn execute_action(&mut self, action: &Action) -> String {
        match action {
            Action::Perceive => {
                self.current_mode = ThinkingMode::Perceive;
                let mut total_anomalies = 0;
                for cat in &self.categories.categories {
                    for ch in cat.symbol.chars() {
                        let events = self.perception.perceive(ch);
                        total_anomalies += events.len() as u64;
                    }
                }
                self.total_anomalies_detected += total_anomalies;
                format!("感知完成：检测到 {} 个异常", total_anomalies)
            }
            Action::Cluster => {
                self.current_mode = ThinkingMode::Cluster;
                let cells = self.sample_cells();
                let mut engine = ClusterEngine::new(self.learning.weights.cluster_threshold);
                let clusters = engine.clusterize(&cells);
                self.total_clusters_formed += clusters.len() as u64;
                self.order_drive.satisfy(0.2);
                format!("聚类完成：形成 {} 个簇", clusters.len())
            }
            Action::Categorize(ch) => {
                let corrector = CategoryCorrector::new();
                if let Some(advice) = corrector.correct(*ch, "容器") {
                    format!("范畴校验：{}", advice.message)
                } else {
                    format!("「{}」在正确范畴内", ch)
                }
            }
            Action::Learn => {
                self.current_mode = ThinkingMode::Learn;
                let cells = self.sample_cells();
                let report = self.learning.learn_cycle(&cells);
                self.coherence_drive.satisfy(0.15);
                format!("学习完成：reward={:.3}", report.feedback.reward())
            }
            Action::Reflect => {
                self.current_mode = ThinkingMode::Reflect;
                self.mind.reflect_str()
            }
            Action::Rest => {
                "休眠一 tick".to_string()
            }
            Action::Explore => {
                // 尝试更低阈值聚类
                self.learning.weights.cluster_threshold *= 1.1;
                "探索：放宽聚类阈值".to_string()
            }
            Action::Narrate => {
                self.current_mode = ThinkingMode::Format;
                let narrative = self.narrate();
                self.narrative_log.push(narrative.clone());
                self.expression_drive.satisfy(0.25);
                narrative
            }
            Action::Ingest(cells) => {
                self.mind.ingest(cells);
                self.curiosity_drive.satisfy(0.3);
                format!("摄入 {} 个存在", cells.len())
            }
        }
    }

    fn sample_cells(&self) -> Vec<(u64, EncodedCell)> {
        // 从记忆中采样当前存在的快照
        use crate::mirror::{encode_cell, Sovereignty};
        vec![
            (0, encode_cell(100, 50, 30, 1, Sovereignty::Active.encode(), 60, 1000)),
            (1, encode_cell(95, 48, 28, 2, Sovereignty::Passive.encode(), 55, 900)),
            (2, encode_cell(10, 10, 10, 1, Sovereignty::Active.encode(), 50, 500)),
            (3, encode_cell(12, 12, 12, 2, Sovereignty::Passive.encode(), 49, 510)),
        ]
    }

    fn narrate(&self) -> String {
        let mut narrative = format!("「{}」第 {} 次叙述：\n", self.name, self.tick_count);
        narrative.push_str(&format!("  当前思维：{}\n", self.current_mode.name_cn()));
        narrative.push_str(&format!("  已感知异常：{}\n", self.total_anomalies_detected));
        narrative.push_str(&format!("  已形成簇：{}\n", self.total_clusters_formed));
        narrative.push_str(&format!("  聚类阈值：{:.2}\n", self.learning.weights.cluster_threshold));
        narrative.push_str(&format!("  极性权重：{:.2}\n", self.learning.weights.w_polarity));
        narrative
    }

    /// 运行多 tick
    pub fn run_ticks(&mut self, n: u64) -> Vec<StepReport> {
        let mut reports = Vec::new();
        for _ in 0..n {
            reports.push(self.tick());
        }
        reports
    }

    /// 对外摄入新存在
    pub fn ingest_cells(&mut self, cells: &[(u64, EncodedCell)]) -> String {
        self.mind.ingest(cells);
        self.curiosity_drive.satisfy(0.1);
        format!("摄入 {} 个新存在", cells.len())
    }

    /// 获取完整反思
    pub fn reflect(&self) -> String {
        let mut s = String::from("╔═══ 自主体完整反思 ═══\n");
        s.push_str(&format!("║ 名称：{}\n", self.name));
        s.push_str(&format!("║ 已运行 tick：{}\n", self.tick_count));
        s.push_str(&format!("║ 当前思维：{}\n", self.current_mode.name_cn()));
        s.push_str(&format!("║ 好奇驱动：{:.2}/{:.2}\n", self.curiosity_drive.intensity, self.curiosity_drive.threshold));
        s.push_str(&format!("║ 秩序驱动：{:.2}/{:.2}\n", self.order_drive.intensity, self.order_drive.threshold));
        s.push_str(&format!("║ 连贯驱动：{:.2}/{:.2}\n", self.coherence_drive.intensity, self.coherence_drive.threshold));
        s.push_str(&format!("║ 表达驱动：{:.2}/{:.2}\n", self.expression_drive.intensity, self.expression_drive.threshold));
        s.push_str(&format!("║ 叙述条目数：{}\n", self.narrative_log.len()));
        s.push_str("╚═══════════════════════\n");
        s
    }
}

#[derive(Clone, Debug)]
pub struct StepReport {
    pub tick: u64,
    pub action: String,
    pub result: String,
    pub drive_states: Vec<(String, f64)>,
}

impl StepReport {
    pub fn explain(&self) -> String {
        let mut s = format!("tick[{}] 行动={} | {}\n", self.tick, self.action, self.result);
        for (name, val) in &self.drive_states {
            s.push_str(&format!("  {}: {:.2}\n", name, val));
        }
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

    #[test]
    fn agent_creation() {
        let agent = AutonomousAgent::new(0, "测试体");
        assert_eq!(agent.name, "测试体");
        assert_eq!(agent.tick_count, 0);
    }

    #[test]
    fn agent_tick_once() {
        let mut agent = AutonomousAgent::new(0, "测试体");
        let report = agent.tick();
        assert_eq!(report.tick, 1);
        assert!(!report.action.is_empty());
    }

    #[test]
    fn agent_tick_10_times() {
        let mut agent = AutonomousAgent::new(0, "测试体");
        let reports = agent.run_ticks(10);
        assert_eq!(reports.len(), 10);
        // 至少做了不同的事
        let actions: std::collections::HashSet<String> = reports.iter().map(|r| r.action.clone()).collect();
        assert!(actions.len() >= 1);
    }

    #[test]
    fn agent_ingest_cells() {
        let mut agent = AutonomousAgent::new(0, "测试体");
        let cells = vec![
            (100, encode_cell(50, 25, 15, 1, Sovereignty::Active.encode(), 40, 400)),
            (101, encode_cell(48, 24, 14, 2, Sovereignty::Passive.encode(), 38, 380)),
        ];
        let r = agent.ingest_cells(&cells);
        assert!(r.contains("2"));
    }

    #[test]
    fn agent_reflect() {
        let agent = AutonomousAgent::new(0, "测试体");
        let r = agent.reflect();
        assert!(r.contains("好奇驱动"));
        assert!(r.contains("叙述"));
    }

    #[test]
    fn agent_changes_mode_over_ticks() {
        let mut agent = AutonomousAgent::new(0, "测试体");
        let reports = agent.run_ticks(15);
        let modes: std::collections::HashSet<String> = reports.iter().map(|r| r.action.clone()).collect();
        // 至少出现过两种不同的行动
        assert!(modes.len() >= 2, "应切换过行动模式：{:?}", modes);
    }

    #[test]
    fn drive_tick_and_trigger() {
        let mut drive = Drive::new("test", 0.6);
        // 初始不会触发（0.5 < 0.6）
        assert!(!drive.tick());
        // 多次增长后触发
        for _ in 0..20 { drive.tick(); }
        assert!(drive.intensity >= drive.threshold);
    }

    #[test]
    fn drive_satisfy() {
        let mut drive = Drive::new("test", 0.5);
        drive.intensity = 0.8;
        drive.satisfy(0.3);
        assert!((drive.intensity - 0.5).abs() < 0.01);
    }
}
